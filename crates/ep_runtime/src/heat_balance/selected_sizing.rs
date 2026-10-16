//! Actual eight-getter ManageSizing path for the admitted empty-sizing domain.
//! Input DoSizing switches and calculation globals are separate borrowed owners.
use super::error_backend::{SourceErrorBackend, SourceErrorBackendFailure};
use super::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use super::selected_input_processor::{
    SelectedInputProcessor, SelectedInputUnavailable, SourceArgumentCounts,
    SourceInputProcessorShortCut,
};
use super::selected_schedule_manager::{
    SelectedScheduleError, SelectedScheduleManager, SourceScheduleIndex,
};
use super::selected_simulation_startup::SelectedHvacGlobals;
use crate::weather::day::SetupSimulationControls;
use crate::weather::day::{WeatherEnvironmentState, WeatherGlobalState};

/// Actual ordered Output:Table:SummaryReports strings, not a precomputed request flag.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SelectedSizingInput {
    /// One string vector per actual source object; normally zero or one after admission.
    pub summary_report_objects: Vec<SelectedSummaryReportInput>,
}

/// One actual SummaryReports object, in source getter order.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SelectedSummaryReportInput {
    /// Exact permanent InputProcessor object key.
    pub object_name: String,
    /// Actual report strings, including explicit blank fields.
    pub reports: Vec<String>,
    /// Actual staged Native IDF parser count, absent for direct epJSON.
    pub idf_max_fields: Option<usize>,
    /// Actual staged extensible report count; never inferred from Native answers.
    pub idf_max_extensible_fields: Option<usize>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SelectedOAFlowMethod {
    PerPerson,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SelectedOARequirement {
    pub(crate) name: String,
    pub(crate) number_of_dsoa: usize,
    pub(crate) dsoa_indices: Vec<usize>,
    pub(crate) dsoa_space_names: Vec<String>,
    pub(crate) dsoa_space_indices: Vec<usize>,
    pub(crate) flow_method: SelectedOAFlowMethod,
    pub(crate) flow_per_person: f64,
    pub(crate) flow_per_area: f64,
    pub(crate) flow_per_zone: f64,
    pub(crate) flow_air_changes_per_hour: f64,
    pub(crate) flow_fraction_schedule: Option<SourceScheduleIndex>,
    pub(crate) proportional_control_minimum_schedule: Option<SourceScheduleIndex>,
    pub(crate) co2_error_counters: [i32; 4],
    pub(crate) my_environment: bool,
}
impl Default for SelectedOARequirement {
    fn default() -> Self {
        Self {
            name: String::new(),
            number_of_dsoa: 0,
            dsoa_indices: Vec::new(),
            dsoa_space_names: Vec::new(),
            dsoa_space_indices: Vec::new(),
            flow_method: SelectedOAFlowMethod::PerPerson,
            flow_per_person: 0.0,
            flow_per_area: 0.0,
            flow_per_zone: 0.0,
            flow_air_changes_per_hour: 0.0,
            flow_fraction_schedule: None,
            proportional_control_minimum_schedule: None,
            co2_error_counters: [0; 4],
            my_environment: true,
        }
    }
}

/// Exact DataSizing.hh708–764 ZoneEqSizingData defaults; both array members stay unallocated.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceZoneEquipmentSizing {
    pub(crate) air__vol__flow: f64,                   // Native AirVolFlow
    pub(crate) max_hw_vol__flow: f64,                 // Native MaxHWVolFlow
    pub(crate) max_cw_vol__flow: f64,                 // Native MaxCWVolFlow
    pub(crate) oa_vol__flow: f64,                     // Native OAVolFlow
    pub(crate) at_mixer__vol__flow: f64,              // Native ATMixerVolFlow
    pub(crate) at_mixer__cool__pri__dry__bulb: f64,   // Native ATMixerCoolPriDryBulb
    pub(crate) at_mixer__cool__pri__hum__rat: f64,    // Native ATMixerCoolPriHumRat
    pub(crate) at_mixer__heat__pri__dry__bulb: f64,   // Native ATMixerHeatPriDryBulb
    pub(crate) at_mixer__heat__pri__hum__rat: f64,    // Native ATMixerHeatPriHumRat
    pub(crate) des__cooling__load: f64,               // Native DesCoolingLoad
    pub(crate) des__heating__load: f64,               // Native DesHeatingLoad
    pub(crate) cooling__air__vol__flow: f64,          // Native CoolingAirVolFlow
    pub(crate) heating__air__vol__flow: f64,          // Native HeatingAirVolFlow
    pub(crate) system__air__vol__flow: f64,           // Native SystemAirVolFlow
    pub(crate) air__flow: bool,                       // Native AirFlow
    pub(crate) cooling__air__flow: bool,              // Native CoolingAirFlow
    pub(crate) heating__air__flow: bool,              // Native HeatingAirFlow
    pub(crate) system__air__flow: bool,               // Native SystemAirFlow
    pub(crate) capacity: bool,                        // Native Capacity
    pub(crate) cooling__capacity: bool,               // Native CoolingCapacity
    pub(crate) heating__capacity: bool,               // Native HeatingCapacity
    pub(crate) system__capacity: bool,                // Native SystemCapacity
    pub(crate) design__size__from__parent: bool,      // Native DesignSizeFromParent
    pub(crate) hvac_sizing__index: i32,               // Native HVACSizingIndex
    pub(crate) sizing__method: Option<Vec<i32>>,      // Native SizingMethod
    pub(crate) cap__sizing__method: Option<Vec<i32>>, // Native CapSizingMethod
}

