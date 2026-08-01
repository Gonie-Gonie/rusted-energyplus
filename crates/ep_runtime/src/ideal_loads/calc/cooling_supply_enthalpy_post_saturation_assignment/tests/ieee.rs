//! CP379 canonical psychrometric and raw IEEE characterization tests.

use super::super::{
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentRuntimeState as State,
    advance_cooling_supply_enthalpy_post_saturation_assignment_state as advance,
    cooling_supply_enthalpy_post_saturation_assignment_snapshot_is_exact_direct_release,
    cooling_supply_enthalpy_post_saturation_assignment_snapshots_match_bit_exact,
};
use super::{prefix_for_route, prefix_for_route_with_psychrometrics};
use crate::ideal_loads::calc::psychrometrics::moist_air_enthalpy_j_per_kg;
use crate::psychrometrics::energyplus_psy_h_fn_tdb_w;

#[test]
fn cp379_uses_the_canonical_humidity_floor_and_operation_order() {
    let mut floor_results = Vec::new();
    for humidity_ratio in [-0.0, 0.0, f64::from_bits(1), 5.0e-6] {
        let prefix = prefix_for_route_with_psychrometrics(4, humidity_ratio, 20.0, 101_325.0);
        let retained_humidity_ratio = prefix
            .cp378
            .resulting_supply_humidity_ratio
            .expect("active retained humidity ratio");
        assert_eq!(retained_humidity_ratio.to_bits(), humidity_ratio.to_bits());
        let mut state = State::new(prefix.cp378.system);
        let snapshot = advance(&mut state, prefix.cp378, prefix.input).expect("raw floor case");
        let expected = energyplus_psy_h_fn_tdb_w(20.0, humidity_ratio);
        assert_eq!(
            snapshot
                .psychrometric_supply_enthalpy_j_per_kg
                .map(f64::to_bits),
            Some(expected.to_bits()),
        );
        assert!(
            cooling_supply_enthalpy_post_saturation_assignment_snapshot_is_exact_direct_release(
                snapshot,
            )
        );
        floor_results.push(expected.to_bits());
    }
    assert!(floor_results.windows(2).all(|pair| pair[0] == pair[1]));

    let prefix = prefix_for_route_with_psychrometrics(4, 0.008, 14.0, 101_325.0);
    let humidity_ratio = prefix
        .cp378
        .resulting_supply_humidity_ratio
        .expect("active retained humidity ratio");
    let mut state = State::new(prefix.cp378.system);
    let snapshot = advance(&mut state, prefix.cp378, prefix.input).expect("canonical grouping");
    let canonical = energyplus_psy_h_fn_tdb_w(14.0, humidity_ratio);
    let regrouped = moist_air_enthalpy_j_per_kg(14.0, humidity_ratio);
    assert_eq!(
        snapshot
            .resulting_supply_enthalpy_j_per_kg
            .map(f64::to_bits),
        Some(canonical.to_bits()),
    );
    assert_ne!(canonical.to_bits(), regrouped.to_bits());
}

#[test]
fn cp379_pure_transition_preserves_raw_ieee_and_public_gate_rejects_it() {
    let nan_temperature = f64::from_bits(0x7ff8_0000_0000_0379);
    let mut prefix = prefix_for_route(4, 0.008);
    prefix
        .input
        .as_mut()
        .expect("active input")
        .supply_temperature_c = nan_temperature;
    let mut state = State::new(prefix.cp378.system);
    let snapshot = advance(&mut state, prefix.cp378, prefix.input).expect("raw NaN temperature");
    assert_eq!(
        snapshot.supply_temperature_c.map(f64::to_bits),
        Some(nan_temperature.to_bits()),
    );
    assert!(
        snapshot
            .psychrometric_supply_enthalpy_j_per_kg
            .is_some_and(f64::is_nan)
    );
    assert!(
        !cooling_supply_enthalpy_post_saturation_assignment_snapshot_is_exact_direct_release(
            snapshot,
        )
    );

    let prefix = prefix_for_route_with_psychrometrics(4, -0.001, 18.0, 101_325.0);
    let mut state = State::new(prefix.cp378.system);
    let snapshot = advance(&mut state, prefix.cp378, prefix.input).expect("raw negative humidity");
    assert_eq!(snapshot.supply_humidity_ratio, Some(-0.001));
    assert!(
        !cooling_supply_enthalpy_post_saturation_assignment_snapshot_is_exact_direct_release(
            snapshot,
        )
    );

    let mut prefix = prefix_for_route(4, 0.008);
    prefix
        .input
        .as_mut()
        .expect("active input")
        .supply_temperature_c = f64::MAX;
    let mut state = State::new(prefix.cp378.system);
    let snapshot = advance(&mut state, prefix.cp378, prefix.input).expect("raw overflow");
    assert!(
        snapshot
            .psychrometric_supply_enthalpy_j_per_kg
            .is_some_and(|value| !value.is_finite())
    );
    assert!(
        !cooling_supply_enthalpy_post_saturation_assignment_snapshot_is_exact_direct_release(
            snapshot,
        )
    );
}

#[test]
fn cp379_bit_matcher_rejects_nan_payload_and_one_bit_result_drift() {
    let mut prefix = prefix_for_route(4, 0.008);
    prefix
        .input
        .as_mut()
        .expect("active input")
        .supply_temperature_c = f64::from_bits(0x7ff8_0000_0000_0379);
    let mut state = State::new(prefix.cp378.system);
    let exact = advance(&mut state, prefix.cp378, prefix.input).expect("raw exact snapshot");

    let mut drifted = exact;
    drifted.supply_temperature_c = drifted
        .supply_temperature_c
        .map(|value| f64::from_bits(value.to_bits() ^ 1));
    assert!(
        !cooling_supply_enthalpy_post_saturation_assignment_snapshots_match_bit_exact(
            exact, drifted,
        )
    );

    let mut drifted = exact;
    drifted.resulting_supply_enthalpy_j_per_kg = drifted
        .resulting_supply_enthalpy_j_per_kg
        .map(|value| f64::from_bits(value.to_bits() ^ 1));
    assert!(
        !cooling_supply_enthalpy_post_saturation_assignment_snapshots_match_bit_exact(
            exact, drifted,
        )
    );
}
