use ep_model::IdealLoadsAirSystemId;

use super::super::{
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitRetainedRoute as Route,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitRuntimeState as State,
    advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_state as advance,
    release::snapshots_match_bit_exact_for_test,
    transition::source_shaped_two_argument_maximum,
};
use super::{active_operands, predecessor};
use crate::psychrometrics::energyplus_psy_h_fn_tdb_w;

const Q: Route =
    Route::DehumidificationControlConstantSensibleHeatRatioOverdryingLimitExecuted;

#[test]
fn source_shaped_maximum_is_left_biased_for_nan_ties_zero_and_infinity() {
    let left_nan = f64::from_bits(0x7ff8_0000_0000_0042);
    let right_nan = f64::from_bits(0x7ff8_0000_0000_0099);
    let cases = [
        (left_nan, 42.0, left_nan),
        (42.0, right_nan, 42.0),
        (-0.0, 0.0, -0.0),
        (0.0, -0.0, 0.0),
        (f64::INFINITY, f64::INFINITY, f64::INFINITY),
        (f64::NEG_INFINITY, f64::INFINITY, f64::INFINITY),
        (42.0, 42.0, 42.0),
    ];
    for (left, right, expected) in cases {
        assert_eq!(
            source_shaped_two_argument_maximum(left, right).to_bits(),
            expected.to_bits()
        );
    }
}

#[test]
fn transition_preserves_nan_payload_and_canonical_psychrometric_bits() {
    let pre_limit = f64::from_bits(0x7ff8_0000_0000_1234);
    let temperature = -12.345_678_9;
    let mut state = State::new(IdealLoadsAirSystemId(7));
    let snapshot = advance(
        &mut state,
        predecessor(Q, 1),
        Some(active_operands(pre_limit, temperature)),
    );
    assert!(snapshot.is_some());
    let Some(snapshot) = snapshot else {
        return;
    };
    let minimum = energyplus_psy_h_fn_tdb_w(temperature, 1.0e-5);
    assert_eq!(
        snapshot
            .psychrometric_minimum_supply_enthalpy_j_per_kg
            .map(f64::to_bits),
        Some(minimum.to_bits())
    );
    for value in [
        snapshot.maximum_supply_enthalpy_j_per_kg,
        snapshot.assigned_supply_enthalpy_j_per_kg,
        snapshot.resulting_supply_enthalpy_j_per_kg,
    ] {
        assert_eq!(value.map(f64::to_bits), Some(pre_limit.to_bits()));
    }
    assert!(snapshots_match_bit_exact_for_test(snapshot, snapshot));
    let mut drift = snapshot;
    drift.psychrometric_minimum_supply_enthalpy_j_per_kg = drift
        .psychrometric_minimum_supply_enthalpy_j_per_kg
        .map(|value| f64::from_bits(value.to_bits().wrapping_add(1)));
    assert!(!snapshots_match_bit_exact_for_test(snapshot, drift));
}
