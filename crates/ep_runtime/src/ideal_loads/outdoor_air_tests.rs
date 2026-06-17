use super::*;
use ep_model::{
    DehumidificationControlType, DemandControlledVentilationType, DesignSpecificationOutdoorAir,
    DesignSpecificationOutdoorAirId, DesignSpecificationOutdoorAirMethod, HeatRecoveryType,
    HumidificationControlType, IdealLoadsAirSystem, IdealLoadsAirSystemId, IdealLoadsFuelType,
    IdealLoadsLimit, NormalizedName, OutdoorAirEconomizerType,
};

#[test]
fn flow_zone_uses_declared_zone_volume_flow() {
    let mut specification = test_specification();
    specification.method = DesignSpecificationOutdoorAirMethod::FlowPerZone;
    specification.outdoor_air_flow_per_zone_m3_per_s = 0.05;

    let result = calc_design_outdoor_air_volume_flow_m3_per_s(
        &specification,
        IdealLoadsOutdoorAirContext {
            design_people_count: 3.0,
            zone_floor_area_m2: 20.0,
            zone_volume_m3: 60.0,
        },
    );

    assert_eq!(result, Some(0.05));
}

#[test]
fn sum_combines_supported_terms() {
    let mut specification = test_specification();
    specification.method = DesignSpecificationOutdoorAirMethod::Sum;
    specification.outdoor_air_flow_per_person_m3_per_s_person = 0.004;
    specification.outdoor_air_flow_per_zone_floor_area_m3_per_s_m2 = 0.0003;
    specification.outdoor_air_flow_per_zone_m3_per_s = 0.02;
    specification.outdoor_air_flow_air_changes_per_hour = 0.6;

    let result = calc_design_outdoor_air_volume_flow_m3_per_s(
        &specification,
        IdealLoadsOutdoorAirContext {
            design_people_count: 5.0,
            zone_floor_area_m2: 40.0,
            zone_volume_m3: 90.0,
        },
    )
    .expect("sum method is supported");

    let expected = 0.004 * 5.0 + 0.0003 * 40.0 + 0.02 + 0.6 * 90.0 / 3600.0;
    assert_close(result, expected, 1.0e-12);
}

#[test]
fn maximum_selects_largest_supported_term() {
    let mut specification = test_specification();
    specification.method = DesignSpecificationOutdoorAirMethod::Maximum;
    specification.outdoor_air_flow_per_person_m3_per_s_person = 0.004;
    specification.outdoor_air_flow_per_zone_floor_area_m3_per_s_m2 = 0.002;
    specification.outdoor_air_flow_per_zone_m3_per_s = 0.03;
    specification.outdoor_air_flow_air_changes_per_hour = 1.0;

    let result = calc_design_outdoor_air_volume_flow_m3_per_s(
        &specification,
        IdealLoadsOutdoorAirContext {
            design_people_count: 4.0,
            zone_floor_area_m2: 40.0,
            zone_volume_m3: 120.0,
        },
    )
    .expect("maximum method is supported");

    assert_close(result, 0.002 * 40.0, 1.0e-12);
}

#[test]
fn mass_flow_applies_clamped_schedule_and_standard_density() {
    let mut specification = test_specification();
    specification.method = DesignSpecificationOutdoorAirMethod::FlowPerZone;
    specification.outdoor_air_flow_per_zone_m3_per_s = 0.1;

    let result = calc_scheduled_outdoor_air_mass_flow_rate_kg_per_s(
        &specification,
        IdealLoadsOutdoorAirContext {
            design_people_count: 0.0,
            zone_floor_area_m2: 0.0,
            zone_volume_m3: 0.0,
        },
        Some(2.0),
        1.2,
    );

    assert_eq!(result, Some(0.12));
}

#[test]
fn sensible_report_sorts_cold_oa_as_heating_load_when_heat_mode_is_active() {
    let result = calc_outdoor_air_sensible_report_rates_compat(
        &test_system(),
        IdealLoadsOutdoorAirNodeState {
            air_temperature_c: 21.0,
            air_humidity_ratio: 0.006,
        },
        IdealLoadsOutdoorAirNodeState {
            air_temperature_c: 21.0,
            air_humidity_ratio: 0.006,
        },
        IdealLoadsOutdoorAirNodeState {
            air_temperature_c: 5.0,
            air_humidity_ratio: 0.004,
        },
        ZoneSysEnergyDemand::sensible_only(ep_model::ZoneId(0), 100.0, 0.0),
        0.05,
        0.25,
        101_325.0,
        true,
    );

    assert_eq!(result.mode, IdealLoadsSensibleMode::Heating);
    assert!(result.supply_mass_flow_rate_kg_per_s >= 0.05);
    assert!(result.mixed_air_temperature_c < 21.0);
    assert!(result.supply_air_temperature_c >= result.mixed_air_temperature_c);
    assert_eq!(
        result.supply_air_humidity_ratio,
        result.mixed_air_humidity_ratio
    );
    assert!(result.outdoor_air_sensible_output_w < 0.0);
    assert!(result.outdoor_air_latent_output_w.is_finite());
    assert_eq!(
        result.outdoor_air_total_heating_rate_w,
        result.outdoor_air_sensible_heating_rate_w
    );
    assert_eq!(result.outdoor_air_latent_heating_rate_w, 0.0);
    assert_close(
        result.outdoor_air_sensible_heating_rate_w,
        -result.outdoor_air_sensible_output_w,
        1.0e-12,
    );
    assert_eq!(result.outdoor_air_sensible_cooling_rate_w, 0.0);
}

