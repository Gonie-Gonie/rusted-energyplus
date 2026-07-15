//! Schedule-cache ownership adapters for heat-balance initialization.

use super::initialize_heat_balance_state_with_ctf_coefficients_from_schedule_cache;
use crate::error::RuntimeError;
use crate::heat_balance::ctf::ConstructionCtfCoefficientOverride;
use crate::heat_balance::state::HeatBalanceState;
use crate::schedules::{ScheduleSeriesCache, precompute_hour_only_internal_gain_schedule_cache};
use ep_model::SimulationModel;

/// Initializes the heat-balance state shell with diagnostic CTF coefficient rows.
///
/// This is an oracle-isolation hook for heat-balance diagnostics. It does not
/// calculate EnergyPlus CTF coefficients; callers may provide rows already
/// emitted by EnergyPlus so surface history behavior can be tested separately
/// from coefficient generation.
pub fn initialize_heat_balance_state_with_ctf_coefficients(
    model: &SimulationModel,
    initial_zone_air_temperature_c: f64,
    ctf_coefficients: &[ConstructionCtfCoefficientOverride],
) -> Result<HeatBalanceState, RuntimeError> {
    let schedule_cache = precompute_hour_only_internal_gain_schedule_cache(&model.typed)?;
    initialize_heat_balance_state_with_ctf_coefficients_and_schedule_cache(
        model,
        initial_zone_air_temperature_c,
        ctf_coefficients,
        &schedule_cache,
    )
}

pub(crate) fn initialize_heat_balance_state_with_ctf_coefficients_and_schedule_cache(
    model: &SimulationModel,
    initial_zone_air_temperature_c: f64,
    ctf_coefficients: &[ConstructionCtfCoefficientOverride],
    schedule_cache: &ScheduleSeriesCache,
) -> Result<HeatBalanceState, RuntimeError> {
    initialize_heat_balance_state_with_ctf_coefficients_from_schedule_cache(
        model,
        initial_zone_air_temperature_c,
        ctf_coefficients,
        schedule_cache,
    )
}
