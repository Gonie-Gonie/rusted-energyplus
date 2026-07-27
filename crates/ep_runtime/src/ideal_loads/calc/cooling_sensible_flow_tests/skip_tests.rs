use super::{characterize, non_cooling_predecessor, poison_input, unit_off_predecessor};

#[test]
fn unit_off_skips_every_source_site_with_poisoned_inputs() {
    let (snapshot, state) = characterize(unit_off_predecessor(), poison_input());

    assert!(snapshot.unit_off_skipped);
    assert!(!snapshot.cooling_body_entered);
    assert!(!snapshot.supply_mass_flow_rate_for_cool_reset_assigned);
    assert!(!snapshot.cooling_on_read);
    assert!(!snapshot.zone_humidity_ratio_read);
    assert!(!snapshot.zone_cooling_setpoint_load_read);
    assert!(
        snapshot
            .resulting_supply_mass_flow_rate_for_cool_kg_per_s
            .is_none()
    );
    assert_eq!(state.transition_count, 1);
    assert_eq!(state.unit_off_skip_count, 1);
    assert_eq!(state.cooling_body_entry_count, 0);
}

#[test]
fn active_non_cooling_skips_every_source_site_with_poisoned_inputs() {
    let (snapshot, state) = characterize(non_cooling_predecessor(), poison_input());

    assert!(snapshot.non_cooling_skipped);
    assert!(!snapshot.cooling_body_entered);
    assert!(!snapshot.supply_mass_flow_rate_for_cool_reset_assigned);
    assert!(!snapshot.cooling_on_read);
    assert!(!snapshot.zone_humidity_ratio_read);
    assert!(!snapshot.zone_cooling_setpoint_load_read);
    assert!(
        snapshot
            .resulting_supply_mass_flow_rate_for_cool_kg_per_s
            .is_none()
    );
    assert_eq!(state.transition_count, 1);
    assert_eq!(state.non_cooling_skip_count, 1);
    assert_eq!(state.cooling_body_entry_count, 0);
}
