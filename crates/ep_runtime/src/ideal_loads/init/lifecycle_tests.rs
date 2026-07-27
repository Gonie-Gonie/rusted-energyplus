use ep_model::{
    AutosizeOrNumber, DehumidificationControlType, DemandControlledVentilationType,
    HeatRecoveryType, HumidificationControlType, IdealLoadsAirSystem, IdealLoadsAirSystemId,
    IdealLoadsFuelType, IdealLoadsLimit, NodeId, NormalizedName, OutdoorAirEconomizerType,
    ZoneEquipmentListId, ZoneId,
};

use super::super::{
    PurchasedAirHardSizeLegacyContext, PurchasedAirHardSizeLegacyError,
    PurchasedAirHardSizeLegacyRoute,
};
use super::*;

pub(super) const SYSTEM: IdealLoadsAirSystemId = IdealLoadsAirSystemId(0);

#[test]
fn first_call_runs_source_order_and_caches_environment_limits() {
    let system = finite_flow_system();
    let plan = single_manager_plan();
    let mut state = PurchasedAirRuntimeState::default();
    let snapshot =
        init_purchased_air_runtime(&mut state, &plan, &topology(), &system, context(true))
            .expect("hard-sized direct-Zone initialization");
    assert_eq!(
        snapshot.transition,
        PurchasedAirInitTransition {
            module_initialized: true,
            equipment_list_checked: true,
            equipment_list_units_scanned: 1,
            one_time_initialized: true,
            topology_completed: true,
            sizing_checked: true,
            environment_initialized: true,
            ..PurchasedAirInitTransition::default()
        }
    );
    assert!(snapshot.flags.state_machine_used);
    assert!(snapshot.flags.one_time_checked);
    assert!(snapshot.flags.topology_ready);
    assert!(snapshot.flags.environment_initialized);
    assert!(!snapshot.flags.environment_initialization_needed);
    assert!(snapshot.flags.sizing_checked);
    assert!(snapshot.flags.equipment_list_checked);
    assert!(snapshot.flags.return_plenum_inactive);
    assert_close(snapshot.maximum_heating_air_mass_flow_rate_kg_per_s, 0.6);
    assert_close(snapshot.maximum_cooling_air_mass_flow_rate_kg_per_s, 0.48);
    assert_eq!(snapshot.standard_air_density_kg_per_m3, Some(1.2));

    let summary = purchased_air_init_lifecycle_summary(&state, SYSTEM)
        .expect("initialized lifecycle summary");
    assert_eq!(summary.module_initialization_count, 1);
    assert_eq!(summary.equipment_list_check_count, 1);
    assert_eq!(summary.declared_system_order, vec![SYSTEM]);
    assert_eq!(summary.equipment_list_scan_order, vec![SYSTEM]);
    assert_eq!(summary.equipment_list_scanned_unit_count, 1);
    assert_eq!(summary.equipment_list_missing_unit_count, 0);
    assert!(summary.equipment_list_diagnostics.is_empty());
    assert_eq!(summary.equipment_list_scan_ordinal, Some(1));
    assert_eq!(
        summary.first_matching_equipment_list,
        Some(ZoneEquipmentListId(0))
    );
    assert_eq!(summary.equipment_list_membership_found, Some(true));
    assert_eq!(summary.controlled_zone, Some(ZoneId(0)));
    assert_eq!(summary.equipment_list, Some(ZoneEquipmentListId(0)));
    assert_eq!(summary.supply_node, Some(NodeId(1)));
    assert_eq!(summary.recirculation_node, Some(NodeId(2)));
    assert_eq!(
        summary.recirculation_source,
        Some(PurchasedAirRecirculationSource::SingleZoneReturn)
    );
    assert!(summary.topology_diagnostics.is_empty());
    assert_eq!(summary.topology_failure, None);
    assert_eq!(summary.init_call_count, 1);
    assert_eq!(summary.one_time_initialization_count, 1);
    assert_eq!(summary.topology_completion_count, 1);
    assert_eq!(summary.sizing_attempt_count, 1);
    assert_eq!(summary.sizing_check_count, 1);
    assert_eq!(
        summary.sizing_outcome.map(|outcome| outcome.route),
        Some(PurchasedAirHardSizeLegacyRoute::DirectHardSizedNoSizingRun)
    );
    assert_eq!(summary.sized_limits, Some(snapshot.sized_limits));
    assert_eq!(summary.environment_initialization_count, 1);
    assert_eq!(summary.environment_rearm_count, 0);
}

