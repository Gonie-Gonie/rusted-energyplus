//! CP377 eight-route, four-site, and temperature-owner tests.

use super::super::{
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentActiveInput as ActiveInput,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentTemperatureOwner as Owner,
    advance_cooling_supply_humidity_ratio_saturation_assignment_state as advance,
    cooling_supply_humidity_ratio_saturation_assignment_snapshot_is_exact_direct_release,
};
use super::predecessor_for_route;
use crate::psychrometrics::energyplus_psy_w_fn_tdb_rh_pb;

#[test]
fn cp377_retains_eight_routes_and_executes_four_sites_on_exactly_five_routes() {
    let mut state = State::new(predecessor_for_route(0, 0.0).system);
    for route in 0..8 {
        let predecessor = predecessor_for_route(route, 0.008 + route as f64 * 0.0001);
        let input = input_for_route(route);
        let snapshot = advance(&mut state, predecessor, input).expect("valid CP377 route");
        let active = route >= 3;
        assert_eq!(
            snapshot.purchased_air_supply_temperature_for_saturation_humidity_ratio_read,
            active,
        );
        assert_eq!(
            snapshot.environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read,
            active,
        );
        assert_eq!(
            snapshot.psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluated,
            active,
        );
        assert_eq!(
            snapshot.local_saturation_supply_humidity_ratio_assignment_performed,
            active,
        );
        if let Some(input) = input {
            let expected = energyplus_psy_w_fn_tdb_rh_pb(
                input.supply_temperature_c,
                1.0,
                input.outdoor_barometric_pressure_pa,
            );
            assert_eq!(
                snapshot.saturation_supply_humidity_ratio.map(f64::to_bits),
                Some(expected.to_bits()),
            );
        }
    }
    assert_eq!(state.transition_count, 8);
    assert_eq!(state.source_site_execution_count, 20);
    assert_eq!(
        state.purchased_air_supply_temperature_for_saturation_humidity_ratio_read_count,
        5,
    );
    assert_eq!(
        state.environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read_count,
        5,
    );
    assert_eq!(
        state.psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluation_count,
        5,
    );
    assert_eq!(
        state.local_saturation_supply_humidity_ratio_assignment_count,
        5
    );
    assert_eq!(
        state.cp334_supply_temperature_mixed_air_limit_owner_count,
        3
    );
    assert_eq!(
        state.cp344_capacity_limit_supply_temperature_mixed_air_limit_owner_count,
        2,
    );
    assert_eq!(state.environment_outdoor_barometric_pressure_owner_count, 5);
}

#[test]
fn cp377_enforces_complete_null_skip_and_complete_active_input_shapes() {
    let skipped = predecessor_for_route(0, 0.0);
    let active = predecessor_for_route(4, 0.008);
    let input = input_for_route(4);

    let mut state = State::new(skipped.system);
    assert!(advance(&mut state, skipped, input).is_none());
    assert_eq!(state, State::new(skipped.system));

    let mut state = State::new(active.system);
    assert!(advance(&mut state, active, None).is_none());
    assert_eq!(state, State::new(active.system));
}

#[test]
fn cp377_exact_direct_admits_finite_positive_direct_shape_only() {
    let predecessor = predecessor_for_route(4, 0.008);
    let mut state = State::new(predecessor.system);
    let direct = advance(&mut state, predecessor, input_for_route(4)).expect("direct shape");
    assert!(
        cooling_supply_humidity_ratio_saturation_assignment_snapshot_is_exact_direct_release(
            direct,
        )
    );

    let mut state = State::new(predecessor.system);
    let raw = advance(
        &mut state,
        predecessor,
        Some(ActiveInput {
            outdoor_barometric_pressure_pa: f64::NAN,
            ..input_for_route(4).expect("active")
        }),
    )
    .expect("pure raw IEEE shape");
    assert!(
        !cooling_supply_humidity_ratio_saturation_assignment_snapshot_is_exact_direct_release(raw,)
    );
}

pub(super) fn input_for_route(route: usize) -> Option<ActiveInput> {
    if route < 3 {
        return None;
    }
    Some(ActiveInput {
        supply_temperature_c: 12.0 + route as f64,
        temperature_owner: if route.is_multiple_of(2) {
            Owner::Cp344CapacityMixedAirLimit
        } else {
            Owner::Cp334MixedAirLimit
        },
        outdoor_barometric_pressure_pa: 101_325.0,
    })
}
