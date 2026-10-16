//! DataZoneControls and DataHeatBalFanSys genuine retained constructor state.
use super::*;
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceTStatObject {
    pub(crate) name: String,
    pub(crate) zone_or_list_pointer: i32,
    pub(crate) number_zones: i32,
    pub(crate) temperature_zone_start: i32,
    pub(crate) comfort_zone_start: i32,
    pub(crate) stage_zone_start: i32,
    pub(crate) zone_list_active: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceZoneAirControls {
    pub(crate) number_temperature_zones: i32,
    pub(crate) number_humidity_zones: i32,
    pub(crate) number_comfort_zones: i32,
    pub(crate) number_thermostat_statements: usize,
    pub(crate) number_comfort_statements: usize,
    pub(crate) number_operative_zones: usize,
    pub(crate) number_temperature_humidity_zones: usize,
    pub(crate) any_operative_control: bool,
    pub(crate) any_temperature_humidity_control: bool,
    pub(crate) stage_zone_logic: Option<Vec<bool>>,
    pub(crate) occupied_heating_setpoints: Option<Vec<Option<f64>>>,
    pub(crate) occupied_cooling_setpoints: Option<Vec<Option<f64>>>,
    pub(crate) get_zone_air_stats_input: bool,
    pub(crate) thermostat_objects: Option<Vec<SourceTStatObject>>,
    pub(crate) comfort_thermostat_objects: Option<Vec<SourceTStatObject>>,
}
impl Default for SourceZoneAirControls {
    fn default() -> Self {
        Self {
            number_temperature_zones: 0,
            number_humidity_zones: 0,
            number_comfort_zones: 0,
            number_thermostat_statements: 0,
            number_comfort_statements: 0,
            number_operative_zones: 0,
            number_temperature_humidity_zones: 0,
            any_operative_control: false,
            any_temperature_humidity_control: false,
            stage_zone_logic: None,
            occupied_heating_setpoints: None,
            occupied_cooling_setpoints: None,
            get_zone_air_stats_input: true,
            thermostat_objects: None,
            comfort_thermostat_objects: None,
        }
    }
}
/// Complete actual ZoneTempPredictorCorrector.hh74-79 row constructor.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceZoneSetpointSchedule {
    pub(crate) name: String,
    pub(crate) heat_schedule: Option<crate::heat_balance::selected_schedule_manager::SourceScheduleIndex>,
    pub(crate) cool_schedule: Option<crate::heat_balance::selected_schedule_manager::SourceScheduleIndex>,
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceZoneAirFlags {
    pub(crate) number_temperature_controls: [usize; 5],
    /// Same native five Array1D owners default-unallocated; allocate only actual positive counts.
    pub(crate) temperature_setpoint_schedules: [Option<Vec<SourceZoneSetpointSchedule>>; 5],
    pub(crate) number_comfort_controls: [usize; 5],
    pub(crate) number_stage_zones: i32,
    pub(crate) number_on_off_zones: i32,
    pub(crate) initialize_setpoints_once: bool,
    pub(crate) my_environment: bool,
    pub(crate) my_day: bool,
    pub(crate) errors_found: bool,
    pub(crate) controlled_zones_checked: bool,
}
impl Default for SourceZoneAirFlags {
    fn default() -> Self {
        Self {
            number_temperature_controls: [0; 5],
            temperature_setpoint_schedules: std::array::from_fn(|_| None),
            number_comfort_controls: [0; 5],
            number_stage_zones: 0,
            number_on_off_zones: 0,
            initialize_setpoints_once: true,
            my_environment: true,
            my_day: true,
            errors_found: false,
            controlled_zones_checked: false,
        }
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceZoneTstatSetpoint {
    pub(crate) setpt: f64,
    pub(crate) setpt_adap_comfort_cool: f64,
    pub(crate) setpt_hi: f64,
    pub(crate) setpt_lo: f64,
    pub(crate) setpt_hi_average: f64,
    pub(crate) setpt_lo_average: f64,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceAirThermostatArrays {
    pub(crate) zone_tstat_setpoints: Option<Vec<SourceZoneTstatSetpoint>>,
    pub(crate) load_correction_factor: Option<Vec<f64>>,
    pub(crate) temp_control_type: Option<Vec<i32>>,
    pub(crate) temp_control_type_report: Option<Vec<i32>>,
    pub(crate) previous_measured_temperature_1: Option<Vec<f64>>,
    pub(crate) previous_measured_temperature_2: Option<Vec<f64>>,
    pub(crate) previous_measured_temperature_3: Option<Vec<f64>>,
    pub(crate) previous_measured_humidity_1: Option<Vec<f64>>,
    pub(crate) previous_measured_humidity_2: Option<Vec<f64>>,
    pub(crate) previous_measured_humidity_3: Option<Vec<f64>>,
}

/// Sole DataHeatBalance ZoneList/ZoneGroup sensible report arrays, ctor-unallocated.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceZoneListGroupLoadReports {
    pub(crate) list_heat_energy: Option<Vec<f64>>,
    pub(crate) list_cool_energy: Option<Vec<f64>>,
    pub(crate) list_heat_rate: Option<Vec<f64>>,
    pub(crate) list_cool_rate: Option<Vec<f64>>,
    pub(crate) group_heat_energy: Option<Vec<f64>>,
    pub(crate) group_cool_energy: Option<Vec<f64>>,
    pub(crate) group_heat_rate: Option<Vec<f64>>,
    pub(crate) group_cool_rate: Option<Vec<f64>>,
}
