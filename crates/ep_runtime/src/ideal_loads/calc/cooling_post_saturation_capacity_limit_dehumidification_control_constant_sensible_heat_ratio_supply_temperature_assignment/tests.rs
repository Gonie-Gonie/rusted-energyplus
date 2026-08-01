//! CP389 source boundary, retained-state, route, IEEE, and corruption tests.

use ep_model::DehumidificationControlType as D;

use super::*;

mod corruption;
pub(in crate::ideal_loads::calc) mod fixtures;
mod ieee;
mod routes;

#[test]
fn cp389_boundaries_and_physical_eight_site_order_are_exact() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2279",
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2281",
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER.len(),
        8,
    );
}

#[test]
fn inactive_route_rejects_active_owner_payload_atomically() {
    let chain = fixtures::chain(3, 1, true, Some(D::None), 1, 0.7, 18.0, 1.0);
    let mut state = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentRuntimeState::new(chain.cp388.system);
    let before = state.clone();
    let mut retained = chain.retained_input();
    retained.active_owners = Some(chain.formula_owners);
    assert!(advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_state(
        &mut state,
        chain.cp388,
        retained,
    ).is_none());
    assert_eq!(state, before);
}
