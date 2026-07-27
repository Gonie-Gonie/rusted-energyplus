use super::*;

use crate::{
    ideal_loads::{
        DirectZonePurchasedAirBindingFeature, IdealLoadsSensibleLimitContext,
        PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_SOURCE,
        PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_SOURCE,
        PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_SOURCE,
        PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_FIRST_EXCLUDED_SOURCE,
        PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_SOURCE,
        PURCHASED_AIR_CALC_MINIMUM_OA_CHILD_SOURCE, PURCHASED_AIR_CALC_MINIMUM_OA_PREFIX_SOURCE,
        PURCHASED_AIR_INIT_LIFECYCLE_SOURCE, PurchasedAirTemperatureControlType,
        ZONE_IDEAL_LOADS_SUPPLY_AIR_HUMIDITY_RATIO, ZONE_IDEAL_LOADS_SUPPLY_AIR_MASS_FLOW_RATE,
        ZONE_IDEAL_LOADS_SUPPLY_AIR_TEMPERATURE, ZONE_IDEAL_LOADS_ZONE_TOTAL_HEATING_ENERGY,
        ZONE_IDEAL_LOADS_ZONE_TOTAL_HEATING_RATE,
        ZONE_SYSTEM_PREDICTED_SENSIBLE_LOAD_TO_COOLING_SETPOINT_RATE,
        ZONE_SYSTEM_PREDICTED_SENSIBLE_LOAD_TO_HEATING_SETPOINT_RATE,
    },
    schedules::precompute_schedule_cache,
    time_axis::run_period_first_hour_interpolation_starting_values,
    weather::{EpwRecord, WeatherTimestepSeries},
};
use ep_model::{
    AutoOrNumber, AutosizeOrNumber, DehumidificationControlType, DemandControlledVentilationType,
    HeatRecoveryType, HumidificationControlType, IdealLoadsAirSystem, IdealLoadsAirSystemId,
    IdealLoadsFuelType, IdealLoadsLimit, LoadDistributionScheme, Node, NodeId, NormalizedName,
    OutdoorAirEconomizerType, Point3, ScheduleConstant, ScheduleId, SimulationModel, SiteLocation,
    ThermostatControlObjectType, ThermostatDualSetpoint, ThermostatSetpointId, TypedModel, Zone,
    ZoneConvectionAlgorithm, ZoneEquipmentConnection, ZoneEquipmentConnectionId, ZoneEquipmentList,
    ZoneEquipmentListEntry, ZoneEquipmentListId, ZoneEquipmentObjectType, ZoneId, ZoneThermostat,
    ZoneThermostatControl, ZoneThermostatId,
};

const HOURS: usize = 2;
const STEPS_PER_HOUR: u32 = 4;
const INITIAL_ZONE_TEMPERATURE_C: f64 = 23.0;
const HEATING_SETPOINT_C: f64 = 30.0;
const COOLING_SETPOINT_C: f64 = 35.0;
const SYSTEM_KEY: &str = "ZONE IDEAL LOADS";
const ZONE_KEY: &str = "ZONE ONE";
const SUPPLY_NODE_KEY: &str = "SUPPLY";
const RETURN_NODE_KEY: &str = "RETURN";
const ABS_TOLERANCE: f64 = 1.0e-9;

#[test]
fn cooling_economizer_guard_partition_overflow_fails_closed() {
    let error = super::cooling_economizer_guard_validation::checked_add(
        usize::MAX,
        1,
        "test_partition_overflow",
        1,
    )
    .expect_err("overflow must fail closed");
    assert!(matches!(
        error,
        DirectZonePurchasedAirCoupledRuntimeError::CalcCoolingEconomizerGuardLifecycleInvariant {
            field: "test_partition_overflow",
            expected: 1,
            actual: usize::MAX,
        }
    ));
}

#[test]
fn cooling_oa_max_flow_body_partition_overflow_fails_closed() {
    let error = super::cooling_oa_max_flow_body_validation::checked_add(
        usize::MAX,
        1,
        "test_partition_overflow",
        1,
    )
    .expect_err("overflow must fail closed");
    assert!(matches!(
        error,
        DirectZonePurchasedAirCoupledRuntimeError::CalcCoolingOaMaxFlowBodyLifecycleInvariant {
            field: "test_partition_overflow",
            expected: 1,
            actual: usize::MAX,
        }
    ));
}

#[test]
fn cooling_entry_mode_reconciliation_rejects_forged_numerical_modes() {
    use IdealLoadsSensibleMode::{Cooling, Deadband, Heating, Off};

    for (unit_body_entered, expected_cooling, actual, expected) in [
        (false, false, Off, true),
        (true, true, Cooling, true),
        (true, false, Heating, true),
        (true, false, Deadband, true),
        (true, false, Cooling, false),
        (true, false, Off, false),
        (true, true, Heating, false),
        (false, false, Heating, false),
    ] {
        assert_eq!(
            super::cooling_entry_validation::numerical_mode_matches_release(
                unit_body_entered,
                expected_cooling,
                actual,
            ),
            expected,
            "unit_body_entered={unit_body_entered}, expected_cooling={expected_cooling}, actual={actual:?}"
        );
    }
}

#[test]
fn cooling_entry_wrapper_requires_dual_heat_cool_and_finite_active_demand() {
    for (control_type, unit_body_entered, cooling_demand_w, expected) in [
        (4.0, true, -1.0, true),
        (4.0, false, 0.0, true),
        (3.0, true, -1.0, false),
        (3.0, false, 0.0, false),
        (4.0, true, f64::NAN, false),
        (4.0, true, f64::INFINITY, false),
    ] {
        assert_eq!(
            super::cooling_entry_validation::release_wrapper_inputs_match(
                control_type,
                unit_body_entered,
                cooling_demand_w,
            ),
            expected
        );
    }
}

#[test]
fn cooling_entry_count_partitions_reject_both_overflow_routes() {
    for field in [
        "source_skip_partition_overflow",
        "cooling_fallthrough_partition_overflow",
    ] {
        assert!(matches!(
            super::cooling_entry_validation::checked_partition(usize::MAX, 1, field, 7),
            Err(
                DirectZonePurchasedAirCoupledRuntimeError::CalcCoolingEntryGateLifecycleInvariant {
                    field: actual_field,
                    expected: 7,
                    actual: usize::MAX,
                }
            ) if actual_field == field
        ));
    }
}