/// Selected actual DataSizing/SizingManager storage, retained before and after sizing.
/// Unsupported positive array element types are not represented by this empty path.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SelectedSizingManager {
    pub(crate) zone_equipment_sizing: Option<Vec<SourceZoneEquipmentSizing>>,
    /// Same DataSizing.hh1368 OASysEqSizing EPVector, constructor-unallocated.
    pub(crate) outside_air_system_equipment_sizing: Option<Vec<SourceZoneEquipmentSizing>>,
    pub(crate) unitary_system_equipment_sizing: Option<Vec<SourceZoneEquipmentSizing>>,
    pub(crate) current_system_number: i32,
    pub(crate) current_zone_equipment_number: i32,
    pub(crate) current_terminal_unit_sizing_number: i32,
    pub(crate) system_sizing_run_done: bool,
    pub(crate) zone_sizing_run_done: bool,
    pub(crate) number_of_oa_requirements: usize,
    pub(crate) default_oa_requirement: usize,
    pub(crate) oa_requirements: Option<Vec<SelectedOARequirement>>,
    pub(crate) number_of_zone_air_distributions: usize,
    pub(crate) number_of_zone_hvac_sizing: usize,
    pub(crate) number_of_air_terminal_sizing: usize,
    pub(crate) number_of_zone_sizing_input: usize,
    pub(crate) number_of_system_sizing_input: usize,
    pub(crate) number_of_plant_sizing_input: usize,
    pub(crate) number_of_air_loops: usize,
    pub(crate) global_heat_sizing_factor: f64,
    pub(crate) global_cool_sizing_factor: f64,
    pub(crate) number_of_timesteps_in_average: i32,
    pub(crate) sizing_file_column_separator: char,
    pub(crate) zone_sizing: Option<Vec<()>>,
    pub(crate) space_sizing: Option<Vec<()>>,
}
impl Default for SelectedSizingManager {
    fn default() -> Self {
        Self {
            zone_equipment_sizing: None,
            outside_air_system_equipment_sizing: None,
            unitary_system_equipment_sizing: None,
            current_system_number: 0,
            current_zone_equipment_number: 0,
            current_terminal_unit_sizing_number: 0,
            system_sizing_run_done: false,
            zone_sizing_run_done: false,
            number_of_oa_requirements: 0,
            default_oa_requirement: 0,
            oa_requirements: None,
            number_of_zone_air_distributions: 0,
            number_of_zone_hvac_sizing: 0,
            number_of_air_terminal_sizing: 0,
            number_of_zone_sizing_input: 0,
            number_of_system_sizing_input: 0,
            number_of_plant_sizing_input: 0,
            number_of_air_loops: 0,
            global_heat_sizing_factor: 0.0,
            global_cool_sizing_factor: 0.0,
            number_of_timesteps_in_average: 0,
            sizing_file_column_separator: ',',
            zone_sizing: None,
            space_sizing: None,
        }
    }
}

