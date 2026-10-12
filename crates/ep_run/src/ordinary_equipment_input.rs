//! Finite raw admission for the consumed ordinary geometry equipment getter.
//! No Native observations, result names, input hashes or typed-empty inference.
use ep_raw_model::{FieldName, ObjectType, RawModel, RawObject, RawValue};
use ep_runtime::heat_balance::ordinary_equipment::OrdinaryEquipmentInput;

fn number(value: &RawValue) -> Option<f64> {
    match value {
        RawValue::Number(text) => text.parse::<f64>().ok().filter(|n| n.is_finite()),
        _ => None,
    }
}
fn text_is(value: &RawValue, choices: &[&str]) -> bool {
    matches!(value, RawValue::String(text) if choices.iter().any(|choice| text.eq_ignore_ascii_case(choice)))
}
fn blank(value: &RawValue) -> bool {
    matches!(value, RawValue::Null)
        || matches!(value, RawValue::String(text) if text.trim().is_empty())
}
fn field<'a>(object: &'a RawObject, name: &str) -> Option<&'a RawValue> {
    object.fields.get(&FieldName(name.into()))
}
fn text<'a>(object: &'a RawObject, name: &str) -> Option<&'a str> {
    match field(object, name) {
        Some(RawValue::String(text)) if !text.trim().is_empty() => Some(text),
        _ => None,
    }
}
fn selected(object: &RawObject, name: &str, choices: &[&str]) -> bool {
    field(object, name).is_none_or(|value| blank(value) || text_is(value, choices))
}

mod fields;
use fields::{admitted_field, required_fields};

