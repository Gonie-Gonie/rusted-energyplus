use super::super::{
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioTotalOutputAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioTotalOutputAssignmentRuntimeState as State,
    advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_state as advance,
    release::snapshots_match_bit_exact_for_test,
};
use super::{active_input, predecessor};
use ep_model::IdealLoadsAirSystemId;

const Q: Route = Route::DehumidificationControlConstantSensibleHeatRatioTotalOutputAssigned;

#[test]
fn zero_denominator_preserves_source_ieee_infinity_without_line_local_gate() {
    let mut positive = predecessor(Q, 1);
    positive.cooling_sensible_output_w = Some(42.0);
    positive.calculated_cooling_sensible_output_w = Some(42.0);
    let mut positive_state = State::new(IdealLoadsAirSystemId(7));
    let positive = advance(&mut positive_state, positive, Some(active_input(0.0)));
    assert!(positive.is_some());
    let Some(positive) = positive else {
        return;
    };
    assert_eq!(
        positive.cooling_sensible_heat_ratio.map(f64::to_bits),
        Some(0.0f64.to_bits())
    );
    assert_eq!(
        positive.cooling_total_output_w.map(f64::to_bits),
        Some(f64::INFINITY.to_bits())
    );

    let mut negative = predecessor(Q, 1);
    negative.cooling_sensible_output_w = Some(-42.0);
    negative.calculated_cooling_sensible_output_w = Some(-42.0);
    let mut negative_state = State::new(IdealLoadsAirSystemId(7));
    let negative = advance(&mut negative_state, negative, Some(active_input(-0.0)));
    assert!(negative.is_some());
    let Some(negative) = negative else {
        return;
    };
    assert_eq!(
        negative.cooling_total_output_w.map(f64::to_bits),
        Some(f64::INFINITY.to_bits())
    );
}

#[test]
fn signed_zero_nan_and_infinite_division_bits_are_preserved() {
    let cases = [
        (0.0, -0.0),
        (-0.0, 2.0),
        (f64::INFINITY, 2.0),
        (1.0, f64::INFINITY),
        (f64::NAN, 0.7),
        (1.0, f64::NAN),
    ];
    for (ordinal, (sensible, ratio)) in cases.into_iter().enumerate() {
        let mut predecessor = predecessor(Q, ordinal + 1);
        predecessor.cooling_sensible_output_w = Some(sensible);
        predecessor.calculated_cooling_sensible_output_w = Some(sensible);
        let mut state = State::new(IdealLoadsAirSystemId(7));
        let snapshot = advance(
            &mut state,
            predecessor,
            Some(active_input(ratio)),
        );
        assert!(snapshot.is_some());
        let Some(snapshot) = snapshot else {
            return;
        };
        let expected = sensible / ratio;
        assert_eq!(
            snapshot.cooling_sensible_output_w.map(f64::to_bits),
            Some(sensible.to_bits())
        );
        assert_eq!(
            snapshot.cooling_sensible_heat_ratio.map(f64::to_bits),
            Some(ratio.to_bits())
        );
        assert_eq!(
            snapshot.cooling_total_output_w.map(f64::to_bits),
            Some(expected.to_bits())
        );
        assert!(snapshots_match_bit_exact_for_test(snapshot, snapshot));
        let mut drift = snapshot;
        drift.cooling_total_output_w = drift
            .cooling_total_output_w
            .map(|value| f64::from_bits(value.to_bits().wrapping_add(1)));
        assert!(!snapshots_match_bit_exact_for_test(snapshot, drift));
    }
}
