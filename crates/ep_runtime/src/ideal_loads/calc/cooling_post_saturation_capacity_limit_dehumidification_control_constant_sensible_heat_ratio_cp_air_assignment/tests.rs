//! CP387 source boundary, 30-route refinement, psychrometric, and corruption tests.

use ep_model::DehumidificationControlType as D;

use super::*;
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_control_switch::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchActiveInput as Cp386Input,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchRuntimeState as Cp386State,
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_switch_state as advance_cp386,
};
use crate::ideal_loads::calc::cooling_mixed_air_call::{
    PurchasedAirCalcCoolingMixedAirCallRuntimeState as Cp329State,
    advance_cooling_mixed_air_call_state as advance_cp329,
    advance_direct_no_oa_calc_cooling_mixed_air_call,
    release_tests::release_case as cp329_release_case,
    tests::{Route as Cp329Route, active_input as cp329_active_input, predecessor as cp329_predecessor},
};
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
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentActiveOperands as Cp385Operands,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentRetainedInput as Cp385Input,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentRuntimeState as Cp385State,
    advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_state as advance_cp385,
};
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchSnapshot as Predecessor;
use crate::ideal_loads::PurchasedAirCalcCoolingMixedAirCallSnapshot as Cp329Snapshot;
use crate::psychrometrics::energyplus_psy_cp_air_fn_w;

mod routes;

fn predecessor(
    inherited: usize,
    outcome: usize,
    assignment: bool,
    selector: Option<D>,
    ordinal: usize,
) -> Predecessor {
    let cp382 = cp382_predecessor(inherited, outcome, ordinal);
    let mut cp383_state = Cp383State::new(cp382.system);
    let cp383_input = (outcome == 1).then(|| {
        cp383_active_input(cp382, if assignment { 99.0 } else { 100.0 })
            .expect("active CP383 input")
    });
    let cp383 = advance_cp383(&mut cp383_state, cp382, cp383_input).expect("CP383");
    let mut cp384_state = Cp384State::new(cp383.system);
    let cp384 = advance_cp384(&mut cp384_state, cp383).expect("CP384");
    let cp385_input = cp384
        .predecessor_dehumidification_total_output_capacity_guard_evaluated
        .then(|| Cp385Input {
            preexisting_supply_enthalpy_j_per_kg: f64::from_bits(0x40e4_86a0_0000_0001),
            active_operands: cp384
                .dehumidification_total_output_maximum_capacity_assignment_executed
                .then(|| Cp385Operands {
                    mixed_air_enthalpy_j_per_kg: 50_000.0,
                    cooling_total_output_w: cp384
                        .resulting_cooling_total_output_w
                        .expect("CP384 output"),
                    supply_mass_flow_rate_kg_per_s: 2.0,
                }),
        });
    let mut cp385_state = Cp385State::new(cp384.system);
    let cp385 = advance_cp385(&mut cp385_state, cp384, cp385_input).expect("CP385");
    let mut cp386_state = Cp386State::new(cp385.system);
    let cp386_input = assignment.then(|| Cp386Input {
        dehumidification_control_type: selector.expect("active selector"),
    });
    advance_cp386(&mut cp386_state, cp385, cp386_input).expect("CP386")
}

fn owner_for(predecessor: Predecessor) -> Cp329Snapshot {
    let cp328 = cp329_predecessor(Cp329Route::CoolingFallthrough);
    let mut state = Cp329State::new(cp328.system);
    let mut owner = advance_cp329(&mut state, cp328, Some(cp329_active_input(0.25)));
    owner.system = predecessor.system;
    owner.parent_call_ordinal = predecessor.parent_call_ordinal;
    owner.controlled_zone = predecessor.controlled_zone;
    owner
}

const fn input(
    humidity: f64,
) -> PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentActiveInput
{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentActiveInput {
        mixed_air_humidity_ratio: humidity,
    }
}

#[test]
fn cp387_boundaries_and_physical_four_site_order_are_exact() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2273-2277",
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2278",
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_SOURCE_ORDER,
        [
            "enter-purchased-air-post-saturation-capacity-limit-dehumidification-control-constant-sensible-heat-ratio-case",
            "read-purchased-air-mixed-air-humidity-ratio-for-post-saturation-capacity-limit-constant-sensible-heat-ratio-cp-air",
            "evaluate-psy-cp-air-fn-w-for-post-saturation-capacity-limit-constant-sensible-heat-ratio-cp-air",
            "assign-local-cp-air-for-post-saturation-capacity-limit-constant-sensible-heat-ratio-case",
        ],
    );
}