#[test]
fn cooling_oa_max_flow_count_arithmetic_rejects_overflow_and_underflow() {
    assert!(matches!(
        super::cooling_oa_max_flow_validation::checked_add(usize::MAX, 1, "partition_overflow", 7,),
        Err(
            DirectZonePurchasedAirCoupledRuntimeError::CalcCoolingOaMaxFlowGateLifecycleInvariant {
                field: "partition_overflow",
                expected: 7,
                actual: usize::MAX,
            }
        )
    ));
    assert!(matches!(
        super::cooling_oa_max_flow_validation::checked_sub(0, 1, "selector_underflow", 7,),
        Err(
            DirectZonePurchasedAirCoupledRuntimeError::CalcCoolingOaMaxFlowGateLifecycleInvariant {
                field: "selector_underflow",
                expected: 7,
                actual: usize::MAX,
            }
        )
    ));
}

#[test]
fn exact_model_runs_one_source_threshold_coupling_per_fixed_timestep() {
    let model = exact_model(STEPS_PER_HOUR);
    let required_steps = HOURS * STEPS_PER_HOUR as usize;
    let schedule_cache =
        precompute_schedule_cache(&model.typed, required_steps).expect("constant schedule cache");
    let weather = weather_series(&model, HOURS);
    let mut options = DirectZonePurchasedAirCoupledOptions::hourly_samples(HOURS);
    options.initial_zone_air_temperature_c = INITIAL_ZONE_TEMPERATURE_C;

    let simulation = simulate_direct_zone_purchased_air_coupled_heat_balance(
        &model,
        &weather,
        &schedule_cache,
        options,
    )
    .expect("exact bounded direct-Zone model");

    assert_eq!(simulation.summary.samples, HOURS);
    assert_eq!(simulation.summary.timestep_count, required_steps);
    assert_eq!(simulation.summary.coupling_call_count, required_steps);
    assert_eq!(simulation.summary.zone_timesteps_per_hour, STEPS_PER_HOUR);
    assert_close(
        simulation.summary.timestep_seconds,
        3_600.0 / f64::from(STEPS_PER_HOUR),
    );
    assert_eq!(simulation.summary.system_name, SYSTEM_KEY);
    assert_eq!(simulation.summary.supply_node_name, SUPPLY_NODE_KEY);
    assert_eq!(simulation.summary.return_node_name, RETURN_NODE_KEY);
    assert_eq!(
        simulation.summary.branch,
        IdealLoadsPurchasedAirBranch::NoOaNoLimitSensible
    );
    assert_eq!(
        simulation.summary.zone_demand_source,
        DIRECT_ZONE_PURCHASED_AIR_DEMAND_SOURCE
    );
    assert!(!simulation.summary.fixture_demand_injection_used);
    assert_eq!(
        simulation.summary.recirculation_state_source,
        DIRECT_ZONE_PURCHASED_AIR_RECIRCULATION_SOURCE
    );
    assert_eq!(
        simulation.summary.actual_coupled_source_order,
        DIRECT_ZONE_PURCHASED_AIR_COUPLED_SOURCE_ORDER
    );
    let lifecycle = simulation.summary.init_lifecycle;
    assert_eq!(lifecycle.source, PURCHASED_AIR_INIT_LIFECYCLE_SOURCE);
    assert!(lifecycle.flags.state_machine_used);
    assert!(lifecycle.flags.one_time_checked);
    assert!(lifecycle.flags.topology_ready);
    assert!(lifecycle.flags.environment_initialized);
    assert!(lifecycle.flags.environment_initialization_needed);
    assert!(lifecycle.flags.sizing_checked);
    assert!(lifecycle.flags.equipment_list_checked);
    assert!(lifecycle.flags.return_plenum_inactive);
    assert_eq!(lifecycle.module_initialization_count, 1);
    assert_eq!(lifecycle.equipment_list_check_count, 1);
    assert_eq!(lifecycle.init_call_count, required_steps);
    assert_eq!(lifecycle.one_time_initialization_count, 1);
    assert_eq!(lifecycle.topology_completion_count, 1);
    assert_eq!(lifecycle.controlled_zone, Some(ZoneId(0)));
    assert_eq!(lifecycle.equipment_list, Some(ZoneEquipmentListId(0)));
    assert_eq!(lifecycle.supply_node, Some(NodeId(0)));
    assert_eq!(lifecycle.recirculation_node, Some(NodeId(2)));
    assert_eq!(
        lifecycle.recirculation_source,
        Some(PurchasedAirRecirculationSource::SingleZoneReturn)
    );
    assert!(lifecycle.topology_diagnostics.is_empty());
    assert_eq!(lifecycle.topology_failure, None);
    assert_eq!(lifecycle.economizer_flow_limit_warning_count, 0);
    assert_eq!(lifecycle.sizing_check_count, 1);
    assert_eq!(lifecycle.environment_initialization_count, 1);
    assert_eq!(lifecycle.environment_rearm_count, 1);
    assert_close(lifecycle.maximum_heating_air_mass_flow_rate_kg_per_s, 0.0);
    assert_close(lifecycle.maximum_cooling_air_mass_flow_rate_kg_per_s, 0.0);
    let calc_lifecycle = simulation.summary.calc_entry_lifecycle;
    assert_eq!(calc_lifecycle.source, PURCHASED_AIR_CALC_ENTRY_SOURCE);
    assert_eq!(calc_lifecycle.state.call_count, required_steps);
    assert_eq!(calc_lifecycle.state.reset_count, required_steps);
    assert_eq!(calc_lifecycle.state.demand_read_count, required_steps);
    assert_eq!(
        calc_lifecycle.state.overall_availability_read_count,
        required_steps
    );
    assert_eq!(
        calc_lifecycle.state.heating_availability_read_count,
        required_steps
    );
    assert_eq!(
        calc_lifecycle.state.cooling_availability_read_count,
        required_steps
    );
    assert_eq!(
        calc_lifecycle.state.availability_manager_read_count,
        required_steps
    );
    assert_eq!(
        calc_lifecycle.state.availability_manager_zone_write_count,
        required_steps
    );
    assert_eq!(
        calc_lifecycle.state.availability_status_copy_count,
        required_steps
    );
    assert_eq!(
        calc_lifecycle.state.availability_manager_zone,
        Some(ZoneId(0))
    );
    assert_eq!(calc_lifecycle.state.force_off_count, 0);
    assert_eq!(calc_lifecycle.state.heating_on_count, required_steps);
    assert_eq!(calc_lifecycle.state.cooling_on_count, required_steps);
    let latest_calc = calc_lifecycle
        .state
        .latest
        .expect("latest Calc-entry lifecycle snapshot");
    assert_eq!(latest_calc.call_ordinal, required_steps);
    assert_eq!(latest_calc.controlled_zone, ZoneId(0));
    assert_eq!(latest_calc.supply_node, NodeId(0));
    assert_eq!(latest_calc.zone_node, NodeId(1));
    assert_eq!(latest_calc.outdoor_air_node, None);
    assert_eq!(latest_calc.recirculation_node, NodeId(2));
    assert!(latest_calc.reset.all_zero());
    assert!(latest_calc.unit_on);
    assert!(latest_calc.heating_on);
    assert!(latest_calc.cooling_on);
    let minimum_oa_lifecycle = simulation.summary.calc_minimum_oa_prefix_lifecycle;
    assert_eq!(
        minimum_oa_lifecycle.source,
        PURCHASED_AIR_CALC_MINIMUM_OA_PREFIX_SOURCE
    );
    assert_eq!(
        minimum_oa_lifecycle.minimum_oa_child_source,
        PURCHASED_AIR_CALC_MINIMUM_OA_CHILD_SOURCE
    );
    assert_eq!(minimum_oa_lifecycle.state.transition_count, required_steps);
    assert_eq!(
        minimum_oa_lifecycle.state.source_execution_count,
        required_steps
    );
    assert_eq!(minimum_oa_lifecycle.state.unit_off_skip_count, 0);
    assert_eq!(
        minimum_oa_lifecycle.state.minimum_oa_child_call_count,
        required_steps
    );
    assert_eq!(minimum_oa_lifecycle.state.ems_override_apply_count, 0);
    assert_eq!(minimum_oa_lifecycle.state.outdoor_air_effect_count, 0);
    assert_eq!(
        minimum_oa_lifecycle.state.no_outdoor_air_zero_branch_count,
        required_steps
    );
    let latest_minimum_oa = minimum_oa_lifecycle
        .state
        .latest
        .expect("latest minimum-OA prefix snapshot");
    assert_eq!(latest_minimum_oa.parent_call_ordinal, required_steps);
    assert!(latest_minimum_oa.minimum_oa_child_called);
    assert_eq!(
        latest_minimum_oa.retained_minimum_outdoor_air_mass_flow_rate_kg_per_s,
        Some(0.0)
    );
    assert_eq!(
        latest_minimum_oa.working_outdoor_air_mass_flow_rate_kg_per_s,
        Some(0.0)
    );
    assert_eq!(
        latest_minimum_oa.minimum_outdoor_air_sensible_output_w,
        Some(0.0)
    );
    assert_eq!(
        latest_minimum_oa.minimum_outdoor_air_moisture_output_kg_per_s,
        Some(0.0)
    );
    let cooling_entry_lifecycle = simulation.summary.calc_cooling_entry_gate_lifecycle;
    assert_eq!(
        cooling_entry_lifecycle.source,
        PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_SOURCE
    );
    assert_eq!(
        cooling_entry_lifecycle.first_excluded_source,
        PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_FIRST_EXCLUDED_SOURCE
    );
    assert_eq!(
        cooling_entry_lifecycle.state.transition_count,
        required_steps
    );
    assert_eq!(
        cooling_entry_lifecycle.state.source_execution_count,
        required_steps
    );
    assert_eq!(cooling_entry_lifecycle.state.unit_off_skip_count, 0);
    assert_eq!(
        cooling_entry_lifecycle.state.sensible_comparison_count,
        required_steps
    );
    assert_eq!(
        cooling_entry_lifecycle
            .state
            .sensible_comparison_satisfied_count,
        0
    );
    assert_eq!(
        cooling_entry_lifecycle
            .state
            .temperature_control_type_read_count,
        0
    );
    assert_eq!(cooling_entry_lifecycle.state.single_heat_block_count, 0);
    assert_eq!(cooling_entry_lifecycle.state.cooling_body_entry_count, 0);
    assert_eq!(
        cooling_entry_lifecycle.state.active_fallthrough_count,
        required_steps
    );
    let latest_cooling_entry = cooling_entry_lifecycle
        .state
        .latest
        .expect("latest cooling-entry gate snapshot");
    assert_eq!(latest_cooling_entry.parent_call_ordinal, required_steps);
    assert_eq!(
        latest_cooling_entry.minimum_outdoor_air_sensible_output_w,
        Some(0.0)
    );
    assert_eq!(
        latest_cooling_entry.sensible_comparison_satisfied,
        Some(false)
    );
    assert!(!latest_cooling_entry.temperature_control_type_read);
    assert!(!latest_cooling_entry.cooling_body_entered);
    assert_eq!(latest_cooling_entry.assigned_operating_mode, None);
    let cooling_oa_gate = simulation.summary.calc_cooling_oa_max_flow_gate_lifecycle;
    assert_eq!(
        cooling_oa_gate.source,
        PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_SOURCE
    );
    assert_eq!(
        cooling_oa_gate.first_excluded_source,
        PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_GATE_FIRST_EXCLUDED_SOURCE
    );
    assert_eq!(cooling_oa_gate.state.transition_count, required_steps);
    assert_eq!(cooling_oa_gate.state.source_execution_count, 0);
    assert_eq!(cooling_oa_gate.state.unit_off_skip_count, 0);
    assert_eq!(cooling_oa_gate.state.non_cooling_skip_count, required_steps);
    assert_eq!(
        cooling_oa_gate
            .state
            .cooling_limit_flow_rate_comparison_count,
        0
    );
    assert_eq!(
        cooling_oa_gate.state.maximum_cooling_flow_body_entry_count,
        0
    );
    let latest_cooling_oa = cooling_oa_gate.state.latest.expect("latest CP313 snapshot");
    assert!(latest_cooling_oa.non_cooling_skipped);
    assert!(!latest_cooling_oa.cooling_limit_flow_rate_read);
    let cooling_oa_body = simulation.summary.calc_cooling_oa_max_flow_body_lifecycle;
    assert_eq!(
        cooling_oa_body.source,
        PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_SOURCE
    );
    assert_eq!(
        cooling_oa_body.first_excluded_source,
        PURCHASED_AIR_CALC_COOLING_OA_MAX_FLOW_BODY_FIRST_EXCLUDED_SOURCE
    );
    assert_eq!(cooling_oa_body.state.transition_count, required_steps);
    assert_eq!(cooling_oa_body.state.body_entry_count, 0);
    assert_eq!(cooling_oa_body.state.body_skip_count, required_steps);
    assert_eq!(cooling_oa_body.state.unit_off_skip_count, 0);
    assert_eq!(cooling_oa_body.state.non_cooling_skip_count, required_steps);
    assert_eq!(
        cooling_oa_body
            .state
            .active_guard_false_economizer_fallthrough_count,
        0
    );
    assert_eq!(
        cooling_oa_body.state.outdoor_air_mass_flow_rate_read_count,
        0
    );
    assert_eq!(cooling_oa_body.state.standard_air_density_read_count, 0);
    assert_eq!(cooling_oa_body.state.warning_counter_read_count, 0);
    assert_eq!(
        cooling_oa_body
            .state
            .characterized_total_warning_error_increment_count,
        0
    );
    assert_eq!(
        cooling_oa_body
            .state
            .outdoor_air_mass_flow_clamp_assignment_count,
        0
    );
    let latest_cooling_oa_body = cooling_oa_body.state.latest.expect("latest CP314 snapshot");
    assert!(latest_cooling_oa_body.body_skipped);
    assert!(latest_cooling_oa_body.non_cooling_skipped);
    assert!(!latest_cooling_oa_body.active_guard_false_economizer_fallthrough);
    assert!(!latest_cooling_oa_body.warning_counter_read);
    assert!(!latest_cooling_oa_body.outdoor_air_mass_flow_clamp_assignment_performed);
    let economizer_guard = simulation.summary.calc_cooling_economizer_guard_lifecycle;
    assert_eq!(
        economizer_guard.source,
        PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_SOURCE
    );
    assert_eq!(
        economizer_guard.first_excluded_source,
        PURCHASED_AIR_CALC_COOLING_ECONOMIZER_GUARD_FIRST_EXCLUDED_SOURCE
    );
    assert_eq!(economizer_guard.state.transition_count, required_steps);
    assert_eq!(economizer_guard.state.guard_evaluation_count, 0);
    assert_eq!(economizer_guard.state.unit_off_skip_count, 0);
    assert_eq!(
        economizer_guard.state.non_cooling_skip_count,
        required_steps
    );
    assert_eq!(
        economizer_guard
            .state
            .maximum_cooling_flow_body_sibling_skip_count,
        0
    );
    assert_eq!(economizer_guard.state.economizer_type_read_count, 0);
    assert_eq!(economizer_guard.state.no_economizer_comparison_count, 0);
    assert_eq!(economizer_guard.state.economizer_body_entry_count, 0);
    assert_eq!(economizer_guard.state.no_economizer_fallthrough_count, 0);
    let latest_economizer_guard = economizer_guard
        .state
        .latest
        .expect("latest CP315 snapshot");
    assert!(latest_economizer_guard.non_cooling_skipped);
    assert!(!latest_economizer_guard.economizer_guard_evaluated);
    assert_eq!(latest_economizer_guard.economizer_type, None);
    assert!(!latest_economizer_guard.economizer_body_entered);

    let zone = simulation.state.zones.first().expect("bound Zone state");
    assert_eq!(simulation.state.timestep_index, required_steps);
    assert!(!zone.use_zone_timestep_history);
    assert!(!zone.shorten_timestep_sys);
    assert_eq!(zone.previous_system_timestep_count, 1);
    assert_close(
        zone.prior_timestep_seconds,
        3_600.0 / f64::from(STEPS_PER_HOUR),
    );
    let heating_rate = simulation
        .results
        .find_series(SYSTEM_KEY, ZONE_IDEAL_LOADS_ZONE_TOTAL_HEATING_RATE)
        .expect("hourly heating rate");
    let heating_energy = simulation
        .results
        .find_series(SYSTEM_KEY, ZONE_IDEAL_LOADS_ZONE_TOTAL_HEATING_ENERGY)
        .expect("hourly heating energy");
    let heating_threshold = simulation
        .results
        .find_series(
            ZONE_KEY,
            ZONE_SYSTEM_PREDICTED_SENSIBLE_LOAD_TO_HEATING_SETPOINT_RATE,
        )
        .expect("source heating threshold");
    let cooling_threshold = simulation
        .results
        .find_series(
            ZONE_KEY,
            ZONE_SYSTEM_PREDICTED_SENSIBLE_LOAD_TO_COOLING_SETPOINT_RATE,
        )
        .expect("source cooling threshold");
    for series in [
        heating_rate,
        heating_energy,
        heating_threshold,
        cooling_threshold,
    ] {
        assert_eq!(series.values.len(), HOURS);
        assert!(series.values.iter().all(|value| value.is_finite()));
    }
    assert!(heating_rate.values.iter().any(|value| *value > 0.0));
    for (rate_w, energy_j) in heating_rate.values.iter().zip(&heating_energy.values) {
        assert_close(*energy_j, *rate_w * 3_600.0);
    }
    assert!(simulation.results.diagnostics().is_empty());
}

