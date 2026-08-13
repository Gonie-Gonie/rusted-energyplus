//! CP424 scheduled-binding order and no-feed contracts.

#[test]
fn scheduled_binding_advances_cp424_after_cp423_before_unchanged_coupling() {
    let source = include_str!("../binding.rs");
    let predecessor = source
        .find("let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment =")
        .expect("CP423 binding");
    let entry = source
        .find("let calculation_cooling_supply_mass_flow_positive_guard_else_branch_entry =")
        .expect("CP424 binding");
    let coupling = source
        .find("let coupling = complete_direct_zone_purchased_air_coupling(")
        .expect("numerical coupling");
    assert!(predecessor < entry && entry < coupling);
    let between = &source[entry..coupling];
    assert!(!between.contains("DirectZonePurchasedAirCouplingInput {"));
    assert!(!between.contains("zone_sensible_cooling_rate_w ="));
}

#[test]
fn cp424_adapter_accepts_only_the_cp423_snapshot() {
    let source = include_str!("cooling_supply_mass_flow_positive_guard_else_branch_entry.rs");
    assert!(source.contains("predecessor_cp423"));
    assert!(source.contains("SensibleOutputSupplyTemperatureAssignmentSnapshot"));
    for forbidden in ["numerical_dto", "prediction", "feedback", "reports"] {
        assert!(!source.contains(forbidden), "{forbidden}");
    }
}

#[test]
fn cp424_extends_scheduled_binding_from_114_to_exactly_115_snapshots() {
    let source = include_str!("scheduled_output.rs");
    assert_eq!(source.matches("    pub calculation_").count(), 115);
}
