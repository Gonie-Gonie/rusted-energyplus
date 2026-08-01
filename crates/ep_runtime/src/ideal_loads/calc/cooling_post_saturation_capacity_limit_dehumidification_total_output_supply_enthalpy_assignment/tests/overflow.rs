//! CP385 checked-counter overflow rejection.

use super::{predecessor_for_route, retained_input};
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentRuntimeState as State,
    advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_state as advance,
};

#[test]
fn cp385_rejects_each_active_source_counter_overflow_without_mutation() {
    let predecessor = predecessor_for_route(3, 1, true, 1);
    let input = retained_input(predecessor, 1.0, 2.0, 3.0);
    for index in 0..12 {
        let mut state = State::new(predecessor.system);
        match index {
            0 => state.source_site_execution_count = usize::MAX - 5,
            1 => state.post_saturation_capacity_limited_dehumidification_supply_enthalpy_assignment_count = usize::MAX,
            2 => state.cp379_retained_supply_enthalpy_owned_read_count = usize::MAX,
            3 => state.cp329_retained_mixed_air_enthalpy_owned_read_count = usize::MAX,
            4 => state.mixed_air_enthalpy_read_count = usize::MAX,
            5 => state.cp384_retained_cooling_total_output_owned_read_count = usize::MAX,
            6 => state.cooling_total_output_read_count = usize::MAX,
            7 => state.cp330_retained_supply_mass_flow_rate_owned_read_count = usize::MAX,
            8 => state.supply_mass_flow_rate_read_count = usize::MAX,
            9 => state.specific_cooling_output_calculation_count = usize::MAX,
            10 => state.supply_enthalpy_difference_calculation_count = usize::MAX,
            11 => state.supply_enthalpy_assignment_write_count = usize::MAX,
            _ => unreachable!(),
        }
        let before = state.clone();
        assert!(advance(&mut state, predecessor, input).is_none());
        assert_eq!(state, before);
    }
}
