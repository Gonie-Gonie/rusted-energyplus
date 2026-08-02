//! CP401 source boundary, exhaustive routes, owners, IEEE, and overflow tests.

// This file is included only by its parent's `cfg(test)` module declaration.
#[cfg(test)]
const _: () = ();

use ep_model::DehumidificationControlType as D;

use super::*;
use crate::ideal_loads::calc::cooling_mixed_air_call::tests::{
    Route as MixedAirRoute, predecessor as mixed_air_predecessor,
};
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment::tests::fixtures as cp388_fixtures;
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry::tests::fixtures as cp398_fixtures;
use crate::ideal_loads::calc::{
    PurchasedAirCalcCoolingMixedAirCallActiveInput,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioSensibleOutputAssignmentActiveOwners as Cp400Owners,
    advance_cooling_mixed_air_call_state,
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry_state,
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_state,
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_state,
    advance_cooling_supply_mass_flow_positive_guard_state,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingMixedAirCallRuntimeState,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCaseEntryRuntimeState as Cp398State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCaseEntrySnapshot as Cp398,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCpAirAssignmentActiveInput as Cp399Input,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCpAirAssignmentRuntimeState as Cp399State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioSensibleOutputAssignmentRuntimeState as Cp400State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioSensibleOutputAssignmentSnapshot as Predecessor,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardRuntimeState,
};

type State = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputAssignmentRuntimeState;
type Snapshot = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputAssignmentSnapshot;
type ActiveOwners = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputAssignmentActiveOwners;

#[derive(Clone, Copy)]
struct Chain {
    predecessor: Predecessor,
    active_owners: Option<ActiveOwners>,
}

#[test]
fn cp401_boundary_and_four_source_sites_are_exact() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_ASSIGNMENT_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2296",
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2297",
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
        &[
            "read-retained-cooling-total-output-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-latent-output-difference",
            "read-local-cooling-sensible-output-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-latent-output-difference",
            "calculate-cooling-total-output-minus-cooling-sensible-output-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-latent-output",
            "assign-local-cooling-latent-output-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-case",
        ],
    );
}

#[test]
fn thirty_routes_execute_exactly_six_assignments_and_preserve_all_carriers() {
    let chains = all_chains();
    let mut state = State::new(chains[0].predecessor.system);
    for (index, chain) in chains.into_iter().enumerate() {
        let active = matches!(index, 20 | 21 | 24 | 25 | 27 | 29);
        assert_eq!(chain.active_owners.is_some(), active);
        let snapshot = advance(&mut state, chain.predecessor, chain.active_owners)
            .expect("CP401 route");
        assert!(cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment_snapshot_is_exact(snapshot));
        assert_eq!(
            cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment_snapshot_is_exact_direct_release(snapshot),
            matches!(index, 0..=8 | 20 | 24),
        );
        assert_eq!(
            snapshot.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_assignment_executed,
            active,
        );
        assert_eq!(snapshot.cooling_total_output_read, active);
        assert_eq!(snapshot.cooling_sensible_output_read, active);
        assert_eq!(snapshot.cooling_latent_output_calculated, active);
        assert_eq!(snapshot.cooling_latent_output_assigned, active);
        assert_bits_eq(
            snapshot.predecessor_cp400_resulting_supply_humidity_ratio,
            chain.predecessor.resulting_supply_humidity_ratio,
        );
        assert_bits_eq(
            snapshot.predecessor_cp400_resulting_supply_enthalpy_j_per_kg,
            chain.predecessor.resulting_supply_enthalpy_j_per_kg,
        );
        assert_bits_eq(
            snapshot.predecessor_cp400_resulting_supply_temperature_c,
            chain.predecessor.resulting_supply_temperature_c,
        );
        assert_bits_eq(
            snapshot.resulting_supply_humidity_ratio,
            chain.predecessor.resulting_supply_humidity_ratio,
        );
        assert_bits_eq(
            snapshot.resulting_supply_enthalpy_j_per_kg,
            chain.predecessor.resulting_supply_enthalpy_j_per_kg,
        );
        assert_bits_eq(
            snapshot.resulting_supply_temperature_c,
            chain.predecessor.resulting_supply_temperature_c,
        );
        if let Some(owners) = chain.active_owners {
            let total = owners
                .cooling_total_output_owner
                .resulting_cooling_total_output_w
                .expect("CP384 total output");
            let sensible = chain
                .predecessor
                .cooling_sensible_output_w
                .expect("CP400 sensible output");
            let expected = total - sensible;
            assert_bits_eq(snapshot.cooling_total_output_w, Some(total));
            assert_bits_eq(snapshot.cooling_sensible_output_w, Some(sensible));
            assert_bits_eq(snapshot.calculated_cooling_latent_output_w, Some(expected));
            assert_bits_eq(snapshot.cooling_latent_output_w, Some(expected));
        } else {
            assert!(snapshot.cooling_total_output_w.is_none());
            assert!(snapshot.cooling_sensible_output_w.is_none());
            assert!(snapshot.calculated_cooling_latent_output_w.is_none());
            assert!(snapshot.cooling_latent_output_w.is_none());
        }
    }
    assert_eq!(state.transition_count, 30);
    assert_eq!(state.inactive_transition_count, 24);
    assert_eq!(
        state.dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment_count,
        6,
    );
    assert_eq!(state.predecessor_route_counts, [1; 30]);
    assert_eq!(state.source_site_execution_count, 24);
    assert_eq!(state.cp400_supply_humidity_ratio_state_owner_count, 6);
    assert_eq!(state.cp400_supply_enthalpy_state_owner_count, 17);
    assert_eq!(state.cp400_supply_temperature_state_owner_count, 27);
    assert_eq!(state.unchanged_supply_humidity_ratio_preservation_count, 6);
    assert_eq!(state.unchanged_supply_enthalpy_preservation_count, 17);
    assert_eq!(state.unchanged_supply_temperature_preservation_count, 27);
    for count in active_counts(&state) {
        assert_eq!(count, 6);
    }
}

