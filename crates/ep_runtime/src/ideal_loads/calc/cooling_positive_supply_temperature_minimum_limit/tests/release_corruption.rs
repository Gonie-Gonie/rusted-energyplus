use super::super::*;
use crate::ideal_loads::calc::cooling_mixed_air_call::release_tests::release_case_with_demand;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitError, PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_mixed_air_call,
    advance_direct_no_oa_calc_cooling_positive_supply_cp_air_assignment,
    advance_direct_no_oa_calc_cooling_positive_supply_temperature_assignment,
    advance_direct_no_oa_calc_cooling_positive_supply_temperature_minimum_limit,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_positive_guard,
};

fn completed_cp332_case() -> (
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    crate::ideal_loads::PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot,
) {
    completed_cp332_case_with_demand(-1_000.0)
}

fn completed_cp332_case_with_demand(
    cooling_demand_w: f64,
) -> (
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    crate::ideal_loads::PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentSnapshot,
) {
    let (mut runtime, system, predecessor, zone_state) =
        release_case_with_demand(cooling_demand_w);
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
    (runtime, system, temperature_assignment)
}

#[test]
fn public_release_commits_source_shaped_maximum_once_and_rejects_replay() {
    let (mut runtime, system, predecessor) = completed_cp332_case();
    assert!(predecessor.supply_temperature_assignment_executed);
    let left = predecessor.supply_temperature_c.expect("CP332 supply temperature");
    let right = system.minimum_cooling_supply_air_temperature_c;
    let expected = source_shaped_two_argument_maximum(left, right);

    let snapshot =
        advance_direct_no_oa_calc_cooling_positive_supply_temperature_minimum_limit(
            &mut runtime,
            &system,
            predecessor,
        )
        .expect("CP333");
    assert!(
        cooling_positive_supply_temperature_minimum_limit_snapshot_is_exact_direct_release(
            snapshot
        )
    );
    assert_eq!(
        snapshot
            .supply_temperature_before_minimum_limit_c
            .map(f64::to_bits),
        Some(left.to_bits())
    );
    assert_eq!(
        snapshot
            .minimum_cooling_supply_air_temperature_c
            .map(f64::to_bits),
        Some(right.to_bits())
    );
    assert_eq!(
        snapshot.maximum_supply_temperature_c.map(f64::to_bits),
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
        .calc_cooling_positive_supply_temperature_minimum_limit;
    assert_eq!(state.transition_count, 1);
    assert_eq!(state.supply_temperature_minimum_limit_count, 1);
    assert_eq!(state.source_site_execution_count, 4);
    assert_eq!(state.supply_temperature_for_maximum_read_count, 1);
    assert_eq!(
        state.minimum_cooling_supply_air_temperature_for_maximum_read_count,
        1
    );
    assert_eq!(
        state.source_shaped_two_argument_maximum_evaluation_count,
        1
    );
    assert_eq!(state.supply_temperature_assignment_count, 1);

    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_positive_supply_temperature_minimum_limit(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}

#[test]
fn skipped_routes_do_not_read_or_validate_the_typed_minimum_temperature() {
    for cooling_demand_w in [1.0, -1.0e-40] {
        let (mut runtime, mut system, predecessor) =
            completed_cp332_case_with_demand(cooling_demand_w);
        assert!(!predecessor.supply_temperature_assignment_executed);
        system.minimum_cooling_supply_air_temperature_c = f64::NAN;

        let snapshot =
            advance_direct_no_oa_calc_cooling_positive_supply_temperature_minimum_limit(
                &mut runtime,
                &system,
                predecessor,
            )
            .expect("skipped CP333");
        assert!(
            cooling_positive_supply_temperature_minimum_limit_snapshot_is_exact_direct_release(
                snapshot
            )
        );
        assert!(!snapshot.minimum_cooling_supply_air_temperature_for_maximum_read);
        assert!(
            snapshot
                .minimum_cooling_supply_air_temperature_c
                .is_none()
        );
        assert_eq!(
            runtime
                .units
                .get(&system.id)
                .expect("known unit")
                .calc_cooling_positive_supply_temperature_minimum_limit
                .source_site_execution_count,
            0
        );
    }
}

#[test]
fn active_typed_minimum_drift_and_nonfinite_values_fail_before_mutation() {
    let (runtime, system, predecessor) = completed_cp332_case();

    let mut drift_runtime = runtime.clone();
    let mut drift_system = system.clone();
    drift_system.minimum_cooling_supply_air_temperature_c = f64::from_bits(
        drift_system
            .minimum_cooling_supply_air_temperature_c
            .to_bits()
            + 1,
    );
    let before = drift_runtime.clone();
    assert_eq!(
        advance_direct_no_oa_calc_cooling_positive_supply_temperature_minimum_limit(
            &mut drift_runtime,
            &drift_system,
            predecessor,
        ),
        Err(
            PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitError::
                CoolingActiveOperandLineageMismatch { system: system.id }
        )
    );
    assert_eq!(drift_runtime, before);

    for nonfinite in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let mut case_runtime = runtime.clone();
        let mut case_system = system.clone();
        case_system.minimum_cooling_supply_air_temperature_c = nonfinite;
        let before = case_runtime.clone();
        assert_eq!(
            advance_direct_no_oa_calc_cooling_positive_supply_temperature_minimum_limit(
                &mut case_runtime,
                &case_system,
                predecessor,
            ),
            Err(
                PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitError::
                    InvalidMinimumCoolingSupplyAirTemperature { system: system.id }
            )
        );
        assert_eq!(case_runtime, before);
    }
}

#[test]
fn cp318_same_call_minimum_is_lineage_evidence_for_the_right_operand_only() {
    let (runtime, system, predecessor) = completed_cp332_case();
    let unit = runtime.units.get(&system.id).expect("known unit");
    let sensible = unit.calc_cooling_sensible_flow.latest.expect("CP318");
    let left = predecessor.supply_temperature_c;
    let right = Some(system.minimum_cooling_supply_air_temperature_c);

    assert!(
        super::super::release::active_operands_link_to_retained_prefix_for_test(
            &system,
            sensible,
            predecessor,
            left,
            right,
        )
    );

    let mut unrelated_numerical_drift = sensible;
    unrelated_numerical_drift.zone_cooling_setpoint_load_w = unrelated_numerical_drift
        .zone_cooling_setpoint_load_w
        .map(|value| f64::from_bits(value.to_bits() + 1));
    assert!(
        super::super::release::active_operands_link_to_retained_prefix_for_test(
            &system,
            unrelated_numerical_drift,
            predecessor,
            left,
            right,
        )
    );

    let mut minimum_drift = sensible;
    minimum_drift.minimum_cooling_supply_air_temperature_c = minimum_drift
        .minimum_cooling_supply_air_temperature_c
        .map(|value| f64::from_bits(value.to_bits() + 1));
    assert!(
        !super::super::release::active_operands_link_to_retained_prefix_for_test(
            &system,
            minimum_drift,
            predecessor,
            left,
            right,
        )
    );
}

#[test]
fn forged_cp332_latest_argument_or_private_witness_fails_without_mutation() {
    let (runtime, system, predecessor) = completed_cp332_case();
    let mut forged_source = predecessor;
    forged_source.source = "forged";
    let mut forged_supply = predecessor;
    forged_supply.supply_temperature_c = forged_supply
        .supply_temperature_c
        .map(|value| f64::from_bits(value.to_bits() + 1));

    for forged in [forged_source, forged_supply] {
        let mut case_runtime = runtime.clone();
        let before = case_runtime.clone();
        assert_eq!(
            advance_direct_no_oa_calc_cooling_positive_supply_temperature_minimum_limit(
                &mut case_runtime,
                &system,
                forged,
            ),
            Err(
                PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitError::
                    CoolingPositiveSupplyTemperatureAssignmentSnapshotMismatch {
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
        .calc_cooling_positive_supply_temperature_assignment
        .latest = Some(forged_supply);
    let before = latest_runtime.clone();
    assert_eq!(
        advance_direct_no_oa_calc_cooling_positive_supply_temperature_minimum_limit(
            &mut latest_runtime,
            &system,
            predecessor,
        ),
        Err(
            PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitError::
                CoolingPositiveSupplyTemperatureAssignmentSnapshotMismatch { system: system.id }
        )
    );
    assert_eq!(latest_runtime, before);

    let mut witness_runtime = runtime;
    witness_runtime.set_cooling_positive_supply_temperature_assignment_latest_witness(
        system.id,
        forged_supply,
    );
    let before = witness_runtime.clone();
    assert_eq!(
        advance_direct_no_oa_calc_cooling_positive_supply_temperature_minimum_limit(
            &mut witness_runtime,
            &system,
            predecessor,
        ),
        Err(
            PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitError::
                CoolingPositiveSupplyTemperatureAssignmentSnapshotMismatch { system: system.id }
        )
    );
    assert_eq!(witness_runtime, before);
}

#[test]
fn orphan_cp333_latest_or_private_witness_is_fail_closed_and_transactional() {
    let (runtime, system, predecessor) = completed_cp332_case();
    let mut completed_runtime = runtime.clone();
    let prospective =
        advance_direct_no_oa_calc_cooling_positive_supply_temperature_minimum_limit(
            &mut completed_runtime,
            &system,
            predecessor,
        )
        .expect("prospective CP333");

    for latest_only in [true, false] {
        let mut case_runtime = runtime.clone();
        if latest_only {
            case_runtime
                .units
                .get_mut(&system.id)
                .expect("known unit")
                .calc_cooling_positive_supply_temperature_minimum_limit
                .latest = Some(prospective);
        } else {
            case_runtime
                .set_cooling_positive_supply_temperature_minimum_limit_latest_witness(
                    system.id,
                    prospective,
                );
        }
        let before = case_runtime.clone();
        assert_eq!(
            advance_direct_no_oa_calc_cooling_positive_supply_temperature_minimum_limit(
                &mut case_runtime,
                &system,
                predecessor,
            ),
            Err(
                PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitError::
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
        Maximum,
        Assignment,
        WitnessedLimit,
    }

    for counter in [
        Counter::Transition,
        Counter::Limit,
        Counter::SourceSite,
        Counter::LeftRead,
        Counter::RightRead,
        Counter::Maximum,
        Counter::Assignment,
        Counter::WitnessedLimit,
    ] {
        let (mut runtime, system, predecessor) = completed_cp332_case();
        let unit = runtime.units.get_mut(&system.id).expect("known unit");
        let state = &mut unit.calc_cooling_positive_supply_temperature_minimum_limit;
        match counter {
            Counter::Transition => state.transition_count = usize::MAX,
            Counter::Limit => state.supply_temperature_minimum_limit_count = usize::MAX,
            Counter::SourceSite => state.source_site_execution_count = usize::MAX - 3,
            Counter::LeftRead => state.supply_temperature_for_maximum_read_count = usize::MAX,
            Counter::RightRead => {
                state.minimum_cooling_supply_air_temperature_for_maximum_read_count = usize::MAX;
            }
            Counter::Maximum => {
                state.source_shaped_two_argument_maximum_evaluation_count = usize::MAX;
            }
            Counter::Assignment => state.supply_temperature_assignment_count = usize::MAX,
            Counter::WitnessedLimit => {
                state.witnessed_supply_temperature_minimum_limit_count = usize::MAX;
            }
        }
        assert!(
            !super::super::release::next_supply_temperature_minimum_limit_transition_fits_for_test(
                unit,
                predecessor,
            )
        );
        let before = runtime.clone();
        assert_eq!(
            advance_direct_no_oa_calc_cooling_positive_supply_temperature_minimum_limit(
                &mut runtime,
                &system,
                predecessor,
            ),
            Err(
                PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitError::
                    RuntimeStateInvariantViolation { system: system.id }
            )
        );
        assert_eq!(runtime, before);
    }
}

#[test]
fn skipped_route_preflight_checks_public_and_private_counters_transactionally() {
    for public_counter in [true, false] {
        let (mut runtime, system, predecessor) =
            completed_cp332_case_with_demand(-1.0e-40);
        assert!(predecessor.positive_guard_false_fallthrough_skipped);
        let unit = runtime.units.get_mut(&system.id).expect("known unit");
        let state = &mut unit.calc_cooling_positive_supply_temperature_minimum_limit;
        if public_counter {
            state.positive_guard_false_fallthrough_skip_count = usize::MAX;
        } else {
            state.witnessed_positive_guard_false_fallthrough_skip_count = usize::MAX;
        }
        assert!(
            !super::super::release::next_supply_temperature_minimum_limit_transition_fits_for_test(
                unit,
                predecessor,
            )
        );
        let before = runtime.clone();
        assert_eq!(
            advance_direct_no_oa_calc_cooling_positive_supply_temperature_minimum_limit(
                &mut runtime,
                &system,
                predecessor,
            ),
            Err(
                PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitError::
                    RuntimeStateInvariantViolation { system: system.id }
            )
        );
        assert_eq!(runtime, before);
    }

    let (mut runtime, system, predecessor) = completed_cp332_case_with_demand(1.0);
    assert!(predecessor.non_cooling_skipped);
    let unit = runtime.units.get_mut(&system.id).expect("known unit");
    unit.calc_cooling_positive_supply_temperature_minimum_limit
        .non_cooling_skip_count = usize::MAX;
    assert!(
        !super::super::release::next_supply_temperature_minimum_limit_transition_fits_for_test(
            unit,
            predecessor,
        )
    );
    let before = runtime.clone();
    assert_eq!(
        advance_direct_no_oa_calc_cooling_positive_supply_temperature_minimum_limit(
            &mut runtime,
            &system,
            predecessor,
        ),
        Err(
            PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitError::
                RuntimeStateInvariantViolation { system: system.id }
        )
    );
    assert_eq!(runtime, before);
}

#[test]
fn recursive_cp318_or_cp332_corruption_fails_closed_without_mutation() {
    for corrupt_cp332 in [false, true] {
        let (mut runtime, system, predecessor) = completed_cp332_case();
        let unit = runtime.units.get_mut(&system.id).expect("known unit");
        if corrupt_cp332 {
            unit.calc_cooling_positive_supply_temperature_assignment
                .source_site_execution_count += 1;
        } else {
            let corrupted = {
                let latest = unit
                    .calc_cooling_sensible_flow
                    .latest
                    .as_mut()
                    .expect("CP318");
                let minimum = latest
                    .minimum_cooling_supply_air_temperature_c
                    .as_mut()
                    .expect("CP318 minimum");
                *minimum = f64::from_bits(minimum.to_bits() + 1);
                *latest
            };
            runtime.set_cooling_sensible_flow_latest_witness(system.id, corrupted);
        }
        let before = runtime.clone();
        assert_eq!(
            advance_direct_no_oa_calc_cooling_positive_supply_temperature_minimum_limit(
                &mut runtime,
                &system,
                predecessor,
            ),
            Err(
                PurchasedAirCalcCoolingPositiveSupplyTemperatureMinimumLimitError::
                    RuntimeStateInvariantViolation { system: system.id }
            )
        );
        assert_eq!(runtime, before);
    }
}
