use super::super::*;
use crate::ideal_loads::calc::cooling_mixed_air_call::release_tests::{
    release_case, release_case_with_demand,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentError,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentInput, PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_mixed_air_call,
    advance_direct_no_oa_calc_cooling_positive_supply_cp_air_assignment,
    advance_direct_no_oa_calc_cooling_positive_supply_temperature_assignment,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_positive_guard,
};

fn completed_cp331_case() -> (
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    crate::ideal_loads::PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot,
    crate::heat_balance::state::ZoneHeatBalanceState,
) {
    let (mut runtime, system, predecessor, zone_state) = release_case();
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
    assert!(cp_air_assignment.cp_air_assignment_executed);
    (runtime, system, cp_air_assignment, zone_state)
}

fn completed_cp331_case_with_demand(
    cooling_demand_w: f64,
) -> (
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    crate::ideal_loads::PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentSnapshot,
    crate::heat_balance::state::ZoneHeatBalanceState,
) {
    let (mut runtime, system, predecessor, zone_state) = release_case_with_demand(cooling_demand_w);
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
    (runtime, system, cp_air_assignment, zone_state)
}

#[test]
fn public_release_commits_raw_source_grouping_once_and_rejects_replay() {
    let (mut runtime, system, predecessor, zone_state) = completed_cp331_case();
    let unit = runtime.units.get(&system.id).expect("known unit");
    let load = unit
        .calc_entry
        .latest
        .expect("CP310")
        .demand
        .remaining_output_req_to_cool_sp_w;
    let flow = unit
        .calc_cooling_supply_mass_flow_positive_guard
        .latest
        .expect("CP330")
        .supply_mass_flow_rate_kg_per_s
        .expect("active flow");
    let cp_air = predecessor.cp_air_j_per_kg_k.expect("active CpAir");
    let denominator = cp_air * flow;
    let quotient = load / denominator;
    let expected = quotient + zone_state.mean_air_temperature_c;

    let snapshot = advance_direct_no_oa_calc_cooling_positive_supply_temperature_assignment(
        &mut runtime,
        &system,
        predecessor,
        &zone_state,
    )
    .expect("CP332");
    assert!(
        cooling_positive_supply_temperature_assignment_snapshot_is_exact_direct_release(snapshot)
    );
    assert_eq!(
        snapshot.zone_cooling_setpoint_load_w.map(f64::to_bits),
        Some(load.to_bits())
    );
    assert_eq!(
        snapshot.cp_air_j_per_kg_k.map(f64::to_bits),
        Some(cp_air.to_bits())
    );
    assert_eq!(
        snapshot.supply_mass_flow_rate_kg_per_s.map(f64::to_bits),
        Some(flow.to_bits())
    );
    assert_eq!(
        snapshot
            .cp_air_times_supply_mass_flow_rate_w_per_k
            .map(f64::to_bits),
        Some(denominator.to_bits())
    );
    assert_eq!(
        snapshot
            .zone_cooling_setpoint_load_over_denominator_c
            .map(f64::to_bits),
        Some(quotient.to_bits())
    );
    assert_eq!(
        snapshot.supply_temperature_c.map(f64::to_bits),
        Some(expected.to_bits())
    );
    let state = &runtime
        .units
        .get(&system.id)
        .expect("known unit")
        .calc_cooling_positive_supply_temperature_assignment;
    assert_eq!(state.transition_count, 1);
    assert_eq!(state.supply_temperature_assignment_count, 1);
    assert_eq!(state.source_site_execution_count, 8);

    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_positive_supply_temperature_assignment(
            &mut runtime,
            &system,
            predecessor,
            &zone_state,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}

#[test]
fn skipped_routes_do_not_read_or_validate_live_zone_node_temperature() {
    for cooling_demand_w in [1.0, -1.0e-40] {
        let (mut runtime, system, predecessor, mut zone_state) =
            completed_cp331_case_with_demand(cooling_demand_w);
        assert!(!predecessor.cp_air_assignment_executed);
        zone_state.mean_air_temperature_c = f64::NAN;

        let snapshot = advance_direct_no_oa_calc_cooling_positive_supply_temperature_assignment(
            &mut runtime,
            &system,
            predecessor,
            &zone_state,
        )
        .expect("skipped CP332");
        assert!(
            cooling_positive_supply_temperature_assignment_snapshot_is_exact_direct_release(
                snapshot
            )
        );
        assert!(!snapshot.zone_node_temperature_read);
        assert!(snapshot.zone_node_temperature_c.is_none());
        assert_eq!(
            runtime
                .units
                .get(&system.id)
                .expect("known unit")
                .calc_cooling_positive_supply_temperature_assignment
                .source_site_execution_count,
            0
        );
    }
}

#[test]
fn live_zone_temperature_drift_fails_closed_and_transactionally() {
    let (mut runtime, system, predecessor, mut zone_state) = completed_cp331_case();
    zone_state.mean_air_temperature_c =
        f64::from_bits(zone_state.mean_air_temperature_c.to_bits() + 1);
    let before = runtime.clone();

    assert_eq!(
        advance_direct_no_oa_calc_cooling_positive_supply_temperature_assignment(
            &mut runtime,
            &system,
            predecessor,
            &zone_state,
        ),
        Err(
            PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentError::
                CoolingActiveOperandLineageMismatch { system: system.id }
        )
    );
    assert_eq!(runtime, before);
}

#[test]
fn active_nonfinite_temperature_and_wrong_zone_identity_fail_before_mutation() {
    let (runtime, system, predecessor, zone_state) = completed_cp331_case();

    let mut nonfinite_runtime = runtime.clone();
    let mut nonfinite_zone = zone_state.clone();
    nonfinite_zone.mean_air_temperature_c = f64::NAN;
    let before = nonfinite_runtime.clone();
    assert_eq!(
        advance_direct_no_oa_calc_cooling_positive_supply_temperature_assignment(
            &mut nonfinite_runtime,
            &system,
            predecessor,
            &nonfinite_zone,
        ),
        Err(
            PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentError::InvalidActiveInput {
                system: system.id,
                input:
                    PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentInput::
                        ZoneNodeTemperature,
            }
        )
    );
    assert_eq!(nonfinite_runtime, before);

    let mut wrong_zone_runtime = runtime;
    let mut wrong_zone = zone_state;
    wrong_zone.zone_id = ep_model::ZoneId(wrong_zone.zone_id.0 + 1);
    let before = wrong_zone_runtime.clone();
    assert_eq!(
        advance_direct_no_oa_calc_cooling_positive_supply_temperature_assignment(
            &mut wrong_zone_runtime,
            &system,
            predecessor,
            &wrong_zone,
        ),
        Err(
            PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentError::ZoneIdentityMismatch {
                expected: predecessor.controlled_zone,
                actual: wrong_zone.zone_id,
            }
        )
    );
    assert_eq!(wrong_zone_runtime, before);
}

#[test]
fn forged_cp331_snapshot_or_private_witness_fails_without_mutation() {
    let (runtime, system, predecessor, zone_state) = completed_cp331_case();
    let mut forged_source = predecessor;
    forged_source.source = "forged";
    let mut forged_cp_air = predecessor;
    forged_cp_air.cp_air_j_per_kg_k = forged_cp_air
        .cp_air_j_per_kg_k
        .map(|value| f64::from_bits(value.to_bits() + 1));

    for forged in [forged_source, forged_cp_air] {
        let mut case_runtime = runtime.clone();
        let before = case_runtime.clone();
        assert_eq!(
            advance_direct_no_oa_calc_cooling_positive_supply_temperature_assignment(
                &mut case_runtime,
                &system,
                forged,
                &zone_state,
            ),
            Err(
                PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentError::
                    CoolingPositiveSupplyCpAirAssignmentSnapshotMismatch { system: system.id }
            )
        );
        assert_eq!(case_runtime, before);
    }

    let mut case_runtime = runtime;
    let mut forged_witness = predecessor;
    forged_witness.source = "forged-private-witness";
    case_runtime
        .set_cooling_positive_supply_cp_air_assignment_latest_witness(system.id, forged_witness);
    let before = case_runtime.clone();
    assert_eq!(
        advance_direct_no_oa_calc_cooling_positive_supply_temperature_assignment(
            &mut case_runtime,
            &system,
            predecessor,
            &zone_state,
        ),
        Err(
            PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentError::
                CoolingPositiveSupplyCpAirAssignmentSnapshotMismatch { system: system.id }
        )
    );
    assert_eq!(case_runtime, before);
}

#[test]
fn private_latest_or_witness_corruption_is_fail_closed_and_transactional() {
    let (runtime, system, predecessor, zone_state) = completed_cp331_case();
    let mut completed_runtime = runtime.clone();
    let prospective = advance_direct_no_oa_calc_cooling_positive_supply_temperature_assignment(
        &mut completed_runtime,
        &system,
        predecessor,
        &zone_state,
    )
    .expect("prospective CP332");

    for latest_only in [true, false] {
        let mut case_runtime = runtime.clone();
        if latest_only {
            case_runtime
                .units
                .get_mut(&system.id)
                .expect("known unit")
                .calc_cooling_positive_supply_temperature_assignment
                .latest = Some(prospective);
        } else {
            case_runtime.set_cooling_positive_supply_temperature_assignment_latest_witness(
                system.id,
                prospective,
            );
        }
        let before = case_runtime.clone();
        assert_eq!(
            advance_direct_no_oa_calc_cooling_positive_supply_temperature_assignment(
                &mut case_runtime,
                &system,
                predecessor,
                &zone_state,
            ),
            Err(
                PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentError::
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
        Assignment,
        SourceSite,
        LoadRead,
        CpAirRead,
        FlowRead,
        Product,
        Quotient,
        ZoneRead,
        Sum,
        Write,
        WitnessedAssignment,
    }

    for counter in [
        Counter::Transition,
        Counter::Assignment,
        Counter::SourceSite,
        Counter::LoadRead,
        Counter::CpAirRead,
        Counter::FlowRead,
        Counter::Product,
        Counter::Quotient,
        Counter::ZoneRead,
        Counter::Sum,
        Counter::Write,
        Counter::WitnessedAssignment,
    ] {
        let (mut runtime, system, predecessor, zone_state) = completed_cp331_case();
        let state = &mut runtime
            .units
            .get_mut(&system.id)
            .expect("known unit")
            .calc_cooling_positive_supply_temperature_assignment;
        match counter {
            Counter::Transition => state.transition_count = usize::MAX,
            Counter::Assignment => state.supply_temperature_assignment_count = usize::MAX,
            Counter::SourceSite => state.source_site_execution_count = usize::MAX - 7,
            Counter::LoadRead => state.zone_cooling_setpoint_load_read_count = usize::MAX,
            Counter::CpAirRead => state.cp_air_read_count = usize::MAX,
            Counter::FlowRead => state.supply_mass_flow_rate_read_count = usize::MAX,
            Counter::Product => {
                state.cp_air_times_supply_mass_flow_rate_calculation_count = usize::MAX;
            }
            Counter::Quotient => {
                state.zone_cooling_setpoint_load_over_denominator_calculation_count = usize::MAX;
            }
            Counter::ZoneRead => state.zone_node_temperature_read_count = usize::MAX,
            Counter::Sum => state.supply_temperature_calculation_count = usize::MAX,
            Counter::Write => state.supply_temperature_assignment_write_count = usize::MAX,
            Counter::WitnessedAssignment => {
                state.witnessed_supply_temperature_assignment_count = usize::MAX;
            }
        }
        let before = runtime.clone();
        assert_eq!(
            advance_direct_no_oa_calc_cooling_positive_supply_temperature_assignment(
                &mut runtime,
                &system,
                predecessor,
                &zone_state,
            ),
            Err(
                PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentError::
                    RuntimeStateInvariantViolation { system: system.id }
            )
        );
        assert_eq!(runtime, before);
    }
}

#[test]
fn guard_false_preflight_checks_public_and_private_skip_counters_transactionally() {
    for public_counter in [true, false] {
        let (mut runtime, system, predecessor, zone_state) =
            completed_cp331_case_with_demand(-1.0e-40);
        assert!(predecessor.positive_guard_false_fallthrough_skipped);
        let state = &mut runtime
            .units
            .get_mut(&system.id)
            .expect("known unit")
            .calc_cooling_positive_supply_temperature_assignment;
        if public_counter {
            state.positive_guard_false_fallthrough_skip_count = usize::MAX;
        } else {
            state.witnessed_positive_guard_false_fallthrough_skip_count = usize::MAX;
        }
        let before = runtime.clone();

        assert_eq!(
            advance_direct_no_oa_calc_cooling_positive_supply_temperature_assignment(
                &mut runtime,
                &system,
                predecessor,
                &zone_state,
            ),
            Err(
                PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentError::
                    RuntimeStateInvariantViolation { system: system.id }
            )
        );
        assert_eq!(runtime, before);
    }
}

#[test]
fn cp318_and_cp329_operand_lineage_drift_is_fail_closed_without_mutation() {
    #[derive(Clone, Copy)]
    enum Corruption {
        SensibleLoad,
        SensibleCpAir,
        SensibleZoneTemperature,
        RecirculationTemperature,
        MixedAirTemperature,
    }

    for corruption in [
        Corruption::SensibleLoad,
        Corruption::SensibleCpAir,
        Corruption::SensibleZoneTemperature,
        Corruption::RecirculationTemperature,
        Corruption::MixedAirTemperature,
    ] {
        let (mut runtime, system, predecessor, zone_state) = completed_cp331_case();
        match corruption {
            Corruption::SensibleLoad
            | Corruption::SensibleCpAir
            | Corruption::SensibleZoneTemperature => {
                let corrupted = {
                    let latest = runtime
                        .units
                        .get_mut(&system.id)
                        .expect("known unit")
                        .calc_cooling_sensible_flow
                        .latest
                        .as_mut()
                        .expect("CP318");
                    let value = match corruption {
                        Corruption::SensibleLoad => latest
                            .zone_cooling_setpoint_load_w
                            .as_mut()
                            .expect("CP318 Q"),
                        Corruption::SensibleCpAir => {
                            latest.cp_air_j_per_kg_k.as_mut().expect("CP318 CpAir")
                        }
                        Corruption::SensibleZoneTemperature => latest
                            .zone_temperature_c
                            .as_mut()
                            .expect("CP318 Zone temperature"),
                        _ => unreachable!(),
                    };
                    *value = f64::from_bits(value.to_bits() + 1);
                    *latest
                };
                runtime.set_cooling_sensible_flow_latest_witness(system.id, corrupted);
            }
            Corruption::RecirculationTemperature | Corruption::MixedAirTemperature => {
                let corrupted = {
                    let latest = runtime
                        .units
                        .get_mut(&system.id)
                        .expect("known unit")
                        .calc_cooling_mixed_air_call
                        .latest
                        .as_mut()
                        .expect("CP329");
                    let value = match corruption {
                        Corruption::RecirculationTemperature => latest
                            .recirculation_temperature_c
                            .as_mut()
                            .expect("CP329 recirculation temperature"),
                        Corruption::MixedAirTemperature => latest
                            .mixed_air_temperature_c
                            .as_mut()
                            .expect("CP329 mixed-air temperature"),
                        _ => unreachable!(),
                    };
                    *value = f64::from_bits(value.to_bits() + 1);
                    *latest
                };
                runtime.set_cooling_mixed_air_call_latest_witness(system.id, corrupted);
            }
        }
        let before = runtime.clone();
        assert_eq!(
            advance_direct_no_oa_calc_cooling_positive_supply_temperature_assignment(
                &mut runtime,
                &system,
                predecessor,
                &zone_state,
            ),
            Err(
                PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentError::
                    RuntimeStateInvariantViolation { system: system.id }
            )
        );
        assert_eq!(runtime, before);
    }
}

#[test]
fn recursive_cp330_or_cp331_counter_corruption_fails_closed_without_mutation() {
    for corrupt_cp331 in [false, true] {
        let (mut runtime, system, predecessor, zone_state) = completed_cp331_case();
        let unit = runtime.units.get_mut(&system.id).expect("known unit");
        if corrupt_cp331 {
            unit.calc_cooling_positive_supply_cp_air_assignment
                .source_site_execution_count += 1;
        } else {
            unit.calc_cooling_supply_mass_flow_positive_guard
                .source_site_execution_count += 1;
        }
        let before = runtime.clone();

        assert_eq!(
            advance_direct_no_oa_calc_cooling_positive_supply_temperature_assignment(
                &mut runtime,
                &system,
                predecessor,
                &zone_state,
            ),
            Err(
                PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentError::
                    RuntimeStateInvariantViolation { system: system.id }
            )
        );
        assert_eq!(runtime, before);
    }
}

#[test]
fn cp318_optional_load_lineage_allows_generic_humidity_driven_positive_route_shape() {
    let (runtime, system, predecessor, zone_state) = completed_cp331_case();
    let unit = runtime.units.get(&system.id).expect("known unit");
    let entry = unit.calc_entry.latest.expect("CP310");
    let mut sensible = unit.calc_cooling_sensible_flow.latest.expect("CP318");
    let mixed_air = unit.calc_cooling_mixed_air_call.latest.expect("CP329");
    let positive_guard = unit
        .calc_cooling_supply_mass_flow_positive_guard
        .latest
        .expect("CP330");
    sensible.zone_cooling_setpoint_load_read = false;
    sensible.zone_cooling_setpoint_load_w = None;

    assert!(
        super::super::release::active_operands_link_to_retained_prefix_for_test(
            entry,
            sensible,
            mixed_air,
            positive_guard,
            predecessor,
            zone_state.mean_air_temperature_c,
        )
    );
}
