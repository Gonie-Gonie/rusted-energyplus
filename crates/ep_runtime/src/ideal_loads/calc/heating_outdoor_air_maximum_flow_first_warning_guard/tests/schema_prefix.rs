//! CP437 flat-schema, lossless-prefix, enum, and cold/validated parity locks.

use super::*;

#[test]
fn cp437_schema_is_exact_416_143_9_6_1_with_cp436_first_399_and_locked_tail() {
    let cp436 = public_fields(include_str!(
        "../../heating_outdoor_air_maximum_flow_body_volume_flow_assignment.rs"
    ));
    let cp437 = public_fields(include_str!(
        "../../heating_outdoor_air_maximum_flow_first_warning_guard.rs"
    ));
    assert_eq!(cp436.len(), 402);
    assert_eq!(cp437.len(), 416);
    assert_eq!(&cp437[..399], &cp436[..399]);
    assert_eq!(
        &cp437[399..],
        &[
            "predecessor_cp436_resulting_supply_humidity_ratio",
            "predecessor_cp436_resulting_supply_enthalpy_j_per_kg",
            "predecessor_cp436_resulting_supply_temperature_c",
            "heating_outdoor_air_maximum_flow_first_warning_guard_evaluated",
            "cp436_retained_supply_humidity_ratio_state_owned",
            "cp436_retained_supply_enthalpy_state_owned",
            "cp436_retained_supply_temperature_state_owned",
            "outdoor_air_flow_maximum_heating_output_error_count_state_owned",
            "outdoor_air_flow_maximum_heating_output_error_count_read",
            "outdoor_air_flow_maximum_heating_output_error_count_before",
            "outdoor_air_flow_maximum_heating_output_error_count_less_than_one_comparison_evaluated",
            "outdoor_air_flow_maximum_heating_output_error_count_less_than_one",
            "heating_outdoor_air_maximum_flow_first_warning_branch_entered",
            "heating_outdoor_air_maximum_flow_first_warning_guard_false_fallthrough",
            "resulting_supply_humidity_ratio",
            "resulting_supply_enthalpy_j_per_kg",
            "resulting_supply_temperature_c",
        ],
    );
    let mut unique = cp437.clone();
    unique.sort_unstable();
    unique.dedup();
    assert_eq!(unique.len(), 416);

    let block = snapshot_block(include_str!(
        "../../heating_outdoor_air_maximum_flow_first_warning_guard.rs"
    ));
    assert_eq!(block.matches("Option<f64>").count(), 143);
    assert_eq!(block.matches("Option<bool>").count(), 9);
    assert_eq!(block.matches("Option<usize>").count(), 1);
    assert_eq!(block.matches("Option<").count() - 143 - 9 - 1, 6);
}

#[test]
fn predecessor_reconstruction_and_cold_validated_paths_are_bit_exact_for_all_67_routes() {
    let predecessors = cp436_all_snapshots_for_successor_tests();
    let mut outcomes = 0usize;
    for predecessor in predecessors {
        let predecessor_route = predecessor_route_for(predecessor);
        let counters: &[usize] = if predecessor_route.assignment_executed {
            &[0, 1]
        } else {
            &[0]
        };
        for &counter in counters {
            let route = route_for(predecessor, counter);
            let mut cold_state = State::new(predecessor.system);
            cold_state.outdoor_air_flow_maximum_heating_output_error_count = counter;
            let cold = advance(&mut cold_state, predecessor).expect("cold CP437");
            let mut validated_state = State::new(predecessor.system);
            validated_state.outdoor_air_flow_maximum_heating_output_error_count = counter;
            let validated =
                advance_validated(&mut validated_state, predecessor, predecessor_route, route)
                    .expect("validated CP437");
            let reconstructed = super::super::heating_outdoor_air_maximum_flow_first_warning_guard_predecessor_cp436_snapshot(cold);
            assert!(
                crate::ideal_loads::heating_outdoor_air_maximum_flow_body_volume_flow_assignment_snapshots_match_bit_exact(
                    reconstructed,
                    predecessor,
                )
            );
            assert!(super::super::heating_outdoor_air_maximum_flow_first_warning_guard_snapshots_match_bit_exact(
                cold,
                validated,
            ));
            outcomes += 1;
        }
    }
    assert_eq!(outcomes, 67);
}

fn public_fields(source: &'static str) -> Vec<&'static str> {
    snapshot_block(source)
        .lines()
        .filter_map(|line| line.trim().strip_prefix("pub "))
        .filter_map(|line| line.split_once(':').map(|(field, _)| field))
        .collect()
}

fn snapshot_block(source: &'static str) -> &'static str {
    let start = source
        .find("pub struct PurchasedAirCalc")
        .expect("snapshot start");
    let source = &source[start..];
    let end = source
        .find("\n}\n/// Final")
        .or_else(|| source.find("\n}\n\n/// Final"))
        .expect("snapshot end");
    &source[..end]
}