#[test]
fn all_hard_sized_finite_limit_branches_run_with_source_threshold_demand() {
    for (limit, expected_branch) in [
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
    ] {
        let mut typed = exact_model(STEPS_PER_HOUR).typed;
        let system = &mut typed.ideal_loads_air_systems[0];
        system.heating_limit = limit;
        system.maximum_heating_air_flow_rate_m3_per_s = Some(AutosizeOrNumber::Value(0.005));
        system.maximum_sensible_heating_capacity_w = Some(AutosizeOrNumber::Value(300.0));
        system.cooling_limit = limit;
        system.maximum_cooling_air_flow_rate_m3_per_s = Some(AutosizeOrNumber::Value(0.005));
        system.maximum_total_cooling_capacity_w = Some(AutosizeOrNumber::Value(300.0));
        let model = SimulationModel::from_typed(typed);
        let required_steps = STEPS_PER_HOUR as usize;
        let schedule_cache =
            precompute_schedule_cache(&model.typed, required_steps).expect("finite schedule cache");
        let weather = weather_series(&model, 1);

        let simulation = simulate_direct_zone_purchased_air_coupled_heat_balance(
            &model,
            &weather,
            &schedule_cache,
            DirectZonePurchasedAirCoupledOptions::hourly_samples(1),
        )
        .expect("hard-sized finite branch in the live coupled loop");

        assert_eq!(simulation.summary.branch, expected_branch);
        assert_eq!(simulation.summary.coupling_call_count, required_steps);
        assert_eq!(
            simulation.summary.zone_demand_source,
            DIRECT_ZONE_PURCHASED_AIR_DEMAND_SOURCE
        );
        assert!(!simulation.summary.fixture_demand_injection_used);
        assert_eq!(
            simulation.summary.recirculation_state_source,
            DIRECT_ZONE_PURCHASED_AIR_RECIRCULATION_SOURCE
        );
        assert_eq!(
            simulation.summary.actual_coupled_source_order,
            DIRECT_ZONE_PURCHASED_AIR_COUPLED_SOURCE_ORDER
        );
        assert_eq!(simulation.summary.return_node_name, RETURN_NODE_KEY);
        let lifecycle = simulation.summary.init_lifecycle;
        assert_eq!(lifecycle.init_call_count, required_steps);
        assert_eq!(lifecycle.environment_initialization_count, 1);
        assert_eq!(lifecycle.environment_rearm_count, 1);
        let calc_lifecycle = simulation.summary.calc_entry_lifecycle;
        assert_eq!(calc_lifecycle.state.call_count, required_steps);
        assert_eq!(calc_lifecycle.state.reset_count, required_steps);
        assert_eq!(calc_lifecycle.state.heating_on_count, required_steps);
        assert_eq!(calc_lifecycle.state.cooling_on_count, required_steps);
        assert_eq!(
            calc_lifecycle.state.availability_manager_read_count,
            required_steps
        );
        let minimum_oa_lifecycle = simulation.summary.calc_minimum_oa_prefix_lifecycle;
        assert_eq!(minimum_oa_lifecycle.state.transition_count, required_steps);
        assert_eq!(
            minimum_oa_lifecycle.state.source_execution_count,
            required_steps
        );
        assert_eq!(
            minimum_oa_lifecycle.state.minimum_oa_child_call_count,
            required_steps
        );
        assert_eq!(minimum_oa_lifecycle.state.ems_override_apply_count, 0);
        assert_eq!(minimum_oa_lifecycle.state.outdoor_air_effect_count, 0);
        let cooling_entry_lifecycle = simulation.summary.calc_cooling_entry_gate_lifecycle;
        assert_eq!(
            cooling_entry_lifecycle.state.transition_count,
            required_steps
        );
        assert_eq!(
            cooling_entry_lifecycle.state.source_execution_count,
            required_steps
        );
        assert_eq!(
            cooling_entry_lifecycle.state.sensible_comparison_count,
            required_steps
        );
        assert_eq!(cooling_entry_lifecycle.state.single_heat_block_count, 0);
        assert_eq!(cooling_entry_lifecycle.state.cooling_body_entry_count, 0);
        assert_eq!(
            cooling_entry_lifecycle.state.active_fallthrough_count,
            required_steps
        );
        let cooling_oa_gate = simulation.summary.calc_cooling_oa_max_flow_gate_lifecycle;
        assert_eq!(cooling_oa_gate.state.transition_count, required_steps);
        assert_eq!(cooling_oa_gate.state.source_execution_count, 0);
        assert_eq!(cooling_oa_gate.state.non_cooling_skip_count, required_steps);
        assert_eq!(cooling_oa_gate.state.strict_mass_flow_comparison_count, 0);
        assert_eq!(
            cooling_oa_gate.state.maximum_cooling_flow_body_entry_count,
            0
        );
        let cooling_oa_body = simulation.summary.calc_cooling_oa_max_flow_body_lifecycle;
        assert_eq!(cooling_oa_body.state.transition_count, required_steps);
        assert_eq!(cooling_oa_body.state.body_entry_count, 0);
        assert_eq!(cooling_oa_body.state.body_skip_count, required_steps);
        assert_eq!(cooling_oa_body.state.unit_off_skip_count, 0);
        assert_eq!(cooling_oa_body.state.non_cooling_skip_count, required_steps);
        assert_eq!(
            cooling_oa_body
                .state
                .active_guard_false_economizer_fallthrough_count,
            0
        );
        assert_eq!(
            cooling_oa_body.state.outdoor_air_mass_flow_rate_read_count,
            0
        );
        assert_eq!(cooling_oa_body.state.warning_counter_read_count, 0);
        assert_eq!(
            cooling_oa_body
                .state
                .outdoor_air_mass_flow_clamp_assignment_count,
            0
        );
        let latest_cooling_oa_body = cooling_oa_body
            .state
            .latest
            .expect("latest finite CP314 snapshot");
        assert!(latest_cooling_oa_body.body_skipped);
        assert!(latest_cooling_oa_body.non_cooling_skipped);
        let density = lifecycle
            .standard_air_density_kg_per_m3
            .expect("initialized standard density");
        let expected_mass_flow = if matches!(
            limit,
            IdealLoadsLimit::LimitFlowRate | IdealLoadsLimit::LimitFlowRateAndCapacity
        ) {
            0.005 * density
        } else {
            0.0
        };
        assert_close(
            lifecycle.maximum_heating_air_mass_flow_rate_kg_per_s,
            expected_mass_flow,
        );
        assert_close(
            lifecycle.maximum_cooling_air_mass_flow_rate_kg_per_s,
            expected_mass_flow,
        );

        let heating_rate = simulation
            .results
            .find_series(SYSTEM_KEY, ZONE_IDEAL_LOADS_ZONE_TOTAL_HEATING_RATE)
            .expect("finite hourly heating rate");
        let heating_energy = simulation
            .results
            .find_series(SYSTEM_KEY, ZONE_IDEAL_LOADS_ZONE_TOTAL_HEATING_ENERGY)
            .expect("finite hourly heating energy");
        let predicted_heating = simulation
            .results
            .find_series(
                ZONE_KEY,
                ZONE_SYSTEM_PREDICTED_SENSIBLE_LOAD_TO_HEATING_SETPOINT_RATE,
            )
            .expect("source heating threshold");
        assert!(heating_rate.values[0] > 0.0);
        assert!(
            heating_rate.values[0] < predicted_heating.values[0],
            "the deliberately small hard-sized limit must constrain the live predicted demand"
        );
        assert_close(heating_energy.values[0], heating_rate.values[0] * 3_600.0);
    }
}