#[test]
fn owner_cardinality_identity_and_bits_reject_transactionally() {
    let chains = all_chains();
    let active = chains[20];
    let inactive = chains[19];
    let owners = active.active_owners.expect("active owner bundle");
    let mut wrong_identity = owners;
    wrong_identity
        .cooling_total_output_corroborator
        .parent_call_ordinal += 1;
    let mut wrong_bits = owners;
    wrong_bits
        .cooling_total_output_corroborator
        .cooling_total_output_w = flip(
        wrong_bits
            .cooling_total_output_corroborator
            .cooling_total_output_w,
    );
    for (predecessor, input) in [
        (active.predecessor, None),
        (inactive.predecessor, Some(owners)),
        (active.predecessor, Some(wrong_identity)),
        (active.predecessor, Some(wrong_bits)),
    ] {
        let mut state = State::new(predecessor.system);
        let before = state.clone();
        assert!(advance(&mut state, predecessor, input).is_none());
        assert_eq!(state, before);
    }
}

#[test]
fn native_binary64_nonfinite_difference_bits_are_preserved() {
    let chain = chain(
        3,
        1,
        true,
        Some(D::None),
        1,
        0.7,
        18.0,
        1.0,
        f64::INFINITY,
    );
    let owners = chain.active_owners.expect("active owners");
    let total = owners
        .cooling_total_output_owner
        .resulting_cooling_total_output_w
        .expect("total");
    let sensible = chain
        .predecessor
        .cooling_sensible_output_w
        .expect("nonfinite sensible");
    let expected = total - sensible;
    assert!(!expected.is_finite());

    let mut state = State::new(chain.predecessor.system);
    let snapshot = advance(&mut state, chain.predecessor, Some(owners))
        .expect("source-valid nonfinite CP401 route");
    assert_bits_eq(snapshot.calculated_cooling_latent_output_w, Some(expected));
    assert_bits_eq(snapshot.cooling_latent_output_w, Some(expected));
    assert!(cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment_snapshot_is_exact(snapshot));
}

