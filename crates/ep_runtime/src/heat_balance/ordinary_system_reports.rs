//! Genuine controlled-zone SystemReports on its sole live Source owner.
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
    #[allow(
        dead_code,
        reason = "The generic SystemReports caller view retains real Zone/air-loop/predefined owner borrows; controlled report entries use narrower live-owner arguments instead of copied values."
    )]
    pub(crate) zones: &'a SelectedZoneInputOwners,
    #[allow(
        dead_code,
        reason = "The generic SystemReports caller view retains real Zone/air-loop/predefined owner borrows; controlled report entries use narrower live-owner arguments instead of copied values."
    )]
    pub(crate) air_loop: &'a SourceAirLoopData,
    #[allow(
        dead_code,
        reason = "The generic SystemReports caller view retains real Zone/air-loop/predefined owner borrows; controlled report entries use narrower live-owner arguments instead of copied values."
    )]
    pub(crate) predefined_registry: &'a PredefinedRegistry,
    #[allow(
        dead_code,
        reason = "The generic SystemReports caller view retains real Zone/air-loop/predefined owner borrows; controlled report entries use narrower live-owner arguments instead of copied values."
    )]
    pub(crate) predefined_table: &'a mut PredefinedStringTable,
    #[allow(
        dead_code,
        reason = "The generic SystemReports caller view retains real Zone/air-loop/predefined owner borrows; controlled report entries use narrower live-owner arguments instead of copied values."
    )]
    pub(crate) predefined_summary: &'a mut SourceComfortPredefinedSummary,
}
fn at(ordinal: i32) -> Result<usize, Error> {
    ordinal
        .checked_sub(1)
        .and_then(|v| usize::try_from(v).ok())
        .ok_or(Error::Unavailable("actual SystemReports Source ordinal"))
}

// Captured entry restoration on the same report allocation, test target only.
#[cfg(all(test, feature = "native-controlled-air-cohort"))]
impl SourceSystemReports {
    pub(crate) fn prepare_native_report_cohort_rows(&mut self, count: usize) -> Result<(), Error> {
        if let Some(rows) = &self.zone_vent_rep_vars {
            if rows.len() != count {
                return Err(Error::Unavailable(
                    "report cohort stable ZoneVent allocation",
                ));
            }
        } else {
            self.zone_vent_rep_vars =
                Some(vec![state::SourceZoneVentReportVariables::default(); count]);
        }
        Ok(())
    }
    pub(crate) fn restore_native_report_cohort_row(
        &mut self,
        number: i32,
        values: [f64; 20],
    ) -> Result<usize, Error> {
        let row = self
            .zone_vent_rep_vars
            .as_mut()
            .and_then(|rows| rows.get_mut(at(number).ok()?))
            .ok_or(Error::Unavailable("report cohort real ZoneVent row"))?;
        row.cooling_load_met_by_vent = values[0];
        row.cooling_load_added_by_vent = values[1];
        row.overcooling_by_vent = values[2];
        row.heating_load_met_by_vent = values[3];
        row.heating_load_added_by_vent = values[4];
        row.overheating_by_vent = values[5];
        row.no_load_heating_by_vent = values[6];
        row.no_load_cooling_by_vent = values[7];
        row.oa_mass_flow = values[8];
        row.oa_mass = values[9];
        row.oa_vol_flow_std_rho = values[10];
        row.oa_vol_std_rho = values[11];
        row.oa_vol_flow_crnt_rho = values[12];
        row.oa_vol_crnt_rho = values[13];
        row.mech_ach = values[14];
        row.target_ventilation_flow_voz = values[15];
        row.time_below_voz_dyn = values[16];
        row.time_at_voz_dyn = values[17];
        row.time_above_voz_dyn = values[18];
        row.time_vent_unocc = values[19];
        Ok(std::ptr::from_ref(row) as usize)
    }
    pub(crate) fn native_report_cohort_row_address(&self, number: i32) -> Result<usize, Error> {
        let row = self
            .zone_vent_rep_vars
            .as_ref()
            .and_then(|rows| rows.get(at(number).ok()?))
            .ok_or(Error::Unavailable("report cohort same ZoneVent address"))?;
        Ok(std::ptr::from_ref(row) as usize)
    }
    pub(crate) fn native_report_cohort_row(&self, number: i32) -> Result<[f64; 20], Error> {
        let row = self
            .zone_vent_rep_vars
            .as_ref()
            .and_then(|rows| rows.get(at(number).ok()?))
            .ok_or(Error::Unavailable("report cohort ZoneVent read"))?;
        Ok([
            row.cooling_load_met_by_vent,
            row.cooling_load_added_by_vent,
            row.overcooling_by_vent,
            row.heating_load_met_by_vent,
            row.heating_load_added_by_vent,
            row.overheating_by_vent,
            row.no_load_heating_by_vent,
            row.no_load_cooling_by_vent,
            row.oa_mass_flow,
            row.oa_mass,
            row.oa_vol_flow_std_rho,
            row.oa_vol_std_rho,
            row.oa_vol_flow_crnt_rho,
            row.oa_vol_crnt_rho,
            row.mech_ach,
            row.target_ventilation_flow_voz,
            row.time_below_voz_dyn,
            row.time_at_voz_dyn,
            row.time_above_voz_dyn,
            row.time_vent_unocc,
        ])
    }
}
