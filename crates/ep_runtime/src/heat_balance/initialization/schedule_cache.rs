//! Schedule-cache ownership adapters for heat-balance initialization.

use super::initialize_heat_balance_state_with_ctf_coefficients_from_schedule_cache;
use crate::error::RuntimeError;
use crate::heat_balance::ctf::ConstructionCtfCoefficientOverride;
use crate::heat_balance::state::HeatBalanceState;
use crate::schedules::{
    HeatBalanceInternalGainScheduleOperationProfile, InternalGainSchedulePhaseOperations,
    ScheduleSeriesCache, convective_internal_gain_w_from_cache,
    convective_internal_gain_w_from_cache_profiled,
    precompute_hour_only_internal_gain_schedule_cache,
    update_surface_radiant_internal_gain_source_terms_from_cache,
    update_surface_radiant_internal_gain_source_terms_from_cache_profiled,
};
use ep_model::SimulationModel;

/// Initializes the heat-balance state shell without advancing the solver.
pub fn initialize_heat_balance_state(
    model: &SimulationModel,
    initial_zone_air_temperature_c: f64,
) -> Result<HeatBalanceState, RuntimeError> {
    initialize_heat_balance_state_with_ctf_coefficients(model, initial_zone_air_temperature_c, &[])
}

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
        None,
    )
}

pub(crate) fn initialize_heat_balance_state_with_ctf_coefficients_and_schedule_cache_profiled(
    model: &SimulationModel,
    initial_zone_air_temperature_c: f64,
    ctf_coefficients: &[ConstructionCtfCoefficientOverride],
    schedule_cache: &ScheduleSeriesCache,
    profile: &mut HeatBalanceInternalGainScheduleOperationProfile,
) -> Result<HeatBalanceState, RuntimeError> {
    initialize_heat_balance_state_with_ctf_coefficients_from_schedule_cache(
        model,
        initial_zone_air_temperature_c,
        ctf_coefficients,
        schedule_cache,
        Some(&mut profile.initialization),
    )
}

pub(super) fn convective_internal_gain_w_for_initialization(
    model: &ep_model::TypedModel,
    schedule_cache: &ScheduleSeriesCache,
    zone_id: ep_model::ZoneId,
    operations: Option<&mut InternalGainSchedulePhaseOperations>,
) -> f64 {
    operations.map_or_else(
        || convective_internal_gain_w_from_cache(model, schedule_cache, zone_id, 1),
        |operations| {
            convective_internal_gain_w_from_cache_profiled(
                model,
                schedule_cache,
                zone_id,
                1,
                operations,
            )
        },
    )
}

pub(super) fn initialize_surface_radiant_internal_gains(
    model: &ep_model::TypedModel,
    schedule_cache: &ScheduleSeriesCache,
    surfaces: &mut [crate::heat_balance::state::SurfaceHeatBalanceState],
    operations: Option<&mut InternalGainSchedulePhaseOperations>,
) {
    if let Some(operations) = operations {
        update_surface_radiant_internal_gain_source_terms_from_cache_profiled(
            model,
            schedule_cache,
            surfaces,
            1,
            operations,
        );
    } else {
        update_surface_radiant_internal_gain_source_terms_from_cache(
            model,
            schedule_cache,
            surfaces,
            1,
        );
    }
}
