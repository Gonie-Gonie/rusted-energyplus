#[test]
fn scheduled_binding_advances_cp429_after_cp428_before_unchanged_coupling() {
    let source = include_str!("../binding.rs");
    let predecessor = source
        .find("let calculation_cooling_zero_supply_mass_flow_sensible_output_positive_zero_assignment =")
        .expect("CP428 scheduled binding");
    let successor = source
        .find(
            "let calculation_cooling_zero_supply_mass_flow_total_output_positive_zero_assignment =",
        )
        .expect("CP429 scheduled binding");
    let coupling = source
        .find("let coupling = complete_direct_zone_purchased_air_coupling(")
        .expect("unchanged numerical coupling");
    assert!(predecessor < successor && successor < coupling);
    let between = &source[successor..coupling];
    assert!(!between.contains("DirectZonePurchasedAirCouplingInput {"));
    assert!(!between.contains("zone_sensible_cooling_rate_w ="));
}

#[test]
fn cp429_adapter_accepts_only_the_cp428_snapshot_and_no_scalar_input() {
    let source =
        include_str!("cooling_zero_supply_mass_flow_total_output_positive_zero_assignment.rs");
    assert!(source.contains(
        "predecessor_cp428: PurchasedAirCalcCoolingZeroSupplyMassFlowSensibleOutputPositiveZeroAssignmentSnapshot"
    ));
    assert!(!source.contains("DirectZonePurchasedAirCouplingInput"));
    assert!(!source.contains("cooling_total_output_w: f64"));
}

#[test]
fn cp429_extends_scheduled_binding_from_119_to_exactly_120_snapshots() {
    assert_eq!(
        include_str!("scheduled_output.rs")
            .matches("    pub calculation_")
            .count(),
        120
    );
}