#[test]
fn canonical_cp_air_preserves_signed_zero_operands_without_clamping() {
    for humidity in [0.0, -0.0] {
        let cp386 = predecessor(3, 1, true, Some(D::ConstantSensibleHeatRatio), 1);
        let mut state = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentRuntimeState::new(cp386.system);
        let snapshot = advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_state(
            &mut state,
            cp386,
            Some(input(humidity)),
        )
        .expect("signed zero humidity");
        assert_eq!(
            snapshot.mixed_air_humidity_ratio.map(f64::to_bits),
            Some(humidity.to_bits()),
        );
        assert_eq!(
            snapshot.cp_air_j_per_kg_k.map(f64::to_bits),
            Some(energyplus_psy_cp_air_fn_w(humidity).to_bits()),
        );
    }
}

#[test]
fn active_input_is_derived_only_from_same_call_bit_exact_cp329_owner_evidence() {
    let cp386 = predecessor(3, 1, true, Some(D::ConstantSensibleHeatRatio), 1);
    let owner = owner_for(cp386);
    let active_input = release::active_input_from_owner_for_test(cp386, owner, owner)
        .expect("same-call CP329 owner");
    assert_eq!(
        active_input.mixed_air_humidity_ratio.to_bits(),
        owner
            .mixed_air_humidity_ratio
            .expect("CP329 humidity")
            .to_bits(),
    );

    let mut wrong_call = owner;
    wrong_call.parent_call_ordinal += 1;
    assert!(
        release::active_input_from_owner_for_test(cp386, wrong_call, owner).is_none()
    );
    let mut wrong_identity = owner;
    wrong_identity.controlled_zone = ep_model::ZoneId(owner.controlled_zone.0 + 1);
    assert!(
        release::active_input_from_owner_for_test(cp386, wrong_identity, owner).is_none()
    );
    let mut wrong_bits = owner;
    wrong_bits.mixed_air_humidity_ratio = wrong_bits
        .mixed_air_humidity_ratio
        .map(|value| f64::from_bits(value.to_bits() ^ 1));
    assert!(
        release::active_input_from_owner_for_test(cp386, owner, wrong_bits).is_none()
    );
}

#[test]
fn private_active_characterization_requires_retained_cp329_latest_witness_and_completion() {
    let (mut runtime, system, cp328, zone_state) = cp329_release_case();
    let owner = advance_direct_no_oa_calc_cooling_mixed_air_call(
        &mut runtime,
        &system,
        cp328,
        &zone_state,
    )
    .expect("CP329 owner");
    let mut cp386 = predecessor(3, 1, true, Some(D::ConstantSensibleHeatRatio), 1);
    cp386.system = owner.system;
    cp386.parent_call_ordinal = owner.parent_call_ordinal;
    cp386.controlled_zone = owner.controlled_zone;

    let snapshot = private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_characterization(
        &runtime,
        &system,
        cp386,
    )
    .expect("retained CP329-owned private CP387");
    assert_eq!(
        snapshot.mixed_air_humidity_ratio.map(f64::to_bits),
        owner.mixed_air_humidity_ratio.map(f64::to_bits),
    );

    let mut witness_drift = runtime.clone();
    let mut bad_witness = owner;
    bad_witness.mixed_air_humidity_ratio = bad_witness
        .mixed_air_humidity_ratio
        .map(|value| f64::from_bits(value.to_bits() ^ 1));
    witness_drift.set_cooling_mixed_air_call_latest_witness(owner.system, bad_witness);
    assert!(
        private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_characterization(
            &witness_drift,
            &system,
            cp386,
        )
        .is_none(),
    );

    let mut latest_drift = runtime;
    latest_drift
        .units
        .get_mut(&owner.system)
        .expect("unit")
        .calc_cooling_mixed_air_call
        .latest = Some(bad_witness);
    assert!(
        private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_characterization(
            &latest_drift,
            &system,
            cp386,
        )
        .is_none(),
    );
}

#[test]
fn activity_payload_and_nonfinite_mismatches_are_atomic() {
    let inactive = predecessor(3, 1, true, Some(D::None), 1);
    let active = predecessor(3, 1, true, Some(D::ConstantSensibleHeatRatio), 1);
    for (cp386, active_input) in [
        (inactive, Some(input(0.008))),
        (active, None),
        (active, Some(input(-0.001))),
        (active, Some(input(f64::NAN))),
        (active, Some(input(f64::INFINITY))),
    ] {
        let mut state = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentRuntimeState::new(cp386.system);
        let before = state.clone();
        assert!(
            advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_state(
                &mut state,
                cp386,
                active_input,
            )
            .is_none(),
        );
        assert_eq!(state, before);
    }
}

