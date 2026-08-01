//! CP377 raw IEEE psychrometric characterization tests.

use super::super::{
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentActiveInput as ActiveInput,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentTemperatureOwner as Owner,
    advance_cooling_supply_humidity_ratio_saturation_assignment_state as advance,
    cooling_supply_humidity_ratio_saturation_assignment_snapshots_match_bit_exact,
};
use super::predecessor_for_route;
use crate::psychrometrics::energyplus_psy_w_fn_tdb_rh_pb;

#[test]
fn cp377_pure_transition_uses_canonical_helper_for_raw_ieee_operands() {
    let predecessor = predecessor_for_route(4, 0.008);
    for (temperature, pressure) in [
        (-0.0, 101_325.0),
        (f64::from_bits(1), 101_325.0),
        (f64::NAN, 101_325.0),
        (30.0, f64::from_bits(0x7ff8_0000_0000_0377)),
        (30.0, f64::INFINITY),
        (30.0, -0.0),
    ] {
        let input = ActiveInput {
            supply_temperature_c: temperature,
            temperature_owner: Owner::Cp334MixedAirLimit,
            outdoor_barometric_pressure_pa: pressure,
        };
        let mut state = State::new(predecessor.system);
        let snapshot = advance(&mut state, predecessor, Some(input)).expect("raw IEEE case");
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
        assert_eq!(
            snapshot.saturation_supply_humidity_ratio.map(f64::to_bits),
            Some(expected.to_bits()),
        );
        assert!(
            cooling_supply_humidity_ratio_saturation_assignment_snapshots_match_bit_exact(
                snapshot, snapshot,
            )
        );
    }
}

#[test]
fn cp377_bit_exact_matcher_rejects_pressure_or_result_drift() {
    let predecessor = predecessor_for_route(4, 0.008);
    let input = ActiveInput {
        supply_temperature_c: 18.0,
        temperature_owner: Owner::Cp344CapacityMixedAirLimit,
        outdoor_barometric_pressure_pa: 101_325.0,
    };
    let mut state = State::new(predecessor.system);
    let snapshot = advance(&mut state, predecessor, Some(input)).expect("active");

    let mut pressure_drift = snapshot;
    pressure_drift.outdoor_barometric_pressure_pa = Some(f64::from_bits(
        input.outdoor_barometric_pressure_pa.to_bits() ^ 1,
    ));
    assert!(
        !cooling_supply_humidity_ratio_saturation_assignment_snapshots_match_bit_exact(
            snapshot,
            pressure_drift,
        )
    );

    let mut result_drift = snapshot;
    result_drift.saturation_supply_humidity_ratio = result_drift
        .saturation_supply_humidity_ratio
        .map(|value| f64::from_bits(value.to_bits() ^ 1));
    assert!(
        !cooling_supply_humidity_ratio_saturation_assignment_snapshots_match_bit_exact(
            snapshot,
            result_drift,
        )
    );
}
