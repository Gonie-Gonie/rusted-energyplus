use super::*;
use ep_model::{
    DehumidificationControlType, DemandControlledVentilationType, HeatRecoveryType,
    HumidificationControlType, IdealLoadsAirSystem, IdealLoadsAirSystemId, IdealLoadsFuelType,
    IdealLoadsLimit, NormalizedName, OutdoorAirEconomizerType, ZoneId,
};

const MEAN_HUMIDITY_SEED: [f64; 3] = [0.0085, 0.0082, 0.0081];
const AIR_HUMIDITY_SEED: [f64; 3] = [0.0084, 0.0083, 0.0082];

#[test]
fn no_oa_humidistat_closed_loop_advances_corrected_humidity_histories() {
    let system = test_system();
    let mut state = seeded_state();
    let input = test_step_input(&system, 1.0);

    let output = advance_no_oa_humidistat_zone_timestep_compat(&mut state, input)
        .expect("closed-loop step should succeed");

    let corrected = output.humidity_correction.zone_air_humidity_ratio;
    assert_eq!(
        state.zone_mean_air_humidity_ratio_history(),
        [corrected, MEAN_HUMIDITY_SEED[0], MEAN_HUMIDITY_SEED[1]]
    );
    assert_eq!(
        state.zone_air_humidity_ratio_history(),
        [corrected, AIR_HUMIDITY_SEED[0], AIR_HUMIDITY_SEED[1]]
    );
    assert_eq!(
        output
            .purchased_air
            .trace
            .demand
            .remaining_output_req_to_humid_sp_kg_per_s,
        output.moisture_demand.humidifying_setpoint_load_kg_per_s
    );
    assert_eq!(
        output
            .purchased_air
            .trace
            .demand
            .remaining_output_req_to_dehumid_sp_kg_per_s,
        output.moisture_demand.dehumidifying_setpoint_load_kg_per_s
    );
    assert_eq!(
        output.purchased_air.trace.zone_state.air_temperature_c,
        input.purchased_air_zone_temperature_c
    );
    assert_eq!(
        output
            .purchased_air
            .trace
            .recirculation_state
            .air_temperature_c,
        input.recirculation_air_temperature_c
    );
}

#[test]
fn no_oa_humidistat_closed_loop_reuses_committed_history_on_next_step() {
    let mut system = test_system();
    system.dehumidification_control_type = DehumidificationControlType::None;
    system.humidification_control_type = HumidificationControlType::Humidistat;
    let mut state = seeded_state();

    advance_no_oa_humidistat_zone_timestep_compat(&mut state, humidification_step_input(&system))
        .expect("first closed-loop step should succeed");
    let committed_state = state;
    let current_humidity_ratio = committed_state.zone_air_humidity_ratio_history()[0];
    let expected_moisture_demand =
        calc_no_oa_third_order_moisture_demand_compat(NoOaThirdOrderMoistureDemandInput {
            zone_state: IdealLoadsZoneState {
                air_temperature_c: 22.0,
                air_humidity_ratio: current_humidity_ratio,
            },
            previous_zone_timestep_humidity_ratios: committed_state
                .zone_mean_air_humidity_ratio_history(),
            zone_volume_m3: 100.0,
            zone_moisture_capacity_multiplier: 1.0,
            timestep_seconds: 900.0,
            barometric_pressure_pa: 101_325.0,
            latent_gain_w: 600.0,
            humidifying_relative_humidity_percent: 30.0,
            dehumidifying_relative_humidity_percent: 60.0,
            zone_multiplier: 1.0,
        })
        .expect("committed history should remain valid predictor input");

    let second = advance_no_oa_humidistat_zone_timestep_compat(
        &mut state,
        humidification_step_input(&system),
    )
    .expect("second closed-loop step should succeed");

    assert_eq!(second.moisture_demand, expected_moisture_demand);
    assert_eq!(
        second.purchased_air.trace.zone_state.air_humidity_ratio,
        current_humidity_ratio
    );
    assert_eq!(
        state.zone_mean_air_humidity_ratio_history()[1],
        committed_state.zone_mean_air_humidity_ratio_history()[0]
    );
}

#[test]
fn no_oa_humidistat_closed_loop_divides_supply_flow_by_zone_multiplier_before_correction() {
    let system = test_system();
    let mut state = seeded_state();
    let state_before = state;
    let input = test_step_input(&system, 2.0);

    let output = advance_no_oa_humidistat_zone_timestep_compat(&mut state, input)
        .expect("multiplied closed-loop step should succeed");
    let expected =
        correct_no_oa_third_order_humidity_ratio_compat(NoOaThirdOrderHumidityCorrectorInput {
            zone_state: IdealLoadsZoneState {
                air_temperature_c: input.corrector_zone_air_temperature_c,
                air_humidity_ratio: state_before.zone_air_humidity_ratio_history()[0],
            },
            previous_zone_timestep_humidity_ratios: state_before
                .zone_mean_air_humidity_ratio_history(),
            zone_volume_m3: input.zone_volume_m3,
            zone_moisture_capacity_multiplier: input.zone_moisture_capacity_multiplier,
            timestep_seconds: input.zone_timestep_seconds,
            barometric_pressure_pa: input.barometric_pressure_pa,
            latent_gain_w: input.latent_gain_w,
            supply_mass_flow_rate_kg_per_s: output
                .purchased_air
                .calculation
                .supply_mass_flow_rate_kg_per_s
                / 2.0,
            supply_humidity_ratio: output.purchased_air.calculation.supply_humidity_ratio,
        })
        .expect("independent corrector input should be valid");

    assert_eq!(output.humidity_correction, expected);
}

