//! Literal pinned Native IdealLoads schema grammar; no runtime schema engine.
#[derive(Clone, Copy)]
pub(super) enum DefaultValue { Alpha(&'static str), Number(f64) }
pub(super) struct Field { pub(super) json: &'static str, pub(super) legacy: &'static str, pub(super) default: DefaultValue }
pub(super) const MIN_FIELDS: usize = 27;
pub(super) const FIELDS: [Field; 32] = [
    Field { json: "name", legacy: "Name", default: DefaultValue::Alpha("") },
    Field { json: "availability_schedule_name", legacy: "Availability Schedule Name", default: DefaultValue::Alpha("") },
    Field { json: "zone_supply_air_node_name", legacy: "Zone Supply Air Node Name", default: DefaultValue::Alpha("") },
    Field { json: "zone_exhaust_air_node_name", legacy: "Zone Exhaust Air Node Name", default: DefaultValue::Alpha("") },
    Field { json: "system_inlet_air_node_name", legacy: "System Inlet Air Node Name", default: DefaultValue::Alpha("") },
    Field { json: "maximum_heating_supply_air_temperature", legacy: "Maximum Heating Supply Air Temperature", default: DefaultValue::Number(50.0) },
    Field { json: "minimum_cooling_supply_air_temperature", legacy: "Minimum Cooling Supply Air Temperature", default: DefaultValue::Number(13.0) },
    Field { json: "maximum_heating_supply_air_humidity_ratio", legacy: "Maximum Heating Supply Air Humidity Ratio", default: DefaultValue::Number(0.0156) },
    Field { json: "minimum_cooling_supply_air_humidity_ratio", legacy: "Minimum Cooling Supply Air Humidity Ratio", default: DefaultValue::Number(0.0077) },
    Field { json: "heating_limit", legacy: "Heating Limit", default: DefaultValue::Alpha("NoLimit") },
    Field { json: "maximum_heating_air_flow_rate", legacy: "Maximum Heating Air Flow Rate", default: DefaultValue::Number(0.0) },
    Field { json: "maximum_sensible_heating_capacity", legacy: "Maximum Sensible Heating Capacity", default: DefaultValue::Number(0.0) },
    Field { json: "cooling_limit", legacy: "Cooling Limit", default: DefaultValue::Alpha("NoLimit") },
    Field { json: "maximum_cooling_air_flow_rate", legacy: "Maximum Cooling Air Flow Rate", default: DefaultValue::Number(0.0) },
    Field { json: "maximum_total_cooling_capacity", legacy: "Maximum Total Cooling Capacity", default: DefaultValue::Number(0.0) },
    Field { json: "heating_availability_schedule_name", legacy: "Heating Availability Schedule Name", default: DefaultValue::Alpha("") },
    Field { json: "cooling_availability_schedule_name", legacy: "Cooling Availability Schedule Name", default: DefaultValue::Alpha("") },
    Field { json: "dehumidification_control_type", legacy: "Dehumidification Control Type", default: DefaultValue::Alpha("ConstantSensibleHeatRatio") },
    Field { json: "cooling_sensible_heat_ratio", legacy: "Cooling Sensible Heat Ratio", default: DefaultValue::Number(0.7) },
    Field { json: "humidification_control_type", legacy: "Humidification Control Type", default: DefaultValue::Alpha("None") },
    Field { json: "design_specification_outdoor_air_object_name", legacy: "Design Specification Outdoor Air Object Name", default: DefaultValue::Alpha("") },
    Field { json: "outdoor_air_inlet_node_name", legacy: "Outdoor Air Inlet Node Name", default: DefaultValue::Alpha("") },
    Field { json: "demand_controlled_ventilation_type", legacy: "Demand Controlled Ventilation Type", default: DefaultValue::Alpha("None") },
    Field { json: "outdoor_air_economizer_type", legacy: "Outdoor Air Economizer Type", default: DefaultValue::Alpha("NoEconomizer") },
    Field { json: "heat_recovery_type", legacy: "Heat Recovery Type", default: DefaultValue::Alpha("None") },
    Field { json: "sensible_heat_recovery_effectiveness", legacy: "Sensible Heat Recovery Effectiveness", default: DefaultValue::Number(0.7) },
    Field { json: "latent_heat_recovery_effectiveness", legacy: "Latent Heat Recovery Effectiveness", default: DefaultValue::Number(0.65) },
    Field { json: "design_specification_zonehvac_sizing_object_name", legacy: "Design Specification ZoneHVAC Sizing Object Name", default: DefaultValue::Alpha("") },
    Field { json: "heating_fuel_efficiency_schedule_name", legacy: "Heating Fuel Efficiency Schedule Name", default: DefaultValue::Alpha("") },
    Field { json: "heating_fuel_type", legacy: "Heating Fuel Type", default: DefaultValue::Alpha("DistrictHeatingWater") },
    Field { json: "cooling_fuel_efficiency_schedule_name", legacy: "Cooling Fuel Efficiency Schedule Name", default: DefaultValue::Alpha("") },
    Field { json: "cooling_fuel_type", legacy: "Cooling Fuel Type", default: DefaultValue::Alpha("DistrictCooling") },
];
