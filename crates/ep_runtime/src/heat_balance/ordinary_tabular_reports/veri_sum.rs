//! Actual WriteVeriSumTable11101-11972 and required space child11974-12354.
mod envelope;
mod helpers;
mod spaces;
mod zones;
use super::{Error, SourceTabularBody, SourceTabularReports, SourceTabularWriterOwners};
use crate::heat_balance::ctf_initialization_context::internal_heat_gains_input::SelectedInternalHeatGains;
use crate::heat_balance::ctf_reports::PredefinedStringTable;
use crate::heat_balance::ordinary_air_heat_balance::predictor_corrector::SourceZoneAirPredictorCorrector;
use crate::heat_balance::ordinary_output_control::SourceResultsFrameworkOptions;
use crate::heat_balance::predefined_tables::PredefinedRegistry;
use crate::heat_balance::radiation::ordinary::OrdinarySolarRadiation;
use crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::{
    SelectedHeatBalanceInputState, surface::SurfaceGeometryInputState,
    zone::SelectedZoneInputOwners,
};
use crate::heat_balance::selected_input_processor::SelectedInputProcessor;
use crate::heat_balance::source_initialization::SourceSqliteState;
use crate::heat_balance::source_output_registration::SourceOutputStoreType;
use crate::weather::day::WeatherEnvironmentState;
use helpers::{Table, count, label, real};

