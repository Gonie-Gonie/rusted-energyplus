//! CP379 eight-route, four-site, and transitive-owner tests.

use super::super::{
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentRuntimeState as State,
    advance_cooling_supply_enthalpy_post_saturation_assignment_state as advance,
    cooling_supply_enthalpy_post_saturation_assignment_snapshot_is_exact_direct_release,
};
use super::prefix_for_route;
use crate::psychrometrics::energyplus_psy_h_fn_tdb_w;

#[test]
fn cp379_retains_eight_routes_and_executes_four_sites_on_exactly_five_routes() {
    let mut state = State::new(prefix_for_route(0, 0.0).cp378.system);
    for route in 0..8 {
        let prefix = prefix_for_route(route, 0.008 + route as f64 * 0.0001);
        let snapshot = advance(&mut state, prefix.cp378, prefix.input).expect("valid CP379 route");
        let active = route >= 3;
        for flag in [
            snapshot.cp377_supply_temperature_owned_read,
            snapshot.cp378_supply_humidity_ratio_saturation_limit_owned_read,
            snapshot.purchased_air_supply_temperature_for_post_saturation_enthalpy_read,
            snapshot.purchased_air_supply_humidity_ratio_for_post_saturation_enthalpy_read,
            snapshot.psy_h_fn_tdb_w_for_post_saturation_enthalpy_evaluated,
            snapshot.local_supply_enthalpy_after_saturation_limit_assignment_performed,
        ] {
            assert_eq!(flag, active);
        }
        assert_eq!(
            usize::from(snapshot.cp334_supply_temperature_mixed_air_limit_owned_read)
                + usize::from(
                    snapshot.cp344_capacity_limit_supply_temperature_mixed_air_limit_owned_read,
                ),
            usize::from(active),
        );
        if active {
            let temperature = prefix
                .cp377
                .supply_temperature_for_saturation_humidity_ratio_c
                .expect("active CP377 temperature");
            let humidity_ratio = prefix
                .cp378
                .resulting_supply_humidity_ratio
                .expect("active CP378 humidity ratio");
            let expected = energyplus_psy_h_fn_tdb_w(temperature, humidity_ratio);
            assert_eq!(
                snapshot
                    .resulting_supply_enthalpy_j_per_kg
                    .map(f64::to_bits),
                Some(expected.to_bits()),
            );
        } else {
            assert!(snapshot.supply_temperature_c.is_none());
            assert!(snapshot.supply_humidity_ratio.is_none());
            assert!(snapshot.psychrometric_supply_enthalpy_j_per_kg.is_none());
            assert!(snapshot.assigned_supply_enthalpy_j_per_kg.is_none());
            assert!(snapshot.resulting_supply_enthalpy_j_per_kg.is_none());
        }
    }
    assert_eq!(state.transition_count, 8);
    assert_eq!(state.source_site_execution_count, 20);
    assert_eq!(
        state.purchased_air_supply_temperature_for_post_saturation_enthalpy_read_count,
        5,
    );
    assert_eq!(
        state.purchased_air_supply_humidity_ratio_for_post_saturation_enthalpy_read_count,
        5,
    );
    assert_eq!(
        state.psy_h_fn_tdb_w_for_post_saturation_enthalpy_evaluation_count,
        5,
    );
    assert_eq!(
        state.local_supply_enthalpy_after_saturation_limit_assignment_count,
        5,
    );
    assert_eq!(
        state.cp334_supply_temperature_mixed_air_limit_owner_count
            + state.cp344_capacity_limit_supply_temperature_mixed_air_limit_owner_count,
        5,
    );
    assert_eq!(
        state.cp378_supply_humidity_ratio_saturation_limit_owner_count,
        5,
    );
}

#[test]
fn cp379_exact_direct_admits_only_the_predecessor_derived_direct_routes() {
    for route in 0..8 {
        let prefix = prefix_for_route(route, 0.008);
        let mut state = State::new(prefix.cp378.system);
        let snapshot = advance(&mut state, prefix.cp378, prefix.input).expect("pure CP379 route");
        assert_eq!(
            cooling_supply_enthalpy_post_saturation_assignment_snapshot_is_exact_direct_release(
                snapshot,
            ),
            route <= 4,
        );
    }
}

#[test]
fn cp379_rejects_wrong_system_and_skip_placeholder_input_without_mutation() {
    let prefix = prefix_for_route(4, 0.008);
    let mut state = State::new(ep_model::IdealLoadsAirSystemId(99));
    let before = state.clone();
    assert!(advance(&mut state, prefix.cp378, prefix.input).is_none());
    assert_eq!(state, before);

    let prefix = prefix_for_route(0, 0.0);
    let mut state = State::new(prefix.cp378.system);
    let before = state.clone();
    let placeholder = Some(
        super::super::PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentActiveInput {
            supply_temperature_c: 20.0,
            temperature_owner: crate::ideal_loads::calc::cooling_supply_humidity_ratio_saturation_assignment::PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentTemperatureOwner::Cp334MixedAirLimit,
        },
    );
    assert!(advance(&mut state, prefix.cp378, placeholder).is_none());
    assert_eq!(state, before);
}
