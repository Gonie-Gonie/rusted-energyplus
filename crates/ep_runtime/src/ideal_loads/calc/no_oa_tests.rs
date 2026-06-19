use super::{
    limits::IdealLoadsSensibleLimitContext,
    moisture_demand::{
        NoOaThirdOrderMoistureDemandInput, calc_no_oa_third_order_moisture_demand_compat,
    },
    no_oa::*,
    psychrometrics::{
        DEFAULT_STANDARD_AIR_DENSITY_KG_PER_M3, energyplus_standard_air_density_kg_per_m3,
        moist_air_enthalpy_j_per_kg,
    },
    types::*,
};
use crate::zone_equipment::ZoneSysEnergyDemand;
use crate::{
    energyplus_moist_air_density_kg_per_m3, energyplus_moist_air_specific_heat_j_per_kg_k,
    energyplus_water_vapor_gas_enthalpy_j_per_kg,
};
use ep_model::{
    AutosizeOrNumber, DehumidificationControlType, DemandControlledVentilationType,
    HeatRecoveryType, HumidificationControlType, IdealLoadsAirSystem, IdealLoadsAirSystemId,
    IdealLoadsFuelType, IdealLoadsLimit, NormalizedName, OutdoorAirEconomizerType, ZoneId,
};

#[test]
fn no_oa_sensible_heating_uses_supply_delta_t_and_moist_air_cp() {
    let system = test_system();
    let zone_state = IdealLoadsZoneState {
        air_temperature_c: 20.0,
        air_humidity_ratio: 0.008,
    };
    let result = calc_no_oa_no_limit_sensible_compat(
        &system,
        zone_state,
        ZoneSysEnergyDemand::sensible_only(ZoneId(0), 3000.0, 0.0),
        true,
    );

    let cp = energyplus_moist_air_specific_heat_j_per_kg_k(0.008);
    assert_eq!(result.mode, IdealLoadsSensibleMode::Heating);
    assert!((result.cp_air_j_per_kg_k - cp).abs() < 1.0e-12);
    assert!((result.supply_temperature_c - 50.0).abs() < 1.0e-12);
    assert!((result.supply_mass_flow_rate_kg_per_s - 3000.0 / (cp * 30.0)).abs() < 1.0e-12);
    assert!((result.zone_total_heating_rate_w - 3000.0).abs() < 1.0e-12);
    assert_eq!(result.zone_total_cooling_rate_w, 0.0);
}

#[test]
fn no_oa_sensible_cooling_uses_absolute_cooling_demand() {
    let system = test_system();
    let zone_state = IdealLoadsZoneState {
        air_temperature_c: 25.0,
        air_humidity_ratio: 0.008,
    };
    let result = calc_no_oa_no_limit_sensible_compat(
        &system,
        zone_state,
        ZoneSysEnergyDemand::sensible_only(ZoneId(0), 0.0, -2400.0),
        true,
    );

    let cp = energyplus_moist_air_specific_heat_j_per_kg_k(0.008);
    assert_eq!(result.mode, IdealLoadsSensibleMode::Cooling);
    assert!((result.supply_temperature_c - 13.0).abs() < 1.0e-12);
    assert!((result.supply_mass_flow_rate_kg_per_s - 2400.0 / (cp * 12.0)).abs() < 1.0e-12);
    assert!((result.zone_sensible_cooling_rate_w - 2400.0).abs() < 1.0e-12);
    assert_eq!(result.zone_total_heating_rate_w, 0.0);
}

