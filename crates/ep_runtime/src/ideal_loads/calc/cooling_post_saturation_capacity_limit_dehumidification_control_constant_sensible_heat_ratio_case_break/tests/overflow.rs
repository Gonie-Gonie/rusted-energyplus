//! CP393 checked-counter overflow tests.

use super::*;
use ep_model::DehumidificationControlType as D;

type State = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseBreakRuntimeState;

#[test]
fn every_active_counter_overflow_rejects_before_mutation() {
    let chain = fixtures::chain(
        3,
        1,
        true,
        Some(D::ConstantSensibleHeatRatio),
        1,
        0.7,
        18.0,
        1.0,
    );
    let setters: &[fn(&mut State)] = &[
        |state| state.transition_count = usize::MAX,
        |state| state.predecessor_route_counts[18] = usize::MAX,
        |state| {
            state.dehumidification_control_constant_sensible_heat_ratio_case_break_count =
                usize::MAX
        },
        |state| state.source_site_execution_count = usize::MAX,
    ];
    for set_overflow in setters {
        let mut state = State::new(chain.cp392.system);
        set_overflow(&mut state);
        let before = state.clone();
        assert!(advance(&mut state, chain.cp392).is_none());
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
        let mut state = State::new(chain.cp392.system);
        set_overflow(&mut state);
        let before = state.clone();
        assert!(advance(&mut state, chain.cp392).is_none());
        assert_eq!(state, before);
    }
}

fn advance(
    state: &mut State,
    predecessor: crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyHumidityRatioAssignmentSnapshot,
) -> Option<PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseBreakSnapshot>
{
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break_state(
        state,
        predecessor,
    )
}
