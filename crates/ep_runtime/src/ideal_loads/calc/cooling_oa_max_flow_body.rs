//! Bounded `CalcPurchAirLoads` cooling OA/max-flow warning-and-clamp body.

use ep_model::{IdealLoadsAirSystemId, ZoneId};

use super::super::PurchasedAirRuntimeState;

mod release;
mod transition;

pub use release::*;
pub(super) use transition::advance_cooling_oa_max_flow_body_state;

/// EnergyPlus source slice represented by this bounded transition.
pub const PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2058-2078";

/// First lexically subsequent executable statement outside this slice.
pub const PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_FIRST_EXCLUDED_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2082";

/// Recurring-warning child behavior characterized without a real message sink.
pub const PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_RECURRING_WARNING_CHILD_SOURCE: &str =
    "EnergyPlus 26.1 UtilityRoutines.cc:1146-1194,1293-1379; max-only optional argument";

/// Exact parent and characterized child sites represented in source order.
pub const PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_SOURCE_ORDER: &[&str] = &[
    "read-outdoor-air-mass-flow-for-volume-conversion",
    "read-standard-air-density-for-volume-conversion",
    "calculate-outdoor-air-volume-flow",
    "read-first-warning-counter",
    "compare-first-warning-counter-below-one",
    "enter-first-warning-branch-if-satisfied",
    "increment-first-warning-counter",
    "reach-first-warning-call-site",
    "read-maximum-cooling-air-volume-flow-for-continue-warning",
    "reach-continue-warning-call-site",
    "reach-continue-warning-timestamp-call-site",
    "enter-recurring-warning-branch-otherwise",
    "reach-recurring-warning-call-site-with-max-only-value",
    "characterize-recurring-warning-index-allocation-or-reuse",
    "characterize-recurring-warning-report-maximum",
    "read-maximum-cooling-air-mass-flow-for-clamp",
    "assign-clamped-outdoor-air-mass-flow",
];

