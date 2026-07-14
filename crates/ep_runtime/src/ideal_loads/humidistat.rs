//! Runtime-owned no-OA Humidistat closed-loop state transitions.

use crate::zone_equipment::ZoneSysEnergyDemand;
use ep_model::{IdealLoadsAirSystem, NodeId};

use super::calc::{
    IdealLoadsSensibleLimitContext, IdealLoadsZoneState, NoOaThirdOrderHumidityCorrector,
    NoOaThirdOrderHumidityCorrectorInput, NoOaThirdOrderMoistureDemand,
    NoOaThirdOrderMoistureDemandInput, calc_no_oa_third_order_moisture_demand_compat,
    correct_no_oa_third_order_humidity_ratio_compat,
};
use super::dispatch::{
    IdealLoadsCompiledBranchFlags, IdealLoadsPurchasedAirBranch, SimPurchasedAirCompatError,
    SimPurchasedAirCompatInput, SimPurchasedAirCompatOutput,
    sim_purchased_air_compat_with_branch_flags,
};

/// Runtime-owned humidity histories for the seeded no-OA Humidistat loop.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NoOaHumidistatClosedLoopState {
    zone_mean_air_humidity_ratio_history: [f64; 3],
    zone_air_humidity_ratio_history: [f64; 3],
}

impl NoOaHumidistatClosedLoopState {
    /// Seeds the runtime state from source-order zone-mean and zone-air histories.
    #[must_use]
    pub const fn from_seed_histories(
        zone_mean_air_humidity_ratio_history: [f64; 3],
        zone_air_humidity_ratio_history: [f64; 3],
    ) -> Self {
        Self {
            zone_mean_air_humidity_ratio_history,
            zone_air_humidity_ratio_history,
        }
    }

    /// Returns the current ThirdOrder zone-mean humidity history.
    #[must_use]
    pub const fn zone_mean_air_humidity_ratio_history(&self) -> [f64; 3] {
        self.zone_mean_air_humidity_ratio_history
    }

    /// Returns the current closed-loop zone-air humidity history.
    #[must_use]
    pub const fn zone_air_humidity_ratio_history(&self) -> [f64; 3] {
        self.zone_air_humidity_ratio_history
    }
}

/// Typed inputs for one seeded no-OA Humidistat zone-timestep transition.
///
/// This compatibility boundary advances both zone-timestep humidity histories
/// once per call. It therefore represents a fixed zone timestep whose single
/// system timestep has the same duration; adaptive or multiple system substeps
/// require a separate state boundary.
#[derive(Clone, Copy, Debug)]
pub struct NoOaHumidistatZoneTimestepInput<'a> {
    /// Prebound IdealLoads system evaluated by `SimPurchasedAir`.
    pub system: &'a IdealLoadsAirSystem,
    /// Resolved supply node updated by `SimPurchasedAir`.
    pub supply_node: NodeId,
    /// Sensible demand snapshot whose moisture fields are replaced by the predictor.
    pub sensible_demand: ZoneSysEnergyDemand,
    /// Zone-air temperature visible to the moisture predictor in C.
    pub predictor_zone_air_temperature_c: f64,
    /// Zone-air temperature visible to `CalcPurchAirLoads` in C.
    pub purchased_air_zone_temperature_c: f64,
    /// Recirculation-air temperature visible to `CalcPurchAirLoads` in C.
    pub recirculation_air_temperature_c: f64,
    /// Zone-air temperature visible to the humidity corrector in C.
    pub corrector_zone_air_temperature_c: f64,
    /// Zone volume in m3.
    pub zone_volume_m3: f64,
    /// EnergyPlus `ZoneVolCapMultpMoist` value.
    pub zone_moisture_capacity_multiplier: f64,
    /// Fixed zone/system timestep duration in seconds.
    pub zone_timestep_seconds: f64,
    /// Timestep barometric pressure in Pa.
    pub barometric_pressure_pa: f64,
    /// Internal latent gain in W.
    pub latent_gain_w: f64,
    /// Humidifying relative-humidity setpoint in percent.
    pub humidifying_relative_humidity_percent: f64,
    /// Dehumidifying relative-humidity setpoint in percent.
    pub dehumidifying_relative_humidity_percent: f64,
    /// Zone multiplier times list multiplier.
    pub zone_multiplier: f64,
    /// Availability-schedule result for this timestep.
    pub unit_available: bool,
    /// Psychrometric and standard-density context.
    pub limit_context: IdealLoadsSensibleLimitContext,
}

/// Results produced by one seeded no-OA Humidistat zone-timestep transition.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NoOaHumidistatZoneTimestepOutput {
    /// ThirdOrder moisture demand supplied to `SimPurchasedAir`.
    pub moisture_demand: NoOaThirdOrderMoistureDemand,
    /// Source-order PurchasedAir calculation, node update, and report snapshot.
    pub purchased_air: SimPurchasedAirCompatOutput,
    /// Corrected zone-air humidity ratio and corrector coefficients.
    pub humidity_correction: NoOaThirdOrderHumidityCorrector,
}

/// Failure stage for a seeded no-OA Humidistat zone-timestep transition.
#[derive(Clone, Debug, PartialEq)]
pub enum NoOaHumidistatZoneTimestepError {
    /// The selected PurchasedAir branch is not a supported no-OA Humidistat branch.
    UnsupportedBranch(IdealLoadsPurchasedAirBranch),
    /// The ThirdOrder moisture predictor rejected its typed inputs.
    MoisturePredictorRejected,
    /// The source-order PurchasedAir wrapper rejected the selected branch.
    PurchasedAir(SimPurchasedAirCompatError),
    /// The ThirdOrder humidity corrector rejected its typed inputs.
    HumidityCorrectorRejected,
}