#[test]
fn constant_sensible_heat_ratio_cooling_adds_latent_output() {
    let mut system = test_system();
    system.dehumidification_control_type = DehumidificationControlType::ConstantSensibleHeatRatio;
    system.cooling_sensible_heat_ratio = 0.7;
    let zone_state = IdealLoadsZoneState {
        air_temperature_c: 25.0,
        air_humidity_ratio: 0.009,
    };

    let result = calc_no_oa_no_limit_sensible_compat(
        &system,
        zone_state,
        ZoneSysEnergyDemand::sensible_only(ZoneId(0), 0.0, -2400.0),
        true,
    );

    let cp = energyplus_moist_air_specific_heat_j_per_kg_k(zone_state.air_humidity_ratio);
    let mass_flow = 2400.0 / (cp * 12.0);
    let supply_enthalpy =
        moist_air_enthalpy_j_per_kg(result.supply_temperature_c, result.supply_humidity_ratio);
    let zone_enthalpy =
        moist_air_enthalpy_j_per_kg(zone_state.air_temperature_c, zone_state.air_humidity_ratio);
    let sensible_output_to_zone = mass_flow * cp * (13.0 - 25.0);
    let expected_latent_cooling = (mass_flow * (supply_enthalpy - zone_enthalpy)
        - sensible_output_to_zone)
        .min(0.0)
        .abs();

    assert_eq!(result.mode, IdealLoadsSensibleMode::Cooling);
    assert_close(result.supply_temperature_c, 13.0, 1.0e-12);
    assert_close(
        result.supply_humidity_ratio,
        system.minimum_cooling_supply_air_humidity_ratio,
        1.0e-12,
    );
    assert_close(result.supply_air_sensible_cooling_rate_w, 2400.0, 1.0e-9);
    assert!(result.supply_air_latent_cooling_rate_w > 0.0);
    assert_close(
        result.zone_latent_cooling_rate_w,
        expected_latent_cooling,
        1.0e-9,
    );
    assert_close(
        result.zone_total_cooling_rate_w,
        2400.0 + expected_latent_cooling,
        1.0e-9,
    );
}

#[test]
fn constant_supply_humidity_ratio_cooling_uses_minimum_cooling_humidity() {
    let mut system = test_system();
    system.dehumidification_control_type = DehumidificationControlType::ConstantSupplyHumidityRatio;
    system.minimum_cooling_supply_air_humidity_ratio = 0.0077;
    let zone_state = IdealLoadsZoneState {
        air_temperature_c: 25.0,
        air_humidity_ratio: 0.009,
    };

    let result = calc_no_oa_no_limit_sensible_compat(
        &system,
        zone_state,
        ZoneSysEnergyDemand::sensible_only(ZoneId(0), 0.0, -2400.0),
        true,
    );

    assert_eq!(result.mode, IdealLoadsSensibleMode::Cooling);
    assert_close(
        result.supply_humidity_ratio,
        system.minimum_cooling_supply_air_humidity_ratio,
        1.0e-12,
    );
    assert!(result.zone_latent_cooling_rate_w > 0.0);
    assert!(result.supply_air_latent_cooling_rate_w > 0.0);
}

#[test]
fn constant_supply_humidity_ratio_cooling_can_humidify_mixed_air_when_heat_available() {
    let mut system = test_system();
    system.dehumidification_control_type = DehumidificationControlType::ConstantSupplyHumidityRatio;
    system.minimum_cooling_supply_air_humidity_ratio = 0.009;
    let zone_state = IdealLoadsZoneState {
        air_temperature_c: 25.0,
        air_humidity_ratio: 0.008,
    };

    let result = calc_no_oa_no_limit_sensible_compat(
        &system,
        zone_state,
        ZoneSysEnergyDemand::sensible_only(ZoneId(0), 0.0, -2400.0),
        true,
    );

    assert_eq!(result.mode, IdealLoadsSensibleMode::Cooling);
    assert_close(
        result.supply_humidity_ratio,
        system.minimum_cooling_supply_air_humidity_ratio,
        1.0e-12,
    );
    assert!(result.zone_latent_heating_rate_w > 0.0);
    assert!(result.supply_air_latent_heating_rate_w > 0.0);
}

