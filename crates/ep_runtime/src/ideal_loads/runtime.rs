//! Arbitrary-run IdealLoads compatibility runtime.

use std::fmt::{Display, Formatter};

use ep_model::{
    AutoOrNumber, DesignSpecificationOutdoorAir, IdealLoadsAirSystem, NodeId, OutputHandle,
    PeopleNumberCalculationMethod, SimulationModel, ZoneId,
};

use crate::{
    OutputSeries, ResultStore,
    ideal_loads::{
        IdealLoadsCompiledBranchFlags, IdealLoadsOutdoorAirContext, IdealLoadsOutdoorAirNodeState,
        IdealLoadsPurchasedAirBranch, IdealLoadsSensibleLimitContext, IdealLoadsZoneState,
        SimPurchasedAirCompatError, SimPurchasedAirCompatInput, SimPurchasedAirCompatOutput,
        SimPurchasedAirOutdoorAirCompatInput, SimPurchasedAirOutdoorAirCompatOutput,
        ZONE_IDEAL_LOADS_ECONOMIZER_ACTIVE_TIME, ZONE_IDEAL_LOADS_HEAT_RECOVERY_ACTIVE_TIME,
        ZONE_IDEAL_LOADS_HEAT_RECOVERY_LATENT_COOLING_RATE,
        ZONE_IDEAL_LOADS_HEAT_RECOVERY_LATENT_HEATING_RATE,
        ZONE_IDEAL_LOADS_HEAT_RECOVERY_SENSIBLE_COOLING_RATE,
        ZONE_IDEAL_LOADS_HEAT_RECOVERY_SENSIBLE_HEATING_RATE,
        ZONE_IDEAL_LOADS_HEAT_RECOVERY_TOTAL_COOLING_RATE,
        ZONE_IDEAL_LOADS_HEAT_RECOVERY_TOTAL_HEATING_RATE,
        ZONE_IDEAL_LOADS_MIXED_AIR_HUMIDITY_RATIO, ZONE_IDEAL_LOADS_MIXED_AIR_TEMPERATURE,
        ZONE_IDEAL_LOADS_OUTDOOR_AIR_LATENT_COOLING_RATE,
        ZONE_IDEAL_LOADS_OUTDOOR_AIR_LATENT_HEATING_RATE,
        ZONE_IDEAL_LOADS_OUTDOOR_AIR_MASS_FLOW_RATE,
        ZONE_IDEAL_LOADS_OUTDOOR_AIR_SENSIBLE_COOLING_RATE,
        ZONE_IDEAL_LOADS_OUTDOOR_AIR_SENSIBLE_HEATING_RATE,
        ZONE_IDEAL_LOADS_OUTDOOR_AIR_STANDARD_DENSITY_VOLUME_FLOW_RATE,
        ZONE_IDEAL_LOADS_OUTDOOR_AIR_TOTAL_COOLING_RATE,
        ZONE_IDEAL_LOADS_OUTDOOR_AIR_TOTAL_HEATING_RATE,
        ZONE_IDEAL_LOADS_SUPPLY_AIR_HUMIDITY_RATIO,
        ZONE_IDEAL_LOADS_SUPPLY_AIR_LATENT_COOLING_RATE,
        ZONE_IDEAL_LOADS_SUPPLY_AIR_LATENT_HEATING_RATE,
        ZONE_IDEAL_LOADS_SUPPLY_AIR_MASS_FLOW_RATE,
        ZONE_IDEAL_LOADS_SUPPLY_AIR_SENSIBLE_COOLING_RATE,
        ZONE_IDEAL_LOADS_SUPPLY_AIR_SENSIBLE_HEATING_RATE,
        ZONE_IDEAL_LOADS_SUPPLY_AIR_STANDARD_DENSITY_VOLUME_FLOW_RATE,
        ZONE_IDEAL_LOADS_SUPPLY_AIR_TEMPERATURE, ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_COOLING_ENERGY,
        ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_COOLING_RATE,
        ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_HEATING_ENERGY,
        ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_HEATING_RATE, ZONE_IDEAL_LOADS_ZONE_LATENT_COOLING_RATE,
        ZONE_IDEAL_LOADS_ZONE_LATENT_HEATING_RATE, ZONE_IDEAL_LOADS_ZONE_SENSIBLE_COOLING_RATE,
        ZONE_IDEAL_LOADS_ZONE_SENSIBLE_HEATING_RATE, ZONE_IDEAL_LOADS_ZONE_TOTAL_COOLING_ENERGY,
        ZONE_IDEAL_LOADS_ZONE_TOTAL_COOLING_RATE, ZONE_IDEAL_LOADS_ZONE_TOTAL_HEATING_ENERGY,
        ZONE_IDEAL_LOADS_ZONE_TOTAL_HEATING_RATE,
        calc_scheduled_outdoor_air_mass_flow_rate_kg_per_s,
        sim_purchased_air_compat_with_branch_flags, sim_purchased_air_outdoor_air_compat,
    },
    zone_equipment::{
        IdealLoadsZoneEquipmentDispatchIssue, ZoneSysEnergyDemand,
        validate_ideal_loads_zone_equipment_dispatch,
    },
};

