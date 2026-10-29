//! Real Source getter/manager bootstrap for named zero-surface OtherEquipment.
//! Primary-only ingress; no expected output reads and no group dispatch here.
use crate::heat_balance as hb;
use hb::ctf_initialization_context::internal_heat_gains_input::{
    SourceGainRadiationOwners, SourceGainStepOwners, SourceZoneGenericGainAllocation,
};
use hb::ctf_initialization_context::{CtfInitializationContext, CtfInitializationReports};
use hb::error_backend::{SourceErrorBackend, SourceErrorState, SourceErrorTracking};
use hb::ordinary_air_heat_balance::SourceAirHeatBalance;
use hb::ordinary_equipment::{
    OrdinaryEquipment, OrdinaryEquipmentInput, SourceEquipmentInputOwners,
};
use hb::ordinary_hvac_manager::SourceHvacManager;
use hb::ordinary_surface_initialization::SourceSurfaceHeatBalanceArrays;
use hb::radiation::ordinary::{
    OrdinaryRadiationInput, OrdinarySolarRadiation, RadiantRadiationEnclosure,
};
use hb::selected_gain_modules::{SourceGainModules, SourceModuleGainProvider};
use hb::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use hb::selected_heat_balance_input::surface::SurfaceGeometryInputState;
use hb::selected_heat_balance_input::zone::{SelectedZoneInputOwners, ZoneGetterOwners};
use hb::selected_heat_balance_input::{
    HeatBalanceInputError, SelectedHeatBalanceInput, SelectedHeatBalanceInputState,
};
use hb::selected_input_processor::{SelectedInputProcessor, SourceInputProcessorShortCut};
use hb::selected_project_data::{
    ProjectDataOwners, ProjectInputObject, ProjectInputValue, SelectedProjectData,
    SelectedProjectDataInput,
};
use hb::selected_schedule_manager::{
    SelectedScheduleInput, SelectedScheduleManager, SourceConstantScheduleInput,
    SourceScheduleTypeInput,
};
use hb::source_output_prescan::{SourceOutputPreScan, SourceReportingPreScanInput};
use hb::source_output_registration::{SourceOutputFileHost, SourceOutputRegistration};
use serde_json::{Map, Value, json};
use std::{collections::BTreeMap, io};
mod beam_observe;
mod gains_observe;
mod geometry;
mod producer_calls;
mod surface_gain_observe;
type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;
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
        let unit = match alpha(raw, "unit_type")?.as_deref() {
            None => ep_model::ScheduleUnitType::Invalid,
            Some("DIMENSIONLESS") => ep_model::ScheduleUnitType::Dimensionless,
            _ => return Err(invalid("unrepresented entered schedule unit").into()),
        };
        out.types.push(SourceScheduleTypeInput {
            object_name: name.into(),
            name: name.to_ascii_uppercase(),
            lower: numeric(raw, "lower_limit_value")?,
            upper: numeric(raw, "upper_limit_value")?,
            numeric_type: alpha(raw, "numeric_type")?,
            unit,
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

fn error_backend<'a>(
    state: &'a mut SourceErrorState,
    error: &'a mut Vec<u8>,
    console: &'a mut Vec<u8>,
) -> SourceErrorBackend<'a> {
    SourceErrorBackend {
        state,
        error_stream: Some(error),
        console,
        out_unit1: None,
        out_unit2: None,
        f_message: None,
        message_callback: None,
        error_callback: None,
        sqlite: None,
    }
}
// Same physical owners as production SourceSurfaceGainRadiation. This adapter
// contains field bindings only; no scientific arithmetic or zero-return stub.
struct GainRadiation<'a> {
    owner: &'a mut OrdinarySolarRadiation,
    geometry: &'a SurfaceGeometryInputState,
    arrays: &'a mut SourceSurfaceHeatBalanceArrays,
}
impl SourceGainRadiationOwners for GainRadiation<'_> {
    fn num_radiant_enclosures(&self) -> i32 {
        self.owner.num_radiant_enclosures()
    }
    fn radiant_enclosure_mut(
        &mut self,
        n: i32,
    ) -> std::result::Result<&mut RadiantRadiationEnclosure, HeatBalanceInputError> {
        self.owner
            .source_radiant_enclosures_mut()
            .get_mut(
                n.checked_sub(1)
                    .and_then(|n| usize::try_from(n).ok())
                    .ok_or(HeatBalanceInputError::Unavailable(
                        "positive actual radiant enclosure",
                    ))?,
            )
            .ok_or(HeatBalanceInputError::Unavailable(
                "actual radiant enclosure",
            ))
    }
    fn surface_radiant_enclosure(&self, n: i32) -> std::result::Result<i32, HeatBalanceInputError> {
        self.geometry
            .final_surfaces
            .as_deref()
            .and_then(|r| {
                n.checked_sub(1)
                    .and_then(|i| usize::try_from(i).ok())
                    .and_then(|i| r.get(i))
            })
            .map(|r| r.row.rad_encl_index)
            .ok_or(HeatBalanceInputError::Unavailable(
                "actual physical Surface enclosure",
            ))
    }
    fn surface_thermal_absorptance(
        &self,
        n: i32,
    ) -> std::result::Result<&f64, HeatBalanceInputError> {
        self.arrays
            .surf_abs_thermal_int
            .as_deref()
            .and_then(|r| {
                n.checked_sub(1)
                    .and_then(|i| usize::try_from(i).ok())
                    .and_then(|i| r.get(i))
            })
            .ok_or(HeatBalanceInputError::Unavailable(
                "actual physical thermal absorptance",
            ))
    }
    fn surface_internal_radiant_gain_mut(
        &mut self,
        n: i32,
    ) -> std::result::Result<&mut f64, HeatBalanceInputError> {
        self.arrays
            .surf_qdot_rad_int_gains_in_per_area
            .as_deref_mut()
            .and_then(|r| {
                n.checked_sub(1)
                    .and_then(|i| usize::try_from(i).ok())
                    .and_then(|i| r.get_mut(i))
            })
            .ok_or(HeatBalanceInputError::Unavailable(
                "actual physical radiant gain sink",
            ))
    }
}