#[test]
fn cooling_oa_max_flow_gate_reconciles_every_release_limit_shape() {
    for (limit, flow_m3_per_s, capacity_w) in [
        (IdealLoadsLimit::NoLimit, None, None),
        (IdealLoadsLimit::LimitCapacity, None, Some(1.0e9)),
        (IdealLoadsLimit::LimitFlowRate, Some(0.005), None),
        (
            IdealLoadsLimit::LimitFlowRateAndCapacity,
            Some(0.005),
            Some(1.0e9),
        ),
        (IdealLoadsLimit::LimitFlowRate, Some(0.0), None),
    ] {
        let mut typed = exact_model(1).typed;
        typed.schedules[1].hourly_value = 0.0;
        typed.schedules[2].hourly_value = 15.0;
        let system = &mut typed.ideal_loads_air_systems[0];
        system.cooling_limit = limit;
        system.maximum_cooling_air_flow_rate_m3_per_s = flow_m3_per_s.map(AutosizeOrNumber::Value);
        system.maximum_total_cooling_capacity_w = capacity_w.map(AutosizeOrNumber::Value);
        let model = SimulationModel::from_typed(typed);
        let schedule_cache =
            precompute_schedule_cache(&model.typed, 1).expect("one cooling schedule sample");
        let weather = weather_series_with_conditions(&model, 1, 30.0, 15.0, 30.0, 101_325.0);
        let mut options = DirectZonePurchasedAirCoupledOptions::hourly_samples(1);
        options.initial_zone_air_temperature_c = INITIAL_ZONE_TEMPERATURE_C;

        let simulation = simulate_direct_zone_purchased_air_coupled_heat_balance(
            &model,
            &weather,
            &schedule_cache,
            options,
        )
        .expect("CP313 release limit shape");
        let initialized_max = simulation
            .summary
            .init_lifecycle
            .maximum_cooling_air_mass_flow_rate_kg_per_s;
        let lifecycle = simulation.summary.calc_cooling_oa_max_flow_gate_lifecycle;
        let state = lifecycle.state;
        let flow_rate = limit == IdealLoadsLimit::LimitFlowRate;
        let combined = limit == IdealLoadsLimit::LimitFlowRateAndCapacity;
        let flow_active = flow_rate || combined;

        assert_eq!(state.transition_count, 1, "{limit:?}");
        assert_eq!(state.source_execution_count, 1, "{limit:?}");
        assert_eq!(state.unit_off_skip_count, 0, "{limit:?}");
        assert_eq!(state.non_cooling_skip_count, 0, "{limit:?}");
        assert_eq!(state.cooling_limit_flow_rate_comparison_count, 1);
        assert_eq!(
            state.cooling_limit_flow_rate_match_count,
            usize::from(flow_rate)
        );
        assert_eq!(
            state.cooling_limit_flow_rate_and_capacity_comparison_count,
            usize::from(!flow_rate)
        );
        assert_eq!(
            state.cooling_limit_flow_rate_and_capacity_match_count,
            usize::from(combined)
        );
        assert_eq!(
            state.outdoor_air_mass_flow_rate_read_count,
            usize::from(flow_active)
        );
        assert_eq!(
            state.maximum_cooling_air_mass_flow_rate_read_count,
            usize::from(flow_active)
        );
        assert_eq!(
            state.strict_mass_flow_comparison_count,
            usize::from(flow_active)
        );
        assert_eq!(state.strict_mass_flow_comparison_satisfied_count, 0);
        assert_eq!(state.maximum_cooling_flow_body_entry_count, 0);
        assert_eq!(state.active_fallthrough_count, 1);
        let latest = state.latest.expect("latest CP313 snapshot");
        assert!(latest.predecessor_cooling_body_entered);
        assert_eq!(latest.cooling_flow_limit_active, Some(flow_active));
        assert_eq!(latest.maximum_cooling_air_mass_flow_rate_read, flow_active);
        if flow_active {
            assert_eq!(
                latest
                    .maximum_cooling_air_mass_flow_rate_kg_per_s
                    .expect("selected-flow cache")
                    .to_bits(),
                initialized_max.to_bits()
            );
            assert_eq!(
                latest
                    .outdoor_air_mass_flow_rate_kg_per_s
                    .expect("selected-flow OA")
                    .to_bits(),
                0.0_f64.to_bits()
            );
        } else {
            assert_eq!(latest.maximum_cooling_air_mass_flow_rate_kg_per_s, None);
            assert_eq!(latest.outdoor_air_mass_flow_rate_kg_per_s, None);
        }
        assert!(!latest.maximum_cooling_flow_body_entered);
        let body_lifecycle = simulation.summary.calc_cooling_oa_max_flow_body_lifecycle;
        let body_state = body_lifecycle.state;
        assert_eq!(body_state.transition_count, 1, "{limit:?}");
        assert_eq!(body_state.body_entry_count, 0, "{limit:?}");
        assert_eq!(body_state.body_skip_count, 1, "{limit:?}");
        assert_eq!(body_state.unit_off_skip_count, 0, "{limit:?}");
        assert_eq!(body_state.non_cooling_skip_count, 0, "{limit:?}");
        assert_eq!(
            body_state.active_guard_false_economizer_fallthrough_count, 1,
            "{limit:?}"
        );
        assert_eq!(body_state.outdoor_air_mass_flow_rate_read_count, 0);
        assert_eq!(body_state.standard_air_density_read_count, 0);
        assert_eq!(body_state.outdoor_air_volume_flow_calculation_count, 0);
        assert_eq!(body_state.warning_counter_read_count, 0);
        assert_eq!(
            body_state.outdoor_air_flow_max_cooling_output_error_count,
            0
        );
        assert_eq!(body_state.outdoor_air_flow_max_cooling_output_index, 0);
        assert_eq!(
            body_state.characterized_total_warning_error_increment_count,
            0
        );
        assert_eq!(body_state.outdoor_air_mass_flow_clamp_assignment_count, 0);
        let latest_body = body_state.latest.expect("latest CP314 skip snapshot");
        assert!(latest_body.body_skipped);
        assert!(latest_body.active_guard_false_economizer_fallthrough);
        assert!(!latest_body.unit_off_skipped);
        assert!(!latest_body.non_cooling_skipped);
        assert!(!latest_body.warning_counter_read);
        assert!(!latest_body.outdoor_air_mass_flow_clamp_assignment_performed);
        let economizer_guard = simulation.summary.calc_cooling_economizer_guard_lifecycle;
        assert_eq!(economizer_guard.state.transition_count, 1, "{limit:?}");
        assert_eq!(
            economizer_guard.state.guard_evaluation_count, 1,
            "{limit:?}"
        );
        assert_eq!(economizer_guard.state.unit_off_skip_count, 0, "{limit:?}");
        assert_eq!(
            economizer_guard.state.non_cooling_skip_count, 0,
            "{limit:?}"
        );
        assert_eq!(
            economizer_guard
                .state
                .maximum_cooling_flow_body_sibling_skip_count,
            0,
            "{limit:?}"
        );
        assert_eq!(economizer_guard.state.economizer_type_read_count, 1);
        assert_eq!(economizer_guard.state.no_economizer_comparison_count, 1);
        assert_eq!(economizer_guard.state.economizer_body_entry_count, 0);
        assert_eq!(economizer_guard.state.no_economizer_fallthrough_count, 1);
        let latest_guard = economizer_guard
            .state
            .latest
            .expect("latest CP315 guard snapshot");
        assert!(latest_guard.economizer_guard_evaluated);
        assert!(latest_guard.economizer_type_read);
        assert_eq!(
            latest_guard.economizer_type,
            Some(OutdoorAirEconomizerType::NoEconomizer)
        );
        assert_eq!(latest_guard.economizer_not_no_economizer, Some(false));
        assert!(!latest_guard.economizer_body_entered);
        assert!(latest_guard.no_economizer_fallthrough);

        if flow_m3_per_s == Some(0.0) {
            let mass_flow = simulation
                .results
                .find_series(SYSTEM_KEY, ZONE_IDEAL_LOADS_SUPPLY_AIR_MASS_FLOW_RATE)
                .expect("zero-limit numerical mass flow");
            assert!(
                mass_flow.values[0] > 0.0,
                "CP313 OA guard must not claim ownership of the later positive-only supply-flow clamp"
            );
        }
    }
}

