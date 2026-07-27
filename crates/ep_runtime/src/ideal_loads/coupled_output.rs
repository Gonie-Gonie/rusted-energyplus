//! Hourly reporting for the fixed-timestep direct-Zone PurchasedAir loop.

use ep_model::{IdealLoadsAirSystem, NodeId, OutputHandle};

use crate::{OutputSeries, ResultStore, ZoneSensibleDemandInputKind};

use super::{
    DirectZonePurchasedAirScheduledCouplingOutput, IdealLoadsSensibleLimitContext,
    ZONE_IDEAL_LOADS_SUPPLY_AIR_HUMIDITY_RATIO, ZONE_IDEAL_LOADS_SUPPLY_AIR_LATENT_COOLING_RATE,
    ZONE_IDEAL_LOADS_SUPPLY_AIR_LATENT_HEATING_RATE, ZONE_IDEAL_LOADS_SUPPLY_AIR_MASS_FLOW_RATE,
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
};

/// EnergyPlus predictor threshold retained as the heating-side CP301 demand input.
pub const ZONE_SYSTEM_PREDICTED_SENSIBLE_LOAD_TO_HEATING_SETPOINT_RATE: &str =
    "Zone System Predicted Sensible Load to Heating Setpoint Heat Transfer Rate";

/// EnergyPlus predictor threshold retained as the cooling-side CP301 demand input.
pub const ZONE_SYSTEM_PREDICTED_SENSIBLE_LOAD_TO_COOLING_SETPOINT_RATE: &str =
    "Zone System Predicted Sensible Load to Cooling Setpoint Heat Transfer Rate";

const SYSTEM_NODE_TEMPERATURE: &str = "System Node Temperature";
const SYSTEM_NODE_HUMIDITY_RATIO: &str = "System Node Humidity Ratio";
const SYSTEM_NODE_MASS_FLOW_RATE: &str = "System Node Mass Flow Rate";
const COUPLED_OUTPUT_SERIES_COUNT: u32 = 25;

/// Fail-closed validation error for direct-Zone PurchasedAir hourly reporting.
#[derive(Clone, Debug, PartialEq)]
pub enum DirectZonePurchasedAirHourlyOutputError {
    /// The caller supplied no Zone timesteps per reporting hour.
    ZeroZoneTimestepsPerHour,
    /// The fixed system timestep was not finite and strictly positive.
    InvalidTimestepSeconds {
        /// Rejected timestep in seconds.
        value: f64,
    },
    /// The standard air density could not safely convert mass flow to volume flow.
    InvalidStandardAirDensity {
        /// Rejected standard air density in kg/m3.
        value: f64,
    },
    /// The collected timestep outputs do not form complete reporting hours.
    OutputCountNotDivisible {
        /// Collected system-timestep output count.
        output_count: usize,
        /// Required Zone timesteps per reporting hour.
        zone_timesteps_per_hour: u32,
    },
    /// One PurchasedAir result targeted a node other than the bound supply node.
    SupplyNodeMismatch {
        /// Zero-based system-timestep output index.
        timestep_index: usize,
        /// Bound supply node.
        expected: NodeId,
        /// Node carried by the PurchasedAir update.
        actual: NodeId,
    },
    /// One predictor output did not retain source setpoint threshold semantics.
    UnexpectedDemandInputKind {
        /// Zero-based system-timestep output index.
        timestep_index: usize,
        /// Unexpected demand interpretation.
        actual: ZoneSensibleDemandInputKind,
    },
    /// No collision-free contiguous output-handle range remained.
    OutputHandleSpaceExhausted {
        /// Maximum handle already present, if any.
        maximum_existing_handle: Option<u32>,
    },
}

struct PendingSeries<'a> {
    key: &'a str,
    variable_name: &'static str,
    units: &'static str,
    values: Vec<f64>,
}

