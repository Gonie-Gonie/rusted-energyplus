//! Direct procedure-owner checks; profiles are authored input, never weather results.
use super::super::source_design_day::{
    SourceDesignDayInputOwners, SourceDesignDayInputState, SourceDesignDayModifiers,
    SourceDesignDayOutputContext, SourceDesignDaySiteScheduleOutputOwners,
};
use crate::heat_balance::ctf_reports::PredefinedStringTable;
use crate::heat_balance::error_backend::{
    SourceErrorBackend, SourceErrorLevel, SourceErrorState, SourceErrorTracking,
};
use crate::heat_balance::predefined_tables::PredefinedRegistry;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::SelectedHeatBalanceInput;
use crate::heat_balance::selected_input_processor::{
    SelectedInputProcessor, SourceInputFamily, SourceInputProcessorShortCut,
    SourceInputStatisticsInput, SourceInputStatisticsValue, SourceObjectDefinition,
    SourceObjectShape,
};
use crate::heat_balance::selected_project_data::{ProjectInputObject, ProjectInputValue};
use crate::heat_balance::selected_schedule_manager::{
    SelectedScheduleInput, SelectedScheduleManager, SourceScheduleTypeInput,
};
use crate::heat_balance::source_initialization::ScheduleAuditFileHost;
use crate::heat_balance::source_output_prescan::SourceOutputPreScan;
use crate::heat_balance::source_output_registration::{
    SourceOutputDictionaryHost, SourceOutputFileHost, SourceOutputRegistration,
    SourceOutputReportStreams, SourceOutputUnits, SourceRealOutputTarget,
    SourceWeatherRealOutputField, TimeStepKind,
};
use crate::weather::day::{
    DailyWeatherVariables, SourceEnvironmentData, WeatherEnvironmentState, WeatherGlobalState,
};
use ep_model::{ReportInputs, ScheduleUnitType, SourceOutputVariableInput};
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::io::{self, Write};
use std::path::Path;
type TestResult = Result<(), Box<dyn std::error::Error>>;

#[derive(Default)]
struct Files {
    eso: Vec<u8>,
    mtr: Vec<u8>,
    framework: Vec<String>,
    mtd_open: bool,
}
impl SourceOutputFileHost for Files {
    fn ensure_mtd_open(&mut self) -> io::Result<()> {
        self.mtd_open = true;
        Ok(())
    }
}
impl SourceOutputReportStreams for Files {
    fn eso_good(&self) -> bool {
        true
    }
    fn mtr_good(&self) -> bool {
        true
    }
    fn write_eso(&mut self, bytes: &[u8]) -> io::Result<()> {
        self.eso.write_all(bytes)
    }
    fn write_mtr(&mut self, bytes: &[u8]) -> io::Result<()> {
        self.mtr.write_all(bytes)
    }
    fn sqlite_present(&self) -> bool {
        false
    }
}
impl SourceOutputDictionaryHost for Files {
    fn eso_good(&self) -> bool {
        true
    }
    fn write_eso_dictionary(&mut self, line: &str) -> io::Result<()> {
        self.eso.write_all(line.as_bytes())
    }
    fn sqlite_present(&self) -> bool {
        false
    }
    fn results_framework_output_variables(&mut self) -> &mut Vec<String> {
        &mut self.framework
    }
}
#[derive(Default)]
struct Eio(Vec<u8>);
impl Write for Eio {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.0.write(bytes)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }
}
impl super::super::source_weather_initialization::SourceWeatherEio for Eio {
    fn good(&self) -> bool {
        true
    }
}
#[derive(Default)]
struct Audit(Vec<u8>);
impl ScheduleAuditFileHost for Audit {
    fn audit_path(&self) -> &Path {
        Path::new("unit-design-day-schedules.audit")
    }
    fn ensure_audit_open(&mut self) -> io::Result<&mut dyn Write> {
        Ok(&mut self.0)
    }
}

