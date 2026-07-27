use super::*;

use crate::{
    heat_balance::state::{ZoneAirTemperatureCoefficients, ZoneHeatBalanceState},
    ideal_loads::{
        DirectZonePurchasedAirCouplingInput, DirectZonePurchasedAirScheduleSnapshot,
        IdealLoadsInitFlags, IdealLoadsZoneState, PurchasedAirInitSnapshot,
        PurchasedAirInitTransition, PurchasedAirRecirculationSource,
        couple_direct_zone_predicted_demand_to_purchased_air,
    },
};
use ep_model::{
    DehumidificationControlType, DemandControlledVentilationType, HeatRecoveryType,
    HumidificationControlType, IdealLoadsAirSystemId, IdealLoadsFuelType, IdealLoadsLimit,
    NormalizedName, OutdoorAirEconomizerType, OutputHandle, ZoneId,
};

const SYSTEM_KEY: &str = "ZONE ONE IDEAL LOADS";
const ZONE_KEY: &str = "ZONE ONE";
const SUPPLY_NODE_NAME: &str = "ZONE ONE INLET";
const SUPPLY_NODE: NodeId = NodeId(3);
const ABS_TOLERANCE: f64 = 1.0e-9;

#[test]
fn appends_all_no_oa_and_predictor_series_with_hourly_semantics() {
    let system = test_system();
    let outputs = (0..8)
        .map(|index| scaled_output(&system, index, (index + 1) as f64))
        .collect::<Vec<_>>();
    for output in &outputs {
        assert_eq!(
            output.coupling.prediction.zone_demand.sensible_input_kind,
            ZoneSensibleDemandInputKind::SourceSetpointThresholds
        );
    }

    let mut results = ResultStore::new();
    results.add_series(OutputSeries {
        handle: OutputHandle(7),
        key: "EXISTING".to_string(),
        variable_name: "Existing Variable".to_string(),
        units: "W".to_string(),
        values: vec![1.0, 2.0],
    });
    let limit_context = IdealLoadsSensibleLimitContext {
        standard_air_density_kg_per_m3: 2.0,
        barometric_pressure_pa: 101_325.0,
        ..IdealLoadsSensibleLimitContext::default()
    };

    append_direct_zone_purchased_air_hourly_output_series(
        &mut results,
        &system,
        ZONE_KEY,
        SUPPLY_NODE,
        SUPPLY_NODE_NAME,
        limit_context,
        &outputs,
        4,
        900.0,
    )
    .expect("valid fixed-timestep hourly outputs");

    assert_eq!(results.series.len(), 26);
    for (offset, series) in results.series.iter().skip(1).enumerate() {
        assert_eq!(series.handle, OutputHandle(8 + offset as u32));
    }
    assert!(results.diagnostics().is_empty());

    let rate_multipliers = [
        (ZONE_IDEAL_LOADS_ZONE_TOTAL_HEATING_RATE, 1.0),
        (ZONE_IDEAL_LOADS_ZONE_TOTAL_COOLING_RATE, 2.0),
        (ZONE_IDEAL_LOADS_ZONE_SENSIBLE_HEATING_RATE, 3.0),
        (ZONE_IDEAL_LOADS_ZONE_SENSIBLE_COOLING_RATE, 4.0),
        (ZONE_IDEAL_LOADS_ZONE_LATENT_HEATING_RATE, 5.0),
        (ZONE_IDEAL_LOADS_ZONE_LATENT_COOLING_RATE, 6.0),
        (ZONE_IDEAL_LOADS_SUPPLY_AIR_SENSIBLE_HEATING_RATE, 7.0),
        (ZONE_IDEAL_LOADS_SUPPLY_AIR_SENSIBLE_COOLING_RATE, 8.0),
        (ZONE_IDEAL_LOADS_SUPPLY_AIR_LATENT_HEATING_RATE, 9.0),
        (ZONE_IDEAL_LOADS_SUPPLY_AIR_LATENT_COOLING_RATE, 10.0),
        (ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_HEATING_RATE, 11.0),
        (ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_COOLING_RATE, 12.0),
    ];
    for (variable, multiplier) in rate_multipliers {
        assert_values(
            results
                .find_series(SYSTEM_KEY, variable)
                .expect("rate series"),
            &[2.5 * multiplier, 6.5 * multiplier],
        );
    }

    assert_values(
        results
            .find_series(SYSTEM_KEY, ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_HEATING_ENERGY)
            .expect("supply heating energy"),
        &[11.0 * 10.0 * 900.0, 11.0 * 26.0 * 900.0],
    );
    assert_values(
        results
            .find_series(SYSTEM_KEY, ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_COOLING_ENERGY)
            .expect("supply cooling energy"),
        &[12.0 * 10.0 * 900.0, 12.0 * 26.0 * 900.0],
    );
    assert_values(
        results
            .find_series(SYSTEM_KEY, ZONE_IDEAL_LOADS_ZONE_TOTAL_HEATING_ENERGY)
            .expect("zone heating energy"),
        &[10.0 * 900.0, 26.0 * 900.0],
    );
    assert_values(
        results
            .find_series(SYSTEM_KEY, ZONE_IDEAL_LOADS_ZONE_TOTAL_COOLING_ENERGY)
            .expect("zone cooling energy"),
        &[2.0 * 10.0 * 900.0, 2.0 * 26.0 * 900.0],
    );

    assert_values(
        results
            .find_series(SYSTEM_KEY, ZONE_IDEAL_LOADS_SUPPLY_AIR_MASS_FLOW_RATE)
            .expect("supply mass flow"),
        &[13.0 * 2.5, 13.0 * 6.5],
    );
    assert_values(
        results
            .find_series(
                SYSTEM_KEY,
                ZONE_IDEAL_LOADS_SUPPLY_AIR_STANDARD_DENSITY_VOLUME_FLOW_RATE,
            )
            .expect("supply volume flow"),
        &[13.0 * 2.5 / 2.0, 13.0 * 6.5 / 2.0],
    );
    assert_values(
        results
            .find_series(SYSTEM_KEY, ZONE_IDEAL_LOADS_SUPPLY_AIR_TEMPERATURE)
            .expect("supply temperature"),
        &[14.0 * 2.5, 14.0 * 6.5],
    );
    assert_values(
        results
            .find_series(SYSTEM_KEY, ZONE_IDEAL_LOADS_SUPPLY_AIR_HUMIDITY_RATIO)
            .expect("supply humidity ratio"),
        &[15.0 * 2.5, 15.0 * 6.5],
    );
    assert_values(
        results
            .find_series(SUPPLY_NODE_NAME, SYSTEM_NODE_TEMPERATURE)
            .expect("node temperature"),
        &[16.0 * 2.5, 16.0 * 6.5],
    );
    assert_values(
        results
            .find_series(SUPPLY_NODE_NAME, SYSTEM_NODE_HUMIDITY_RATIO)
            .expect("node humidity ratio"),
        &[17.0 * 2.5, 17.0 * 6.5],
    );
    assert_values(
        results
            .find_series(SUPPLY_NODE_NAME, SYSTEM_NODE_MASS_FLOW_RATE)
            .expect("node mass flow"),
        &[18.0 * 2.5, 18.0 * 6.5],
    );
    assert_values(
        results
            .find_series(
                ZONE_KEY,
                ZONE_SYSTEM_PREDICTED_SENSIBLE_LOAD_TO_HEATING_SETPOINT_RATE,
            )
            .expect("heating threshold"),
        &[19.0 * 2.5, 19.0 * 6.5],
    );
    assert_values(
        results
            .find_series(
                ZONE_KEY,
                ZONE_SYSTEM_PREDICTED_SENSIBLE_LOAD_TO_COOLING_SETPOINT_RATE,
            )
            .expect("cooling threshold"),
        &[-20.0 * 2.5, -20.0 * 6.5],
    );
}

