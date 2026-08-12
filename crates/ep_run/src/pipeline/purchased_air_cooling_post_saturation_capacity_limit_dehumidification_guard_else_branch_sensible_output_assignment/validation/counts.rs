fn base_counts<'a>(
    state: &'a State,
    predecessor: &'a PredecessorState,
    transitions: usize,
    inactive: usize,
    assignments: usize,
    sites: usize,
) -> [(&'static str, usize, usize); 18] {
    [
        ("route_partition", state.transition_count, transitions),
        ("predecessor_transition_count", predecessor.transition_count, state.transition_count),
        ("inactive_transition_count", inactive, state.inactive_transition_count),
        ("predecessor_saturation_temperature_assignment_count", predecessor.predecessor_supply_temperature_saturation_assignment_count, state.predecessor_supply_temperature_saturation_assignment_count),
        ("predecessor_saturation_temperature_mixed_air_limit_count", predecessor.predecessor_supply_temperature_saturation_mixed_air_limit_count, state.predecessor_supply_temperature_saturation_mixed_air_limit_count),
        ("predecessor_humidity_assignment_count", predecessor.predecessor_supply_humidity_ratio_assignment_count, state.predecessor_supply_humidity_ratio_assignment_count),
        ("predecessor_enthalpy_assignment_count", predecessor.predecessor_supply_enthalpy_assignment_count, state.predecessor_supply_enthalpy_assignment_count),
        ("predecessor_else_entry_count", predecessor.predecessor_dehumidification_guard_else_branch_entry_count, state.predecessor_dehumidification_guard_else_branch_entry_count),
        ("predecessor_cp_air_assignment_count", predecessor.dehumidification_guard_else_branch_cp_air_assignment_count, state.predecessor_dehumidification_guard_else_branch_cp_air_assignment_count),
        ("sensible_output_assignment_count", assignments, state.dehumidification_guard_else_branch_sensible_output_assignment_count),
        ("source_site_execution_count", sites, state.source_site_execution_count),
        ("humidity_owner_count", predecessor.cp418_supply_humidity_ratio_state_owner_count, state.cp419_supply_humidity_ratio_state_owner_count),
        ("humidity_preservation_count", predecessor.unchanged_supply_humidity_ratio_preservation_count, state.unchanged_supply_humidity_ratio_preservation_count),
        ("enthalpy_owner_count", predecessor.cp418_supply_enthalpy_state_owner_count, state.cp419_supply_enthalpy_state_owner_count),
        ("enthalpy_preservation_count", predecessor.unchanged_supply_enthalpy_preservation_count, state.unchanged_supply_enthalpy_preservation_count),
        ("temperature_owner_count", predecessor.cp418_supply_temperature_state_owner_count, state.cp419_supply_temperature_state_owner_count),
        ("temperature_preservation_count", predecessor.unchanged_supply_temperature_preservation_count, state.unchanged_supply_temperature_preservation_count),
        ("assignment_array_matches_predecessor", 1, usize::from(state.predecessor_dehumidification_guard_else_branch_cp_air_assignment_route_counts == state.dehumidification_guard_else_branch_sensible_output_assignment_route_counts)),
    ]
}

fn active_counters(state: &State) -> [(&'static str, usize); 13] {
    [
        ("supply_mass_flow_rate_owned_read_count", state.supply_mass_flow_rate_owned_read_count),
        ("supply_mass_flow_rate_bit_corroboration_count", state.supply_mass_flow_rate_bit_corroboration_count),
        ("supply_mass_flow_rate_read_count", state.supply_mass_flow_rate_read_count),
        ("cp_air_owned_read_count", state.cp_air_owned_read_count),
        ("cp_air_read_count", state.cp_air_read_count),
        ("supply_mass_flow_rate_times_cp_air_calculation_count", state.supply_mass_flow_rate_times_cp_air_calculation_count),
        ("mixed_air_temperature_owned_read_count", state.mixed_air_temperature_owned_read_count),
        ("mixed_air_temperature_read_count", state.mixed_air_temperature_read_count),
        ("supply_temperature_owned_read_count", state.supply_temperature_owned_read_count),
        ("supply_temperature_read_count", state.supply_temperature_read_count),
        ("mixed_air_minus_supply_temperature_calculation_count", state.mixed_air_minus_supply_temperature_calculation_count),
        ("cooling_sensible_output_calculation_count", state.cooling_sensible_output_calculation_count),
        ("cooling_sensible_output_assignment_write_count", state.cooling_sensible_output_assignment_write_count),
    ]
}
