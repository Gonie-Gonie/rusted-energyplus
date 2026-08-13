//! Focused CP423 committed-route acceptance and forgery tests.

use super::*;
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment_committed_latest_route as committed;

#[test]
fn cp423_committed_seal_accepts_entry_inactive_guard_false_and_assignment_routes() {
    for predecessor in representative_predecessors() {
        let (unit, snapshot, route) = fixture_unit(predecessor);
        assert_eq!(committed(&unit, snapshot), Some(route));
    }
}

#[test]
fn cp423_committed_seal_rejects_each_route_component_and_state_accounting_forgery() {
    for predecessor in representative_predecessors() {
        let (unit, snapshot, route) = fixture_unit(predecessor);
        let mut cases = Vec::new();

        let mut logical = unit.clone();
        logical.calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment
            .latest_route.as_mut().expect("route").logical_index = (route.logical_index + 1) % 36;
        cases.push(logical);
        let mut active = unit.clone();
        active.calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment
            .latest_route.as_mut().expect("route").active ^= true;
        cases.push(active);
        let mut assignment = unit.clone();
        assignment.calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment
            .latest_route.as_mut().expect("route").assignment_executed ^= true;
        cases.push(assignment);
        let mut count = unit.clone();
        count.calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment
            .transition_count += 1;
        cases.push(count);
        let mut ordinal = unit.clone();
        ordinal.calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment
            .latest_transition_ordinal = Some(0);
        cases.push(ordinal);
        let mut route_count = unit.clone();
        route_count.calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment
            .predecessor_route_counts[route.logical_index] += 1;
        cases.push(route_count);
        let mut partition = unit.clone();
        partition.calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment
            .cooling_sensible_output_supply_temperature_assignment_route_counts[route.logical_index] += 1;
        cases.push(partition);

        for (case_index, forged) in cases.into_iter().enumerate() {
            assert!(committed(&forged, snapshot).is_none(), "route {:?} case {case_index}", route);
        }
    }
}

#[test]
fn cp423_committed_seal_rejects_latest_witness_identity_and_assigned_owner_value_drift() {
    for predecessor in representative_predecessors() {
        let (unit, snapshot, route) = fixture_unit(predecessor);
        let mut cases = Vec::new();

        let mut witness_system = snapshot;
        witness_system.system = ep_model::IdealLoadsAirSystemId(witness_system.system.0.wrapping_add(1));
        cases.push((unit.clone(), witness_system));
        let mut witness_zone = snapshot;
        witness_zone.controlled_zone = ep_model::ZoneId(witness_zone.controlled_zone.0.wrapping_add(1));
        cases.push((unit.clone(), witness_zone));
        let mut witness_ordinal = snapshot;
        witness_ordinal.parent_call_ordinal = witness_ordinal.parent_call_ordinal.wrapping_add(1);
        cases.push((unit.clone(), witness_ordinal));

        let mut latest_system = unit.clone();
        latest_system.calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment
            .latest.as_mut().expect("latest").system = ep_model::IdealLoadsAirSystemId(snapshot.system.0.wrapping_add(1));
        let coordinated = latest_system.calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment.latest.expect("latest");
        cases.push((latest_system, coordinated));
        let mut latest_zone = unit.clone();
        latest_zone.calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment
            .latest.as_mut().expect("latest").controlled_zone = ep_model::ZoneId(snapshot.controlled_zone.0.wrapping_add(1));
        let coordinated = latest_zone.calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment.latest.expect("latest");
        cases.push((latest_zone, coordinated));
        let mut latest_ordinal = unit.clone();
        latest_ordinal.calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment
            .latest.as_mut().expect("latest").parent_call_ordinal = snapshot.parent_call_ordinal.wrapping_add(1);
        let coordinated = latest_ordinal.calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment.latest.expect("latest");
        cases.push((latest_ordinal, coordinated));

        if route.assignment_executed {
            let mut owner = unit.clone();
            let witness = {
                let latest = owner.calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment
                    .latest.as_mut().expect("latest");
                latest.cooling_sensible_output_for_supply_temperature_w = latest.cooling_sensible_output_for_supply_temperature_w.map(flip);
                *latest
            };
            cases.push((owner, witness));

            let mut mixed = unit.clone();
            mixed.calc_cooling_mixed_air_call.latest.as_mut().expect("CP329").mixed_air_temperature_c = Some(f64::from_bits(
                snapshot.mixed_air_temperature_for_sensible_output_supply_temperature_c.expect("mixed").to_bits() ^ 1,
            ));
            cases.push((mixed, snapshot));
            let mut flow = unit.clone();
            flow.calc_cooling_supply_mass_flow_positive_guard.latest.as_mut().expect("CP330").supply_mass_flow_rate_kg_per_s = Some(flip(
                snapshot.supply_mass_flow_rate_for_sensible_output_supply_temperature_kg_per_s.expect("flow"),
            ));
            cases.push((flow, snapshot));
            let mut cp_air = unit.clone();
            cp_air.calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment.latest.as_mut().expect("CP419").cp_air_j_per_kg_k = Some(flip(
                snapshot.cp_air_for_sensible_output_supply_temperature_j_per_kg_k.expect("CpAir"),
            ));
            cases.push((cp_air, snapshot));
        }

        for (case_index, (forged, witness)) in cases.into_iter().enumerate() {
            assert!(committed(&forged, witness).is_none(), "route {:?} case {case_index}", route);
        }
    }
}