#[test]
fn validation_errors_leave_the_result_store_unchanged() {
    let system = test_system();
    let outputs = vec![
        scaled_output(&system, 0, 1.0),
        scaled_output(&system, 1, 2.0),
    ];
    let valid_context = IdealLoadsSensibleLimitContext::default();
    let original = sentinel_results(OutputHandle(7));

    assert_rejected_without_mutation(
        original.clone(),
        &system,
        &outputs,
        0,
        900.0,
        valid_context,
        DirectZonePurchasedAirHourlyOutputError::ZeroZoneTimestepsPerHour,
    );
    assert_rejected_without_mutation(
        original.clone(),
        &system,
        &outputs,
        2,
        0.0,
        valid_context,
        DirectZonePurchasedAirHourlyOutputError::InvalidTimestepSeconds { value: 0.0 },
    );
    assert_rejected_without_mutation(
        original.clone(),
        &system,
        &outputs[..1],
        2,
        900.0,
        valid_context,
        DirectZonePurchasedAirHourlyOutputError::OutputCountNotDivisible {
            output_count: 1,
            zone_timesteps_per_hour: 2,
        },
    );

    let mut wrong_node = outputs.clone();
    wrong_node[1].coupling.purchased_air.supply_node_update.node = NodeId(99);
    assert_rejected_without_mutation(
        original.clone(),
        &system,
        &wrong_node,
        2,
        900.0,
        valid_context,
        DirectZonePurchasedAirHourlyOutputError::SupplyNodeMismatch {
            timestep_index: 1,
            expected: SUPPLY_NODE,
            actual: NodeId(99),
        },
    );

    let mut wrong_demand_kind = outputs.clone();
    wrong_demand_kind[0]
        .coupling
        .prediction
        .zone_demand
        .sensible_input_kind = ZoneSensibleDemandInputKind::ActiveLoadSplitCompatibility;
    assert_rejected_without_mutation(
        original.clone(),
        &system,
        &wrong_demand_kind,
        2,
        900.0,
        valid_context,
        DirectZonePurchasedAirHourlyOutputError::UnexpectedDemandInputKind {
            timestep_index: 0,
            actual: ZoneSensibleDemandInputKind::ActiveLoadSplitCompatibility,
        },
    );

    assert_rejected_without_mutation(
        original,
        &system,
        &outputs,
        2,
        900.0,
        IdealLoadsSensibleLimitContext {
            standard_air_density_kg_per_m3: 0.0,
            barometric_pressure_pa: 101_325.0,
            ..IdealLoadsSensibleLimitContext::default()
        },
        DirectZonePurchasedAirHourlyOutputError::InvalidStandardAirDensity { value: 0.0 },
    );
}

