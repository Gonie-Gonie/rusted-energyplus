//! Bounded `CalcPurchAirLoads` cooling economizer inner condition.

use ep_model::{IdealLoadsAirSystemId, OutdoorAirEconomizerType, ZoneId};

use super::super::PurchasedAirRuntimeState;

mod release;
mod transition;

pub use release::*;
pub(in crate::ideal_loads::calc) use release::{
    completed_direct_economizer_condition_is_consistent,
    completed_direct_prefix_through_economizer_guard_is_consistent,
    exact_direct_initialization_is_consistent,
};
pub(super) use transition::advance_cooling_economizer_condition_state;

/// EnergyPlus source slice represented by this bounded transition.
pub const PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2083-2086";

/// Lexically first executable statement deliberately left for the next slice.
pub const PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_FIRST_EXCLUDED_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2089";

/// Exact left-to-right short-circuit sites represented by this condition.
pub const PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_SOURCE_ORDER: &[&str] = &[
    "read-economizer-type-for-differential-dry-bulb",
    "compare-economizer-type-equal-to-differential-dry-bulb",
    "read-outdoor-air-node-temperature-after-dry-bulb-match",
    "read-zone-recirculation-air-node-temperature-after-dry-bulb-match",
    "compare-strict-outdoor-temperature-below-zone-recirculation-temperature",
    "read-economizer-type-for-differential-enthalpy-after-dry-bulb-arm-false",
    "compare-economizer-type-equal-to-differential-enthalpy",
    "read-outdoor-air-node-enthalpy-after-enthalpy-match",
    "read-zone-recirculation-air-node-enthalpy-after-enthalpy-match",
    "compare-strict-outdoor-enthalpy-below-zone-recirculation-enthalpy",
    "select-excluded-line-2089-if-compound-condition-satisfied",
    "select-excluded-line-2109-if-compound-condition-false",
];

/// Inputs used only by the internal source-characterization transition.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcCoolingEconomizerConditionInput {
    pub economizer_type: OutdoorAirEconomizerType,
    pub outdoor_air_temperature_c: f64,
    pub recirculation_air_temperature_c: f64,
    pub outdoor_air_enthalpy_j_per_kg: f64,
    pub recirculation_air_enthalpy_j_per_kg: f64,
}

