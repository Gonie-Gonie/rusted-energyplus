use super::*;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitSnapshot,
    cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_snapshot_is_exact_direct_release,
    purchased_air_calc_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_lifecycle_summary,
};

fn run_case(
    cooling_limit: IdealLoadsLimit,
    maximum_capacity_w: Option<f64>,
    independent_load_w: f64,
    availability: f64,
) -> (
    PurchasedAirRuntimeState,
    DirectZonePurchasedAirScheduledCouplingOutput,
) {
    let (model, cache) = fixture(|typed| {
        let system = &mut typed.ideal_loads_air_systems[0];
        system.cooling_limit = cooling_limit;
        system.maximum_cooling_air_flow_rate_m3_per_s = None;
        system.maximum_total_cooling_capacity_w = maximum_capacity_w.map(AutosizeOrNumber::Value);
        system.dehumidification_control_type = DehumidificationControlType::None;
        system.minimum_cooling_supply_air_humidity_ratio = f64::NAN;
        schedule_mut(typed, ScheduleId(3)).hourly_value = availability;
    });
    let binding = bind_direct_zone_purchased_air_model(&model).expect("bounded model binding");
    let mut zone_state = zone_state_for_temp_independent_load(independent_load_w);
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
    .expect("source-ordered CP356 coupling");
    (runtime, output)
}

#[test]
fn scheduled_binding_places_cp356_after_cp355_as_complete_null_none_skip_without_feeding_numerical_output()
 {
    for (cooling_limit, maximum_capacity_w) in [
        (IdealLoadsLimit::NoLimit, None),
        (IdealLoadsLimit::LimitCapacity, Some(1.0e9)),
        (IdealLoadsLimit::LimitCapacity, Some(1.0)),
    ] {
        let (runtime, output) = run_case(cooling_limit, maximum_capacity_w, 3_000.0, 1.0);
        let predecessor =
            output.calculation_cooling_constant_shr_supply_humidity_ratio_minimum_limit;
        let snapshot =
            output.calculation_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit;

        assert!(predecessor.dehumidification_control_none_case_completed_skip);
        assert_eq!(
            snapshot.parent_call_ordinal,
            predecessor.parent_call_ordinal
        );
        assert_eq!(snapshot.system, predecessor.system);
        assert!(
            cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_snapshot_is_exact_direct_release(
                snapshot,
            )
        );
        assert!(snapshot.dehumidification_control_none_case_completed_skip);
        assert_complete_null(snapshot);

        let state =
            purchased_air_calc_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_lifecycle_summary(
                &runtime,
                output.initialization.system,
            )
            .expect("CP356 lifecycle")
            .state;
        assert_eq!(state.transition_count, 1);
        assert_eq!(
            state.dehumidification_control_none_case_completed_skip_count,
            1
        );
        assert_eq!(
            state
                .dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_mixed_air_limit_count,
            0
        );
        assert_eq!(state.source_site_execution_count, 0);
        assert!(
            output
                .coupling
                .purchased_air
                .supply_node_update
                .humidity_ratio
                .is_finite(),
            "CP356 evidence must not replace the numerical PurchasedAir result"
        );
    }
}

#[test]
fn scheduled_binding_preserves_inherited_u_n_and_p_as_complete_null_skips() {
    for (cooling_limit, maximum_capacity_w, load, availability, expected) in [
        (
            IdealLoadsLimit::NoLimit,
            None,
            3_000.0,
            0.0,
            (true, false, false),
        ),
        (
            IdealLoadsLimit::NoLimit,
            None,
            0.0,
            1.0,
            (false, true, false),
        ),
        (
            IdealLoadsLimit::LimitCapacity,
            Some(-0.0),
            3_000.0,
            1.0,
            (false, false, true),
        ),
    ] {
        let (_, output) = run_case(cooling_limit, maximum_capacity_w, load, availability);
        let snapshot =
            output.calculation_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit;
        assert_eq!(
            (
                snapshot.unit_off_skipped,
                snapshot.non_cooling_skipped,
                snapshot.positive_guard_false_fallthrough_skipped,
            ),
            expected
        );
        assert_complete_null(snapshot);
    }
}

fn assert_complete_null(
    snapshot: PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMixedAirLimitSnapshot,
) {
    assert!(
        !snapshot
            .dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_mixed_air_limit_executed
    );
    assert!(!snapshot.supply_humidity_ratio_for_mixed_air_limit_minimum_read);
    assert!(!snapshot.mixed_air_humidity_ratio_for_minimum_read);
    assert!(!snapshot.source_shaped_two_argument_minimum_evaluated);
    assert!(!snapshot.supply_humidity_ratio_assignment_performed);
    assert_eq!(
        [
            snapshot.supply_humidity_ratio_before_mixed_air_limit,
            snapshot.mixed_air_humidity_ratio,
            snapshot.minimum_supply_humidity_ratio,
            snapshot.assigned_supply_humidity_ratio,
            snapshot.resulting_supply_humidity_ratio,
        ],
        [None; 5]
    );
}
