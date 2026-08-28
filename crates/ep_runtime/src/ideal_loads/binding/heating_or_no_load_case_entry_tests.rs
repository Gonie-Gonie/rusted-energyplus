#[test]
fn scheduled_binding_advances_cp430_after_cp429_before_unchanged_coupling() {
    let source = include_str!("../binding.rs");
    let predecessor = source
        .find(
            "let calculation_cooling_zero_supply_mass_flow_total_output_positive_zero_assignment =",
        )
        .expect("CP429 scheduled binding");
    let successor = source
        .find("let calculation_heating_or_no_load_case_entry =")
        .expect("CP430 scheduled binding");
    let coupling = source
        .find("let coupling = complete_direct_zone_purchased_air_coupling(")
        .expect("unchanged numerical coupling");
    assert!(predecessor < successor && successor < coupling);
    let between = &source[successor..coupling];
    assert!(!between.contains("DirectZonePurchasedAirCouplingInput {"));
    assert!(!between.contains("zone_sensible_cooling_rate_w ="));
}

#[test]
fn cp430_adapter_accepts_only_the_cp429_snapshot_and_no_scalar_input() {
    let source = include_str!("heating_or_no_load_case_entry.rs");
    assert!(source.contains(
        "predecessor_cp429: PurchasedAirCalcCoolingZeroSupplyMassFlowTotalOutputPositiveZeroAssignmentSnapshot"
    ));
    assert!(!source.contains("DirectZonePurchasedAirCouplingInput"));
    assert!(!source.contains(": f64"));
}

#[test]
fn cp430_extends_current_scheduled_binding_from_120_to_121_snapshots() {
    let fields = include_str!("scheduled_output.rs")
        .lines()
        .filter(|line| line.starts_with("    pub calculation_"))
        .collect::<Vec<_>>();
    assert_eq!(fields.len(), 121);
    assert!(fields[119].contains(
        "calculation_cooling_zero_supply_mass_flow_total_output_positive_zero_assignment"
    ));
    assert!(fields[120].contains("calculation_heating_or_no_load_case_entry"));
}
