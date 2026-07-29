use ep_model::{
    DehumidificationControlType, IdealLoadsAirSystemId, NodeId, ZoneEquipmentListId, ZoneId,
};
use ep_runtime::{
    IdealLoadsInitFlags, PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_ENTRY_SOURCE_ORDER,
    PURCHASED_AIR_INIT_LIFECYCLE_SOURCE, PurchasedAirCalcCoolingHumidistatCaseEntryRuntimeState,
    PurchasedAirHardSizeField, PurchasedAirHardSizeFieldOutcome, PurchasedAirHardSizeLegacyOutcome,
    PurchasedAirHardSizeLegacyRoute, PurchasedAirRecirculationSource, PurchasedAirSizedLimits,
};

use super::*;

#[derive(Clone, Copy, Debug)]
enum Route {
    UnitOff,
    NonCooling,
    PositiveGuardFalse,
    NoneCase,
    ConstantShr,
    Humidistat,
    ConstantSupplyHumidityRatio,
}

#[test]
fn route_partition_overflow_fails_closed() {
    assert!(checked_sum(&[usize::MAX, 1]).is_err());
}

#[test]
fn source_and_read_assignment_counter_mismatches_fail_closed() {
    let mut state = PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentRuntimeState::new(
        IdealLoadsAirSystemId(0),
    );
    state.source_site_execution_count = 1;
    assert!(validate_source_counters(&state).is_err());
    state.source_site_execution_count = 0;
    state.zone_dehumidifying_setpoint_moisture_demand_read_count = 1;
    assert!(validate_source_counters(&state).is_err());
    state.zone_dehumidifying_setpoint_moisture_demand_read_count = 0;
    state.zone_dehumidifying_setpoint_moisture_demand_assignment_count = 1;
    assert!(validate_source_counters(&state).is_err());
}

#[test]
fn inherited_u_n_p_and_c0_routes_validate() {
    let init = valid_init_lifecycle();
    for route in [
        Route::UnitOff,
        Route::NonCooling,
        Route::PositiveGuardFalse,
        Route::NoneCase,
    ] {
        let (lifecycle, predecessor) = lifecycles(route);
        let result = validate(&lifecycle, &predecessor, &init);
        assert!(result.is_ok(), "{route:?}: {result:?}");
    }
}

#[test]
fn self_consistent_q_h_and_csh_routes_are_rejected() {
    let init = valid_init_lifecycle();
    for route in [
        Route::ConstantShr,
        Route::Humidistat,
        Route::ConstantSupplyHumidityRatio,
    ] {
        let (lifecycle, predecessor) = lifecycles(route);
        let result = validate(&lifecycle, &predecessor, &init);
        assert!(result.is_err(), "{route:?} must be rejected");
    }
}

#[test]
fn snapshot_identity_route_and_numeric_corruptions_fail_closed() {
    let init = valid_init_lifecycle();
    let (valid, predecessor) = lifecycles(Route::NoneCase);
    for field in [
        "source",
        "first_excluded_source",
        "system",
        "parent_call_ordinal",
        "controlled_zone",
        "route",
        "read",
        "assigned",
        "raw",
        "assigned_value",
        "resulting_value",
    ] {
        let mut corrupted = valid.clone();
        let Some(snapshot) = corrupted.state.latest.as_mut() else {
            return;
        };
        match field {
            "source" => snapshot.source = "forged-cp359-source",
            "first_excluded_source" => {
                snapshot.first_excluded_source = "forged-cp359-first-excluded"
            }
            "system" => snapshot.system = IdealLoadsAirSystemId(1),
            "parent_call_ordinal" => snapshot.parent_call_ordinal = 2,
            "controlled_zone" => snapshot.controlled_zone = ZoneId(1),
            "route" => snapshot.dehumidification_control_none_case_completed_skip = false,
            "read" => snapshot.zone_dehumidifying_setpoint_moisture_demand_read = true,
            "assigned" => snapshot.zone_dehumidifying_setpoint_moisture_demand_assigned = true,
            "raw" => snapshot.zone_dehumidifying_setpoint_moisture_demand_kg_per_s = Some(-0.0),
            "assigned_value" => {
                snapshot.assigned_zone_dehumidifying_setpoint_moisture_demand_kg_per_s =
                    Some(f64::NAN)
            }
            "resulting_value" => {
                snapshot.resulting_zone_dehumidifying_setpoint_moisture_demand_kg_per_s =
                    Some(f64::INFINITY)
            }
            _ => unreachable!(),
        }
        assert!(
            validate(&corrupted, &predecessor, &init).is_err(),
            "{field}"
        );
    }
}