#[test]
fn live_coupling_uses_weather_pressure_for_supply_saturation() {
    const WEATHER_PRESSURE_PA: f64 = 101_325.0;
    const SITE_ELEVATION_M: f64 = 3_000.0;

    let mut typed = exact_model(1).typed;
    typed.site = Some(SiteLocation {
        name: NormalizedName::new("HIGH SITE"),
        latitude_deg: 0.0,
        longitude_deg: 0.0,
        time_zone_hours: 0.0,
        elevation_m: SITE_ELEVATION_M,
    });
    typed.schedules[1].hourly_value = 0.0;
    typed.schedules[2].hourly_value = 15.0;
    let system = &mut typed.ideal_loads_air_systems[0];
    system.cooling_limit = IdealLoadsLimit::LimitCapacity;
    system.maximum_total_cooling_capacity_w = Some(AutosizeOrNumber::Value(1.0e9));
    let model = SimulationModel::from_typed(typed);
    let schedule_cache =
        precompute_schedule_cache(&model.typed, 1).expect("one-step cooling schedule cache");
    let weather = weather_series_with_conditions(&model, 1, 30.0, 30.0, 100.0, WEATHER_PRESSURE_PA);
    let mut options = DirectZonePurchasedAirCoupledOptions::hourly_samples(1);
    options.initial_zone_air_temperature_c = INITIAL_ZONE_TEMPERATURE_C;

    let simulation = simulate_direct_zone_purchased_air_coupled_heat_balance(
        &model,
        &weather,
        &schedule_cache,
        options,
    )
    .expect("live finite cooling with weather-derived saturation pressure");

    let cooling_entry = &simulation.summary.calc_cooling_entry_gate_lifecycle;
    assert_eq!(cooling_entry.state.transition_count, 1);
    assert_eq!(cooling_entry.state.sensible_comparison_satisfied_count, 1);
    assert_eq!(cooling_entry.state.temperature_control_type_read_count, 1);
    assert_eq!(cooling_entry.state.single_heat_block_count, 0);
    assert_eq!(cooling_entry.state.cooling_body_entry_count, 1);
    assert_eq!(cooling_entry.state.operating_mode_assignment_count, 1);
    assert_eq!(cooling_entry.state.active_fallthrough_count, 0);
    let latest_cooling_entry = cooling_entry
        .state
        .latest
        .expect("live cooling-entry snapshot");
    assert_eq!(
        latest_cooling_entry.temperature_control_type,
        Some(PurchasedAirTemperatureControlType::DualHeatCool)
    );
    assert!(latest_cooling_entry.cooling_body_entered);
    assert_eq!(
        latest_cooling_entry.assigned_operating_mode,
        Some(IdealLoadsSensibleMode::Cooling)
    );

    let supply_temperature_c = simulation
        .results
        .find_series(SYSTEM_KEY, ZONE_IDEAL_LOADS_SUPPLY_AIR_TEMPERATURE)
        .expect("supply temperature")
        .values[0];
    let supply_humidity_ratio = simulation
        .results
        .find_series(SYSTEM_KEY, ZONE_IDEAL_LOADS_SUPPLY_AIR_HUMIDITY_RATIO)
        .expect("supply humidity ratio")
        .values[0];
    let expected_weather_saturation = crate::energyplus_psychrometric_humidity_ratio_from_rh(
        supply_temperature_c,
        1.0,
        WEATHER_PRESSURE_PA,
    )
    .expect("weather-pressure saturation humidity ratio");
    let site_pressure_pa = IdealLoadsSensibleLimitContext::from_site_elevation_m(SITE_ELEVATION_M)
        .expect("site-derived context")
        .barometric_pressure_pa;
    let stale_site_saturation = crate::energyplus_psychrometric_humidity_ratio_from_rh(
        supply_temperature_c,
        1.0,
        site_pressure_pa,
    )
    .expect("site-pressure saturation humidity ratio");

    assert_close(supply_humidity_ratio, expected_weather_saturation);
    assert!(
        (supply_humidity_ratio - stale_site_saturation).abs() > 1.0e-3,
        "the live clamp must not reuse site-derived standard pressure"
    );
}

