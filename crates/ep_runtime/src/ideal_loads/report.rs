//! IdealLoads output variable names and report snapshots.

use crate::ideal_loads::{IdealLoadsSensibleMode, IdealLoadsSensibleResult};

/// Zone thermostat heating setpoint output variable.
pub const ZONE_THERMOSTAT_HEATING_SETPOINT_TEMPERATURE: &str =
    "Zone Thermostat Heating Setpoint Temperature";
/// Zone thermostat cooling setpoint output variable.
pub const ZONE_THERMOSTAT_COOLING_SETPOINT_TEMPERATURE: &str =
    "Zone Thermostat Cooling Setpoint Temperature";
/// IdealLoads zone total heating rate output variable.
pub const ZONE_IDEAL_LOADS_ZONE_TOTAL_HEATING_RATE: &str =
    "Zone Ideal Loads Zone Total Heating Rate";
/// IdealLoads zone total cooling rate output variable.
pub const ZONE_IDEAL_LOADS_ZONE_TOTAL_COOLING_RATE: &str =
    "Zone Ideal Loads Zone Total Cooling Rate";
/// IdealLoads zone sensible heating rate output variable.
pub const ZONE_IDEAL_LOADS_ZONE_SENSIBLE_HEATING_RATE: &str =
    "Zone Ideal Loads Zone Sensible Heating Rate";
/// IdealLoads zone sensible cooling rate output variable.
pub const ZONE_IDEAL_LOADS_ZONE_SENSIBLE_COOLING_RATE: &str =
    "Zone Ideal Loads Zone Sensible Cooling Rate";
/// IdealLoads supply-air total heating rate output variable.
pub const ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_HEATING_RATE: &str =
    "Zone Ideal Loads Supply Air Total Heating Rate";
/// IdealLoads supply-air total cooling rate output variable.
pub const ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_COOLING_RATE: &str =
    "Zone Ideal Loads Supply Air Total Cooling Rate";

/// Report payload produced by the no-OA/no-limit sensible calculation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IdealLoadsReportSnapshot {
    /// Operating mode.
    pub mode: IdealLoadsSensibleMode,
    /// Zone total heating rate in W.
    pub zone_total_heating_rate_w: f64,
    /// Zone total cooling rate in W.
    pub zone_total_cooling_rate_w: f64,
    /// Zone sensible heating rate in W.
    pub zone_sensible_heating_rate_w: f64,
    /// Zone sensible cooling rate in W.
    pub zone_sensible_cooling_rate_w: f64,
    /// Supply air total heating rate in W.
    pub supply_air_total_heating_rate_w: f64,
    /// Supply air total cooling rate in W.
    pub supply_air_total_cooling_rate_w: f64,
}

impl From<IdealLoadsSensibleResult> for IdealLoadsReportSnapshot {
    fn from(result: IdealLoadsSensibleResult) -> Self {
        Self {
            mode: result.mode,
            zone_total_heating_rate_w: result.zone_total_heating_rate_w,
            zone_total_cooling_rate_w: result.zone_total_cooling_rate_w,
            zone_sensible_heating_rate_w: result.zone_sensible_heating_rate_w,
            zone_sensible_cooling_rate_w: result.zone_sensible_cooling_rate_w,
            supply_air_total_heating_rate_w: result.supply_air_total_heating_rate_w,
            supply_air_total_cooling_rate_w: result.supply_air_total_cooling_rate_w,
        }
    }
}