fn error_state() -> SourceErrorState {
    SourceErrorState {
        output_error_header: true,
        version_string: "unit caller".into(),
        idd_version_string: "unit input".into(),
        doing_input_processing: false,
        print_console_output: false,
        kickoff_simulation: false,
        developer_flag: false,
        warmup_flag: false,
        doing_sizing: false,
        tracking: SourceErrorTracking {
            ask_for_plant_check_on_abort: false,
            exit_during_simulations: false,
            ask_for_connections_report: false,
            ask_for_surfaces_report: false,
            total_multiplied_windows: 0,
            total_coincident_vertices: 0,
            total_degenerate_surfaces: 0,
            total_receiving_non_convex_surfaces: 0,
            total_casting_non_convex_surfaces: 0,
            match_counts: [0; 20],
            abort_processing: false,
            total_severe_errors: 0,
            total_severe_errors_during_warmup: 0,
            total_severe_errors_during_sizing: 0,
            last_severe_error: String::new(),
            total_warning_errors: 0,
            total_warning_errors_during_warmup: 0,
            total_warning_errors_during_sizing: 0,
        },
    }
}

fn raw_input(raw: &Value) -> Result<SelectedInputProcessor, Box<dyn std::error::Error>> {
    // Literal official26.1 legacy_idd shapes, not runtime-generated expected values.
    let definitions = [
        ("SizingPeriod:DesignDay", 13, 15),
        ("ScheduleTypeLimits", 3, 2),
        ("Schedule:Day:Hourly", 2, 24),
        ("Output:Schedules", 1, 0),
        ("Output:Variable", 4, 0),
    ]
    .into_iter()
    .map(|(name, a, n)| SourceObjectDefinition {
        object_type: name.into(),
        alpha_fields: a,
        numeric_fields: n,
        alpha_extensions: 0,
        numeric_extensions: 0,
        extension: None,
    })
    .collect();
    let mut statistics = SourceInputStatisticsInput::default();
    let mut families = BTreeMap::new();
    for (family, objects) in raw.as_object().ok_or("raw root")? {
        let mut shapes = Vec::new();
        let mut retained = BTreeMap::new();
        let mut order = Vec::new();
        for (name, fields) in objects.as_object().ok_or("raw family")? {
            let fields = fields.as_object().ok_or("raw fields")?;
            shapes.push(SourceObjectShape {
                object_name: name.clone(),
                field_sizes: fields.keys().map(|key| (key.clone(), 1)).collect(),
            });
            retained.insert(
                name.clone(),
                fields
                    .iter()
                    .map(|(key, value)| {
                        let value = match value {
                            Value::String(value) => {
                                SourceInputStatisticsValue::String(value.clone())
                            }
                            Value::Number(value) => {
                                SourceInputStatisticsValue::Number(value.to_string())
                            }
                            _ => unreachable!(
                                "this authored owner input has scalar known fields only"
                            ),
                        };
                        (key.clone(), value)
                    })
                    .collect(),
            );
            order.push(name.clone());
        }
        families.insert(family.clone(), SourceInputFamily::Objects(shapes));
        statistics.objects.insert(family.clone(), retained);
        statistics.object_order.insert(family.clone(), order);
    }
    let mut input = SelectedInputProcessor::from_admitted_input(definitions, families)?;
    input.retain_source_input_statistics(statistics)?;
    Ok(input)
}

fn day(dry: &str, dry_name: &str, humidity: &str, humidity_name: &str, solar: &str) -> Value {
    json!({"month":7,"day_of_month":21,"day_type":"SummerDesignDay",
        "maximum_dry_bulb_temperature":32,"daily_dry_bulb_temperature_range":15.2,
        "dry_bulb_temperature_range_modifier_type":dry,
        "dry_bulb_temperature_range_modifier_day_schedule_name":dry_name,
        "humidity_condition_type":humidity,"humidity_condition_day_schedule_name":humidity_name,
        "wetbulb_or_dewpoint_at_maximum_dry_bulb":15,"daily_wet_bulb_temperature_range":3,
        "wind_speed":4,"wind_direction":0,"barometric_pressure":81198,
        "solar_model_indicator":solar,"sky_clearness":1,
        "beam_solar_day_schedule_name":"BEAM","diffuse_solar_day_schedule_name":"DIFFUSE"})
}

