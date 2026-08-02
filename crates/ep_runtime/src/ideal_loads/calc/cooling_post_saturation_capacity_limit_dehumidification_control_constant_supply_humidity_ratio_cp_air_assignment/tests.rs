//! CP399 source boundary, exhaustive routes, binary64, corruption, and overflow tests.

use super::*;
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry::tests::fixtures;
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment::tests::release_corruption::completed_cp391_case;
use crate::psychrometrics::energyplus_psy_cp_air_fn_w;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatCaseBreakRuntimeState as Cp396State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatSupplyHumidityRatioAssignmentRuntimeState as Cp395State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlNoneCaseEntryRuntimeState as Cp397State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCaseEntryRuntimeState as Cp398State,
    PurchasedAirRuntimeState,
};
use crate::ideal_loads::calc::{
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_break_state,
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment_state,
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry_state,
};
use ep_model::{DehumidificationControlType, IdealLoadsAirSystem};

type State = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCpAirAssignmentRuntimeState;
type Snapshot = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCpAirAssignmentSnapshot;
type Predecessor = crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCaseEntrySnapshot;
type ActiveInput = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCpAirAssignmentActiveInput;

#[test]
fn cp399_boundary_and_three_source_sites_are_exact() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CP_AIR_ASSIGNMENT_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2294",
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2295",
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CP_AIR_ASSIGNMENT_SOURCE_ORDER,
        &[
            "read-purchased-air-mixed-air-humidity-ratio-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-cp-air",
            "evaluate-psy-cp-air-fn-w-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-cp-air",
            "assign-local-cp-air-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-case",
        ],
    );
}

#[test]
fn thirty_routes_execute_exactly_six_assignments_and_preserve_carriers() {
    let predecessors = cp398_snapshots();
    let mut state = State::new(predecessors[0].system);
    let humidity = 0.007_25;
    let cp_air = energyplus_psy_cp_air_fn_w(humidity);
    for (index, predecessor) in predecessors.into_iter().enumerate() {
        let active = matches!(index, 20 | 21 | 24 | 25 | 27 | 29);
        let input = active.then_some(ActiveInput { mixed_air_humidity_ratio: humidity });
        let snapshot = advance(&mut state, predecessor, input).expect("CP399 route");
        assert!(cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_snapshot_is_exact(snapshot));
        assert_eq!(
            cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_snapshot_is_exact_direct_release(snapshot),
            matches!(index, 0..=8 | 20 | 24),
        );
        assert_eq!(snapshot.dehumidification_control_none_or_constant_supply_humidity_ratio_cp_air_assignment_executed, active);
        assert_eq!(snapshot.mixed_air_humidity_ratio_read, active);
        assert_eq!(snapshot.psychrometric_cp_air_evaluated, active);
        assert_eq!(snapshot.cp_air_assigned, active);
        assert_eq!(snapshot.mixed_air_humidity_ratio.map(f64::to_bits), active.then_some(humidity.to_bits()));
        assert_eq!(snapshot.psychrometric_cp_air_result_j_per_kg_k.map(f64::to_bits), active.then_some(cp_air.to_bits()));
        assert_eq!(snapshot.cp_air_j_per_kg_k.map(f64::to_bits), active.then_some(cp_air.to_bits()));
        assert_bits_eq(snapshot.predecessor_cp398_resulting_supply_humidity_ratio, predecessor.resulting_supply_humidity_ratio);
        assert_bits_eq(snapshot.predecessor_cp398_resulting_supply_enthalpy_j_per_kg, predecessor.resulting_supply_enthalpy_j_per_kg);
        assert_bits_eq(snapshot.predecessor_cp398_resulting_supply_temperature_c, predecessor.resulting_supply_temperature_c);
        assert_bits_eq(snapshot.resulting_supply_humidity_ratio, predecessor.resulting_supply_humidity_ratio);
        assert_bits_eq(snapshot.resulting_supply_enthalpy_j_per_kg, predecessor.resulting_supply_enthalpy_j_per_kg);
        assert_bits_eq(snapshot.resulting_supply_temperature_c, predecessor.resulting_supply_temperature_c);
        assert_eq!(snapshot.resulting_supply_humidity_ratio.is_some(), matches!(index, 18 | 19 | 22 | 23 | 26 | 28));
        assert_eq!(snapshot.resulting_supply_enthalpy_j_per_kg.is_some(), matches!(index, 5 | 8 | 11 | 14 | 17..=29));
        assert_eq!(snapshot.resulting_supply_temperature_c.is_some(), index >= 3);
    }
    assert_eq!(state.transition_count, 30);
    assert_eq!(state.inactive_transition_count, 24);
    assert_eq!(state.dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_count, 6);
    assert_eq!(state.predecessor_route_counts, [1; 30]);
    assert_eq!(state.source_site_execution_count, 18);
    assert_eq!(state.mixed_air_humidity_ratio_read_count, 6);
    assert_eq!(state.psychrometric_cp_air_evaluation_count, 6);
    assert_eq!(state.cp_air_assignment_write_count, 6);
}

