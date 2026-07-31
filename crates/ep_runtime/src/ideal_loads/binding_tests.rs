use super::*;
use crate::{
    heat_balance::state::ZoneAirTemperatureCoefficients,
    ideal_loads::{
        IdealLoadsSensibleMode, IdealLoadsZoneState, PurchasedAirCalcEntryIdentityRelation,
        purchased_air_calc_entry_lifecycle_summary,
        purchased_air_calc_minimum_oa_prefix_lifecycle_summary,
        purchased_air_init_lifecycle_summary,
    },
    schedules::{ScheduleSeriesCache, precompute_schedule_cache},
};
use ep_model::{
    AutoOrNumber, AutosizeOrNumber, DehumidificationControlType, DemandControlledVentilationType,
    HeatRecoveryType, HumidificationControlType, IdealLoadsAirSystemId, IdealLoadsFuelType,
    IdealLoadsLimit, LoadDistributionScheme, Node, NodeId, NodeList, NodeListId, NormalizedName,
    OutdoorAirEconomizerType, Point3, ScheduleConstant, ScheduleId, SimulationModel,
    ThermostatControlObjectType, ThermostatDualSetpoint, ThermostatSetpointId, TypedModel, Zone,
    ZoneConvectionAlgorithm, ZoneEquipmentConnection, ZoneEquipmentConnectionId, ZoneEquipmentList,
    ZoneEquipmentListEntry, ZoneEquipmentListId, ZoneEquipmentObjectType, ZoneId, ZoneThermostat,
    ZoneThermostatControl, ZoneThermostatId,
};

#[path = "binding/cooling_capacity_zero_flow_reset_tests.rs"]
mod cooling_capacity_zero_flow_reset_tests;
#[path = "binding/cooling_dehumidification_flow_tests.rs"]
mod cooling_dehumidification_flow_tests;
#[path = "binding/cooling_economizer_body_integrity_tests.rs"]
mod cooling_economizer_body_integrity_tests;
#[path = "binding/cooling_economizer_body_tests.rs"]
mod cooling_economizer_body_tests;
#[path = "binding/cooling_economizer_condition_integrity_tests.rs"]
mod cooling_economizer_condition_integrity_tests;
#[path = "binding/cooling_economizer_condition_tests.rs"]
mod cooling_economizer_condition_tests;
#[path = "binding/cooling_economizer_guard_integrity_tests.rs"]
mod cooling_economizer_guard_integrity_tests;
#[path = "binding/cooling_economizer_guard_tests.rs"]
mod cooling_economizer_guard_tests;
#[path = "binding/cooling_entry_gate_tests.rs"]
mod cooling_entry_gate_tests;
#[path = "binding/cooling_humidification_flow_tests.rs"]
mod cooling_humidification_flow_tests;
#[path = "binding/cooling_mixed_air_call_tests.rs"]
mod cooling_mixed_air_call_tests;
#[path = "binding/cooling_oa_max_flow_body_tests.rs"]
mod cooling_oa_max_flow_body_tests;
#[path = "binding/cooling_oa_max_flow_gate_tests.rs"]
mod cooling_oa_max_flow_gate_tests;
#[rustfmt::skip] #[path = "binding/cooling_constant_shr_case_break_tests.rs"] mod cooling_constant_shr_case_break_tests;
#[rustfmt::skip] #[path = "binding/cooling_humidistat_case_entry_tests.rs"] mod cooling_humidistat_case_entry_tests;
#[rustfmt::skip] #[path = "binding/cooling_humidistat_moisture_demand_assignment_tests.rs"] mod cooling_humidistat_moisture_demand_assignment_tests;
#[rustfmt::skip] #[path = "binding/cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_tests.rs"] mod cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_tests;
#[rustfmt::skip] #[path = "binding/cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_tests.rs"] mod cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_tests;
#[rustfmt::skip] #[path = "binding/cooling_humidistat_supply_humidity_ratio_mixed_air_limit_tests.rs"] mod cooling_humidistat_supply_humidity_ratio_mixed_air_limit_tests;
#[rustfmt::skip] #[path = "binding/cooling_humidistat_case_break_tests.rs"] mod cooling_humidistat_case_break_tests;
#[rustfmt::skip] #[path = "binding/cooling_constant_supply_humidity_ratio_case_entry_tests.rs"] mod cooling_constant_supply_humidity_ratio_case_entry_tests;
#[rustfmt::skip] #[path = "binding/cooling_constant_supply_humidity_ratio_assignment_tests.rs"] mod cooling_constant_supply_humidity_ratio_assignment_tests;
#[rustfmt::skip] #[path = "binding/cooling_constant_supply_humidity_ratio_case_break_tests.rs"] mod cooling_constant_supply_humidity_ratio_case_break_tests;
#[rustfmt::skip] #[path = "binding/cooling_default_supply_humidity_ratio_mixed_air_assignment_tests.rs"] mod cooling_default_supply_humidity_ratio_mixed_air_assignment_tests;
#[rustfmt::skip] #[path = "binding/cooling_constant_shr_supply_humidity_ratio_minimum_limit_tests.rs"] mod cooling_constant_shr_supply_humidity_ratio_minimum_limit_tests;
#[rustfmt::skip] #[path = "binding/cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_tests.rs"] mod cooling_constant_shr_supply_humidity_ratio_mixed_air_limit_tests;
#[rustfmt::skip] #[path = "binding/cooling_constant_shr_supply_humidity_ratio_overdrying_limit_tests.rs"] mod cooling_constant_shr_supply_humidity_ratio_overdrying_limit_tests;
#[rustfmt::skip] #[path = "binding/cooling_positive_supply_cp_air_assignment_tests.rs"] mod cooling_positive_supply_cp_air_assignment_tests;
#[rustfmt::skip] #[path = "binding/cooling_positive_supply_temperature_assignment_tests.rs"] mod cooling_positive_supply_temperature_assignment_tests;
#[rustfmt::skip] #[path = "binding/cooling_positive_supply_temperature_minimum_limit_tests.rs"] mod cooling_positive_supply_temperature_minimum_limit_tests;
#[rustfmt::skip] #[path = "binding/cooling_positive_supply_temperature_mixed_air_limit_tests.rs"] mod cooling_positive_supply_temperature_mixed_air_limit_tests;
#[rustfmt::skip] #[path = "binding/cooling_positive_supply_humidity_ratio_mixed_air_assignment_tests.rs"] mod cooling_positive_supply_humidity_ratio_mixed_air_assignment_tests;
#[rustfmt::skip] #[path = "binding/cooling_positive_supply_enthalpy_assignment_tests.rs"] mod cooling_positive_supply_enthalpy_assignment_tests;
#[rustfmt::skip] #[path = "binding/cooling_positive_supply_capacity_limit_guard_tests.rs"] mod cooling_positive_supply_capacity_limit_guard_tests;
#[rustfmt::skip] #[path = "binding/cooling_positive_supply_capacity_limit_cp_air_assignment_tests.rs"] mod cooling_positive_supply_capacity_limit_cp_air_assignment_tests;
#[rustfmt::skip] #[path = "binding/cooling_positive_supply_capacity_limit_sensible_output_assignment_tests.rs"] mod cooling_positive_supply_capacity_limit_sensible_output_assignment_tests;
#[rustfmt::skip] #[path = "binding/cooling_positive_supply_capacity_limit_sensible_output_guard_tests.rs"] mod cooling_positive_supply_capacity_limit_sensible_output_guard_tests;
#[rustfmt::skip] #[path = "binding/cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_tests.rs"] mod cooling_positive_supply_capacity_limit_sensible_output_maximum_capacity_assignment_tests;
#[rustfmt::skip] #[path = "binding/cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_tests.rs"] mod cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment_tests;
#[rustfmt::skip] #[path = "binding/cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_tests.rs"] mod cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_tests;
#[rustfmt::skip] #[path = "binding/cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_tests.rs"] mod cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_tests;
#[rustfmt::skip] #[path = "binding/cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_tests.rs"] mod cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_tests;
#[rustfmt::skip] #[path = "binding/cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_tests.rs"] mod cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_tests;
#[rustfmt::skip] #[path = "binding/cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_tests.rs"] mod cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_tests;
#[rustfmt::skip] #[path = "binding/cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry_tests.rs"] mod cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry_tests;
#[rustfmt::skip] #[path = "binding/cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_tests.rs"] mod cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_tests;
#[rustfmt::skip] #[path = "binding/cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_tests.rs"] mod cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_tests;
#[rustfmt::skip] #[path = "binding/cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_tests.rs"] mod cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_tests;
#[rustfmt::skip] #[path = "binding/cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_tests.rs"] mod cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_tests;
#[rustfmt::skip] #[path = "binding/cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_tests.rs"] mod cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_tests;
#[path = "binding/cooling_sensible_flow_tests.rs"]
mod cooling_sensible_flow_tests;
#[path = "binding/cooling_supply_mass_flow_ems_override_body_tests.rs"]
mod cooling_supply_mass_flow_ems_override_body_tests;
#[path = "binding/cooling_supply_mass_flow_ems_override_guard_tests.rs"]
mod cooling_supply_mass_flow_ems_override_guard_tests;
#[path = "binding/cooling_supply_mass_flow_limit_body_tests.rs"]
mod cooling_supply_mass_flow_limit_body_tests;
#[path = "binding/cooling_supply_mass_flow_limit_guard_tests.rs"]
mod cooling_supply_mass_flow_limit_guard_tests;
#[path = "binding/cooling_supply_mass_flow_maximum_tests.rs"]
mod cooling_supply_mass_flow_maximum_tests;
#[rustfmt::skip] #[path = "binding/cooling_supply_mass_flow_positive_guard_tests.rs"] mod cooling_supply_mass_flow_positive_guard_tests;
#[path = "binding/cooling_supply_mass_flow_very_small_guard_body_tests.rs"]
mod cooling_supply_mass_flow_very_small_guard_body_tests;
#[path = "binding/cooling_supply_mass_flow_very_small_guard_tests.rs"]
mod cooling_supply_mass_flow_very_small_guard_tests;
#[path = "binding/minimum_oa_prefix_tests.rs"]
mod minimum_oa_prefix_tests;
#[path = "binding/model_multiplier_tests.rs"]
mod model_multiplier_tests;