fn representative_predecessors() -> [Predecessor; 4] {
    let all = cp422_all_snapshots_for_successor_tests();
    let mut representatives = [
        all.iter().copied().find(|snapshot| snapshot.positive_guard_false_fallthrough_skipped).expect("entry"),
        all.iter().copied().find(|snapshot| cp422_route(*snapshot).is_some_and(|route| !route.active) && !snapshot.positive_guard_false_fallthrough_skipped).expect("inactive"),
        all.iter().copied().find(|snapshot| cp422_route(*snapshot).is_some_and(|route| route.active && !route.assignment_executed)).expect("guard false"),
        all.iter().copied().find(|snapshot| cp422_route(*snapshot).is_some_and(|route| route.assignment_executed)).expect("assignment"),
    ];
    for snapshot in &mut representatives { snapshot.parent_call_ordinal = 1; }
    representatives
}

fn fixture_unit(predecessor: Predecessor) -> (
    crate::ideal_loads::PurchasedAirUnitRuntimeState,
    super::super::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputSupplyTemperatureAssignmentSnapshot,
    Route,
) {
    let desired = cp422_route(predecessor).expect("desired CP422 route");
    let cp421_template = crate::ideal_loads::cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_predecessor_cp421_snapshot(predecessor);
    let cp420_template = crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_predecessor_cp420_snapshot(cp421_template);
    let cp419 = crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_predecessor_cp419_snapshot(cp420_template);
    let cp419_route = crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_snapshot_route(cp419)
        .expect("CP419 route");
    let mut unit = crate::ideal_loads::calc::cp419_fixture_unit_for_successor_tests(cp419);
    align_cp419_unit(&mut unit, cp419, cp419_route, None);
    let mixed = unit.calc_cooling_mixed_air_call.latest.and_then(|snapshot| snapshot.mixed_air_temperature_c);
    let flow = unit.calc_cooling_supply_mass_flow_positive_guard.latest.and_then(|snapshot| snapshot.supply_mass_flow_rate_kg_per_s);
    let mut cp420_state = crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentRuntimeState::new(cp419.system);
    let cp420 = crate::ideal_loads::calc::advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_state(
        &mut cp420_state,
        cp419,
        cp419_route.active.then(|| crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentActiveInput {
            supply_mass_flow_rate_kg_per_s: flow.expect("CP330 flow"),
            mixed_air_temperature_c: mixed.expect("CP329 mixed temperature"),
        }),
    ).expect("CP420");
    unit.calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment = cp420_state;
    let output = cp420.cooling_sensible_output_w;
    let capacity = output.map(|value| if desired.assignment_executed { value } else { value.max(0.0) + 1.0 });
    let mut cp421_state = crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardRuntimeState::new(cp420.system);
    let cp421 = crate::ideal_loads::calc::advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard_state(
        &mut cp421_state,
        cp420,
        output.zip(capacity).map(|(output, capacity)| crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardActiveInput {
            cooling_sensible_output_w: output,
            maximum_total_cooling_capacity_w: capacity,
            cp420_cooling_sensible_output_owned_read: true,
            cp321_maximum_total_cooling_capacity_owned_read: true,
            cp340_same_call_maximum_total_cooling_capacity_bit_corroborated: true,
        }),
    ).expect("CP421");
    assert_eq!(cp421.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_capacity_adjustment_body_entered, desired.assignment_executed);
    unit.calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard = cp421_state;
    align_cp419_unit(&mut unit, cp419, cp419_route, capacity);
    let mut cp422_state = crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputMaximumCapacityAssignmentRuntimeState::new(cp421.system);
    let cp422 = crate::ideal_loads::calc::advance_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment_state(
        &mut cp422_state,
        cp421,
        capacity.map(|capacity| crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputMaximumCapacityAssignmentActiveInput {
            preexisting_cooling_sensible_output_w: output.expect("CP420 output"),
            maximum_total_cooling_capacity_w: capacity,
            cp421_retained_maximum_total_cooling_capacity_owned_read: true,
        }),
    ).expect("CP422");
    unit.calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment = cp422_state;
    let route = successor_route_for(cp422);
    let mut state = State::new(cp422.system);
    let snapshot = advance_validated(&mut state, cp422, route, active_input(cp422)).expect("CP423");
    unit.calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment = state;
    (unit, snapshot, route)
}

