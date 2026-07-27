//! Release validation for the bounded cooling economizer guard.

use ep_model::OutdoorAirEconomizerType;

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    IdealLoadsSensibleMode, PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingEconomizerGuardLifecycleSummary,
    PurchasedAirCalcCoolingEconomizerGuardSnapshot,
    PurchasedAirCalcCoolingOaMaxFlowBodyLifecycleSummary,
    PurchasedAirCalcCoolingOaMaxFlowBodySnapshot,
};

use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

pub(super) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output.calculation_cooling_oa_max_flow_body;
    let guard = output.calculation_cooling_economizer_guard;
    let expected = expected_snapshot(predecessor, call_ordinal, binding);
    let numerical_cooling =
        output.coupling.purchased_air.calculation.mode == IdealLoadsSensibleMode::Cooling;

    predecessor.system == binding.ideal_loads_air_system
        && predecessor.parent_call_ordinal == call_ordinal
        && predecessor.controlled_zone == binding.zone
        && output.initialization.system == predecessor.system
        && output.initialization.controlled_zone == predecessor.controlled_zone
        && guard.economizer_guard_evaluated == numerical_cooling
        && guard == expected
}

fn expected_snapshot(
    predecessor: PurchasedAirCalcCoolingOaMaxFlowBodySnapshot,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> PurchasedAirCalcCoolingEconomizerGuardSnapshot {
    let guard_evaluated = predecessor.active_guard_false_economizer_fallthrough;
    PurchasedAirCalcCoolingEconomizerGuardSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_FIRST_EXCLUDED_SOURCE,
        system: binding.ideal_loads_air_system,
        parent_call_ordinal: call_ordinal,
        source_order: PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_SOURCE_ORDER,
        controlled_zone: binding.zone,
        unit_body_entered: predecessor.unit_body_entered,
        predecessor_cooling_body_entered: predecessor.predecessor_cooling_body_entered,
        predecessor_maximum_cooling_flow_body_entered: predecessor
            .predecessor_maximum_cooling_flow_body_entered,
        predecessor_active_guard_false_economizer_fallthrough: predecessor
            .active_guard_false_economizer_fallthrough,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        maximum_cooling_flow_body_sibling_skipped: predecessor
            .predecessor_maximum_cooling_flow_body_entered,
        economizer_guard_evaluated: guard_evaluated,
        economizer_type_read: guard_evaluated,
        economizer_type: guard_evaluated.then_some(OutdoorAirEconomizerType::NoEconomizer),
        no_economizer_comparison_evaluated: guard_evaluated,
        economizer_not_no_economizer: guard_evaluated.then_some(false),
        economizer_body_entered: false,
        no_economizer_fallthrough: guard_evaluated,
    }
}

pub(super) fn validate_lifecycle(
    lifecycle: &PurchasedAirCalcCoolingEconomizerGuardLifecycleSummary,
    predecessor_lifecycle: &PurchasedAirCalcCoolingOaMaxFlowBodyLifecycleSummary,
    timestep_count: usize,
    numerical_cooling_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    let state = &lifecycle.state;
    let predecessor = &predecessor_lifecycle.state;
    let skip_partition = checked_add(
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        "skip_partition_overflow",
        timestep_count,
    )
    .and_then(|partial| {
        checked_add(
            partial,
            state.maximum_cooling_flow_body_sibling_skip_count,
            "skip_partition_overflow",
            timestep_count,
        )
    })?;
    let transition_partition = checked_add(
        state.guard_evaluation_count,
        skip_partition,
        "transition_partition_overflow",
        timestep_count,
    )?;
    let guard_result_partition = checked_add(
        state.economizer_body_entry_count,
        state.no_economizer_fallthrough_count,
        "guard_result_partition_overflow",
        state.guard_evaluation_count,
    )?;
    let expected_skip_partition = checked_sub(
        timestep_count,
        numerical_cooling_count,
        "skip_partition_underflow",
        timestep_count,
    )?;

    macro_rules! count {
        ($field:ident, $expected:expr) => {
            ensure_count(state.$field, $expected, stringify!($field))?
        };
        ($actual:expr, $expected:expr, $field:literal) => {
            ensure_count($actual, $expected, $field)?
        };
    }

    count!(transition_count, timestep_count);
    count!(
        state.transition_count,
        predecessor.transition_count,
        "predecessor_transition_count"
    );
    count!(
        guard_evaluation_count,
        predecessor.active_guard_false_economizer_fallthrough_count
    );
    count!(
        state.guard_evaluation_count,
        numerical_cooling_count,
        "numerical_cooling_count"
    );
    count!(unit_off_skip_count, predecessor.unit_off_skip_count);
    count!(non_cooling_skip_count, predecessor.non_cooling_skip_count);
    count!(
        maximum_cooling_flow_body_sibling_skip_count,
        predecessor.body_entry_count
    );
    count!(maximum_cooling_flow_body_sibling_skip_count, 0);
    count!(economizer_type_read_count, state.guard_evaluation_count);
    count!(no_economizer_comparison_count, state.guard_evaluation_count);
    count!(economizer_body_entry_count, 0);
    count!(
        no_economizer_fallthrough_count,
        state.guard_evaluation_count
    );
    count!(skip_partition, expected_skip_partition, "skip_partition");
    count!(
        transition_partition,
        state.transition_count,
        "transition_partition"
    );
    count!(
        guard_result_partition,
        state.guard_evaluation_count,
        "guard_result_partition"
    );

    let latest = state
        .latest
        .as_ref()
        .ok_or_else(|| violation("latest_release_snapshot_ready", 1, 0))?;
    if lifecycle.source != PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_FIRST_EXCLUDED_SOURCE
        || state.system != binding.ideal_loads_air_system
        || binding.system.outdoor_air_economizer_type != OutdoorAirEconomizerType::NoEconomizer
        || latest != &latest_output.calculation_cooling_economizer_guard
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

fn checked_sub(
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
    Error::CalcCoolingEconomizerGuardLifecycleInvariant {
        field,
        expected,
        actual,
    }
}