#[test]
fn model_binding_resolves_exact_typed_ids_and_schedule_roles() {
    let (model, _) = fixture(|_| {});

    let binding = bind_direct_zone_purchased_air_model(&model).expect("bounded model binding");

    assert_eq!(binding.zone, ZoneId(0));
    assert_eq!(binding.thermostat, ZoneThermostatId(0));
    assert_eq!(binding.dual_setpoint, ThermostatSetpointId(0));
    assert_eq!(binding.control_type_schedule, ScheduleId(0));
    assert_eq!(binding.heating_setpoint_schedule, ScheduleId(1));
    assert_eq!(binding.cooling_setpoint_schedule, ScheduleId(2));
    assert_eq!(binding.overall_availability_schedule, Some(ScheduleId(3)));
    assert_eq!(binding.equipment_list, ZoneEquipmentListId(0));
    assert_eq!(binding.ideal_loads_air_system, IdealLoadsAirSystemId(0));
    assert_eq!([binding.supply_node.0, binding.return_node.0], [0, 2]);
    assert_eq!(binding.zone_air_node, NodeId(1));
    assert_eq!(binding.nominal_system_timestep_seconds, 600.0);
}

#[test]
fn binding_rejects_stale_public_model_graph() {
    let (mut model, _) = fixture(|_| {});
    model.typed.ideal_loads_air_systems[0].zone_supply_air_node_name =
        NormalizedName::new("ZONE AIR");

    assert_eq!(
        bind_direct_zone_purchased_air_model(&model)
            .expect_err("typed mutation must not reuse a stale graph"),
        DirectZonePurchasedAirBindingError::UnsupportedFeature {
            feature: DirectZonePurchasedAirBindingFeature::CoherentTypedModelGraph,
        }
    );
}

#[test]
fn scheduled_binding_samples_thermostat_and_drives_heating_feedback() {
    let (model, cache) = fixture(|_| {});
    let binding = bind_direct_zone_purchased_air_model(&model).expect("bounded model binding");
    let mut state = zone_state_for_temp_independent_load(0.0);
    let original = state.clone();

    let output = couple(&binding, &cache, &mut state, 0).expect("scheduled heating coupling");

    assert_eq!(
        output.schedules,
        DirectZonePurchasedAirScheduleSnapshot {
            sample_index: 0,
            control_type: 4.0,
            heating_setpoint_c: 20.0,
            cooling_setpoint_c: 24.0,
            overall_availability: 1.0,
            unit_available: true,
        }
    );
    assert_eq!(
        output.coupling.purchased_air.calculation.mode,
        IdealLoadsSensibleMode::Heating
    );
    assert_eq!(
        output
            .coupling
            .purchased_air
            .trace
            .zone_state
            .air_temperature_c,
        original.mean_air_temperature_c
    );
    assert_eq!(
        output.coupling.purchased_air.trace.recirculation_state,
        IdealLoadsZoneState {
            air_temperature_c: original.mean_air_temperature_c,
            air_humidity_ratio: original.air_humidity_ratio,
        },
        "the source-valid single return receives the bounded direct-Zone T/W projection"
    );
    assert_eq!(
        output.coupling.prediction.predicted_loads.predicted_rate_w,
        output.coupling.prediction.predicted_loads.raw_total_load_w
    );
    assert!(state.sum_sys_mcp_w_per_k > 0.0);
    assert_only_system_air_sums_changed(&original, &state);
}