#[test]
fn humidistat_dehumidification_can_drive_cooling_without_sensible_load() {
    let mut system = test_system();
    system.dehumidification_control_type = DehumidificationControlType::Humidistat;
    system.minimum_cooling_supply_air_humidity_ratio = 0.0077;
    let zone_state = IdealLoadsZoneState {
        air_temperature_c: 24.0,
        air_humidity_ratio: 0.011,
    };
    let mut demand = ZoneSysEnergyDemand::sensible_only(ZoneId(0), 0.0, 0.0);
    demand.remaining_output_req_to_dehumid_sp_kg_per_s = -0.00033;

    let result = calc_no_oa_no_limit_sensible_compat(&system, zone_state, demand, true);

    assert_eq!(result.mode, IdealLoadsSensibleMode::Cooling);
    assert_close(result.supply_mass_flow_rate_kg_per_s, 0.1, 1.0e-12);
    assert_close(
        result.supply_temperature_c,
        zone_state.air_temperature_c,
        1.0e-12,
    );
    assert_close(
        result.supply_humidity_ratio,
        system.minimum_cooling_supply_air_humidity_ratio,
        1.0e-12,
    );
    assert_close(result.zone_sensible_cooling_rate_w, 0.0, 1.0e-9);
    assert!(result.zone_latent_cooling_rate_w > 0.0);
}

#[test]
fn no_oa_third_order_moisture_demand_matches_energyplus_predictor_formula() {
    let zone_state = IdealLoadsZoneState {
        air_temperature_c: 24.0,
        air_humidity_ratio: 0.008,
    };
    let input = NoOaThirdOrderMoistureDemandInput {
        zone_state,
        previous_zone_timestep_humidity_ratios: [0.0085, 0.0082, 0.0081],
        zone_volume_m3: 1.0,
        zone_moisture_capacity_multiplier: 1.0,
        timestep_seconds: 900.0,
        barometric_pressure_pa: 83_411.819_665_895_82,
        latent_gain_w: 600.0,
        humidifying_relative_humidity_percent: 10.0,
        dehumidifying_relative_humidity_percent: 45.0,
        zone_multiplier: 2.0,
    };

    let result = calc_no_oa_third_order_moisture_demand_compat(input)
        .expect("predictor input should be valid");

    let density = energyplus_moist_air_density_kg_per_m3(
        input.barometric_pressure_pa,
        zone_state.air_temperature_c,
        zone_state.air_humidity_ratio,
    )
    .expect("density should be valid");
    let c = density * input.zone_volume_m3 * input.zone_moisture_capacity_multiplier / 900.0;
    let b = input.latent_gain_w
        / energyplus_water_vapor_gas_enthalpy_j_per_kg(zone_state.air_temperature_c);
    let history = 3.0 * 0.0085 - 1.5 * 0.0082 + (1.0 / 3.0) * 0.0081;
    let expected_humidifying = ((11.0 / 6.0) * c * result.humidifying_setpoint_humidity_ratio
        - (b + c * history))
        * input.zone_multiplier;
    let expected_dehumidifying = ((11.0 / 6.0) * c * result.dehumidifying_setpoint_humidity_ratio
        - (b + c * history))
        * input.zone_multiplier;

    assert_close(
        result.humidifying_setpoint_load_kg_per_s,
        expected_humidifying,
        1.0e-15,
    );
    assert_close(
        result.dehumidifying_setpoint_load_kg_per_s,
        expected_dehumidifying,
        1.0e-15,
    );
    assert_close(
        result.total_output_required_kg_per_s,
        expected_dehumidifying,
        1.0e-15,
    );
}

#[test]
fn no_oa_third_order_moisture_demand_clamps_invalid_setpoint_order() {
    let input = NoOaThirdOrderMoistureDemandInput {
        zone_state: IdealLoadsZoneState {
            air_temperature_c: 22.0,
            air_humidity_ratio: 0.007,
        },
        previous_zone_timestep_humidity_ratios: [0.007, 0.007, 0.007],
        zone_volume_m3: 10.0,
        zone_moisture_capacity_multiplier: 1.0,
        timestep_seconds: 900.0,
        barometric_pressure_pa: 101_325.0,
        latent_gain_w: 0.0,
        humidifying_relative_humidity_percent: 60.0,
        dehumidifying_relative_humidity_percent: 45.0,
        zone_multiplier: 1.0,
    };

    let result = calc_no_oa_third_order_moisture_demand_compat(input)
        .expect("predictor input should be valid");

    assert_close(
        result.humidifying_setpoint_humidity_ratio,
        result.dehumidifying_setpoint_humidity_ratio,
        1.0e-15,
    );
    assert_close(
        result.total_output_required_kg_per_s,
        result.humidifying_setpoint_load_kg_per_s,
        1.0e-15,
    );
}

