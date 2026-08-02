//! CP400 source boundary, exhaustive routes, owners, IEEE, and overflow tests.

use super::*;
use crate::ideal_loads::calc::cooling_mixed_air_call::tests::{
    Route as MixedAirRoute, predecessor as mixed_air_predecessor,
};
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry::tests::fixtures;
use crate::ideal_loads::calc::{
    PurchasedAirCalcCoolingMixedAirCallActiveInput,
    advance_cooling_mixed_air_call_state,
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry_state,
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_state,
    advance_cooling_supply_mass_flow_positive_guard_state,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingMixedAirCallRuntimeState,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCaseEntryRuntimeState as Cp398State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCaseEntrySnapshot as Cp398,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCpAirAssignmentActiveInput as Cp399Input,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCpAirAssignmentRuntimeState as Cp399State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCpAirAssignmentSnapshot as Predecessor,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardRuntimeState,
};
use crate::psychrometrics::energyplus_psy_cp_air_fn_w;

type State = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioSensibleOutputAssignmentRuntimeState;
type Snapshot = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioSensibleOutputAssignmentSnapshot;
type ActiveOwners = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioSensibleOutputAssignmentActiveOwners;

#[test]
fn cp400_boundary_and_eight_source_sites_are_exact() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2295",
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2296",
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER.len(),
        8,
    );
}

#[test]
fn thirty_routes_execute_exactly_six_assignments_and_preserve_all_carriers() {
    let predecessors = cp399_snapshots();
    let mut state = State::new(predecessors[0].system);
    for (index, predecessor) in predecessors.into_iter().enumerate() {
        let active = matches!(index, 20 | 21 | 24 | 25 | 27 | 29);
        let owners = active.then(|| owners(predecessor, 2.0, 24.0));
        let snapshot = advance(&mut state, predecessor, owners).expect("CP400 route");
        assert!(cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_snapshot_is_exact(snapshot));
        assert_eq!(
            cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_snapshot_is_exact_direct_release(snapshot),
            matches!(index, 0..=8 | 20 | 24),
        );
        assert_eq!(
            snapshot.dehumidification_control_none_or_constant_supply_humidity_ratio_sensible_output_assignment_executed,
            active,
        );
        assert_eq!(snapshot.supply_mass_flow_rate_read, active);
        assert_eq!(snapshot.cp_air_read, active);
        assert_eq!(snapshot.mixed_air_temperature_read, active);
        assert_eq!(snapshot.supply_temperature_read, active);
        assert_eq!(snapshot.cooling_sensible_output_assigned, active);
        assert_bits_eq(
            snapshot.predecessor_cp399_resulting_supply_humidity_ratio,
            predecessor.resulting_supply_humidity_ratio,
        );
        assert_bits_eq(
            snapshot.predecessor_cp399_resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        );
        assert_bits_eq(
            snapshot.predecessor_cp399_resulting_supply_temperature_c,
            predecessor.resulting_supply_temperature_c,
        );
        assert_bits_eq(
            snapshot.resulting_supply_humidity_ratio,
            predecessor.resulting_supply_humidity_ratio,
        );
        assert_bits_eq(
            snapshot.resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        );
        assert_bits_eq(
            snapshot.resulting_supply_temperature_c,
            predecessor.resulting_supply_temperature_c,
        );
        if active {
            let flow = snapshot.supply_mass_flow_rate_kg_per_s.expect("flow");
            let cp_air = snapshot.cp_air_j_per_kg_k.expect("CpAir");
            let first = flow * cp_air;
            let difference = snapshot.mixed_air_temperature_c.expect("mixed")
                - snapshot.supply_temperature_c.expect("supply");
            assert_eq!(
                snapshot
                    .supply_mass_flow_rate_times_cp_air_w_per_k
                    .map(f64::to_bits),
                Some(first.to_bits()),
            );
            assert_eq!(
                snapshot
                    .mixed_air_minus_supply_temperature_k
                    .map(f64::to_bits),
                Some(difference.to_bits()),
            );
            assert_eq!(
                snapshot.cooling_sensible_output_w.map(f64::to_bits),
                Some((first * difference).to_bits()),
            );
        } else {
            assert!(snapshot.supply_mass_flow_rate_kg_per_s.is_none());
            assert!(snapshot.cooling_sensible_output_w.is_none());
        }
    }
    assert_eq!(state.transition_count, 30);
    assert_eq!(state.inactive_transition_count, 24);
    assert_eq!(
        state.dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_count,
        6,
    );
    assert_eq!(state.predecessor_route_counts, [1; 30]);
    assert_eq!(state.source_site_execution_count, 48);
    assert_eq!(state.cp399_supply_humidity_ratio_state_owner_count, 6);
    assert_eq!(state.cp399_supply_enthalpy_state_owner_count, 17);
    assert_eq!(state.cp399_supply_temperature_state_owner_count, 27);
    assert_eq!(state.unchanged_supply_humidity_ratio_preservation_count, 6);
    assert_eq!(state.unchanged_supply_enthalpy_preservation_count, 17);
    assert_eq!(state.unchanged_supply_temperature_preservation_count, 27);
    for count in active_counts(&state) {
        assert_eq!(count, 6);
    }
}

