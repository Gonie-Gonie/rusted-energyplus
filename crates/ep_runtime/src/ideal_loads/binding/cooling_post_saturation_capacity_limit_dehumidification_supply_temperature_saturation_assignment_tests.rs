use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationAssignmentError as Cp414Error,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationAssignmentRuntimeState as Cp414State,
    advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment as advance_cp414,
};
use ep_model::{
    AutosizeOrNumber, DehumidificationControlType, HumidificationControlType, IdealLoadsLimit,
};

#[test]
fn cp414_binding_contract_is_source_ordered_after_cp413() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_ASSIGNMENT_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2316",
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2319",
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_ASSIGNMENT_SOURCE_ORDER.len(),
        4,
    );
}

#[test]
fn public_release_accepts_finite_active_input_and_rejects_nonpositive_pressure_transactionally() {
    let (runtime, output) = active_fixture();
    let predecessor = output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard;
    assert!(predecessor.saturation_supply_humidity_ratio_guard_body_entered);
    let mut accepted_runtime = runtime.clone();
    reset_cp414(&mut accepted_runtime, predecessor.system);
    let system = matching_system();
    let snapshot = advance_cp414(&mut accepted_runtime, &system, predecessor, 97_321.0)
        .expect("finite positive CP414 release");
    assert!(
        snapshot
            .resulting_supply_temperature_c
            .is_some_and(f64::is_finite)
    );

    let mut runtime = runtime;
    reset_cp414(&mut runtime, predecessor.system);
    let before = runtime
        .units
        .get(&predecessor.system)
        .expect("CP414 unit")
        .calc_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment
        .clone();
    assert_eq!(
        advance_cp414(&mut runtime, &system, predecessor, -1.0),
        Err(Cp414Error::BarometricPressureOutsideDirectSubset {
            system: predecessor.system,
            bits: (-1.0f64).to_bits(),
        }),
    );
    assert_eq!(
        runtime
            .units
            .get(&predecessor.system)
            .expect("CP414 unit")
            .calc_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment,
        before,
    );
}

#[test]
fn public_release_rejects_nonfinite_enthalpy_before_mutating_cp414_state() {
    let (mut runtime, output) = active_fixture();
    let mut predecessor = output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard;
    assert!(predecessor.saturation_supply_humidity_ratio_guard_body_entered);
    let enthalpy_nan = f64::from_bits(0x7ff8_0000_0000_4144);
    set_retained_enthalpy(&mut predecessor, enthalpy_nan);
    let unit = runtime
        .units
        .get_mut(&predecessor.system)
        .expect("CP414 unit");
    let retained = unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard
        .latest
        .as_mut()
        .expect("CP413 latest");
    set_retained_enthalpy(retained, enthalpy_nan);
    runtime.set_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_latest_witness(
        predecessor.system,
        predecessor,
    );
    reset_cp414(&mut runtime, predecessor.system);
    let before = runtime
        .units
        .get(&predecessor.system)
        .expect("CP414 unit")
        .calc_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment
        .clone();
    assert_eq!(
        advance_cp414(&mut runtime, &matching_system(), predecessor, 97_321.0),
        Err(Cp414Error::SupplyEnthalpyOutsideDirectSubset {
            system: predecessor.system,
            bits: enthalpy_nan.to_bits(),
        }),
    );
    assert_eq!(
        runtime
            .units
            .get(&predecessor.system)
            .expect("CP414 unit")
            .calc_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment,
        before,
    );
}

#[test]
fn public_release_rejects_forged_cp413_hidden_witness_transactionally() {
    let (mut runtime, output) = active_fixture();
    let predecessor = output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard;
    reset_cp414(&mut runtime, predecessor.system);
    let unit = runtime.units.get(&predecessor.system).expect("CP413 unit");
    assert!(crate::ideal_loads::cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_latest_metadata_is_consistent(
        unit,
        unit.init_call_count,
    ));
    assert!(runtime
        .cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_latest_witness(predecessor.system)
        .is_some());
    let mut forged_witness = predecessor;
    forged_witness.source_order = &["forged-hidden-cp413-witness"];
    runtime.set_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_latest_witness(
        predecessor.system,
        forged_witness,
    );
    let before = runtime
        .units
        .get(&predecessor.system)
        .expect("CP414 unit")
        .calc_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment
        .clone();

    assert_eq!(
        advance_cp414(&mut runtime, &matching_system(), predecessor, 97_321.0,),
        Err(Cp414Error::RuntimeStateInvariantViolation {
            system: predecessor.system,
        }),
    );
    assert_eq!(
        runtime
            .units
            .get(&predecessor.system)
            .expect("CP414 unit")
            .calc_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment,
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

fn reset_cp414(
    runtime: &mut crate::ideal_loads::PurchasedAirRuntimeState,
    system: ep_model::IdealLoadsAirSystemId,
) {
    runtime
        .units
        .get_mut(&system)
        .expect("CP414 unit")
        .calc_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment = Cp414State::new(system);
    runtime.clear_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_latest_witness_for_test(system);
}

fn set_retained_enthalpy(
    snapshot: &mut crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationGuardSnapshot,
    enthalpy: f64,
) {
    let enthalpy = Some(enthalpy);
    snapshot.predecessor_cp409_resulting_supply_enthalpy_j_per_kg = enthalpy;
    snapshot.predecessor_cp410_resulting_supply_enthalpy_j_per_kg = enthalpy;
    snapshot.predecessor_cp411_resulting_supply_enthalpy_j_per_kg = enthalpy;
    snapshot.predecessor_cp412_resulting_supply_enthalpy_j_per_kg = enthalpy;
    snapshot.resulting_supply_enthalpy_j_per_kg = enthalpy;
}
