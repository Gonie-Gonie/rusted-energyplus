//! CP385 source-boundary, route, IEEE, overflow, and release-validation tests.

use super::super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER,
};
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputMaximumCapacityAssignmentSnapshot as Predecessor;
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_total_output_guard::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardRuntimeState as Cp383State,
    active_input_for_cp384_test as cp383_active_input,
    advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_guard_state as advance_cp383,
    predecessor_for_cp384_test as cp382_predecessor,
};
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputMaximumCapacityAssignmentRuntimeState as Cp384State,
    advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment_state as advance_cp384,
};
use super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentActiveOperands as ActiveOperands,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentRetainedInput as RetainedInput,
};

mod ieee;
mod overflow;
mod release;
mod routes;

#[test]
fn cp385_source_boundary_and_order_are_exact() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2270",
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2272",
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER.len(),
        6,
    );
}

pub(super) fn predecessor_for_route(
    inherited_route: usize,
    outcome: usize,
    assignment: bool,
    ordinal: usize,
) -> Predecessor {
    let cp382 = cp382_predecessor(inherited_route, outcome, ordinal);
    let mut cp383_state = Cp383State::new(cp382.system);
    let input = (outcome == 1).then(|| {
        cp383_active_input(cp382, if assignment { 99.0 } else { 100.0 })
            .expect("active CP383 input")
    });
    let cp383 = advance_cp383(&mut cp383_state, cp382, input).expect("CP383 predecessor");
    let mut cp384_state = Cp384State::new(cp383.system);
    advance_cp384(&mut cp384_state, cp383).expect("CP384 predecessor")
}

pub(super) fn retained_input(
    predecessor: Predecessor,
    preexisting: f64,
    mixed_air: f64,
    flow: f64,
) -> Option<RetainedInput> {
    predecessor
        .predecessor_dehumidification_total_output_capacity_guard_evaluated
        .then(|| RetainedInput {
            preexisting_supply_enthalpy_j_per_kg: preexisting,
            active_operands: predecessor
                .dehumidification_total_output_maximum_capacity_assignment_executed
                .then(|| ActiveOperands {
                    mixed_air_enthalpy_j_per_kg: mixed_air,
                    cooling_total_output_w: predecessor
                        .resulting_cooling_total_output_w
                        .expect("CP384 resulting output"),
                    supply_mass_flow_rate_kg_per_s: flow,
                }),
        })
}

pub(super) fn with_resulting_output(
    mut predecessor: Predecessor,
    output: f64,
) -> Predecessor {
    assert!(predecessor.dehumidification_total_output_maximum_capacity_assignment_executed);
    predecessor.maximum_total_cooling_capacity_w = Some(output);
    predecessor.assigned_cooling_total_output_w = Some(output);
    predecessor.resulting_cooling_total_output_w = Some(output);
    predecessor
}
