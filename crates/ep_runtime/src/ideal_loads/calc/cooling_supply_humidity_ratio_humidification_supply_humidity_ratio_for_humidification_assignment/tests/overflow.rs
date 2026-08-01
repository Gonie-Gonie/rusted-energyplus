//! CP373 bounded-counter overflow tests.

use super::*;

#[test]
fn cp373_every_active_counter_overflow_is_transactional() {
    let predecessor = active_cp372(DehumidificationControlType::None, 0.001);
    let mutators: [fn(&mut State); 9] = [
        |state| state.transition_count = usize::MAX,
        |state| {
            state.dehumidification_control_none_supply_humidity_ratio_for_humidification_assignment_count = usize::MAX
        },
        |state| state.source_site_execution_count = usize::MAX - 5,
        |state| state.zone_humidifying_setpoint_moisture_demand_read_count = usize::MAX,
        |state| state.supply_mass_flow_rate_read_count = usize::MAX,
        |state| {
            state.moisture_demand_derived_supply_humidity_ratio_calculation_count = usize::MAX
        },
        |state| state.zone_node_humidity_ratio_read_count = usize::MAX,
        |state| state.supply_humidity_ratio_for_humidification_calculation_count = usize::MAX,
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
                    supply_mass_flow_rate_kg_per_s: 1.0,
                    zone_node_humidity_ratio: 0.004,
                }),
            )
            .is_none()
        );
        assert_eq!(state, before);
    }
}