/// One CP315-to-CP316 cooling economizer condition result.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingEconomizerConditionSnapshot {
    /// EnergyPlus source slice.
    pub source: &'static str,
    /// First executable source statement deliberately excluded.
    pub first_excluded_source: &'static str,
    /// Selected IdealLoads system.
    pub system: IdealLoadsAirSystemId,
    /// CP315 parent call ordinal consumed by this transition.
    pub parent_call_ordinal: usize,
    /// Source-order sites represented by the bounded route.
    pub source_order: &'static [&'static str],
    /// Controlled Zone inherited from CP315.
    pub controlled_zone: ZoneId,
    /// Whether the enclosing CP310 unit body was entered.
    pub unit_body_entered: bool,
    /// Whether CP312 entered cooling.
    pub predecessor_cooling_body_entered: bool,
    /// Whether CP313 admitted the CP314 sibling body.
    pub predecessor_maximum_cooling_flow_body_entered: bool,
    /// Whether CP314 fell through toward the economizer path.
    pub predecessor_active_guard_false_economizer_fallthrough: bool,
    /// Whether CP315 evaluated its outer economizer guard.
    pub predecessor_economizer_guard_evaluated: bool,
    /// Whether CP315 admitted this inner condition.
    pub predecessor_economizer_body_entered: bool,
    /// Whether CP315's `NoEconomizer` comparison fell through.
    pub predecessor_no_economizer_fallthrough: bool,
    /// Whether UnitOff skipped every CP316 source site.
    pub unit_off_skipped: bool,
    /// Whether an active non-cooling route skipped every CP316 site.
    pub non_cooling_skipped: bool,
    /// Whether the CP313 true sibling skipped every CP316 site.
    pub maximum_cooling_flow_body_sibling_skipped: bool,
    /// Whether a false CP315 outer guard skipped every CP316 site.
    pub no_economizer_outer_guard_fallthrough_skipped: bool,
    /// Whether the compound lines-2083 through-2086 condition was evaluated.
    pub economizer_condition_evaluated: bool,
    /// Whether the first economizer-type read executed.
    pub differential_dry_bulb_economizer_type_read: bool,
    /// Economizer type read by the DryBulb selector.
    pub differential_dry_bulb_economizer_type: Option<OutdoorAirEconomizerType>,
    /// Whether the DryBulb selector comparison executed.
    pub differential_dry_bulb_selector_comparison_evaluated: bool,
    /// Result of the DryBulb selector comparison.
    pub differential_dry_bulb_selector_matched: Option<bool>,
    /// Whether the outdoor-air Node temperature was read.
    pub outdoor_air_temperature_read: bool,
    /// Raw outdoor-air Node temperature, absent when short-circuited.
    pub outdoor_air_temperature_c: Option<f64>,
    /// Whether the recirculation-air Node temperature was read.
    pub recirculation_air_temperature_read: bool,
    /// Raw recirculation-air Node temperature, absent when short-circuited.
    pub recirculation_air_temperature_c: Option<f64>,
    /// Whether the strict temperature comparison executed.
    pub dry_bulb_temperature_comparison_evaluated: bool,
    /// Result of the strict outdoor-below-recirculation temperature comparison.
    pub outdoor_air_temperature_below_recirculation_temperature: Option<bool>,
    /// Whether the second economizer-type read executed.
    pub differential_enthalpy_economizer_type_read: bool,
    /// Economizer type read by the Enthalpy selector.
    pub differential_enthalpy_economizer_type: Option<OutdoorAirEconomizerType>,
    /// Whether the Enthalpy selector comparison executed.
    pub differential_enthalpy_selector_comparison_evaluated: bool,
    /// Result of the Enthalpy selector comparison.
    pub differential_enthalpy_selector_matched: Option<bool>,
    /// Whether the outdoor-air Node stored enthalpy was read.
    pub outdoor_air_enthalpy_read: bool,
    /// Raw outdoor-air Node stored enthalpy, absent when short-circuited.
    pub outdoor_air_enthalpy_j_per_kg: Option<f64>,
    /// Whether the recirculation-air Node stored enthalpy was read.
    pub recirculation_air_enthalpy_read: bool,
    /// Raw recirculation-air Node stored enthalpy, absent when short-circuited.
    pub recirculation_air_enthalpy_j_per_kg: Option<f64>,
    /// Whether the strict stored-enthalpy comparison executed.
    pub enthalpy_comparison_evaluated: bool,
    /// Result of the strict outdoor-below-recirculation enthalpy comparison.
    pub outdoor_air_enthalpy_below_recirculation_enthalpy: Option<bool>,
    /// Result of the complete compound condition, absent when skipped.
    pub economizer_condition_satisfied: Option<bool>,
    /// Whether a true result selected excluded line 2089.
    pub economizer_calculation_body_entered: bool,
    /// Whether a false evaluated result selected excluded line 2109.
    pub economizer_condition_fallthrough: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PurchasedAirCalcCoolingEconomizerConditionRetainedRoute {
    UnitOff,
    NonCooling,
    MaximumCoolingFlowBodySibling,
    NoEconomizerOuterGuardFallthrough,
    Evaluated,
}

