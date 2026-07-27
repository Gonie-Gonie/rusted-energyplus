use super::*;
use ep_model::{
    DehumidificationControlType, DemandControlledVentilationType, DesignSpecificationOutdoorAir,
    DesignSpecificationOutdoorAirId, DesignSpecificationOutdoorAirMethod, HeatRecoveryType,
    HumidificationControlType, IdealLoadsAirSystem, IdealLoadsAirSystemId, IdealLoadsFuelType,
    IdealLoadsLimit, NodeId, NormalizedName, OutdoorAirEconomizerType, ScheduleId,
};

#[test]
fn wrapper_resolves_design_flow_before_calc_update_and_trace() {
    let system = test_system(DemandControlledVentilationType::None);
    let specification = flow_person_specification();
    let context = test_context();
    let output = run_wrapper(
        &system,
        &specification,
        context,
        Some(0.0),
        Some(0.0),
        None,
        IdealLoadsSensibleLimitContext {
            standard_air_density_kg_per_m3: 1.2,
            barometric_pressure_pa: 101_325.0,
            ..IdealLoadsSensibleLimitContext::default()
        },
        true,
    )
    .expect("base Flow/Person minimum is supported");
    let minimum = output
        .minimum_outdoor_air
        .expect("available unit resolves minimum outdoor air");
    let expected_calculation = calc_outdoor_air_sensible_report_rates_compat(
        &system,
        zone_state(),
        zone_state(),
        outdoor_air_state(),
        demand(),
        0.06,
        0.25,
        101_325.0,
        true,
    );

    assert_eq!(output.system_id, system.id);
    assert_eq!(output.selected_branch, "outdoor_air");
    assert_eq!(
        output.init_flags,
        IdealLoadsInitFlags::diagnostic_adapter_assumed_ready()
    );
    assert_eq!(output.calculation, expected_calculation);
    assert_eq!(output.supply_node_update.node, NodeId(9));
    assert_close(
        minimum
            .design_flow_components
            .final_design_volume_flow_rate_m3_per_s,
        0.05,
    );
    assert_close(minimum.scheduled_design_mass_flow_rate_kg_per_s, 0.06);
    assert_close(minimum.final_minimum_mass_flow_rate_kg_per_s, 0.06);
    assert_eq!(output.trace.current_people_count, Some(0.0));
    assert_eq!(output.trace.outdoor_air_schedule_value, Some(0.0));
    assert_close(
        output.trace.minimum_outdoor_air_mass_flow_rate_kg_per_s,
        0.06,
    );
}

#[test]
fn wrapper_occupancy_dcv_recomputes_sum_before_schedule_and_density() {
    let system = test_system(DemandControlledVentilationType::OccupancySchedule);
    let mut specification = flow_person_specification();
    specification.method = DesignSpecificationOutdoorAirMethod::Sum;
    specification.outdoor_air_flow_per_zone_floor_area_m3_per_s_m2 = 0.001;
    specification.outdoor_air_flow_per_zone_m3_per_s = 0.03;
    specification.outdoor_air_flow_air_changes_per_hour = 0.6;
    specification.outdoor_air_schedule = Some(ScheduleId(3));
    let output = run_wrapper(
        &system,
        &specification,
        test_context(),
        Some(0.5),
        Some(2.0),
        None,
        IdealLoadsSensibleLimitContext {
            standard_air_density_kg_per_m3: 1.2,
            barometric_pressure_pa: 101_325.0,
            ..IdealLoadsSensibleLimitContext::default()
        },
        true,
    )
    .expect("OccupancySchedule Sum minimum is supported");
    let minimum = output
        .minimum_outdoor_air
        .expect("available unit resolves minimum outdoor air");

    assert_close(
        minimum.design_flow_components.flow_per_person_m3_per_s,
        0.05,
    );
    assert_close(
        minimum.selected_flow_components.flow_per_person_m3_per_s,
        0.02,
    );
    assert_close(minimum.applied_schedule_multiplier, 0.5);
    assert_close(minimum.scheduled_design_mass_flow_rate_kg_per_s, 0.066);
    assert_close(minimum.final_minimum_mass_flow_rate_kg_per_s, 0.048);
}

#[test]
fn wrapper_co2_dcv_applies_max_then_explicit_nonfinite_guard() {
    let mut system = test_system(DemandControlledVentilationType::Co2Setpoint);
    let specification = flow_person_specification();
    for (required, expected_adjusted, expected_final) in [
        (0.03, 0.06, 0.06),
        (0.08, 0.08, 0.08),
        (f64::NAN, 0.06, 0.06),
        (f64::INFINITY, f64::INFINITY, 0.0),
    ] {
        system.id = IdealLoadsAirSystemId(system.id.0 + 1);
        let output = run_wrapper(
            &system,
            &specification,
            test_context(),
            None,
            None,
            Some(required),
            IdealLoadsSensibleLimitContext {
                standard_air_density_kg_per_m3: 1.2,
                barometric_pressure_pa: 101_325.0,
                ..IdealLoadsSensibleLimitContext::default()
            },
            true,
        )
        .expect("CO2Setpoint minimum is supported");
        let minimum = output
            .minimum_outdoor_air
            .expect("available unit resolves minimum outdoor air");
        assert_eq!(
            minimum.dcv_adjusted_mass_flow_rate_kg_per_s,
            expected_adjusted
        );
        assert_close(
            minimum.final_minimum_mass_flow_rate_kg_per_s,
            expected_final,
        );
    }
}

