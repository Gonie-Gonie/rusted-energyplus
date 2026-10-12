#[test]
fn serializers_carry_two_route_arrays_and_no_numerical_feed() {
    for (name, source) in [
        (
            "cooling_supply_mass_flow_positive_guard_else_branch_entry",
            include_str!(
                "purchased_air_cooling_supply_mass_flow_positive_guard_else_branch_entry/serialization.rs"
            ),
        ),
        (
            "cooling_zero_supply_mass_flow_sensible_output_positive_zero_assignment",
            include_str!(
                "purchased_air_cooling_zero_supply_mass_flow_sensible_output_positive_zero_assignment/serialization.rs"
            ),
        ),
        (
            "cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment",
            include_str!(
                "purchased_air_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment/serialization.rs"
            ),
        ),
        (
            "cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment",
            include_str!(
                "purchased_air_cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment/serialization.rs"
            ),
        ),
        (
            "cooling_zero_supply_mass_flow_supply_temperature_mixed_air_assignment",
            include_str!(
                "purchased_air_cooling_zero_supply_mass_flow_supply_temperature_mixed_air_assignment/serialization.rs"
            ),
        ),
        (
            "cooling_zero_supply_mass_flow_total_output_positive_zero_assignment",
            include_str!(
                "purchased_air_cooling_zero_supply_mass_flow_total_output_positive_zero_assignment/serialization.rs"
            ),
        ),
        (
            "heating_or_no_load_case_entry",
            include_str!("purchased_air_heating_or_no_load_case_entry/serialization.rs"),
        ),
    ] {
        let source = source
            .split_once("#[cfg(test)]")
            .map_or(source, |(production, _)| production);
        assert_eq!(source.matches("route_counts\":").count(), 2, "{name}");
        for forbidden in [
            "DirectZonePurchasedAirCouplingInput",
            "numerical_dto",
            "prediction",
            "feedback",
            "nodes",
            "loads",
            "reports",
        ] {
            assert!(!source.contains(forbidden), "{name}: {forbidden}");
        }
    }
}