#[test]
fn humidistat_dehumidification_mass_flow_can_exceed_sensible_cooling_flow() {
    let mut system = test_system();
    system.dehumidification_control_type = DehumidificationControlType::Humidistat;
    system.minimum_cooling_supply_air_humidity_ratio = 0.0077;
    let zone_state = IdealLoadsZoneState {
        air_temperature_c: 25.0,
        air_humidity_ratio: 0.011,
    };
    let mut demand = ZoneSysEnergyDemand::sensible_only(ZoneId(0), 0.0, -1000.0);
    demand.remaining_output_req_to_dehumid_sp_kg_per_s = -0.00033;

    let result = calc_no_oa_no_limit_sensible_compat(&system, zone_state, demand, true);

    let cp = energyplus_moist_air_specific_heat_j_per_kg_k(zone_state.air_humidity_ratio);
    let sensible_mass_flow = 1000.0 / (cp * (25.0 - 13.0));
    assert_eq!(result.mode, IdealLoadsSensibleMode::Cooling);
    assert!(result.supply_mass_flow_rate_kg_per_s > sensible_mass_flow);
    assert_close(result.supply_mass_flow_rate_kg_per_s, 0.1, 1.0e-12);
    assert!(result.supply_temperature_c > system.minimum_cooling_supply_air_temperature_c);
    assert_close(result.zone_sensible_cooling_rate_w, 1000.0, 1.0e-9);
    assert!(result.zone_latent_cooling_rate_w > 0.0);
}

#[test]
fn humidistat_dehumidification_can_coexist_with_sensible_heating() {
    let mut system = test_system();
    system.dehumidification_control_type = DehumidificationControlType::Humidistat;
    system.minimum_cooling_supply_air_humidity_ratio = 0.0077;
    let zone_state = IdealLoadsZoneState {
        air_temperature_c: 20.0,
        air_humidity_ratio: 0.011,
    };
    let mut demand = ZoneSysEnergyDemand::sensible_only(ZoneId(0), 1000.0, 0.0);
    demand.remaining_output_req_to_dehumid_sp_kg_per_s = -0.00033;

    let result = calc_no_oa_no_limit_sensible_compat(&system, zone_state, demand, true);

    let cp = energyplus_moist_air_specific_heat_j_per_kg_k(zone_state.air_humidity_ratio);
    let sensible_mass_flow = 1000.0 / (cp * (50.0 - 20.0));
    assert_eq!(result.mode, IdealLoadsSensibleMode::Heating);
    assert!(result.supply_mass_flow_rate_kg_per_s > sensible_mass_flow);
    assert_close(result.supply_mass_flow_rate_kg_per_s, 0.1, 1.0e-12);
    assert_close(result.zone_sensible_heating_rate_w, 1000.0, 1.0e-9);
    assert_close(
        result.supply_humidity_ratio,
        system.minimum_cooling_supply_air_humidity_ratio,
        1.0e-12,
    );
    assert!(result.supply_air_sensible_heating_rate_w > 0.0);
    assert!(result.supply_air_latent_cooling_rate_w > 0.0);
    assert!(result.supply_air_total_heating_rate_w > 0.0);
    assert!(result.supply_air_total_cooling_rate_w > 0.0);
    assert!(result.zone_latent_cooling_rate_w > 0.0);
}

