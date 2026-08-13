//! CP421 scheduled-binding topology checks.

#[test]
fn cp421_binding_is_after_cp420_and_before_unchanged_numerical_coupling() {
    let source = include_str!("../binding.rs");
    let cp420 = source
        .find("let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment =")
        .expect("CP420 binding");
    let cp421 = source
        .find("let calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard =")
        .expect("CP421 binding");
    let coupling = source
        .find("let coupling = complete_direct_zone_purchased_air_coupling(")
        .expect("numerical coupling");
    assert!(cp420 < cp421 && cp421 < coupling);
}

#[test]
fn cp421_does_not_feed_the_numerical_coupling_input() {
    let source = include_str!("../binding.rs");
    let coupling = source
        .split_once("let coupling = complete_direct_zone_purchased_air_coupling(")
        .expect("numerical coupling")
        .1
        .split_once("Ok(DirectZonePurchasedAirScheduledCouplingOutput")
        .expect("scheduled output")
        .0;
    assert!(!coupling.contains("sensible_output_guard"));
}

#[test]
fn cp421_extends_scheduled_binding_to_exactly_112_calculation_snapshots() {
    let source = include_str!("scheduled_output.rs");
    assert_eq!(source.matches("    pub calculation_").count(), 112);
}
