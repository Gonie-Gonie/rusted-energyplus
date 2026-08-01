use super::super::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardError as Error,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_heating_availability_guard,
    cooling_supply_humidity_ratio_humidification_heating_availability_guard_snapshot_is_exact_direct_release,
    private_cooling_supply_humidity_ratio_humidification_heating_availability_guard_csh_counterfactual_from_direct_release,
    private_cooling_supply_humidity_ratio_humidification_heating_availability_guard_csh_counterfactual_links_to_direct_release,
};
use crate::ideal_loads::calc::cooling_constant_shr_supply_humidity_ratio_mixed_air_limit::completed_cp355_case;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingDefaultSupplyHumidityRatioCaseBreakSnapshot, PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_constant_shr_case_break,
    advance_direct_no_oa_calc_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit,
    advance_direct_no_oa_calc_cooling_constant_supply_humidity_ratio_assignment,
    advance_direct_no_oa_calc_cooling_constant_supply_humidity_ratio_case_break,
    advance_direct_no_oa_calc_cooling_constant_supply_humidity_ratio_case_entry,
    advance_direct_no_oa_calc_cooling_default_supply_humidity_ratio_case_break,
    advance_direct_no_oa_calc_cooling_default_supply_humidity_ratio_mixed_air_assignment,
    advance_direct_no_oa_calc_cooling_humidistat_case_break,
    advance_direct_no_oa_calc_cooling_humidistat_case_entry,
    advance_direct_no_oa_calc_cooling_humidistat_moisture_demand_assignment,
    advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment,
    advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit,
    advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_mixed_air_limit,
};

#[test]
fn direct_release_and_private_csh_reconstruction_enter_the_true_body() {
    let case = completed_cp368_case();
    assert!(case.is_some(), "CP368 fixture must complete");
    let Some((mut runtime, system, predecessor)) = case else {
        return;
    };
    let direct =
        advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_heating_availability_guard(
            &mut runtime,
            &system,
            predecessor,
        )
        .expect("CP369 direct release");
    assert!(
        cooling_supply_humidity_ratio_humidification_heating_availability_guard_snapshot_is_exact_direct_release(
            direct,
        )
    );
    assert!(direct.heating_on_read);
    assert_eq!(direct.heating_on, Some(true));
    assert!(direct.cooling_supply_humidity_ratio_humidification_body_entered);
    assert!(!direct.heating_on_guard_false_fallthrough);
    assert_eq!(direct.source_order.len(), 2);

    let unit = runtime.units.get(&system.id).expect("unit");
    let private =
        private_cooling_supply_humidity_ratio_humidification_heating_availability_guard_csh_counterfactual_from_direct_release(
            &runtime, unit, &system, direct,
        )
        .expect("canonical private CSH reconstruction");
    assert!(
        private.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip
    );
    assert!(private.heating_on_read);
    assert_eq!(private.heating_on, Some(true));
    assert!(private.cooling_supply_humidity_ratio_humidification_body_entered);
    assert!(!private.heating_on_guard_false_fallthrough);
    assert_eq!(private.source_order.len(), 2);
    assert!(
        private_cooling_supply_humidity_ratio_humidification_heating_availability_guard_csh_counterfactual_links_to_direct_release(
            &runtime, unit, &system, direct, private,
        )
    );
}

#[test]
fn cp320_signed_zero_witness_corruption_rejects_transactionally() {
    let case = completed_cp368_case();
    assert!(case.is_some(), "CP368 fixture must complete");
    let Some((mut runtime, system, predecessor)) = case else {
        return;
    };
    let retained = runtime
        .units
        .get(&system.id)
        .and_then(|unit| unit.calc_cooling_humidification_flow.latest)
        .expect("retained CP320");
    assert_eq!(
        retained
            .reset_supply_mass_flow_rate_for_humidification_kg_per_s
            .expect("CP320 reset")
            .to_bits(),
        0.0_f64.to_bits()
    );
    let mut witness = runtime
        .cooling_humidification_flow_latest_witness(system.id)
        .expect("CP320 witness");
    witness.reset_supply_mass_flow_rate_for_humidification_kg_per_s = Some(-0.0);
    assert_eq!(retained, witness, "derived equality hides signed zero");
    assert_ne!(
        retained
            .reset_supply_mass_flow_rate_for_humidification_kg_per_s
            .expect("retained reset")
            .to_bits(),
        witness
            .reset_supply_mass_flow_rate_for_humidification_kg_per_s
            .expect("witness reset")
            .to_bits()
    );
    runtime.set_cooling_humidification_flow_latest_witness(system.id, witness);

    let before = runtime.clone();
    let result =
        advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_heating_availability_guard(
            &mut runtime,
            &system,
            predecessor,
        );
    assert!(matches!(
        result,
        Err(Error::HeatingOnProvenanceMismatch { .. })
    ));
    assert_eq!(runtime, before);
}

