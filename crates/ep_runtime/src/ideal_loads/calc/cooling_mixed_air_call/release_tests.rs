use crate::ideal_loads::{
    PurchasedAirCalcCoolingMixedAirCallError,
    PurchasedAirCalcCoolingMixedAirCallRecirculationInput,
    advance_direct_no_oa_calc_cooling_capacity_zero_flow_reset,
    advance_direct_no_oa_calc_cooling_dehumidification_flow,
    advance_direct_no_oa_calc_cooling_humidification_flow,
    advance_direct_no_oa_calc_cooling_mixed_air_call,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_body,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_guard,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_body,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_guard,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_maximum,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard,
    advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard_body,
    moist_air_enthalpy_j_per_kg,
};

#[derive(Clone, Copy)]
enum Corruption {
    Temperature(f64),
    HumidityRatio(f64),
    EnthalpyProjectionOverflow,
}

#[test]
fn active_nonfinite_recirculation_inputs_fail_before_any_cp329_mutation() {
    let (runtime, system, predecessor, zone_state) = release_case();
    let cases = [
        (
            Corruption::Temperature(f64::from_bits(0x7ff8_0000_0000_00a1)),
            PurchasedAirCalcCoolingMixedAirCallRecirculationInput::Temperature,
        ),
        (
            Corruption::Temperature(f64::INFINITY),
            PurchasedAirCalcCoolingMixedAirCallRecirculationInput::Temperature,
        ),
        (
            Corruption::Temperature(f64::NEG_INFINITY),
            PurchasedAirCalcCoolingMixedAirCallRecirculationInput::Temperature,
        ),
        (
            Corruption::HumidityRatio(f64::from_bits(0xfff8_0000_0000_00b2)),
            PurchasedAirCalcCoolingMixedAirCallRecirculationInput::HumidityRatio,
        ),
        (
            Corruption::HumidityRatio(f64::INFINITY),
            PurchasedAirCalcCoolingMixedAirCallRecirculationInput::HumidityRatio,
        ),
        (
            Corruption::HumidityRatio(f64::NEG_INFINITY),
            PurchasedAirCalcCoolingMixedAirCallRecirculationInput::HumidityRatio,
        ),
        (
            Corruption::EnthalpyProjectionOverflow,
            PurchasedAirCalcCoolingMixedAirCallRecirculationInput::EnthalpyProjection,
        ),
    ];

    for (corruption, expected_input) in cases {
        let mut corrupted_zone_state = zone_state.clone();
        match corruption {
            Corruption::Temperature(value) => {
                corrupted_zone_state.mean_air_temperature_c = value;
            }
            Corruption::HumidityRatio(value) => {
                corrupted_zone_state.air_humidity_ratio = value;
            }
            Corruption::EnthalpyProjectionOverflow => {
                corrupted_zone_state.mean_air_temperature_c = f64::MAX / 2.0;
                corrupted_zone_state.air_humidity_ratio = 0.0;
                assert!(corrupted_zone_state.mean_air_temperature_c.is_finite());
                assert!(corrupted_zone_state.air_humidity_ratio.is_finite());
                assert!(
                    !moist_air_enthalpy_j_per_kg(
                        corrupted_zone_state.mean_air_temperature_c,
                        corrupted_zone_state.air_humidity_ratio,
                    )
                    .is_finite()
                );
            }
        }

        let mut case_runtime = runtime.clone();
        let before = case_runtime.clone();
        assert_eq!(
            advance_direct_no_oa_calc_cooling_mixed_air_call(
                &mut case_runtime,
                &system,
                predecessor,
                &corrupted_zone_state,
            ),
            Err(
                PurchasedAirCalcCoolingMixedAirCallError::NonFiniteRecirculationState {
                    system: system.id,
                    input: expected_input,
                }
            )
        );
        assert_eq!(case_runtime, before);
    }
}

#[test]
fn core_counter_products_reject_overflow_instead_of_saturating() {
    assert!(!super::release::counter_product_matches(
        usize::MAX,
        usize::MAX,
        2
    ));
    assert!(super::release::counter_product_matches(18, 2, 9));
}

fn release_case() -> (
    crate::ideal_loads::PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    crate::ideal_loads::PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot,
    crate::heat_balance::state::ZoneHeatBalanceState,
) {
    let (mut runtime, system, sensible) =
        super::super::cooling_dehumidification_flow_release_tests::release_case(-1_000.0);
    let dehumidification =
        advance_direct_no_oa_calc_cooling_dehumidification_flow(&mut runtime, &system, sensible)
            .expect("CP319");
    let humidification = advance_direct_no_oa_calc_cooling_humidification_flow(
        &mut runtime,
        &system,
        dehumidification,
    )
    .expect("CP320");
    let reset = advance_direct_no_oa_calc_cooling_capacity_zero_flow_reset(
        &mut runtime,
        &system,
        humidification,
    )
    .expect("CP321");
    let maximum =
        advance_direct_no_oa_calc_cooling_supply_mass_flow_maximum(&mut runtime, &system, reset)
            .expect("CP322");
    let ems_guard = advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_guard(
        &mut runtime,
        &system,
        maximum,
    )
    .expect("CP323");
    let ems_body = advance_direct_no_oa_calc_cooling_supply_mass_flow_ems_override_body(
        &mut runtime,
        &system,
        ems_guard,
    )
    .expect("CP324");
    let limit_guard = advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_guard(
        &mut runtime,
        &system,
        ems_body,
    )
    .expect("CP325");
    let limit_body = advance_direct_no_oa_calc_cooling_supply_mass_flow_limit_body(
        &mut runtime,
        &system,
        limit_guard,
    )
    .expect("CP326");
    let very_small_guard = advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard(
        &mut runtime,
        &system,
        limit_body,
    )
    .expect("CP327");
    let very_small_body = advance_direct_no_oa_calc_cooling_supply_mass_flow_very_small_guard_body(
        &mut runtime,
        &system,
        very_small_guard,
    )
    .expect("CP328");
    let zone_state = super::super::cooling_sensible_flow_release_tests::zone_state(
        very_small_body.controlled_zone,
    );
    (runtime, system, very_small_body, zone_state)
}