#[test]
fn environment_latch_rearms_and_recomputes_on_the_next_environment() {
    let system = finite_flow_system();
    let plan = single_manager_plan();
    let mut state = PurchasedAirRuntimeState::default();
    init_purchased_air_runtime(&mut state, &plan, &topology(), &system, context(true))
        .expect("first begin environment");
    let stable = init_purchased_air_runtime(&mut state, &plan, &topology(), &system, context(true))
        .expect("same begin environment");
    assert_eq!(stable.transition, PurchasedAirInitTransition::default());

    let rearmed =
        init_purchased_air_runtime(&mut state, &plan, &topology(), &system, context(false))
            .expect("leave begin environment");
    assert!(rearmed.transition.environment_rearmed);
    assert!(rearmed.flags.environment_initialized);
    assert!(rearmed.flags.environment_initialization_needed);

    let mut next_environment = context(true);
    next_environment.standard_air_density_kg_per_m3 = 1.1;
    let recomputed =
        init_purchased_air_runtime(&mut state, &plan, &topology(), &system, next_environment)
            .expect("next begin environment");
    assert!(recomputed.transition.environment_initialized);
    assert_close(recomputed.maximum_heating_air_mass_flow_rate_kg_per_s, 0.55);
    assert_eq!(recomputed.standard_air_density_kg_per_m3, Some(1.1));

    let summary = purchased_air_init_lifecycle_summary(&state, SYSTEM)
        .expect("reinitialized lifecycle summary");
    assert_eq!(summary.init_call_count, 4);
    assert_eq!(summary.environment_initialization_count, 2);
    assert_eq!(summary.environment_rearm_count, 1);
}

#[test]
fn environment_reinitialization_uses_the_persistent_sizing_overlay() {
    let system = finite_flow_system();
    let plan = single_manager_plan();
    let mut state = PurchasedAirRuntimeState::default();
    init_purchased_air_runtime(&mut state, &plan, &topology(), &system, context(true))
        .expect("initial hard-size and environment pass");
    init_purchased_air_runtime(&mut state, &plan, &topology(), &system, context(false))
        .expect("environment latch rearm");

    let mut changed_model_view = system.clone();
    changed_model_view.maximum_heating_air_flow_rate_m3_per_s = Some(AutosizeOrNumber::Value(0.9));
    changed_model_view.maximum_cooling_air_flow_rate_m3_per_s = Some(AutosizeOrNumber::Value(0.8));
    let mut next_environment = context(true);
    next_environment.standard_air_density_kg_per_m3 = 1.0;
    let snapshot = init_purchased_air_runtime(
        &mut state,
        &plan,
        &topology(),
        &changed_model_view,
        next_environment,
    )
    .expect("retained sizing overlay drives the next environment");

    assert_close(snapshot.maximum_heating_air_mass_flow_rate_kg_per_s, 0.5);
    assert_close(snapshot.maximum_cooling_air_mass_flow_rate_kg_per_s, 0.4);
    assert_eq!(
        snapshot.sized_limits.maximum_heating_air_flow_rate_m3_per_s,
        Some(AutosizeOrNumber::Value(0.5))
    );
    assert_eq!(
        snapshot.sized_limits.maximum_cooling_air_flow_rate_m3_per_s,
        Some(AutosizeOrNumber::Value(0.4))
    );
    let summary = purchased_air_init_lifecycle_summary(&state, SYSTEM).expect("lifecycle summary");
    assert_eq!(summary.sizing_attempt_count, 1);
    assert_eq!(summary.sizing_check_count, 1);
}

