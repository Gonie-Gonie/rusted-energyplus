//! CP388 owner, bridge, predecessor, and identity corruption tests.

use super::*;

#[test]
fn cp384_call_zone_value_and_inherited_flag_drift_are_rejected() {
    let chain = fixtures::chain(3, 1, true, Some(D::ConstantSensibleHeatRatio), 1, 99.0, 50_000.0, 0.008);
    let system = fixtures::selected_system(chain, 0.7);

    let mut wrong_call = chain.cp384;
    wrong_call.parent_call_ordinal += 1;
    assert_rejected(&system, chain, wrong_call, chain.cp385);

    let mut wrong_zone = chain.cp384;
    wrong_zone.controlled_zone = ep_model::ZoneId(u32::MAX - 4);
    assert_rejected(&system, chain, wrong_zone, chain.cp385);

    let mut wrong_value = chain.cp384;
    let changed = wrong_value
        .resulting_cooling_total_output_w
        .map(|value| f64::from_bits(value.to_bits() ^ 1));
    wrong_value.maximum_total_cooling_capacity_w = changed;
    wrong_value.assigned_cooling_total_output_w = changed;
    wrong_value.resulting_cooling_total_output_w = changed;
    assert_rejected(&system, chain, wrong_value, chain.cp385);

    let mut wrong_flag = chain.cp384;
    wrong_flag.heating_availability_guard_false_fallthrough = false;
    wrong_flag.humidification_control_guard_false_fallthrough = true;
    assert_rejected(&system, chain, wrong_flag, chain.cp385);
}

#[test]
fn cp385_bridge_and_cp387_predecessor_corruption_are_rejected_atomically() {
    let chain = fixtures::chain(7, 1, true, Some(D::ConstantSensibleHeatRatio), 1, 99.0, 50_000.0, 0.012);
    let system = fixtures::selected_system(chain, 0.65);

    let mut bridge = chain.cp385;
    bridge.cooling_total_output_w = bridge
        .cooling_total_output_w
        .map(|value| f64::from_bits(value.to_bits() ^ 1));
    assert_rejected(&system, chain, chain.cp384, bridge);

    let mut wrong_cp387 = chain.cp387;
    wrong_cp387.cp_air_j_per_kg_k = wrong_cp387
        .cp_air_j_per_kg_k
        .map(|value| f64::from_bits(value.to_bits() ^ 1));
    assert!(private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_characterization(
        &system,
        wrong_cp387,
        Some(chain.cp384),
        Some(chain.cp385),
    ).is_none());
}

#[test]
fn model_identity_selector_and_owner_absence_are_rejected() {
    let chain = fixtures::chain(3, 1, true, Some(D::ConstantSensibleHeatRatio), 1, 99.0, 50_000.0, 0.008);
    let mut system = fixtures::selected_system(chain, 0.7);
    system.id = ep_model::IdealLoadsAirSystemId(u32::MAX - 3);
    assert!(private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_characterization(
        &system, chain.cp387, Some(chain.cp384), Some(chain.cp385),
    ).is_none());
    system.id = chain.cp387.system;
    system.dehumidification_control_type = D::None;
    assert!(private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_characterization(
        &system, chain.cp387, Some(chain.cp384), Some(chain.cp385),
    ).is_none());
    system.dehumidification_control_type = D::ConstantSensibleHeatRatio;
    assert!(private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_characterization(
        &system, chain.cp387, None, Some(chain.cp385),
    ).is_none());
    assert!(private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_characterization(
        &system, chain.cp387, Some(chain.cp384), None,
    ).is_none());
}

fn assert_rejected(
    system: &ep_model::IdealLoadsAirSystem,
    chain: fixtures::Chain,
    owner: crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputMaximumCapacityAssignmentSnapshot,
    bridge: crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentSnapshot,
) {
    assert!(private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_characterization(
        system,
        chain.cp387,
        Some(owner),
        Some(bridge),
    ).is_none());
}
