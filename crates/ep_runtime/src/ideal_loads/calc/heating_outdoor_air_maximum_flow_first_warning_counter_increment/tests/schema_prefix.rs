//! CP438 flat-schema, lossless-prefix, enum, and cold/validated parity locks.

use super::*;

#[test]
fn cp438_schema_is_exact_426_146_9_6_2_with_cp437_first_413_and_locked_tail() {
    let cp437 = public_fields(include_str!(
        "../../heating_outdoor_air_maximum_flow_first_warning_guard.rs"
    ));
    let cp438 = public_fields(include_str!(
        "../../heating_outdoor_air_maximum_flow_first_warning_counter_increment.rs"
    ));
    assert_eq!(cp437.len(), 416);
    assert_eq!(cp438.len(), 426);
    assert_eq!(&cp438[..413], &cp437[..413]);
    assert_eq!(
        &cp438[413..],
        &[
            "predecessor_cp437_resulting_supply_humidity_ratio",
            "predecessor_cp437_resulting_supply_enthalpy_j_per_kg",
            "predecessor_cp437_resulting_supply_temperature_c",
            "heating_outdoor_air_maximum_flow_first_warning_counter_increment_executed",
            "cp437_retained_supply_humidity_ratio_state_owned",
            "cp437_retained_supply_enthalpy_state_owned",
            "cp437_retained_supply_temperature_state_owned",
            "cp437_retained_outdoor_air_flow_maximum_heating_output_error_count_state_owned",
            "outdoor_air_flow_maximum_heating_output_error_count_increment_performed",
            "assigned_outdoor_air_flow_maximum_heating_output_error_count",
            "resulting_supply_humidity_ratio",
            "resulting_supply_enthalpy_j_per_kg",
            "resulting_supply_temperature_c",
        ],
    );
    let mut unique = cp438.clone();
    unique.sort_unstable();
    unique.dedup();
    assert_eq!(unique.len(), 426);

    let block = snapshot_block(include_str!(
        "../../heating_outdoor_air_maximum_flow_first_warning_counter_increment.rs"
    ));
    assert_eq!(block.matches("Option<f64>").count(), 146);
    assert_eq!(block.matches("Option<bool>").count(), 9);
    assert_eq!(block.matches("Option<usize>").count(), 2);
    assert_eq!(block.matches("Option<").count() - 146 - 9 - 2, 6);
}

#[test]
fn predecessor_reconstruction_and_cold_validated_paths_are_bit_exact_for_all_67_routes() {
    let predecessors = cp437_all_snapshots_for_successor_tests();
    assert_eq!(predecessors.len(), 67);
    for predecessor in predecessors {
        let predecessor_route = predecessor_route_for(predecessor);
        let route = route_for(predecessor);
        let mut cold_state = State::new(predecessor.system);
        let mut cold_owner = counter_owner_for(predecessor);
        let cold = advance(&mut cold_state, &mut cold_owner, predecessor).expect("cold CP438");
        let mut validated_state = State::new(predecessor.system);
        let mut validated_owner = counter_owner_for(predecessor);
        let validated = advance_validated(
            &mut validated_state,
            &mut validated_owner,
            predecessor,
            predecessor_route,
            route,
        )
        .expect("validated CP438");
        let reconstructed = super::super::heating_outdoor_air_maximum_flow_first_warning_counter_increment_predecessor_cp437_snapshot(cold);
        assert!(
            crate::ideal_loads::heating_outdoor_air_maximum_flow_first_warning_guard_snapshots_match_bit_exact(
                reconstructed,
                predecessor,
            )
        );
        assert!(super::super::heating_outdoor_air_maximum_flow_first_warning_counter_increment_snapshots_match_bit_exact(
            cold,
            validated,
        ));
        assert_eq!(cold_owner, validated_owner);
    }
}

#[test]
fn every_cp438_counter_increment_is_preflighted_transactionally() {
    macro_rules! scalar_overflow {
        ($predecessor:expr; $($field:ident),+ $(,)?) => {{
            let predecessor = $predecessor;
            $(
            let mut state = State::new(predecessor.system);
            let mut owner = counter_owner_for(predecessor);
            state.$field = usize::MAX;
            let state_before = state.clone();
            let owner_before = owner.clone();
            assert!(advance(&mut state, &mut owner, predecessor).is_none(), stringify!($field));
            assert_eq!(state, state_before, stringify!($field));
            assert_eq!(owner, owner_before, stringify!($field));
            )+
        }};
    }
    let active = active_predecessor();
    scalar_overflow!(
        active;
        transition_count,
        outdoor_air_flow_maximum_heating_output_error_count_increment_count,
        source_site_execution_count,
        cp437_outdoor_air_flow_maximum_heating_output_error_count_state_owner_count,
        outdoor_air_flow_maximum_heating_output_error_count_increment_write_count,
    );
    scalar_overflow!(
        predecessor_matching(
            |snapshot| snapshot.resulting_supply_humidity_ratio.is_some(),
            "CP437 humidity owner",
        );
        cp437_supply_humidity_ratio_state_owner_count,
        unchanged_supply_humidity_ratio_preservation_count,
    );
    scalar_overflow!(
        predecessor_matching(
            |snapshot| snapshot.resulting_supply_enthalpy_j_per_kg.is_some(),
            "CP437 enthalpy owner",
        );
        cp437_supply_enthalpy_state_owner_count,
        unchanged_supply_enthalpy_preservation_count,
    );
    scalar_overflow!(
        predecessor_matching(
            |snapshot| snapshot.resulting_supply_temperature_c.is_some(),
            "CP437 temperature owner",
        );
        cp437_supply_temperature_state_owner_count,
        unchanged_supply_temperature_preservation_count,
    );
    macro_rules! array_overflow {
        ($predecessor:expr; $($field:ident),+ $(,)?) => {{
            let predecessor = $predecessor;
            let route = route_for(predecessor);
            $(
            let mut state = State::new(predecessor.system);
            let mut owner = counter_owner_for(predecessor);
            state.$field[route.logical_index] = usize::MAX;
            let state_before = state.clone();
            let owner_before = owner.clone();
            assert!(advance(&mut state, &mut owner, predecessor).is_none(), stringify!($field));
            assert_eq!(state, state_before, stringify!($field));
            assert_eq!(owner, owner_before, stringify!($field));
            )+
        }};
    }
    array_overflow!(
        active;
        predecessor_route_counts,
        predecessor_guard_body_entry_route_counts,
        predecessor_volume_flow_assignment_route_counts,
        predecessor_first_warning_branch_entry_route_counts,
        heating_outdoor_air_maximum_flow_first_warning_counter_increment_route_counts,
    );
    array_overflow!(
        predecessor_matching(
            |snapshot| route_for(snapshot).predecessor_guard_false_fallthrough,
            "CP437 inherited guard-false route",
        );
        predecessor_guard_false_fallthrough_route_counts,
    );
    array_overflow!(
        predecessor_matching(
            |snapshot| route_for(snapshot).predecessor_first_warning_guard_false_fallthrough,
            "CP437 first-warning guard-false route",
        );
        predecessor_first_warning_guard_false_fallthrough_route_counts,
    );
    let inactive = predecessor_matching(
        |snapshot| !route_for(snapshot).counter_increment_executed,
        "inactive CP437",
    );
    let mut state = State::new(inactive.system);
    let mut owner = counter_owner_for(inactive);
    state.inactive_transition_count = usize::MAX;
    let state_before = state.clone();
    let owner_before = owner.clone();
    assert!(advance(&mut state, &mut owner, inactive).is_none());
    assert_eq!(state, state_before);
    assert_eq!(owner, owner_before);
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
