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
mod people;
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
        let source_people_family = source_calendar_ingress && family.0 == "People";
        let source_angle_family = source_calendar_ingress && family.0 == "ComfortViewFactorAngles";
        let source_compact_family = source_calendar_ingress
            && matches!(
                family.0.as_str(),
                "Schedule:Compact"
                    | "Output:Schedules"
                    | "Schedule:Day:Hourly"
                    | "Schedule:Day:Interval"
                    | "Schedule:Day:List"
                    | "Schedule:Week:Compact"
                    | "Schedule:Week:Daily"
                    | "Schedule:Year"
            );
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
            && !source_people_family
            && !source_angle_family
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
                        | "Output:Schedules"
                        | "Output:Meter"
                        | "Output:Meter:MeterFileOnly"
                )
                && !source_angle_family
            {
                return Err(format!("blank ordinary {} name", family.0));
            }
            for (field, value) in &object.fields {
                if !admitted_field(&family.0, &field.0, value)
                    && !((source_b_family || source_compact_family)
                        && source_b::admitted_field(&family.0, &field.0, value))
                    && !(source_people_family && people::admitted_field(&field.0, value))
                    && !(source_angle_family && people::admitted_angle_field(&field.0, value))
                {
                    // The connected schedule and OtherEquipment instance schemas
                    // allow additional members. Only an unconsumed top-level member
                    // may pass here, after both existing operand validators.
                    // The three actual IDF metadata predicates stay unchanged.
                    let metadata = source_calendar_ingress
                        && !matches!(
                            field.0.as_str(),
                            "idf_order" | "idf_max_fields" | "idf_max_extensible_fields"
                        )
                        && match family.0.as_str() {
                            "People" if source_people_family => !people::known_field(&field.0),
                            "ScheduleTypeLimits" => !matches!(
                                field.0.as_str(),
                                "lower_limit_value"
                                    | "upper_limit_value"
                                    | "numeric_type"
                                    | "unit_type"
                            ),
                            "Schedule:Constant" => !matches!(
                                field.0.as_str(),
                                "schedule_type_limits_name" | "hourly_value"
                            ),
                            "Schedule:Compact" => {
                                !matches!(field.0.as_str(), "schedule_type_limits_name" | "data")
                            }
                            "Schedule:Year" => !matches!(
                                field.0.as_str(),
                                "schedule_type_limits_name" | "schedule_weeks"
                            ),
                            "Schedule:Day:Hourly" => {
                                field.0 != "schedule_type_limits_name"
                                    && !field
                                        .0
                                        .strip_prefix("hour_")
                                        .and_then(|suffix| suffix.parse::<u32>().ok())
                                        .is_some_and(|hour| {
                                            (1..=24).contains(&hour)
                                                && field.0 == format!("hour_{hour}")
                                        })
                            }
                            "Schedule:Day:Interval" => !matches!(
                                field.0.as_str(),
                                "schedule_type_limits_name" | "interpolate_to_timestep" | "data"
                            ),
                            "Schedule:Day:List" => !matches!(
                                field.0.as_str(),
                                "schedule_type_limits_name"
                                    | "interpolate_to_timestep"
                                    | "minutes_per_item"
                                    | "extensions"
                            ),
                            "Schedule:Week:Daily" => !matches!(
                                field.0.as_str(),
                                "sunday_schedule_day_name"
                                    | "monday_schedule_day_name"
                                    | "tuesday_schedule_day_name"
                                    | "wednesday_schedule_day_name"
                                    | "thursday_schedule_day_name"
                                    | "friday_schedule_day_name"
                                    | "saturday_schedule_day_name"
                                    | "holiday_schedule_day_name"
                                    | "summerdesignday_schedule_day_name"
                                    | "winterdesignday_schedule_day_name"
                                    | "customday1_schedule_day_name"
                                    | "customday2_schedule_day_name"
                            ),
                            "Schedule:Week:Compact" => field.0 != "data",
                            "OtherEquipment" => !matches!(
                                field.0.as_str(),
                                "fuel_type"
                                    | "zone_or_zonelist_or_space_or_spacelist_name"
                                    | "schedule_name"
                                    | "design_level_calculation_method"
                                    | "design_level"
                                    | "power_per_floor_area"
                                    | "power_per_person"
                                    | "fraction_latent"
                                    | "fraction_radiant"
                                    | "fraction_lost"
                                    | "carbon_dioxide_generation_rate"
                                    | "end_use_subcategory"
                            ),
                            _ => false,
                        };
                    if !metadata {
                        return Err(format!(
                            "ordinary equipment field unavailable: {}.{}",
                            family.0, field.0
                        ));
                    }
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
            if source_b_family || source_compact_family {
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
                "People" => source_people_family && people::admitted_object(raw, object),
                "OtherEquipment" => {
                    (selected(object, "fuel_type", &["None"])
                        || source_calendar_ingress
                            && text_is(
                                field(object, "fuel_type").unwrap_or(&RawValue::Null),
                                &[
                                    "Electricity",
                                    "NaturalGas",
                                    "Gasoline",
                                    "Diesel",
                                    "Coal",
                                    "Propane",
                                    "FuelOilNo1",
                                    "FuelOilNo2",
                                    "OtherFuel1",
                                    "OtherFuel2",
                                    "DistrictCooling",
                                    "DistrictHeatingWater",
                                    "DistrictHeatingSteam",
                                ],
                            ))
                        && (if selected(
                            object,
                            "design_level_calculation_method",
                            &["EquipmentLevel"],
                        ) {
                            if source_calendar_ingress {
                                // Missing Numeric1 is a genuine schema blank;
                                // finite Numeric2/3 are retained but not selected.
                                ["design_level", "power_per_floor_area", "power_per_person"]
                                    .iter()
                                    .all(|name| {
                                        field(object, name)
                                            .is_none_or(|value| number(value).is_some())
                                    })
                            } else {
                                // Keep the original unscoped legacy boundary.
                                field(object, "design_level").and_then(number).is_some()
                                    && ["power_per_floor_area", "power_per_person"]
                                        .iter()
                                        .all(|name| field(object, name).is_none_or(blank))
                            }
                        } else {
                            source_calendar_ingress
                                && (if text_is(
                                    field(object, "design_level_calculation_method")
                                        .unwrap_or(&RawValue::Null),
                                    &["Watts/Area", "Power/Area"],
                                ) {
                                    // Missing selected fields reach the real blank
                                    // child; explicit null/string is not a number.
                                    field(object, "power_per_floor_area")
                                        .is_none_or(|value| number(value).is_some_and(|n| n >= 0.0))
                                        && field(object, "power_per_person")
                                            .is_none_or(|value| number(value).is_some())
                                } else if text_is(
                                    field(object, "design_level_calculation_method")
                                        .unwrap_or(&RawValue::Null),
                                    &["Watts/Person", "Power/Person"],
                                ) {
                                    field(object, "power_per_person")
                                        .is_none_or(|value| number(value).is_some_and(|n| n >= 0.0))
                                        && ["design_level", "power_per_floor_area"].iter().all(
                                            |name| {
                                                field(object, name)
                                                    .is_none_or(|value| number(value).is_some())
                                            },
                                        )
                                } else {
                                    false
                                })
                        })
                        && if source_calendar_ingress {
                            let names = ["fraction_latent", "fraction_radiant", "fraction_lost"];
                            let schema_numbers = names.iter().all(|name| {
                                field(object, name).is_none_or(|value| number(value).is_some())
                            });
                            let [latent, radiant, lost] = names
                                .map(|name| field(object, name).and_then(number).unwrap_or(0.0));
                            let mut convected = 1.0 - (latent + radiant + lost);
                            if convected.abs() <= 0.001 {
                                convected = 0.0;
                            }
                            schema_numbers && convected >= 0.0
                        } else {
                            ["fraction_latent", "fraction_radiant", "fraction_lost"]
                                .iter()
                                .map(|name| field(object, name).and_then(number).unwrap_or(0.0))
                                .sum::<f64>()
                                <= 1.0
                        }
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
                    && source_b::has_source_detailed_reference(raw, schedule)
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
        // The Source-only regular family chain transfers raw numbers/order on
        // A as well as B; the unscoped legacy resolver remains unchanged.
        let mut regular = input.clone();
        regular
            .as_object_mut()
            .ok_or("actual raw model")?
            .remove("Schedule:Compact");
        regular["Schedule:Day:Hourly"] =
            serde_json::json!({"H":{"schedule_type_limits_name":"Fraction","hour_7":1}});
        let week = [
            "sunday_schedule_day_name",
            "monday_schedule_day_name",
            "tuesday_schedule_day_name",
            "wednesday_schedule_day_name",
            "thursday_schedule_day_name",
            "friday_schedule_day_name",
            "saturday_schedule_day_name",
            "holiday_schedule_day_name",
            "summerdesignday_schedule_day_name",
            "winterdesignday_schedule_day_name",
            "customday1_schedule_day_name",
            "customday2_schedule_day_name",
        ]
        .into_iter()
        .map(|field| (field.to_owned(), serde_json::json!("H")))
        .collect::<serde_json::Map<_, _>>();
        regular["Schedule:Week:Daily"] = serde_json::json!({"W":week});
        regular["Schedule:Year"] = serde_json::json!({"variable schedule":{"schedule_type_limits_name":"Fraction","schedule_weeks":[{"schedule_week_name":"W","start_month":1,"start_day":1,"end_month":12,"end_day":31}]}});
        let raw = ep_raw_model::parse_epjson_str(&regular.to_string())?;
        assert!(resolve(&raw).is_err());
        resolve_source_a(&raw)?;
        resolve_source_b(&raw)?;
        let ip = crate::ordinary_input_processor::resolve_source_a(&raw)?;
        assert!(
            matches!(ip.source_raw_object_fields("Schedule:Day:Hourly","H")?.get("hour_7"),Some(ep_runtime::heat_balance::selected_input_processor::SourceInputStatisticsValue::Number(text)) if text=="1")
        );
        assert_eq!(
            ip.source_raw_object_names("Schedule:Year")?,
            ["variable schedule"]
        );
        let mut optional = regular.clone();
        optional["Schedule:Day:Hourly"]["H"]["schedule_type_limits_name"] =
            serde_json::json!("unknown type");
        optional["Schedule:Year"]["variable schedule"]["schedule_type_limits_name"] =
            serde_json::json!("unknown type");
        let compiled =
            ep_compiler::compile_raw_model(&ep_raw_model::parse_epjson_str(&optional.to_string())?);
        assert!(!compiled.has_errors(), "{:?}", compiled.report.diagnostics);
        let typed = compiled.model.ok_or("actual optional-type regular model")?;
        assert!(typed.day_schedules[0].schedule_type_limits.is_none());
        assert!(typed.year_schedules[0].schedule_type_limits.is_none());
        for value in [serde_json::json!(null), serde_json::json!("1")] {
            let mut invalid = regular.clone();
            invalid["Schedule:Day:Hourly"]["H"]["hour_7"] = value;
            assert!(
                resolve_source_a(&ep_raw_model::parse_epjson_str(&invalid.to_string())?).is_err()
            );
        }
        // Both further Day/Week families retain genuine raw values/default
        // omissions on A/B; the legacy lane remains unchanged. Item metadata
        // does not manufacture a value when the declared numeric field is absent.
        let mut further = regular.clone();
        further
            .as_object_mut()
            .ok_or("actual schedule map")?
            .remove("Schedule:Day:Hourly");
        further
            .as_object_mut()
            .ok_or("actual schedule map")?
            .remove("Schedule:Week:Daily");
        further["Schedule:Day:Interval"] = serde_json::json!({"I":{"schedule_type_limits_name":"Fraction","data":[{"time":"06:00","metadata":true},{"time":"18:00","value_until_time":1},{"time":"24:00","value_until_time":0}]}});
        let list = (0..24)
            .map(|hour| {
                if hour == 0 {
                    serde_json::json!({"metadata":true})
                } else {
                    serde_json::json!({"value":if hour < 3 {0} else {1}})
                }
            })
            .collect::<Vec<_>>();
        further["Schedule:Day:List"] = serde_json::json!({"L":{"schedule_type_limits_name":"Fraction","minutes_per_item":60,"extensions":list}});
        further["Schedule:Week:Compact"] = serde_json::json!({"WI":{"data":[{"daytype_list":"AllDays","schedule_day_name":"I"}]},"WL":{"data":[{"daytype_list":"AllDays","schedule_day_name":"L"}]}});
        further["Schedule:Year"]["variable schedule"]["schedule_weeks"][0]["schedule_week_name"] =
            serde_json::json!("WI");
        further["Schedule:Year"]["list schedule"] = serde_json::json!({"schedule_type_limits_name":"Fraction","schedule_weeks":[{"schedule_week_name":"WL","start_month":1,"start_day":1,"end_month":12,"end_day":31}]});
        let raw = ep_raw_model::parse_epjson_str(&further.to_string())?;
        assert!(resolve(&raw).is_err());
        resolve_source_a(&raw)?;
        resolve_source_b(&raw)?;
        let ip = crate::ordinary_input_processor::resolve_source_a(&raw)?;
        use ep_runtime::heat_balance::selected_input_processor::SourceInputStatisticsValue as V;
        let Some(V::Array(interval)) = ip
            .source_raw_object_fields("Schedule:Day:Interval", "I")?
            .get("data")
        else {
            return Err("actual Interval raw data".into());
        };
        assert!(
            matches!(&interval[1], V::Object(fields) if matches!(fields.get("value_until_time"),Some(V::Number(text)) if text=="1"))
        );
        assert!(
            matches!(&interval[0], V::Object(fields) if !fields.contains_key("value_until_time") && fields.get("metadata")==Some(&V::Bool(true)))
        );
        let Some(V::Array(list)) = ip
            .source_raw_object_fields("Schedule:Day:List", "L")?
            .get("extensions")
        else {
            return Err("actual List raw values".into());
        };
        assert_eq!(list.len(), 24);
        assert!(
            matches!(&list[0], V::Object(fields) if !fields.contains_key("value") && fields.get("metadata")==Some(&V::Bool(true)))
        );
        assert_eq!(
            ip.source_raw_object_names("Schedule:Week:Compact")?,
            ["WI", "WL"]
        );
        for mode in ["", "No", "Average", "Linear"] {
            let mut modes = further.clone();
            modes["Schedule:Day:Interval"]["I"]["interpolate_to_timestep"] = mode.into();
            modes["Schedule:Day:List"]["L"]["interpolate_to_timestep"] = mode.into();
            let raw = ep_raw_model::parse_epjson_str(&modes.to_string())?;
            resolve_source_a(&raw)?;
            resolve_source_b(&raw)?;
        }
        for (family, name, field, value) in [
            (
                "Schedule:Day:Interval",
                "I",
                "interpolate_to_timestep",
                serde_json::json!(null),
            ),
            (
                "Schedule:Day:Interval",
                "I",
                "data",
                serde_json::json!([null]),
            ),
            (
                "Schedule:Day:Interval",
                "I",
                "data",
                serde_json::json!([{"time":null,"value_until_time":0}]),
            ),
            (
                "Schedule:Day:Interval",
                "I",
                "data",
                serde_json::json!([{"time":"24:00","value_until_time":"1"}]),
            ),
            (
                "Schedule:Day:List",
                "L",
                "minutes_per_item",
                serde_json::json!(1.5),
            ),
            (
                "Schedule:Day:List",
                "L",
                "minutes_per_item",
                serde_json::json!("60"),
            ),
            (
                "Schedule:Day:List",
                "L",
                "extensions",
                serde_json::json!([{"value":null}]),
            ),
            (
                "Schedule:Week:Compact",
                "WI",
                "data",
                serde_json::json!([{"daytype_list":"AllDays","schedule_day_name":null}]),
            ),
        ] {
            let mut invalid = further.clone();
            invalid[family][name][field] = value;
            assert!(
                resolve_source_a(&ep_raw_model::parse_epjson_str(&invalid.to_string())?).is_err(),
                "invalid Source schedule scalar {family}.{field}"
            );
        }
        // Output:Schedules has no Name field or nonblank outer-key rule.
        // Scope-only Source admission retains its actual report operands;
        // legacy resolution stays blocked and no typed object is invented.
        let mut reports = input.clone();
        reports["Output:Schedules"] = serde_json::json!({"":{"key_field":"Hourly","user_metadata":true},"   ":{"key_field":"Timestep"},"duplicate":{"key_field":"Hourly"}});
        let raw = ep_raw_model::parse_epjson_str(&reports.to_string())?;
        assert!(resolve(&raw).is_err());
        resolve_source_a(&raw)?;
        resolve_source_b(&raw)?;
        let ip = crate::ordinary_input_processor::resolve_source_a(&raw)?;
        assert_eq!(
            ip.source_raw_object_names("Output:Schedules")?,
            ["", "   ", "duplicate"]
        );
        assert_eq!(
            ip.source_raw_object_fields("Output:Schedules", "   ")?
                .get("key_field"),
            Some(&V::String("Timestep".into()))
        );
        assert_eq!(
            ip.source_raw_object_fields("Output:Schedules", "")?
                .get("user_metadata"),
            Some(&V::Bool(true))
        );
        for value in [
            serde_json::json!(null),
            serde_json::json!(1),
            serde_json::json!(""),
            serde_json::json!("All"),
            serde_json::json!(" Hourly "),
        ] {
            let mut invalid = reports.clone();
            invalid["Output:Schedules"][""]["key_field"] = value;
            let raw = ep_raw_model::parse_epjson_str(&invalid.to_string())?;
            assert!(resolve_source_a(&raw).is_err());
            assert!(resolve_source_b(&raw).is_err());
        }
        let mut missing = reports.clone();
        missing["Output:Schedules"][""] = serde_json::json!({});
        assert!(resolve_source_a(&ep_raw_model::parse_epjson_str(&missing.to_string())?).is_err());
        // Actual frontend TypeLimits/Constant projection follows Native's
        // JSON integer->signed-i64->Real64, retaining raw values for schema.
        let mut unsigned = reports.clone();
        unsigned["ScheduleTypeLimits"]["Wrapped"] = serde_json::json!({
            "lower_limit_value":u64::MAX,"upper_limit_value":200,"numeric_type":"Continuous"
        });
        unsigned["Schedule:Constant"]["Wrapped Constant"] = serde_json::json!({
            "schedule_type_limits_name":"Wrapped","hourly_value":u64::MAX
        });
        let raw = ep_raw_model::parse_epjson_str(&unsigned.to_string())?;
        resolve_source_a(&raw)?;
        let projected = crate::ordinary_schedule_input::resolve(&raw)?;
        let limits = projected
            .types
            .iter()
            .find(|row| row.name == "WRAPPED")
            .ok_or("actual wrapped type")?;
        assert_eq!((limits.lower, limits.upper), (Some(-1.0), Some(200.0)));
        assert_eq!(
            projected
                .constants
                .iter()
                .find(|row| row.name == "WRAPPED CONSTANT")
                .ok_or("actual wrapped Constant")?
                .value,
            -1.0
        );
        unsigned["Schedule:Constant"]["Wrapped Constant"]["hourly_value"] =
            serde_json::json!(18446744073709551616.0_f64);
        let raw = ep_raw_model::parse_epjson_str(&unsigned.to_string())?;
        let projected = crate::ordinary_schedule_input::resolve(&raw)?;
        assert_eq!(
            projected
                .constants
                .iter()
                .find(|row| row.name == "WRAPPED CONSTANT")
                .ok_or("actual float Constant")?
                .value,
            18446744073709551616.0
        );
        // Direct literal text retains numeric kind through actual RawModel and
        // Source A/B IP. A serde Value roundtrip here would hide bare -0.
        for (operand, expected, integer) in [
            ("-0", 0.0_f64, true),
            ("-0.0", -0.0, false),
            ("-0e0", -0.0, false),
            ("1e-9999", 0.0, false),
            ("-1e-9999", -0.0, false),
            ("1E+00", 1.0, false),
            ("18446744073709551615", -1.0, true),
        ] {
            let source = format!(
                r#"{{
                "Version":{{"v":{{"version_identifier":"26.1"}}}},"Building":{{"b":{{}}}},
                "Timestep":{{"t":{{"number_of_timesteps_per_hour":4}}}},"Zone":{{"actual zone":{{}}}},
                "ScheduleTypeLimits":{{"T":{{"lower_limit_value":{operand},"upper_limit_value":200,"numeric_type":"Continuous"}}}},
                "Schedule:Constant":{{"C":{{"schedule_type_limits_name":"T","hourly_value":{operand}}}}},
                "Schedule:Compact":{{"S":{{"schedule_type_limits_name":"T","data":[
                    {{"field":"Through:12/31"}},{{"field":"For:AllDays"}},{{"field":"Until:24:00"}},{{"field":{operand}}}]}}}}
            }}"#
            );
            let raw = ep_raw_model::parse_epjson_str(&source)?;
            resolve_source_a(&raw)?;
            resolve_source_b(&raw)?;
            // A retains statistics-only scalars for these two families; its
            // real numeric values come from the raw-model projection below.
            // B retains the complete raw scalar. Compact is complete in both.
            for (ip, scalar) in [
                (
                    crate::ordinary_input_processor::resolve_source_a(&raw)?,
                    V::OtherScalar,
                ),
                (
                    crate::ordinary_input_processor::resolve_source_b(&raw)?,
                    V::Number(operand.into()),
                ),
            ] {
                assert_eq!(
                    ip.source_raw_object_fields("Schedule:Constant", "C")?
                        .get("hourly_value"),
                    Some(&scalar)
                );
                assert_eq!(
                    ip.source_raw_object_fields("ScheduleTypeLimits", "T")?
                        .get("lower_limit_value"),
                    Some(&scalar)
                );
                let V::Array(data) = &ip.source_raw_object_fields("Schedule:Compact", "S")?["data"]
                else {
                    return Err("actual raw Compact array".into());
                };
                let V::Object(value) = &data[3] else {
                    return Err("actual raw Compact scalar".into());
                };
                assert_eq!(value.get("field"), Some(&V::Number(operand.into())));
            }
            let kind = ep_model::classify_source_json_number(operand)
                .map_err(|_| "actual retained JSON kind")?;
            assert_eq!(
                matches!(kind, ep_model::SourceJsonNumber::Integer(_)),
                integer
            );
            let projected = crate::ordinary_schedule_input::resolve(&raw)?;
            assert_eq!(
                projected.types[0]
                    .lower
                    .ok_or("actual lower value")?
                    .to_bits(),
                expected.to_bits()
            );
            assert_eq!(projected.constants[0].value.to_bits(), expected.to_bits());
        }
        let raw = ep_raw_model::parse_epjson_str(
            r#"{
            "Zone":{"actual zone":{}},"ScheduleTypeLimits":{"T":{"lower_limit_value":-1,"upper_limit_value":1,"numeric_type":"Continuous"}},
            "Schedule:Compact":{"S":{"schedule_type_limits_name":"T","data":[
                {"field":"Through:12/31"},{"field":"For:AllDays"},{"field":"Until:24:00"},{"field":"-0"}]}}
        }"#,
        )?;
        resolve_source_a(&raw)?;
        let ip = crate::ordinary_input_processor::resolve_source_a(&raw)?;
        let V::Array(data) = &ip.source_raw_object_fields("Schedule:Compact", "S")?["data"] else {
            return Err("actual String Compact array".into());
        };
        let V::Object(value) = &data[3] else {
            return Err("actual String Compact scalar".into());
        };
        assert_eq!(value.get("field"), Some(&V::String("-0".into())));
        let compiled = ep_compiler::compile_raw_model(&raw);
        assert!(!compiled.has_errors(), "{:?}", compiled.report.diagnostics);
        let model = compiled.model.ok_or("actual String Compact model")?;
        assert_eq!(
            model.compact_schedules[0].periods[0].day_profiles[0].segments[0]
                .value
                .to_bits(),
            (-0.0_f64).to_bits()
        );
        // Unknown instance members are retained metadata, not additional
        // schedule operands. All nine families still use the same known fields.
        let mut metadata_input = input.clone();
        metadata_input["Schedule:Constant"] =
            serde_json::json!({"C":{"schedule_type_limits_name":"Fraction","hourly_value":0.25}});
        metadata_input["Schedule:Day:Hourly"] =
            serde_json::json!({"H":{"schedule_type_limits_name":"Fraction","hour_7":1}});
        metadata_input["Schedule:Day:Interval"] = serde_json::json!({"I":{"schedule_type_limits_name":"Fraction","data":[{"time":"06:00","value_until_time":0},{"time":"18:00","value_until_time":1},{"time":"24:00","value_until_time":0}]}});
        let metadata_values = (0..24)
            .map(|hour| serde_json::json!({"value":if hour<3 {0}else{1}}))
            .collect::<Vec<_>>();
        metadata_input["Schedule:Day:List"] = serde_json::json!({"L":{"schedule_type_limits_name":"Fraction","minutes_per_item":60,"extensions":metadata_values}});
        metadata_input["Schedule:Week:Daily"] = regular["Schedule:Week:Daily"].clone();
        metadata_input["Schedule:Week:Compact"] =
            serde_json::json!({"WI":{"data":[{"daytype_list":"AllDays","schedule_day_name":"I"}]}});
        metadata_input["Schedule:Year"] = serde_json::json!({"Y":{"schedule_type_limits_name":"Fraction","schedule_weeks":[{"schedule_week_name":"WI","start_month":1,"start_day":1,"end_month":12,"end_day":31}]}});
        let schedule_objects = [
            ("ScheduleTypeLimits", "Fraction"),
            ("Schedule:Constant", "C"),
            ("Schedule:Compact", "variable schedule"),
            ("Schedule:Day:Hourly", "H"),
            ("Schedule:Day:Interval", "I"),
            ("Schedule:Day:List", "L"),
            ("Schedule:Week:Daily", "W"),
            ("Schedule:Week:Compact", "WI"),
            ("Schedule:Year", "Y"),
        ];
        let baseline = ep_raw_model::parse_epjson_str(&metadata_input.to_string())?;
        let baseline_a = crate::ordinary_input_processor::resolve_source_a(&baseline)?;
        let baseline_b = crate::ordinary_input_processor::resolve_source_b(&baseline)?;
        let baseline_declarations = crate::ordinary_schedule_input::resolve(&baseline)?;
        let compiled = ep_compiler::compile_raw_model(&baseline);
        assert!(!compiled.has_errors(), "{:?}", compiled.report.diagnostics);
        let baseline_model = compiled.model.ok_or("actual metadata baseline model")?;
        let nested_full = V::Object(std::collections::BTreeMap::from([(
            "nested".into(),
            V::Array(vec![V::Bool(true), V::Number("1".into()), V::Null]),
        )]));
        let nested_statistics = V::Object(std::collections::BTreeMap::from([(
            "nested".into(),
            V::Array(vec![V::OtherScalar, V::OtherScalar, V::Null]),
        )]));
        for (metadata, full, statistics, size) in [
            (serde_json::json!(null), V::Null, V::Null, 0_usize),
            (serde_json::json!(true), V::Bool(true), V::OtherScalar, 1),
            (
                serde_json::json!(1),
                V::Number("1".into()),
                V::OtherScalar,
                1,
            ),
            (
                serde_json::json!("literal -0 [1E+0]"),
                V::String("literal -0 [1E+0]".into()),
                V::String("literal -0 [1E+0]".into()),
                1,
            ),
            (
                serde_json::json!([null, true, 1]),
                V::Array(vec![V::Null, V::Bool(true), V::Number("1".into())]),
                V::Array(vec![V::Null, V::OtherScalar, V::OtherScalar]),
                3,
            ),
            (
                serde_json::json!({"nested":[true,1,null]}),
                nested_full,
                nested_statistics,
                1,
            ),
        ] {
            let mut tagged = metadata_input.clone();
            for (family, name) in schedule_objects {
                tagged[family][name]["user_metadata"] = metadata.clone();
            }
            let raw = ep_raw_model::parse_epjson_str(&tagged.to_string())?;
            assert!(raw.is_epjson());
            assert_eq!(
                crate::ordinary_schedule_input::resolve(&raw)?,
                baseline_declarations
            );
            let compiled = ep_compiler::compile_raw_model(&raw);
            assert!(!compiled.has_errors(), "{:?}", compiled.report.diagnostics);
            assert_eq!(compiled.model.as_ref(), Some(&baseline_model));
            for (source_b, mut ip, mut original) in [
                (
                    false,
                    crate::ordinary_input_processor::resolve_source_a(&raw)?,
                    baseline_a.clone(),
                ),
                (
                    true,
                    crate::ordinary_input_processor::resolve_source_b(&raw)?,
                    baseline_b.clone(),
                ),
            ] {
                assert_eq!(ip.get_max_schema_args()?, original.get_max_schema_args()?);
                let mut actual_stats =
                    ep_runtime::heat_balance::source_output_prescan::SourceOutputPreScan::default();
                let mut original_stats =
                    ep_runtime::heat_balance::source_output_prescan::SourceOutputPreScan::default();
                ip.report_source_idf_records_stats(&mut actual_stats)?;
                original.report_source_idf_records_stats(&mut original_stats)?;
                assert_eq!(actual_stats, original_stats);
                for (family, name) in schedule_objects {
                    assert_eq!(
                        ip.get_object_def_max_args(family)?,
                        original.get_object_def_max_args(family)?
                    );
                    assert_eq!(
                        ip.source_raw_object_names(family)?,
                        original.source_raw_object_names(family)?
                    );
                    let Some(ep_runtime::heat_balance::selected_input_processor::SourceInputFamily::Objects(shapes)) = ip.family(family) else {
                        return Err("actual metadata family shape".into());
                    };
                    let shape = shapes
                        .iter()
                        .find(|shape| shape.object_name == name)
                        .ok_or("actual metadata object shape")?;
                    assert_eq!(shape.field_sizes.get("user_metadata"), Some(&size));
                    let expected = if !source_b
                        && matches!(family, "ScheduleTypeLimits" | "Schedule:Constant")
                    {
                        &statistics
                    } else {
                        &full
                    };
                    assert_eq!(
                        ip.source_raw_object_fields(family, name)?
                            .get("user_metadata"),
                        Some(expected)
                    );
                    assert!(ip.is_object_unused(family, name)); // Transfer/statistics are not getters.
                }
            }
        }
        // Unknown-member acceptance cannot hide a bad known operand. No
        // extensible-row validator or schema scalar grammar is widened.
        for (family, name, field, value) in [
            (
                "ScheduleTypeLimits",
                "Fraction",
                "numeric_type",
                serde_json::json!("NotANumericType"),
            ),
            (
                "Schedule:Constant",
                "C",
                "hourly_value",
                serde_json::json!("not a number"),
            ),
            (
                "Schedule:Compact",
                "variable schedule",
                "data",
                serde_json::json!([null]),
            ),
            ("Schedule:Day:Hourly", "H", "hour_7", serde_json::json!("1")),
            (
                "Schedule:Day:Interval",
                "I",
                "interpolate_to_timestep",
                serde_json::json!("Future"),
            ),
            (
                "Schedule:Day:List",
                "L",
                "minutes_per_item",
                serde_json::json!(1.5),
            ),
            (
                "Schedule:Week:Daily",
                "W",
                "monday_schedule_day_name",
                serde_json::json!(true),
            ),
            (
                "Schedule:Week:Compact",
                "WI",
                "data",
                serde_json::json!([{"daytype_list":"AllDays","schedule_day_name":null}]),
            ),
            (
                "Schedule:Year",
                "Y",
                "schedule_weeks",
                serde_json::json!([{"schedule_week_name":"W","start_month":13,"start_day":1,"end_month":12,"end_day":31}]),
            ),
        ] {
            let mut invalid = metadata_input.clone();
            invalid[family][name]["user_metadata"] = serde_json::json!({"kept":true});
            invalid[family][name][field] = value;
            let raw = ep_raw_model::parse_epjson_str(&invalid.to_string())?;
            assert!(resolve_source_a(&raw).is_err(), "{family}.{field}");
            assert!(resolve_source_b(&raw).is_err(), "{family}.{field}");
        }
        for (family, name) in schedule_objects {
            for metadata in ["idf_order", "idf_max_fields", "idf_max_extensible_fields"] {
                let mut invalid = metadata_input.clone();
                invalid[family][name][metadata] = serde_json::json!(null);
                let raw = ep_raw_model::parse_epjson_str(&invalid.to_string())?;
                assert!(resolve_source_a(&raw).is_err(), "{family}.{metadata}");
                assert!(resolve_source_b(&raw).is_err(), "{family}.{metadata}");
            }
        }
        let legacy = serde_json::json!({"Zone":{"actual zone":{}},"ScheduleTypeLimits":{"T":{}},"Schedule:Constant":{"C":{"schedule_type_limits_name":"T","hourly_value":0}}});
        resolve(&ep_raw_model::parse_epjson_str(&legacy.to_string())?)?;
        for family in ["ScheduleTypeLimits", "Schedule:Constant"] {
            let mut tagged = legacy.clone();
            let name = if family == "ScheduleTypeLimits" {
                "T"
            } else {
                "C"
            };
            tagged[family][name]["user_metadata"] = serde_json::json!([null]);
            let raw = ep_raw_model::parse_epjson_str(&tagged.to_string())?;
            assert!(resolve(&raw).is_err());
            resolve_source_a(&raw)?;
            resolve_source_b(&raw)?;
        }
        let mut unrelated = metadata_input.clone();
        unrelated["Building"]["b"]["user_metadata"] = serde_json::json!(true);
        assert!(
            resolve_source_a(&ep_raw_model::parse_epjson_str(&unrelated.to_string())?).is_err()
        );
        // Actual staged-IDF origin, physical counts and reverse order remain
        // separate from arbitrary retained instance members.
        let zeros = vec!["0"; 24].join(",");
        let idf = format!(
            "Zone,actual zone;ScheduleTypeLimits,T;Schedule:Day:Hourly,Z,T,{zeros};Schedule:Day:Hourly,A,T,{zeros};"
        );
        let mut staged = serde_json::json!({"Zone":{"actual zone":{}},"ScheduleTypeLimits":{"T":{}},"Schedule:Day:Hourly":{"Z":{"schedule_type_limits_name":"T"},"A":{"schedule_type_limits_name":"T"}}});
        for name in ["Z", "A"] {
            for hour in 1..=24 {
                staged["Schedule:Day:Hourly"][name][format!("hour_{hour}")] = serde_json::json!(0);
            }
        }
        let baseline = ep_raw_model::parse_epjson_str_with_idf_order(&staged.to_string(), &idf)?;
        staged["Schedule:Day:Hourly"]["Z"]["user_metadata"] =
            serde_json::json!({"retained":[true,null,1]});
        let tagged = ep_raw_model::parse_epjson_str_with_idf_order(&staged.to_string(), &idf)?;
        assert!(!baseline.is_epjson() && !tagged.is_epjson());
        assert_eq!(
            crate::ordinary_schedule_input::resolve(&tagged)?.source_is_epjson,
            Some(false)
        );
        for (mut original, mut ip) in [
            (
                crate::ordinary_input_processor::resolve_source_a(&baseline)?,
                crate::ordinary_input_processor::resolve_source_a(&tagged)?,
            ),
            (
                crate::ordinary_input_processor::resolve_source_b(&baseline)?,
                crate::ordinary_input_processor::resolve_source_b(&tagged)?,
            ),
        ] {
            assert_eq!(
                ip.source_raw_object_names("Schedule:Day:Hourly")?,
                ["Z", "A"]
            );
            assert_eq!(
                ip.get_object_def_max_args("Schedule:Day:Hourly")?,
                original.get_object_def_max_args("Schedule:Day:Hourly")?
            );
            for name in ["Z", "A"] {
                for field in ["idf_max_fields", "idf_max_extensible_fields"] {
                    assert_eq!(
                        ip.source_raw_object_fields("Schedule:Day:Hourly", name)?
                            .get(field),
                        original
                            .source_raw_object_fields("Schedule:Day:Hourly", name)?
                            .get(field)
                    );
                }
            }
        }
        // Source-only per-area methods preserve the unused direct level and
        // selected raw Numeric2. Legacy and known operand domains stay bounded.
        for method in ["Watts/Area", "Power/Area"] {
            for density in [None, Some(0.0), Some(-0.0), Some(3.7)] {
                let mut value = serde_json::json!({
                    "Zone":{"actual zone":{}},
                    "Schedule:Constant":{"on":{"hourly_value":1}},
                    "OtherEquipment":{"area":{"zone_or_zonelist_or_space_or_spacelist_name":"actual zone",
                        "schedule_name":"on","design_level_calculation_method":method,"design_level":352}}
                });
                if let Some(density) = density {
                    value["OtherEquipment"]["area"]["power_per_floor_area"] =
                        serde_json::json!(density);
                }
                let raw = ep_raw_model::parse_epjson_str(&value.to_string())?;
                assert!(resolve(&raw).is_err());
                resolve_source_a(&raw)?;
                resolve_source_b(&raw)?;
                let compiled = ep_compiler::compile_raw_model(&raw);
                let typed = compiled
                    .model
                    .ok_or_else(|| format!("{:?}", compiled.report.diagnostics))?;
                assert_eq!(typed.other_equipment[0].design_level_w, 352.0);
                let ip = crate::ordinary_input_processor::resolve_source_a(&raw)?;
                let hb = crate::ordinary_heat_balance_input::resolve(&raw, &ip)?;
                let fields = &hb.objects["OtherEquipment"][0].fields;
                assert_eq!(fields.get("power_per_floor_area"), density.map(ep_runtime::heat_balance::selected_project_data::ProjectInputValue::Number).as_ref());
                assert_eq!(
                    fields.get("design_level"),
                    Some(
                        &ep_runtime::heat_balance::selected_project_data::ProjectInputValue::Number(
                            352.0
                        )
                    )
                );
            }
        }
        for invalid in [
            serde_json::json!(-0.1),
            serde_json::json!(null),
            serde_json::json!(""),
            serde_json::json!(true),
        ] {
            let value = serde_json::json!({"Zone":{"actual zone":{}},"Schedule:Constant":{"on":{"hourly_value":1}},
                "OtherEquipment":{"area":{"zone_or_zonelist_or_space_or_spacelist_name":"actual zone","schedule_name":"on",
                    "design_level_calculation_method":"Power/Area","design_level":352,"power_per_floor_area":invalid}}});
            let bad = ep_raw_model::parse_epjson_str(&value.to_string())?;
            assert!(resolve_source_a(&bad).is_err() && resolve_source_b(&bad).is_err());
        }
        // Numeric3 remains retained, but it is not the per-area selected
        // operand. Native imposes no sign restriction on this unused field.
        for unused in [9.0_f64, -9.0] {
            let value = serde_json::json!({"Zone":{"actual zone":{}},"Schedule:Constant":{"on":{"hourly_value":1}},
                "OtherEquipment":{"area":{"zone_or_zonelist_or_space_or_spacelist_name":"actual zone","schedule_name":"on",
                    "design_level_calculation_method":"Power/Area","design_level":352,"power_per_floor_area":3.7,"power_per_person":unused}}});
            let raw = ep_raw_model::parse_epjson_str(&value.to_string())?;
            assert!(resolve(&raw).is_err());
            resolve_source_a(&raw)?;
            resolve_source_b(&raw)?;
            let compiled = ep_compiler::compile_raw_model(&raw);
            let typed = compiled
                .model
                .ok_or_else(|| format!("{:?}", compiled.report.diagnostics))?;
            assert_eq!(
                typed.other_equipment[0].power_per_person_w.to_bits(),
                unused.to_bits()
            );
            let ip = crate::ordinary_input_processor::resolve_source_a(&raw)?;
            let hb = crate::ordinary_heat_balance_input::resolve(&raw, &ip)?;
            assert_eq!(
                hb.objects["OtherEquipment"][0]
                    .fields
                    .get("power_per_person"),
                Some(
                    &ep_runtime::heat_balance::selected_project_data::ProjectInputValue::Number(
                        unused
                    )
                )
            );
        }
        // Source keeps actual omitted Numeric1 and finite unselected Numeric2/3;
        // the original legacy raw boundary still rejects both new cases.
        for design in [None, Some(7.0_f64)] {
            for unused in [9.0_f64, -9.0] {
                let mut value = serde_json::json!({"Zone":{"actual zone":{}},"Schedule:Constant":{"on":{"hourly_value":1}},
                    "OtherEquipment":{"direct":{"zone_or_zonelist_or_space_or_spacelist_name":"actual zone","schedule_name":"on",
                        "power_per_floor_area":unused,"power_per_person":unused}}});
                if let Some(design) = design {
                    value["OtherEquipment"]["direct"]["design_level"] = serde_json::json!(design);
                }
                let raw = ep_raw_model::parse_epjson_str(&value.to_string())?;
                assert!(resolve(&raw).is_err());
                resolve_source_a(&raw)?;
                resolve_source_b(&raw)?;
                let compiled = ep_compiler::compile_raw_model(&raw);
                let typed = compiled
                    .model
                    .ok_or_else(|| format!("{:?}", compiled.report.diagnostics))?;
                assert_eq!(
                    typed.other_equipment[0].design_level_w,
                    design.unwrap_or(0.0)
                );
                assert_eq!(
                    typed.other_equipment[0]
                        .power_per_floor_area_w_per_m2
                        .to_bits(),
                    unused.to_bits()
                );
                assert_eq!(
                    typed.other_equipment[0].power_per_person_w.to_bits(),
                    unused.to_bits()
                );
                let ip = crate::ordinary_input_processor::resolve_source_a(&raw)?;
                let hb = crate::ordinary_heat_balance_input::resolve(&raw, &ip)?;
                let fields = &hb.objects["OtherEquipment"][0].fields;
                assert_eq!(fields.contains_key("design_level"), design.is_some());
                for name in ["power_per_floor_area", "power_per_person"] {
                    assert_eq!(
                        fields[name],
                        ep_runtime::heat_balance::selected_project_data::ProjectInputValue::Number(
                            unused
                        )
                    );
                }
            }
        }
        for (lost, valid) in [
            (0.3995_f64, true),
            (0.4005, true),
            (f64::from_bits(0.401_f64.to_bits() - 1), true),
            (0.401, true),
            (f64::from_bits(0.401_f64.to_bits() + 1), false),
            (0.4011, false),
        ] {
            let value = serde_json::json!({"Zone":{"actual zone":{}},"Schedule:Constant":{"on":{"hourly_value":1}},
                "OtherEquipment":{"clamp":{"zone_or_zonelist_or_space_or_spacelist_name":"actual zone","schedule_name":"on",
                    "design_level":7,"fraction_latent":0,"fraction_radiant":0.6,"fraction_lost":lost}}});
            let raw = ep_raw_model::parse_epjson_str(&value.to_string())?;
            assert_eq!(resolve_source_a(&raw).is_ok(), valid, "{lost:?}");
            assert_eq!(resolve_source_b(&raw).is_ok(), valid, "{lost:?}");
            assert_eq!(resolve(&raw).is_ok(), 0.6 + lost <= 1.0, "legacy sum fence");
        }
        // An absent schema-number operand is distinct from explicit null/string.
        for name in [
            "design_level",
            "power_per_floor_area",
            "power_per_person",
            "fraction_lost",
        ] {
            for invalid in [
                serde_json::json!(null),
                serde_json::json!(""),
                serde_json::json!("1D0"),
                serde_json::json!(true),
            ] {
                let mut value = serde_json::json!({"Zone":{"actual zone":{}},"Schedule:Constant":{"on":{"hourly_value":1}},
                    "OtherEquipment":{"direct":{"zone_or_zonelist_or_space_or_spacelist_name":"actual zone","schedule_name":"on","design_level":7}}});
                value["OtherEquipment"]["direct"][name] = invalid;
                let raw = ep_raw_model::parse_epjson_str(&value.to_string())?;
                assert!(resolve_source_a(&raw).is_err() && resolve_source_b(&raw).is_err());
            }
        }
        // Exact JSON lexemes reach every HB numeric operand through the same
        // Native number-kind projection; existing A statistics remain scalar-only.
        for (token, expected) in [
            ("-0", 0.0_f64),
            ("-0.0", -0.0),
            ("-0e0", -0.0),
            ("1e-9999", 0.0),
            ("-1e-9999", -0.0),
        ] {
            let text = format!(
                r#"{{"Zone":{{"actual zone":{{}}}},"Schedule:Constant":{{"on":{{"hourly_value":1}}}},
                "OtherEquipment":{{"numbers":{{"zone_or_zonelist_or_space_or_spacelist_name":"actual zone","schedule_name":"on",
                "design_level":{token},"power_per_floor_area":{token},"power_per_person":{token},
                "fraction_latent":{token},"fraction_radiant":{token},"fraction_lost":{token},
                "carbon_dioxide_generation_rate":{token}}}}}}}"#
            );
            let raw = ep_raw_model::parse_epjson_str(&text)?;
            resolve_source_a(&raw)?;
            resolve_source_b(&raw)?;
            for (ip, full) in [
                (
                    crate::ordinary_input_processor::resolve_source_a(&raw)?,
                    false,
                ),
                (
                    crate::ordinary_input_processor::resolve_source_b(&raw)?,
                    true,
                ),
            ] {
                let hb = crate::ordinary_heat_balance_input::resolve(&raw, &ip)?;
                let fields = &hb.objects["OtherEquipment"][0].fields;
                let original = ip.source_raw_object_fields("OtherEquipment", "numbers")?;
                for name in [
                    "design_level",
                    "power_per_floor_area",
                    "power_per_person",
                    "fraction_latent",
                    "fraction_radiant",
                    "fraction_lost",
                    "carbon_dioxide_generation_rate",
                ] {
                    let ep_runtime::heat_balance::selected_project_data::ProjectInputValue::Number(
                        value,
                    ) = &fields[name]
                    else {
                        return Err("actual HB numeric field lost".into());
                    };
                    assert_eq!(value.to_bits(), expected.to_bits(), "{name}/{token}");
                    assert_eq!(
                        original[name],
                        if full {
                            V::Number(token.into())
                        } else {
                            V::OtherScalar
                        }
                    );
                }
            }
        }
        // Six harmless metadata shapes remain in RawModel and genuine IP, but
        // never become an HB getter operand. The unsigned selected loss is a
        // literal JSON number in both baseline and tagged inputs.
        let equipment_json = r#"{"Zone":{"actual zone":{}},"Schedule:Constant":{"on":{"hourly_value":1}},
            "OtherEquipment":{
                "N":{"fuel_type":"None","zone_or_zonelist_or_space_or_spacelist_name":"actual zone","schedule_name":"on","design_level":18446744073709551615},
                "P":{"fuel_type":"None","zone_or_zonelist_or_space_or_spacelist_name":"actual zone","schedule_name":"on","design_level":7}}}"#;
        let baseline = ep_raw_model::parse_epjson_str(equipment_json)?;
        let compiled = ep_compiler::compile_raw_model(&baseline);
        let baseline_model = compiled
            .model
            .ok_or_else(|| format!("{:?}", compiled.report.diagnostics))?;
        assert_eq!(baseline_model.other_equipment[0].design_level_w, -1.0);
        let baseline_a = crate::ordinary_input_processor::resolve_source_a(&baseline)?;
        let baseline_b = crate::ordinary_input_processor::resolve_source_b(&baseline)?;
        let baseline_hb = crate::ordinary_heat_balance_input::resolve(&baseline, &baseline_a)?;
        assert_eq!(
            baseline_hb.objects["OtherEquipment"][0].fields["design_level"],
            ep_runtime::heat_balance::selected_project_data::ProjectInputValue::Number(-1.0)
        );
        for (metadata, full, statistics, size) in [
            ("null", V::Null, V::Null, 0_usize),
            ("true", V::Bool(true), V::OtherScalar, 1),
            (
                "18446744073709551615",
                V::Number("18446744073709551615".into()),
                V::OtherScalar,
                1,
            ),
            (
                r#""literal -0""#,
                V::String("literal -0".into()),
                V::String("literal -0".into()),
                1,
            ),
            (
                "[null,true,1]",
                V::Array(vec![V::Null, V::Bool(true), V::Number("1".into())]),
                V::Array(vec![V::Null, V::OtherScalar, V::OtherScalar]),
                3,
            ),
            (
                r#"{"nested":null}"#,
                V::Object(std::collections::BTreeMap::from([(
                    "nested".into(),
                    V::Null,
                )])),
                V::Object(std::collections::BTreeMap::from([(
                    "nested".into(),
                    V::Null,
                )])),
                1,
            ),
        ] {
            let tagged = equipment_json.replace(
                r#""fuel_type":"None""#,
                &format!(r#""fuel_type":"None","user_metadata":{metadata},"name":{metadata}"#),
            );
            let raw = ep_raw_model::parse_epjson_str(&tagged)?;
            assert!(resolve(&raw).is_err());
            resolve_source_a(&raw)?;
            resolve_source_b(&raw)?;
            assert_eq!(
                ep_compiler::compile_raw_model(&raw).model.as_ref(),
                Some(&baseline_model)
            );
            for (mut ip, mut original, expected) in [
                (
                    crate::ordinary_input_processor::resolve_source_a(&raw)?,
                    baseline_a.clone(),
                    &statistics,
                ),
                (
                    crate::ordinary_input_processor::resolve_source_b(&raw)?,
                    baseline_b.clone(),
                    &full,
                ),
            ] {
                assert_eq!(
                    crate::ordinary_heat_balance_input::resolve(&raw, &ip)?,
                    baseline_hb
                );
                assert_eq!(ip.get_max_schema_args()?, original.get_max_schema_args()?);
                assert_eq!(
                    ip.get_object_def_max_args("OtherEquipment")?,
                    original.get_object_def_max_args("OtherEquipment")?
                );
                let mut actual_stats =
                    ep_runtime::heat_balance::source_output_prescan::SourceOutputPreScan::default();
                let mut original_stats =
                    ep_runtime::heat_balance::source_output_prescan::SourceOutputPreScan::default();
                ip.report_source_idf_records_stats(&mut actual_stats)?;
                original.report_source_idf_records_stats(&mut original_stats)?;
                assert_eq!(actual_stats, original_stats);
                for name in ["N", "P"] {
                    let fields = ip.source_raw_object_fields("OtherEquipment", name)?;
                    assert_eq!(fields.get("user_metadata"), Some(expected));
                    assert_eq!(fields.get("name"), Some(expected));
                    let Some(ep_runtime::heat_balance::selected_input_processor::SourceInputFamily::Objects(shapes)) = ip.family("OtherEquipment") else {
                        return Err("actual OtherEquipment raw shape".into());
                    };
                    assert_eq!(
                        shapes
                            .iter()
                            .find(|s| s.object_name == name)
                            .ok_or("actual shape")?
                            .field_sizes
                            .get("user_metadata"),
                        Some(&size)
                    );
                }
            }
        }
        // A malformed real property or reserved IDF member cannot become metadata.
        for name in [
            "fuel_type",
            "zone_or_zonelist_or_space_or_spacelist_name",
            "schedule_name",
            "design_level_calculation_method",
            "design_level",
            "power_per_floor_area",
            "power_per_person",
            "fraction_latent",
            "fraction_radiant",
            "fraction_lost",
            "carbon_dioxide_generation_rate",
            "end_use_subcategory",
            "idf_order",
            "idf_max_fields",
            "idf_max_extensible_fields",
        ] {
            let mut raw = ep_raw_model::parse_epjson_str(equipment_json)?;
            raw.objects
                .get_mut(&ObjectType("OtherEquipment".into()))
                .ok_or("known fields family")?
                .get_mut(&ep_raw_model::ObjectName("N".into()))
                .ok_or("known fields object")?
                .fields
                .insert(FieldName(name.into()), RawValue::Array(Vec::new()));
            assert!(
                resolve_source_a(&raw).is_err() && resolve_source_b(&raw).is_err(),
                "{name}"
            );
        }
        // The IDF overflow fallback is a stored double in its actual converted
        // JSON. Preserve its positive value, true order and padded max_fields.
        let converted = r#"{"Zone":{"actual zone":{}},"Schedule:Constant":{"on":{"hourly_value":1}},
            "OtherEquipment":{
                "N":{"fuel_type":"None","zone_or_zonelist_or_space_or_spacelist_name":"actual zone","schedule_name":"on","design_level":18446744073709551615.0},
                "P":{"fuel_type":"None","zone_or_zonelist_or_space_or_spacelist_name":"actual zone","schedule_name":"on","design_level":7}}}"#;
        let idf = "Zone,actual zone;Schedule:Constant,on,,1;OtherEquipment,P,None,actual zone,on,EquipmentLevel,7;OtherEquipment,N,None,actual zone,on,EquipmentLevel,18446744073709551615;";
        let raw = ep_raw_model::parse_epjson_str_with_idf_order(converted, idf)?;
        let ip = crate::ordinary_input_processor::resolve_source_a(&raw)?;
        let hb = crate::ordinary_heat_balance_input::resolve(&raw, &ip)?;
        assert!(!hb.source_is_epjson);
        assert_eq!(
            hb.objects["OtherEquipment"]
                .iter()
                .map(|o| o.object_name.as_str())
                .collect::<Vec<_>>(),
            ["P", "N"]
        );
        assert!(
            hb.objects["OtherEquipment"]
                .iter()
                .all(|o| o.idf_max_fields == Some(11))
        );
        assert_eq!(
            hb.objects["OtherEquipment"][1].fields["design_level"],
            ep_runtime::heat_balance::selected_project_data::ProjectInputValue::Number(
                18446744073709551616.0
            )
        );
        // The same selected raw gain getter consumes the real eFuel, while
        // legacy admission retains its original None-only boundary.
        for fuel in [
            "Electricity",
            "NaturalGas",
            "Gasoline",
            "Diesel",
            "Coal",
            "Propane",
            "FuelOilNo1",
            "FuelOilNo2",
            "OtherFuel1",
            "OtherFuel2",
            "DistrictCooling",
            "DistrictHeatingWater",
            "DistrictHeatingSteam",
        ] {
            let value = serde_json::json!({"Zone":{"actual zone":{}},"Schedule:Constant":{"on":{"hourly_value":1}},
                "OtherEquipment":{"fuel":{"zone_or_zonelist_or_space_or_spacelist_name":"actual zone","schedule_name":"on",
                    "fuel_type":fuel,"design_level_calculation_method":"Power/Area","design_level":352,"power_per_floor_area":3.7}}});
            let raw = ep_raw_model::parse_epjson_str(&value.to_string())?;
            assert!(resolve(&raw).is_err());
            resolve_source_a(&raw)?;
            resolve_source_b(&raw)?;
            let result = ep_compiler::compile_raw_model(&raw);
            let typed = result
                .model
                .ok_or_else(|| format!("{:?}", result.report.diagnostics))?;
            assert_eq!(
                typed.other_equipment[0].fuel_type.0,
                fuel.to_ascii_uppercase()
            );
            let ip = crate::ordinary_input_processor::resolve_source_a(&raw)?;
            let hb = crate::ordinary_heat_balance_input::resolve(&raw, &ip)?;
            assert_eq!(
                hb.objects["OtherEquipment"][0].fields["fuel_type"],
                ep_runtime::heat_balance::selected_project_data::ProjectInputValue::String(
                    fuel.into()
                )
            );
        }
        for fuel in ["Water", "Unrecognized fuel"] {
            let value = serde_json::json!({"Zone":{"actual zone":{}},"Schedule:Constant":{"on":{"hourly_value":1}},
                "OtherEquipment":{"fuel":{"zone_or_zonelist_or_space_or_spacelist_name":"actual zone","schedule_name":"on",
                    "fuel_type":fuel,"design_level_calculation_method":"EquipmentLevel","design_level":7}}});
            let raw = ep_raw_model::parse_epjson_str(&value.to_string())?;
            assert!(resolve_source_a(&raw).is_err() && resolve_source_b(&raw).is_err());
        }
        // The actual admission requires its Zone even for this isolated
        // format/count transfer check; no simulation or Native execution runs.
        let json = r#"{"Zone":{"actual zone":{}},"Output:Schedules":{"Output:Schedules 1":{"key_field":"Hourly"},"Output:Schedules 2":{"key_field":"Timestep"}}}"#;
        let raw = ep_raw_model::parse_epjson_str_with_idf_order(
            json,
            "Zone,actual zone;Output:Schedules,Hourly;Output:Schedules,Timestep",
        )?;
        let ip = crate::ordinary_input_processor::resolve_source_a(&raw)?;
        assert_eq!(
            ip.source_raw_object_names("Output:Schedules")?,
            ["Output:Schedules 1", "Output:Schedules 2"]
        );
        for (name, count) in [("Output:Schedules 1", "1"), ("Output:Schedules 2", "0")] {
            assert_eq!(
                ip.source_raw_object_fields("Output:Schedules", name)?
                    .get("idf_max_fields"),
                Some(&V::Number(count.into()))
            );
        }
        for method in ["Watts/Person", "Power/Person"] {
            for (unused, projected) in
                [("-9", -9.0_f64), ("9", 9.0), ("18446744073709551615", -1.0)]
            {
                let text = format!(
                    r#"{{"Zone":{{"actual zone":{{}}}},"Schedule:Constant":{{"on":{{"hourly_value":1}},"activity":{{"hourly_value":120}}}},"People":{{"Occupants":{{"zone_or_zonelist_or_space_or_spacelist_name":"actual zone","number_of_people_schedule_name":"on","activity_level_schedule_name":"activity","number_of_people":2,"sensible_heat_fraction":0.6}}}},"OtherEquipment":{{"Per person":{{"zone_or_zonelist_or_space_or_spacelist_name":"actual zone","schedule_name":"on","design_level_calculation_method":"{method}","design_level":-7,"power_per_floor_area":{unused},"power_per_person":3.5}}}}}}"#
                );
                let raw = ep_raw_model::parse_epjson_str(&text)?;
                resolve_source_a(&raw)?;
                resolve_source_b(&raw)?;
                assert!(resolve(&raw).is_err());
                for (source_b, ip) in [
                    (
                        false,
                        crate::ordinary_input_processor::resolve_source_a(&raw)?,
                    ),
                    (
                        true,
                        crate::ordinary_input_processor::resolve_source_b(&raw)?,
                    ),
                ] {
                    // A retains the existing statistics-only numeric marker;
                    // the genuine HB projection below still reads the raw number.
                    let expected = if source_b {
                        V::Number(unused.into())
                    } else {
                        V::OtherScalar
                    };
                    assert_eq!(
                        ip.source_raw_object_fields("OtherEquipment", "Per person")?
                            .get("power_per_floor_area"),
                        Some(&expected)
                    );
                    let hb = crate::ordinary_heat_balance_input::resolve(&raw, &ip)?;
                    assert_eq!(hb.objects["OtherEquipment"][0].fields.get("power_per_floor_area"),Some(&ep_runtime::heat_balance::selected_project_data::ProjectInputValue::Number(projected)));
                }
                let result = ep_compiler::compile_raw_model(&raw);
                assert!(
                    !result.has_errors(),
                    "{method}: {:?}",
                    result.report.diagnostics
                );
                let typed = result.model.ok_or("typed real per-person operands")?;
                assert_eq!(
                    typed.other_equipment[0]
                        .power_per_floor_area_w_per_m2
                        .to_bits(),
                    projected.to_bits()
                );
                assert_eq!(typed.other_equipment[0].power_per_person_w, 3.5);
                if unused == "-9" {
                    let missing = text.replace(",\"power_per_person\":3.5", "");
                    assert_ne!(missing, text);
                    let raw = ep_raw_model::parse_epjson_str(&missing)?;
                    resolve_source_a(&raw)?;
                    resolve_source_b(&raw)?;
                    let result = ep_compiler::compile_raw_model(&raw);
                    assert!(
                        !result.has_errors(),
                        "missing selected operand: {:?}",
                        result.report.diagnostics
                    );
                    assert_eq!(
                        result
                            .model
                            .ok_or("typed missing per-person operand")?
                            .other_equipment[0]
                            .power_per_person_w,
                        0.0
                    );
                    for invalid in ["null", "\"3.5\"", "\"\""] {
                        let text = text.replace(
                            "\"power_per_person\":3.5",
                            &format!("\"power_per_person\":{invalid}"),
                        );
                        let raw = ep_raw_model::parse_epjson_str(&text)?;
                        assert!(resolve_source_a(&raw).is_err());
                        assert!(resolve_source_b(&raw).is_err());
                    }
                }
            }
        }
        // Genuine built-in occupancy stays raw and typed without an authored
        // replacement schedule. Both Source frontends preserve the same name.
        for (reference, expected) in [
            (
                "Constant-0.0",
                ep_model::PeopleScheduleReference::ConstantZero,
            ),
            (
                "cOnStAnT-1.0",
                ep_model::PeopleScheduleReference::ConstantOne,
            ),
        ] {
            let text = format!(
                r#"{{"Zone":{{"actual zone":{{}}}},"Schedule:Constant":{{"activity":{{"hourly_value":120}}}},"People":{{"P":{{"zone_or_zonelist_or_space_or_spacelist_name":"actual zone","number_of_people_schedule_name":"{reference}","activity_level_schedule_name":"activity","number_of_people":2,"sensible_heat_fraction":0.6}}}}}}"#
            );
            let raw = ep_raw_model::parse_epjson_str(&text)?;
            resolve_source_a(&raw)?;
            resolve_source_b(&raw)?;
            for ip in [
                crate::ordinary_input_processor::resolve_source_a(&raw)?,
                crate::ordinary_input_processor::resolve_source_b(&raw)?,
            ] {
                assert_eq!(
                    ip.source_raw_object_fields("People", "P")?
                        .get("number_of_people_schedule_name"),
                    Some(&V::String(reference.into()))
                );
                let hb = crate::ordinary_heat_balance_input::resolve(&raw, &ip)?;
                assert_eq!(
                    hb.objects["People"][0]
                        .fields
                        .get("number_of_people_schedule_name"),
                    Some(
                        &ep_runtime::heat_balance::selected_project_data::ProjectInputValue::String(
                            reference.into()
                        )
                    )
                );
            }
            let result = ep_compiler::compile_raw_model(&raw);
            assert!(
                !result.has_errors(),
                "{reference}: {:?}",
                result.report.diagnostics
            );
            let model = result.model.ok_or("typed real builtin occupancy")?;
            assert_eq!(model.people[0].number_of_people_schedule, Some(expected));
            assert_eq!(model.schedules.len(), 1);
            assert_eq!(model.schedule_names.resolve(reference), None);
        }
        // Direct People shares the genuine Source A/B getter. Literal JSON kinds pass
        // raw schema checks before the same actual numeric getter conversion.
        for (token, expected) in [
            ("-0", 0.0_f64),
            ("-0.0", -0.0),
            ("18446744073709551615", -1.0),
            ("18446744073709551615.0", 18446744073709551616.0),
        ] {
            let text = format!(
                r#"{{"Zone":{{"actual zone":{{}}}},"Schedule:Constant":{{"on":{{"hourly_value":1}},"activity":{{"hourly_value":120}}}},"People":{{"P":{{"zone_or_zonelist_or_space_or_spacelist_name":"actual zone","number_of_people_schedule_name":"on","activity_level_schedule_name":"activity","number_of_people":{token},"people_per_floor_area":{token},"floor_area_per_person":{token},"sensible_heat_fraction":0.6,"mean_radiant_temperature_calculation_type":"SurfaceWeighted","surface_name_angle_factor_list_name":"missing unused surface","work_efficiency_schedule_name":"missing unused work","clothing_insulation_calculation_method":"CalculationMethodSchedule","clothing_insulation_calculation_method_schedule_name":"missing unused method","clothing_insulation_schedule_name":"missing unused clothing","air_velocity_schedule_name":"missing unused velocity","ankle_level_air_velocity_schedule_name":"missing unused ankle","metadata":{{"kept":[null,true,7,"text",[]]}}}}}}}}"#
            );
            let raw = ep_raw_model::parse_epjson_str(&text)?;
            resolve_source_a(&raw)?;
            resolve_source_b(&raw)?;
            assert!(resolve(&raw).is_err());
            for ip in [
                crate::ordinary_input_processor::resolve_source_a(&raw)?,
                crate::ordinary_input_processor::resolve_source_b(&raw)?,
            ] {
                let original = ip.source_raw_object_fields("People", "P")?;
                assert_eq!(original["number_of_people"], V::Number(token.into()));
                assert!(matches!(original["metadata"], V::Object(_)));
                let hb = crate::ordinary_heat_balance_input::resolve(&raw, &ip)?;
                for name in [
                    "number_of_people",
                    "people_per_floor_area",
                    "floor_area_per_person",
                ] {
                    let ep_runtime::heat_balance::selected_project_data::ProjectInputValue::Number(
                        value,
                    ) = hb.objects["People"][0].fields[name]
                    else {
                        return Err("actual People numeric getter".into());
                    };
                    assert_eq!(value.to_bits(), expected.to_bits());
                }
                assert!(!hb.objects["People"][0].fields.contains_key("metadata"));
            }
            let typed = ep_compiler::compile_raw_model(&raw);
            assert!(!typed.has_errors(), "{:?}", typed.report.diagnostics);
        }
        // These are original literal JSON operands. The real A/B IP and HB
        // retain omitted, blank, automatic and numeric sensible forms; only the
        // genuine runtime getter later selects a sizing operand and default.
        for (method, expected_method) in [
            ("People", ep_model::PeopleNumberCalculationMethod::People),
            (
                "People/Area",
                ep_model::PeopleNumberCalculationMethod::PeoplePerArea,
            ),
            (
                "Area/Person",
                ep_model::PeopleNumberCalculationMethod::AreaPerPerson,
            ),
        ] {
            for (sensible_field, expected_sensible) in [
                ("", None),
                (
                    r#","sensible_heat_fraction":"""#,
                    Some(V::String("".into())),
                ),
                (
                    r#","sensible_heat_fraction":"Autocalculate""#,
                    Some(V::String("Autocalculate".into())),
                ),
                (
                    r#","sensible_heat_fraction":0.6"#,
                    Some(V::Number("0.6".into())),
                ),
            ] {
                let text = format!(
                    r#"{{"Zone":{{"actual zone":{{}}}},"Schedule:Constant":{{"on":{{"hourly_value":1}},"activity":{{"hourly_value":120}}}},"People":{{"P":{{"zone_or_zonelist_or_space_or_spacelist_name":"actual zone","number_of_people_schedule_name":"on","activity_level_schedule_name":"activity","number_of_people_calculation_method":"{method}","number_of_people":2.0,"people_per_floor_area":0.5,"floor_area_per_person":4.0{sensible_field}}}}}}}"#
                );
                let raw = ep_raw_model::parse_epjson_str(&text)?;
                resolve_source_a(&raw)?;
                resolve_source_b(&raw)?;
                assert!(resolve(&raw).is_err());
                for ip in [
                    crate::ordinary_input_processor::resolve_source_a(&raw)?,
                    crate::ordinary_input_processor::resolve_source_b(&raw)?,
                ] {
                    let original = ip.source_raw_object_fields("People", "P")?;
                    assert_eq!(
                        original.get("sensible_heat_fraction"),
                        expected_sensible.as_ref()
                    );
                    assert_eq!(
                        original["number_of_people_calculation_method"],
                        V::String(method.into())
                    );
                    let hb = crate::ordinary_heat_balance_input::resolve(&raw, &ip)?;
                    let fields = &hb.objects["People"][0].fields;
                    for (name, expected) in [
                        ("number_of_people", 2.0_f64),
                        ("people_per_floor_area", 0.5),
                        ("floor_area_per_person", 4.0),
                    ] {
                        let ep_runtime::heat_balance::selected_project_data::ProjectInputValue::Number(value) = fields[name] else {
                            return Err("real People sizing HB operand".into());
                        };
                        assert_eq!(value.to_bits(), expected.to_bits());
                    }
                    match expected_sensible.as_ref() {
                        None => assert!(!fields.contains_key("sensible_heat_fraction")),
                        Some(V::String(value)) => assert_eq!(fields.get("sensible_heat_fraction"), Some(&ep_runtime::heat_balance::selected_project_data::ProjectInputValue::String(value.clone()))),
                        Some(V::Number(_)) => assert_eq!(fields.get("sensible_heat_fraction"), Some(&ep_runtime::heat_balance::selected_project_data::ProjectInputValue::Number(0.6))),
                        _ => return Err("authored People sensible test operand".into()),
                    }
                }
                let result = ep_compiler::compile_raw_model(&raw);
                assert!(
                    !result.has_errors(),
                    "{method}: {:?}",
                    result.report.diagnostics
                );
                let model = result.model.ok_or("typed People sizing method")?;
                assert_eq!(
                    model.people[0].number_of_people_calculation_method,
                    expected_method
                );
            }
        }
        // Positive Fanger keeps its true raw schedule references; unrelated
        // surface/method names remain unconsumed for these selected methods.
        for (slot, auxiliary) in [
            (1, ""),
            (
                7,
                r#","mean_radiant_temperature_calculation_type":"EnclosureAveraged","clothing_insulation_calculation_method":"ClothingInsulationSchedule","ankle_level_air_velocity_schedule_name":"ankle""#,
            ),
        ] {
            let text = format!(
                r#"{{"Zone":{{"actual zone":{{}}}},"Schedule:Constant":{{"on":{{"hourly_value":1}},"activity":{{"hourly_value":120}},"work":{{"hourly_value":0.2}},"clothes":{{"hourly_value":0.7}},"velocity":{{"hourly_value":0.1}},"ankle":{{"hourly_value":-0.4}}}},"People":{{"Fanger person":{{"zone_or_zonelist_or_space_or_spacelist_name":"actual zone","number_of_people_schedule_name":"on","activity_level_schedule_name":"activity","number_of_people":2,"sensible_heat_fraction":0.6,"work_efficiency_schedule_name":"work","clothing_insulation_schedule_name":"clothes","air_velocity_schedule_name":"velocity","surface_name_angle_factor_list_name":"unused surface","clothing_insulation_calculation_method_schedule_name":"unused method","thermal_comfort_model_{slot}_type":"Fanger"{auxiliary}}}}}}}"#
            );
            let raw = ep_raw_model::parse_epjson_str(&text)?;
            resolve_source_a(&raw)?;
            resolve_source_b(&raw)?;
            assert!(resolve(&raw).is_err());
            for ip in [
                crate::ordinary_input_processor::resolve_source_a(&raw)?,
                crate::ordinary_input_processor::resolve_source_b(&raw)?,
            ] {
                let original = ip.source_raw_object_fields("People", "Fanger person")?;
                let hb = crate::ordinary_heat_balance_input::resolve(&raw, &ip)?;
                for (name, expected) in [
                    ("work_efficiency_schedule_name", "work"),
                    ("clothing_insulation_schedule_name", "clothes"),
                    ("air_velocity_schedule_name", "velocity"),
                    ("surface_name_angle_factor_list_name", "unused surface"),
                    (
                        "clothing_insulation_calculation_method_schedule_name",
                        "unused method",
                    ),
                ] {
                    assert_eq!(original.get(name), Some(&V::String(expected.into())));
                    assert_eq!(hb.objects["People"][0].fields.get(name), Some(&ep_runtime::heat_balance::selected_project_data::ProjectInputValue::String(expected.into())));
                }
                if slot == 7 {
                    assert_eq!(
                        original.get("ankle_level_air_velocity_schedule_name"),
                        Some(&V::String("ankle".into()))
                    );
                }
            }
            let result = ep_compiler::compile_raw_model(&raw);
            assert!(
                !result.has_errors(),
                "Fanger slot{slot}: {:?}",
                result.report.diagnostics
            );
            for (old, new) in [
                ("\"Fanger\"", "\"Pierce\""),
                (
                    "\"work_efficiency_schedule_name\":\"work\"",
                    "\"work_efficiency_schedule_name\":\"missing work\"",
                ),
                (
                    "\"clothing_insulation_schedule_name\":\"clothes\"",
                    "\"clothing_insulation_schedule_name\":\"\"",
                ),
                (
                    "\"air_velocity_schedule_name\":\"velocity\"",
                    "\"air_velocity_schedule_name\":null",
                ),
            ] {
                let changed = text.replace(old, new);
                assert_ne!(changed, text);
                let raw = ep_raw_model::parse_epjson_str(&changed)?;
                assert!(resolve_source_a(&raw).is_err());
                assert!(resolve_source_b(&raw).is_err());
            }
            if slot == 7 {
                for (old, new) in [
                    ("EnclosureAveraged", "SurfaceWeighted"),
                    ("EnclosureAveraged", "AngleFactor"),
                    (
                        "\"ankle_level_air_velocity_schedule_name\":\"ankle\"",
                        "\"ankle_level_air_velocity_schedule_name\":\"missing ankle\"",
                    ),
                ] {
                    let changed = text.replace(old, new);
                    assert_ne!(changed, text);
                    let raw = ep_raw_model::parse_epjson_str(&changed)?;
                    assert!(resolve_source_a(&raw).is_err());
                    assert!(resolve_source_b(&raw).is_err());
                }
            }
        }
        // Either supported model consumes genuine shared auxiliaries. Ankle
        // can occupy every model slot, alone or alongside Fanger; duplicates
        // preserve the same raw operands rather than inventing extra objects.
        let mut ankle_models = Vec::new();
        for slot in 1..=7 {
            for with_fanger in [false, true] {
                let mut models =
                    format!("\"thermal_comfort_model_{slot}_type\":\"AnkleDraftASH55\"");
                if with_fanger {
                    let fanger_slot = if slot == 7 { 1 } else { slot + 1 };
                    models.push_str(&format!(
                        ",\"thermal_comfort_model_{fanger_slot}_type\":\"Fanger\""
                    ));
                }
                ankle_models.push(models);
            }
        }
        ankle_models.push(
            (1..=7)
                .map(|slot| format!("\"thermal_comfort_model_{slot}_type\":\"AnkleDraftASH55\""))
                .collect::<Vec<_>>()
                .join(","),
        );
        for models in ankle_models {
            let text = format!(
                r#"{{"Zone":{{"actual zone":{{}}}},"Schedule:Constant":{{"on":{{"hourly_value":1}},"activity":{{"hourly_value":120}},"work":{{"hourly_value":0.2}},"clothes":{{"hourly_value":0.5}},"velocity":{{"hourly_value":0.1}},"ankle":{{"hourly_value":-0.4}}}},"People":{{"Ankle person":{{"zone_or_zonelist_or_space_or_spacelist_name":"actual zone","number_of_people_schedule_name":"on","activity_level_schedule_name":"activity","number_of_people":2,"sensible_heat_fraction":0.6,"work_efficiency_schedule_name":"work","clothing_insulation_schedule_name":"clothes","air_velocity_schedule_name":"velocity","ankle_level_air_velocity_schedule_name":"ankle",{models}}}}}}}"#
            );
            let raw = ep_raw_model::parse_epjson_str(&text)?;
            resolve_source_a(&raw)?;
            resolve_source_b(&raw)?;
            assert!(resolve(&raw).is_err());
            for ip in [
                crate::ordinary_input_processor::resolve_source_a(&raw)?,
                crate::ordinary_input_processor::resolve_source_b(&raw)?,
            ] {
                let original = ip.source_raw_object_fields("People", "Ankle person")?;
                let hb = crate::ordinary_heat_balance_input::resolve(&raw, &ip)?;
                for (name, expected) in [
                    ("work_efficiency_schedule_name", "work"),
                    ("clothing_insulation_schedule_name", "clothes"),
                    ("air_velocity_schedule_name", "velocity"),
                    ("ankle_level_air_velocity_schedule_name", "ankle"),
                ] {
                    assert_eq!(original.get(name), Some(&V::String(expected.into())));
                    assert_eq!(
                        hb.objects["People"][0].fields.get(name),
                        Some(&ep_runtime::heat_balance::selected_project_data::ProjectInputValue::String(expected.into()))
                    );
                }
                for slot in 1..=7 {
                    let field = format!("thermal_comfort_model_{slot}_type");
                    if let Some(V::String(model)) = original.get(&field) {
                        assert!(model == "AnkleDraftASH55" || model == "Fanger");
                        assert_eq!(hb.objects["People"][0].fields.get(&field),
                            Some(&ep_runtime::heat_balance::selected_project_data::ProjectInputValue::String(model.clone())));
                    }
                }
            }
            // Check the required A21 boundary once, after a genuine selected
            // model and all other references have passed both actual IP gates.
            if models == "\"thermal_comfort_model_1_type\":\"AnkleDraftASH55\"" {
                for replacement in [
                    "",
                    ",\"ankle_level_air_velocity_schedule_name\":\"\"",
                    ",\"ankle_level_air_velocity_schedule_name\":\"missing ankle\"",
                    ",\"ankle_level_air_velocity_schedule_name\":null",
                ] {
                    let changed = text.replace(
                        ",\"ankle_level_air_velocity_schedule_name\":\"ankle\"",
                        replacement,
                    );
                    assert_ne!(changed, text);
                    let raw = ep_raw_model::parse_epjson_str(&changed)?;
                    assert!(resolve_source_a(&raw).is_err());
                    assert!(resolve_source_b(&raw).is_err());
                    assert!(crate::ordinary_input_processor::resolve_source_a(&raw).is_err());
                    assert!(crate::ordinary_input_processor::resolve_source_b(&raw).is_err());
                }
                let changed = text.replace(&format!(",{models}"), "").replace(
                    "\"ankle_level_air_velocity_schedule_name\":\"ankle\"",
                    "\"ankle_level_air_velocity_schedule_name\":\"unused missing ankle\"",
                );
                let raw = ep_raw_model::parse_epjson_str(&changed)?;
                resolve_source_a(&raw)?;
                resolve_source_b(&raw)?;
                for ip in [
                    crate::ordinary_input_processor::resolve_source_a(&raw)?,
                    crate::ordinary_input_processor::resolve_source_b(&raw)?,
                ] {
                    let original = ip.source_raw_object_fields("People", "Ankle person")?;
                    assert_eq!(
                        original.get("ankle_level_air_velocity_schedule_name"),
                        Some(&V::String("unused missing ankle".into()))
                    );
                    crate::ordinary_heat_balance_input::resolve(&raw, &ip)?;
                }
            }
        }
        // Clothing selection controls which raw references the real getter
        // consumes. CalculationSchedule defers A12 to its actual hasVal(1).
        for (method, method_schedule, clothing_schedule) in [
            (
                "DynamicClothingModelASHRAE55",
                "missing unused method",
                Some("missing unused clothing"),
            ),
            ("DynamicClothingModelASHRAE55", "", None),
            (
                "CalculationMethodSchedule",
                "method one",
                Some("wide clothes"),
            ),
            ("CalculationMethodSchedule", "method two", None),
        ] {
            let clothing_member = clothing_schedule.map_or_else(String::new, |name| {
                format!(r#","clothing_insulation_schedule_name":"{name}""#)
            });
            let text = format!(
                r#"{{"Zone":{{"actual zone":{{}}}},"Schedule:Constant":{{"on":{{"hourly_value":1}},"activity":{{"hourly_value":120}},"work":{{"hourly_value":0.2}},"velocity":{{"hourly_value":0.1}},"method one":{{"hourly_value":1}},"method two":{{"hourly_value":2}},"wide clothes":{{"hourly_value":3}}}},"People":{{"Clothing person":{{"zone_or_zonelist_or_space_or_spacelist_name":"actual zone","number_of_people_schedule_name":"on","activity_level_schedule_name":"activity","number_of_people":2,"sensible_heat_fraction":0.6,"thermal_comfort_model_1_type":"Fanger","mean_radiant_temperature_calculation_type":"EnclosureAveraged","work_efficiency_schedule_name":"work","air_velocity_schedule_name":"velocity","clothing_insulation_calculation_method":"{method}","clothing_insulation_calculation_method_schedule_name":"{method_schedule}"{clothing_member}}}}}}}"#
            );
            let raw = ep_raw_model::parse_epjson_str(&text)?;
            resolve_source_a(&raw)?;
            resolve_source_b(&raw)?;
            assert!(resolve(&raw).is_err());
            let compiled = ep_compiler::compile_raw_model(&raw);
            assert!(
                !compiled.has_errors(),
                "clothing {method}: {:?}",
                compiled.report.diagnostics
            );
            for ip in [
                crate::ordinary_input_processor::resolve_source_a(&raw)?,
                crate::ordinary_input_processor::resolve_source_b(&raw)?,
            ] {
                let original = ip.source_raw_object_fields("People", "Clothing person")?;
                let hb = crate::ordinary_heat_balance_input::resolve(&raw, &ip)?;
                let row = &hb.objects["People"][0];
                for (field, value) in [
                    ("clothing_insulation_calculation_method", method),
                    (
                        "clothing_insulation_calculation_method_schedule_name",
                        method_schedule,
                    ),
                ] {
                    assert_eq!(original.get(field), Some(&V::String(value.into())));
                    assert_eq!(row.fields.get(field), Some(&ep_runtime::heat_balance::selected_project_data::ProjectInputValue::String(value.into())));
                }
                let field = "clothing_insulation_schedule_name";
                if let Some(name) = clothing_schedule {
                    assert_eq!(original.get(field), Some(&V::String(name.into())));
                    assert_eq!(row.fields.get(field), Some(&ep_runtime::heat_balance::selected_project_data::ProjectInputValue::String(name.into())));
                } else {
                    assert!(original.get(field).is_none());
                    assert!(!row.fields.contains_key(field));
                }
            }
            if method == "CalculationMethodSchedule" {
                let changed = text.replace(
                    &format!(r#""clothing_insulation_calculation_method_schedule_name":"{method_schedule}""#),
                    r#""clothing_insulation_calculation_method_schedule_name":"absent method""#,
                );
                assert_ne!(changed, text);
                let invalid = ep_raw_model::parse_epjson_str(&changed)?;
                assert!(resolve_source_a(&invalid).is_err());
                assert!(resolve_source_b(&invalid).is_err());
            }
        }
        // SurfaceWeighted retains the actual opaque surface name for the real
        // getter/enclosure lookup; admission never invents a surface ordinal.
        let surface_weighted = r#"{
            "Zone":{"actual zone":{}},
            "Material:NoMass":{"Wall material":{"roughness":"Rough","thermal_resistance":1}},
            "Construction":{"Wall construction":{"outside_layer":"Wall material"}},
            "BuildingSurface:Detailed":{"Wall One":{
                "surface_type":"Wall","construction_name":"Wall construction",
                "zone_name":"actual zone","outside_boundary_condition":"Outdoors",
                "vertices":[
                    {"vertex_x_coordinate":0,"vertex_y_coordinate":0,"vertex_z_coordinate":0},
                    {"vertex_x_coordinate":1,"vertex_y_coordinate":0,"vertex_z_coordinate":0},
                    {"vertex_x_coordinate":1,"vertex_y_coordinate":0,"vertex_z_coordinate":1},
                    {"vertex_x_coordinate":0,"vertex_y_coordinate":0,"vertex_z_coordinate":1}]}},
            "Schedule:Constant":{"on":{"hourly_value":1},"activity":{"hourly_value":120},
                "work":{"hourly_value":0.2},"clothes":{"hourly_value":0.7},"velocity":{"hourly_value":0.1}},
            "People":{"Surface person":{
                "zone_or_zonelist_or_space_or_spacelist_name":"actual zone",
                "number_of_people_schedule_name":"on","activity_level_schedule_name":"activity",
                "number_of_people":2,"sensible_heat_fraction":0.6,"thermal_comfort_model_1_type":"Fanger",
                "mean_radiant_temperature_calculation_type":"SurfaceWeighted",
                "surface_name_angle_factor_list_name":"wALL oNE",
                "work_efficiency_schedule_name":"work","clothing_insulation_schedule_name":"clothes",
                "air_velocity_schedule_name":"velocity"}}
        }"#;
        let raw = ep_raw_model::parse_epjson_str(surface_weighted)?;
        resolve_source_a(&raw)?;
        resolve_source_b(&raw)?;
        assert!(resolve(&raw).is_err());
        let compiled = ep_compiler::compile_raw_model(&raw);
        assert!(!compiled.has_errors(), "{:?}", compiled.report.diagnostics);
        let typed = compiled.model.ok_or("actual SurfaceWeighted typed model")?;
        assert_eq!(typed.surfaces.len(), 1);
        assert_eq!(typed.surfaces[0].name.0, "WALL ONE");
        for ip in [
            crate::ordinary_input_processor::resolve_source_a(&raw)?,
            crate::ordinary_input_processor::resolve_source_b(&raw)?,
        ] {
            assert_eq!(
                ip.source_raw_object_names("BuildingSurface:Detailed")?,
                ["Wall One"]
            );
            let original = ip.source_raw_object_fields("People", "Surface person")?;
            let hb = crate::ordinary_heat_balance_input::resolve(&raw, &ip)?;
            for (name, value) in [
                (
                    "mean_radiant_temperature_calculation_type",
                    "SurfaceWeighted",
                ),
                ("surface_name_angle_factor_list_name", "wALL oNE"),
            ] {
                assert_eq!(original.get(name), Some(&V::String(value.into())));
                assert_eq!(
                    hb.objects["People"][0].fields.get(name),
                    Some(
                        &ep_runtime::heat_balance::selected_project_data::ProjectInputValue::String(
                            value.into()
                        )
                    )
                );
            }
        }
        for (old, new) in [
            (
                r#""surface_name_angle_factor_list_name":"wALL oNE""#,
                r#""surface_name_angle_factor_list_name":"missing surface""#,
            ),
            (
                r#""surface_name_angle_factor_list_name":"wALL oNE""#,
                r#""surface_name_angle_factor_list_name":"""#,
            ),
            (
                r#""surface_name_angle_factor_list_name":"wALL oNE""#,
                r#""surface_name_angle_factor_list_name":null"#,
            ),
            (r#""surface_name_angle_factor_list_name":"wALL oNE","#, ""),
            ("SurfaceWeighted", "AngleFactor"),
        ] {
            let changed = surface_weighted.replace(old, new);
            assert_ne!(changed, surface_weighted);
            let raw = ep_raw_model::parse_epjson_str(&changed)?;
            assert!(resolve_source_a(&raw).is_err());
            assert!(resolve_source_b(&raw).is_err());
        }
        // The genuine extensible IP owner retains lexical numbers and all
        // metadata kinds; no angle becomes a scalar HB getter or typed list.
        let angle_zone_only = ep_raw_model::parse_epjson_str(r#"{"Zone":{"actual zone":{}}}"#)?;
        let angle_zone_ip = crate::ordinary_input_processor::resolve_source_a(&angle_zone_only)?;
        let mut angle_zone_stats =
            ep_runtime::heat_balance::source_output_prescan::SourceOutputPreScan::default();
        angle_zone_ip.report_source_idf_records_stats(&mut angle_zone_stats)?;
        for (token, metadata, expected) in [
            ("0", "null", V::Null),
            ("1", "true", V::Bool(true)),
            ("-0", "0.25", V::Number("0.25".into())),
            ("-0.0", r#""text""#, V::String("text".into())),
            ("-0e0", "[]", V::Array(Vec::new())),
            ("1E-9999", "{}", V::Object(Default::default())),
        ] {
            let text = format!(
                r#"{{"Zone":{{"actual zone":{{}}}},"ComfortViewFactorAngles":{{"list":{{"metadata":{metadata},"angles":[{{"surface_name":"Wall One","angle_factor":{token},"metadata":{metadata}}}]}}}}}}"#
            );
            let raw = ep_raw_model::parse_epjson_str(&text)?;
            resolve_source_a(&raw)?;
            resolve_source_b(&raw)?;
            assert!(resolve(&raw).is_err());
            assert_eq!(
                ep_compiler::typed_coverage_status("ComfortViewFactorAngles"),
                ep_compiler::ObjectCoverageStatus::RawOnly
            );
            for mut ip in [
                crate::ordinary_input_processor::resolve_source_a(&raw)?,
                crate::ordinary_input_processor::resolve_source_b(&raw)?,
            ] {
                assert_eq!(
                    ip.source_raw_object_names("ComfortViewFactorAngles")?,
                    ["list"]
                );
                let fields = ip.source_raw_object_fields("ComfortViewFactorAngles", "list")?;
                assert_eq!(fields.get("metadata"), Some(&expected));
                let Some(V::Array(rows)) = fields.get("angles") else {
                    return Err("actual raw angles".into());
                };
                let V::Object(row) = &rows[0] else {
                    return Err("actual raw angle row".into());
                };
                assert_eq!(row.get("angle_factor"), Some(&V::Number(token.into())));
                assert_eq!(row.get("surface_name"), Some(&V::String("Wall One".into())));
                assert_eq!(row.get("metadata"), Some(&expected));
                let counts = ip.get_object_def_max_args("ComfortViewFactorAngles")?;
                assert_eq!((counts.alphas, counts.numerics), (2, 1));
                let mut stats =
                    ep_runtime::heat_balance::source_output_prescan::SourceOutputPreScan::default();
                ip.report_source_idf_records_stats(&mut stats)?;
                assert_eq!(
                    stats.number_input_records,
                    angle_zone_stats.number_input_records + 1
                );
                assert_eq!(
                    (
                        stats.total_fields_with_defaults,
                        stats.number_defaulted_fields,
                        stats.total_autosizable_fields,
                        stats.total_autocalculatable_fields
                    ),
                    (
                        angle_zone_stats.total_fields_with_defaults,
                        angle_zone_stats.number_defaulted_fields,
                        angle_zone_stats.total_autosizable_fields,
                        angle_zone_stats.total_autocalculatable_fields
                    )
                );
            }
        }
        // Omitted members and empty arrays stay raw, so the real NumNumbers-
        // driven consumer owns any later sum/surface diagnostics.
        for fields in [
            "{}",
            r#"{"angles":[]}"#,
            r#"{"angles":[{}]}"#,
            r#"{"angles":[{"surface_name":""},{"angle_factor":-0.0}]}"#,
        ] {
            let text = format!(
                r#"{{"Zone":{{"actual zone":{{}}}},"ComfortViewFactorAngles":{{"":{fields}}}}}"#
            );
            let raw = ep_raw_model::parse_epjson_str(&text)?;
            resolve_source_a(&raw)?;
            resolve_source_b(&raw)?;
        }
        let many = vec![r#"{"surface_name":"Wall One","angle_factor":0.01}"#; 100].join(",");
        let text = format!(
            r#"{{"Zone":{{"actual zone":{{}}}},"ComfortViewFactorAngles":{{"many":{{"angles":[{many}]}}}}}}"#
        );
        let raw = ep_raw_model::parse_epjson_str(&text)?;
        for mut ip in [
            crate::ordinary_input_processor::resolve_source_a(&raw)?,
            crate::ordinary_input_processor::resolve_source_b(&raw)?,
        ] {
            let counts = ip.get_object_def_max_args("ComfortViewFactorAngles")?;
            assert_eq!((counts.alphas, counts.numerics), (101, 100));
        }
        for angles in [
            "null",
            "{}",
            "[null]",
            r#"[{"surface_name":7}]"#,
            r#"[{"angle_factor":null}]"#,
            r#"[{"angle_factor":"0.5"}]"#,
            r#"[{"angle_factor":-0.1}]"#,
            r#"[{"angle_factor":1.1}]"#,
        ] {
            let text = format!(
                r#"{{"Zone":{{"actual zone":{{}}}},"ComfortViewFactorAngles":{{"bad":{{"angles":{angles}}}}}}}"#
            );
            let raw = ep_raw_model::parse_epjson_str(&text)?;
            assert!(resolve_source_a(&raw).is_err());
            assert!(resolve_source_b(&raw).is_err());
        }
        // Original list identities are not normalized. Blank/space A8 and
        // case-variant lists reach the actual later association in list order.
        for (name, reference) in [
            ("", Some("")),
            ("", None),
            (" ", Some(" ")),
            ("List", Some("lIST")),
        ] {
            let mut value: serde_json::Value = serde_json::from_str(surface_weighted)?;
            value["People"]["Surface person"]["mean_radiant_temperature_calculation_type"] =
                "AngleFactor".into();
            let person = value["People"]["Surface person"]
                .as_object_mut()
                .ok_or("authored AngleFactor person")?;
            if let Some(reference) = reference {
                person.insert(
                    "surface_name_angle_factor_list_name".into(),
                    reference.into(),
                );
            } else {
                person.remove("surface_name_angle_factor_list_name");
            }
            value["ComfortViewFactorAngles"] = serde_json::json!({(name):{"angles":[{"surface_name":"Wall One","angle_factor":1}]}});
            let raw = ep_raw_model::parse_epjson_str(&value.to_string())?;
            resolve_source_a(&raw)?;
            resolve_source_b(&raw)?;
            for ip in [
                crate::ordinary_input_processor::resolve_source_a(&raw)?,
                crate::ordinary_input_processor::resolve_source_b(&raw)?,
            ] {
                assert_eq!(
                    ip.source_raw_object_names("ComfortViewFactorAngles")?,
                    [name]
                );
            }
        }
        let names = ep_raw_model::parse_epjson_str(
            r#"{"Zone":{"actual zone":{}},"ComfortViewFactorAngles":{"a":{},"A":{},"":{}," ":{}}}"#,
        )?;
        for ip in [
            crate::ordinary_input_processor::resolve_source_a(&names)?,
            crate::ordinary_input_processor::resolve_source_b(&names)?,
        ] {
            assert_eq!(
                ip.source_raw_object_names("ComfortViewFactorAngles")?,
                ["", " ", "A", "a"]
            );
        }
        for (field, value) in [
            ("number_of_people", serde_json::json!(-1)),
            ("people_per_floor_area", serde_json::json!(-1)),
            ("floor_area_per_person", serde_json::json!(-1)),
            ("sensible_heat_fraction", serde_json::json!(null)),
            ("sensible_heat_fraction", serde_json::json!(1.1)),
            ("sensible_heat_fraction", serde_json::json!("0.6")),
            (
                "sensible_heat_fraction",
                serde_json::json!("AutocalculateX"),
            ),
            (
                "number_of_people_calculation_method",
                serde_json::json!("PeopleX"),
            ),
            ("thermal_comfort_model_1_type", serde_json::json!("Fanger")),
            (
                "mean_radiant_temperature_calculation_type",
                serde_json::json!("invalid MRT enum"),
            ),
            (
                "clothing_insulation_calculation_method",
                serde_json::json!("invalid clothing enum"),
            ),
            ("work_efficiency_schedule_name", serde_json::json!(1)),
            ("idf_max_fields", serde_json::json!(-1)),
        ] {
            let mut value_model = serde_json::json!({"Zone":{"actual zone":{}},"Schedule:Constant":{"on":{"hourly_value":1},"activity":{"hourly_value":120}},"People":{"P":{"zone_or_zonelist_or_space_or_spacelist_name":"actual zone","number_of_people_schedule_name":"on","activity_level_schedule_name":"activity","number_of_people":1,"sensible_heat_fraction":0.6}}});
            value_model["People"]["P"][field] = value;
            let raw = ep_raw_model::parse_epjson_str(&value_model.to_string())?;
            assert!(resolve_source_a(&raw).is_err());
            assert!(resolve_source_b(&raw).is_err());
        }
        Ok(())
    }
}