pub(super) struct GainFactory {
    pub(super) reports: CtfInitializationReports,
    pub(super) weather: crate::weather::day::WeatherDayState,
    pub(super) air: SourceAirHeatBalance,
    pub(super) modules: SourceGainModules,
    pub(super) radiation: OrdinarySolarRadiation,
    pub(super) geometry: SurfaceGeometryInputState,
    pub(super) arrays: SourceSurfaceHeatBalanceArrays,
    history: Option<hb::ctf_thermal_history::CtfThermalHistoryState>,
    public_ctf: hb::ctf_public_initialization::OrderedCtfPublicInitialization,
    shading_stream: hb::radiation::ordinary::shading_report::SourceShadingStream,
    daylighting: hb::ordinary_daylighting::SourceDaylighting,
    eso: Vec<u8>,
    pub(super) error_state: SourceErrorState,
    pub(super) errors: Vec<u8>,
    console: Vec<u8>,
    eio: Vec<u8>,
    files: Files,
    model: ep_model::TypedModel,
}
impl GainFactory {
    pub(super) fn new(ingress: &Value, name: &str) -> Result<Self> {
        if ingress["schema"] != "src06-Exterior-primary-Source-ingress.v1" {
            return Err(invalid("primary-only ingress schema").into());
        }
        let factory = member(member(ingress, "factories")?, name)?;
        for family in [
            "Output:Variable",
            "Output:Meter",
            "BuildingSurface:Detailed",
            "FenestrationSurface:Detailed",
        ] {
            if !raw_family(factory, family)?.is_empty() {
                return Err(invalid("genuine zero-surface/no-output-request unit domain").into());
            }
        }
        let mut input = make_input(member(ingress, "definitions")?, factory)?;
        let mut shortcut = SourceInputProcessorShortCut::default();
        shortcut.allocate_from_input(&input)?;
        let mut globals = SelectedHeatBalanceGlobals::default();
        let control = SelectedHeatBalanceInputState::default();
        let mut weather = crate::weather::day::WeatherDayState::default();
        let mut project = SelectedProjectData::default();
        let mut startup = hb::selected_simulation_startup::SelectedSimulationStartup::default();
        let report = ep_model::ReportInputs::default();
        let mut prescan = SourceOutputPreScan::default();
        let mut errors = Vec::new();
        let mut console = Vec::new();
        let mut error_state = error_state();
        let mut eio = Vec::new();
        let mut schedules = SelectedScheduleManager::default();
        schedules.init_constant_state(0)?;
        {
            let mut backend = error_backend(&mut error_state, &mut errors, &mut console);
            hb::source_output_prescan::post_input_processing(
                &input,
                &mut shortcut,
                &SourceReportingPreScanInput::default(),
                &mut prescan,
                false,
                &mut backend,
            )?;
            let raw = SelectedProjectDataInput {
                source_is_epjson: false,
                objects: project_objects(
                    factory,
                    &[
                        "Version",
                        "HeatBalanceAlgorithm",
                        "Timestep",
                        "SimulationControl",
                    ],
                )?,
                diagnostics: Vec::new(),
            };
            project.get_project_data(
                &raw,
                &mut ProjectDataOwners {
                    input: &mut input,
                    shortcut: &mut shortcut,
                    globals: &mut globals,
                    weather: &mut weather,
                    hvac: &mut startup.hvac_globals,
                    heat_balance_current_module_object: &mut String::new(),
                },
                &mut backend,
                &mut eio,
            )?;
        }
        let mut files = Files::default();
        let steps = usize::try_from(weather.global.time_steps_in_hour)?;
        {
            let mut backend = error_backend(&mut error_state, &mut errors, &mut console);
            schedules.process_schedule_input_with_backend(
                &mut input,
                &schedule_inputs(factory)?,
                steps,
                &mut files,
                &mut backend,
            )?;
        }
        let model = identity_model(factory, &control, weather.global.time_steps_in_hour)?;
        let mut registration = SourceOutputRegistration::default();
        let mut zones = SelectedZoneInputOwners::default();
        {
            let mut backend = error_backend(&mut error_state, &mut errors, &mut console);
            for kind in [
                hb::source_output_registration::TimeStepKind::Zone,
                hb::source_output_registration::TimeStepKind::System,
            ] {
                registration
                    .time_pointers
                    .setup_time_pointer(kind, &mut backend)?;
            }
        }
        let mut zone_backend = error_backend(&mut error_state, &mut errors, &mut console);
        let mut zone_errors_found = false;
        zones.get_zone_data(
            &SelectedHeatBalanceInput {
                source_is_epjson: false,
                surface: Default::default(),
                objects: project_objects(factory, &["Zone"])?,
            },
            &control,
            &model,
            &mut ZoneGetterOwners {
                input: &mut input,
                shortcut: &mut shortcut,
                globals: &mut globals,
                output: &mut registration,
                prescan: &prescan,
                report_input: &report,
                minimum_report_frequency: &project.system.minimum_report_frequency,
                developer: &project.system.developer_flag,
                clock: Some(&weather.global),
                backend: &mut zone_backend,
                minutes_in_time_step: weather.global.minutes_in_time_step,
                host: &mut files,
                eio: &mut eio,
            },
            &mut zone_errors_found,
        )?;
        let inventory = object(member(factory, "raw")?)?
            .iter()
            .map(|(k, v)| Ok((k.as_str(), object(v)?.len())))
            .collect::<Result<Vec<_>>>()?;
        let radiation_input =
            OrdinaryRadiationInput::from_admitted_inventory(inventory.clone(), false)?;
        let equipment_input =
            OrdinaryEquipmentInput::from_source_family_inventory(inventory, steps)?;
        let mut equipment = OrdinaryEquipment::new_for_source(&model, equipment_input)?;
        let mut reports = CtfInitializationReports::default();
        let mut manager = SourceHvacManager::default();
        reports
            .registry
            .set_predefined_tables(globals.do_zone_sizing, globals.do_system_sizing)?;
        equipment.get_zone_equipment_source(
            &mut input,
            equipment_input,
            false,
            false,
            &globals,
            &mut reports.selected_internal_heat_gains,
            SourceEquipmentInputOwners {
                zones: zones
                    .zones
                    .as_deref_mut()
                    .ok_or("actual getter Zone rows")?,
                schedules: &mut schedules,
                sizing: &mut reports.selected_sizing_manager,
                branch: Some(&mut manager.ancillary.branch_node_connections),
                source_is_epjson: false,
            },
        )?;
        let mut air = SourceAirHeatBalance::default();
        air.allocate_source_arrays(&globals)?;
        reports.heat_balance_input = Some(SelectedHeatBalanceInput {
            source_is_epjson: false,
            surface: Default::default(),
            objects: project_objects(
                factory,
                &[
                    "People",
                    "Lights",
                    "ElectricEquipment",
                    "GasEquipment",
                    "HotWaterEquipment",
                    "SteamEquipment",
                    "OtherEquipment",
                    "ElectricEquipment:ITE:AirCooled",
                    "ZoneBaseboard:OutdoorTemperatureControlled",
                    "ZoneContaminantSourceAndSink:CarbonDioxide",
                ],
            )?,
        });
        reports.selected_input_processor = Some(input);
        reports.input_processor_shortcut = shortcut;
        reports.selected_heat_balance_globals = Some(globals);
        reports.selected_zone_input = Some(zones);
        reports.selected_schedule_manager = Some(schedules);
        reports.selected_project_data = Some(project);
        reports.selected_simulation_startup = Some(startup);
        reports.output_registration = registration;
        reports.output_prescan = prescan;
        reports.source_zone_equipment = Some(equipment);
        reports.facility_electric_service =
            Some(hb::facility_electric_service::FacilityElectricService::new());
        Ok(Self {
            reports,
            weather,
            air,
            modules: SourceGainModules::default(),
            radiation: {
                let mut owner = OrdinarySolarRadiation::for_surface_input(radiation_input);
                owner.allocate_source_solar_enclosure_arrays()?;
                owner
            },
            geometry: SurfaceGeometryInputState::default(),
            arrays: SourceSurfaceHeatBalanceArrays::default(),
            history: None,
            public_ctf: hb::ctf_public_initialization::OrderedCtfPublicInitialization::begin(
                true, 0,
            ),
            shading_stream: Default::default(),
            daylighting: Default::default(),
            eso: Vec::new(),
            error_state,
            errors,
            console,
            eio,
            files,
            model,
        })
    }
    pub(super) fn get_input(&mut self) -> Result {
        let mut context = CtfInitializationContext {
            reports: &mut self.reports,
            eio: &mut self.eio,
            backend: error_backend(&mut self.error_state, &mut self.errors, &mut self.console),
        };
        context.manage_selected_internal_heat_gains_input(
            &self.model,
            self.weather.global.minutes_in_time_step,
            &mut self.files,
        )?;
        Ok(())
    }
    pub(super) fn prepare_gain_clock(&mut self, primary: &Value) -> Result {
        self.weather.global.hour_of_day = int(member(primary, "hour")?)?;
        self.weather.global.time_step = int(member(primary, "timestep")?)?;
        self.weather.global.begin_envrn_flag = boolean(member(primary, "begin_environment")?)?;
        self.weather.environment.day_of_year_schedule = int(member(primary, "day_of_year")?)?;
        self.weather.environment.day_of_week = int(member(primary, "day_of_week")?)?;
        let schedules = self
            .reports
            .selected_schedule_manager
            .as_mut()
            .ok_or("same schedule owner")?;
        schedules.update_source_schedule_values(&self.weather.global, &self.weather.environment)?;
        Ok(())
    }
    pub(super) fn invoke_actual_gains(&mut self) -> Result {
        let schedules = self
            .reports
            .selected_schedule_manager
            .as_mut()
            .ok_or("same schedule owner")?;
        let mut module_provider = SourceModuleGainProvider {
            modules: &mut self.modules,
            input: self
                .reports
                .selected_input_processor
                .as_mut()
                .ok_or("same IP owner")?,
            facility: self.reports.facility_electric_service.as_mut(),
        };
        let mut radiation = GainRadiation {
            owner: &mut self.radiation,
            geometry: &self.geometry,
            arrays: &mut self.arrays,
        };
        let mut backend = error_backend(&mut self.error_state, &mut self.errors, &mut self.console);
        let mut owners = SourceGainStepOwners {
            clock: &self.weather.global,
            globals: self
                .reports
                .selected_heat_balance_globals
                .as_ref()
                .ok_or("same globals owner")?,
            control: &self.reports.heat_balance_input_state,
            zones: self
                .reports
                .selected_zone_input
                .as_mut()
                .ok_or("same Zone/Space owner")?,
            schedules,
            backend: &mut backend,
            modules: &mut module_provider,
            latent: Some(&mut self.air),
            // This existing OtherEquipment-only observer owns no controller.
            room_air: None,
            radiation: Some(&mut radiation),
            zone_generic_gain: SourceZoneGenericGainAllocation::Unallocated,
        };
        self.reports
            .selected_internal_heat_gains
            .manage_timestep(&mut owners)?;
        Ok(())
    }
}
