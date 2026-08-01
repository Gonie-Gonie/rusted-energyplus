//! CP382 public release, retained-owner, and commit-atomicity tests.

use super::super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentError as Error,
    advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment,
    completed_direct_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_is_consistent,
};
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_guard::advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_guard;
use crate::ideal_loads::calc::cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard::completed_cp370_case_with_capacity_limit_for_later_test;
use crate::ideal_loads::{
    advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_guard,
    advance_direct_no_oa_calc_cooling_supply_enthalpy_post_saturation_assignment,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_pre_saturation_original_assignment,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_saturation_assignment,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_saturation_limit_assignment,
    cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_latest_metadata_is_consistent,
    cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_snapshot_is_exact_direct_release,
    purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_lifecycle_summary,
};

#[test]
fn cp382_public_direct_reads_exact_retained_owners_and_assigns_grouped_product() {
    let (mut runtime, system, cp381) = completed_cp381_case(true);
    assert!(
        cp381.dehumidification_body_entered,
        "active fixture unexpectedly took CP381 guard-false route: {cp381:?}",
    );
    let unit = runtime.units.get(&system.id).expect("selected unit");
    let flow = unit
        .calc_cooling_supply_mass_flow_positive_guard
        .latest
        .and_then(|snapshot| snapshot.supply_mass_flow_rate_kg_per_s)
        .expect("CP330 supply-flow owner");
    let mixed = unit
        .calc_cooling_mixed_air_call
        .latest
        .and_then(|snapshot| snapshot.mixed_air_enthalpy_projection_j_per_kg)
        .expect("CP329 mixed-enthalpy owner");
    let supply = unit
        .calc_cooling_supply_enthalpy_post_saturation_assignment
        .latest
        .and_then(|snapshot| snapshot.resulting_supply_enthalpy_j_per_kg)
        .expect("CP379 supply-enthalpy owner");
    let difference = mixed - supply;
    let expected = flow * difference;

    let snapshot = advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment(
        &mut runtime,
        &system,
        cp381,
    )
    .expect("CP382 direct release");
    assert!(
        cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_snapshot_is_exact_direct_release(snapshot)
    );
    assert!(snapshot.dehumidification_total_output_assignment_executed);
    assert!(snapshot.cp330_supply_mass_flow_rate_owned_read);
    assert!(snapshot.cp329_same_call_supply_mass_flow_rate_bit_corroborated);
    assert!(snapshot.cp339_same_call_supply_mass_flow_rate_bit_corroborated);
    assert!(snapshot.cp329_mixed_air_enthalpy_owned_read);
    assert!(snapshot.cp329_same_call_recirculation_enthalpy_bit_corroborated);
    assert!(snapshot.cp339_same_call_mixed_air_enthalpy_bit_corroborated);
    assert!(snapshot.cp379_post_saturation_supply_enthalpy_owned_read);
    assert!(snapshot.cp379_same_call_supply_enthalpy_bits_corroborated);
    assert_eq!(
        snapshot
            .mixed_air_minus_supply_enthalpy_j_per_kg
            .map(f64::to_bits),
        Some(difference.to_bits()),
    );
    assert_eq!(
        snapshot.cooling_total_output_w.map(f64::to_bits),
        Some(expected.to_bits()),
    );

    let unit = runtime.units.get(&system.id).expect("selected unit");
    let witness = runtime
        .cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_latest_witness(system.id);
    assert!(
        completed_direct_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_is_consistent(
            &runtime, unit, &system, snapshot, witness,
        )
    );
    assert!(
        cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_latest_metadata_is_consistent(
            unit, 1,
        )
    );
    let summary = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_lifecycle_summary(
        &runtime,
        system.id,
    )
    .expect("CP382 summary");
    assert_eq!(summary.state.transition_count, 1);
    assert_eq!(summary.state.latest, Some(snapshot));
}

#[test]
fn cp382_public_direct_predecessor_false_routes_skip_all_line_2267_sites() {
    let (mut runtime, system, cp381) = completed_cp381_case(false);
    assert!(!cp381.dehumidification_body_entered);
    let snapshot = advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment(
        &mut runtime,
        &system,
        cp381,
    )
    .expect("CP382 skipped direct release");
    assert!(!snapshot.dehumidification_total_output_assignment_executed);
    assert!(!snapshot.supply_mass_flow_rate_read);
    assert!(snapshot.supply_mass_flow_rate_kg_per_s.is_none());
    assert!(!snapshot.mixed_air_enthalpy_read);
    assert!(snapshot.mixed_air_enthalpy_j_per_kg.is_none());
    assert!(!snapshot.supply_enthalpy_read);
    assert!(snapshot.supply_enthalpy_j_per_kg.is_none());
    assert!(!snapshot.enthalpy_difference_calculated);
    assert!(snapshot.mixed_air_minus_supply_enthalpy_j_per_kg.is_none());
    assert!(!snapshot.cooling_total_output_assigned);
    assert!(snapshot.cooling_total_output_w.is_none());
    let state = &runtime
        .units
        .get(&system.id)
        .expect("selected unit")
        .calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment;
    assert_eq!(state.dehumidification_total_output_assignment_count, 0);
    assert_eq!(state.source_site_execution_count, 0);
}