fn validate(
    lifecycle: &PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentLifecycleSummary,
    predecessor: &PurchasedAirCalcCoolingHumidistatCaseEntryLifecycleSummary,
    init: &PurchasedAirInitLifecycleSummary,
) -> Result<(), String> {
    validate_direct_lifecycle(
        Some(lifecycle),
        DirectLifecyclePredecessors {
            humidistat_case_entry_cp358: Some(predecessor),
        },
        Some(init),
        Some(1),
    )
}

fn lifecycles(
    route: Route,
) -> (
    PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentLifecycleSummary,
    PurchasedAirCalcCoolingHumidistatCaseEntryLifecycleSummary,
) {
    let system = IdealLoadsAirSystemId(0);
    let zone = ZoneId(0);
    let unit_off = matches!(route, Route::UnitOff);
    let non_cooling = matches!(route, Route::NonCooling);
    let positive_guard_false = matches!(route, Route::PositiveGuardFalse);
    let none_case = matches!(route, Route::NoneCase);
    let constant_shr = matches!(route, Route::ConstantShr);
    let humidistat = matches!(route, Route::Humidistat);
    let constant_supply = matches!(route, Route::ConstantSupplyHumidityRatio);
    let active = none_case || constant_shr || humidistat || constant_supply;
    let control = match route {
        Route::NoneCase => Some(DehumidificationControlType::None),
        Route::ConstantShr => Some(DehumidificationControlType::ConstantSensibleHeatRatio),
        Route::Humidistat => Some(DehumidificationControlType::Humidistat),
        Route::ConstantSupplyHumidityRatio => {
            Some(DehumidificationControlType::ConstantSupplyHumidityRatio)
        }
        Route::UnitOff | Route::NonCooling | Route::PositiveGuardFalse => None,
    };
    let predecessor_latest = PurchasedAirCalcCoolingHumidistatCaseEntrySnapshot {
        source: PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_ENTRY_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_ENTRY_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_ENTRY_SOURCE_ORDER,
        system,
        parent_call_ordinal: 1,
        controlled_zone: zone,
        unit_body_entered: !unit_off,
        predecessor_cooling_body_entered: !unit_off && !non_cooling,
        predecessor_no_outdoor_air_fallback_entered: positive_guard_false || active,
        predecessor_positive_supply_mass_flow_body_entered: active,
        unit_off_skipped: unit_off,
        non_cooling_skipped: non_cooling,
        positive_guard_false_fallthrough_skipped: positive_guard_false,
        predecessor_dehumidification_control_type: control,
        predecessor_dehumidification_control_none_case_completed_skip: none_case,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break:
            constant_shr,
        predecessor_dehumidification_control_humidistat_case_selected_skip: humidistat,
        predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
            constant_supply,
        dehumidification_control_none_case_completed_skip: none_case,
        dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: constant_shr,
        dehumidification_control_humidistat_case_entered: humidistat,
        dehumidification_control_constant_supply_humidity_ratio_case_selected_skip: constant_supply,
    };
    let mut predecessor_state = PurchasedAirCalcCoolingHumidistatCaseEntryRuntimeState::new(system);
    predecessor_state.transition_count = 1;
    predecessor_state.unit_off_skip_count = usize::from(unit_off);
    predecessor_state.non_cooling_skip_count = usize::from(non_cooling);
    predecessor_state.positive_guard_false_fallthrough_skip_count =
        usize::from(positive_guard_false);
    predecessor_state.dehumidification_control_none_case_completed_skip_count =
        usize::from(none_case);
    predecessor_state
        .dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count =
        usize::from(constant_shr);
    predecessor_state.dehumidification_control_humidistat_case_entry_count =
        usize::from(humidistat);
    predecessor_state
        .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count =
        usize::from(constant_supply);
    predecessor_state.source_site_execution_count = usize::from(humidistat);
    predecessor_state.latest = Some(predecessor_latest);
    let predecessor = PurchasedAirCalcCoolingHumidistatCaseEntryLifecycleSummary {
        source: PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_ENTRY_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_CASE_ENTRY_FIRST_EXCLUDED_SOURCE,
        state: predecessor_state,
    };

    let latest = expected_snapshot(predecessor_latest);
    let mut state =
        PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentRuntimeState::new(system);
    state.transition_count = 1;
    state.unit_off_skip_count = usize::from(unit_off);
    state.non_cooling_skip_count = usize::from(non_cooling);
    state.positive_guard_false_fallthrough_skip_count = usize::from(positive_guard_false);
    state.dehumidification_control_none_case_completed_skip_count = usize::from(none_case);
    state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count =
        usize::from(constant_shr);
    state.dehumidification_control_humidistat_moisture_demand_assignment_count =
        usize::from(humidistat);
    state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count =
        usize::from(constant_supply);
    state.source_site_execution_count = 2 * usize::from(humidistat);
    state.zone_dehumidifying_setpoint_moisture_demand_read_count = usize::from(humidistat);
    state.zone_dehumidifying_setpoint_moisture_demand_assignment_count = usize::from(humidistat);
    state.latest = Some(latest);
    (
        PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentLifecycleSummary {
            source: PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_MOISTURE_DEMAND_ASSIGNMENT_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_MOISTURE_DEMAND_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
            state,
        },
        predecessor,
    )
}