#[test]
fn rejects_nonfinite_timestep_and_exhausted_handle_range() {
    let system = test_system();
    let outputs = vec![scaled_output(&system, 0, 1.0)];
    let mut results = ResultStore::new();
    let error = append_direct_zone_purchased_air_hourly_output_series(
        &mut results,
        &system,
        ZONE_KEY,
        SUPPLY_NODE,
        SUPPLY_NODE_NAME,
        IdealLoadsSensibleLimitContext::default(),
        &outputs,
        1,
        f64::NAN,
    )
    .expect_err("NaN timestep must fail");
    assert!(matches!(
        error,
        DirectZonePurchasedAirHourlyOutputError::InvalidTimestepSeconds { value }
            if value.is_nan()
    ));
    assert!(results.series.is_empty());

    let mut results = sentinel_results(OutputHandle(u32::MAX - 10));
    let original = results.clone();
    assert_eq!(
        append_direct_zone_purchased_air_hourly_output_series(
            &mut results,
            &system,
            ZONE_KEY,
            SUPPLY_NODE,
            SUPPLY_NODE_NAME,
            IdealLoadsSensibleLimitContext::default(),
            &outputs,
            1,
            3_600.0,
        ),
        Err(
            DirectZonePurchasedAirHourlyOutputError::OutputHandleSpaceExhausted {
                maximum_existing_handle: Some(u32::MAX - 10),
            }
        )
    );
    assert_eq!(results, original);

    let mut results = sentinel_results(OutputHandle(u32::MAX - 25));
    append_direct_zone_purchased_air_hourly_output_series(
        &mut results,
        &system,
        ZONE_KEY,
        SUPPLY_NODE,
        SUPPLY_NODE_NAME,
        IdealLoadsSensibleLimitContext::default(),
        &outputs,
        1,
        3_600.0,
    )
    .expect("the final assigned handle may equal u32::MAX");
    assert_eq!(
        results.series.last().map(|series| series.handle),
        Some(OutputHandle(u32::MAX))
    );
}

