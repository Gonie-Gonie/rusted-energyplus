//! Exact direct-lane shape checks for one CP321 source-site snapshot.

use ep_model::IdealLoadsLimit;
use ep_runtime::PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot;

pub(in crate::pipeline) fn snapshot_shape(
    snapshot: &PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot,
) -> bool {
    if !snapshot.cooling_body_entered {
        return usize::from(snapshot.unit_off_skipped) + usize::from(snapshot.non_cooling_skipped)
            == 1
            && skipped_source_shape(snapshot);
    }
    if snapshot.unit_off_skipped
        || snapshot.non_cooling_skipped
        || !snapshot.first_cooling_limit_read
        || snapshot.first_cooling_limit.is_none()
        || snapshot.cooling_limit_capacity.is_none()
        || !candidate_inputs_present(snapshot)
    {
        return false;
    }

    let Some(limit) = snapshot.first_cooling_limit else {
        return false;
    };
    let first_matched = limit == IdealLoadsLimit::LimitCapacity;
    let second_executed = !first_matched;
    let second_matched = limit == IdealLoadsLimit::LimitFlowRateAndCapacity;
    let selected = first_matched || second_matched;
    if snapshot.cooling_limit_capacity != Some(first_matched)
        || snapshot.second_cooling_limit_read != second_executed
        || snapshot.second_cooling_limit != second_executed.then_some(limit)
        || snapshot.cooling_limit_flow_rate_and_capacity
            != second_executed.then_some(second_matched)
        || snapshot.cooling_limit_condition_satisfied != Some(selected)
        || snapshot.maximum_total_cooling_capacity_read != selected
        || snapshot.maximum_total_cooling_capacity_comparison_evaluated != selected
    {
        return false;
    }

    if !selected {
        return snapshot.maximum_total_cooling_capacity_w.is_none()
            && snapshot
                .maximum_total_cooling_capacity_equal_to_zero
                .is_none()
            && !snapshot.zero_cooling_capacity_body_entered
            && assignments_and_results_match(snapshot, false);
    }

    let Some(capacity) = snapshot.maximum_total_cooling_capacity_w else {
        return false;
    };
    if !capacity.is_finite() || capacity < 0.0 {
        return false;
    }
    let is_zero = capacity == 0.0;
    snapshot.maximum_total_cooling_capacity_equal_to_zero == Some(is_zero)
        && snapshot.zero_cooling_capacity_body_entered == is_zero
        && assignments_and_results_match(snapshot, is_zero)
}

fn skipped_source_shape(snapshot: &PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot) -> bool {
    !snapshot.first_cooling_limit_read
        && snapshot.first_cooling_limit.is_none()
        && snapshot.cooling_limit_capacity.is_none()
        && !snapshot.second_cooling_limit_read
        && snapshot.second_cooling_limit.is_none()
        && snapshot.cooling_limit_flow_rate_and_capacity.is_none()
        && snapshot.cooling_limit_condition_satisfied.is_none()
        && !snapshot.maximum_total_cooling_capacity_read
        && snapshot.maximum_total_cooling_capacity_w.is_none()
        && !snapshot.maximum_total_cooling_capacity_comparison_evaluated
        && snapshot
            .maximum_total_cooling_capacity_equal_to_zero
            .is_none()
        && !snapshot.zero_cooling_capacity_body_entered
        && candidate_inputs_absent(snapshot)
        && assignments_absent(snapshot)
        && results_absent(snapshot)
}

fn candidate_inputs_present(
    snapshot: &PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot,
) -> bool {
    snapshot
        .predecessor_supply_mass_flow_rate_for_cool_kg_per_s
        .is_some()
        && snapshot
            .predecessor_supply_mass_flow_rate_for_dehumidification_kg_per_s
            .is_some()
        && snapshot
            .predecessor_supply_mass_flow_rate_for_humidification_kg_per_s
            .is_some()
}

fn candidate_inputs_absent(
    snapshot: &PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot,
) -> bool {
    snapshot
        .predecessor_supply_mass_flow_rate_for_cool_kg_per_s
        .is_none()
        && snapshot
            .predecessor_supply_mass_flow_rate_for_dehumidification_kg_per_s
            .is_none()
        && snapshot
            .predecessor_supply_mass_flow_rate_for_humidification_kg_per_s
            .is_none()
}

fn assignments_and_results_match(
    snapshot: &PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot,
    body_entered: bool,
) -> bool {
    if body_entered {
        snapshot.supply_mass_flow_rate_for_cool_zero_assigned
            && snapshot.supply_mass_flow_rate_for_dehumidification_zero_assigned
            && snapshot.supply_mass_flow_rate_for_humidification_zero_assigned
            && same_option(
                snapshot.assigned_supply_mass_flow_rate_for_cool_kg_per_s,
                Some(0.0),
            )
            && same_option(
                snapshot.assigned_supply_mass_flow_rate_for_dehumidification_kg_per_s,
                Some(0.0),
            )
            && same_option(
                snapshot.assigned_supply_mass_flow_rate_for_humidification_kg_per_s,
                Some(0.0),
            )
            && same_option(
                snapshot.resulting_supply_mass_flow_rate_for_cool_kg_per_s,
                Some(0.0),
            )
            && same_option(
                snapshot.resulting_supply_mass_flow_rate_for_dehumidification_kg_per_s,
                Some(0.0),
            )
            && same_option(
                snapshot.resulting_supply_mass_flow_rate_for_humidification_kg_per_s,
                Some(0.0),
            )
    } else {
        assignments_absent(snapshot)
            && same_option(
                snapshot.resulting_supply_mass_flow_rate_for_cool_kg_per_s,
                snapshot.predecessor_supply_mass_flow_rate_for_cool_kg_per_s,
            )
            && same_option(
                snapshot.resulting_supply_mass_flow_rate_for_dehumidification_kg_per_s,
                snapshot.predecessor_supply_mass_flow_rate_for_dehumidification_kg_per_s,
            )
            && same_option(
                snapshot.resulting_supply_mass_flow_rate_for_humidification_kg_per_s,
                snapshot.predecessor_supply_mass_flow_rate_for_humidification_kg_per_s,
            )
    }
}

fn assignments_absent(snapshot: &PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot) -> bool {
    !snapshot.supply_mass_flow_rate_for_cool_zero_assigned
        && snapshot
            .assigned_supply_mass_flow_rate_for_cool_kg_per_s
            .is_none()
        && !snapshot.supply_mass_flow_rate_for_dehumidification_zero_assigned
        && snapshot
            .assigned_supply_mass_flow_rate_for_dehumidification_kg_per_s
            .is_none()
        && !snapshot.supply_mass_flow_rate_for_humidification_zero_assigned
        && snapshot
            .assigned_supply_mass_flow_rate_for_humidification_kg_per_s
            .is_none()
}

fn results_absent(snapshot: &PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot) -> bool {
    snapshot
        .resulting_supply_mass_flow_rate_for_cool_kg_per_s
        .is_none()
        && snapshot
            .resulting_supply_mass_flow_rate_for_dehumidification_kg_per_s
            .is_none()
        && snapshot
            .resulting_supply_mass_flow_rate_for_humidification_kg_per_s
            .is_none()
}

pub(in crate::pipeline) fn same_option(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
