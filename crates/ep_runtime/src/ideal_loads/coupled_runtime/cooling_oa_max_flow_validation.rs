//! Release validation for the bounded cooling OA maximum-flow gate.

use ep_model::IdealLoadsLimit;

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_SOURCE,
    PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_SOURCE_ORDER,
    PurchasedAirCalcCoolingEntryGateLifecycleSummary,
    PurchasedAirCalcCoolingOaMaxFlowGateLifecycleSummary,
    PurchasedAirCalcCoolingOaMaxFlowGateSnapshot,
};

use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

pub(super) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let minimum_oa = output.calculation_minimum_outdoor_air;
    let predecessor = output.calculation_cooling_entry_gate;
    let gate = output.calculation_cooling_oa_max_flow_gate;
    let linked = predecessor.system == binding.ideal_loads_air_system
        && minimum_oa.system == predecessor.system
        && output.initialization.system == predecessor.system
        && predecessor.parent_call_ordinal == call_ordinal
        && minimum_oa.parent_call_ordinal == call_ordinal
        && predecessor.controlled_zone == binding.zone
        && minimum_oa.controlled_zone == binding.zone
        && output.initialization.controlled_zone == binding.zone
        && minimum_oa.unit_body_entered == predecessor.unit_body_entered
        && (!predecessor.cooling_body_entered || predecessor.unit_body_entered)
        && super::cooling_entry_validation::numerical_mode_matches_release(
            predecessor.unit_body_entered,
            predecessor.cooling_body_entered,
            output.coupling.purchased_air.calculation.mode,
        );
    let maximum = output
        .initialization
        .maximum_cooling_air_mass_flow_rate_kg_per_s;
    let expected = expected_snapshot(
        binding,
        call_ordinal,
        predecessor.unit_body_entered,
        predecessor.cooling_body_entered,
        maximum,
    );
    let flow_read = expected.outdoor_air_mass_flow_rate_read;
    linked
        && gate == expected
        && (!flow_read
            || (option_has_bits(minimum_oa.working_outdoor_air_mass_flow_rate_kg_per_s, 0.0)
                && maximum.is_finite()
                && maximum >= 0.0
                && option_has_bits(gate.outdoor_air_mass_flow_rate_kg_per_s, 0.0)
                && option_has_bits(gate.maximum_cooling_air_mass_flow_rate_kg_per_s, maximum)))
}

fn expected_snapshot(
    binding: &DirectZonePurchasedAirModelBinding<'_>,
    call_ordinal: usize,
    unit_body_entered: bool,
    cooling_body_entered: bool,
    maximum: f64,
) -> PurchasedAirCalcCoolingOaMaxFlowGateSnapshot {
    let limit = binding.system.cooling_limit;
    let first_match = limit == IdealLoadsLimit::LimitFlowRate;
    let second_evaluated = cooling_body_entered && !first_match;
    let second_match = limit == IdealLoadsLimit::LimitFlowRateAndCapacity;
    let flow_active = first_match || second_match;
    let flow_read = cooling_body_entered && flow_active;
    PurchasedAirCalcCoolingOaMaxFlowGateSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_FIRST_EXCLUDED_SOURCE,
        system: binding.ideal_loads_air_system,
        parent_call_ordinal: call_ordinal,
        source_order: PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_SOURCE_ORDER,
        controlled_zone: binding.zone,
        unit_body_entered,
        predecessor_cooling_body_entered: cooling_body_entered,
        unit_off_skipped: !unit_body_entered,
        non_cooling_skipped: unit_body_entered && !cooling_body_entered,
        cooling_limit_flow_rate_comparison_evaluated: cooling_body_entered,
        cooling_limit_flow_rate_read: cooling_body_entered,
        cooling_limit_flow_rate_value: cooling_body_entered.then_some(limit),
        cooling_limit_flow_rate_comparison_satisfied: cooling_body_entered.then_some(first_match),
        cooling_limit_flow_rate_and_capacity_comparison_evaluated: second_evaluated,
        cooling_limit_flow_rate_and_capacity_read: second_evaluated,
        cooling_limit_flow_rate_and_capacity_value: second_evaluated.then_some(limit),
        cooling_limit_flow_rate_and_capacity_comparison_satisfied: second_evaluated
            .then_some(second_match),
        cooling_flow_limit_active: cooling_body_entered.then_some(flow_active),
        outdoor_air_mass_flow_rate_read: flow_read,
        outdoor_air_mass_flow_rate_kg_per_s: flow_read.then_some(0.0),
        maximum_cooling_air_mass_flow_rate_read: flow_read,
        maximum_cooling_air_mass_flow_rate_kg_per_s: flow_read.then_some(maximum),
        strict_mass_flow_comparison_evaluated: flow_read,
        outdoor_air_mass_flow_above_maximum: flow_read.then_some(false),
        maximum_cooling_flow_body_entered: false,
    }
}

