use ep_model::{
    DehumidificationControlType, DemandControlledVentilationType, HeatRecoveryType,
    HumidificationControlType, IdealLoadsAirSystem, IdealLoadsAirSystemId, IdealLoadsFuelType,
    IdealLoadsLimit, NodeId, NormalizedName, OutdoorAirEconomizerType, ZoneEquipmentListId, ZoneId,
};

use crate::{
    ideal_loads::{
        PurchasedAirCalcCoolingEconomizerConditionError, PurchasedAirCalcEntryContext,
        PurchasedAirHardSizeLegacyContext, PurchasedAirInitCallContext,
        PurchasedAirInitManagerPlan, PurchasedAirInitManagerPlanRow, PurchasedAirInitTopologyPlan,
        PurchasedAirRuntimeState, PurchasedAirTemperatureControlType,
        advance_direct_no_oa_calc_cooling_economizer_condition, init_purchased_air_runtime,
    },
    zone_equipment::ZoneSysEnergyDemand,
};

use super::{
    cooling_economizer_guard::{
        PurchasedAirCalcCoolingEconomizerGuardSnapshot, advance_cooling_economizer_guard_state,
    },
    cooling_entry_gate::advance_cooling_entry_gate_state,
    cooling_oa_max_flow_body::advance_cooling_oa_max_flow_body_state,
    cooling_oa_max_flow_gate::advance_cooling_oa_max_flow_gate_state,
    lifecycle::{PurchasedAirAvailabilityStatus, advance_entry_state},
    minimum_oa_prefix::advance_minimum_oa_prefix_state,
};

const SYSTEM: IdealLoadsAirSystemId = IdealLoadsAirSystemId(5);
const ZONE: ZoneId = ZoneId(3);
const EQUIPMENT_LIST: ZoneEquipmentListId = ZoneEquipmentListId(4);
const SUPPLY_NODE: NodeId = NodeId(10);
const ZONE_NODE: NodeId = NodeId(11);
const RECIRCULATION_NODE: NodeId = NodeId(12);

mod corruption_tests;
mod provenance_tests;

#[test]
fn public_no_oa_condition_never_accepts_or_reads_node_values() {
    let (mut runtime, system, predecessor) = release_fixture();
    let snapshot =
        advance_direct_no_oa_calc_cooling_economizer_condition(&mut runtime, &system, predecessor)
            .expect("exact CP316 release transition");
    assert!(snapshot.no_economizer_outer_guard_fallthrough_skipped);
    assert!(!snapshot.economizer_condition_evaluated);
    assert!(!snapshot.differential_dry_bulb_economizer_type_read);
    assert!(!snapshot.differential_enthalpy_economizer_type_read);
    assert!(!snapshot.outdoor_air_temperature_read);
    assert!(snapshot.outdoor_air_temperature_c.is_none());
    assert!(!snapshot.outdoor_air_enthalpy_read);
    assert!(snapshot.outdoor_air_enthalpy_j_per_kg.is_none());
    assert!(!snapshot.economizer_calculation_body_entered);
    let state = &runtime
        .units
        .get(&SYSTEM)
        .expect("selected unit")
        .calc_cooling_economizer_condition;
    assert_eq!(state.transition_count, 1);
    assert_eq!(state.condition_evaluation_count, 0);
    assert_eq!(state.no_economizer_outer_guard_fallthrough_skip_count, 1);
    assert_eq!(state.outdoor_air_temperature_read_count, 0);
    assert_eq!(state.outdoor_air_enthalpy_read_count, 0);
}

