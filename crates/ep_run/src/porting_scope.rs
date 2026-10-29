//! Opt-in input boundary for the two CON-01 porting paths.
//!
//! Admission records input/settings scope, not numerical conformance. Ordinary
//! arbitrary runs keep their existing support assessment and runtime behavior.

mod environment;
mod settings;
mod surface_pairs;
mod zone_groups;

use ep_compiler::CompileReport;
use ep_model::{
    AutosizeOrNumber, DehumidificationControlType, DemandControlledVentilationType,
    HeatRecoveryType, HumidificationControlType, IdealLoadsLimit, InsideSurfaceConvectionAlgorithm,
    OutdoorAirEconomizerType, OutsideBoundaryCondition, OutsideSurfaceConvectionAlgorithm,
    SolarDistribution, TypedModel,
};
use ep_raw_model::RawModel;
use serde::Serialize;
use serde_json::{Value, json};
use std::collections::BTreeMap;

pub(crate) use environment::{
    porting_source_environment_trace, source_coverage_axis, validate_source_environment,
};

/// Frozen input family selected for card-based porting work.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum PortingScope {
    /// Unconditioned opaque Zones; multiple Zones require bounded Source topology and gain admission.
    A,
    /// One fully mixed zone with direct no-OA sensible IdealLoads and hard limits.
    B,
}

impl PortingScope {
    /// Parses a scope letter, ignoring case and surrounding whitespace.
    #[must_use]
    pub fn parse(value: &str) -> Option<Self> {
        match value.trim().to_ascii_uppercase().as_str() {
            "A" => Some(Self::A),
            "B" => Some(Self::B),
            _ => None,
        }
    }

    /// Stable scope identifier.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::A => "A",
            Self::B => "B",
        }
    }
}

/// Actual compiled input boundary and settings exposed before runtime dispatch.
#[derive(Clone, Debug, Serialize)]
pub struct PortingScopeTrace {
    /// Version of this bounded input trace.
    pub schema: &'static str,
    /// Selected input family.
    pub scope: PortingScope,
    /// Whether the input is inside the selected card scope.
    pub admissible: bool,
    /// Reasons that prevent bounded execution.
    pub violations: Vec<String>,
    /// Effective typed settings and fixed source branches used by this runtime.
    pub settings: Value,
    /// Defaults actually recorded by the existing compiler.
    pub compile_defaults: Vec<Value>,
    /// Original object counts, including disabled/reporting declarations.
    pub object_counts: BTreeMap<String, usize>,
    /// Declared branches excluded from the thermal computation.
    pub inactive_branches: Vec<String>,
    /// Resolved EPW/calendar input state, when weather input is provided.
    pub environment: Option<Value>,
    /// Input provenance and the limits of this admission result.
    pub production: Value,
}