const DEFAULT_ZONE_AIR_TEMPERATURE_C: f64 = 23.0;
const DEFAULT_ZONE_AIR_HUMIDITY_RATIO: f64 = 0.008;
const DEFAULT_OUTDOOR_AIR_TEMPERATURE_C: f64 = 10.0;
const DEFAULT_OUTDOOR_AIR_HUMIDITY_RATIO: f64 = 0.004;
const DEFAULT_HEATING_DEMAND_W: f64 = 0.0;
const DEFAULT_COOLING_DEMAND_W: f64 = 0.0;
const SECONDS_PER_HOUR: f64 = 3600.0;

/// Options for the source-order IdealLoads compatibility runtime.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IdealLoadsCompatibilityOptions {
    /// Number of hourly samples to write.
    pub sample_count: usize,
    /// Zone air temperature passed to `CalcPurchAirLoads`.
    pub default_zone_air_temperature_c: f64,
    /// Zone air humidity ratio passed to `CalcPurchAirLoads`.
    pub default_zone_air_humidity_ratio: f64,
    /// Outdoor-air dry-bulb temperature passed to selected OA branches.
    pub default_outdoor_air_temperature_c: f64,
    /// Outdoor-air humidity ratio passed to selected OA branches.
    pub default_outdoor_air_humidity_ratio: f64,
    /// Source-order heating demand snapshot in W.
    pub default_heating_demand_w: f64,
    /// Source-order cooling demand snapshot in W.
    pub default_cooling_demand_w: f64,
    /// Availability schedule result for the current compatibility path.
    pub unit_available: bool,
}

impl IdealLoadsCompatibilityOptions {
    /// Creates options with fixed hourly samples and neutral zone demand.
    #[must_use]
    pub const fn hourly_samples(sample_count: usize) -> Self {
        Self {
            sample_count,
            default_zone_air_temperature_c: DEFAULT_ZONE_AIR_TEMPERATURE_C,
            default_zone_air_humidity_ratio: DEFAULT_ZONE_AIR_HUMIDITY_RATIO,
            default_outdoor_air_temperature_c: DEFAULT_OUTDOOR_AIR_TEMPERATURE_C,
            default_outdoor_air_humidity_ratio: DEFAULT_OUTDOOR_AIR_HUMIDITY_RATIO,
            default_heating_demand_w: DEFAULT_HEATING_DEMAND_W,
            default_cooling_demand_w: DEFAULT_COOLING_DEMAND_W,
            unit_available: true,
        }
    }
}

/// One IdealLoads system executed by the compatibility runtime.
#[derive(Clone, Debug, PartialEq)]
pub struct IdealLoadsCompatibilitySystemSummary {
    /// IdealLoads object name.
    pub system_name: String,
    /// Selected PurchasedAir branch.
    pub branch: IdealLoadsPurchasedAirBranch,
    /// Supply node name receiving `UpdatePurchasedAir`.
    pub supply_node_name: String,
}

/// Summary for the arbitrary-run IdealLoads compatibility runtime.
#[derive(Clone, Debug, PartialEq)]
pub struct IdealLoadsCompatibilitySummary {
    /// Hourly output sample count.
    pub samples: usize,
    /// Number of IdealLoads systems executed.
    pub system_count: usize,
    /// Per-system source-order dispatch summary.
    pub systems: Vec<IdealLoadsCompatibilitySystemSummary>,
}

