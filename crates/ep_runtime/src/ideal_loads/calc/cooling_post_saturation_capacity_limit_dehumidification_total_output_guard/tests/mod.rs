//! CP383 source-boundary, route, IEEE, overflow, and release tests.

use ep_model::IdealLoadsLimit;

use super::super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputGuardActiveInput as ActiveInput,
};
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentSnapshot as Predecessor;
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_guard::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardActiveInput as Cp381Input,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardRuntimeState as Cp381State,
    advance_cooling_post_saturation_capacity_limit_dehumidification_guard_state as advance_cp381,
};
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentActiveInput as Cp382Input,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentRuntimeState as Cp382State,
    advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_state as advance_cp382,
};
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_guard::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardRuntimeState as Cp380State,
    active_input_for_cp381_test as cp380_active_input,
    advance_cooling_post_saturation_capacity_limit_guard_state as advance_cp380,
    predecessor_for_cp381_test as cp379_predecessor,
};

mod ieee;
mod overflow;
mod release;
mod routes;

#[test]
fn cp383_source_boundary_and_order_are_exact() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_GUARD_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2268",
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2269",
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_GUARD_SOURCE_ORDER,
        &[
            "read-retained-cooling-total-output-for-post-saturation-dehumidification-maximum-capacity-comparison",
            "read-retained-maximum-total-cooling-capacity-for-post-saturation-dehumidification-total-output-comparison",
            "compare-post-saturation-dehumidification-cooling-total-output-strictly-greater-than-maximum-total-cooling-capacity",
            "enter-post-saturation-dehumidification-total-output-capacity-adjustment-body-if-comparison-satisfied",
        ],
    );
}

pub(super) fn predecessor_for_route(
    inherited_route: usize,
    outcome: usize,
    ordinal: usize,
) -> Predecessor {
    assert!(inherited_route < 8);
    assert!(outcome == 0 || inherited_route >= 3 && outcome < 3);

    let cp379 = cp379_predecessor(inherited_route, ordinal);
    let mut cp380_state = Cp380State::new(cp379.system);
    let cp380_input = (inherited_route >= 3).then(|| {
        cp380_active_input(if outcome == 0 {
            IdealLoadsLimit::NoLimit
        } else {
            IdealLoadsLimit::LimitCapacity
        })
        .expect("active CP380 input")
    });
    let cp380 = advance_cp380(&mut cp380_state, cp379, cp380_input).expect("CP380");

    let mut cp381_state = Cp381State::new(cp380.system);
    let cp381_input = match outcome {
        0 => None,
        1 => Some(Cp381Input {
            supply_humidity_ratio: 0.007,
            mixed_air_humidity_ratio: 0.009,
            cp378_supply_humidity_ratio_saturation_limit_owned_read: true,
            cp379_same_call_supply_humidity_ratio_bit_corroborated: true,
            cp329_mixed_air_humidity_ratio_owned_read: true,
        }),
        2 => Some(Cp381Input {
            supply_humidity_ratio: 0.009,
            mixed_air_humidity_ratio: 0.009,
            cp378_supply_humidity_ratio_saturation_limit_owned_read: true,
            cp379_same_call_supply_humidity_ratio_bit_corroborated: true,
            cp329_mixed_air_humidity_ratio_owned_read: true,
        }),
        _ => unreachable!(),
    };
    let cp381 = advance_cp381(&mut cp381_state, cp380, cp381_input).expect("CP381");

    let mut cp382_state = Cp382State::new(cp381.system);
    let cp382_input = (outcome == 1).then_some(Cp382Input {
        supply_mass_flow_rate_kg_per_s: 1.0,
        mixed_air_enthalpy_j_per_kg: 100.0,
        supply_enthalpy_j_per_kg: 0.0,
        cp330_supply_mass_flow_rate_owned_read: true,
        cp329_same_call_supply_mass_flow_rate_bit_corroborated: true,
        cp339_same_call_supply_mass_flow_rate_bit_corroborated: true,
        cp329_mixed_air_enthalpy_owned_read: true,
        cp329_same_call_recirculation_enthalpy_bit_corroborated: true,
        cp339_same_call_mixed_air_enthalpy_bit_corroborated: true,
        cp379_post_saturation_supply_enthalpy_owned_read: true,
        cp379_same_call_supply_enthalpy_bits_corroborated: true,
    });
    advance_cp382(&mut cp382_state, cp381, cp382_input).expect("CP382")
}

pub(super) fn active_input(predecessor: Predecessor, capacity: f64) -> Option<ActiveInput> {
    Some(ActiveInput {
        cooling_total_output_w: predecessor
            .cooling_total_output_w
            .expect("active CP382 output"),
        maximum_total_cooling_capacity_w: capacity,
        cp382_cooling_total_output_owned_read: true,
        cp321_maximum_total_cooling_capacity_owned_read: true,
        cp340_same_call_maximum_total_cooling_capacity_bit_corroborated: true,
    })
}

pub(super) fn predecessor_with_output(mut predecessor: Predecessor, mixed: f64) -> Predecessor {
    assert!(predecessor.dehumidification_total_output_assignment_executed);
    let difference = mixed - 0.0;
    let output = 1.0 * difference;
    predecessor.supply_mass_flow_rate_kg_per_s = Some(1.0);
    predecessor.mixed_air_enthalpy_j_per_kg = Some(mixed);
    predecessor.supply_enthalpy_j_per_kg = Some(0.0);
    predecessor.mixed_air_minus_supply_enthalpy_j_per_kg = Some(difference);
    predecessor.calculated_cooling_total_output_w = Some(output);
    predecessor.cooling_total_output_w = Some(output);
    predecessor
}