/// Inspects compiled inputs without running EP or reading any expected results.
#[must_use]
pub fn inspect_porting_scope(
    raw: &RawModel,
    report: &CompileReport,
    model: Option<&TypedModel>,
    scope: PortingScope,
) -> PortingScopeTrace {
    let mut violations = Vec::new();
    let settings = settings::project_settings(raw, model, &mut violations);
    let object_counts: BTreeMap<_, _> = raw
        .objects
        .iter()
        .map(|(kind, objects)| (kind.0.clone(), objects.len()))
        .collect();
    let mut inactive_branches = Vec::new();
    for (kind, objects) in &raw.objects {
        if !objects.is_empty() && !permitted_object(&kind.0, scope) {
            violations.push(format!("{} is outside scope {}", kind.0, scope.id()));
        }
    }
    if scope == PortingScope::A
        && (raw
            .objects
            .get(&ep_raw_model::ObjectType("Zone".into()))
            .is_some_and(|rows| rows.len() > 1)
            || raw
                .objects
                .get(&ep_raw_model::ObjectType("ZoneList".into()))
                .is_some_and(|rows| !rows.is_empty())
            || raw
                .objects
                .get(&ep_raw_model::ObjectType("ZoneGroup".into()))
                .is_some_and(|rows| !rows.is_empty())
            || raw
                .objects
                .get(&ep_raw_model::ObjectType("Space".into()))
                .is_some_and(|rows| !rows.is_empty())
            || raw
                .objects
                .get(&ep_raw_model::ObjectType("SpaceList".into()))
                .is_some_and(|rows| !rows.is_empty()))
        && let Err(error) = crate::ordinary_equipment_input::resolve_source_a(raw)
    {
        violations.push(error);
    }
    validate_project_controls(raw, &settings, &mut violations);
    if object_counts.contains_key("SizingPeriod:DesignDay")
        && settings["simulation_control"]["run_simulation_for_sizing_periods"] == false
    {
        inactive_branches.push("SizingPeriod:DesignDay (sizing-period execution disabled)".into());
    }
    if object_counts.contains_key("Exterior:Lights") {
        inactive_branches
            .push("Exterior:Lights (external meter only; excluded from zone heat sources)".into());
    }
    if let Some(model) = model {
        if scope == PortingScope::A
            && (!model.zone_groups.is_empty()
                || raw
                    .objects
                    .get(&ep_raw_model::ObjectType("ZoneGroup".into()))
                    .is_some_and(|rows| !rows.is_empty()))
        {
            zone_groups::validate(raw, model, &mut violations);
        }
        if scope == PortingScope::A
            && raw
                .objects
                .get(&ep_raw_model::ObjectType("SpaceList".into()))
                .is_some_and(|rows| !rows.is_empty())
        {
            validate_empty_source_gain_bindings(raw, model, &mut violations);
        }
        validate_typed_scope(model, scope, &mut violations);
        if object_counts.contains_key("RoomAirModelType") {
            if settings["room_air_controls"]["air_temperature_coupling_strategy"] != "Direct" {
                violations
                    .push("RoomAirModelType coupling must use the Direct default branch".into());
            }
            let named = settings["room_air_controls"]["zone_name"].as_str();
            if !model
                .zones
                .iter()
                .any(|zone| named.is_some_and(|name| name.eq_ignore_ascii_case(&zone.name.0)))
            {
                violations.push("RoomAirModelType must name the selected zone".into());
            }
        }
    } else {
        violations.push("typed compilation failed; bounded runtime cannot execute".into());
    }
    let compile_defaults = report
        .defaults_applied
        .iter()
        .map(|default| {
            json!({
                "object_type": default.object_type, "object_name": default.object_name,
                "field": default.field, "value": default.value,
            })
        })
        .collect();
    PortingScopeTrace {
        schema: "porting-scope.v1",
        scope,
        admissible: violations.is_empty(),
        violations,
        settings,
        compile_defaults,
        object_counts,
        inactive_branches,
        environment: None,
        production: json!({
            "calculation_input_source": "original input and EPW; Rust compiled state",
            "oracle_inputs_used": false, "fixture_inputs_used": false,
            "conformance_claim": false,
            "admission_semantics": "input scope only; unfinished numerical cards remain unconfirmed",
            "excluded_reports": ["Exterior:Lights energy and nonselected meters"],
        }),
    }
}

fn permitted_object(kind: &str, scope: PortingScope) -> bool {
    if kind.starts_with("Output:") || kind.starts_with("OutputControl:") {
        return true;
    }
    matches!(
        kind,
        "Version"
            | "SimulationControl"
            | "Building"
            | "Timestep"
            | "RunPeriod"
            | "Site:Location"
            | "Site:GroundTemperature:BuildingSurface"
            | "SizingPeriod:DesignDay"
            | "Exterior:Lights"
            | "GlobalGeometryRules"
            | "HeatBalanceAlgorithm"
            | "ZoneAirHeatBalanceAlgorithm"
            | "SurfaceConvectionAlgorithm:Inside"
            | "SurfaceConvectionAlgorithm:Outside"
            | "RoomAirModelType"
            | "Material"
            | "Material:NoMass"
            | "Material:AirGap"
            | "Material:InfraredTransparent"
            | "Construction"
            | "Zone"
            | "BuildingSurface:Detailed"
            | "OtherEquipment"
            | "People"
            | "ComfortViewFactorAngles"
            | "ScheduleTypeLimits"
            | "Schedule:Constant"
            | "Schedule:Compact"
            | "Schedule:File"
            | "Schedule:Day:Hourly"
            | "Schedule:Day:Interval"
            | "Schedule:Day:List"
            | "Schedule:Week:Compact"
            | "Schedule:Week:Daily"
            | "Schedule:Year"
            | "RunPeriodControl:DaylightSavingTime"
            | "RunPeriodControl:SpecialDays"
    ) || (scope == PortingScope::A
        && matches!(kind, "ZoneList" | "ZoneGroup" | "Space" | "SpaceList"))
        || (scope == PortingScope::B
            && matches!(
                kind,
                "ZoneControl:Thermostat"
                    | "ThermostatSetpoint:DualSetpoint"
                    | "ZoneHVAC:EquipmentConnections"
                    | "ZoneHVAC:EquipmentList"
                    | "ZoneHVAC:IdealLoadsAirSystem"
                    | "NodeList"
            ))
}

