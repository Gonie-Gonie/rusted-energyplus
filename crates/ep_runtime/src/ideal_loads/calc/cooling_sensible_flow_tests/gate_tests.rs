use super::{active_predecessor, assert_bits, base_input, characterize};

#[test]
fn strict_small_temperature_gate_falls_through_at_exact_negative_threshold() {
    let mut input = base_input();
    input.minimum_cooling_supply_air_temperature_c = -1.0e-5;
    input.zone_temperature_c = 0.0;
    let (snapshot, state) = characterize(active_predecessor(), input);

    assert_eq!(
        snapshot.delta_temperature_below_negative_small_temp_diff,
        Some(false)
    );
    assert!(!snapshot.delta_temperature_body_entered);
    assert!(!snapshot.zone_cooling_setpoint_load_read);
    assert!(!snapshot.supply_mass_flow_rate_for_cool_calculated);
    assert_bits(
        snapshot.resulting_supply_mass_flow_rate_for_cool_kg_per_s,
        0.0,
    );
    assert_eq!(state.delta_temperature_fallthrough_count, 1);
    assert_eq!(state.supply_mass_flow_rate_for_cool_assignment_count, 0);
}

#[test]
fn delta_just_below_negative_threshold_enters_assignment_body() {
    let mut input = base_input();
    input.minimum_cooling_supply_air_temperature_c = -1.000_000_1e-5;
    input.zone_temperature_c = 0.0;
    let (snapshot, state) = characterize(active_predecessor(), input);

    assert_eq!(
        snapshot.delta_temperature_below_negative_small_temp_diff,
        Some(true)
    );
    assert!(snapshot.delta_temperature_body_entered);
    assert!(snapshot.zone_cooling_setpoint_load_read);
    assert!(snapshot.supply_mass_flow_rate_for_cool_assigned);
    assert_eq!(state.delta_temperature_body_entry_count, 1);
}

#[test]
fn false_cooling_availability_only_executes_reset_and_cooling_on_read() {
    let mut input = base_input();
    input.cooling_on = false;
    input.zone_humidity_ratio = f64::NAN;
    input.minimum_cooling_supply_air_temperature_c = f64::NAN;
    input.zone_temperature_c = f64::NAN;
    input.zone_cooling_setpoint_load_w = f64::NAN;
    let (snapshot, state) = characterize(active_predecessor(), input);

    assert!(snapshot.supply_mass_flow_rate_for_cool_reset_assigned);
    assert_bits(snapshot.reset_supply_mass_flow_rate_for_cool_kg_per_s, 0.0);
    assert!(snapshot.cooling_on_read);
    assert_eq!(snapshot.cooling_on, Some(false));
    assert!(!snapshot.cooling_on_body_entered);
    assert!(!snapshot.zone_humidity_ratio_read);
    assert!(snapshot.zone_humidity_ratio.is_none());
    assert!(!snapshot.minimum_cooling_supply_air_temperature_read);
    assert!(!snapshot.zone_temperature_read);
    assert!(!snapshot.zone_cooling_setpoint_load_read);
    assert_bits(
        snapshot.resulting_supply_mass_flow_rate_for_cool_kg_per_s,
        0.0,
    );
    assert_eq!(state.cooling_on_fallthrough_count, 1);
    assert_eq!(state.psychrometric_cp_air_evaluation_count, 0);
}
