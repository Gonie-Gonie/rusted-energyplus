use super::*;
use crate::ideal_loads::calc::cooling_capacity_zero_flow_reset_snapshot_is_exact_direct_release;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingCapacityZeroFlowResetRuntimeState,
    advance_direct_no_oa_calc_cooling_capacity_zero_flow_reset,
    purchased_air_calc_cooling_capacity_zero_flow_reset_lifecycle_summary,
};

fn run_case(
    limit: IdealLoadsLimit,
    capacity: Option<f64>,
    independent_load_w: f64,
    availability: f64,
) -> (
    PurchasedAirRuntimeState,
    DirectZonePurchasedAirScheduledCouplingOutput,
) {
    let (model, cache) = fixture(|typed| {
        let system = &mut typed.ideal_loads_air_systems[0];
        system.cooling_limit = limit;
        system.maximum_cooling_air_flow_rate_m3_per_s = matches!(
            limit,
            IdealLoadsLimit::LimitFlowRate | IdealLoadsLimit::LimitFlowRateAndCapacity
        )
        .then_some(AutosizeOrNumber::Value(0.25));
        system.maximum_total_cooling_capacity_w = capacity.map(AutosizeOrNumber::Value);
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
    .expect("source-ordered CP321 coupling");
    (runtime, output)
}

#[test]
fn scheduled_binding_covers_all_limit_routes_and_zero_or_positive_capacity() {
    for (limit, capacity, capacity_read, zeroed) in [
        (IdealLoadsLimit::NoLimit, None, false, false),
        (IdealLoadsLimit::LimitFlowRate, None, false, false),
        (IdealLoadsLimit::LimitCapacity, Some(0.0), true, true),
        (IdealLoadsLimit::LimitCapacity, Some(900.0), true, false),
        (
            IdealLoadsLimit::LimitFlowRateAndCapacity,
            Some(0.0),
            true,
            true,
        ),
        (
            IdealLoadsLimit::LimitFlowRateAndCapacity,
            Some(900.0),
            true,
            false,
        ),
    ] {
        let (runtime, output) = run_case(limit, capacity, 3_000.0, 1.0);
        let reset = output.calculation_cooling_capacity_zero_flow_reset;
        assert!(cooling_capacity_zero_flow_reset_snapshot_is_exact_direct_release(reset));
        assert_eq!(
            reset.parent_call_ordinal,
            output.calculation_entry.call_ordinal
        );
        assert_eq!(
            reset.predecessor_cooling_body_entered,
            output
                .calculation_cooling_humidification_flow
                .cooling_body_entered
        );
        assert!(reset.cooling_body_entered);
        assert_eq!(reset.first_cooling_limit, Some(limit));
        assert_eq!(reset.maximum_total_cooling_capacity_read, capacity_read);
        assert_eq!(reset.zero_cooling_capacity_body_entered, zeroed);

        for (predecessor, result) in [
            (
                output
                    .calculation_cooling_sensible_flow
                    .resulting_supply_mass_flow_rate_for_cool_kg_per_s,
                reset.resulting_supply_mass_flow_rate_for_cool_kg_per_s,
            ),
            (
                output
                    .calculation_cooling_dehumidification_flow
                    .resulting_supply_mass_flow_rate_for_dehumidification_kg_per_s,
                reset.resulting_supply_mass_flow_rate_for_dehumidification_kg_per_s,
            ),
            (
                output
                    .calculation_cooling_humidification_flow
                    .resulting_supply_mass_flow_rate_for_humidification_kg_per_s,
                reset.resulting_supply_mass_flow_rate_for_humidification_kg_per_s,
            ),
        ] {
            let expected = if zeroed { Some(0.0) } else { predecessor };
            assert_option_bits_eq(result, expected);
        }

        let lifecycle = purchased_air_calc_cooling_capacity_zero_flow_reset_lifecycle_summary(
            &runtime,
            output.initialization.system,
        )
        .expect("CP321 lifecycle");
        assert_eq!(lifecycle.state.transition_count, 1);
        assert_eq!(
            lifecycle.state.maximum_total_cooling_capacity_read_count,
            usize::from(capacity_read)
        );
        assert_eq!(
            lifecycle.state.zero_cooling_capacity_body_entry_count,
            usize::from(zeroed)
        );
    }
}

#[test]
fn scheduled_binding_records_unit_off_and_non_cooling_skips() {
    for (availability, independent_load_w, unit_off, non_cooling) in
        [(0.0, 3_000.0, true, false), (1.0, 0.0, false, true)]
    {
        let (runtime, output) = run_case(
            IdealLoadsLimit::LimitCapacity,
            Some(0.0),
            independent_load_w,
            availability,
        );
        let reset = output.calculation_cooling_capacity_zero_flow_reset;
        assert!(cooling_capacity_zero_flow_reset_snapshot_is_exact_direct_release(reset));
        assert_eq!(reset.unit_off_skipped, unit_off);
        assert_eq!(reset.non_cooling_skipped, non_cooling);
        assert!(!reset.cooling_body_entered);
        assert!(!reset.first_cooling_limit_read);
        assert!(!reset.maximum_total_cooling_capacity_read);
        assert_eq!(
            reset.resulting_supply_mass_flow_rate_for_cool_kg_per_s,
            None
        );

        let state = purchased_air_calc_cooling_capacity_zero_flow_reset_lifecycle_summary(
            &runtime,
            output.initialization.system,
        )
        .expect("CP321 skip lifecycle")
        .state;
        assert_eq!(state.transition_count, 1);
        assert_eq!(state.unit_off_skip_count, usize::from(unit_off));
        assert_eq!(state.non_cooling_skip_count, usize::from(non_cooling));
        assert_eq!(state.cooling_body_entry_count, 0);
    }
}

#[test]
fn public_release_replay_and_corrupt_state_fail_without_mutation() {
    let (model, cache) = fixture(|_| {});
    let binding = bind_direct_zone_purchased_air_model(&model).expect("bounded model binding");
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
    .expect("source-ordered CP321 call");

    let predecessor = output.calculation_cooling_humidification_flow;
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_capacity_zero_flow_reset(
            &mut runtime,
            binding.system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);

    let state = &mut runtime
        .units
        .get_mut(&binding.ideal_loads_air_system)
        .expect("selected unit")
        .calc_cooling_capacity_zero_flow_reset;
    *state = PurchasedAirCalcCoolingCapacityZeroFlowResetRuntimeState::new(
        binding.ideal_loads_air_system,
    );
    state.transition_count = usize::MAX;
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_capacity_zero_flow_reset(
            &mut runtime,
            binding.system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}

fn assert_option_bits_eq(actual: Option<f64>, expected: Option<f64>) {
    assert_eq!(
        actual.map(f64::to_bits),
        expected.map(f64::to_bits),
        "candidate bits"
    );
}
