#[test]
fn scheduled_binding_advances_cp425_after_cp424_before_unchanged_coupling() {
    let source = include_str!("../binding.rs");
    let predecessor = source
        .find("let calculation_cooling_supply_mass_flow_positive_guard_else_branch_entry =")
        .expect("CP424 scheduled binding");
    let successor = source
        .find(
            "let calculation_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment =",
        )
        .expect("CP425 scheduled binding");
    let coupling = source
        .find("let coupling = complete_direct_zone_purchased_air_coupling(")
        .expect("unchanged numerical coupling");
    assert!(predecessor < successor && successor < coupling);
    let between = &source[successor..coupling];
    assert!(!between.contains("DirectZonePurchasedAirCouplingInput {"));
    assert!(!between.contains("zone_sensible_cooling_rate_w ="));
}

#[test]
fn cp425_adapter_accepts_only_the_cp424_snapshot() {
    let source =
        include_str!("cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment.rs");
    assert!(source.contains(
        "predecessor_cp424: PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardElseBranchEntrySnapshot"
    ));
    assert!(!source.contains("DirectZonePurchasedAirCouplingInput"));
}

#[test]
fn cp425_is_preserved_before_cp426_in_current_120_snapshot_binding() {
    let source = include_str!("scheduled_output.rs");
    assert_eq!(source.matches("    pub calculation_").count(), 120);
    let cp425 = source
        .find("calculation_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment")
        .expect("CP425 output");
    let cp426 = source
        .find(
            "calculation_cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment",
        )
        .expect("CP426 output");
    assert!(cp425 < cp426);
}
