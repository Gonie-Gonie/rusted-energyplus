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
/// IdealLoads outdoor-air mass flow rate output variable.
pub const ZONE_IDEAL_LOADS_OUTDOOR_AIR_MASS_FLOW_RATE: &str =
    "Zone Ideal Loads Outdoor Air Mass Flow Rate";
/// IdealLoads outdoor-air standard-density volume flow output variable.
pub const ZONE_IDEAL_LOADS_OUTDOOR_AIR_STANDARD_DENSITY_VOLUME_FLOW_RATE: &str =
    "Zone Ideal Loads Outdoor Air Standard Density Volume Flow Rate";
/// IdealLoads outdoor-air sensible heating rate output variable.
pub const ZONE_IDEAL_LOADS_OUTDOOR_AIR_SENSIBLE_HEATING_RATE: &str =
    "Zone Ideal Loads Outdoor Air Sensible Heating Rate";
/// IdealLoads outdoor-air sensible cooling rate output variable.
pub const ZONE_IDEAL_LOADS_OUTDOOR_AIR_SENSIBLE_COOLING_RATE: &str =
    "Zone Ideal Loads Outdoor Air Sensible Cooling Rate";
/// IdealLoads outdoor-air latent heating rate output variable.
pub const ZONE_IDEAL_LOADS_OUTDOOR_AIR_LATENT_HEATING_RATE: &str =
    "Zone Ideal Loads Outdoor Air Latent Heating Rate";
/// IdealLoads outdoor-air latent cooling rate output variable.
pub const ZONE_IDEAL_LOADS_OUTDOOR_AIR_LATENT_COOLING_RATE: &str =
    "Zone Ideal Loads Outdoor Air Latent Cooling Rate";
/// IdealLoads outdoor-air total heating rate output variable.
pub const ZONE_IDEAL_LOADS_OUTDOOR_AIR_TOTAL_HEATING_RATE: &str =
    "Zone Ideal Loads Outdoor Air Total Heating Rate";
/// IdealLoads outdoor-air total cooling rate output variable.
pub const ZONE_IDEAL_LOADS_OUTDOOR_AIR_TOTAL_COOLING_RATE: &str =
    "Zone Ideal Loads Outdoor Air Total Cooling Rate";
/// IdealLoads mixed-air temperature output variable.
pub const ZONE_IDEAL_LOADS_MIXED_AIR_TEMPERATURE: &str = "Zone Ideal Loads Mixed Air Temperature";
/// IdealLoads mixed-air humidity ratio output variable.
pub const ZONE_IDEAL_LOADS_MIXED_AIR_HUMIDITY_RATIO: &str =
    "Zone Ideal Loads Mixed Air Humidity Ratio";

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
