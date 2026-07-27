use super::*;

#[test]
fn source_order_preserves_delta_limit_clamp_and_assignment_short_circuits() {
    let input = PurchasedAirCalcCoolingEconomizerBodyInput {
        zone_humidity_ratio: 0.008,
        outdoor_air_temperature_c: 17.0,
        zone_temperature_c: 20.0,
        zone_cooling_setpoint_load_w: -1.0,
        cooling_limit: IdealLoadsLimit::NoLimit,
        maximum_cooling_air_mass_flow_rate_kg_per_s: f64::NAN,
        outdoor_air_mass_flow_rate_kg_per_s: -1.0,
        system_time_step_hours: 0.25,
    };
    let (snapshot, state) = characterize(input);

    let cp_air = energyplus_psy_cp_air_fn_w(input.zone_humidity_ratio);
    let expected_first_division = input.zone_cooling_setpoint_load_w / cp_air;
    let expected_left_associated =
        expected_first_division / (input.outdoor_air_temperature_c - input.zone_temperature_c);
    let reassociated = input.zone_cooling_setpoint_load_w
        / (cp_air * (input.outdoor_air_temperature_c - input.zone_temperature_c));
    assert_ne!(
        expected_left_associated.to_bits(),
        reassociated.to_bits(),
        "the characterization operands must distinguish source association"
    );

    assert!(snapshot.economizer_calculation_body_executed);
    assert!(snapshot.zone_humidity_ratio_read);
    assert_bits(snapshot.zone_humidity_ratio, input.zone_humidity_ratio);
    assert!(snapshot.psychrometric_cp_air_evaluated);
    assert_bits(snapshot.psychrometric_cp_air_result_j_per_kg_k, cp_air);
    assert!(snapshot.cp_air_assigned);
    assert_bits(snapshot.cp_air_j_per_kg_k, cp_air);
    assert!(snapshot.outdoor_air_temperature_read);
    assert_bits(
        snapshot.outdoor_air_temperature_c,
        input.outdoor_air_temperature_c,
    );
    assert!(snapshot.zone_temperature_read);
    assert_bits(snapshot.zone_temperature_c, input.zone_temperature_c);
    assert!(snapshot.delta_temperature_calculated);
    assert_bits(snapshot.delta_temperature_c, -3.0);
    assert!(snapshot.delta_temperature_assigned);
    assert_bits(snapshot.assigned_delta_temperature_c, -3.0);
    assert!(snapshot.delta_temperature_for_gate_read);
    assert_bits(snapshot.delta_temperature_for_gate_c, -3.0);
    assert!(snapshot.delta_temperature_comparison_evaluated);
    assert_eq!(
        snapshot.delta_temperature_below_negative_small_temp_diff,
        Some(true)
    );
    assert!(snapshot.delta_temperature_body_entered);
    assert!(snapshot.zone_cooling_setpoint_load_read);
    assert_bits(
        snapshot.zone_cooling_setpoint_load_w,
        input.zone_cooling_setpoint_load_w,
    );
    assert!(snapshot.cp_air_for_first_division_read);
    assert_bits(snapshot.cp_air_for_first_division_j_per_kg_k, cp_air);
    assert!(snapshot.zone_cooling_setpoint_load_over_cp_air_calculated);
    assert_bits(
        snapshot.zone_cooling_setpoint_load_over_cp_air_kg_k_per_s,
        expected_first_division,
    );
    assert!(snapshot.delta_temperature_for_second_division_read);
    assert_bits(snapshot.delta_temperature_for_second_division_c, -3.0);
    assert!(snapshot.supply_mass_flow_rate_calculated);
    assert_bits(
        snapshot.calculated_supply_mass_flow_rate_kg_per_s,
        expected_left_associated,
    );
    assert!(snapshot.initial_supply_mass_flow_rate_assigned);
    assert_bits(
        snapshot.initial_supply_mass_flow_rate_kg_per_s,
        expected_left_associated,
    );
    assert_bits(
        snapshot.resulting_supply_mass_flow_rate_kg_per_s,
        expected_left_associated,
    );
    assert_eq!(
        snapshot.supply_mass_flow_above_outdoor_air_mass_flow,
        Some(true)
    );
    assert!(snapshot.economizer_on_assigned);
    assert_eq!(snapshot.economizer_on, Some(true));
    assert!(snapshot.outdoor_air_mass_flow_rate_assigned);
    assert_bits(
        snapshot.assigned_outdoor_air_mass_flow_rate_kg_per_s,
        expected_left_associated,
    );
    assert!(snapshot.system_time_step_read);
    assert_bits(
        snapshot.system_time_step_hours,
        input.system_time_step_hours,
    );
    assert!(snapshot.economizer_active_time_assigned);
    assert_bits(
        snapshot.assigned_economizer_active_time_hours,
        input.system_time_step_hours,
    );

    assert_eq!(state.transition_count, 1);
    assert_eq!(state.body_execution_count, 1);
    assert_eq!(state.zone_humidity_ratio_read_count, 1);
    assert_eq!(state.psychrometric_cp_air_evaluation_count, 1);
    assert_eq!(state.cp_air_assignment_count, 1);
    assert_eq!(state.delta_temperature_assignment_count, 1);
    assert_eq!(state.delta_temperature_for_gate_read_count, 1);
    assert_eq!(state.delta_temperature_comparison_satisfied_count, 1);
    assert_eq!(state.delta_temperature_body_entry_count, 1);
    assert_eq!(state.zone_cooling_setpoint_load_read_count, 1);
    assert_eq!(state.cp_air_for_first_division_read_count, 1);
    assert_eq!(
        state.zone_cooling_setpoint_load_over_cp_air_calculation_count,
        1
    );
    assert_eq!(state.delta_temperature_for_second_division_read_count, 1);
    assert_eq!(state.supply_mass_flow_rate_calculation_count, 1);
    assert_eq!(state.initial_supply_mass_flow_rate_assignment_count, 1);
    assert_eq!(state.cooling_limit_flow_rate_read_count, 1);
    assert_eq!(state.cooling_limit_flow_rate_comparison_count, 1);
    assert_eq!(
        state.cooling_limit_flow_rate_and_capacity_comparison_count,
        1
    );
    assert_eq!(state.maximum_cooling_air_mass_flow_rate_read_count, 0);
    assert_eq!(
        state.maximum_cooling_air_mass_flow_rate_clamp_upper_bound_read_count,
        0
    );
    assert_eq!(state.supply_mass_flow_rate_clamp_count, 0);
    assert_eq!(state.resulting_supply_mass_flow_rate_read_count, 1);
    assert_eq!(state.outdoor_air_mass_flow_rate_read_count, 1);
    assert_eq!(
        state.supply_above_outdoor_air_mass_flow_comparison_satisfied_count,
        1
    );
    assert_eq!(state.economizer_on_assignment_count, 1);
    assert_eq!(state.economizer_activation_body_entry_count, 1);
    assert_eq!(
        state.supply_mass_flow_rate_for_outdoor_air_assignment_read_count,
        1
    );
    assert_eq!(state.outdoor_air_mass_flow_rate_assignment_count, 1);
    assert_eq!(state.system_time_step_read_count, 1);
    assert_eq!(state.economizer_active_time_assignment_count, 1);
}

