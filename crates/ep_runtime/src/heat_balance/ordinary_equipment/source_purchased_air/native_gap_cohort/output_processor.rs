// Same factory and physical owners as the accepted P1 cfg helper. This child
// invokes genuine OutputProcessor operations, never a copied report equation.
use super::*;
use hb::source_output_registration as op;
use op::{SourceMeterReportingHost, SourceOutputReportStreams, SourceP4Values};
use std::path::{Path, PathBuf};

struct ExtraStreams {
    mtr: Vec<u8>,
    mtd: Vec<u8>,
    eso_path: PathBuf,
    mtr_path: PathBuf,
}
struct Streams<'a> {
    eso: &'a mut Vec<u8>,
    mtr: &'a mut Vec<u8>,
    mtd: &'a mut Vec<u8>,
    rf: &'a mut Vec<String>,
    mtd_open: bool,
    sqlite: bool,
    eso_path: &'a Path,
    mtr_path: &'a Path,
}
impl SourceOutputReportStreams for Streams<'_> {
    fn eso_good(&self) -> bool {
        true
    }
    fn mtr_good(&self) -> bool {
        true
    }
    fn write_eso(&mut self, b: &[u8]) -> io::Result<()> {
        self.eso.extend_from_slice(b);
        Ok(())
    }
    fn write_mtr(&mut self, b: &[u8]) -> io::Result<()> {
        self.mtr.extend_from_slice(b);
        Ok(())
    }
    fn sqlite_present(&self) -> bool {
        self.sqlite
    }
}
impl SourceMeterReportingHost for Streams<'_> {
    fn write_mtd(&mut self, b: &[u8]) -> io::Result<()> {
        if !self.mtd_open {
            return Err(io::Error::new(
                io::ErrorKind::NotConnected,
                "real P4 MTD open phase",
            ));
        }
        self.mtd.extend_from_slice(b);
        Ok(())
    }
    fn results_framework_output_variables(&mut self) -> &mut Vec<String> {
        self.rf
    }
    fn eso_file_path(&self) -> &Path {
        self.eso_path
    }
    fn mtr_file_path(&self) -> &Path {
        self.mtr_path
    }
}
fn prescan(s: &Factory) -> Value {
    Value::Array(s.prescan.output_variables_for_simulation.values().map(|group|json!({"map_name":group.variable_name,"keys":group.keys.values().map(|v|json!({"map_key":v.key,"key":v.key,"variableName":v.variable_name,"is_simple_string":v.is_simple_string})).collect::<Vec<_>>()})).collect())
}
fn context(s: &Factory) -> Value {
    let g = &s.weather.global;
    let h = &s.startup.hvac_globals;
    let e = &s.weather.environment;
    json!({"state_address":pointer(s),"global_address":pointer(g),"hvac_address":pointer(h),"environment_address":pointer(e),"TimeStepZone":number(g.time_step_zone),"TimeStepSys":number(h.time_step_sys),"TimeStepSysSec":number(h.time_step_sys_seconds),"MinutesInTimeStep":g.minutes_in_time_step,"TimeStep":g.time_step,"TimeStepsInHour":g.time_steps_in_hour,"HourOfDay":g.hour_of_day,"DayOfSim":g.day_of_sim,"DayOfSimChr":g.day_of_sim_chr,"CalendarYear":g.calendar_year,"CalendarYearChr":g.calendar_year_chr,"NumOfDayInEnvrn":g.num_of_day_in_envrn,"WarmupFlag":g.warmup_flag,"EndHourFlag":g.end_hour_flag,"EndDayFlag":g.end_day_flag,"EndEnvrnFlag":g.end_envrn_flag,"Month":e.month,"DayOfMonth":e.day_of_month,"DayOfWeek":e.day_of_week,"HolidayIndex":e.holiday_index,"DSTIndicator":e.dst_indicator,"EndMonthFlag":e.end_month_flag,"EndYearFlag":e.end_year_flag,"StdOutputRecordCount":s.ancillary.source_standard_output_record_count,"StdMeterRecordCount":s.ancillary.source_standard_meter_record_count,"sqlite_present":s.ancillary.source_sqlite.present(),"prescan_address":pointer(&s.prescan),"NumConsideredOutputVariables":s.prescan.num_considered_output_variables,"MaxConsideredOutputVariables":s.prescan.max_considered_output_variables,"prescan":prescan(s)})
}
fn snapshot(s: &Factory, v: &SourceP4Values, x: &ExtraStreams) -> Result<Value> {
    if s.ancillary.source_sqlite.present() {
        return Err(invalid("P4 real null SQLite domain").into());
    }
    Ok(
        json!({"context":context(s),"named_lvalues":v.native_p4_observe(),"output_processor":s.registration.native_p4_observe(v,&op::LiveTimeStepOwners{global:&s.weather.global,hvac:&s.startup.hvac_globals})?,"results_framework":op::native_p4_results(&s.ancillary.source_results_framework)?,"streams":{"eso":std::str::from_utf8(&s.eso)?,"mtr":std::str::from_utf8(&x.mtr)?,"eio":std::str::from_utf8(&s.eio)?,"mtd":std::str::from_utf8(&x.mtd)?},"errors":errors(s)?}),
    )
}
fn apply_inputs(s: &mut Factory, v: &mut SourceP4Values, p: &Value) -> Result<()> {
    const ALLOWED: &[&str] = &[
        "zone_average",
        "system_average",
        "system_energy",
        "system_integer",
        "zone_sum_integer",
        "zone_hours",
        "system_hours",
        "system_seconds",
        "hour",
        "timestep",
        "day_of_sim",
        "calendar_year",
        "environment_days",
        "month",
        "day_of_month",
        "day_of_week",
        "holiday",
        "dst",
        "warmup",
        "end_hour",
        "end_day",
        "end_environment",
        "end_month",
        "end_year",
    ];
    for key in object(p)?.keys() {
        if !ALLOWED.contains(&key.as_str()) {
            return Err(invalid("P4 only named Which/caller primary inputs").into());
        }
    }
    macro_rules! real_field {
        ($key:literal,$target:expr) => {
            if let Some(x) = p.get($key) {
                $target = real(x)?;
            }
        };
    }
    macro_rules! int_field {
        ($key:literal,$target:expr) => {
            if let Some(x) = p.get($key) {
                $target = int(x)?;
            }
        };
    }
    macro_rules! bool_field {
        ($key:literal,$target:expr) => {
            if let Some(x) = p.get($key) {
                $target = x
                    .as_bool()
                    .ok_or_else(|| invalid("P4 actual bool primary"))?;
            }
        };
    }
    real_field!("zone_average", v.zone_average);
    real_field!("system_average", v.system_average);
    real_field!("system_energy", v.system_energy);
    int_field!("system_integer", v.system_integer);
    int_field!("zone_sum_integer", v.zone_sum_integer);
    let g = &mut s.weather.global;
    let h = &mut s.startup.hvac_globals;
    let e = &mut s.weather.environment;
    real_field!("zone_hours", g.time_step_zone);
    real_field!("system_hours", h.time_step_sys);
    real_field!("system_seconds", h.time_step_sys_seconds);
    int_field!("hour", g.hour_of_day);
    int_field!("timestep", g.time_step);
    int_field!("environment_days", g.num_of_day_in_envrn);
    if let Some(x) = p.get("day_of_sim") {
        g.day_of_sim = int(x)?;
        g.day_of_sim_chr = g.day_of_sim.to_string();
    }
    if let Some(x) = p.get("calendar_year") {
        g.calendar_year = int(x)?;
        g.calendar_year_chr = g.calendar_year.to_string();
    }
    int_field!("month", e.month);
    int_field!("day_of_month", e.day_of_month);
    int_field!("day_of_week", e.day_of_week);
    int_field!("holiday", e.holiday_index);
    int_field!("dst", e.dst_indicator);
    bool_field!("warmup", g.warmup_flag);
    bool_field!("end_hour", g.end_hour_flag);
    bool_field!("end_day", g.end_day_flag);
    bool_field!("end_environment", g.end_envrn_flag);
    bool_field!("end_month", e.end_month_flag);
    bool_field!("end_year", e.end_year_flag);
    if g.minutes_in_time_step <= 0
        || !(1..=24).contains(&g.hour_of_day)
        || !(1..=12).contains(&e.month)
        || !(1..=31).contains(&e.day_of_month)
        || !(1..=7).contains(&e.day_of_week)
        || !(0..13).contains(&e.holiday_index)
        || g.time_step_zone <= 0.0
        || h.time_step_sys <= 0.0
    {
        return Err(invalid("P4 actual finite caller clock domain").into());
    }
    Ok(())
}
fn setup(
    s: &mut Factory,
    v: &SourceP4Values,
    x: &mut ExtraStreams,
    definition: &Value,
    which: usize,
) -> Result<()> {
    if which <= 1 {
        let mut backend = SourceErrorBackend {
            state: &mut s.error_state,
            error_stream: Some(&mut s.error),
            console: &mut s.console,
            out_unit1: None,
            out_unit2: None,
            f_message: None,
            message_callback: None,
            error_callback: None,
            sqlite: None,
        };
        s.registration.time_pointers.setup_time_pointer(
            if which == 0 {
                op::TimeStepKind::Zone
            } else {
                op::TimeStepKind::System
            },
            &mut backend,
        )?;
        return Ok(());
    }
    if which == 2 {
        let mut input = op::SourceReportVariableInput::new(
            &s.report,
            &mut s.input,
            &s.project.system.minimum_report_frequency,
        );
        s.registration
            .native_p4_get_report_input(&mut input, &s.files, &mut s.eio)?;
        return Ok(());
    }
    if (3..=7).contains(&which) {
        let mut backend = SourceErrorBackend {
            state: &mut s.error_state,
            error_stream: Some(&mut s.error),
            console: &mut s.console,
            out_unit1: None,
            out_unit2: None,
            f_message: None,
            message_callback: None,
            error_callback: None,
            sqlite: None,
        };
        let mut input = op::SourceReportVariableInput::new(
            &s.report,
            &mut s.input,
            &s.project.system.minimum_report_frequency,
        );
        let mut dict = op::SourceOutputDictionaryStreams {
            eso: Some(&mut s.eso),
            sqlite_present: s.ancillary.source_sqlite.present(),
            results_framework_output_variables: &mut s.rf,
        };
        if which <= 5 {
            let (name, unit, store, kind, slot) = match which {
                3 => (
                    "P4 Zone Average",
                    op::SourceOutputUnits::C,
                    op::SourceOutputStoreType::Average,
                    op::TimeStepKind::Zone,
                    0,
                ),
                4 => (
                    "P4 System Average",
                    op::SourceOutputUnits::C,
                    op::SourceOutputStoreType::Average,
                    op::TimeStepKind::System,
                    1,
                ),
                5 => (
                    "P4 System Energy",
                    op::SourceOutputUnits::J,
                    op::SourceOutputStoreType::Sum,
                    op::TimeStepKind::System,
                    2,
                ),
                _ => return Err(invalid("P4 setup selector").into()),
            };
            let meter = if which == 5 {
                Some(op::SourceGeneralMeterArguments {
                    resource: op::SourceMeterResource::Electricity,
                    end_use: op::SourceMeterEndUse::Heating,
                    end_use_subcategory: "General",
                    group: op::SourceMeterGroup::Hvac,
                    zone_name: "",
                    space_type: "",
                    zone_multiplier: 2,
                    zone_list_multiplier: 3,
                })
            } else {
                None
            };
            let binding = op::SourceRealOutputBinding::new(
                op::SourceRealOutputTarget::CohortReal { slot },
                v,
            )?;
            s.registration.setup_bound_general_real(
                &mut input,
                &s.prescan,
                op::SourceRealOutputDeclaration {
                    name,
                    key: "P4 OWNER",
                    units: unit,
                    store_type: store,
                },
                s.weather.global.minutes_in_time_step,
                &mut s.files,
                &mut s.eio,
                binding,
                &mut dict,
                op::SourceGeneralRealRegistrationOptions {
                    time_step: kind,
                    meter,
                },
                &mut backend,
            )?;
        } else {
            let (name, store, kind, slot) = if which == 6 {
                (
                    "P4 System Integer",
                    op::SourceOutputStoreType::Average,
                    op::TimeStepKind::System,
                    0,
                )
            } else {
                (
                    "P4 Zone Sum Integer",
                    op::SourceOutputStoreType::Sum,
                    op::TimeStepKind::Zone,
                    1,
                )
            };
            let binding = op::SourceIntegerOutputBinding::new(
                op::SourceIntegerOutputTarget::CohortInteger { slot },
                v,
            )?;
            s.registration.setup_bound_integer(
                &mut input,
                &s.prescan,
                op::SourceIntegerOutputDeclaration {
                    name,
                    key: "P4 OWNER",
                    units: op::SourceOutputUnits::Dimensionless,
                    store_type: store,
                },
                s.weather.global.minutes_in_time_step,
                &mut s.files,
                &mut s.eio,
                binding,
                &mut dict,
                kind,
            )?;
        }
        return Ok(());
    }
    if which == 8 {
        let raw = SelectedHeatBalanceInput {
            source_is_epjson: false,
            surface: Default::default(),
            objects: project_objects(
                definition,
                &[
                    "Output:Meter",
                    "Output:Meter:MeterFileOnly",
                    "Output:Meter:Cumulative",
                    "Output:Meter:Cumulative:MeterFileOnly",
                    "Meter:Custom",
                    "Meter:CustomDecrement",
                ],
            )?,
        };
        let mut backend = SourceErrorBackend {
            state: &mut s.error_state,
            error_stream: Some(&mut s.error),
            console: &mut s.console,
            out_unit1: None,
            out_unit2: None,
            f_message: None,
            message_callback: None,
            error_callback: None,
            sqlite: None,
        };
        let mut host = Streams {
            eso: &mut s.eso,
            mtr: &mut x.mtr,
            mtd: &mut x.mtd,
            rf: &mut s.rf,
            mtd_open: s.files.mtd_open,
            sqlite: s.ancillary.source_sqlite.present(),
            eso_path: &x.eso_path,
            mtr_path: &x.mtr_path,
        };
        s.registration.update_source_meter_reporting(
            &mut op::SourceMeterReportingOwners {
                raw: &raw,
                input: &mut s.input,
                shortcut: &mut s.shortcut,
                backend: &mut backend,
            },
            &mut host,
        )?;
        return Ok(());
    }
    Err(invalid("P4 actual setup operation").into())
}
fn update(
    s: &mut Factory,
    v: &SourceP4Values,
    x: &mut ExtraStreams,
    operation: &str,
) -> Result<()> {
    if operation == "warmup_reset" {
        s.registration
            .reset_source_accumulation_when_warmup_complete()?;
        return Ok(());
    }
    let kind = match operation {
        "system" => op::TimeStepKind::System,
        "zone" => op::TimeStepKind::Zone,
        _ => return Err(invalid("P4 full operation").into()),
    };
    let e = &mut s.weather.environment;
    let mut host = Streams {
        eso: &mut s.eso,
        mtr: &mut x.mtr,
        mtd: &mut x.mtd,
        rf: &mut s.rf,
        mtd_open: s.files.mtd_open,
        sqlite: s.ancillary.source_sqlite.present(),
        eso_path: &x.eso_path,
        mtr_path: &x.mtr_path,
    };
    s.registration.update_source_data_and_report(
        kind,
        &mut op::SourceOutputUpdateOwners {
            clock: &mut s.weather.global,
            environment: op::SourceOutputUpdateEnvironment {
                month: &e.month,
                day_of_month: &e.day_of_month,
                day_of_week: &e.day_of_week,
                holiday_index: &e.holiday_index,
                dst_indicator: &e.dst_indicator,
                end_month_flag: &mut e.end_month_flag,
                end_year_flag: &e.end_year_flag,
            },
            hvac: &s.startup.hvac_globals,
            project: &s.project,
            results: &mut s.ancillary.source_results_framework,
            standard_output_record_count: &mut s.ancillary.source_standard_output_record_count,
            standard_meter_record_count: &mut s.ancillary.source_standard_meter_record_count,
            real: v,
            integers: v,
        },
        &mut host,
    )?;
    Ok(())
}