pub(crate) fn resolve(raw: &RawModel) -> Result<OrdinaryEquipmentInput, String> {
    for (family, objects) in &raw.objects {
        if objects.is_empty() {
            continue;
        }
        if !matches!(
            family.0.as_str(),
            "Version"
                | "Timestep"
                | "Building"
                | "HeatBalanceAlgorithm"
                | "SurfaceConvectionAlgorithm:Inside"
                | "SurfaceConvectionAlgorithm:Outside"
                | "SimulationControl"
                | "RunPeriod"
                | "Site:Location"
                | "SizingPeriod:DesignDay"
                | "Material:NoMass"
                | "Material"
                | "Construction"
                | "Zone"
                | "ScheduleTypeLimits"
                | "GlobalGeometryRules"
                | "BuildingSurface:Detailed"
                | "Output:Diagnostics"
                | "Output:VariableDictionary"
                | "Output:Surfaces:Drawing"
                | "Output:Constructions"
                | "Output:Meter:MeterFileOnly"
                | "OutputControl:Table:Style"
                | "Output:Table:SummaryReports"
                | "Exterior:Lights"
                | "OtherEquipment"
                | "Schedule:Constant"
                | "ZoneAirHeatBalanceAlgorithm"
                | "Output:Variable"
        ) {
            return Err(format!(
                "ordinary equipment excludes positive family {}",
                family.0
            ));
        }
        if matches!(
            family.0.as_str(),
            "Version"
                | "Timestep"
                | "Building"
                | "HeatBalanceAlgorithm"
                | "SurfaceConvectionAlgorithm:Inside"
                | "SurfaceConvectionAlgorithm:Outside"
                | "SimulationControl"
                | "RunPeriod"
                | "Site:Location"
                | "Zone"
                | "GlobalGeometryRules"
                | "Output:Diagnostics"
                | "Output:VariableDictionary"
                | "Output:Table:SummaryReports"
                | "OutputControl:Table:Style"
                | "ZoneAirHeatBalanceAlgorithm"
        ) && objects.len() != 1
        {
            return Err(format!("ordinary equipment requires unique {}", family.0));
        }
        for (name, object) in objects {
            if name.0.trim().is_empty() {
                return Err(format!("blank ordinary {} name", family.0));
            }
            for (field, value) in &object.fields {
                if !admitted_field(&family.0, &field.0, value) {
                    return Err(format!(
                        "ordinary equipment field unavailable: {}.{}",
                        family.0, field.0
                    ));
                }
            }
            for &required in required_fields(&family.0) {
                if field(object, required).is_none_or(blank) {
                    return Err(format!("missing {}.{}", family.0, required));
                }
            }
            let allowed = match family.0.as_str() {
                "HeatBalanceAlgorithm" => {
                    selected(object, "algorithm", &["ConductionTransferFunction"])
                        && field(object, "surface_temperature_upper_limit").is_none_or(blank)
                        && field(
                            object,
                            "maximum_surface_convection_heat_transfer_coefficient_value",
                        )
                        .is_none_or(blank)
                }
                "SurfaceConvectionAlgorithm:Inside" => selected(object, "algorithm", &["TARP"]),
                "SurfaceConvectionAlgorithm:Outside" => selected(object, "algorithm", &["DOE-2"]),
                "ZoneAirHeatBalanceAlgorithm" => {
                    selected(object, "algorithm", &["ThirdOrderBackwardDifference"])
                        && selected(object, "do_space_heat_balance_for_sizing", &["No"])
                        && selected(object, "do_space_heat_balance_for_simulation", &["No"])
                }
                "Zone" => {
                    selected(object, "zone_inside_convection_algorithm", &["TARP"])
                        && selected(object, "zone_outside_convection_algorithm", &["DOE-2"])
                }
                "BuildingSurface:Detailed" => {
                    selected(object, "surface_type", &["Wall", "Floor", "Roof"])
                        && selected(
                            object,
                            "outside_boundary_condition",
                            &["Outdoors", "Adiabatic"],
                        )
                        && field(object, "outside_boundary_condition_object").is_none_or(blank)
                        && field(object, "space_name").is_none_or(blank)
                }
                "SizingPeriod:DesignDay" => {
                    selected(
                        object,
                        "dry_bulb_temperature_range_modifier_type",
                        &["DefaultMultipliers"],
                    ) && selected(object, "humidity_condition_type", &["WetBulb", "DewPoint"])
                        && selected(object, "solar_model_indicator", &["ASHRAEClearSky"])
                        && selected(
                            object,
                            "begin_environment_reset_mode",
                            &["FullResetAtBeginEnvironment"],
                        )
                        && field(object, "wetbulb_or_dewpoint_at_maximum_dry_bulb")
                            .and_then(number)
                            .is_some()
                        && [
                            "dry_bulb_temperature_range_modifier_day_schedule_name",
                            "humidity_condition_day_schedule_name",
                            "beam_solar_day_schedule_name",
                            "diffuse_solar_day_schedule_name",
                        ]
                        .iter()
                        .all(|name| field(object, name).is_none_or(blank))
                }
                "Output:Surfaces:Drawing" => {
                    selected(object, "report_type", &["DXF:WireFrame"])
                        && ["report_specifications_1", "report_specifications_2"]
                            .iter()
                            .all(|name| field(object, name).is_none_or(blank))
                }
                "Exterior:Lights" => {
                    selected(
                        object,
                        "control_option",
                        &["ScheduleNameOnly", "AstronomicalClock"],
                    ) && field(object, "design_level")
                        .and_then(number)
                        .is_some_and(|value| value >= 0.0)
                }
                "OtherEquipment" => {
                    selected(object, "fuel_type", &["None"])
                        && selected(
                            object,
                            "design_level_calculation_method",
                            &["EquipmentLevel"],
                        )
                        && field(object, "design_level").and_then(number).is_some()
                        && ["power_per_floor_area", "power_per_person"]
                            .iter()
                            .all(|name| field(object, name).is_none_or(blank))
                        && ["fraction_latent", "fraction_radiant", "fraction_lost"]
                            .iter()
                            .map(|name| field(object, name).and_then(number).unwrap_or(0.0))
                            .sum::<f64>()
                            <= 1.0
                }
                "Output:Variable" => text(object, "variable_name").is_some(),
                "Output:Meter:MeterFileOnly" => text(object, "key_name").is_some(),
                _ => true,
            };
            if !allowed {
                return Err(format!(
                    "ordinary equipment excludes selected {} branch",
                    family.0
                ));
            }
        }
    }
    // A present Timestep object's omitted field uses the real schema default 6.
    // A wholly absent object remains unavailable: Original then defaults to 4,
    // while the compiler currently defaults to 6.
    let single = |family: &str| {
        raw.objects
            .get(&ObjectType(family.into()))
            .filter(|objects| objects.len() == 1)
            .and_then(|objects| objects.values().next())
    };
    let timestep_object =
        single("Timestep").ok_or("ordinary equipment requires one actual Timestep object")?;
    let timestep = field(timestep_object, "number_of_timesteps_per_hour")
        .map_or(Some(6.0), number)
        .filter(|n| n.fract() == 0.0 && (1.0..=60.0).contains(n) && 60 % (*n as usize) == 0)
        .ok_or("ordinary equipment requires a divisor-of-60 Timestep")? as usize;
    if single("Version").and_then(|object| text(object, "version_identifier")) != Some("26.1") {
        return Err("ordinary equipment requires the explicit supported Version 26.1".into());
    }
    let zone_name = raw
        .objects
        .get(&ObjectType("Zone".into()))
        .filter(|objects| objects.len() == 1)
        .and_then(|objects| objects.keys().next())
        .ok_or("ordinary equipment requires one actual Zone")?;
    // Consumer-specific actual Constant schedule references; no schedule-name or
    // case-name list. The compiler still validates type limits and associations.
    for family in ["OtherEquipment", "Exterior:Lights", "Output:Variable"] {
        if let Some(objects) = raw.objects.get(&ObjectType(family.into())) {
            for object in objects.values() {
                if family == "OtherEquipment"
                    && !text(object, "zone_or_zonelist_or_space_or_spacelist_name")
                        .is_some_and(|name| name.eq_ignore_ascii_case(&zone_name.0))
                {
                    return Err("ordinary OtherEquipment requires the direct actual Zone".into());
                }
                let Some(schedule) = text(object, "schedule_name") else {
                    if family == "Output:Variable" {
                        continue;
                    }
                    return Err(format!(
                        "ordinary {family} requires an actual Constant schedule"
                    ));
                };
                let schedule = raw
                    .objects
                    .get(&ObjectType("Schedule:Constant".into()))
                    .and_then(|objects| {
                        objects
                            .iter()
                            .find(|(name, _)| name.0.eq_ignore_ascii_case(schedule))
                    })
                    .map(|(_, object)| object)
                    .ok_or("ordinary schedule reference is not a real Constant input")?;
                // Original Schedule:Constant's absent hourly value is 0.0.
                if !field(schedule, "hourly_value")
                    .filter(|value| !blank(value))
                    .map_or(Some(0.0), number)
                    .is_some_and(|value| value >= 0.0)
                {
                    return Err(format!(
                        "ordinary {family} requires a nonnegative Constant schedule"
                    ));
                }
            }
        }
    }
    OrdinaryEquipmentInput::from_family_inventory(
        raw.objects
            .iter()
            .map(|(family, objects)| (family.0.as_str(), objects.len())),
        timestep,
    )
    .map_err(|error| error.to_string())
}