#[test]
fn wrapper_normalizes_schedule_at_the_runtime_boundary() {
    let system = test_system(DemandControlledVentilationType::None);
    let mut specification = flow_person_specification();
    specification.method = DesignSpecificationOutdoorAirMethod::FlowPerZone;
    specification.outdoor_air_flow_per_zone_m3_per_s = 0.1;
    specification.outdoor_air_schedule = Some(ScheduleId(4));
    for (schedule, expected_multiplier, expected_mass) in [
        (-1.0, 0.0, 0.0),
        (0.0, 0.0, 0.0),
        (0.25, 0.25, 0.03),
        (1.0, 1.0, 0.12),
        (2.0, 1.0, 0.12),
        (f64::NAN, 0.0, 0.0),
    ] {
        let output = run_wrapper(
            &system,
            &specification,
            test_context(),
            Some(schedule),
            None,
            None,
            IdealLoadsSensibleLimitContext {
                standard_air_density_kg_per_m3: 1.2,
                barometric_pressure_pa: 101_325.0,
                ..IdealLoadsSensibleLimitContext::default()
            },
            true,
        )
        .expect("scheduled Flow/Zone minimum is supported");
        let minimum = output
            .minimum_outdoor_air
            .expect("available unit resolves minimum outdoor air");
        assert_close(minimum.applied_schedule_multiplier, expected_multiplier);
        assert_close(minimum.final_minimum_mass_flow_rate_kg_per_s, expected_mass);
    }
}

#[test]
fn wrapper_applies_energyplus_very_small_mass_flow_cutoff() {
    let system = test_system(DemandControlledVentilationType::None);
    let mut specification = flow_person_specification();
    specification.method = DesignSpecificationOutdoorAirMethod::FlowPerZone;
    for (design_flow, expected_mass_flow) in [(1.0e-30, 0.0), (1.01e-30, 1.01e-30)] {
        specification.outdoor_air_flow_per_zone_m3_per_s = design_flow;
        let output = run_wrapper(
            &system,
            &specification,
            test_context(),
            None,
            None,
            None,
            IdealLoadsSensibleLimitContext {
                standard_air_density_kg_per_m3: 1.0,
                barometric_pressure_pa: 101_325.0,
                ..IdealLoadsSensibleLimitContext::default()
            },
            true,
        )
        .expect("tiny Flow/Zone minimum is supported");
        let minimum = output
            .minimum_outdoor_air
            .expect("available unit resolves minimum outdoor air");
        assert_eq!(
            minimum.final_minimum_mass_flow_rate_kg_per_s,
            expected_mass_flow
        );
    }
}

#[test]
fn wrapper_reports_missing_or_unsupported_minimum_flow_inputs() {
    let system = test_system(DemandControlledVentilationType::None);
    let mut specification = flow_person_specification();
    specification.outdoor_air_schedule = Some(ScheduleId(7));
    assert!(matches!(
        run_wrapper(
            &system,
            &specification,
            test_context(),
            None,
            None,
            None,
            IdealLoadsSensibleLimitContext::default(),
            true,
        ),
        Err(SimPurchasedAirOutdoorAirCompatError::MissingOutdoorAirScheduleValue { .. })
    ));

    specification.outdoor_air_schedule = None;
    specification.method = DesignSpecificationOutdoorAirMethod::IndoorAirQualityProcedure;
    assert!(matches!(
        run_wrapper(
            &system,
            &specification,
            test_context(),
            None,
            None,
            None,
            IdealLoadsSensibleLimitContext::default(),
            true,
        ),
        Err(SimPurchasedAirOutdoorAirCompatError::UnsupportedDesignFlowMethod { .. })
    ));

    specification.method = DesignSpecificationOutdoorAirMethod::FlowPerPerson;
    assert!(matches!(
        run_wrapper(
            &system,
            &specification,
            test_context(),
            None,
            None,
            None,
            IdealLoadsSensibleLimitContext {
                standard_air_density_kg_per_m3: f64::NAN,
                barometric_pressure_pa: 101_325.0,
                ..IdealLoadsSensibleLimitContext::default()
            },
            true,
        ),
        Err(SimPurchasedAirOutdoorAirCompatError::InvalidStandardAirDensity { .. })
    ));

    let occupancy_system = test_system(DemandControlledVentilationType::OccupancySchedule);
    assert!(matches!(
        run_wrapper(
            &occupancy_system,
            &specification,
            test_context(),
            None,
            None,
            None,
            IdealLoadsSensibleLimitContext::default(),
            true,
        ),
        Err(SimPurchasedAirOutdoorAirCompatError::MissingOccupancySchedulePeopleCount { .. })
    ));

    let co2_system = test_system(DemandControlledVentilationType::Co2Setpoint);
    assert!(matches!(
        run_wrapper(
            &co2_system,
            &specification,
            test_context(),
            None,
            None,
            None,
            IdealLoadsSensibleLimitContext::default(),
            true,
        ),
        Err(SimPurchasedAirOutdoorAirCompatError::MissingCo2SetpointDemand { .. })
    ));
}