#[test]
fn cp382_rejects_each_retained_owner_witness_drift_transactionally() {
    for owner in 0..4 {
        let (mut runtime, system, cp381) = completed_cp381_case(true);
        match owner {
            0 => {
                let mut forged = runtime
                    .cooling_supply_mass_flow_positive_guard_latest_witness(system.id)
                    .expect("CP330 witness");
                forged.supply_mass_flow_rate_kg_per_s = forged
                    .supply_mass_flow_rate_kg_per_s
                    .map(|value| f64::from_bits(value.to_bits() ^ 1));
                runtime
                    .set_cooling_supply_mass_flow_positive_guard_latest_witness(system.id, forged);
            }
            1 => {
                let mut forged = runtime
                    .cooling_mixed_air_call_latest_witness(system.id)
                    .expect("CP329 witness");
                forged.mixed_air_enthalpy_projection_j_per_kg = forged
                    .mixed_air_enthalpy_projection_j_per_kg
                    .map(|value| f64::from_bits(value.to_bits() ^ 1));
                runtime.set_cooling_mixed_air_call_latest_witness(system.id, forged);
            }
            2 => {
                let mut forged = runtime
                    .cooling_positive_supply_capacity_limit_sensible_output_assignment_latest_witness(system.id)
                    .expect("CP339 witness");
                forged.mixed_air_enthalpy_j_per_kg = forged
                    .mixed_air_enthalpy_j_per_kg
                    .map(|value| f64::from_bits(value.to_bits() ^ 1));
                runtime.set_cooling_positive_supply_capacity_limit_sensible_output_assignment_latest_witness(
                    system.id,
                    forged,
                );
            }
            3 => {
                let mut forged = runtime
                    .cooling_supply_enthalpy_post_saturation_assignment_latest_witness(system.id)
                    .expect("CP379 witness");
                forged.resulting_supply_enthalpy_j_per_kg = forged
                    .resulting_supply_enthalpy_j_per_kg
                    .map(|value| f64::from_bits(value.to_bits() ^ 1));
                runtime.set_cooling_supply_enthalpy_post_saturation_assignment_latest_witness(
                    system.id, forged,
                );
            }
            _ => unreachable!(),
        }
        let before = runtime.clone();
        let error = advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment(
            &mut runtime,
            &system,
            cp381,
        )
        .expect_err("owner witness drift must fail closed");
        assert!(matches!(
            error,
            Error::CoolingPostSaturationCapacityLimitDehumidificationGuardSnapshotMismatch { .. }
                | Error::ActiveOperandOwnerLineageMismatch { .. }
        ));
        assert_eq!(runtime, before);
    }
}

#[test]
fn cp382_duplicate_public_release_is_rejected_transactionally() {
    let (mut runtime, system, cp381) = completed_cp381_case(true);
    advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment(
        &mut runtime,
        &system,
        cp381,
    )
    .expect("first CP382 release");
    let before = runtime.clone();
    assert!(matches!(
        advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment(
            &mut runtime,
            &system,
            cp381,
        ),
        Err(Error::PredecessorCallOrder { .. } | Error::RuntimeStateInvariantViolation { .. })
    ));
    assert_eq!(runtime, before);
}

fn completed_cp381_case(
    capacity_limit: bool,
) -> (
    crate::ideal_loads::PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardSnapshot,
){
    let (mut runtime, system, cp370) =
        completed_cp370_case_with_capacity_limit_for_later_test(capacity_limit)
            .expect("completed CP370 fixture");
    let cp371 = advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard(
        &mut runtime,
        &system,
        cp370,
    )
    .expect("CP371 direct release");
    let cp372 = advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment(
        &mut runtime,
        &system,
        cp371,
    )
    .expect("CP372 direct release");
    let cp373 = advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment(
        &mut runtime,
        &system,
        cp372,
    )
    .expect("CP373 direct release");
    let cp374 = advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit(
        &mut runtime,
        &system,
        cp373,
    )
    .expect("CP374 direct release");
    let cp375 = advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment(
        &mut runtime,
        &system,
        cp374,
    )
    .expect("CP375 direct release");
    let cp376 =
        advance_direct_no_oa_calc_cooling_supply_humidity_ratio_pre_saturation_original_assignment(
            &mut runtime,
            &system,
            cp375,
        )
        .expect("CP376 direct release");
    let pressure = if capacity_limit { 300_000.0 } else { 101_325.0 };
    let cp377 = advance_direct_no_oa_calc_cooling_supply_humidity_ratio_saturation_assignment(
        &mut runtime,
        &system,
        cp376,
        pressure,
    )
    .expect("CP377 direct release");
    let cp378 =
        advance_direct_no_oa_calc_cooling_supply_humidity_ratio_saturation_limit_assignment(
            &mut runtime,
            &system,
            cp377,
        )
        .expect("CP378 direct release");
    let cp379 = advance_direct_no_oa_calc_cooling_supply_enthalpy_post_saturation_assignment(
        &mut runtime,
        &system,
        cp378,
    )
    .expect("CP379 direct release");
    let cp380 = advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_guard(
        &mut runtime,
        &system,
        cp379,
    )
    .expect("CP380 direct release");
    let cp381 =
        advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_guard(
            &mut runtime,
            &system,
            cp380,
        )
        .expect("CP381 direct release");
    (runtime, system, cp381)
}
