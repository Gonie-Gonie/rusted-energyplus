//! Genuine SystemReports zero-primary/uncontrolled route on its sole Source owner.
mod initialization;
mod output;
mod reporting;
mod state;
pub(crate) use output::SourceSystemReportsOutputContext;
pub(crate) use state::SourceSystemReports;

use super::ctf_reports::PredefinedStringTable;
use super::ordinary_equipment::OrdinaryEquipment;
use super::ordinary_hvac_manager::simulation::state::SourceAirLoopData;
use super::ordinary_thermal_comfort::SourceComfortPredefinedSummary;
use super::predefined_tables::PredefinedRegistry;
use super::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use super::selected_heat_balance_input::{
    HeatBalanceInputError as Error, zone::SelectedZoneInputOwners,
};
use super::selected_simulation_startup::SelectedHvacGlobals;

/// Constructed per call from the same retained simulation owners.
pub(crate) struct SourceSystemReportOwners<'a> {
    pub(crate) globals: &'a SelectedHeatBalanceGlobals,
    pub(crate) hvac: &'a SelectedHvacGlobals,
    pub(crate) equipment: &'a OrdinaryEquipment,
    pub(crate) zones: &'a SelectedZoneInputOwners,
    pub(crate) air_loop: &'a SourceAirLoopData,
    pub(crate) predefined_registry: &'a PredefinedRegistry,
    pub(crate) predefined_table: &'a mut PredefinedStringTable,
    pub(crate) predefined_summary: &'a mut SourceComfortPredefinedSummary,
}
fn at(ordinal: i32) -> Result<usize, Error> {
    ordinal
        .checked_sub(1)
        .and_then(|v| usize::try_from(v).ok())
        .ok_or(Error::Unavailable("actual SystemReports Source ordinal"))
}
