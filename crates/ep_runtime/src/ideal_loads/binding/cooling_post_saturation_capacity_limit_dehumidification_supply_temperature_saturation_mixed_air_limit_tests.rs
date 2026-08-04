use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_MIXED_AIR_LIMIT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_MIXED_AIR_LIMIT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationMixedAirLimitRuntimeState as Cp415State,
    advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit as advance_cp415,
    cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_snapshot_is_exact_direct_release,
};
use ep_model::{
    AutosizeOrNumber, DehumidificationControlType, HumidificationControlType, IdealLoadsLimit,
};

#[test]
fn cp415_binding_contract_is_source_ordered_after_cp414() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_MIXED_AIR_LIMIT_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2319",
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_MIXED_AIR_LIMIT_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2320",
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_MIXED_AIR_LIMIT_SOURCE_ORDER,
        [
            "read-purchased-air-supply-temperature-for-minimum",
            "read-purchased-air-mixed-air-temperature-for-minimum",
            "apply-source-shaped-two-argument-minimum",
            "assign-purchased-air-supply-temperature",
        ],
    );
}

#[test]
fn public_release_applies_the_source_shaped_minimum() {
    let (mut runtime, output) = active_fixture();
    let predecessor = output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment;
    let mixed_air = output.calculation_cooling_mixed_air_call;
    assert!(
        predecessor
            .post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_executed
    );
    reset_cp415(&mut runtime, predecessor.system);

    let snapshot = advance_cp415(&mut runtime, &matching_system(), predecessor)
        .expect("finite active CP415 release");
    assert!(
        cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_snapshot_is_exact_direct_release(snapshot)
    );
    let left = predecessor
        .resulting_supply_temperature_c
        .expect("CP414 supply temperature");
    let right = mixed_air
        .mixed_air_temperature_c
        .expect("CP329 mixed-air temperature");
    let expected = if left < right { left } else { right };
    assert_eq!(
        snapshot.resulting_supply_temperature_c.map(f64::to_bits),
        Some(expected.to_bits()),
    );
    assert_eq!(
        snapshot.minimum_supply_temperature_c.map(f64::to_bits),
        Some(expected.to_bits()),
    );
}

#[test]
fn public_release_rejects_forged_cp414_hidden_witness_transactionally() {
    let (mut runtime, output) = active_fixture();
    let predecessor = output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment;
    reset_cp415(&mut runtime, predecessor.system);
    let mut forged = predecessor;
    forged.source_order = &["forged-hidden-cp414-witness"];
    runtime.set_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_latest_witness(
        predecessor.system,
        forged,
    );
    let before = runtime
        .units
        .get(&predecessor.system)
        .expect("CP415 unit")
        .calc_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit
        .clone();

    assert!(advance_cp415(&mut runtime, &matching_system(), predecessor).is_err());
    assert_eq!(
        runtime
            .units
            .get(&predecessor.system)
            .expect("CP415 unit")
            .calc_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit,
        before,
    );
}

#[test]
fn public_release_rejects_forged_cp329_hidden_owner_transactionally() {
    let (mut runtime, output) = active_fixture();
    let predecessor = output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment;
    reset_cp415(&mut runtime, predecessor.system);
    let mut forged_owner = output.calculation_cooling_mixed_air_call;
    forged_owner.source_order = &["forged-hidden-cp329-owner"];
    runtime.set_cooling_mixed_air_call_latest_witness(predecessor.system, forged_owner);
    let before = runtime
        .units
        .get(&predecessor.system)
        .expect("CP415 unit")
        .calc_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit
        .clone();

    assert!(advance_cp415(&mut runtime, &matching_system(), predecessor).is_err());
    assert_eq!(
        runtime
            .units
            .get(&predecessor.system)
            .expect("CP415 unit")
            .calc_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit,
        before,
    );
}

fn active_fixture() -> (
    crate::ideal_loads::PurchasedAirRuntimeState,
    crate::ideal_loads::DirectZonePurchasedAirScheduledCouplingOutput,
) {
    super::cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_tests::run_case(
        IdealLoadsLimit::LimitCapacity,
        0.020,
        1.0,
        1.0e-100,
    )
}

fn matching_system() -> ep_model::IdealLoadsAirSystem {
    let mut system = super::ideal_loads_system();
    system.cooling_limit = IdealLoadsLimit::LimitCapacity;
    system.maximum_cooling_air_flow_rate_m3_per_s = None;
    system.maximum_total_cooling_capacity_w = Some(AutosizeOrNumber::Value(1.0e-100));
    system.dehumidification_control_type = DehumidificationControlType::None;
    system.humidification_control_type = HumidificationControlType::None;
    system.minimum_cooling_supply_air_humidity_ratio = f64::NAN;
    system
}

fn reset_cp415(
    runtime: &mut crate::ideal_loads::PurchasedAirRuntimeState,
    system: ep_model::IdealLoadsAirSystemId,
) {
    runtime
        .units
        .get_mut(&system)
        .expect("CP415 unit")
        .calc_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit = Cp415State::new(system);
    runtime.clear_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_latest_witness_for_test(system);
}
