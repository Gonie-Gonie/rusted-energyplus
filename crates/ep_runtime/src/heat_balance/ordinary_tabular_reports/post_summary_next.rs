//! Genuine WriteEioTables5239, LoadComponent5240 and HeatEmission5241 children.
mod eio;
use super::beps_report::source_report_real;
use super::{Error, SourceTabularBody, SourceTabularReports, SourceTabularWriterOwners};
use crate::heat_balance::ordinary_output_control::SourceResultsFrameworkOptions;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::zone::SelectedZoneInputOwners;
use crate::heat_balance::selected_simulation_startup::SelectedHvacGlobals;
use crate::heat_balance::source_initialization::SourceSqliteState;
use crate::heat_balance::source_output_registration::SourceOutputStoreType;

pub(crate) struct SourcePostSummaryReportOwners<'a, 'backend> {
    pub(crate) globals: &'a SelectedHeatBalanceGlobals,
    pub(crate) zones: &'a SelectedZoneInputOwners,
    pub(crate) hvac: &'a SelectedHvacGlobals,
    pub(crate) results: &'a SourceResultsFrameworkOptions,
    pub(crate) sqlite: &'a SourceSqliteState,
    pub(crate) writer: SourceTabularWriterOwners<'a, 'backend>,
}

impl SourceTabularReports {
    /// Native15258 guard on the same three retained flags and actual CompLoadReportIsReq.
    pub(crate) fn write_source_load_component_summary(
        &mut self,
        o: &SourcePostSummaryReportOwners<'_, '_>,
    ) -> Result<(), Error> {
        if !((self.display_zone_component_load_summary
            || self.display_air_loop_component_load_summary
            || self.display_facility_component_load_summary)
            && o.globals.component_load_report_requested)
        {
            return Ok(());
        }
        // Actual15263 takes a reference, before the first missing sizing-owner read.
        let _number_of_primary_air_systems = &o.hvac.number_of_primary_air_systems;
        Err(Error::Unavailable(
            "actual WriteLoadComponentSummaryTables15265 CalcFinalFacilitySizing owner read",
        ))
    }

    /// Native13868-13946 on the existing, actually gathered BuildingPreDefRep cells.
    // Keep Original constructor/local initialization and later assignment order, including unconsumed suffix writes.
    #[allow(unused_assignments)]
    pub(crate) fn write_source_heat_emission_table(
        &mut self,
        o: &mut SourcePostSummaryReportOwners<'_, '_>,
    ) -> Result<(), Error> {
        // The six string cells are allocated before the actual report guard.
        let mut column_head = vec![Vec::new(); 6];
        if self.display_heat_emissions_summary {
            for pass in 0..self.input_state.tabular_report_passes.len() {
                if self.input_state.tabular_report_passes[pass].produce_tabular {
                    self.write_source_report_headers(
                        b"Annual Heat Emissions Report",
                        b"Entire Facility",
                        SourceOutputStoreType::Average,
                    )?;
                    self.write_source_subtitle(b"Annual Heat Emissions Summary")?;
                }
                let mut column_width = vec![10; 6];
                let mut row_head = vec![Vec::new(); 1];
                let mut table_body = SourceTabularBody::allocate(6, 1)?;
                let mut energy_conversion = 1.0;
                match self.input_state.tabular_report_passes[pass].units_style {
                    4 | 5 => {
                        row_head[0] = b"Heat Emissions [kBtu]".to_vec();
                        return Err(Error::Unavailable(
                            "actual HeatEmission13896 getSpecificUnitDivider GJ/kBtu operation",
                        ));
                    }
                    3 => {
                        row_head[0] = b"Heat Emissions [GJ]".to_vec();
                        energy_conversion = 1.0;
                    }
                    1 => {
                        row_head[0] = b"Heat Emissions [kWh]".to_vec();
                        energy_conversion = 1.0e3 / 3.6;
                    }
                    2 => {
                        row_head[0] = b"Heat Emissions [MJ]".to_vec();
                        energy_conversion = 1.0e3;
                    }
                    0 => {
                        row_head[0] = b"Heat Emissions [GJ]".to_vec();
                        energy_conversion = 1.0;
                    }
                    _ => {
                        row_head[0] = b"Heat Emissions [GJ]".to_vec();
                        energy_conversion = 1.0;
                    }
                }
                column_head[0] = b"Envelope Convection".to_vec();
                column_head[1] = b"Zone Exfiltration".to_vec();
                column_head[2] = b"Zone Exhaust Air".to_vec();
                column_head[3] = b"HVAC Relief Air".to_vec();
                column_head[4] = b"HVAC Reject Heat".to_vec();
                column_head[5] = b"Total".to_vec();
                table_body.fill(b"");
                table_body.set(
                    1,
                    1,
                    source_report_real(
                        self.input_state.tabular_report_passes[pass].format_reals,
                        o.zones.building_predefined.emi_envelop_conv * energy_conversion,
                        2,
                    )?,
                )?;
                table_body.set(
                    2,
                    1,
                    source_report_real(
                        self.input_state.tabular_report_passes[pass].format_reals,
                        o.zones.building_predefined.emi_zone_exfiltration * energy_conversion,
                        2,
                    )?,
                )?;
                table_body.set(
                    3,
                    1,
                    source_report_real(
                        self.input_state.tabular_report_passes[pass].format_reals,
                        o.zones.building_predefined.emi_zone_exhaust * energy_conversion,
                        2,
                    )?,
                )?;
                table_body.set(
                    4,
                    1,
                    source_report_real(
                        self.input_state.tabular_report_passes[pass].format_reals,
                        o.zones.building_predefined.emi_hvac_relief * energy_conversion,
                        2,
                    )?,
                )?;
                table_body.set(
                    5,
                    1,
                    source_report_real(
                        self.input_state.tabular_report_passes[pass].format_reals,
                        o.zones.building_predefined.emi_hvac_reject * energy_conversion,
                        2,
                    )?,
                )?;
                table_body.set(
                    6,
                    1,
                    source_report_real(
                        self.input_state.tabular_report_passes[pass].format_reals,
                        o.zones.building_predefined.emi_tot_heat * energy_conversion,
                        2,
                    )?,
                )?;
                if self.input_state.tabular_report_passes[pass].produce_tabular {
                    self.write_source_table(
                        &table_body,
                        &row_head,
                        &column_head,
                        &mut column_width,
                        false,
                        b"",
                        &mut o.writer,
                    )?;
                }
                // Native HeatEmission uses JSON before SQLite, unlike BEPS/Energy.
                if self.input_state.tabular_report_passes[pass].produce_json {
                    if o.results.time_series_and_tabular_enabled {
                        return Err(Error::Unavailable(
                            "actual HeatEmission13934 TabularReportsCollection.addReportTable",
                        ));
                    }
                }
                if self.input_state.tabular_report_passes[pass].produce_sqlite {
                    if o.sqlite.present() {
                        return Err(Error::Unavailable(
                            "actual HeatEmission13940 createSQLiteTabularDataRecords",
                        ));
                    }
                }
            }
        }
        Ok(())
    }
}