#[test]
fn one_step_forced_heating_commits_feedback_into_the_returned_state() {
    let model = exact_model(1);
    let schedule_cache =
        precompute_schedule_cache(&model.typed, 1).expect("one-step schedule cache");
    let weather = weather_series(&model, 1);
    let mut options = DirectZonePurchasedAirCoupledOptions::hourly_samples(1);
    options.initial_zone_air_temperature_c = INITIAL_ZONE_TEMPERATURE_C;

    let simulation = simulate_direct_zone_purchased_air_coupled_heat_balance(
        &model,
        &weather,
        &schedule_cache,
        options,
    )
    .expect("one forced-heating predictor/PurchasedAir/corrector step");

    assert_eq!(simulation.summary.coupling_call_count, 1);
    let zone = simulation.state.zones.first().expect("bound Zone state");
    assert!(
        zone.sum_sys_mcp_w_per_k > 0.0,
        "the active one-step PurchasedAir call must commit system-air feedback"
    );
    assert_close(
        zone.sum_sys_mcp_t_w / zone.sum_sys_mcp_w_per_k,
        model.typed.ideal_loads_air_systems[0].maximum_heating_supply_air_temperature_c,
    );
    assert!(
        zone.mean_air_temperature_c > INITIAL_ZONE_TEMPERATURE_C,
        "the same-step corrector must consume the committed feedback"
    );
    let heating_threshold = simulation
        .results
        .find_series(
            ZONE_KEY,
            ZONE_SYSTEM_PREDICTED_SENSIBLE_LOAD_TO_HEATING_SETPOINT_RATE,
        )
        .expect("one-step source heating threshold");
    assert!(heating_threshold.values[0] > 0.0);
}

