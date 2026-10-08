//! Passive copies of actual environment initialization and solver-entry state.
//!
//! The transient selected owner is distinct from its stored three-slot solver
//! projection. No source weather, EnergyPlus callback phase or member count is
//! inferred. Later history evolution remains outside these observations.

use super::{
    HeatBalanceState, ZoneAirCallerTemperatureInputs, ZoneAirEnvironmentInvocation,
    ZoneAirInitializationState, ZoneHeatBalanceState,
};
use crate::psychrometrics::production_trace::{ExecutionContext, current_execution_scope};
use ep_model::ZoneId;
use std::{cell::RefCell, panic::Location};

const OBSERVATION_LIMIT: usize = 10_000;
thread_local! {
    static ACTIVE: RefCell<Option<ZoneAirInitializationTrace>> = RefCell::default();
}

/// Copies of actual legacy fields after owner projection or at solver entry.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LegacyZoneAirInitializationProjection {
    /// Actual stored MAT projection.
    pub mean_air_temperature_c: f64,
    /// Actual stored ZTAV projection.
    pub zone_timestep_average_air_temperature_c: f64,
    /// Actual stored current humidity ratio.
    pub air_humidity_ratio: f64,
    /// Actual stored zone-average humidity ratio.
    pub zone_timestep_average_air_humidity_ratio: f64,
    /// Actual XMAT first-three projection, distinct from ZTM.
    pub previous_mean_air_temperatures_c: [f64; 3],
    /// Actual DSXMAT first-three projection.
    pub previous_system_mean_air_temperatures_c: [f64; 3],
    /// Actual WPrevZoneTS first-three projection.
    pub previous_air_humidity_ratios: [f64; 3],
    /// Actual DSWPrevZoneTS first-three projection.
    pub previous_system_air_humidity_ratios: [f64; 3],
    /// Actual stored tempIndLoad projection, separate from later coefficients.
    pub third_order_temp_independent_load_w: f64,
    /// Actual stored tempDepLoad projection.
    pub third_order_temp_dependent_load_w_per_k: f64,
    /// Actual stored AirPowerCap W/K; not air heat capacity J/K.
    pub air_power_cap_w_per_k: f64,
}

/// Already-computed owner snapshots and actual post-store reads supplied by caller.
#[derive(Clone, Debug)]
pub struct ZoneAirInitializationObservation {
    /// Own typed identity, never a native array index.
    pub zone_id: ZoneId,
    /// Own normalized zone name.
    pub zone_name: String,
    /// Actual existing weather-provider output passed to this owner.
    pub out_hum_rat: f64,
    /// Actual existing caller temperatures before selected preparation.
    pub caller_temperature_inputs: ZoneAirCallerTemperatureInputs,
    /// Actual newly constructed selected owner.
    pub constructor: ZoneAirInitializationState,
    /// Actual owner after reconstruction/current-W seeding.
    pub after_bulk: ZoneAirInitializationState,
    /// Actual owner after caller preparation and before guarded initialization.
    pub before_begin: ZoneAirInitializationState,
    /// Actual owner returned by the guarded operation.
    pub after_begin: ZoneAirInitializationState,
    /// Actual persistent global guard invocation; count is Rust-only.
    pub guard: ZoneAirEnvironmentInvocation,
    /// Actual stored solver fields read after projection.
    pub handoff: LegacyZoneAirInitializationProjection,
}

/// One initializer observation with real enclosing Rust scope and caller.
#[derive(Clone, Debug)]
pub struct ZoneAirInitializationRow {
    /// Actual caller location.
    pub caller: &'static Location<'static>,
    /// Actual Rust operation label, not an EnergyPlus stage.
    pub phase: &'static str,
    /// Existing execution context, when available.
    pub context: Option<ExecutionContext>,
    /// Copies supplied after real initialization and stores.
    pub observation: ZoneAirInitializationObservation,
}

/// One zone's stored fields as the shared timestep solver receives them.
#[derive(Clone, Debug)]
pub struct ZoneAirTimestepEntryZone {
    /// Own typed zone identity.
    pub zone_id: ZoneId,
    /// Own zone name.
    pub zone_name: String,
    /// Stored fields read before history shifts or solver work.
    pub handoff: LegacyZoneAirInitializationProjection,
}

/// A real solver entry whose actual state index is zero, possibly during warmup.
#[derive(Clone, Debug)]
pub struct ZoneAirTimestepEntryObservation {
    /// Actual caller, before the shared timestep solver's state reads.
    pub caller: &'static Location<'static>,
    /// Actual Rust scope, with no inferred native phase.
    pub phase: &'static str,
    /// Existing invocation context, when available.
    pub context: Option<ExecutionContext>,
    /// Actual state index, retained only when zero.
    pub timestep_index: usize,
    /// Actual persistent environment guard read at this entry.
    pub my_environment_flag: bool,
    /// Actual stored zone projections, in state storage order.
    pub zones: Vec<ZoneAirTimestepEntryZone>,
}

