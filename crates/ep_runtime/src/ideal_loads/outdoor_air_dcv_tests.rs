use super::*;
use ep_model::{
    DehumidificationControlType, DemandControlledVentilationType, DesignSpecificationOutdoorAir,
    DesignSpecificationOutdoorAirId, DesignSpecificationOutdoorAirMethod, HeatRecoveryType,
    HumidificationControlType, IdealLoadsAirSystem, IdealLoadsAirSystemId, IdealLoadsFuelType,
    IdealLoadsLimit, NormalizedName, OutdoorAirEconomizerType,
};

#[test]
fn occupancy_schedule_dcv_uses_current_people_for_flow_person() {
    let mut specification = test_specification();
    specification.method = DesignSpecificationOutdoorAirMethod::FlowPerPerson;
    specification.outdoor_air_flow_per_person_m3_per_s_person = 0.01;

    let context = IdealLoadsOutdoorAirContext {
        design_people_count: 5.0,
        zone_floor_area_m2: 20.0,
        zone_volume_m3: 60.0,
    };
    let design_result =
        calc_scheduled_outdoor_air_mass_flow_rate_kg_per_s(&specification, context, None, 1.2)
            .expect("design Flow/Person is supported");
    let dcv_result = calc_occupancy_schedule_dcv_outdoor_air_mass_flow_rate_kg_per_s(
        &specification,
        context,
        2.5,
        None,
        1.2,
    )
    .expect("OccupancySchedule DCV Flow/Person is supported");

    assert_close(design_result, 0.06, 1.0e-12);
    assert_close(dcv_result, 0.03, 1.0e-12);
}

#[test]
fn zero_minimum_oa_still_conditions_recirculation_when_unit_available() {
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
        ZoneSysEnergyDemand::sensible_only(ep_model::ZoneId(0), 100.0, 50.0),
        0.0,
        0.25,
        101_325.0,
        true,
    );

    assert_eq!(result.mode, IdealLoadsSensibleMode::Heating);
    assert_eq!(result.outdoor_air_mass_flow_rate_kg_per_s, 0.0);
    assert!(result.supply_mass_flow_rate_kg_per_s > 0.0);
    assert_eq!(result.mixed_air_temperature_c, 21.0);
    assert_eq!(result.supply_air_temperature_c, 50.0);
    assert_eq!(result.outdoor_air_sensible_heating_rate_w, 0.0);
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
