//! Actual WriteBEPSTable7770-9660 and its required end-use subtable child.
//! This unsealed draft is not an executable or accepted reporting route.
mod end_uses;
mod helpers;
mod leed;
mod normalized;
mod prepare;
mod site;
mod subcategories;
mod suffix;
use super::{Error, SourceTabularBody, SourceTabularReports, SourceTabularWriterOwners};
use crate::heat_balance::ctf_reports::PredefinedStringTable;
use crate::heat_balance::facility_electric_service::FacilityElectricService;
use crate::heat_balance::ordinary_hvac_manager::water_manager::SourceWaterData;
use crate::heat_balance::ordinary_output_control::SourceResultsFrameworkOptions;
use crate::heat_balance::ordinary_thermal_comfort::SourceComfortPredefinedSummary;
use crate::heat_balance::predefined_tables::PredefinedRegistry;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::zone::SelectedZoneInputOwners;
use crate::heat_balance::selected_schedule_manager::SelectedScheduleManager;
use crate::heat_balance::selected_simulation_startup::SelectedHvacGlobals;
use crate::heat_balance::source_initialization::SourceSqliteState;
use crate::heat_balance::source_output_registration::{
    SourceOutputRegistration, SourceOutputStoreType,
};

pub(crate) struct SourceBepsReportOwners<'a, 'backend> {
    pub(crate) registration: &'a SourceOutputRegistration,
    pub(crate) zones: &'a SelectedZoneInputOwners,
    pub(crate) globals: &'a SelectedHeatBalanceGlobals,
    pub(crate) facility: &'a FacilityElectricService,
    pub(crate) water: &'a SourceWaterData,
    pub(crate) hvac: &'a SelectedHvacGlobals,
    pub(crate) predefined_summary: &'a SourceComfortPredefinedSummary,
    pub(crate) registry: &'a mut PredefinedRegistry,
    pub(crate) table: &'a mut PredefinedStringTable,
    pub(crate) schedules: &'a SelectedScheduleManager,
    pub(crate) results: &'a SourceResultsFrameworkOptions,
    pub(crate) sqlite: &'a SourceSqliteState,
    pub(crate) writer: SourceTabularWriterOwners<'a, 'backend>,
}

impl SourceTabularReports {
    pub(crate) fn write_source_beps_report(
        &mut self,
        o: &mut SourceBepsReportOwners<'_, '_>,
    ) -> Result<(), Error> {
        if !self.display_tabular_beps && !self.display_leed_summary {
            return Ok(());
        }
        let mut local = self.allocate_source_beps_locals(o)?;
        if self.display_tabular_beps {
            self.write_source_report_headers(
                b"Annual Building Utility Performance Summary",
                b"Entire Facility",
                SourceOutputStoreType::Average,
            )?;
            let mut line = b"Values gathered over ".to_vec();
            line.extend_from_slice(&helpers::real(
                true,
                self.input_state.gather_elapsed_time_beps,
                2,
            )?);
            line.extend_from_slice(b" hours");
            self.write_source_text_line(&line, true)?;
            if self.input_state.gather_elapsed_time_beps < 8759.0 {
                self.write_source_text_line(
                    b"WARNING: THE REPORT DOES NOT REPRESENT A FULL ANNUAL SIMULATION.",
                    true,
                )?;
            }
            self.write_source_text_line(b"", true)?;
        }
        let mut first_style = true;
        // Current-style cells are read from the actual vector at each call;
        // no style/format/production flag snapshot or clone is held.
        for pass in 0..self.input_state.tabular_report_passes.len() {
            let conversion = self.prepare_source_beps_pass(pass, first_style, &mut local, o)?;
            let (gross, conditioned) = self.source_beps_site_tables(pass, &conversion, o)?;
            let (_unconvert, unconvert_ip) =
                self.source_beps_end_use_tables(pass, &conversion, &mut local, o)?;
            self.source_beps_normalized_tables(
                pass,
                &conversion,
                &mut local,
                gross,
                conditioned,
                o,
            )?;
            self.source_beps_electric_table(pass, &conversion, &local, unconvert_ip, o)?;
            self.source_beps_thermal_table(pass, first_style, &conversion, &mut local, o)?;
            self.source_beps_water_table(pass, &conversion, o)?;
            self.source_beps_comfort_tables(pass, o)?;
            first_style = false;
        }
        if self.display_tabular_beps {
            self.write_source_text_line(
                b"Note 1: An asterisk (*) indicates that the feature is not yet implemented.",
                false,
            )?;
        }
        Ok(())
    }
    fn beps_real(&self, pass: usize, value: f64, digits: i32) -> Result<Vec<u8>, Error> {
        helpers::real(
            self.input_state.tabular_report_passes[pass].format_reals,
            value,
            digits,
        )
    }
    fn emit_source_beps_table(
        &mut self,
        pass: usize,
        title: &[u8],
        body: &SourceTabularBody,
        rows: &[Vec<u8>],
        cols: &[Vec<u8>],
        widths: &mut [i32],
        footnote: &[u8],
        o: &mut SourceBepsReportOwners<'_, '_>,
    ) -> Result<(), Error> {
        if self.display_tabular_beps {
            if self.input_state.tabular_report_passes[pass].produce_tabular {
                self.write_source_subtitle(title)?;
                self.write_source_table(body, rows, cols, widths, false, footnote, &mut o.writer)?;
            }
            if self.input_state.tabular_report_passes[pass].produce_sqlite {
                if o.sqlite.present() {
                    return Err(Error::Unavailable(
                        "actual BEPS createSQLiteTabularDataRecords operation",
                    ));
                }
            }
            if self.input_state.tabular_report_passes[pass].produce_json {
                if o.results.time_series_and_tabular_enabled {
                    return Err(Error::Unavailable(
                        "actual BEPS TabularReportsCollection.addReportTable operation",
                    ));
                }
            }
        }
        Ok(())
    }
    fn determine_source_building_floor_area(
        &mut self,
        o: &SourceBepsReportOwners<'_, '_>,
    ) -> Result<(), Error> {
        self.input_state.building_gross_floor_area = 0.0;
        self.input_state.building_conditioned_floor_area = 0.0;
        for zone_number in 1..=o.globals.num_of_zones {
            let zone = o
                .zones
                .zones
                .as_ref()
                .and_then(|zones| zones.get((zone_number - 1) as usize))
                .ok_or(Error::Unavailable(
                    "same actual DetermineBuildingFloorArea Zone row",
                ))?;
            let area =
                zone.floor_area * f64::from(zone.multiplier) * f64::from(zone.list_multiplier);
            if zone.is_part_of_total_area {
                self.input_state.building_gross_floor_area += area;
                if zone.system_zone_node_number > 0 {
                    self.input_state.building_conditioned_floor_area += area;
                }
            }
        }
        Ok(())
    }
}
