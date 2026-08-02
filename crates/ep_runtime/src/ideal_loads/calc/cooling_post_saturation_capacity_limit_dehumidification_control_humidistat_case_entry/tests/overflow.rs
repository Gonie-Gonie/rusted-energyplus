//! CP394 checked-counter overflow tests.

use super::*;
use ep_model::DehumidificationControlType as D;

type State = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatCaseEntryRuntimeState;

#[test]
fn every_active_counter_overflow_rejects_before_mutation() {
    let chain = fixtures::chain(3, 1, true, Some(D::Humidistat), 1, 0.7, 18.0, 1.0);
    let setters: &[fn(&mut State)] = &[
        |state| state.transition_count = usize::MAX,
        |state| state.predecessor_route_counts[19] = usize::MAX,
        |state| state.dehumidification_control_humidistat_case_entry_count = usize::MAX,
        |state| state.source_site_execution_count = usize::MAX,
    ];
    for set_overflow in setters {
        let mut state = State::new(chain.cp393.system);
        set_overflow(&mut state);
        let before = state.clone();
        assert!(advance(&mut state, chain.cp393).is_none());
        assert_eq!(state, before);
    }
}

#[test]
fn inactive_counter_overflow_rejects_before_mutation() {
    let chain = fixtures::chain(3, 1, true, Some(D::None), 1, 0.7, 18.0, 1.0);
    let setters: &[fn(&mut State)] = &[
        |state| state.transition_count = usize::MAX,
        |state| state.predecessor_route_counts[20] = usize::MAX,
        |state| state.inactive_transition_count = usize::MAX,
    ];
    for set_overflow in setters {
        let mut state = State::new(chain.cp393.system);
        set_overflow(&mut state);
        let before = state.clone();
        assert!(advance(&mut state, chain.cp393).is_none());
        assert_eq!(state, before);
    }
}

fn advance(
    state: &mut State,
    predecessor: crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseBreakSnapshot,
) -> Option<PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatCaseEntrySnapshot>
{
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry_state(
        state,
        predecessor,
    )
}