#[test]
fn sensible_report_sorts_warm_oa_as_cooling_load_when_cool_mode_is_active() {
    let result = calc_outdoor_air_sensible_report_rates_compat(
        &test_system(),
        IdealLoadsOutdoorAirNodeState {
            air_temperature_c: 24.0,
            air_humidity_ratio: 0.006,
        },
        IdealLoadsOutdoorAirNodeState {
            air_temperature_c: 24.0,
            air_humidity_ratio: 0.006,
        },
        IdealLoadsOutdoorAirNodeState {
            air_temperature_c: 32.0,
            air_humidity_ratio: 0.008,
        },
        ZoneSysEnergyDemand::sensible_only(ep_model::ZoneId(0), 0.0, -50.0),
        0.05,
        0.25,
        101_325.0,
        true,
    );

    assert_eq!(result.mode, IdealLoadsSensibleMode::Cooling);
    assert!(result.supply_mass_flow_rate_kg_per_s >= 0.05);
    assert!(result.mixed_air_temperature_c > 24.0);
    assert!(result.supply_air_temperature_c <= result.mixed_air_temperature_c);
    assert_eq!(
        result.supply_air_humidity_ratio,
        result.mixed_air_humidity_ratio
    );
    assert!(result.outdoor_air_sensible_output_w > 0.0);
    assert!(result.outdoor_air_latent_output_w.is_finite());
    assert_eq!(
        result.outdoor_air_total_cooling_rate_w,
        result.outdoor_air_sensible_cooling_rate_w
    );
    assert_eq!(result.outdoor_air_latent_cooling_rate_w, 0.0);
    assert_close(
        result.outdoor_air_sensible_cooling_rate_w,
        result.outdoor_air_sensible_output_w,
        1.0e-12,
    );
    assert_eq!(result.outdoor_air_sensible_heating_rate_w, 0.0);
}

#[test]
fn differential_dry_bulb_economizer_raises_outdoor_air_flow_when_cooling() {
    let mut system = test_system();
    system.outdoor_air_economizer_type = OutdoorAirEconomizerType::DifferentialDryBulb;
    let result = calc_outdoor_air_sensible_report_rates_compat(
        &system,
        IdealLoadsOutdoorAirNodeState {
            air_temperature_c: 24.0,
            air_humidity_ratio: 0.006,
        },
        IdealLoadsOutdoorAirNodeState {
            air_temperature_c: 24.0,
            air_humidity_ratio: 0.006,
        },
        IdealLoadsOutdoorAirNodeState {
            air_temperature_c: 10.0,
            air_humidity_ratio: 0.004,
        },
        ZoneSysEnergyDemand::sensible_only(ep_model::ZoneId(0), 0.0, -500.0),
        0.01,
        0.25,
        101_325.0,
        true,
    );

    assert_eq!(result.mode, IdealLoadsSensibleMode::Cooling);
    assert!(result.outdoor_air_mass_flow_rate_kg_per_s > 0.01);
    assert_eq!(result.economizer_active_time_hr, 0.25);
    assert!(result.supply_mass_flow_rate_kg_per_s >= result.outdoor_air_mass_flow_rate_kg_per_s);
}

#[test]
fn differential_enthalpy_economizer_raises_outdoor_air_flow_when_cooling() {
    let mut system = test_system();
    system.outdoor_air_economizer_type = OutdoorAirEconomizerType::DifferentialEnthalpy;
    let result = calc_outdoor_air_sensible_report_rates_compat(
        &system,
        IdealLoadsOutdoorAirNodeState {
            air_temperature_c: 24.0,
            air_humidity_ratio: 0.010,
        },
        IdealLoadsOutdoorAirNodeState {
            air_temperature_c: 24.0,
            air_humidity_ratio: 0.010,
        },
        IdealLoadsOutdoorAirNodeState {
            air_temperature_c: 12.0,
            air_humidity_ratio: 0.002,
        },
        ZoneSysEnergyDemand::sensible_only(ep_model::ZoneId(0), 0.0, -500.0),
        0.01,
        0.25,
        101_325.0,
        true,
    );

    assert_eq!(result.mode, IdealLoadsSensibleMode::Cooling);
    assert!(result.outdoor_air_mass_flow_rate_kg_per_s > 0.01);
    assert_eq!(result.economizer_active_time_hr, 0.25);
    assert!(result.supply_mass_flow_rate_kg_per_s >= result.outdoor_air_mass_flow_rate_kg_per_s);
}

