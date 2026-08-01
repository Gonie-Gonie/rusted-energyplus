use super::*;
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentRuntimeState,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentSnapshot,
    cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_snapshot_is_exact_direct_release,
    purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_lifecycle_summary,
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
    .expect("source-ordered CP382 coupling");
    (runtime, output)
}

#[test]
fn binding_places_cp382_after_cp381_and_uses_exact_same_call_numerical_owners() {
    let (runtime, output) = run_case(IdealLoadsLimit::LimitCapacity, 0.020, 3_000.0, 1.0, 5_000.0);
    let predecessor =
        output.calculation_cooling_post_saturation_capacity_limit_dehumidification_guard;
    let supply_mass_flow_owner = output.calculation_cooling_supply_mass_flow_positive_guard;
    let mixed_air_owner = output.calculation_cooling_mixed_air_call;
    let early_total_corroborator =
        output.calculation_cooling_positive_supply_capacity_limit_sensible_output_assignment;
    let supply_enthalpy_owner =
        output.calculation_cooling_supply_enthalpy_post_saturation_assignment;
    let snapshot = output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment;

    assert!(predecessor.dehumidification_body_entered);
    assert!(
        cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_snapshot_is_exact_direct_release(
            snapshot,
        )
    );
    assert!(snapshot.dehumidification_total_output_assignment_executed);
    assert_all_owner_evidence(snapshot);
    assert_eq!(
        snapshot.supply_mass_flow_rate_kg_per_s.unwrap().to_bits(),
        supply_mass_flow_owner
            .supply_mass_flow_rate_kg_per_s
            .unwrap()
            .to_bits(),
    );
    assert_eq!(
        snapshot.supply_mass_flow_rate_kg_per_s.unwrap().to_bits(),
        mixed_air_owner
            .supply_mass_flow_rate_kg_per_s
            .unwrap()
            .to_bits(),
    );
    assert_eq!(
        snapshot.supply_mass_flow_rate_kg_per_s.unwrap().to_bits(),
        early_total_corroborator
            .supply_mass_flow_rate_kg_per_s
            .unwrap()
            .to_bits(),
    );
    assert_eq!(
        snapshot.mixed_air_enthalpy_j_per_kg.unwrap().to_bits(),
        mixed_air_owner
            .mixed_air_enthalpy_projection_j_per_kg
            .unwrap()
            .to_bits(),
    );
    assert_eq!(
        snapshot.mixed_air_enthalpy_j_per_kg.unwrap().to_bits(),
        early_total_corroborator
            .mixed_air_enthalpy_j_per_kg
            .unwrap()
            .to_bits(),
    );
    assert_eq!(
        snapshot.supply_enthalpy_j_per_kg.unwrap().to_bits(),
        supply_enthalpy_owner
            .resulting_supply_enthalpy_j_per_kg
            .unwrap()
            .to_bits(),
    );
    let expected_difference =
        snapshot.mixed_air_enthalpy_j_per_kg.unwrap() - snapshot.supply_enthalpy_j_per_kg.unwrap();
    let expected_total = snapshot.supply_mass_flow_rate_kg_per_s.unwrap() * expected_difference;
    assert_eq!(
        snapshot
            .mixed_air_minus_supply_enthalpy_j_per_kg
            .unwrap()
            .to_bits(),
        expected_difference.to_bits(),
    );
    assert_eq!(
        snapshot.cooling_total_output_w.unwrap().to_bits(),
        expected_total.to_bits(),
    );

    let state =
        purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_lifecycle_summary(
            &runtime,
            output.initialization.system,
        )
        .expect("CP382 lifecycle")
        .state;
    assert_counter_shape(&state, 1);
}

#[test]
fn binding_keeps_cp382_complete_null_for_cp381_false_and_outer_false_routes() {
    for (limit, humidity, load, availability, maximum_capacity_w) in [
        (IdealLoadsLimit::LimitCapacity, 0.008, 3_000.0, 1.0, 5_000.0),
        (IdealLoadsLimit::NoLimit, 0.020, 3_000.0, 1.0, 5_000.0),
        (IdealLoadsLimit::NoLimit, 0.020, 3_000.0, 0.0, 5_000.0),
        (IdealLoadsLimit::NoLimit, 0.020, 0.0, 1.0, 5_000.0),
        (IdealLoadsLimit::LimitCapacity, 0.020, 3_000.0, 1.0, 0.0),
    ] {
        let (runtime, output) = run_case(limit, humidity, load, availability, maximum_capacity_w);
        let predecessor =
            output.calculation_cooling_post_saturation_capacity_limit_dehumidification_guard;
        let snapshot = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment;
        assert!(!predecessor.dehumidification_body_entered);
        assert_complete_null(snapshot);
        let state =
            purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_assignment_lifecycle_summary(
                &runtime,
                output.initialization.system,
            )
            .expect("CP382 skipped lifecycle")
            .state;
        assert_counter_shape(&state, 0);
    }
}

fn assert_all_owner_evidence(
    snapshot: PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentSnapshot,
) {
    assert!(snapshot.cp330_supply_mass_flow_rate_owned_read);
    assert!(snapshot.cp329_same_call_supply_mass_flow_rate_bit_corroborated);
    assert!(snapshot.cp339_same_call_supply_mass_flow_rate_bit_corroborated);
    assert!(snapshot.cp329_mixed_air_enthalpy_owned_read);
    assert!(snapshot.cp329_same_call_recirculation_enthalpy_bit_corroborated);
    assert!(snapshot.cp339_same_call_mixed_air_enthalpy_bit_corroborated);
    assert!(snapshot.cp379_post_saturation_supply_enthalpy_owned_read);
    assert!(snapshot.cp379_same_call_supply_enthalpy_bits_corroborated);
}

fn assert_complete_null(
    snapshot: PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentSnapshot,
) {
    assert!(!snapshot.dehumidification_total_output_assignment_executed);
    assert!(!snapshot.supply_mass_flow_rate_read);
    assert!(snapshot.supply_mass_flow_rate_kg_per_s.is_none());
    assert!(!snapshot.mixed_air_enthalpy_read);
    assert!(snapshot.mixed_air_enthalpy_j_per_kg.is_none());
    assert!(!snapshot.supply_enthalpy_read);
    assert!(snapshot.supply_enthalpy_j_per_kg.is_none());
    assert!(!snapshot.enthalpy_difference_calculated);
    assert!(snapshot.mixed_air_minus_supply_enthalpy_j_per_kg.is_none());
    assert!(!snapshot.cooling_total_output_calculated);
    assert!(snapshot.calculated_cooling_total_output_w.is_none());
    assert!(!snapshot.cooling_total_output_assigned);
    assert!(snapshot.cooling_total_output_w.is_none());
}

fn assert_counter_shape(
    state: &PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentRuntimeState,
    assignments: usize,
) {
    assert_eq!(state.transition_count, 1);
    assert_eq!(
        state.dehumidification_total_output_assignment_count,
        assignments
    );
    assert_eq!(
        state.source_site_execution_count,
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_ASSIGNMENT_SOURCE_ORDER.len()
            * assignments,
    );
    assert_eq!(state.supply_mass_flow_rate_read_count, assignments);
    assert_eq!(state.mixed_air_enthalpy_read_count, assignments);
    assert_eq!(state.supply_enthalpy_read_count, assignments);
    assert_eq!(state.enthalpy_difference_calculation_count, assignments);
    assert_eq!(state.cooling_total_output_calculation_count, assignments);
    assert_eq!(
        state.cooling_total_output_assignment_write_count,
        assignments
    );
}