#[test]
fn public_condition_rejects_forgery_replay_overflow_and_prefix_corruption_transactionally() {
    let (runtime, system, predecessor) = release_fixture();

    let mut forged_predecessor = predecessor;
    forged_predecessor.parent_call_ordinal += 1;
    assert_rejected_without_mutation(runtime.clone(), &system, forged_predecessor);

    let mut body_entry_runtime = runtime.clone();
    let forged_guard = body_entry_runtime
        .units
        .get_mut(&SYSTEM)
        .expect("selected unit")
        .calc_cooling_economizer_guard
        .latest
        .as_mut()
        .expect("retained guard");
    forged_guard.economizer_type = Some(OutdoorAirEconomizerType::DifferentialDryBulb);
    forged_guard.economizer_not_no_economizer = Some(true);
    forged_guard.economizer_body_entered = true;
    forged_guard.no_economizer_fallthrough = false;
    let supplied = *forged_guard;
    assert_rejected_without_mutation(body_entry_runtime, &system, supplied);

    let mut prefix_corruption = runtime.clone();
    prefix_corruption
        .units
        .get_mut(&SYSTEM)
        .expect("selected unit")
        .calc_cooling_oa_max_flow_gate
        .strict_mass_flow_comparison_count = usize::MAX;
    assert_rejected_without_mutation(prefix_corruption, &system, predecessor);

    let mut overflow = runtime.clone();
    overflow
        .units
        .get_mut(&SYSTEM)
        .expect("selected unit")
        .calc_cooling_economizer_condition
        .transition_count = usize::MAX;
    assert_rejected_without_mutation(overflow, &system, predecessor);

    for owner in 0..8 {
        let mut identity = runtime.clone();
        let wrong = IdealLoadsAirSystemId(SYSTEM.0 + 1);
        let unit = identity.units.get_mut(&SYSTEM).expect("selected unit");
        match owner {
            0 => unit.system = wrong,
            1 => unit.calc_entry.system = wrong,
            2 => unit.calc_minimum_oa_prefix.system = wrong,
            3 => unit.calc_cooling_entry_gate.system = wrong,
            4 => unit.calc_cooling_oa_max_flow_gate.system = wrong,
            5 => unit.calc_cooling_oa_max_flow_body.system = wrong,
            6 => unit.calc_cooling_economizer_guard.system = wrong,
            7 => unit.calc_cooling_economizer_condition.system = wrong,
            _ => unreachable!(),
        }
        assert_rejected_without_mutation(identity, &system, predecessor);
    }

    let mut replay = runtime;
    advance_direct_no_oa_calc_cooling_economizer_condition(&mut replay, &system, predecessor)
        .expect("first CP316 call");
    let before_replay = replay.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_economizer_condition(&mut replay, &system, predecessor)
            .is_err()
    );
    assert_eq!(replay, before_replay);
}

pub(super) fn release_fixture() -> (
    PurchasedAirRuntimeState,
    IdealLoadsAirSystem,
    PurchasedAirCalcCoolingEconomizerGuardSnapshot,
) {
    release_fixture_with_cooling_demand(-1.0)
}

pub(super) fn release_fixture_with_cooling_demand(
    cooling_demand_w: f64,
) -> (
    PurchasedAirRuntimeState,
    IdealLoadsAirSystem,
    PurchasedAirCalcCoolingEconomizerGuardSnapshot,
) {
    release_fixture_with_cooling_demand_and_availability(cooling_demand_w, 1.0)
}

pub(super) fn release_fixture_with_cooling_demand_and_availability(
    cooling_demand_w: f64,
    overall_availability: f64,
) -> (
    PurchasedAirRuntimeState,
    IdealLoadsAirSystem,
    PurchasedAirCalcCoolingEconomizerGuardSnapshot,
) {
    let system = test_system();
    let mut runtime = PurchasedAirRuntimeState::default();
    initialize_fixture_call(&mut runtime, &system, true, overall_availability);
    let guard = advance_fixture_prefix(
        &mut runtime,
        &system,
        cooling_demand_w,
        overall_availability,
    );
    (runtime, system, guard)
}

pub(super) fn advance_subsequent_fixture_call(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    cooling_demand_w: f64,
) -> PurchasedAirCalcCoolingEconomizerGuardSnapshot {
    initialize_fixture_call(runtime, system, false, 1.0);
    advance_fixture_prefix(runtime, system, cooling_demand_w, 1.0)
}

fn initialize_fixture_call(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    begin_environment: bool,
    overall_availability: f64,
) {
    init_purchased_air_runtime(
        runtime,
        &fixture_manager_plan(),
        &fixture_topology(),
        system,
        PurchasedAirInitCallContext {
            zone_equipment_inputs_filled: true,
            system_sizing_calculation: false,
            sizing: PurchasedAirHardSizeLegacyContext {
                current_zone_equipment_index: 1,
                zone_sizing_run_done: false,
            },
            begin_environment,
            standard_air_density_kg_per_m3: 1.0,
            heating_setpoint_c: 20.0,
            cooling_setpoint_c: 22.0,
            overall_availability,
            heating_availability: 1.0,
            cooling_availability: 1.0,
        },
    )
    .expect("normal exact-direct initialization fixture");
}

fn fixture_manager_plan() -> PurchasedAirInitManagerPlan {
    PurchasedAirInitManagerPlan::try_from_rows(vec![PurchasedAirInitManagerPlanRow {
        system: SYSTEM,
        first_matching_equipment_list: Some(EQUIPMENT_LIST),
        return_plenum_active: false,
    }])
    .expect("valid single-unit manager plan")
}

