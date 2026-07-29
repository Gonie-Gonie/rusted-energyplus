use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId, ZoneId};
use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_ASSIGNMENT_SOURCE_ORDER,
};

use super::*;

#[test]
fn missing_direct_lifecycle_fails_closed() {
    assert!(
        validate_direct_lifecycle(
            None,
            DirectLifecyclePredecessors {
                supply_humidity_ratio_assignment_cp360: None,
            },
            None,
            None,
        )
        .is_err()
    );
}

#[test]
fn route_partition_overflow_fails_closed() {
    let mut state =
        PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitRuntimeState::new(
            IdealLoadsAirSystemId(0),
        );
    state.unit_off_skip_count = usize::MAX;
    state.non_cooling_skip_count = 1;
    assert!(validate_route_partition(&state).is_err());
}

#[test]
fn four_site_counters_are_exact_and_fail_closed_on_each_mismatch() {
    let valid = active_state();
    assert!(validate_source_counters(&valid).is_ok());

    for field in [
        "source_sites",
        "left_read",
        "right_read",
        "maximum",
        "assignment",
    ] {
        let mut state = valid.clone();
        match field {
            "source_sites" => state.source_site_execution_count = 3,
            "left_read" => state
                .supply_humidity_ratio_for_dehumidification_for_minimum_limit_maximum_read_count =
                0,
            "right_read" => {
                state.minimum_cooling_supply_air_humidity_ratio_for_maximum_read_count = 0
            }
            "maximum" => state.source_shaped_two_argument_maximum_evaluation_count = 0,
            "assignment" => state.supply_humidity_ratio_for_dehumidification_assignment_count = 0,
            _ => unreachable!(),
        }
        assert!(validate_source_counters(&state).is_err(), "{field}");
    }
}

#[test]
fn direct_expected_snapshot_is_complete_null_and_exact_bit_comparison_is_strict() {
    let predecessor = predecessor_snapshot(None, false);
    let expected = expected_snapshot(predecessor);
    assert!(expected.dehumidification_control_none_case_completed_skip);
    assert!(
        !expected
            .dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_executed
    );
    assert!(
        numeric_values(&expected)
            .into_iter()
            .all(|value| value.is_none())
    );
    assert!(snapshots_match_exact_bits(&expected, &expected));

    for field in 0..numeric_values(&expected).len() {
        let mut corrupted = expected;
        set_numeric(&mut corrupted, field, Some(f64::from_bits(1)));
        assert!(
            !snapshots_match_exact_bits(&corrupted, &expected),
            "numeric field {field}"
        );
    }

    let mut route_corruption = expected;
    route_corruption.dehumidification_control_none_case_completed_skip = false;
    assert!(!snapshots_match_exact_bits(&route_corruption, &expected));
}

#[test]
fn predecessor_numeric_bits_are_preserved_in_expected_snapshot() {
    for scalar in [-0.0, f64::INFINITY, f64::from_bits(0x7ff8_0000_0000_0061)] {
        let expected = expected_snapshot(predecessor_snapshot(Some(scalar), true));
        assert_eq!(
            expected
                .predecessor_resulting_supply_humidity_ratio_for_dehumidification
                .map(f64::to_bits),
            Some(scalar.to_bits())
        );
        assert!(snapshots_match_exact_bits(&expected, &expected));
    }
}

fn active_state()
-> PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitRuntimeState {
    let mut state =
        PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitRuntimeState::new(
            IdealLoadsAirSystemId(0),
        );
    state
        .dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_count =
        1;
    state.source_site_execution_count = 4;
    state.supply_humidity_ratio_for_dehumidification_for_minimum_limit_maximum_read_count = 1;
    state.minimum_cooling_supply_air_humidity_ratio_for_maximum_read_count = 1;
    state.source_shaped_two_argument_maximum_evaluation_count = 1;
    state.supply_humidity_ratio_for_dehumidification_assignment_count = 1;
    state
}

fn predecessor_snapshot(
    scalar: Option<f64>,
    humidistat: bool,
) -> PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentSnapshot {
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentSnapshot {
        source:
            PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_ASSIGNMENT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_ASSIGNMENT_SOURCE_ORDER,
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
        predecessor_dehumidification_control_type: Some(if humidistat {
            DehumidificationControlType::Humidistat
        } else {
            DehumidificationControlType::None
        }),
        predecessor_dehumidification_control_none_case_completed_skip: !humidistat,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip:
            false,
        predecessor_dehumidification_control_humidistat_moisture_demand_assignment_executed:
            humidistat,
        predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
            false,
        predecessor_resulting_zone_dehumidifying_setpoint_moisture_demand_kg_per_s: scalar,
        dehumidification_control_none_case_completed_skip: !humidistat,
        dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: false,
        dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_assignment_executed:
            humidistat,
        dehumidification_control_constant_supply_humidity_ratio_case_selected_skip: false,
        zone_dehumidifying_setpoint_moisture_demand_read: humidistat,
        zone_dehumidifying_setpoint_moisture_demand_kg_per_s: scalar,
        supply_mass_flow_rate_read: humidistat,
        supply_mass_flow_rate_kg_per_s: scalar,
        moisture_demand_derived_supply_humidity_ratio_calculated: humidistat,
        moisture_demand_derived_supply_humidity_ratio: scalar,
        zone_node_humidity_ratio_read: humidistat,
        zone_node_humidity_ratio: scalar,
        supply_humidity_ratio_for_dehumidification_calculated: humidistat,
        calculated_supply_humidity_ratio_for_dehumidification: scalar,
        supply_humidity_ratio_for_dehumidification_assigned: humidistat,
        assigned_supply_humidity_ratio_for_dehumidification: scalar,
        resulting_supply_humidity_ratio_for_dehumidification: scalar,
    }
}

fn numeric_values(
    snapshot: &PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitSnapshot,
) -> [Option<f64>; 6] {
    [
        snapshot.predecessor_resulting_supply_humidity_ratio_for_dehumidification,
        snapshot.supply_humidity_ratio_for_dehumidification_before_minimum_limit,
        snapshot.minimum_cooling_supply_air_humidity_ratio,
        snapshot.maximum_supply_humidity_ratio_for_dehumidification,
        snapshot.assigned_supply_humidity_ratio_for_dehumidification,
        snapshot.resulting_supply_humidity_ratio_for_dehumidification,
    ]
}

fn set_numeric(
    snapshot: &mut PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitSnapshot,
    field: usize,
    value: Option<f64>,
) {
    match field {
        0 => snapshot.predecessor_resulting_supply_humidity_ratio_for_dehumidification = value,
        1 => snapshot.supply_humidity_ratio_for_dehumidification_before_minimum_limit = value,
        2 => snapshot.minimum_cooling_supply_air_humidity_ratio = value,
        3 => snapshot.maximum_supply_humidity_ratio_for_dehumidification = value,
        4 => snapshot.assigned_supply_humidity_ratio_for_dehumidification = value,
        5 => snapshot.resulting_supply_humidity_ratio_for_dehumidification = value,
        _ => unreachable!(),
    }
}
