use super::*;
use crate::ideal_loads::{
    cooling_humidistat_moisture_demand_assignment_snapshot_is_exact_direct_release,
    purchased_air_calc_cooling_humidistat_moisture_demand_assignment_lifecycle_summary,
};

fn run_case(
    cooling_limit: IdealLoadsLimit,
    maximum_capacity_w: Option<f64>,
    independent_load_w: f64,
    availability: f64,
) -> Option<(
    PurchasedAirRuntimeState,
    DirectZonePurchasedAirScheduledCouplingOutput,
)> {
    let (model, cache) = fixture(|typed| {
        let system = &mut typed.ideal_loads_air_systems[0];
        system.cooling_limit = cooling_limit;
        system.maximum_cooling_air_flow_rate_m3_per_s = None;
        system.maximum_total_cooling_capacity_w = maximum_capacity_w.map(AutosizeOrNumber::Value);
        system.dehumidification_control_type = DehumidificationControlType::None;
        system.minimum_cooling_supply_air_humidity_ratio = f64::NAN;
        schedule_mut(typed, ScheduleId(3)).hourly_value = availability;
    });
    let binding_result = bind_direct_zone_purchased_air_model(&model);
    assert!(binding_result.is_ok());
    let Ok(binding) = binding_result else {
        return None;
    };
    let mut zone_state = zone_state_for_temp_independent_load(independent_load_w);
    let mut runtime = PurchasedAirRuntimeState::default();
    let output_result = couple_model_bound_direct_zone_purchased_air(
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
    );
    assert!(output_result.is_ok());
    output_result.ok().map(|output| (runtime, output))
}

#[test]
fn scheduled_binding_places_cp359_after_cp358_without_reading_moisture_demand() {
    for (cooling_limit, maximum_capacity_w) in [
        (IdealLoadsLimit::NoLimit, None),
        (IdealLoadsLimit::LimitCapacity, Some(1.0e9)),
        (IdealLoadsLimit::LimitCapacity, Some(1.0)),
    ] {
        let Some((runtime, output)) = run_case(cooling_limit, maximum_capacity_w, 3_000.0, 1.0)
        else {
            return;
        };
        let predecessor = output.calculation_cooling_humidistat_case_entry;
        let snapshot = output.calculation_cooling_humidistat_moisture_demand_assignment;

        assert_eq!(
            (
                snapshot.system,
                snapshot.parent_call_ordinal,
                snapshot.controlled_zone,
            ),
            (
                predecessor.system,
                predecessor.parent_call_ordinal,
                predecessor.controlled_zone,
            )
        );
        assert!(
            cooling_humidistat_moisture_demand_assignment_snapshot_is_exact_direct_release(snapshot)
        );
        assert!(snapshot.dehumidification_control_none_case_completed_skip);
        assert!(
            !snapshot.dehumidification_control_humidistat_moisture_demand_assignment_executed
        );
        assert!(!snapshot.zone_dehumidifying_setpoint_moisture_demand_read);
        assert!(
            snapshot
                .zone_dehumidifying_setpoint_moisture_demand_kg_per_s
                .is_none()
        );
        assert!(!snapshot.zone_dehumidifying_setpoint_moisture_demand_assigned);
        assert!(
            snapshot
                .assigned_zone_dehumidifying_setpoint_moisture_demand_kg_per_s
                .is_none()
        );
        assert!(
            snapshot
                .resulting_zone_dehumidifying_setpoint_moisture_demand_kg_per_s
                .is_none()
        );

        let summary_result =
            purchased_air_calc_cooling_humidistat_moisture_demand_assignment_lifecycle_summary(
                &runtime,
                output.initialization.system,
            );
        assert!(summary_result.is_ok());
        let Ok(summary) = summary_result else {
            return;
        };
        assert_eq!(summary.state.transition_count, 1);
        assert_eq!(
            summary
                .state
                .dehumidification_control_none_case_completed_skip_count,
            1
        );
        assert_eq!(
            summary
                .state
                .dehumidification_control_humidistat_moisture_demand_assignment_count,
            0
        );
        assert_eq!(summary.state.source_site_execution_count, 0);
        assert_eq!(
            summary
                .state
                .zone_dehumidifying_setpoint_moisture_demand_read_count,
            0
        );
        assert_eq!(
            summary
                .state
                .zone_dehumidifying_setpoint_moisture_demand_assignment_count,
            0
        );

        let cp345 = output
            .calculation_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment;
        assert_eq!(
            cp345.assigned_supply_humidity_ratio.map(f64::to_bits),
            Some(
                output
                    .coupling
                    .purchased_air
                    .supply_node_update
                    .humidity_ratio
                    .to_bits()
            )
        );
    }
}

#[test]
fn scheduled_binding_preserves_u_n_p_skips_and_rejects_private_case_routes() {
    for (load, availability, capacity, route) in [
        (3_000.0, 0.0, None, (true, false, false)),
        (0.0, 1.0, None, (false, true, false)),
        (3_000.0, 1.0, Some(-0.0), (false, false, true)),
    ] {
        let limit = if capacity.is_some() {
            IdealLoadsLimit::LimitCapacity
        } else {
            IdealLoadsLimit::NoLimit
        };
        let Some((_, output)) = run_case(limit, capacity, load, availability) else {
            return;
        };
        let snapshot = output.calculation_cooling_humidistat_moisture_demand_assignment;
        assert_eq!(
            (
                snapshot.unit_off_skipped,
                snapshot.non_cooling_skipped,
                snapshot.positive_guard_false_fallthrough_skipped,
            ),
            route
        );
        assert!(
            cooling_humidistat_moisture_demand_assignment_snapshot_is_exact_direct_release(snapshot)
        );
        assert!(!snapshot.zone_dehumidifying_setpoint_moisture_demand_read);
        assert!(!snapshot.zone_dehumidifying_setpoint_moisture_demand_assigned);
    }

    for control in [
        DehumidificationControlType::ConstantSensibleHeatRatio,
        DehumidificationControlType::Humidistat,
        DehumidificationControlType::ConstantSupplyHumidityRatio,
    ] {
        let (model, _) = fixture(|typed| {
            typed.ideal_loads_air_systems[0].dehumidification_control_type = control;
        });
        assert!(bind_direct_zone_purchased_air_model(&model).is_err());
    }
}