pub(crate) struct SourceVeriSumOwners<'a, 'backend> {
    pub(crate) globals: &'a SelectedHeatBalanceGlobals,
    pub(crate) environment: &'a WeatherEnvironmentState,
    pub(crate) weather_file_location_title: &'a str,
    pub(crate) control: &'a SelectedHeatBalanceInputState,
    pub(crate) geometry: &'a SurfaceGeometryInputState,
    pub(crate) zones: &'a SelectedZoneInputOwners,
    pub(crate) gains: &'a SelectedInternalHeatGains,
    pub(crate) controller: &'a SourceZoneAirPredictorCorrector,
    pub(crate) radiation: &'a OrdinarySolarRadiation,
    pub(crate) input: &'a SelectedInputProcessor,
    pub(crate) registry: &'a PredefinedRegistry,
    pub(crate) table: &'a mut PredefinedStringTable,
    pub(crate) results: &'a SourceResultsFrameworkOptions,
    pub(crate) sqlite: &'a SourceSqliteState,
    pub(crate) writer: SourceTabularWriterOwners<'a, 'backend>,
}
pub(super) struct Locals {
    opening: SourceSolarArray<f64, 1>,
    glass: SourceSolarArray<f64, 1>,
}
impl SourceTabularReports {
    pub(crate) fn write_source_verification_summary(
        &mut self,
        o: &mut SourceVeriSumOwners<'_, '_>,
    ) -> Result<(), Error> {
        if !self.display_tabular_veri_sum {
            return Ok(());
        }
        let mut opening = SourceSolarArray::default();
        opening.dimension([count(o.globals.num_of_zones)?], None)?;
        let mut glass = SourceSolarArray::default();
        glass.dimension([count(o.globals.num_of_zones)?], None)?;
        let mut local = Locals { opening, glass };
        for pass in 0..self.input_state.tabular_report_passes.len() {
            if self.input_state.tabular_report_passes[pass].produce_tabular {
                self.write_source_report_headers(
                    b"Input Verification and Results Summary",
                    b"Entire Facility",
                    SourceOutputStoreType::Average,
                )?;
            }
            // Actual whole-array assignments11170-11181, in their Source order.
            for cell in 1..=4 {
                self.input_state.zst_area.set([cell], 0.0)?;
            }
            for cell in 1..=4 {
                self.input_state.zst_volume.set([cell], 0.0)?;
            }
            for cell in 1..=4 {
                self.input_state.zst_wall_area.set([cell], 0.0)?;
            }
            for cell in 1..=4 {
                self.input_state.zst_und_wall_area.set([cell], 0.0)?;
            }
            for cell in 1..=4 {
                self.input_state.zst_window_area.set([cell], 0.0)?;
            }
            for cell in 1..=4 {
                self.input_state.zst_opening_area.set([cell], 0.0)?;
            }
            for cell in 1..=4 {
                self.input_state.zst_light.set([cell], 0.0)?;
            }
            for cell in 1..=4 {
                self.input_state.zst_people.set([cell], 0.0)?;
            }
            for cell in 1..=4 {
                self.input_state.zst_plug.set([cell], 0.0)?;
            }
            for cell in 1..=count(o.globals.num_of_zones)? {
                local.opening.set([cell], 0.0)?;
            }
            for cell in 1..=count(o.globals.num_of_zones)? {
                local.glass.set([cell], 0.0)?;
            }
            if self.input_state.tabular_report_passes[pass].units_style == 4
                || self.input_state.tabular_report_passes[pass].units_style == 5
            {
                return Err(Error::Unavailable(
                    "actual VeriSum LookupSItoIP11201/11187 first operation",
                ));
            }
            self.input_state.m_unit_name = b"[m]".to_vec();
            self.input_state.m_unit_conv = 1.0;
            self.input_state.m2_unit_name = b"[m2]".to_vec();
            self.input_state.m2_unit_conv_wvst = 1.0;
            self.input_state.m3_unit_name = b"[m3]".to_vec();
            self.input_state.m3_unit_conv = 1.0;
            self.input_state.wm2_unit_name = b"[W/m2]".to_vec();
            self.input_state.wm2_unit_conv = 1.0;
            self.veri_general(pass, o)?;
            let walls = self.veri_envelope(pass, &mut local, o)?;
            self.veri_hybrid(pass, o)?;
            self.veri_wall_area_warning(pass, &walls, o)?;
            if self.input_state.tabular_report_passes[pass].produce_tabular {
                self.write_source_text_line(b"PERFORMANCE", true)?;
            }
            self.veri_zones(pass, &local, o)?;
            self.veri_spaces(pass, o)?;
        }
        Ok(())
    }
    fn veri_real(&self, pass: usize, value: f64, digits: i32) -> Result<Vec<u8>, Error> {
        real(
            self.input_state.tabular_report_passes[pass].format_reals,
            value,
            digits,
        )
    }
    fn veri_emit(
        &mut self,
        pass: usize,
        title: &[u8],
        table: &mut Table,
        o: &mut SourceVeriSumOwners<'_, '_>,
    ) -> Result<(), Error> {
        if self.input_state.tabular_report_passes[pass].produce_tabular {
            self.write_source_subtitle(title)?;
            self.write_source_table(
                &table.body,
                &table.rows,
                &table.cols,
                &mut table.widths,
                false,
                b"",
                &mut o.writer,
            )?;
        }
        if self.input_state.tabular_report_passes[pass].produce_sqlite && o.sqlite.present() {
            return Err(Error::Unavailable(
                "actual VeriSum createSQLiteTabularDataRecords operation",
            ));
        }
        if self.input_state.tabular_report_passes[pass].produce_json
            && o.results.time_series_and_tabular_enabled
        {
            return Err(Error::Unavailable(
                "actual VeriSum TabularReportsCollection.addReportTable operation",
            ));
        }
        Ok(())
    }
    fn veri_general(
        &mut self,
        pass: usize,
        o: &mut SourceVeriSumOwners<'_, '_>,
    ) -> Result<(), Error> {
        let mut t = Table::new(1, 10)?;
        t.cols[0] = b"Value".to_vec();
        for (i, text) in [
            "Program Version and Build",
            "RunPeriod",
            "Weather File",
            "Latitude [deg]",
            "Longitude [deg]",
        ]
        .iter()
        .enumerate()
        {
            t.rows[i] = text.as_bytes().to_vec();
        }
        t.rows[5] = label(b"Elevation ", &self.input_state.m_unit_name);
        t.rows[6] = b"Time Zone".to_vec();
        t.rows[7] = b"North Axis Angle [deg]".to_vec();
        t.rows[8] = b"Rotation for Appendix G [deg]".to_vec();
        t.rows[9] = b"Hours Simulated [hrs]".to_vec();
        t.body.fill(b"");
        t.body.set(
            1,
            1,
            o.writer.backend.state.version_string.as_bytes().to_vec(),
        )?;
        t.body
            .set(1, 2, o.environment.environment_name.as_bytes().to_vec())?;
        t.body
            .set(1, 3, o.weather_file_location_title.as_bytes().to_vec())?;
        t.body
            .set(1, 4, self.veri_real(pass, o.environment.latitude, 2)?)?;
        t.body
            .set(1, 5, self.veri_real(pass, o.environment.longitude, 2)?)?;
        t.body.set(
            1,
            6,
            self.veri_real(
                pass,
                o.environment.elevation * self.input_state.m_unit_conv,
                2,
            )?,
        )?;
        t.body.set(
            1,
            7,
            self.veri_real(pass, o.environment.time_zone_number, 2)?,
        )?;
        t.body
            .set(1, 8, self.veri_real(pass, o.control.building_azimuth, 2)?)?;
        t.body.set(
            1,
            9,
            self.veri_real(pass, o.control.building_rotation_appendix_g, 2)?,
        )?;
        t.body.set(
            1,
            10,
            self.veri_real(pass, self.input_state.gather_elapsed_time_beps, 2)?,
        )?;
        self.veri_emit(pass, b"General", &mut t, o)
    }
    fn veri_hybrid(
        &mut self,
        pass: usize,
        o: &mut SourceVeriSumOwners<'_, '_>,
    ) -> Result<(), Error> {
        if o.controller.subsystems.hybrid_model.thermal_mass {
            let mut t = Table::new(2, count(o.globals.num_of_zones)?)?;
            t.cols[0] = b"Hybrid Modeling (Y/N)".to_vec();
            t.cols[1] = b"Temperature Capacitance Multiplier ".to_vec();
            t.body.fill(b"");
            // Preserve empty-range behavior, checked first-row access and the first unsupported positive-child failure after prior writes.
            #[allow(clippy::never_loop)]
            for number in 1..=o.globals.num_of_zones {
                let zone = helpers::zone(o.zones, number)?;
                t.rows[(number - 1) as usize] = zone.name.as_bytes().to_vec();
                return Err(Error::Unavailable(
                    "actual VeriSum hybridModelZones.InternalThermalMassCalc_T11688 first read",
                ));
            }
            self.veri_emit(pass, b"Hybrid Model: Internal Thermal Mass", &mut t, o)?;
        }
        Ok(())
    }
}
