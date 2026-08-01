use super::*;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentRuntimeState,
    cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_snapshot_is_exact_direct_release,
    purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_lifecycle_summary,
};

fn run_case(
    cooling_limit: IdealLoadsLimit,
    air_humidity_ratio: f64,
    independent_load_w: f64,
    availability: f64,
    maximum_capacity_w: f64,
) -> (
    PurchasedAirRuntimeState,
    DirectZonePurchasedAirScheduledCouplingOutput,
) {
    let (model, cache) = fixture(|typed| {
        let system = &mut typed.ideal_loads_air_systems[0];
        system.cooling_limit = cooling_limit;
        system.maximum_cooling_air_flow_rate_m3_per_s = matches!(
            cooling_limit,
            IdealLoadsLimit::LimitFlowRate | IdealLoadsLimit::LimitFlowRateAndCapacity
        )
        .then_some(AutosizeOrNumber::Value(0.05));
        system.maximum_total_cooling_capacity_w = matches!(
            cooling_limit,
            IdealLoadsLimit::LimitCapacity | IdealLoadsLimit::LimitFlowRateAndCapacity
        )
        .then_some(AutosizeOrNumber::Value(maximum_capacity_w));
        system.dehumidification_control_type = DehumidificationControlType::None;
        system.humidification_control_type = HumidificationControlType::None;
        system.minimum_cooling_supply_air_humidity_ratio = f64::NAN;
        schedule_mut(typed, ScheduleId(3)).hourly_value = availability;
    });
    let binding = bind_direct_zone_purchased_air_model(&model).expect("bounded model binding");
    let mut zone_state = zone_state_for_temp_independent_load(independent_load_w);
    zone_state.air_humidity_ratio = air_humidity_ratio;
    zone_state.zone_timestep_average_air_humidity_ratio = air_humidity_ratio;
    zone_state.previous_air_humidity_ratios = [air_humidity_ratio; 3];
    zone_state.previous_system_air_humidity_ratios = [air_humidity_ratio; 3];
    let mut runtime = PurchasedAirRuntimeState::default();
    let output = couple_model_bound_direct_zone_purchased_air(
        DirectZonePurchasedAirScheduledCouplingInput {
            binding: &binding,
            schedule_cache: &cache,
            schedule_sample_index: 0,
            zone_state: &mut zone_state,
            purchased_air_runtime_state: &mut runtime,
            begin_environment: true,
            barometric_pressure_pa: binding.limit_context.barometric_pressure_pa,
            system_timestep_seconds: binding.nominal_system_timestep_seconds,
        },
    )
    .expect("source-ordered CP385 coupling");
    (runtime, output)
}

#[test]
fn binding_places_cp385_after_cp384_and_uses_only_retained_operands() {
    let mut saw_guard_false = false;
    let mut saw_assignment = false;
    for maximum_capacity_w in [500.0, 1.0e9] {
        let (runtime, output) = run_case(
            IdealLoadsLimit::LimitCapacity,
            0.020,
            3_000.0,
            1.0,
            maximum_capacity_w,
        );
        let cp382 = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment;
        let cp384 = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment;
        let cp385 = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment;
        let assignment = cp384.dehumidification_total_output_maximum_capacity_assignment_executed;
        let guard_false = cp384.dehumidification_total_output_capacity_guard_false_fallthrough;

        assert!(
            cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_snapshot_is_exact_direct_release(cp385)
        );
        assert_eq!(cp385.supply_enthalpy_assignment_executed, assignment);
        assert_eq!(
            cp385.dehumidification_total_output_capacity_guard_false_fallthrough,
            guard_false,
        );
        assert_eq!(
            cp385.preexisting_supply_enthalpy_j_per_kg.map(f64::to_bits),
            cp382.supply_enthalpy_j_per_kg.map(f64::to_bits),
        );

        if assignment {
            let mixed = cp382.mixed_air_enthalpy_j_per_kg.expect("CP382 mixed air");
            let total = cp384
                .resulting_cooling_total_output_w
                .expect("CP384 total output");
            let flow = cp382
                .supply_mass_flow_rate_kg_per_s
                .expect("CP382 supply flow");
            let specific = total / flow;
            let expected = mixed - specific;
            assert!(cp385.cp379_retained_supply_enthalpy_owned_read);
            assert!(cp385.cp329_retained_mixed_air_enthalpy_owned_read);
            assert!(cp385.cp384_retained_cooling_total_output_owned_read);
            assert!(cp385.cp330_retained_supply_mass_flow_rate_owned_read);
            assert!(cp385.mixed_air_enthalpy_read);
            assert!(cp385.cooling_total_output_read);
            assert!(cp385.supply_mass_flow_rate_read);
            assert_eq!(
                cp385.specific_cooling_output_j_per_kg.map(f64::to_bits),
                Some(specific.to_bits()),
            );
            assert_eq!(
                cp385.resulting_supply_enthalpy_j_per_kg.map(f64::to_bits),
                Some(expected.to_bits()),
            );
        } else {
            assert!(guard_false);
            assert!(cp385.cp379_retained_supply_enthalpy_owned_read);
            assert!(!cp385.mixed_air_enthalpy_read);
            assert!(!cp385.cooling_total_output_read);
            assert!(!cp385.supply_mass_flow_rate_read);
            assert!(!cp385.supply_enthalpy_assigned);
            assert_eq!(
                cp385.resulting_supply_enthalpy_j_per_kg.map(f64::to_bits),
                cp385.preexisting_supply_enthalpy_j_per_kg.map(f64::to_bits),
            );
        }

        let state = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_lifecycle_summary(
            &runtime,
            output.initialization.system,
        )
        .expect("CP385 lifecycle")
        .state;
        assert_counter_shape(&state, guard_false || assignment, assignment);
        saw_guard_false |= guard_false;
        saw_assignment |= assignment;
    }
    assert!(saw_guard_false, "fixture must exercise a guard-false route");
    assert!(saw_assignment, "fixture must exercise an assignment route");
}

