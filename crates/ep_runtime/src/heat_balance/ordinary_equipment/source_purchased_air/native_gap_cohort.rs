//! Genuine Source PurchasedAir unit caller. Named inputs only; no expected seeding.
use super::super::OrdinaryEquipmentInput;
use super::*;
use crate::heat_balance as hb;
use hb::error_backend::{SourceErrorState, SourceErrorTracking};
use hb::ordinary_air_heat_balance::environment::SourceZoneFanHeatBalanceArrays;
use hb::ordinary_air_heat_balance::predictor_corrector::SourceZoneAirPredictorCorrector;
use hb::ordinary_air_heat_balance::{SourceAirHeatBalance, SourceZoneHeatBalanceData};
use hb::ordinary_hvac_manager::{SourceHvacManager, SourceHvacOwners};
use hb::predefined_tables::PredefinedRegistry;
use hb::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use hb::selected_heat_balance_input::zone::{SelectedZoneInputOwners, ZoneGetterOwners};
use hb::selected_heat_balance_input::{SelectedHeatBalanceInput, SelectedHeatBalanceInputState};
use hb::selected_heat_balance_space_controls::SelectedHeatBalanceSpaceControls;
use hb::selected_input_processor::SelectedInputProcessor;
use hb::selected_project_data::{
    ProjectDataOwners, ProjectInputObject, ProjectInputValue, SelectedProjectData,
    SelectedProjectDataInput,
};
use hb::selected_schedule_manager::{
    SelectedScheduleInput, SourceConstantScheduleInput, SourceScheduleIndex,
    SourceScheduleTypeInput,
};
use hb::selected_simulation_startup::SelectedSimulationStartup;
use hb::source_output_prescan::{
    SourceOutputPreScan, SourcePreScanSummary, SourcePreScanVariable, SourceReportingPreScanInput,
};
use hb::source_output_registration::{
    SourceIntegerOutputOwners, SourceOutputDictionaryStreams, SourceOutputRegistration,
    SourceRealOutputOwners,
};
use serde_json::{Map, Value, json};
use std::{collections::BTreeMap, io};
type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;
mod input;
fn invalid(s: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, s)
}
fn object(v: &Value) -> Result<&Map<String, Value>> {
    Ok(v.as_object().ok_or_else(|| invalid("input object"))?)
}
fn array(v: &Value) -> Result<&Vec<Value>> {
    Ok(v.as_array().ok_or_else(|| invalid("input array"))?)
}
fn member<'a>(v: &'a Value, k: &str) -> Result<&'a Value> {
    Ok(object(v)?.get(k).ok_or_else(|| invalid(k))?)
}
fn text(v: &Value) -> Result<&str> {
    Ok(v.as_str().ok_or_else(|| invalid("input string"))?)
}
fn int(v: &Value) -> Result<i32> {
    Ok(i32::try_from(
        v.as_i64().ok_or_else(|| invalid("input integer"))?,
    )?)
}
fn count(v: &Value) -> Result<usize> {
    Ok(usize::try_from(
        v.as_u64().ok_or_else(|| invalid("input count"))?,
    )?)
}
fn boolean(v: &Value) -> Result<bool> {
    Ok(v.as_bool().ok_or_else(|| invalid("input bool"))?)
}
fn index(n: i32) -> Result<usize> {
    Ok(usize::try_from(
        n.checked_sub(1)
            .ok_or_else(|| invalid("positive Source ordinal"))?,
    )?)
}
fn real(v: &Value) -> Result<f64> {
    let bits = text(member(v, "value_bits")?)?;
    if bits.len() != 16
        || !bits
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        return Err(invalid("raw binary64 input").into());
    }
    let value = f64::from_bits(u64::from_str_radix(bits, 16)?);
    if !value.is_finite() {
        return Err(invalid("finite named input").into());
    }
    Ok(value)
}
fn number(v: f64) -> Value {
    assert!(v.is_finite());
    json!({"value_bits":format!("{:016x}",v.to_bits())})
}
fn raw_value(v: &Value) -> Result<hb::selected_input_processor::SourceInputStatisticsValue> {
    use hb::selected_input_processor::SourceInputStatisticsValue as R;
    Ok(match v {
        Value::Null => R::Null,
        Value::Bool(b) => R::Bool(*b),
        Value::Number(n) => R::Number(n.to_string()),
        Value::String(s) => R::String(s.clone()),
        Value::Array(a) => R::Array(a.iter().map(raw_value).collect::<Result<_>>()?),
        Value::Object(o) => R::Object(
            o.iter()
                .map(|(k, v)| Ok((k.clone(), raw_value(v)?)))
                .collect::<Result<_>>()?,
        ),
    })
}
fn make_input(definitions: &Value, factory: &Value) -> Result<SelectedInputProcessor> {
    use hb::selected_input_processor::{
        SourceInputFamily, SourceInputStatisticsInput, SourceObjectDefinition, SourceObjectShape,
    };
    let definitions = array(definitions)?
        .iter()
        .map(|r| {
            let r = array(r)?;
            if r.len() != 6 {
                return Err(invalid("six definition operands").into());
            }
            Ok(SourceObjectDefinition {
                object_type: text(&r[0])?.into(),
                alpha_fields: count(&r[1])?,
                numeric_fields: count(&r[2])?,
                alpha_extensions: count(&r[3])?,
                numeric_extensions: count(&r[4])?,
                extension: if r[5].is_null() {
                    None
                } else {
                    Some(text(&r[5])?.into())
                },
            })
        })
        .collect::<Result<Vec<_>>>()?;
    let mut raw = SourceInputStatisticsInput::default();
    let mut families = BTreeMap::new();
    for (f, objects) in object(member(factory, "raw")?)? {
        let mut shapes = Vec::new();
        let mut retained = BTreeMap::new();
        for (n, fields) in object(objects)? {
            let fields = object(fields)?;
            shapes.push(SourceObjectShape {
                object_name: n.clone(),
                field_sizes: fields
                    .iter()
                    .map(|(k, v)| {
                        (
                            k.clone(),
                            match v {
                                Value::Null => 0,
                                Value::Array(a) => a.len(),
                                Value::Object(o) => o.len(),
                                _ => 1,
                            },
                        )
                    })
                    .collect(),
            });
            retained.insert(
                n.clone(),
                fields
                    .iter()
                    .map(|(k, v)| Ok((k.clone(), raw_value(v)?)))
                    .collect::<Result<_>>()?,
            );
        }
        families.insert(f.clone(), SourceInputFamily::Objects(shapes));
        raw.objects.insert(f.clone(), retained);
        raw.object_order.insert(
            f.clone(),
            array(member(member(factory, "object_order")?, f)?)?
                .iter()
                .map(|v| Ok(text(v)?.to_owned()))
                .collect::<Result<_>>()?,
        );
    }
    let mut input = SelectedInputProcessor::from_admitted_input(definitions, families)?;
    input.retain_source_input_statistics(raw)?;
    Ok(input)
}
fn raw_family<'a>(factory: &'a Value, family: &str) -> Result<Vec<(&'a str, &'a Value)>> {
    let raw = object(member(factory, "raw")?)?;
    let Some(objects) = raw.get(family) else {
        return Ok(Vec::new());
    };
    let objects = object(objects)?;
    array(member(member(factory, "object_order")?, family)?)?
        .iter()
        .map(|n| {
            let n = text(n)?;
            Ok((
                n,
                objects
                    .get(n)
                    .ok_or_else(|| invalid("ordered raw identity"))?,
            ))
        })
        .collect()
}
fn project_objects(
    factory: &Value,
    families: &[&str],
) -> Result<BTreeMap<String, Vec<ProjectInputObject>>> {
    let mut out = BTreeMap::new();
    for &family in families {
        let mut rows = Vec::new();
        for (name, raw) in raw_family(factory, family)? {
            let fields = object(raw)?;
            let idf_max_fields = fields.get("idf_max_fields").map(count).transpose()?;
            if idf_max_fields.is_none() {
                return Err(invalid("genuine physical IDF field count").into());
            }
            let fields = fields
                .iter()
                .filter(|(k, _)| {
                    !matches!(
                        k.as_str(),
                        "idf_order" | "idf_max_fields" | "idf_max_extensible_fields"
                    )
                })
                .map(|(k, v)| {
                    Ok((
                        k.clone(),
                        match v {
                            Value::String(s) => ProjectInputValue::String(s.clone()),
                            Value::Number(n) => {
                                let n = n.as_f64().ok_or_else(|| invalid("JSON binary64"))?;
                                if !n.is_finite() {
                                    return Err(invalid("finite raw number").into());
                                }
                                ProjectInputValue::Number(n)
                            }
                            _ => return Err(invalid("scalar getter input").into()),
                        },
                    ))
                })
                .collect::<Result<_>>()?;
            rows.push(ProjectInputObject {
                object_name: name.into(),
                fields,
                idf_max_fields,
            });
        }
        out.insert(family.into(), rows);
    }
    Ok(out)
}
fn alpha(raw: &Value, key: &str) -> Result<Option<String>> {
    Ok(match object(raw)?.get(key) {
        None | Some(Value::Null) => None,
        Some(Value::String(s)) if s.is_empty() => None,
        Some(Value::String(s)) => Some(s.to_ascii_uppercase()),
        _ => return Err(invalid("raw alpha input").into()),
    })
}
fn numeric(raw: &Value, key: &str) -> Result<Option<f64>> {
    Ok(match object(raw)?.get(key) {
        None | Some(Value::Null) => None,
        Some(Value::String(s)) if s.is_empty() => None,
        Some(Value::Number(n)) => Some(
            n.as_f64()
                .filter(|v| v.is_finite())
                .ok_or_else(|| invalid("finite raw number"))?,
        ),
        _ => return Err(invalid("raw numeric input").into()),
    })
}
fn schedule_inputs(factory: &Value) -> Result<SelectedScheduleInput> {
    let mut out = SelectedScheduleInput::default();
    for (name, raw) in raw_family(factory, "ScheduleTypeLimits")? {
        // All actual four retained files omit unit_type. Require that observed
        // input shape instead of silently substituting for a future entered unit.
        if alpha(raw, "unit_type")?.is_some() {
            return Err(invalid("entered Schedule unit transport not selected").into());
        }
        out.types.push(SourceScheduleTypeInput {
            object_name: name.into(),
            name: name.to_ascii_uppercase(),
            lower: numeric(raw, "lower_limit_value")?,
            upper: numeric(raw, "upper_limit_value")?,
            numeric_type: alpha(raw, "numeric_type")?,
            unit: ep_model::ScheduleUnitType::Invalid,
        });
    }
    for (name, raw) in raw_family(factory, "Schedule:Constant")? {
        out.constants.push(SourceConstantScheduleInput {
            object_name: name.into(),
            name: name.to_ascii_uppercase(),
            type_name: alpha(raw, "schedule_type_limits_name")?,
            value: numeric(raw, "hourly_value")?.unwrap_or(0.0),
        });
    }
    Ok(out)
}
fn report_inputs(factory: &Value) -> Result<(ep_model::ReportInputs, SourceReportingPreScanInput)> {
    let mut out = ep_model::ReportInputs::default();
    let mut prescan = SourceReportingPreScanInput::default();
    for (name, raw) in raw_family(factory, "Output:Variable")? {
        let mut a = [String::new(), String::new(), String::new(), String::new()];
        let mut blank = [false; 4];
        let mut defaults = [false; 4];
        for (i, (key, default)) in [
            ("key_value", Some("*")),
            ("variable_name", None),
            ("reporting_frequency", Some("Hourly")),
            ("schedule_name", None),
        ]
        .into_iter()
        .enumerate()
        {
            let supplied = match object(raw)?.get(key) {
                None | Some(Value::Null) => "",
                Some(Value::String(s)) => s,
                _ => return Err(invalid("OutputVariable alpha input").into()),
            };
            blank[i] = supplied.is_empty();
            let actual = if supplied.is_empty() {
                if let Some(default) = default {
                    defaults[i] = true;
                    default
                } else {
                    ""
                }
            } else {
                supplied
            };
            a[i] = if i == 0 {
                actual.into()
            } else {
                actual.to_ascii_uppercase()
            };
        }
        prescan.variables.push(SourcePreScanVariable {
            object_name: name.into(),
            key: a[0].clone(),
            variable_name: object(raw)?
                .get("variable_name")
                .and_then(Value::as_str)
                .ok_or_else(|| invalid("OutputVariable name"))?
                .into(),
        });
        let [key, variable_name, reporting_frequency, schedule_name] = a;
        out.output_variables
            .push(ep_model::SourceOutputVariableInput {
                object_name: name.into(),
                key,
                variable_name,
                reporting_frequency,
                schedule_name,
                alpha_blank: blank,
                default_applied: defaults,
                num_alphas: u8::try_from(count(member(raw, "idf_max_fields")?)?)?,
                is_epjson: false,
            });
    }
    for (name, raw) in raw_family(factory, "Output:Table:SummaryReports")? {
        prescan.summaries.push(SourcePreScanSummary {
            object_name: name.into(),
            report_names: array(member(raw, "reports")?)?
                .iter()
                .map(|r| Ok(text(member(r, "report_name")?)?.to_owned()))
                .collect::<Result<_>>()?,
        });
    }
    // The exact getter count/identity for report scan families is kept even
    // though the P1 full calls do not request report generation.
    for (name, raw) in raw_family(factory, "Output:VariableDictionary")? {
        let mut alphas = [String::new(), String::new(), String::new()];
        let mut blank = [true; 3];
        let mut defaults = [false; 3];
        for (i, key) in ["key_field", "sort_option"].into_iter().enumerate() {
            let value = alpha(raw, key)?;
            blank[i] = value.is_none();
            alphas[i] = value.unwrap_or_default();
        }
        if alphas[0].is_empty() {
            alphas[0] = "REGULAR".into();
            defaults[0] = true;
        }
        out.objects.push(ep_model::ReportInputObject {
            object_name: name.into(),
            object_type: ep_model::ReportInputObjectType::VariableDictionary,
            alphas,
            alpha_blank: blank,
            default_applied: defaults,
            num_alphas: u8::try_from(count(member(raw, "idf_max_fields")?)?)?,
            is_epjson: false,
        });
    }
    // Production reporting_prescan::resolve walks raw object maps lexically.
    // Getter rows above retain their genuine IDF order; these raw pre-scan
    // associations are distinct and must match SelectedInputProcessor shapes.
    prescan.variables.sort_by(|a, b| a.object_name.cmp(&b.object_name));
    prescan.summaries.sort_by(|a, b| a.object_name.cmp(&b.object_name));
    Ok((out, prescan))
}
fn identity_model(
    factory: &Value,
    control: &SelectedHeatBalanceInputState,
    steps: i32,
) -> Result<ep_model::TypedModel> {
    use ep_model::*;
    let mut model = TypedModel::default();
    model.timestep.number_of_timesteps_per_hour = u32::try_from(steps)?;
    for (name, raw) in raw_family(factory, "Zone")? {
        let id = ZoneId(u32::try_from(model.zones.len())?);
        let space = SpaceId(u32::try_from(model.spaces.len())?);
        let auto = |key: &str| -> Result<AutoOrNumber> {
            Ok(match object(raw)?.get(key) {
                None => AutoOrNumber::AutoCalculate,
                Some(Value::String(s)) if s.eq_ignore_ascii_case("Autocalculate") => {
                    AutoOrNumber::AutoCalculate
                }
                Some(Value::Number(n)) => {
                    AutoOrNumber::Value(n.as_f64().ok_or_else(|| invalid("Zone binary64"))?)
                }
                _ => return Err(invalid("Zone auto input").into()),
            })
        };
        let inside = control.inside_convection_algorithm;
        let outside = control.outside_convection_algorithm;
        model.zones.push(Zone {
            id,
            name: NormalizedName::new(name),
            direction_of_relative_north_deg: numeric(raw, "direction_of_relative_north")?
                .unwrap_or(0.0),
            origin: Point3 {
                x_m: numeric(raw, "x_origin")?.unwrap_or(0.0),
                y_m: numeric(raw, "y_origin")?.unwrap_or(0.0),
                z_m: numeric(raw, "z_origin")?.unwrap_or(0.0),
            },
            zone_type: 1,
            multiplier: numeric(raw, "multiplier")?.unwrap_or(1.0) as u32,
            list_multiplier: 1,
            list_group: None,
            ceiling_height: auto("ceiling_height")?,
            volume: auto("volume")?,
            floor_area: auto("floor_area")?,
            inside_convection_algorithm: ZoneConvectionAlgorithm::Inherited(inside),
            outside_convection_algorithm: ZoneConvectionAlgorithm::Inherited(outside),
            is_part_of_total_floor_area: true,
            is_nominal_controlled: false,
            linked_outdoor_air_node: None,
            spaces: vec![space],
        });
        model.spaces.push(Space {
            id: space,
            name: NormalizedName::new(name),
            zone: id,
            ceiling_height: AutoOrNumber::AutoCalculate,
            volume: AutoOrNumber::AutoCalculate,
            floor_area: AutoOrNumber::AutoCalculate,
            space_type: NormalizedName::new("GENERAL"),
            space_type_id: SpaceTypeId(0),
            tags: Vec::new(),
            origin: SpaceOrigin::AutoZoneDefault,
        });
    }
    Ok(model)
}
#[derive(Default)]
struct Files {
    audit: Vec<u8>,
    mtd_open: bool,
}
impl SourceOutputFileHost for Files {
    fn ensure_mtd_open(&mut self) -> io::Result<()> {
        self.mtd_open = true;
        Ok(())
    }
}
impl hb::source_initialization::ScheduleAuditFileHost for Files {
    fn audit_path(&self) -> &std::path::Path {
        std::path::Path::new("native-gap-batch-memory.audit")
    }
    fn ensure_audit_open(&mut self) -> io::Result<&mut dyn std::io::Write> {
        Ok(&mut self.audit)
    }
}
fn error_state() -> SourceErrorState {
    SourceErrorState {
        output_error_header: true,
        version_string: String::new(),
        idd_version_string: String::new(),
        doing_input_processing: false,
        print_console_output: false,
        kickoff_simulation: false,
        developer_flag: false,
        warmup_flag: false,
        doing_sizing: false,
        tracking: SourceErrorTracking {
            ask_for_plant_check_on_abort: false,
            exit_during_simulations: false,
            match_counts: [0; 20],
            abort_processing: false,
            ask_for_connections_report: false,
            ask_for_surfaces_report: false,
            total_multiplied_windows: 0,
            total_coincident_vertices: 0,
            total_degenerate_surfaces: 0,
            total_receiving_non_convex_surfaces: 0,
            total_casting_non_convex_surfaces: 0,
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

struct Factory {
    equipment: OrdinaryEquipment,
    input: SelectedInputProcessor,
    shortcut: SourceInputProcessorShortCut,
    globals: SelectedHeatBalanceGlobals,
    zones: SelectedZoneInputOwners,
    schedules: SelectedScheduleManager,
    sizing: hb::selected_sizing::SelectedSizingManager,
    startup: SelectedSimulationStartup,
    hvac_manager: SourceHvacManager,
    air: SourceAirHeatBalance,
    controller: SourceZoneAirPredictorCorrector,
    fan: SourceZoneFanHeatBalanceArrays,
    weather: crate::weather::day::WeatherDayState,
    project: SelectedProjectData,
    control: SelectedHeatBalanceInputState,
    space_controls: SelectedHeatBalanceSpaceControls,
    report: ep_model::ReportInputs,
    prescan: SourceOutputPreScan,
    registration: SourceOutputRegistration,
    registry: PredefinedRegistry,
    files: Files,
    eio: Vec<u8>,
    eso: Vec<u8>,
    rf: Vec<String>,
    error_state: SourceErrorState,
    error: Vec<u8>,
    console: Vec<u8>,
    psych: crate::psychrometrics::SourcePsychrometricAccess,
    ancillary: hb::ctf_initialization_context::CtfInitializationReports,
    surface: hb::ordinary_surface_initialization::OrdinarySurfaceInitialization,
    radiation: hb::radiation::ordinary::OrdinarySolarRadiation,
    history: hb::ctf_thermal_history::CtfThermalHistoryState,
    gain_modules: hb::selected_gain_modules::SourceGainModules,
    contaminants: hb::ordinary_heat_balance_initialization::SourceContaminantArrays,
    tabular: hb::ordinary_tabular_reports::SourceTabularReports,
    comfort: hb::ordinary_thermal_comfort::SourceThermalComfort,
    comfort_summary: hb::ordinary_thermal_comfort::SourceComfortPredefinedSummary,
    final_surface: hb::ordinary_surface_source_averages::SourceFinalSurfaceHeatBalance,
    exterior: hb::ordinary_exterior_energy_use::SourceExteriorEnergyUse,
    zone: i32,
    list: i32,
    pointer: i32,
    sensible: f64,
    moisture: f64,
}

impl Factory {
    fn new(definitions: &Value, definition: &Value) -> Result<Self> {
        let input = make_input(definitions, definition)?;
        let mut shortcut = SourceInputProcessorShortCut::default();
        shortcut.allocate_from_input(&input)?;
        let control = SelectedHeatBalanceInputState::default();
        let (report, raw_prescan) = report_inputs(definition)?;
        let inventory = object(member(definition, "raw")?)?
            .iter()
            .map(|(f, v)| Ok((f.as_str(), object(v)?.len())))
            .collect::<Result<Vec<_>>>()?;
        let radiation_input =
            hb::radiation::ordinary::OrdinaryRadiationInput::from_admitted_inventory(
                inventory.clone(),
                false,
            )?;
        let mut input = input;
        let mut prescan = SourceOutputPreScan::default();
        let mut state = error_state();
        let mut error = Vec::new();
        let mut console = Vec::new();
        let mut project = SelectedProjectData::default();
        let mut weather = crate::weather::day::WeatherDayState::default();
        let mut globals = SelectedHeatBalanceGlobals::default();
        let mut startup = SelectedSimulationStartup::default();
        // Genuine init_state -> OpenOutputFiles precedes GetProjectData.
        // The cold version string is actual owner input, not expected output.
        let output_header = format!("Program Version,{}\n", state.version_string);
        let mut eio = output_header.as_bytes().to_vec();
        let eso = output_header.into_bytes();
        let mut fluid = hb::selected_fluid_properties::SelectedFluidProperties::default();
        fluid.init_constant_state()?;
        let mut schedules = SelectedScheduleManager::default();
        schedules.init_constant_state(0)?;
        let mut scan = hb::report_scan::ReportScanState::default();
        {
            let mut backend = SourceErrorBackend {
                state: &mut state,
                error_stream: Some(&mut error),
                console: &mut console,
                out_unit1: None,
                out_unit2: None,
                f_message: None,
                message_callback: None,
                error_callback: None,
                sqlite: None,
            };
            hb::source_output_prescan::post_input_processing(
                &input,
                &mut shortcut,
                &raw_prescan,
                &mut prescan,
                false,
                &mut backend,
            )?;
            let raw = SelectedProjectDataInput {
                source_is_epjson: false,
                objects: project_objects(
                    definition,
                    &[
                        "Version",
                        "HeatBalanceAlgorithm",
                        "Timestep",
                        "SimulationControl",
                    ],
                )?,
                diagnostics: Vec::new(),
            };
            let mut current_module = String::new();
            project.get_project_data(
                &raw,
                &mut ProjectDataOwners {
                    input: &mut input,
                    shortcut: &mut shortcut,
                    globals: &mut globals,
                    weather: &mut weather,
                    hvac: &mut startup.hvac_globals,
                    heat_balance_current_module_object: &mut current_module,
                },
                &mut backend,
                &mut eio,
            )?;
            startup.check_if_any_ems(
                &mut input,
                &mut shortcut,
                &mut globals,
                &report,
                &mut scan,
                false,
                false,
                0,
                &mut backend,
            )?;
        }
        fluid.get_fluid_properties_data(&mut input)?;
        let steps = usize::try_from(weather.global.time_steps_in_hour)?;
        let mut files = Files::default();
        schedules.process_schedule_input(
            &mut input,
            &schedule_inputs(definition)?,
            steps,
            &mut files,
        )?;
        let model = identity_model(definition, &control, weather.global.time_steps_in_hour)?;
        let mut registration = SourceOutputRegistration::default();
        let mut zones = SelectedZoneInputOwners::default();
        let raw = SelectedHeatBalanceInput {
            source_is_epjson: false,
            surface: Default::default(),
            objects: project_objects(definition, &["Zone"])?,
        };
        zones.get_zone_data(
            &raw,
            &control,
            &model,
            &mut ZoneGetterOwners {
                input: &mut input,
                shortcut: &mut shortcut,
                globals: &mut globals,
                output: &mut registration,
                prescan: &prescan,
                report_input: &report,
                minimum_report_frequency: "",
                minutes_in_time_step: weather.global.minutes_in_time_step,
                host: &mut files,
                eio: &mut eio,
            },
        )?;
        let admitted = OrdinaryEquipmentInput::from_source_family_inventory(inventory, steps)?;
        let mut equipment = OrdinaryEquipment::new_for_source(&model, admitted)?;
        let mut ancillary = hb::ctf_initialization_context::CtfInitializationReports::default();
        let mut manager = SourceHvacManager::default();
        let mut sizing = hb::selected_sizing::SelectedSizingManager::default();
        equipment.get_zone_equipment_source(
            &mut input,
            admitted,
            false,
            false,
            &globals,
            &mut ancillary.selected_internal_heat_gains,
            super::super::SourceEquipmentInputOwners {
                zones: zones
                    .zones
                    .as_deref_mut()
                    .ok_or_else(|| invalid("getter Zone rows"))?,
                schedules: &mut schedules,
                sizing: &mut sizing,
                branch: Some(&mut manager.ancillary.branch_node_connections),
                source_is_epjson: false,
            },
        )?;
        let mut selected = None;
        for (z, c) in equipment
            .zone_config
            .as_deref()
            .ok_or_else(|| invalid("actual Config allocation"))?
            .iter()
            .enumerate()
        {
            if !c.is_controlled {
                continue;
            }
            let l = c.equipment_list_index;
            let row = equipment
                .equipment_lists
                .as_deref()
                .and_then(|r| l.checked_sub(1).and_then(|i| r.get(i)))
                .ok_or_else(|| invalid("actual List index"))?;
            for p in 0..row.number_of_equipment_types {
                if row.equipment_types.as_deref().and_then(|v| v.get(p)) == Some(&12) {
                    if selected.is_some() {
                        return Err(invalid("unique actual PurchasedAir component").into());
                    }
                    selected = Some((
                        i32::try_from(z + 1)?,
                        i32::try_from(l)?,
                        i32::try_from(p + 1)?,
                    ));
                }
            }
        }
        let (zone, list, pointer) = selected.ok_or_else(|| invalid("real PurchasedAir binding"))?;
        let space_controls = SelectedHeatBalanceSpaceControls::default();
        let mut controller = SourceZoneAirPredictorCorrector::default();
        controller
            .demand
            .allocate_source_demands(&globals, &space_controls)?;
        let n = usize::try_from(globals.num_of_zones)?;
        let mut air = SourceAirHeatBalance::default();
        air.zones = Some(vec![SourceZoneHeatBalanceData::default(); n]);
        let mut fan = SourceZoneFanHeatBalanceArrays::default();
        fan.thermostat.zone_tstat_setpoints = Some((0..n).map(|_| Default::default()).collect());
        fan.thermostat.temp_control_type = Some(vec![0; n]);
        let mut out = Self {
            equipment,
            input,
            shortcut,
            globals,
            zones,
            schedules,
            sizing,
            startup,
            hvac_manager: manager,
            air,
            controller,
            fan,
            weather,
            project,
            control,
            space_controls,
            report,
            prescan,
            registration,
            registry: Default::default(),
            files,
            eio,
            eso,
            rf: Vec::new(),
            error_state: state,
            error,
            console,
            psych: Default::default(),
            ancillary,
            surface: Default::default(),
            radiation: hb::radiation::ordinary::OrdinarySolarRadiation::for_surface_input(
                radiation_input,
            ),
            history: hb::ctf_thermal_history::CtfThermalHistoryState::new_source_unallocated(
                Vec::new(),
            ),
            gain_modules: Default::default(),
            contaminants: Default::default(),
            tabular: Default::default(),
            comfort: Default::default(),
            comfort_summary: Default::default(),
            final_surface: Default::default(),
            exterior: Default::default(),
            zone,
            list,
            pointer,
            sensible: 0.0,
            moisture: 0.0,
        };
        out.initialize_prefix(true, true)?;
        out.registry
            .set_predefined_tables(out.globals.do_zone_sizing, out.globals.do_system_sizing)?;
        out.sizing.current_zone_equipment_number = zone;
        Ok(out)
    }
    fn initialize_prefix(&mut self, first: bool, availability_input: bool) -> Result {
        let mut backend = SourceErrorBackend {
            state: &mut self.error_state,
            error_stream: Some(&mut self.error),
            console: &mut self.console,
            out_unit1: None,
            out_unit2: None,
            f_message: None,
            message_callback: None,
            error_callback: None,
            sqlite: None,
        };
        let source_is_epjson = false;
        let mut owners = SourceHvacOwners {
            runtime_language: &self.startup.runtime_language,
            air: &mut self.air,
            controller: &mut self.controller,
            zones: &mut self.zones,
            sizing: &mut self.sizing,
            equipment: &mut self.equipment,
            geometry: &self.ancillary.surface_geometry_input,
            surface_arrays: &mut self.surface.arrays,
            fan: &mut self.fan,
            gains: &mut self.ancillary.selected_internal_heat_gains,
            control: &mut self.control,
            space_controls: &self.space_controls,
            globals: &mut self.globals,
            clock: &mut self.weather.global,
            environment: &mut self.weather.environment,
            hvac: &mut self.startup.hvac_globals,
            input: &mut self.input,
            shortcut: &mut self.shortcut,
            source_is_epjson: &source_is_epjson,
            faults: &self.ancillary.source_faults_manager,
            psychrometrics: &mut self.psych,
            backend: &mut backend,
            plant_loops: &mut self.startup.plant_loops,
            project: &mut self.project,
            contaminants: &mut self.contaminants,
            gain_modules: &mut self.gain_modules,
            facility: None,
            schedules: &mut self.schedules,
            tabular: &mut self.tabular,
            radiation: &mut self.radiation,
            history: &mut self.history,
            physical_surfaces: &mut [],
            comfort: &mut self.comfort,
            comfort_summary: &mut self.comfort_summary,
            predefined_registry: &mut self.registry,
            predefined_table: &mut self.ancillary.table,
            weather: &self.weather.weather,
            inside_surface_iterations: &self.surface.inside_surface_iterations,
            surface_heat_emission: &self.surface.sum_surface_heat_emission,
            surface_averages: &self.final_surface,
            exterior: &self.exterior,
        };
        if availability_input {
            self.hvac_manager
                .manage_source_system_availability(&mut owners)?;
        }
        self.hvac_manager
            .initialize_source_zone_equipment_cohort_bridge(first, &mut owners)?;
        Ok(())
    }
    fn phase_inputs(&mut self, v: &Value) -> Result {
        let o = object(v)?;
        macro_rules! put {
            ($name:literal,$slot:expr,$read:ident) => {
                if let Some(v) = o.get($name) {
                    $slot = $read(v)?;
                }
            };
        }
        put!(
            "begin_environment",
            self.weather.global.begin_envrn_flag,
            boolean
        );
        put!("std_rho", self.weather.environment.std_rho_air, real);
        put!(
            "pressure",
            self.weather.environment.current_weather.out_baro_press,
            real
        );
        put!(
            "system_hours",
            self.startup.hvac_globals.time_step_sys,
            real
        );
        put!(
            "system_seconds",
            self.startup.hvac_globals.time_step_sys_seconds,
            real
        );
        put!("hour", self.weather.global.hour_of_day, int);
        put!("timestep", self.weather.global.time_step, int);
        put!(
            "day_of_year",
            self.weather.environment.day_of_year_schedule,
            int
        );
        put!("day_of_week", self.weather.environment.day_of_week, int);
        put!("dst", self.weather.environment.dst_indicator, int);
        put!("warmup", self.weather.global.warmup_flag, boolean);
        self.error_state.warmup_flag = self.weather.global.warmup_flag;
        Ok(())
    }
    fn caller_inputs(&mut self, v: &Value) -> Result {
        let o = object(v)?;
        let z = index(self.zone)?;
        let d = self
            .controller
            .demand
            .zones
            .as_mut()
            .and_then(|r| r.get_mut(z))
            .ok_or_else(|| invalid("actual demand"))?;
        if let Some(v) = o.get("heat_demand") {
            d.remaining_output_to_heat_sp = real(v)?;
        }
        if let Some(v) = o.get("cool_demand") {
            d.remaining_output_to_cool_sp = real(v)?;
        }
        let t = self
            .fan
            .thermostat
            .zone_tstat_setpoints
            .as_mut()
            .and_then(|r| r.get_mut(z))
            .ok_or_else(|| invalid("actual thermostat row"))?;
        if let Some(v) = o.get("thermostat_low") {
            t.setpt_lo = real(v)?;
        }
        if let Some(v) = o.get("thermostat_high") {
            t.setpt_hi = real(v)?;
        }
        if let Some(v) = o.get("control_type") {
            let n = int(v)?;
            if !(0..5).contains(&n) {
                return Err(invalid("actual control enum domain").into());
            }
            *self
                .fan
                .thermostat
                .temp_control_type
                .as_mut()
                .and_then(|r| r.get_mut(z))
                .ok_or_else(|| invalid("actual control type"))? = n;
        }
        let config = self
            .equipment
            .zone_config
            .as_deref()
            .and_then(|r| r.get(z))
            .ok_or_else(|| invalid("actual Config"))?;
        let node = config.zone_node;
        let return_nodes = config
            .return_node_numbers
            .as_deref()
            .ok_or_else(|| invalid("actual ReturnNode array"))?;
        if config.return_nodes != 1 || return_nodes.len() != 1 {
            return Err(invalid("actual one-return input domain").into());
        }
        let return_node = return_nodes[0];
        let mut changed = false;
        {
            let row = self
                .equipment
                .nodes
                .physical
                .as_mut()
                .and_then(|r| r.get_mut(node.checked_sub(1)?))
                .ok_or_else(|| invalid("actual ZoneNode"))?;
            if let Some(v) = o.get("zone_temp") {
                row.temperature = real(v)?;
                changed = true;
            }
            if let Some(v) = o.get("zone_humidity") {
                row.humidity_ratio = real(v)?;
                changed = true;
                self.air
                    .zones
                    .as_mut()
                    .and_then(|r| r.get_mut(z))
                    .ok_or_else(|| invalid("actual Zone HB"))?
                    .air
                    .air_hum_rat = row.humidity_ratio;
            }
            if changed {
                row.enthalpy = crate::psychrometrics::energyplus_psy_h_fn_tdb_w(
                    row.temperature,
                    row.humidity_ratio,
                );
            }
        }
        changed = false;
        {
            let row = self
                .equipment
                .nodes
                .physical
                .as_mut()
                .and_then(|r| r.get_mut(return_node.checked_sub(1)?))
                .ok_or_else(|| invalid("actual ReturnNode"))?;
            if let Some(v) = o.get("return_temp") {
                row.temperature = real(v)?;
                changed = true;
            }
            if let Some(v) = o.get("return_humidity") {
                row.humidity_ratio = real(v)?;
                changed = true;
            }
            if changed {
                row.enthalpy = crate::psychrometrics::energyplus_psy_h_fn_tdb_w(
                    row.temperature,
                    row.humidity_ratio,
                );
            }
        }
        self.schedules
            .update_source_schedule_values(&self.weather.global, &self.weather.environment)?;
        Ok(())
    }
    fn call(&mut self, step: &Value) -> Result {
        let operation = text(member(step, "operation")?)?;
        let first = object(step)?
            .get("first_hvac_iteration")
            .map(boolean)
            .transpose()?
            .unwrap_or(false);
        let unit = *self
            .equipment
            .purchased_equipment_index(self.list, self.pointer)?;
        let mut backend = SourceErrorBackend {
            state: &mut self.error_state,
            error_stream: Some(&mut self.error),
            console: &mut self.console,
            out_unit1: None,
            out_unit2: None,
            f_message: None,
            message_callback: None,
            error_callback: None,
            sqlite: None,
        };
        let mut init = SourcePurchasedAirInitOwners {
            sizing: &mut self.sizing,
            fan: &self.fan,
            globals: &self.globals,
            clock: &self.weather.global,
            hvac: &self.startup.hvac_globals,
            air_loop: &self.hvac_manager.simulation.air_loop,
            source_is_epjson: &false,
        };
        let mut calc = SourcePurchasedAirCalcOwners {
            availability: &mut self.hvac_manager.ancillary.availability,
            controller: &self.controller,
            air: &self.air,
            fan: &self.fan,
            hvac: &self.startup.hvac_globals,
            environment: &mut self.weather.environment,
            control: &self.control,
            psychrometrics: &mut self.psych,
            backend: &mut backend,
        };
        let mut dictionary = SourceOutputDictionaryStreams {
            eso: Some(&mut self.eso),
            sqlite_present: false,
            results_framework_output_variables: &mut self.rf,
        };
        match operation {
            "simulate" => {
                let list = self
                    .equipment
                    .equipment_lists
                    .as_deref()
                    .and_then(|r| r.get(index(self.list).ok()?))
                    .ok_or_else(|| invalid("same List"))?;
                let name = list
                    .equipment_names
                    .as_deref()
                    .and_then(|r| r.get(index(self.pointer).ok()?))
                    .ok_or_else(|| invalid("same equipment name"))?
                    .clone();
                let mut output = SourcePurchasedAirReportingOutputContext {
                    registration: &mut self.registration,
                    input: &mut self.input,
                    prescan: &self.prescan,
                    reports: &self.report,
                    minimum_report_frequency: "",
                    minutes_in_time_step: self.weather.global.minutes_in_time_step,
                    host: &mut self.files,
                    dictionary_host: &mut dictionary,
                    eio: &mut self.eio,
                };
                self.equipment.simulate_source_purchased_air(
                    &name,
                    self.list,
                    self.pointer,
                    self.zone,
                    &mut self.shortcut,
                    false,
                    &mut self.schedules,
                    self.globals.num_of_zones,
                    &mut self.hvac_manager.ancillary.branch_node_connections,
                    self.globals.any_energy_management_system_in_model,
                    &mut output,
                    &mut init,
                    &mut calc,
                    &mut self.registry,
                    first,
                    &mut self.sensible,
                    &mut self.moisture,
                )?;
            }
            "init" => {
                let mut reporting = hb::selected_sizing::sizer_output::SourceSizerReportingOwners {
                    registry: &mut self.registry,
                    environment: calc.environment,
                    dictionary_host: &dictionary,
                    eio: &mut self.eio,
                };
                self.equipment.initialize_source_purchased_air(
                    unit,
                    self.zone,
                    &self.schedules,
                    &mut init,
                    &mut reporting,
                )?;
            }
            "calc" => self.equipment.calculate_source_purchased_air(
                unit,
                self.zone,
                &mut self.sensible,
                &mut self.moisture,
                &self.schedules,
                &mut calc,
            )?,
            "update" => self.equipment.update_source_purchased_air(unit, first)?,
            "report" => {
                self.equipment
                    .report_source_purchased_air(unit, &self.schedules, &mut calc)?
            }
            _ => return Err(invalid("actual operation name").into()),
        }
        Ok(())
    }
}

// Observers and actual Native expected checks are appended only after their
// complete readsets are closed. No placeholder PASS is exposed by this scaffold.

// BEGIN GENERATED PASSIVE OBSERVERS
fn pointer<T>(v: &T) -> Value {
    json!(format!("{:p}", std::ptr::from_ref(v)))
}
fn shape<T>(v: &Option<Vec<T>>) -> Value {
    json!({"allocated":v.is_some(),"size":v.as_ref().map_or(0,Vec::len)})
}
fn array1<T>(v: &Option<Vec<T>>) -> Value {
    let mut out = shape(v);
    out["lower"] = json!(1);
    out["upper"] = json!(v.as_ref().map_or(0, Vec::len));
    out
}
fn integers(v: &Option<Vec<i32>>) -> Value {
    let mut out = array1(v);
    out["items"] = json!(v.as_deref().unwrap_or(&[]));
    out
}
fn booleans(v: &Option<Vec<bool>>) -> Value {
    let mut out = array1(v);
    out["items"] = json!(v.as_deref().unwrap_or(&[]));
    out
}
fn strings(v: &Option<Vec<String>>) -> Value {
    let mut out = array1(v);
    out["items"] = json!(v.as_deref().unwrap_or(&[]));
    out
}
fn schedule(s: &SelectedScheduleManager, index: Option<SourceScheduleIndex>) -> Result<Value> {
    let Some(index) = index else {
        return Ok(Value::Null);
    };
    let row = s
        .schedules
        .get(index.0)
        .ok_or_else(|| invalid("actual Schedule pointer"))?;
    Ok(
        json!({"address":pointer(row),"Name":row.name,"Num":row.number.0,"currentVal":number(row.current_value),"EMSActuatedOn":row.ems_actuated_on,"EMSVal":number(row.ems_value),"isUsed":row.is_used,"same_actual_schedule_owner_ordinals_zero_based":[index.0]}),
    )
}
fn unit(s: &Factory, row: &SourceZonePurchasedAir) -> Result<Value> {
    let mut out = Map::new();
    out.insert("cObjectName".into(), json!(row.c_object_name));
    out.insert("Name".into(), json!(row.name));
    out.insert(
        "availSched".into(),
        schedule(&s.schedules, row.avail_sched)?,
    );
    out.insert(
        "ZoneSupplyAirNodeNum".into(),
        json!(row.zone_supply_air_node_num),
    );
    out.insert(
        "ZoneExhaustAirNodeNum".into(),
        json!(row.zone_exhaust_air_node_num),
    );
    out.insert(
        "PlenumExhaustAirNodeNum".into(),
        json!(row.plenum_exhaust_air_node_num),
    );
    out.insert("ReturnPlenumIndex".into(), json!(row.return_plenum_index));
    out.insert(
        "PurchAirArrayIndex".into(),
        json!(row.purch_air_array_index),
    );
    out.insert("ReturnPlenumName".into(), json!(row.return_plenum_name));
    out.insert(
        "ZoneRecircAirNodeNum".into(),
        json!(row.zone_recirc_air_node_num),
    );
    out.insert(
        "MaxHeatSuppAirTemp".into(),
        number(row.max_heat_supp_air_temp),
    );
    out.insert(
        "MinCoolSuppAirTemp".into(),
        number(row.min_cool_supp_air_temp),
    );
    out.insert(
        "MaxHeatSuppAirHumRat".into(),
        number(row.max_heat_supp_air_hum_rat),
    );
    out.insert(
        "MinCoolSuppAirHumRat".into(),
        number(row.min_cool_supp_air_hum_rat),
    );
    out.insert("HeatingLimit".into(), json!(row.heating_limit as i32));
    out.insert(
        "MaxHeatVolFlowRate".into(),
        number(row.max_heat_vol_flow_rate),
    );
    out.insert("MaxHeatSensCap".into(), number(row.max_heat_sens_cap));
    out.insert("CoolingLimit".into(), json!(row.cooling_limit as i32));
    out.insert(
        "MaxCoolVolFlowRate".into(),
        number(row.max_cool_vol_flow_rate),
    );
    out.insert("MaxCoolTotCap".into(), number(row.max_cool_tot_cap));
    out.insert(
        "heatAvailSched".into(),
        schedule(&s.schedules, row.heat_avail_sched)?,
    );
    out.insert(
        "coolAvailSched".into(),
        schedule(&s.schedules, row.cool_avail_sched)?,
    );
    out.insert(
        "DehumidCtrlType".into(),
        json!(row.dehumid_ctrl_type as i32),
    );
    out.insert("CoolSHR".into(), number(row.cool_shr));
    out.insert("HumidCtrlType".into(), json!(row.humid_ctrl_type as i32));
    out.insert("OARequirementsPtr".into(), json!(row.oa_requirements_ptr));
    out.insert("DCVType".into(), json!(row.dcv_type as i32));
    out.insert("EconomizerType".into(), json!(row.economizer_type as i32));
    out.insert("OutdoorAir".into(), json!(row.outdoor_air));
    out.insert("OutdoorAirNodeNum".into(), json!(row.outdoor_air_node_num));
    out.insert("HtRecType".into(), json!(row.ht_rec_type as i32));
    out.insert("HtRecSenEff".into(), number(row.ht_rec_sen_eff));
    out.insert("HtRecLatEff".into(), number(row.ht_rec_lat_eff));
    out.insert(
        "oaFlowFracSched".into(),
        schedule(&s.schedules, row.oa_flow_frac_sched)?,
    );
    out.insert(
        "MaxHeatMassFlowRate".into(),
        number(row.max_heat_mass_flow_rate),
    );
    out.insert(
        "MaxCoolMassFlowRate".into(),
        number(row.max_cool_mass_flow_rate),
    );
    out.insert("EMSOverrideMdotOn".into(), json!(row.ems_override_mdot_on));
    out.insert(
        "EMSValueMassFlowRate".into(),
        number(row.ems_value_mass_flow_rate),
    );
    out.insert(
        "EMSOverrideOAMdotOn".into(),
        json!(row.ems_override_oa_mdot_on),
    );
    out.insert(
        "EMSValueOAMassFlowRate".into(),
        number(row.ems_value_oa_mass_flow_rate),
    );
    out.insert(
        "EMSOverrideSupplyTempOn".into(),
        json!(row.ems_override_supply_temp_on),
    );
    out.insert(
        "EMSValueSupplyTemp".into(),
        number(row.ems_value_supply_temp),
    );
    out.insert(
        "EMSOverrideSupplyHumRatOn".into(),
        json!(row.ems_override_supply_hum_rat_on),
    );
    out.insert(
        "EMSValueSupplyHumRat".into(),
        number(row.ems_value_supply_hum_rat),
    );
    out.insert(
        "MinOAMassFlowRate".into(),
        number(row.min_oa_mass_flow_rate),
    );
    out.insert(
        "OutdoorAirMassFlowRate".into(),
        number(row.outdoor_air_mass_flow_rate),
    );
    out.insert(
        "OutdoorAirVolFlowRateStdRho".into(),
        number(row.outdoor_air_vol_flow_rate_std_rho),
    );
    out.insert(
        "SupplyAirMassFlowRate".into(),
        number(row.supply_air_mass_flow_rate),
    );
    out.insert(
        "SupplyAirVolFlowRateStdRho".into(),
        number(row.supply_air_vol_flow_rate_std_rho),
    );
    out.insert("HtRecSenOutput".into(), number(row.ht_rec_sen_output));
    out.insert("HtRecLatOutput".into(), number(row.ht_rec_lat_output));
    out.insert("OASenOutput".into(), number(row.oa_sen_output));
    out.insert("OALatOutput".into(), number(row.oa_lat_output));
    out.insert("SenOutputToZone".into(), number(row.sen_output_to_zone));
    out.insert("LatOutputToZone".into(), number(row.lat_output_to_zone));
    out.insert("SenCoilLoad".into(), number(row.sen_coil_load));
    out.insert("LatCoilLoad".into(), number(row.lat_coil_load));
    out.insert(
        "OAFlowMaxCoolOutputError".into(),
        json!(row.oa_flow_max_cool_output_error),
    );
    out.insert(
        "OAFlowMaxHeatOutputError".into(),
        json!(row.oa_flow_max_heat_output_error),
    );
    out.insert(
        "SaturationOutputError".into(),
        json!(row.saturation_output_error),
    );
    out.insert(
        "OAFlowMaxCoolOutputIndex".into(),
        json!(row.oa_flow_max_cool_output_index),
    );
    out.insert(
        "OAFlowMaxHeatOutputIndex".into(),
        json!(row.oa_flow_max_heat_output_index),
    );
    out.insert(
        "SaturationOutputIndex".into(),
        json!(row.saturation_output_index),
    );
    out.insert("availStatus".into(), json!(row.avail_status));
    out.insert("CoolErrIndex".into(), json!(row.cool_err_index));
    out.insert("HeatErrIndex".into(), json!(row.heat_err_index));
    out.insert("SenHeatEnergy".into(), number(row.sen_heat_energy));
    out.insert("LatHeatEnergy".into(), number(row.lat_heat_energy));
    out.insert("TotHeatEnergy".into(), number(row.tot_heat_energy));
    out.insert("SenCoolEnergy".into(), number(row.sen_cool_energy));
    out.insert("LatCoolEnergy".into(), number(row.lat_cool_energy));
    out.insert("TotCoolEnergy".into(), number(row.tot_cool_energy));
    out.insert("ZoneSenHeatEnergy".into(), number(row.zone_sen_heat_energy));
    out.insert("ZoneLatHeatEnergy".into(), number(row.zone_lat_heat_energy));
    out.insert("ZoneTotHeatEnergy".into(), number(row.zone_tot_heat_energy));
    out.insert("ZoneSenCoolEnergy".into(), number(row.zone_sen_cool_energy));
    out.insert("ZoneLatCoolEnergy".into(), number(row.zone_lat_cool_energy));
    out.insert("ZoneTotCoolEnergy".into(), number(row.zone_tot_cool_energy));
    out.insert("OASenHeatEnergy".into(), number(row.oa_sen_heat_energy));
    out.insert("OALatHeatEnergy".into(), number(row.oa_lat_heat_energy));
    out.insert("OATotHeatEnergy".into(), number(row.oa_tot_heat_energy));
    out.insert("OASenCoolEnergy".into(), number(row.oa_sen_cool_energy));
    out.insert("OALatCoolEnergy".into(), number(row.oa_lat_cool_energy));
    out.insert("OATotCoolEnergy".into(), number(row.oa_tot_cool_energy));
    out.insert(
        "HtRecSenHeatEnergy".into(),
        number(row.ht_rec_sen_heat_energy),
    );
    out.insert(
        "HtRecLatHeatEnergy".into(),
        number(row.ht_rec_lat_heat_energy),
    );
    out.insert(
        "HtRecTotHeatEnergy".into(),
        number(row.ht_rec_tot_heat_energy),
    );
    out.insert(
        "HtRecSenCoolEnergy".into(),
        number(row.ht_rec_sen_cool_energy),
    );
    out.insert(
        "HtRecLatCoolEnergy".into(),
        number(row.ht_rec_lat_cool_energy),
    );
    out.insert(
        "HtRecTotCoolEnergy".into(),
        number(row.ht_rec_tot_cool_energy),
    );
    out.insert("SenHeatRate".into(), number(row.sen_heat_rate));
    out.insert("LatHeatRate".into(), number(row.lat_heat_rate));
    out.insert("TotHeatRate".into(), number(row.tot_heat_rate));
    out.insert("SenCoolRate".into(), number(row.sen_cool_rate));
    out.insert("LatCoolRate".into(), number(row.lat_cool_rate));
    out.insert("TotCoolRate".into(), number(row.tot_cool_rate));
    out.insert("ZoneSenHeatRate".into(), number(row.zone_sen_heat_rate));
    out.insert("ZoneLatHeatRate".into(), number(row.zone_lat_heat_rate));
    out.insert("ZoneTotHeatRate".into(), number(row.zone_tot_heat_rate));
    out.insert("ZoneSenCoolRate".into(), number(row.zone_sen_cool_rate));
    out.insert("ZoneLatCoolRate".into(), number(row.zone_lat_cool_rate));
    out.insert("ZoneTotCoolRate".into(), number(row.zone_tot_cool_rate));
    out.insert("OASenHeatRate".into(), number(row.oa_sen_heat_rate));
    out.insert("OALatHeatRate".into(), number(row.oa_lat_heat_rate));
    out.insert("OATotHeatRate".into(), number(row.oa_tot_heat_rate));
    out.insert("OASenCoolRate".into(), number(row.oa_sen_cool_rate));
    out.insert("OALatCoolRate".into(), number(row.oa_lat_cool_rate));
    out.insert("OATotCoolRate".into(), number(row.oa_tot_cool_rate));
    out.insert("HtRecSenHeatRate".into(), number(row.ht_rec_sen_heat_rate));
    out.insert("HtRecLatHeatRate".into(), number(row.ht_rec_lat_heat_rate));
    out.insert("HtRecTotHeatRate".into(), number(row.ht_rec_tot_heat_rate));
    out.insert("HtRecSenCoolRate".into(), number(row.ht_rec_sen_cool_rate));
    out.insert("HtRecLatCoolRate".into(), number(row.ht_rec_lat_cool_rate));
    out.insert("HtRecTotCoolRate".into(), number(row.ht_rec_tot_cool_rate));
    out.insert("TimeEconoActive".into(), number(row.time_econo_active));
    out.insert("TimeHtRecActive".into(), number(row.time_ht_rec_active));
    out.insert("ZonePtr".into(), json!(row.zone_ptr));
    out.insert("HVACSizingIndex".into(), json!(row.hvac_sizing_index));
    out.insert("SupplyTemp".into(), number(row.supply_temp));
    out.insert("SupplyHumRat".into(), number(row.supply_hum_rat));
    out.insert("MixedAirTemp".into(), number(row.mixed_air_temp));
    out.insert("MixedAirHumRat".into(), number(row.mixed_air_hum_rat));
    out.insert(
        "heatFuelEffSched".into(),
        schedule(&s.schedules, row.heat_fuel_eff_sched)?,
    );
    out.insert(
        "coolFuelEffSched".into(),
        schedule(&s.schedules, row.cool_fuel_eff_sched)?,
    );
    out.insert(
        "ZoneTotHeatFuelRate".into(),
        number(row.zone_tot_heat_fuel_rate),
    );
    out.insert(
        "ZoneTotCoolFuelRate".into(),
        number(row.zone_tot_cool_fuel_rate),
    );
    out.insert(
        "ZoneTotHeatFuelEnergy".into(),
        number(row.zone_tot_heat_fuel_energy),
    );
    out.insert(
        "ZoneTotCoolFuelEnergy".into(),
        number(row.zone_tot_cool_fuel_energy),
    );
    out.insert("TotHeatFuelRate".into(), number(row.tot_heat_fuel_rate));
    out.insert("TotCoolFuelRate".into(), number(row.tot_cool_fuel_rate));
    out.insert("TotHeatFuelEnergy".into(), number(row.tot_heat_fuel_energy));
    out.insert("TotCoolFuelEnergy".into(), number(row.tot_cool_fuel_energy));
    out.insert(
        "heatingFuelType".into(),
        json!(row.heating_fuel_type as i32),
    );
    out.insert(
        "coolingFuelType".into(),
        json!(row.cooling_fuel_type as i32),
    );
    out.insert("address".into(), pointer(row));
    Ok(Value::Object(out))
}
fn plenum(v: &Option<Vec<SourcePurchasedAirPlenumArrayData>>) -> Value {
    let mut out = array1(v);
    out["items"]=Value::Array(v.as_deref().unwrap_or(&[]).iter().map(|r|json!({"NumPurchAir":r.number_purchased_air,"ReturnPlenumIndex":r.return_plenum_index,"PurchAirArray":integers(&r.purchased_air),"IsSimulated":booleans(&r.is_simulated)})).collect());
    out
}
fn manager(s: &Factory) -> Result<Value> {
    let m = &s.equipment.purchased_air;
    let mut units = array1(&m.units);
    units["items"] = Value::Array(
        m.units
            .as_deref()
            .unwrap_or(&[])
            .iter()
            .map(|r| unit(s, r))
            .collect::<Result<_>>()?,
    );
    let mut numeric = array1(&m.numeric_fields);
    numeric["items"] = Value::Array(
        m.numeric_fields
            .as_deref()
            .unwrap_or(&[])
            .iter()
            .map(|r| strings(&r.field_names))
            .collect(),
    );
    Ok(
        json!({"NumPurchAir":m.number_purchased_air,"NumPlenumArrays":m.number_plenum_arrays,"GetPurchAirInputFlag":m.get_purchased_air_input,"CheckEquipName":booleans(&m.check_equipment_name),"PurchAir":units,"PurchAirNumericFields":numeric,"PurchAirPlenumArrays":plenum(&m.plenum_arrays),"InitPurchasedAirMyOneTimeFlag":m.initialize_once,"InitPurchasedAirZoneEquipmentListChecked":m.zone_equipment_list_checked,"InitPurchasedAirMyEnvrnFlag":booleans(&m.environment_flags),"InitPurchasedAirMySizeFlag":booleans(&m.sizing_flags),"InitPurchasedAirOneTimeUnitInitsDone":booleans(&m.unit_initializations_done),"TempPurchAirPlenumArrays":plenum(&m.temporary_plenum_arrays)}),
    )
}
fn node(s: &Factory, row: &super::super::source_nodes::SourceNodeData) -> Result<Value> {
    let mut out = Map::new();
    out.insert("fluidType".into(), json!(row.fluid_type as i32));
    out.insert("FluidIndex".into(), json!(row.fluid_index));
    out.insert("Temp".into(), number(row.temperature));
    out.insert("TempMin".into(), number(row.temperature_min));
    out.insert("TempMax".into(), number(row.temperature_max));
    out.insert("TempSetPoint".into(), number(row.temperature_setpoint));
    out.insert(
        "TempLastTimestep".into(),
        number(row.temperature_last_timestep),
    );
    out.insert(
        "MassFlowRateRequest".into(),
        number(row.mass_flow_rate_request),
    );
    out.insert("MassFlowRate".into(), number(row.mass_flow_rate));
    out.insert("MassFlowRateMin".into(), number(row.mass_flow_rate_min));
    out.insert("MassFlowRateMax".into(), number(row.mass_flow_rate_max));
    out.insert(
        "MassFlowRateMinAvail".into(),
        number(row.mass_flow_rate_min_available),
    );
    out.insert(
        "MassFlowRateMaxAvail".into(),
        number(row.mass_flow_rate_max_available),
    );
    out.insert(
        "MassFlowRateSetPoint".into(),
        number(row.mass_flow_rate_setpoint),
    );
    out.insert("Quality".into(), number(row.quality));
    out.insert("Press".into(), number(row.pressure));
    out.insert("Enthalpy".into(), number(row.enthalpy));
    out.insert(
        "EnthalpyLastTimestep".into(),
        number(row.enthalpy_last_timestep),
    );
    out.insert("HumRat".into(), number(row.humidity_ratio));
    out.insert("HumRatMin".into(), number(row.humidity_ratio_min));
    out.insert("HumRatMax".into(), number(row.humidity_ratio_max));
    out.insert("HumRatSetPoint".into(), number(row.humidity_ratio_setpoint));
    out.insert(
        "TempSetPointHi".into(),
        number(row.temperature_setpoint_high),
    );
    out.insert(
        "TempSetPointLo".into(),
        number(row.temperature_setpoint_low),
    );
    out.insert("Height".into(), number(row.height));
    out.insert("IsLocalNode".into(), json!(row.is_local_node));
    out.insert(
        "outAirDryBulbSched".into(),
        schedule(&s.schedules, row.outdoor_air_dry_bulb_schedule)?,
    );
    out.insert(
        "outAirWetBulbSched".into(),
        schedule(&s.schedules, row.outdoor_air_wet_bulb_schedule)?,
    );
    out.insert(
        "outAirWindSpeedSched".into(),
        schedule(&s.schedules, row.outdoor_air_wind_speed_schedule)?,
    );
    out.insert(
        "outAirWindDirSched".into(),
        schedule(&s.schedules, row.outdoor_air_wind_direction_schedule)?,
    );
    out.insert("OutAirDryBulb".into(), number(row.outdoor_air_dry_bulb));
    out.insert(
        "EMSOverrideOutAirDryBulb".into(),
        json!(row.ems_override_outdoor_air_dry_bulb),
    );
    out.insert(
        "EMSValueForOutAirDryBulb".into(),
        number(row.ems_value_outdoor_air_dry_bulb),
    );
    out.insert("OutAirWetBulb".into(), number(row.outdoor_air_wet_bulb));
    out.insert(
        "EMSOverrideOutAirWetBulb".into(),
        json!(row.ems_override_outdoor_air_wet_bulb),
    );
    out.insert(
        "EMSValueForOutAirWetBulb".into(),
        number(row.ems_value_outdoor_air_wet_bulb),
    );
    out.insert("OutAirWindSpeed".into(), number(row.outdoor_air_wind_speed));
    out.insert(
        "EMSOverrideOutAirWindSpeed".into(),
        json!(row.ems_override_outdoor_air_wind_speed),
    );
    out.insert(
        "EMSValueForOutAirWindSpeed".into(),
        number(row.ems_value_outdoor_air_wind_speed),
    );
    out.insert(
        "OutAirWindDir".into(),
        number(row.outdoor_air_wind_direction),
    );
    out.insert(
        "EMSOverrideOutAirWindDir".into(),
        json!(row.ems_override_outdoor_air_wind_direction),
    );
    out.insert(
        "EMSValueForOutAirWindDir".into(),
        number(row.ems_value_outdoor_air_wind_direction),
    );
    out.insert("CO2".into(), number(row.carbon_dioxide));
    out.insert("CO2SetPoint".into(), number(row.carbon_dioxide_setpoint));
    out.insert("GenContam".into(), number(row.generic_contaminant));
    out.insert(
        "GenContamSetPoint".into(),
        number(row.generic_contaminant_setpoint),
    );
    out.insert(
        "SPMNodeWetBulbRepReq".into(),
        json!(row.setpoint_manager_wet_bulb_reporting_required),
    );
    out.insert(
        "plantNodeErrorMsgIssued".into(),
        json!(row.plant_node_error_message_issued),
    );
    out.insert("address".into(), pointer(row));
    Ok(Value::Object(out))
}
fn nodes(s: &Factory) -> Result<Value> {
    let n = &s.equipment.nodes;
    let mut ids = shape(&n.node_ids);
    ids["lower"] = json!(0);
    ids["upper"] = json!(n.node_ids.as_ref().map_or(0, Vec::len).saturating_sub(1));
    ids["items"] = json!(n.node_ids.as_deref().unwrap_or(&[]));
    let mut values = array1(&n.physical);
    values["items"] = Value::Array(
        n.physical
            .as_deref()
            .unwrap_or(&[])
            .iter()
            .map(|r| node(s, r))
            .collect::<Result<_>>()?,
    );
    Ok(json!({"NodeID":ids,"Node":values}))
}
fn zone_row(r: &hb::selected_sizing::SourceZoneEquipmentSizing) -> Value {
    let mut out = Map::new();
    out.insert("AirVolFlow".into(), number(r.air__vol__flow));
    out.insert("MaxHWVolFlow".into(), number(r.max_hw_vol__flow));
    out.insert("MaxCWVolFlow".into(), number(r.max_cw_vol__flow));
    out.insert("OAVolFlow".into(), number(r.oa_vol__flow));
    out.insert("ATMixerVolFlow".into(), number(r.at_mixer__vol__flow));
    out.insert(
        "ATMixerCoolPriDryBulb".into(),
        number(r.at_mixer__cool__pri__dry__bulb),
    );
    out.insert(
        "ATMixerCoolPriHumRat".into(),
        number(r.at_mixer__cool__pri__hum__rat),
    );
    out.insert(
        "ATMixerHeatPriDryBulb".into(),
        number(r.at_mixer__heat__pri__dry__bulb),
    );
    out.insert(
        "ATMixerHeatPriHumRat".into(),
        number(r.at_mixer__heat__pri__hum__rat),
    );
    out.insert("DesCoolingLoad".into(), number(r.des__cooling__load));
    out.insert("DesHeatingLoad".into(), number(r.des__heating__load));
    out.insert(
        "CoolingAirVolFlow".into(),
        number(r.cooling__air__vol__flow),
    );
    out.insert(
        "HeatingAirVolFlow".into(),
        number(r.heating__air__vol__flow),
    );
    out.insert("SystemAirVolFlow".into(), number(r.system__air__vol__flow));
    out.insert("AirFlow".into(), json!(r.air__flow));
    out.insert("CoolingAirFlow".into(), json!(r.cooling__air__flow));
    out.insert("HeatingAirFlow".into(), json!(r.heating__air__flow));
    out.insert("SystemAirFlow".into(), json!(r.system__air__flow));
    out.insert("Capacity".into(), json!(r.capacity));
    out.insert("CoolingCapacity".into(), json!(r.cooling__capacity));
    out.insert("HeatingCapacity".into(), json!(r.heating__capacity));
    out.insert("SystemCapacity".into(), json!(r.system__capacity));
    out.insert(
        "DesignSizeFromParent".into(),
        json!(r.design__size__from__parent),
    );
    out.insert("HVACSizingIndex".into(), json!(r.hvac_sizing__index));
    out.insert("SizingMethod".into(), integers(&r.sizing__method));
    out.insert("CapSizingMethod".into(), integers(&r.cap__sizing__method));
    Value::Object(out)
}
fn zone_array(a: &Option<Vec<hb::selected_sizing::SourceZoneEquipmentSizing>>) -> Value {
    let mut v = shape(a);
    v["items"] = Value::Array(a.as_deref().unwrap_or(&[]).iter().map(zone_row).collect());
    v
}
fn globals(s: &Factory) -> Value {
    let mut out = Map::new();
    out.insert("CurOASysNum".into(), json!(s.sizing.autosizing.CurOASysNum));
    out.insert("CurDuctType".into(), json!(s.sizing.autosizing.CurDuctType));
    out.insert(
        "TermUnitSingDuct".into(),
        json!(s.sizing.autosizing.TermUnitSingDuct),
    );
    out.insert("TermUnitPIU".into(), json!(s.sizing.autosizing.TermUnitPIU));
    out.insert("TermUnitIU".into(), json!(s.sizing.autosizing.TermUnitIU));
    out.insert(
        "ZoneEqFanCoil".into(),
        json!(s.sizing.autosizing.ZoneEqFanCoil),
    );
    out.insert(
        "ZoneEqUnitHeater".into(),
        json!(s.sizing.autosizing.ZoneEqUnitHeater),
    );
    out.insert(
        "ZoneEqUnitVent".into(),
        json!(s.sizing.autosizing.ZoneEqUnitVent),
    );
    out.insert(
        "ZoneEqVentedSlab".into(),
        json!(s.sizing.autosizing.ZoneEqVentedSlab),
    );
    out.insert(
        "ZoneSizingInput".into(),
        shape(&s.sizing.autosizing.ZoneSizingInput),
    );
    out.insert(
        "FinalZoneSizing".into(),
        shape(&s.sizing.autosizing.FinalZoneSizing),
    );
    out.insert(
        "TermUnitFinalZoneSizing".into(),
        shape(&s.sizing.autosizing.TermUnitFinalZoneSizing),
    );
    out.insert(
        "SysSizInput".into(),
        shape(&s.sizing.autosizing.SysSizInput),
    );
    out.insert(
        "FinalSysSizing".into(),
        shape(&s.sizing.autosizing.FinalSysSizing),
    );
    out.insert(
        "PlantSizData".into(),
        shape(&s.sizing.autosizing.PlantSizData),
    );
    out.insert(
        "DataEMSOverrideON".into(),
        json!(s.sizing.autosizing.DataEMSOverrideON),
    );
    out.insert(
        "DataEMSOverride".into(),
        number(s.sizing.autosizing.DataEMSOverride),
    );
    out.insert(
        "DataAutosizable".into(),
        json!(s.sizing.autosizing.DataAutosizable),
    );
    out.insert(
        "DataConstantUsedForSizing".into(),
        number(s.sizing.autosizing.DataConstantUsedForSizing),
    );
    out.insert(
        "DataFractionUsedForSizing".into(),
        number(s.sizing.autosizing.DataFractionUsedForSizing),
    );
    out.insert(
        "DataFanIndex".into(),
        json!(s.sizing.autosizing.DataFanIndex),
    );
    out.insert("DataFanType".into(), json!(s.sizing.autosizing.DataFanType));
    out.insert(
        "DataPltSizHeatNum".into(),
        json!(s.sizing.autosizing.DataPltSizHeatNum),
    );
    out.insert(
        "DataWaterLoopNum".into(),
        json!(s.sizing.autosizing.DataWaterLoopNum),
    );
    out.insert(
        "DataPltSizCoolNum".into(),
        json!(s.sizing.autosizing.DataPltSizCoolNum),
    );
    out.insert(
        "DataWaterCoilSizHeatDeltaT".into(),
        number(s.sizing.autosizing.DataWaterCoilSizHeatDeltaT),
    );
    out.insert(
        "DataWaterCoilSizCoolDeltaT".into(),
        number(s.sizing.autosizing.DataWaterCoilSizCoolDeltaT),
    );
    out.insert(
        "DataCapacityUsedForSizing".into(),
        number(s.sizing.autosizing.DataCapacityUsedForSizing),
    );
    out.insert(
        "DataHeatSizeRatio".into(),
        number(s.sizing.autosizing.DataHeatSizeRatio),
    );
    out.insert(
        "DataAirFlowUsedForSizing".into(),
        number(s.sizing.autosizing.DataAirFlowUsedForSizing),
    );
    out.insert(
        "DataDesInletAirTemp".into(),
        number(s.sizing.autosizing.DataDesInletAirTemp),
    );
    out.insert(
        "DataDesAccountForFanHeat".into(),
        json!(s.sizing.autosizing.DataDesAccountForFanHeat),
    );
    out.insert(
        "DataFanPlacement".into(),
        json!(s.sizing.autosizing.DataFanPlacement),
    );
    out.insert(
        "DataDesInletAirHumRat".into(),
        number(s.sizing.autosizing.DataDesInletAirHumRat),
    );
    out.insert(
        "DataDesOutletAirHumRat".into(),
        number(s.sizing.autosizing.DataDesOutletAirHumRat),
    );
    out.insert(
        "DataDesOutletAirTemp".into(),
        number(s.sizing.autosizing.DataDesOutletAirTemp),
    );
    out.insert(
        "DataDesInletWaterTemp".into(),
        number(s.sizing.autosizing.DataDesInletWaterTemp),
    );
    out.insert(
        "DataFlowUsedForSizing".into(),
        number(s.sizing.autosizing.DataFlowUsedForSizing),
    );
    out.insert(
        "DataWaterFlowUsedForSizing".into(),
        number(s.sizing.autosizing.DataWaterFlowUsedForSizing),
    );
    out.insert(
        "DataSizingFraction".into(),
        number(s.sizing.autosizing.DataSizingFraction),
    );
    out.insert(
        "DataDXSpeedNum".into(),
        json!(s.sizing.autosizing.DataDXSpeedNum),
    );
    out.insert(
        "DataDesicRegCoil".into(),
        json!(s.sizing.autosizing.DataDesicRegCoil),
    );
    out.insert(
        "DataZoneUsedForSizing".into(),
        json!(s.sizing.autosizing.DataZoneUsedForSizing),
    );
    out.insert(
        "DataDesicDehumNum".into(),
        json!(s.sizing.autosizing.DataDesicDehumNum),
    );
    out.insert(
        "DataNomCapInpMeth".into(),
        json!(s.sizing.autosizing.DataNomCapInpMeth),
    );
    out.insert("DataCoilNum".into(), json!(s.sizing.autosizing.DataCoilNum));
    out.insert("DataFanOp".into(), json!(s.sizing.autosizing.DataFanOp));
    out.insert(
        "DataDesignCoilCapacity".into(),
        number(s.sizing.autosizing.DataDesignCoilCapacity),
    );
    out.insert(
        "DataErrorsFound".into(),
        json!(s.sizing.autosizing.DataErrorsFound),
    );
    out.insert(
        "DataBypassFrac".into(),
        number(s.sizing.autosizing.DataBypassFrac),
    );
    out.insert(
        "DataIsDXCoil".into(),
        json!(s.sizing.autosizing.DataIsDXCoil),
    );
    out.insert(
        "DataNonZoneNonAirloopValue".into(),
        number(s.sizing.autosizing.DataNonZoneNonAirloopValue),
    );
    out.insert(
        "DataDXCoolsLowSpeedsAutozize".into(),
        json!(s.sizing.autosizing.DataDXCoolsLowSpeedsAutozize),
    );
    out.insert(
        "DataScalableSizingON".into(),
        json!(s.sizing.autosizing.DataScalableSizingON),
    );
    out.insert(
        "DataScalableCapSizingON".into(),
        json!(s.sizing.autosizing.DataScalableCapSizingON),
    );
    out.insert(
        "HRFlowSizingFlag".into(),
        json!(s.sizing.autosizing.HRFlowSizingFlag),
    );
    out.insert(
        "DataFracOfAutosizedCoolingAirflow".into(),
        number(s.sizing.autosizing.DataFracOfAutosizedCoolingAirflow),
    );
    out.insert(
        "DataFracOfAutosizedHeatingAirflow".into(),
        number(s.sizing.autosizing.DataFracOfAutosizedHeatingAirflow),
    );
    out.insert(
        "DataFlowPerCoolingCapacity".into(),
        number(s.sizing.autosizing.DataFlowPerCoolingCapacity),
    );
    out.insert(
        "DataAutosizedCoolingCapacity".into(),
        number(s.sizing.autosizing.DataAutosizedCoolingCapacity),
    );
    out.insert(
        "DataFlowPerHeatingCapacity".into(),
        number(s.sizing.autosizing.DataFlowPerHeatingCapacity),
    );
    out.insert(
        "DataAutosizedHeatingCapacity".into(),
        number(s.sizing.autosizing.DataAutosizedHeatingCapacity),
    );
    out.insert(
        "DataCoilSizingAirInTemp".into(),
        number(s.sizing.autosizing.DataCoilSizingAirInTemp),
    );
    out.insert(
        "DataCoilSizingAirInHumRat".into(),
        number(s.sizing.autosizing.DataCoilSizingAirInHumRat),
    );
    out.insert(
        "DataCoilSizingAirOutTemp".into(),
        number(s.sizing.autosizing.DataCoilSizingAirOutTemp),
    );
    out.insert(
        "DataCoilSizingAirOutHumRat".into(),
        number(s.sizing.autosizing.DataCoilSizingAirOutHumRat),
    );
    out.insert(
        "DataCoilSizingFanCoolLoad".into(),
        number(s.sizing.autosizing.DataCoilSizingFanCoolLoad),
    );
    out.insert(
        "DataCoilSizingCapFT".into(),
        number(s.sizing.autosizing.DataCoilSizingCapFT),
    );
    out.insert(
        "DataTotCapCurveIndex".into(),
        json!(s.sizing.autosizing.DataTotCapCurveIndex),
    );
    out.insert(
        "DataTotCapCurveValue".into(),
        number(s.sizing.autosizing.DataTotCapCurveValue),
    );
    out.insert(
        "DataFracOfAutosizedCoolingCapacity".into(),
        number(s.sizing.autosizing.DataFracOfAutosizedCoolingCapacity),
    );
    out.insert(
        "DataFracOfAutosizedHeatingCapacity".into(),
        number(s.sizing.autosizing.DataFracOfAutosizedHeatingCapacity),
    );
    out.insert(
        "DataCoilIsSuppHeater".into(),
        json!(s.sizing.autosizing.DataCoilIsSuppHeater),
    );
    out.insert(
        "SuppHeatCap".into(),
        number(s.sizing.autosizing.SuppHeatCap),
    );
    out.insert(
        "UnitaryHeatCap".into(),
        number(s.sizing.autosizing.UnitaryHeatCap),
    );
    out.insert(
        "DataCoolCoilType".into(),
        json!(s.sizing.autosizing.DataCoolCoilType),
    );
    out.insert(
        "DataCoolCoilIndex".into(),
        json!(s.sizing.autosizing.DataCoolCoilIndex),
    );
    out.insert(
        "ZoneHVACSizing".into(),
        shape(&s.sizing.autosizing.ZoneHVACSizing),
    );
    out.insert(
        "DataZoneNumber".into(),
        json!(s.sizing.autosizing.DataZoneNumber),
    );
    out.insert(
        "AutoVsHardSizingThreshold".into(),
        number(s.sizing.autosizing.AutoVsHardSizingThreshold),
    );
    out.insert(
        "SysSizingRunDone".into(),
        json!(s.sizing.system_sizing_run_done),
    );
    out.insert(
        "ZoneSizingRunDone".into(),
        json!(s.sizing.zone_sizing_run_done),
    );
    out.insert("CurSysNum".into(), json!(s.sizing.current_system_number));
    out.insert(
        "CurZoneEqNum".into(),
        json!(s.sizing.current_zone_equipment_number),
    );
    out.insert(
        "CurTermUnitSizingNum".into(),
        json!(s.sizing.current_terminal_unit_sizing_number),
    );
    out.insert(
        "NumSysSizInput".into(),
        json!(s.sizing.number_of_system_sizing_input),
    );
    out.insert(
        "NumZoneSizingInput".into(),
        json!(s.sizing.number_of_zone_sizing_input),
    );
    out.insert(
        "UnitarySysEqSizing".into(),
        zone_array(&s.sizing.unitary_system_equipment_sizing),
    );
    out.insert(
        "OASysEqSizing".into(),
        zone_array(&s.sizing.outside_air_system_equipment_sizing),
    );
    out.insert(
        "TermUnitSizing".into(),
        shape(&s.sizing.terminal_unit_sizing),
    );
    out.insert(
        "ZoneEqSizing".into(),
        zone_array(&s.sizing.zone_equipment_sizing),
    );
    out.insert(
        "ZoneCoolingOnlyFan".into(),
        json!(s.sizing.zone_cooling_only_fan),
    );
    out.insert(
        "ZoneHeatingOnlyFan".into(),
        json!(s.sizing.zone_heating_only_fan),
    );
    out.insert(
        "DataCoolCoilCap".into(),
        number(s.sizing.data_cool_coil_capacity),
    );
    out.insert(
        "StdRhoAir".into(),
        number(s.weather.environment.std_rho_air),
    );
    out.insert("isEpJSON".into(), json!(false));
    out.insert("DoZoneSizing".into(), json!(s.globals.do_zone_sizing));
    out.insert("DoSystemSizing".into(), json!(s.globals.do_system_sizing));
    out.insert(
        "DisplayExtraWarnings".into(),
        json!(s.globals.display_extra_warnings),
    );
    out.insert(
        "NumPrimaryAirSys".into(),
        json!(s.startup.hvac_globals.number_of_primary_air_systems),
    );
    Value::Object(out)
}
fn reports(s: &Factory) -> Result<Value> {
    let mut rows = array1(&s.registry.component_sizing_entries);
    rows["items"]=Value::Array(s.registry.component_sizing_entries.as_deref().unwrap_or(&[]).iter().map(|r|json!({"typeField":r.component_type,"nameField":r.component_name,"description":r.description,"valField":number(r.value),"strField":r.string_value,"active":r.active,"written":r.written})).collect());
    Ok(
        json!({"oneTimeCompRptHeaderFlag":s.weather.environment.one_time_component_report_header_flag,"eio":std::str::from_utf8(&s.eio)?,"sqlite_present":false,"sizeCompSizeTableEntry":s.registry.component_sizing_capacity,"numCompSizeTableEntry":s.registry.component_sizing_count,"CompSizeTableEntry":rows}),
    )
}

fn units(u: hb::source_output_registration::SourceOutputUnits) -> i32 {
    use hb::source_output_registration::SourceOutputUnits as U;
    // Literal pinned Constant::Units enumeration, not fixture-derived values.
    match u {
        U::Invalid => -1,
        U::Dimensionless => 6,
        U::C => 1,
        U::Pa => 4,
        U::M => 21,
        U::KgPerM3 => 36,
        U::Lux => 14,
        U::LumPerW => 15,
        U::Percentage => 33,
        U::Hr => 16,
        U::KgWaterPerSecond => 49,
        U::KgWaterPerKgDryAir => 2,
        U::JPerKg => 31,
        U::MPerS => 19,
        U::Deg => 34,
        U::M2 => 41,
        U::Rad => 43,
        U::W => 8,
        U::J => 9,
        U::Kg => 11,
        U::WPerM2 => 20,
        U::WPerM2K => 30,
        U::KgPerSecond => 0,
        U::M3PerSecond => 5,
        U::M3 => 10,
        U::Ach => 12,
        U::JPerKgK => 38,
        U::Ppm => 3,
    }
}
/// Compose the same persistent output domains for passive registered-Which reads.
struct RegisteredOutputOwners<'a> {
    equipment: &'a OrdinaryEquipment,
    air: hb::ordinary_air_heat_balance::output_registration::SourceAirOutputOwners<'a>,
}
impl SourceRealOutputOwners for RegisteredOutputOwners<'_> {
    fn real_output(
        &self,
        target: hb::source_output_registration::SourceRealOutputTarget,
    ) -> std::result::Result<&f64, hb::radiation::ordinary::OrdinaryRadiationError> {
        match target {
            hb::source_output_registration::SourceRealOutputTarget::Air { .. } => {
                self.air.real_output(target)
            }
            _ => self.equipment.real_output(target),
        }
    }
}
impl SourceIntegerOutputOwners for RegisteredOutputOwners<'_> {
    fn integer_output(
        &self,
        target: hb::source_output_registration::SourceIntegerOutputTarget,
    ) -> std::result::Result<&i32, hb::radiation::ordinary::OrdinaryRadiationError> {
        match target {
            hb::source_output_registration::SourceIntegerOutputTarget::ZoneThermostatControlType {
                ..
            } => self.air.integer_output(target),
            _ => self.equipment.integer_output(target),
        }
    }
}
fn registration(s: &Factory) -> Result<Value> {
    use hb::source_output_registration::{
        SourceIntegerOutputTarget as I, SourceRealOutputTarget as R,
    };
    let report = hb::ordinary_air_heat_balance::predictor_corrector::simple_air::SourceAirReportOutputOwners {
        controller: &s.controller,
    };
    let resilience = hb::ordinary_surface_management::SourceResilienceOutputOwners {
        zones: &s.zones,
    };
    let owners = RegisteredOutputOwners {
        equipment: &s.equipment,
        air: hb::ordinary_air_heat_balance::output_registration::SourceAirOutputOwners {
            air: &s.air,
            demand: &s.controller.demand,
            fan: &s.fan,
            report: &report,
            resilience: &resilience,
        },
    };
    let mut dictionary = Vec::new();
    for r in &s.registration.dictionary_variables {
        dictionary.push(json!({"name":r.name,"units":units(r.units),"timeStepType":r.time_step_type as i32,"storeType":r.store_type as i32,"variableType":if r.integer{0}else{1},"keyOutVarNums":r.key_out_var_nums}));
    }
    let mut live = Vec::new();
    for (i, r) in s.registration.out_vars.iter().enumerate() {
        let mut row = json!({"name":r.name,"key":r.key,"units":units(r.units),"timeStepType":r.time_step_type as i32,"storeType":r.store_type as i32,"freq":r.frequency as i32,"meterNums":r.meter_numbers,"owner_address":pointer(r)});
        if let Some(target) = r.which {
            let value = s.registration.read_registered_real(i, &owners)?;
            row["Which"] = pointer(value);
            row["live_value"] = number(*value);
            row["same_unit_field"] = match target {
                R::PurchasedAir { field, index } => {
                    json!([{"unit":index,"member":format!("{field:?}")}])
                }
                _ => json!([]),
            };
        } else if let Some(target) = r.integer_which {
            let value = s.registration.read_registered_integer(i, &owners)?;
            row["Which"] = pointer(value);
            row["live_value"] = json!(*value);
            row["same_unit_field"] = match target {
                I::PurchasedAirAvailabilityStatus { index } => {
                    json!([{"unit":index,"member":"availStatus"}])
                }
                _ => json!([]),
            };
        } else {
            return Err(invalid("actual registered live Which binding").into());
        }
        live.push(row);
    }
    let meters=s.registration.meters.meters.iter().map(|r|json!({"Name":r.name,"type":r.meter_type as i32,"resource":r.resource as i32,"endUseCat":r.end_use as i32,"group":r.group as i32,"units":units(r.units),"srcVarNums":r.source_variable_numbers,"srcMeterNums":r.source_meter_numbers,"CurTSValue":number(r.current_time_step_value)})).collect::<Vec<_>>();
    Ok(json!({"dictionary":dictionary,"live":live,"meters":meters}))
}
fn context(s: &Factory) -> Result<Value> {
    let z = index(s.zone)?;
    let c = s
        .equipment
        .zone_config
        .as_deref()
        .and_then(|v| v.get(z))
        .ok_or_else(|| invalid("actual Config observer"))?;
    let l = s
        .equipment
        .equipment_lists
        .as_deref()
        .and_then(|v| v.get(index(s.list).ok()?))
        .ok_or_else(|| invalid("actual List observer"))?;
    let d = s
        .controller
        .demand
        .zones
        .as_deref()
        .and_then(|v| v.get(z))
        .ok_or_else(|| invalid("actual demand observer"))?;
    let w = s
        .controller
        .demand
        .zone_moisture
        .as_deref()
        .and_then(|v| v.get(z))
        .ok_or_else(|| invalid("actual moisture observer"))?;
    let t = s
        .fan
        .thermostat
        .zone_tstat_setpoints
        .as_deref()
        .and_then(|v| v.get(z))
        .ok_or_else(|| invalid("actual thermostat observer"))?;
    let h = &s.startup.hvac_globals;
    let g = &s.weather.global;
    let e = &s.weather.environment;
    let ordinal_array = |v: &Option<Vec<usize>>| {
        let mut out = array1(v);
        out["items"] = json!(v.as_deref().unwrap_or(&[]));
        out
    };
    let mut out = json!({"state_address":pointer(s),"zone":s.zone,"list":s.list,"equipment":s.pointer,"CompIndex":l.equipment_indices.as_deref().and_then(|v|v.get(index(s.pointer).ok()?)).ok_or_else(||invalid("actual CompIndex cell"))?,
        "config":{"address":pointer(c),"ZoneName":c.zone_name,"IsControlled":c.is_controlled,"ZoneNode":c.zone_node,"EquipListIndex":c.equipment_list_index,"InletNode":ordinal_array(&c.inlet_node_numbers),"ReturnNode":ordinal_array(&c.return_node_numbers),"ExhaustNode":ordinal_array(&c.exhaust_node_numbers)},
        "demand":{"address":pointer(d),"RemainingOutputRequired":number(d.base.remaining_output_required),"RemainingOutputReqToHeatSP":number(d.remaining_output_to_heat_sp),"RemainingOutputReqToCoolSP":number(d.remaining_output_to_cool_sp),"SequencedOutputRequired":shape(&d.sequenced_output_required),"SequencedOutputRequiredToHeatingSP":shape(&d.sequenced_output_to_heating_sp),"SequencedOutputRequiredToCoolingSP":shape(&d.sequenced_output_to_cooling_sp)},
        "moisture":{"address":pointer(w),"RemainingOutputReqToHumidSP":number(w.remaining_output_to_humid_sp),"RemainingOutputReqToDehumidSP":number(w.remaining_output_to_dehumid_sp),"SequencedOutputRequired":shape(&w.sequenced_output_required),"SequencedOutputRequiredToHumidSP":shape(&w.sequenced_output_to_humid_sp),"SequencedOutputRequiredToDehumidSP":shape(&w.sequenced_output_to_dehumid_sp)},
        "TimeStepSys":number(h.time_step_sys),"TimeStepSysSec":number(h.time_step_sys_seconds),"HourOfDay":g.hour_of_day,"TimeStep":g.time_step,"BeginEnvrnFlag":g.begin_envrn_flag,"SysSizingCalc":s.globals.system_sizing_calculation,"WarmupFlag":g.warmup_flag,
        "TempControlType":s.fan.thermostat.temp_control_type.as_deref().and_then(|v|v.get(z)).ok_or_else(||invalid("control type observer"))?,"setptLo":number(t.setpt_lo),"setptHi":number(t.setpt_hi),
        "airHumRat":number(s.air.zones.as_deref().and_then(|v|v.get(z)).ok_or_else(||invalid("actual HB observer"))?.air.air_hum_rat),"StdRhoAir":number(e.std_rho_air),"OutBaroPress":number(e.current_weather.out_baro_press),"OutHumRat":number(e.current_weather.out_hum_rat),
        "DayOfYear_Schedule":e.day_of_year_schedule,"DayOfWeek":e.day_of_week,"DSTIndicator":e.dst_indicator,"HolidayIndex":e.holiday_index,
        "CO2Simulation":s.control.carbon_dioxide_simulation,"GenericContamSimulation":s.control.generic_contaminant_simulation,
        "GetZoneEquipmentInputFlag":s.equipment.get_equipment_input,"ZoneEquipInputsFilled":s.equipment.equipment_inputs_filled,"InitZoneEquipmentOneTimeFlag":s.equipment.init_equipment_one_time_flag,"InitZoneEquipmentEnvrnFlag":s.equipment.init_equipment_environment_flag,"GetAvailMgrInputFlag":s.hvac_manager.ancillary.availability.get_input_flag});
    let availability = &s.hvac_manager.ancillary.availability.zone_components;
    out["availability_allocated"] = json!(availability.is_some());
    if let Some(rows) = availability {
        let a = rows
            .get(11)
            .ok_or_else(|| invalid("actual PurchasedAir type12"))?;
        let mut v = json!({"TotalNumComp":a.total_components,"rows":array1(&a.managers)});
        if let Some(rows) = &a.managers {
            v["items"]=Value::Array(rows.iter().map(|r|json!({"address":pointer(r),"NumAvailManagers":r.number_managers,"availStatus":r.status,"ZoneNum":r.zone_number})).collect());
        }
        out["availability"] = v;
    }
    Ok(out)
}
fn errors(s: &Factory) -> Result<Value> {
    let e = &s.error_state.tracking;
    Ok(
        json!({"text":std::str::from_utf8(&s.error)?,"TotalWarningErrors":e.total_warning_errors,"TotalSevereErrors":e.total_severe_errors,"TotalWarningErrorsDuringWarmup":e.total_warning_errors_during_warmup,"TotalSevereErrorsDuringWarmup":e.total_severe_errors_during_warmup,"TotalWarningErrorsDuringSizing":e.total_warning_errors_during_sizing,"TotalSevereErrorsDuringSizing":e.total_severe_errors_during_sizing,"NumRecurringErrors":s.ancillary.source_recurring_errors.number,"RecurringErrors":array1(&s.ancillary.source_recurring_errors.rows),"MatchCounts":{"allocated":true,"size":e.match_counts.len(),"lower":1,"upper":e.match_counts.len(),"items":e.match_counts}}),
    )
}
fn snapshot(s: &Factory) -> Result<Value> {
    Ok(
        json!({"manager":manager(s)?,"nodes":nodes(s)?,"context":context(s)?,"sizing":globals(s),"reports":reports(s)?,"registration":registration(s)?,"errors":errors(s)?,"returned_sensible":number(s.sensible),"returned_moisture":number(s.moisture)}),
    )
}

