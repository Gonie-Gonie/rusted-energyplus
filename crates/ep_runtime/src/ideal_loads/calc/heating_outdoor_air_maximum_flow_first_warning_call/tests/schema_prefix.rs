//! CP439 flat-schema, exact-prefix, and marker locks.

#[test]
fn cp439_schema_is_exact_427_with_cp438_first_426_and_one_marker() {
    let cp438 = public_fields(include_str!(
        "../../heating_outdoor_air_maximum_flow_first_warning_counter_increment.rs"
    ));
    let cp439 = public_fields(include_str!(
        "../../heating_outdoor_air_maximum_flow_first_warning_call.rs"
    ));
    assert_eq!(cp438.len(), 426);
    assert_eq!(cp439.len(), 427);
    assert_eq!(&cp439[..426], &cp438[..]);
    assert_eq!(
        cp439[426],
        "heating_outdoor_air_maximum_flow_first_warning_call_site_reached"
    );
    let mut unique = cp439.clone();
    unique.sort_unstable();
    unique.dedup();
    assert_eq!(unique.len(), 427);
}

#[test]
fn cp439_preserves_cp438_optional_type_counts() {
    let source = snapshot_block(include_str!(
        "../../heating_outdoor_air_maximum_flow_first_warning_call.rs"
    ));
    assert_eq!(source.matches("Option<f64>").count(), 146);
    assert_eq!(source.matches("Option<bool>").count(), 9);
    assert_eq!(source.matches("Option<usize>").count(), 2);
    assert_eq!(source.matches("Option<").count() - 146 - 9 - 2, 6);
}

#[test]
fn reconstruction_is_exact_for_all_67_predecessors() {
    use crate::ideal_loads::calc::{
        advance_heating_outdoor_air_maximum_flow_first_warning_call_state as advance,
        cp438_all_snapshots_for_successor_tests,
    };
    use crate::ideal_loads::{
        PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCallRuntimeState as State,
        heating_outdoor_air_maximum_flow_first_warning_call_predecessor_cp438_snapshot as prefix,
        heating_outdoor_air_maximum_flow_first_warning_counter_increment_snapshots_match_bit_exact,
    };

    let predecessors = cp438_all_snapshots_for_successor_tests();
    assert_eq!(predecessors.len(), 67);
    for predecessor in predecessors {
        let mut state = State::new(predecessor.system);
        let snapshot = advance(&mut state, predecessor).expect("CP439");
        assert!(
            heating_outdoor_air_maximum_flow_first_warning_counter_increment_snapshots_match_bit_exact(
                prefix(snapshot),
                predecessor,
            )
        );
    }
}

fn snapshot_block(source: &str) -> &str {
    let start = source
        .find("pub struct PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCallSnapshot")
        .expect("snapshot start");
    let end = source[start..]
        .find("/// Final selected-unit CP439")
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