#[test]
fn binding_keeps_cp385_complete_null_for_outer_skips() {
    for (limit, humidity, load, availability, capacity) in [
        (IdealLoadsLimit::LimitCapacity, 0.008, 3_000.0, 1.0, 5_000.0),
        (IdealLoadsLimit::NoLimit, 0.020, 3_000.0, 1.0, 5_000.0),
        (IdealLoadsLimit::NoLimit, 0.020, 3_000.0, 0.0, 5_000.0),
        (IdealLoadsLimit::NoLimit, 0.020, 0.0, 1.0, 5_000.0),
    ] {
        let (runtime, output) = run_case(limit, humidity, load, availability, capacity);
        let snapshot = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment;
        assert!(!snapshot.cp379_retained_supply_enthalpy_owned_read);
        assert!(!snapshot.supply_enthalpy_assignment_executed);
        assert!(snapshot.preexisting_supply_enthalpy_j_per_kg.is_none());
        assert!(snapshot.mixed_air_enthalpy_j_per_kg.is_none());
        assert!(snapshot.cooling_total_output_w.is_none());
        assert!(snapshot.supply_mass_flow_rate_kg_per_s.is_none());
        assert!(snapshot.specific_cooling_output_j_per_kg.is_none());
        assert!(snapshot.calculated_supply_enthalpy_j_per_kg.is_none());
        assert!(snapshot.resulting_supply_enthalpy_j_per_kg.is_none());
        let state = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_lifecycle_summary(
            &runtime,
            output.initialization.system,
        )
        .expect("CP385 skipped lifecycle")
        .state;
        assert_counter_shape(&state, false, false);
    }
}

fn assert_counter_shape(
    state: &PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentRuntimeState,
    retained: bool,
    assignment: bool,
) {
    let retained = usize::from(retained);
    let assignment = usize::from(assignment);
    assert_eq!(state.transition_count, 1);
    assert_eq!(
        state.cp379_retained_supply_enthalpy_owned_read_count,
        retained
    );
    assert_eq!(
        state.post_saturation_capacity_limited_dehumidification_supply_enthalpy_assignment_count,
        assignment,
    );
    assert_eq!(state.source_site_execution_count, 6 * assignment);
    for count in [
        state.cp329_retained_mixed_air_enthalpy_owned_read_count,
        state.mixed_air_enthalpy_read_count,
        state.cp384_retained_cooling_total_output_owned_read_count,
        state.cooling_total_output_read_count,
        state.cp330_retained_supply_mass_flow_rate_owned_read_count,
        state.supply_mass_flow_rate_read_count,
        state.specific_cooling_output_calculation_count,
        state.supply_enthalpy_difference_calculation_count,
        state.supply_enthalpy_assignment_write_count,
    ] {
        assert_eq!(count, assignment);
    }
}