#[test]
fn undersized_schedule_cache_is_a_preflight_coverage_error() {
    let model = exact_model(STEPS_PER_HOUR);
    let required = HOURS * STEPS_PER_HOUR as usize;
    let available = required - 1;
    let schedule_cache =
        precompute_schedule_cache(&model.typed, available).expect("undersized cache fixture");
    let original_cache = schedule_cache.clone();
    let weather = weather_series(&model, HOURS);

    let error = simulate_direct_zone_purchased_air_coupled_heat_balance(
        &model,
        &weather,
        &schedule_cache,
        DirectZonePurchasedAirCoupledOptions::hourly_samples(HOURS),
    )
    .expect_err("cache coverage must fail before state initialization or stepping");

    assert_eq!(
        error,
        DirectZonePurchasedAirCoupledRuntimeError::ScheduleCacheCoverage {
            required,
            available,
        }
    );
    assert_eq!(
        schedule_cache, original_cache,
        "the preflight error does not expose a stepped state and makes no rollback claim"
    );
}

#[test]
fn binder_failure_is_returned_before_coupled_execution() {
    let mut model = exact_model(STEPS_PER_HOUR);
    model.typed.ideal_loads_air_systems[0].zone_supply_air_node_name =
        NormalizedName::new("ZONE AIR");
    let schedule_cache =
        precompute_schedule_cache(&model.typed, STEPS_PER_HOUR as usize).expect("schedule cache");
    let weather = weather_series(&model, 1);

    let error = simulate_direct_zone_purchased_air_coupled_heat_balance(
        &model,
        &weather,
        &schedule_cache,
        DirectZonePurchasedAirCoupledOptions::hourly_samples(1),
    )
    .expect_err("stale typed/graph model must fail binding");

    assert_eq!(
        error,
        DirectZonePurchasedAirCoupledRuntimeError::Binding(
            DirectZonePurchasedAirBindingError::UnsupportedFeature {
                feature: DirectZonePurchasedAirBindingFeature::CoherentTypedModelGraph,
            }
        )
    );
}

