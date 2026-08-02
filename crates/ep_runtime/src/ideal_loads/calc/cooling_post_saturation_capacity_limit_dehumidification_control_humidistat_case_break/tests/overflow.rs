//! CP396 checked-counter overflow tests.

use super::*;
use ep_model::DehumidificationControlType as D;

type State = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatCaseBreakRuntimeState;

#[test]
fn every_active_counter_overflow_rejects_before_mutation() {
    let chain = fixtures::chain(3, 1, true, Some(D::Humidistat), 1, 0.7, 18.0, 1.0);
    let setters: &[fn(&mut State)] = &[
        |state| state.transition_count = usize::MAX,
        |state| state.predecessor_route_counts[19] = usize::MAX,
        |state| state.dehumidification_control_humidistat_case_break_count = usize::MAX,
        |state| state.source_site_execution_count = usize::MAX,
    ];
    assert_each_overflow_rejected(chain.cp395, setters);
}

#[test]
fn inactive_counter_overflow_rejects_before_mutation() {
    let chain = fixtures::chain(3, 1, true, Some(D::None), 1, 0.7, 18.0, 1.0);
    let setters: &[fn(&mut State)] = &[
        |state| state.transition_count = usize::MAX,
        |state| state.predecessor_route_counts[20] = usize::MAX,
        |state| state.inactive_transition_count = usize::MAX,
    ];
    assert_each_overflow_rejected(chain.cp395, setters);
}

fn assert_each_overflow_rejected(
    predecessor: crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatSupplyHumidityRatioAssignmentSnapshot,
    setters: &[fn(&mut State)],
) {
    for set_overflow in setters {
        let mut state = State::new(predecessor.system);
        set_overflow(&mut state);
        let before = state.clone();
        assert!(
            advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_break_state(
                &mut state,
                predecessor,
            )
            .is_none()
        );
        assert_eq!(state, before);
    }
}
