//! Bounded `CalcPurchAirLoads` cooling OA/max-flow gate lifecycle.

use ep_model::{IdealLoadsAirSystemId, IdealLoadsLimit, ZoneId};

use super::super::PurchasedAirRuntimeState;
use super::cooling_entry_gate::PurchasedAirCalcCoolingEntryGateSnapshot;

mod release;

pub use release::*;

/// EnergyPlus source slice represented by this bounded transition.
pub const PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2056-2057";

/// Lexically first executable statement deliberately left for the next slice.
pub const PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_FIRST_EXCLUDED_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2058";

/// Exact source-order sites represented by the bounded gate.
pub const PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_SOURCE_ORDER: &[&str] = &[
    "compare-cooling-limit-to-flow-rate",
    "compare-cooling-limit-to-flow-rate-and-capacity-after-short-circuit",
    "read-outdoor-air-mass-flow-after-limit-short-circuit",
    "read-maximum-cooling-air-mass-flow-after-limit-short-circuit",
    "compare-strict-outdoor-air-above-maximum-cooling-flow",
    "enter-maximum-cooling-flow-body-if-satisfied",
];

/// One CP312-to-CP313 cooling OA/max-flow transition result.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingOaMaxFlowGateSnapshot {
    /// EnergyPlus source slice.
    pub source: &'static str,
    /// First executable source statement not represented by this slice.
    pub first_excluded_source: &'static str,
    /// Selected IdealLoads system.
    pub system: IdealLoadsAirSystemId,
    /// CP312 parent call ordinal consumed by this transition.
    pub parent_call_ordinal: usize,
    /// Source-order sites represented by the bounded route.
    pub source_order: &'static [&'static str],
    /// Controlled Zone inherited from CP312.
    pub controlled_zone: ZoneId,
    /// Whether the enclosing CP310 `UnitOn` body was entered.
    pub unit_body_entered: bool,
    /// Whether CP312 admitted the cooling body containing lines 2056-2057.
    pub predecessor_cooling_body_entered: bool,
    /// Whether UnitOff skipped every CP313 site.
    pub unit_off_skipped: bool,
    /// Whether an active non-cooling result skipped every CP313 site.
    pub non_cooling_skipped: bool,
    /// Whether the first `CoolingLimit == FlowRate` comparison executed.
    pub cooling_limit_flow_rate_comparison_evaluated: bool,
    /// Whether the first comparison read `CoolingLimit`.
    pub cooling_limit_flow_rate_read: bool,
    /// Cooling-limit value read by the first comparison.
    pub cooling_limit_flow_rate_value: Option<IdealLoadsLimit>,
    /// Result of the first selector comparison, absent when skipped.
    pub cooling_limit_flow_rate_comparison_satisfied: Option<bool>,
    /// Whether `||` short-circuiting reached the second selector comparison.
    pub cooling_limit_flow_rate_and_capacity_comparison_evaluated: bool,
    /// Whether the second comparison read `CoolingLimit`.
    pub cooling_limit_flow_rate_and_capacity_read: bool,
    /// Cooling-limit value read by the second comparison.
    pub cooling_limit_flow_rate_and_capacity_value: Option<IdealLoadsLimit>,
    /// Result of the second selector comparison, absent when skipped.
    pub cooling_limit_flow_rate_and_capacity_comparison_satisfied: Option<bool>,
    /// Result of the complete flow-limit selector, absent when CP313 was skipped.
    pub cooling_flow_limit_active: Option<bool>,
    /// Whether `&&` short-circuiting reached the local OA mass-flow read.
    pub outdoor_air_mass_flow_rate_read: bool,
    /// Local `OAMassFlowRate`, absent when the read site was skipped.
    pub outdoor_air_mass_flow_rate_kg_per_s: Option<f64>,
    /// Whether `&&` short-circuiting reached the cached maximum-flow read.
    pub maximum_cooling_air_mass_flow_rate_read: bool,
    /// Cached `MaxCoolMassFlowRate`, absent when the read site was skipped.
    pub maximum_cooling_air_mass_flow_rate_kg_per_s: Option<f64>,
    /// Whether the strict mass-flow comparison executed.
    pub strict_mass_flow_comparison_evaluated: bool,
    /// Result of `OAMassFlowRate > MaxCoolMassFlowRate`, absent when skipped.
    pub outdoor_air_mass_flow_above_maximum: Option<bool>,
    /// Whether execution entered the line-2058 body deliberately left excluded.
    pub maximum_cooling_flow_body_entered: bool,
}

