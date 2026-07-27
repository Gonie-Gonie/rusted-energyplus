use super::*;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingEconomizerConditionError,
    PurchasedAirCalcCoolingEconomizerGuardSnapshot, PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_economizer_condition,
};
use ep_model::{IdealLoadsAirSystem, IdealLoadsAirSystemId};

#[test]
fn public_condition_rejects_retained_identity_and_route_forgery_without_mutation() {
    let (model, cache) = fixture(|_| {});
    let binding = bind_direct_zone_purchased_air_model(&model).expect("bounded model binding");
    let mut first_zone_state = zone_state_for_temp_independent_load(3_000.0);
    let mut runtime = PurchasedAirRuntimeState::default();
    couple_model_bound_direct_zone_purchased_air(DirectZonePurchasedAirScheduledCouplingInput {
        binding: &binding,
        schedule_cache: &cache,
        schedule_sample_index: 0,
        zone_state: &mut first_zone_state,
        purchased_air_runtime_state: &mut runtime,
        begin_environment: true,
        barometric_pressure_pa: binding.limit_context.barometric_pressure_pa,
        system_timestep_seconds: binding.nominal_system_timestep_seconds,
    })
    .expect("first source-ordered cooling coupling");
    let first_condition_state = runtime
        .units
        .get(&binding.ideal_loads_air_system)
        .expect("selected unit")
        .calc_cooling_economizer_condition
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
    runtime
        .units
        .get_mut(&binding.ideal_loads_air_system)
        .expect("selected unit")
        .calc_cooling_economizer_condition = first_condition_state;
    let predecessor = second.calculation_cooling_economizer_guard;

    assert_condition_invariant_rejected_without_mutation(
        binding.system,
        predecessor,
        binding.ideal_loads_air_system,
        runtime.clone(),
    );

    let mut forged_latest_route = runtime.clone();
    let latest = forged_latest_route
        .units
        .get_mut(&binding.ideal_loads_air_system)
        .expect("selected unit")
        .calc_cooling_economizer_condition
        .latest
        .as_mut()
        .expect("retained latest");
    latest.no_economizer_outer_guard_fallthrough_skipped = false;
    latest.economizer_condition_evaluated = true;
    assert_condition_invariant_rejected_without_mutation(
        binding.system,
        predecessor,
        binding.ideal_loads_air_system,
        forged_latest_route,
    );

    let mut non_cooling_zone_state = zone_state_for_temp_independent_load(0.0);
    let mut non_cooling_runtime = PurchasedAirRuntimeState::default();
    let non_cooling = couple_model_bound_direct_zone_purchased_air(
        DirectZonePurchasedAirScheduledCouplingInput {
            binding: &binding,
            schedule_cache: &cache,
            schedule_sample_index: 0,
            zone_state: &mut non_cooling_zone_state,
            purchased_air_runtime_state: &mut non_cooling_runtime,
            begin_environment: true,
            barometric_pressure_pa: binding.limit_context.barometric_pressure_pa,
            system_timestep_seconds: binding.nominal_system_timestep_seconds,
        },
    )
    .expect("exact non-cooling CP316 snapshot");
    assert!(
        non_cooling
            .calculation_cooling_economizer_condition
            .non_cooling_skipped
    );
    let mut unrecorded_latest_route = runtime.clone();
    unrecorded_latest_route
        .units
        .get_mut(&binding.ideal_loads_air_system)
        .expect("selected unit")
        .calc_cooling_economizer_condition
        .latest = Some(non_cooling.calculation_cooling_economizer_condition);
    assert_eq!(
        unrecorded_latest_route
            .units
            .get(&binding.ideal_loads_air_system)
            .expect("selected unit")
            .calc_cooling_economizer_condition
            .non_cooling_skip_count,
        0,
        "the exact-valid latest non-cooling route is deliberately absent from retained history"
    );
    assert_condition_invariant_rejected_without_mutation(
        binding.system,
        predecessor,
        binding.ideal_loads_air_system,
        unrecorded_latest_route,
    );

    let wrong_system = IdealLoadsAirSystemId(binding.ideal_loads_air_system.0 + 1);
    for state_owner in 0..8 {
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
            7 => unit.calc_cooling_economizer_condition.system = wrong_system,
            _ => unreachable!("bounded state-owner index"),
        }
        assert_condition_invariant_rejected_without_mutation(
            binding.system,
            predecessor,
            binding.ideal_loads_air_system,
            forged_identity,
        );
    }
}

fn assert_condition_invariant_rejected_without_mutation(
    system: &IdealLoadsAirSystem,
    predecessor: PurchasedAirCalcCoolingEconomizerGuardSnapshot,
    selected: IdealLoadsAirSystemId,
    mut runtime: PurchasedAirRuntimeState,
) {
    let before = runtime.clone();
    assert_eq!(
        advance_direct_no_oa_calc_cooling_economizer_condition(&mut runtime, system, predecessor,),
        Err(
            PurchasedAirCalcCoolingEconomizerConditionError::RuntimeStateInvariantViolation {
                system: selected,
            }
        )
    );
    assert_eq!(runtime, before);
}