#[derive(Debug)]
pub(crate) enum SelectedSizingError {
    Input(SelectedInputUnavailable),
    Schedule(SelectedScheduleError),
    Backend(SourceErrorBackendFailure),
    Unavailable(&'static str),
}
impl From<SelectedInputUnavailable> for SelectedSizingError {
    fn from(value: SelectedInputUnavailable) -> Self {
        Self::Input(value)
    }
}
impl From<SelectedScheduleError> for SelectedSizingError {
    fn from(value: SelectedScheduleError) -> Self {
        Self::Schedule(value)
    }
}
impl From<SourceErrorBackendFailure> for SelectedSizingError {
    fn from(value: SourceErrorBackendFailure) -> Self {
        Self::Backend(value)
    }
}
impl std::fmt::Display for SelectedSizingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Input(e) => std::fmt::Display::fmt(e, f),
            Self::Schedule(e) => std::fmt::Display::fmt(e, f),
            Self::Backend(e) => std::fmt::Display::fmt(e, f),
            Self::Unavailable(e) => write!(f, "selected ManageSizing unavailable: {e}"),
        }
    }
}
impl std::error::Error for SelectedSizingError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Input(e) => Some(e),
            Self::Schedule(e) => Some(e),
            Self::Backend(e) => Some(e),
            Self::Unavailable(_) => None,
        }
    }
}

type LocalArrays = (
    Vec<String>,
    Vec<String>,
    Vec<String>,
    Vec<f64>,
    Vec<bool>,
    Vec<bool>,
);
fn local_arrays(args: SourceArgumentCounts) -> LocalArrays {
    (
        vec![String::new(); args.alphas],
        vec![String::new(); args.alphas],
        vec![String::new(); args.numerics],
        vec![0.0; args.numerics],
        vec![true; args.alphas],
        vec![true; args.numerics],
    )
}

impl SelectedSizingManager {
    fn get_oa_requirements(
        &mut self,
        input: &mut SelectedInputProcessor,
        schedules: &SelectedScheduleManager,
    ) -> Result<(), SelectedSizingError> {
        let count = input.get_num_objects_found("DesignSpecification:OutdoorAir");
        let args = input.get_object_def_max_args("DesignSpecification:OutdoorAir")?;
        let lists = input.get_num_objects_found("DesignSpecification:OutdoorAir:SpaceList");
        self.number_of_oa_requirements = count
            .checked_add(lists)
            .ok_or(SelectedSizingError::Unavailable("OA count size"))?;
        let locals = local_arrays(args);
        self.number_of_oa_requirements = self
            .number_of_oa_requirements
            .checked_add(1)
            .ok_or(SelectedSizingError::Unavailable("default OA count size"))?;
        self.oa_requirements = Some(vec![
            SelectedOARequirement::default();
            self.number_of_oa_requirements
        ]);
        self.default_oa_requirement = self.number_of_oa_requirements;
        let default = self
            .oa_requirements
            .as_mut()
            .and_then(|rows| rows.last_mut())
            .ok_or(SelectedSizingError::Unavailable("OA allocation"))?;
        default.name = "Default-0.00944[m3/s-person]".into();
        default.flow_method = SelectedOAFlowMethod::PerPerson;
        default.flow_per_person = 0.00944;
        default.flow_fraction_schedule = Some(schedules.always_on()?);
        if count > 0 || lists > 0 {
            return Err(SelectedSizingError::Unavailable("positive OA input"));
        }
        drop(locals);
        Ok(())
    }