/// Result of the source-order IdealLoads compatibility runtime.
#[derive(Clone, Debug, PartialEq)]
pub struct IdealLoadsCompatibilitySimulation {
    /// Native output results.
    pub results: ResultStore,
    /// Runtime summary.
    pub summary: IdealLoadsCompatibilitySummary,
}

struct IdealLoadsRuntimeSystem<'a> {
    system: &'a IdealLoadsAirSystem,
    branch_flags: IdealLoadsCompiledBranchFlags,
}

/// Runtime error for the IdealLoads compatibility path.
#[derive(Clone, Debug, PartialEq)]
pub enum IdealLoadsCompatibilityRuntimeError {
    /// No IdealLoads systems were available to execute.
    NoIdealLoadsSystems,
    /// Zone equipment dispatch prerequisites were not met.
    DispatchNotSupported {
        /// IdealLoads object name.
        system_name: String,
        /// Blocking dispatch issue codes.
        issues: Vec<IdealLoadsZoneEquipmentDispatchIssue>,
    },
    /// The dispatch path did not resolve a concrete supply node.
    MissingSupplyNode {
        /// IdealLoads object name.
        system_name: String,
    },
    /// The dispatch path did not resolve a controlled zone.
    MissingZone {
        /// IdealLoads object name.
        system_name: String,
    },
    /// Selected OA branch had no resolved DesignSpecification:OutdoorAir edge.
    MissingOutdoorAirSpecification {
        /// IdealLoads object name.
        system_name: String,
    },
    /// The selected OA branch uses an unsupported or unresolved design-flow method.
    UnsupportedOutdoorAirDesignFlow {
        /// IdealLoads object name.
        system_name: String,
    },
    /// The selected PurchasedAir branch is not inside the compatibility subset.
    UnsupportedPurchasedAirBranch {
        /// Error returned by `sim_purchased_air_compat`.
        error: SimPurchasedAirCompatError,
    },
}

impl Display for IdealLoadsCompatibilityRuntimeError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoIdealLoadsSystems => write!(
                formatter,
                "IdealLoads compatibility runtime requires at least one ZoneHVAC:IdealLoadsAirSystem"
            ),
            Self::DispatchNotSupported {
                system_name,
                issues,
            } => write!(
                formatter,
                "IdealLoads system {system_name} is not dispatchable through ZoneEquipmentManager: {issues:?}"
            ),
            Self::MissingSupplyNode { system_name } => write!(
                formatter,
                "IdealLoads system {system_name} has no resolved supply node"
            ),
            Self::MissingZone { system_name } => write!(
                formatter,
                "IdealLoads system {system_name} has no resolved controlled zone"
            ),
            Self::MissingOutdoorAirSpecification { system_name } => write!(
                formatter,
                "IdealLoads system {system_name} has no resolved DesignSpecification:OutdoorAir"
            ),
            Self::UnsupportedOutdoorAirDesignFlow { system_name } => write!(
                formatter,
                "IdealLoads system {system_name} has an unsupported outdoor-air design-flow method"
            ),
            Self::UnsupportedPurchasedAirBranch { error } => write!(
                formatter,
                "IdealLoads system {:?} selected an unsupported PurchasedAir branch: {:?}",
                error.system_id, error.unsupported_features
            ),
        }
    }
}

impl std::error::Error for IdealLoadsCompatibilityRuntimeError {}

