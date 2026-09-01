#[test]
fn scheduled_binding_advances_cp436_after_cp435_before_unchanged_coupling() {
    let source = include_str!("../binding.rs");
    let predecessor = source
        .find("let calculation_heating_outdoor_air_maximum_flow_guard =")
        .expect("CP435 scheduled binding");
    let successor = source
        .find("let calculation_heating_outdoor_air_maximum_flow_body_volume_flow_assignment =")
        .expect("CP436 scheduled binding");
    let coupling = source
        .find("let coupling = complete_direct_zone_purchased_air_coupling(")
        .expect("unchanged numerical coupling");
    assert!(predecessor < successor && successor < coupling);
    let between = &source[successor..coupling];
    assert!(!between.contains("DirectZonePurchasedAirCouplingInput {"));
    assert!(!between.contains("calculation.mode ="));
}

#[test]
fn cp436_adapter_accepts_only_the_cp435_snapshot_and_no_scalar_input() {
    let source = include_str!("heating_outdoor_air_maximum_flow_body_volume_flow_assignment.rs");
    assert!(
        source.contains(
            "predecessor_cp435: PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardSnapshot"
        )
    );
    assert!(!source.contains("DirectZonePurchasedAirCouplingInput"));
    assert!(!source.contains(": f64"));
}

#[test]
fn cp436_remains_index_126_in_current_129_snapshot_binding() {
    let fields = include_str!("scheduled_output.rs")
        .lines()
        .filter(|line| line.starts_with("    pub calculation_"))
        .collect::<Vec<_>>();
    assert_eq!(fields.len(), 129);
    assert!(fields[124].contains("calculation_heating_operating_mode_deadband_assignment"));
    assert!(fields[125].contains("calculation_heating_outdoor_air_maximum_flow_guard"));
    assert!(
        fields[126]
            .contains("calculation_heating_outdoor_air_maximum_flow_body_volume_flow_assignment")
    );
}
