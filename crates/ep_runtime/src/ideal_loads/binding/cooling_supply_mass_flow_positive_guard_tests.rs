use super::*;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardRuntimeState,
    cooling_supply_mass_flow_positive_guard_snapshot_is_exact_direct_release,
    purchased_air_calc_cooling_supply_mass_flow_positive_guard_lifecycle_summary,
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
        system.maximum_total_cooling_capacity_w = maximum_capacity_w.map(AutosizeOrNumber::Value);
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
    .expect("source-ordered CP330 coupling");
    (runtime, output)
}

#[test]
fn scheduled_binding_enters_cp330_body_only_for_strictly_positive_supply() {
    for (limit, capacity, expected_positive) in [
        (IdealLoadsLimit::NoLimit, None, true),
        (IdealLoadsLimit::LimitCapacity, Some(0.0), false),
    ] {
        let (runtime, output) = run_case(limit, capacity, 3_000.0, 1.0);
        let predecessor = output.calculation_cooling_mixed_air_call;
        let guard = output.calculation_cooling_supply_mass_flow_positive_guard;

        assert!(cooling_supply_mass_flow_positive_guard_snapshot_is_exact_direct_release(guard));
        assert_eq!(guard.system, predecessor.system);
        assert_eq!(guard.parent_call_ordinal, predecessor.parent_call_ordinal);
        assert_eq!(guard.controlled_zone, predecessor.controlled_zone);
        assert!(guard.predecessor_cooling_call_executed);
        assert!(guard.predecessor_no_outdoor_air_fallback_entered);
        assert!(guard.cooling_body_entered);
        assert!(guard.supply_mass_flow_rate_read);
        assert_eq!(
            guard.supply_mass_flow_rate_kg_per_s.map(f64::to_bits),
            predecessor.supply_mass_flow_rate_kg_per_s.map(f64::to_bits)
        );
        assert_eq!(
            guard.supply_mass_flow_rate_kg_per_s.map(f64::to_bits),
            predecessor
                .resulting_recirculation_mass_flow_rate_kg_per_s
                .map(f64::to_bits)
        );
        assert_eq!(
            guard.supply_mass_flow_rate_strictly_positive,
            Some(expected_positive)
        );
        assert_eq!(
            guard.positive_supply_mass_flow_body_entered,
            expected_positive
        );
        assert_eq!(guard.active_guard_false_fallthrough, !expected_positive);

        let state = purchased_air_calc_cooling_supply_mass_flow_positive_guard_lifecycle_summary(
            &runtime,
            output.initialization.system,
        )
        .expect("CP330 lifecycle")
        .state;
        assert_active_counter_shape(&state, expected_positive);
    }
}

fn assert_active_counter_shape(
    state: &PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardRuntimeState,
    positive: bool,
) {
    assert_eq!(state.transition_count, 1);
    assert_eq!(state.cooling_body_entry_count, 1);
    assert_eq!(state.unit_off_skip_count, 0);
    assert_eq!(state.non_cooling_skip_count, 0);
    assert_eq!(state.source_site_execution_count, 2 + usize::from(positive));
    assert_eq!(state.supply_mass_flow_rate_read_count, 1);
    assert_eq!(
        state.supply_mass_flow_rate_strictly_positive_comparison_count,
        1
    );
    assert_eq!(
        state.positive_supply_mass_flow_body_entry_count,
        usize::from(positive)
    );
    assert_eq!(
        state.active_guard_false_fallthrough_count,
        usize::from(!positive)
    );
}

#[test]
fn scheduled_binding_skips_every_cp330_site_when_cooling_is_inactive() {
    for (availability, independent_load_w, unit_off, non_cooling) in
        [(0.0, 3_000.0, true, false), (1.0, 0.0, false, true)]
    {
        let (runtime, output) = run_case(
            IdealLoadsLimit::NoLimit,
            None,
            independent_load_w,
            availability,
        );
        let guard = output.calculation_cooling_supply_mass_flow_positive_guard;

        assert!(cooling_supply_mass_flow_positive_guard_snapshot_is_exact_direct_release(guard));
        assert_eq!(guard.unit_off_skipped, unit_off);
        assert_eq!(guard.non_cooling_skipped, non_cooling);
        assert!(!guard.cooling_body_entered);
        assert!(!guard.supply_mass_flow_rate_read);
        assert!(guard.supply_mass_flow_rate_kg_per_s.is_none());
        assert!(!guard.supply_mass_flow_rate_strictly_positive_comparison_evaluated);
        assert!(guard.supply_mass_flow_rate_strictly_positive.is_none());
        assert!(!guard.positive_supply_mass_flow_body_entered);
        assert!(!guard.active_guard_false_fallthrough);

        let state = purchased_air_calc_cooling_supply_mass_flow_positive_guard_lifecycle_summary(
            &runtime,
            output.initialization.system,
        )
        .expect("CP330 skip lifecycle")
        .state;
        assert_eq!(state.transition_count, 1);
        assert_eq!(state.cooling_body_entry_count, 0);
        assert_eq!(state.unit_off_skip_count, usize::from(unit_off));
        assert_eq!(state.non_cooling_skip_count, usize::from(non_cooling));
        assert_eq!(state.source_site_execution_count, 0);
        assert_eq!(state.supply_mass_flow_rate_read_count, 0);
        assert_eq!(
            state.supply_mass_flow_rate_strictly_positive_comparison_count,
            0
        );
    }
}
