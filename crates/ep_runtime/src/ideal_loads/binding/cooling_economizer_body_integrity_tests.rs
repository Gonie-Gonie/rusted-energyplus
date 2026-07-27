use super::*;

#[test]
fn public_body_rejects_retained_identity_and_route_forgery_without_mutation() {
    let (model, cache) = fixture(|_| {});
    let binding = bind_direct_zone_purchased_air_model(&model).expect("bounded model binding");

    let mut runtime = PurchasedAirRuntimeState::default();
    let mut zone = zone_state_for_temp_independent_load(3_000.0);
    let output = couple_model_bound_direct_zone_purchased_air(
        DirectZonePurchasedAirScheduledCouplingInput {
            binding: &binding,
            schedule_cache: &cache,
            schedule_sample_index: 0,
            zone_state: &mut zone,
            purchased_air_runtime_state: &mut runtime,
            begin_environment: true,
            barometric_pressure_pa: binding.limit_context.barometric_pressure_pa,
            system_timestep_seconds: binding.nominal_system_timestep_seconds,
        },
    )
    .expect("source-ordered CP317 call");

    runtime
        .units
        .get_mut(&binding.ideal_loads_air_system)
        .expect("selected unit")
        .calc_cooling_economizer_body =
        crate::ideal_loads::PurchasedAirCalcCoolingEconomizerBodyRuntimeState::new(
            binding.ideal_loads_air_system,
        );
    let before = runtime.clone();

    assert_eq!(
        crate::ideal_loads::advance_direct_no_oa_calc_cooling_economizer_body(
            &mut runtime,
            binding.system,
            output.calculation_cooling_economizer_condition,
        ),
        Err(
            crate::ideal_loads::PurchasedAirCalcCoolingEconomizerBodyError::
                RuntimeStateInvariantViolation {
                    system: binding.ideal_loads_air_system,
                }
        )
    );
    assert_eq!(runtime, before);
}