/// Executes supported IdealLoads systems through the source-order
/// ZoneEquipmentManager -> PurchasedAirManager compatibility path.
pub fn simulate_ideal_loads_purchased_air_compat(
    model: &SimulationModel,
    options: IdealLoadsCompatibilityOptions,
) -> Result<IdealLoadsCompatibilitySimulation, IdealLoadsCompatibilityRuntimeError> {
    if model.typed.ideal_loads_air_systems.is_empty() {
        return Err(IdealLoadsCompatibilityRuntimeError::NoIdealLoadsSystems);
    }

    let zone_state = IdealLoadsZoneState {
        air_temperature_c: options.default_zone_air_temperature_c,
        air_humidity_ratio: options.default_zone_air_humidity_ratio,
    };
    let limit_context = model
        .typed
        .site
        .as_ref()
        .and_then(|site| IdealLoadsSensibleLimitContext::from_site_elevation_m(site.elevation_m))
        .unwrap_or_default();

    let mut results = ResultStore::new();
    let mut handle_index = 0_u32;
    let mut systems = Vec::new();
    let compiled_systems = model
        .typed
        .ideal_loads_air_systems
        .iter()
        .map(|system| IdealLoadsRuntimeSystem {
            system,
            branch_flags: IdealLoadsCompiledBranchFlags::from_system(system),
        })
        .collect::<Vec<_>>();

    for compiled_system in compiled_systems {
        let system = compiled_system.system;
        let validation = validate_ideal_loads_zone_equipment_dispatch(model, system.id);
        if !validation.is_dispatchable() {
            return Err(IdealLoadsCompatibilityRuntimeError::DispatchNotSupported {
                system_name: system.name.0.clone(),
                issues: validation.issues,
            });
        }

        let Some(zone) = validation.zone else {
            return Err(IdealLoadsCompatibilityRuntimeError::MissingZone {
                system_name: system.name.0.clone(),
            });
        };
        let Some(supply_node) = validation.supply_nodes.first().copied() else {
            return Err(IdealLoadsCompatibilityRuntimeError::MissingSupplyNode {
                system_name: system.name.0.clone(),
            });
        };

        let demand = ZoneSysEnergyDemand::sensible_only(
            zone,
            options.default_heating_demand_w,
            options.default_cooling_demand_w,
        );
        if compiled_system.branch_flags.purchased_air_branch
            == IdealLoadsPurchasedAirBranch::OutdoorAirSelected
        {
            let output = simulate_outdoor_air_purchased_air_system(
                model,
                system,
                supply_node,
                zone,
                demand,
                limit_context,
                options,
            )?;
            write_purchased_air_outdoor_air_output_series(
                &mut results,
                &mut handle_index,
                system,
                supply_node,
                &node_name(model, supply_node),
                output,
                limit_context,
                options.sample_count,
            );
            systems.push(IdealLoadsCompatibilitySystemSummary {
                system_name: system.name.0.clone(),
                branch: IdealLoadsPurchasedAirBranch::OutdoorAirSelected,
                supply_node_name: node_name(model, supply_node),
            });
            continue;
        }

        let output = sim_purchased_air_compat_with_branch_flags(
            SimPurchasedAirCompatInput {
                system,
                supply_node,
                zone_state,
                recirculation_state: zone_state,
                demand,
                unit_available: options.unit_available,
                limit_context,
            },
            compiled_system.branch_flags,
        )
        .map_err(|error| {
            IdealLoadsCompatibilityRuntimeError::UnsupportedPurchasedAirBranch { error }
        })?;

        write_purchased_air_output_series(
            &mut results,
            &mut handle_index,
            system,
            supply_node,
            &node_name(model, supply_node),
            output,
            limit_context,
            options.sample_count,
        );
        systems.push(IdealLoadsCompatibilitySystemSummary {
            system_name: system.name.0.clone(),
            branch: output.branch,
            supply_node_name: node_name(model, supply_node),
        });
    }

    Ok(IdealLoadsCompatibilitySimulation {
        summary: IdealLoadsCompatibilitySummary {
            samples: options.sample_count,
            system_count: systems.len(),
            systems,
        },
        results,
    })
}

fn simulate_outdoor_air_purchased_air_system(
    model: &SimulationModel,
    system: &IdealLoadsAirSystem,
    supply_node: NodeId,
    zone: ZoneId,
    demand: ZoneSysEnergyDemand,
    limit_context: IdealLoadsSensibleLimitContext,
    options: IdealLoadsCompatibilityOptions,
) -> Result<SimPurchasedAirOutdoorAirCompatOutput, IdealLoadsCompatibilityRuntimeError> {
    let specification = outdoor_air_specification(model, system).ok_or_else(|| {
        IdealLoadsCompatibilityRuntimeError::MissingOutdoorAirSpecification {
            system_name: system.name.0.clone(),
        }
    })?;
    let context = outdoor_air_context(model, zone);
    let minimum_outdoor_air_mass_flow_rate_kg_per_s =
        calc_scheduled_outdoor_air_mass_flow_rate_kg_per_s(
            specification,
            context,
            None,
            limit_context.standard_air_density_kg_per_m3,
        )
        .ok_or_else(|| {
            IdealLoadsCompatibilityRuntimeError::UnsupportedOutdoorAirDesignFlow {
                system_name: system.name.0.clone(),
            }
        })?;
    let zone_state = IdealLoadsOutdoorAirNodeState {
        air_temperature_c: options.default_zone_air_temperature_c,
        air_humidity_ratio: options.default_zone_air_humidity_ratio,
    };
    let outdoor_air_state = IdealLoadsOutdoorAirNodeState {
        air_temperature_c: options.default_outdoor_air_temperature_c,
        air_humidity_ratio: options.default_outdoor_air_humidity_ratio,
    };

    Ok(sim_purchased_air_outdoor_air_compat(
        SimPurchasedAirOutdoorAirCompatInput {
            system,
            supply_node,
            zone_state,
            recirculation_state: zone_state,
            outdoor_air_state,
            demand,
            minimum_outdoor_air_mass_flow_rate_kg_per_s,
            system_timestep_hours: 1.0,
            barometric_pressure_pa: limit_context.barometric_pressure_pa,
            unit_available: options.unit_available,
        },
    ))
}