#[test]
fn humidistat_humidification_mass_flow_can_exceed_sensible_heating_flow() {
    let mut system = test_system();
    system.humidification_control_type = HumidificationControlType::Humidistat;
    system.maximum_heating_supply_air_humidity_ratio = 0.0156;
    let zone_state = IdealLoadsZoneState {
        air_temperature_c: 20.0,
        air_humidity_ratio: 0.008,
    };
    let mut demand = ZoneSysEnergyDemand::sensible_only(ZoneId(0), 3000.0, 0.0);
    demand.remaining_output_req_to_humid_sp_kg_per_s = 0.001;

    let result = calc_no_oa_no_limit_sensible_compat(&system, zone_state, demand, true);

    let cp = energyplus_moist_air_specific_heat_j_per_kg_k(zone_state.air_humidity_ratio);
    let sensible_mass_flow = 3000.0 / (cp * (50.0 - 20.0));
    let humidification_mass_flow =
        0.001 / (system.maximum_heating_supply_air_humidity_ratio - zone_state.air_humidity_ratio);
    assert_eq!(result.mode, IdealLoadsSensibleMode::Heating);
    assert!(result.supply_mass_flow_rate_kg_per_s > sensible_mass_flow);
    assert_close(
        result.supply_mass_flow_rate_kg_per_s,
        humidification_mass_flow,
        1.0e-12,
    );
    assert!(result.supply_temperature_c < system.maximum_heating_supply_air_temperature_c);
    assert_close(result.zone_sensible_heating_rate_w, 3000.0, 1.0e-9);
    assert_close(
        result.supply_humidity_ratio,
        system.maximum_heating_supply_air_humidity_ratio,
        1.0e-12,
    );
    assert!(result.zone_latent_heating_rate_w > 0.0);
    assert!(result.supply_air_latent_heating_rate_w > 0.0);
    assert!(result.zone_total_heating_rate_w > result.zone_sensible_heating_rate_w);
}

#[test]
fn constant_supply_humidity_ratio_heating_uses_maximum_heating_humidity() {
    let mut system = test_system();
    system.humidification_control_type = HumidificationControlType::ConstantSupplyHumidityRatio;
    system.maximum_heating_supply_air_humidity_ratio = 0.0156;
    let zone_state = IdealLoadsZoneState {
        air_temperature_c: 20.0,
        air_humidity_ratio: 0.008,
    };

    let result = calc_no_oa_no_limit_sensible_compat(
        &system,
        zone_state,
        ZoneSysEnergyDemand::sensible_only(ZoneId(0), 3000.0, 0.0),
        true,
    );

    assert_eq!(result.mode, IdealLoadsSensibleMode::Heating);
    assert_close(
        result.supply_humidity_ratio,
        system.maximum_heating_supply_air_humidity_ratio,
        1.0e-12,
    );
    assert!(result.zone_latent_heating_rate_w > 0.0);
    assert!(result.supply_air_latent_heating_rate_w > 0.0);
    assert!(result.zone_total_heating_rate_w > result.zone_sensible_heating_rate_w);
}

#[test]
fn unavailable_unit_writes_dead_flow_and_zone_condition() {
    let system = test_system();
    let zone_state = IdealLoadsZoneState {
        air_temperature_c: 22.5,
        air_humidity_ratio: 0.007,
    };
    let result = calc_no_oa_no_limit_sensible_compat(
        &system,
        zone_state,
        ZoneSysEnergyDemand::sensible_only(ZoneId(0), 3000.0, -3000.0),
        false,
    );

    assert_eq!(result.mode, IdealLoadsSensibleMode::Off);
    assert_eq!(result.supply_mass_flow_rate_kg_per_s, 0.0);
    assert!((result.supply_temperature_c - 22.5).abs() < 1.0e-12);
}

#[test]
fn standard_air_density_uses_energyplus_elevation_formula() {
    let density = energyplus_standard_air_density_kg_per_m3(1829.0)
        .expect("valid Golden CO elevation standard density");
    assert_close(density, 0.965_081_520_139_901_8, 1.0e-12);

    let context = IdealLoadsSensibleLimitContext::from_site_elevation_m(1829.0)
        .expect("valid Golden CO IdealLoads limit context");
    assert_close(context.standard_air_density_kg_per_m3, density, 1.0e-12);
}

