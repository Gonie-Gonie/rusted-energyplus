use super::super::*;
use crate::ideal_loads::calc::cooling_mixed_air_call::release_tests::release_case_with_demand_and_availability;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot,
    PurchasedAirRuntimeState, advance_direct_no_oa_calc_cooling_mixed_air_call,
    advance_direct_no_oa_calc_cooling_positive_supply_cp_air_assignment,
    advance_direct_no_oa_calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment,
    advance_direct_no_oa_calc_cooling_positive_supply_temperature_assignment,
    advance_direct_no_oa_calc_cooling_positive_supply_temperature_minimum_limit,
    advance_direct_no_oa_calc_cooling_positive_supply_temperature_mixed_air_limit,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_positive_guard,
};
use crate::psychrometrics::energyplus_psy_h_fn_tdb_w;

fn completed_cp335_case() -> (
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot,
) {
    completed_cp335_case_with_demand_availability_and_humidity(-1_000.0, 1.0, None)
}

fn completed_cp335_case_with_demand(
    cooling_demand_w: f64,
) -> (
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot,
) {
    completed_cp335_case_with_demand_availability_and_humidity(cooling_demand_w, 1.0, None)
}

fn completed_cp335_case_with_demand_availability_and_humidity(
    cooling_demand_w: f64,
    overall_availability: f64,
    humidity_ratio: Option<f64>,
) -> (
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot,
) {
    let (mut runtime, system, predecessor, mut zone_state) =
        release_case_with_demand_and_availability(cooling_demand_w, overall_availability);
    if let Some(humidity_ratio) = humidity_ratio {
        zone_state.air_humidity_ratio = humidity_ratio;
    }
    let mixed_air = advance_direct_no_oa_calc_cooling_mixed_air_call(
        &mut runtime,
        &system,
        predecessor,
        &zone_state,
    )
    .expect("CP329");
    let positive_guard = advance_direct_no_oa_calc_cooling_supply_mass_flow_positive_guard(
        &mut runtime,
        &system,
        mixed_air,
    )
    .expect("CP330");
    let cp_air_assignment = advance_direct_no_oa_calc_cooling_positive_supply_cp_air_assignment(
        &mut runtime,
        &system,
        positive_guard,
        &zone_state,
    )
    .expect("CP331");
    let temperature_assignment =
        advance_direct_no_oa_calc_cooling_positive_supply_temperature_assignment(
            &mut runtime,
            &system,
            cp_air_assignment,
            &zone_state,
        )
        .expect("CP332");
    let minimum_limit =
        advance_direct_no_oa_calc_cooling_positive_supply_temperature_minimum_limit(
            &mut runtime,
            &system,
            temperature_assignment,
        )
        .expect("CP333");
    let mixed_air_limit =
        advance_direct_no_oa_calc_cooling_positive_supply_temperature_mixed_air_limit(
            &mut runtime,
            &system,
            minimum_limit,
        )
        .expect("CP334");
    let humidity_assignment =
        advance_direct_no_oa_calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment(
            &mut runtime,
            &system,
            mixed_air_limit,
        )
        .expect("CP335");
    (runtime, system, humidity_assignment)
}