/// Persistent bounded state for one system's cooling OA/max-flow transitions.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingOaMaxFlowGateRuntimeState {
    /// Selected IdealLoads system.
    pub system: IdealLoadsAirSystemId,
    /// CP312 snapshots consumed, including both skip classes.
    pub transition_count: usize,
    /// Transitions that executed the line-2056 selector.
    pub source_execution_count: usize,
    /// Transitions skipped because the enclosing unit was off.
    pub unit_off_skip_count: usize,
    /// Active transitions skipped because CP312 did not select cooling.
    pub non_cooling_skip_count: usize,
    /// First selector comparisons executed.
    pub cooling_limit_flow_rate_comparison_count: usize,
    /// First selector comparisons that matched `FlowRate`.
    pub cooling_limit_flow_rate_match_count: usize,
    /// Second selector comparisons reached after `||` short-circuiting.
    pub cooling_limit_flow_rate_and_capacity_comparison_count: usize,
    /// Second selector comparisons that matched `FlowRateAndCapacity`.
    pub cooling_limit_flow_rate_and_capacity_match_count: usize,
    /// Local outdoor-air mass-flow reads reached.
    pub outdoor_air_mass_flow_rate_read_count: usize,
    /// Cached maximum cooling mass-flow reads reached.
    pub maximum_cooling_air_mass_flow_rate_read_count: usize,
    /// Strict `OAMassFlowRate > MaxCoolMassFlowRate` comparisons executed.
    pub strict_mass_flow_comparison_count: usize,
    /// Strict mass-flow comparisons that succeeded.
    pub strict_mass_flow_comparison_satisfied_count: usize,
    /// Entries into the deliberately excluded line-2058 body.
    pub maximum_cooling_flow_body_entry_count: usize,
    /// Executed source predicates that fell through to the economizer path.
    pub active_fallthrough_count: usize,
    /// Latest transition snapshot; no timestep log is retained.
    pub latest: Option<PurchasedAirCalcCoolingOaMaxFlowGateSnapshot>,
}

impl PurchasedAirCalcCoolingOaMaxFlowGateRuntimeState {
    /// Creates bounded state for one selected system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            source_execution_count: 0,
            unit_off_skip_count: 0,
            non_cooling_skip_count: 0,
            cooling_limit_flow_rate_comparison_count: 0,
            cooling_limit_flow_rate_match_count: 0,
            cooling_limit_flow_rate_and_capacity_comparison_count: 0,
            cooling_limit_flow_rate_and_capacity_match_count: 0,
            outdoor_air_mass_flow_rate_read_count: 0,
            maximum_cooling_air_mass_flow_rate_read_count: 0,
            strict_mass_flow_comparison_count: 0,
            strict_mass_flow_comparison_satisfied_count: 0,
            maximum_cooling_flow_body_entry_count: 0,
            active_fallthrough_count: 0,
            latest: None,
        }
    }
}

/// Final selected-unit CP313 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingOaMaxFlowGateLifecycleSummary {
    /// EnergyPlus source slice.
    pub source: &'static str,
    /// First executable source statement deliberately excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state: PurchasedAirCalcCoolingOaMaxFlowGateRuntimeState,
}

/// Returns the bounded selected-unit CP313 lifecycle summary.
pub fn purchased_air_calc_cooling_oa_max_flow_gate_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingOaMaxFlowGateLifecycleSummary,
    PurchasedAirCalcCoolingOaMaxFlowGateError,
> {
    let unit = runtime
        .units
        .get(&system)
        .ok_or(PurchasedAirCalcCoolingOaMaxFlowGateError::UnknownSystem { system })?;
    Ok(PurchasedAirCalcCoolingOaMaxFlowGateLifecycleSummary {
        source: PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_FIRST_EXCLUDED_SOURCE,
        state: unit.calc_cooling_oa_max_flow_gate.clone(),
    })
}

