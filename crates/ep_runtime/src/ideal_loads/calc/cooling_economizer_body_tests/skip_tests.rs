use super::*;

#[test]
fn unit_off_non_cooling_outer_false_and_condition_false_are_complete_skips() {
    let mut state = PurchasedAirCalcCoolingEconomizerBodyRuntimeState::new(SYSTEM);
    let routes = [
        PredecessorRoute::UnitOff,
        PredecessorRoute::NonCooling,
        PredecessorRoute::MaximumFlowSibling,
        PredecessorRoute::NoEconomizer,
        PredecessorRoute::ConditionFallthrough,
    ];

    for route in routes {
        let snapshot = advance_cooling_economizer_body_state(
            &mut state,
            body_predecessor(route),
            poison_input(),
        );
        assert_body_sites_skipped(snapshot);
        assert_eq!(
            snapshot.unit_off_skipped,
            matches!(route, PredecessorRoute::UnitOff)
        );
        assert_eq!(
            snapshot.non_cooling_skipped,
            matches!(route, PredecessorRoute::NonCooling)
        );
        assert_eq!(
            snapshot.maximum_cooling_flow_body_sibling_skipped,
            matches!(route, PredecessorRoute::MaximumFlowSibling)
        );
        assert_eq!(
            snapshot.no_economizer_outer_guard_fallthrough_skipped,
            matches!(route, PredecessorRoute::NoEconomizer)
        );
        assert_eq!(
            snapshot.economizer_condition_fallthrough_skipped,
            matches!(route, PredecessorRoute::ConditionFallthrough)
        );
    }

    assert_eq!(state.transition_count, 5);
    assert_eq!(state.body_execution_count, 0);
    assert_eq!(state.unit_off_skip_count, 1);
    assert_eq!(state.non_cooling_skip_count, 1);
    assert_eq!(state.maximum_cooling_flow_body_sibling_skip_count, 1);
    assert_eq!(state.no_economizer_outer_guard_fallthrough_skip_count, 1);
    assert_eq!(state.economizer_condition_fallthrough_skip_count, 1);
    assert_eq!(state.zone_humidity_ratio_read_count, 0);
    assert_eq!(state.psychrometric_cp_air_evaluation_count, 0);
    assert_eq!(state.cp_air_assignment_count, 0);
    assert_eq!(state.delta_temperature_assignment_count, 0);
    assert_eq!(state.delta_temperature_for_gate_read_count, 0);
    assert_eq!(state.delta_temperature_comparison_count, 0);
    assert_eq!(state.delta_temperature_body_entry_count, 0);
    assert_eq!(state.cp_air_for_first_division_read_count, 0);
    assert_eq!(
        state.zone_cooling_setpoint_load_over_cp_air_calculation_count,
        0
    );
    assert_eq!(state.delta_temperature_for_second_division_read_count, 0);
    assert_eq!(state.supply_mass_flow_rate_calculation_count, 0);
    assert_eq!(state.initial_supply_mass_flow_rate_assignment_count, 0);
    assert_eq!(state.cooling_limit_flow_rate_read_count, 0);
    assert_eq!(state.cooling_limit_flow_rate_and_capacity_read_count, 0);
    assert_eq!(state.maximum_flow_clamp_body_entry_count, 0);
    assert_eq!(state.supply_mass_flow_rate_for_clamp_read_count, 0);
    assert_eq!(state.inner_max_evaluation_count, 0);
    assert_eq!(
        state.maximum_cooling_air_mass_flow_rate_clamp_upper_bound_read_count,
        0
    );
    assert_eq!(state.outer_min_evaluation_count, 0);
    assert_eq!(state.clamped_supply_mass_flow_rate_assignment_count, 0);
    assert_eq!(state.supply_above_outdoor_air_mass_flow_comparison_count, 0);
    assert_eq!(state.economizer_activation_body_entry_count, 0);
    assert_eq!(
        state.supply_mass_flow_rate_for_outdoor_air_assignment_read_count,
        0
    );
}