/// Reuse the one existing complete positive-family/field admission once, then
/// retain actual radiation query counts and selected project diagnostic input.
pub(crate) fn resolve_with_radiation(
    raw: &RawModel,
    advanced_report_variables: bool,
) -> Result<
    (
        OrdinaryEquipmentInput,
        ep_runtime::heat_balance::radiation::ordinary::OrdinaryRadiationInput,
    ),
    String,
> {
    let equipment = resolve(raw)?;
    let radiation = ep_runtime::heat_balance::radiation::ordinary::OrdinaryRadiationInput::from_admitted_inventory(
        raw.objects.iter().map(|(family, objects)| (family.0.as_str(), objects.len())),
        advanced_report_variables,
    ).map_err(|error| error.to_string())?;
    Ok((equipment, radiation))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ep_raw_model::parse_epjson_str;

    #[test]
    fn raw_equipment_admission_preserves_empty_presence_and_rejects_positive_or_field_branches()
    -> Result<(), Box<dyn std::error::Error>> {
        let base = r#""Version":{"v":{"version_identifier":"26.1"}},"Building":{"b":{"solar_distribution":"MinimalShadowing"}},"Timestep":{"t":{"number_of_timesteps_per_hour":4}},"Zone":{"actual zone":{}}"#;
        for (name, addition, accepted) in [
            ("absent queries", "", true),
            (
                "explicit empty connection map",
                r#", "ZoneHVAC:EquipmentConnections":{}"#,
                true,
            ),
            ("explicit empty NodeList map", r#", "NodeList":{}"#, true),
            (
                "unknown empty family",
                r#", "Unimplemented:Empty":{}"#,
                true,
            ),
            (
                "positive connection",
                r#", "ZoneHVAC:EquipmentConnections":{"x":{}}"#,
                false,
            ),
            ("positive NodeList", r#", "NodeList":{"x":{}}"#, false),
            (
                "Space find empty is a reached warning branch",
                r#", "SpaceHVAC:ZoneEquipmentSplitter":{}"#,
                false,
            ),
            (
                "unknown positive output",
                r#", "Output:Unknown":{"x":{}}"#,
                false,
            ),
            (
                "EMS output",
                r#", "Output:EnergyManagementSystem":{"x":{}}"#,
                false,
            ),
            ("node-bearing Lights", r#", "Lights":{"x":{}}"#, false),
            (
                "ITE node branch",
                r#", "ElectricEquipment:ITE:AirCooled":{"x":{}}"#,
                false,
            ),
            (
                "unknown HVAC",
                r#", "ZoneHVAC:SomeNewDevice":{"x":{}}"#,
                false,
            ),
            ("plant", r#", "PlantLoop":{"x":{}}"#, false),
            (
                "AFN",
                r#", "AirflowNetwork:SimulationControl":{"x":{}}"#,
                false,
            ),
            (
                "EMS actuator",
                r#", "EnergyManagementSystem:Actuator":{"x":{}}"#,
                false,
            ),
            (
                "local environment",
                r#", "ZoneProperty:LocalEnvironment":{"x":{}}"#,
                false,
            ),
            (
                "unknown scalar field",
                r#", "Output:Variable":{"x":{"variable_name":"arbitrary variable","node_name":"secret"}}"#,
                false,
            ),
            (
                "unknown nested field",
                r#", "Output:Diagnostics":{"x":{"diagnostics":[{"key":"DisplayAdvancedReportVariables","ignored":1}]}}"#,
                false,
            ),
            (
                "unknown diagnostic branch",
                r#", "Output:Diagnostics":{"x":{"diagnostics":[{"key":"DisplayExtraWarnings"}]}}"#,
                false,
            ),
            (
                "known output names remain inputs",
                r#", "Output:Variable":{"x":{"variable_name":"some future report variable","key_value":"*","reporting_frequency":"Hourly"}}"#,
                true,
            ),
            (
                "numeric operand has wrong type",
                r#", "Material:NoMass":{"x":{"roughness":"Rough","thermal_resistance":"1"}}"#,
                false,
            ),
        ] {
            let raw = parse_epjson_str(&format!("{{{base}{addition}}}"))?;
            assert_eq!(resolve(&raw).is_ok(), accepted, "{name}");
        }
        for (name, input, expected_timestep) in [
            (
                "present empty Timestep uses schema default",
                base.replace("\"number_of_timesteps_per_hour\":4", ""),
                Some(6),
            ),
            (
                "explicit six remains six",
                base.replace(
                    "\"number_of_timesteps_per_hour\":4",
                    "\"number_of_timesteps_per_hour\":6",
                ),
                Some(6),
            ),
            ("explicit four remains four", base.to_owned(), Some(4)),
            (
                "absent Timestep remains unavailable",
                base.replace(
                    "\"Timestep\":{\"t\":{\"number_of_timesteps_per_hour\":4}},",
                    "",
                ),
                None,
            ),
            (
                "missing solar field is not an equipment branch",
                base.replace("\"solar_distribution\":\"MinimalShadowing\"", ""),
                Some(4),
            ),
            (
                "explicit FullExterior is not an equipment branch",
                base.replace("MinimalShadowing", "FullExterior"),
                Some(4),
            ),
        ] {
            let actual = resolve(&parse_epjson_str(&format!("{{{input}}}"))?);
            if let Some(timestep) = expected_timestep {
                assert_eq!(
                    actual?,
                    OrdinaryEquipmentInput::from_family_inventory([], timestep)?,
                    "{name}"
                );
            } else {
                assert!(actual.is_err(), "{name}");
            }
        }
        for timestep in ["0", "7", "61", "4.5", "\"4\"", "null"] {
            let raw = parse_epjson_str(&format!(
                "{{{}}}",
                base.replace(
                    "\"number_of_timesteps_per_hour\":4",
                    &format!("\"number_of_timesteps_per_hour\":{timestep}")
                )
            ))?;
            assert!(resolve(&raw).is_err(), "invalid real Timestep {timestep}");
        }
        assert!(
            resolve(&parse_epjson_str("{}")?).is_err(),
            "no inferred timing or geometry defaults"
        );
        super::design_day_projection::verify_admitted_design_day_projection()?;
        Ok(())
    }
}

#[cfg(test)]
#[path = "ordinary_design_day_input_tests.rs"]
mod design_day_projection;
