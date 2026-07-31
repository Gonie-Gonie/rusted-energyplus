use super::*;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentRuntimeState,
    cooling_default_supply_humidity_ratio_mixed_air_assignment_snapshot_is_exact_direct_release,
    purchased_air_calc_cooling_default_supply_humidity_ratio_mixed_air_assignment_lifecycle_summary,
};

use super::cooling_humidistat_moisture_demand_assignment_tests::run_case;

#[test]
fn binding_orders_cp366_then_cp367_before_numerical_and_does_not_feed_result() {
    for (cooling_limit, maximum_capacity_w) in [
        (IdealLoadsLimit::NoLimit, None),
        (IdealLoadsLimit::LimitCapacity, Some(1.0e9)),
        (IdealLoadsLimit::LimitCapacity, Some(1.0)),
    ] {
        let case = run_case(cooling_limit, maximum_capacity_w, 3_000.0, 1.0);
        assert!(case.is_some(), "CP367 binding fixture must succeed");
        let Some((runtime, output)) = case else {
            return;
        };
        let predecessor = output.calculation_cooling_constant_supply_humidity_ratio_case_break;
        let snapshot =
            output.calculation_cooling_default_supply_humidity_ratio_mixed_air_assignment;
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
            cooling_default_supply_humidity_ratio_mixed_air_assignment_snapshot_is_exact_direct_release(
                snapshot,
            )
        );
        assert!(snapshot.dehumidification_control_none_case_completed_skip);
        assert!(
            !snapshot
                .dehumidification_control_default_supply_humidity_ratio_mixed_air_assignment_executed
        );

        let summary =
            purchased_air_calc_cooling_default_supply_humidity_ratio_mixed_air_assignment_lifecycle_summary(
                &runtime,
                output.initialization.system,
            );
        assert!(summary.is_ok(), "CP367 lifecycle summary must be available");
        let Ok(summary) = summary else {
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
                .dehumidification_control_constant_supply_humidity_ratio_case_completed_skip_count,
            0
        );
        assert_eq!(summary.state.mixed_air_humidity_ratio_read_count, 0);
        assert_eq!(summary.state.supply_humidity_ratio_assignment_count, 0);
        assert_eq!(summary.state.source_site_execution_count, 0);

        let numerical_owner = output
            .calculation_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment
            .assigned_supply_humidity_ratio;
        let numerical = output
            .coupling
            .purchased_air
            .supply_node_update
            .humidity_ratio;
        assert_eq!(numerical_owner.map(f64::to_bits), Some(numerical.to_bits()));
    }
}

#[test]
fn binding_cp367_is_complete_skip_for_every_direct_route() {
    for (load, availability, capacity, expected) in [
        (3_000.0, 0.0, None, (true, false, false, false)),
        (0.0, 1.0, None, (false, true, false, false)),
        (3_000.0, 1.0, Some(-0.0), (false, false, true, false)),
        (3_000.0, 1.0, Some(1.0), (false, false, false, true)),
    ] {
        let limit = if capacity.is_some() {
            IdealLoadsLimit::LimitCapacity
        } else {
            IdealLoadsLimit::NoLimit
        };
        let case = run_case(limit, capacity, load, availability);
        assert!(case.is_some(), "CP367 route fixture must succeed");
        let Some((_, output)) = case else {
            return;
        };
        let snapshot =
            output.calculation_cooling_default_supply_humidity_ratio_mixed_air_assignment;
        assert_eq!(
            (
                snapshot.unit_off_skipped,
                snapshot.non_cooling_skipped,
                snapshot.positive_guard_false_fallthrough_skipped,
                snapshot.dehumidification_control_none_case_completed_skip,
            ),
            expected
        );
        assert!(
            cooling_default_supply_humidity_ratio_mixed_air_assignment_snapshot_is_exact_direct_release(
                snapshot,
            )
        );
        assert!(
            !snapshot
                .dehumidification_control_default_supply_humidity_ratio_mixed_air_assignment_executed
        );
    }
}

#[test]
fn binding_rejects_corrupt_cp366_without_mutation() {
    let (model, cache) = fixture(|typed| {
        let system = &mut typed.ideal_loads_air_systems[0];
        system.cooling_limit = IdealLoadsLimit::NoLimit;
        system.maximum_cooling_air_flow_rate_m3_per_s = None;
        system.maximum_total_cooling_capacity_w = None;
        system.dehumidification_control_type = DehumidificationControlType::None;
        system.minimum_cooling_supply_air_humidity_ratio = f64::NAN;
        schedule_mut(typed, ScheduleId(3)).hourly_value = 1.0;
    });
    let binding = bind_direct_zone_purchased_air_model(&model).expect("CP367 binding");
    let mut zone_state = zone_state_for_temp_independent_load(3_000.0);
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
    .expect("canonical CP367 call");
    let system = output.initialization.system;
    runtime
        .units
        .get_mut(&system)
        .expect("retained CP367 unit")
        .calc_cooling_default_supply_humidity_ratio_mixed_air_assignment =
        PurchasedAirCalcCoolingDefaultSupplyHumidityRatioMixedAirAssignmentRuntimeState::new(
            system,
        );
    runtime
        .clear_cooling_default_supply_humidity_ratio_mixed_air_assignment_latest_witness_for_test(
            system,
        );
    let before = runtime
        .units
        .get(&system)
        .expect("retained CP367 unit")
        .calc_cooling_default_supply_humidity_ratio_mixed_air_assignment
        .clone();

    let mut corrupt = output.calculation_cooling_constant_supply_humidity_ratio_case_break;
    corrupt.parent_call_ordinal = corrupt.parent_call_ordinal.wrapping_add(1);
    assert!(
        advance_cooling_default_supply_humidity_ratio_mixed_air_assignment(
            &mut runtime,
            binding.system,
            corrupt,
        )
        .is_err()
    );
    assert_eq!(
        runtime
            .units
            .get(&system)
            .expect("retained CP367 unit")
            .calc_cooling_default_supply_humidity_ratio_mixed_air_assignment,
        before
    );
    assert!(
        runtime
            .cooling_default_supply_humidity_ratio_mixed_air_assignment_latest_witness(system)
            .is_none()
    );
}
