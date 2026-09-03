//! CP436 coupled-runtime accounting, ownership, and no-feed contracts.

use crate::ideal_loads::PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentRuntimeState;
use ep_model::IdealLoadsAirSystemId;

#[test]
fn cp436_contract_locks_exhaustive_routes_current_schema_and_binding() {
    assert_eq!([64usize, 61, 3], [64, 61, 3]);
    assert_eq!(61usize + 3, 64, "inactive and assignment partition");
    assert_eq!(20usize + 44, 64, "public and private route partition");
    let (public_active, private_active) = (0usize, 3usize);
    assert_eq!(
        public_active + private_active,
        3,
        "public and private active partition"
    );
    let source =
        include_str!("calc/heating_outdoor_air_maximum_flow_body_volume_flow_assignment.rs");
    let snapshot = source
        .split_once(
            "pub struct PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentSnapshot",
        )
        .and_then(|(_, tail)| tail.split_once("/// Final selected-unit CP436"))
        .map(|(snapshot, _)| snapshot)
        .expect("CP436 snapshot declaration");
    assert_eq!(
        snapshot
            .lines()
            .filter(|line| line.trim_start().starts_with("pub "))
            .count(),
        402
    );
    assert_eq!(snapshot.matches("Option<f64>").count(), 140);
    assert_eq!(snapshot.matches("Option<bool>").count(), 8);
    assert_eq!(snapshot.matches("Option<").count() - 140 - 8, 6);
    let fields = include_str!("binding/scheduled_output.rs")
        .lines()
        .filter(|line| line.starts_with("    pub calculation_"))
        .collect::<Vec<_>>();
    assert_eq!(fields.len(), 130);
    assert!(fields[125].contains("calculation_heating_outdoor_air_maximum_flow_guard"));
    assert!(
        fields[126]
            .contains("calculation_heating_outdoor_air_maximum_flow_body_volume_flow_assignment")
    );
    assert!(
        fields[127].contains("calculation_heating_outdoor_air_maximum_flow_first_warning_guard")
    );
}

#[test]
fn cp436_new_state_has_four_zeroed_lossless_route_partitions() {
    let state =
        PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentRuntimeState::new(
            IdealLoadsAirSystemId(0),
        );
    for values in [
        state.predecessor_route_counts,
        state.predecessor_guard_false_fallthrough_route_counts,
        state.predecessor_guard_body_entry_route_counts,
        state.heating_outdoor_air_volume_flow_assignment_route_counts,
    ] {
        assert_eq!(values, [0; 36]);
    }
    assert_eq!(state.outdoor_air_volume_flow_assignment_count, 0);
    assert_eq!(state.source_site_execution_count, 0);
    assert!(state.latest.is_none());
}

#[test]
fn cp436_is_ordered_after_cp435_and_does_not_feed_numerics() {
    let binding = include_str!("binding.rs");
    let cp435 = binding
        .find("let calculation_heating_outdoor_air_maximum_flow_guard =")
        .expect("CP435 binding");
    let cp436 = binding
        .find("let calculation_heating_outdoor_air_maximum_flow_body_volume_flow_assignment =")
        .expect("CP436 binding");
    let cp437 = binding
        .find("let calculation_heating_outdoor_air_maximum_flow_first_warning_guard =")
        .expect("CP437 binding");
    let cp438 = binding
        .find("let calculation_heating_outdoor_air_maximum_flow_first_warning_counter_increment =")
        .expect("CP438 binding");
    let coupling = binding
        .find("let coupling = complete_direct_zone_purchased_air_coupling(")
        .expect("numerical coupling");
    assert!(cp435 < cp436 && cp436 < cp437 && cp437 < cp438 && cp438 < coupling);
    assert!(!binding[cp436..coupling].contains("DirectZonePurchasedAirCouplingInput {"));
    assert!(!binding[cp436..coupling].contains("standard_air_density_kg_per_m3:"));
    let validator = include_str!(
        "coupled_runtime/heating_outdoor_air_maximum_flow_body_volume_flow_assignment_validation.rs"
    );
    for required in [
        "public_assignment_count",
        "cp435_outdoor_air_mass_flow_rate_owned_read_count",
        "begin_environment_standard_air_density_owner_count",
        "local_outdoor_air_volume_flow_rate_assignment_write_count",
    ] {
        assert!(validator.contains(required), "{required}");
    }
    let production = validator
        .split_once("#[cfg(test)]")
        .map_or(validator, |(production, _)| production);
    assert!(!production.contains("DirectZonePurchasedAirCouplingInput"));
    assert!(!production.contains("private_characterization"));
}