fn assert_rejected_transactionally(
    runtime: &mut PurchasedAirRuntimeState,
    system: &ep_model::IdealLoadsAirSystem,
    predecessor: PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot,
) {
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_positive_supply_enthalpy_assignment(
            runtime,
            system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(*runtime, before);
}

#[test]
fn public_release_reads_only_same_call_cp334_and_cp335_operands_and_rejects_replay() {
    let (mut runtime, system, predecessor) = completed_cp335_case();
    assert!(predecessor.supply_humidity_ratio_mixed_air_assignment_executed);
    let expected_temperature = runtime
        .units
        .get(&system.id)
        .and_then(|unit| {
            unit.calc_cooling_positive_supply_temperature_mixed_air_limit
                .latest
        })
        .and_then(|temperature| temperature.assigned_supply_temperature_c)
        .expect("CP334 supply temperature");
    let expected_humidity = predecessor
        .assigned_supply_humidity_ratio
        .expect("CP335 supply humidity");
    let expected_enthalpy =
        energyplus_psy_h_fn_tdb_w(expected_temperature, expected_humidity);

    let snapshot = advance_direct_no_oa_calc_cooling_positive_supply_enthalpy_assignment(
        &mut runtime,
        &system,
        predecessor,
    )
    .expect("CP336");
    assert!(
        cooling_positive_supply_enthalpy_assignment_snapshot_is_exact_direct_release(snapshot)
    );
    assert_eq!(
        snapshot.supply_temperature_c.map(f64::to_bits),
        Some(expected_temperature.to_bits())
    );
    assert_eq!(
        snapshot.supply_humidity_ratio.map(f64::to_bits),
        Some(expected_humidity.to_bits())
    );
    assert_eq!(
        snapshot
            .psychrometric_supply_enthalpy_result_j_per_kg
            .map(f64::to_bits),
        Some(expected_enthalpy.to_bits())
    );
    assert_eq!(
        snapshot.supply_enthalpy_j_per_kg.map(f64::to_bits),
        Some(expected_enthalpy.to_bits())
    );
    let unit = runtime.units.get(&system.id).expect("known unit");
    let state = &unit.calc_cooling_positive_supply_enthalpy_assignment;
    assert_eq!(state.transition_count, 1);
    assert_eq!(state.supply_enthalpy_assignment_count, 1);
    assert_eq!(state.source_site_execution_count, 4);
    assert_eq!(state.supply_temperature_for_enthalpy_read_count, 1);
    assert_eq!(state.supply_humidity_ratio_for_enthalpy_read_count, 1);
    assert_eq!(state.psychrometric_supply_enthalpy_evaluation_count, 1);
    assert_eq!(state.supply_enthalpy_assignment_write_count, 1);
    assert!(
        completed_direct_cooling_positive_supply_enthalpy_assignment_is_consistent(
            &runtime,
            unit,
            &system,
            snapshot,
            runtime.cooling_positive_supply_enthalpy_assignment_latest_witness(system.id),
        )
    );

    assert_rejected_transactionally(&mut runtime, &system, predecessor);
}

#[test]
fn skipped_routes_never_read_or_project_cp334_or_cp335_operands() {
    for (cooling_demand_w, availability, unit_off, non_cooling, guard_false) in [
        (-1_000.0, 0.0, true, false, false),
        (1.0, 1.0, false, true, false),
        (-1.0e-40, 1.0, false, false, true),
    ] {
        let (mut runtime, system, predecessor) =
            completed_cp335_case_with_demand_availability_and_humidity(
                cooling_demand_w,
                availability,
                None,
            );
        assert!(!predecessor.supply_humidity_ratio_mixed_air_assignment_executed);
        let snapshot = advance_direct_no_oa_calc_cooling_positive_supply_enthalpy_assignment(
            &mut runtime,
            &system,
            predecessor,
        )
        .expect("skipped CP336");
        assert_eq!(snapshot.unit_off_skipped, unit_off);
        assert_eq!(snapshot.non_cooling_skipped, non_cooling);
        assert_eq!(
            snapshot.positive_guard_false_fallthrough_skipped,
            guard_false
        );
        assert!(!snapshot.supply_temperature_for_enthalpy_read);
        assert!(snapshot.supply_temperature_c.is_none());
        assert!(!snapshot.supply_humidity_ratio_for_enthalpy_read);
        assert!(snapshot.supply_humidity_ratio.is_none());
        assert!(!snapshot.psychrometric_supply_enthalpy_evaluated);
        assert!(
            snapshot
                .psychrometric_supply_enthalpy_result_j_per_kg
                .is_none()
        );
        assert!(!snapshot.supply_enthalpy_assigned);
        assert!(snapshot.supply_enthalpy_j_per_kg.is_none());
        assert_eq!(
            runtime
                .units
                .get(&system.id)
                .expect("known unit")
                .calc_cooling_positive_supply_enthalpy_assignment
                .source_site_execution_count,
            0
        );
    }
}

#[test]
fn active_operands_require_exact_same_call_cp334_public_private_pair_and_cp335_humidity() {
    let (runtime, system, predecessor) = completed_cp335_case();
    let temperature = runtime
        .units
        .get(&system.id)
        .expect("known unit")
        .calc_cooling_positive_supply_temperature_mixed_air_limit
        .latest
        .expect("CP334");
    let temperature_witness = runtime
        .cooling_positive_supply_temperature_mixed_air_limit_latest_witness(system.id)
        .expect("private CP334");
    let supply_temperature_c = temperature.assigned_supply_temperature_c;
    let supply_humidity_ratio = predecessor.assigned_supply_humidity_ratio;
    assert!(
        super::super::release::active_operands_link_to_retained_prefix_for_test(
            predecessor,
            temperature,
            temperature_witness,
            supply_temperature_c,
            supply_humidity_ratio,
        )
    );

    let mut stale_temperature = temperature;
    stale_temperature.parent_call_ordinal += 1;
    assert!(
        !super::super::release::active_operands_link_to_retained_prefix_for_test(
            predecessor,
            stale_temperature,
            temperature_witness,
            supply_temperature_c,
            supply_humidity_ratio,
        )
    );
    let mut witness_drift = temperature_witness;
    witness_drift.assigned_supply_temperature_c = witness_drift
        .assigned_supply_temperature_c
        .map(|value| f64::from_bits(value.to_bits() + 1));
    assert!(
        !super::super::release::active_operands_link_to_retained_prefix_for_test(
            predecessor,
            temperature,
            witness_drift,
            supply_temperature_c,
            supply_humidity_ratio,
        )
    );
    assert!(
        !super::super::release::active_operands_link_to_retained_prefix_for_test(
            predecessor,
            temperature,
            temperature_witness,
            supply_temperature_c.map(|value| f64::from_bits(value.to_bits() + 1)),
            supply_humidity_ratio,
        )
    );
    assert!(
        !super::super::release::active_operands_link_to_retained_prefix_for_test(
            predecessor,
            temperature,
            temperature_witness,
            supply_temperature_c,
            supply_humidity_ratio.map(|value| f64::from_bits(value.to_bits() + 1)),
        )
    );
}

#[test]
fn forged_cp335_argument_or_private_witness_fails_without_mutation() {
    let (runtime, system, predecessor) = completed_cp335_case();
    let mut forged_source = predecessor;
    forged_source.source = "forged";
    let mut forged_value = predecessor;
    forged_value.assigned_supply_humidity_ratio = forged_value
        .assigned_supply_humidity_ratio
        .map(|value| f64::from_bits(value.to_bits() + 1));

    for forged in [forged_source, forged_value] {
        let mut case_runtime = runtime.clone();
        assert_rejected_transactionally(&mut case_runtime, &system, forged);
    }

    let mut private_drift_runtime = runtime;
    let mut private_drift = predecessor;
    private_drift.mixed_air_humidity_ratio = private_drift
        .mixed_air_humidity_ratio
        .map(|value| f64::from_bits(value.to_bits() + 1));
    private_drift_runtime
        .set_cooling_positive_supply_humidity_ratio_mixed_air_assignment_latest_witness(
            system.id,
            private_drift,
        );
    assert_rejected_transactionally(&mut private_drift_runtime, &system, predecessor);
}

#[test]
fn cp334_public_or_private_drift_fails_recursively_without_mutation() {
    let (runtime, system, predecessor) = completed_cp335_case();

    let mut public_drift_runtime = runtime.clone();
    let temperature = public_drift_runtime
        .units
        .get_mut(&system.id)
        .expect("known unit")
        .calc_cooling_positive_supply_temperature_mixed_air_limit
        .latest
        .as_mut()
        .expect("CP334");
    temperature.assigned_supply_temperature_c = temperature
        .assigned_supply_temperature_c
        .map(|value| f64::from_bits(value.to_bits() + 1));
    assert_rejected_transactionally(&mut public_drift_runtime, &system, predecessor);

    let mut private_drift_runtime = runtime;
    let mut private_drift = private_drift_runtime
        .cooling_positive_supply_temperature_mixed_air_limit_latest_witness(system.id)
        .expect("private CP334");
    private_drift.minimum_supply_temperature_c = private_drift
        .minimum_supply_temperature_c
        .map(|value| f64::from_bits(value.to_bits() + 1));
    private_drift_runtime.set_cooling_positive_supply_temperature_mixed_air_limit_latest_witness(
        system.id,
        private_drift,
    );
    assert_rejected_transactionally(&mut private_drift_runtime, &system, predecessor);
}

#[test]
fn every_active_counter_overflow_is_preflighted_transactionally() {
    for counter in 0..8 {
        let (mut runtime, system, predecessor) = completed_cp335_case();
        let unit = runtime.units.get_mut(&system.id).expect("known unit");
        let state = &mut unit.calc_cooling_positive_supply_enthalpy_assignment;
        match counter {
            0 => state.transition_count = usize::MAX,
            1 => state.supply_enthalpy_assignment_count = usize::MAX,
            2 => state.source_site_execution_count = usize::MAX,
            3 => state.supply_temperature_for_enthalpy_read_count = usize::MAX,
            4 => state.supply_humidity_ratio_for_enthalpy_read_count = usize::MAX,
            5 => state.psychrometric_supply_enthalpy_evaluation_count = usize::MAX,
            6 => state.supply_enthalpy_assignment_write_count = usize::MAX,
            7 => state.witnessed_supply_enthalpy_assignment_count = usize::MAX,
            _ => unreachable!(),
        }
        assert!(
            !super::super::release::next_supply_enthalpy_assignment_transition_fits_for_test(
                unit,
                predecessor,
            )
        );
        assert_rejected_transactionally(&mut runtime, &system, predecessor);
    }
}

#[test]
fn active_source_site_product_overflow_fails_pending_validation_transactionally() {
    let (mut runtime, system, predecessor) = completed_cp335_case();
    let unit = runtime.units.get_mut(&system.id).expect("known unit");
    let state = &mut unit.calc_cooling_positive_supply_enthalpy_assignment;
    state.supply_enthalpy_assignment_count = usize::MAX / 4 + 1;
    state.source_site_execution_count = 0;
    assert!(
        !super::super::release::pending_supply_enthalpy_assignment_state_is_consistent_for_test(
            unit,
            predecessor,
            None,
        )
    );
    assert_rejected_transactionally(&mut runtime, &system, predecessor);
}

#[test]
fn every_skipped_route_counter_overflow_fails_without_mutation() {
    for (cooling_demand_w, availability, counter) in [
        (1.0, 1.0, 0),
        (-1.0e-40, 1.0, 1),
        (-1.0e-40, 1.0, 2),
        (-1_000.0, 0.0, 3),
    ] {
        let (mut runtime, system, predecessor) =
            completed_cp335_case_with_demand_availability_and_humidity(
                cooling_demand_w,
                availability,
                None,
            );
        let unit = runtime.units.get_mut(&system.id).expect("known unit");
        let state = &mut unit.calc_cooling_positive_supply_enthalpy_assignment;
        match counter {
            0 => state.non_cooling_skip_count = usize::MAX,
            1 => state.positive_guard_false_fallthrough_skip_count = usize::MAX,
            2 => state.witnessed_positive_guard_false_fallthrough_skip_count = usize::MAX,
            3 => state.unit_off_skip_count = usize::MAX,
            _ => unreachable!(),
        }
        assert!(
            !super::super::release::next_supply_enthalpy_assignment_transition_fits_for_test(
                unit,
                predecessor,
            )
        );
        assert_rejected_transactionally(&mut runtime, &system, predecessor);
    }
}

#[test]
fn orphan_public_or_private_cp336_latest_fails_without_mutation() {
    let (runtime, system, predecessor) = completed_cp335_case();
    let temperature = runtime
        .units
        .get(&system.id)
        .and_then(|unit| {
            unit.calc_cooling_positive_supply_temperature_mixed_air_limit
                .latest
        })
        .and_then(|snapshot| snapshot.assigned_supply_temperature_c)
        .expect("CP334 temperature");
    let humidity = predecessor
        .assigned_supply_humidity_ratio
        .expect("CP335 humidity");
    let mut isolated_state =
        PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentRuntimeState::new(system.id);
    let orphan = advance_cooling_positive_supply_enthalpy_assignment_state(
        &mut isolated_state,
        predecessor,
        Some(
            PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentActiveInput {
                supply_temperature_c: temperature,
                supply_humidity_ratio: humidity,
            },
        ),
    );

    let mut public_orphan_runtime = runtime.clone();
    public_orphan_runtime
        .units
        .get_mut(&system.id)
        .expect("known unit")
        .calc_cooling_positive_supply_enthalpy_assignment
        .latest = Some(orphan);
    assert_rejected_transactionally(&mut public_orphan_runtime, &system, predecessor);

    let mut private_orphan_runtime = runtime;
    private_orphan_runtime.set_cooling_positive_supply_enthalpy_assignment_latest_witness(
        system.id, orphan,
    );
    assert_rejected_transactionally(&mut private_orphan_runtime, &system, predecessor);
}

#[test]
fn completed_proof_detects_post_commit_result_and_witness_drift() {
    let (mut runtime, system, predecessor) = completed_cp335_case();
    let snapshot = advance_direct_no_oa_calc_cooling_positive_supply_enthalpy_assignment(
        &mut runtime,
        &system,
        predecessor,
    )
    .expect("CP336");

    let mut public_drift = runtime.clone();
    public_drift
        .units
        .get_mut(&system.id)
        .expect("known unit")
        .calc_cooling_positive_supply_enthalpy_assignment
        .latest
        .as_mut()
        .expect("latest")
        .supply_enthalpy_j_per_kg = snapshot
        .supply_enthalpy_j_per_kg
        .map(|value| f64::from_bits(value.to_bits() + 1));
    assert!(
        !completed_direct_cooling_positive_supply_enthalpy_assignment_is_consistent(
            &public_drift,
            public_drift.units.get(&system.id).expect("known unit"),
            &system,
            snapshot,
            public_drift.cooling_positive_supply_enthalpy_assignment_latest_witness(system.id),
        )
    );

    let mut witness_drift = runtime;
    let mut corrupted_witness = snapshot;
    corrupted_witness.psychrometric_supply_enthalpy_result_j_per_kg = corrupted_witness
        .psychrometric_supply_enthalpy_result_j_per_kg
        .map(|value| f64::from_bits(value.to_bits() + 1));
    witness_drift.set_cooling_positive_supply_enthalpy_assignment_latest_witness(
        system.id,
        corrupted_witness,
    );
    assert!(
        !completed_direct_cooling_positive_supply_enthalpy_assignment_is_consistent(
            &witness_drift,
            witness_drift.units.get(&system.id).expect("known unit"),
            &system,
            snapshot,
            witness_drift.cooling_positive_supply_enthalpy_assignment_latest_witness(system.id),
        )
    );
}

#[test]
fn recursive_cp335_state_corruption_fails_closed_without_mutation() {
    let (mut runtime, system, predecessor) = completed_cp335_case();
    runtime
        .units
        .get_mut(&system.id)
        .expect("known unit")
        .calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment
        .source_site_execution_count += 1;
    assert_rejected_transactionally(&mut runtime, &system, predecessor);
}

#[test]
fn lifecycle_accessor_returns_the_retained_cp336_state() {
    let (mut runtime, system, predecessor) = completed_cp335_case_with_demand(-1_000.0);
    let snapshot = advance_direct_no_oa_calc_cooling_positive_supply_enthalpy_assignment(
        &mut runtime,
        &system,
        predecessor,
    )
    .expect("CP336");
    let lifecycle =
        purchased_air_calc_cooling_positive_supply_enthalpy_assignment_lifecycle_summary(
            &runtime, system.id,
        )
        .expect("lifecycle");
    assert_eq!(
        lifecycle.source,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE
    );
    assert_eq!(
        lifecycle.first_excluded_source,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE
    );
    assert_eq!(lifecycle.state.latest, Some(snapshot));
}
