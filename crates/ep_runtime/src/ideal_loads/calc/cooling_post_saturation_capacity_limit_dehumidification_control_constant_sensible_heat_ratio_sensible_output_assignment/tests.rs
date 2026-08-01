//! CP388 source boundary, owner, route, IEEE, and corruption tests.

use ep_model::DehumidificationControlType as D;

use super::*;

mod corruption;
mod fixtures;
mod ieee;
mod routes;

#[test]
fn cp388_boundaries_and_physical_four_site_order_are_exact() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2278",
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2279",
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
        &[
            "read-retained-cooling-total-output-for-post-saturation-capacity-limit-constant-sensible-heat-ratio-sensible-output-first-factor",
            "read-purchased-air-cooling-sensible-heat-ratio-for-post-saturation-capacity-limit-constant-sensible-heat-ratio-sensible-output-second-factor",
            "calculate-cooling-total-output-times-cooling-sensible-heat-ratio-for-post-saturation-capacity-limit-constant-sensible-heat-ratio-sensible-output",
            "assign-local-cooling-sensible-output-for-post-saturation-capacity-limit-constant-sensible-heat-ratio-case",
        ],
    );
}

#[test]
fn inactive_route_rejects_owner_payload_atomically() {
    let chain = fixtures::chain(3, 1, true, Some(D::None), 1, 99.0, 50_000.0, 0.008);
    let mut state = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentRuntimeState::new(chain.cp387.system);
    let before = state.clone();
    assert!(
        advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_state(
            &mut state,
            chain.cp387,
            Some(fixtures::input(chain, 0.7)),
        )
        .is_none()
    );
    assert_eq!(state, before);
}