    fn get_zone_air_distribution(
        &mut self,
        input: &mut SelectedInputProcessor,
    ) -> Result<(), SelectedSizingError> {
        self.number_of_zone_air_distributions =
            input.get_num_objects_found("DesignSpecification:ZoneAirDistribution");
        let args = input.get_object_def_max_args("DesignSpecification:ZoneAirDistribution")?;
        let locals = local_arrays(args);
        if self.number_of_zone_air_distributions > 0 {
            return Err(SelectedSizingError::Unavailable(
                "positive ZoneAirDistribution input",
            ));
        }
        drop(locals);
        Ok(())
    }

    fn get_zone_hvac_sizing(
        &mut self,
        input: &mut SelectedInputProcessor,
    ) -> Result<(), SelectedSizingError> {
        self.number_of_zone_hvac_sizing =
            input.get_num_objects_found("DesignSpecification:ZoneHVAC:Sizing");
        let args = input.get_object_def_max_args("DesignSpecification:ZoneHVAC:Sizing")?;
        let locals = local_arrays(args);
        if self.number_of_zone_hvac_sizing > 0 {
            return Err(SelectedSizingError::Unavailable(
                "positive ZoneHVAC sizing input",
            ));
        }
        drop(locals); // Original explicitly deallocates all six locals at this boundary.
        Ok(())
    }

    fn get_air_terminal_sizing(
        &mut self,
        input: &mut SelectedInputProcessor,
        shortcut: &mut SourceInputProcessorShortCut,
    ) -> Result<(), SelectedSizingError> {
        shortcut.current_module_object = "DesignSpecification:AirTerminal:Sizing".into();
        self.number_of_air_terminal_sizing =
            input.get_num_objects_found(&shortcut.current_module_object);
        input.get_object_def_max_args(&shortcut.current_module_object)?;
        if self.number_of_air_terminal_sizing > 0 {
            return Err(SelectedSizingError::Unavailable(
                "positive AirTerminal sizing input",
            ));
        }
        Ok(())
    }

    fn get_sizing_params(
        &mut self,
        input: &SelectedInputProcessor,
        shortcut: &mut SourceInputProcessorShortCut,
        time_steps_in_hour: i32,
        override_timestep: bool,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), SelectedSizingError> {
        shortcut.current_module_object = "Sizing:Parameters".into();
        if input.get_num_objects_found(&shortcut.current_module_object) > 0 {
            return Err(SelectedSizingError::Unavailable(
                "positive Sizing:Parameters input",
            ));
        }
        self.global_heat_sizing_factor = 1.0;
        self.global_cool_sizing_factor = 1.0;
        self.number_of_timesteps_in_average = time_steps_in_hour;
        if override_timestep {
            self.number_of_timesteps_in_average = time_steps_in_hour;
            backend.show_warning_error("Due to the use of the fast simulation mode, the time step for simulation and averaging window of sizing is overwritten to one hour. Original user inputs for averaging window and timestep are no longer used.")?;
        }
        shortcut.current_module_object = "OutputControl:Sizing:Style".into();
        if input.get_num_objects_found(&shortcut.current_module_object) > 0 {
            return Err(SelectedSizingError::Unavailable(
                "positive sizing output style",
            ));
        }
        *shortcut
            .alpha_args
            .first_mut()
            .ok_or(SelectedSizingError::Unavailable(
                "InputProcessor shared allocation prerequisite",
            ))? = "Comma".into();
        self.sizing_file_column_separator = ',';
        Ok(())
    }

