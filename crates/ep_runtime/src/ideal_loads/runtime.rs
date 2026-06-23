//! Arbitrary-run IdealLoads compatibility runtime.

use std::fmt::{Display, Formatter};

use ep_model::{IdealLoadsAirSystem, NodeId, OutputHandle, SimulationModel};

use crate::{
    OutputSeries, ResultStore,
    ideal_loads::{
        IdealLoadsPurchasedAirBranch, IdealLoadsSensibleLimitContext, IdealLoadsZoneState,
        SimPurchasedAirCompatError, SimPurchasedAirCompatInput, SimPurchasedAirCompatOutput,
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
        ZONE_IDEAL_LOADS_ZONE_TOTAL_HEATING_RATE, sim_purchased_air_compat,
    },
    zone_equipment::{
        IdealLoadsZoneEquipmentDispatchIssue, ZoneSysEnergyDemand,
        validate_ideal_loads_zone_equipment_dispatch,
    },
};

const DEFAULT_ZONE_AIR_TEMPERATURE_C: f64 = 23.0;
const DEFAULT_ZONE_AIR_HUMIDITY_RATIO: f64 = 0.008;
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

    for system in &model.typed.ideal_loads_air_systems {
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
        let output = sim_purchased_air_compat(SimPurchasedAirCompatInput {
            system,
            supply_node,
            zone_state,
            recirculation_state: zone_state,
            demand,
            unit_available: options.unit_available,
            limit_context,
        })
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
    let calculation = output.calculation;
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
        calculation.zone_latent_heating_rate_w,
        sample_count,
    );
    add_constant_output_series(
        results,
        handle_index,
        key,
        ZONE_IDEAL_LOADS_ZONE_LATENT_COOLING_RATE,
        "W",
        calculation.zone_latent_cooling_rate_w,
        sample_count,
    );
    add_constant_output_series(
        results,
        handle_index,
        key,
        ZONE_IDEAL_LOADS_SUPPLY_AIR_SENSIBLE_HEATING_RATE,
        "W",
        calculation.supply_air_sensible_heating_rate_w,
        sample_count,
    );
    add_constant_output_series(
        results,
        handle_index,
        key,
        ZONE_IDEAL_LOADS_SUPPLY_AIR_SENSIBLE_COOLING_RATE,
        "W",
        calculation.supply_air_sensible_cooling_rate_w,
        sample_count,
    );
    add_constant_output_series(
        results,
        handle_index,
        key,
        ZONE_IDEAL_LOADS_SUPPLY_AIR_LATENT_HEATING_RATE,
        "W",
        calculation.supply_air_latent_heating_rate_w,
        sample_count,
    );
    add_constant_output_series(
        results,
        handle_index,
        key,
        ZONE_IDEAL_LOADS_SUPPLY_AIR_LATENT_COOLING_RATE,
        "W",
        calculation.supply_air_latent_cooling_rate_w,
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
