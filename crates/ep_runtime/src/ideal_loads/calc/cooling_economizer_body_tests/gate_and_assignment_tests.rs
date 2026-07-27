use super::*;

#[test]
fn strict_delta_temperature_gate_preserves_boundary_nan_and_infinity_behavior() {
    let exact_boundary = -PURCHASED_AIR_CALC_COOLING_ECONOMIZER_BODY_SMALL_TEMP_DIFF_C;
    let immediately_below = f64::from_bits(exact_boundary.to_bits() + 1);
    for (delta_temperature_c, expected) in [
        (exact_boundary, false),
        (immediately_below, true),
        (-0.0, false),
        (f64::NAN, false),
        (f64::NEG_INFINITY, true),
        (f64::INFINITY, false),
    ] {
        let (snapshot, state) = characterize(PurchasedAirCalcCoolingEconomizerBodyInput {
            outdoor_air_temperature_c: delta_temperature_c,
            zone_temperature_c: 0.0,
            zone_cooling_setpoint_load_w: -1.0,
            outdoor_air_mass_flow_rate_kg_per_s: -1.0,
            ..base_input()
        });

        assert_bits(snapshot.delta_temperature_c, delta_temperature_c);
        assert_bits(snapshot.assigned_delta_temperature_c, delta_temperature_c);
        assert_bits(snapshot.delta_temperature_for_gate_c, delta_temperature_c);
        assert_eq!(
            snapshot.delta_temperature_below_negative_small_temp_diff,
            Some(expected),
            "{delta_temperature_c:?} < -SmallTempDiff"
        );
        assert_eq!(snapshot.delta_temperature_body_entered, expected);
        assert_eq!(snapshot.zone_cooling_setpoint_load_read, expected);
        assert_eq!(snapshot.cp_air_for_first_division_read, expected);
        assert_eq!(
            snapshot.zone_cooling_setpoint_load_over_cp_air_calculated,
            expected
        );
        assert_eq!(
            snapshot.delta_temperature_for_second_division_read,
            expected
        );
        assert_eq!(snapshot.supply_mass_flow_rate_calculated, expected);
        assert_eq!(snapshot.initial_supply_mass_flow_rate_assigned, expected);
        assert_eq!(
            snapshot.cooling_limit_flow_rate_comparison_evaluated,
            expected
        );
        assert_eq!(snapshot.resulting_supply_mass_flow_rate_read, expected);
        assert_eq!(snapshot.outdoor_air_mass_flow_rate_read, expected);
        assert_eq!(
            snapshot.supply_above_outdoor_air_mass_flow_comparison_evaluated,
            expected
        );
        if !expected {
            assert!(snapshot.zone_cooling_setpoint_load_w.is_none());
            assert!(snapshot.cp_air_for_first_division_j_per_kg_k.is_none());
            assert!(
                snapshot
                    .zone_cooling_setpoint_load_over_cp_air_kg_k_per_s
                    .is_none()
            );
            assert!(snapshot.delta_temperature_for_second_division_c.is_none());
            assert!(snapshot.calculated_supply_mass_flow_rate_kg_per_s.is_none());
            assert!(snapshot.initial_supply_mass_flow_rate_kg_per_s.is_none());
            assert!(snapshot.cooling_limit_flow_rate_value.is_none());
            assert!(snapshot.resulting_supply_mass_flow_rate_kg_per_s.is_none());
            assert!(snapshot.outdoor_air_mass_flow_rate_kg_per_s.is_none());
            assert!(
                snapshot
                    .supply_mass_flow_above_outdoor_air_mass_flow
                    .is_none()
            );
        }
        assert_eq!(
            state.delta_temperature_comparison_satisfied_count,
            usize::from(expected)
        );
        assert_eq!(
            state.delta_temperature_body_entry_count,
            usize::from(expected)
        );
        assert_eq!(
            state.cp_air_for_first_division_read_count,
            usize::from(expected)
        );
        assert_eq!(
            state.zone_cooling_setpoint_load_over_cp_air_calculation_count,
            usize::from(expected)
        );
        assert_eq!(
            state.delta_temperature_for_second_division_read_count,
            usize::from(expected)
        );
        assert_eq!(
            state.initial_supply_mass_flow_rate_assignment_count,
            usize::from(expected)
        );
        assert_eq!(
            state.delta_temperature_fallthrough_count,
            usize::from(!expected)
        );
    }
}