#[test]
fn activity_payload_and_nonfinite_values_reject_before_mutation() {
    let predecessors = cp398_snapshots();
    let active = predecessors[20];
    let inactive = predecessors[19];
    for (predecessor, input) in [
        (active, None),
        (active, Some(ActiveInput { mixed_air_humidity_ratio: -0.001 })),
        (active, Some(ActiveInput { mixed_air_humidity_ratio: f64::NAN })),
        (active, Some(ActiveInput { mixed_air_humidity_ratio: f64::INFINITY })),
        (inactive, Some(ActiveInput { mixed_air_humidity_ratio: 0.007 })),
    ] {
        let mut state = State::new(predecessor.system);
        let before = state.clone();
        assert!(advance(&mut state, predecessor, input).is_none());
        assert_eq!(state, before);
    }

    for humidity in [-0.0, 0.0, f64::from_bits(1)] {
        let mut state = State::new(active.system);
        let snapshot = advance(&mut state, active, Some(ActiveInput { mixed_air_humidity_ratio: humidity })).expect("source-valid finite humidity");
        assert_eq!(snapshot.mixed_air_humidity_ratio.map(f64::to_bits), Some(humidity.to_bits()));
        assert_eq!(snapshot.cp_air_j_per_kg_k.map(f64::to_bits), Some(energyplus_psy_cp_air_fn_w(humidity).to_bits()));
    }
}

#[test]
fn twelve_numeric_fields_and_four_local_flags_are_exactly_validated() {
    let predecessor = cp398_snapshots()[20];
    let mut state = State::new(predecessor.system);
    let snapshot = advance(&mut state, predecessor, Some(ActiveInput { mixed_air_humidity_ratio: 0.007 })).expect("active CP399");
    let mutations: &[fn(&mut Snapshot)] = &[
        |s| s.predecessor_cp397_resulting_supply_humidity_ratio = flip(s.predecessor_cp397_resulting_supply_humidity_ratio),
        |s| s.predecessor_cp397_resulting_supply_enthalpy_j_per_kg = flip(s.predecessor_cp397_resulting_supply_enthalpy_j_per_kg),
        |s| s.predecessor_cp397_resulting_supply_temperature_c = flip(s.predecessor_cp397_resulting_supply_temperature_c),
        |s| s.predecessor_cp398_resulting_supply_humidity_ratio = flip(s.predecessor_cp398_resulting_supply_humidity_ratio),
        |s| s.predecessor_cp398_resulting_supply_enthalpy_j_per_kg = flip(s.predecessor_cp398_resulting_supply_enthalpy_j_per_kg),
        |s| s.predecessor_cp398_resulting_supply_temperature_c = flip(s.predecessor_cp398_resulting_supply_temperature_c),
        |s| s.mixed_air_humidity_ratio = Some(0.123),
        |s| s.psychrometric_cp_air_result_j_per_kg_k = flip(s.psychrometric_cp_air_result_j_per_kg_k),
        |s| s.cp_air_j_per_kg_k = flip(s.cp_air_j_per_kg_k),
        |s| s.resulting_supply_humidity_ratio = flip(s.resulting_supply_humidity_ratio),
        |s| s.resulting_supply_enthalpy_j_per_kg = flip(s.resulting_supply_enthalpy_j_per_kg),
        |s| s.resulting_supply_temperature_c = flip(s.resulting_supply_temperature_c),
        |s| s.dehumidification_control_none_or_constant_supply_humidity_ratio_cp_air_assignment_executed = !s.dehumidification_control_none_or_constant_supply_humidity_ratio_cp_air_assignment_executed,
        |s| s.mixed_air_humidity_ratio_read = !s.mixed_air_humidity_ratio_read,
        |s| s.psychrometric_cp_air_evaluated = !s.psychrometric_cp_air_evaluated,
        |s| s.cp_air_assigned = !s.cp_air_assigned,
    ];
    for (index, mutate) in mutations.iter().enumerate() {
        let mut corrupted = snapshot;
        mutate(&mut corrupted);
        assert!(
            !cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_snapshot_is_exact(corrupted),
            "corruption mutation {index} was accepted",
        );
    }
}

