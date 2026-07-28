use ep_model::{AutosizeOrNumber, IdealLoadsAirSystem};

use super::runtime_validation::completed_supply_maximum_state_is_consistent;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot, PurchasedAirRuntimeState,
    PurchasedAirUnitRuntimeState,
};

pub(super) fn completed_direct_prefix_through_supply_maximum_is_consistent(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    maximum: PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot,
    maximum_witness: Option<PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot>,
) -> bool {
    let Some(guard) = unit.calc_cooling_economizer_guard.latest else {
        return false;
    };
    let Some(condition) = unit.calc_cooling_economizer_condition.latest else {
        return false;
    };
    let Some(body) = unit.calc_cooling_economizer_body.latest else {
        return false;
    };
    let Some(sensible) = unit.calc_cooling_sensible_flow.latest else {
        return false;
    };
    let Some(dehumidification) = unit.calc_cooling_dehumidification_flow.latest else {
        return false;
    };
    let Some(humidification) = unit.calc_cooling_humidification_flow.latest else {
        return false;
    };
    let Some(reset) = unit.calc_cooling_capacity_zero_flow_reset.latest else {
        return false;
    };
    let Some(minimum_oa) = unit.calc_minimum_oa_prefix.latest else {
        return false;
    };

    crate::ideal_loads::calc::cooling_economizer_condition::
        completed_direct_prefix_through_economizer_guard_is_consistent(unit, system, guard)
        && crate::ideal_loads::calc::cooling_economizer_condition::
            completed_direct_economizer_condition_is_consistent(
                unit,
                condition,
                runtime.cooling_economizer_condition_latest_witness(system.id),
            )
        && crate::ideal_loads::calc::cooling_economizer_body::
            completed_direct_cooling_economizer_body_is_consistent(
                unit,
                condition,
                body,
                runtime.cooling_economizer_body_latest_witness(system.id),
            )
        && crate::ideal_loads::calc::cooling_sensible_flow::
            completed_direct_cooling_sensible_flow_is_consistent(
                unit,
                body,
                sensible,
                runtime.cooling_sensible_flow_latest_witness(system.id),
            )
        && crate::ideal_loads::calc::cooling_dehumidification_flow::
            completed_direct_cooling_dehumidification_flow_is_consistent(
                unit,
                sensible,
                dehumidification,
                runtime.cooling_dehumidification_flow_latest_witness(system.id),
            )
        && crate::ideal_loads::calc::cooling_humidification_flow::
            completed_direct_cooling_humidification_flow_is_consistent(
                unit,
                dehumidification,
                humidification,
                runtime.cooling_humidification_flow_latest_witness(system.id),
            )
        && completed_capacity_zero_reset_is_consistent(
            unit,
            system,
            reset,
            runtime.cooling_capacity_zero_flow_reset_latest_witness(system.id),
        )
        && maximum_links_to_predecessors(maximum, reset, minimum_oa)
        && completed_supply_maximum_state_is_consistent(unit, maximum, maximum_witness)
}