    fn get_zone_sizing_input(
        &mut self,
        input: &SelectedInputProcessor,
        shortcut: &mut SourceInputProcessorShortCut,
    ) -> Result<(), SelectedSizingError> {
        shortcut.current_module_object = "Sizing:Zone".into();
        let statements = input.get_num_objects_found(&shortcut.current_module_object);
        let locals = vec![(); statements]; // Real local SizingZoneObjects allocation, including zero.
        if statements > 0 {
            return Err(SelectedSizingError::Unavailable(
                "positive Sizing:Zone input",
            ));
        }
        // The real GetZoneAndZoneListNames and DD queries are not called at zero.
        shortcut.current_module_object = "Sizing:Zone".into();
        self.number_of_zone_sizing_input = 0;
        drop(locals);
        Ok(())
    }

    fn get_system_sizing_input(
        &mut self,
        input: &SelectedInputProcessor,
        shortcut: &mut SourceInputProcessorShortCut,
    ) -> Result<(), SelectedSizingError> {
        self.number_of_air_loops = input.get_num_objects_found("AirLoopHVAC");
        shortcut.current_module_object = "Sizing:System".into();
        self.number_of_system_sizing_input =
            input.get_num_objects_found(&shortcut.current_module_object);
        if self.number_of_system_sizing_input > 0 {
            return Err(SelectedSizingError::Unavailable(
                "positive Sizing:System input",
            ));
        }
        Ok(())
    }

    fn get_plant_sizing_input(
        &mut self,
        input: &SelectedInputProcessor,
        shortcut: &mut SourceInputProcessorShortCut,
    ) -> Result<(), SelectedSizingError> {
        shortcut.current_module_object = "Sizing:Plant".into();
        self.number_of_plant_sizing_input =
            input.get_num_objects_found(&shortcut.current_module_object);
        if self.number_of_plant_sizing_input > 0 {
            return Err(SelectedSizingError::Unavailable(
                "positive Sizing:Plant input",
            ));
        }
        Ok(())
    }

