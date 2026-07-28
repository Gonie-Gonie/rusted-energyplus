use super::super::*;
use crate::ideal_loads::calc::cooling_mixed_air_call::release_tests::release_case_with_demand_and_availability;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot, PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_mixed_air_call,
    advance_direct_no_oa_calc_cooling_positive_supply_cp_air_assignment,
    advance_direct_no_oa_calc_cooling_positive_supply_enthalpy_assignment,
    advance_direct_no_oa_calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment,
    advance_direct_no_oa_calc_cooling_positive_supply_temperature_assignment,
    advance_direct_no_oa_calc_cooling_positive_supply_temperature_minimum_limit,
    advance_direct_no_oa_calc_cooling_positive_supply_temperature_mixed_air_limit,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_positive_guard,
};

fn completed_cp336_case() -> (
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
) {
    completed_cp336_case_with_demand_and_availability(-1_000.0, 1.0)
}

fn completed_cp336_case_with_demand_and_availability(
    cooling_demand_w: f64,
    overall_availability: f64,
) -> (
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
) {
    let (mut runtime, system, predecessor, zone_state) =
        release_case_with_demand_and_availability(cooling_demand_w, overall_availability);
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
    let enthalpy_assignment =
        advance_direct_no_oa_calc_cooling_positive_supply_enthalpy_assignment(
            &mut runtime,
            &system,
            humidity_assignment,
        )
        .expect("CP336");
    (runtime, system, enthalpy_assignment)
}