fn validate_project_controls(raw: &RawModel, settings: &Value, violations: &mut Vec<String>) {
    for kind in [
        "SimulationControl",
        "HeatBalanceAlgorithm",
        "ZoneAirHeatBalanceAlgorithm",
        "RoomAirModelType",
    ] {
        if raw
            .objects
            .iter()
            .any(|(key, values)| key.0 == kind && values.len() > 1)
        {
            violations.push(format!("{kind} must have at most one declaration"));
        }
    }
    let control = &settings["simulation_control"];
    for field in [
        "do_zone_sizing_calculation",
        "do_system_sizing_calculation",
        "do_plant_sizing_calculation",
    ] {
        if control[field] == true {
            violations.push(format!(
                "SimulationControl.{field}=Yes is outside the weather-run scope"
            ));
        }
    }
    // Original GetProjectData1146-1158 resets HVAC sizing to No when
    // plant sizing is No. This changes only admission: preserve the authored
    // request for the real getter and its warning/continue messages.
    if control["do_hvac_sizing_simulation_for_sizing_periods"] == true
        && control["do_plant_sizing_calculation"] == true
    {
        violations.push(
            "SimulationControl.do_hvac_sizing_simulation_for_sizing_periods=Yes is outside the weather-run scope".into(),
        );
    }
    let design_days = raw
        .objects
        .iter()
        .any(|(key, values)| key.0 == "SizingPeriod:DesignDay" && !values.is_empty());
    if design_days && control["run_simulation_for_sizing_periods"] == true {
        violations
            .push("active sizing-period environments are outside the weather-run scope".into());
    }
    if control["run_simulation_for_weather_file_run_periods"] != true {
        violations.push("weather-file RunPeriod execution must be enabled".into());
    }
    for (field, required) in [
        ("heat_balance_algorithm", "ConductionTransferFunction"),
        (
            "zone_air_heat_balance_algorithm",
            "ThirdOrderBackwardDifference",
        ),
        ("room_air_model_type", "Mixing"),
    ] {
        if settings[field] != required {
            violations.push(format!("{field} must be {required} in the selected scope"));
        }
    }
    for (field, minimum, exclusive) in [
        ("surface_temperature_upper_limit", 200.0, false),
        (
            "minimum_surface_convection_heat_transfer_coefficient_value",
            0.0,
            true,
        ),
        (
            "maximum_surface_convection_heat_transfer_coefficient_value",
            1.0,
            false,
        ),
    ] {
        let valid = settings["heat_balance_controls"][field]
            .as_f64()
            .is_some_and(|value| {
                value.is_finite()
                    && if exclusive {
                        value > minimum
                    } else {
                        value >= minimum
                    }
            });
        if !valid {
            violations.push(format!(
                "HeatBalanceAlgorithm.{field} is outside its numeric schema domain"
            ));
        }
    }
    for field in [
        "do_space_heat_balance_for_sizing",
        "do_space_heat_balance_for_simulation",
    ] {
        if settings["zone_air_controls"][field] == true {
            violations.push(format!(
                "ZoneAirHeatBalanceAlgorithm.{field}=Yes is outside the single-zone scope"
            ));
        }
    }
}

