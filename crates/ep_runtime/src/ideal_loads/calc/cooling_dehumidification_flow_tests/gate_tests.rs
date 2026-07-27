use ep_model::DehumidificationControlType;

use super::{active_predecessor, assert_bits, base_input, characterize, poison_input};

#[test]
fn false_cooling_availability_only_executes_reset_and_cooling_on_read() {
    let (snapshot, state) = characterize(
        active_predecessor(),
        poison_input(DehumidificationControlType::Humidistat),
    );
    assert!(snapshot.supply_mass_flow_rate_for_dehumidification_reset_assigned);
    assert_bits(
        snapshot.reset_supply_mass_flow_rate_for_dehumidification_kg_per_s,
        0.0,
    );
    assert_eq!(snapshot.cooling_on, Some(false));
    assert!(!snapshot.dehumidification_control_type_read);
    assert!(!snapshot.zone_dehumidifying_setpoint_moisture_demand_read);
    assert_bits(
        snapshot.resulting_supply_mass_flow_rate_for_dehumidification_kg_per_s,
        0.0,
    );
    assert_eq!(state.cooling_on_fallthrough_count, 1);
}

#[test]
fn non_humidistat_control_skips_all_live_humidity_inputs() {
    let mut input = poison_input(DehumidificationControlType::None);
    input.cooling_on = true;
    let (snapshot, state) = characterize(active_predecessor(), input);
    assert!(snapshot.dehumidification_control_type_read);
    assert_eq!(
        snapshot.dehumidification_control_type,
        Some(DehumidificationControlType::None)
    );
    assert_eq!(
        snapshot.dehumidification_control_type_humidistat,
        Some(false)
    );
    assert!(!snapshot.dehumidification_control_body_entered);
    assert!(!snapshot.zone_dehumidifying_setpoint_moisture_demand_read);
    assert!(!snapshot.minimum_cooling_supply_air_humidity_ratio_read);
    assert!(!snapshot.zone_humidity_ratio_read);
    assert_eq!(state.dehumidification_control_type_fallthrough_count, 1);
}

#[test]
fn exact_negative_delta_threshold_short_circuits_second_gate_read() {
    let mut input = base_input();
    input.minimum_cooling_supply_air_humidity_ratio_kg_water_per_kg_dry_air = -0.00025;
    input.zone_humidity_ratio_kg_water_per_kg_dry_air = 0.0;
    let (snapshot, state) = characterize(active_predecessor(), input);
    assert_eq!(
        snapshot.delta_humidity_ratio_below_negative_small_delta,
        Some(false)
    );
    assert!(snapshot.zone_dehumidifying_setpoint_moisture_demand_read);
    assert!(!snapshot.zone_dehumidifying_setpoint_moisture_demand_for_gate_read);
    assert!(!snapshot.zone_dehumidifying_setpoint_moisture_demand_comparison_evaluated);
    assert!(!snapshot.dehumidification_flow_body_entered);
    assert_eq!(state.delta_humidity_ratio_fallthrough_count, 1);
}

#[test]
fn negative_zero_moisture_demand_falls_through_second_strict_gate() {
    let mut input = base_input();
    input.zone_dehumidifying_setpoint_moisture_demand_kg_per_s = -0.0;
    let (snapshot, state) = characterize(active_predecessor(), input);
    assert_eq!(
        snapshot.delta_humidity_ratio_below_negative_small_delta,
        Some(true)
    );
    assert_eq!(
        snapshot.zone_dehumidifying_setpoint_moisture_demand_below_zero,
        Some(false)
    );
    assert!(!snapshot.dehumidification_flow_body_entered);
    assert_bits(
        snapshot.resulting_supply_mass_flow_rate_for_dehumidification_kg_per_s,
        0.0,
    );
    assert_eq!(
        state.zone_dehumidifying_setpoint_moisture_demand_fallthrough_count,
        1
    );
}

#[test]
fn raw_ieee_division_is_not_clamped_or_reassociated() {
    let mut input = base_input();
    input.zone_dehumidifying_setpoint_moisture_demand_kg_per_s = -1.0;
    input.minimum_cooling_supply_air_humidity_ratio_kg_water_per_kg_dry_air = f64::NEG_INFINITY;
    input.zone_humidity_ratio_kg_water_per_kg_dry_air = 0.0;
    let (snapshot, _) = characterize(active_predecessor(), input);
    assert!(snapshot.dehumidification_flow_body_entered);
    assert_bits(
        snapshot.calculated_supply_mass_flow_rate_for_dehumidification_kg_per_s,
        0.0,
    );

    input.zone_dehumidifying_setpoint_moisture_demand_kg_per_s = f64::NEG_INFINITY;
    let (snapshot, _) = characterize(active_predecessor(), input);
    assert!(
        snapshot
            .calculated_supply_mass_flow_rate_for_dehumidification_kg_per_s
            .is_some_and(f64::is_nan)
    );
}