// Which cells and real owners keep their actual stable addresses. Native heap
// row objects versus Source Vec rows have a declared storage-layout bridge:
// semantic row ordinals and every field are compared, allocator addresses are
// never fabricated or claimed to have the same relocation policy.
#[derive(Default)]
struct Compare {
    pairs: BTreeMap<(String, String), String>,
    reverse: BTreeMap<(String, String), String>,
    bits: usize,
    rows: usize,
    unrepresented: BTreeMap<String, usize>,
}
// Explicit observation exclusions, never restored or replaced with constants.
// The Source owner has no storage for these legacy/disabled Native members.
// Every other key must be present in BOTH snapshots, including nested rows.
fn unrepresented_native_field(path: &str, key: &str) -> Option<&'static str> {
    if path.ends_with(".output_processor")
        && matches!(
            key,
            "NumVariablesForOutput"
                | "MaxVariablesForOutput"
                | "GetMeterIndexFirstCall"
                | "InitFlag"
                | "Rept"
                | "apiVarRequests_size"
        )
    {
        Some("unrepresented unused legacy/API-request storage")
    } else if path.ends_with(".results_framework") && matches!(key, "detailed" | "frequency") {
        Some("unrepresented disabled variable-frame metadata")
    } else if path.contains(".context.prescan[")
        && path.contains(".keys[")
        && matches!(key, "pattern_present" | "case_insensitive_pattern_present")
    {
        Some("unrepresented regex-pointer storage; Source literal-key domain remains checked")
    } else if path.contains(".output_processor.ddOutVars[") && key == "unitNameCustomEMS" {
        Some(
            "unrepresented dictionary custom-unit storage; actual named declarations use typed units",
        )
    } else if path.contains(".output_processor.reqVars[")
        && key == "case_insensitive_pattern_present"
    {
        Some(
            "unrepresented request regex-pointer storage; Source literal-key domain remains checked",
        )
    } else {
        None
    }
}
fn pointer_kind(path: &str) -> &str {
    if path.ends_with(".Which")
        || path.ends_with(".TimeStep")
        || path.contains(".named_lvalues.addresses.")
    {
        "actual scalar cell"
    } else if path.contains(".time_values[") {
        "actual time-value row"
    } else if path.contains(".results_framework.meters[") {
        "actual meter frame"
    } else if path.contains(".reqVars[") {
        "actual request row"
    } else if path.ends_with(".results_framework.owner_address") {
        "actual ResultsFramework owner"
    } else if path.ends_with(".output_processor.owner_address") {
        "actual OutputProcessor owner"
    } else {
        path.rsplit('.').next().unwrap_or(path)
    }
}
impl Compare {
    fn check(&mut self, s: &Value, n: &Value, path: &str) -> Result<()> {
        match (s, n) {
            (Value::Object(a), Value::Object(b)) => {
                if a.len() == 1 && a.contains_key("value_bits") {
                    assert_eq!(s, n, "{path} rawbinary64");
                    self.bits += 1;
                    return Ok(());
                }
                let native_keys = b
                    .keys()
                    .filter(|key| {
                        if let Some(reason) = unrepresented_native_field(path, key) {
                            *self.unrepresented.entry(reason.to_owned()).or_default() += 1;
                            false
                        } else {
                            true
                        }
                    })
                    .collect::<Vec<_>>();
                assert_eq!(
                    a.keys().collect::<Vec<_>>(),
                    native_keys,
                    "{path} complete represented keys; explicit exclusions only"
                );
                for (key, value) in a {
                    self.check(
                        value,
                        b.get(key).ok_or_else(|| {
                            invalid(&format!("P4 actual Native field {path}.{key}"))
                        })?,
                        &format!("{path}.{key}"),
                    )?;
                }
            }
            (Value::Array(a), Value::Array(b)) => {
                assert_eq!(a.len(), b.len(), "{path} extent");
                for (i, (a, b)) in a.iter().zip(b).enumerate() {
                    self.check(a, b, &format!("{path}[{i}]"))?;
                }
            }
            (Value::String(a), Value::String(b))
                if path.ends_with("address")
                    || path.ends_with("Which")
                    || path.ends_with("TimeStep")
                    || path.contains("named_lvalues.addresses.") =>
            {
                if path.ends_with(".address")
                    && (path.contains(".outVars[")
                        || path.contains(".ddOutVars[")
                        || path.contains(".meters[") && !path.contains("results_framework"))
                {
                    self.rows += 1;
                } else {
                    let kind = pointer_kind(path).to_owned();
                    if let Some(previous) = self.pairs.insert((kind.clone(), b.clone()), a.clone())
                    {
                        assert_eq!(previous, *a, "{path} same live Source owner/lvalue");
                    }
                    if let Some(previous) = self.reverse.insert((kind, a.clone()), b.clone()) {
                        assert_eq!(
                            previous, *b,
                            "{path} real Native alias association within the same logical pointer class"
                        );
                    }
                }
            }
            _ => assert_eq!(s, n, "{path} exact observed state/bytes"),
        }
        Ok(())
    }
}
pub(crate) fn run(fixture: &Value) -> Result<usize> {
    let ingress = member(fixture, "source_inputs")?;
    assert_eq!(
        member(ingress, "schema")?,
        "src06-output-processor-Source-primary-input.v1"
    );
    let inputs = member(member(fixture, "named_inputs")?, "output_processor")?;
    assert_eq!(
        member(inputs, "schema")?,
        "src06-output-processor-gap-input.v1"
    );
    let native = member(
        member(member(fixture, "native_output")?, "groups")?,
        "output_processor",
    )?;
    assert_eq!(
        member(native, "schema")?,
        "src06-output-processor-full-method-batch-output.v1"
    );
    let definitions = member(ingress, "definitions")?;
    let sources = array(member(ingress, "factories")?)?;
    let named = array(member(inputs, "cases")?)?;
    let expected = array(member(native, "cases")?)?;
    if sources.len() != 1 || named.len() != 1 || expected.len() != 1 {
        return Err(invalid("complete one-lifetime P4 named cohort").into());
    }
    let definition = &sources[0];
    let input = &named[0];
    let oracle = &expected[0];
    assert_eq!(member(definition, "name")?, member(input, "name")?);
    assert_eq!(member(oracle, "name")?, member(input, "name")?);
    assert_eq!(member(definition, "steps")?, member(input, "steps")?);
    let mut s = Box::new(Factory::new(definitions, definition)?);
    let mut values = Box::<SourceP4Values>::default();
    // IOFiles.hh597/624 name the actual Native unit memory streams; Source
    // chooses these same explicit test-host names, without physical files.
    let mut extra = ExtraStreams {
        mtr: format!("Program Version,{}\n", s.error_state.version_string).into_bytes(),
        mtd: Vec::new(),
        eso_path: "eplusout.eso".into(),
        mtr_path: "eplusout.mtr".into(),
    };
    // The real Zone getter already declared these four unrequested weather
    // fields and consumed the Output:Variable getter. Preserve that state.
    let factory_dictionary_names = [
        "Zone Outdoor Air Drybulb Temperature",
        "Zone Outdoor Air Wetbulb Temperature",
        "Zone Outdoor Air Wind Speed",
        "Zone Outdoor Air Wind Direction",
    ];
    if !s.registration.out_vars.is_empty()
        || !s.registration.meters.meters.is_empty()
        || !s.registration.output_initialized
        || s.registration.get_output_input
        || s.registration
            .dictionary_variables
            .iter()
            .map(|r| r.name.as_str())
            .collect::<Vec<_>>()
            != factory_dictionary_names
        || s.registration
            .dictionary_variables
            .iter()
            .any(|r| !r.key_out_var_nums.is_empty())
        || s.registration.requested_variables.len() != s.report.output_variables.len()
        || s.registration.time_pointers.zone.target.is_some()
        || s.registration.time_pointers.system.target.is_some()
    {
        return Err(
            invalid("real P4 getter-produced dictionary/request/time-pointer domain").into(),
        );
    }
    let mut cmp = Compare::default();
    cmp.check(
        &snapshot(&s, &values, &extra)?,
        member(oracle, "initial")?,
        "initial",
    )?;
    let setup_expected = array(member(oracle, "setup")?)?;
    const SETUP: &[&str] = &[
        "SetupTimePointers.Zone",
        "SetupTimePointers.System",
        "GetReportVariableInput",
        "SetupOutputVariable.ZoneAverage",
        "SetupOutputVariable.SystemAverage",
        "SetupOutputVariable.SystemEnergy",
        "SetupOutputVariable.SystemInteger",
        "SetupOutputVariable.ZoneIntegerSum",
        "UpdateMeterReporting",
    ];
    assert_eq!(setup_expected.len(), SETUP.len());
    for (i, (label, event)) in SETUP.iter().zip(setup_expected).enumerate() {
        assert_eq!(member(event, "operation")?, *label);
        cmp.check(
            &snapshot(&s, &values, &extra)?,
            member(event, "entry")?,
            &format!("setup[{i}].entry"),
        )?;
        setup(&mut s, &values, &mut extra, definition, i)?;
        cmp.check(
            &snapshot(&s, &values, &extra)?,
            member(event, "exit")?,
            &format!("setup[{i}].exit"),
        )?;
    }
    assert_eq!(s.registration.out_vars.len(), 35);
    if s.registration.meters.meters.is_empty()
        || s.registration.meter_values.len() != s.registration.meters.meters.len()
    {
        return Err(invalid("P4 actual meter storage producer").into());
    }
    let steps = array(member(input, "steps")?)?;
    let outputs = array(member(oracle, "steps")?)?;
    assert_eq!(steps.len(), 38);
    assert_eq!(steps.len(), outputs.len());
    for (i, (step, event)) in steps.iter().zip(outputs).enumerate() {
        assert_eq!(member(step, "name")?, member(event, "name")?);
        assert_eq!(member(step, "operation")?, member(event, "operation")?);
        assert_eq!(member(step, "inputs")?, member(event, "named_inputs")?);
        apply_inputs(&mut s, &mut values, member(step, "inputs")?)?;
        cmp.check(
            &snapshot(&s, &values, &extra)?,
            member(event, "entry")?,
            &format!("step[{i}].entry"),
        )?;
        update(
            &mut s,
            &values,
            &mut extra,
            text(member(step, "operation")?)?,
        )?;
        cmp.check(
            &snapshot(&s, &values, &extra)?,
            member(event, "exit")?,
            &format!("step[{i}].exit"),
        )?;
    }
    assert_eq!(member(oracle, "calls")?, 38);
    println!(
        "P4 original-core full-operation cohort: 38 calls / 35 live OutVars / {} exact rawbinary64 observations",
        cmp.bits
    );
    Ok(38)
}
impl OrdinaryEquipment {
    pub(crate) fn run_native_gap_output_processor_cohort(fixture: &Value) -> Result<usize> {
        run(fixture)
    }
}
