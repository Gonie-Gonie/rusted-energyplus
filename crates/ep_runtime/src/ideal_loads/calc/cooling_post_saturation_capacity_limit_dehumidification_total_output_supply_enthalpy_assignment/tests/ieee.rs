//! CP385 raw binary64 grouping and preservation.

use super::{predecessor_for_route, retained_input, with_resulting_output};
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentRuntimeState as State,
    advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_state as advance,
};

#[test]
fn cp385_executes_division_then_subtraction_without_value_gates() {
    let cases = [
        (f64::from_bits(1), 0.0, -0.0),
        (f64::INFINITY, 1.0, 2.0),
        (f64::NEG_INFINITY, -1.0, f64::INFINITY),
        (f64::from_bits(0x7ff8_0000_0000_1234), 3.0, 0.0),
    ];
    for (output, flow, mixed) in cases {
        let predecessor = with_resulting_output(predecessor_for_route(3, 1, true, 1), output);
        let input = retained_input(predecessor, -0.0, mixed, flow);
        let mut state = State::new(predecessor.system);
        let snapshot = advance(&mut state, predecessor, input).expect("raw IEEE assignment");
        let expected_specific = output / flow;
        let expected = mixed - expected_specific;
        assert_eq!(snapshot.specific_cooling_output_j_per_kg.unwrap().to_bits(), expected_specific.to_bits());
        assert_eq!(snapshot.resulting_supply_enthalpy_j_per_kg.unwrap().to_bits(), expected.to_bits());
    }
}

#[test]
fn cp385_guard_false_preserves_arbitrary_preexisting_bits_without_sites() {
    for value in [-0.0, f64::INFINITY, f64::from_bits(0x7ff8_0000_0000_4321)] {
        let predecessor = predecessor_for_route(3, 1, false, 1);
        let input = retained_input(predecessor, value, 1.0, 1.0);
        let mut state = State::new(predecessor.system);
        let snapshot = advance(&mut state, predecessor, input).expect("guard false");
        assert_eq!(snapshot.preexisting_supply_enthalpy_j_per_kg.unwrap().to_bits(), value.to_bits());
        assert_eq!(snapshot.resulting_supply_enthalpy_j_per_kg.unwrap().to_bits(), value.to_bits());
        assert_eq!(state.source_site_execution_count, 0);
    }
}