#[test]
fn active_and_inactive_owner_cardinality_rejects_transactionally() {
    let predecessors = cp399_snapshots();
    let active = predecessors[20];
    let inactive = predecessors[19];
    let owner = owners(active, 2.0, 24.0);
    for (predecessor, input) in [
        (active, None),
        (inactive, Some(owner)),
        (
            active,
            Some(ActiveOwners {
                mixed_air_owner: owner.mixed_air_owner,
                supply_mass_flow_owner: {
                    let mut forged = owner.supply_mass_flow_owner;
                    forged.parent_call_ordinal += 1;
                    forged
                },
            }),
        ),
    ] {
        let mut state = State::new(predecessor.system);
        let before = state.clone();
        assert!(advance(&mut state, predecessor, input).is_none());
        assert_eq!(state, before);
    }
}

#[test]
fn native_binary64_nonfinite_result_bits_are_preserved() {
    let predecessor = cp399_snapshots()[20];
    let supply = predecessor
        .resulting_supply_temperature_c
        .expect("active route temperature");
    let owner = owners(predecessor, f64::INFINITY, supply);
    let mut state = State::new(predecessor.system);
    let snapshot = advance(&mut state, predecessor, Some(owner)).expect("source-valid IEEE route");
    assert_eq!(
        snapshot.supply_mass_flow_rate_kg_per_s.map(f64::to_bits),
        Some(f64::INFINITY.to_bits()),
    );
    assert_eq!(
        snapshot
            .mixed_air_minus_supply_temperature_k
            .map(f64::to_bits),
        Some(0.0f64.to_bits()),
    );
    let expected_first_product = f64::INFINITY
        * predecessor
            .cp_air_j_per_kg_k
            .expect("active route CP399 CpAir");
    let expected_result = expected_first_product * (supply - supply);
    assert_eq!(
        snapshot.cooling_sensible_output_w.map(f64::to_bits),
        Some(expected_result.to_bits()),
    );
    assert!(cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_snapshot_is_exact(snapshot));
}

