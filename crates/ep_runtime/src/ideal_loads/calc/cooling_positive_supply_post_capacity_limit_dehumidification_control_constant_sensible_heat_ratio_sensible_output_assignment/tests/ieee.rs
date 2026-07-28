use super::super::{
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentRuntimeState as State,
    advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_state as advance,
    release::snapshots_match_bit_exact_for_test,
};
use super::{active_input, predecessor};
use ep_model::IdealLoadsAirSystemId;

const Q: Route = Route::DehumidificationControlConstantSensibleHeatRatioSensibleOutputAssigned;

#[test]
fn source_ast_groups_first_product_before_temperature_difference_product() {
    let mut state = State::new(IdealLoadsAirSystemId(7));
    let snapshot = advance(
        &mut state,
        predecessor(Q, 1),
        Some(active_input(1.0e308, 1.0e-308, 0.0)),
    );
    assert!(snapshot.is_some());
    let Some(snapshot) = snapshot else {
        return;
    };
    let cp_air = snapshot.cp_air_j_per_kg_k.unwrap_or_default();
    let first = 1.0e308 * cp_air;
    let delta = 1.0e-308 - 0.0;
    let grouped = first * delta;
    let reassociated = 1.0e308 * (cp_air * delta);
    assert!(grouped.is_infinite());
    assert!(reassociated.is_finite());
    assert_eq!(
        snapshot
            .supply_mass_flow_rate_times_cp_air_w_per_k
            .map(f64::to_bits),
        Some(first.to_bits())
    );
    assert_eq!(
        snapshot
            .mixed_air_minus_supply_temperature_k
            .map(f64::to_bits),
        Some(delta.to_bits())
    );
    assert_eq!(
        snapshot.cooling_sensible_output_w.map(f64::to_bits),
        Some(grouped.to_bits())
    );
}

#[test]
fn signed_zero_and_positive_infinity_flow_are_preserved_as_some_bits() {
    let mut signed_zero_state = State::new(IdealLoadsAirSystemId(7));
    let signed_zero = advance(
        &mut signed_zero_state,
        predecessor(Q, 1),
        Some(active_input(1.0, -0.0, 0.0)),
    );
    assert!(signed_zero.is_some());
    let Some(signed_zero) = signed_zero else {
        return;
    };
    assert_eq!(
        signed_zero
            .mixed_air_minus_supply_temperature_k
            .map(f64::to_bits),
        Some((-0.0f64).to_bits())
    );
    assert_eq!(
        signed_zero.cooling_sensible_output_w.map(f64::to_bits),
        Some((-0.0f64).to_bits())
    );

    let mut infinite_state = State::new(IdealLoadsAirSystemId(7));
    let infinite = advance(
        &mut infinite_state,
        predecessor(Q, 1),
        Some(active_input(f64::INFINITY, 20.0, 20.0)),
    );
    assert!(infinite.is_some());
    let Some(infinite) = infinite else {
        return;
    };
    assert!(
        infinite
            .supply_mass_flow_rate_times_cp_air_w_per_k
            .is_some_and(f64::is_infinite)
    );
    assert!(infinite.cooling_sensible_output_w.is_some_and(f64::is_nan));
    assert!(snapshots_match_bit_exact_for_test(infinite, infinite));
}

#[test]
fn nonfinite_supply_temperature_produces_bit_exact_some_values() {
    for (ordinal, supply) in [(1, f64::INFINITY), (2, f64::NAN)] {
        let mut state = State::new(IdealLoadsAirSystemId(7));
        let snapshot = advance(
            &mut state,
            predecessor(Q, ordinal),
            Some(active_input(2.0, 25.0, supply)),
        );
        assert!(snapshot.is_some());
        let Some(snapshot) = snapshot else {
            return;
        };
        let difference = 25.0 - supply;
        let cp_air = snapshot.cp_air_j_per_kg_k.unwrap_or_default();
        let expected = (2.0 * cp_air) * difference;
        assert_eq!(
            snapshot
                .mixed_air_minus_supply_temperature_k
                .map(f64::to_bits),
            Some(difference.to_bits())
        );
        assert_eq!(
            snapshot.cooling_sensible_output_w.map(f64::to_bits),
            Some(expected.to_bits())
        );
        let mut drift = snapshot;
        drift.cooling_sensible_output_w = drift
            .cooling_sensible_output_w
            .map(|value| f64::from_bits(value.to_bits().wrapping_add(1)));
        assert!(!snapshots_match_bit_exact_for_test(snapshot, drift));
    }
}

#[test]
fn cp350_reads_predecessor_cp_air_without_reinvoking_humidity_helper() {
    let mut normal = predecessor(Q, 1);
    let expected_cp_air = normal.cp_air_j_per_kg_k;
    normal.mixed_air_humidity_ratio = Some(0.5);
    let mut state = State::new(IdealLoadsAirSystemId(7));
    let snapshot = advance(&mut state, normal, Some(active_input(1.0, 21.0, 20.0)));
    assert!(snapshot.is_some());
    let Some(snapshot) = snapshot else {
        return;
    };
    assert_eq!(
        snapshot.cp_air_j_per_kg_k.map(f64::to_bits),
        expected_cp_air.map(f64::to_bits)
    );
}