#[test]
fn strict_final_comparison_controls_all_four_economizer_assignments() {
    for (load, outdoor_air_flow, expected) in [
        (-1.0, -1.0, true),
        (-1.0, f64::NAN, false),
        (f64::NAN, 0.0, false),
        (f64::NEG_INFINITY, f64::INFINITY, false),
        (f64::NEG_INFINITY, 0.0, true),
    ] {
        let time_step = f64::from_bits(0x7ff8_0000_0000_00a5);
        let (snapshot, state) = characterize(PurchasedAirCalcCoolingEconomizerBodyInput {
            zone_cooling_setpoint_load_w: load,
            cooling_limit: IdealLoadsLimit::NoLimit,
            outdoor_air_mass_flow_rate_kg_per_s: outdoor_air_flow,
            system_time_step_hours: time_step,
            ..base_input()
        });

        assert!(snapshot.resulting_supply_mass_flow_rate_read);
        assert!(snapshot.outdoor_air_mass_flow_rate_read);
        assert!(snapshot.supply_above_outdoor_air_mass_flow_comparison_evaluated);
        assert_eq!(
            snapshot.supply_mass_flow_above_outdoor_air_mass_flow,
            Some(expected)
        );
        assert_eq!(snapshot.economizer_activation_body_entered, expected);
        assert_eq!(snapshot.economizer_on_assigned, expected);
        assert_eq!(snapshot.economizer_on, expected.then_some(true));
        assert_eq!(
            snapshot.supply_mass_flow_rate_for_outdoor_air_assignment_read,
            expected
        );
        assert_eq!(snapshot.outdoor_air_mass_flow_rate_assigned, expected);
        assert_eq!(snapshot.system_time_step_read, expected);
        assert_eq!(snapshot.economizer_active_time_assigned, expected);
        if expected {
            assert_eq!(
                snapshot
                    .supply_mass_flow_rate_for_outdoor_air_assignment_kg_per_s
                    .expect("outdoor-air assignment source")
                    .to_bits(),
                snapshot
                    .resulting_supply_mass_flow_rate_kg_per_s
                    .expect("comparison supply mass flow")
                    .to_bits()
            );
            assert_eq!(
                snapshot
                    .system_time_step_hours
                    .expect("assigned timestep")
                    .to_bits(),
                time_step.to_bits()
            );
            assert_eq!(
                snapshot
                    .assigned_economizer_active_time_hours
                    .expect("assigned economizer time")
                    .to_bits(),
                time_step.to_bits()
            );
        } else {
            assert!(
                snapshot
                    .supply_mass_flow_rate_for_outdoor_air_assignment_kg_per_s
                    .is_none()
            );
            assert!(
                snapshot
                    .assigned_outdoor_air_mass_flow_rate_kg_per_s
                    .is_none()
            );
            assert!(snapshot.system_time_step_hours.is_none());
            assert!(snapshot.assigned_economizer_active_time_hours.is_none());
        }
        assert_eq!(
            state.supply_above_outdoor_air_mass_flow_comparison_satisfied_count,
            usize::from(expected)
        );
        assert_eq!(
            state.economizer_activation_body_entry_count,
            usize::from(expected)
        );
        assert_eq!(
            state.outdoor_air_mass_flow_comparison_fallthrough_count,
            usize::from(!expected)
        );
        assert_eq!(state.economizer_on_assignment_count, usize::from(expected));
        assert_eq!(
            state.supply_mass_flow_rate_for_outdoor_air_assignment_read_count,
            usize::from(expected)
        );
        assert_eq!(
            state.outdoor_air_mass_flow_rate_assignment_count,
            usize::from(expected)
        );
        assert_eq!(state.system_time_step_read_count, usize::from(expected));
        assert_eq!(
            state.economizer_active_time_assignment_count,
            usize::from(expected)
        );
    }
}
