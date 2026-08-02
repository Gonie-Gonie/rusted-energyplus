//! CP396 passive binary64 carrier tests.

use super::*;
use ep_model::DehumidificationControlType as D;

type State = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatCaseBreakRuntimeState;

#[test]
fn compressed_snapshot_preserves_arbitrary_carrier_bits_without_numeric_gates() {
    let mut predecessor = fixtures::chain(
        3,
        1,
        true,
        Some(D::ConstantSensibleHeatRatio),
        1,
        0.7,
        18.0,
        1.0,
    )
    .cp395;
    let humidity = f64::from_bits(0x7ff8_0000_0000_0396);
    let enthalpy = f64::NEG_INFINITY;
    let temperature = -0.0f64;
    set_recursive_carriers(&mut predecessor, humidity, enthalpy, temperature);
    assert!(
        crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment_snapshot_is_exact(
            predecessor,
        )
    );

    let mut state = State::new(predecessor.system);
    let snapshot = advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_break_state(
        &mut state,
        predecessor,
    )
    .expect("arbitrary-carrier CP396");
    for (predecessor_value, resulting, expected) in [
        (
            snapshot.predecessor_cp395_resulting_supply_humidity_ratio,
            snapshot.resulting_supply_humidity_ratio,
            humidity,
        ),
        (
            snapshot.predecessor_cp395_resulting_supply_enthalpy_j_per_kg,
            snapshot.resulting_supply_enthalpy_j_per_kg,
            enthalpy,
        ),
        (
            snapshot.predecessor_cp395_resulting_supply_temperature_c,
            snapshot.resulting_supply_temperature_c,
            temperature,
        ),
    ] {
        assert_eq!(
            predecessor_value.map(f64::to_bits),
            Some(expected.to_bits())
        );
        assert_eq!(resulting.map(f64::to_bits), Some(expected.to_bits()));
    }
    assert!(
        cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_break_snapshot_is_exact(
            snapshot,
        )
    );
}

#[test]
fn binary64_snapshot_comparison_distinguishes_nan_payloads() {
    let mut predecessor = fixtures::chain(
        3,
        1,
        true,
        Some(D::ConstantSensibleHeatRatio),
        1,
        0.7,
        18.0,
        1.0,
    )
    .cp395;
    let nan = f64::from_bits(0x7ff8_0000_0000_0396);
    let enthalpy = predecessor
        .resulting_supply_enthalpy_j_per_kg
        .expect("enthalpy");
    let temperature = predecessor
        .resulting_supply_temperature_c
        .expect("temperature");
    set_recursive_carriers(&mut predecessor, nan, enthalpy, temperature);
    let mut state = State::new(predecessor.system);
    let snapshot = advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_break_state(
        &mut state,
        predecessor,
    )
    .expect("NaN CP396");
    let mut forged = snapshot;
    forged.resulting_supply_humidity_ratio = Some(f64::from_bits(nan.to_bits() ^ 1));
    assert!(
        !cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_break_snapshots_match_bit_exact(
            snapshot,
            forged,
        )
    );
}

fn set_recursive_carriers(
    snapshot: &mut crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatSupplyHumidityRatioAssignmentSnapshot,
    humidity: f64,
    enthalpy: f64,
    temperature: f64,
) {
    snapshot.predecessor_cp393_resulting_supply_humidity_ratio = Some(humidity);
    snapshot.predecessor_cp393_resulting_supply_enthalpy_j_per_kg = Some(enthalpy);
    snapshot.predecessor_cp393_resulting_supply_temperature_c = Some(temperature);
    snapshot.predecessor_cp394_resulting_supply_humidity_ratio = Some(humidity);
    snapshot.predecessor_cp394_resulting_supply_enthalpy_j_per_kg = Some(enthalpy);
    snapshot.predecessor_cp394_resulting_supply_temperature_c = Some(temperature);
    snapshot.resulting_supply_humidity_ratio = Some(humidity);
    snapshot.resulting_supply_enthalpy_j_per_kg = Some(enthalpy);
    snapshot.resulting_supply_temperature_c = Some(temperature);
}