#[test]
fn wrapper_distinguishes_zero_outdoor_air_from_unit_off() {
    let available_system = test_system(DemandControlledVentilationType::None);
    let mut available_specification = flow_person_specification();
    available_specification.outdoor_air_flow_per_person_m3_per_s_person = 0.0;
    let available = run_wrapper(
        &available_system,
        &available_specification,
        test_context(),
        None,
        None,
        None,
        IdealLoadsSensibleLimitContext::default(),
        true,
    )
    .expect("zero outdoor air is valid");

    let unavailable_system = test_system(DemandControlledVentilationType::OccupancySchedule);
    let mut unavailable_specification = flow_person_specification();
    unavailable_specification.outdoor_air_schedule = Some(ScheduleId(8));
    let unavailable = run_wrapper(
        &unavailable_system,
        &unavailable_specification,
        test_context(),
        None,
        None,
        None,
        IdealLoadsSensibleLimitContext::default(),
        false,
    )
    .expect("unit off skips missing schedule and occupancy inputs");

    assert!(available.minimum_outdoor_air.is_some());
    assert_eq!(available.calculation.mode, IdealLoadsSensibleMode::Heating);
    assert!(available.calculation.supply_mass_flow_rate_kg_per_s > 0.0);
    assert_eq!(unavailable.minimum_outdoor_air, None);
    assert_eq!(
        unavailable
            .trace
            .minimum_outdoor_air_mass_flow_rate_kg_per_s,
        0.0
    );
    assert_eq!(unavailable.calculation.mode, IdealLoadsSensibleMode::Off);
    assert_eq!(unavailable.calculation.supply_mass_flow_rate_kg_per_s, 0.0);
}

fn run_wrapper(
    system: &IdealLoadsAirSystem,
    specification: &DesignSpecificationOutdoorAir,
    context: IdealLoadsOutdoorAirContext,
    schedule: Option<f64>,
    current_people_count: Option<f64>,
    co2_required_mass_flow_rate_kg_per_s: Option<f64>,
    limit_context: IdealLoadsSensibleLimitContext,
    unit_available: bool,
) -> Result<SimPurchasedAirOutdoorAirCompatOutput, SimPurchasedAirOutdoorAirCompatError> {
    sim_purchased_air_outdoor_air_compat(SimPurchasedAirOutdoorAirCompatInput {
        system,
        supply_node: NodeId(9),
        zone_state: zone_state(),
        recirculation_state: zone_state(),
        outdoor_air_state: outdoor_air_state(),
        demand: demand(),
        minimum_outdoor_air: IdealLoadsMinimumOutdoorAirCompatInput {
            specification,
            context,
            outdoor_air_schedule_value: schedule,
            current_people_count,
            co2_setpoint_required_mass_flow_rate_kg_per_s: co2_required_mass_flow_rate_kg_per_s,
        },
        system_timestep_hours: 0.25,
        limit_context,
        unit_available,
    })
}

fn test_context() -> IdealLoadsOutdoorAirContext {
    IdealLoadsOutdoorAirContext {
        design_people_count: 5.0,
        zone_floor_area_m2: 20.0,
        zone_volume_m3: 60.0,
    }
}

fn zone_state() -> IdealLoadsOutdoorAirNodeState {
    IdealLoadsOutdoorAirNodeState {
        air_temperature_c: 21.0,
        air_humidity_ratio: 0.006,
    }
}

fn outdoor_air_state() -> IdealLoadsOutdoorAirNodeState {
    IdealLoadsOutdoorAirNodeState {
        air_temperature_c: 5.0,
        air_humidity_ratio: 0.004,
    }
}

fn demand() -> ZoneSysEnergyDemand {
    ZoneSysEnergyDemand::sensible_only(ep_model::ZoneId(0), 100.0, 50.0)
}

fn flow_person_specification() -> DesignSpecificationOutdoorAir {
    DesignSpecificationOutdoorAir {
        id: DesignSpecificationOutdoorAirId(0),
        name: NormalizedName::new("OUTDOOR AIR SPEC"),
        method: DesignSpecificationOutdoorAirMethod::FlowPerPerson,
        outdoor_air_flow_per_person_m3_per_s_person: 0.01,
        outdoor_air_flow_per_zone_floor_area_m3_per_s_m2: 0.0,
        outdoor_air_flow_per_zone_m3_per_s: 0.0,
        outdoor_air_flow_air_changes_per_hour: 0.0,
        outdoor_air_schedule: None,
        proportional_control_minimum_outdoor_air_flow_rate_schedule: None,
    }
}

fn test_system(dcv_type: DemandControlledVentilationType) -> IdealLoadsAirSystem {
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
        demand_controlled_ventilation_type: dcv_type,
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

fn assert_close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() <= 1.0e-12,
        "{actual} was not within 1e-12 of {expected}"
    );
}