#[test]
fn local_formula_carrier_and_flag_corruption_is_rejected() {
    let chain = all_chains()[20];
    let mut state = State::new(chain.predecessor.system);
    let snapshot = advance(&mut state, chain.predecessor, chain.active_owners)
        .expect("active CP401");
    let mutations: &[fn(&mut Snapshot)] = &[
        |s| s.predecessor_cp400_resulting_supply_temperature_c =
            flip(s.predecessor_cp400_resulting_supply_temperature_c),
        |s| s.cooling_total_output_w = flip_sign(s.cooling_total_output_w),
        |s| s.cooling_sensible_output_w = flip_sign(s.cooling_sensible_output_w),
        |s| s.calculated_cooling_latent_output_w =
            flip(s.calculated_cooling_latent_output_w),
        |s| s.cooling_latent_output_w = flip(s.cooling_latent_output_w),
        |s| s.resulting_supply_enthalpy_j_per_kg =
            flip(s.resulting_supply_enthalpy_j_per_kg),
        |s| s.cp384_retained_cooling_total_output_owned_read = false,
        |s| s.cp385_cooling_total_output_bit_corroborated = false,
        |s| s.cp400_retained_cooling_sensible_output_owned_read = false,
        |s| s.cooling_latent_output_assigned = false,
    ];
    for (index, mutate) in mutations.iter().enumerate() {
        let mut corrupted = snapshot;
        mutate(&mut corrupted);
        assert!(
            !cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment_snapshot_is_exact(corrupted),
            "corruption mutation {index} was accepted",
        );
    }
}

#[test]
fn every_cp401_counter_overflow_is_transactional() {
    let chain = all_chains()[20];
    let owners = chain.active_owners.expect("active owners");
    let setters: &[fn(&mut State)] = &[
        |s| s.transition_count = usize::MAX,
        |s| s.predecessor_route_counts[20] = usize::MAX,
        |s| {
            s.dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment_count = usize::MAX
        },
        |s| s.source_site_execution_count = usize::MAX,
        |s| s.cp400_supply_enthalpy_state_owner_count = usize::MAX,
        |s| s.unchanged_supply_enthalpy_preservation_count = usize::MAX,
        |s| s.cp400_supply_temperature_state_owner_count = usize::MAX,
        |s| s.unchanged_supply_temperature_preservation_count = usize::MAX,
        |s| s.cooling_total_output_owned_read_count = usize::MAX,
        |s| s.cooling_total_output_bit_corroboration_count = usize::MAX,
        |s| s.cooling_total_output_read_count = usize::MAX,
        |s| s.cooling_sensible_output_owned_read_count = usize::MAX,
        |s| s.cooling_sensible_output_read_count = usize::MAX,
        |s| s.cooling_latent_output_calculation_count = usize::MAX,
        |s| s.cooling_latent_output_assignment_write_count = usize::MAX,
    ];
    for set in setters {
        let mut state = State::new(chain.predecessor.system);
        set(&mut state);
        let before = state.clone();
        assert!(advance(&mut state, chain.predecessor, Some(owners)).is_none());
        assert_eq!(state, before);
    }

    let humidity_route = all_chains()[18];
    for set in [
        (|s: &mut State| s.cp400_supply_humidity_ratio_state_owner_count = usize::MAX)
            as fn(&mut State),
        |s: &mut State| s.unchanged_supply_humidity_ratio_preservation_count = usize::MAX,
    ] {
        let mut state = State::new(humidity_route.predecessor.system);
        set(&mut state);
        let before = state.clone();
        assert!(advance(&mut state, humidity_route.predecessor, None).is_none());
        assert_eq!(state, before);
    }

    let inactive = all_chains()[0];
    let mut state = State::new(inactive.predecessor.system);
    state.inactive_transition_count = usize::MAX;
    let before = state.clone();
    assert!(advance(&mut state, inactive.predecessor, None).is_none());
    assert_eq!(state, before);
}

fn all_chains() -> Vec<Chain> {
    let mut chains = Vec::new();
    let mut ordinal = 1;
    for inherited in 0..3 {
        chains.push(chain(
            inherited, 0, false, None, ordinal, 0.7, 18.0, 1.0, 2.0,
        ));
        ordinal += 1;
    }
    for inherited in 3..8 {
        for outcome in [0, 2, 1] {
            chains.push(chain(
                inherited, outcome, false, None, ordinal, 0.7, 18.0, 1.0, 2.0,
            ));
            ordinal += 1;
        }
    }
    for inherited in [3, 4] {
        for selector in [
            D::ConstantSensibleHeatRatio,
            D::Humidistat,
            D::None,
            D::ConstantSupplyHumidityRatio,
        ] {
            chains.push(chain(
                inherited,
                1,
                true,
                Some(selector),
                ordinal,
                0.7,
                18.0,
                1.0,
                2.0,
            ));
            ordinal += 1;
        }
    }
    for (inherited, selectors) in [
        (5, &[D::Humidistat][..]),
        (6, &[D::None][..]),
        (
            7,
            &[D::ConstantSensibleHeatRatio, D::ConstantSupplyHumidityRatio][..],
        ),
    ] {
        for selector in selectors {
            chains.push(chain(
                inherited,
                1,
                true,
                Some(*selector),
                ordinal,
                0.65,
                19.0,
                1.0,
                2.0,
            ));
            ordinal += 1;
        }
    }
    chains
}

