use super::super::*;
use crate::ideal_loads::calc::cooling_mixed_air_call::release_tests::
    release_case_with_demand_and_availability;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitError, PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_mixed_air_call,
    advance_direct_no_oa_calc_cooling_positive_supply_cp_air_assignment,
    advance_direct_no_oa_calc_cooling_positive_supply_temperature_assignment,
    advance_direct_no_oa_calc_cooling_positive_supply_temperature_minimum_limit,
    advance_direct_no_oa_calc_cooling_positive_supply_temperature_mixed_air_limit,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_positive_guard,
};

fn completed_cp333_case() -> (
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    crate::ideal_loads::PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitSnapshot,
) {
    completed_cp333_case_with_demand(-1_000.0)
}

fn completed_cp333_case_with_demand(
    cooling_demand_w: f64,
) -> (
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    crate::ideal_loads::PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitSnapshot,
) {
    completed_cp333_case_with_demand_and_availability(cooling_demand_w, 1.0)
}

fn completed_cp333_case_with_demand_and_availability(
    cooling_demand_w: f64,
    overall_availability: f64,
) -> (
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    crate::ideal_loads::PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitSnapshot,
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
    (runtime, system, minimum_limit)
}

#[test]
fn public_release_commits_source_shaped_minimum_once_and_rejects_replay() {
    let (mut runtime, system, predecessor) = completed_cp333_case();
    assert!(predecessor.supply_temperature_minimum_limit_executed);
    let left = predecessor
        .assigned_supply_temperature_c
        .expect("CP333 assigned supply temperature");
    let right = runtime
        .units
        .get(&system.id)
        .and_then(|unit| unit.calc_cooling_mixed_air_call.latest)
        .and_then(|mixed_air| mixed_air.mixed_air_temperature_c)
        .expect("CP329 mixed-air temperature");
    let expected = source_shaped_two_argument_minimum(left, right);

    let snapshot = advance_direct_no_oa_calc_cooling_positive_supply_temperature_mixed_air_limit(
        &mut runtime,
        &system,
        predecessor,
    )
    .expect("CP334");
    assert!(
        cooling_positive_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release(
            snapshot
        )
    );
    assert_eq!(
        snapshot
            .supply_temperature_before_mixed_air_limit_c
            .map(f64::to_bits),
        Some(left.to_bits())
    );
    assert_eq!(
        snapshot.mixed_air_temperature_c.map(f64::to_bits),
        Some(right.to_bits())
    );
    assert_eq!(
        snapshot.minimum_supply_temperature_c.map(f64::to_bits),
        Some(expected.to_bits())
    );
    assert_eq!(
        snapshot.assigned_supply_temperature_c.map(f64::to_bits),
        Some(expected.to_bits())
    );
    let state = &runtime
        .units
        .get(&system.id)
        .expect("known unit")
        .calc_cooling_positive_supply_temperature_mixed_air_limit;
    assert_eq!(state.transition_count, 1);
    assert_eq!(state.supply_temperature_mixed_air_limit_count, 1);
    assert_eq!(state.source_site_execution_count, 4);
    assert_eq!(state.supply_temperature_for_minimum_read_count, 1);
    assert_eq!(state.mixed_air_temperature_for_minimum_read_count, 1);
    assert_eq!(state.source_shaped_two_argument_minimum_evaluation_count, 1);
    assert_eq!(state.supply_temperature_assignment_count, 1);

    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_positive_supply_temperature_mixed_air_limit(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}

#[test]
fn skipped_routes_do_not_read_the_retained_mixed_air_operand() {
    for (cooling_demand_w, guard_false) in [(1.0, false), (-1.0e-40, true)] {
        let (mut runtime, system, predecessor) = completed_cp333_case_with_demand(cooling_demand_w);
        assert!(!predecessor.supply_temperature_minimum_limit_executed);
        let retained_mixed_air = runtime
            .units
            .get(&system.id)
            .expect("known unit")
            .calc_cooling_mixed_air_call
            .latest
            .expect("CP329");
        assert_eq!(
            retained_mixed_air.mixed_air_temperature_c.is_some(),
            guard_false
        );

        let snapshot =
            advance_direct_no_oa_calc_cooling_positive_supply_temperature_mixed_air_limit(
                &mut runtime,
                &system,
                predecessor,
            )
            .expect("skipped CP334");
        assert!(
            cooling_positive_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release(
                snapshot
            )
        );
        assert_eq!(
            snapshot.positive_guard_false_fallthrough_skipped,
            guard_false
        );
        assert!(!snapshot.supply_temperature_for_minimum_read);
        assert!(
            snapshot
                .supply_temperature_before_mixed_air_limit_c
                .is_none()
        );
        assert!(!snapshot.mixed_air_temperature_for_minimum_read);
        assert!(snapshot.mixed_air_temperature_c.is_none());
        assert_eq!(
            runtime
                .units
                .get(&system.id)
                .expect("known unit")
                .calc_cooling_positive_supply_temperature_mixed_air_limit
                .source_site_execution_count,
            0
        );
    }
}

#[test]
fn active_operands_link_only_to_cp333_assignment_and_cp329_mixed_air_output() {
    let (runtime, system, predecessor) = completed_cp333_case();
    let mixed_air = runtime
        .units
        .get(&system.id)
        .expect("known unit")
        .calc_cooling_mixed_air_call
        .latest
        .expect("CP329");
    let left = predecessor.assigned_supply_temperature_c;
    let right = mixed_air.mixed_air_temperature_c;

    assert!(
        super::super::release::active_operands_link_to_retained_prefix_for_test(
            predecessor,
            mixed_air,
            left,
            right,
        )
    );

    let mut right_drift = mixed_air;
    right_drift.mixed_air_temperature_c = right_drift
        .mixed_air_temperature_c
        .map(|value| f64::from_bits(value.to_bits() + 1));
    assert!(
        !super::super::release::active_operands_link_to_retained_prefix_for_test(
            predecessor,
            right_drift,
            left,
            right,
        )
    );

    let mut stale_call = mixed_air;
    stale_call.parent_call_ordinal += 1;
    assert!(
        !super::super::release::active_operands_link_to_retained_prefix_for_test(
            predecessor,
            stale_call,
            left,
            right,
        )
    );
}

#[test]
fn forged_cp333_latest_argument_or_private_witness_fails_without_mutation() {
    let (runtime, system, predecessor) = completed_cp333_case();
    let mut forged_source = predecessor;
    forged_source.source = "forged";
    let mut forged_supply = predecessor;
    forged_supply.assigned_supply_temperature_c = forged_supply
        .assigned_supply_temperature_c
        .map(|value| f64::from_bits(value.to_bits() + 1));

    for forged in [forged_source, forged_supply] {
        let mut case_runtime = runtime.clone();
        let before = case_runtime.clone();
        assert_eq!(
            advance_direct_no_oa_calc_cooling_positive_supply_temperature_mixed_air_limit(
                &mut case_runtime,
                &system,
                forged,
            ),
            Err(
                PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitError::
                    CoolingPositiveSupplyTemperatureMinimumLimitSnapshotMismatch {
                        system: system.id
                    }
            )
        );
        assert_eq!(case_runtime, before);
    }

    let mut latest_runtime = runtime.clone();
    latest_runtime
        .units
        .get_mut(&system.id)
        .expect("known unit")
        .calc_cooling_positive_supply_temperature_minimum_limit
        .latest = Some(forged_supply);
    let before = latest_runtime.clone();
    assert_eq!(
        advance_direct_no_oa_calc_cooling_positive_supply_temperature_mixed_air_limit(
            &mut latest_runtime,
            &system,
            predecessor,
        ),
        Err(
            PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitError::
                CoolingPositiveSupplyTemperatureMinimumLimitSnapshotMismatch {
                    system: system.id
                }
        )
    );
    assert_eq!(latest_runtime, before);

    let mut witness_runtime = runtime;
    witness_runtime.set_cooling_positive_supply_temperature_minimum_limit_latest_witness(
        system.id,
        forged_supply,
    );
    let before = witness_runtime.clone();
    assert_eq!(
        advance_direct_no_oa_calc_cooling_positive_supply_temperature_mixed_air_limit(
            &mut witness_runtime,
            &system,
            predecessor,
        ),
        Err(
            PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitError::
                CoolingPositiveSupplyTemperatureMinimumLimitSnapshotMismatch {
                    system: system.id
                }
        )
    );
    assert_eq!(witness_runtime, before);
}

#[test]
fn cp329_mixed_air_latest_witness_or_source_drift_fails_closed_transactionally() {
    #[derive(Clone, Copy)]
    enum Corruption {
        LatestOnly,
        WitnessOnly,
        CoordinatedSourceDrift,
    }

    for corruption in [
        Corruption::LatestOnly,
        Corruption::WitnessOnly,
        Corruption::CoordinatedSourceDrift,
    ] {
        let (mut runtime, system, predecessor) = completed_cp333_case();
        let mut forged = runtime
            .units
            .get(&system.id)
            .expect("known unit")
            .calc_cooling_mixed_air_call
            .latest
            .expect("CP329");
        forged.mixed_air_temperature_c = forged
            .mixed_air_temperature_c
            .map(|value| f64::from_bits(value.to_bits() + 1));

        match corruption {
            Corruption::LatestOnly => {
                runtime
                    .units
                    .get_mut(&system.id)
                    .expect("known unit")
                    .calc_cooling_mixed_air_call
                    .latest = Some(forged);
            }
            Corruption::WitnessOnly => {
                runtime.set_cooling_mixed_air_call_latest_witness(system.id, forged);
            }
            Corruption::CoordinatedSourceDrift => {
                runtime
                    .units
                    .get_mut(&system.id)
                    .expect("known unit")
                    .calc_cooling_mixed_air_call
                    .latest = Some(forged);
                runtime.set_cooling_mixed_air_call_latest_witness(system.id, forged);
            }
        }

        let before = runtime.clone();
        assert_eq!(
            advance_direct_no_oa_calc_cooling_positive_supply_temperature_mixed_air_limit(
                &mut runtime,
                &system,
                predecessor,
            ),
            Err(
                PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitError::
                    RuntimeStateInvariantViolation { system: system.id }
            )
        );
        assert_eq!(runtime, before);
    }
}

#[test]
fn orphan_cp334_latest_or_private_witness_is_fail_closed_and_transactional() {
    let (runtime, system, predecessor) = completed_cp333_case();
    let mut completed_runtime = runtime.clone();
    let prospective =
        advance_direct_no_oa_calc_cooling_positive_supply_temperature_mixed_air_limit(
            &mut completed_runtime,
            &system,
            predecessor,
        )
        .expect("prospective CP334");

    for latest_only in [true, false] {
        let mut case_runtime = runtime.clone();
        if latest_only {
            case_runtime
                .units
                .get_mut(&system.id)
                .expect("known unit")
                .calc_cooling_positive_supply_temperature_mixed_air_limit
                .latest = Some(prospective);
        } else {
            case_runtime.set_cooling_positive_supply_temperature_mixed_air_limit_latest_witness(
                system.id,
                prospective,
            );
        }
        let before = case_runtime.clone();
        assert_eq!(
            advance_direct_no_oa_calc_cooling_positive_supply_temperature_mixed_air_limit(
                &mut case_runtime,
                &system,
                predecessor,
            ),
            Err(
                PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitError::
                    RuntimeStateInvariantViolation { system: system.id }
            )
        );
        assert_eq!(case_runtime, before);
    }
}

#[test]
fn checked_preflight_rejects_every_active_counter_overflow_without_mutation() {
    #[derive(Clone, Copy)]
    enum Counter {
        Transition,
        Limit,
        SourceSite,
        LeftRead,
        RightRead,
        Minimum,
        Assignment,
        WitnessedLimit,
    }

    for counter in [
        Counter::Transition,
        Counter::Limit,
        Counter::SourceSite,
        Counter::LeftRead,
        Counter::RightRead,
        Counter::Minimum,
        Counter::Assignment,
        Counter::WitnessedLimit,
    ] {
        let (mut runtime, system, predecessor) = completed_cp333_case();
        let unit = runtime.units.get_mut(&system.id).expect("known unit");
        let state = &mut unit.calc_cooling_positive_supply_temperature_mixed_air_limit;
        match counter {
            Counter::Transition => state.transition_count = usize::MAX,
            Counter::Limit => state.supply_temperature_mixed_air_limit_count = usize::MAX,
            Counter::SourceSite => state.source_site_execution_count = usize::MAX - 3,
            Counter::LeftRead => state.supply_temperature_for_minimum_read_count = usize::MAX,
            Counter::RightRead => state.mixed_air_temperature_for_minimum_read_count = usize::MAX,
            Counter::Minimum => {
                state.source_shaped_two_argument_minimum_evaluation_count = usize::MAX;
            }
            Counter::Assignment => state.supply_temperature_assignment_count = usize::MAX,
            Counter::WitnessedLimit => {
                state.witnessed_supply_temperature_mixed_air_limit_count = usize::MAX;
            }
        }
        assert!(
            !super::super::release::
                next_supply_temperature_mixed_air_limit_transition_fits_for_test(
                    unit,
                    predecessor,
                )
        );
        let before = runtime.clone();
        assert_eq!(
            advance_direct_no_oa_calc_cooling_positive_supply_temperature_mixed_air_limit(
                &mut runtime,
                &system,
                predecessor,
            ),
            Err(
                PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitError::
                    RuntimeStateInvariantViolation { system: system.id }
            )
        );
        assert_eq!(runtime, before);
    }
}

#[test]
fn skipped_route_preflight_checks_public_private_and_unit_off_counters() {
    for public_counter in [true, false] {
        let (mut runtime, system, predecessor) = completed_cp333_case_with_demand(-1.0e-40);
        assert!(predecessor.positive_guard_false_fallthrough_skipped);
        let unit = runtime.units.get_mut(&system.id).expect("known unit");
        let state = &mut unit.calc_cooling_positive_supply_temperature_mixed_air_limit;
        if public_counter {
            state.positive_guard_false_fallthrough_skip_count = usize::MAX;
        } else {
            state.witnessed_positive_guard_false_fallthrough_skip_count = usize::MAX;
        }
        assert!(
            !super::super::release::
                next_supply_temperature_mixed_air_limit_transition_fits_for_test(
                    unit,
                    predecessor,
                )
        );
        let before = runtime.clone();
        assert_eq!(
            advance_direct_no_oa_calc_cooling_positive_supply_temperature_mixed_air_limit(
                &mut runtime,
                &system,
                predecessor,
            ),
            Err(
                PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitError::
                    RuntimeStateInvariantViolation { system: system.id }
            )
        );
        assert_eq!(runtime, before);
    }

    let (mut runtime, system, predecessor) = completed_cp333_case_with_demand(1.0);
    assert!(predecessor.non_cooling_skipped);
    let unit = runtime.units.get_mut(&system.id).expect("known unit");
    unit.calc_cooling_positive_supply_temperature_mixed_air_limit
        .non_cooling_skip_count = usize::MAX;
    assert!(
        !super::super::release::next_supply_temperature_mixed_air_limit_transition_fits_for_test(
            unit,
            predecessor
        )
    );
    let before = runtime.clone();
    assert_eq!(
        advance_direct_no_oa_calc_cooling_positive_supply_temperature_mixed_air_limit(
            &mut runtime,
            &system,
            predecessor,
        ),
        Err(
            PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitError::
                RuntimeStateInvariantViolation { system: system.id }
        )
    );
    assert_eq!(runtime, before);

    let (mut runtime, system, unit_off) =
        completed_cp333_case_with_demand_and_availability(-1_000.0, 0.0);
    assert!(unit_off.unit_off_skipped);
    let unit = runtime.units.get_mut(&system.id).expect("known unit");
    unit.calc_cooling_positive_supply_temperature_mixed_air_limit
        .unit_off_skip_count = usize::MAX;
    assert!(
        !super::super::release::next_supply_temperature_mixed_air_limit_transition_fits_for_test(
            unit,
            unit_off,
        )
    );
    let before = runtime.clone();
    assert_eq!(
        advance_direct_no_oa_calc_cooling_positive_supply_temperature_mixed_air_limit(
            &mut runtime,
            &system,
            unit_off,
        ),
        Err(
            PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitError::
                RuntimeStateInvariantViolation { system: system.id }
        )
    );
    assert_eq!(runtime, before);
}

#[test]
fn recursive_cp333_state_corruption_fails_closed_without_mutation() {
    let (mut runtime, system, predecessor) = completed_cp333_case();
    runtime
        .units
        .get_mut(&system.id)
        .expect("known unit")
        .calc_cooling_positive_supply_temperature_minimum_limit
        .source_site_execution_count += 1;
    let before = runtime.clone();
    assert_eq!(
        advance_direct_no_oa_calc_cooling_positive_supply_temperature_mixed_air_limit(
            &mut runtime,
            &system,
            predecessor,
        ),
        Err(
            PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitError::
                RuntimeStateInvariantViolation { system: system.id }
        )
    );
    assert_eq!(runtime, before);
}
