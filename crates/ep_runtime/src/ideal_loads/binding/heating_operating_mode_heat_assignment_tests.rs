#[test]
fn scheduled_binding_advances_cp432_after_cp431_before_unchanged_coupling() {
    let source = include_str!("../binding.rs");
    let predecessor = source
        .find("let calculation_heating_mode_guard =")
        .expect("CP431 scheduled binding");
    let successor = source
        .find("let calculation_heating_operating_mode_heat_assignment =")
        .expect("CP432 scheduled binding");
    let coupling = source
        .find("let coupling = complete_direct_zone_purchased_air_coupling(")
        .expect("unchanged numerical coupling");
    assert!(predecessor < successor && successor < coupling);
    let between = &source[successor..coupling];
    assert!(!between.contains("DirectZonePurchasedAirCouplingInput {"));
    assert!(!between.contains("assigned_heating_operating_mode"));
}

#[test]
fn cp432_adapter_accepts_only_the_cp431_snapshot_and_no_scalar_input() {
    let source = include_str!("heating_operating_mode_heat_assignment.rs");
    assert!(source.contains("predecessor_cp431: PurchasedAirCalcHeatingModeGuardSnapshot"));
    assert!(!source.contains("DirectZonePurchasedAirCouplingInput"));
    assert!(!source.contains(": f64"));
}

#[test]
fn cp432_is_preserved_at_index_122_in_current_128_snapshot_binding() {
    let fields = include_str!("scheduled_output.rs")
        .lines()
        .filter(|line| line.starts_with("    pub calculation_"))
        .collect::<Vec<_>>();
    assert_eq!(fields.len(), 128);
    assert!(fields[121].contains("calculation_heating_mode_guard"));
    assert!(fields[122].contains("calculation_heating_operating_mode_heat_assignment"));
    assert!(fields[124].contains("calculation_heating_operating_mode_deadband_assignment"));
}