/// One CP313-to-CP314 warning-and-clamp body result.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingOaMaxFlowBodySnapshot {
    /// EnergyPlus source slice.
    pub source: &'static str,
    /// First executable source statement deliberately excluded.
    pub first_excluded_source: &'static str,
    /// Characterized recurring-warning child source.
    pub recurring_warning_child_source: &'static str,
    /// Selected IdealLoads system.
    pub system: IdealLoadsAirSystemId,
    /// CP313 parent call ordinal consumed by this transition.
    pub parent_call_ordinal: usize,
    /// Source-order sites represented by the bounded route.
    pub source_order: &'static [&'static str],
    /// Controlled Zone inherited from CP313.
    pub controlled_zone: ZoneId,
    /// Whether the enclosing CP310 unit body was entered.
    pub unit_body_entered: bool,
    /// Whether CP312 entered cooling.
    pub predecessor_cooling_body_entered: bool,
    /// Whether CP313 admitted this warning-and-clamp body.
    pub predecessor_maximum_cooling_flow_body_entered: bool,
    /// Whether CP313 skipped every CP314 source site.
    pub body_skipped: bool,
    /// Whether UnitOff skipped the body.
    pub unit_off_skipped: bool,
    /// Whether an active non-cooling route skipped the body.
    pub non_cooling_skipped: bool,
    /// Whether a false active CP313 guard continued toward the economizer path.
    pub active_guard_false_economizer_fallthrough: bool,
    /// Whether the pre-clamp outdoor-air mass flow was read.
    pub outdoor_air_mass_flow_rate_read: bool,
    /// Pre-clamp outdoor-air mass flow, absent when skipped.
    pub outdoor_air_mass_flow_rate_before_clamp_kg_per_s: Option<f64>,
    /// Whether standard air density was read.
    pub standard_air_density_read: bool,
    /// Standard air density, absent when skipped.
    pub standard_air_density_kg_per_m3: Option<f64>,
    /// Whether outdoor-air volume flow was calculated.
    pub outdoor_air_volume_flow_rate_calculated: bool,
    /// Calculated outdoor-air volume flow, absent when skipped.
    pub outdoor_air_volume_flow_rate_m3_per_s: Option<f64>,
    /// Whether the first-warning counter was read.
    pub warning_counter_read: bool,
    /// Source counter before the `< 1` comparison.
    pub warning_counter_before: Option<usize>,
    /// Result of the first-warning predicate, absent when skipped.
    pub first_warning_predicate_satisfied: Option<bool>,
    /// Whether the first-warning branch was entered.
    pub first_warning_branch_entered: bool,
    /// Whether the source warning counter was incremented.
    pub warning_counter_incremented: bool,
    /// Source counter after this body, absent when skipped.
    pub warning_counter_after: Option<usize>,
    /// Whether the first warning call site was reached.
    pub first_warning_call_site_reached: bool,
    /// Whether maximum cooling volume flow was read for the continue warning.
    pub maximum_cooling_air_volume_flow_rate_read: bool,
    /// Maximum cooling volume flow read by the first-warning branch.
    pub maximum_cooling_air_volume_flow_rate_m3_per_s: Option<f64>,
    /// Whether the continue-warning call site was reached.
    pub continue_warning_call_site_reached: bool,
    /// Whether the timestamp continue-warning call site was reached.
    pub continue_warning_timestamp_call_site_reached: bool,
    /// Whether the recurring-warning branch was entered.
    pub recurring_warning_branch_entered: bool,
    /// Whether the recurring-warning call site was reached.
    pub recurring_warning_call_site_reached: bool,
    /// Max-only numeric argument supplied at the recurring call site.
    pub recurring_warning_report_maximum_input_m3_per_s: Option<f64>,
    /// Whether the local child characterization allocated an identity this call.
    pub characterized_recurring_warning_index_allocated_on_call: bool,
    /// Whether the local child characterization reused its identity this call.
    pub characterized_recurring_warning_index_reused_on_call: bool,
    /// Rust-local recurring index before the child call.
    pub characterized_recurring_warning_index_before: Option<usize>,
    /// Rust-local recurring index after the child call.
    pub characterized_recurring_warning_index_after: Option<usize>,
    /// One-based local recurring occurrence, absent outside that branch.
    pub characterized_recurring_warning_occurrence_ordinal: Option<usize>,
    /// Local max-only report value after this recurring call.
    pub characterized_recurring_warning_report_maximum_m3_per_s: Option<f64>,
    /// Whether this body characterized one total-warning increment.
    pub characterized_total_warning_error_incremented: bool,
    /// Whether maximum cooling mass flow was read for the clamp.
    pub maximum_cooling_air_mass_flow_rate_read: bool,
    /// Maximum cooling mass flow read for the clamp.
    pub maximum_cooling_air_mass_flow_rate_kg_per_s: Option<f64>,
    /// Whether the final source clamp assignment was performed.
    pub outdoor_air_mass_flow_clamp_assignment_performed: bool,
    /// Post-clamp outdoor-air mass flow, absent when skipped.
    pub outdoor_air_mass_flow_rate_after_clamp_kg_per_s: Option<f64>,
}

/// Persistent bounded state for one system's CP314 transitions.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingOaMaxFlowBodyRuntimeState {
    /// Selected IdealLoads system.
    pub system: IdealLoadsAirSystemId,
    /// CP313 snapshots consumed, including skips.
    pub transition_count: usize,
    /// Transitions that entered the lines-2058 through-2078 body.
    pub body_entry_count: usize,
    /// Transitions that skipped every represented site.
    pub body_skip_count: usize,
    /// Transitions skipped because the enclosing unit was off.
    pub unit_off_skip_count: usize,
    /// Active transitions skipped because cooling was not selected.
    pub non_cooling_skip_count: usize,
    /// False active CP313 guards continuing toward the economizer path.
    pub active_guard_false_economizer_fallthrough_count: usize,
    /// Pre-clamp outdoor-air mass-flow reads.
    pub outdoor_air_mass_flow_rate_read_count: usize,
    /// Standard-air-density reads.
    pub standard_air_density_read_count: usize,
    /// Outdoor-air volume-flow calculations.
    pub outdoor_air_volume_flow_calculation_count: usize,
    /// First-warning counter reads.
    pub warning_counter_read_count: usize,
    /// Rust-owned mirror of `OAFlowMaxCoolOutputError`.
    pub outdoor_air_flow_max_cooling_output_error_count: usize,
    /// First-warning branch entries.
    pub first_warning_branch_count: usize,
    /// First-warning counter increments.
    pub warning_counter_increment_count: usize,
    /// First-warning call sites reached.
    pub first_warning_call_site_count: usize,
    /// Maximum cooling volume-flow reads.
    pub maximum_cooling_air_volume_flow_rate_read_count: usize,
    /// Continue-warning call sites reached.
    pub continue_warning_call_site_count: usize,
    /// Timestamp continue-warning call sites reached.
    pub continue_warning_timestamp_call_site_count: usize,
    /// Recurring-warning branch entries.
    pub recurring_warning_branch_count: usize,
    /// Recurring-warning call sites reached.
    pub recurring_warning_call_site_count: usize,
    /// Rust-local recurring identities allocated.
    pub characterized_recurring_warning_index_allocation_count: usize,
    /// Rust-local recurring identities reused.
    pub characterized_recurring_warning_index_reuse_count: usize,
    /// Rust-local recurring-warning occurrences characterized.
    pub characterized_recurring_warning_occurrence_count: usize,
    /// Whether the Rust-local recurring identity has been allocated.
    pub characterized_recurring_warning_index_allocated: bool,
    /// Rust-local mirror of the zero-sentinel recurring index.
    pub outdoor_air_flow_max_cooling_output_index: usize,
    /// Rust-local max-only report value; not a process-global message sink.
    pub characterized_recurring_warning_report_maximum_m3_per_s: Option<f64>,
    /// Characterized total-warning increments, one per entered body.
    pub characterized_total_warning_error_increment_count: usize,
    /// Maximum cooling mass-flow reads.
    pub maximum_cooling_air_mass_flow_rate_read_count: usize,
    /// Final outdoor-air mass-flow clamp assignments.
    pub outdoor_air_mass_flow_clamp_assignment_count: usize,
    /// Latest transition snapshot.
    pub latest: Option<PurchasedAirCalcCoolingOaMaxFlowBodySnapshot>,
}

