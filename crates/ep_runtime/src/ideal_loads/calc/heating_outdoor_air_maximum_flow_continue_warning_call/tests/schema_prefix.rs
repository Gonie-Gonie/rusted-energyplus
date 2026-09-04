//! CP440 flat-schema, exact-prefix, and marker locks.

#[test]
fn cp440_schema_is_exact_428_with_cp439_first_427_and_one_marker() {
    let cp439 = public_fields(include_str!(
        "../../heating_outdoor_air_maximum_flow_first_warning_call.rs"
    ));
    let cp440 = public_fields(include_str!(
        "../../heating_outdoor_air_maximum_flow_continue_warning_call.rs"
    ));
    assert_eq!(cp439.len(), 427);
    assert_eq!(cp440.len(), 428);
    assert_eq!(&cp440[..427], &cp439[..]);
    assert_eq!(
        cp440[427],
        "heating_outdoor_air_maximum_flow_continue_warning_call_site_reached"
    );
    let mut unique = cp440.clone();
    unique.sort_unstable();
    unique.dedup();
    assert_eq!(unique.len(), 428);
}

#[test]
fn cp440_preserves_cp439_optional_type_counts() {
    let source = snapshot_block(include_str!(
        "../../heating_outdoor_air_maximum_flow_continue_warning_call.rs"
    ));
    assert_eq!(source.matches("Option<f64>").count(), 146);
    assert_eq!(source.matches("Option<bool>").count(), 9);
    assert_eq!(source.matches("Option<usize>").count(), 2);
    assert_eq!(source.matches("Option<").count() - 146 - 9 - 2, 6);
}

#[test]
fn reconstruction_is_exact_for_all_67_predecessors() {
    use crate::ideal_loads::calc::{
        advance_heating_outdoor_air_maximum_flow_continue_warning_call_state as advance,
        cp439_all_snapshots_for_successor_tests,
    };
    use crate::ideal_loads::{
        PurchasedAirCalcHeatingOutdoorAirMaximumFlowContinueWarningCallRuntimeState as State,
        heating_outdoor_air_maximum_flow_continue_warning_call_predecessor_cp439_snapshot as prefix,
        heating_outdoor_air_maximum_flow_first_warning_call_snapshots_match_bit_exact,
    };

    let predecessors = cp439_all_snapshots_for_successor_tests();
    assert_eq!(predecessors.len(), 67);
    for predecessor in predecessors {
        let mut state = State::new(predecessor.system);
        let snapshot = advance(&mut state, predecessor).expect("CP440");
        assert!(
            heating_outdoor_air_maximum_flow_first_warning_call_snapshots_match_bit_exact(
                prefix(snapshot),
                predecessor,
            )
        );
    }
}

fn snapshot_block(source: &str) -> &str {
    let start = source
        .find("pub struct PurchasedAirCalcHeatingOutdoorAirMaximumFlowContinueWarningCallSnapshot")
        .expect("snapshot start");
    let end = source[start..]
        .find("/// Final selected-unit CP440")
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
