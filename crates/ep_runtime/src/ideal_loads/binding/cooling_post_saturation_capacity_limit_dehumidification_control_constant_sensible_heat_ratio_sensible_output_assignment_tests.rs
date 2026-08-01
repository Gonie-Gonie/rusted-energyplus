use super::*;
use crate::ideal_loads::{
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_predecessor_counts_match_exact_direct_cp_air_assignment,
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_snapshot_is_exact_direct_release,
    purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_lifecycle_summary,
    purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_lifecycle_summary,
};

fn run_case(
    cooling_limit: IdealLoadsLimit,
    air_humidity_ratio: f64,
    availability: f64,
    maximum_capacity_w: f64,
) -> (
    PurchasedAirRuntimeState,
    DirectZonePurchasedAirScheduledCouplingOutput,
) {
    let (model, cache) = fixture(|typed| {
        let system = &mut typed.ideal_loads_air_systems[0];
        system.cooling_limit = cooling_limit;
        system.maximum_cooling_air_flow_rate_m3_per_s = None;
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
    let mut zone_state = zone_state_for_temp_independent_load(3_000.0);
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
    .expect("source-ordered CP388 coupling");
    (runtime, output)
}

#[test]
fn binding_completes_cp388_as_exact_none_selector_skip_after_cp387() {
    for (limit, humidity_ratio, capacity, availability) in [
        (IdealLoadsLimit::LimitCapacity, 0.020, 500.0, 1.0),
        (IdealLoadsLimit::LimitCapacity, 0.020, 1.0e9, 1.0),
        (IdealLoadsLimit::NoLimit, 0.008, 5_000.0, 0.0),
    ] {
        let (runtime, output) = run_case(limit, humidity_ratio, availability, capacity);
        let predecessor = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment;
        let snapshot = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment;
        assert!(
            cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_snapshot_is_exact_direct_release(
                snapshot,
            )
        );
        assert_eq!(
            snapshot
                .predecessor_resulting_supply_enthalpy_j_per_kg
                .map(f64::to_bits),
            predecessor
                .resulting_supply_enthalpy_j_per_kg
                .map(f64::to_bits)
        );
        assert_eq!(
            snapshot.predecessor_cp_air_j_per_kg_k.map(f64::to_bits),
            predecessor.cp_air_j_per_kg_k.map(f64::to_bits)
        );
        assert_eq!(
            snapshot
                .resulting_supply_enthalpy_j_per_kg
                .map(f64::to_bits),
            predecessor
                .resulting_supply_enthalpy_j_per_kg
                .map(f64::to_bits)
        );
        assert_complete_null(snapshot);

        let system = output.initialization.system;
        let lifecycle = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_lifecycle_summary(
            &runtime,
            system,
        )
        .expect("CP388 lifecycle");
        let predecessor_lifecycle = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_lifecycle_summary(
            &runtime,
            system,
        )
        .expect("CP387 lifecycle");
        assert_eq!(lifecycle.state.transition_count, 1);
        assert_eq!(lifecycle.state.inactive_transition_count, 1);
        assert_eq!(lifecycle.state.source_site_execution_count, 0);
        assert_eq!(
            lifecycle.state.predecessor_route_counts,
            predecessor_lifecycle.state.predecessor_route_counts
        );
        assert!(
            cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_predecessor_counts_match_exact_direct_cp_air_assignment(
                &lifecycle.state,
                &predecessor_lifecycle.state,
            )
        );
    }
}

fn assert_complete_null(
    snapshot: crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentSnapshot,
) {
    assert!(
        !snapshot
            .dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_executed
    );
    for flag in [
        snapshot.cp384_retained_cooling_total_output_owned_read,
        snapshot.cp385_cooling_total_output_bit_corroborated,
        snapshot.cooling_total_output_read,
        snapshot.cooling_sensible_heat_ratio_read,
        snapshot.cooling_sensible_output_calculated,
        snapshot.cooling_sensible_output_assigned,
    ] {
        assert!(!flag);
    }
    for value in [
        snapshot.cooling_total_output_w,
        snapshot.cooling_sensible_heat_ratio,
        snapshot.calculated_cooling_sensible_output_w,
        snapshot.cooling_sensible_output_w,
    ] {
        assert!(value.is_none());
    }
}
