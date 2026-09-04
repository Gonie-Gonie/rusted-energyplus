#[test]
fn scheduled_binding_advances_cp437_after_cp436_before_unchanged_coupling() {
    let source = include_str!("../binding.rs");
    let predecessor = source
        .find("let calculation_heating_outdoor_air_maximum_flow_body_volume_flow_assignment =")
        .expect("CP436 scheduled binding");
    let successor = source
        .find("let calculation_heating_outdoor_air_maximum_flow_first_warning_guard =")
        .expect("CP437 scheduled binding");
    let coupling = source
        .find("let coupling = complete_direct_zone_purchased_air_coupling(")
        .expect("unchanged numerical coupling");
    assert!(predecessor < successor && successor < coupling);
    let between = &source[successor..coupling];
    assert!(!between.contains("DirectZonePurchasedAirCouplingInput {"));
    assert!(!between.contains("calculation.mode ="));
}

#[test]
fn cp437_adapter_accepts_only_the_cp436_snapshot_and_no_scalar_input() {
    let source = include_str!("heating_outdoor_air_maximum_flow_first_warning_guard.rs");
    assert!(source.contains(
        "predecessor_cp436: PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentSnapshot"
    ));
    assert!(!source.contains("DirectZonePurchasedAirCouplingInput"));
    assert!(!source.contains(": f64"));
    assert!(!source.contains(": usize"));
}

#[test]
fn cp437_remains_index_127_in_current_131_snapshot_binding() {
    let fields = include_str!("scheduled_output.rs")
        .lines()
        .filter(|line| line.starts_with("    pub calculation_"))
        .collect::<Vec<_>>();
    assert_eq!(fields.len(), 131);
    assert!(fields[125].contains("calculation_heating_outdoor_air_maximum_flow_guard"));
    assert!(
        fields[126]
            .contains("calculation_heating_outdoor_air_maximum_flow_body_volume_flow_assignment")
    );
    assert!(
        fields[127].contains("calculation_heating_outdoor_air_maximum_flow_first_warning_guard")
    );
    assert!(
        fields[128].contains(
            "calculation_heating_outdoor_air_maximum_flow_first_warning_counter_increment"
        )
    );
}