#[test]
fn no_oa_humidistat_closed_loop_preserves_state_when_predictor_rejects_input() {
    let system = test_system();
    let mut state = seeded_state();
    let state_before = state;
    let mut input = test_step_input(&system, 1.0);
    input.zone_volume_m3 = 0.0;

    let error = advance_no_oa_humidistat_zone_timestep_compat(&mut state, input)
        .expect_err("zero zone volume should be rejected");

    assert_eq!(
        error,
        NoOaHumidistatZoneTimestepError::MoisturePredictorRejected
    );
    assert_eq!(state, state_before);
}

#[test]
fn no_oa_humidistat_closed_loop_rejects_non_humidistat_branch_without_advancing_state() {
    let mut system = test_system();
    system.dehumidification_control_type = DehumidificationControlType::None;
    let mut state = seeded_state();
    let state_before = state;

    let error =
        advance_no_oa_humidistat_zone_timestep_compat(&mut state, test_step_input(&system, 1.0))
            .expect_err("sensible-only system must not enter the Humidistat state transition");

    assert_eq!(
        error,
        NoOaHumidistatZoneTimestepError::UnsupportedBranch(
            IdealLoadsPurchasedAirBranch::NoOaNoLimitSensible
        )
    );
    assert_eq!(state, state_before);
}

#[test]
fn no_oa_humidistat_closed_loop_preserves_state_when_purchased_air_rejects_input() {
    let mut system = test_system();
    system.demand_controlled_ventilation_type = DemandControlledVentilationType::OccupancySchedule;
    let mut state = seeded_state();
    let state_before = state;

    let error =
        advance_no_oa_humidistat_zone_timestep_compat(&mut state, test_step_input(&system, 1.0))
            .expect_err("unsupported DCV must be rejected by PurchasedAir");

    assert!(matches!(
        error,
        NoOaHumidistatZoneTimestepError::PurchasedAir(_)
    ));
    assert_eq!(state, state_before);
}

#[test]
fn no_oa_humidistat_closed_loop_preserves_state_when_corrector_rejects_input() {
    let system = test_system();
    let mut state = seeded_state();
    let state_before = state;
    let mut input = test_step_input(&system, 1.0);
    input.corrector_zone_air_temperature_c = f64::NAN;

    let error = advance_no_oa_humidistat_zone_timestep_compat(&mut state, input)
        .expect_err("non-finite corrector temperature should be rejected");

    assert_eq!(
        error,
        NoOaHumidistatZoneTimestepError::HumidityCorrectorRejected
    );
    assert_eq!(state, state_before);
}

fn seeded_state() -> NoOaHumidistatClosedLoopState {
    NoOaHumidistatClosedLoopState::from_seed_histories(MEAN_HUMIDITY_SEED, AIR_HUMIDITY_SEED)
}

fn test_step_input(
    system: &IdealLoadsAirSystem,
    zone_multiplier: f64,
) -> NoOaHumidistatZoneTimestepInput<'_> {
    NoOaHumidistatZoneTimestepInput {
        system,
        supply_node: ep_model::NodeId(0),
        sensible_demand: ZoneSysEnergyDemand::sensible_only(ZoneId(0), 0.0, -1_000.0),
        predictor_zone_air_temperature_c: 22.0,
        purchased_air_zone_temperature_c: 24.0,
        recirculation_air_temperature_c: 26.0,
        corrector_zone_air_temperature_c: 23.0,
        zone_volume_m3: 100.0,
        zone_moisture_capacity_multiplier: 1.0,
        zone_timestep_seconds: 900.0,
        barometric_pressure_pa: 101_325.0,
        latent_gain_w: 600.0,
        humidifying_relative_humidity_percent: 10.0,
        dehumidifying_relative_humidity_percent: 45.0,
        zone_multiplier,
        unit_available: true,
        limit_context: IdealLoadsSensibleLimitContext::default(),
    }
}

fn humidification_step_input(system: &IdealLoadsAirSystem) -> NoOaHumidistatZoneTimestepInput<'_> {
    NoOaHumidistatZoneTimestepInput {
        sensible_demand: ZoneSysEnergyDemand::sensible_only(ZoneId(0), 1_000.0, 0.0),
        humidifying_relative_humidity_percent: 30.0,
        dehumidifying_relative_humidity_percent: 60.0,
        ..test_step_input(system, 1.0)
    }
}

fn test_system() -> IdealLoadsAirSystem {
    IdealLoadsAirSystem {
        id: IdealLoadsAirSystemId(0),
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
        dehumidification_control_type: DehumidificationControlType::Humidistat,
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
