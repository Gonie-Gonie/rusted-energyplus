use super::super::{
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyEnthalpyAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyEnthalpyAssignmentRuntimeState as State,
    advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_state as advance,
    release::snapshots_match_bit_exact_for_test,
};
use super::{active_operands, predecessor};
use ep_model::IdealLoadsAirSystemId;

const Q: Route =
    Route::DehumidificationControlConstantSensibleHeatRatioSupplyEnthalpyAssigned;

#[test]
fn pure_transition_preserves_zero_negative_and_nonfinite_ieee_bits_without_line_gate() {
    let cases = [
        (50_000.0, 42.0, 0.0),
        (50_000.0, -42.0, -0.0),
        (-0.0, 0.0, -0.0),
        (f64::INFINITY, 2.0, f64::INFINITY),
        (50_000.0, f64::INFINITY, 2.0),
        (f64::NAN, 2.0, 1.0),
        (50_000.0, f64::NAN, 1.0),
    ];
    for (ordinal, (mixed, total, flow)) in cases.into_iter().enumerate() {
        let mut state = State::new(IdealLoadsAirSystemId(7));
        let snapshot = advance(
            &mut state,
            predecessor(Q, ordinal + 1),
            Some(active_operands(mixed, total, flow)),
        );
        assert!(snapshot.is_some());
        let Some(snapshot) = snapshot else {
            return;
        };
        let specific = total / flow;
        let expected = mixed - specific;
        assert_eq!(
            snapshot.specific_cooling_output_j_per_kg.map(f64::to_bits),
            Some(specific.to_bits())
        );
        assert_eq!(
            snapshot.resulting_supply_enthalpy_j_per_kg.map(f64::to_bits),
            Some(expected.to_bits())
        );
        assert!(snapshots_match_bit_exact_for_test(snapshot, snapshot));
        let mut drift = snapshot;
        drift.resulting_supply_enthalpy_j_per_kg = drift
            .resulting_supply_enthalpy_j_per_kg
            .map(|value| f64::from_bits(value.to_bits().wrapping_add(1)));
        assert!(!snapshots_match_bit_exact_for_test(snapshot, drift));
    }
}