pub(super) fn verify() -> TestResult {
    let mut raw = json!({"ScheduleTypeLimits":{"Any":{"numeric_type":"Continuous"}},
        "Schedule:Day:Hourly":{},"SizingPeriod:DesignDay":{},"Output:Variable":{}});
    for name in ["BEAM", "DIFFERENCE", "DIFFUSE", "RH", "SHARED", "TEMP"] {
        let mut fields = json!({"schedule_type_limits_name":"Any"});
        for hour in 1..=24 {
            let value = match name {
                "BEAM" => 100.0 + 4.0 * f64::from(hour),
                "DIFFERENCE" => 0.5 + f64::from(hour) / 16.0,
                "DIFFUSE" => 32.0 + 2.0 * f64::from(hour),
                "RH" => 30.0 + f64::from(hour),
                "SHARED" => 0.25 + f64::from(hour % 4) / 8.0,
                "TEMP" => -30.0 + f64::from(hour) / 4.0,
                _ => unreachable!(),
            };
            fields[format!("hour_{hour}")] = json!(value);
        }
        raw["Schedule:Day:Hourly"][name] = fields;
    }
    raw["SizingPeriod:DesignDay"]["01 MULTI"] = day(
        "MultiplierSchedule",
        "SHARED",
        "RelativeHumiditySchedule",
        "RH",
        "Schedule",
    );
    raw["SizingPeriod:DesignDay"]["02 DIFFERENCE"] = day(
        "DifferenceSchedule",
        "DIFFERENCE",
        "WetBulbProfileMultiplierSchedule",
        "SHARED",
        "ASHRAEClearSky",
    );
    let mut profile = day(
        "TemperatureProfileSchedule",
        "TEMP",
        "WetBulb",
        "unused",
        "ASHRAEClearSky",
    );
    profile["wetbulb_or_dewpoint_at_maximum_dry_bulb"] = json!(-5);
    raw["SizingPeriod:DesignDay"]["03 PROFILE"] = profile.clone();
    profile
        .as_object_mut()
        .ok_or("profile fields")?
        .remove("maximum_dry_bulb_temperature");
    profile["humidity_condition_type"] = json!("WetBulbProfileDifferenceSchedule");
    profile["humidity_condition_day_schedule_name"] = json!("DIFFERENCE");
    raw["SizingPeriod:DesignDay"]["04 PROFILE OMITTED DB"] = profile;
    let mut missing = day(
        "DefaultMultipliers",
        "unused",
        "RelativeHumiditySchedule",
        "MISSING RH",
        "Schedule",
    );
    missing["beam_solar_day_schedule_name"] = json!("ABSENT BEAM");
    missing["diffuse_solar_day_schedule_name"] = json!("ABSENT DIFFUSE");
    raw["SizingPeriod:DesignDay"]["05 MISSING"] = missing;
    let mut missing = day(
        "MultiplierSchedule",
        "ABSENT DRY",
        "WetBulb",
        "unused",
        "Schedule",
    );
    missing["beam_solar_day_schedule_name"] = json!("");
    missing["begin_environment_reset_mode"] = json!("SuppressAllBeginEnvironmentResets");
    raw["SizingPeriod:DesignDay"]["06 MISSING DRY"] = missing;
    raw["SizingPeriod:DesignDay"]["07 MISSING WB"] = day(
        "DefaultMultipliers",
        "unused",
        "WetBulbProfileMultiplierSchedule",
        "MISSING WB",
        "ASHRAEClearSky",
    );
    let labels = [
        "Sizing Period Site Drybulb Temperature Range Modifier Schedule Value",
        "Sizing Period Site Humidity Condition Schedule Value",
        "Sizing Period Site Beam Solar Schedule Value",
        "Sizing Period Site Diffuse Solar Schedule Value",
    ];
    let mut report_inputs = ReportInputs::default();
    for (i, label) in labels.iter().enumerate() {
        let name = format!("Request {i}");
        raw["Output:Variable"][&name] =
            json!({"key_value":"*","variable_name":label,"reporting_frequency":"Timestep"});
        report_inputs
            .output_variables
            .push(SourceOutputVariableInput {
                object_name: name,
                key: "*".into(),
                variable_name: label.to_ascii_uppercase(),
                reporting_frequency: "TIMESTEP".into(),
                schedule_name: String::new(),
                alpha_blank: [false, false, false, true],
                default_applied: [false; 4],
                num_alphas: 3,
                is_epjson: true,
            });
    }
    let mut input = raw_input(&raw)?;
    assert_eq!(
        input
            .source_unused_objects()
            .filter(|(kind, _)| *kind == "SizingPeriod:DesignDay")
            .count(),
        7
    );
    let mut shortcut = SourceInputProcessorShortCut::default();
    shortcut.allocate_from_input(&input)?;
    let mut schedules = SelectedScheduleManager::default();
    schedules.init_constant_state(0)?;
    let declarations = SelectedScheduleInput {
        source_is_epjson: Some(true),
        constants: Vec::new(),
        types: vec![SourceScheduleTypeInput {
            object_name: "Any".into(),
            name: "ANY".into(),
            lower: None,
            upper: None,
            numeric_type: Some("CONTINUOUS".into()),
            unit: ScheduleUnitType::Invalid,
        }],
    };
    schedules.process_schedule_input(&mut input, &declarations, 4, &mut Audit::default())?;
    assert!(schedules.schedule_input_processed);
    assert_eq!(schedules.day_schedules.len(), 7);
    assert!(
        schedules
            .day_schedules
            .iter()
            .skip(1)
            .all(|day| !day.is_used)
    );
    let day_objects = raw["SizingPeriod:DesignDay"]
        .as_object()
        .ok_or("DD map")?
        .iter()
        .map(|(name, fields)| {
            Ok(ProjectInputObject {
                object_name: name.clone(),
                idf_max_fields: None,
                fields: fields
                    .as_object()
                    .ok_or("DD scalar fields")?
                    .iter()
                    .map(|(key, value)| {
                        let value = match value {
                            Value::String(value) => ProjectInputValue::String(value.clone()),
                            Value::Number(value) => ProjectInputValue::Number(
                                value.as_f64().expect("finite numeric input"),
                            ),
                            _ => unreachable!(),
                        };
                        (key.clone(), value)
                    })
                    .collect(),
            })
        })
        .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>()?;
    let hb = SelectedHeatBalanceInput {
        source_is_epjson: true,
        objects: BTreeMap::from([("SizingPeriod:DesignDay".into(), day_objects)]),
        ..Default::default()
    };
    let clock = WeatherGlobalState {
        time_steps_in_hour: 4,
        minutes_in_time_step: 15,
        time_step_zone: 0.25,
        ..Default::default()
    };
    let interpolation = Some(vec![0.25, 0.5, 0.75, 1.0]);
    let mut state = SourceDesignDayInputState::default();
    let mut environments = vec![SourceEnvironmentData::default(); 7];
    let weather = vec![DailyWeatherVariables::default(); 7];
    let mut registry = PredefinedRegistry::default();
    registry.set_predefined_tables(false, false)?;
    let mut table = PredefinedStringTable::default();
    let globals = SelectedHeatBalanceGlobals::default();
    let (mut files, mut output_host, mut eio) =
        (Files::default(), Files::default(), Eio::default());
    let (mut error_bytes, mut console) = (Vec::new(), Vec::new());
    let mut errors = error_state();
    let mut callbacks = Vec::new();
    let mut registration = SourceOutputRegistration::default();
    let mut prescan = SourceOutputPreScan::default();
    let mut errors_found = false;
    {
        let mut callback = |_: &mut SourceErrorState, level: SourceErrorLevel, message: &[u8]| {
            callbacks.push((
                level,
                std::str::from_utf8(message)
                    .expect("ASCII diagnostic")
                    .to_owned(),
            ));
        };
        let mut backend = SourceErrorBackend {
            state: &mut errors,
            error_stream: Some(&mut error_bytes),
            console: &mut console,
            out_unit1: None,
            out_unit2: None,
            f_message: None,
            message_callback: None,
            error_callback: Some(&mut callback),
            sqlite: None,
        };
        registration
            .time_pointers
            .setup_time_pointer(TimeStepKind::Zone, &mut backend)?;
        registration
            .time_pointers
            .setup_time_pointer(TimeStepKind::System, &mut backend)?;
        for label in labels {
            prescan.add_record("*", label, &mut backend)?;
        }
        let mut output = SourceDesignDayOutputContext {
            output_host: &mut output_host,
            prescan: &prescan,
            report_inputs: &report_inputs,
            minimum_report_frequency: "",
        };
        let mut owners = SourceDesignDayInputOwners {
            raw: &hb,
            input: &mut input,
            shortcut: &mut shortcut,
            schedules: &mut schedules,
            output: &mut output,
            registration: &mut registration,
            files: &mut files,
            eio: &mut eio,
            globals: &globals,
            developer_flag: &false,
            clock: &clock,
            force_begin_env_reset_suppress: &false,
            reverse_design_days: &false,
            interpolation: &interpolation,
            environments: &mut environments,
            design_day_weather: &weather,
            registry: &registry,
            table: &mut table,
            backend: &mut backend,
        };
        state.get_source_design_day_data(7, &mut owners, &mut errors_found)?;
    }
    // Deliberately missing operands exercise the direct getter. Its parent would
    // terminate on errors_found; this is not a successful full simulation claim.
    assert!(errors_found);
    assert_eq!(errors.tracking.total_warning_errors, 3);
    assert_eq!(errors.tracking.total_severe_errors, 4);
    assert_eq!(
        input
            .source_unused_objects()
            .filter(|(kind, _)| *kind == "SizingPeriod:DesignDay")
            .count(),
        0
    );
    assert!(
        schedules
            .day_schedules
            .iter()
            .skip(1)
            .all(|day| day.is_used)
    );
    let data = state
        .design_day_input
        .as_ref()
        .ok_or("actual DD allocation")?;
    assert_eq!(
        data.iter()
            .map(|day| day.dry_bulb_range_type)
            .collect::<Vec<_>>(),
        [1, 2, 3, 3, 0, 1, 0]
    );
    assert_eq!(
        data.iter()
            .map(|day| day.humidity_indicator_type)
            .collect::<Vec<_>>(),
        [4, 7, 0, 6, 4, 0, 7]
    );
    assert_eq!(
        data[2].maximum_dry_bulb.to_bits(),
        f64::MIN_POSITIVE.to_bits()
    );
    assert_eq!(
        data[3].maximum_dry_bulb.to_bits(),
        f64::MIN_POSITIVE.to_bits()
    );
    assert!(!data[5].suppress_begin_environment_reset); // Native's official-key spelling mismatch.
    assert!(
        data[4].humidity_indicator_schedule.is_none()
            && data[6].humidity_indicator_schedule.is_none()
    );
    assert_eq!(data[4].humidity_indicator_value, 0.0); // RH switch ignores the entered WB15.
    assert_eq!(data[6].humidity_indicator_value, 15.0);
    assert!(data[4].beam_solar_schedule.is_none() && data[4].diffuse_solar_schedule.is_none());
    assert!(data[5].temperature_range_schedule.is_none() && data[5].beam_solar_schedule.is_none());
    assert!(data[5].diffuse_solar_schedule.is_some());
    assert_eq!(state.sizing_period_site_schedule_numbers, ["SHARED", "RH", "BEAM", "DIFFUSE", "DIFFERENCE", "TEMP"]
        .map(|name| i32::try_from(schedules.day_schedule_map[name]).expect("small input index")));
    let check_copy = |environment: usize,
                      name: &str,
                      select: fn(&SourceDesignDayModifiers) -> f64|
     -> TestResult {
        let grid = state
            .design_day_modifiers
            .as_ref()
            .ok_or("modifier outer array")?[environment]
            .as_ref()
            .ok_or("actual modifier grid")?;
        let day = &schedules.day_schedules[schedules.day_schedule_map[name]];
        for hour in 0..24 {
            for timestep in 0..4 {
                // Real processed day operands, not expected weather histories.
                assert_eq!(
                    select(&grid.values[timestep * 24 + hour]).to_bits(),
                    day.timestep_values[hour * 4 + timestep].to_bits(),
                    "{environment}/{name}/{hour}/{timestep}"
                );
            }
        }
        Ok(())
    };
    check_copy(0, "SHARED", |v| v.outdoor_dry_bulb_temperature)?;
    check_copy(0, "RH", |v| v.outdoor_relative_humidity)?;
    check_copy(0, "BEAM", |v| v.beam_solar_radiation)?;
    check_copy(0, "DIFFUSE", |v| v.diffuse_solar_radiation)?;
    check_copy(1, "DIFFERENCE", |v| v.outdoor_dry_bulb_temperature)?;
    check_copy(1, "SHARED", |v| v.outdoor_relative_humidity)?;
    check_copy(2, "TEMP", |v| v.outdoor_dry_bulb_temperature)?;
    check_copy(3, "TEMP", |v| v.outdoor_dry_bulb_temperature)?;
    check_copy(3, "DIFFERENCE", |v| v.outdoor_relative_humidity)?;
    check_copy(5, "DIFFUSE", |v| v.diffuse_solar_radiation)?;
    for environment in [4, 6] {
        assert!(
            state.design_day_modifiers.as_ref().ok_or("modifiers")?[environment]
                .as_ref()
                .ok_or("missing humidity grid")?
                .values
                .iter()
                .all(|cell| cell.outdoor_relative_humidity == 0.0)
        );
    }
    assert_eq!(registration.out_vars.len(), 6);
    assert!(output_host.mtd_open);
    use SourceOutputUnits as U;
    use SourceWeatherRealOutputField as F;
    let bindings = [
        (
            "SHARED",
            F::SizingPeriodDryBulbSchedule { environment: 1 },
            U::Dimensionless,
            7.75,
        ),
        (
            "RH",
            F::SizingPeriodHumiditySchedule { environment: 1 },
            U::Percentage,
            8.5,
        ),
        (
            "BEAM",
            F::SizingPeriodBeamSolarSchedule { environment: 1 },
            U::WPerM2,
            125.0,
        ),
        (
            "DIFFUSE",
            F::SizingPeriodDiffuseSolarSchedule { environment: 1 },
            U::WPerM2,
            53.0,
        ),
        (
            "DIFFERENCE",
            F::SizingPeriodDryBulbSchedule { environment: 2 },
            U::DeltaC,
            2.25,
        ),
        (
            "TEMP",
            F::SizingPeriodDryBulbSchedule { environment: 3 },
            U::C,
            -24.5,
        ),
    ];
    let mut indices = Vec::new();
    for (name, field, units, _) in bindings {
        let index = registration
            .out_vars
            .iter()
            .position(|var| var.key == name)
            .ok_or("actual registered day key")?;
        assert_eq!(
            registration.out_vars[index].which,
            Some(SourceRealOutputTarget::Weather { field })
        );
        assert_eq!(registration.out_vars[index].units, units);
        indices.push(index);
    }
    assert!(
        !registration
            .out_vars
            .iter()
            .any(|var| var.key == "SHARED" && var.name == labels[1])
    );
    let owners = SourceDesignDaySiteScheduleOutputOwners {
        schedules: &state.sizing_period_site_schedules,
    };
    for &index in &indices {
        assert_eq!(*registration.read_registered_real(index, &owners)?, 0.0);
    }
    state.sizing_period_site_schedules[0].outdoor_dry_bulb_temperature = 7.75;
    state.sizing_period_site_schedules[0].outdoor_relative_humidity = 8.5;
    state.sizing_period_site_schedules[0].beam_solar_radiation = 125.0;
    state.sizing_period_site_schedules[0].diffuse_solar_radiation = 53.0;
    state.sizing_period_site_schedules[1].outdoor_dry_bulb_temperature = 2.25;
    state.sizing_period_site_schedules[1].outdoor_relative_humidity = 99.25;
    state.sizing_period_site_schedules[2].outdoor_dry_bulb_temperature = -24.5;
    state.sizing_period_site_schedules[3].outdoor_dry_bulb_temperature = 123.0;
    state.sizing_period_site_schedules[3].outdoor_relative_humidity = 73.0;
    let owners = SourceDesignDaySiteScheduleOutputOwners {
        schedules: &state.sizing_period_site_schedules,
    };
    for (&index, (_, _, _, value)) in indices.iter().zip(bindings) {
        assert_eq!(*registration.read_registered_real(index, &owners)?, value);
    }
    let weather_clock = WeatherGlobalState {
        kind_of_sim: 3,
        ..clock
    };
    let environment = WeatherEnvironmentState {
        tot_des_days: 7,
        ..Default::default()
    };
    state.update_source_site_schedule_values(&weather_clock, &environment, 0, None)?;
    let owners = SourceDesignDaySiteScheduleOutputOwners {
        schedules: &state.sizing_period_site_schedules,
    };
    for &index in &indices {
        assert_eq!(*registration.read_registered_real(index, &owners)?, -999.0);
    }
    use SourceErrorLevel::{Continue, Severe, Warning};
    let expected = [
        (Warning, "SizingPeriod:DesignDay=\"03 PROFILE\", data override."),
        (Continue, "..maximum_dry_bulb_temperature=[32.00] will be overwritten."),
        (Continue, "..dry_bulb_temperature_range_modifier_type=\"TEMPERATUREPROFILESCHEDULE\"."),
        (Continue, "..with max value=[2.23E-308]."),
        (Warning, "GetDesignDayData: SizingPeriod:DesignDay = 05 MISSING"),
        (Continue, "humidity_condition_day_schedule_name = MISSING RH, item not found, Default Humidity (constant for day using Humidity Indicator Temp). will be used."),
        (Severe, "GetDesignDayData: SizingPeriod:DesignDay = 05 MISSING"),
        (Continue, "beam_solar_day_schedule_name = ABSENT BEAM, item not found."),
        (Severe, "GetDesignDayData: SizingPeriod:DesignDay = 05 MISSING"),
        (Continue, "diffuse_solar_day_schedule_name = ABSENT DIFFUSE, item not found."),
        (Severe, "GetDesignDayData: SizingPeriod:DesignDay = 06 MISSING DRY"),
        (Continue, "dry_bulb_temperature_range_modifier_day_schedule_name = ABSENT DRY, item not found."),
        (Severe, "GetDesignDayData: SizingPeriod:DesignDay = 06 MISSING DRY"),
        (Continue, "beam_solar_day_schedule_name cannot be empty."),
        (Warning, "GetDesignDayData: SizingPeriod:DesignDay = 07 MISSING WB"),
        (Continue, "humidity_condition_day_schedule_name = MISSING WB, item not found, Default Humidity (constant for day using Humidity Indicator Temp). will be used."),
    ].map(|(level, text)| (level, text.to_owned()));
    assert_eq!(callbacks, expected);
    Ok(())
}