#[test]
fn deferred_gates_replay_topology_and_invalid_density_fail_closed() {
    let system = finite_flow_system();
    let plan = single_manager_plan();
    let mut state = PurchasedAirRuntimeState::default();
    let mut deferred = context(false);
    deferred.zone_equipment_inputs_filled = false;
    deferred.system_sizing_calculation = true;
    let first = init_purchased_air_runtime(&mut state, &plan, &topology(), &system, deferred)
        .expect("deferred source gates");
    assert_eq!(
        first.transition,
        PurchasedAirInitTransition {
            module_initialized: true,
            one_time_initialized: true,
            topology_completed: true,
            ..PurchasedAirInitTransition::default()
        }
    );
    assert!(!first.flags.equipment_list_checked);
    assert!(!first.flags.sizing_checked);
    assert!(!first.flags.environment_initialized);

    let ready = init_purchased_air_runtime(&mut state, &plan, &topology(), &system, context(true))
        .expect("deferred checks complete on replay");
    assert!(ready.transition.equipment_list_checked);
    assert!(ready.transition.sizing_checked);
    assert!(ready.transition.environment_initialized);
    let cached = (
        ready.maximum_heating_air_mass_flow_rate_kg_per_s,
        ready.maximum_cooling_air_mass_flow_rate_kg_per_s,
    );

    let changed = topology_with_supply(NodeId(99));
    assert_eq!(
        init_purchased_air_runtime(&mut state, &plan, &changed, &system, context(true),),
        Err(PurchasedAirInitError::LatchedTopologyChanged { system: SYSTEM })
    );

    init_purchased_air_runtime(&mut state, &plan, &topology(), &system, context(false))
        .expect("environment latch rearm");
    let mut invalid = context(true);
    invalid.standard_air_density_kg_per_m3 = f64::NAN;
    assert!(matches!(
        init_purchased_air_runtime(&mut state, &plan, &topology(), &system, invalid),
        Err(PurchasedAirInitError::InvalidStandardAirDensity { value }) if value.is_nan()
    ));
    let unit = &state.units[&SYSTEM];
    assert!(unit.environment_initialization_needed);
    assert_eq!(unit.environment_initialization_count, 1);
    assert_eq!(
        (
            unit.maximum_heating_air_mass_flow_rate_kg_per_s,
            unit.maximum_cooling_air_mass_flow_rate_kg_per_s,
        ),
        cached
    );
    assert_eq!(unit.standard_air_density_kg_per_m3, Some(1.2));
}

#[test]
fn unported_sizing_routes_leave_the_size_latch_armed() {
    let plan = single_manager_plan();

    let mut custom_system = finite_flow_system();
    custom_system.design_specification_zonehvac_sizing_object_name =
        Some(NormalizedName::new("CUSTOM SIZING"));
    let mut custom_state = PurchasedAirRuntimeState::default();
    assert_eq!(
        init_purchased_air_runtime(
            &mut custom_state,
            &plan,
            &topology(),
            &custom_system,
            context(true),
        ),
        Err(PurchasedAirInitError::Sizing(
            PurchasedAirHardSizeLegacyError::CustomZoneHvacSizingNotImplemented { system: SYSTEM }
        ))
    );
    let custom_unit = &custom_state.units[&SYSTEM];
    assert!(custom_unit.sizing_needed);
    assert_eq!(custom_unit.sizing_attempt_count, 1);
    assert_eq!(custom_unit.sizing_check_count, 0);
    assert_eq!(custom_unit.environment_initialization_count, 0);

    let system = finite_flow_system();
    let mut zone_sizing_state = PurchasedAirRuntimeState::default();
    let mut zone_sizing_context = context(true);
    zone_sizing_context.sizing.zone_sizing_run_done = true;
    assert_eq!(
        init_purchased_air_runtime(
            &mut zone_sizing_state,
            &plan,
            &topology(),
            &system,
            zone_sizing_context,
        ),
        Err(PurchasedAirInitError::Sizing(
            PurchasedAirHardSizeLegacyError::ZoneSizingRunNotImplemented { system: SYSTEM }
        ))
    );
    let zone_sizing_unit = &zone_sizing_state.units[&SYSTEM];
    assert!(zone_sizing_unit.sizing_needed);
    assert_eq!(zone_sizing_unit.sizing_attempt_count, 1);
    assert_eq!(zone_sizing_unit.sizing_check_count, 0);
    assert_eq!(zone_sizing_unit.environment_initialization_count, 0);
}

#[test]
fn absent_current_zone_equipment_completes_the_source_suppression_path() {
    let system = finite_flow_system();
    let plan = single_manager_plan();
    let mut state = PurchasedAirRuntimeState::default();
    let mut call = context(true);
    call.sizing.current_zone_equipment_index = 0;

    let snapshot = init_purchased_air_runtime(&mut state, &plan, &topology(), &system, call)
        .expect("source outer condition returns normally");

    assert!(snapshot.transition.sizing_checked);
    assert!(snapshot.transition.environment_initialized);
    assert_eq!(
        snapshot.sizing_outcome.map(|outcome| outcome.route),
        Some(PurchasedAirHardSizeLegacyRoute::NoCurrentZoneEquipment)
    );
    let unit = &state.units[&SYSTEM];
    assert!(!unit.sizing_needed);
    assert_eq!(unit.sizing_attempt_count, 1);
    assert_eq!(unit.sizing_check_count, 1);
}