#[allow(clippy::too_many_arguments)]
fn chain(
    inherited: usize,
    outcome: usize,
    assignment: bool,
    selector: Option<D>,
    ordinal: usize,
    ratio: f64,
    supply_temperature_c: f64,
    flow: f64,
    formula_flow: f64,
) -> Chain {
    let base = cp398_fixtures::chain(
        inherited,
        outcome,
        assignment,
        selector,
        ordinal,
        ratio,
        supply_temperature_c,
        flow,
    );
    let mut cp398_state = Cp398State::new(base.cp397.system);
    let cp398: Cp398 = advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry_state(
        &mut cp398_state,
        base.cp397,
    )
    .expect("CP398");
    let active = assignment
        && matches!(
            selector,
            Some(D::None | D::ConstantSupplyHumidityRatio)
        );
    let mut cp399_state = Cp399State::new(cp398.system);
    let cp399 = advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_state(
        &mut cp399_state,
        cp398,
        active.then_some(Cp399Input {
            mixed_air_humidity_ratio: 0.007_25,
        }),
    )
    .expect("CP399");
    let mut cp400_state = Cp400State::new(cp399.system);
    let cp400 = advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_state(
        &mut cp400_state,
        cp399,
        active.then(|| cp400_owners(cp399, formula_flow, 24.0)),
    )
    .expect("CP400");

    let owner_chain = cp388_fixtures::chain(
        inherited,
        outcome,
        assignment,
        selector,
        ordinal,
        if assignment { 99.0 } else { 100.0 },
        50_000.0,
        0.008,
    );
    let active_owners = active.then_some(ActiveOwners {
        cooling_total_output_owner: owner_chain.cp384,
        cooling_total_output_corroborator: owner_chain.cp385,
    });
    if let Some(owners) = active_owners {
        assert_bits_eq(
            cp400.resulting_supply_enthalpy_j_per_kg,
            owners
                .cooling_total_output_corroborator
                .resulting_supply_enthalpy_j_per_kg,
        );
    }
    Chain {
        predecessor: cp400,
        active_owners,
    }
}

fn cp400_owners(predecessor: crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCpAirAssignmentSnapshot, flow: f64, mixed_temperature: f64) -> Cp400Owners {
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
    Cp400Owners {
        mixed_air_owner,
        supply_mass_flow_owner,
    }
}

fn advance(
    state: &mut State,
    predecessor: Predecessor,
    active_owners: Option<ActiveOwners>,
) -> Option<Snapshot> {
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment_state(
        state,
        predecessor,
        active_owners,
    )
}

fn active_counts(state: &State) -> [usize; 7] {
    [
        state.cooling_total_output_owned_read_count,
        state.cooling_total_output_bit_corroboration_count,
        state.cooling_total_output_read_count,
        state.cooling_sensible_output_owned_read_count,
        state.cooling_sensible_output_read_count,
        state.cooling_latent_output_calculation_count,
        state.cooling_latent_output_assignment_write_count,
    ]
}

fn flip(value: Option<f64>) -> Option<f64> {
    match value {
        Some(value) => Some(f64::from_bits(value.to_bits() ^ 1)),
        None => Some(0.123),
    }
}

fn flip_sign(value: Option<f64>) -> Option<f64> {
    match value {
        Some(value) => Some(f64::from_bits(value.to_bits() ^ (1_u64 << 63))),
        None => Some(0.123),
    }
}

fn assert_bits_eq(left: Option<f64>, right: Option<f64>) {
    assert_eq!(left.map(f64::to_bits), right.map(f64::to_bits));
}