fn assert_rejected_transactionally(
    runtime: &mut PurchasedAirRuntimeState,
    system: &ep_model::IdealLoadsAirSystem,
    predecessor: PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
) {
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_guard(
            runtime,
            system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(*runtime, before);
}

#[test]
fn public_release_commits_once_and_rejects_replay_without_mutation() {
    let (mut runtime, system, predecessor) = completed_cp336_case();
    assert!(predecessor.supply_enthalpy_assignment_executed);
    assert_eq!(system.cooling_limit, ep_model::IdealLoadsLimit::NoLimit);

    let snapshot =
        advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_guard(
            &mut runtime,
            &system,
            predecessor,
        )
        .expect("CP337");
    assert!(
        cooling_positive_supply_capacity_limit_guard_snapshot_is_exact_direct_release(snapshot)
    );
    assert_eq!(snapshot.first_cooling_limit, Some(system.cooling_limit));
    assert_eq!(
        snapshot.second_cooling_limit,
        Some(system.cooling_limit)
    );
    assert!(snapshot.cooling_limit_rejected);
    assert!(snapshot.active_guard_false_fallthrough);
    assert!(!snapshot.capacity_limit_body_entered);

    let unit = runtime.units.get(&system.id).expect("known unit");
    assert!(
        completed_direct_cooling_positive_supply_capacity_limit_guard_is_consistent(
            &runtime,
            unit,
            &system,
            snapshot,
            runtime.cooling_positive_supply_capacity_limit_guard_latest_witness(system.id),
        )
    );
    let before_replay = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_guard(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before_replay);
}

#[test]
fn all_inherited_skip_routes_commit_without_reading_the_selector() {
    for (demand, availability, unit_off, non_cooling, guard_false) in [
        (1.0, 1.0, false, true, false),
        (-1.0e-40, 1.0, false, false, true),
        (-1_000.0, 0.0, true, false, false),
    ] {
        let (mut runtime, system, predecessor) =
            completed_cp336_case_with_demand_and_availability(demand, availability);
        let snapshot =
            advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_guard(
                &mut runtime,
                &system,
                predecessor,
            )
            .expect("skipped CP337");

        assert_eq!(snapshot.unit_off_skipped, unit_off);
        assert_eq!(snapshot.non_cooling_skipped, non_cooling);
        assert_eq!(
            snapshot.positive_guard_false_fallthrough_skipped,
            guard_false
        );
        assert!(!snapshot.capacity_limit_guard_evaluated);
        assert!(snapshot.first_cooling_limit.is_none());
        assert!(snapshot.second_cooling_limit.is_none());
        assert_eq!(
            runtime
                .units
                .get(&system.id)
                .expect("known unit")
                .calc_cooling_positive_supply_capacity_limit_guard
                .source_site_execution_count,
            0
        );
    }
}

#[test]
fn supplied_retained_or_private_cp336_drift_is_rejected_before_cp337_mutation() {
    let (runtime, system, predecessor) = completed_cp336_case();

    let mut supplied_source = predecessor;
    supplied_source.source = "forged";
    let mut case_runtime = runtime.clone();
    assert_rejected_transactionally(&mut case_runtime, &system, supplied_source);

    let mut supplied_bits = predecessor;
    supplied_bits.supply_temperature_c = supplied_bits
        .supply_temperature_c
        .map(|value| f64::from_bits(value.to_bits() + 1));
    let mut case_runtime = runtime.clone();
    assert_rejected_transactionally(&mut case_runtime, &system, supplied_bits);

    let mut retained_runtime = runtime.clone();
    retained_runtime
        .units
        .get_mut(&system.id)
        .expect("known unit")
        .calc_cooling_positive_supply_enthalpy_assignment
        .latest
        .as_mut()
        .expect("CP336 latest")
        .source = "retained-forgery";
    assert_rejected_transactionally(&mut retained_runtime, &system, predecessor);

    let mut private_runtime = runtime;
    let mut private = private_runtime
        .cooling_positive_supply_enthalpy_assignment_latest_witness(system.id)
        .expect("private CP336");
    private.supply_enthalpy_j_per_kg = private
        .supply_enthalpy_j_per_kg
        .map(|value| f64::from_bits(value.to_bits() + 1));
    private_runtime
        .set_cooling_positive_supply_enthalpy_assignment_latest_witness(system.id, private);
    assert_rejected_transactionally(&mut private_runtime, &system, predecessor);
}

#[test]
fn recursive_cp335_cp334_and_same_call_selector_corruption_fail_closed() {
    let (runtime, system, predecessor) = completed_cp336_case();

    let mut cp335_runtime = runtime.clone();
    cp335_runtime
        .units
        .get_mut(&system.id)
        .expect("known unit")
        .calc_cooling_positive_supply_humidity_ratio_mixed_air_assignment
        .source_site_execution_count += 1;
    assert_rejected_transactionally(&mut cp335_runtime, &system, predecessor);

    let mut cp334_runtime = runtime.clone();
    cp334_runtime
        .units
        .get_mut(&system.id)
        .expect("known unit")
        .calc_cooling_positive_supply_temperature_mixed_air_limit
        .source_site_execution_count += 1;
    assert_rejected_transactionally(&mut cp334_runtime, &system, predecessor);

    let mut cp321_runtime = runtime.clone();
    cp321_runtime
        .units
        .get_mut(&system.id)
        .expect("known unit")
        .calc_cooling_capacity_zero_flow_reset
        .latest
        .as_mut()
        .expect("CP321")
        .first_cooling_limit = Some(ep_model::IdealLoadsLimit::LimitCapacity);
    assert_rejected_transactionally(&mut cp321_runtime, &system, predecessor);

    let mut cp325_runtime = runtime;
    cp325_runtime
        .units
        .get_mut(&system.id)
        .expect("known unit")
        .calc_cooling_supply_mass_flow_limit_guard
        .latest
        .as_mut()
        .expect("CP325")
        .first_cooling_limit = Some(ep_model::IdealLoadsLimit::LimitCapacity);
    assert_rejected_transactionally(&mut cp325_runtime, &system, predecessor);
}

#[test]
fn post_initialization_selector_mutation_is_rejected_without_mutation() {
    let (mut runtime, mut system, predecessor) = completed_cp336_case();
    system.cooling_limit = ep_model::IdealLoadsLimit::LimitCapacity;
    system.maximum_total_cooling_capacity_w =
        Some(ep_model::AutosizeOrNumber::Value(1_000.0));
    assert_rejected_transactionally(&mut runtime, &system, predecessor);
}

#[test]
fn every_active_counter_overflow_is_preflighted_transactionally() {
    for (counter, preflight_limit) in [
        (0, ep_model::IdealLoadsLimit::NoLimit),
        (1, ep_model::IdealLoadsLimit::NoLimit),
        (2, ep_model::IdealLoadsLimit::NoLimit),
        (3, ep_model::IdealLoadsLimit::NoLimit),
        (4, ep_model::IdealLoadsLimit::NoLimit),
        (5, ep_model::IdealLoadsLimit::LimitCapacity),
        (6, ep_model::IdealLoadsLimit::NoLimit),
        (7, ep_model::IdealLoadsLimit::NoLimit),
        (
            8,
            ep_model::IdealLoadsLimit::LimitFlowRateAndCapacity,
        ),
        (9, ep_model::IdealLoadsLimit::NoLimit),
        (10, ep_model::IdealLoadsLimit::LimitCapacity),
        (11, ep_model::IdealLoadsLimit::NoLimit),
        (12, ep_model::IdealLoadsLimit::NoLimit),
    ] {
        let (mut runtime, system, predecessor) = completed_cp336_case();
        let unit = runtime.units.get_mut(&system.id).expect("known unit");
        let state = &mut unit.calc_cooling_positive_supply_capacity_limit_guard;
        match counter {
            0 => state.transition_count = usize::MAX,
            1 => state.capacity_limit_guard_evaluation_count = usize::MAX,
            2 => state.source_site_execution_count = usize::MAX,
            3 => state.first_cooling_limit_read_count = usize::MAX,
            4 => state.cooling_limit_capacity_comparison_count = usize::MAX,
            5 => state.cooling_limit_capacity_match_count = usize::MAX,
            6 => state.second_cooling_limit_read_count = usize::MAX,
            7 => state.cooling_limit_flow_rate_and_capacity_comparison_count = usize::MAX,
            8 => state.cooling_limit_flow_rate_and_capacity_match_count = usize::MAX,
            9 => state.cooling_limit_rejected_count = usize::MAX,
            10 => state.capacity_limit_body_entry_count = usize::MAX,
            11 => state.active_guard_false_fallthrough_count = usize::MAX,
            12 => state.witnessed_active_guard_false_fallthrough_count = usize::MAX,
            _ => unreachable!(),
        }
        assert!(
            !super::super::release::next_capacity_limit_guard_transition_fits_for_test(
                unit,
                predecessor,
                preflight_limit,
            )
        );
        assert_rejected_transactionally(&mut runtime, &system, predecessor);
    }
}

#[test]
fn body_and_positive_skip_witness_overflows_are_preflighted() {
    let (mut body_runtime, body_system, body_predecessor) = completed_cp336_case();
    let body_unit = body_runtime
        .units
        .get_mut(&body_system.id)
        .expect("known unit");
    body_unit
        .calc_cooling_positive_supply_capacity_limit_guard
        .witnessed_capacity_limit_body_entry_count = usize::MAX;
    assert!(
        !super::super::release::next_capacity_limit_guard_transition_fits_for_test(
            body_unit,
            body_predecessor,
            ep_model::IdealLoadsLimit::LimitCapacity,
        )
    );
    assert_rejected_transactionally(&mut body_runtime, &body_system, body_predecessor);

    let (mut skip_runtime, skip_system, skip_predecessor) =
        completed_cp336_case_with_demand_and_availability(-1.0e-40, 1.0);
    assert!(skip_predecessor.positive_guard_false_fallthrough_skipped);
    let skip_unit = skip_runtime
        .units
        .get_mut(&skip_system.id)
        .expect("known unit");
    skip_unit
        .calc_cooling_positive_supply_capacity_limit_guard
        .positive_guard_false_fallthrough_skip_count = usize::MAX;
    assert!(
        !super::super::release::next_capacity_limit_guard_transition_fits_for_test(
            skip_unit,
            skip_predecessor,
            skip_system.cooling_limit,
        )
    );
    assert_rejected_transactionally(&mut skip_runtime, &skip_system, skip_predecessor);
}

#[test]
fn every_inherited_skip_counter_overflow_is_transactional() {
    for (demand, availability, counter) in [
        (1.0, 1.0, 0),
        (-1.0e-40, 1.0, 1),
        (-1.0e-40, 1.0, 2),
        (-1_000.0, 0.0, 3),
    ] {
        let (mut runtime, system, predecessor) =
            completed_cp336_case_with_demand_and_availability(demand, availability);
        let unit = runtime.units.get_mut(&system.id).expect("known unit");
        let state = &mut unit.calc_cooling_positive_supply_capacity_limit_guard;
        match counter {
            0 => state.non_cooling_skip_count = usize::MAX,
            1 => state.positive_guard_false_fallthrough_skip_count = usize::MAX,
            2 => state.witnessed_positive_guard_false_fallthrough_skip_count = usize::MAX,
            3 => state.unit_off_skip_count = usize::MAX,
            _ => unreachable!(),
        }
        assert!(
            !super::super::release::next_capacity_limit_guard_transition_fits_for_test(
                unit,
                predecessor,
                system.cooling_limit,
            )
        );
        assert_rejected_transactionally(&mut runtime, &system, predecessor);
    }
}

#[test]
fn source_site_product_and_redundant_false_corruption_are_transactional() {
    let (mut product_runtime, product_system, product_predecessor) = completed_cp336_case();
    {
        let state = &mut product_runtime
            .units
            .get_mut(&product_system.id)
            .expect("known unit")
            .calc_cooling_positive_supply_capacity_limit_guard;
        state.capacity_limit_guard_evaluation_count = usize::MAX / 2 + 1;
        state.source_site_execution_count = 0;
    }
    let unit = product_runtime
        .units
        .get(&product_system.id)
        .expect("known unit");
    assert!(
        !super::super::release::pending_capacity_limit_guard_state_is_consistent_for_test(
            unit,
            &product_system,
            product_predecessor,
            None,
        )
    );
    assert_rejected_transactionally(
        &mut product_runtime,
        &product_system,
        product_predecessor,
    );

    let (mut coordinated, system, predecessor) = completed_cp336_case();
    {
        let state = &mut coordinated
            .units
            .get_mut(&system.id)
            .expect("known unit")
            .calc_cooling_positive_supply_capacity_limit_guard;
        state.cooling_limit_rejected_count = 1;
        state.active_guard_false_fallthrough_count = 1;
        state.witnessed_active_guard_false_fallthrough_count = 1;
    }
    assert_rejected_transactionally(&mut coordinated, &system, predecessor);
}

#[test]
fn orphan_public_or_private_cp337_latest_is_rejected_without_mutation() {
    let (runtime, system, predecessor) = completed_cp336_case();
    let mut seed_state =
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardRuntimeState::new(system.id);
    let orphan = advance_cooling_positive_supply_capacity_limit_guard_state(
        &mut seed_state,
        predecessor,
        Some(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardActiveInput {
                cooling_limit: system.cooling_limit,
            },
        ),
    );

    let mut public_orphan = runtime.clone();
    public_orphan
        .units
        .get_mut(&system.id)
        .expect("known unit")
        .calc_cooling_positive_supply_capacity_limit_guard
        .latest = Some(orphan);
    assert_rejected_transactionally(&mut public_orphan, &system, predecessor);

    let mut private_orphan = runtime;
    private_orphan
        .set_cooling_positive_supply_capacity_limit_guard_latest_witness(system.id, orphan);
    assert_rejected_transactionally(&mut private_orphan, &system, predecessor);
}

#[test]
fn completed_proof_detects_post_commit_public_and_private_drift() {
    let (mut runtime, system, predecessor) = completed_cp336_case();
    let snapshot =
        advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_guard(
            &mut runtime,
            &system,
            predecessor,
        )
        .expect("CP337");

    let mut public_drift = runtime.clone();
    public_drift
        .units
        .get_mut(&system.id)
        .expect("known unit")
        .calc_cooling_positive_supply_capacity_limit_guard
        .latest
        .as_mut()
        .expect("CP337 latest")
        .cooling_limit_rejected = false;
    let unit = public_drift.units.get(&system.id).expect("known unit");
    assert!(
        !completed_direct_cooling_positive_supply_capacity_limit_guard_is_consistent(
            &public_drift,
            unit,
            &system,
            snapshot,
            public_drift
                .cooling_positive_supply_capacity_limit_guard_latest_witness(system.id),
        )
    );

    let mut private_drift = runtime;
    let mut witness = private_drift
        .cooling_positive_supply_capacity_limit_guard_latest_witness(system.id)
        .expect("private CP337");
    witness.active_guard_false_fallthrough = false;
    private_drift
        .set_cooling_positive_supply_capacity_limit_guard_latest_witness(system.id, witness);
    let unit = private_drift.units.get(&system.id).expect("known unit");
    assert!(
        !completed_direct_cooling_positive_supply_capacity_limit_guard_is_consistent(
            &private_drift,
            unit,
            &system,
            snapshot,
            private_drift
                .cooling_positive_supply_capacity_limit_guard_latest_witness(system.id),
        )
    );
}

#[test]
fn coordinated_public_state_and_private_witness_selector_forgery_cannot_bypass_lineage() {
    let (mut runtime, system, predecessor) = completed_cp336_case();
    let snapshot =
        advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_guard(
            &mut runtime,
            &system,
            predecessor,
        )
        .expect("CP337");
    assert_eq!(snapshot.first_cooling_limit, Some(ep_model::IdealLoadsLimit::NoLimit));

    let mut forged = snapshot;
    forged.first_cooling_limit = Some(ep_model::IdealLoadsLimit::LimitCapacity);
    forged.cooling_limit_capacity = Some(true);
    forged.second_cooling_limit_read = false;
    forged.second_cooling_limit = None;
    forged.cooling_limit_flow_rate_and_capacity_comparison_evaluated = false;
    forged.cooling_limit_flow_rate_and_capacity = None;
    forged.cooling_limit_condition_satisfied = Some(true);
    forged.cooling_limit_rejected = false;
    forged.capacity_limit_body_entered = true;
    forged.active_guard_false_fallthrough = false;
    assert!(
        cooling_positive_supply_capacity_limit_guard_snapshot_is_exact_direct_release(forged)
    );

    {
        let state = &mut runtime
            .units
            .get_mut(&system.id)
            .expect("known unit")
            .calc_cooling_positive_supply_capacity_limit_guard;
        state.source_site_execution_count = 3;
        state.cooling_limit_capacity_match_count = 1;
        state.second_cooling_limit_read_count = 0;
        state.cooling_limit_flow_rate_and_capacity_comparison_count = 0;
        state.cooling_limit_rejected_count = 0;
        state.capacity_limit_body_entry_count = 1;
        state.active_guard_false_fallthrough_count = 0;
        state.latest = Some(forged);
        state.latest_route = Some(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardRetainedRoute::
                CapacityLimitBodyEntered,
        );
        state.witnessed_capacity_limit_body_entry_count = 1;
        state.witnessed_active_guard_false_fallthrough_count = 0;
    }
    runtime.set_cooling_positive_supply_capacity_limit_guard_latest_witness(system.id, forged);

    let unit = runtime.units.get(&system.id).expect("known unit");
    assert!(
        !completed_direct_cooling_positive_supply_capacity_limit_guard_is_consistent(
            &runtime,
            unit,
            &system,
            forged,
            runtime.cooling_positive_supply_capacity_limit_guard_latest_witness(system.id),
        )
    );
}

#[test]
fn lifecycle_accessor_returns_the_retained_cp337_state() {
    let (mut runtime, system, predecessor) = completed_cp336_case();
    let snapshot =
        advance_direct_no_oa_calc_cooling_positive_supply_capacity_limit_guard(
            &mut runtime,
            &system,
            predecessor,
        )
        .expect("CP337");
    let lifecycle =
        purchased_air_calc_cooling_positive_supply_capacity_limit_guard_lifecycle_summary(
            &runtime, system.id,
        )
        .expect("CP337 lifecycle");

    assert_eq!(lifecycle.state.latest, Some(snapshot));
    assert_eq!(
        lifecycle.source,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_SOURCE
    );
    assert_eq!(
        lifecycle.first_excluded_source,
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_CAPACITY_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE
    );
}
