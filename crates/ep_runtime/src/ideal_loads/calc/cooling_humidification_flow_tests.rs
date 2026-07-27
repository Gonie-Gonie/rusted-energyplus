//! CP320 pure-transition tests.

use ep_model::{DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystemId};

use super::{
    PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_SOURCE_ORDER,
    PurchasedAirCalcCoolingHumidificationFlowInput,
    PurchasedAirCalcCoolingHumidificationFlowRuntimeState,
    advance_cooling_humidification_flow_state,
    advance_direct_no_oa_calc_cooling_dehumidification_flow,
};

const SYSTEM: IdealLoadsAirSystemId = IdealLoadsAirSystemId(5);

fn predecessor(
    cooling_demand_w: f64,
) -> super::PurchasedAirCalcCoolingDehumidificationFlowSnapshot {
    let (mut runtime, system, sensible) =
        super::cooling_dehumidification_flow_release_tests::release_case(cooling_demand_w);
    advance_direct_no_oa_calc_cooling_dehumidification_flow(&mut runtime, &system, sensible)
        .expect("exact CP319 predecessor")
}

fn input(
    dehumidification_control_type: DehumidificationControlType,
) -> PurchasedAirCalcCoolingHumidificationFlowInput {
    PurchasedAirCalcCoolingHumidificationFlowInput {
        heating_on: true,
        humidification_control_type: HumidificationControlType::Humidistat,
        dehumidification_control_type,
        zone_humidifying_setpoint_moisture_demand_kg_per_s: 0.0002,
        maximum_heating_supply_air_humidity_ratio_kg_water_per_kg_dry_air: 0.012,
        zone_humidity_ratio_kg_water_per_kg_dry_air: 0.009,
    }
}

fn run(
    predecessor: super::PurchasedAirCalcCoolingDehumidificationFlowSnapshot,
    input: PurchasedAirCalcCoolingHumidificationFlowInput,
) -> super::PurchasedAirCalcCoolingHumidificationFlowSnapshot {
    let mut state = PurchasedAirCalcCoolingHumidificationFlowRuntimeState::new(SYSTEM);
    advance_cooling_humidification_flow_state(&mut state, predecessor, input)
}

#[test]
fn source_boundary_and_all_twenty_six_sites_are_stable() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2133-2144"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2147"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_HUMIDIFICATION_FLOW_SOURCE_ORDER.len(),
        26
    );
}

#[test]
fn cp320_links_to_the_immediate_cp319_cooling_body() {
    let mut cp319 = predecessor(-1_000.0);
    cp319.predecessor_cooling_body_entered = false;
    cp319.cooling_body_entered = true;
    let snapshot = run(cp319, input(DehumidificationControlType::None));
    assert!(snapshot.predecessor_cooling_body_entered);
}

#[test]
fn humidistat_dehumidification_short_circuits_second_or_read() {
    let snapshot = run(
        predecessor(-1_000.0),
        input(DehumidificationControlType::Humidistat),
    );
    assert!(snapshot.dehumidification_control_type_first_read);
    assert_eq!(
        snapshot.dehumidification_control_type_humidistat,
        Some(true)
    );
    assert!(!snapshot.dehumidification_control_type_second_read);
    assert!(snapshot.humidification_control_condition_admitted);
    assert!(snapshot.humidification_flow_body_entered);
}

#[test]
fn none_dehumidification_requires_the_second_repeated_read() {
    let snapshot = run(
        predecessor(-1_000.0),
        input(DehumidificationControlType::None),
    );
    assert_eq!(
        snapshot.dehumidification_control_type_humidistat,
        Some(false)
    );
    assert!(snapshot.dehumidification_control_type_second_read);
    assert_eq!(
        snapshot.second_dehumidification_control_type,
        Some(DehumidificationControlType::None)
    );
    assert_eq!(snapshot.dehumidification_control_type_none, Some(true));
}