#[test]
fn scheduled_binding_preserves_negative_cooling_threshold() {
    let (model, cache) = fixture(|_| {});
    let binding = bind_direct_zone_purchased_air_model(&model).expect("bounded model binding");
    let mut state = zone_state_for_temp_independent_load(3_000.0);

    let output = couple(&binding, &cache, &mut state, 0).expect("scheduled cooling coupling");

    assert_eq!(
        output.coupling.purchased_air.calculation.mode,
        IdealLoadsSensibleMode::Cooling
    );
    assert_eq!(
        output
            .coupling
            .prediction
            .zone_demand
            .remaining_output_req_to_cool_sp_w,
        -600.0
    );
    assert_eq!(
        output
            .calculation_entry
            .demand
            .remaining_output_req_to_cool_sp_w,
        -600.0
    );
    assert_eq!(output.calculation_entry.call_ordinal, 1);
    assert!(output.calculation_entry.unit_body_entered);
    let minimum_oa = output.calculation_minimum_outdoor_air;
    assert_eq!(minimum_oa.parent_call_ordinal, 1);
    assert!(minimum_oa.zone_heat_balance_reference_bound);
    assert!(minimum_oa.minimum_oa_child_called);
    assert!(minimum_oa.minimum_oa_child_no_outdoor_air_route);
    assert_eq!(
        minimum_oa.retained_minimum_outdoor_air_mass_flow_rate_kg_per_s,
        Some(0.0)
    );
    assert_eq!(
        minimum_oa.working_outdoor_air_mass_flow_rate_kg_per_s,
        Some(0.0)
    );
    assert_eq!(minimum_oa.minimum_outdoor_air_sensible_output_w, Some(0.0));
    assert_eq!(
        minimum_oa.minimum_outdoor_air_moisture_output_kg_per_s,
        Some(0.0)
    );
    assert!(state.sum_sys_mcp_w_per_k > 0.0);
}

#[test]
fn overall_availability_off_clears_stale_feedback_without_changing_other_state() {
    let (model, cache) = fixture(|typed| schedule_mut(typed, ScheduleId(3)).hourly_value = 0.0);
    let binding = bind_direct_zone_purchased_air_model(&model).expect("bounded model binding");
    let mut state = zone_state_for_temp_independent_load(0.0);
    let original = state.clone();

    let output = couple(&binding, &cache, &mut state, 0).expect("scheduled off coupling");

    assert!(!output.schedules.unit_available);
    assert!(!output.calculation_entry.unit_on);
    assert!(output.calculation_entry.heating_on);
    assert!(output.calculation_entry.cooling_on);
    assert!(output.calculation_entry.reset.all_zero());
    let minimum_oa = output.calculation_minimum_outdoor_air;
    assert!(!minimum_oa.unit_body_entered);
    assert!(!minimum_oa.zone_heat_balance_reference_bound);
    assert!(!minimum_oa.minimum_oa_child_called);
    assert!(!minimum_oa.ems_override_flag_read);
    assert!(!minimum_oa.outdoor_air_flag_read);
    assert_eq!(
        minimum_oa.retained_minimum_outdoor_air_mass_flow_rate_kg_per_s,
        None
    );
    assert_eq!(minimum_oa.minimum_outdoor_air_sensible_output_w, None);
    assert_eq!(
        output.coupling.purchased_air.calculation.mode,
        IdealLoadsSensibleMode::Off
    );
    assert_eq!(state.sum_sys_mcp_w_per_k, 0.0);
    assert_eq!(state.sum_sys_mcp_t_w, 0.0);
    assert_only_system_air_sums_changed(&original, &state);
}

#[test]
fn deadband_sample_clears_stale_feedback_exactly() {
    let (model, cache) = fixture(|_| {});
    let binding = bind_direct_zone_purchased_air_model(&model).expect("bounded model binding");
    let mut state = zone_state_for_temp_independent_load(2_200.0);

    let output = couple(&binding, &cache, &mut state, 0).expect("scheduled deadband coupling");

    assert_eq!(
        output.coupling.purchased_air.calculation.mode,
        IdealLoadsSensibleMode::Deadband
    );
    assert_eq!(state.sum_sys_mcp_w_per_k, 0.0);
    assert_eq!(state.sum_sys_mcp_t_w, 0.0);
}

#[test]
fn binding_rejects_ambiguous_thermostat_topology() {
    let (model, _) = fixture(|typed| {
        let mut thermostat = typed.zone_thermostats[0].clone();
        thermostat.id = ZoneThermostatId(1);
        typed.zone_thermostats.push(thermostat);
    });

    let error =
        bind_direct_zone_purchased_air_model(&model).expect_err("two thermostat edges must fail");

    assert_eq!(
        error,
        DirectZonePurchasedAirBindingError::Cardinality {
            relation: DirectZonePurchasedAirBindingRelation::ZoneThermostatEdge,
            expected: 1,
            actual: 2,
        }
    );
}

#[test]
fn binding_rejects_distribution_sequence_and_fraction_variants() {
    let cases = [
        (
            (|typed: &mut TypedModel| {
                typed.zone_equipment_lists[0].load_distribution_scheme =
                    LoadDistributionScheme::UniformLoad;
            }) as fn(&mut TypedModel),
            DirectZonePurchasedAirBindingFeature::SequentialLoadDistribution,
        ),
        (
            (|typed: &mut TypedModel| {
                typed.zone_equipment_lists[0].equipment[0].cooling_sequence = 2;
            }) as fn(&mut TypedModel),
            DirectZonePurchasedAirBindingFeature::FirstEquipmentSequence,
        ),
        (
            (|typed: &mut TypedModel| {
                typed.zone_equipment_lists[0].equipment[0].sequential_heating_fraction_schedule =
                    Some(ScheduleId(3));
            }) as fn(&mut TypedModel),
            DirectZonePurchasedAirBindingFeature::NoSequentialFractionSchedules,
        ),
    ];

    for (mutate, expected_feature) in cases {
        let (model, _) = fixture(mutate);
        let error = bind_direct_zone_purchased_air_model(&model)
            .expect_err("unsupported equipment topology must fail");
        assert_eq!(
            error,
            DirectZonePurchasedAirBindingError::UnsupportedFeature {
                feature: expected_feature,
            }
        );
    }
}