/// Appends hourly CP301/PurchasedAir series after all handles already in `results`.
///
/// PurchasedAir variables use the system key, predictor thresholds use the
/// controlled Zone key, and node variables use the bound supply-node key.
/// Rate, demand, node, flow, temperature, and humidity values are arithmetic
/// averages of the fixed Zone-timestep samples in each hour. Energy values are
/// the exact sum of each contributing rate multiplied by `timestep_seconds`.
pub(crate) fn append_direct_zone_purchased_air_hourly_output_series(
    results: &mut ResultStore,
    system: &IdealLoadsAirSystem,
    zone_name: &str,
    supply_node: NodeId,
    supply_node_name: &str,
    limit_context: IdealLoadsSensibleLimitContext,
    timestep_outputs: &[DirectZonePurchasedAirScheduledCouplingOutput],
    zone_timesteps_per_hour: u32,
    timestep_seconds: f64,
) -> Result<(), DirectZonePurchasedAirHourlyOutputError> {
    if zone_timesteps_per_hour == 0 {
        return Err(DirectZonePurchasedAirHourlyOutputError::ZeroZoneTimestepsPerHour);
    }
    if !timestep_seconds.is_finite() || timestep_seconds <= 0.0 {
        return Err(
            DirectZonePurchasedAirHourlyOutputError::InvalidTimestepSeconds {
                value: timestep_seconds,
            },
        );
    }
    let standard_air_density = limit_context.standard_air_density_kg_per_m3;
    if !standard_air_density.is_finite() || standard_air_density <= 0.0 {
        return Err(
            DirectZonePurchasedAirHourlyOutputError::InvalidStandardAirDensity {
                value: standard_air_density,
            },
        );
    }

    let steps_per_hour = zone_timesteps_per_hour as usize;
    if !timestep_outputs.len().is_multiple_of(steps_per_hour) {
        return Err(
            DirectZonePurchasedAirHourlyOutputError::OutputCountNotDivisible {
                output_count: timestep_outputs.len(),
                zone_timesteps_per_hour,
            },
        );
    }

    for (timestep_index, output) in timestep_outputs.iter().enumerate() {
        let actual_node = output.coupling.purchased_air.supply_node_update.node;
        if actual_node != supply_node {
            return Err(
                DirectZonePurchasedAirHourlyOutputError::SupplyNodeMismatch {
                    timestep_index,
                    expected: supply_node,
                    actual: actual_node,
                },
            );
        }
        let actual_kind = output.coupling.prediction.zone_demand.sensible_input_kind;
        if actual_kind != ZoneSensibleDemandInputKind::SourceSetpointThresholds {
            return Err(
                DirectZonePurchasedAirHourlyOutputError::UnexpectedDemandInputKind {
                    timestep_index,
                    actual: actual_kind,
                },
            );
        }
    }

    let maximum_existing_handle = results.series.iter().map(|series| series.handle.0).max();
    let first_handle = match maximum_existing_handle {
        Some(maximum) => maximum.checked_add(1),
        None => Some(0),
    }
    .ok_or(
        DirectZonePurchasedAirHourlyOutputError::OutputHandleSpaceExhausted {
            maximum_existing_handle,
        },
    )?;
    let last_handle = first_handle
        .checked_add(COUPLED_OUTPUT_SERIES_COUNT - 1)
        .ok_or(
            DirectZonePurchasedAirHourlyOutputError::OutputHandleSpaceExhausted {
                maximum_existing_handle,
            },
        )?;

    let key = system.name.0.as_str();
    let pending = vec![
        hourly_average_series(
            key,
            ZONE_IDEAL_LOADS_ZONE_TOTAL_HEATING_RATE,
            "W",
            timestep_outputs,
            steps_per_hour,
            zone_timesteps_per_hour,
            |output| {
                output
                    .coupling
                    .purchased_air
                    .report
                    .zone_total_heating_rate_w
            },
        ),
        hourly_average_series(
            key,
            ZONE_IDEAL_LOADS_ZONE_TOTAL_COOLING_RATE,
            "W",
            timestep_outputs,
            steps_per_hour,
            zone_timesteps_per_hour,
            |output| {
                output
                    .coupling
                    .purchased_air
                    .report
                    .zone_total_cooling_rate_w
            },
        ),
        hourly_average_series(
            key,
            ZONE_IDEAL_LOADS_ZONE_SENSIBLE_HEATING_RATE,
            "W",
            timestep_outputs,
            steps_per_hour,
            zone_timesteps_per_hour,
            |output| {
                output
                    .coupling
                    .purchased_air
                    .report
                    .zone_sensible_heating_rate_w
            },
        ),
        hourly_average_series(
            key,
            ZONE_IDEAL_LOADS_ZONE_SENSIBLE_COOLING_RATE,
            "W",
            timestep_outputs,
            steps_per_hour,
            zone_timesteps_per_hour,
            |output| {
                output
                    .coupling
                    .purchased_air
                    .report
                    .zone_sensible_cooling_rate_w
            },
        ),
        hourly_average_series(
            key,
            ZONE_IDEAL_LOADS_ZONE_LATENT_HEATING_RATE,
            "W",
            timestep_outputs,
            steps_per_hour,
            zone_timesteps_per_hour,
            |output| {
                output
                    .coupling
                    .purchased_air
                    .report
                    .zone_latent_heating_rate_w
            },
        ),
        hourly_average_series(
            key,
            ZONE_IDEAL_LOADS_ZONE_LATENT_COOLING_RATE,
            "W",
            timestep_outputs,
            steps_per_hour,
            zone_timesteps_per_hour,
            |output| {
                output
                    .coupling
                    .purchased_air
                    .report
                    .zone_latent_cooling_rate_w
            },
        ),
        hourly_average_series(
            key,
            ZONE_IDEAL_LOADS_SUPPLY_AIR_SENSIBLE_HEATING_RATE,
            "W",
            timestep_outputs,
            steps_per_hour,
            zone_timesteps_per_hour,
            |output| {
                output
                    .coupling
                    .purchased_air
                    .report
                    .supply_air_sensible_heating_rate_w
            },
        ),
        hourly_average_series(
            key,
            ZONE_IDEAL_LOADS_SUPPLY_AIR_SENSIBLE_COOLING_RATE,
            "W",
            timestep_outputs,
            steps_per_hour,
            zone_timesteps_per_hour,
            |output| {
                output
                    .coupling
                    .purchased_air
                    .report
                    .supply_air_sensible_cooling_rate_w
            },
        ),
        hourly_average_series(
            key,
            ZONE_IDEAL_LOADS_SUPPLY_AIR_LATENT_HEATING_RATE,
            "W",
            timestep_outputs,
            steps_per_hour,
            zone_timesteps_per_hour,
            |output| {
                output
                    .coupling
                    .purchased_air
                    .report
                    .supply_air_latent_heating_rate_w
            },
        ),
        hourly_average_series(
            key,
            ZONE_IDEAL_LOADS_SUPPLY_AIR_LATENT_COOLING_RATE,
            "W",
            timestep_outputs,
            steps_per_hour,
            zone_timesteps_per_hour,
            |output| {
                output
                    .coupling
                    .purchased_air
                    .report
                    .supply_air_latent_cooling_rate_w
            },
        ),
        hourly_average_series(
            key,
            ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_HEATING_RATE,
            "W",
            timestep_outputs,
            steps_per_hour,
            zone_timesteps_per_hour,
            |output| {
                output
                    .coupling
                    .purchased_air
                    .report
                    .supply_air_total_heating_rate_w
            },
        ),
        hourly_average_series(
            key,
            ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_COOLING_RATE,
            "W",
            timestep_outputs,
            steps_per_hour,
            zone_timesteps_per_hour,
            |output| {
                output
                    .coupling
                    .purchased_air
                    .report
                    .supply_air_total_cooling_rate_w
            },
        ),
        hourly_energy_series(
            key,
            ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_HEATING_ENERGY,
            timestep_outputs,
            steps_per_hour,
            timestep_seconds,
            |output| {
                output
                    .coupling
                    .purchased_air
                    .report
                    .supply_air_total_heating_rate_w
            },
        ),
        hourly_energy_series(
            key,
            ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_COOLING_ENERGY,
            timestep_outputs,
            steps_per_hour,
            timestep_seconds,
            |output| {
                output
                    .coupling
                    .purchased_air
                    .report
                    .supply_air_total_cooling_rate_w
            },
        ),
        hourly_energy_series(
            key,
            ZONE_IDEAL_LOADS_ZONE_TOTAL_HEATING_ENERGY,
            timestep_outputs,
            steps_per_hour,
            timestep_seconds,
            |output| {
                output
                    .coupling
                    .purchased_air
                    .report
                    .zone_total_heating_rate_w
            },
        ),
        hourly_energy_series(
            key,
            ZONE_IDEAL_LOADS_ZONE_TOTAL_COOLING_ENERGY,
            timestep_outputs,
            steps_per_hour,
            timestep_seconds,
            |output| {
                output
                    .coupling
                    .purchased_air
                    .report
                    .zone_total_cooling_rate_w
            },
        ),
        hourly_average_series(
            key,
            ZONE_IDEAL_LOADS_SUPPLY_AIR_MASS_FLOW_RATE,
            "kg/s",
            timestep_outputs,
            steps_per_hour,
            zone_timesteps_per_hour,
            |output| {
                output
                    .coupling
                    .purchased_air
                    .report
                    .supply_mass_flow_rate_kg_per_s
            },
        ),
        hourly_average_series(
            key,
            ZONE_IDEAL_LOADS_SUPPLY_AIR_STANDARD_DENSITY_VOLUME_FLOW_RATE,
            "m3/s",
            timestep_outputs,
            steps_per_hour,
            zone_timesteps_per_hour,
            |output| {
                output
                    .coupling
                    .purchased_air
                    .report
                    .supply_mass_flow_rate_kg_per_s
                    / standard_air_density
            },
        ),
        hourly_average_series(
            key,
            ZONE_IDEAL_LOADS_SUPPLY_AIR_TEMPERATURE,
            "C",
            timestep_outputs,
            steps_per_hour,
            zone_timesteps_per_hour,
            |output| output.coupling.purchased_air.report.supply_temperature_c,
        ),
        hourly_average_series(
            key,
            ZONE_IDEAL_LOADS_SUPPLY_AIR_HUMIDITY_RATIO,
            "kgWater/kgDryAir",
            timestep_outputs,
            steps_per_hour,
            zone_timesteps_per_hour,
            |output| output.coupling.purchased_air.report.supply_humidity_ratio,
        ),
        hourly_average_series(
            supply_node_name,
            SYSTEM_NODE_TEMPERATURE,
            "C",
            timestep_outputs,
            steps_per_hour,
            zone_timesteps_per_hour,
            |output| {
                output
                    .coupling
                    .purchased_air
                    .supply_node_update
                    .temperature_c
            },
        ),
        hourly_average_series(
            supply_node_name,
            SYSTEM_NODE_HUMIDITY_RATIO,
            "kgWater/kgDryAir",
            timestep_outputs,
            steps_per_hour,
            zone_timesteps_per_hour,
            |output| {
                output
                    .coupling
                    .purchased_air
                    .supply_node_update
                    .humidity_ratio
            },
        ),
        hourly_average_series(
            supply_node_name,
            SYSTEM_NODE_MASS_FLOW_RATE,
            "kg/s",
            timestep_outputs,
            steps_per_hour,
            zone_timesteps_per_hour,
            |output| {
                output
                    .coupling
                    .purchased_air
                    .supply_node_update
                    .mass_flow_rate_kg_per_s
            },
        ),
        hourly_average_series(
            zone_name,
            ZONE_SYSTEM_PREDICTED_SENSIBLE_LOAD_TO_HEATING_SETPOINT_RATE,
            "W",
            timestep_outputs,
            steps_per_hour,
            zone_timesteps_per_hour,
            |output| {
                output
                    .coupling
                    .prediction
                    .zone_demand
                    .remaining_output_req_to_heat_sp_w
            },
        ),
        hourly_average_series(
            zone_name,
            ZONE_SYSTEM_PREDICTED_SENSIBLE_LOAD_TO_COOLING_SETPOINT_RATE,
            "W",
            timestep_outputs,
            steps_per_hour,
            zone_timesteps_per_hour,
            |output| {
                output
                    .coupling
                    .prediction
                    .zone_demand
                    .remaining_output_req_to_cool_sp_w
            },
        ),
    ];
    debug_assert_eq!(pending.len(), COUPLED_OUTPUT_SERIES_COUNT as usize);

    for (handle, series) in (first_handle..=last_handle).zip(pending) {
        results.add_series(OutputSeries {
            handle: OutputHandle(handle),
            key: series.key.to_string(),
            variable_name: series.variable_name.to_string(),
            units: series.units.to_string(),
            values: series.values,
        });
    }
    Ok(())
}