#[test]
fn sensible_heat_recovery_reports_active_heating_output() {
    let mut system = test_system();
    system.heat_recovery_type = HeatRecoveryType::Sensible;
    let result = calc_outdoor_air_sensible_report_rates_compat(
        &system,
        IdealLoadsOutdoorAirNodeState {
            air_temperature_c: 21.0,
            air_humidity_ratio: 0.006,
        },
        IdealLoadsOutdoorAirNodeState {
            air_temperature_c: 21.0,
            air_humidity_ratio: 0.006,
        },
        IdealLoadsOutdoorAirNodeState {
            air_temperature_c: 0.0,
            air_humidity_ratio: 0.004,
        },
        ZoneSysEnergyDemand::sensible_only(ep_model::ZoneId(0), 500.0, 0.0),
        0.01,
        0.25,
        101_325.0,
        true,
    );

    assert_eq!(result.mode, IdealLoadsSensibleMode::Heating);
    assert_eq!(result.heat_recovery_active_time_hr, 0.25);
    assert!(result.heat_recovery_sensible_heating_rate_w > 0.0);
    assert_eq!(result.heat_recovery_sensible_cooling_rate_w, 0.0);
    assert_close(result.heat_recovery_latent_output_w, 0.0, 1.0e-9);
    assert_eq!(
        result.heat_recovery_total_heating_rate_w,
        result.heat_recovery_sensible_heating_rate_w
    );
}

#[test]
fn enthalpy_heat_recovery_reports_active_latent_heating_output() {
    let mut system = test_system();
    system.heat_recovery_type = HeatRecoveryType::Enthalpy;
    let result = calc_outdoor_air_sensible_report_rates_compat(
        &system,
        IdealLoadsOutdoorAirNodeState {
            air_temperature_c: 21.0,
            air_humidity_ratio: 0.010,
        },
        IdealLoadsOutdoorAirNodeState {
            air_temperature_c: 21.0,
            air_humidity_ratio: 0.010,
        },
        IdealLoadsOutdoorAirNodeState {
            air_temperature_c: 0.0,
            air_humidity_ratio: 0.002,
        },
        ZoneSysEnergyDemand::sensible_only(ep_model::ZoneId(0), 500.0, 0.0),
        0.01,
        0.25,
        101_325.0,
        true,
    );

    assert_eq!(result.mode, IdealLoadsSensibleMode::Heating);
    assert_eq!(result.heat_recovery_active_time_hr, 0.25);
    assert!(result.heat_recovery_sensible_heating_rate_w > 0.0);
    assert!(result.heat_recovery_latent_heating_rate_w > 0.0);
    assert_eq!(result.heat_recovery_latent_cooling_rate_w, 0.0);
    assert!(
        result.heat_recovery_total_heating_rate_w > result.heat_recovery_sensible_heating_rate_w
    );
    assert!(result.mixed_air_humidity_ratio > 0.002);
}

#[test]
fn unsupported_methods_remain_unresolved() {
    let mut specification = test_specification();
    specification.method = DesignSpecificationOutdoorAirMethod::IndoorAirQualityProcedure;

    assert_eq!(
        calc_design_outdoor_air_volume_flow_m3_per_s(
            &specification,
            IdealLoadsOutdoorAirContext {
                design_people_count: 1.0,
                zone_floor_area_m2: 1.0,
                zone_volume_m3: 1.0,
            },
        ),
        None
    );
}

fn test_specification() -> DesignSpecificationOutdoorAir {
    DesignSpecificationOutdoorAir {
        id: DesignSpecificationOutdoorAirId(0),
        name: NormalizedName::new("OUTDOOR AIR SPEC"),
        method: DesignSpecificationOutdoorAirMethod::FlowPerPerson,
        outdoor_air_flow_per_person_m3_per_s_person: 0.00944,
        outdoor_air_flow_per_zone_floor_area_m3_per_s_m2: 0.0,
        outdoor_air_flow_per_zone_m3_per_s: 0.0,
        outdoor_air_flow_air_changes_per_hour: 0.0,
        outdoor_air_schedule: None,
        proportional_control_minimum_outdoor_air_flow_rate_schedule: None,
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
        dehumidification_control_type: DehumidificationControlType::ConstantSensibleHeatRatio,
        cooling_sensible_heat_ratio: 0.7,
        humidification_control_type: HumidificationControlType::None,
        design_specification_outdoor_air_object_name: Some(NormalizedName::new("OUTDOOR AIR SPEC")),
        outdoor_air_inlet_node_name: Some(NormalizedName::new("OUTDOOR AIR NODE")),
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

fn assert_close(actual: f64, expected: f64, tolerance: f64) {
    assert!(
        (actual - expected).abs() <= tolerance,
        "{actual} was not within {tolerance} of {expected}"
    );
}
