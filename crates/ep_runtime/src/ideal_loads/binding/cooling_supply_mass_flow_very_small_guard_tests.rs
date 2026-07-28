use super::*;
use crate::ideal_loads::{
    ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_KG_PER_S, ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_SOURCE,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardRuntimeState,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard,
    cooling_supply_mass_flow_very_small_guard_snapshot_is_exact_direct_release,
    purchased_air_calc_cooling_supply_mass_flow_very_small_guard_lifecycle_summary,
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
    .expect("source-ordered CP327 coupling");
    (runtime, output)
}

#[test]
fn scheduled_binding_consumes_the_retained_cp326_supply_bits_for_the_guard() {
    for (limit, capacity, expected_body_entry) in [
        (IdealLoadsLimit::NoLimit, None, false),
        (IdealLoadsLimit::LimitCapacity, Some(0.0), true),
    ] {
        let (runtime, output) = run_case(limit, capacity, 3_000.0, 1.0);
        let predecessor = output.calculation_cooling_supply_mass_flow_limit_body;
        let guard = output.calculation_cooling_supply_mass_flow_very_small_guard;

        assert!(cooling_supply_mass_flow_very_small_guard_snapshot_is_exact_direct_release(guard));
        assert_eq!(guard.system, predecessor.system);
        assert_eq!(guard.parent_call_ordinal, predecessor.parent_call_ordinal);
        assert_eq!(guard.controlled_zone, predecessor.controlled_zone);
        assert_eq!(
            guard.predecessor_supply_mass_flow_limit_body_entered,
            predecessor.supply_mass_flow_limit_body_entered
        );
        assert_eq!(
            guard.predecessor_supply_mass_flow_limit_body_skipped,
            predecessor.body_skipped
        );
        assert_eq!(
            guard.predecessor_supply_mass_flow_limit_active_guard_false_fallthrough,
            predecessor.active_guard_false_fallthrough
        );
        assert!(guard.cooling_body_entered);
        assert!(guard.supply_mass_flow_rate_read);
        assert_eq!(
            guard.supply_mass_flow_rate_kg_per_s.map(f64::to_bits),
            predecessor
                .resulting_supply_mass_flow_rate_kg_per_s
                .map(f64::to_bits)
        );
        assert!(guard.hvac_very_small_mass_flow_read);
        assert_eq!(
            guard.hvac_very_small_mass_flow_source,
            Some(ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_SOURCE)
        );
        assert_eq!(
            guard.hvac_very_small_mass_flow_kg_per_s.map(f64::to_bits),
            Some(ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_KG_PER_S.to_bits())
        );
        assert!(guard.supply_mass_flow_rate_at_or_below_very_small_mass_flow_comparison_evaluated);
        assert_eq!(
            guard.supply_mass_flow_rate_at_or_below_very_small_mass_flow,
            Some(expected_body_entry)
        );
        assert_eq!(guard.zero_flow_reset_body_entered, expected_body_entry);
        assert_eq!(guard.active_guard_false_fallthrough, !expected_body_entry);

        let state = purchased_air_calc_cooling_supply_mass_flow_very_small_guard_lifecycle_summary(
            &runtime,
            output.initialization.system,
        )
        .expect("CP327 lifecycle")
        .state;
        assert_counter_shape(&state, usize::from(expected_body_entry));
    }
}

fn assert_counter_shape(
    state: &PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardRuntimeState,
    body_entries: usize,
) {
    assert_eq!(state.transition_count, 1);
    assert_eq!(state.cooling_body_entry_count, 1);
    assert_eq!(state.unit_off_skip_count, 0);
    assert_eq!(state.non_cooling_skip_count, 0);
    assert_eq!(state.supply_mass_flow_rate_read_count, 1);
    assert_eq!(state.hvac_very_small_mass_flow_read_count, 1);
    assert_eq!(
        state.supply_mass_flow_rate_at_or_below_very_small_mass_flow_comparison_count,
        1
    );
    assert_eq!(state.zero_flow_reset_body_entry_count, body_entries);
    assert_eq!(state.active_guard_false_fallthrough_count, 1 - body_entries);
}

#[test]
fn scheduled_binding_skips_all_cp327_sites_when_cooling_is_inactive() {
    for (availability, independent_load_w, unit_off, non_cooling) in
        [(0.0, 3_000.0, true, false), (1.0, 0.0, false, true)]
    {
        let (runtime, output) = run_case(
            IdealLoadsLimit::NoLimit,
            None,
            independent_load_w,
            availability,
        );
        let guard = output.calculation_cooling_supply_mass_flow_very_small_guard;

        assert!(cooling_supply_mass_flow_very_small_guard_snapshot_is_exact_direct_release(guard));
        assert_eq!(guard.unit_off_skipped, unit_off);
        assert_eq!(guard.non_cooling_skipped, non_cooling);
        assert!(!guard.cooling_body_entered);
        assert!(!guard.supply_mass_flow_rate_read);
        assert_eq!(guard.supply_mass_flow_rate_kg_per_s, None);
        assert!(!guard.hvac_very_small_mass_flow_read);
        assert_eq!(guard.hvac_very_small_mass_flow_source, None);
        assert_eq!(guard.hvac_very_small_mass_flow_kg_per_s, None);
        assert!(!guard.supply_mass_flow_rate_at_or_below_very_small_mass_flow_comparison_evaluated);
        assert_eq!(
            guard.supply_mass_flow_rate_at_or_below_very_small_mass_flow,
            None
        );
        assert!(!guard.zero_flow_reset_body_entered);
        assert!(!guard.active_guard_false_fallthrough);

        let state = purchased_air_calc_cooling_supply_mass_flow_very_small_guard_lifecycle_summary(
            &runtime,
            output.initialization.system,
        )
        .expect("CP327 skip lifecycle")
        .state;
        assert_eq!(state.transition_count, 1);
        assert_eq!(state.cooling_body_entry_count, 0);
        assert_eq!(state.unit_off_skip_count, usize::from(unit_off));
        assert_eq!(state.non_cooling_skip_count, usize::from(non_cooling));
        assert_eq!(state.supply_mass_flow_rate_read_count, 0);
        assert_eq!(state.hvac_very_small_mass_flow_read_count, 0);
        assert_eq!(
            state.supply_mass_flow_rate_at_or_below_very_small_mass_flow_comparison_count,
            0
        );
        assert_eq!(state.zero_flow_reset_body_entry_count, 0);
        assert_eq!(state.active_guard_false_fallthrough_count, 0);
    }
}

#[test]
fn public_cp327_release_rejects_replay_and_forged_cp326_ordinal_without_mutation() {
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
    .expect("completed CP327 release");
    let predecessor = output.calculation_cooling_supply_mass_flow_limit_body;

    let before_replay = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard(
            &mut runtime,
            binding.system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before_replay);

    let mut forged = predecessor;
    forged.parent_call_ordinal += 1;
    let before_forgery = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard(
            &mut runtime,
            binding.system,
            forged,
        )
        .is_err()
    );
    assert_eq!(runtime, before_forgery);
}