fn hourly_average_series<'a>(
    key: &'a str,
    variable_name: &'static str,
    units: &'static str,
    timestep_outputs: &[DirectZonePurchasedAirScheduledCouplingOutput],
    steps_per_hour: usize,
    zone_timesteps_per_hour: u32,
    value: impl Fn(&DirectZonePurchasedAirScheduledCouplingOutput) -> f64,
) -> PendingSeries<'a> {
    let divisor = f64::from(zone_timesteps_per_hour);
    let values = timestep_outputs
        .chunks_exact(steps_per_hour)
        .map(|hour| hour.iter().map(&value).sum::<f64>() / divisor)
        .collect();
    PendingSeries {
        key,
        variable_name,
        units,
        values,
    }
}

fn hourly_energy_series<'a>(
    key: &'a str,
    variable_name: &'static str,
    timestep_outputs: &[DirectZonePurchasedAirScheduledCouplingOutput],
    steps_per_hour: usize,
    timestep_seconds: f64,
    rate: impl Fn(&DirectZonePurchasedAirScheduledCouplingOutput) -> f64,
) -> PendingSeries<'a> {
    let values = timestep_outputs
        .chunks_exact(steps_per_hour)
        .map(|hour| {
            hour.iter()
                .map(|output| rate(output) * timestep_seconds)
                .sum()
        })
        .collect();
    PendingSeries {
        key,
        variable_name,
        units: "J",
        values,
    }
}

#[cfg(test)]
#[path = "coupled_output_tests.rs"]
mod tests;
