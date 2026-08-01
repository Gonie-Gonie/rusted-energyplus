//! CP389 retained-owner, formula-owner, predecessor, and identity corruption tests.

use super::*;

#[test]
fn cp379_temperature_and_transitive_owner_corruption_are_rejected_atomically() {
    let chain = active_chain();
    let mut changed_temperature = chain.cp379;
    changed_temperature.supply_temperature_c = changed_temperature
        .supply_temperature_c
        .map(|value| value + 1.0);
    assert_rejected(
        chain,
        changed_temperature,
        chain.formula_owners,
        chain.cp388,
    );

    let mut duplicate_owner = chain.cp379;
    duplicate_owner.cp344_capacity_limit_supply_temperature_mixed_air_limit_owned_read = true;
    assert_rejected(chain, duplicate_owner, chain.formula_owners, chain.cp388);
}

#[test]
fn individually_exact_cp379_with_a_different_selector_is_rejected_atomically() {
    let chain = fixtures::chain(
        7,
        1,
        true,
        Some(D::ConstantSupplyHumidityRatio),
        1,
        0.7,
        18.0,
        1.0,
    );
    assert!(
        !chain
            .cp388
            .dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_executed
    );
    assert!(chain.cp388.predecessor_dehumidification_control_type_read);
    assert_eq!(
        chain.cp388.predecessor_dehumidification_control_type,
        Some(D::ConstantSupplyHumidityRatio),
    );
    assert!(
        crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment::cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_snapshot_is_exact(chain.cp388)
    );
    assert!(chain.retained_input().active_owners.is_none());
    let mut wrong_selector = chain.cp379;
    wrong_selector.predecessor_dehumidification_control_type = Some(D::ConstantSensibleHeatRatio);
    assert!(
        crate::ideal_loads::calc::cooling_supply_enthalpy_post_saturation_assignment::cooling_supply_enthalpy_post_saturation_assignment_snapshot_is_exact(wrong_selector)
    );
    let mut state = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentRuntimeState::new(chain.cp388.system);
    let before = state.clone();
    assert!(advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_state(
        &mut state,
        chain.cp388,
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentRetainedInput {
            cp379_temperature_owner: wrong_selector,
            active_owners: None,
        },
    ).is_none());
    assert_eq!(state, before);
}

#[test]
fn cp329_cp330_cp387_and_cp388_bit_drift_are_rejected_atomically() {
    let chain = active_chain();

    let mut mixed = chain.formula_owners;
    mixed.mixed_air_owner.mixed_air_temperature_c = mixed
        .mixed_air_owner
        .mixed_air_temperature_c
        .map(|value| f64::from_bits(value.to_bits() ^ 1));
    assert_rejected(chain, chain.cp379, mixed, chain.cp388);

    let mut flow = chain.formula_owners;
    flow.supply_mass_flow_owner.supply_mass_flow_rate_kg_per_s = Some(2.0);
    assert_rejected(chain, chain.cp379, flow, chain.cp388);

    let mut cp_air = chain.formula_owners;
    cp_air.cp_air_owner.cp_air_j_per_kg_k = cp_air
        .cp_air_owner
        .cp_air_j_per_kg_k
        .map(|value| f64::from_bits(value.to_bits() ^ 1));
    assert_rejected(chain, chain.cp379, cp_air, chain.cp388);

    let mut predecessor = chain.cp388;
    predecessor.cooling_sensible_output_w = predecessor
        .cooling_sensible_output_w
        .map(|value| f64::from_bits(value.to_bits() ^ 1));
    assert_rejected(chain, chain.cp379, chain.formula_owners, predecessor);
}

#[test]
fn individually_exact_cp329_and_cp330_from_different_branches_are_rejected_atomically() {
    let chain = active_chain();
    let mut crossed = chain.formula_owners;
    crossed
        .mixed_air_owner
        .predecessor_zero_flow_reset_body_entered = true;
    crossed
        .mixed_air_owner
        .predecessor_active_guard_false_fallthrough = false;
    assert!(
        crate::ideal_loads::cooling_mixed_air_call_snapshot_is_exact_direct_release(
            crossed.mixed_air_owner,
        )
    );
    assert!(
        crate::ideal_loads::cooling_supply_mass_flow_positive_guard_snapshot_is_exact_direct_release(
            crossed.supply_mass_flow_owner,
        )
    );
    assert_rejected(chain, chain.cp379, crossed, chain.cp388);
}

#[test]
fn missing_active_owner_and_wrong_cp379_identity_are_rejected() {
    let chain = active_chain();
    assert!(private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_characterization(
        chain.cp388,
        chain.cp379,
        None,
        Some(chain.formula_owners.supply_mass_flow_owner),
        Some(chain.cp387),
    ).is_none());
    let mut owner = chain.cp379;
    owner.parent_call_ordinal += 1;
    assert_rejected(chain, owner, chain.formula_owners, chain.cp388);
}

fn active_chain() -> fixtures::Chain {
    fixtures::chain(
        3,
        1,
        true,
        Some(D::ConstantSensibleHeatRatio),
        1,
        0.7,
        18.0,
        1.0,
    )
}

fn assert_rejected(
    chain: fixtures::Chain,
    cp379: crate::ideal_loads::PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentSnapshot,
    owners: PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentActiveOwners,
    predecessor: crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentSnapshot,
) {
    let mut state = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentRuntimeState::new(chain.cp388.system);
    let before = state.clone();
    assert!(advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_state(
        &mut state,
        predecessor,
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentRetainedInput {
            cp379_temperature_owner: cp379,
            active_owners: Some(owners),
        },
    ).is_none());
    assert_eq!(state, before);
}