#[derive(Default)]
struct Identities {
    pairs: BTreeMap<(String, String), String>,
    raw_bits: usize,
}
impl Identities {
    fn check(&mut self, source: &Value, native: &Value, path: &str) -> Result {
        match (source, native) {
            (Value::Object(a), Value::Object(b)) => {
                assert_eq!(
                    a.keys().collect::<Vec<_>>(),
                    b.keys().collect::<Vec<_>>(),
                    "{path} complete observed keys"
                );
                if a.len() == 1 && a.contains_key("value_bits") {
                    self.raw_bits += 1;
                }
                for (k, v) in a {
                    let expected = b.get(k).ok_or_else(|| invalid("same observed key"))?;
                    if matches!(
                        k.as_str(),
                        "address" | "Which" | "owner_address" | "state_address"
                    ) {
                        match (v, expected) {
                            (Value::Null, Value::Null) => {}
                            (Value::String(actual), Value::String(expected)) => {
                                assert!(
                                    actual.starts_with("0x") && expected.starts_with("0x"),
                                    "{path} nonnull real pointer transport"
                                );
                                // Record identity separately from numerical bits. Different
                                // Rust/C++ layouts have no cross-language address equality.
                                let kind = if k == "address" {
                                    if path.contains(".Node.items") {
                                        "Node"
                                    } else if path.contains(".PurchAir.items") {
                                        "Unit"
                                    } else if path.contains(".availability.items") {
                                        "Availability"
                                    } else if path.ends_with(".config") {
                                        "Config"
                                    } else if path.ends_with(".demand") {
                                        "Demand"
                                    } else if path.ends_with(".moisture") {
                                        "Moisture"
                                    } else {
                                        "Schedule"
                                    }
                                } else {
                                    k
                                };
                                let key = (kind.to_owned(), expected.clone());
                                if let Some(old) = self.pairs.get(&key) {
                                    assert_eq!(
                                        old, actual,
                                        "{path} same real Native owner retains same real Source owner"
                                    );
                                } else {
                                    assert!(
                                        !self.pairs.iter().any(
                                            |((old_kind, old_native), old_source)| old_kind == kind
                                                && old_native != expected
                                                && old_source == actual
                                        ),
                                        "{path} distinct owners stay distinct"
                                    );
                                    self.pairs.insert(key, actual.clone());
                                }
                            }
                            _ => return Err(invalid("actual pointer/null shape").into()),
                        }
                    } else {
                        self.check(v, expected, &format!("{path}.{k}"))?;
                    }
                }
            }
            (Value::Array(a), Value::Array(b)) => {
                assert_eq!(a.len(), b.len(), "{path} full array extent");
                for (i, (a, b)) in a.iter().zip(b).enumerate() {
                    self.check(a, b, &format!("{path}[{i}]"))?;
                }
            }
            _ => assert_eq!(source, native, "{path} exact observed value"),
        }
        Ok(())
    }
}