#[test]
fn local_formula_and_carrier_corruption_is_rejected() {
    let predecessor = cp399_snapshots()[20];
    let mut state = State::new(predecessor.system);
    let snapshot = advance(
        &mut state,
        predecessor,
        Some(owners(predecessor, 2.0, 24.0)),
    )
    .expect("active CP400");
    let mutations: &[fn(&mut Snapshot)] = &[
        |s| s.predecessor_cp_air_j_per_kg_k = flip(s.predecessor_cp_air_j_per_kg_k),
        |s| {
            s.predecessor_cp399_resulting_supply_temperature_c =
                flip(s.predecessor_cp399_resulting_supply_temperature_c)
        },
        |s| s.supply_mass_flow_rate_kg_per_s = flip(s.supply_mass_flow_rate_kg_per_s),
        |s| s.cp_air_j_per_kg_k = flip(s.cp_air_j_per_kg_k),
        |s| {
            s.supply_mass_flow_rate_times_cp_air_w_per_k =
                flip(s.supply_mass_flow_rate_times_cp_air_w_per_k)
        },
        |s| s.mixed_air_temperature_c = flip(s.mixed_air_temperature_c),
        |s| s.supply_temperature_c = flip(s.supply_temperature_c),
        |s| s.mixed_air_minus_supply_temperature_k = flip(s.mixed_air_minus_supply_temperature_k),
        |s| s.calculated_cooling_sensible_output_w = flip(s.calculated_cooling_sensible_output_w),
        |s| s.cooling_sensible_output_w = flip(s.cooling_sensible_output_w),
        |s| s.resulting_supply_enthalpy_j_per_kg = flip(s.resulting_supply_enthalpy_j_per_kg),
        |s| s.cooling_sensible_output_assigned = !s.cooling_sensible_output_assigned,
    ];
    for (index, mutate) in mutations.iter().enumerate() {
        let mut corrupted = snapshot;
        mutate(&mut corrupted);
        assert!(
            !cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_snapshot_is_exact(corrupted),
            "corruption mutation {index} was accepted",
        );
    }
}

#[test]
fn every_counter_overflow_is_transactional() {
    let predecessors = cp399_snapshots();
    let active = predecessors[20];
    let active_owner = owners(active, 2.0, 24.0);
    let setters: &[fn(&mut State)] = &[
        |s| s.transition_count = usize::MAX,
        |s| s.predecessor_route_counts[20] = usize::MAX,
        |s| {
            s.dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_count = usize::MAX
        },
        |s| s.source_site_execution_count = usize::MAX,
        |s| s.cp399_supply_enthalpy_state_owner_count = usize::MAX,
        |s| s.unchanged_supply_enthalpy_preservation_count = usize::MAX,
        |s| s.cp399_supply_temperature_state_owner_count = usize::MAX,
        |s| s.unchanged_supply_temperature_preservation_count = usize::MAX,
        |s| s.supply_mass_flow_rate_owned_read_count = usize::MAX,
        |s| s.supply_mass_flow_rate_bit_corroboration_count = usize::MAX,
        |s| s.supply_mass_flow_rate_read_count = usize::MAX,
        |s| s.cp_air_owned_read_count = usize::MAX,
        |s| s.cp_air_read_count = usize::MAX,
        |s| s.supply_mass_flow_rate_times_cp_air_calculation_count = usize::MAX,
        |s| s.mixed_air_temperature_owned_read_count = usize::MAX,
        |s| s.mixed_air_temperature_read_count = usize::MAX,
        |s| s.supply_temperature_owned_read_count = usize::MAX,
        |s| s.supply_temperature_read_count = usize::MAX,
        |s| s.mixed_air_minus_supply_temperature_calculation_count = usize::MAX,
        |s| s.cooling_sensible_output_calculation_count = usize::MAX,
        |s| s.cooling_sensible_output_assignment_write_count = usize::MAX,
    ];
    for set in setters {
        let mut state = State::new(active.system);
        set(&mut state);
        let before = state.clone();
        assert!(advance(&mut state, active, Some(active_owner)).is_none());
        assert_eq!(state, before);
    }

    let humidity_route = predecessors[18];
    for set in [
        (|s: &mut State| s.cp399_supply_humidity_ratio_state_owner_count = usize::MAX)
            as fn(&mut State),
        |s: &mut State| s.unchanged_supply_humidity_ratio_preservation_count = usize::MAX,
    ] {
        let mut state = State::new(humidity_route.system);
        set(&mut state);
        let before = state.clone();
        assert!(advance(&mut state, humidity_route, None).is_none());
        assert_eq!(state, before);
    }

    let inactive = predecessors[0];
    let mut state = State::new(inactive.system);
    state.inactive_transition_count = usize::MAX;
    let before = state.clone();
    assert!(advance(&mut state, inactive, None).is_none());
    assert_eq!(state, before);
}