pub(super) fn advance_cooling_oa_max_flow_gate_state(
    state: &mut PurchasedAirCalcCoolingOaMaxFlowGateRuntimeState,
    cooling_entry_gate: PurchasedAirCalcCoolingEntryGateSnapshot,
    cooling_limit: IdealLoadsLimit,
    outdoor_air_mass_flow_rate_kg_per_s: f64,
    maximum_cooling_air_mass_flow_rate_kg_per_s: f64,
) -> PurchasedAirCalcCoolingOaMaxFlowGateSnapshot {
    state.transition_count += 1;
    let source_executed = cooling_entry_gate.cooling_body_entered;
    let unit_off_skipped = !source_executed && !cooling_entry_gate.unit_body_entered;
    let non_cooling_skipped = !source_executed && cooling_entry_gate.unit_body_entered;

    let (cooling_limit_flow_rate_value, cooling_limit_flow_rate_comparison_satisfied) =
        if source_executed {
            (
                Some(cooling_limit),
                Some(cooling_limit == IdealLoadsLimit::LimitFlowRate),
            )
        } else {
            (None, None)
        };
    let cooling_limit_flow_rate_and_capacity_comparison_evaluated =
        cooling_limit_flow_rate_comparison_satisfied == Some(false);
    let (
        cooling_limit_flow_rate_and_capacity_value,
        cooling_limit_flow_rate_and_capacity_comparison_satisfied,
    ) = if cooling_limit_flow_rate_and_capacity_comparison_evaluated {
        (
            Some(cooling_limit),
            Some(cooling_limit == IdealLoadsLimit::LimitFlowRateAndCapacity),
        )
    } else {
        (None, None)
    };
    let cooling_flow_limit_active = source_executed.then_some(
        cooling_limit_flow_rate_comparison_satisfied == Some(true)
            || cooling_limit_flow_rate_and_capacity_comparison_satisfied == Some(true),
    );
    let mass_flow_operands_read = cooling_flow_limit_active == Some(true);
    let (
        outdoor_air_mass_flow_rate_value,
        maximum_cooling_air_mass_flow_rate_value,
        outdoor_air_mass_flow_above_maximum,
    ) = if mass_flow_operands_read {
        (
            Some(outdoor_air_mass_flow_rate_kg_per_s),
            Some(maximum_cooling_air_mass_flow_rate_kg_per_s),
            Some(outdoor_air_mass_flow_rate_kg_per_s > maximum_cooling_air_mass_flow_rate_kg_per_s),
        )
    } else {
        (None, None, None)
    };
    let maximum_cooling_flow_body_entered = outdoor_air_mass_flow_above_maximum == Some(true);

    if source_executed {
        state.source_execution_count += 1;
        state.cooling_limit_flow_rate_comparison_count += 1;
        if cooling_limit_flow_rate_comparison_satisfied == Some(true) {
            state.cooling_limit_flow_rate_match_count += 1;
        }
        if cooling_limit_flow_rate_and_capacity_comparison_evaluated {
            state.cooling_limit_flow_rate_and_capacity_comparison_count += 1;
        }
        if cooling_limit_flow_rate_and_capacity_comparison_satisfied == Some(true) {
            state.cooling_limit_flow_rate_and_capacity_match_count += 1;
        }
        if mass_flow_operands_read {
            state.outdoor_air_mass_flow_rate_read_count += 1;
            state.maximum_cooling_air_mass_flow_rate_read_count += 1;
            state.strict_mass_flow_comparison_count += 1;
        }
        if maximum_cooling_flow_body_entered {
            state.strict_mass_flow_comparison_satisfied_count += 1;
            state.maximum_cooling_flow_body_entry_count += 1;
        } else {
            state.active_fallthrough_count += 1;
        }
    } else if unit_off_skipped {
        state.unit_off_skip_count += 1;
    } else {
        state.non_cooling_skip_count += 1;
    }

    let snapshot = PurchasedAirCalcCoolingOaMaxFlowGateSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_FIRST_EXCLUDED_SOURCE,
        system: state.system,
        parent_call_ordinal: cooling_entry_gate.parent_call_ordinal,
        source_order: PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_SOURCE_ORDER,
        controlled_zone: cooling_entry_gate.controlled_zone,
        unit_body_entered: cooling_entry_gate.unit_body_entered,
        predecessor_cooling_body_entered: cooling_entry_gate.cooling_body_entered,
        unit_off_skipped,
        non_cooling_skipped,
        cooling_limit_flow_rate_comparison_evaluated: source_executed,
        cooling_limit_flow_rate_read: source_executed,
        cooling_limit_flow_rate_value,
        cooling_limit_flow_rate_comparison_satisfied,
        cooling_limit_flow_rate_and_capacity_comparison_evaluated,
        cooling_limit_flow_rate_and_capacity_read:
            cooling_limit_flow_rate_and_capacity_comparison_evaluated,
        cooling_limit_flow_rate_and_capacity_value,
        cooling_limit_flow_rate_and_capacity_comparison_satisfied,
        cooling_flow_limit_active,
        outdoor_air_mass_flow_rate_read: mass_flow_operands_read,
        outdoor_air_mass_flow_rate_kg_per_s: outdoor_air_mass_flow_rate_value,
        maximum_cooling_air_mass_flow_rate_read: mass_flow_operands_read,
        maximum_cooling_air_mass_flow_rate_kg_per_s: maximum_cooling_air_mass_flow_rate_value,
        strict_mass_flow_comparison_evaluated: mass_flow_operands_read,
        outdoor_air_mass_flow_above_maximum,
        maximum_cooling_flow_body_entered,
    };
    state.latest = Some(snapshot);
    snapshot
}
