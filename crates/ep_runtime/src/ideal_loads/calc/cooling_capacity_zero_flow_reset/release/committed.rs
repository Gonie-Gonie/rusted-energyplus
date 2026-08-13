//! Sealed CP321 maximum-total-cooling-capacity owner.

use ep_model::{AutosizeOrNumber, IdealLoadsLimit};

use super::super::{
    PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_FIRST_EXCLUDED_SOURCE as EXCLUDED,
    PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE as SOURCE,
    PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE_ORDER as ORDER,
};
use crate::ideal_loads::calc::cooling_capacity_zero_flow_reset::PurchasedAirCalcCoolingCapacityZeroFlowResetRetainedRoute as Route;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingCapacityZeroFlowResetRuntimeState as State,
    PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot as Snapshot,
    PurchasedAirUnitRuntimeState,
};

pub(in crate::ideal_loads::calc) fn cooling_capacity_zero_flow_reset_committed_latest_maximum_total_cooling_capacity(
    unit: &PurchasedAirUnitRuntimeState,
    witness: Snapshot,
) -> Option<f64> {
    let state = &unit.calc_cooling_capacity_zero_flow_reset;
    let latest = state.latest?;
    let capacity = latest.maximum_total_cooling_capacity_w?;
    let sized_capacity = match unit
        .sized_limits?
        .maximum_total_cooling_capacity_w?
    {
        AutosizeOrNumber::Value(value) => value,
        AutosizeOrNumber::Autosize => return None,
    };
    (state.system == unit.system
        && state.transition_count == unit.init_call_count
        && state.transition_count > 0
        && state.latest_transition_ordinal == Some(state.transition_count)
        && latest.parent_call_ordinal == state.transition_count
        && latest.system == unit.system
        && unit.controlled_zone == Some(latest.controlled_zone)
        && state.latest_route == Some(Route::MaximumCoolingCapacityNonZero)
        && committed_snapshot_shape(latest)
        && snapshots_match_bits(latest, witness)
        && counters_are_consistent(state)
        && latest.maximum_total_cooling_capacity_read
        && latest.maximum_total_cooling_capacity_comparison_evaluated
        && latest.maximum_total_cooling_capacity_equal_to_zero == Some(false)
        && capacity.is_finite()
        && capacity > 0.0
        && capacity.to_bits() == sized_capacity.to_bits())
        .then_some(capacity)
}

fn committed_snapshot_shape(snapshot: Snapshot) -> bool {
    let Some(limit) = snapshot.first_cooling_limit else {
        return false;
    };
    let limit_capacity = limit == IdealLoadsLimit::LimitCapacity;
    let limit_flow_capacity = limit == IdealLoadsLimit::LimitFlowRateAndCapacity;
    snapshot.source == SOURCE
        && snapshot.first_excluded_source == EXCLUDED
        && snapshot.source_order == ORDER
        && snapshot.unit_body_entered
        && snapshot.predecessor_cooling_body_entered
        && !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && snapshot.cooling_body_entered
        && snapshot.first_cooling_limit_read
        && snapshot.cooling_limit_capacity == Some(limit_capacity)
        && snapshot.second_cooling_limit_read != limit_capacity
        && snapshot.second_cooling_limit == (!limit_capacity).then_some(limit)
        && snapshot.cooling_limit_flow_rate_and_capacity
            == (!limit_capacity).then_some(limit_flow_capacity)
        && snapshot.cooling_limit_condition_satisfied == Some(true)
        && snapshot.maximum_total_cooling_capacity_read
        && snapshot.maximum_total_cooling_capacity_comparison_evaluated
        && snapshot.maximum_total_cooling_capacity_equal_to_zero == Some(false)
        && !snapshot.zero_cooling_capacity_body_entered
        && candidates_are_preserved(snapshot)
        && assignments_are_absent(snapshot)
}

fn candidates_are_preserved(snapshot: Snapshot) -> bool {
    same_bits(
        snapshot.predecessor_supply_mass_flow_rate_for_cool_kg_per_s,
        snapshot.resulting_supply_mass_flow_rate_for_cool_kg_per_s,
    ) && same_bits(
        snapshot.predecessor_supply_mass_flow_rate_for_dehumidification_kg_per_s,
        snapshot.resulting_supply_mass_flow_rate_for_dehumidification_kg_per_s,
    ) && same_bits(
        snapshot.predecessor_supply_mass_flow_rate_for_humidification_kg_per_s,
        snapshot.resulting_supply_mass_flow_rate_for_humidification_kg_per_s,
    )
}