#[test]
fn binding_rejects_multi_inlet_return_and_mode_availability_topology() {
    let (multi_inlet, _) = fixture(|typed| {
        typed.nodes.push(Node {
            id: NodeId(3),
            name: NormalizedName::new("SECOND SUPPLY"),
        });
        typed.node_names.insert("SECOND SUPPLY", NodeId(3));
        typed.node_lists.push(NodeList {
            id: NodeListId(0),
            name: NormalizedName::new("INLETS"),
            nodes: vec![NodeId(0), NodeId(3)],
        });
        typed.node_list_names.insert("INLETS", NodeListId(0));
        typed.ideal_loads_air_systems[0].zone_supply_air_node_name = NormalizedName::new("INLETS");
        typed.zone_equipment_connections[0].zone_air_inlet_node_or_nodelist_name =
            Some(NormalizedName::new("INLETS"));
    });
    assert_eq!(
        bind_direct_zone_purchased_air_model(&multi_inlet).expect_err("two supply edges must fail"),
        DirectZonePurchasedAirBindingError::Cardinality {
            relation: DirectZonePurchasedAirBindingRelation::IdealLoadsSupplyNode,
            expected: 1,
            actual: 2,
        }
    );

    let (missing_return, _) = fixture(|typed| {
        typed.zone_equipment_connections[0].zone_return_air_node_or_nodelist_name = None;
    });
    assert_eq!(
        bind_direct_zone_purchased_air_model(&missing_return)
            .expect_err("blank exhaust requires one Zone return node"),
        DirectZonePurchasedAirBindingError::Cardinality {
            relation: DirectZonePurchasedAirBindingRelation::ZoneReturnNode,
            expected: 1,
            actual: 0,
        }
    );
    let (multiple_returns, _) = fixture(|typed| {
        typed.nodes.push(Node {
            id: NodeId(3),
            name: NormalizedName::new("SECOND RETURN"),
        });
        typed.node_names.insert("SECOND RETURN", NodeId(3));
        typed.node_lists.push(NodeList {
            id: NodeListId(0),
            name: NormalizedName::new("RETURNS"),
            nodes: vec![NodeId(2), NodeId(3)],
        });
        typed.node_list_names.insert("RETURNS", NodeListId(0));
        typed.zone_equipment_connections[0].zone_return_air_node_or_nodelist_name =
            Some(NormalizedName::new("RETURNS"));
    });
    assert_eq!(
        bind_direct_zone_purchased_air_model(&multiple_returns)
            .expect_err("the bounded blank-exhaust fallback requires exactly one return"),
        DirectZonePurchasedAirBindingError::Cardinality {
            relation: DirectZonePurchasedAirBindingRelation::ZoneReturnNode,
            expected: 1,
            actual: 2,
        }
    );

    let cases = [
        (
            (|typed: &mut TypedModel| {
                typed.zone_equipment_connections[0].zone_return_air_node_or_nodelist_name =
                    Some(NormalizedName::new("ZONE AIR"));
            }) as fn(&mut TypedModel),
            DirectZonePurchasedAirBindingFeature::DistinctZoneReturnNode,
        ),
        (
            (|typed: &mut TypedModel| {
                typed.zone_equipment_connections[0]
                    .zone_return_air_node_1_flow_rate_fraction_schedule = Some(ScheduleId(3));
            }) as fn(&mut TypedModel),
            DirectZonePurchasedAirBindingFeature::NoZoneReturnFlowControl,
        ),
        (
            (|typed: &mut TypedModel| {
                typed.ideal_loads_air_systems[0].heating_availability_schedule =
                    Some(ScheduleId(3));
            }) as fn(&mut TypedModel),
            DirectZonePurchasedAirBindingFeature::NoHeatingAvailabilitySchedule,
        ),
        (
            (|typed: &mut TypedModel| {
                typed.ideal_loads_air_systems[0].cooling_availability_schedule =
                    Some(ScheduleId(3));
            }) as fn(&mut TypedModel),
            DirectZonePurchasedAirBindingFeature::NoCoolingAvailabilitySchedule,
        ),
        (
            (|typed: &mut TypedModel| {
                typed.ideal_loads_air_systems[0].design_specification_zonehvac_sizing_object_name =
                    Some(NormalizedName::new("ZONE HVAC SIZING"));
            }) as fn(&mut TypedModel),
            DirectZonePurchasedAirBindingFeature::NoZoneHvacSizingObject,
        ),
        (
            (|typed: &mut TypedModel| {
                typed.ideal_loads_air_systems[0].heating_fuel_efficiency_schedule =
                    Some(ScheduleId(3));
            }) as fn(&mut TypedModel),
            DirectZonePurchasedAirBindingFeature::NoFuelEfficiencySchedules,
        ),
    ];
    for (mutate, expected_feature) in cases {
        let (model, _) = fixture(mutate);
        assert_eq!(
            bind_direct_zone_purchased_air_model(&model)
                .expect_err("unsupported direct topology must fail"),
            DirectZonePurchasedAirBindingError::UnsupportedFeature {
                feature: expected_feature,
            }
        );
    }
}

#[test]
fn binding_accepts_all_resolved_numeric_finite_limit_branches() {
    let cases = [
        (
            IdealLoadsLimit::LimitCapacity,
            IdealLoadsPurchasedAirBranch::NoOaFiniteCapacity,
        ),
        (
            IdealLoadsLimit::LimitFlowRate,
            IdealLoadsPurchasedAirBranch::NoOaFiniteFlow,
        ),
        (
            IdealLoadsLimit::LimitFlowRateAndCapacity,
            IdealLoadsPurchasedAirBranch::NoOaFiniteFlowAndCapacity,
        ),
    ];

    for (limit, expected_branch) in cases {
        let (model, _) = fixture(|typed| {
            let system = &mut typed.ideal_loads_air_systems[0];
            system.heating_limit = limit;
            system.maximum_heating_air_flow_rate_m3_per_s = Some(AutosizeOrNumber::Value(0.25));
            system.maximum_sensible_heating_capacity_w = Some(AutosizeOrNumber::Value(1_000.0));
            system.cooling_limit = limit;
            system.maximum_cooling_air_flow_rate_m3_per_s = Some(AutosizeOrNumber::Value(0.20));
            system.maximum_total_cooling_capacity_w = Some(AutosizeOrNumber::Value(900.0));
        });

        let binding =
            bind_direct_zone_purchased_air_model(&model).expect("resolved finite-limit binding");
        assert_eq!(binding.branch, expected_branch);
    }
}

#[test]
fn binding_rejects_autosized_or_missing_finite_limit_values() {
    for mutate in [
        (|typed: &mut TypedModel| {
            let system = &mut typed.ideal_loads_air_systems[0];
            system.heating_limit = IdealLoadsLimit::LimitFlowRate;
            system.maximum_heating_air_flow_rate_m3_per_s = Some(AutosizeOrNumber::Autosize);
        }) as fn(&mut TypedModel),
        (|typed: &mut TypedModel| {
            typed.ideal_loads_air_systems[0].cooling_limit = IdealLoadsLimit::LimitCapacity;
        }) as fn(&mut TypedModel),
    ] {
        let (model, _) = fixture(mutate);
        assert_eq!(
            bind_direct_zone_purchased_air_model(&model)
                .expect_err("unresolved finite limits must fail closed"),
            DirectZonePurchasedAirBindingError::UnsupportedFeature {
                feature: DirectZonePurchasedAirBindingFeature::NoOaSensibleNumericLimitSubset,
            }
        );
    }

    let (negative, _) = fixture(|typed| {
        let system = &mut typed.ideal_loads_air_systems[0];
        system.heating_limit = IdealLoadsLimit::LimitCapacity;
        system.maximum_sensible_heating_capacity_w = Some(AutosizeOrNumber::Value(-1.0));
    });
    assert_eq!(
        bind_direct_zone_purchased_air_model(&negative)
            .expect_err("negative public TypedModel hard size must fail closed"),
        DirectZonePurchasedAirBindingError::UnsupportedFeature {
            feature: DirectZonePurchasedAirBindingFeature::NoOaSensibleNumericLimitSubset,
        }
    );

    for value in [f64::NAN, f64::INFINITY] {
        let (nonfinite, _) = fixture(|typed| {
            let system = &mut typed.ideal_loads_air_systems[0];
            system.cooling_limit = IdealLoadsLimit::LimitFlowRate;
            system.maximum_cooling_air_flow_rate_m3_per_s = Some(AutosizeOrNumber::Value(value));
        });
        assert_eq!(
            bind_direct_zone_purchased_air_model(&nonfinite)
                .expect_err("nonfinite public TypedModel hard size must fail closed"),
            DirectZonePurchasedAirBindingError::UnsupportedFeature {
                feature: DirectZonePurchasedAirBindingFeature::NoOaSensibleNumericLimitSubset,
            }
        );
    }
}

