//! Genuine ManageSurfaceHeatBalance145-229 and retained distinct manager flags.
use crate::heat_balance::ordinary_tabular_reports::SourceTabularReports;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::{
    HeatBalanceInputError as Error, SelectedHeatBalanceInputState,
};
pub(crate) mod provider_host;
mod reports;
mod resilience;
pub(crate) use reports::SourceResilienceReportOwners;
pub(crate) use resilience::{SourceResilienceOutputOwners, SourceResilienceOwners};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceSurfaceManagement {
    pub(crate) manage_surface_heat_balance_first_time: bool,
    pub(crate) report_thermal_resilience_first_time: bool,
    pub(crate) report_var_heat_index: bool,
    pub(crate) report_var_humidex: bool,
    pub(crate) has_pierce_set: bool,
    pub(crate) report_co2_resilience_first_time: bool,
    pub(crate) report_visual_resilience_first_time: bool,
    pub(crate) low_set_longest_hours: Vec<f64>,
    pub(crate) high_set_longest_hours: Vec<f64>,
    pub(crate) low_set_longest_start: Vec<i32>,
    pub(crate) high_set_longest_start: Vec<i32>,
}
impl Default for SourceSurfaceManagement {
    fn default() -> Self {
        Self {
            manage_surface_heat_balance_first_time: true,
            report_thermal_resilience_first_time: true,
            report_var_heat_index: false,
            report_var_humidex: false,
            has_pierce_set: true,
            report_co2_resilience_first_time: true,
            report_visual_resilience_first_time: true,
            low_set_longest_hours: Vec::new(),
            high_set_longest_hours: Vec::new(),
            low_set_longest_start: Vec::new(),
            high_set_longest_start: Vec::new(),
        }
    }
}
/// Every child must be a genuine port on the same moved state/context owners.
/// No successful default body or copied flag/temperature callback is provided.
pub(crate) trait SourceManageSurfaceChildren {
    fn control(&self) -> &SelectedHeatBalanceInputState;
    fn globals(&self) -> &SelectedHeatBalanceGlobals;
    fn tabular(&self) -> &SourceTabularReports;
    fn display_string(&mut self, message: &str) -> Result<(), Error>;
    fn initialize_surface_heat_balance(&mut self) -> Result<(), Error>;
    fn calculate_outside_surface_heat_balance(&mut self) -> Result<(), Error>;
    fn calculate_inside_surface_heat_balance(&mut self) -> Result<(), Error>;
    fn manage_air_heat_balance(&mut self) -> Result<(), Error>;
    fn update_final_surface_heat_balance(&mut self) -> Result<(), Error>;
    fn update_thermal_histories(&mut self) -> Result<(), Error>;
    fn update_condfd_moisture_balance(&mut self) -> Result<(), Error>;
    fn manage_thermal_comfort(&mut self, initialize_only: bool) -> Result<(), Error>;
    fn report_surface_heat_balance(&mut self) -> Result<(), Error>;
    fn gather_component_loads_surface(&mut self) -> Result<(), Error>;
    fn calculate_thermal_resilience(
        &mut self,
        state: &mut SourceSurfaceManagement,
    ) -> Result<(), Error>;
    fn report_thermal_resilience(
        &mut self,
        state: &mut SourceSurfaceManagement,
    ) -> Result<(), Error>;
    fn report_co2_resilience(&mut self, state: &mut SourceSurfaceManagement) -> Result<(), Error>;
    fn report_visual_resilience(
        &mut self,
        state: &mut SourceSurfaceManagement,
    ) -> Result<(), Error>;
}
impl SourceSurfaceManagement {
    pub(crate) fn manage_source_surface_heat_balance(
        &mut self,
        children: &mut impl SourceManageSurfaceChildren,
    ) -> Result<(), Error> {
        if self.manage_surface_heat_balance_first_time {
            children.display_string("Initializing Surfaces")?;
        }
        children.initialize_surface_heat_balance()?;
        if self.manage_surface_heat_balance_first_time {
            children.display_string("Calculate Outside Surface Heat Balance")?;
        }
        children.calculate_outside_surface_heat_balance()?;
        if self.manage_surface_heat_balance_first_time {
            children.display_string("Calculate Inside Surface Heat Balance")?;
        }
        children.calculate_inside_surface_heat_balance()?;
        if self.manage_surface_heat_balance_first_time {
            children.display_string("Calculate Air Heat Balance")?;
        }
        children.manage_air_heat_balance()?;
        children.update_final_surface_heat_balance()?;
        if children.control().any_ctf || children.control().any_empd {
            children.update_thermal_histories()?;
        }
        if children.control().any_cond_fd {
            children.update_condfd_moisture_balance()?;
        }
        children.manage_thermal_comfort(false)?;
        children.report_surface_heat_balance()?;
        if children.globals().zone_sizing_calculation {
            children.gather_component_loads_surface()?;
        }
        children.calculate_thermal_resilience(self)?;
        if children.tabular().display_thermal_resilience_summary {
            children.report_thermal_resilience(self)?;
        }
        if children.tabular().display_co2_resilience_summary {
            children.report_co2_resilience(self)?;
        }
        if children.tabular().display_visual_resilience_summary {
            children.report_visual_resilience(self)?;
        }
        self.manage_surface_heat_balance_first_time = false;
        Ok(())
    }
}
