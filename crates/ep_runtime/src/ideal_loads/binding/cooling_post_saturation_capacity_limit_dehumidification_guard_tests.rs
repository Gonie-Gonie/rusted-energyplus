use super::*;
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardRuntimeState,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardSnapshot,
    cooling_post_saturation_capacity_limit_dehumidification_guard_snapshot_is_exact_direct_release,
    purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_guard_lifecycle_summary,
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
    .expect("source-ordered CP381 coupling");
    (runtime, output)
}

#[test]
fn binding_places_cp381_after_cp380_and_uses_exact_same_call_humidity_owners() {
    for (humidity_ratio, expected_less_than) in [(0.008, false), (0.020, true)] {
        let (runtime, output) = run_case(
            IdealLoadsLimit::LimitCapacity,
            humidity_ratio,
            3_000.0,
            1.0,
            5_000.0,
        );
        let predecessor = output.calculation_cooling_post_saturation_capacity_limit_guard;
        let supply_owner =
            output.calculation_cooling_supply_humidity_ratio_saturation_limit_assignment;
        let supply_corroborator =
            output.calculation_cooling_supply_enthalpy_post_saturation_assignment;
        let mixed_air_owner = output.calculation_cooling_mixed_air_call;
        let snapshot =
            output.calculation_cooling_post_saturation_capacity_limit_dehumidification_guard;

        assert!(predecessor.capacity_limit_body_entered);
        assert!(
            cooling_post_saturation_capacity_limit_dehumidification_guard_snapshot_is_exact_direct_release(
                snapshot,
            )
        );
        assert!(snapshot.dehumidification_guard_evaluated);
        assert!(snapshot.cp378_supply_humidity_ratio_saturation_limit_owned_read);
        assert!(snapshot.cp379_same_call_supply_humidity_ratio_bit_corroborated);
        assert!(snapshot.cp329_mixed_air_humidity_ratio_owned_read);
        assert_eq!(
            snapshot.supply_humidity_ratio.unwrap().to_bits(),
            supply_owner
                .resulting_supply_humidity_ratio
                .unwrap()
                .to_bits(),
        );
        assert_eq!(
            snapshot.supply_humidity_ratio.unwrap().to_bits(),
            supply_corroborator.supply_humidity_ratio.unwrap().to_bits(),
        );
        assert_eq!(
            snapshot.mixed_air_humidity_ratio.unwrap().to_bits(),
            mixed_air_owner.mixed_air_humidity_ratio.unwrap().to_bits(),
        );
        assert_eq!(
            snapshot.supply_humidity_ratio_strictly_less_than_mixed_air_humidity_ratio,
            Some(expected_less_than),
        );
        assert_eq!(snapshot.dehumidification_body_entered, expected_less_than);
        assert_eq!(
            snapshot.dehumidification_guard_false_fallthrough,
            !expected_less_than,
        );

        let state =
            purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_guard_lifecycle_summary(
                &runtime,
                output.initialization.system,
            )
            .expect("CP381 lifecycle")
            .state;
        assert_counter_shape(&state, 1, usize::from(expected_less_than));
    }
}

#[test]
fn binding_keeps_cp381_complete_null_when_cp380_does_not_enter_its_body() {
    for (limit, load, availability, maximum_capacity_w, expected_base) in [
        (
            IdealLoadsLimit::NoLimit,
            3_000.0,
            0.0,
            5_000.0,
            (true, false, false),
        ),
        (
            IdealLoadsLimit::NoLimit,
            0.0,
            1.0,
            5_000.0,
            (false, true, false),
        ),
        (
            IdealLoadsLimit::LimitCapacity,
            3_000.0,
            1.0,
            0.0,
            (false, false, true),
        ),
        (
            IdealLoadsLimit::LimitFlowRate,
            3_000.0,
            1.0,
            5_000.0,
            (false, false, false),
        ),
    ] {
        let (runtime, output) = run_case(limit, 0.020, load, availability, maximum_capacity_w);
        let snapshot =
            output.calculation_cooling_post_saturation_capacity_limit_dehumidification_guard;
        assert_eq!(
            (
                snapshot.unit_off_skipped,
                snapshot.non_cooling_skipped,
                snapshot.positive_guard_false_fallthrough_skipped,
            ),
            expected_base,
        );
        assert_no_guard_evidence(snapshot);
        let state =
            purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_guard_lifecycle_summary(
                &runtime,
                output.initialization.system,
            )
            .expect("CP381 skipped lifecycle")
            .state;
        assert_counter_shape(&state, 0, 0);
    }
}

fn assert_no_guard_evidence(
    snapshot: PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardSnapshot,
) {
    assert!(!snapshot.dehumidification_guard_evaluated);
    assert!(!snapshot.cp378_supply_humidity_ratio_saturation_limit_owned_read);
    assert!(!snapshot.cp379_same_call_supply_humidity_ratio_bit_corroborated);
    assert!(!snapshot.purchased_air_supply_humidity_ratio_read);
    assert!(snapshot.supply_humidity_ratio.is_none());
    assert!(!snapshot.cp329_mixed_air_humidity_ratio_owned_read);
    assert!(!snapshot.purchased_air_mixed_air_humidity_ratio_read);
    assert!(snapshot.mixed_air_humidity_ratio.is_none());
    assert!(!snapshot.supply_humidity_ratio_mixed_air_humidity_ratio_comparison_evaluated);
    assert!(
        snapshot
            .supply_humidity_ratio_strictly_less_than_mixed_air_humidity_ratio
            .is_none()
    );
    assert!(!snapshot.dehumidification_body_entered);
    assert!(!snapshot.dehumidification_guard_false_fallthrough);
}

fn assert_counter_shape(
    state: &PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardRuntimeState,
    evaluations: usize,
    body_entries: usize,
) {
    let false_fallthroughs = evaluations - body_entries;
    assert_eq!(state.dehumidification_guard_evaluation_count, evaluations);
    assert_eq!(
        state.source_site_execution_count,
        3 * evaluations + body_entries
    );
    assert_eq!(
        state.cp378_supply_humidity_ratio_saturation_limit_owned_read_count,
        evaluations,
    );
    assert_eq!(
        state.cp379_same_call_supply_humidity_ratio_bit_corroboration_count,
        evaluations,
    );
    assert_eq!(
        state.purchased_air_supply_humidity_ratio_read_count,
        evaluations
    );
    assert_eq!(
        state.cp329_mixed_air_humidity_ratio_owned_read_count,
        evaluations
    );
    assert_eq!(
        state.purchased_air_mixed_air_humidity_ratio_read_count,
        evaluations
    );
    assert_eq!(
        state.supply_humidity_ratio_mixed_air_humidity_ratio_comparison_count,
        evaluations,
    );
    assert_eq!(
        state.supply_humidity_ratio_strictly_less_than_mixed_air_humidity_ratio_count,
        body_entries,
    );
    assert_eq!(state.dehumidification_body_entry_count, body_entries);
    assert_eq!(
        state.dehumidification_guard_false_fallthrough_count,
        false_fallthroughs,
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_SOURCE_ORDER.len(),
        4,
    );
}