fn fixture_topology() -> PurchasedAirInitTopologyPlan {
    PurchasedAirInitTopologyPlan::from_resolved_nodes(
        SYSTEM,
        ZONE,
        EQUIPMENT_LIST,
        SUPPLY_NODE,
        vec![SUPPLY_NODE],
        None,
        Vec::new(),
        vec![RECIRCULATION_NODE],
        false,
    )
}

fn advance_fixture_prefix(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    cooling_demand_w: f64,
    overall_availability: f64,
) -> PurchasedAirCalcCoolingEconomizerGuardSnapshot {
    let unit = runtime.units.get_mut(&SYSTEM).expect("selected unit");
    let entry = advance_entry_state(
        &mut unit.calc_entry,
        PurchasedAirCalcEntryContext {
            controlled_zone: ZONE,
            supply_node: SUPPLY_NODE,
            zone_node: ZONE_NODE,
            outdoor_air_node: None,
            recirculation_node: RECIRCULATION_NODE,
            demand: ZoneSysEnergyDemand::from_output_required_setpoint_loads(
                ZONE,
                1.0,
                cooling_demand_w,
            ),
            zone_component_availability: Some(PurchasedAirAvailabilityStatus::NoAction),
            overall_availability,
            heating_availability: 1.0,
            cooling_availability: 1.0,
        },
    );
    let minimum = advance_minimum_oa_prefix_state(
        &mut unit.calc_entry,
        &mut unit.calc_minimum_oa_prefix,
        entry,
    );
    let cooling = advance_cooling_entry_gate_state(
        &mut unit.calc_cooling_entry_gate,
        entry,
        minimum,
        PurchasedAirTemperatureControlType::DualHeatCool,
    );
    let gate = advance_cooling_oa_max_flow_gate_state(
        &mut unit.calc_cooling_oa_max_flow_gate,
        cooling,
        system.cooling_limit,
        0.0,
        unit.maximum_cooling_air_mass_flow_rate_kg_per_s,
    );
    let body = advance_cooling_oa_max_flow_body_state(
        &mut unit.calc_cooling_oa_max_flow_body,
        gate,
        0.0,
        unit.standard_air_density_kg_per_m3
            .expect("initialized standard density"),
        0.0,
        unit.maximum_cooling_air_mass_flow_rate_kg_per_s,
    );
    advance_cooling_economizer_guard_state(
        &mut unit.calc_cooling_economizer_guard,
        body,
        system.outdoor_air_economizer_type,
    )
}

fn assert_rejected_without_mutation(
    mut runtime: PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: PurchasedAirCalcCoolingEconomizerGuardSnapshot,
) {
    let before = runtime.clone();
    let result =
        advance_direct_no_oa_calc_cooling_economizer_condition(&mut runtime, system, predecessor);
    assert!(
        matches!(
            result,
            Err(PurchasedAirCalcCoolingEconomizerConditionError::CoolingEconomizerGuardSnapshotMismatch { .. })
                | Err(PurchasedAirCalcCoolingEconomizerConditionError::PredecessorCallOrder { .. })
                | Err(PurchasedAirCalcCoolingEconomizerConditionError::PredecessorOutsideDirectSubset { .. })
                | Err(PurchasedAirCalcCoolingEconomizerConditionError::RuntimeStateInvariantViolation { .. })
        ),
        "{result:?}"
    );
    assert_eq!(runtime, before);
}

fn test_system() -> IdealLoadsAirSystem {
    IdealLoadsAirSystem {
        id: SYSTEM,
        name: NormalizedName::new("ZONE ONE IDEAL LOADS"),
        availability_schedule: None,
        zone_supply_air_node_name: NormalizedName::new("ZONE ONE INLETS"),
        zone_exhaust_air_node_name: None,
        system_inlet_air_node_name: None,
        maximum_heating_supply_air_temperature_c: 50.0,
        minimum_cooling_supply_air_temperature_c: 13.0,
        maximum_heating_supply_air_humidity_ratio: 0.0156,
        minimum_cooling_supply_air_humidity_ratio: 0.0077,
        heating_limit: IdealLoadsLimit::NoLimit,
        maximum_heating_air_flow_rate_m3_per_s: None,
        maximum_sensible_heating_capacity_w: None,
        cooling_limit: IdealLoadsLimit::NoLimit,
        maximum_cooling_air_flow_rate_m3_per_s: None,
        maximum_total_cooling_capacity_w: None,
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