#[test]
fn limit_aware_helper_matches_no_limit_heating_result() {
    let system = test_system();
    let zone_state = IdealLoadsZoneState {
        air_temperature_c: 20.0,
        air_humidity_ratio: 0.008,
    };
    let demand = ZoneSysEnergyDemand::sensible_only(ZoneId(0), 3000.0, 0.0);

    let expected = calc_no_oa_no_limit_sensible_compat(&system, zone_state, demand, true);
    let actual = calc_no_oa_sensible_with_limits_compat(
        &system,
        zone_state,
        demand,
        true,
        IdealLoadsSensibleLimitContext::default(),
    );

    assert_eq!(actual.mode, expected.mode);
    assert_close(
        actual.supply_temperature_c,
        expected.supply_temperature_c,
        1.0e-12,
    );
    assert_close(
        actual.supply_mass_flow_rate_kg_per_s,
        expected.supply_mass_flow_rate_kg_per_s,
        1.0e-12,
    );
    assert_close(
        actual.zone_total_heating_rate_w,
        expected.zone_total_heating_rate_w,
        1.0e-9,
    );
}

#[test]
fn heating_flow_limit_clamps_mass_flow_and_actual_output() {
    let mut system = test_system();
    system.heating_limit = IdealLoadsLimit::LimitFlowRate;
    system.maximum_heating_air_flow_rate_m3_per_s = Some(AutosizeOrNumber::Value(0.05));
    let zone_state = IdealLoadsZoneState {
        air_temperature_c: 20.0,
        air_humidity_ratio: 0.008,
    };

    let result = calc_no_oa_sensible_with_limits_compat(
        &system,
        zone_state,
        ZoneSysEnergyDemand::sensible_only(ZoneId(0), 3000.0, 0.0),
        true,
        IdealLoadsSensibleLimitContext::default(),
    );

    let cp = energyplus_moist_air_specific_heat_j_per_kg_k(0.008);
    let maximum_mass_flow_rate_kg_per_s = 0.05 * DEFAULT_STANDARD_AIR_DENSITY_KG_PER_M3;
    let expected_output_w = maximum_mass_flow_rate_kg_per_s * cp * 30.0;
    assert_eq!(result.mode, IdealLoadsSensibleMode::Heating);
    assert_close(
        result.supply_mass_flow_rate_kg_per_s,
        maximum_mass_flow_rate_kg_per_s,
        1.0e-12,
    );
    assert_close(result.supply_temperature_c, 50.0, 1.0e-12);
    assert_close(result.zone_total_heating_rate_w, expected_output_w, 1.0e-9);
    assert!(result.zone_total_heating_rate_w < 3000.0);
}

#[test]
fn heating_capacity_limit_caps_output_and_adjusts_supply_temperature() {
    let mut system = test_system();
    system.heating_limit = IdealLoadsLimit::LimitCapacity;
    system.maximum_sensible_heating_capacity_w = Some(AutosizeOrNumber::Value(1000.0));
    let zone_state = IdealLoadsZoneState {
        air_temperature_c: 20.0,
        air_humidity_ratio: 0.008,
    };

    let result = calc_no_oa_sensible_with_limits_compat(
        &system,
        zone_state,
        ZoneSysEnergyDemand::sensible_only(ZoneId(0), 3000.0, 0.0),
        true,
        IdealLoadsSensibleLimitContext::default(),
    );

    let cp = energyplus_moist_air_specific_heat_j_per_kg_k(0.008);
    let unlimited_mass_flow_rate_kg_per_s = 3000.0 / (cp * 30.0);
    let expected_supply_temperature_c = 20.0 + 1000.0 / (cp * unlimited_mass_flow_rate_kg_per_s);
    assert_eq!(result.mode, IdealLoadsSensibleMode::Heating);
    assert_close(result.zone_total_heating_rate_w, 1000.0, 1.0e-12);
    assert_close(
        result.supply_temperature_c,
        expected_supply_temperature_c,
        1.0e-12,
    );
}