#[test]
fn every_counter_overflow_is_transactional() {
    let predecessors = cp398_snapshots();
    let active = predecessors[20];
    let setters: &[fn(&mut State)] = &[
        |s| s.transition_count = usize::MAX,
        |s| s.predecessor_route_counts[20] = usize::MAX,
        |s| s.dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_count = usize::MAX,
        |s| s.source_site_execution_count = usize::MAX,
        |s| s.mixed_air_humidity_ratio_read_count = usize::MAX,
        |s| s.psychrometric_cp_air_evaluation_count = usize::MAX,
        |s| s.cp_air_assignment_write_count = usize::MAX,
    ];
    for set in setters {
        let mut state = State::new(active.system);
        set(&mut state);
        let before = state.clone();
        assert!(advance(&mut state, active, Some(ActiveInput { mixed_air_humidity_ratio: 0.007 })).is_none());
        assert_eq!(state, before);
    }
    let inactive = predecessors[19];
    let mut state = State::new(inactive.system);
    state.inactive_transition_count = usize::MAX;
    let before = state.clone();
    assert!(advance(&mut state, inactive, None).is_none());
    assert_eq!(state, before);
}

#[test]
fn direct_release_derives_active_operand_from_cp329_owner_and_commits_once() {
    let (mut runtime, system, predecessor) = completed_cp398_case();
    let owner = runtime
        .units
        .get(&system.id)
        .and_then(|unit| unit.calc_cooling_mixed_air_call.latest)
        .and_then(|owner| owner.mixed_air_humidity_ratio)
        .expect("CP329 mixed-air humidity owner");
    let snapshot = advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment(
        &mut runtime,
        &system,
        predecessor,
    )
    .expect("CP399 direct release");
    assert!(cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_snapshot_is_exact_direct_release(snapshot));
    assert!(snapshot.dehumidification_control_none_or_constant_supply_humidity_ratio_cp_air_assignment_executed);
    assert_eq!(snapshot.mixed_air_humidity_ratio.map(f64::to_bits), Some(owner.to_bits()));
    assert_eq!(
        snapshot.cp_air_j_per_kg_k.map(f64::to_bits),
        Some(energyplus_psy_cp_air_fn_w(owner).to_bits()),
    );
    let state = &runtime.units[&system.id]
        .calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment;
    assert_eq!(state.transition_count, 1);
    assert_eq!(state.dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_count, 1);
    assert_eq!(state.source_site_execution_count, 3);
}