#[test]
fn binding_still_rejects_non_sensible_purchased_air_branches() {
    let (model, _) = fixture(|typed| {
        typed.ideal_loads_air_systems[0].dehumidification_control_type =
            DehumidificationControlType::ConstantSensibleHeatRatio;
    });

    assert_eq!(
        bind_direct_zone_purchased_air_model(&model)
            .expect_err("humidity-selected branch remains outside direct coupling"),
        DirectZonePurchasedAirBindingError::UnsupportedBranch {
            branch: IdealLoadsPurchasedAirBranch::NoOaConstantSensibleHeatRatioCooling,
        }
    );

    let (finite_constant_shr, _) = fixture(|typed| {
        let system = &mut typed.ideal_loads_air_systems[0];
        system.heating_limit = IdealLoadsLimit::LimitCapacity;
        system.maximum_sensible_heating_capacity_w = Some(AutosizeOrNumber::Value(1_000.0));
        system.dehumidification_control_type =
            DehumidificationControlType::ConstantSensibleHeatRatio;
    });
    assert_eq!(
        bind_direct_zone_purchased_air_model(&finite_constant_shr)
            .expect_err("finite-limit branch selection must not hide humidity controls"),
        DirectZonePurchasedAirBindingError::UnsupportedFeature {
            feature: DirectZonePurchasedAirBindingFeature::SensibleOnlyHumidityControlsInactive,
        }
    );
}

#[test]
fn binding_rejects_hysteresis_and_hidden_no_oa_feature_flags() {
    let cases = [
        (
            (|typed: &mut TypedModel| {
                typed.zone_thermostats[0]
                    .temperature_difference_between_cutout_and_setpoint_delta_c = 0.5;
            }) as fn(&mut TypedModel),
            DirectZonePurchasedAirBindingFeature::ZeroCutoutDelta,
        ),
        (
            (|typed: &mut TypedModel| {
                typed.ideal_loads_air_systems[0].outdoor_air_economizer_type =
                    OutdoorAirEconomizerType::DifferentialDryBulb;
            }) as fn(&mut TypedModel),
            DirectZonePurchasedAirBindingFeature::NoOaSensibleNumericLimitSubset,
        ),
    ];

    for (mutate, expected_feature) in cases {
        let (model, _) = fixture(mutate);
        assert_eq!(
            bind_direct_zone_purchased_air_model(&model)
                .expect_err("unsupported thermostat/system feature must fail"),
            DirectZonePurchasedAirBindingError::UnsupportedFeature {
                feature: expected_feature,
            }
        );
    }
}

#[test]
fn schedule_errors_are_distinct_and_transactional() {
    let (model, cache) = fixture(|typed| schedule_mut(typed, ScheduleId(0)).hourly_value = 3.0);
    let binding = bind_direct_zone_purchased_air_model(&model).expect("bounded model binding");
    let mut state = zone_state_for_temp_independent_load(0.0);
    let original = state.clone();

    assert_eq!(
        couple(&binding, &cache, &mut state, 0).expect_err("control type three must fail"),
        DirectZonePurchasedAirScheduledCouplingError::UnsupportedControlType { value: 3.0 }
    );
    assert_eq!(state, original);

    assert_eq!(
        couple(&binding, &cache, &mut state, 2).expect_err("sample two is out of range"),
        DirectZonePurchasedAirScheduledCouplingError::SampleIndexOutOfRange {
            sample_index: 2,
            sample_count: 2,
        }
    );
    assert_eq!(state, original);

    let (missing_model, missing_cache) = fixture(|typed| {
        typed.thermostat_dual_setpoints[0].heating_setpoint_schedule = ScheduleId(99);
    });
    let missing_binding =
        bind_direct_zone_purchased_air_model(&missing_model).expect("topology still binds");
    assert_eq!(
        couple(&missing_binding, &missing_cache, &mut state, 0)
            .expect_err("missing heating schedule must fail"),
        DirectZonePurchasedAirScheduledCouplingError::MissingSchedule {
            role: DirectZonePurchasedAirScheduleRole::HeatingSetpoint,
            schedule: ScheduleId(99),
        }
    );
    assert_eq!(state, original);
}

#[test]
fn nonfinite_and_inverted_setpoints_are_transactional() {
    let cases = [
        (
            f64::NAN,
            24.0,
            DirectZonePurchasedAirScheduledCouplingError::NonFiniteScheduleValue {
                role: DirectZonePurchasedAirScheduleRole::HeatingSetpoint,
                schedule: ScheduleId(1),
            },
        ),
        (
            25.0,
            24.0,
            DirectZonePurchasedAirScheduledCouplingError::HeatingSetpointAboveCoolingSetpoint {
                heating_setpoint_c: 25.0,
                cooling_setpoint_c: 24.0,
            },
        ),
    ];

    for (heating, cooling, expected) in cases {
        let (model, cache) = fixture(|typed| {
            schedule_mut(typed, ScheduleId(1)).hourly_value = heating;
            schedule_mut(typed, ScheduleId(2)).hourly_value = cooling;
        });
        let binding = bind_direct_zone_purchased_air_model(&model).expect("bounded model binding");
        let mut state = zone_state_for_temp_independent_load(0.0);
        let original = state.clone();
        let error = couple(&binding, &cache, &mut state, 0)
            .expect_err("invalid current setpoints must fail");
        assert_eq!(error, expected);
        assert_eq!(state, original);
    }
}

#[test]
fn dual_setpoint_schedule_error_precedence_is_cooling_then_heating() {
    let (model, cache) = fixture(|typed| {
        schedule_mut(typed, ScheduleId(1)).hourly_value = f64::NAN;
        schedule_mut(typed, ScheduleId(2)).hourly_value = f64::NAN;
    });
    let binding = bind_direct_zone_purchased_air_model(&model).expect("bounded model binding");
    let mut state = zone_state_for_temp_independent_load(0.0);
    let original = state.clone();

    assert_eq!(
        couple(&binding, &cache, &mut state, 0)
            .expect_err("cooling schedule is sampled before heating"),
        DirectZonePurchasedAirScheduledCouplingError::NonFiniteScheduleValue {
            role: DirectZonePurchasedAirScheduleRole::CoolingSetpoint,
            schedule: ScheduleId(2),
        }
    );
    assert_eq!(state, original);
}

#[test]
fn wrapped_cp300_failure_is_transactional() {
    let (model, cache) = fixture(|_| {});
    let binding = bind_direct_zone_purchased_air_model(&model).expect("bounded model binding");
    let mut state = zone_state_for_temp_independent_load(0.0);
    state.mean_air_temperature_c = f64::INFINITY;
    let original = state.clone();

    assert_eq!(
        couple(&binding, &cache, &mut state, 0).expect_err("CP300 must reject infinite Zone MAT"),
        DirectZonePurchasedAirScheduledCouplingError::Coupling(
            DirectZonePurchasedAirCouplingError::InputNotFinite {
                field: "zone_node_temperature_c",
            }
        )
    );
    assert_eq!(state, original);
}

