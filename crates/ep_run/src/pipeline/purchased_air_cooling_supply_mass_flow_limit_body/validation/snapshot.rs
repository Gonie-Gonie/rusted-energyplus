//! Exact direct-lane shape checks for one CP326 snapshot.

use ep_runtime::PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot;

pub(in crate::pipeline) fn snapshot_shape(
    snapshot: &PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot,
    source_supply: Option<f64>,
    maximum: f64,
) -> bool {
    if !snapshot.cooling_body_entered {
        return !snapshot.supply_mass_flow_limit_body_entered
            && snapshot.body_skipped
            && !snapshot.active_guard_false_fallthrough
            && !snapshot.supply_mass_flow_rate_for_minimum_read
            && snapshot
                .supply_mass_flow_rate_before_limit_kg_per_s
                .is_none()
            && !snapshot.maximum_cooling_air_mass_flow_rate_for_minimum_read
            && snapshot
                .maximum_cooling_air_mass_flow_rate_kg_per_s
                .is_none()
            && !snapshot.source_shaped_two_argument_minimum_evaluated
            && snapshot.minimum_supply_mass_flow_rate_kg_per_s.is_none()
            && !snapshot.supply_mass_flow_rate_assignment_performed
            && snapshot.assigned_supply_mass_flow_rate_kg_per_s.is_none()
            && snapshot.resulting_supply_mass_flow_rate_kg_per_s.is_none()
            && usize::from(snapshot.unit_off_skipped) + usize::from(snapshot.non_cooling_skipped)
                == 1
            && snapshot.unit_body_entered == snapshot.non_cooling_skipped;
    }

    let Some(source_supply) = source_supply else {
        return false;
    };
    if !snapshot.supply_mass_flow_limit_body_entered {
        return snapshot.unit_body_entered
            && snapshot.predecessor_cooling_body_entered
            && !snapshot.predecessor_ems_supply_mass_flow_override_body_entered
            && snapshot.predecessor_ems_supply_mass_flow_override_body_skipped
            && snapshot.predecessor_ems_disabled_fallthrough
            && !snapshot.unit_off_skipped
            && !snapshot.non_cooling_skipped
            && snapshot.body_skipped
            && snapshot.active_guard_false_fallthrough
            && !snapshot.supply_mass_flow_rate_for_minimum_read
            && snapshot
                .supply_mass_flow_rate_before_limit_kg_per_s
                .is_none()
            && !snapshot.maximum_cooling_air_mass_flow_rate_for_minimum_read
            && snapshot
                .maximum_cooling_air_mass_flow_rate_kg_per_s
                .is_none()
            && !snapshot.source_shaped_two_argument_minimum_evaluated
            && snapshot.minimum_supply_mass_flow_rate_kg_per_s.is_none()
            && !snapshot.supply_mass_flow_rate_assignment_performed
            && snapshot.assigned_supply_mass_flow_rate_kg_per_s.is_none()
            && option_has_bits(
                snapshot.resulting_supply_mass_flow_rate_kg_per_s,
                source_supply,
            );
    }

    let expected = source_min(source_supply, maximum);
    snapshot.unit_body_entered
        && snapshot.predecessor_cooling_body_entered
        && maximum > 0.0
        && !snapshot.predecessor_ems_supply_mass_flow_override_body_entered
        && snapshot.predecessor_ems_supply_mass_flow_override_body_skipped
        && snapshot.predecessor_ems_disabled_fallthrough
        && !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && !snapshot.body_skipped
        && !snapshot.active_guard_false_fallthrough
        && snapshot.supply_mass_flow_rate_for_minimum_read
        && option_has_bits(
            snapshot.supply_mass_flow_rate_before_limit_kg_per_s,
            source_supply,
        )
        && snapshot.maximum_cooling_air_mass_flow_rate_for_minimum_read
        && option_has_bits(
            snapshot.maximum_cooling_air_mass_flow_rate_kg_per_s,
            maximum,
        )
        && snapshot.source_shaped_two_argument_minimum_evaluated
        && option_has_bits(snapshot.minimum_supply_mass_flow_rate_kg_per_s, expected)
        && snapshot.supply_mass_flow_rate_assignment_performed
        && option_has_bits(snapshot.assigned_supply_mass_flow_rate_kg_per_s, expected)
        && option_has_bits(snapshot.resulting_supply_mass_flow_rate_kg_per_s, expected)
}

fn option_has_bits(value: Option<f64>, expected: f64) -> bool {
    value.is_some_and(|value| value.to_bits() == expected.to_bits())
}

fn source_min(left: f64, right: f64) -> f64 {
    if left < right { left } else { right }
}