impl PurchasedAirCalcCoolingOaMaxFlowBodyRuntimeState {
    /// Creates bounded state for one selected system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            body_entry_count: 0,
            body_skip_count: 0,
            unit_off_skip_count: 0,
            non_cooling_skip_count: 0,
            active_guard_false_economizer_fallthrough_count: 0,
            outdoor_air_mass_flow_rate_read_count: 0,
            standard_air_density_read_count: 0,
            outdoor_air_volume_flow_calculation_count: 0,
            warning_counter_read_count: 0,
            outdoor_air_flow_max_cooling_output_error_count: 0,
            first_warning_branch_count: 0,
            warning_counter_increment_count: 0,
            first_warning_call_site_count: 0,
            maximum_cooling_air_volume_flow_rate_read_count: 0,
            continue_warning_call_site_count: 0,
            continue_warning_timestamp_call_site_count: 0,
            recurring_warning_branch_count: 0,
            recurring_warning_call_site_count: 0,
            characterized_recurring_warning_index_allocation_count: 0,
            characterized_recurring_warning_index_reuse_count: 0,
            characterized_recurring_warning_occurrence_count: 0,
            characterized_recurring_warning_index_allocated: false,
            outdoor_air_flow_max_cooling_output_index: 0,
            characterized_recurring_warning_report_maximum_m3_per_s: None,
            characterized_total_warning_error_increment_count: 0,
            maximum_cooling_air_mass_flow_rate_read_count: 0,
            outdoor_air_mass_flow_clamp_assignment_count: 0,
            latest: None,
        }
    }
}

/// Final selected-unit CP314 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingOaMaxFlowBodyLifecycleSummary {
    /// EnergyPlus source slice.
    pub source: &'static str,
    /// First executable statement deliberately excluded.
    pub first_excluded_source: &'static str,
    /// Characterized recurring-warning child source.
    pub recurring_warning_child_source: &'static str,
    /// Final bounded per-unit state.
    pub state: PurchasedAirCalcCoolingOaMaxFlowBodyRuntimeState,
}

/// Returns the bounded selected-unit CP314 lifecycle summary.
pub fn purchased_air_calc_cooling_oa_max_flow_body_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<
    PurchasedAirCalcCoolingOaMaxFlowBodyLifecycleSummary,
    PurchasedAirCalcCoolingOaMaxFlowBodyError,
> {
    let unit = runtime
        .units
        .get(&system)
        .ok_or(PurchasedAirCalcCoolingOaMaxFlowBodyError::UnknownSystem { system })?;
    Ok(PurchasedAirCalcCoolingOaMaxFlowBodyLifecycleSummary {
        source: PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_FIRST_EXCLUDED_SOURCE,
        recurring_warning_child_source:
            PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_RECURRING_WARNING_CHILD_SOURCE,
        state: unit.calc_cooling_oa_max_flow_body.clone(),
    })
}
