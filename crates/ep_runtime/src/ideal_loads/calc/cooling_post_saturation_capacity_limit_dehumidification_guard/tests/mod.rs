use ep_model::IdealLoadsLimit;

use super::super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardActiveInput as ActiveInput,
};
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardSnapshot as Predecessor;
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
fn cp381_source_boundary_and_order_are_exact() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2266",
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2267",
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_SOURCE_ORDER,
        &[
            "read-retained-purchased-air-supply-humidity-ratio-for-post-saturation-dehumidification-comparison",
            "read-retained-purchased-air-mixed-air-humidity-ratio-for-post-saturation-dehumidification-comparison",
            "compare-purchased-air-supply-humidity-ratio-strictly-less-than-mixed-air-humidity-ratio-for-post-saturation-dehumidification-guard",
            "enter-post-saturation-capacity-limit-dehumidification-body-if-comparison-satisfied",
        ],
    );
}

pub(super) fn predecessor_for_route(
    inherited_route: usize,
    capacity_body: bool,
    ordinal: usize,
) -> Predecessor {
    let cp379 = cp379_predecessor(inherited_route, ordinal);
    let mut state = Cp380State::new(cp379.system);
    let input = (inherited_route >= 3).then(|| {
        cp380_active_input(if capacity_body {
            IdealLoadsLimit::LimitCapacity
        } else {
            IdealLoadsLimit::NoLimit
        })
        .expect("active CP380 input")
    });
    advance_cp380(&mut state, cp379, input).expect("valid CP380 predecessor")
}

pub(super) fn active_input(supply: f64, mixed: f64) -> Option<ActiveInput> {
    Some(ActiveInput {
        supply_humidity_ratio: supply,
        mixed_air_humidity_ratio: mixed,
        cp378_supply_humidity_ratio_saturation_limit_owned_read: true,
        cp379_same_call_supply_humidity_ratio_bit_corroborated: true,
        cp329_mixed_air_humidity_ratio_owned_read: true,
    })
}