fn assignments_are_absent(snapshot: Snapshot) -> bool {
    !snapshot.supply_mass_flow_rate_for_cool_zero_assigned
        && snapshot.assigned_supply_mass_flow_rate_for_cool_kg_per_s.is_none()
        && !snapshot.supply_mass_flow_rate_for_dehumidification_zero_assigned
        && snapshot
            .assigned_supply_mass_flow_rate_for_dehumidification_kg_per_s
            .is_none()
        && !snapshot.supply_mass_flow_rate_for_humidification_zero_assigned
        && snapshot
            .assigned_supply_mass_flow_rate_for_humidification_kg_per_s
            .is_none()
}

fn same_bits(left: Option<f64>, right: Option<f64>) -> bool {
    left.zip(right)
        .is_some_and(|(left, right)| left.to_bits() == right.to_bits())
}

fn counters_are_consistent(state: &State) -> bool {
    let route_partition = state
        .unit_off_skip_count
        .checked_add(state.non_cooling_skip_count)
        .and_then(|count| count.checked_add(state.cooling_body_entry_count));
    let limit_partition = state
        .cooling_limit_capacity_count
        .checked_add(state.cooling_limit_flow_rate_and_capacity_count)
        .and_then(|count| count.checked_add(state.cooling_limit_rejected_count));
    let capacity_partition = state
        .maximum_total_cooling_capacity_zero_count
        .checked_add(state.maximum_total_cooling_capacity_nonzero_count);
    let first_read_partition = state
        .second_cooling_limit_read_count
        .checked_add(state.cooling_limit_capacity_count);
    let selected_capacity = state
        .cooling_limit_capacity_count
        .checked_add(state.cooling_limit_flow_rate_and_capacity_count);
    route_partition == Some(state.transition_count)
        && state.first_cooling_limit_read_count == state.cooling_body_entry_count
        && first_read_partition == Some(state.cooling_body_entry_count)
        && limit_partition == Some(state.cooling_body_entry_count)
        && selected_capacity == Some(state.maximum_total_cooling_capacity_read_count)
        && state.maximum_total_cooling_capacity_comparison_count
            == state.maximum_total_cooling_capacity_read_count
        && capacity_partition == Some(state.maximum_total_cooling_capacity_read_count)
        && state.zero_cooling_capacity_body_entry_count
            == state.maximum_total_cooling_capacity_zero_count
        && state.supply_mass_flow_rate_for_cool_zero_assignment_count
            == state.zero_cooling_capacity_body_entry_count
        && state.supply_mass_flow_rate_for_dehumidification_zero_assignment_count
            == state.zero_cooling_capacity_body_entry_count
        && state.supply_mass_flow_rate_for_humidification_zero_assignment_count
            == state.zero_cooling_capacity_body_entry_count
}

fn snapshots_match_bits(mut left: Snapshot, mut right: Snapshot) -> bool {
    macro_rules! clear {
        ($field:ident) => {{
            let matches = option_bits(left.$field, right.$field);
            left.$field = None;
            right.$field = None;
            matches
        }};
    }
    let floats = clear!(maximum_total_cooling_capacity_w)
        && clear!(predecessor_supply_mass_flow_rate_for_cool_kg_per_s)
        && clear!(predecessor_supply_mass_flow_rate_for_dehumidification_kg_per_s)
        && clear!(predecessor_supply_mass_flow_rate_for_humidification_kg_per_s)
        && clear!(assigned_supply_mass_flow_rate_for_cool_kg_per_s)
        && clear!(assigned_supply_mass_flow_rate_for_dehumidification_kg_per_s)
        && clear!(assigned_supply_mass_flow_rate_for_humidification_kg_per_s)
        && clear!(resulting_supply_mass_flow_rate_for_cool_kg_per_s)
        && clear!(resulting_supply_mass_flow_rate_for_dehumidification_kg_per_s)
        && clear!(resulting_supply_mass_flow_rate_for_humidification_kg_per_s);
    floats && left == right
}

fn option_bits(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