#[test]
fn cooling_flow_limit_clamps_mass_flow_and_actual_output() {
    let mut system = test_system();
    system.cooling_limit = IdealLoadsLimit::LimitFlowRate;
    system.maximum_cooling_air_flow_rate_m3_per_s = Some(AutosizeOrNumber::Value(0.05));
    let zone_state = IdealLoadsZoneState {
        air_temperature_c: 25.0,
        air_humidity_ratio: 0.008,
    };

    let result = calc_no_oa_sensible_with_limits_compat(
        &system,
        zone_state,
        ZoneSysEnergyDemand::sensible_only(ZoneId(0), 0.0, -2400.0),
        true,
        IdealLoadsSensibleLimitContext::default(),
    );

    let cp = energyplus_moist_air_specific_heat_j_per_kg_k(0.008);
    let maximum_mass_flow_rate_kg_per_s = 0.05 * DEFAULT_STANDARD_AIR_DENSITY_KG_PER_M3;
    let expected_output_w = maximum_mass_flow_rate_kg_per_s * cp * 12.0;
    assert_eq!(result.mode, IdealLoadsSensibleMode::Cooling);
    assert_close(
        result.supply_mass_flow_rate_kg_per_s,
        maximum_mass_flow_rate_kg_per_s,
        1.0e-12,
    );
    assert_close(result.supply_temperature_c, 13.0, 1.0e-12);
    assert_close(
        result.zone_sensible_cooling_rate_w,
        expected_output_w,
        1.0e-9,
    );
    assert_close(
        result.supply_air_sensible_cooling_rate_w,
        expected_output_w,
        1.0e-9,
    );
    assert!(result.zone_sensible_cooling_rate_w < 2400.0);
}

#[test]
fn cooling_capacity_limit_caps_output_and_adjusts_supply_temperature() {
    let mut system = test_system();
    system.cooling_limit = IdealLoadsLimit::LimitCapacity;
    system.maximum_total_cooling_capacity_w = Some(AutosizeOrNumber::Value(1000.0));
    let zone_state = IdealLoadsZoneState {
        air_temperature_c: 25.0,
        air_humidity_ratio: 0.008,
    };

    let result = calc_no_oa_sensible_with_limits_compat(
        &system,
        zone_state,
        ZoneSysEnergyDemand::sensible_only(ZoneId(0), 0.0, -2400.0),
        true,
        IdealLoadsSensibleLimitContext::default(),
    );

    let cp = energyplus_moist_air_specific_heat_j_per_kg_k(0.008);
    let unlimited_mass_flow_rate_kg_per_s = 2400.0 / (cp * 12.0);
    let expected_supply_temperature_c = 25.0 - 1000.0 / (cp * unlimited_mass_flow_rate_kg_per_s);
    assert_eq!(result.mode, IdealLoadsSensibleMode::Cooling);
    assert_close(result.zone_sensible_cooling_rate_w, 1000.0, 1.0e-12);
    assert_close(
        result.supply_temperature_c,
        expected_supply_temperature_c,
        1.0e-12,
    );
}

#[test]
fn zero_capacity_limit_disables_sensible_branch_flow() {
    let mut system = test_system();
    system.heating_limit = IdealLoadsLimit::LimitCapacity;
    system.maximum_sensible_heating_capacity_w = Some(AutosizeOrNumber::Value(0.0));
    let zone_state = IdealLoadsZoneState {
        air_temperature_c: 20.0,
        air_humidity_ratio: 0.008,
    };

    let result = calc_no_oa_sensible_with_limits_compat(
        &system,
        zone_state,
        ZoneSysEnergyDemand::sensible_only(ZoneId(0), 3000.0, 0.0),
        true,
        IdealLoadsSensibleLimitContext::default(),
    );

    assert_eq!(result.mode, IdealLoadsSensibleMode::Deadband);
    assert_eq!(result.supply_mass_flow_rate_kg_per_s, 0.0);
    assert_eq!(result.zone_total_heating_rate_w, 0.0);
}

fn assert_close(actual: f64, expected: f64, tolerance: f64) {
    assert!(
        (actual - expected).abs() <= tolerance,
        "{actual} was not within {tolerance} of {expected}"
    );
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
