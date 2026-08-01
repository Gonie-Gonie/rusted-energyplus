//! CP378 eight-route, four-site, and local-owner tests.

use super::super::{
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentRuntimeState as State,
    advance_cooling_supply_humidity_ratio_saturation_limit_assignment_state as advance,
    cooling_supply_humidity_ratio_saturation_limit_assignment_snapshot_is_exact_direct_release,
};
use super::predecessor_for_route;
use crate::ideal_loads::calc::cooling_positive_supply_temperature_mixed_air_limit::source_shaped_two_argument_minimum;

#[test]
fn cp378_retains_eight_routes_and_executes_four_sites_on_exactly_five_routes() {
    let mut state = State::new(predecessor_for_route(0, 0.0).system);
    for route in 0..8 {
        let predecessor = predecessor_for_route(route, 0.008 + route as f64 * 0.0001);
        let snapshot = advance(&mut state, predecessor).expect("valid CP378 route");
        let active = route >= 3;
        for flag in [
            snapshot.cp376_original_supply_humidity_ratio_owned_read,
            snapshot.cp377_saturation_supply_humidity_ratio_owned_read,
            snapshot.local_original_supply_humidity_ratio_for_saturation_limit_minimum_read,
            snapshot.local_saturation_supply_humidity_ratio_for_saturation_limit_minimum_read,
            snapshot.source_shaped_two_argument_minimum_evaluated,
            snapshot.purchased_air_supply_humidity_ratio_saturation_limit_assignment_performed,
        ] {
            assert_eq!(flag, active);
        }
        if active {
            let left = predecessor
                .predecessor_resulting_supply_humidity_ratio_original
                .expect("active original");
            let right = predecessor
                .resulting_saturation_supply_humidity_ratio
                .expect("active saturation");
            let expected = source_shaped_two_argument_minimum(left, right);
            assert_eq!(
                snapshot.resulting_supply_humidity_ratio.map(f64::to_bits),
                Some(expected.to_bits()),
            );
        } else {
            assert!(snapshot.resulting_supply_humidity_ratio.is_none());
        }
    }
    assert_eq!(state.transition_count, 8);
    assert_eq!(state.source_site_execution_count, 20);
    assert_eq!(
        state.local_original_supply_humidity_ratio_for_saturation_limit_minimum_read_count,
        5,
    );
    assert_eq!(
        state.local_saturation_supply_humidity_ratio_for_saturation_limit_minimum_read_count,
        5,
    );
    assert_eq!(state.source_shaped_two_argument_minimum_evaluation_count, 5);
    assert_eq!(
        state.purchased_air_supply_humidity_ratio_saturation_limit_assignment_count,
        5,
    );
    assert_eq!(state.cp376_original_supply_humidity_ratio_owner_count, 5);
    assert_eq!(state.cp377_saturation_supply_humidity_ratio_owner_count, 5);
}

#[test]
fn cp378_exact_direct_admits_only_the_predecessor_derived_direct_routes() {
    for route in 0..8 {
        let predecessor = predecessor_for_route(route, 0.008);
        let mut state = State::new(predecessor.system);
        let snapshot = advance(&mut state, predecessor).expect("pure CP378 route");
        assert_eq!(
            cooling_supply_humidity_ratio_saturation_limit_assignment_snapshot_is_exact_direct_release(
                snapshot,
            ),
            route <= 4,
        );
    }
}

#[test]
fn cp378_rejects_wrong_system_without_mutation() {
    let predecessor = predecessor_for_route(4, 0.008);
    let mut state = State::new(ep_model::IdealLoadsAirSystemId(99));
    let before = state.clone();
    assert!(advance(&mut state, predecessor).is_none());
    assert_eq!(state, before);
}