#[test]
fn cp329_owner_and_cp398_boundary_corruption_reject_transactionally() {
    let (runtime, system, predecessor) = completed_cp398_case();

    let mut owner_drift = runtime.clone();
    let owner = owner_drift.units.get_mut(&system.id).expect("unit")
        .calc_cooling_mixed_air_call.latest.as_mut().expect("CP329 latest");
    owner.mixed_air_humidity_ratio = flip(owner.mixed_air_humidity_ratio);
    assert_release_rejected_unchanged(&mut owner_drift, &system, predecessor);

    let mut predecessor_latest_drift = runtime.clone();
    predecessor_latest_drift.units.get_mut(&system.id).expect("unit")
        .calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry.latest.as_mut().expect("CP398 latest")
        .source = "forged CP398 source";
    assert_release_rejected_unchanged(&mut predecessor_latest_drift, &system, predecessor);

    let mut argument_drift = runtime;
    let mut forged = predecessor;
    forged.source = "forged passed CP398 source";
    assert_release_rejected_unchanged(&mut argument_drift, &system, forged);
}

fn cp398_snapshots() -> Vec<Predecessor> {
    let chains = fixtures::all_chains();
    let mut state = crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCaseEntryRuntimeState::new(chains[0].cp397.system);
    chains
        .into_iter()
        .map(|chain| {
            crate::ideal_loads::calc::advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry_state(
                &mut state,
                chain.cp397,
            )
            .expect("CP398 predecessor")
        })
        .collect()
}

fn completed_cp398_case() -> (PurchasedAirRuntimeState, IdealLoadsAirSystem, Predecessor) {
    let (mut runtime, system, _) = completed_cp391_case();
    let unit = runtime.units.get(&system.id).expect("selected unit");
    let ordinal = unit.init_call_count;
    let controlled_zone = unit.controlled_zone.expect("controlled zone");
    let mut cp394 = crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment::tests::fixtures::chain(
        3,
        1,
        true,
        Some(DehumidificationControlType::None),
        ordinal,
        0.7,
        18.0,
        1.0,
    )
    .cp394;
    cp394.system = system.id;
    cp394.parent_call_ordinal = ordinal;
    cp394.controlled_zone = controlled_zone;

    let mut cp395_state = Cp395State::new(system.id);
    let cp395 = advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment_state(&mut cp395_state, cp394).expect("route-20 CP395");
    let mut cp396_state = Cp396State::new(system.id);
    let cp396 = advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_break_state(&mut cp396_state, cp395).expect("route-20 CP396");
    let mut cp397_state = Cp397State::new(system.id);
    let cp397 = advance_cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry_state(&mut cp397_state, cp396).expect("route-20 CP397");
    let mut cp398_state = Cp398State::new(system.id);
    let cp398 = crate::ideal_loads::calc::advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry_state(&mut cp398_state, cp397).expect("route-20 CP398");

    let unit = runtime.units.get_mut(&system.id).expect("selected unit");
    unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment = cp395_state;
    unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_break = cp396_state;
    unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry = cp397_state;
    unit.calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry = cp398_state;
    runtime.set_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment_latest_witness(system.id, cp395);
    runtime.set_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_break_latest_witness(system.id, cp396);
    runtime.set_cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry_latest_witness(system.id, cp397);
    runtime.set_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry_latest_witness(system.id, cp398);
    (runtime, system, cp398)
}

fn assert_release_rejected_unchanged(runtime: &mut PurchasedAirRuntimeState, system: &IdealLoadsAirSystem, predecessor: Predecessor) {
    let before = runtime.clone();
    assert!(advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment(runtime, system, predecessor).is_err());
    assert_eq!(*runtime, before);
}

fn advance(state: &mut State, predecessor: Predecessor, active_input: Option<ActiveInput>) -> Option<Snapshot> {
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_state(state, predecessor, active_input)
}

fn flip(value: Option<f64>) -> Option<f64> {
    match value {
        Some(value) => Some(f64::from_bits(value.to_bits() ^ 1)),
        None => Some(0.123),
    }
}

fn assert_bits_eq(left: Option<f64>, right: Option<f64>) {
    assert_eq!(left.map(f64::to_bits), right.map(f64::to_bits));
}
