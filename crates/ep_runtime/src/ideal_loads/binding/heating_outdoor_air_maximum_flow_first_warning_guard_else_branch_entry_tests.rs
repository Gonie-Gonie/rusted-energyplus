#[test]
fn scheduled_binding_advances_cp442_after_cp441_before_unchanged_coupling() {
    let source = include_str!("../binding.rs");
    let predecessor = source
        .find("let calculation_heating_outdoor_air_maximum_flow_continue_warning_timestamp_call =")
        .expect("CP441 scheduled binding");
    let successor = source
        .find("let calculation_heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry =")
        .expect("CP442 scheduled binding");
    let coupling = source
        .find("let coupling = complete_direct_zone_purchased_air_coupling(")
        .expect("unchanged numerical coupling");
    assert!(predecessor < successor && successor < coupling);
    let between = &source[successor..coupling];
    assert!(!between.contains("DirectZonePurchasedAirCouplingInput {"));
    assert!(!between.contains("ShowRecurringWarningErrorAtEnd"));
}

#[test]
fn cp442_adapter_accepts_only_the_cp441_snapshot_and_no_scalar_input() {
    let source =
        include_str!("heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry.rs");
    assert!(source.contains(
        "predecessor_cp441: PurchasedAirCalcHeatingOutdoorAirMaximumFlowContinueWarningTimestampCallSnapshot"
    ));
    assert!(!source.contains("DirectZonePurchasedAirCouplingInput"));
    assert!(!source.contains(": f64"));
    assert!(!source.contains(": usize"));
}

#[test]
fn cp442_extends_current_scheduled_binding_from_132_to_133_snapshots() {
    let fields = include_str!("scheduled_output.rs")
        .lines()
        .filter(|line| line.contains("pub calculation_"))
        .collect::<Vec<_>>();
    assert_eq!(fields.len(), 133);
    assert!(
        fields[130].contains("calculation_heating_outdoor_air_maximum_flow_continue_warning_call")
    );
    assert!(
        fields[131].contains(
            "calculation_heating_outdoor_air_maximum_flow_continue_warning_timestamp_call"
        )
    );
    assert!(fields[132].contains(
        "calculation_heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry"
    ));
}
