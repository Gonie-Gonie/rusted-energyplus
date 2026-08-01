//! CP384 source-boundary, route, IEEE, overflow, and release tests.

use super::super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE_ORDER,
};
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardSnapshot as Predecessor;
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_total_output_guard::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardRuntimeState as Cp383State,
    active_input_for_cp384_test as cp383_active_input,
    advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_state as advance_cp383,
    predecessor_for_cp384_test as cp382_predecessor,
};

mod ieee;
mod overflow;
mod release;
mod routes;

#[test]
fn cp384_source_boundary_and_order_are_exact() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2269",
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2270",
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE_ORDER,
        &[
            "read-retained-maximum-total-cooling-capacity-for-post-saturation-dehumidification-total-output-assignment",
            "assign-local-cooling-total-output-from-maximum-total-cooling-capacity",
        ],
    );
}

pub(super) fn predecessor_for_route(
    inherited_route: usize,
    outcome: usize,
    assignment: bool,
    ordinal: usize,
) -> Predecessor {
    let cp382 = cp382_predecessor(inherited_route, outcome, ordinal);
    let mut state = Cp383State::new(cp382.system);
    let input = (outcome == 1).then(|| {
        cp383_active_input(cp382, if assignment { 99.0 } else { 100.0 })
            .expect("active CP383 input")
    });
    let predecessor = advance_cp383(&mut state, cp382, input).expect("CP383 predecessor");
    assert_eq!(
        predecessor.dehumidification_total_output_capacity_adjustment_body_entered,
        outcome == 1 && assignment,
    );
    predecessor
}

pub(super) fn with_output(mut predecessor: Predecessor, output: f64) -> Predecessor {
    assert!(predecessor.dehumidification_total_output_capacity_guard_evaluated);
    predecessor.cooling_total_output_w = Some(output);
    predecessor.cooling_total_output_strictly_greater_than_maximum_total_cooling_capacity =
        predecessor
            .maximum_total_cooling_capacity_w
            .map(|maximum| output > maximum);
    let body = predecessor
        .cooling_total_output_strictly_greater_than_maximum_total_cooling_capacity
        .unwrap_or(false);
    predecessor.dehumidification_total_output_capacity_adjustment_body_entered = body;
    predecessor.dehumidification_total_output_capacity_guard_false_fallthrough = !body;
    predecessor
}

pub(super) fn with_maximum(mut predecessor: Predecessor, maximum: f64) -> Predecessor {
    assert!(predecessor.dehumidification_total_output_capacity_guard_evaluated);
    predecessor.maximum_total_cooling_capacity_w = Some(maximum);
    let output = predecessor.cooling_total_output_w.expect("output");
    let body = output > maximum;
    predecessor.cooling_total_output_strictly_greater_than_maximum_total_cooling_capacity =
        Some(body);
    predecessor.dehumidification_total_output_capacity_adjustment_body_entered = body;
    predecessor.dehumidification_total_output_capacity_guard_false_fallthrough = !body;
    predecessor
}