    pub(crate) fn manage_sizing(
        &mut self,
        input: &mut SelectedInputProcessor,
        shortcut: &mut SourceInputProcessorShortCut,
        schedules: &SelectedScheduleManager,
        declarations: &SelectedSizingInput,
        controls: SetupSimulationControls,
        globals: &mut SelectedHeatBalanceGlobals,
        weather_global: &mut WeatherGlobalState,
        environment: &mut WeatherEnvironmentState,
        override_timestep: bool,
        do_space_heat_balance_sizing: bool,
        do_space_heat_balance_simulation: bool,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), SelectedSizingError> {
        // Domain admission precedes the real selected body. These input controls do
        // not initialize or overwrite any calculation global.
        if controls.zone_sizing()
            || controls.system_sizing()
            || controls.plant_sizing()
            || controls.hvac_sizing_simulation()
            || do_space_heat_balance_sizing
            || do_space_heat_balance_simulation
        {
            return Err(SelectedSizingError::Unavailable(
                "active sizing or SpaceHVAC calculation",
            ));
        }
        self.system_sizing_run_done = false;
        self.zone_sizing_run_done = false;
        self.get_oa_requirements(input, schedules)?;
        self.get_zone_air_distribution(input)?;
        self.get_zone_hvac_sizing(input)?;
        self.get_air_terminal_sizing(input, shortcut)?;
        self.get_sizing_params(
            input,
            shortcut,
            weather_global.time_steps_in_hour,
            override_timestep,
            backend,
        )?;
        self.get_zone_sizing_input(input, shortcut)?;
        self.get_system_sizing_input(input, shortcut)?;
        self.get_plant_sizing_input(input, shortcut)?;
        // OutputReportTabular::isCompLoadRepReq has local arrays/module name only.
        let reports = input.get_num_objects_found("Output:Table:SummaryReports");
        if declarations.summary_report_objects.len() != reports {
            return Err(SelectedSizingError::Unavailable(
                "summary report raw association",
            ));
        }
        let mut user_requested_component_load = false;
        if reports == 1 {
            let args = input.get_object_def_max_args("Output:Table:SummaryReports")?;
            let mut alpha = vec![String::new(); args.alphas];
            let numeric = vec![0.0; args.numerics];
            let actual = &declarations.summary_report_objects[0];
            if actual.reports.len() != alpha.len()
                || !input.has_object("Output:Table:SummaryReports", &actual.object_name)
            {
                return Err(SelectedSizingError::Unavailable(
                    "summary permanent/extensible association",
                ));
            }
            // Actual InputProcessor1026 erase, after local reset and before values.
            input.mark_object_as_used("Output:Table:SummaryReports", &actual.object_name);
            for (slot, value) in alpha.iter_mut().zip(&actual.reports) {
                *slot = value.clone();
            }
            for value in &alpha {
                if [
                    "ZoneComponentLoadSummary",
                    "AirLoopComponentLoadSummary",
                    "FacilityComponentLoadSummary",
                    "AllSummaryAndSizingPeriod",
                    "AllSummaryMonthlyAndSizingPeriod",
                ]
                .iter()
                .any(|key| value.eq_ignore_ascii_case(key))
                {
                    user_requested_component_load = true;
                }
            }
            drop((alpha, numeric));
        }
        // hasSizingPeriodsDays queries exactly these two families, even with sizing off.
        let design_days = input.get_num_objects_found("SizingPeriod:DesignDay");
        let weather_days = input.get_num_objects_found("SizingPeriod:WeatherFileDays");
        if user_requested_component_load {
            let message = if design_days > 0 || weather_days > 0 {
                "ManageSizing: The ZoneComponentLoadSummary report was requested but no sizing objects were found so that report cannot be generated."
            } else {
                "ManageSizing: The ZoneComponentLoadSummary report was requested but no SizingPeriod:DesignDay or SizingPeriod:WeatherFileDays objects were found so that report cannot be generated."
            };
            backend.show_warning_error(message)?;
        }
        // The selected false-sizing branches never assign CompLoadReportIsReq.
        globals.zone_sizing_calculation = false;
        weather_global.do_output_reporting = false;
        environment.month = 0; // actual local LastMonth constructor, not current weather.
        environment.day_of_month = 0; // actual local LastDayOfMonth constructor.
        globals.system_sizing_calculation = false;
        self.zone_sizing = None; // unconditional real ZoneSizing.deallocate613.
        // SpaceSizing is retained because its actual selected sizing switch is false.
        // DoingSizing233/240 belongs to the real caller, including failure retention.
        Ok(())
    }
}

impl SelectedSizingManager {
    /// Actual770-1207 has only a reference alias outside this literal predicate.
    /// The unentered positive body performs zone equipment and sizing/report work.
    pub(crate) fn manage_system_sizing_adjustments(
        &self,
        globals: &SelectedHeatBalanceGlobals,
    ) -> Result<(), SelectedSizingError> {
        if self.number_of_system_sizing_input > 0 && globals.do_system_sizing {
            return Err(SelectedSizingError::Unavailable(
                "positive ManageSystemSizingAdjustments body",
            ));
        }
        Ok(())
    }

    /// Actual1209-2023 has four reference aliases and two NumPrimaryAirSys loops.
    /// Count0 executes no numeric, equipment or predefined-table write at all.
    pub(crate) fn manage_system_ventilation_adjustments(
        &self,
        hvac: &SelectedHvacGlobals,
    ) -> Result<(), SelectedSizingError> {
        if hvac.number_of_primary_air_systems != 0 {
            return Err(SelectedSizingError::Unavailable(
                "nonzero ManageSystemVentilationAdjustments primary-air-system domain",
            ));
        }
        Ok(())
    }
}