#[test]
fn non_humidistat_outer_control_skips_every_live_input() {
    let mut value = input(DehumidificationControlType::None);
    value.humidification_control_type = HumidificationControlType::None;
    value.zone_humidifying_setpoint_moisture_demand_kg_per_s = f64::NAN;
    let snapshot = run(predecessor(-1_000.0), value);
    assert!(!snapshot.dehumidification_control_type_first_read);
    assert!(!snapshot.zone_humidifying_setpoint_moisture_demand_read);
    assert_eq!(
        snapshot
            .resulting_supply_mass_flow_rate_for_humidification_kg_per_s
            .expect("reset")
            .to_bits(),
        0.0_f64.to_bits()
    );
}

#[test]
fn false_heating_availability_stops_before_control_reads() {
    let mut value = input(DehumidificationControlType::None);
    value.heating_on = false;
    value.zone_humidifying_setpoint_moisture_demand_kg_per_s = f64::NAN;
    let snapshot = run(predecessor(-1_000.0), value);
    assert_eq!(snapshot.heating_on, Some(false));
    assert!(!snapshot.humidification_control_type_read);
    assert!(!snapshot.zone_humidifying_setpoint_moisture_demand_read);
}

#[test]
fn other_dehumidification_control_repeats_read_then_rejects_body() {
    let snapshot = run(
        predecessor(-1_000.0),
        input(DehumidificationControlType::ConstantSupplyHumidityRatio),
    );
    assert!(snapshot.dehumidification_control_type_first_read);
    assert!(snapshot.dehumidification_control_type_second_read);
    assert_eq!(snapshot.dehumidification_control_type_none, Some(false));
    assert!(!snapshot.humidification_control_condition_admitted);
    assert!(!snapshot.zone_humidifying_setpoint_moisture_demand_read);
}

#[test]
fn strict_positive_delta_gate_short_circuits_demand_gate_at_equality() {
    let mut value = input(DehumidificationControlType::None);
    value.maximum_heating_supply_air_humidity_ratio_kg_water_per_kg_dry_air = 0.00025;
    value.zone_humidity_ratio_kg_water_per_kg_dry_air = 0.0;
    let snapshot = run(predecessor(-1_000.0), value);
    assert_eq!(snapshot.delta_humidity_ratio_above_small_delta, Some(false));
    assert!(!snapshot.zone_humidifying_setpoint_moisture_demand_for_gate_read);
}

#[test]
fn positive_zero_demand_falls_through_strict_second_gate() {
    let mut value = input(DehumidificationControlType::None);
    value.zone_humidifying_setpoint_moisture_demand_kg_per_s = 0.0;
    let snapshot = run(predecessor(-1_000.0), value);
    assert_eq!(
        snapshot.zone_humidifying_setpoint_moisture_demand_above_zero,
        Some(false)
    );
    assert!(!snapshot.humidification_flow_body_entered);
}

#[test]
fn raw_ieee_division_is_not_normalized() {
    let mut value = input(DehumidificationControlType::None);
    value.maximum_heating_supply_air_humidity_ratio_kg_water_per_kg_dry_air = f64::INFINITY;
    value.zone_humidity_ratio_kg_water_per_kg_dry_air = 0.0;
    let snapshot = run(predecessor(-1_000.0), value);
    assert_eq!(
        snapshot
            .calculated_supply_mass_flow_rate_for_humidification_kg_per_s
            .expect("division")
            .to_bits(),
        0.0_f64.to_bits()
    );
    value.zone_humidifying_setpoint_moisture_demand_kg_per_s = f64::INFINITY;
    let snapshot = run(predecessor(-1_000.0), value);
    assert!(
        snapshot
            .calculated_supply_mass_flow_rate_for_humidification_kg_per_s
            .is_some_and(f64::is_nan)
    );
}

#[test]
fn unit_off_and_non_cooling_skip_all_sites() {
    let non_cooling = predecessor(1.0);
    let mut cp319 = non_cooling;
    cp319.unit_body_entered = false;
    cp319.unit_off_skipped = true;
    cp319.non_cooling_skipped = false;
    let a = run(non_cooling, input(DehumidificationControlType::None));
    let b = run(cp319, input(DehumidificationControlType::None));
    assert!(a.non_cooling_skipped && !a.heating_on_read);
    assert!(b.unit_off_skipped && !b.heating_on_read);
}

#[path = "cooling_humidification_flow_release_tests.rs"]
mod release_tests;
