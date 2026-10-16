//! Reached genuine EarthTube/CoolTower/ThermalChimney module and report constructors.
use std::convert::Infallible;
#[derive(Clone, Debug, Default, PartialEq)]
pub(super) struct SourceEarthTubeZoneReport {
    pub(super) heat_loss: f64,
    pub(super) heat_loss_rate: f64,
    pub(super) heat_gain: f64,
    pub(super) heat_gain_rate: f64,
    pub(super) outdoor_air_treatment_power: f64,
    pub(super) volume: f64,
    pub(super) volume_flow_rate: f64,
    pub(super) volume_flow_rate_standard: f64,
    pub(super) mass: f64,
    pub(super) mass_flow_rate: f64,
    pub(super) water_mass_flow_rate: f64,
    pub(super) fan_electricity: f64,
    pub(super) fan_electric_power: f64,
    pub(super) air_temperature: f64,
    pub(super) wet_bulb_temperature: f64,
    pub(super) humidity_ratio: f64,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub(super) struct SourceThermalChimneyZoneReport {
    pub(super) heat_loss: f64,
    pub(super) heat_gain: f64,
    pub(super) volume: f64,
    pub(super) mass: f64,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub(super) struct SourceThermalChimneyReport {
    pub(super) overall_volume_flow: f64,
    pub(super) overall_volume_flow_standard: f64,
    pub(super) overall_mass_flow: f64,
    pub(super) outlet_air_temperature: f64,
}
#[derive(Clone, Debug, PartialEq)]
pub(super) struct SourceEarthTube {
    pub(super) get_input_flag: bool,
    pub(super) initialize_first_time: bool,
    pub(super) time_elapsed: f64,
    /// Actual EarthTube namespace-scope totEarthTube constructor0.
    pub(super) total_earth_tubes: usize,
    pub(super) systems: Option<Vec<Infallible>>,
    pub(super) zone_reports: Option<Vec<SourceEarthTubeZoneReport>>,
    pub(super) parameters: Option<Vec<Infallible>>,
}
impl Default for SourceEarthTube {
    fn default() -> Self {
        Self {
            get_input_flag: true,
            initialize_first_time: true,
            time_elapsed: 0.0,
            total_earth_tubes: 0,
            systems: None,
            zone_reports: None,
            parameters: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub(super) struct SourceCoolTower {
    pub(super) get_input_flag: bool,
    pub(super) systems: Option<Vec<Infallible>>,
}
impl Default for SourceCoolTower {
    fn default() -> Self {
        Self {
            get_input_flag: true,
            systems: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub(super) struct SourceThermalChimney {
    pub(super) get_input_flag: bool,
    pub(super) total_chimneys: usize,
    pub(super) systems: Option<Vec<Infallible>>,
    pub(super) zone_reports: Option<Vec<SourceThermalChimneyZoneReport>>,
    pub(super) reports: Option<Vec<SourceThermalChimneyReport>>,
}
impl Default for SourceThermalChimney {
    fn default() -> Self {
        Self {
            get_input_flag: true,
            total_chimneys: 0,
            systems: None,
            zone_reports: None,
            reports: None,
        }
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceSimpleAirflowSystems {
    pub(super) earth_tube: SourceEarthTube,
    pub(super) cool_tower: SourceCoolTower,
    pub(super) thermal_chimney: SourceThermalChimney,
}