fn validate_empty_source_gain_bindings(
    raw: &RawModel,
    model: &TypedModel,
    violations: &mut Vec<String>,
) {
    let families = [
        (
            "People",
            model
                .source_people_inputs
                .iter()
                .map(|input| {
                    (
                        input.object_name.as_str(),
                        input.target,
                        input.instances.is_empty(),
                    )
                })
                .collect::<Vec<_>>(),
        ),
        (
            "OtherEquipment",
            model
                .source_other_equipment_inputs
                .iter()
                .map(|input| {
                    (
                        input.object_name.as_str(),
                        input.target,
                        input.instances.is_empty(),
                    )
                })
                .collect::<Vec<_>>(),
        ),
    ];
    for (family, bindings) in families {
        let Ok(rows) = raw.source_ordered_instances(family) else {
            violations.push(format!("Source {family} raw binding order is unavailable"));
            continue;
        };
        if rows.len() != bindings.len()
            || rows
                .iter()
                .zip(&bindings)
                .any(|((name, _), (bound, _, _))| name.0 != *bound)
        {
            violations.push(format!(
                "Source {family} bindings must retain every ordered raw statement"
            ));
            continue;
        }
        for ((_, row), (_, target, empty)) in rows.iter().zip(bindings) {
            if !empty {
                continue;
            }
            let valid = row.fields.get(&ep_raw_model::FieldName(
                "zone_or_zonelist_or_space_or_spacelist_name".into()))
                .is_some_and(|value| {
                    let ep_raw_model::RawValue::String(reference) = value else { return false; };
                    let higher_priority = ["Zone", "Space", "ZoneList"].iter().any(|family|
                        raw.objects.get(&ep_raw_model::ObjectType((*family).into()))
                            .is_some_and(|rows| rows.keys().any(|name| name.0.eq_ignore_ascii_case(reference))));
                    let raw_empty = raw.objects.get(&ep_raw_model::ObjectType("SpaceList".into()))
                        .and_then(|rows| rows.iter().find(|(name, _)| name.0.eq_ignore_ascii_case(reference)))
                        .is_some_and(|(_, list)| match list.fields.get(&ep_raw_model::FieldName("spaces".into())) {
                            None => true,
                            Some(ep_raw_model::RawValue::Array(members)) => members.is_empty(),
                            _ => false,
                        });
                    !higher_priority && raw_empty && matches!(target,
                        ep_model::SourceGainInputTarget::SpaceList(id)
                            if model.space_list_names.resolve(reference) == Some(id)
                                && model.space_lists.iter().any(|list| list.id == id && list.spaces.is_empty()))
                });
            if !valid {
                violations.push(format!(
                    "Source {family} empty range requires its actual raw empty SpaceList"
                ));
            }
        }
    }
}

