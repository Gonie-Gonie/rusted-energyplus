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
mod source_b;
use fields::{admitted_field, required_fields};

pub(crate) fn resolve(raw: &RawModel) -> Result<OrdinaryEquipmentInput, String> {
    resolve_raw(raw, false, false)
}
/// Source A calendar fields reach the same actual getters only on the explicit
/// validated Source lane. Unscoped legacy raw admission is unchanged.
pub(crate) fn resolve_source_a(raw: &RawModel) -> Result<OrdinaryEquipmentInput, String> {
    resolve_raw(raw, false, true)
}
/// SourceB preserves positive family operands for the same actual IP/getters.
pub(crate) fn resolve_source_b(raw: &RawModel) -> Result<OrdinaryEquipmentInput, String> {
    resolve_raw(raw, true, true)
}
// Preserve first general admission then Source-B fallback evaluation and rejection order.
#[allow(clippy::nonminimal_bool)]
fn resolve_raw(
    raw: &RawModel,
    source_b_ingress: bool,
    source_calendar_ingress: bool,
) -> Result<OrdinaryEquipmentInput, String> {
    for (family, objects) in &raw.objects {
        if objects.is_empty() {
            continue;
        }
        let source_b_family = source_b_ingress && source_b::is_family(&family.0);
        let source_compact_family = source_calendar_ingress && family.0 == "Schedule:Compact";
        let source_calendar_family = source_calendar_ingress
            && matches!(
                family.0.as_str(),
                "RunPeriodControl:DaylightSavingTime" | "RunPeriodControl:SpecialDays"
            );
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
                | "Site:GroundTemperature:BuildingSurface"
                | "Site:GroundTemperature:FCfactorMethod"
                | "Site:GroundTemperature:Shallow"
                | "Site:GroundTemperature:Deep"
                | "SizingPeriod:DesignDay"
                | "Material:NoMass"
                | "Material:AirGap"
                | "Material"
                | "Construction"
                | "Zone"
                | "ScheduleTypeLimits"
                | "GlobalGeometryRules"
                | "BuildingSurface:Detailed"
                | "Output:Diagnostics"
                | "Output:VariableDictionary"
                | "Output:Surfaces:Drawing"
                | "Output:Surfaces:List"
                | "Output:Constructions"
                | "Output:Meter"
                | "Output:Meter:MeterFileOnly"
                | "OutputControl:Table:Style"
                | "Output:Table:SummaryReports"
                | "Exterior:Lights"
                | "OtherEquipment"
                | "Schedule:Constant"
                | "ZoneAirHeatBalanceAlgorithm"
                | "Output:Variable"
        ) && !source_b_family
            && !source_calendar_family
            && !source_compact_family
        {
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
                | "RunPeriodControl:DaylightSavingTime"
                | "Site:Location"
                | "Site:GroundTemperature:BuildingSurface"
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
            // These unnamed singletons have no authored Name. Their official
            // epJSON patterns accept arbitrary keys; keep exact IP identity.
            if name.0.trim().is_empty()
                && !matches!(
                    family.0.as_str(),
                    "Version"
                        | "Timestep"
                        | "GlobalGeometryRules"
                        | "RunPeriodControl:DaylightSavingTime"
                        | "Site:GroundTemperature:BuildingSurface"
                        | "SurfaceConvectionAlgorithm:Inside"
                        | "SurfaceConvectionAlgorithm:Outside"
                        | "Output:Surfaces:List"
                        | "Output:Meter"
                        | "Output:Meter:MeterFileOnly"
                )
            {
                return Err(format!("blank ordinary {} name", family.0));
            }
            for (field, value) in &object.fields {
                if !admitted_field(&family.0, &field.0, value)
                    && !((source_b_family || source_compact_family)
                        && source_b::admitted_field(&family.0, &field.0, value))
                {
                    return Err(format!(
                        "ordinary equipment field unavailable: {}.{}",
                        family.0, field.0
                    ));
                }
            }
            // All twelve monthly fields are schema number/default18. IDF
            // omissions are converted/padded separately; direct epJSON null or
            // string is not a blank/default accepted by the official schema.
            if raw.is_epjson() && family.0 == "Site:GroundTemperature:BuildingSurface" {
                for (field, value) in &object.fields {
                    if !matches!(
                        field.0.as_str(),
                        "idf_order" | "idf_max_fields" | "idf_max_extensible_fields"
                    ) && number(value).is_none()
                    {
                        return Err(format!(
                            "{}.{} requires an actual epJSON number",
                            family.0, field.0
                        ));
                    }
                }
            }
            // The optional enum has no epJSON empty-string/null value.
            // IDF blank conversion keeps its separately owned blank semantics.
            if raw.is_epjson()
                && family.0 == "Output:Surfaces:List"
                && field(object, "report_specifications")
                    .is_some_and(|value| !text_is(value, &["IDF"]))
            {
                return Err("Output:Surfaces:List.report_specifications requires IDF when explicitly supplied in epJSON".into());
            }
            for &required in required_fields(&family.0) {
                // Meter keys are required strings but the schema has no
                // minLength: an authored empty key reaches the real not-found warning.
                let required_is_missing = if matches!(
                    family.0.as_str(),
                    "Output:Meter" | "Output:Meter:MeterFileOnly"
                ) && required == "key_name"
                {
                    !matches!(field(object, required), Some(RawValue::String(_)))
                } else {
                    field(object, required).is_none_or(blank)
                };
                if required_is_missing {
                    return Err(format!("missing {}.{}", family.0, required));
                }
            }
            if family.0 == "Version" && field(object, "version_identifier").is_none() {
                return Err(
                    "present Version requires its actual raw version_identifier string".into(),
                );
            }
            if source_b_family {
                for &required in source_b::required_fields(&family.0) {
                    if field(object, required).is_none_or(blank) {
                        return Err(format!("missing {}.{}", family.0, required));
                    }
                }
            }
            let allowed = match family.0.as_str() {
                "HeatBalanceAlgorithm" => {
                    // Numeric domains are checked above; the same Source getter
                    // retains their live limits for CTF diagnostics and convection.
                    selected(object, "algorithm", &["ConductionTransferFunction"])
                }
                "SurfaceConvectionAlgorithm:Inside" => {
                    selected(object, "algorithm", &["Simple", "TARP", "CeilingDiffuser"])
                }
                "SurfaceConvectionAlgorithm:Outside" => selected(
                    object,
                    "algorithm",
                    &["SimpleCombined", "TARP", "MoWiTT", "DOE-2"],
                ),
                "ZoneAirHeatBalanceAlgorithm" => {
                    selected(object, "algorithm", &["ThirdOrderBackwardDifference"])
                        && selected(object, "do_space_heat_balance_for_sizing", &["No"])
                        && selected(object, "do_space_heat_balance_for_simulation", &["No"])
                }
                "Zone" => {
                    // The real GetZoneData writes these same enums into the
                    // owning Zone; both Source dispatchers resolve SetByZone.
                    selected(
                        object,
                        "zone_inside_convection_algorithm",
                        &["Simple", "TARP", "CeilingDiffuser"],
                    ) && selected(
                        object,
                        "zone_outside_convection_algorithm",
                        &["SimpleCombined", "TARP", "MoWiTT", "DOE-2"],
                    )
                }
                "BuildingSurface:Detailed" => {
                    selected(object, "surface_type", &["Wall", "Floor", "Roof"])
                        && selected(
                            object,
                            "outside_boundary_condition",
                            &["Outdoors", "Adiabatic", "Ground"],
                        )
                        && (if text(object, "outside_boundary_condition")
                            .is_some_and(|value| value.eq_ignore_ascii_case("Ground"))
                        {
                            // Original assigns ExtBoundCondName but does not
                            // resolve or consume this string for Ground.
                            field(object, "outside_boundary_condition_object")
                                .is_none_or(|value| matches!(value, RawValue::String(_)))
                        } else {
                            field(object, "outside_boundary_condition_object").is_none_or(blank)
                        })
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
                "Output:Meter" | "Output:Meter:MeterFileOnly" => {
                    matches!(field(object, "key_name"), Some(RawValue::String(_)))
                }
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
    // GetProjectData distinguishes an absent object (4) from its schema field
    // default (6), and normalizes positive non-divisors before building owners.
    let single = |family: &str| {
        raw.objects
            .get(&ObjectType(family.into()))
            .filter(|objects| objects.len() == 1)
            .and_then(|objects| objects.values().next())
    };
    let requested = single("Timestep")
        .map(|object| {
            field(object, "number_of_timesteps_per_hour")
                .map_or(Some(6), |value| match value {
                    RawValue::Number(text) => text.parse::<u32>().ok(),
                    _ => None,
                })
                .filter(|n| (1..=60).contains(n))
                .ok_or("ordinary equipment requires an integer Timestep between 1 and 60")
        })
        .transpose()?;
    let timestep = ep_model::TimestepConfig::from_energyplus_input(requested, false)
        .number_of_timesteps_per_hour as usize;
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
                // ScheduleManager's two real built-in Constant owners exist
                // without authored definitions (ScheduleManager.cc:316-320).
                if family == "Output:Variable"
                    && ["Constant-0.0", "Constant-1.0"]
                        .iter()
                        .any(|name| schedule.eq_ignore_ascii_case(name))
                {
                    continue;
                }
                let constant = raw
                    .objects
                    .get(&ObjectType("Schedule:Constant".into()))
                    .and_then(|objects| {
                        objects
                            .iter()
                            .find(|(name, _)| name.0.eq_ignore_ascii_case(schedule))
                    })
                    .map(|(_, object)| object);
                if constant.is_none()
                    && (source_b_ingress || source_calendar_ingress)
                    && source_b::has_compact_reference(raw, schedule)
                {
                    // All ordered Compact fields remain raw on the same IP.
                    // The actual Source producer populates retained day/week
                    // owners, followed by live currentVal/consumer validation.
                    continue;
                }
                let schedule =
                    constant.ok_or("ordinary schedule reference is not a real Constant input")?;
                // Original Schedule:Constant's absent hourly value is 0.0.
                // OutputProcessor tests zero/nonzero, including negative ON
                // values. Equipment schedules retain their consumer boundary.
                if !field(schedule, "hourly_value")
                    .filter(|value| !blank(value))
                    .map_or(Some(0.0), number)
                    .is_some_and(|value| family == "Output:Variable" || value >= 0.0)
                {
                    return Err(format!(
                        "ordinary {family} requires a finite Constant schedule within its consumer domain"
                    ));
                }
            }
        }
    }
    let inventory = raw
        .objects
        .iter()
        .map(|(family, objects)| (family.0.as_str(), objects.len()));
    if source_b_ingress {
        OrdinaryEquipmentInput::from_source_family_inventory(inventory, timestep)
    } else {
        OrdinaryEquipmentInput::from_family_inventory(inventory, timestep)
    }
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

/// Same genuine radiation counts after Source A calendar raw admission.
pub(crate) fn resolve_with_radiation_source_a(
    raw: &RawModel,
    advanced_report_variables: bool,
) -> Result<
    (
        OrdinaryEquipmentInput,
        ep_runtime::heat_balance::radiation::ordinary::OrdinaryRadiationInput,
    ),
    String,
> {
    let equipment = resolve_source_a(raw)?;
    let radiation = ep_runtime::heat_balance::radiation::ordinary::OrdinaryRadiationInput::from_admitted_inventory(
        raw.objects.iter().map(|(family, objects)| (family.0.as_str(), objects.len())),
        advanced_report_variables,
    ).map_err(|error| error.to_string())?;
    Ok((equipment, radiation))
}

/// Same genuine radiation counts after explicit SourceB raw ingress.
pub(crate) fn resolve_with_radiation_source_b(
    raw: &RawModel,
    advanced_report_variables: bool,
) -> Result<
    (
        OrdinaryEquipmentInput,
        ep_runtime::heat_balance::radiation::ordinary::OrdinaryRadiationInput,
    ),
    String,
> {
    let equipment = resolve_source_b(raw)?;
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
        let converted = r#"{
            "Version":{"Version 1":{"version_identifier":"26.1"}},
            "Timestep":{"Timestep 1":{"number_of_timesteps_per_hour":4}},
            "HeatBalanceAlgorithm":{"HeatBalanceAlgorithm 1":{"algorithm":"ConductionTransferFunction"}},
            "SimulationControl":{"SimulationControl 1":{"run_simulation_for_weather_file_run_periods":"Yes"}},
            "Building":{"b":{"solar_distribution":"MinimalShadowing"}},"Zone":{"z":{}}
        }"#;
        let mut converted = ep_raw_model::parse_epjson_str_with_idf_order(
            converted,
            "Version,26.1;Timestep,4;HeatBalanceAlgorithm,ConductionTransferFunction;SimulationControl,No,No,No,No,Yes;Building,b,,,,,MinimalShadowing;Zone,z;",
        )?;
        assert!(resolve(&converted).is_ok());
        let metadata = converted
            .objects
            .get_mut(&ObjectType("Version".into()))
            .and_then(|rows| rows.get_mut(&ep_raw_model::ObjectName("Version 1".into())))
            .ok_or("actual Version object missing")?;
        metadata.fields.insert(
            FieldName("idf_max_fields".into()),
            RawValue::Number("-1".into()),
        );
        assert!(resolve(&converted).is_err());
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
                "actual built-in output ON schedule",
                r#", "Output:Variable":{"x":{"variable_name":"Site Outdoor Air Drybulb Temperature","schedule_name":"Constant-1.0"}}"#,
                true,
            ),
            (
                "actual built-in output OFF schedule",
                r#", "Output:Variable":{"x":{"variable_name":"Site Outdoor Air Drybulb Temperature","schedule_name":"constant-0.0"}}"#,
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
        for value in [-2.0, 0.0, 3.0] {
            let addition = format!(
                r#", "Schedule:Constant":{{"filter":{{"hourly_value":{value}}}}}, "Output:Variable":{{"v":{{"variable_name":"Site Outdoor Air Drybulb Temperature","schedule_name":"filter"}}}}"#
            );
            let raw = parse_epjson_str(&format!("{{{base}{addition}}}"))?;
            assert!(resolve(&raw).is_ok(), "output filter value {value}");
            let equipment = addition.replace(
                r#""Output:Variable":{"v":{"variable_name":"Site Outdoor Air Drybulb Temperature","schedule_name":"filter"}}"#,
                r#""Exterior:Lights":{"v":{"schedule_name":"filter","design_level":1}}"#,
            );
            let raw = parse_epjson_str(&format!("{{{base}{equipment}}}"))?;
            assert_eq!(
                resolve(&raw).is_ok(),
                value >= 0.0,
                "equipment schedule value {value}"
            );
        }
        let raw = parse_epjson_str(&format!(
            r#"{{{base},"Output:Variable":{{"v":{{"variable_name":"Site Outdoor Air Drybulb Temperature","schedule_name":"missing"}}}}}}"#
        ))?;
        assert!(resolve(&raw).is_err(), "unresolved reporting schedule");
        for (fields, accepted) in [
            (r#""surface_temperature_upper_limit":200"#, true),
            (
                r#""maximum_surface_convection_heat_transfer_coefficient_value":1000"#,
                true,
            ),
            (
                r#""algorithm":"ConductionTransferFunction","surface_temperature_upper_limit":200,"minimum_surface_convection_heat_transfer_coefficient_value":0.1,"maximum_surface_convection_heat_transfer_coefficient_value":1000"#,
                true,
            ),
            (r#""surface_temperature_upper_limit":201"#, true),
            (
                r#""maximum_surface_convection_heat_transfer_coefficient_value":1001"#,
                true,
            ),
            (r#""surface_temperature_upper_limit":"200""#, false),
            (
                r#""maximum_surface_convection_heat_transfer_coefficient_value":"1000""#,
                false,
            ),
            (
                r#""surface_temperature_upper_limit":null,"maximum_surface_convection_heat_transfer_coefficient_value":" ""#,
                false,
            ),
            (r#""algorithm":"ConductionFiniteDifference""#, false),
            (r#""unknown_high_limit":1000"#, false),
        ] {
            let raw = parse_epjson_str(&format!(
                "{{{base},\"HeatBalanceAlgorithm\":{{\"h\":{{{fields}}}}}}}"
            ))?;
            assert_eq!(resolve(&raw).is_ok(), accepted, "HB fields {fields}");
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
                "absent Timestep uses Original non-CondFD default",
                base.replace(
                    "\"Timestep\":{\"t\":{\"number_of_timesteps_per_hour\":4}},",
                    "",
                ),
                Some(4),
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
        for timestep in ["0", "61", "4.5", "\"4\"", "null"] {
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
        crate::ordinary_input_processor::tests::verify_shared_input_boundary()?;
        Ok(())
    }
}

#[cfg(test)]
#[path = "ordinary_design_day_input_tests.rs"]
mod design_day_projection;

#[cfg(test)]
mod source_surface_list_ingress_tests {
    use super::*;

    fn project_with_report(fields: serde_json::Value) -> serde_json::Value {
        serde_json::json!({
            "Version": {"v": {"version_identifier": "26.1"}},
            "Building": {"b": {"solar_distribution": "MinimalShadowing"}},
            "Timestep": {"t": {"number_of_timesteps_per_hour": 4}},
            "Zone": {"actual zone": {}},
            "Output:Surfaces:List": {"": fields}
        })
    }

    #[test]
    fn source_opaque_surface_list_keeps_raw_identity_and_requires_real_report_branches()
    -> Result<(), Box<dyn std::error::Error>> {
        for report in [
            "Lines",
            "Vertices",
            "Details",
            "DetailsWithVertices",
            "CostInfo",
            "ViewFactorInfo",
        ] {
            for specification in [None, Some("IDF")] {
                let mut fields = serde_json::json!({"report_type": report});
                if let Some(value) = specification {
                    fields["report_specifications"] = value.into();
                }
                let raw = ep_raw_model::parse_epjson_str(&project_with_report(fields).to_string())?;
                resolve(&raw)?;
                // Requires the companion exact2alpha schema patch: actual IP
                // shape/identity and typed report source must remain associated.
                let input = crate::ordinary_input_processor::resolve(&raw)?;
                assert!(input.has_object("Output:Surfaces:List", ""));
                let compiled = ep_compiler::compile_raw_model(&raw);
                let [request] = compiled
                    .model
                    .as_ref()
                    .ok_or("report compiler rejected valid source")?
                    .report_inputs
                    .objects
                    .as_slice()
                else {
                    return Err("one typed report request required".into());
                };
                assert_eq!(request.object_name, "");
                assert_eq!(request.alphas[0], report.to_ascii_uppercase());
                assert_eq!(request.alphas[1], specification.unwrap_or(""));
            }
        }
        for fields in [
            serde_json::json!({}),
            serde_json::json!({"report_type": true}),
            serde_json::json!({"report_type": "DecayCurvesFromComponentLoadsSummary"}),
            serde_json::json!({"report_type": "Details", "report_specifications": null}),
            serde_json::json!({"report_type": "Details", "report_specifications": ""}),
            serde_json::json!({"report_type": "Details", "report_specifications": "other"}),
            serde_json::json!({"report_type": "Details", "unknown": 1}),
        ] {
            let raw = ep_raw_model::parse_epjson_str(&project_with_report(fields).to_string())?;
            let error = resolve(&raw).expect_err("unsupported report input accepted");
            assert!(error.contains("Output:Surfaces:List"), "{error}");
        }
        Ok(())
    }
}

#[cfg(test)]
mod meter_domain_tests {
    use super::*;
    fn project(family: &str, fields: serde_json::Value, name: &str) -> serde_json::Value {
        serde_json::json!({"Version":{"v":{"version_identifier":"26.1"}},
            "Building":{"b":{"solar_distribution":"MinimalShadowing"}},
            "Timestep":{"t":{"number_of_timesteps_per_hour":4}}, "Zone":{"actual zone":{}},
            (family): {(name): fields}})
    }

    #[test]
    fn source_normal_meter_ingress_preserves_schema_strings_and_raw_ip_identity()
    -> Result<(), Box<dyn std::error::Error>> {
        for family in ["Output:Meter", "Output:Meter:MeterFileOnly"] {
            for frequency in [
                None,
                Some(""),
                Some("Detailed"),
                Some("Timestep"),
                Some("Hourly"),
                Some("Daily"),
                Some("Monthly"),
                Some("RunPeriod"),
                Some("Environment"),
                Some("Annual"),
            ] {
                // The official unnamed pattern is .*; the blank epJSON key and
                // empty required key_name string are genuine preserved inputs.
                let mut fields = serde_json::json!({"key_name": ""});
                if let Some(value) = frequency {
                    fields["reporting_frequency"] = value.into();
                }
                let raw = ep_raw_model::parse_epjson_str(&project(family, fields, "").to_string())?;
                resolve(&raw)?;
                let input = crate::ordinary_input_processor::resolve(&raw)?;
                let projected = crate::ordinary_heat_balance_input::resolve(&raw, &input)?;
                let [request] = projected
                    .objects
                    .get(family)
                    .ok_or("meter projection absent")?
                    .as_slice()
                else {
                    return Err("one exact meter projection required".into());
                };
                assert_eq!(input.get_num_objects_found(family), 1);
                assert_eq!(request.object_name, "");
                assert_eq!(
                    request.fields.get("key_name"),
                    Some(
                        &ep_runtime::heat_balance::selected_project_data::ProjectInputValue::String(
                            String::new()
                        )
                    )
                );
                assert_eq!(request.fields.get("reporting_frequency"), frequency.map(|value| ep_runtime::heat_balance::selected_project_data::ProjectInputValue::String(value.into())).as_ref());
            }
            for fields in [
                serde_json::json!({}),
                serde_json::json!({"key_name": null}),
                serde_json::json!({"key_name": 1}),
                serde_json::json!({"key_name": true}),
                serde_json::json!({"key_name": "Electricity:*", "reporting_frequency": null}),
                serde_json::json!({"key_name": "Electricity:*", "reporting_frequency": 1}),
                serde_json::json!({"key_name": "Electricity:*", "reporting_frequency": "Weekly"}),
                serde_json::json!({"key_name": "Electricity:*", "unexpected": "field"}),
            ] {
                let raw = ep_raw_model::parse_epjson_str(
                    &project(family, fields, "request").to_string(),
                )?;
                assert!(
                    resolve(&raw).is_err(),
                    "wrong scalar/field admitted for {family}"
                );
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod source_a_compact_ingress {
    use super::*;
    #[test]
    fn source_a_compact_keeps_real_ordered_ip_operands_and_legacy_boundary()
    -> Result<(), Box<dyn std::error::Error>> {
        let input = serde_json::json!({"Version":{"v":{"version_identifier":"26.1"}},"Building":{"b":{}},
            "Timestep":{"t":{"number_of_timesteps_per_hour":4}},"Zone":{"actual zone":{}},
            "ScheduleTypeLimits":{"Fraction":{"lower_limit_value":0,"upper_limit_value":1,"numeric_type":"Continuous"}},
            "Schedule:Compact":{"variable schedule":{"schedule_type_limits_name":"Fraction","data":[
                {"field":"Through: 12/31"},{"field":"For: AllDays"},{"field":"Interpolate: Average"},
                {"field":"Until: 00:20"},{"field":0},{"field":"Until: 24:00"},{"field":1}]}},
            "Output:Variable":{"v":{"key_value":"Environment","variable_name":"Site Outdoor Air Drybulb Temperature","reporting_frequency":"Timestep","schedule_name":"variable schedule"}}});
        let raw = ep_raw_model::parse_epjson_str(&input.to_string())?;
        assert!(resolve(&raw).is_err()); // No new unscoped legacy route.
        resolve_source_a(&raw)?;
        let ip = crate::ordinary_input_processor::resolve_source_a(&raw)?;
        assert_eq!(ip.get_num_objects_found("Schedule:Compact"), 1);
        assert!(ip.has_object("Schedule:Compact", "variable schedule"));
        let fields = ip.source_raw_object_fields("Schedule:Compact", "variable schedule")?;
        let ep_runtime::heat_balance::selected_input_processor::SourceInputStatisticsValue::Array(
            data,
        ) = &fields["data"]
        else {
            return Err("real raw Compact array lost".into());
        };
        assert_eq!(data.len(), 7);
        assert_eq!(
            ip.source_raw_object_names("Schedule:Compact")?,
            ["variable schedule"]
        );
        let ep_runtime::heat_balance::selected_input_processor::SourceInputStatisticsValue::Object(
            value,
        ) = &data[4]
        else {
            return Err("actual Compact numeric member lost".into());
        };
        assert_eq!(
            value["field"],
            ep_runtime::heat_balance::selected_input_processor::SourceInputStatisticsValue::Number(
                "0".into()
            )
        );
        assert_eq!(
            crate::ordinary_schedule_input::resolve(&raw)?.types.len(),
            1
        );
        // Native items permit optionalfield plus arbitrary other members. The
        // raw IP keeps that exact present Object and still advances one alpha.
        for entry in [
            serde_json::json!({}),
            serde_json::json!({"unknown":{"kept":null}}),
            serde_json::json!({"field":"0E"}),
        ] {
            let mut value = input.clone();
            value["Schedule:Compact"]["variable schedule"]["data"][4] = entry;
            let raw = ep_raw_model::parse_epjson_str(&value.to_string())?;
            resolve_source_a(&raw)?;
            let ip = crate::ordinary_input_processor::resolve_source_a(&raw)?;
            let fields = ip.source_raw_object_fields("Schedule:Compact", "variable schedule")?;
            let ep_runtime::heat_balance::selected_input_processor::SourceInputStatisticsValue::Array(data) = &fields["data"] else {
                return Err("actual Compact extensible count lost".into());
            };
            assert_eq!(data.len(), 7);
            let ep_runtime::heat_balance::selected_input_processor::SourceInputStatisticsValue::Object(entry) = &data[4] else {
                return Err("actual blank/string item custody lost".into());
            };
            assert_eq!(
                entry.contains_key("field"),
                value["Schedule:Compact"]["variable schedule"]["data"][4]
                    .get("field")
                    .is_some()
            );
        }
        for bad in [
            serde_json::json!({"field":true}),
            serde_json::json!({"field":null}),
            serde_json::json!(null),
        ] {
            let mut value = input.clone();
            value["Schedule:Compact"]["variable schedule"]["data"][0] = bad;
            let raw = ep_raw_model::parse_epjson_str(&value.to_string())?;
            assert!(resolve_source_a(&raw).is_err());
        }
        Ok(())
    }
}