fn exact_model(steps_per_hour: u32) -> SimulationModel {
    let mut typed = TypedModel::default();
    typed.timestep.number_of_timesteps_per_hour = steps_per_hour;
    for (id, name, value) in [
        (ScheduleId(0), "CONTROL TYPE", 4.0),
        (ScheduleId(1), "HEATING SETPOINT", HEATING_SETPOINT_C),
        (ScheduleId(2), "COOLING SETPOINT", COOLING_SETPOINT_C),
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
        name: NormalizedName::new(ZONE_KEY),
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
        (NodeId(0), SUPPLY_NODE_KEY),
        (NodeId(1), "ZONE AIR"),
        (NodeId(2), RETURN_NODE_KEY),
    ] {
        typed.nodes.push(Node {
            id,
            name: NormalizedName::new(name),
        });
        typed.node_names.insert(name, id);
    }
    typed
        .ideal_loads_air_systems
        .push(exact_ideal_loads_system());
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
            zone_air_inlet_node_or_nodelist_name: Some(NormalizedName::new(SUPPLY_NODE_KEY)),
            zone_air_exhaust_node_or_nodelist_name: None,
            zone_air_node_name: NormalizedName::new("ZONE AIR"),
            zone_return_air_node_or_nodelist_name: Some(NormalizedName::new(RETURN_NODE_KEY)),
            zone_return_air_node_1_flow_rate_fraction_schedule: None,
            zone_return_air_node_1_flow_rate_basis_node_or_nodelist_name: None,
        });
    SimulationModel::from_typed(typed)
}

fn exact_ideal_loads_system() -> IdealLoadsAirSystem {
    IdealLoadsAirSystem {
        id: IdealLoadsAirSystemId(0),
        name: NormalizedName::new(SYSTEM_KEY),
        availability_schedule: Some(ScheduleId(3)),
        zone_supply_air_node_name: NormalizedName::new(SUPPLY_NODE_KEY),
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

fn weather_series(model: &SimulationModel, hours: usize) -> WeatherTimestepSeries {
    weather_series_with_conditions(model, hours, 5.0, 0.0, 50.0, 101_325.0)
}

fn weather_series_with_conditions(
    model: &SimulationModel,
    hours: usize,
    dry_bulb_c: f64,
    dew_point_c: f64,
    relative_humidity_percent: f64,
    atmospheric_pressure_pa: f64,
) -> WeatherTimestepSeries {
    let records = (0..hours)
        .map(|hour_index| EpwRecord {
            year: 2013,
            month: 1,
            day: 1,
            hour: hour_index as u32 + 1,
            minute: 60,
            dry_bulb_c,
            dew_point_c,
            relative_humidity_percent,
            atmospheric_pressure_pa,
            horizontal_infrared_radiation_wh_per_m2: 0.0,
            global_horizontal_radiation_wh_per_m2: 0.0,
            direct_normal_radiation_wh_per_m2: 0.0,
            diffuse_horizontal_radiation_wh_per_m2: 0.0,
            wind_direction_deg: 0.0,
            wind_speed_m_per_s: 0.0,
            liquid_precipitation_depth_mm: 0.0,
        })
        .collect::<Vec<_>>();
    WeatherTimestepSeries::from_records(
        &records,
        model.typed.timestep.number_of_timesteps_per_hour,
        run_period_first_hour_interpolation_starting_values(&model.typed),
    )
}

fn assert_close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() <= ABS_TOLERANCE,
        "expected {expected}, got {actual}"
    );
}