pub(crate) fn run(fixture: &Value) -> Result<usize> {
    let ingress: Value = serde_json::from_str(input::SOURCE_PURCHASED_AIR_INPUT_JSON)?;
    assert_eq!(
        member(&ingress, "schema")?,
        "src06-purchased-air-Source-input-only.v1"
    );
    let definitions = member(&ingress, "definitions")?;
    let inputs = member(member(fixture, "named_inputs")?, "purchased_air")?;
    let expected = member(
        member(member(fixture, "native_output")?, "groups")?,
        "purchased_air",
    )?;
    assert_eq!(
        member(expected, "schema")?,
        "src06-purchased-air-full-function-batch-output.v1"
    );
    let factories = array(member(&ingress, "factories")?)?;
    let named = array(member(inputs, "factories")?)?;
    let cases = array(member(expected, "cases")?)?;
    assert_eq!(factories.len(), 4);
    assert_eq!(named.len(), 4);
    assert_eq!(cases.len(), 4);
    let mut calls = 0;
    let mut total_bits = 0;
    for ((definition, named), expected) in factories.iter().zip(named).zip(cases) {
        assert_eq!(member(definition, "name")?, member(named, "name")?);
        assert_eq!(member(definition, "steps")?, member(named, "steps")?);
        assert_eq!(member(expected, "factory")?, member(named, "name")?);
        let factory_name = text(member(definition, "name")?)?;
        let mut source = Box::new(Factory::new(definitions, definition).map_err(|error| {
            invalid(&format!("Source P1 factory {factory_name} phase Factory::new/preparation: {error}"))
        })?);
        let mut ids = Identities::default();
        ids.check(&snapshot(&source)?, member(expected, "setup")?, "setup")?;
        let inputs = array(member(definition, "steps")?)?;
        let actual = array(member(expected, "steps")?)?;
        assert_eq!(inputs.len(), 15);
        assert_eq!(actual.len(), 15);
        for (at, (input, expected)) in inputs.iter().zip(actual).enumerate() {
            assert_eq!(member(input, "name")?, member(expected, "name")?);
            assert_eq!(member(input, "operation")?, member(expected, "operation")?);
            assert_eq!(member(input, "inputs")?, member(expected, "named_inputs")?);
            assert_eq!(count(member(expected, "ordinal")?)?, at + 1);
            let named_inputs = member(input, "inputs")?;
            source.phase_inputs(named_inputs)?;
            if text(member(input, "operation")?)? == "simulate" {
                let prefix = member(expected, "actual_caller_InitZoneEquipment_prefix")?;
                ids.check(
                    &snapshot(&source)?,
                    member(prefix, "entry")?,
                    "InitZoneEquipment.entry",
                )?;
                source.initialize_prefix(
                    object(input)?
                        .get("first_hvac_iteration")
                        .map(boolean)
                        .transpose()?
                        .unwrap_or(false),
                    false,
                )?;
                ids.check(
                    &snapshot(&source)?,
                    member(prefix, "exit")?,
                    "InitZoneEquipment.exit",
                )?;
            } else {
                assert!(member(expected, "actual_caller_InitZoneEquipment_prefix")?.is_null());
            }
            source.caller_inputs(named_inputs)?;
            ids.check(&snapshot(&source)?, member(expected, "entry")?, "entry")?;
            // No expected data crosses into the live factory or input arguments.
            source.call(input)?;
            assert!(member(expected, "exception")?.is_null());
            ids.check(&snapshot(&source)?, member(expected, "exit")?, "exit")?;
            calls += 1;
        }
        total_bits += ids.raw_bits;
    }
    assert_eq!(calls, 60);
    assert!(total_bits > 0);
    println!(
        "PurchasedAir genuine persistent calls={calls}; finite raw-bit observations={total_bits}; four true getter factories; private Psy cache state unobserved"
    );
    Ok(calls)
}

impl OrdinaryEquipment {
    pub(crate) fn run_native_gap_purchased_air_cohort(fixture: &Value) -> Result<usize> {
        run(fixture)
    }
}

mod output_processor;

mod air_history;

mod warmup;

mod multiplier_predictor;
