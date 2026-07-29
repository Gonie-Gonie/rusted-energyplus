use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId, ZoneId};

use super::*;

#[test]
fn missing_direct_lifecycle_fails_closed() {
    assert!(
        validate_direct_lifecycle(
            None,
            DirectLifecyclePredecessors {
                case_entry_cp364: None,
            },
            None,
            None,
        )
        .is_err()
    );
}

#[test]
fn checked_partitions_and_source_counts_fail_closed() {
    let mut state = PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentRuntimeState::new(
        IdealLoadsAirSystemId(0),
    );
    state.unit_off_skip_count = usize::MAX;
    state.non_cooling_skip_count = 1;
    assert!(validate_route_partition(&state).is_err());

    let mut active = PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentRuntimeState::new(
        IdealLoadsAirSystemId(0),
    );
    active.dehumidification_control_constant_supply_humidity_ratio_assignment_count = 1;
    assert!(validate_source_counters(&active).is_err());
}

#[test]
fn direct_release_is_complete_null_and_matches_immediate_predecessor() {
    let (lifecycle, predecessor) = direct_lifecycles();
    assert!(
        validate_release_state(
            &lifecycle,
            &predecessor,
            IdealLoadsAirSystemId(0),
            ZoneId(0),
            1,
        )
        .is_ok()
    );

    let latest = lifecycle.state.latest.expect("latest");
    assert!(latest.dehumidification_control_none_case_completed_skip);
    assert!(!latest.dehumidification_control_constant_supply_humidity_ratio_assignment_executed);
    assert!(!latest.minimum_cooling_supply_air_humidity_ratio_read);
    assert!(latest.minimum_cooling_supply_air_humidity_ratio.is_none());
    assert!(!latest.supply_humidity_ratio_assigned);
    assert!(latest.assigned_supply_humidity_ratio.is_none());
    assert!(latest.resulting_supply_humidity_ratio.is_none());
}

#[test]
fn forged_selector_prefix_lineage_and_numeric_payload_fail_closed() {
    let (lifecycle, predecessor) = direct_lifecycles();

    let mut forged_selector = predecessor.clone();
    let latest = forged_selector.state.latest.as_mut().expect("latest");
    latest.predecessor_dehumidification_control_type =
        Some(DehumidificationControlType::ConstantSupplyHumidityRatio);
    let mut matching = lifecycle.clone();
    matching.state.latest = Some(expected_snapshot(*latest));
    assert!(
        validate_release_state(
            &matching,
            &forged_selector,
            IdealLoadsAirSystemId(0),
            ZoneId(0),
            1,
        )
        .is_err()
    );

    let mut forged_prefix = predecessor.clone();
    let latest = forged_prefix.state.latest.as_mut().expect("latest");
    latest.predecessor_positive_supply_mass_flow_body_entered = false;
    let mut matching = lifecycle.clone();
    matching.state.latest = Some(expected_snapshot(*latest));
    assert!(
        validate_release_state(
            &matching,
            &forged_prefix,
            IdealLoadsAirSystemId(0),
            ZoneId(0),
            1,
        )
        .is_err()
    );

    let mut forged_lineage = predecessor.clone();
    let latest = forged_lineage.state.latest.as_mut().expect("latest");
    latest.predecessor_dehumidification_control_none_case_completed_skip = false;
    let mut matching = lifecycle.clone();
    matching.state.latest = Some(expected_snapshot(*latest));
    assert!(
        validate_release_state(
            &matching,
            &forged_lineage,
            IdealLoadsAirSystemId(0),
            ZoneId(0),
            1,
        )
        .is_err()
    );

    let mut numeric = lifecycle;
    let latest = numeric.state.latest.as_mut().expect("latest");
    latest.minimum_cooling_supply_air_humidity_ratio = Some(0.0077);
    assert!(
        validate_release_state(
            &numeric,
            &predecessor,
            IdealLoadsAirSystemId(0),
            ZoneId(0),
            1,
        )
        .is_err()
    );
}

fn direct_lifecycles() -> (
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryLifecycleSummary,
) {
    let system = IdealLoadsAirSystemId(0);
    let predecessor_latest = predecessor_snapshot();
    let mut predecessor_state =
        PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryRuntimeState::new(system);
    predecessor_state.transition_count = 1;
    predecessor_state.dehumidification_control_none_case_completed_skip_count = 1;
    predecessor_state.latest = Some(predecessor_latest);
    let predecessor =
        PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntryLifecycleSummary {
            source:
                PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_ENTRY_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_ENTRY_FIRST_EXCLUDED_SOURCE,
            state: predecessor_state,
        };

    let mut state =
        PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentRuntimeState::new(system);
    state.transition_count = 1;
    state.dehumidification_control_none_case_completed_skip_count = 1;
    state.latest = Some(expected_snapshot(predecessor_latest));
    (
        PurchasedAirCalcCoolingConstantSupplyHumidityRatioAssignmentLifecycleSummary {
            source: PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
            state,
        },
        predecessor,
    )
}

fn predecessor_snapshot() -> PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntrySnapshot {
    PurchasedAirCalcCoolingConstantSupplyHumidityRatioCaseEntrySnapshot {
        source: PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_ENTRY_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_ENTRY_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_ENTRY_SOURCE_ORDER,
        system: IdealLoadsAirSystemId(0),
        parent_call_ordinal: 1,
        controlled_zone: ZoneId(0),
        unit_body_entered: true,
        predecessor_cooling_body_entered: true,
        predecessor_no_outdoor_air_fallback_entered: true,
        predecessor_positive_supply_mass_flow_body_entered: true,
        unit_off_skipped: false,
        non_cooling_skipped: false,
        positive_guard_false_fallthrough_skipped: false,
        predecessor_dehumidification_control_type: Some(DehumidificationControlType::None),
        predecessor_dehumidification_control_none_case_completed_skip: true,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip:
            false,
        predecessor_dehumidification_control_humidistat_case_exited_via_break: false,
        predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
            false,
        dehumidification_control_none_case_completed_skip: true,
        dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: false,
        dehumidification_control_humidistat_case_completed_skip: false,
        dehumidification_control_constant_supply_humidity_ratio_case_entered: false,
    }
}
