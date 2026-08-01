use super::*;
use crate::ideal_loads::{
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_snapshot_is_exact_direct_release,
    purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_lifecycle_summary,
    purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_lifecycle_summary,
};

pub(super) fn run_case(
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
    .expect("source-ordered CP389 coupling");
    (runtime, output)
}

#[test]
fn binding_preserves_cp379_temperature_on_direct_cp389_skips() {
    for (limit, humidity_ratio, availability, capacity) in [
        (IdealLoadsLimit::NoLimit, 0.008, 0.0, 5_000.0),
        (IdealLoadsLimit::NoLimit, 0.008, 1.0, 5_000.0),
        (IdealLoadsLimit::LimitCapacity, 0.020, 1.0, 500.0),
    ] {
        let (runtime, output) = run_case(limit, humidity_ratio, availability, capacity);
        let owner_cp379 = output.calculation_cooling_supply_enthalpy_post_saturation_assignment;
        let predecessor_cp388 = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment;
        let snapshot = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment;
        assert!(
            cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_snapshot_is_exact_direct_release(snapshot)
        );
        assert_eq!(
            snapshot.preexisting_supply_temperature_c.map(f64::to_bits),
            owner_cp379.supply_temperature_c.map(f64::to_bits)
        );
        assert_eq!(
            snapshot.resulting_supply_temperature_c.map(f64::to_bits),
            owner_cp379.supply_temperature_c.map(f64::to_bits)
        );
        assert_eq!(
            snapshot
                .resulting_supply_enthalpy_j_per_kg
                .map(f64::to_bits),
            predecessor_cp388
                .resulting_supply_enthalpy_j_per_kg
                .map(f64::to_bits)
        );
        assert_source_local_skip(snapshot);

        let system = output.initialization.system;
        let lifecycle = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_lifecycle_summary(
            &runtime,
            system,
        )
        .expect("CP389 lifecycle");
        let predecessor = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_lifecycle_summary(
            &runtime,
            system,
        )
        .expect("CP388 lifecycle");
        assert_eq!(lifecycle.state.transition_count, 1);
        assert_eq!(lifecycle.state.inactive_transition_count, 1);
        assert_eq!(lifecycle.state.source_site_execution_count, 0);
        assert_eq!(
            lifecycle.state.predecessor_route_counts,
            predecessor.state.predecessor_route_counts
        );
    }
}

fn assert_source_local_skip(
    snapshot: crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentSnapshot,
) {
    assert!(!snapshot.dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_executed);
    for flag in [
        snapshot.mixed_air_temperature_read,
        snapshot.cooling_sensible_output_read,
        snapshot.cp_air_read,
        snapshot.supply_mass_flow_rate_read,
        snapshot.cp_air_times_supply_mass_flow_rate_calculated,
        snapshot.cooling_sensible_output_over_air_capacity_rate_calculated,
        snapshot.supply_temperature_calculated,
        snapshot.supply_temperature_assigned,
    ] {
        assert!(!flag);
    }
    for value in [
        snapshot.mixed_air_temperature_c,
        snapshot.cooling_sensible_output_w,
        snapshot.cp_air_j_per_kg_k,
        snapshot.supply_mass_flow_rate_kg_per_s,
        snapshot.cp_air_times_supply_mass_flow_rate_w_per_k,
        snapshot.cooling_sensible_output_over_air_capacity_rate_k,
        snapshot.calculated_supply_temperature_c,
        snapshot.assigned_supply_temperature_c,
    ] {
        assert!(value.is_none());
    }
}