#[test]
fn calc_entry_and_cp320_heating_on_mismatch_rejects_transactionally() {
    let case = completed_cp368_case();
    assert!(case.is_some(), "CP368 fixture must complete");
    let Some((mut runtime, system, predecessor)) = case else {
        return;
    };
    let mut cp320 = runtime
        .units
        .get(&system.id)
        .and_then(|unit| unit.calc_cooling_humidification_flow.latest)
        .expect("retained CP320");
    assert!(
        runtime
            .units
            .get(&system.id)
            .and_then(|unit| unit.calc_entry.latest)
            .is_some_and(|entry| entry.heating_on)
    );
    cp320.heating_on = Some(false);
    runtime
        .units
        .get_mut(&system.id)
        .expect("unit")
        .calc_cooling_humidification_flow
        .latest = Some(cp320);
    runtime.set_cooling_humidification_flow_latest_witness(system.id, cp320);

    let before = runtime.clone();
    let result =
        advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_heating_availability_guard(
            &mut runtime,
            &system,
            predecessor,
        );
    assert!(matches!(
        result,
        Err(Error::HeatingOnProvenanceMismatch { .. })
    ));
    assert_eq!(runtime, before);
}

#[test]
fn corrupted_source_counter_rejects_without_state_or_witness_mutation() {
    let case = completed_cp368_case();
    assert!(case.is_some(), "CP368 fixture must complete");
    let Some((mut runtime, system, predecessor)) = case else {
        return;
    };
    runtime
        .units
        .get_mut(&system.id)
        .expect("unit")
        .calc_cooling_supply_humidity_ratio_humidification_heating_availability_guard
        .source_site_execution_count = usize::MAX;

    let before = runtime.clone();
    let result =
        advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_heating_availability_guard(
            &mut runtime,
            &system,
            predecessor,
        );
    assert!(matches!(
        result,
        Err(Error::RuntimeStateInvariantViolation { .. })
    ));
    assert_eq!(runtime, before);
}

fn completed_cp368_case() -> Option<(
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    PurchasedAirCalcCoolingDefaultSupplyHumidityRatioCaseBreakSnapshot,
)> {
    let (mut runtime, mut system, cp355) = completed_cp355_case(-1_000.0, 1.0, false)?;
    system.minimum_cooling_supply_air_humidity_ratio = 0.0077;
    let cp356 =
        advance_direct_no_oa_calc_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit(
            &mut runtime,
            &system,
            cp355,
        )
        .ok()?;
    let cp357 =
        advance_direct_no_oa_calc_cooling_constant_shr_case_break(&mut runtime, &system, cp356)
            .ok()?;
    let cp358 =
        advance_direct_no_oa_calc_cooling_humidistat_case_entry(&mut runtime, &system, cp357)
            .ok()?;
    let cp359 = advance_direct_no_oa_calc_cooling_humidistat_moisture_demand_assignment(
        &mut runtime,
        &system,
        cp358,
    )
    .ok()?;
    let cp360 =
        advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment(
            &mut runtime,
            &system,
            cp359,
        )
        .ok()?;
    let cp361 =
        advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit(
            &mut runtime,
            &system,
            cp360,
        )
        .ok()?;
    let cp362 = advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_mixed_air_limit(
        &mut runtime,
        &system,
        cp361,
    )
    .ok()?;
    let cp363 =
        advance_direct_no_oa_calc_cooling_humidistat_case_break(&mut runtime, &system, cp362)
            .ok()?;
    let cp364 = advance_direct_no_oa_calc_cooling_constant_supply_humidity_ratio_case_entry(
        &mut runtime,
        &system,
        cp363,
    )
    .ok()?;
    let cp365 = advance_direct_no_oa_calc_cooling_constant_supply_humidity_ratio_assignment(
        &mut runtime,
        &system,
        cp364,
    )
    .ok()?;
    let cp366 = advance_direct_no_oa_calc_cooling_constant_supply_humidity_ratio_case_break(
        &mut runtime,
        &system,
        cp365,
    )
    .ok()?;
    let cp367 =
        advance_direct_no_oa_calc_cooling_default_supply_humidity_ratio_mixed_air_assignment(
            &mut runtime,
            &system,
            cp366,
        )
        .ok()?;
    let cp368 = advance_direct_no_oa_calc_cooling_default_supply_humidity_ratio_case_break(
        &mut runtime,
        &system,
        cp367,
    )
    .ok()?;
    Some((runtime, system, cp368))
}