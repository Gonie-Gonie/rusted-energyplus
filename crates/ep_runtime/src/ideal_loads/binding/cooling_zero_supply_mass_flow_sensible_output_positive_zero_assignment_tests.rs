#[test]
fn scheduled_binding_advances_cp428_after_cp427_before_unchanged_coupling() {
    let source = include_str!("../binding.rs");
    let predecessor = source
        .find("let calculation_cooling_zero_supply_mass_flow_supply_temperature_mixed_air_assignment =")
        .expect("CP427 scheduled binding");
    let successor = source
        .find("let calculation_cooling_zero_supply_mass_flow_sensible_output_positive_zero_assignment =")
        .expect("CP428 scheduled binding");
    let coupling = source
        .find("let coupling = complete_direct_zone_purchased_air_coupling(")
        .expect("unchanged numerical coupling");
    assert!(predecessor < successor && successor < coupling);
    let between = &source[successor..coupling];
    assert!(!between.contains("DirectZonePurchasedAirCouplingInput {"));
    assert!(!between.contains("zone_sensible_cooling_rate_w ="));
}

#[test]
fn cp428_adapter_accepts_only_the_cp427_snapshot_and_no_scalar_input() {
    let source =
        include_str!("cooling_zero_supply_mass_flow_sensible_output_positive_zero_assignment.rs");
    assert!(source.contains(
        "predecessor_cp427: PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyTemperatureMixedAirAssignmentSnapshot"
    ));
    assert!(!source.contains("DirectZonePurchasedAirCouplingInput"));
    assert!(!source.contains("cooling_sensible_output_w: f64"));
}

#[test]
fn cp428_is_preserved_before_cp429_in_current_128_snapshot_binding() {
    let source = include_str!("scheduled_output.rs");
    assert_eq!(source.matches("    pub calculation_").count(), 128);
    let cp428 = source
        .find("calculation_cooling_zero_supply_mass_flow_sensible_output_positive_zero_assignment")
        .expect("CP428 output");
    let cp429 = source
        .find("calculation_cooling_zero_supply_mass_flow_total_output_positive_zero_assignment")
        .expect("CP429 output");
    assert!(cp428 < cp429);
}