fn option_has_bits(value: Option<f64>, expected: f64) -> bool {
    value.is_some_and(|value| value.to_bits() == expected.to_bits())
}

pub(super) fn validate_lifecycle(
    lifecycle: &PurchasedAirCalcCoolingOaMaxFlowGateLifecycleSummary,
    predecessor_lifecycle: &PurchasedAirCalcCoolingEntryGateLifecycleSummary,
    timestep_count: usize,
    numerical_cooling_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    let s = &lifecycle.state;
    let p = &predecessor_lifecycle.state;
    let source = s.source_execution_count;
    let first = s.cooling_limit_flow_rate_match_count;
    let second = s.cooling_limit_flow_rate_and_capacity_match_count;
    let body = s.maximum_cooling_flow_body_entry_count;
    let skip_count = checked_add(
        s.unit_off_skip_count,
        s.non_cooling_skip_count,
        "skip_overflow",
        timestep_count,
    )?;
    let transition_partition =
        checked_add(source, skip_count, "transition_overflow", timestep_count)?;
    let second_count = checked_sub(source, first, "second_selector_underflow", source)?;
    let selected_count = checked_add(first, second, "selected_flow_overflow", source)?;
    let active_partition =
        checked_add(body, s.active_fallthrough_count, "active_overflow", source)?;
    let limit = binding.system.cooling_limit;
    let first_matches = usize::from(limit == IdealLoadsLimit::LimitFlowRate) * source;
    let second_matches = usize::from(limit == IdealLoadsLimit::LimitFlowRateAndCapacity) * source;
    macro_rules! count {
        ($field:ident, $expected:expr) => {
            ensure_count(s.$field, $expected, stringify!($field))?
        };
        ($actual:expr, $expected:expr, $field:literal) => {
            ensure_count($actual, $expected, $field)?
        };
    }
    count!(transition_count, timestep_count);
    count!(
        s.transition_count,
        p.transition_count,
        "predecessor_transition_count"
    );
    count!(source_execution_count, p.cooling_body_entry_count);
    count!(
        s.source_execution_count,
        numerical_cooling_count,
        "numerical_cooling_count"
    );
    count!(unit_off_skip_count, p.unit_off_skip_count);
    count!(non_cooling_skip_count, p.active_fallthrough_count);
    count!(
        cooling_limit_flow_rate_comparison_count,
        s.source_execution_count
    );
    count!(cooling_limit_flow_rate_match_count, first_matches);
    count!(
        cooling_limit_flow_rate_and_capacity_comparison_count,
        second_count
    );
    count!(
        cooling_limit_flow_rate_and_capacity_match_count,
        second_matches
    );
    count!(outdoor_air_mass_flow_rate_read_count, selected_count);
    count!(
        maximum_cooling_air_mass_flow_rate_read_count,
        selected_count
    );
    count!(strict_mass_flow_comparison_count, selected_count);
    count!(strict_mass_flow_comparison_satisfied_count, 0);
    count!(maximum_cooling_flow_body_entry_count, 0);
    count!(active_fallthrough_count, s.source_execution_count);
    count!(transition_partition, timestep_count, "transition_partition");
    count!(
        active_partition,
        s.source_execution_count,
        "active_partition"
    );
    let latest = s
        .latest
        .as_ref()
        .ok_or_else(|| violation("latest_snapshot_present", 1, 0))?;
    if lifecycle.source != PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_FIRST_EXCLUDED_SOURCE
        || s.system != binding.ideal_loads_air_system
        || latest != &latest_output.calculation_cooling_oa_max_flow_gate
        || !snapshot_matches_release(latest_output, timestep_count, binding)
    {
        return Err(violation("latest_release_snapshot_ready", 1, 0));
    }
    Ok(())
}

pub(super) fn checked_add(
    left: usize,
    right: usize,
    field: &'static str,
    expected: usize,
) -> Result<usize, Error> {
    left.checked_add(right)
        .ok_or_else(|| violation(field, expected, usize::MAX))
}

pub(super) fn checked_sub(
    left: usize,
    right: usize,
    field: &'static str,
    expected: usize,
) -> Result<usize, Error> {
    left.checked_sub(right)
        .ok_or_else(|| violation(field, expected, usize::MAX))
}

fn ensure_count(actual: usize, expected: usize, field: &'static str) -> Result<(), Error> {
    if actual == expected {
        Ok(())
    } else {
        Err(violation(field, expected, actual))
    }
}

fn violation(field: &'static str, expected: usize, actual: usize) -> Error {
    Error::CalcCoolingOaMaxFlowGateLifecycleInvariant {
        field,
        expected,
        actual,
    }
}
