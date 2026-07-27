//! Bounded `CalcPurchAirLoads` cooling economizer outer guard.

use ep_model::{IdealLoadsAirSystemId, OutdoorAirEconomizerType, ZoneId};

use super::super::PurchasedAirRuntimeState;

mod release;
mod transition;

pub use release::*;
pub(super) use transition::advance_cooling_economizer_guard_state;

/// EnergyPlus source slice represented by this bounded transition.
pub const PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2082";

/// Lexically first executable statement deliberately left for the next slice.
pub const PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_FIRST_EXCLUDED_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2083";

/// Exact source-order sites represented by the bounded guard.
pub const PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_SOURCE_ORDER: &[&str] = &[
    "read-economizer-type",
    "compare-economizer-type-not-equal-to-no-economizer",
    "enter-economizer-body-if-satisfied",
];

/// One CP314-to-CP315 cooling economizer outer-guard result.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingEconomizerGuardSnapshot {
    /// EnergyPlus source slice.
    pub source: &'static str,
    /// First executable source statement deliberately excluded.
    pub first_excluded_source: &'static str,
    /// Selected IdealLoads system.
    pub system: IdealLoadsAirSystemId,
    /// CP314 parent call ordinal consumed by this transition.
    pub parent_call_ordinal: usize,
    /// Source-order sites represented by the bounded route.
    pub source_order: &'static [&'static str],
    /// Controlled Zone inherited from CP314.
    pub controlled_zone: ZoneId,
    /// Whether the enclosing CP310 unit body was entered.
    pub unit_body_entered: bool,
    /// Whether CP312 entered cooling.
    pub predecessor_cooling_body_entered: bool,
    /// Whether CP313 admitted the CP314 maximum-flow body.
    pub predecessor_maximum_cooling_flow_body_entered: bool,
    /// Whether CP314 fell through to line 2082 after a false CP313 guard.
    pub predecessor_active_guard_false_economizer_fallthrough: bool,
    /// Whether UnitOff skipped every CP315 source site.
    pub unit_off_skipped: bool,
    /// Whether an active non-cooling route skipped every CP315 source site.
    pub non_cooling_skipped: bool,
    /// Whether the true CP313 sibling body skipped every CP315 source site.
    pub maximum_cooling_flow_body_sibling_skipped: bool,
    /// Whether the line-2082 outer guard was evaluated.
    pub economizer_guard_evaluated: bool,
    /// Whether `PurchAir.EconomizerType` was read.
    pub economizer_type_read: bool,
    /// Economizer type read at line 2082, absent when the guard was skipped.
    pub economizer_type: Option<OutdoorAirEconomizerType>,
    /// Whether the `!= Econ::NoEconomizer` comparison executed.
    pub no_economizer_comparison_evaluated: bool,
    /// Result of the line-2082 comparison, absent when skipped.
    pub economizer_not_no_economizer: Option<bool>,
    /// Whether a true guard would next enter excluded line 2083; CP315 does not execute it.
    pub economizer_body_entered: bool,
    /// Whether `NoEconomizer` would next reach excluded line 2109; CP315 does not execute it.
    pub no_economizer_fallthrough: bool,
}

/// Persistent bounded state for one system's CP315 transitions.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingEconomizerGuardRuntimeState {
    /// Selected IdealLoads system.
    pub system: IdealLoadsAirSystemId,
    /// CP314 snapshots consumed, including all skip classes.
    pub transition_count: usize,
    /// Transitions that evaluated the line-2082 guard.
    pub guard_evaluation_count: usize,
    /// Transitions skipped because the enclosing unit was off.
    pub unit_off_skip_count: usize,
    /// Active transitions skipped because cooling was not selected.
    pub non_cooling_skip_count: usize,
    /// Transitions skipped because the CP313 sibling body was entered.
    pub maximum_cooling_flow_body_sibling_skip_count: usize,
    /// Line-2082 economizer-type reads.
    pub economizer_type_read_count: usize,
    /// Line-2082 `!= NoEconomizer` comparisons.
    pub no_economizer_comparison_count: usize,
    /// True guards selecting excluded line 2083 as the next dynamic site.
    pub economizer_body_entry_count: usize,
    /// False guards selecting excluded line 2109 as the next dynamic site.
    pub no_economizer_fallthrough_count: usize,
    /// Latest transition snapshot; no timestep log is retained.
    pub latest: Option<PurchasedAirCalcCoolingEconomizerGuardSnapshot>,
}

impl PurchasedAirCalcCoolingEconomizerGuardRuntimeState {
    /// Creates bounded state for one selected system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            guard_evaluation_count: 0,
            unit_off_skip_count: 0,
            non_cooling_skip_count: 0,
            maximum_cooling_flow_body_sibling_skip_count: 0,
            economizer_type_read_count: 0,
            no_economizer_comparison_count: 0,
            economizer_body_entry_count: 0,
            no_economizer_fallthrough_count: 0,
            latest: None,
        }
    }
}

/// Final selected-unit CP315 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingEconomizerGuardLifecycleSummary {
    /// EnergyPlus source slice.
    pub source: &'static str,
    /// First executable statement deliberately excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state: PurchasedAirCalcCoolingEconomizerGuardRuntimeState,
}

/// Returns the bounded selected-unit CP315 lifecycle summary.
pub fn purchased_air_calc_cooling_economizer_guard_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingEconomizerGuardLifecycleSummary,
    PurchasedAirCalcCoolingEconomizerGuardError,
> {
    let unit = runtime
        .units
        .get(&system)
        .ok_or(PurchasedAirCalcCoolingEconomizerGuardError::UnknownSystem { system })?;
    Ok(PurchasedAirCalcCoolingEconomizerGuardLifecycleSummary {
        source: PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_FIRST_EXCLUDED_SOURCE,
        state: unit.calc_cooling_economizer_guard.clone(),
    })
}