fn outdoor_air_specification<'a>(
    model: &'a SimulationModel,
    system: &IdealLoadsAirSystem,
) -> Option<&'a DesignSpecificationOutdoorAir> {
    let edge = model
        .graph
        .ideal_loads_outdoor_air_specs
        .iter()
        .find(|edge| edge.ideal_loads_air_system == system.id)?;
    model
        .typed
        .design_specification_outdoor_air
        .iter()
        .find(|specification| specification.id == edge.design_specification_outdoor_air)
}

fn outdoor_air_context(model: &SimulationModel, zone_id: ZoneId) -> IdealLoadsOutdoorAirContext {
    let zone_volume_m3 = model
        .typed
        .zones
        .iter()
        .find(|zone| zone.id == zone_id)
        .and_then(|zone| match zone.volume {
            AutoOrNumber::Value(value) if value.is_finite() => Some(value.max(0.0)),
            AutoOrNumber::Value(_) | AutoOrNumber::AutoCalculate => None,
        })
        .unwrap_or(0.0);
    let design_people_count = model
        .typed
        .people
        .iter()
        .filter(|people| people.zone == zone_id)
        .filter_map(|people| match people.number_of_people_calculation_method {
            PeopleNumberCalculationMethod::People if people.number_of_people.is_finite() => {
                Some(people.number_of_people.max(0.0))
            }
            PeopleNumberCalculationMethod::People
            | PeopleNumberCalculationMethod::PeoplePerArea
            | PeopleNumberCalculationMethod::AreaPerPerson => None,
        })
        .sum();

    IdealLoadsOutdoorAirContext {
        design_people_count,
        zone_floor_area_m2: 0.0,
        zone_volume_m3,
    }
}

