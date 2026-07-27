use ep_model::IdealLoadsLimit;

use super::super::{
    PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE_ORDER,
    PurchasedAirCalcCoolingCapacityZeroFlowResetRetainedRoute,
    PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot,
};

pub(in crate::ideal_loads) fn cooling_capacity_zero_flow_reset_snapshot_is_exact_direct_release(
    snapshot: PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot,
) -> bool {
    let provenance = snapshot.source == PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE
        && snapshot.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_FIRST_EXCLUDED_SOURCE
        && snapshot.source_order
            == PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE_ORDER;
    let unit_off =
        snapshot.unit_off_skipped && !snapshot.unit_body_entered && !snapshot.cooling_body_entered;
    let non_cooling = snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && !snapshot.cooling_body_entered;
    let cooling = !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && snapshot.cooling_body_entered;
    provenance
        && snapshot.predecessor_cooling_body_entered == snapshot.cooling_body_entered
        && usize::from(unit_off) + usize::from(non_cooling) + usize::from(cooling) == 1
        && if cooling {
            cooling_sites_are_exact(snapshot)
        } else {
            skipped_sites_are_exact(snapshot)
        }
}

pub(super) fn cooling_capacity_zero_flow_reset_snapshot_route(
    snapshot: PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot,
) -> Option<PurchasedAirCalcCoolingCapacityZeroFlowResetRetainedRoute> {
    if !cooling_capacity_zero_flow_reset_snapshot_is_exact_direct_release(snapshot) {
        return None;
    }
    if snapshot.unit_off_skipped {
        Some(PurchasedAirCalcCoolingCapacityZeroFlowResetRetainedRoute::UnitOff)
    } else if snapshot.non_cooling_skipped {
        Some(PurchasedAirCalcCoolingCapacityZeroFlowResetRetainedRoute::NonCooling)
    } else if snapshot.cooling_limit_condition_satisfied != Some(true) {
        Some(PurchasedAirCalcCoolingCapacityZeroFlowResetRetainedRoute::CoolingLimitRejected)
    } else if !snapshot.zero_cooling_capacity_body_entered {
        Some(
            PurchasedAirCalcCoolingCapacityZeroFlowResetRetainedRoute::
                MaximumCoolingCapacityNonZero,
        )
    } else {
        Some(PurchasedAirCalcCoolingCapacityZeroFlowResetRetainedRoute::CandidatesZeroed)
    }
}

fn cooling_sites_are_exact(snapshot: PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot) -> bool {
    if !snapshot.first_cooling_limit_read {
        return false;
    }
    let Some(limit) = snapshot.first_cooling_limit else {
        return false;
    };
    let first = limit == IdealLoadsLimit::LimitCapacity;
    let second = limit == IdealLoadsLimit::LimitFlowRateAndCapacity;
    let limit_shape = snapshot.cooling_limit_capacity == Some(first)
        && snapshot.second_cooling_limit_read != first
        && snapshot.second_cooling_limit == (!first).then_some(limit)
        && snapshot.cooling_limit_flow_rate_and_capacity == (!first).then_some(second)
        && snapshot.cooling_limit_condition_satisfied == Some(first || second);
    let candidates_present = snapshot
        .predecessor_supply_mass_flow_rate_for_cool_kg_per_s
        .is_some()
        && snapshot
            .predecessor_supply_mass_flow_rate_for_dehumidification_kg_per_s
            .is_some()
        && snapshot
            .predecessor_supply_mass_flow_rate_for_humidification_kg_per_s
            .is_some();
    limit_shape
        && candidates_present
        && if first || second {
            capacity_sites_are_exact(snapshot)
        } else {
            capacity_sites_are_skipped(snapshot) && results_preserve_candidates(snapshot)
        }
}