#[test]
fn predictor_failure_precedes_and_preserves_purchased_air_initialization() {
    let (model, cache) = fixture(|_| {});
    let binding = bind_direct_zone_purchased_air_model(&model).expect("bounded model binding");
    let mut state = zone_state_for_temp_independent_load(0.0);
    state.convective_internal_gain_w = f64::INFINITY;
    let original = state.clone();
    let mut purchased_air_runtime_state = PurchasedAirRuntimeState::default();
    purchased_air_runtime_state.module_initialized = true;
    let original_init_state = purchased_air_runtime_state.clone();

    let error = couple_model_bound_direct_zone_purchased_air(
        DirectZonePurchasedAirScheduledCouplingInput {
            binding: &binding,
            schedule_cache: &cache,
            schedule_sample_index: 0,
            zone_state: &mut state,
            purchased_air_runtime_state: &mut purchased_air_runtime_state,
            begin_environment: true,
            barometric_pressure_pa: binding.limit_context.barometric_pressure_pa,
            system_timestep_seconds: binding.nominal_system_timestep_seconds,
        },
    )
    .expect_err("predictor must reject the nonfinite source term before Init");

    assert!(matches!(
        error,
        DirectZonePurchasedAirScheduledCouplingError::Coupling(
            DirectZonePurchasedAirCouplingError::Prediction(_)
        )
    ));
    assert_eq!(purchased_air_runtime_state, original_init_state);
    assert_eq!(state, original);
}

#[test]
fn initialization_failure_precedes_calc_only_input_validation() {
    let (model, cache) = fixture(|_| {});
    let binding = bind_direct_zone_purchased_air_model(&model).expect("bounded model binding");
    let mut state = zone_state_for_temp_independent_load(0.0);
    state.air_humidity_ratio = -0.001;
    let mut purchased_air_runtime_state = PurchasedAirRuntimeState::default();
    purchased_air_runtime_state.module_initialized = true;

    let error = couple_model_bound_direct_zone_purchased_air(
        DirectZonePurchasedAirScheduledCouplingInput {
            binding: &binding,
            schedule_cache: &cache,
            schedule_sample_index: 0,
            zone_state: &mut state,
            purchased_air_runtime_state: &mut purchased_air_runtime_state,
            begin_environment: true,
            barometric_pressure_pa: binding.limit_context.barometric_pressure_pa,
            system_timestep_seconds: binding.nominal_system_timestep_seconds,
        },
    )
    .expect_err("Init failure must precede the Calc-only humidity rejection");

    assert_eq!(
        error,
        DirectZonePurchasedAirScheduledCouplingError::Initialization(
            PurchasedAirInitError::DeclaredSystemOrderChanged {
                expected: Vec::new(),
                actual: vec![binding.ideal_loads_air_system],
            }
        )
    );
}

#[test]
fn post_init_calc_failure_retains_init_but_preserves_zone_state() {
    let (model, cache) = fixture(|_| {});
    let binding = bind_direct_zone_purchased_air_model(&model).expect("bounded model binding");
    let mut state = zone_state_for_temp_independent_load(0.0);
    state.air_humidity_ratio = -0.001;
    let original = state.clone();
    let mut purchased_air_runtime_state = PurchasedAirRuntimeState::default();

    let error = couple_model_bound_direct_zone_purchased_air(
        DirectZonePurchasedAirScheduledCouplingInput {
            binding: &binding,
            schedule_cache: &cache,
            schedule_sample_index: 0,
            zone_state: &mut state,
            purchased_air_runtime_state: &mut purchased_air_runtime_state,
            begin_environment: true,
            barometric_pressure_pa: binding.limit_context.barometric_pressure_pa,
            system_timestep_seconds: binding.nominal_system_timestep_seconds,
        },
    )
    .expect_err("Calc-only humidity validation must run after successful Init");

    assert_eq!(
        error,
        DirectZonePurchasedAirScheduledCouplingError::Coupling(
            DirectZonePurchasedAirCouplingError::InputNegative {
                field: "recirculation_state.air_humidity_ratio",
                value: -0.001,
            }
        )
    );
    assert_eq!(state, original);
    let lifecycle = purchased_air_init_lifecycle_summary(
        &purchased_air_runtime_state,
        binding.ideal_loads_air_system,
    )
    .expect("successful Init prefix must remain reportable");
    assert_eq!(lifecycle.init_call_count, 1);
    assert_eq!(lifecycle.topology_completion_count, 1);
    assert!(lifecycle.flags.topology_ready);
    assert_eq!(lifecycle.environment_initialization_count, 1);
    let calc_lifecycle = purchased_air_calc_entry_lifecycle_summary(
        &purchased_air_runtime_state,
        binding.ideal_loads_air_system,
    )
    .expect("successful Calc-entry prefix must remain reportable");
    assert_eq!(calc_lifecycle.state.call_count, 1);
    assert_eq!(calc_lifecycle.state.reset_count, 1);
    assert_eq!(calc_lifecycle.state.demand_read_count, 1);
    assert_eq!(
        calc_lifecycle
            .state
            .latest
            .expect("retained Calc-entry snapshot")
            .demand
            .zone,
        binding.zone
    );
}