fn cp399_snapshots() -> Vec<Predecessor> {
    let chains = fixtures::all_chains();
    let system = chains[0].cp397.system;
    let mut cp398_state = Cp398State::new(system);
    let mut cp399_state = Cp399State::new(system);
    chains
        .into_iter()
        .enumerate()
        .map(|(index, chain)| {
            let cp398: Cp398 = advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry_state(
                &mut cp398_state,
                chain.cp397,
            )
            .expect("CP398");
            let active = matches!(index, 20 | 21 | 24 | 25 | 27 | 29);
            advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_state(
                &mut cp399_state,
                cp398,
                active.then_some(Cp399Input {
                    mixed_air_humidity_ratio: 0.007_25,
                }),
            )
            .expect("CP399")
        })
        .collect()
}

fn owners(predecessor: Predecessor, flow: f64, mixed_temperature: f64) -> ActiveOwners {
    let mut mixed_predecessor = mixed_air_predecessor(MixedAirRoute::CoolingFallthrough);
    mixed_predecessor.system = predecessor.system;
    mixed_predecessor.parent_call_ordinal = predecessor.parent_call_ordinal;
    mixed_predecessor.controlled_zone = predecessor.controlled_zone;
    let humidity = 0.008;
    let mut mixed_state = PurchasedAirCalcCoolingMixedAirCallRuntimeState::new(predecessor.system);
    let mixed_air_owner = advance_cooling_mixed_air_call_state(
        &mut mixed_state,
        mixed_predecessor,
        Some(PurchasedAirCalcCoolingMixedAirCallActiveInput {
            recirculation_node: ep_model::NodeId(9),
            recirculation_temperature_c: mixed_temperature,
            recirculation_humidity_ratio: humidity,
            recirculation_enthalpy_projection_j_per_kg:
                crate::ideal_loads::moist_air_enthalpy_j_per_kg(mixed_temperature, humidity),
            outdoor_air_mass_flow_rate_kg_per_s: 0.0,
            supply_mass_flow_rate_kg_per_s: flow,
        }),
    );
    let mut flow_state =
        PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardRuntimeState::new(predecessor.system);
    let supply_mass_flow_owner =
        advance_cooling_supply_mass_flow_positive_guard_state(&mut flow_state, mixed_air_owner);
    ActiveOwners {
        mixed_air_owner,
        supply_mass_flow_owner,
    }
}

fn advance(
    state: &mut State,
    predecessor: Predecessor,
    active_owners: Option<ActiveOwners>,
) -> Option<Snapshot> {
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_state(
        state,
        predecessor,
        active_owners,
    )
}

fn active_counts(state: &State) -> [usize; 13] {
    [
        state.supply_mass_flow_rate_owned_read_count,
        state.supply_mass_flow_rate_bit_corroboration_count,
        state.supply_mass_flow_rate_read_count,
        state.cp_air_owned_read_count,
        state.cp_air_read_count,
        state.supply_mass_flow_rate_times_cp_air_calculation_count,
        state.mixed_air_temperature_owned_read_count,
        state.mixed_air_temperature_read_count,
        state.supply_temperature_owned_read_count,
        state.supply_temperature_read_count,
        state.mixed_air_minus_supply_temperature_calculation_count,
        state.cooling_sensible_output_calculation_count,
        state.cooling_sensible_output_assignment_write_count,
    ]
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

#[test]
fn cp_air_owner_is_the_cp399_assigned_value_without_reevaluation() {
    let predecessor = cp399_snapshots()[20];
    let expected = predecessor.cp_air_j_per_kg_k.expect("CP399 CpAir");
    assert_eq!(
        expected.to_bits(),
        energyplus_psy_cp_air_fn_w(
            predecessor
                .mixed_air_humidity_ratio
                .expect("CP399 humidity"),
        )
        .to_bits(),
    );
    let mut state = State::new(predecessor.system);
    let snapshot = advance(
        &mut state,
        predecessor,
        Some(owners(predecessor, 2.0, 24.0)),
    )
    .expect("CP400");
    assert_eq!(
        snapshot.cp_air_j_per_kg_k.map(f64::to_bits),
        Some(expected.to_bits())
    );
}
