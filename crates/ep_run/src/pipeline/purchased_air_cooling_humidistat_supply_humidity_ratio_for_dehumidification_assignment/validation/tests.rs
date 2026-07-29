use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId, ZoneId};
use ep_runtime::{
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_MOISTURE_DEMAND_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_MOISTURE_DEMAND_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_MOISTURE_DEMAND_ASSIGNMENT_SOURCE_ORDER,
};

use super::*;

#[test]
fn missing_direct_lifecycle_fails_closed() {
    assert!(
        validate_direct_lifecycle(
            None,
            DirectLifecyclePredecessors {
                moisture_demand_assignment_cp359: None,
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
        PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentRuntimeState::new(
            IdealLoadsAirSystemId(0),
        );
    state.unit_off_skip_count = usize::MAX;
    state.non_cooling_skip_count = 1;
    assert!(validate_route_partition(&state).is_err());
}

#[test]
fn six_site_counters_are_exact_and_fail_closed_on_each_mismatch() {
    let valid = active_state();
    assert!(validate_source_counters(&valid).is_ok());

    for field in [
        "source_sites",
        "moisture_demand_read",
        "mass_flow_read",
        "division",
        "node_humidity_read",
        "addition",
        "assignment",
    ] {
        let mut state = valid.clone();
        match field {
            "source_sites" => state.source_site_execution_count = 5,
            "moisture_demand_read" => {
                state.zone_dehumidifying_setpoint_moisture_demand_read_count = 0
            }
            "mass_flow_read" => state.supply_mass_flow_rate_read_count = 0,
            "division" => state.moisture_demand_derived_supply_humidity_ratio_calculation_count = 0,
            "node_humidity_read" => state.zone_node_humidity_ratio_read_count = 0,
            "addition" => state.supply_humidity_ratio_for_dehumidification_calculation_count = 0,
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
            .dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_assignment_executed
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
    for scalar in [-0.0, f64::INFINITY, f64::from_bits(0x7ff8_0000_0000_0042)] {
        let expected = expected_snapshot(predecessor_snapshot(Some(scalar), true));
        assert_eq!(
            expected
                .predecessor_resulting_zone_dehumidifying_setpoint_moisture_demand_kg_per_s
                .map(f64::to_bits),
            Some(scalar.to_bits())
        );
        assert!(snapshots_match_exact_bits(&expected, &expected));
    }
}

fn active_state()
-> PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentRuntimeState {
    let mut state =
        PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentRuntimeState::new(
            IdealLoadsAirSystemId(0),
        );
    state
        .dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_assignment_count =
        1;
    state.source_site_execution_count = 6;
    state.zone_dehumidifying_setpoint_moisture_demand_read_count = 1;
    state.supply_mass_flow_rate_read_count = 1;
    state.moisture_demand_derived_supply_humidity_ratio_calculation_count = 1;
    state.zone_node_humidity_ratio_read_count = 1;
    state.supply_humidity_ratio_for_dehumidification_calculation_count = 1;
    state.supply_humidity_ratio_for_dehumidification_assignment_count = 1;
    state
}

fn predecessor_snapshot(
    scalar: Option<f64>,
    humidistat: bool,
) -> PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentSnapshot {
    PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_MOISTURE_DEMAND_ASSIGNMENT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_MOISTURE_DEMAND_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_MOISTURE_DEMAND_ASSIGNMENT_SOURCE_ORDER,
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
        predecessor_dehumidification_control_humidistat_case_entered: humidistat,
        predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
            false,
        dehumidification_control_none_case_completed_skip: !humidistat,
        dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: false,
        dehumidification_control_humidistat_moisture_demand_assignment_executed: humidistat,
        dehumidification_control_constant_supply_humidity_ratio_case_selected_skip: false,
        zone_dehumidifying_setpoint_moisture_demand_read: humidistat,
        zone_dehumidifying_setpoint_moisture_demand_kg_per_s: scalar,
        zone_dehumidifying_setpoint_moisture_demand_assigned: humidistat,
        assigned_zone_dehumidifying_setpoint_moisture_demand_kg_per_s: scalar,
        resulting_zone_dehumidifying_setpoint_moisture_demand_kg_per_s: scalar,
    }
}

fn numeric_values(
    snapshot: &PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentSnapshot,
) -> [Option<f64>; 8] {
    [
        snapshot.predecessor_resulting_zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
        snapshot.zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
        snapshot.supply_mass_flow_rate_kg_per_s,
        snapshot.moisture_demand_derived_supply_humidity_ratio,
        snapshot.zone_node_humidity_ratio,
        snapshot.calculated_supply_humidity_ratio_for_dehumidification,
        snapshot.assigned_supply_humidity_ratio_for_dehumidification,
        snapshot.resulting_supply_humidity_ratio_for_dehumidification,
    ]
}

fn set_numeric(
    snapshot: &mut PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentSnapshot,
    field: usize,
    value: Option<f64>,
) {
    match field {
        0 => {
            snapshot.predecessor_resulting_zone_dehumidifying_setpoint_moisture_demand_kg_per_s =
                value
        }
        1 => snapshot.zone_dehumidifying_setpoint_moisture_demand_kg_per_s = value,
        2 => snapshot.supply_mass_flow_rate_kg_per_s = value,
        3 => snapshot.moisture_demand_derived_supply_humidity_ratio = value,
        4 => snapshot.zone_node_humidity_ratio = value,
        5 => snapshot.calculated_supply_humidity_ratio_for_dehumidification = value,
        6 => snapshot.assigned_supply_humidity_ratio_for_dehumidification = value,
        7 => snapshot.resulting_supply_humidity_ratio_for_dehumidification = value,
        _ => unreachable!(),
    }
}
