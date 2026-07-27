use super::*;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingEconomizerGuardError, PurchasedAirCalcCoolingOaMaxFlowBodySnapshot,
    PurchasedAirRuntimeState, advance_direct_no_oa_calc_cooling_economizer_guard,
};
use ep_model::{IdealLoadsAirSystem, IdealLoadsAirSystemId};

#[test]
fn public_guard_rejects_retained_identity_and_route_forgery_without_mutation() {
    let (model, cache) = fixture(|_| {});
    let binding = bind_direct_zone_purchased_air_model(&model).expect("bounded model binding");
    let mut first_zone_state = zone_state_for_temp_independent_load(3_000.0);
    let mut runtime = PurchasedAirRuntimeState::default();
    let first = couple_model_bound_direct_zone_purchased_air(
        DirectZonePurchasedAirScheduledCouplingInput {
            binding: &binding,
            schedule_cache: &cache,
            schedule_sample_index: 0,
            zone_state: &mut first_zone_state,
            purchased_air_runtime_state: &mut runtime,
            begin_environment: true,
            barometric_pressure_pa: binding.limit_context.barometric_pressure_pa,
            system_timestep_seconds: binding.nominal_system_timestep_seconds,
        },
    )
    .expect("first source-ordered cooling coupling");
    assert!(
        first
            .calculation_cooling_economizer_guard
            .economizer_guard_evaluated
    );
    let first_guard_state = runtime
        .units
        .get(&binding.ideal_loads_air_system)
        .expect("selected unit")
        .calc_cooling_economizer_guard
        .clone();

    let mut second_zone_state = zone_state_for_temp_independent_load(3_000.0);
    let second = couple_model_bound_direct_zone_purchased_air(
        DirectZonePurchasedAirScheduledCouplingInput {
            binding: &binding,
            schedule_cache: &cache,
            schedule_sample_index: 1,
            zone_state: &mut second_zone_state,
            purchased_air_runtime_state: &mut runtime,
            begin_environment: false,
            barometric_pressure_pa: binding.limit_context.barometric_pressure_pa,
            system_timestep_seconds: binding.nominal_system_timestep_seconds,
        },
    )
    .expect("second source-ordered cooling coupling");
    assert!(
        second
            .calculation_cooling_economizer_guard
            .economizer_guard_evaluated
    );
    assert_eq!(
        second
            .calculation_cooling_oa_max_flow_body
            .parent_call_ordinal,
        2
    );
    runtime
        .units
        .get_mut(&binding.ideal_loads_air_system)
        .expect("selected unit")
        .calc_cooling_economizer_guard = first_guard_state;

    let predecessor = second.calculation_cooling_oa_max_flow_body;
    let mut valid_pending = runtime.clone();
    advance_direct_no_oa_calc_cooling_economizer_guard(
        &mut valid_pending,
        binding.system,
        predecessor,
    )
    .expect("otherwise valid pending CP315 transition");

    let wrong_system = IdealLoadsAirSystemId(binding.ideal_loads_air_system.0 + 1);
    let mut forged_latest_system = runtime.clone();
    forged_latest_system
        .units
        .get_mut(&binding.ideal_loads_air_system)
        .expect("selected unit")
        .calc_cooling_economizer_guard
        .latest
        .as_mut()
        .expect("retained latest")
        .system = wrong_system;
    assert_guard_invariant_rejected_without_mutation(
        binding.system,
        predecessor,
        binding.ideal_loads_air_system,
        forged_latest_system,
    );

    let mut forged_latest_route = runtime.clone();
    let latest = forged_latest_route
        .units
        .get_mut(&binding.ideal_loads_air_system)
        .expect("selected unit")
        .calc_cooling_economizer_guard
        .latest
        .as_mut()
        .expect("retained latest");
    latest.unit_body_entered = false;
    latest.predecessor_cooling_body_entered = false;
    latest.predecessor_maximum_cooling_flow_body_entered = false;
    latest.predecessor_active_guard_false_economizer_fallthrough = false;
    latest.unit_off_skipped = true;
    latest.non_cooling_skipped = false;
    latest.maximum_cooling_flow_body_sibling_skipped = false;
    latest.economizer_guard_evaluated = false;
    latest.economizer_type_read = false;
    latest.economizer_type = None;
    latest.no_economizer_comparison_evaluated = false;
    latest.economizer_not_no_economizer = None;
    latest.economizer_body_entered = false;
    latest.no_economizer_fallthrough = false;
    assert_guard_invariant_rejected_without_mutation(
        binding.system,
        predecessor,
        binding.ideal_loads_air_system,
        forged_latest_route,
    );

    for state_owner in 0..7 {
        let mut forged_identity = runtime.clone();
        let unit = forged_identity
            .units
            .get_mut(&binding.ideal_loads_air_system)
            .expect("selected unit");
        match state_owner {
            0 => unit.system = wrong_system,
            1 => unit.calc_entry.system = wrong_system,
            2 => unit.calc_minimum_oa_prefix.system = wrong_system,
            3 => unit.calc_cooling_entry_gate.system = wrong_system,
            4 => unit.calc_cooling_oa_max_flow_gate.system = wrong_system,
            5 => unit.calc_cooling_oa_max_flow_body.system = wrong_system,
            6 => unit.calc_cooling_economizer_guard.system = wrong_system,
            _ => unreachable!("bounded state-owner index"),
        }
        assert_guard_invariant_rejected_without_mutation(
            binding.system,
            predecessor,
            binding.ideal_loads_air_system,
            forged_identity,
        );
    }
}

fn assert_guard_invariant_rejected_without_mutation(
    system: &IdealLoadsAirSystem,
    predecessor: PurchasedAirCalcCoolingOaMaxFlowBodySnapshot,
    selected: IdealLoadsAirSystemId,
    mut runtime: PurchasedAirRuntimeState,
) {
    let before = runtime.clone();
    assert_eq!(
        advance_direct_no_oa_calc_cooling_economizer_guard(&mut runtime, system, predecessor,),
        Err(
            PurchasedAirCalcCoolingEconomizerGuardError::RuntimeStateInvariantViolation {
                system: selected,
            }
        )
    );
    assert_eq!(runtime, before);
}
