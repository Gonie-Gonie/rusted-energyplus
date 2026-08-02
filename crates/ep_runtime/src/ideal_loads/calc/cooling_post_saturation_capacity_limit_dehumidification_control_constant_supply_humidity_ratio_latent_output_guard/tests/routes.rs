//! CP402 boundary, 36-route partition, owner, and evidence-only tests.

use super::fixtures::{active_input, advance, all_predecessors};
use super::super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputGuardRuntimeState as State,
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard_snapshot_is_exact,
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard_snapshot_is_exact_direct_release,
};

const ACTIVE: &[usize] = &[20, 21, 24, 25, 27, 29];

#[test]
fn cp402_boundary_and_four_conditional_source_sites_are_exact() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_GUARD_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2297",
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_GUARD_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2298",
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_GUARD_SOURCE_ORDER,
        &[
            "read-retained-cooling-latent-output-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-maximum-capacity-comparison",
            "read-retained-maximum-total-cooling-capacity-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-latent-output-comparison",
            "compare-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-cooling-latent-output-greater-than-or-equal-to-maximum-total-cooling-capacity",
            "enter-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-latent-output-capacity-adjustment-body-if-comparison-satisfied",
        ],
    );
}

#[test]
fn thirty_six_successors_partition_24_inactive_and_six_false_true_pairs() {
    let predecessors = all_predecessors();
    assert_eq!(predecessors.len(), 30);
    let mut state = State::new(predecessors[0].system);
    let mut public_successors = 0;
    for (index, predecessor) in predecessors.into_iter().enumerate() {
        if ACTIVE.contains(&index) {
            let latent = predecessor.cooling_latent_output_w.expect("active latent");
            assert!(latent.is_finite());
            assert!(latent >= 0.0);
            for (capacity, expected) in [(latent.abs() + 1.0, false), (latent, true)] {
                let snapshot = advance(&mut state, predecessor, active_input(predecessor, capacity))
                    .expect("active CP402 successor");
                assert!(cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard_snapshot_is_exact(snapshot));
                assert_eq!(
                    snapshot.cooling_latent_output_greater_than_or_equal_to_maximum_total_cooling_capacity,
                    Some(expected),
                );
                assert_eq!(
                    snapshot.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_adjustment_body_entered,
                    expected,
                );
                assert_eq!(
                    snapshot.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough,
                    !expected,
                );
                assert_supply_is_unchanged(snapshot, predecessor);
                let public = matches!(index, 20 | 24);
                assert_eq!(
                    cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard_snapshot_is_exact_direct_release(snapshot),
                    public,
                );
                public_successors += usize::from(public);
            }
        } else {
            let snapshot = advance(&mut state, predecessor, None).expect("inactive CP402 route");
            assert!(cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard_snapshot_is_exact(snapshot));
            assert!(snapshot
                .cooling_latent_output_greater_than_or_equal_to_maximum_total_cooling_capacity
                .is_none());
            assert!(!snapshot
                .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough);
            assert_supply_is_unchanged(snapshot, predecessor);
            let public = matches!(index, 0..=8);
            assert_eq!(
                cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard_snapshot_is_exact_direct_release(snapshot),
                public,
            );
            public_successors += usize::from(public);
        }
    }
    assert_eq!(public_successors, 13);
    assert_eq!(state.transition_count, 36);
    assert_eq!(state.inactive_transition_count, 24);
    assert_eq!(
        state.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_evaluation_count,
        12,
    );
    let mut predecessor_counts = [1; 30];
    let mut false_counts = [0; 30];
    let mut body_counts = [0; 30];
    for index in ACTIVE {
        predecessor_counts[*index] = 2;
        false_counts[*index] = 1;
        body_counts[*index] = 1;
    }
    assert_eq!(state.predecessor_route_counts, predecessor_counts);
    assert_eq!(state.guard_false_fallthrough_route_counts, false_counts);
    assert_eq!(state.adjustment_body_entry_route_counts, body_counts);
    assert_eq!(
        state.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough_count,
        6,
    );
    assert_eq!(
        state.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_adjustment_body_entry_count,
        6,
    );
    assert_eq!(
        state.cooling_latent_output_greater_than_or_equal_to_maximum_total_cooling_capacity_count,
        6,
    );
    assert_eq!(state.source_site_execution_count, 42);
    assert_eq!(state.cp401_supply_humidity_ratio_state_owner_count, 6);
    assert_eq!(state.cp401_supply_enthalpy_state_owner_count, 23);
    assert_eq!(state.cp401_supply_temperature_state_owner_count, 33);
    for count in common_active_counts(&state) {
        assert_eq!(count, 12);
    }
}

#[test]
fn owner_cardinality_flags_and_left_operand_bits_reject_transactionally() {
    let predecessors = all_predecessors();
    let active = predecessors[20];
    let inactive = predecessors[19];
    let input = active_input(active, 99.0).expect("active input");
    let mut wrong_left = input;
    wrong_left.cooling_latent_output_w =
        f64::from_bits(wrong_left.cooling_latent_output_w.to_bits() ^ 1);
    let mut missing_owner = input;
    missing_owner.cp321_maximum_total_cooling_capacity_owned_read = false;
    let mut missing_corroborator = input;
    missing_corroborator.cp340_same_call_maximum_total_cooling_capacity_bit_corroborated = false;
    for (predecessor, supplied) in [
        (active, None),
        (inactive, Some(input)),
        (active, Some(wrong_left)),
        (active, Some(missing_owner)),
        (active, Some(missing_corroborator)),
    ] {
        let mut state = State::new(predecessor.system);
        let before = state.clone();
        assert!(advance(&mut state, predecessor, supplied).is_none());
        assert_eq!(state, before);
    }
}

fn assert_supply_is_unchanged(
    snapshot: crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputGuardSnapshot,
    predecessor: crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputAssignmentSnapshot,
) {
    assert_bits_eq(snapshot.resulting_supply_humidity_ratio, predecessor.resulting_supply_humidity_ratio);
    assert_bits_eq(
        snapshot.resulting_supply_enthalpy_j_per_kg,
        predecessor.resulting_supply_enthalpy_j_per_kg,
    );
    assert_bits_eq(
        snapshot.resulting_supply_temperature_c,
        predecessor.resulting_supply_temperature_c,
    );
}

fn assert_bits_eq(left: Option<f64>, right: Option<f64>) {
    assert_eq!(left.map(f64::to_bits), right.map(f64::to_bits));
}

fn common_active_counts(state: &State) -> [usize; 6] {
    [
        state.cp401_cooling_latent_output_owned_read_count,
        state.cooling_latent_output_read_count,
        state.cp321_maximum_total_cooling_capacity_owned_read_count,
        state.cp340_same_call_maximum_total_cooling_capacity_bit_corroboration_count,
        state.maximum_total_cooling_capacity_read_count,
        state.cooling_latent_output_maximum_total_cooling_capacity_comparison_count,
    ]
}