fn capacity_sites_are_exact(
    snapshot: PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot,
) -> bool {
    let Some(capacity) = snapshot.maximum_total_cooling_capacity_w else {
        return false;
    };
    let is_zero = capacity == 0.0;
    snapshot.maximum_total_cooling_capacity_read
        && snapshot.maximum_total_cooling_capacity_comparison_evaluated
        && snapshot.maximum_total_cooling_capacity_equal_to_zero == Some(is_zero)
        && snapshot.zero_cooling_capacity_body_entered == is_zero
        && if is_zero {
            assignments_are_positive_zero(snapshot)
        } else {
            assignments_are_skipped(snapshot) && results_preserve_candidates(snapshot)
        }
}

fn skipped_sites_are_exact(snapshot: PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot) -> bool {
    !snapshot.first_cooling_limit_read
        && snapshot.first_cooling_limit.is_none()
        && snapshot.cooling_limit_capacity.is_none()
        && !snapshot.second_cooling_limit_read
        && snapshot.second_cooling_limit.is_none()
        && snapshot.cooling_limit_flow_rate_and_capacity.is_none()
        && snapshot.cooling_limit_condition_satisfied.is_none()
        && capacity_sites_are_skipped(snapshot)
        && snapshot
            .predecessor_supply_mass_flow_rate_for_cool_kg_per_s
            .is_none()
        && snapshot
            .predecessor_supply_mass_flow_rate_for_dehumidification_kg_per_s
            .is_none()
        && snapshot
            .predecessor_supply_mass_flow_rate_for_humidification_kg_per_s
            .is_none()
        && snapshot
            .resulting_supply_mass_flow_rate_for_cool_kg_per_s
            .is_none()
        && snapshot
            .resulting_supply_mass_flow_rate_for_dehumidification_kg_per_s
            .is_none()
        && snapshot
            .resulting_supply_mass_flow_rate_for_humidification_kg_per_s
            .is_none()
}

fn capacity_sites_are_skipped(
    snapshot: PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot,
) -> bool {
    !snapshot.maximum_total_cooling_capacity_read
        && snapshot.maximum_total_cooling_capacity_w.is_none()
        && !snapshot.maximum_total_cooling_capacity_comparison_evaluated
        && snapshot
            .maximum_total_cooling_capacity_equal_to_zero
            .is_none()
        && !snapshot.zero_cooling_capacity_body_entered
        && assignments_are_skipped(snapshot)
}

fn assignments_are_skipped(snapshot: PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot) -> bool {
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

fn assignments_are_positive_zero(
    snapshot: PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot,
) -> bool {
    snapshot.supply_mass_flow_rate_for_cool_zero_assigned
        && has_bits(
            snapshot.assigned_supply_mass_flow_rate_for_cool_kg_per_s,
            0.0,
        )
        && snapshot.supply_mass_flow_rate_for_dehumidification_zero_assigned
        && has_bits(
            snapshot.assigned_supply_mass_flow_rate_for_dehumidification_kg_per_s,
            0.0,
        )
        && snapshot.supply_mass_flow_rate_for_humidification_zero_assigned
        && has_bits(
            snapshot.assigned_supply_mass_flow_rate_for_humidification_kg_per_s,
            0.0,
        )
        && has_bits(
            snapshot.resulting_supply_mass_flow_rate_for_cool_kg_per_s,
            0.0,
        )
        && has_bits(
            snapshot.resulting_supply_mass_flow_rate_for_dehumidification_kg_per_s,
            0.0,
        )
        && has_bits(
            snapshot.resulting_supply_mass_flow_rate_for_humidification_kg_per_s,
            0.0,
        )
}

fn results_preserve_candidates(
    snapshot: PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot,
) -> bool {
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

fn same_bits(left: Option<f64>, right: Option<f64>) -> bool {
    left.zip(right)
        .is_some_and(|(left, right)| left.to_bits() == right.to_bits())
}

fn has_bits(value: Option<f64>, expected: f64) -> bool {
    value.is_some_and(|value| value.to_bits() == expected.to_bits())
}
