#[test]
fn scheduled_binding_advances_cp434_after_cp433_before_unchanged_coupling() {
    let source = include_str!("../binding.rs");
    let predecessor = source
        .find("let calculation_heating_mode_guard_else_branch_entry =")
        .expect("CP433 scheduled binding");
    let successor = source
        .find("let calculation_heating_operating_mode_deadband_assignment =")
        .expect("CP434 scheduled binding");
    let coupling = source
        .find("let coupling = complete_direct_zone_purchased_air_coupling(")
        .expect("unchanged numerical coupling");
    assert!(predecessor < successor && successor < coupling);
    let between = &source[successor..coupling];
    assert!(!between.contains("DirectZonePurchasedAirCouplingInput {"));
    assert!(!between.contains("calculation.mode ="));
}

#[test]
fn cp434_adapter_accepts_only_the_cp433_snapshot_and_no_scalar_input() {
    let source = include_str!("heating_operating_mode_deadband_assignment.rs");
    assert!(
        source
            .contains("predecessor_cp433: PurchasedAirCalcHeatingModeGuardElseBranchEntrySnapshot")
    );
    assert!(!source.contains("DirectZonePurchasedAirCouplingInput"));
    assert!(!source.contains(": f64"));
}

#[test]
fn cp434_historical_124_to_125_transition_is_preserved_in_current_126_snapshot_binding() {
    let fields = include_str!("scheduled_output.rs")
        .lines()
        .filter(|line| line.starts_with("    pub calculation_"))
        .collect::<Vec<_>>();
    assert_eq!(fields.len(), 126);
    assert!(fields[122].contains("calculation_heating_operating_mode_heat_assignment"));
    assert!(fields[123].contains("calculation_heating_mode_guard_else_branch_entry"));
    assert!(fields[124].contains("calculation_heating_operating_mode_deadband_assignment"));
}
