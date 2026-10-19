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
    pub(crate) temperature_controlled_zones: Option<Vec<SourceTemperatureControlledZone>>,
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
            temperature_controlled_zones: None,
            comfort_thermostat_objects: None,
        }
    }
}

/// Exact DataZoneControls.hh79-86 enum; both control modes construct Invalid.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(i32)]
pub(crate) enum SourceTemperatureControlMode {
    #[default]
    Invalid = -1,
    None = 0,
    Constant = 1,
    Scheduled = 2,
    Num = 3,
}
/// Complete DataZoneControls.hh91-97 constructor, including null pointers.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceTemperatureSetpointType {
    pub(crate) name: String,
    pub(crate) is_used: bool,
    pub(crate) heat_schedule: Option<crate::heat_balance::selected_schedule_manager::SourceScheduleIndex>,
    pub(crate) cool_schedule: Option<crate::heat_balance::selected_schedule_manager::SourceScheduleIndex>,
}
/// Sole actual TempControlledZone row; complete DataZoneControls.hh99-154.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceTemperatureControlledZone {
    pub(crate) name: String,
    pub(crate) zone_name: String,
    pub(crate) actual_zone_number: i32,
    pub(crate) control_type_schedule: Option<crate::heat_balance::selected_schedule_manager::SourceScheduleIndex>,
    pub(crate) setpoints: [SourceTemperatureSetpointType; 5],
    pub(crate) manage_demand: bool,
    pub(crate) heating_reset_limit: f64,
    pub(crate) cooling_reset_limit: f64,
    pub(crate) ems_override_heating_on: bool,
    pub(crate) ems_override_heating_value: f64,
    pub(crate) ems_override_cooling_on: bool,
    pub(crate) ems_override_cooling_value: f64,
    pub(crate) operative_control: SourceTemperatureControlMode,
    pub(crate) fixed_radiative_fraction: f64,
    pub(crate) operative_radiative_fraction_schedule: Option<crate::heat_balance::selected_schedule_manager::SourceScheduleIndex>,
    pub(crate) adaptive_comfort_temperature_control: bool,
    pub(crate) adaptive_comfort_model_type_index: i32,
    pub(crate) zone_overcool_range: f64,
    pub(crate) overcool_control: SourceTemperatureControlMode,
    pub(crate) zone_overcool_constant_range: f64,
    pub(crate) overcool_range_schedule: Option<crate::heat_balance::selected_schedule_manager::SourceScheduleIndex>,
    pub(crate) zone_overcool_control_ratio: f64,
    pub(crate) dehumidifying_schedule: Option<crate::heat_balance::selected_schedule_manager::SourceScheduleIndex>,
    pub(crate) delta_temperature_cutout_setpoint: f64,
    pub(crate) thermostat_setpoint_hi: f64,
    pub(crate) thermostat_setpoint_lo: f64,
    pub(crate) cool_mode_last: bool,
    pub(crate) heat_mode_last: bool,
    pub(crate) cool_mode_last_save: bool,
    pub(crate) heat_mode_last_save: bool,
    pub(crate) cool_off: bool,
    pub(crate) heat_off: bool,
}
impl Default for SourceTemperatureControlledZone {
    fn default() -> Self {
        Self {
            name: String::new(), zone_name: String::new(), actual_zone_number: 0,
            control_type_schedule: None,
            setpoints: std::array::from_fn(|_| SourceTemperatureSetpointType::default()),
            manage_demand: false, heating_reset_limit: 0.0, cooling_reset_limit: 0.0,
            ems_override_heating_on: false, ems_override_heating_value: 0.0,
            ems_override_cooling_on: false, ems_override_cooling_value: 0.0,
            operative_control: SourceTemperatureControlMode::Invalid,
            fixed_radiative_fraction: 0.0, operative_radiative_fraction_schedule: None,
            adaptive_comfort_temperature_control: false, adaptive_comfort_model_type_index: 0,
            zone_overcool_range: 0.0, overcool_control: SourceTemperatureControlMode::Invalid,
            zone_overcool_constant_range: 0.0, overcool_range_schedule: None,
            zone_overcool_control_ratio: 0.0, dehumidifying_schedule: None,
            delta_temperature_cutout_setpoint: 0.0,
            thermostat_setpoint_hi: 0.0, thermostat_setpoint_lo: 0.0,
            cool_mode_last: false, heat_mode_last: false,
            cool_mode_last_save: false, heat_mode_last_save: false,
            cool_off: false, heat_off: false,
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