/// Persistent bounded state for one system's CP316 transitions.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingEconomizerConditionRuntimeState {
    /// Selected IdealLoads system.
    pub system: IdealLoadsAirSystemId,
    /// CP315 snapshots consumed, including every skip class.
    pub transition_count: usize,
    /// Transitions that evaluated the compound condition.
    pub condition_evaluation_count: usize,
    /// Transitions skipped because the enclosing unit was off.
    pub unit_off_skip_count: usize,
    /// Active transitions skipped because cooling was not selected.
    pub non_cooling_skip_count: usize,
    /// Transitions skipped by the CP313 true sibling.
    pub maximum_cooling_flow_body_sibling_skip_count: usize,
    /// Transitions skipped by a false CP315 `NoEconomizer` guard.
    pub no_economizer_outer_guard_fallthrough_skip_count: usize,
    /// Economizer-type reads for the DryBulb selector.
    pub differential_dry_bulb_economizer_type_read_count: usize,
    /// DryBulb selector comparisons.
    pub differential_dry_bulb_selector_comparison_count: usize,
    /// DryBulb selector matches.
    pub differential_dry_bulb_selector_match_count: usize,
    /// Outdoor-air temperature reads.
    pub outdoor_air_temperature_read_count: usize,
    /// Recirculation-air temperature reads.
    pub recirculation_air_temperature_read_count: usize,
    /// Strict temperature comparisons.
    pub dry_bulb_temperature_comparison_count: usize,
    /// Strict temperature comparisons that succeeded.
    pub dry_bulb_temperature_comparison_satisfied_count: usize,
    /// Economizer-type reads for the Enthalpy selector.
    pub differential_enthalpy_economizer_type_read_count: usize,
    /// Enthalpy selector comparisons.
    pub differential_enthalpy_selector_comparison_count: usize,
    /// Enthalpy selector matches.
    pub differential_enthalpy_selector_match_count: usize,
    /// Outdoor-air stored-enthalpy reads.
    pub outdoor_air_enthalpy_read_count: usize,
    /// Recirculation-air stored-enthalpy reads.
    pub recirculation_air_enthalpy_read_count: usize,
    /// Strict stored-enthalpy comparisons.
    pub enthalpy_comparison_count: usize,
    /// Strict stored-enthalpy comparisons that succeeded.
    pub enthalpy_comparison_satisfied_count: usize,
    /// True conditions selecting excluded line 2089.
    pub economizer_calculation_body_entry_count: usize,
    /// False evaluated conditions selecting excluded line 2109.
    pub economizer_condition_fallthrough_count: usize,
    /// Latest transition snapshot; no timestep log is retained.
    pub latest: Option<PurchasedAirCalcCoolingEconomizerConditionSnapshot>,
    latest_route: Option<PurchasedAirCalcCoolingEconomizerConditionRetainedRoute>,
    latest_transition_ordinal: Option<usize>,
}

impl PurchasedAirCalcCoolingEconomizerConditionRuntimeState {
    /// Creates bounded CP316 state for one selected system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            condition_evaluation_count: 0,
            unit_off_skip_count: 0,
            non_cooling_skip_count: 0,
            maximum_cooling_flow_body_sibling_skip_count: 0,
            no_economizer_outer_guard_fallthrough_skip_count: 0,
            differential_dry_bulb_economizer_type_read_count: 0,
            differential_dry_bulb_selector_comparison_count: 0,
            differential_dry_bulb_selector_match_count: 0,
            outdoor_air_temperature_read_count: 0,
            recirculation_air_temperature_read_count: 0,
            dry_bulb_temperature_comparison_count: 0,
            dry_bulb_temperature_comparison_satisfied_count: 0,
            differential_enthalpy_economizer_type_read_count: 0,
            differential_enthalpy_selector_comparison_count: 0,
            differential_enthalpy_selector_match_count: 0,
            outdoor_air_enthalpy_read_count: 0,
            recirculation_air_enthalpy_read_count: 0,
            enthalpy_comparison_count: 0,
            enthalpy_comparison_satisfied_count: 0,
            economizer_calculation_body_entry_count: 0,
            economizer_condition_fallthrough_count: 0,
            latest: None,
            latest_route: None,
            latest_transition_ordinal: None,
        }
    }
}

/// Final selected-unit CP316 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingEconomizerConditionLifecycleSummary {
    /// EnergyPlus source slice.
    pub source: &'static str,
    /// First executable statement deliberately excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state: PurchasedAirCalcCoolingEconomizerConditionRuntimeState,
}

/// Returns the bounded selected-unit CP316 lifecycle summary.
pub fn purchased_air_calc_cooling_economizer_condition_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingEconomizerConditionLifecycleSummary,
    PurchasedAirCalcCoolingEconomizerConditionError,
> {
    let unit = runtime
        .units
        .get(&system)
        .ok_or(PurchasedAirCalcCoolingEconomizerConditionError::UnknownSystem { system })?;
    Ok(PurchasedAirCalcCoolingEconomizerConditionLifecycleSummary {
        source: PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_FIRST_EXCLUDED_SOURCE,
        state: unit.calc_cooling_economizer_condition.clone(),
    })
}