fn align_cp419_unit(
    unit: &mut crate::ideal_loads::PurchasedAirUnitRuntimeState,
    snapshot: crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchCpAirAssignmentSnapshot,
    _route: crate::ideal_loads::calc::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchCpAirAssignmentCommittedRoute,
    maximum_capacity: Option<f64>,
) {
    unit.system = snapshot.system;
    unit.controlled_zone = Some(snapshot.controlled_zone);
    unit.init_call_count = snapshot.parent_call_ordinal;
    unit.calc_entry.call_count = snapshot.parent_call_ordinal;
    let latest = unit.calc_cooling_capacity_zero_flow_reset.latest.as_mut().expect("CP321");
    latest.system = snapshot.system;
    latest.controlled_zone = snapshot.controlled_zone;
    latest.parent_call_ordinal = snapshot.parent_call_ordinal;
    unit.calc_cooling_capacity_zero_flow_reset.system = snapshot.system;
    unit.calc_cooling_positive_supply_capacity_limit_sensible_output_guard.system = snapshot.system;
    let latest = unit.calc_cooling_positive_supply_capacity_limit_sensible_output_guard.latest.as_mut().expect("CP340");
    latest.system = snapshot.system;
    latest.controlled_zone = snapshot.controlled_zone;
    latest.parent_call_ordinal = snapshot.parent_call_ordinal;
    let mixed = unit.calc_cooling_mixed_air_call.latest.as_mut().expect("CP329");
    mixed.system = snapshot.system;
    mixed.controlled_zone = snapshot.controlled_zone;
    mixed.parent_call_ordinal = snapshot.parent_call_ordinal;
    unit.calc_cooling_mixed_air_call.system = snapshot.system;
    let flow = unit.calc_cooling_supply_mass_flow_positive_guard.latest.as_mut().expect("CP330");
    flow.system = snapshot.system;
    flow.controlled_zone = snapshot.controlled_zone;
    flow.parent_call_ordinal = snapshot.parent_call_ordinal;
    unit.calc_cooling_supply_mass_flow_positive_guard.system = snapshot.system;
    if let Some(capacity) = maximum_capacity {
        crate::ideal_loads::calc::cooling_positive_supply_capacity_limit_sensible_output_guard::tests::release_fixture::align_completed_cp340_capacity_for_successor_tests(unit, capacity);
    }
}

fn flip(value: f64) -> f64 { f64::from_bits(value.to_bits() ^ 1) }