fn assert_rejected_without_mutation(
    mut results: ResultStore,
    system: &IdealLoadsAirSystem,
    outputs: &[DirectZonePurchasedAirScheduledCouplingOutput],
    zone_timesteps_per_hour: u32,
    timestep_seconds: f64,
    limit_context: IdealLoadsSensibleLimitContext,
    expected_error: DirectZonePurchasedAirHourlyOutputError,
) {
    let original = results.clone();
    assert_eq!(
        append_direct_zone_purchased_air_hourly_output_series(
            &mut results,
            system,
            ZONE_KEY,
            SUPPLY_NODE,
            SUPPLY_NODE_NAME,
            limit_context,
            outputs,
            zone_timesteps_per_hour,
            timestep_seconds,
        ),
        Err(expected_error)
    );
    assert_eq!(results, original);
}

fn sentinel_results(handle: OutputHandle) -> ResultStore {
    let mut results = ResultStore::new();
    results.add_series(OutputSeries {
        handle,
        key: "EXISTING".to_string(),
        variable_name: "Existing Variable".to_string(),
        units: "W".to_string(),
        values: vec![1.0],
    });
    results
}

fn scaled_output(
    system: &IdealLoadsAirSystem,
    sample_index: usize,
    scale: f64,
) -> DirectZonePurchasedAirScheduledCouplingOutput {
    let mut state = zone_state();
    let air_humidity_ratio = state.air_humidity_ratio;
    let coupling =
        couple_direct_zone_predicted_demand_to_purchased_air(DirectZonePurchasedAirCouplingInput {
            zone_state: &mut state,
            heating_setpoint_c: 20.0,
            cooling_setpoint_c: 24.0,
            zone_node_temperature_c: 22.0,
            recirculation_state: IdealLoadsZoneState {
                air_temperature_c: 22.0,
                air_humidity_ratio,
            },
            load_correction_factor: 1.0,
            zone_multiplier: 1,
            zone_list_multiplier: 1,
            system_timestep_seconds: 900.0,
            system,
            supply_node: SUPPLY_NODE,
            recirculation_node: NodeId(4),
            unit_available: true,
            limit_context: IdealLoadsSensibleLimitContext::default(),
            initialization: initialized_snapshot(system),
        })
        .expect("valid bounded coupling fixture");
    let mut output = DirectZonePurchasedAirScheduledCouplingOutput {
        schedules: DirectZonePurchasedAirScheduleSnapshot {
            sample_index,
            control_type: 4.0,
            heating_setpoint_c: 20.0,
            cooling_setpoint_c: 24.0,
            overall_availability: 1.0,
            unit_available: true,
        },
        initialization: initialized_snapshot(system),
        coupling,
    };
    let report = &mut output.coupling.purchased_air.report;
    report.zone_total_heating_rate_w = scale;
    report.zone_total_cooling_rate_w = 2.0 * scale;
    report.zone_sensible_heating_rate_w = 3.0 * scale;
    report.zone_sensible_cooling_rate_w = 4.0 * scale;
    report.zone_latent_heating_rate_w = 5.0 * scale;
    report.zone_latent_cooling_rate_w = 6.0 * scale;
    report.supply_air_sensible_heating_rate_w = 7.0 * scale;
    report.supply_air_sensible_cooling_rate_w = 8.0 * scale;
    report.supply_air_latent_heating_rate_w = 9.0 * scale;
    report.supply_air_latent_cooling_rate_w = 10.0 * scale;
    report.supply_air_total_heating_rate_w = 11.0 * scale;
    report.supply_air_total_cooling_rate_w = 12.0 * scale;
    report.supply_mass_flow_rate_kg_per_s = 13.0 * scale;
    report.supply_temperature_c = 14.0 * scale;
    report.supply_humidity_ratio = 15.0 * scale;

    let node = &mut output.coupling.purchased_air.supply_node_update;
    node.temperature_c = 16.0 * scale;
    node.humidity_ratio = 17.0 * scale;
    node.mass_flow_rate_kg_per_s = 18.0 * scale;

    let demand = &mut output.coupling.prediction.zone_demand;
    demand.remaining_output_req_to_heat_sp_w = 19.0 * scale;
    demand.remaining_output_req_to_cool_sp_w = -20.0 * scale;
    output
}

