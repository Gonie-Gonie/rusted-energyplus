use super::*;

use crate::{
    ideal_loads::{
        DirectZonePurchasedAirBindingFeature, IdealLoadsSensibleLimitContext,
        PURCHASED_AIR_CALC_MINIMUM_OA_CHILD_SOURCE, PURCHASED_AIR_CALC_MINIMUM_OA_PREFIX_SOURCE,
        PURCHASED_AIR_INIT_LIFECYCLE_SOURCE, ZONE_IDEAL_LOADS_SUPPLY_AIR_HUMIDITY_RATIO,
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