fn validate_typed_scope(model: &TypedModel, scope: PortingScope, violations: &mut Vec<String>) {
    if model.version != ep_model::Version::oracle_26_1_0() {
        violations.push("effective engine must match the locked EnergyPlus 26.1.0".into());
    }
    if model.zones.is_empty() || (scope == PortingScope::B && model.zones.len() != 1) {
        violations
            .push("Source A requires positive opaque Zones; Source B requires exactly one".into());
    }
    let authored_spaces = model
        .spaces
        .iter()
        .any(|space| space.origin == ep_model::SpaceOrigin::Authored);
    if (model.zones.len() > 1
        || !model.zone_lists.is_empty()
        || !model.space_lists.is_empty()
        || authored_spaces)
        && (model.spaces.len() != model.zones.len()
            || model.zones.iter().any(|zone| {
                let spaces = model
                    .spaces
                    .iter()
                    .filter(|space| space.zone == zone.id)
                    .collect::<Vec<_>>();
                spaces.len() != 1
                    || zone.spaces.as_slice() != [spaces[0].id]
                    || !matches!(
                        spaces[0].origin,
                        ep_model::SpaceOrigin::AutoZoneDefault | ep_model::SpaceOrigin::Authored
                    )
                    || (scope != PortingScope::A
                        && spaces[0].origin == ep_model::SpaceOrigin::Authored)
            }))
    {
        violations.push(
            "bounded Source A requires exactly one whole-Zone Space per Zone and no remainder"
                .into(),
        );
    }
    if (model.zones.len() > 1 || !model.zone_lists.is_empty())
        && model.zones.iter().any(|zone| {
            !model.surfaces.iter().any(|surface| {
                surface.zone == zone.id && surface.surface_type != ep_model::SurfaceType::Floor
            })
        })
    {
        // The ungrouped Source enclosure loop is generic. A mixture of
        // floor-only and other Spaces enters a separate grouped/remainder
        // producer that remains outside this stage.
        violations.push("multi-Zone requires a non-floor opaque enclosure for every Zone".into());
    }
    if !model.zone_lists.is_empty() || !model.space_lists.is_empty() || authored_spaces {
        if scope != PortingScope::A {
            violations.push(
                "ZoneList, SpaceList and authored Space are admitted only by Source A".into(),
            );
        }
        let mut cursor = 0;
        for binding in &model.source_other_equipment_inputs {
            let range = binding.instances.clone();
            let empty_list = matches!(binding.target,
                ep_model::SourceOtherEquipmentInputTarget::SpaceList(id)
                    if model.space_lists.iter().any(|list| list.id == id && list.spaces.is_empty()));
            if range.start != cursor
                || range.start > range.end
                || (range.is_empty() && !empty_list)
                || range.end > model.other_equipment.len()
            {
                violations.push("Source OtherEquipment raw-instance range is unavailable".into());
                break;
            }
            let instances = &model.other_equipment[range.clone()];
            if instances
                .iter()
                .enumerate()
                .any(|(offset, row)| usize::try_from(row.id.0).ok() != Some(range.start + offset))
            {
                violations
                    .push("Source OtherEquipment instance IDs must remain real and dense".into());
            }
            match binding.target {
                ep_model::SourceOtherEquipmentInputTarget::Zone(zone) => {
                    if instances.len() != 1 || instances[0].zone != zone {
                        violations
                            .push("Source OtherEquipment direct Zone association differs".into());
                    }
                }
                ep_model::SourceOtherEquipmentInputTarget::Space(id) => {
                    if instances.len() != 1
                        || !model
                            .spaces
                            .iter()
                            .find(|space| space.id == id)
                            .is_some_and(|space| {
                                space.origin == ep_model::SpaceOrigin::Authored
                                    && instances[0].zone == space.zone
                            })
                    {
                        violations.push(
                            "Source OtherEquipment direct actual Space association differs".into(),
                        );
                    }
                }
                ep_model::SourceOtherEquipmentInputTarget::ZoneList(id) => {
                    if !model
                        .zone_lists
                        .iter()
                        .find(|list| list.id == id)
                        .is_some_and(|list| {
                            list.zones.len() == instances.len()
                                && list
                                    .zones
                                    .iter()
                                    .zip(instances)
                                    .all(|(zone, row)| *zone == row.zone)
                        })
                    {
                        violations.push(
                            "Source OtherEquipment ordered ZoneList association differs".into(),
                        );
                    }
                }
                ep_model::SourceOtherEquipmentInputTarget::SpaceList(id) => {
                    if !model
                        .space_lists
                        .iter()
                        .find(|list| list.id == id)
                        .is_some_and(|list| {
                            list.spaces.len() == instances.len()
                                && list.spaces.iter().zip(instances).all(|(id, row)| {
                                    model
                                        .spaces
                                        .iter()
                                        .find(|space| space.id == *id)
                                        .is_some_and(|space| {
                                            space.origin == ep_model::SpaceOrigin::Authored
                                                && row.zone == space.zone
                                                && row.name.0
                                                    == format!(
                                                        "{} {}",
                                                        space.name.0,
                                                        binding.object_name.to_ascii_uppercase()
                                                    )
                                        })
                                })
                        })
                    {
                        violations.push(
                            "Source OtherEquipment ordered SpaceList association differs".into(),
                        );
                    }
                }
            }
            cursor = range.end;
        }
        if cursor != model.other_equipment.len() {
            violations.push("Source OtherEquipment ranges must cover all actual instances".into());
        }
        let mut cursor = 0;
        for binding in &model.source_people_inputs {
            let range = binding.instances.clone();
            let empty_list = matches!(binding.target,
                ep_model::SourcePeopleInputTarget::SpaceList(id)
                    if model.space_lists.iter().any(|list| list.id == id && list.spaces.is_empty()));
            if range.start != cursor
                || range.start > range.end
                || (range.is_empty() && !empty_list)
                || range.end > model.people.len()
            {
                violations.push("Source People raw-instance range is unavailable".into());
                break;
            }
            let instances = &model.people[range.clone()];
            if instances
                .iter()
                .enumerate()
                .any(|(offset, row)| usize::try_from(row.id.0).ok() != Some(range.start + offset))
            {
                violations.push("Source People instance IDs must remain real and dense".into());
            }
            match binding.target {
                ep_model::SourcePeopleInputTarget::Zone(zone) => {
                    if instances.len() != 1 || instances[0].zone != zone {
                        violations.push("Source People direct Zone association differs".into());
                    }
                }
                ep_model::SourcePeopleInputTarget::Space(id) => {
                    if instances.len() != 1
                        || !model
                            .spaces
                            .iter()
                            .find(|space| space.id == id)
                            .is_some_and(|space| {
                                space.origin == ep_model::SpaceOrigin::Authored
                                    && instances[0].zone == space.zone
                            })
                    {
                        violations
                            .push("Source People direct actual Space association differs".into());
                    }
                }
                ep_model::SourcePeopleInputTarget::ZoneList(id) => {
                    if !model
                        .zone_lists
                        .iter()
                        .find(|list| list.id == id)
                        .is_some_and(|list| {
                            list.zones.len() == instances.len()
                                && list
                                    .zones
                                    .iter()
                                    .zip(instances)
                                    .all(|(zone, row)| *zone == row.zone)
                        })
                    {
                        violations
                            .push("Source People ordered ZoneList association differs".into());
                    }
                }
                ep_model::SourcePeopleInputTarget::SpaceList(id) => {
                    if !model
                        .space_lists
                        .iter()
                        .find(|list| list.id == id)
                        .is_some_and(|list| {
                            list.spaces.len() == instances.len()
                                && list.spaces.iter().zip(instances).all(|(id, row)| {
                                    model
                                        .spaces
                                        .iter()
                                        .find(|space| space.id == *id)
                                        .is_some_and(|space| {
                                            space.origin == ep_model::SpaceOrigin::Authored
                                                && row.zone == space.zone
                                                && row.name.0
                                                    == format!(
                                                        "{} {}",
                                                        space.name.0,
                                                        binding.object_name.to_ascii_uppercase()
                                                    )
                                        })
                                })
                        })
                    {
                        violations
                            .push("Source People ordered SpaceList association differs".into());
                    }
                }
            }
            cursor = range.end;
        }
        if cursor != model.people.len() {
            violations.push("Source People ranges must cover all actual instances".into());
        }
    }
    if model.building.is_none()
        || model.global_geometry_rules.is_none()
        || model.run_periods.len() != 1
    {
        violations
            .push("Building, GlobalGeometryRules and exactly one RunPeriod are required".into());
    }
    if !matches!(
        model.timestep.number_of_timesteps_per_hour,
        1 | 2 | 3 | 4 | 5 | 6 | 10 | 12 | 15 | 20 | 30 | 60
    ) {
        violations.push("Timestep must use an EnergyPlus divisor of 60 minutes".into());
    }
    if let Some(building) = &model.building {
        if !matches!(
            building.solar_distribution,
            SolarDistribution::MinimalShadowing
                | SolarDistribution::FullExterior
                | SolarDistribution::FullInteriorAndExterior
        ) {
            violations
                .push("solar reflection branches are outside the selected opaque scope".into());
        }
        if building.loads_convergence_tolerance_w <= 0.0
            || building.temperature_convergence_tolerance_delta_c <= 0.0
            || building.minimum_number_of_warmup_days == 0
            || building.maximum_number_of_warmup_days == 0
        {
            violations.push("warmup day counts and convergence tolerances must be positive".into());
        }
    }
    for zone in &model.zones {
        // The real getter stores Zone N6 in a Native signed int. All live
        // one-Zone consumers retain this factor; the schema itself has no max.
        // Keep the defined-conversion boundary and the independent group gate.
        if zone.multiplier == 0 || zone.multiplier > i32::MAX as u32 {
            violations.push("zone multiplier must fit a positive Native signed int".into());
        }
        if (scope != PortingScope::A || model.zone_groups.is_empty())
            && (zone.list_multiplier != 1 || zone.list_group.is_some())
        {
            violations.push(
                "ZoneGroup/list factors require a real validated Source A group assignment".into(),
            );
        }
        if i32::try_from(zone.multiplier)
            .ok()
            .zip(i32::try_from(zone.list_multiplier).ok())
            .and_then(|(zone, list)| zone.checked_mul(list))
            .is_none_or(|factor| factor < 1)
        {
            violations.push("Zone/group multiplier product must fit a positive Native int".into());
        }
        if !matches!(
            zone.inside_convection_algorithm.effective(),
            InsideSurfaceConvectionAlgorithm::Simple
                | InsideSurfaceConvectionAlgorithm::Tarp
                | InsideSurfaceConvectionAlgorithm::CeilingDiffuser
        ) || !matches!(
            zone.outside_convection_algorithm.effective(),
            OutsideSurfaceConvectionAlgorithm::SimpleCombined
                | OutsideSurfaceConvectionAlgorithm::Tarp
                | OutsideSurfaceConvectionAlgorithm::MoWitt
                | OutsideSurfaceConvectionAlgorithm::Doe2
        ) {
            violations.push("selected effective Zone convection requires Simple/TARP/CeilingDiffuser inside and a nonadaptive outside algorithm".into());
        }
    }
    for surface in &model.surfaces {
        if !(matches!(
            surface.outside_boundary_condition,
            OutsideBoundaryCondition::Outdoors
                | OutsideBoundaryCondition::Adiabatic
                | OutsideBoundaryCondition::Ground
        ) || (scope == PortingScope::A
            && model.zones.len() > 1
            && surface.outside_boundary_condition == OutsideBoundaryCondition::Surface))
        {
            violations.push(format!(
                "{} boundary must be Outdoors, Adiabatic or Ground",
                surface.name.0
            ));
        }
    }
    surface_pairs::validate(model, scope, violations);
    match scope {
        PortingScope::A => {
            if !model.ideal_loads_air_systems.is_empty() || !model.zone_thermostats.is_empty() {
                violations.push(
                    "conditioning/thermostat declarations are outside unconditioned scope A".into(),
                );
            }
        }
        PortingScope::B => validate_ideal_loads(model, violations),
    }
}

