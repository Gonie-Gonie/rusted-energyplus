use super::*;
use crate::ideal_loads::{
    cooling_post_saturation_capacity_limit_dehumidification_control_switch_snapshot_is_exact_direct_release,
    purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_switch_lifecycle_summary,
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
    .expect("source-ordered CP386 coupling");
    (runtime, output)
}

#[test]
fn binding_dispatches_cp386_none_selector_only_after_cp385_assignment() {
    for (capacity, expected_active) in [(500.0, true), (1.0e9, false)] {
        let (runtime, output) = run_case(IdealLoadsLimit::LimitCapacity, 0.020, 1.0, capacity);
        let predecessor = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment;
        let snapshot = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_switch;

        assert_eq!(
            predecessor.supply_enthalpy_assignment_executed,
            expected_active
        );
        assert!(
            cooling_post_saturation_capacity_limit_dehumidification_control_switch_snapshot_is_exact_direct_release(snapshot)
        );
        assert_eq!(snapshot.dehumidification_control_type_read, expected_active);
        assert_eq!(
            snapshot.dehumidification_control_type,
            expected_active.then_some(DehumidificationControlType::None),
        );
        assert_eq!(
            snapshot.dehumidification_control_switch_dispatched,
            expected_active
        );

        let lifecycle =
            purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_switch_lifecycle_summary(
                &runtime,
                output.initialization.system,
            )
            .expect("CP386 lifecycle");
        assert_eq!(lifecycle.state.transition_count, 1);
        assert_eq!(lifecycle.state.latest, Some(snapshot));
    }
}

#[test]
fn binding_keeps_cp386_selector_null_on_outer_skip() {
    let (_, output) = run_case(IdealLoadsLimit::NoLimit, 0.008, 0.0, 5_000.0);
    let snapshot =
        output.calculation_cooling_post_saturation_capacity_limit_dehumidification_control_switch;
    assert!(!snapshot.dehumidification_control_type_read);
    assert!(snapshot.dehumidification_control_type.is_none());
    assert!(!snapshot.dehumidification_control_switch_dispatched);
}
