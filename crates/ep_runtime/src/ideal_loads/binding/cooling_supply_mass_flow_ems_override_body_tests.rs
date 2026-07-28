use super::*;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyRuntimeState,
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_body,
    purchased_air_calc_cooling_supply_mass_flow_ems_override_body_lifecycle_summary,
};

fn run_case(
    independent_load_w: f64,
    availability: f64,
) -> (
    PurchasedAirRuntimeState,
    DirectZonePurchasedAirScheduledCouplingOutput,
) {
    let (model, cache) = fixture(|typed| {
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
    .expect("source-ordered CP324 coupling");
    (runtime, output)
}

fn assert_all_body_sites_skipped(
    snapshot: PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot,
) {
    assert!(
        crate::ideal_loads::calc::
            cooling_supply_mass_flow_ems_override_body_snapshot_is_exact_direct_release(snapshot)
    );
    assert!(snapshot.body_skipped);
    assert!(!snapshot.ems_supply_mass_flow_override_value_read);
    assert_eq!(snapshot.ems_supply_mass_flow_override_value_kg_per_s, None);
    assert!(!snapshot.supply_mass_flow_rate_override_assignment_performed);
    assert_eq!(snapshot.assigned_supply_mass_flow_rate_kg_per_s, None);
    assert!(!snapshot.outdoor_air_mass_flow_rate_for_minimum_read);
    assert_eq!(
        snapshot.outdoor_air_mass_flow_rate_before_override_kg_per_s,
        None
    );
    assert!(!snapshot.supply_mass_flow_rate_for_minimum_read);
    assert_eq!(snapshot.supply_mass_flow_rate_for_minimum_kg_per_s, None);
    assert!(!snapshot.source_shaped_two_argument_minimum_evaluated);
    assert_eq!(snapshot.minimum_outdoor_air_mass_flow_rate_kg_per_s, None);
    assert!(!snapshot.outdoor_air_mass_flow_rate_assignment_performed);
    assert_eq!(snapshot.assigned_outdoor_air_mass_flow_rate_kg_per_s, None);
}

fn assert_all_body_site_counters_zero(
    state: &PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyRuntimeState,
) {
    assert_eq!(state.body_entry_count, 0);
    assert_eq!(state.ems_supply_mass_flow_override_value_read_count, 0);
    assert_eq!(state.supply_mass_flow_rate_override_assignment_count, 0);
    assert_eq!(state.outdoor_air_mass_flow_rate_for_minimum_read_count, 0);
    assert_eq!(state.supply_mass_flow_rate_for_minimum_read_count, 0);
    assert_eq!(state.source_shaped_two_argument_minimum_evaluation_count, 0);
    assert_eq!(state.outdoor_air_mass_flow_rate_assignment_count, 0);
}

#[test]
fn scheduled_binding_consumes_the_cp323_false_fallthrough_and_skips_the_cp324_body() {
    let (runtime, output) = run_case(3_000.0, 1.0);
    let predecessor = output.calculation_cooling_supply_mass_flow_ems_override_guard;
    let body = output.calculation_cooling_supply_mass_flow_ems_override_body;

    assert!(predecessor.cooling_body_entered);
    assert!(!predecessor.ems_supply_mass_flow_override_body_entered);
    assert!(predecessor.ems_supply_mass_flow_override_guard_false_fallthrough);
    assert_eq!(body.system, predecessor.system);
    assert_eq!(body.parent_call_ordinal, predecessor.parent_call_ordinal);
    assert_eq!(body.controlled_zone, predecessor.controlled_zone);
    assert_eq!(
        body.predecessor_ems_supply_mass_flow_override_body_entered,
        predecessor.ems_supply_mass_flow_override_body_entered
    );
    assert_eq!(
        body.predecessor_ems_supply_mass_flow_override_guard_false_fallthrough,
        predecessor.ems_supply_mass_flow_override_guard_false_fallthrough
    );
    assert!(body.cooling_body_entered);
    assert!(body.ems_disabled_fallthrough);
    assert_all_body_sites_skipped(body);

    let state = purchased_air_calc_cooling_supply_mass_flow_ems_override_body_lifecycle_summary(
        &runtime,
        output.initialization.system,
    )
    .expect("CP324 lifecycle")
    .state;
    assert_eq!(state.transition_count, 1);
    assert_eq!(state.cooling_body_entry_count, 1);
    assert_eq!(state.body_skip_count, 1);
    assert_eq!(state.unit_off_skip_count, 0);
    assert_eq!(state.non_cooling_skip_count, 0);
    assert_eq!(state.ems_disabled_fallthrough_count, 1);
    assert_all_body_site_counters_zero(&state);
}

#[test]
fn scheduled_binding_skips_all_cp324_sites_when_cooling_is_inactive() {
    for (availability, independent_load_w, unit_off, non_cooling) in
        [(0.0, 3_000.0, true, false), (1.0, 0.0, false, true)]
    {
        let (runtime, output) = run_case(independent_load_w, availability);
        let predecessor = output.calculation_cooling_supply_mass_flow_ems_override_guard;
        let body = output.calculation_cooling_supply_mass_flow_ems_override_body;

        assert_eq!(body.system, predecessor.system);
        assert_eq!(body.parent_call_ordinal, predecessor.parent_call_ordinal);
        assert_eq!(body.unit_off_skipped, unit_off);
        assert_eq!(body.non_cooling_skipped, non_cooling);
        assert!(!body.cooling_body_entered);
        assert!(!body.ems_disabled_fallthrough);
        assert_all_body_sites_skipped(body);

        let state =
            purchased_air_calc_cooling_supply_mass_flow_ems_override_body_lifecycle_summary(
                &runtime,
                output.initialization.system,
            )
            .expect("CP324 skip lifecycle")
            .state;
        assert_eq!(state.transition_count, 1);
        assert_eq!(state.cooling_body_entry_count, 0);
        assert_eq!(state.body_skip_count, 1);
        assert_eq!(state.unit_off_skip_count, usize::from(unit_off));
        assert_eq!(state.non_cooling_skip_count, usize::from(non_cooling));
        assert_eq!(state.ems_disabled_fallthrough_count, 0);
        assert_all_body_site_counters_zero(&state);
    }
}

#[test]
fn public_cp324_release_rejects_replay_and_forged_cp323_ordinal_without_mutation() {
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
    .expect("completed CP324 release");
    let predecessor = output.calculation_cooling_supply_mass_flow_ems_override_guard;

    let before_replay = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_body(
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
        advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_body(
            &mut runtime,
            binding.system,
            forged,
        )
        .is_err()
    );
    assert_eq!(runtime, before_forgery);
}
