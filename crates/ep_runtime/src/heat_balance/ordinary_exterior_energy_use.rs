//! Genuine ExteriorEnergyUse module; same Source schedules, input, clock and meters.
use crate::heat_balance::ctf_reports::PredefinedStringTable;
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::predefined_tables::PredefinedRegistry;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::{
    HeatBalanceInputError as Error, SelectedHeatBalanceInput,
};
use crate::heat_balance::selected_input_processor::{
    SelectedInputProcessor, SourceInputProcessorShortCut,
};
use crate::heat_balance::selected_schedule_manager::{
    SelectedScheduleManager, SourceScheduleIndex,
};
use crate::heat_balance::source_output_prescan::SourceOutputPreScan;
use crate::heat_balance::source_output_registration::{
    SourceOutputDictionaryHost, SourceOutputFileHost, SourceOutputRegistration,
};
use crate::weather::day::{WeatherEnvironmentState, WeatherGlobalState};
use ep_model::ReportInputs;
use std::collections::HashMap;
mod getter;
mod output;
mod report;
mod state;
use state::{SourceExteriorEquipmentUsage, SourceExteriorLightUsage, SourceLightControlType};

/// All seven actual ExteriorEnergyUseData constructor members, retained once.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceExteriorEnergyUse {
    number_lights: i32,
    number_equipment: i32,
    lights: Option<Vec<SourceExteriorLightUsage>>,
    equipment: Option<Vec<SourceExteriorEquipmentUsage>>,
    unique_equipment_names: HashMap<String, String>,
    get_input: bool,
    sum_design_level: f64,
}
impl Default for SourceExteriorEnergyUse {
    fn default() -> Self {
        Self {
            number_lights: 0,
            number_equipment: 0,
            lights: None,
            equipment: None,
            unique_equipment_names: HashMap::new(),
            get_input: true,
            sum_design_level: 0.0,
        }
    }
}

/// Borrow the actual production input/schedules/weather/report owners per call.
pub(crate) struct SourceExteriorEnergyOwners<'a, 'backend, H, D> {
    pub(crate) raw: &'a SelectedHeatBalanceInput,
    pub(crate) input: &'a mut SelectedInputProcessor,
    pub(crate) shortcut: &'a mut SourceInputProcessorShortCut,
    pub(crate) schedules: &'a mut SelectedScheduleManager,
    pub(crate) globals: &'a SelectedHeatBalanceGlobals,
    pub(crate) clock: &'a WeatherGlobalState,
    pub(crate) environment: &'a WeatherEnvironmentState,
    pub(crate) registration: &'a mut SourceOutputRegistration,
    pub(crate) prescan: &'a SourceOutputPreScan,
    pub(crate) reports: &'a ReportInputs,
    pub(crate) minimum_report_frequency: &'a str,
    pub(crate) developer: &'a bool,
    pub(crate) host: &'a mut H,
    pub(crate) dictionary_host: &'a mut D,
    pub(crate) eio: &'a mut dyn std::io::Write,
    pub(crate) registry: &'a PredefinedRegistry,
    pub(crate) table: &'a mut PredefinedStringTable,
    pub(crate) backend: &'a mut SourceErrorBackend<'backend>,
}
fn index(number: i32) -> Result<usize, Error> {
    number
        .checked_sub(1)
        .and_then(|n| usize::try_from(n).ok())
        .ok_or(Error::Unavailable(
            "actual positive Exterior Source ordinal",
        ))
}
impl SourceExteriorEnergyUse {
    /// Native ManageExteriorEnergyUse80-99; input errors retain preceding writes.
    pub(crate) fn manage_source_exterior_energy_use<
        H: SourceOutputFileHost,
        D: SourceOutputDictionaryHost,
    >(
        &mut self,
        owners: &mut SourceExteriorEnergyOwners<'_, '_, H, D>,
    ) -> Result<(), Error> {
        if self.get_input {
            self.get_source_input(owners)?;
            self.get_input = false;
        }
        self.report_source_exterior_energy_use(owners.schedules, owners.clock, owners.environment)
    }
    fn light(&self, number: i32) -> Result<&SourceExteriorLightUsage, Error> {
        self.lights
            .as_ref()
            .and_then(|v| v.get(index(number).ok()?))
            .ok_or(Error::Unavailable("actual ExteriorLights allocated row"))
    }
    fn light_mut(&mut self, number: i32) -> Result<&mut SourceExteriorLightUsage, Error> {
        self.lights
            .as_mut()
            .and_then(|v| v.get_mut(index(number).ok()?))
            .ok_or(Error::Unavailable("actual ExteriorLights allocated row"))
    }
}

/// Per-call borrowed view of the same private ExteriorLights row.
pub(crate) struct SourceExteriorLightReportRef<'a> {
    row: &'a SourceExteriorLightUsage,
}
impl SourceExteriorLightReportRef<'_> {
    pub(crate) fn name(&self) -> &str {
        &self.row.name
    }
    pub(crate) fn is_schedule_only(&self) -> bool {
        self.row.control == SourceLightControlType::ScheduleOnly
    }
    pub(crate) fn schedule(&self) -> &Option<SourceScheduleIndex> {
        &self.row.schedule
    }
    pub(crate) fn design_level(&self) -> &f64 {
        &self.row.design_level
    }
    pub(crate) fn sum_consumption(&self) -> &f64 {
        &self.row.sum_consumption
    }
    pub(crate) fn sum_time_not_zero_consumption(&self) -> &f64 {
        &self.row.sum_time_not_zero_consumption
    }
}
impl SourceExteriorEnergyUse {
    pub(crate) fn source_number_lights(&self) -> &i32 {
        &self.number_lights
    }
    pub(crate) fn source_light_report(
        &self,
        number: i32,
    ) -> Result<SourceExteriorLightReportRef<'_>, Error> {
        Ok(SourceExteriorLightReportRef {
            row: self.light(number)?,
        })
    }
}
