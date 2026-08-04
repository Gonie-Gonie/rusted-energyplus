use super::{all_routes, predecessor_for_outcome};
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationGuardRuntimeState as State,
    advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_state as advance,
};
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard::release::test_counts_are_exact;
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard::transition::routes::{
    logical_route_index, predecessor_index_is_public,
};

#[test]
fn exhaustive_54_outcome_characterization_is_exact() {
    let routes = all_routes();
    assert_eq!(routes.len(), 36);
    assert_eq!(routes.iter().filter(|route| route.active).count(), 18);
    assert_eq!(routes.iter().filter(|route| !route.active).count(), 18);

    let mut state = State::new(ep_model::IdealLoadsAirSystemId(412));
    let mut ordinal = 0;
    let mut public_outcomes = 0;
    let mut private_outcomes = 0;
    let mut expected_predecessor = [0usize; 36];
    let mut expected_false = [0usize; 36];
    let mut expected_body = [0usize; 36];

    for route in routes {
        let outcomes: &[bool] = if route.active { &[false, true] } else { &[false] };
        for &body_entered in outcomes {
            ordinal += 1;
            let predecessor = predecessor_for_outcome(route, ordinal, body_entered);
            let snapshot = advance(&mut state, predecessor).expect("valid CP413 outcome");
            let index = logical_route_index(route);
            expected_predecessor[index] += 1;
            if route.active {
                if body_entered {
                    expected_body[index] += 1;
                } else {
                    expected_false[index] += 1;
                }
                assert_eq!(
                    snapshot.saturation_supply_humidity_ratio_guard_body_entered,
                    body_entered,
                );
                assert_eq!(
                    snapshot.saturation_supply_humidity_ratio_guard_false_fallthrough,
                    !body_entered,
                );
                assert_eq!(
                    snapshot.saturation_supply_humidity_ratio_strictly_less_than_original_supply_humidity_ratio,
                    Some(body_entered),
                );
            } else {
                assert!(!snapshot
                    .post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_evaluated);
                assert!(snapshot
                    .saturation_supply_humidity_ratio_strictly_less_than_original_supply_humidity_ratio
                    .is_none());
            }
            assert!(option_bits_equal(
                snapshot.predecessor_cp412_resulting_supply_humidity_ratio,
                snapshot.resulting_supply_humidity_ratio,
            ));
            assert!(option_bits_equal(
                snapshot.predecessor_cp412_resulting_supply_enthalpy_j_per_kg,
                snapshot.resulting_supply_enthalpy_j_per_kg,
            ));
            assert!(option_bits_equal(
                snapshot.predecessor_cp412_resulting_supply_temperature_c,
                snapshot.resulting_supply_temperature_c,
            ));
            if predecessor_index_is_public(route.predecessor_index) {
                public_outcomes += 1;
            } else {
                private_outcomes += 1;
            }
        }
    }

    assert_eq!(ordinal, 54);
    assert_eq!(public_outcomes, 17);
    assert_eq!(private_outcomes, 37);
    assert_eq!(state.transition_count, 54);
    assert_eq!(state.inactive_transition_count, 18);
    assert_eq!(state.saturation_supply_humidity_ratio_guard_evaluation_count, 36);
    assert_eq!(state.source_site_execution_count, 126);
    assert_eq!(state.saturation_supply_humidity_ratio_guard_false_fallthrough_count, 18);
    assert_eq!(state.saturation_supply_humidity_ratio_guard_body_entry_count, 18);
    assert_eq!(state.saturation_supply_humidity_ratio_strictly_less_than_original_supply_humidity_ratio_count, 18);
    assert_eq!(state.cp412_supply_humidity_ratio_state_owner_count, 36);
    assert_eq!(state.cp412_supply_enthalpy_state_owner_count, 41);
    assert_eq!(state.cp412_supply_temperature_state_owner_count, 51);
    assert_eq!(state.unchanged_supply_humidity_ratio_preservation_count, 36);
    assert_eq!(state.unchanged_supply_enthalpy_preservation_count, 41);
    assert_eq!(state.unchanged_supply_temperature_preservation_count, 51);
    assert_eq!(state.predecessor_route_counts, expected_predecessor);
    assert_eq!(state.guard_false_fallthrough_route_counts, expected_false);
    assert_eq!(state.guard_body_entry_route_counts, expected_body);
    assert!(test_counts_are_exact(&state));
}

fn option_bits_equal(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