fn write_purchased_air_outdoor_air_output_series(
    results: &mut ResultStore,
    handle_index: &mut u32,
    system: &IdealLoadsAirSystem,
    supply_node: NodeId,
    supply_node_name: &str,
    output: SimPurchasedAirOutdoorAirCompatOutput,
    limit_context: IdealLoadsSensibleLimitContext,
    sample_count: usize,
) {
    let key = system.name.0.as_str();
    let calculation = output.calculation;
    add_constant_output_series(
        results,
        handle_index,
        key,
        ZONE_IDEAL_LOADS_OUTDOOR_AIR_MASS_FLOW_RATE,
        "kg/s",
        calculation.outdoor_air_mass_flow_rate_kg_per_s,
        sample_count,
    );
    add_constant_output_series(
        results,
        handle_index,
        key,
        ZONE_IDEAL_LOADS_OUTDOOR_AIR_STANDARD_DENSITY_VOLUME_FLOW_RATE,
        "m3/s",
        calculation.outdoor_air_mass_flow_rate_kg_per_s
            / limit_context.standard_air_density_kg_per_m3,
        sample_count,
    );
    add_constant_output_series(
        results,
        handle_index,
        key,
        ZONE_IDEAL_LOADS_OUTDOOR_AIR_SENSIBLE_HEATING_RATE,
        "W",
        calculation.outdoor_air_sensible_heating_rate_w,
        sample_count,
    );
    add_constant_output_series(
        results,
        handle_index,
        key,
        ZONE_IDEAL_LOADS_OUTDOOR_AIR_SENSIBLE_COOLING_RATE,
        "W",
        calculation.outdoor_air_sensible_cooling_rate_w,
        sample_count,
    );
    add_constant_output_series(
        results,
        handle_index,
        key,
        ZONE_IDEAL_LOADS_OUTDOOR_AIR_LATENT_HEATING_RATE,
        "W",
        calculation.outdoor_air_latent_heating_rate_w,
        sample_count,
    );
    add_constant_output_series(
        results,
        handle_index,
        key,
        ZONE_IDEAL_LOADS_OUTDOOR_AIR_LATENT_COOLING_RATE,
        "W",
        calculation.outdoor_air_latent_cooling_rate_w,
        sample_count,
    );
    add_constant_output_series(
        results,
        handle_index,
        key,
        ZONE_IDEAL_LOADS_OUTDOOR_AIR_TOTAL_HEATING_RATE,
        "W",
        calculation.outdoor_air_total_heating_rate_w,
        sample_count,
    );
    add_constant_output_series(
        results,
        handle_index,
        key,
        ZONE_IDEAL_LOADS_OUTDOOR_AIR_TOTAL_COOLING_RATE,
        "W",
        calculation.outdoor_air_total_cooling_rate_w,
        sample_count,
    );
    add_constant_output_series(
        results,
        handle_index,
        key,
        ZONE_IDEAL_LOADS_MIXED_AIR_TEMPERATURE,
        "C",
        calculation.mixed_air_temperature_c,
        sample_count,
    );
    add_constant_output_series(
        results,
        handle_index,
        key,
        ZONE_IDEAL_LOADS_MIXED_AIR_HUMIDITY_RATIO,
        "kgWater/kgDryAir",
        calculation.mixed_air_humidity_ratio,
        sample_count,
    );
    add_constant_output_series(
        results,
        handle_index,
        key,
        ZONE_IDEAL_LOADS_HEAT_RECOVERY_SENSIBLE_HEATING_RATE,
        "W",
        calculation.heat_recovery_sensible_heating_rate_w,
        sample_count,
    );
    add_constant_output_series(
        results,
        handle_index,
        key,
        ZONE_IDEAL_LOADS_HEAT_RECOVERY_LATENT_HEATING_RATE,
        "W",
        calculation.heat_recovery_latent_heating_rate_w,
        sample_count,
    );
    add_constant_output_series(
        results,
        handle_index,
        key,
        ZONE_IDEAL_LOADS_HEAT_RECOVERY_TOTAL_HEATING_RATE,
        "W",
        calculation.heat_recovery_total_heating_rate_w,
        sample_count,
    );
    add_constant_output_series(
        results,
        handle_index,
        key,
        ZONE_IDEAL_LOADS_HEAT_RECOVERY_SENSIBLE_COOLING_RATE,
        "W",
        calculation.heat_recovery_sensible_cooling_rate_w,
        sample_count,
    );
    add_constant_output_series(
        results,
        handle_index,
        key,
        ZONE_IDEAL_LOADS_HEAT_RECOVERY_LATENT_COOLING_RATE,
        "W",
        calculation.heat_recovery_latent_cooling_rate_w,
        sample_count,
    );
    add_constant_output_series(
        results,
        handle_index,
        key,
        ZONE_IDEAL_LOADS_HEAT_RECOVERY_TOTAL_COOLING_RATE,
        "W",
        calculation.heat_recovery_total_cooling_rate_w,
        sample_count,
    );
    add_constant_output_series(
        results,
        handle_index,
        key,
        ZONE_IDEAL_LOADS_ECONOMIZER_ACTIVE_TIME,
        "hr",
        calculation.economizer_active_time_hr,
        sample_count,
    );
    add_constant_output_series(
        results,
        handle_index,
        key,
        ZONE_IDEAL_LOADS_HEAT_RECOVERY_ACTIVE_TIME,
        "hr",
        calculation.heat_recovery_active_time_hr,
        sample_count,
    );
    add_constant_output_series(
        results,
        handle_index,
        key,
        ZONE_IDEAL_LOADS_SUPPLY_AIR_MASS_FLOW_RATE,
        "kg/s",
        output.supply_node_update.mass_flow_rate_kg_per_s,
        sample_count,
    );
    add_constant_output_series(
        results,
        handle_index,
        key,
        ZONE_IDEAL_LOADS_SUPPLY_AIR_STANDARD_DENSITY_VOLUME_FLOW_RATE,
        "m3/s",
        output.supply_node_update.mass_flow_rate_kg_per_s
            / limit_context.standard_air_density_kg_per_m3,
        sample_count,
    );
    add_constant_output_series(
        results,
        handle_index,
        key,
        ZONE_IDEAL_LOADS_SUPPLY_AIR_TEMPERATURE,
        "C",
        output.supply_node_update.temperature_c,
        sample_count,
    );
    add_constant_output_series(
        results,
        handle_index,
        key,
        ZONE_IDEAL_LOADS_SUPPLY_AIR_HUMIDITY_RATIO,
        "kgWater/kgDryAir",
        output.supply_node_update.humidity_ratio,
        sample_count,
    );
    add_constant_output_series(
        results,
        handle_index,
        supply_node_name,
        "System Node Temperature",
        "C",
        output.supply_node_update.temperature_c,
        sample_count,
    );
    add_constant_output_series(
        results,
        handle_index,
        supply_node_name,
        "System Node Humidity Ratio",
        "kgWater/kgDryAir",
        output.supply_node_update.humidity_ratio,
        sample_count,
    );
    add_constant_output_series(
        results,
        handle_index,
        supply_node_name,
        "System Node Mass Flow Rate",
        "kg/s",
        output.supply_node_update.mass_flow_rate_kg_per_s,
        sample_count,
    );

    debug_assert_eq!(output.supply_node_update.node, supply_node);
}

