use super::*;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyMassFlowLimitBodyRuntimeState,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_body,
    purchased_air_calc_cooling_supply_mass_flow_limit_body_lifecycle_summary,
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
    .expect("source-ordered CP326 coupling");
    (runtime, output)
}

#[test]
fn scheduled_binding_applies_line_2163_only_after_true_cp325_body_entry() {
    for (limit, maximum_flow_m3_per_s) in [
        (IdealLoadsLimit::NoLimit, None),
        (IdealLoadsLimit::LimitCapacity, None),
        (IdealLoadsLimit::LimitFlowRate, Some(0.20)),
        (IdealLoadsLimit::LimitFlowRateAndCapacity, Some(0.20)),
        (IdealLoadsLimit::LimitFlowRate, Some(0.0)),
        (IdealLoadsLimit::LimitFlowRate, Some(-0.0)),
    ] {
        let (runtime, output) = run_case(limit, maximum_flow_m3_per_s, 3_000.0, 1.0);
        let predecessor = output.calculation_cooling_supply_mass_flow_limit_guard;
        let snapshot = output.calculation_cooling_supply_mass_flow_limit_body;
        let source_supply = output
            .calculation_cooling_supply_mass_flow_maximum
            .resulting_supply_mass_flow_rate_kg_per_s
            .expect("active CP322 supply flow");
        let maximum = output
            .initialization
            .maximum_cooling_air_mass_flow_rate_kg_per_s;
        let body_entered = predecessor.supply_mass_flow_limit_body_entered;

        assert_eq!(snapshot.system, predecessor.system);
        assert_eq!(
            snapshot.parent_call_ordinal,
            predecessor.parent_call_ordinal
        );
        assert_eq!(snapshot.controlled_zone, predecessor.controlled_zone);
        assert_eq!(snapshot.supply_mass_flow_limit_body_entered, body_entered);
        assert_eq!(snapshot.body_skipped, !body_entered);
        assert_eq!(
            snapshot.active_guard_false_fallthrough,
            predecessor.active_guard_false_fallthrough
        );
        assert_eq!(
            snapshot.supply_mass_flow_rate_for_minimum_read,
            body_entered
        );
        assert_eq!(
            snapshot.maximum_cooling_air_mass_flow_rate_for_minimum_read,
            body_entered
        );
        assert_eq!(
            snapshot.source_shaped_two_argument_minimum_evaluated,
            body_entered
        );
        assert_eq!(
            snapshot.supply_mass_flow_rate_assignment_performed,
            body_entered
        );

        let expected = if body_entered {
            assert_option_bits(
                snapshot.supply_mass_flow_rate_before_limit_kg_per_s,
                source_supply,
            );
            assert_option_bits(
                snapshot.maximum_cooling_air_mass_flow_rate_kg_per_s,
                maximum,
            );
            if source_supply < maximum {
                source_supply
            } else {
                maximum
            }
        } else {
            assert_eq!(snapshot.supply_mass_flow_rate_before_limit_kg_per_s, None);
            assert_eq!(snapshot.maximum_cooling_air_mass_flow_rate_kg_per_s, None);
            source_supply
        };
        if body_entered {
            assert_option_bits(snapshot.minimum_supply_mass_flow_rate_kg_per_s, expected);
            assert_option_bits(snapshot.assigned_supply_mass_flow_rate_kg_per_s, expected);
        } else {
            assert_eq!(snapshot.minimum_supply_mass_flow_rate_kg_per_s, None);
            assert_eq!(snapshot.assigned_supply_mass_flow_rate_kg_per_s, None);
        }
        assert_option_bits(snapshot.resulting_supply_mass_flow_rate_kg_per_s, expected);

        let state = purchased_air_calc_cooling_supply_mass_flow_limit_body_lifecycle_summary(
            &runtime,
            output.initialization.system,
        )
        .expect("CP326 lifecycle")
        .state;
        assert_counter_shape(&state, usize::from(body_entered));
    }
}

