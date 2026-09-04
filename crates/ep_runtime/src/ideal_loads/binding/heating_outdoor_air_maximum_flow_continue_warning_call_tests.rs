#[test]
fn scheduled_binding_advances_cp440_after_cp439_before_unchanged_coupling() {
    let source = include_str!("../binding.rs");
    let predecessor = source
        .find("let calculation_heating_outdoor_air_maximum_flow_first_warning_call =")
        .expect("CP439 scheduled binding");
    let successor = source
        .find("let calculation_heating_outdoor_air_maximum_flow_continue_warning_call =")
        .expect("CP440 scheduled binding");
    let coupling = source
        .find("let coupling = complete_direct_zone_purchased_air_coupling(")
        .expect("unchanged numerical coupling");
    assert!(predecessor < successor && successor < coupling);
    let between = &source[successor..coupling];
    assert!(!between.contains("DirectZonePurchasedAirCouplingInput {"));
    assert!(!between.contains("calculation.mode ="));
}

#[test]
fn cp440_adapter_accepts_only_the_cp439_snapshot_and_no_scalar_input() {
    let source = include_str!("heating_outdoor_air_maximum_flow_continue_warning_call.rs");
    assert!(source.contains(
        "predecessor_cp439: PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCallSnapshot"
    ));
    assert!(!source.contains("DirectZonePurchasedAirCouplingInput"));
    assert!(!source.contains(": f64"));
    assert!(!source.contains(": usize"));
}

#[test]
fn cp440_extends_current_scheduled_binding_from_130_to_131_snapshots() {
    let fields = include_str!("scheduled_output.rs")
        .lines()
        .filter(|line| line.starts_with("    pub calculation_"))
        .collect::<Vec<_>>();
    assert_eq!(fields.len(), 131);
    assert!(
        fields[128].contains(
            "calculation_heating_outdoor_air_maximum_flow_first_warning_counter_increment"
        )
    );
    assert!(
        fields[129].contains("calculation_heating_outdoor_air_maximum_flow_first_warning_call")
    );
    assert!(
        fields[130].contains("calculation_heating_outdoor_air_maximum_flow_continue_warning_call")
    );
}