#[test]
fn public_calc_entry_replay_and_identity_errors_do_not_mutate_lifecycle() {
    let (model, cache) = fixture(|_| {});
    let binding = bind_direct_zone_purchased_air_model(&model).expect("bounded model binding");
    let mut zone_state = zone_state_for_temp_independent_load(0.0);
    let mut runtime = PurchasedAirRuntimeState::default();
    let output = couple_model_bound_direct_zone_purchased_air(
        DirectZonePurchasedAirScheduledCouplingInput {
            binding: &binding,
            schedule_cache: &cache,
            schedule_sample_index: 0,
            zone_state: &mut zone_state,
            purchased_air_runtime_state: &mut runtime,
            begin_environment: true,
            barometric_pressure_pa: binding.limit_context.barometric_pressure_pa,
            system_timestep_seconds: binding.nominal_system_timestep_seconds,
        },
    )
    .expect("first source-ordered coupling");
    let before_prefix_replay = runtime.clone();
    assert_eq!(
        advance_direct_no_oa_calc_minimum_oa_prefix(
            &mut runtime,
            binding.system,
            output.calculation_entry,
        ),
        Err(
            PurchasedAirCalcMinimumOaPrefixError::CalculationEntryCallOrder {
                system: binding.ideal_loads_air_system,
                calculation_entry_call_count: 1,
                minimum_oa_prefix_transition_count: 1,
            }
        )
    );
    assert_eq!(runtime, before_prefix_replay);
    let base_context = PurchasedAirCalcEntryContext {
        controlled_zone: binding.zone,
        supply_node: binding.supply_node,
        zone_node: binding.zone_air_node,
        outdoor_air_node: None,
        recirculation_node: binding.return_node,
        demand: output.coupling.prediction.zone_demand,
        zone_component_availability: Some(PurchasedAirAvailabilityStatus::NoAction),
        overall_availability: output.schedules.overall_availability,
        heating_availability: 1.0,
        cooling_availability: 1.0,
    };

    let before_replay = runtime.clone();
    assert_eq!(
        advance_purchased_air_calc_entry(
            &mut runtime,
            binding.ideal_loads_air_system,
            base_context
        ),
        Err(PurchasedAirCalcEntryError::InitializationCallOrder {
            system: binding.ideal_loads_air_system,
            init_call_count: 1,
            calc_call_count: 1,
        })
    );
    assert_eq!(runtime, before_replay);

    init_purchased_air_runtime(
        &mut runtime,
        &binding.init_manager_plan,
        &binding.init_topology_plan,
        binding.system,
        PurchasedAirInitCallContext {
            zone_equipment_inputs_filled: true,
            system_sizing_calculation: false,
            sizing: PurchasedAirHardSizeLegacyContext {
                current_zone_equipment_index: 1,
                zone_sizing_run_done: false,
            },
            begin_environment: false,
            standard_air_density_kg_per_m3: binding.limit_context.standard_air_density_kg_per_m3,
            heating_setpoint_c: output.schedules.heating_setpoint_c,
            cooling_setpoint_c: output.schedules.cooling_setpoint_c,
            overall_availability: output.schedules.overall_availability,
            heating_availability: 1.0,
            cooling_availability: 1.0,
        },
    )
    .expect("second initialization prefix");

    for (context, relation) in [
        (
            PurchasedAirCalcEntryContext {
                controlled_zone: ZoneId(99),
                ..base_context
            },
            PurchasedAirCalcEntryIdentityRelation::ControlledZone,
        ),
        (
            PurchasedAirCalcEntryContext {
                supply_node: NodeId(99),
                ..base_context
            },
            PurchasedAirCalcEntryIdentityRelation::SupplyNode,
        ),
        (
            PurchasedAirCalcEntryContext {
                recirculation_node: NodeId(99),
                ..base_context
            },
            PurchasedAirCalcEntryIdentityRelation::RecirculationNode,
        ),
        (
            PurchasedAirCalcEntryContext {
                demand: crate::zone_equipment::ZoneSysEnergyDemand {
                    zone: ZoneId(99),
                    ..base_context.demand
                },
                ..base_context
            },
            PurchasedAirCalcEntryIdentityRelation::DemandZone,
        ),
    ] {
        let before_error = runtime.clone();
        assert_eq!(
            advance_purchased_air_calc_entry(&mut runtime, binding.ideal_loads_air_system, context),
            Err(PurchasedAirCalcEntryError::IdentityMismatch {
                system: binding.ideal_loads_air_system,
                relation,
            })
        );
        assert_eq!(runtime, before_error);
    }

    let second = advance_purchased_air_calc_entry(
        &mut runtime,
        binding.ideal_loads_air_system,
        base_context,
    )
    .expect("valid second Calc-entry prefix");
    assert_eq!(second.call_ordinal, 2);
}

#[test]
fn fixed_timestep_state_guards_are_transactional() {
    let (model, cache) = fixture(|_| {});
    let binding = bind_direct_zone_purchased_air_model(&model).expect("bounded model binding");
    let base = zone_state_for_temp_independent_load(0.0);

    let mut zone_history = base.clone();
    zone_history.use_zone_timestep_history = true;
    assert_runtime_invariant(
        &binding,
        &cache,
        zone_history,
        DirectZonePurchasedAirRuntimeInvariant::SystemTimestepHistory,
    );

    let mut wrong_zone = base.clone();
    wrong_zone.zone_id = ZoneId(7);
    assert_runtime_invariant(
        &binding,
        &cache,
        wrong_zone,
        DirectZonePurchasedAirRuntimeInvariant::BoundZoneIdentity,
    );

    let mut shortened = base.clone();
    shortened.shorten_timestep_sys = true;
    assert_runtime_invariant(
        &binding,
        &cache,
        shortened,
        DirectZonePurchasedAirRuntimeInvariant::UnshortenedSystemTimestep,
    );

    let mut multiple_steps = base.clone();
    multiple_steps.previous_system_timestep_count = 2;
    assert_runtime_invariant(
        &binding,
        &cache,
        multiple_steps,
        DirectZonePurchasedAirRuntimeInvariant::SinglePreviousSystemTimestep,
    );

    let mut wrong_prior = base;
    wrong_prior.prior_timestep_seconds = 300.0;
    assert_runtime_invariant(
        &binding,
        &cache,
        wrong_prior,
        DirectZonePurchasedAirRuntimeInvariant::NominalPriorTimestep,
    );

    let mut wrong_step = zone_state_for_temp_independent_load(0.0);
    let mut purchased_air_runtime_state = PurchasedAirRuntimeState::default();
    let original = wrong_step.clone();
    assert_eq!(
        couple_model_bound_direct_zone_purchased_air(
            DirectZonePurchasedAirScheduledCouplingInput {
                binding: &binding,
                schedule_cache: &cache,
                schedule_sample_index: 0,
                zone_state: &mut wrong_step,
                purchased_air_runtime_state: &mut purchased_air_runtime_state,
                begin_environment: true,
                barometric_pressure_pa: binding.limit_context.barometric_pressure_pa,
                system_timestep_seconds: 300.0,
            }
        )
        .expect_err("non-nominal requested timestep must fail"),
        DirectZonePurchasedAirScheduledCouplingError::RuntimeInvariant {
            invariant: DirectZonePurchasedAirRuntimeInvariant::NominalSystemTimestep,
        }
    );
    assert_eq!(wrong_step, original);
}

fn fixture(mutate: impl FnOnce(&mut TypedModel)) -> (SimulationModel, ScheduleSeriesCache) {
    let mut typed = base_typed_model();
    mutate(&mut typed);
    let cache = precompute_schedule_cache(&typed, 2).expect("test schedule cache");
    (SimulationModel::from_typed(typed), cache)
}