fn assert_counter_shape(
    state: &PurchasedAirCalcCoolingSupplyMassFlowLimitBodyRuntimeState,
    body_entries: usize,
) {
    assert_eq!(state.transition_count, 1);
    assert_eq!(state.cooling_body_entry_count, 1);
    assert_eq!(state.supply_mass_flow_limit_body_entry_count, body_entries);
    assert_eq!(state.body_skip_count, 1 - body_entries);
    assert_eq!(state.active_guard_false_fallthrough_count, 1 - body_entries);
    assert_eq!(
        state.supply_mass_flow_rate_for_minimum_read_count,
        body_entries
    );
    assert_eq!(
        state.maximum_cooling_air_mass_flow_rate_for_minimum_read_count,
        body_entries
    );
    assert_eq!(
        state.source_shaped_two_argument_minimum_evaluation_count,
        body_entries
    );
    assert_eq!(state.supply_mass_flow_rate_assignment_count, body_entries);
}

#[test]
fn scheduled_binding_skips_all_cp326_sites_when_cooling_is_inactive() {
    for (availability, independent_load_w, unit_off, non_cooling) in
        [(0.0, 3_000.0, true, false), (1.0, 0.0, false, true)]
    {
        let (runtime, output) = run_case(
            IdealLoadsLimit::LimitFlowRate,
            Some(0.20),
            independent_load_w,
            availability,
        );
        let snapshot = output.calculation_cooling_supply_mass_flow_limit_body;
        assert_eq!(snapshot.unit_off_skipped, unit_off);
        assert_eq!(snapshot.non_cooling_skipped, non_cooling);
        assert!(!snapshot.cooling_body_entered);
        assert!(!snapshot.supply_mass_flow_limit_body_entered);
        assert!(snapshot.body_skipped);
        assert!(!snapshot.active_guard_false_fallthrough);
        assert!(!snapshot.supply_mass_flow_rate_for_minimum_read);
        assert_eq!(snapshot.supply_mass_flow_rate_before_limit_kg_per_s, None);
        assert!(!snapshot.maximum_cooling_air_mass_flow_rate_for_minimum_read);
        assert_eq!(snapshot.maximum_cooling_air_mass_flow_rate_kg_per_s, None);
        assert!(!snapshot.source_shaped_two_argument_minimum_evaluated);
        assert_eq!(snapshot.minimum_supply_mass_flow_rate_kg_per_s, None);
        assert!(!snapshot.supply_mass_flow_rate_assignment_performed);
        assert_eq!(snapshot.assigned_supply_mass_flow_rate_kg_per_s, None);
        assert_eq!(snapshot.resulting_supply_mass_flow_rate_kg_per_s, None);

        let state = purchased_air_calc_cooling_supply_mass_flow_limit_body_lifecycle_summary(
            &runtime,
            output.initialization.system,
        )
        .expect("CP326 skip lifecycle")
        .state;
        assert_eq!(state.transition_count, 1);
        assert_eq!(state.cooling_body_entry_count, 0);
        assert_eq!(state.unit_off_skip_count, usize::from(unit_off));
        assert_eq!(state.non_cooling_skip_count, usize::from(non_cooling));
        assert_eq!(state.supply_mass_flow_limit_body_entry_count, 0);
        assert_eq!(state.body_skip_count, 1);
        assert_eq!(state.active_guard_false_fallthrough_count, 0);
        assert_eq!(state.supply_mass_flow_rate_assignment_count, 0);
    }
}

#[test]
fn public_cp326_release_rejects_replay_and_forged_cp325_ordinal_without_mutation() {
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
    .expect("completed CP326 release");
    let predecessor = output.calculation_cooling_supply_mass_flow_limit_guard;

    let before_replay = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_body(
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
        advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_body(
            &mut runtime,
            binding.system,
            forged,
        )
        .is_err()
    );
    assert_eq!(runtime, before_forgery);
}

fn assert_option_bits(actual: Option<f64>, expected: f64) {
    assert_eq!(actual.map(f64::to_bits), Some(expected.to_bits()));
}
