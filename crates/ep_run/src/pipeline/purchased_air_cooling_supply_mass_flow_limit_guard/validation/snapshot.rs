//! Exact direct-lane shape checks for one CP325 snapshot.

use ep_model::IdealLoadsLimit;
use ep_runtime::PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot;

pub(in crate::pipeline) fn snapshot_shape(
    snapshot: &PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot,
    maximum: f64,
) -> bool {
    if !snapshot.cooling_body_entered {
        return !snapshot.first_cooling_limit_read
            && snapshot.first_cooling_limit.is_none()
            && !snapshot.cooling_limit_flow_rate_comparison_evaluated
            && snapshot.cooling_limit_flow_rate.is_none()
            && !snapshot.second_cooling_limit_read
            && snapshot.second_cooling_limit.is_none()
            && !snapshot.cooling_limit_flow_rate_and_capacity_comparison_evaluated
            && snapshot.cooling_limit_flow_rate_and_capacity.is_none()
            && snapshot.cooling_limit_condition_satisfied.is_none()
            && !snapshot.maximum_cooling_air_mass_flow_rate_read
            && snapshot
                .maximum_cooling_air_mass_flow_rate_kg_per_s
                .is_none()
            && !snapshot.maximum_cooling_air_mass_flow_rate_positive_comparison_evaluated
            && snapshot
                .maximum_cooling_air_mass_flow_rate_strictly_positive
                .is_none()
            && !snapshot.supply_mass_flow_limit_body_entered
            && !snapshot.active_guard_false_fallthrough
            && usize::from(snapshot.unit_off_skipped) + usize::from(snapshot.non_cooling_skipped)
                == 1
            && snapshot.unit_body_entered == snapshot.non_cooling_skipped;
    }

    let Some(limit) = snapshot.first_cooling_limit else {
        return false;
    };
    let first = limit == IdealLoadsLimit::LimitFlowRate;
    let read_second = !first;
    let combined = limit == IdealLoadsLimit::LimitFlowRateAndCapacity;
    let selected = first || combined;
    let positive = selected && maximum > 0.0;
    snapshot.unit_body_entered
        && snapshot.predecessor_cooling_body_entered
        && !snapshot.predecessor_ems_supply_mass_flow_override_body_entered
        && snapshot.predecessor_ems_supply_mass_flow_override_body_skipped
        && snapshot.predecessor_ems_disabled_fallthrough
        && !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && snapshot.first_cooling_limit_read
        && snapshot.cooling_limit_flow_rate_comparison_evaluated
        && snapshot.cooling_limit_flow_rate == Some(first)
        && snapshot.second_cooling_limit_read == read_second
        && snapshot.second_cooling_limit == read_second.then_some(limit)
        && snapshot.cooling_limit_flow_rate_and_capacity_comparison_evaluated == read_second
        && snapshot.cooling_limit_flow_rate_and_capacity == read_second.then_some(combined)
        && snapshot.cooling_limit_condition_satisfied == Some(selected)
        && snapshot.maximum_cooling_air_mass_flow_rate_read == selected
        && option_matches_selected(
            snapshot.maximum_cooling_air_mass_flow_rate_kg_per_s,
            selected,
            maximum,
        )
        && snapshot.maximum_cooling_air_mass_flow_rate_positive_comparison_evaluated == selected
        && snapshot.maximum_cooling_air_mass_flow_rate_strictly_positive
            == selected.then_some(positive)
        && snapshot.supply_mass_flow_limit_body_entered == positive
        && snapshot.active_guard_false_fallthrough != positive
}

fn option_matches_selected(value: Option<f64>, selected: bool, expected: f64) -> bool {
    if selected {
        value.is_some_and(|value| value.to_bits() == expected.to_bits())
    } else {
        value.is_none()
    }
}
