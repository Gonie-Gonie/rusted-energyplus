use super::{H, operands, predecessor};
use crate::ideal_loads::calc::cooling_humidistat_moisture_demand_assignment::{
    PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentRuntimeState as State,
    advance_cooling_humidistat_moisture_demand_assignment_state as advance,
};
use ep_model::IdealLoadsAirSystemId;

#[test]
fn humidistat_assignment_preserves_every_sampled_ieee_bit() {
    let values = [
        0.0,
        -0.0,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::from_bits(0x7ff8_0000_0000_0042),
        f64::from_bits(0x7ff8_0000_0000_0043),
    ];
    for value in values {
        let snapshot = advance(
            &mut State::new(IdealLoadsAirSystemId(7)),
            predecessor(H, 1),
            operands(H, value),
        )
        .expect("private-H CP359");
        let expected = value.to_bits();
        assert_eq!(
            snapshot
                .zone_dehumidifying_setpoint_moisture_demand_kg_per_s
                .expect("read")
                .to_bits(),
            expected
        );
        assert_eq!(
            snapshot
                .assigned_zone_dehumidifying_setpoint_moisture_demand_kg_per_s
                .expect("assigned")
                .to_bits(),
            expected
        );
        assert_eq!(
            snapshot
                .resulting_zone_dehumidifying_setpoint_moisture_demand_kg_per_s
                .expect("resulting")
                .to_bits(),
            expected
        );
    }
}

#[test]
fn snapshot_matcher_distinguishes_signed_zero_and_nan_payloads() {
    let negative_zero = advance(
        &mut State::new(IdealLoadsAirSystemId(7)),
        predecessor(H, 1),
        operands(H, -0.0),
    )
    .expect("negative-zero CP359");
    let mut forged = negative_zero;
    forged.assigned_zone_dehumidifying_setpoint_moisture_demand_kg_per_s = Some(0.0);
    assert!(!super::super::release::snapshots_match_bit_exact_for_test(
        negative_zero,
        forged
    ));

    let nan = f64::from_bits(0x7ff8_0000_0000_0042);
    let nan_snapshot = advance(
        &mut State::new(IdealLoadsAirSystemId(7)),
        predecessor(H, 1),
        operands(H, nan),
    )
    .expect("NaN CP359");
    assert!(super::super::release::snapshots_match_bit_exact_for_test(
        nan_snapshot,
        nan_snapshot
    ));
}