pub(crate) fn validate_porting_environment(environment: &Value) -> Vec<String> {
    let mut errors = Vec::new();
    if environment["daylight_saving"]["active"] == true {
        errors.push(
            "active daylight-saving periods are outside the fixed CON-01 calendar scope".into(),
        );
    }
    if environment["active_special_day_count"]
        .as_u64()
        .is_some_and(|count| count > 0)
    {
        errors.push("active special-day/holiday schedule overrides are outside the fixed CON-01 calendar scope".into());
    }
    errors
}

fn validate_ideal_loads(model: &TypedModel, violations: &mut Vec<String>) {
    if model.ideal_loads_air_systems.len() != 1 {
        violations.push("scope B requires exactly one direct IdealLoads system".into());
    }
    let simulation = ep_model::SimulationModel::from_typed(model.clone());
    if let Err(error) = ep_runtime::bind_direct_zone_purchased_air_model(&simulation) {
        violations.push(format!("direct IdealLoads binding failed: {error:?}"));
    }
    for system in &model.ideal_loads_air_systems {
        if system
            .design_specification_outdoor_air_object_name
            .is_some()
            || system.outdoor_air_inlet_node_name.is_some()
            || system.demand_controlled_ventilation_type != DemandControlledVentilationType::None
            || system.outdoor_air_economizer_type != OutdoorAirEconomizerType::NoEconomizer
            || system.heat_recovery_type != HeatRecoveryType::None
        {
            violations
                .push("OA, DCV, economizer and heat recovery are outside no-OA scope B".into());
        }
        if system.dehumidification_control_type != DehumidificationControlType::None
            || system.humidification_control_type != HumidificationControlType::None
        {
            violations.push("humidity-control branches are outside sensible-only scope B".into());
        }
        if system.system_inlet_air_node_name.is_some()
            || system
                .design_specification_zonehvac_sizing_object_name
                .is_some()
        {
            violations.push("return-plenum/system-inlet and zone-HVAC sizing branches are outside direct scope B".into());
        }
        for (mode, flow, capacity) in [
            (
                system.heating_limit,
                system.maximum_heating_air_flow_rate_m3_per_s,
                system.maximum_sensible_heating_capacity_w,
            ),
            (
                system.cooling_limit,
                system.maximum_cooling_air_flow_rate_m3_per_s,
                system.maximum_total_cooling_capacity_w,
            ),
        ] {
            if [flow, capacity]
                .into_iter()
                .flatten()
                .any(|value| matches!(value, AutosizeOrNumber::Autosize))
            {
                violations.push("IdealLoads Autosize is outside hard-sized scope B".into());
            }
            for (active, value) in [
                (
                    matches!(
                        mode,
                        IdealLoadsLimit::LimitFlowRate | IdealLoadsLimit::LimitFlowRateAndCapacity
                    ),
                    flow,
                ),
                (
                    matches!(
                        mode,
                        IdealLoadsLimit::LimitCapacity | IdealLoadsLimit::LimitFlowRateAndCapacity
                    ),
                    capacity,
                ),
            ] {
                if active
                    && !matches!(value, Some(AutosizeOrNumber::Value(v)) if v.is_finite() && v >= 0.0)
                {
                    violations.push(
                        "active IdealLoads limits must have nonnegative hard-sized numeric values"
                            .into(),
                    );
                }
            }
        }
    }
}

#[cfg(test)]
mod tests;