fn assert_body_sites_skipped(snapshot: PurchasedAirCalcCoolingEconomizerBodySnapshot) {
    assert_eq!(
        snapshot.source,
        PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_SOURCE
    );
    assert_eq!(
        snapshot.first_excluded_source,
        PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_FIRST_EXCLUDED_SOURCE
    );
    assert_eq!(
        snapshot.source_order,
        PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_SOURCE_ORDER
    );
    assert!(!snapshot.economizer_calculation_body_executed);
    assert!(!snapshot.zone_humidity_ratio_read);
    assert!(snapshot.zone_humidity_ratio.is_none());
    assert!(!snapshot.psychrometric_cp_air_evaluated);
    assert!(snapshot.psychrometric_cp_air_result_j_per_kg_k.is_none());
    assert!(!snapshot.cp_air_assigned);
    assert!(snapshot.cp_air_j_per_kg_k.is_none());
    assert!(!snapshot.outdoor_air_temperature_read);
    assert!(snapshot.outdoor_air_temperature_c.is_none());
    assert!(!snapshot.zone_temperature_read);
    assert!(snapshot.zone_temperature_c.is_none());
    assert!(!snapshot.delta_temperature_calculated);
    assert!(snapshot.delta_temperature_c.is_none());
    assert!(!snapshot.delta_temperature_assigned);
    assert!(snapshot.assigned_delta_temperature_c.is_none());
    assert!(!snapshot.delta_temperature_for_gate_read);
    assert!(snapshot.delta_temperature_for_gate_c.is_none());
    assert!(!snapshot.delta_temperature_comparison_evaluated);
    assert!(
        snapshot
            .delta_temperature_below_negative_small_temp_diff
            .is_none()
    );
    assert!(!snapshot.delta_temperature_body_entered);
    assert!(!snapshot.zone_cooling_setpoint_load_read);
    assert!(snapshot.zone_cooling_setpoint_load_w.is_none());
    assert!(!snapshot.cp_air_for_first_division_read);
    assert!(snapshot.cp_air_for_first_division_j_per_kg_k.is_none());
    assert!(!snapshot.zone_cooling_setpoint_load_over_cp_air_calculated);
    assert!(
        snapshot
            .zone_cooling_setpoint_load_over_cp_air_kg_k_per_s
            .is_none()
    );
    assert!(!snapshot.delta_temperature_for_second_division_read);
    assert!(snapshot.delta_temperature_for_second_division_c.is_none());
    assert!(!snapshot.supply_mass_flow_rate_calculated);
    assert!(snapshot.calculated_supply_mass_flow_rate_kg_per_s.is_none());
    assert!(!snapshot.initial_supply_mass_flow_rate_assigned);
    assert!(snapshot.initial_supply_mass_flow_rate_kg_per_s.is_none());
    assert!(!snapshot.cooling_limit_flow_rate_comparison_evaluated);
    assert!(!snapshot.cooling_limit_flow_rate_read);
    assert!(snapshot.cooling_limit_flow_rate_value.is_none());
    assert!(
        snapshot
            .cooling_limit_flow_rate_comparison_satisfied
            .is_none()
    );
    assert!(!snapshot.cooling_limit_flow_rate_and_capacity_comparison_evaluated);
    assert!(!snapshot.cooling_limit_flow_rate_and_capacity_read);
    assert!(
        snapshot
            .cooling_limit_flow_rate_and_capacity_value
            .is_none()
    );
    assert!(
        snapshot
            .cooling_limit_flow_rate_and_capacity_comparison_satisfied
            .is_none()
    );
    assert!(snapshot.cooling_flow_limit_active.is_none());
    assert!(!snapshot.maximum_cooling_air_mass_flow_rate_read);
    assert!(
        snapshot
            .maximum_cooling_air_mass_flow_rate_kg_per_s
            .is_none()
    );
    assert!(!snapshot.maximum_cooling_air_mass_flow_rate_positive_comparison_evaluated);
    assert!(
        snapshot
            .maximum_cooling_air_mass_flow_rate_positive
            .is_none()
    );
    assert!(!snapshot.maximum_flow_clamp_body_entered);
    assert!(!snapshot.supply_mass_flow_rate_for_clamp_read);
    assert!(snapshot.supply_mass_flow_rate_for_clamp_kg_per_s.is_none());
    assert!(!snapshot.inner_max_evaluated);
    assert!(!snapshot.maximum_cooling_air_mass_flow_rate_clamp_upper_bound_read);
    assert!(
        snapshot
            .maximum_cooling_air_mass_flow_rate_clamp_upper_bound_kg_per_s
            .is_none()
    );
    assert!(!snapshot.supply_mass_flow_rate_clamped);
    assert!(
        snapshot
            .nonnegative_supply_mass_flow_rate_kg_per_s
            .is_none()
    );
    assert!(!snapshot.outer_min_evaluated);
    assert!(snapshot.clamped_supply_mass_flow_rate_kg_per_s.is_none());
    assert!(!snapshot.clamped_supply_mass_flow_rate_assigned);
    assert!(snapshot.resulting_supply_mass_flow_rate_kg_per_s.is_none());
    assert!(!snapshot.resulting_supply_mass_flow_rate_read);
    assert!(!snapshot.outdoor_air_mass_flow_rate_read);
    assert!(snapshot.outdoor_air_mass_flow_rate_kg_per_s.is_none());
    assert!(!snapshot.supply_above_outdoor_air_mass_flow_comparison_evaluated);
    assert!(
        snapshot
            .supply_mass_flow_above_outdoor_air_mass_flow
            .is_none()
    );
    assert!(!snapshot.economizer_activation_body_entered);
    assert!(!snapshot.economizer_on_assigned);
    assert!(snapshot.economizer_on.is_none());
    assert!(!snapshot.supply_mass_flow_rate_for_outdoor_air_assignment_read);
    assert!(
        snapshot
            .supply_mass_flow_rate_for_outdoor_air_assignment_kg_per_s
            .is_none()
    );
    assert!(!snapshot.outdoor_air_mass_flow_rate_assigned);
    assert!(
        snapshot
            .assigned_outdoor_air_mass_flow_rate_kg_per_s
            .is_none()
    );
    assert!(!snapshot.system_time_step_read);
    assert!(snapshot.system_time_step_hours.is_none());
    assert!(!snapshot.economizer_active_time_assigned);
    assert!(snapshot.assigned_economizer_active_time_hours.is_none());
}