fn base_typed_model() -> TypedModel {
    let mut typed = TypedModel::default();
    for (id, name, value) in [
        (ScheduleId(0), "CONTROL TYPE", 4.0),
        (ScheduleId(1), "HEATING SETPOINT", 20.0),
        (ScheduleId(2), "COOLING SETPOINT", 24.0),
        (ScheduleId(3), "IDEAL LOADS AVAILABILITY", 1.0),
    ] {
        typed.schedules.push(ScheduleConstant {
            id,
            name: NormalizedName::new(name),
            schedule_type_limits: None,
            hourly_value: value,
        });
        typed.schedule_names.insert(name, id);
    }
    typed.zones.push(Zone {
        id: ZoneId(0),
        name: NormalizedName::new("ZONE ONE"),
        direction_of_relative_north_deg: 0.0,
        origin: Point3 {
            x_m: 0.0,
            y_m: 0.0,
            z_m: 0.0,
        },
        zone_type: 1,
        multiplier: 1,
        list_multiplier: 1,
        list_group: None,
        ceiling_height: AutoOrNumber::AutoCalculate,
        volume: AutoOrNumber::Value(100.0),
        floor_area: AutoOrNumber::AutoCalculate,
        inside_convection_algorithm: ZoneConvectionAlgorithm::Inherited(
            ep_model::InsideSurfaceConvectionAlgorithm::Tarp,
        ),
        outside_convection_algorithm: ZoneConvectionAlgorithm::Inherited(
            ep_model::OutsideSurfaceConvectionAlgorithm::Doe2,
        ),
        is_part_of_total_floor_area: true,
        is_nominal_controlled: true,
        linked_outdoor_air_node: None,
        spaces: Vec::new(),
    });
    typed
        .thermostat_dual_setpoints
        .push(ThermostatDualSetpoint {
            id: ThermostatSetpointId(0),
            name: NormalizedName::new("DUAL SETPOINT"),
            heating_setpoint_schedule: ScheduleId(1),
            cooling_setpoint_schedule: ScheduleId(2),
        });
    typed.zone_thermostats.push(ZoneThermostat {
        id: ZoneThermostatId(0),
        name: NormalizedName::new("ZONE THERMOSTAT"),
        zone: ZoneId(0),
        control_type_schedule: ScheduleId(0),
        controls: vec![ZoneThermostatControl {
            object_type: ThermostatControlObjectType::DualSetpoint,
            dual_setpoint: ThermostatSetpointId(0),
        }],
        temperature_difference_between_cutout_and_setpoint_delta_c: 0.0,
    });
    for (id, name) in [
        (NodeId(0), "SUPPLY"),
        (NodeId(1), "ZONE AIR"),
        (NodeId(2), "RETURN"),
    ] {
        typed.nodes.push(Node {
            id,
            name: NormalizedName::new(name),
        });
        typed.node_names.insert(name, id);
    }
    typed.ideal_loads_air_systems.push(ideal_loads_system());
    typed.zone_equipment_lists.push(ZoneEquipmentList {
        id: ZoneEquipmentListId(0),
        name: NormalizedName::new("ZONE EQUIPMENT"),
        load_distribution_scheme: LoadDistributionScheme::SequentialLoad,
        equipment: vec![ZoneEquipmentListEntry {
            object_type: ZoneEquipmentObjectType::IdealLoadsAirSystem,
            ideal_loads_air_system: IdealLoadsAirSystemId(0),
            cooling_sequence: 1,
            heating_or_no_load_sequence: 1,
            sequential_cooling_fraction_schedule: None,
            sequential_heating_fraction_schedule: None,
        }],
    });
    typed
        .zone_equipment_connections
        .push(ZoneEquipmentConnection {
            id: ZoneEquipmentConnectionId(0),
            zone: ZoneId(0),
            equipment_list: ZoneEquipmentListId(0),
            zone_air_inlet_node_or_nodelist_name: Some(NormalizedName::new("SUPPLY")),
            zone_air_exhaust_node_or_nodelist_name: None,
            zone_air_node_name: NormalizedName::new("ZONE AIR"),
            zone_return_air_node_or_nodelist_name: Some(NormalizedName::new("RETURN")),
            zone_return_air_node_1_flow_rate_fraction_schedule: None,
            zone_return_air_node_1_flow_rate_basis_node_or_nodelist_name: None,
        });
    typed
}

fn ideal_loads_system() -> ep_model::IdealLoadsAirSystem {
    ep_model::IdealLoadsAirSystem {
        id: IdealLoadsAirSystemId(0),
        name: NormalizedName::new("ZONE IDEAL LOADS"),
        availability_schedule: Some(ScheduleId(3)),
        zone_supply_air_node_name: NormalizedName::new("SUPPLY"),
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

fn zone_state_for_temp_independent_load(temp_independent_load_w: f64) -> ZoneHeatBalanceState {
    ZoneHeatBalanceState {
        zone_id: ZoneId(0),
        zone_name: "ZONE ONE".to_string(),
        mean_air_temperature_c: 22.0,
        zone_timestep_average_air_temperature_c: 22.0,
        previous_mean_air_temperatures_c: [0.0; 3],
        previous_system_mean_air_temperatures_c: [0.0; 3],
        previous_system_timestep_count: 1,
        air_humidity_ratio: 0.008,
        zone_timestep_average_air_humidity_ratio: 0.008,
        previous_air_humidity_ratios: [0.008; 3],
        previous_system_air_humidity_ratios: [0.008; 3],
        use_zone_timestep_history: false,
        shorten_timestep_sys: false,
        prior_timestep_seconds: 600.0,
        volume_m3: 100.0,
        air_heat_capacity_j_per_k: 0.0,
        convective_internal_gain_w: 0.0,
        opaque_surface_conductance_w_per_k: 100.0,
        opaque_surface_heat_gain_w: 0.0,
        opaque_surface_outside_conduction_w: 0.0,
        sum_ha_w_per_k: 100.0,
        sum_hat_surf_w: temp_independent_load_w,
        sum_hat_ref_w: 0.0,
        sum_mcp_w_per_k: 0.0,
        sum_mcp_t_w: 0.0,
        sum_sys_mcp_w_per_k: 7.0,
        sum_sys_mcp_t_w: 11.0,
        system_dependent_zone_loads_lagged_w: 0.0,
        zone_air_temperature_coefficients: ZoneAirTemperatureCoefficients::ZERO,
        system_timestep_average_surface_convection_report_w: None,
        system_timestep_average_air_storage_report_w: None,
    }
}

fn schedule_mut(model: &mut TypedModel, schedule: ScheduleId) -> &mut ScheduleConstant {
    model
        .schedules
        .iter_mut()
        .find(|candidate| candidate.id == schedule)
        .expect("fixture schedule")
}

fn couple(
    binding: &DirectZonePurchasedAirModelBinding<'_>,
    cache: &ScheduleSeriesCache,
    state: &mut ZoneHeatBalanceState,
    sample_index: usize,
) -> Result<
    DirectZonePurchasedAirScheduledCouplingOutput,
    DirectZonePurchasedAirScheduledCouplingError,
> {
    let mut purchased_air_runtime_state = PurchasedAirRuntimeState::default();
    couple_model_bound_direct_zone_purchased_air(DirectZonePurchasedAirScheduledCouplingInput {
        binding,
        schedule_cache: cache,
        schedule_sample_index: sample_index,
        zone_state: state,
        purchased_air_runtime_state: &mut purchased_air_runtime_state,
        begin_environment: true,
        barometric_pressure_pa: binding.limit_context.barometric_pressure_pa,
        system_timestep_seconds: 600.0,
    })
}

fn assert_runtime_invariant(
    binding: &DirectZonePurchasedAirModelBinding<'_>,
    cache: &ScheduleSeriesCache,
    mut state: ZoneHeatBalanceState,
    invariant: DirectZonePurchasedAirRuntimeInvariant,
) {
    let original = state.clone();
    assert_eq!(
        couple(binding, cache, &mut state, 0).expect_err("runtime invariant must fail"),
        DirectZonePurchasedAirScheduledCouplingError::RuntimeInvariant { invariant }
    );
    assert_eq!(state, original);
}

fn assert_only_system_air_sums_changed(
    original: &ZoneHeatBalanceState,
    actual: &ZoneHeatBalanceState,
) {
    let mut expected = original.clone();
    expected.sum_sys_mcp_w_per_k = actual.sum_sys_mcp_w_per_k;
    expected.sum_sys_mcp_t_w = actual.sum_sys_mcp_t_w;
    assert_eq!(*actual, expected);
}
