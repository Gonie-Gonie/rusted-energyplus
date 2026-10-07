//! Opt-in input boundary for the two CON-01 porting paths.
//!
//! Admission records input/settings scope, not numerical conformance. Ordinary
//! arbitrary runs keep their existing support assessment and runtime behavior.

mod environment;
mod settings;

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

pub(crate) use environment::porting_environment_trace;

/// Frozen input family selected for card-based porting work.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum PortingScope {
    /// One unconditioned, opaque, fully mixed zone.
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
            | "Construction"
            | "Zone"
            | "BuildingSurface:Detailed"
            | "OtherEquipment"
            | "ScheduleTypeLimits"
            | "Schedule:Constant"
            | "Schedule:Compact"
            | "RunPeriodControl:DaylightSavingTime"
            | "RunPeriodControl:SpecialDays"
    ) || (scope == PortingScope::B
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
        "do_hvac_sizing_simulation_for_sizing_periods",
    ] {
        if control[field] == true {
            violations.push(format!(
                "SimulationControl.{field}=Yes is outside the weather-run scope"
            ));
        }
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
    for (field, default) in [
        ("surface_temperature_upper_limit", 200.0),
        (
            "minimum_surface_convection_heat_transfer_coefficient_value",
            0.1,
        ),
        (
            "maximum_surface_convection_heat_transfer_coefficient_value",
            1000.0,
        ),
    ] {
        if settings["heat_balance_controls"][field] != default {
            violations.push(format!(
                "HeatBalanceAlgorithm.{field} override is outside the fixed source branch"
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

fn validate_typed_scope(model: &TypedModel, scope: PortingScope, violations: &mut Vec<String>) {
    if model.version != ep_model::Version::oracle_26_1_0() {
        violations.push("input Version must match the locked EnergyPlus 26.1.0".into());
    }
    if model.zones.len() != 1 || model.surfaces.len() != 6 {
        violations.push("exactly one opaque zone with six detailed surfaces is required".into());
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
            SolarDistribution::MinimalShadowing | SolarDistribution::FullExterior
        ) {
            violations.push("solar reflection/interior distribution branches are outside the selected opaque scope".into());
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
        if zone.multiplier != 1 || zone.list_multiplier != 1 {
            violations.push(
                "zone/group multipliers other than one are outside the selected inputs".into(),
            );
        }
        if zone.inside_convection_algorithm.effective() != InsideSurfaceConvectionAlgorithm::Tarp
            || zone.outside_convection_algorithm.effective()
                != OutsideSurfaceConvectionAlgorithm::Doe2
        {
            violations
                .push("selected zone convection must be TARP inside and DOE-2 outside".into());
        }
    }
    for surface in &model.surfaces {
        if !matches!(
            surface.outside_boundary_condition,
            OutsideBoundaryCondition::Outdoors | OutsideBoundaryCondition::Adiabatic
        ) {
            violations.push(format!(
                "{} boundary must be Outdoors or Adiabatic",
                surface.name.0
            ));
        }
    }
    if model
        .other_equipment
        .iter()
        .any(|equipment| equipment.fraction_latent != 0.0)
    {
        violations
            .push("OtherEquipment latent gains are outside the zero-latent-source scope".into());
    }
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
