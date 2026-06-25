use crate::{AutosizeOrNumber, IdealLoadsAirSystemId, NormalizedName, ScheduleId};

/// `ZoneHVAC:IdealLoadsAirSystem` limit mode.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IdealLoadsLimit {
    /// No flow or capacity limit.
    NoLimit,
    /// Limit flow rate only.
    LimitFlowRate,
    /// Limit capacity only.
    LimitCapacity,
    /// Limit both flow rate and capacity.
    LimitFlowRateAndCapacity,
}

/// Ideal loads dehumidification control mode.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DehumidificationControlType {
    /// No dehumidification.
    None,
    /// Constant sensible heat ratio.
    ConstantSensibleHeatRatio,
    /// Constant supply humidity ratio.
    ConstantSupplyHumidityRatio,
    /// Humidistat-controlled.
    Humidistat,
}

/// Ideal loads humidification control mode.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HumidificationControlType {
    /// No humidification.
    None,
    /// Constant supply humidity ratio.
    ConstantSupplyHumidityRatio,
    /// Humidistat-controlled.
    Humidistat,
}

/// Demand-controlled ventilation mode.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DemandControlledVentilationType {
    /// No DCV.
    None,
    /// Occupancy schedule DCV.
    OccupancySchedule,
    /// CO2 setpoint DCV.
    Co2Setpoint,
}

/// Outdoor-air economizer mode.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OutdoorAirEconomizerType {
    /// No economizer.
    NoEconomizer,
    /// Differential dry-bulb economizer.
    DifferentialDryBulb,
    /// Differential enthalpy economizer.
    DifferentialEnthalpy,
}

/// Heat recovery mode.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HeatRecoveryType {
    /// No heat recovery.
    None,
    /// Sensible heat recovery.
    Sensible,
    /// Enthalpy heat recovery.
    Enthalpy,
}

/// `DesignSpecification:OutdoorAir` method.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DesignSpecificationOutdoorAirMethod {
    /// Outdoor air flow per person.
    FlowPerPerson,
    /// Outdoor air flow per zone floor area.
    FlowPerArea,
    /// Outdoor air flow per zone.
    FlowPerZone,
    /// Outdoor air changes per hour.
    AirChangesPerHour,
    /// Sum applicable outdoor air terms.
    Sum,
    /// Maximum applicable outdoor air term.
    Maximum,
    /// Indoor air quality procedure.
    IndoorAirQualityProcedure,
    /// Proportional control based on design occupancy.
    ProportionalControlBasedOnDesignOccupancy,
    /// Proportional control based on occupancy schedule.
    ProportionalControlBasedOnOccupancySchedule,
}

/// Typed `DesignSpecification:OutdoorAir` inputs.
#[derive(Clone, Debug, PartialEq)]
pub struct DesignSpecificationOutdoorAir {
    /// Typed ID.
    pub id: crate::DesignSpecificationOutdoorAirId,
    /// Object name.
    pub name: NormalizedName,
    /// Outdoor air method.
    pub method: DesignSpecificationOutdoorAirMethod,
    /// Outdoor air flow per person in m3/s-person.
    pub outdoor_air_flow_per_person_m3_per_s_person: f64,
    /// Outdoor air flow per zone floor area in m3/s-m2.
    pub outdoor_air_flow_per_zone_floor_area_m3_per_s_m2: f64,
    /// Outdoor air flow per zone in m3/s.
    pub outdoor_air_flow_per_zone_m3_per_s: f64,
    /// Outdoor air changes per hour.
    pub outdoor_air_flow_air_changes_per_hour: f64,
    /// Optional outdoor air schedule.
    pub outdoor_air_schedule: Option<ScheduleId>,
    /// Optional proportional control minimum outdoor air flow rate schedule.
    pub proportional_control_minimum_outdoor_air_flow_rate_schedule: Option<ScheduleId>,
}

