//! CP441 flat-schema, exact-prefix, and marker locks.

#[test]
fn cp441_schema_is_exact_429_with_cp440_first_428_and_one_marker() {
    let cp440 = public_fields(include_str!(
        "../../heating_outdoor_air_maximum_flow_continue_warning_call.rs"
    ));
    let cp441 = public_fields(include_str!(
        "../../heating_outdoor_air_maximum_flow_continue_warning_timestamp_call.rs"
    ));
    assert_eq!(cp440.len(), 428);
    assert_eq!(cp441.len(), 429);
    assert_eq!(&cp441[..428], &cp440[..]);
    assert_eq!(
        cp441[428],
        "heating_outdoor_air_maximum_flow_continue_warning_timestamp_call_site_reached"
    );
    let mut unique = cp441.clone();
    unique.sort_unstable();
    unique.dedup();
    assert_eq!(unique.len(), 429);
}

#[test]
fn cp441_preserves_cp440_optional_type_counts() {
    let source = snapshot_block(include_str!(
        "../../heating_outdoor_air_maximum_flow_continue_warning_timestamp_call.rs"
    ));
    assert_eq!(source.matches("Option<f64>").count(), 146);
    assert_eq!(source.matches("Option<bool>").count(), 9);
    assert_eq!(source.matches("Option<usize>").count(), 2);
    assert_eq!(source.matches("Option<").count() - 146 - 9 - 2, 6);
}

#[test]
fn reconstruction_is_exact_for_all_67_predecessors() {
    use crate::ideal_loads::calc::{
        advance_heating_outdoor_air_maximum_flow_continue_warning_timestamp_call_state as advance,
        cp440_all_snapshots_for_successor_tests,
    };
    use crate::ideal_loads::{
        PurchasedAirCalcHeatingOutdoorAirMaximumFlowContinueWarningTimestampCallRuntimeState as State,
        heating_outdoor_air_maximum_flow_continue_warning_call_snapshots_match_bit_exact,
        heating_outdoor_air_maximum_flow_continue_warning_timestamp_call_predecessor_cp440_snapshot as prefix,
    };

    let predecessors = cp440_all_snapshots_for_successor_tests();
    assert_eq!(predecessors.len(), 67);
    for predecessor in predecessors {
        let mut state = State::new(predecessor.system);
        let snapshot = advance(&mut state, predecessor).expect("CP441");
        assert!(
            heating_outdoor_air_maximum_flow_continue_warning_call_snapshots_match_bit_exact(
                prefix(snapshot),
                predecessor,
            )
        );
    }
}

fn snapshot_block(source: &str) -> &str {
    let start = source
        .find("pub struct PurchasedAirCalcHeatingOutdoorAirMaximumFlowContinueWarningTimestampCallSnapshot")
        .expect("snapshot start");
    let end = source[start..]
        .find("/// Final selected-unit CP441")
        .map(|offset| start + offset)
        .expect("snapshot end");
    &source[start..end]
}

fn public_fields(source: &str) -> Vec<&str> {
    snapshot_block_for_either(source)
        .lines()
        .filter_map(|line| {
            line.trim()
                .strip_prefix("pub ")
                .and_then(|line| line.split_once(':').map(|(name, _)| name.trim()))
        })
        .collect()
}

fn snapshot_block_for_either(source: &str) -> &str {
    let start = source.find("Snapshot {").expect("snapshot start");
    let end = source[start..]
        .find("/// Final selected-unit CP")
        .map(|offset| start + offset)
        .expect("snapshot end");
    &source[start..end]
}