fn completed_capacity_zero_reset_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    reset: PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot,
    witness: Option<PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot>,
) -> bool {
    let state = &unit.calc_cooling_capacity_zero_flow_reset;
    let cooling_count = state.cooling_body_entry_count;
    let capacity_count =
        usize::from(system.cooling_limit == ep_model::IdealLoadsLimit::LimitCapacity)
            * cooling_count;
    let combined_count =
        usize::from(system.cooling_limit == ep_model::IdealLoadsLimit::LimitFlowRateAndCapacity)
            * cooling_count;
    let selected_count = capacity_count + combined_count;
    let selected_capacity_is_zero = selected_count > 0
        && matches!(
            system.maximum_total_cooling_capacity_w,
            Some(AutosizeOrNumber::Value(value)) if value == 0.0
        );
    let zero_count = usize::from(selected_capacity_is_zero) * cooling_count;

    state.latest == Some(reset)
        && witness == Some(reset)
        && crate::ideal_loads::calc::cooling_capacity_zero_flow_reset::
            cooling_capacity_zero_flow_reset_snapshot_is_exact_direct_release(reset)
        && partition(
            state.transition_count,
            state.unit_off_skip_count,
            state.non_cooling_skip_count,
            cooling_count,
        )
        && state.transition_count == unit.calc_entry.call_count
        && state.first_cooling_limit_read_count == cooling_count
        && state.cooling_limit_capacity_count == capacity_count
        && state.second_cooling_limit_read_count == cooling_count - capacity_count
        && state.cooling_limit_flow_rate_and_capacity_count == combined_count
        && state.cooling_limit_rejected_count == cooling_count - selected_count
        && state.maximum_total_cooling_capacity_read_count == selected_count
        && state.maximum_total_cooling_capacity_comparison_count == selected_count
        && state.maximum_total_cooling_capacity_zero_count == zero_count
        && state.maximum_total_cooling_capacity_nonzero_count == selected_count - zero_count
        && state.zero_cooling_capacity_body_entry_count == zero_count
        && state.supply_mass_flow_rate_for_cool_zero_assignment_count == zero_count
        && state.supply_mass_flow_rate_for_dehumidification_zero_assignment_count == zero_count
        && state.supply_mass_flow_rate_for_humidification_zero_assignment_count == zero_count
}

fn maximum_links_to_predecessors(
    maximum: PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot,
    reset: PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot,
    minimum_oa: crate::ideal_loads::PurchasedAirCalcMinimumOaPrefixSnapshot,
) -> bool {
    maximum.system == reset.system
        && maximum.system == minimum_oa.system
        && maximum.parent_call_ordinal == reset.parent_call_ordinal
        && maximum.parent_call_ordinal == minimum_oa.parent_call_ordinal
        && maximum.controlled_zone == reset.controlled_zone
        && maximum.controlled_zone == minimum_oa.controlled_zone
        && maximum.unit_body_entered == reset.unit_body_entered
        && maximum.predecessor_cooling_body_entered == reset.cooling_body_entered
        && maximum.unit_off_skipped == reset.unit_off_skipped
        && maximum.non_cooling_skipped == reset.non_cooling_skipped
        && maximum.cooling_body_entered == reset.cooling_body_entered
        && if maximum.cooling_body_entered {
            has_bits(
                maximum.outdoor_air_mass_flow_rate_kg_per_s,
                minimum_oa.working_outdoor_air_mass_flow_rate_kg_per_s,
            ) && has_bits(
                maximum.supply_mass_flow_rate_for_cool_kg_per_s,
                reset.resulting_supply_mass_flow_rate_for_cool_kg_per_s,
            ) && has_bits(
                maximum.supply_mass_flow_rate_for_dehumidification_kg_per_s,
                reset.resulting_supply_mass_flow_rate_for_dehumidification_kg_per_s,
            ) && has_bits(
                maximum.supply_mass_flow_rate_for_humidification_kg_per_s,
                reset.resulting_supply_mass_flow_rate_for_humidification_kg_per_s,
            )
        } else {
            maximum.outdoor_air_mass_flow_rate_kg_per_s.is_none()
                && maximum.supply_mass_flow_rate_for_cool_kg_per_s.is_none()
                && maximum
                    .supply_mass_flow_rate_for_dehumidification_kg_per_s
                    .is_none()
                && maximum
                    .supply_mass_flow_rate_for_humidification_kg_per_s
                    .is_none()
        }
}

fn has_bits(left: Option<f64>, right: Option<f64>) -> bool {
    left.zip(right)
        .is_some_and(|(left, right)| left.to_bits() == right.to_bits())
}

fn partition(transitions: usize, unit_off: usize, non_cooling: usize, cooling: usize) -> bool {
    unit_off
        .checked_add(non_cooling)
        .and_then(|count| count.checked_add(cooling))
        == Some(transitions)
}