fn initialized_snapshot(system: &IdealLoadsAirSystem) -> PurchasedAirInitSnapshot {
    PurchasedAirInitSnapshot {
        system: system.id,
        controlled_zone: ZoneId(0),
        supply_node: SUPPLY_NODE,
        recirculation_node: Some(NodeId(4)),
        recirculation_source: Some(PurchasedAirRecirculationSource::SingleZoneReturn),
        rejected_exhaust_node: None,
        reported_first_return_node: None,
        topology_diagnostic_count: 0,
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
        transition: PurchasedAirInitTransition::default(),
        maximum_heating_air_mass_flow_rate_kg_per_s: 0.0,
        maximum_cooling_air_mass_flow_rate_kg_per_s: 0.0,
        standard_air_density_kg_per_m3: Some(
            IdealLoadsSensibleLimitContext::default().standard_air_density_kg_per_m3,
        ),
    }
}

fn zone_state() -> ZoneHeatBalanceState {
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
        prior_timestep_seconds: 900.0,
        volume_m3: 100.0,
        air_heat_capacity_j_per_k: 0.0,
        convective_internal_gain_w: 0.0,
        opaque_surface_conductance_w_per_k: 100.0,
        opaque_surface_heat_gain_w: 0.0,
        opaque_surface_outside_conduction_w: 0.0,
        sum_ha_w_per_k: 100.0,
        sum_hat_surf_w: 0.0,
        sum_hat_ref_w: 0.0,
        sum_mcp_w_per_k: 0.0,
        sum_mcp_t_w: 0.0,
        sum_sys_mcp_w_per_k: 0.0,
        sum_sys_mcp_t_w: 0.0,
        system_dependent_zone_loads_lagged_w: 0.0,
        zone_air_temperature_coefficients: ZoneAirTemperatureCoefficients::ZERO,
        system_timestep_average_surface_convection_report_w: None,
        system_timestep_average_air_storage_report_w: None,
    }
}

fn test_system() -> IdealLoadsAirSystem {
    IdealLoadsAirSystem {
        id: IdealLoadsAirSystemId(0),
        name: NormalizedName::new(SYSTEM_KEY),
        availability_schedule: None,
        zone_supply_air_node_name: NormalizedName::new(SUPPLY_NODE_NAME),
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

fn assert_values(series: &OutputSeries, expected: &[f64]) {
    assert_eq!(series.values.len(), expected.len());
    for (actual, expected) in series.values.iter().zip(expected) {
        assert!(
            (actual - expected).abs() <= ABS_TOLERANCE,
            "expected {expected}, got {actual} for {}",
            series.variable_name
        );
    }
}
