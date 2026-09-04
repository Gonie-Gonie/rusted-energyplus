//! CP423 scheduled-binding order and no-feed contracts.

#[test]
fn scheduled_binding_advances_cp423_after_cp422_before_unchanged_coupling() {
    let source = include_str!("../binding.rs");
    let predecessor = source
        .find("let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment =")
        .expect("CP422 binding");
    let assignment = source
        .find("let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment =")
        .expect("CP423 binding");
    let coupling = source
        .find("let coupling = complete_direct_zone_purchased_air_coupling(")
        .expect("numerical coupling");
    assert!(predecessor < assignment && assignment < coupling);
    let between = &source[assignment..coupling];
    assert!(!between.contains("DirectZonePurchasedAirCouplingInput {"));
    assert!(!between.contains("zone_sensible_cooling_rate_w ="));
}

#[test]
fn cp423_adapter_accepts_only_the_cp422_snapshot() {
    let source = include_str!(
        "cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment.rs"
    );
    assert!(source.contains("predecessor_cp422"));
    assert!(source.contains("SensibleOutputMaximumCapacityAssignmentSnapshot"));
    for forbidden in ["numerical_dto", "prediction", "feedback", "reports"] {
        assert!(!source.contains(forbidden), "{forbidden}");
    }
}

#[test]
fn cp423_is_preserved_before_cp424_in_the_current_132_snapshot_binding() {
    let source = include_str!("scheduled_output.rs");
    assert_eq!(source.matches("    pub calculation_").count(), 132);
}
