//! CP412 raw IEEE-754 characterization tests.

use super::{all_routes, predecessor_with_temperature};
use crate::ideal_loads::calc::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationAssignmentActiveInput as ActiveInput,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationAssignmentRuntimeState as State,
    advance_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_state as advance,
    cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_snapshots_match_bit_exact,
};
use crate::psychrometrics::energyplus_psy_w_fn_tdb_rh_pb;

#[test]
fn raw_temperature_pressure_and_result_bits_are_authoritative() {
    let route = all_routes()
        .into_iter()
        .find(|route| route.predecessor_index == 20 && route.predecessor_guard_false_fallthrough)
        .expect("active public route");
    let cases = [
        (f64::from_bits(0x7ff8_0000_0000_0412), 101_325.0),
        (-0.0, 101_325.0),
        (f64::from_bits(1), 101_325.0),
        (18.0, f64::from_bits(0x7ff8_0000_0000_1412)),
        (18.0, f64::INFINITY),
        (18.0, f64::NEG_INFINITY),
        (18.0, -0.0),
        (18.0, f64::from_bits(1)),
    ];
    for (temperature, pressure) in cases {
        let predecessor = predecessor_with_temperature(route, 1, temperature);
        let mut state = State::new(predecessor.system);
        let snapshot = advance(
            &mut state,
            predecessor,
            Some(ActiveInput {
                outdoor_barometric_pressure_pa: pressure,
            }),
        )
        .expect("raw IEEE transition");
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
        assert!(cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_snapshots_match_bit_exact(snapshot, snapshot));
    }
}

#[test]
fn one_bit_input_or_result_drift_is_rejected() {
    let route = all_routes()
        .into_iter()
        .find(|route| route.predecessor_index == 20 && route.predecessor_guard_false_fallthrough)
        .expect("active public route");
    let predecessor = predecessor_with_temperature(route, 1, 18.0);
    let snapshot = advance(
        &mut State::new(predecessor.system),
        predecessor,
        Some(ActiveInput {
            outdoor_barometric_pressure_pa: 101_325.0,
        }),
    )
    .expect("active transition");

    let mut drift = snapshot;
    drift.outdoor_barometric_pressure_pa = drift
        .outdoor_barometric_pressure_pa
        .map(|value| f64::from_bits(value.to_bits() ^ 1));
    assert!(!cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_snapshots_match_bit_exact(snapshot, drift));

    let mut drift = snapshot;
    drift.assigned_saturation_supply_humidity_ratio = drift
        .assigned_saturation_supply_humidity_ratio
        .map(|value| f64::from_bits(value.to_bits() ^ 1));
    assert!(!cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_snapshots_match_bit_exact(snapshot, drift));
}
