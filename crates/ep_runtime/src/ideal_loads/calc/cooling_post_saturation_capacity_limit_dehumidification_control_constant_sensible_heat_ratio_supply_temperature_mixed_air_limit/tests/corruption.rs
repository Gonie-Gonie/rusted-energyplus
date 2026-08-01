//! CP390 fail-closed corruption tests.

use super::*;
use crate::ideal_loads::calc::cooling_positive_supply_temperature_mixed_air_limit::source_shaped_two_argument_minimum;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingMixedAirCallSnapshot as Cp329,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentSnapshot as Cp389,
};

#[test]
fn missing_active_owner_and_inactive_owner_payload_are_rejected_atomically() {
    let active = active_chain();
    assert_rejected(active.cp389, None);

    let inactive = fixtures::chain(3, 1, true, Some(D::Humidistat), 1, 0.7, 18.0, 1.0);
    assert!(!inactive.cp389.dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_executed);
    assert_rejected(inactive.cp389, Some(inactive.mixed_air_owner));
}

#[test]
fn cp389_drift_and_cp329_identity_drift_are_rejected_atomically() {
    let chain = active_chain();
    let mut predecessor = chain.cp389;
    predecessor.resulting_supply_temperature_c = predecessor
        .resulting_supply_temperature_c
        .map(|value| f64::from_bits(value.to_bits() ^ 1));
    assert_rejected(predecessor, Some(chain.mixed_air_owner));

    let mut wrong_identity = chain.mixed_air_owner;
    wrong_identity.parent_call_ordinal += 1;
    assert_rejected(chain.cp389, Some(wrong_identity));
}

#[test]
fn individually_exact_but_different_cp329_mixed_bits_are_rejected_atomically() {
    let chain = active_chain();
    let alternate = fixtures::alternate_exact_mixed_air_owner(chain, 24.5);
    assert!(crate::ideal_loads::cooling_mixed_air_call_snapshot_is_exact_direct_release(alternate));
    assert_ne!(
        alternate.mixed_air_temperature_c.map(f64::to_bits),
        chain.cp389.mixed_air_temperature_c.map(f64::to_bits),
    );
    assert_rejected(chain.cp389, Some(alternate));
}

#[test]
fn nonfinite_cp329_right_is_rejected_by_the_existing_exact_owner_validator() {
    let chain = active_chain();
    let mut owner = chain.mixed_air_owner;
    owner.mixed_air_temperature_c = Some(f64::from_bits(0x7ff8_0000_0000_0042));
    assert!(!crate::ideal_loads::cooling_mixed_air_call_snapshot_is_exact_direct_release(owner));
    assert_rejected(chain.cp389, Some(owner));
}

#[test]
fn exact_snapshot_validator_rejects_bit_coherent_nonfinite_right_operands() {
    for right in [
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::from_bits(0x7ff8_0000_0000_0390),
    ] {
        let mut snapshot = characterize(active_chain());
        assert!(
            cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_snapshot_is_exact(
                snapshot,
            )
        );
        let drop = snapshot
            .predecessor_cooling_sensible_output_over_air_capacity_rate_k
            .expect("CP389 temperature drop");
        let left = right - drop;
        let minimum = source_shaped_two_argument_minimum(left, right);

        snapshot.predecessor_mixed_air_temperature_c = Some(right);
        snapshot.predecessor_calculated_supply_temperature_c = Some(left);
        snapshot.predecessor_assigned_supply_temperature_c = Some(left);
        snapshot.predecessor_resulting_supply_temperature_c = Some(left);
        snapshot.preexisting_supply_temperature_c = Some(left);
        snapshot.supply_temperature_before_mixed_air_limit_c = Some(left);
        snapshot.mixed_air_temperature_c = Some(right);
        snapshot.minimum_supply_temperature_c = Some(minimum);
        snapshot.assigned_supply_temperature_c = Some(minimum);
        snapshot.resulting_supply_temperature_c = Some(minimum);

        assert!(
            !cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_snapshot_is_exact(
                snapshot,
            ),
            "non-finite right operand {:#018x} must fail CP390 exact validation",
            right.to_bits(),
        );
    }
}

#[test]
fn exact_snapshot_validator_rejects_local_and_carried_bit_drift() {
    let chain = active_chain();
    let snapshot = characterize(chain);
    let mut minimum = snapshot;
    minimum.minimum_supply_temperature_c = minimum
        .minimum_supply_temperature_c
        .map(|value| f64::from_bits(value.to_bits() ^ 1));
    assert!(!cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_snapshot_is_exact(minimum));

    let mut enthalpy = snapshot;
    enthalpy.resulting_supply_enthalpy_j_per_kg = enthalpy
        .resulting_supply_enthalpy_j_per_kg
        .map(|value| f64::from_bits(value.to_bits() ^ 1));
    assert!(!cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_snapshot_is_exact(enthalpy));
}

#[test]
fn bit_exact_matcher_detects_signed_zero_and_nan_payload_drift() {
    let snapshot = characterize(active_chain());
    let mut positive_zero = snapshot;
    let mut negative_zero = snapshot;
    positive_zero.resulting_supply_temperature_c = Some(0.0);
    negative_zero.resulting_supply_temperature_c = Some(-0.0);
    assert!(!cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_snapshots_match_bit_exact(
        positive_zero,
        negative_zero,
    ));

    let mut nan_a = snapshot;
    let mut nan_b = snapshot;
    nan_a.minimum_supply_temperature_c = Some(f64::from_bits(0x7ff8_0000_0000_0042));
    nan_b.minimum_supply_temperature_c = Some(f64::from_bits(0x7ff8_0000_0000_0099));
    assert!(!cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_snapshots_match_bit_exact(
        nan_a,
        nan_b,
    ));
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

fn characterize(
    chain: fixtures::Chain,
) -> PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureMixedAirLimitSnapshot{
    private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_characterization(
        chain.cp389,
        Some(chain.mixed_air_owner),
    )
    .expect("private CP390")
}

fn assert_rejected(predecessor: Cp389, owner: Option<Cp329>) {
    let mut state = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureMixedAirLimitRuntimeState::new(predecessor.system);
    let before = state.clone();
    assert!(advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_state(
        &mut state,
        predecessor,
        owner,
    ).is_none());
    assert_eq!(state, before);
}
