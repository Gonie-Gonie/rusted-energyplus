//! Exhaustive CP411-snapshot-to-CP412 advance and accounting regression.

use super::{all_routes, predecessor_for_route};
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment::release::test_counts_are_exact;
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment::transition::routes::{
    logical_route_index, predecessor_index_is_public,
};
use crate::ideal_loads::calc::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationAssignmentActiveInput as ActiveInput,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationAssignmentRuntimeState as State,
    advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_state as advance,
    cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_snapshot_is_exact,
    cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_snapshot_route as snapshot_route,
};
use crate::psychrometrics::energyplus_psy_w_fn_tdb_rh_pb;

#[test]
fn all_thirty_six_valid_cp411_snapshots_advance_with_exact_counts() {
    let routes = all_routes();
    let mut state = State::new(predecessor_for_route(routes[0], 1).system);
    let mut public_active = 0;
    let mut private_active = 0;

    for (logical_index, expected_route) in routes.iter().copied().enumerate() {
        assert_eq!(logical_route_index(expected_route), logical_index);
        let predecessor = predecessor_for_route(expected_route, logical_index + 1);
        let active = logical_index >= 18;
        let pressure = 95_000.0;
        let input = active.then_some(ActiveInput {
            outdoor_barometric_pressure_pa: pressure,
        });
        let snapshot = advance(&mut state, predecessor, input).expect("CP412 advance");
        assert_eq!(snapshot_route(snapshot), Some(expected_route));
        assert!(cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_snapshot_is_exact(snapshot));
        assert_eq!(
            snapshot.post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_executed,
            active,
        );
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
        if active {
            let temperature = predecessor
                .resulting_supply_temperature_c
                .expect("active CP411 temperature");
            let expected = energyplus_psy_w_fn_tdb_rh_pb(temperature, 1.0, pressure);
            assert_eq!(
                snapshot
                    .supply_temperature_for_saturation_humidity_ratio_c
                    .map(f64::to_bits),
                Some(temperature.to_bits()),
            );
            assert_eq!(
                snapshot.outdoor_barometric_pressure_pa.map(f64::to_bits),
                Some(pressure.to_bits()),
            );
            for value in [
                snapshot.saturation_supply_humidity_ratio,
                snapshot.assigned_saturation_supply_humidity_ratio,
                snapshot.resulting_saturation_supply_humidity_ratio,
            ] {
                assert_eq!(value.map(f64::to_bits), Some(expected.to_bits()));
            }
            if predecessor_index_is_public(expected_route.predecessor_index) {
                public_active += 1;
            } else {
                private_active += 1;
            }
        } else {
            for value in [
                snapshot.supply_temperature_for_saturation_humidity_ratio_c,
                snapshot.outdoor_barometric_pressure_pa,
                snapshot.saturation_supply_humidity_ratio,
                snapshot.assigned_saturation_supply_humidity_ratio,
                snapshot.resulting_saturation_supply_humidity_ratio,
            ] {
                assert!(value.is_none());
            }
        }
        assert_eq!(
            snapshot.resulting_supply_humidity_ratio.map(f64::to_bits),
            predecessor.resulting_supply_humidity_ratio.map(f64::to_bits),
        );
        assert_eq!(
            snapshot
                .resulting_supply_enthalpy_j_per_kg
                .map(f64::to_bits),
            predecessor
                .resulting_supply_enthalpy_j_per_kg
                .map(f64::to_bits),
        );
        assert_eq!(
            snapshot.resulting_supply_temperature_c.map(f64::to_bits),
            predecessor.resulting_supply_temperature_c.map(f64::to_bits),
        );
    }

    assert!(test_counts_are_exact(&state));
    assert_eq!(state.transition_count, 36);
    assert_eq!(state.inactive_transition_count, 18);
    assert_eq!(
        state.predecessor_supply_humidity_ratio_pre_saturation_original_assignment_count,
        18,
    );
    assert_eq!(state.supply_humidity_ratio_saturation_assignment_count, 18);
    assert_eq!(state.source_site_execution_count, 72);
    assert_eq!(state.predecessor_guard_false_fallthrough_count, 6);
    assert_eq!(state.predecessor_maximum_capacity_assignment_count, 6);
    assert_eq!(state.cp411_supply_humidity_ratio_state_owner_count, 18);
    assert_eq!(state.unchanged_supply_humidity_ratio_preservation_count, 18);
    assert_eq!(state.cp411_supply_enthalpy_state_owner_count, 23);
    assert_eq!(state.unchanged_supply_enthalpy_preservation_count, 23);
    assert_eq!(state.cp411_supply_temperature_state_owner_count, 33);
    assert_eq!(state.unchanged_supply_temperature_preservation_count, 33);
    assert_eq!(state.cp411_retained_supply_temperature_owned_read_count, 18);
    assert_eq!(
        state.purchased_air_supply_temperature_for_saturation_humidity_ratio_read_count,
        18,
    );
    assert_eq!(state.environment_outdoor_barometric_pressure_owner_count, 18);
    assert_eq!(
        state.environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read_count,
        18,
    );
    assert_eq!(
        state.psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluation_count,
        18,
    );
    assert_eq!(
        state.local_saturation_supply_humidity_ratio_assignment_write_count,
        18,
    );
    assert_eq!(public_active, 4);
    assert_eq!(private_active, 14);
    for index in 0..30 {
        let expected = if matches!(index, 18..=29) {
            state.predecessor_route_counts[index]
        } else {
            0
        };
        assert_eq!(
            state.predecessor_supply_humidity_ratio_pre_saturation_original_assignment_route_counts
                [index],
            expected,
        );
        assert_eq!(
            state.supply_humidity_ratio_saturation_assignment_route_counts[index],
            expected,
        );
    }
}
