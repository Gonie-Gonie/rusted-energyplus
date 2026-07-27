use ep_model::DehumidificationControlType;

use super::{characterize, non_cooling_predecessor, poison_input, unit_off_predecessor};

#[test]
fn unit_off_skips_every_source_site_with_poisoned_inputs() {
    let (snapshot, state) = characterize(
        unit_off_predecessor(),
        poison_input(DehumidificationControlType::Humidistat),
    );
    assert!(snapshot.unit_off_skipped);
    assert!(!snapshot.cooling_body_entered);
    assert!(!snapshot.supply_mass_flow_rate_for_dehumidification_reset_assigned);
    assert!(!snapshot.cooling_on_read);
    assert!(!snapshot.dehumidification_control_type_read);
    assert!(!snapshot.zone_dehumidifying_setpoint_moisture_demand_read);
    assert!(!snapshot.zone_humidity_ratio_read);
    assert!(
        snapshot
            .resulting_supply_mass_flow_rate_for_dehumidification_kg_per_s
            .is_none()
    );
    assert_eq!(state.unit_off_skip_count, 1);
}

#[test]
fn active_non_cooling_skips_every_source_site_with_poisoned_inputs() {
    let (snapshot, state) = characterize(
        non_cooling_predecessor(),
        poison_input(DehumidificationControlType::Humidistat),
    );
    assert!(snapshot.non_cooling_skipped);
    assert!(!snapshot.cooling_body_entered);
    assert!(!snapshot.supply_mass_flow_rate_for_dehumidification_reset_assigned);
    assert!(!snapshot.cooling_on_read);
    assert!(!snapshot.dehumidification_control_type_read);
    assert!(!snapshot.zone_dehumidifying_setpoint_moisture_demand_read);
    assert!(!snapshot.zone_humidity_ratio_read);
    assert_eq!(state.non_cooling_skip_count, 1);
}
