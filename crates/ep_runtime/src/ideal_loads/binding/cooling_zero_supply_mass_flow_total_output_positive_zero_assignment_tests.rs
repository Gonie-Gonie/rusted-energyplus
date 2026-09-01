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
fn cp429_historical_119_to_120_transition_is_preserved_before_cp430_in_current_127_snapshot_binding()
 {
    let fields = include_str!("scheduled_output.rs")
        .lines()
        .filter(|line| line.starts_with("    pub calculation_"))
        .collect::<Vec<_>>();
    assert_eq!(fields.len(), 127);
    assert!(fields[119].contains(
        "calculation_cooling_zero_supply_mass_flow_total_output_positive_zero_assignment"
    ));
    assert!(fields[120].contains("calculation_heating_or_no_load_case_entry"));
    assert!(fields[124].contains("calculation_heating_operating_mode_deadband_assignment"));
}