#[test]
fn cp386_metadata_selector_and_enthalpy_corruption_are_rejected_atomically() {
    let clean = predecessor(3, 1, true, Some(D::ConstantSensibleHeatRatio), 1);
    let mut cases = Vec::new();
    let mut bad_source = clean;
    bad_source.source = "wrong";
    cases.push(bad_source);
    let mut bad_dispatch = clean;
    bad_dispatch.dehumidification_control_switch_dispatched = false;
    cases.push(bad_dispatch);
    let mut bad_selector = clean;
    bad_selector.dehumidification_control_type = Some(D::None);
    cases.push(bad_selector);
    let mut bad_enthalpy = clean;
    bad_enthalpy.resulting_supply_enthalpy_j_per_kg = Some(f64::from_bits(
        clean
            .resulting_supply_enthalpy_j_per_kg
            .expect("active enthalpy")
            .to_bits()
            ^ 1,
    ));
    cases.push(bad_enthalpy);

    for cp386 in cases {
        let mut state = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentRuntimeState::new(cp386.system);
        let before = state.clone();
        assert!(
            advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_state(
                &mut state,
                cp386,
                Some(input(0.008)),
            )
            .is_none(),
        );
        assert_eq!(state, before);
    }
}

#[test]
fn all_ten_inherited_control_flags_are_revalidated_across_u_k_x_f_m_routes() {
    let routes = [
        predecessor(0, 0, false, None, 1),
        predecessor(3, 0, false, None, 1),
        predecessor(3, 2, false, None, 1),
        predecessor(3, 1, false, None, 1),
        predecessor(3, 1, true, Some(D::ConstantSensibleHeatRatio), 1),
    ];
    for cp386 in routes {
        for flag in 0..10 {
            let mut corrupted = cp386;
            match flag {
                0 => corrupted.predecessor_capacity_limit_guard_evaluated =
                    !corrupted.predecessor_capacity_limit_guard_evaluated,
                1 => corrupted.predecessor_capacity_limit_body_entered =
                    !corrupted.predecessor_capacity_limit_body_entered,
                2 => corrupted.predecessor_active_capacity_limit_guard_false_fallthrough =
                    !corrupted.predecessor_active_capacity_limit_guard_false_fallthrough,
                3 => corrupted.predecessor_dehumidification_guard_evaluated =
                    !corrupted.predecessor_dehumidification_guard_evaluated,
                4 => corrupted.predecessor_dehumidification_body_entered =
                    !corrupted.predecessor_dehumidification_body_entered,
                5 => corrupted.predecessor_dehumidification_guard_false_fallthrough =
                    !corrupted.predecessor_dehumidification_guard_false_fallthrough,
                6 => corrupted.predecessor_dehumidification_total_output_assignment_executed =
                    !corrupted.predecessor_dehumidification_total_output_assignment_executed,
                7 => {
                    corrupted.predecessor_dehumidification_total_output_capacity_guard_evaluated =
                        !corrupted
                            .predecessor_dehumidification_total_output_capacity_guard_evaluated;
                }
                8 => {
                    corrupted
                        .predecessor_dehumidification_total_output_capacity_adjustment_body_entered =
                        !corrupted
                            .predecessor_dehumidification_total_output_capacity_adjustment_body_entered;
                }
                9 => {
                    corrupted
                        .predecessor_dehumidification_total_output_capacity_guard_false_fallthrough =
                        !corrupted
                            .predecessor_dehumidification_total_output_capacity_guard_false_fallthrough;
                }
                _ => unreachable!(),
            }
            let active_input = (cp386.dehumidification_control_type
                == Some(D::ConstantSensibleHeatRatio))
            .then_some(input(0.008));
            let mut state = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentRuntimeState::new(corrupted.system);
            assert!(
                advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_state(
                    &mut state,
                    corrupted,
                    active_input,
                )
                .is_none(),
                "route accepted inverted inherited control flag {flag}",
            );
            assert_eq!(state.transition_count, 0);
        }
    }
}

#[test]
fn overflow_rejects_before_mutation() {
    let cp386 = predecessor(3, 1, true, Some(D::ConstantSensibleHeatRatio), 1);
    let mut state = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentRuntimeState::new(cp386.system);
    state.source_site_execution_count = usize::MAX;
    let before = state.clone();
    assert!(
        advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_state(
            &mut state,
            cp386,
            Some(input(0.008)),
        )
        .is_none(),
    );
    assert_eq!(state, before);
}

#[test]
fn cp387_predecessor_validation_uses_bounded_cp386_committed_proof() {
    let source = include_str!("release/prefix_validation.rs");
    assert!(source.contains(
        "cooling_post_saturation_capacity_limit_dehumidification_control_switch_committed_latest_snapshot_is_consistent",
    ));
    assert!(!source.contains(
        "completed_direct_cooling_post_saturation_capacity_limit_dehumidification_control_switch_is_consistent",
    ));
}