/// Ideal loads purchased energy fuel type.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IdealLoadsFuelType {
    /// Coal.
    Coal,
    /// Diesel.
    Diesel,
    /// District cooling.
    DistrictCooling,
    /// District heating steam.
    DistrictHeatingSteam,
    /// District heating water.
    DistrictHeatingWater,
    /// Electricity.
    Electricity,
    /// Fuel oil no. 1.
    FuelOilNo1,
    /// Fuel oil no. 2.
    FuelOilNo2,
    /// Gasoline.
    Gasoline,
    /// Natural gas.
    NaturalGas,
    /// Other fuel 1.
    OtherFuel1,
    /// Other fuel 2.
    OtherFuel2,
    /// Propane.
    Propane,
}

/// Typed IdealLoads air system inputs needed before load-solver parity.
#[derive(Clone, Debug, PartialEq)]
pub struct IdealLoadsAirSystem {
    /// Typed ID.
    pub id: IdealLoadsAirSystemId,
    /// Object name.
    pub name: NormalizedName,
    /// Overall availability schedule.
    pub availability_schedule: Option<ScheduleId>,
    /// Zone supply air node or node list name.
    pub zone_supply_air_node_name: NormalizedName,
    /// Optional zone exhaust air node name.
    pub zone_exhaust_air_node_name: Option<NormalizedName>,
    /// Optional system inlet air node name.
    pub system_inlet_air_node_name: Option<NormalizedName>,
    /// Maximum heating supply air temperature in C.
    pub maximum_heating_supply_air_temperature_c: f64,
    /// Minimum cooling supply air temperature in C.
    pub minimum_cooling_supply_air_temperature_c: f64,
    /// Maximum heating supply humidity ratio.
    pub maximum_heating_supply_air_humidity_ratio: f64,
    /// Minimum cooling supply humidity ratio.
    pub minimum_cooling_supply_air_humidity_ratio: f64,
    /// Heating limit mode.
    pub heating_limit: IdealLoadsLimit,
    /// Maximum heating air flow rate.
    pub maximum_heating_air_flow_rate_m3_per_s: Option<AutosizeOrNumber>,
    /// Maximum sensible heating capacity.
    pub maximum_sensible_heating_capacity_w: Option<AutosizeOrNumber>,
    /// Cooling limit mode.
    pub cooling_limit: IdealLoadsLimit,
    /// Maximum cooling air flow rate.
    pub maximum_cooling_air_flow_rate_m3_per_s: Option<AutosizeOrNumber>,
    /// Maximum total cooling capacity.
    pub maximum_total_cooling_capacity_w: Option<AutosizeOrNumber>,
    /// Heating availability schedule.
    pub heating_availability_schedule: Option<ScheduleId>,
    /// Cooling availability schedule.
    pub cooling_availability_schedule: Option<ScheduleId>,
    /// Dehumidification control type.
    pub dehumidification_control_type: DehumidificationControlType,
    /// Cooling sensible heat ratio.
    pub cooling_sensible_heat_ratio: f64,
    /// Humidification control type.
    pub humidification_control_type: HumidificationControlType,
    /// Optional design specification outdoor air object name.
    pub design_specification_outdoor_air_object_name: Option<NormalizedName>,
    /// Optional outdoor air inlet node name.
    pub outdoor_air_inlet_node_name: Option<NormalizedName>,
    /// Demand-controlled ventilation type.
    pub demand_controlled_ventilation_type: DemandControlledVentilationType,
    /// Outdoor air economizer type.
    pub outdoor_air_economizer_type: OutdoorAirEconomizerType,
    /// Heat recovery type.
    pub heat_recovery_type: HeatRecoveryType,
    /// Sensible heat recovery effectiveness.
    pub sensible_heat_recovery_effectiveness: f64,
    /// Latent heat recovery effectiveness.
    pub latent_heat_recovery_effectiveness: f64,
    /// Optional zone HVAC sizing object name.
    pub design_specification_zonehvac_sizing_object_name: Option<NormalizedName>,
    /// Optional heating fuel efficiency schedule.
    pub heating_fuel_efficiency_schedule: Option<ScheduleId>,
    /// Heating fuel type.
    pub heating_fuel_type: IdealLoadsFuelType,
    /// Optional cooling fuel efficiency schedule.
    pub cooling_fuel_efficiency_schedule: Option<ScheduleId>,
    /// Cooling fuel type.
    pub cooling_fuel_type: IdealLoadsFuelType,
}
