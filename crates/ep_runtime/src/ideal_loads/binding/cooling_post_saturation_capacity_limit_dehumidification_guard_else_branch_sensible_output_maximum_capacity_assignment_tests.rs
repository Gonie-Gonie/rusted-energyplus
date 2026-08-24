//! CP422 scheduled-binding order and no-feed contracts.

#[test]
fn scheduled_binding_advances_cp422_after_cp421_before_unchanged_coupling() {
    let source = include_str!("../binding.rs");
    let guard = source
        .find("let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard =")
        .expect("CP421 binding");
    let assignment = source
        .find("let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment =")
        .expect("CP422 binding");
    let coupling = source
        .find("let coupling = complete_direct_zone_purchased_air_coupling(")
        .expect("numerical coupling");
    assert!(guard < assignment && assignment < coupling);
    let coupling_input = &source[assignment..coupling];
    assert!(!coupling_input.contains("zone_sensible_cooling_rate_w ="));
    assert!(!coupling_input.contains("DirectZonePurchasedAirCouplingInput {"));
}

#[test]
fn cp422_adapter_accepts_only_the_cp421_snapshot() {
    let source = include_str!(
        "cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment.rs"
    );
    assert!(source.contains("predecessor_cp421"));
    assert!(source.contains("SensibleOutputGuardSnapshot"));
    for forbidden in ["numerical_dto", "prediction", "feedback", "reports"] {
        assert!(!source.contains(forbidden), "{forbidden}");
    }
}

#[test]
fn cp422_is_preserved_before_cp423_in_the_current_116_snapshot_binding() {
    let source = include_str!("scheduled_output.rs");
    assert_eq!(source.matches("    pub calculation_").count(), 118);
}
