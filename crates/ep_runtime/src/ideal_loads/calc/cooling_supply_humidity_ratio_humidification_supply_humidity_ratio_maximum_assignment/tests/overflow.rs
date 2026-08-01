//! CP375 bounded-counter overflow tests.

use super::*;

#[test]
fn cp375_every_active_counter_overflow_is_transactional() {
    let predecessor = active_cp374(DehumidificationControlType::None, 0.008, 0.007);
    let mutators: [fn(&mut State); 7] = [
        |state| state.transition_count = usize::MAX,
        |state| {
            state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count =
                usize::MAX
        },
        |state| state.source_site_execution_count = usize::MAX - 3,
        |state| {
            state.purchased_air_supply_humidity_ratio_for_humidification_supply_maximum_read_count =
                usize::MAX
        },
        |state| {
            state.supply_humidity_ratio_for_humidification_for_supply_maximum_read_count =
                usize::MAX
        },
        |state| state.source_shaped_two_argument_maximum_evaluation_count = usize::MAX,
        |state| state.purchased_air_supply_humidity_ratio_assignment_count = usize::MAX,
    ];
    for mutate in mutators {
        let mut state = State::new(predecessor.system);
        mutate(&mut state);
        let before = state.clone();
        assert!(
            advance(
                &mut state,
                predecessor,
                Some(ActiveOperands {
                    purchased_air_supply_humidity_ratio: 0.006,
                }),
            )
            .is_none()
        );
        assert_eq!(state, before);
    }
}
