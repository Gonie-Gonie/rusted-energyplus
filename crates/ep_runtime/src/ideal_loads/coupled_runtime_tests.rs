use super::*;

use crate::{
    ideal_loads::{
        DirectZonePurchasedAirBindingFeature, ZONE_IDEAL_LOADS_ZONE_TOTAL_HEATING_ENERGY,
        ZONE_IDEAL_LOADS_ZONE_TOTAL_HEATING_RATE,
        ZONE_SYSTEM_PREDICTED_SENSIBLE_LOAD_TO_COOLING_SETPOINT_RATE,
        ZONE_SYSTEM_PREDICTED_SENSIBLE_LOAD_TO_HEATING_SETPOINT_RATE,
    },
    schedules::precompute_schedule_cache,
    time_axis::run_period_first_hour_interpolation_starting_values,
    weather::{EpwRecord, WeatherTimestepSeries},
};
use ep_model::{
    AutoOrNumber, DehumidificationControlType, DemandControlledVentilationType, HeatRecoveryType,
    HumidificationControlType, IdealLoadsAirSystem, IdealLoadsAirSystemId, IdealLoadsFuelType,
    IdealLoadsLimit, LoadDistributionScheme, Node, NodeId, NormalizedName,
    OutdoorAirEconomizerType, Point3, ScheduleConstant, ScheduleId, SimulationModel,
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
        simulation.summary.actual_coupled_source_order,
        DIRECT_ZONE_PURCHASED_AIR_COUPLED_SOURCE_ORDER
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
    for (id, name) in [(NodeId(0), SUPPLY_NODE_KEY), (NodeId(1), "ZONE AIR")] {
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
            zone_return_air_node_or_nodelist_name: None,
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
    let records = (0..hours)
        .map(|hour_index| EpwRecord {
            year: 2013,
            month: 1,
            day: 1,
            hour: hour_index as u32 + 1,
            minute: 60,
            dry_bulb_c: 5.0,
            dew_point_c: 0.0,
            relative_humidity_percent: 50.0,
            atmospheric_pressure_pa: 101_325.0,
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
