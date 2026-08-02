//! Transactional checked-overflow coverage for CP402 accounting.

use super::fixtures::{active_input, advance, all_predecessors};
use super::super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputGuardActiveInput as ActiveInput,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputGuardRuntimeState as State,
};
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputAssignmentSnapshot as Predecessor;

#[test]
fn every_counter_family_preflights_before_any_mutation() {
    let predecessors = all_predecessors();
    let active = predecessors[20];
    let latent = active.cooling_latent_output_w.expect("active latent");
    let false_input = active_input(active, f64::INFINITY);
    let true_input = active_input(active, latent);

    let mut state = State::new(active.system);
    state.transition_count = usize::MAX;
    reject_unchanged(state, active, false_input);

    let mut state = State::new(active.system);
    state.predecessor_route_counts[20] = usize::MAX;
    reject_unchanged(state, active, false_input);

    let mut state = State::new(active.system);
    state.guard_false_fallthrough_route_counts[20] = usize::MAX;
    reject_unchanged(state, active, false_input);

    let mut state = State::new(active.system);
    state.adjustment_body_entry_route_counts[20] = usize::MAX;
    reject_unchanged(state, active, true_input);

    let mut state = State::new(active.system);
    state.source_site_execution_count = usize::MAX;
    reject_unchanged(state, active, false_input);

    let mut state = State::new(active.system);
    state.cp401_cooling_latent_output_owned_read_count = usize::MAX;
    reject_unchanged(state, active, false_input);

    let mut state = State::new(active.system);
    state.cp401_supply_temperature_state_owner_count = usize::MAX;
    reject_unchanged(state, active, false_input);

    let mut state = State::new(active.system);
    state.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough_count = usize::MAX;
    reject_unchanged(state, active, false_input);

    let mut state = State::new(active.system);
    state.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_adjustment_body_entry_count = usize::MAX;
    reject_unchanged(state, active, true_input);
}

#[test]
fn inactive_counter_overflow_is_transactional() {
    let predecessor = all_predecessors()[0];
    let mut state = State::new(predecessor.system);
    state.inactive_transition_count = usize::MAX;
    reject_unchanged(state, predecessor, None);
}

fn reject_unchanged(
    mut state: State,
    predecessor: Predecessor,
    input: Option<ActiveInput>,
) {
    let before = state.clone();
    assert!(advance(&mut state, predecessor, input).is_none());
    assert_eq!(state, before);
}