fn write_purchased_air_output_series(
    results: &mut ResultStore,
    handle_index: &mut u32,
    system: &IdealLoadsAirSystem,
    supply_node: NodeId,
    supply_node_name: &str,
    output: SimPurchasedAirCompatOutput,
    limit_context: IdealLoadsSensibleLimitContext,
    sample_count: usize,
) {
    let key = system.name.0.as_str();
    let report = output.report;
    add_constant_output_series(
        results,
        handle_index,
        key,
        ZONE_IDEAL_LOADS_ZONE_TOTAL_HEATING_RATE,
        "W",
        report.zone_total_heating_rate_w,
        sample_count,
    );
    add_constant_output_series(
        results,
        handle_index,
        key,
        ZONE_IDEAL_LOADS_ZONE_TOTAL_COOLING_RATE,
        "W",
        report.zone_total_cooling_rate_w,
        sample_count,
    );
    add_constant_output_series(
        results,
        handle_index,
        key,
        ZONE_IDEAL_LOADS_ZONE_SENSIBLE_HEATING_RATE,
        "W",
        report.zone_sensible_heating_rate_w,
        sample_count,
    );
    add_constant_output_series(
        results,
        handle_index,
        key,
        ZONE_IDEAL_LOADS_ZONE_SENSIBLE_COOLING_RATE,
        "W",
        report.zone_sensible_cooling_rate_w,
        sample_count,
    );
    add_constant_output_series(
        results,
        handle_index,
        key,
        ZONE_IDEAL_LOADS_ZONE_LATENT_HEATING_RATE,
        "W",
        report.zone_latent_heating_rate_w,
        sample_count,
    );
    add_constant_output_series(
        results,
        handle_index,
        key,
        ZONE_IDEAL_LOADS_ZONE_LATENT_COOLING_RATE,
        "W",
        report.zone_latent_cooling_rate_w,
        sample_count,
    );
    add_constant_output_series(
        results,
        handle_index,
        key,
        ZONE_IDEAL_LOADS_SUPPLY_AIR_SENSIBLE_HEATING_RATE,
        "W",
        report.supply_air_sensible_heating_rate_w,
        sample_count,
    );
    add_constant_output_series(
        results,
        handle_index,
        key,
        ZONE_IDEAL_LOADS_SUPPLY_AIR_SENSIBLE_COOLING_RATE,
        "W",
        report.supply_air_sensible_cooling_rate_w,
        sample_count,
    );
    add_constant_output_series(
        results,
        handle_index,
        key,
        ZONE_IDEAL_LOADS_SUPPLY_AIR_LATENT_HEATING_RATE,
        "W",
        report.supply_air_latent_heating_rate_w,
        sample_count,
    );
    add_constant_output_series(
        results,
        handle_index,
        key,
        ZONE_IDEAL_LOADS_SUPPLY_AIR_LATENT_COOLING_RATE,
        "W",
        report.supply_air_latent_cooling_rate_w,
        sample_count,
    );
    add_constant_output_series(
        results,
        handle_index,
        key,
        ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_HEATING_RATE,
        "W",
        report.supply_air_total_heating_rate_w,
        sample_count,
    );
    add_constant_output_series(
        results,
        handle_index,
        key,
        ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_COOLING_RATE,
        "W",
        report.supply_air_total_cooling_rate_w,
        sample_count,
    );
    add_constant_output_series(
        results,
        handle_index,
        key,
        ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_HEATING_ENERGY,
        "J",
        report.supply_air_total_heating_rate_w * SECONDS_PER_HOUR,
        sample_count,
    );
    add_constant_output_series(
        results,
        handle_index,
        key,
        ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_COOLING_ENERGY,
        "J",
        report.supply_air_total_cooling_rate_w * SECONDS_PER_HOUR,
        sample_count,
    );
    add_constant_output_series(
        results,
        handle_index,
        key,
        ZONE_IDEAL_LOADS_ZONE_TOTAL_HEATING_ENERGY,
        "J",
        report.zone_total_heating_rate_w * SECONDS_PER_HOUR,
        sample_count,
    );
    add_constant_output_series(
        results,
        handle_index,
        key,
        ZONE_IDEAL_LOADS_ZONE_TOTAL_COOLING_ENERGY,
        "J",
        report.zone_total_cooling_rate_w * SECONDS_PER_HOUR,
        sample_count,
    );

    add_constant_output_series(
        results,
        handle_index,
        key,
        ZONE_IDEAL_LOADS_SUPPLY_AIR_MASS_FLOW_RATE,
        "kg/s",
        report.supply_mass_flow_rate_kg_per_s,
        sample_count,
    );
    add_constant_output_series(
        results,
        handle_index,
        key,
        ZONE_IDEAL_LOADS_SUPPLY_AIR_STANDARD_DENSITY_VOLUME_FLOW_RATE,
        "m3/s",
        report.supply_mass_flow_rate_kg_per_s / limit_context.standard_air_density_kg_per_m3,
        sample_count,
    );
    add_constant_output_series(
        results,
        handle_index,
        key,
        ZONE_IDEAL_LOADS_SUPPLY_AIR_TEMPERATURE,
        "C",
        report.supply_temperature_c,
        sample_count,
    );
    add_constant_output_series(
        results,
        handle_index,
        key,
        ZONE_IDEAL_LOADS_SUPPLY_AIR_HUMIDITY_RATIO,
        "kgWater/kgDryAir",
        report.supply_humidity_ratio,
        sample_count,
    );

    add_constant_output_series(
        results,
        handle_index,
        supply_node_name,
        "System Node Temperature",
        "C",
        output.supply_node_update.temperature_c,
        sample_count,
    );
    add_constant_output_series(
        results,
        handle_index,
        supply_node_name,
        "System Node Humidity Ratio",
        "kgWater/kgDryAir",
        output.supply_node_update.humidity_ratio,
        sample_count,
    );
    add_constant_output_series(
        results,
        handle_index,
        supply_node_name,
        "System Node Mass Flow Rate",
        "kg/s",
        output.supply_node_update.mass_flow_rate_kg_per_s,
        sample_count,
    );

    debug_assert_eq!(output.supply_node_update.node, supply_node);
}

fn add_constant_output_series(
    results: &mut ResultStore,
    handle_index: &mut u32,
    key: &str,
    variable_name: &str,
    units: &str,
    value: f64,
    sample_count: usize,
) {
    results.add_series(OutputSeries {
        handle: OutputHandle(*handle_index),
        key: key.to_string(),
        variable_name: variable_name.to_string(),
        units: units.to_string(),
        values: vec![value; sample_count],
    });
    *handle_index += 1;
}

fn node_name(model: &SimulationModel, node: NodeId) -> String {
    model
        .typed
        .nodes
        .iter()
        .find(|candidate| candidate.id == node)
        .map(|candidate| candidate.name.0.clone())
        .unwrap_or_else(|| format!("Node {}", node.0))
}
