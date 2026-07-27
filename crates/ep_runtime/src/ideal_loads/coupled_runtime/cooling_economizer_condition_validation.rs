//! Release validation for the bounded cooling economizer inner condition.

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_SOURCE,
    PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_SOURCE_ORDER,
    PurchasedAirCalcCoolingEconomizerConditionLifecycleSummary,
    PurchasedAirCalcCoolingEconomizerConditionSnapshot,
    PurchasedAirCalcCoolingEconomizerGuardLifecycleSummary,
    PurchasedAirCalcCoolingEconomizerGuardSnapshot,
};

use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

pub(super) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output.calculation_cooling_economizer_guard;
    let condition = output.calculation_cooling_economizer_condition;
    let expected = expected_snapshot(predecessor, call_ordinal, binding);

    predecessor.system == binding.ideal_loads_air_system
        && predecessor.parent_call_ordinal == call_ordinal
        && predecessor.controlled_zone == binding.zone
        && output.initialization.system == predecessor.system
        && output.initialization.controlled_zone == predecessor.controlled_zone
        && !condition.economizer_condition_evaluated
        && condition == expected
}

fn expected_snapshot(
    predecessor: PurchasedAirCalcCoolingEconomizerGuardSnapshot,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> PurchasedAirCalcCoolingEconomizerConditionSnapshot {
    PurchasedAirCalcCoolingEconomizerConditionSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_FIRST_EXCLUDED_SOURCE,
        system: binding.ideal_loads_air_system,
        parent_call_ordinal: call_ordinal,
        source_order: PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_SOURCE_ORDER,
        controlled_zone: binding.zone,
        unit_body_entered: predecessor.unit_body_entered,
        predecessor_cooling_body_entered: predecessor.predecessor_cooling_body_entered,
        predecessor_maximum_cooling_flow_body_entered: predecessor
            .predecessor_maximum_cooling_flow_body_entered,
        predecessor_active_guard_false_economizer_fallthrough: predecessor
            .predecessor_active_guard_false_economizer_fallthrough,
        predecessor_economizer_guard_evaluated: predecessor.economizer_guard_evaluated,
        predecessor_economizer_body_entered: predecessor.economizer_body_entered,
        predecessor_no_economizer_fallthrough: predecessor.no_economizer_fallthrough,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        maximum_cooling_flow_body_sibling_skipped: predecessor
            .maximum_cooling_flow_body_sibling_skipped,
        no_economizer_outer_guard_fallthrough_skipped: predecessor.no_economizer_fallthrough,
        economizer_condition_evaluated: false,
        differential_dry_bulb_economizer_type_read: false,
        differential_dry_bulb_economizer_type: None,
        differential_dry_bulb_selector_comparison_evaluated: false,
        differential_dry_bulb_selector_matched: None,
        outdoor_air_temperature_read: false,
        outdoor_air_temperature_c: None,
        recirculation_air_temperature_read: false,
        recirculation_air_temperature_c: None,
        dry_bulb_temperature_comparison_evaluated: false,
        outdoor_air_temperature_below_recirculation_temperature: None,
        differential_enthalpy_economizer_type_read: false,
        differential_enthalpy_economizer_type: None,
        differential_enthalpy_selector_comparison_evaluated: false,
        differential_enthalpy_selector_matched: None,
        outdoor_air_enthalpy_read: false,
        outdoor_air_enthalpy_j_per_kg: None,
        recirculation_air_enthalpy_read: false,
        recirculation_air_enthalpy_j_per_kg: None,
        enthalpy_comparison_evaluated: false,
        outdoor_air_enthalpy_below_recirculation_enthalpy: None,
        economizer_condition_satisfied: None,
        economizer_calculation_body_entered: false,
        economizer_condition_fallthrough: false,
    }
}

pub(super) fn validate_lifecycle(
    lifecycle: &PurchasedAirCalcCoolingEconomizerConditionLifecycleSummary,
    predecessor_lifecycle: &PurchasedAirCalcCoolingEconomizerGuardLifecycleSummary,
    timestep_count: usize,
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
    })
    .and_then(|partial| {
        checked_add(
            partial,
            state.no_economizer_outer_guard_fallthrough_skip_count,
            "skip_partition_overflow",
            timestep_count,
        )
    })?;
    let transition_partition = checked_add(
        state.condition_evaluation_count,
        skip_partition,
        "transition_partition_overflow",
        timestep_count,
    )?;
    let condition_result_partition = checked_add(
        state.economizer_calculation_body_entry_count,
        state.economizer_condition_fallthrough_count,
        "condition_result_partition_overflow",
        state.condition_evaluation_count,
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
        condition_evaluation_count,
        predecessor.economizer_body_entry_count
    );
    count!(condition_evaluation_count, 0);
    count!(unit_off_skip_count, predecessor.unit_off_skip_count);
    count!(non_cooling_skip_count, predecessor.non_cooling_skip_count);
    count!(
        maximum_cooling_flow_body_sibling_skip_count,
        predecessor.maximum_cooling_flow_body_sibling_skip_count
    );
    count!(maximum_cooling_flow_body_sibling_skip_count, 0);
    count!(
        no_economizer_outer_guard_fallthrough_skip_count,
        predecessor.no_economizer_fallthrough_count
    );
    count!(differential_dry_bulb_economizer_type_read_count, 0);
    count!(differential_dry_bulb_selector_comparison_count, 0);
    count!(differential_dry_bulb_selector_match_count, 0);
    count!(outdoor_air_temperature_read_count, 0);
    count!(recirculation_air_temperature_read_count, 0);
    count!(dry_bulb_temperature_comparison_count, 0);
    count!(dry_bulb_temperature_comparison_satisfied_count, 0);
    count!(differential_enthalpy_economizer_type_read_count, 0);
    count!(differential_enthalpy_selector_comparison_count, 0);
    count!(differential_enthalpy_selector_match_count, 0);
    count!(outdoor_air_enthalpy_read_count, 0);
    count!(recirculation_air_enthalpy_read_count, 0);
    count!(enthalpy_comparison_count, 0);
    count!(enthalpy_comparison_satisfied_count, 0);
    count!(economizer_calculation_body_entry_count, 0);
    count!(economizer_condition_fallthrough_count, 0);
    count!(skip_partition, state.transition_count, "skip_partition");
    count!(
        transition_partition,
        state.transition_count,
        "transition_partition"
    );
    count!(
        condition_result_partition,
        state.condition_evaluation_count,
        "condition_result_partition"
    );

    let latest = state
        .latest
        .as_ref()
        .ok_or_else(|| violation("latest_release_snapshot_ready", 1, 0))?;
    if lifecycle.source != PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_ECONOMIZER_CONDITION_FIRST_EXCLUDED_SOURCE
        || state.system != binding.ideal_loads_air_system
        || latest != &latest_output.calculation_cooling_economizer_condition
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

fn ensure_count(actual: usize, expected: usize, field: &'static str) -> Result<(), Error> {
    if actual == expected {
        Ok(())
    } else {
        Err(violation(field, expected, actual))
    }
}

fn violation(field: &'static str, expected: usize, actual: usize) -> Error {
    Error::CalcCoolingEconomizerConditionLifecycleInvariant {
        field,
        expected,
        actual,
    }
}