#[test]
fn repeated_cooling_limit_reads_and_maximum_flow_and_follow_cpp_short_circuiting() {
    for (limit, first_match, second_reached, second_match, maximum_reached) in [
        (IdealLoadsLimit::LimitFlowRate, true, false, None, true),
        (
            IdealLoadsLimit::LimitFlowRateAndCapacity,
            false,
            true,
            Some(true),
            true,
        ),
        (IdealLoadsLimit::NoLimit, false, true, Some(false), false),
        (
            IdealLoadsLimit::LimitCapacity,
            false,
            true,
            Some(false),
            false,
        ),
    ] {
        let (snapshot, state) = characterize(PurchasedAirCalcCoolingEconomizerBodyInput {
            cooling_limit: limit,
            maximum_cooling_air_mass_flow_rate_kg_per_s: 2.0,
            ..base_input()
        });

        assert!(snapshot.cooling_limit_flow_rate_read);
        assert_eq!(snapshot.cooling_limit_flow_rate_value, Some(limit));
        assert_eq!(
            snapshot.cooling_limit_flow_rate_comparison_satisfied,
            Some(first_match)
        );
        assert_eq!(
            snapshot.cooling_limit_flow_rate_and_capacity_comparison_evaluated,
            second_reached
        );
        assert_eq!(
            snapshot.cooling_limit_flow_rate_and_capacity_read,
            second_reached
        );
        assert_eq!(
            snapshot.cooling_limit_flow_rate_and_capacity_value,
            second_reached.then_some(limit)
        );
        assert_eq!(
            snapshot.cooling_limit_flow_rate_and_capacity_comparison_satisfied,
            second_match
        );
        assert_eq!(snapshot.cooling_flow_limit_active, Some(maximum_reached));
        assert_eq!(
            snapshot.maximum_cooling_air_mass_flow_rate_read,
            maximum_reached
        );
        assert_eq!(
            snapshot.maximum_cooling_air_mass_flow_rate_positive_comparison_evaluated,
            maximum_reached
        );
        assert_eq!(
            snapshot.maximum_cooling_air_mass_flow_rate_kg_per_s,
            maximum_reached.then_some(2.0)
        );
        assert_eq!(
            snapshot.maximum_cooling_air_mass_flow_rate_positive,
            maximum_reached.then_some(true)
        );
        assert_eq!(snapshot.maximum_flow_clamp_body_entered, maximum_reached);
        assert_eq!(snapshot.supply_mass_flow_rate_clamped, maximum_reached);
        assert_eq!(
            snapshot.supply_mass_flow_rate_for_clamp_read,
            maximum_reached
        );
        if maximum_reached {
            assert_eq!(
                snapshot
                    .supply_mass_flow_rate_for_clamp_kg_per_s
                    .expect("clamp supply read")
                    .to_bits(),
                snapshot
                    .initial_supply_mass_flow_rate_kg_per_s
                    .expect("initial supply assignment")
                    .to_bits()
            );
        } else {
            assert!(snapshot.supply_mass_flow_rate_for_clamp_kg_per_s.is_none());
        }
        assert_eq!(snapshot.inner_max_evaluated, maximum_reached);
        assert_eq!(
            snapshot.maximum_cooling_air_mass_flow_rate_clamp_upper_bound_read,
            maximum_reached
        );
        assert_eq!(
            snapshot.maximum_cooling_air_mass_flow_rate_clamp_upper_bound_kg_per_s,
            maximum_reached.then_some(2.0)
        );
        assert_eq!(snapshot.outer_min_evaluated, maximum_reached);
        assert_eq!(
            snapshot.clamped_supply_mass_flow_rate_assigned,
            maximum_reached
        );
        assert_eq!(state.cooling_limit_flow_rate_read_count, 1);
        assert_eq!(
            state.cooling_limit_flow_rate_and_capacity_comparison_count,
            usize::from(second_reached)
        );
        assert_eq!(
            state.cooling_limit_flow_rate_and_capacity_read_count,
            usize::from(second_reached)
        );
        assert_eq!(
            state.maximum_cooling_air_mass_flow_rate_read_count,
            usize::from(maximum_reached)
        );
        assert_eq!(
            state.maximum_cooling_air_mass_flow_rate_positive_comparison_count,
            usize::from(maximum_reached)
        );
        assert_eq!(
            state.maximum_flow_clamp_body_entry_count,
            usize::from(maximum_reached)
        );
        assert_eq!(
            state.supply_mass_flow_rate_for_clamp_read_count,
            usize::from(maximum_reached)
        );
        assert_eq!(
            state.inner_max_evaluation_count,
            usize::from(maximum_reached)
        );
        assert_eq!(
            state.maximum_cooling_air_mass_flow_rate_clamp_upper_bound_read_count,
            usize::from(maximum_reached)
        );
        assert_eq!(
            state.outer_min_evaluation_count,
            usize::from(maximum_reached)
        );
        assert_eq!(
            state.supply_mass_flow_rate_clamp_count,
            usize::from(maximum_reached)
        );
        assert_eq!(
            state.clamped_supply_mass_flow_rate_assignment_count,
            usize::from(maximum_reached)
        );
    }
}
