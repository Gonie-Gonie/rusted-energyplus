#[test]
fn scheduled_binding_advances_cp426_after_cp425_before_unchanged_coupling() {
    let source = include_str!("../binding.rs");
    let predecessor = source
        .find(
            "let calculation_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment =",
        )
        .expect("CP425 scheduled binding");
    let successor = source
        .find("let calculation_cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment =")
        .expect("CP426 scheduled binding");
    let coupling = source
        .find("let coupling = complete_direct_zone_purchased_air_coupling(")
        .expect("unchanged numerical coupling");
    assert!(predecessor < successor && successor < coupling);
    let between = &source[successor..coupling];
    assert!(!between.contains("DirectZonePurchasedAirCouplingInput {"));
    assert!(!between.contains("zone_sensible_cooling_rate_w ="));
}

#[test]
fn cp426_adapter_accepts_only_the_cp425_snapshot() {
    let source =
        include_str!("cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment.rs");
    assert!(source.contains(
        "predecessor_cp425: PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyEnthalpyMixedAirAssignmentSnapshot"
    ));
    assert!(!source.contains("DirectZonePurchasedAirCouplingInput"));
}

#[test]
fn cp426_extends_scheduled_binding_from_116_to_exactly_117_snapshots() {
    assert_eq!(
        include_str!("scheduled_output.rs")
            .matches("    pub calculation_")
            .count(),
        117
    );
}