fn valid_init_lifecycle() -> PurchasedAirInitLifecycleSummary {
    let sized_limits = PurchasedAirSizedLimits {
        maximum_heating_air_flow_rate_m3_per_s: None,
        maximum_sensible_heating_capacity_w: None,
        maximum_cooling_air_flow_rate_m3_per_s: None,
        maximum_total_cooling_capacity_w: None,
    };
    let skipped_field = |field| {
        Some(PurchasedAirHardSizeFieldOutcome {
            field,
            input_value: None,
            child_sizer_called: false,
            child_result: None,
            object_writeback: false,
            local_design_value: 0.0,
            child_user_report_records: 0,
            outer_report_records: 0,
            child_sizing_label_unit: "m3/s",
        })
    };
    PurchasedAirInitLifecycleSummary {
        source: PURCHASED_AIR_INIT_LIFECYCLE_SOURCE,
        flags: IdealLoadsInitFlags {
            state_machine_used: true,
            one_time_checked: true,
            topology_ready: true,
            environment_initialized: true,
            environment_initialization_needed: false,
            sizing_checked: true,
            equipment_list_checked: true,
            return_plenum_inactive: true,
        },
        module_initialization_count: 1,
        equipment_list_check_count: 1,
        declared_system_order: vec![IdealLoadsAirSystemId(0)],
        equipment_list_scan_order: vec![IdealLoadsAirSystemId(0)],
        equipment_list_scanned_unit_count: 1,
        equipment_list_missing_unit_count: 0,
        equipment_list_diagnostics: Vec::new(),
        equipment_list_scan_ordinal: Some(1),
        first_matching_equipment_list: Some(ZoneEquipmentListId(0)),
        equipment_list_membership_found: Some(true),
        controlled_zone: Some(ZoneId(0)),
        equipment_list: Some(ZoneEquipmentListId(0)),
        supply_node: Some(NodeId(3)),
        recirculation_node: Some(NodeId(4)),
        recirculation_source: Some(PurchasedAirRecirculationSource::SingleZoneReturn),
        rejected_exhaust_node: None,
        reported_first_return_node: None,
        topology_diagnostics: Vec::new(),
        topology_failure: None,
        init_call_count: 1,
        one_time_initialization_count: 1,
        topology_completion_count: 1,
        sizing_attempt_count: 1,
        sizing_check_count: 1,
        sized_limits: Some(sized_limits),
        sizing_outcome: Some(PurchasedAirHardSizeLegacyOutcome {
            route: PurchasedAirHardSizeLegacyRoute::DirectHardSizedNoSizingRun,
            sized_limits,
            fields: [
                skipped_field(PurchasedAirHardSizeField::MaximumHeatingAirFlowRate),
                skipped_field(PurchasedAirHardSizeField::MaximumSensibleHeatingCapacity),
                skipped_field(PurchasedAirHardSizeField::MaximumCoolingAirFlowRate),
                skipped_field(PurchasedAirHardSizeField::MaximumTotalCoolingCapacity),
            ],
            entry_fan_flags_cleared: true,
        }),
        environment_initialization_count: 1,
        environment_rearm_count: 0,
        maximum_heating_air_mass_flow_rate_kg_per_s: 0.0,
        maximum_cooling_air_mass_flow_rate_kg_per_s: 0.0,
        standard_air_density_kg_per_m3: Some(1.2),
        supply_temperature_registered_recurring_diagnostic_count: 0,
        supply_temperature_diagnostic_event_count: 0,
        supply_temperature_characterized_severe_error_count_increment: 0,
        cooling_supply_temperature_error_index: 0,
        heating_supply_temperature_error_index: 0,
        cooling_supply_temperature_first_diagnostic_count: 0,
        heating_supply_temperature_first_diagnostic_count: 0,
        supply_temperature_diagnostics: Vec::new(),
        cooling_supply_temperature_warning_count: 0,
        heating_supply_temperature_warning_count: 0,
        economizer_flow_limit_warning_count: 0,
    }
}