pub(super) fn single_manager_plan() -> PurchasedAirInitManagerPlan {
    PurchasedAirInitManagerPlan::try_from_rows(vec![PurchasedAirInitManagerPlanRow {
        system: SYSTEM,
        first_matching_equipment_list: Some(ZoneEquipmentListId(0)),
        return_plenum_active: false,
    }])
    .expect("valid single-unit test manager plan")
}

pub(super) fn topology() -> PurchasedAirInitTopologyPlan {
    topology_with_supply(NodeId(1))
}

fn topology_with_supply(supply_node: NodeId) -> PurchasedAirInitTopologyPlan {
    PurchasedAirInitTopologyPlan::from_resolved_nodes(
        SYSTEM,
        ZoneId(0),
        ZoneEquipmentListId(0),
        supply_node,
        vec![supply_node],
        None,
        Vec::new(),
        vec![NodeId(2)],
        false,
    )
}

pub(super) fn context(begin_environment: bool) -> PurchasedAirInitCallContext {
    PurchasedAirInitCallContext {
        zone_equipment_inputs_filled: true,
        system_sizing_calculation: false,
        sizing: PurchasedAirHardSizeLegacyContext {
            current_zone_equipment_index: 1,
            zone_sizing_run_done: false,
        },
        begin_environment,
        standard_air_density_kg_per_m3: 1.2,
        heating_setpoint_c: 20.0,
        cooling_setpoint_c: 22.0,
        overall_availability: 1.0,
        heating_availability: 1.0,
        cooling_availability: 1.0,
    }
}

pub(super) fn finite_flow_system() -> IdealLoadsAirSystem {
    IdealLoadsAirSystem {
        id: SYSTEM,
        name: NormalizedName::new("ZONE ONE IDEAL LOADS"),
        availability_schedule: None,
        zone_supply_air_node_name: NormalizedName::new("ZONE ONE INLET"),
        zone_exhaust_air_node_name: None,
        system_inlet_air_node_name: None,
        maximum_heating_supply_air_temperature_c: 50.0,
        minimum_cooling_supply_air_temperature_c: 13.0,
        maximum_heating_supply_air_humidity_ratio: 0.0156,
        minimum_cooling_supply_air_humidity_ratio: 0.0077,
        heating_limit: IdealLoadsLimit::LimitFlowRateAndCapacity,
        maximum_heating_air_flow_rate_m3_per_s: Some(AutosizeOrNumber::Value(0.5)),
        maximum_sensible_heating_capacity_w: Some(AutosizeOrNumber::Value(5_000.0)),
        cooling_limit: IdealLoadsLimit::LimitFlowRateAndCapacity,
        maximum_cooling_air_flow_rate_m3_per_s: Some(AutosizeOrNumber::Value(0.4)),
        maximum_total_cooling_capacity_w: Some(AutosizeOrNumber::Value(4_000.0)),
        heating_availability_schedule: None,
        cooling_availability_schedule: None,
        dehumidification_control_type: DehumidificationControlType::None,
        cooling_sensible_heat_ratio: 0.7,
        humidification_control_type: HumidificationControlType::None,
        design_specification_outdoor_air_object_name: None,
        outdoor_air_inlet_node_name: None,
        demand_controlled_ventilation_type: DemandControlledVentilationType::None,
        outdoor_air_economizer_type: OutdoorAirEconomizerType::NoEconomizer,
        heat_recovery_type: HeatRecoveryType::None,
        sensible_heat_recovery_effectiveness: 0.7,
        latent_heat_recovery_effectiveness: 0.65,
        design_specification_zonehvac_sizing_object_name: None,
        heating_fuel_efficiency_schedule: None,
        heating_fuel_type: IdealLoadsFuelType::DistrictHeatingWater,
        cooling_fuel_efficiency_schedule: None,
        cooling_fuel_type: IdealLoadsFuelType::DistrictCooling,
    }
}

fn assert_close(actual: f64, expected: f64) {
    assert!((actual - expected).abs() <= 1.0e-12);
}
