use super::*;

#[test]
fn pure_transition_preserves_raw_ieee_subtraction_then_multiplication() {
    for operands in [
        [1.0, 3.0, 2.0],
        [2.0, -0.0, 0.0],
        [f64::INFINITY, 1.0, 1.0],
        [f64::INFINITY, 2.0, 1.0],
        [f64::MAX, f64::MAX, -f64::MAX],
        [
            f64::from_bits(0x7ff8_0000_0000_00a1),
            f64::NEG_INFINITY,
            f64::INFINITY,
        ],
        [
            1.0,
            f64::from_bits(0x7ff8_0000_0000_00b2),
            f64::from_bits(0x7ff8_0000_0000_00c3),
        ],
    ] {
        let mut state =
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentRuntimeState::new(
                ep_model::IdealLoadsAirSystemId(3),
            );
        let snapshot = advance(&mut state, Route::Assigned, 1, operands);
        let expected_difference = operands[1] - operands[2];
        let expected_product = operands[0] * expected_difference;

        assert_eq!(
            snapshot.supply_mass_flow_rate_kg_per_s.map(f64::to_bits),
            Some(operands[0].to_bits())
        );
        assert_eq!(
            snapshot.mixed_air_enthalpy_j_per_kg.map(f64::to_bits),
            Some(operands[1].to_bits())
        );
        assert_eq!(
            snapshot.supply_enthalpy_j_per_kg.map(f64::to_bits),
            Some(operands[2].to_bits())
        );
        assert_eq!(
            snapshot
                .mixed_air_minus_supply_enthalpy_j_per_kg
                .map(f64::to_bits),
            Some(expected_difference.to_bits())
        );
        assert_eq!(
            snapshot
                .calculated_cooling_sensible_output_w
                .map(f64::to_bits),
            Some(expected_product.to_bits())
        );
        assert_eq!(
            snapshot.cooling_sensible_output_w.map(f64::to_bits),
            Some(expected_product.to_bits())
        );
    }
}

#[test]
fn source_grouping_is_not_distributed_or_reassociated() {
    let operands: [f64; 3] = [1.0e308, 1.000_000_000_000_000_2, 1.0];
    let grouped = operands[0] * (operands[1] - operands[2]);
    let distributed = operands[0] * operands[1] - operands[0] * operands[2];
    assert_ne!(grouped.to_bits(), distributed.to_bits());

    let mut state =
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentRuntimeState::new(
            ep_model::IdealLoadsAirSystemId(3),
        );
    let snapshot = advance(&mut state, Route::Assigned, 1, operands);
    assert_eq!(
        snapshot.cooling_sensible_output_w.map(f64::to_bits),
        Some(grouped.to_bits())
    );
}

#[test]
fn exact_snapshot_accepts_derived_nan_and_matcher_is_bit_exact() {
    let mut state =
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentRuntimeState::new(
            ep_model::IdealLoadsAirSystemId(3),
        );
    let snapshot = advance(
        &mut state,
        Route::Assigned,
        1,
        [f64::INFINITY, 1.0, 1.0],
    );
    assert!(
        snapshot
            .cooling_sensible_output_w
            .is_some_and(f64::is_nan)
    );
    assert!(
        cooling_positive_supply_capacity_limit_sensible_output_assignment_snapshot_is_exact_direct_release(
            snapshot,
        )
    );
    assert!(super::super::release::snapshots_match_bit_exact(
        snapshot, snapshot,
    ));

    let mut different_payload = snapshot;
    different_payload.cooling_sensible_output_w =
        Some(f64::from_bits(0x7ff8_0000_0000_00a1));
    assert!(!super::super::release::snapshots_match_bit_exact(
        snapshot,
        different_payload,
    ));
}

#[test]
fn signed_zero_is_preserved_through_delta_product_and_assignment() {
    let mut state =
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputAssignmentRuntimeState::new(
            ep_model::IdealLoadsAirSystemId(3),
        );
    let snapshot = advance(&mut state, Route::Assigned, 1, [2.0, -0.0, 0.0]);
    assert_eq!(
        snapshot
            .mixed_air_minus_supply_enthalpy_j_per_kg
            .map(f64::to_bits),
        Some((-0.0_f64).to_bits())
    );
    assert_eq!(
        snapshot.cooling_sensible_output_w.map(f64::to_bits),
        Some((-0.0_f64).to_bits())
    );
}