/// Separate bounded prefixes of initializer returns and zero-index solver entries.
#[derive(Clone, Debug, Default)]
pub struct ZoneAirInitializationTrace {
    /// All actual initializer records on the collecting thread.
    pub total_initializer_count: u64,
    /// Actual initializer return/store observations in order.
    pub initializer_observations: Vec<ZoneAirInitializationRow>,
    /// All active shared solver-entry hook calls, including nonzero indices.
    pub total_timestep_entry_call_count: u64,
    /// Actual entry-hook calls whose state index was zero.
    pub zero_index_timestep_entry_count: u64,
    /// Actual zero-index entries in order, before any history shift.
    pub timestep_entry_observations: Vec<ZoneAirTimestepEntryObservation>,
}

struct Guard(Option<ZoneAirInitializationTrace>);
impl Drop for Guard {
    fn drop(&mut self) {
        ACTIVE.with_borrow_mut(|active| *active = self.0.take());
    }
}

/// Run unchanged; disabled captures allocate nothing and nested/unwinding calls
/// restore their predecessor. Each retained series is a contiguous prefix.
pub fn capture<R>(
    enabled: bool,
    execute: impl FnOnce() -> R,
) -> (R, Option<ZoneAirInitializationTrace>) {
    if !enabled {
        return (execute(), None);
    }
    let previous =
        ACTIVE.with_borrow_mut(|active| active.replace(ZoneAirInitializationTrace::default()));
    let _guard = Guard(previous);
    let result = execute();
    let trace = ACTIVE.with_borrow_mut(Option::take);
    (result, trace)
}

/// Allows callers to avoid copying optional snapshots while capture is disabled.
#[must_use]
pub fn is_active() -> bool {
    ACTIVE.with_borrow(Option::is_some)
}

/// Copy stored values; no selected initialization or coefficient calculation.
#[must_use]
pub fn capture_projection(zone: &ZoneHeatBalanceState) -> LegacyZoneAirInitializationProjection {
    LegacyZoneAirInitializationProjection {
        mean_air_temperature_c: zone.mean_air_temperature_c,
        zone_timestep_average_air_temperature_c: zone.zone_timestep_average_air_temperature_c,
        air_humidity_ratio: zone.air_humidity_ratio,
        zone_timestep_average_air_humidity_ratio: zone.zone_timestep_average_air_humidity_ratio,
        previous_mean_air_temperatures_c: zone.previous_mean_air_temperatures_c,
        previous_system_mean_air_temperatures_c: zone.previous_system_mean_air_temperatures_c,
        previous_air_humidity_ratios: zone.previous_air_humidity_ratios,
        previous_system_air_humidity_ratios: zone.previous_system_air_humidity_ratios,
        third_order_temp_independent_load_w: zone
            .zone_air_temperature_coefficients
            .third_order_temp_independent_load_w,
        third_order_temp_dependent_load_w_per_k: zone
            .zone_air_temperature_coefficients
            .third_order_temp_dependent_load_w_per_k,
        air_power_cap_w_per_k: zone.zone_air_temperature_coefficients.air_power_cap_w_per_k,
    }
}

/// Record snapshots supplied by the actual initialization path after its stores.
#[track_caller]
pub fn record(observation: ZoneAirInitializationObservation) {
    if !is_active() {
        return;
    }
    let caller = Location::caller();
    let (phase, context) = current_execution_scope();
    ACTIVE.with_borrow_mut(|active| {
        let Some(trace) = active else { return };
        trace.total_initializer_count += 1;
        if trace.initializer_observations.len() == OBSERVATION_LIMIT {
            return;
        }
        trace
            .initializer_observations
            .push(ZoneAirInitializationRow {
                caller,
                phase,
                context,
                observation,
            });
    });
}

/// Copy the actual first-index state at the shared A/B solver entry. The index
/// filter does not assert a physical/warmup phase or native callback correspondence.
#[track_caller]
pub fn record_timestep_entry(state: &HeatBalanceState) {
    if !is_active() {
        return;
    }
    let caller = Location::caller();
    let (phase, context) = current_execution_scope();
    ACTIVE.with_borrow_mut(|active| {
        let Some(trace) = active else { return };
        trace.total_timestep_entry_call_count += 1;
        if state.timestep_index != 0 {
            return;
        }
        trace.zero_index_timestep_entry_count += 1;
        if trace.timestep_entry_observations.len() == OBSERVATION_LIMIT {
            return;
        }
        trace
            .timestep_entry_observations
            .push(ZoneAirTimestepEntryObservation {
                caller,
                phase,
                context,
                timestep_index: state.timestep_index,
                my_environment_flag: state.zone_air_environment_guard.my_environment_flag,
                zones: state
                    .zones
                    .iter()
                    .map(|zone| ZoneAirTimestepEntryZone {
                        zone_id: zone.zone_id,
                        zone_name: zone.zone_name.clone(),
                        handoff: capture_projection(zone),
                    })
                    .collect(),
            });
    });
}

#[cfg(test)]
mod tests;