/// Advances one seeded no-OA Humidistat predictor/PurchasedAir/corrector zone timestep.
///
/// The state is replaced only after every calculation succeeds, so a rejected
/// branch, predictor, PurchasedAir calculation, or corrector leaves both
/// histories intact. This fixed-timestep boundary advances zone histories once;
/// it does not claim adaptive or multiple system-substep behavior.
pub fn advance_no_oa_humidistat_zone_timestep_compat(
    state: &mut NoOaHumidistatClosedLoopState,
    input: NoOaHumidistatZoneTimestepInput<'_>,
) -> Result<NoOaHumidistatZoneTimestepOutput, NoOaHumidistatZoneTimestepError> {
    let branch_flags = IdealLoadsCompiledBranchFlags::from_system(input.system);
    if !matches!(
        branch_flags.purchased_air_branch,
        IdealLoadsPurchasedAirBranch::NoOaHumidistatDehumidification
            | IdealLoadsPurchasedAirBranch::NoOaHumidistatHumidification
    ) {
        return Err(NoOaHumidistatZoneTimestepError::UnsupportedBranch(
            branch_flags.purchased_air_branch,
        ));
    }

    let zone_mean_air_humidity_ratio_history = state.zone_mean_air_humidity_ratio_history;
    let zone_air_humidity_ratio_history = state.zone_air_humidity_ratio_history;
    let current_zone_air_humidity_ratio = zone_air_humidity_ratio_history[0];
    let zone_multiplier = input.zone_multiplier.max(1.0);

    let moisture_demand =
        calc_no_oa_third_order_moisture_demand_compat(NoOaThirdOrderMoistureDemandInput {
            zone_state: IdealLoadsZoneState {
                air_temperature_c: input.predictor_zone_air_temperature_c,
                air_humidity_ratio: current_zone_air_humidity_ratio,
            },
            previous_zone_timestep_humidity_ratios: zone_mean_air_humidity_ratio_history,
            zone_volume_m3: input.zone_volume_m3,
            zone_moisture_capacity_multiplier: input.zone_moisture_capacity_multiplier,
            timestep_seconds: input.zone_timestep_seconds,
            barometric_pressure_pa: input.barometric_pressure_pa,
            latent_gain_w: input.latent_gain_w,
            humidifying_relative_humidity_percent: input.humidifying_relative_humidity_percent,
            dehumidifying_relative_humidity_percent: input.dehumidifying_relative_humidity_percent,
            zone_multiplier,
        })
        .ok_or(NoOaHumidistatZoneTimestepError::MoisturePredictorRejected)?;

    let mut demand = input.sensible_demand;
    demand.remaining_output_req_to_humid_sp_kg_per_s =
        moisture_demand.humidifying_setpoint_load_kg_per_s;
    demand.remaining_output_req_to_dehumid_sp_kg_per_s =
        moisture_demand.dehumidifying_setpoint_load_kg_per_s;

    let purchased_air = sim_purchased_air_compat_with_branch_flags(
        SimPurchasedAirCompatInput {
            system: input.system,
            supply_node: input.supply_node,
            zone_state: IdealLoadsZoneState {
                air_temperature_c: input.purchased_air_zone_temperature_c,
                air_humidity_ratio: current_zone_air_humidity_ratio,
            },
            recirculation_state: IdealLoadsZoneState {
                air_temperature_c: input.recirculation_air_temperature_c,
                air_humidity_ratio: current_zone_air_humidity_ratio,
            },
            demand,
            unit_available: input.unit_available,
            limit_context: input
                .limit_context
                .with_barometric_pressure_pa(input.barometric_pressure_pa),
        },
        branch_flags,
    )
    .map_err(NoOaHumidistatZoneTimestepError::PurchasedAir)?;

    let humidity_correction =
        correct_no_oa_third_order_humidity_ratio_compat(NoOaThirdOrderHumidityCorrectorInput {
            zone_state: IdealLoadsZoneState {
                air_temperature_c: input.corrector_zone_air_temperature_c,
                air_humidity_ratio: current_zone_air_humidity_ratio,
            },
            previous_zone_timestep_humidity_ratios: zone_mean_air_humidity_ratio_history,
            zone_volume_m3: input.zone_volume_m3,
            zone_moisture_capacity_multiplier: input.zone_moisture_capacity_multiplier,
            timestep_seconds: input.zone_timestep_seconds,
            barometric_pressure_pa: input.barometric_pressure_pa,
            latent_gain_w: input.latent_gain_w,
            supply_mass_flow_rate_kg_per_s: purchased_air
                .calculation
                .supply_mass_flow_rate_kg_per_s
                / zone_multiplier,
            supply_humidity_ratio: purchased_air.calculation.supply_humidity_ratio,
        })
        .ok_or(NoOaHumidistatZoneTimestepError::HumidityCorrectorRejected)?;

    let corrected_zone_air_humidity_ratio = humidity_correction.zone_air_humidity_ratio;
    *state = NoOaHumidistatClosedLoopState {
        zone_mean_air_humidity_ratio_history: push_history(
            zone_mean_air_humidity_ratio_history,
            corrected_zone_air_humidity_ratio,
        ),
        zone_air_humidity_ratio_history: push_history(
            zone_air_humidity_ratio_history,
            corrected_zone_air_humidity_ratio,
        ),
    };

    Ok(NoOaHumidistatZoneTimestepOutput {
        moisture_demand,
        purchased_air,
        humidity_correction,
    })
}

const fn push_history(history: [f64; 3], value: f64) -> [f64; 3] {
    [value, history[0], history[1]]
}

#[cfg(test)]
#[path = "humidistat_tests.rs"]
mod humidistat_tests;
