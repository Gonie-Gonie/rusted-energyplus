//! CP382 source-boundary, fixture, route, arithmetic, and release tests.

use ep_model::IdealLoadsLimit;

use super::super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentActiveInput as ActiveInput,
};
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardSnapshot as Predecessor;
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_guard::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardActiveInput as Cp381Input,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardRuntimeState as Cp381State,
    advance_cooling_post_saturation_capacity_limit_dehumidification_guard_state as advance_cp381,
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
fn cp382_source_boundary_and_order_are_exact() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_ASSIGNMENT_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2267",
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2268",
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
        &[
            "read-retained-supply-mass-flow-rate-for-post-saturation-dehumidification-total-output-product",
            "read-retained-mixed-air-enthalpy-for-post-saturation-dehumidification-total-output-difference",
            "read-retained-supply-enthalpy-for-post-saturation-dehumidification-total-output-difference",
            "calculate-mixed-air-enthalpy-minus-supply-enthalpy-for-post-saturation-dehumidification-total-output",
            "calculate-supply-mass-flow-rate-times-enthalpy-difference-for-post-saturation-dehumidification-total-output",
            "assign-local-cooling-total-output-for-post-saturation-dehumidification",
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
    let cp380 =
        advance_cp380(&mut cp380_state, cp379, cp380_input).expect("valid CP380 predecessor");

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
    advance_cp381(&mut cp381_state, cp380, cp381_input).expect("valid CP381 predecessor")
}

pub(super) fn active_input(flow: f64, mixed: f64, supply: f64) -> Option<ActiveInput> {
    Some(ActiveInput {
        supply_mass_flow_rate_kg_per_s: flow,
        mixed_air_enthalpy_j_per_kg: mixed,
        supply_enthalpy_j_per_kg: supply,
        cp330_supply_mass_flow_rate_owned_read: true,
        cp329_same_call_supply_mass_flow_rate_bit_corroborated: true,
        cp339_same_call_supply_mass_flow_rate_bit_corroborated: true,
        cp329_mixed_air_enthalpy_owned_read: true,
        cp329_same_call_recirculation_enthalpy_bit_corroborated: true,
        cp339_same_call_mixed_air_enthalpy_bit_corroborated: true,
        cp379_post_saturation_supply_enthalpy_owned_read: true,
        cp379_same_call_supply_enthalpy_bits_corroborated: true,
    })
}
