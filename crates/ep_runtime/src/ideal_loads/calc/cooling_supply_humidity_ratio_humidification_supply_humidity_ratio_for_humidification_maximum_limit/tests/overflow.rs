//! CP374 bounded-counter overflow tests.

use super::*;

#[test]
fn cp374_every_active_counter_overflow_is_transactional() {
    let predecessor = active_cp373(DehumidificationControlType::None, 0.001, 0.5, 0.004);
    let mutators: [fn(&mut State); 7] = [
        |state| state.transition_count = usize::MAX,
        |state| {
            state.dehumidification_control_none_supply_humidity_ratio_for_humidification_maximum_limit_count = usize::MAX
        },
        |state| state.source_site_execution_count = usize::MAX - 3,
        |state| {
            state.supply_humidity_ratio_for_humidification_for_maximum_limit_minimum_read_count =
                usize::MAX
        },
        |state| state.maximum_heating_supply_air_humidity_ratio_for_minimum_read_count = usize::MAX,
        |state| state.source_shaped_two_argument_minimum_evaluation_count = usize::MAX,
        |state| state.supply_humidity_ratio_for_humidification_assignment_count = usize::MAX,
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
                    maximum_heating_supply_air_humidity_ratio: 0.005,
                }),
            )
            .is_none()
        );
        assert_eq!(state, before);
    }
}
