//! CP435 flat-schema, lossless-prefix, enum, and cold/validated parity locks.

use super::*;

#[test]
fn cp435_schema_is_exact_385_133_8_6_with_cp434_first_358_and_locked_tail() {
    let cp434 = public_fields(include_str!(
        "../../heating_operating_mode_deadband_assignment.rs"
    ));
    let cp435 = public_fields(include_str!(
        "../../heating_outdoor_air_maximum_flow_guard.rs"
    ));
    assert_eq!(cp434.len(), 361);
    assert_eq!(cp435.len(), 385);
    assert_eq!(&cp435[..358], &cp434[..358]);
    assert_eq!(
        &cp435[358..],
        &[
            "predecessor_cp434_resulting_supply_humidity_ratio",
            "predecessor_cp434_resulting_supply_enthalpy_j_per_kg",
            "predecessor_cp434_resulting_supply_temperature_c",
            "heating_outdoor_air_maximum_flow_guard_evaluated",
            "heating_limit_flow_rate_comparison_evaluated",
            "heating_limit_flow_rate_value",
            "heating_limit_flow_rate_comparison_satisfied",
            "heating_limit_flow_rate_and_capacity_comparison_evaluated",
            "heating_limit_flow_rate_and_capacity_value",
            "heating_limit_flow_rate_and_capacity_comparison_satisfied",
            "heating_flow_limit_active",
            "heating_flow_limit_selector_rejected",
            "cp311_same_call_outdoor_air_mass_flow_rate_bit_corroborated",
            "outdoor_air_mass_flow_rate_read_after_heating_limit_short_circuit",
            "outdoor_air_mass_flow_rate_for_heating_maximum_flow_guard_kg_per_s",
            "maximum_heating_air_mass_flow_rate_read_after_heating_limit_short_circuit",
            "maximum_heating_air_mass_flow_rate_for_guard_kg_per_s",
            "outdoor_air_mass_flow_rate_maximum_heating_air_mass_flow_rate_comparison_evaluated",
            "outdoor_air_mass_flow_rate_strictly_greater_than_maximum_heating_air_mass_flow_rate",
            "maximum_heating_flow_body_entered",
            "heating_outdoor_air_maximum_flow_guard_false_fallthrough",
            "cp434_retained_supply_humidity_ratio_state_owned",
            "cp434_retained_supply_enthalpy_state_owned",
            "cp434_retained_supply_temperature_state_owned",
            "resulting_supply_humidity_ratio",
            "resulting_supply_enthalpy_j_per_kg",
            "resulting_supply_temperature_c",
        ],
    );
    let block = snapshot_block(include_str!(
        "../../heating_outdoor_air_maximum_flow_guard.rs"
    ));
    assert_eq!(block.matches("Option<f64>").count(), 133);
    assert_eq!(block.matches("Option<bool>").count(), 8);
    assert_eq!(block.matches("Option<").count() - 133 - 8, 6);
}

#[test]
fn predecessor_reconstruction_and_cold_validated_paths_are_bit_exact_for_all_64_routes() {
    for (predecessor, limit, outdoor, maximum) in route_cases() {
        let predecessor_route = predecessor_route_for(predecessor);
        let route = route_for(predecessor, limit, outdoor, maximum);
        let cold = advance(
            &mut State::new(predecessor.system),
            predecessor,
            limit,
            outdoor,
            maximum,
        )
        .expect("cold CP435");
        let validated = advance_validated(
            &mut State::new(predecessor.system),
            predecessor,
            predecessor_route,
            limit,
            outdoor,
            maximum,
            route,
        )
        .expect("validated CP435");
        let reconstructed =
            super::super::heating_outdoor_air_maximum_flow_guard_predecessor_cp434_snapshot(cold);
        assert!(
            crate::ideal_loads::heating_operating_mode_deadband_assignment_snapshots_match_bit_exact(
                reconstructed,
                predecessor,
            )
        );
        assert!(
            super::super::heating_outdoor_air_maximum_flow_guard_snapshots_match_bit_exact(
                cold, validated,
            )
        );
    }
}

fn public_fields(source: &'static str) -> Vec<&'static str> {
    snapshot_block(source)
        .split("pub ")
        .skip(1)
        .filter_map(|tail| tail.split_once(':').map(|(field, _)| field.trim()))
        .collect()
}

fn snapshot_block(source: &'static str) -> &'static str {
    let start = source.find("pub struct PurchasedAirCalc").expect("snapshot start");
    let source = &source[start..];
    let end = source.find("\n}\n\n/// Final").expect("snapshot end");
    &source[..end]
}
