use super::*;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyMassFlowLimitGuardRuntimeState,
    PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_guard,
    purchased_air_calc_cooling_supply_mass_flow_limit_guard_lifecycle_summary,
};

fn run_case(
    cooling_limit: IdealLoadsLimit,
    maximum_flow_m3_per_s: Option<f64>,
    independent_load_w: f64,
    availability: f64,
) -> (
    PurchasedAirRuntimeState,
    DirectZonePurchasedAirScheduledCouplingOutput,
) {
    let (model, cache) = fixture(|typed| {
        let system = &mut typed.ideal_loads_air_systems[0];
        system.cooling_limit = cooling_limit;
        system.maximum_cooling_air_flow_rate_m3_per_s =
            maximum_flow_m3_per_s.map(AutosizeOrNumber::Value);
        if matches!(
            cooling_limit,
            IdealLoadsLimit::LimitCapacity | IdealLoadsLimit::LimitFlowRateAndCapacity
        ) {
            system.maximum_total_cooling_capacity_w = Some(AutosizeOrNumber::Value(900.0));
        }
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
    .expect("source-ordered CP325 coupling");
    (runtime, output)
}

fn assert_source_shape(
    snapshot: PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot,
    cooling_limit: IdealLoadsLimit,
    maximum_mass_flow_rate_kg_per_s: f64,
    cooling: bool,
) {
    if !cooling {
        assert!(!snapshot.first_cooling_limit_read);
        assert_eq!(snapshot.first_cooling_limit, None);
        assert!(!snapshot.cooling_limit_flow_rate_comparison_evaluated);
        assert_eq!(snapshot.cooling_limit_flow_rate, None);
        assert!(!snapshot.second_cooling_limit_read);
        assert_eq!(snapshot.second_cooling_limit, None);
        assert!(!snapshot.cooling_limit_flow_rate_and_capacity_comparison_evaluated);
        assert_eq!(snapshot.cooling_limit_flow_rate_and_capacity, None);
        assert_eq!(snapshot.cooling_limit_condition_satisfied, None);
        assert!(!snapshot.maximum_cooling_air_mass_flow_rate_read);
        assert_eq!(snapshot.maximum_cooling_air_mass_flow_rate_kg_per_s, None);
        assert!(!snapshot.maximum_cooling_air_mass_flow_rate_positive_comparison_evaluated);
        assert_eq!(
            snapshot.maximum_cooling_air_mass_flow_rate_strictly_positive,
            None
        );
        assert!(!snapshot.supply_mass_flow_limit_body_entered);
        assert!(!snapshot.active_guard_false_fallthrough);
        return;
    }

    let flow_rate = cooling_limit == IdealLoadsLimit::LimitFlowRate;
    let flow_rate_and_capacity = cooling_limit == IdealLoadsLimit::LimitFlowRateAndCapacity;
    let selected = flow_rate || flow_rate_and_capacity;
    let positive = selected && maximum_mass_flow_rate_kg_per_s > 0.0;
    assert!(snapshot.first_cooling_limit_read);
    assert_eq!(snapshot.first_cooling_limit, Some(cooling_limit));
    assert!(snapshot.cooling_limit_flow_rate_comparison_evaluated);
    assert_eq!(snapshot.cooling_limit_flow_rate, Some(flow_rate));
    assert_eq!(snapshot.second_cooling_limit_read, !flow_rate);
    assert_eq!(
        snapshot.second_cooling_limit,
        (!flow_rate).then_some(cooling_limit)
    );
    assert_eq!(
        snapshot.cooling_limit_flow_rate_and_capacity_comparison_evaluated,
        !flow_rate
    );
    assert_eq!(
        snapshot.cooling_limit_flow_rate_and_capacity,
        (!flow_rate).then_some(flow_rate_and_capacity)
    );
    assert_eq!(snapshot.cooling_limit_condition_satisfied, Some(selected));
    assert_eq!(snapshot.maximum_cooling_air_mass_flow_rate_read, selected);
    if selected {
        assert_eq!(
            snapshot
                .maximum_cooling_air_mass_flow_rate_kg_per_s
                .map(f64::to_bits),
            Some(maximum_mass_flow_rate_kg_per_s.to_bits())
        );
    } else {
        assert_eq!(snapshot.maximum_cooling_air_mass_flow_rate_kg_per_s, None);
    }
    assert_eq!(
        snapshot.maximum_cooling_air_mass_flow_rate_positive_comparison_evaluated,
        selected
    );
    assert_eq!(
        snapshot.maximum_cooling_air_mass_flow_rate_strictly_positive,
        selected.then_some(positive)
    );
    assert_eq!(snapshot.supply_mass_flow_limit_body_entered, positive);
    assert_eq!(snapshot.active_guard_false_fallthrough, !positive);
}

fn assert_counter_shape(
    state: &PurchasedAirCalcCoolingSupplyMassFlowLimitGuardRuntimeState,
    cooling_limit: IdealLoadsLimit,
    maximum_mass_flow_rate_kg_per_s: f64,
) {
    let flow_rate = usize::from(cooling_limit == IdealLoadsLimit::LimitFlowRate);
    let combined = usize::from(cooling_limit == IdealLoadsLimit::LimitFlowRateAndCapacity);
    let selected = flow_rate + combined;
    let positive = usize::from(selected == 1 && maximum_mass_flow_rate_kg_per_s > 0.0);
    assert_eq!(state.transition_count, 1);
    assert_eq!(state.cooling_body_entry_count, 1);
    assert_eq!(state.first_cooling_limit_read_count, 1);
    assert_eq!(state.cooling_limit_flow_rate_comparison_count, 1);
    assert_eq!(state.cooling_limit_flow_rate_match_count, flow_rate);
    assert_eq!(state.second_cooling_limit_read_count, 1 - flow_rate);
    assert_eq!(
        state.cooling_limit_flow_rate_and_capacity_comparison_count,
        1 - flow_rate
    );
    assert_eq!(
        state.cooling_limit_flow_rate_and_capacity_match_count,
        combined
    );
    assert_eq!(state.cooling_limit_rejected_count, 1 - selected);
    assert_eq!(
        state.maximum_cooling_air_mass_flow_rate_read_count,
        selected
    );
    assert_eq!(
        state.maximum_cooling_air_mass_flow_rate_positive_comparison_count,
        selected
    );
    assert_eq!(
        state.maximum_cooling_air_mass_flow_rate_strictly_positive_count,
        positive
    );
    assert_eq!(
        state.maximum_cooling_air_mass_flow_rate_not_positive_count,
        selected - positive
    );
    assert_eq!(state.supply_mass_flow_limit_body_entry_count, positive);
    assert_eq!(state.active_guard_false_fallthrough_count, 1 - positive);
}

#[test]
fn scheduled_binding_preserves_both_selector_reads_and_strict_positive_guard() {
    for (limit, maximum_flow_m3_per_s) in [
        (IdealLoadsLimit::NoLimit, None),
        (IdealLoadsLimit::LimitCapacity, None),
        (IdealLoadsLimit::LimitFlowRate, Some(0.20)),
        (IdealLoadsLimit::LimitFlowRateAndCapacity, Some(0.20)),
        (IdealLoadsLimit::LimitFlowRate, Some(0.0)),
        (IdealLoadsLimit::LimitFlowRate, Some(-0.0)),
    ] {
        let (runtime, output) = run_case(limit, maximum_flow_m3_per_s, 3_000.0, 1.0);
        let predecessor = output.calculation_cooling_supply_mass_flow_ems_override_body;
        let snapshot = output.calculation_cooling_supply_mass_flow_limit_guard;
        let maximum = output
            .initialization
            .maximum_cooling_air_mass_flow_rate_kg_per_s;

        assert!(predecessor.ems_disabled_fallthrough);
        assert_eq!(snapshot.system, predecessor.system);
        assert_eq!(
            snapshot.parent_call_ordinal,
            predecessor.parent_call_ordinal
        );
        assert_eq!(snapshot.controlled_zone, predecessor.controlled_zone);
        assert!(snapshot.predecessor_ems_supply_mass_flow_override_body_skipped);
        assert!(snapshot.predecessor_ems_disabled_fallthrough);
        assert_source_shape(snapshot, limit, maximum, true);

        let state = purchased_air_calc_cooling_supply_mass_flow_limit_guard_lifecycle_summary(
            &runtime,
            output.initialization.system,
        )
        .expect("CP325 lifecycle")
        .state;
        assert_counter_shape(&state, limit, maximum);
    }
}

#[test]
fn scheduled_binding_skips_all_cp325_sites_when_cooling_is_inactive() {
    for (availability, independent_load_w, unit_off, non_cooling) in
        [(0.0, 3_000.0, true, false), (1.0, 0.0, false, true)]
    {
        let (runtime, output) = run_case(
            IdealLoadsLimit::LimitFlowRate,
            Some(0.20),
            independent_load_w,
            availability,
        );
        let snapshot = output.calculation_cooling_supply_mass_flow_limit_guard;
        assert_eq!(snapshot.unit_off_skipped, unit_off);
        assert_eq!(snapshot.non_cooling_skipped, non_cooling);
        assert_source_shape(
            snapshot,
            IdealLoadsLimit::LimitFlowRate,
            output
                .initialization
                .maximum_cooling_air_mass_flow_rate_kg_per_s,
            false,
        );
        let state = purchased_air_calc_cooling_supply_mass_flow_limit_guard_lifecycle_summary(
            &runtime,
            output.initialization.system,
        )
        .expect("CP325 skip lifecycle")
        .state;
        assert_eq!(state.transition_count, 1);
        assert_eq!(state.cooling_body_entry_count, 0);
        assert_eq!(state.unit_off_skip_count, usize::from(unit_off));
        assert_eq!(state.non_cooling_skip_count, usize::from(non_cooling));
        assert_eq!(state.first_cooling_limit_read_count, 0);
        assert_eq!(state.supply_mass_flow_limit_body_entry_count, 0);
        assert_eq!(state.active_guard_false_fallthrough_count, 0);
    }
}

#[test]
fn public_cp325_release_rejects_replay_and_forged_cp324_ordinal_without_mutation() {
    let (model, cache) = fixture(|typed| {
        let system = &mut typed.ideal_loads_air_systems[0];
        system.cooling_limit = IdealLoadsLimit::LimitFlowRate;
        system.maximum_cooling_air_flow_rate_m3_per_s = Some(AutosizeOrNumber::Value(0.20));
    });
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
    .expect("completed CP325 release");
    let predecessor = output.calculation_cooling_supply_mass_flow_ems_override_body;

    let before_replay = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_guard(
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
        advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_guard(
            &mut runtime,
            binding.system,
            forged,
        )
        .is_err()
    );
    assert_eq!(runtime, before_forgery);
}
