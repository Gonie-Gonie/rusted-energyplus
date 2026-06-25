use crate::{
    NormalizedName, ScheduleId, ThermostatSetpointId, ZoneHumidistatId, ZoneId, ZoneThermostatId,
};

/// Supported thermostat control object type for the first HVAC subset.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ThermostatControlObjectType {
    /// `ThermostatSetpoint:DualSetpoint`.
    DualSetpoint,
}

/// Heating and cooling setpoint schedules.
#[derive(Clone, Debug, PartialEq)]
pub struct ThermostatDualSetpoint {
    /// Typed ID.
    pub id: ThermostatSetpointId,
    /// Object name.
    pub name: NormalizedName,
    /// Heating setpoint schedule.
    pub heating_setpoint_schedule: ScheduleId,
    /// Cooling setpoint schedule.
    pub cooling_setpoint_schedule: ScheduleId,
}

/// One control entry inside `ZoneControl:Thermostat`.
#[derive(Clone, Debug, PartialEq)]
pub struct ZoneThermostatControl {
    /// Control object type.
    pub object_type: ThermostatControlObjectType,
    /// Referenced dual setpoint object.
    pub dual_setpoint: ThermostatSetpointId,
}

/// Zone thermostat assignment.
#[derive(Clone, Debug, PartialEq)]
pub struct ZoneThermostat {
    /// Typed ID.
    pub id: ZoneThermostatId,
    /// Object name.
    pub name: NormalizedName,
    /// Controlled zone.
    pub zone: ZoneId,
    /// Schedule containing thermostat control type integers.
    pub control_type_schedule: ScheduleId,
    /// Thermostat control entries in EnergyPlus order.
    pub controls: Vec<ZoneThermostatControl>,
    /// Temperature difference between cutout and setpoint in delta C.
    pub temperature_difference_between_cutout_and_setpoint_delta_c: f64,
}

/// Zone humidistat assignment.
#[derive(Clone, Debug, PartialEq)]
pub struct ZoneHumidistat {
    /// Typed ID.
    pub id: ZoneHumidistatId,
    /// Object name.
    pub name: NormalizedName,
    /// Controlled zone.
    pub zone: ZoneId,
    /// Humidifying relative humidity setpoint schedule.
    pub humidifying_relative_humidity_setpoint_schedule: ScheduleId,
    /// Dehumidifying relative humidity setpoint schedule.
    pub dehumidifying_relative_humidity_setpoint_schedule: ScheduleId,
}
