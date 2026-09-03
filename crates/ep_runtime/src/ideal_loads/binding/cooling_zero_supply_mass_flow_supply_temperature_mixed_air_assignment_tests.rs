#[test]
fn scheduled_binding_advances_cp427_after_cp426_before_unchanged_coupling() {
    let source = include_str!("../binding.rs");
    let predecessor = source
        .find("let calculation_cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment =")
        .expect("CP426 scheduled binding");
    let successor = source
        .find("let calculation_cooling_zero_supply_mass_flow_supply_temperature_mixed_air_assignment =")
        .expect("CP427 scheduled binding");
    let coupling = source
        .find("let coupling = complete_direct_zone_purchased_air_coupling(")
        .expect("unchanged numerical coupling");
    assert!(predecessor < successor && successor < coupling);
    let between = &source[successor..coupling];
    assert!(!between.contains("DirectZonePurchasedAirCouplingInput {"));
    assert!(!between.contains("zone_sensible_cooling_rate_w ="));
}

#[test]
fn cp427_adapter_accepts_only_the_cp426_snapshot() {
    let source =
        include_str!("cooling_zero_supply_mass_flow_supply_temperature_mixed_air_assignment.rs");
    assert!(source.contains(
        "predecessor_cp426: PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyHumidityRatioMixedAirAssignmentSnapshot"
    ));
    assert!(!source.contains("DirectZonePurchasedAirCouplingInput"));
}

#[test]
fn cp427_is_preserved_before_cp428_in_current_130_snapshot_binding() {
    assert_eq!(
        include_str!("scheduled_output.rs")
            .matches("    pub calculation_")
            .count(),
        130
    );
}
