//! WriteThermal/CO2/VisualResilienceTables on the actual retained Zone bins.
//! These reporters neither gather nor initialize scientific state.
mod bins;
mod thermal;
mod values;
use super::{Error, SourceTabularBody, SourceTabularReports, SourceTabularWriterOwners};
use crate::heat_balance::ctf_initialization_context::internal_heat_gains_input::SelectedInternalHeatGains;
use crate::heat_balance::ctf_reports::PredefinedStringTable;
use crate::heat_balance::ordinary_output_control::SourceResultsFrameworkOptions;
use crate::heat_balance::predefined_tables::PredefinedRegistry;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::zone::SelectedZoneInputOwners;
use crate::heat_balance::source_initialization::SourceSqliteState;
use crate::heat_balance::source_output_registration::SourceOutputStoreType;
use values::{Field, count, index, value, zone_name};

pub(crate) struct SourceResilienceTableOwners<'a, 'backend> {
    pub(crate) globals: &'a SelectedHeatBalanceGlobals,
    pub(crate) zones: &'a SelectedZoneInputOwners,
    pub(crate) gains: &'a SelectedInternalHeatGains,
    pub(crate) registry: &'a PredefinedRegistry,
    pub(crate) table: &'a mut PredefinedStringTable,
    pub(crate) results: &'a SourceResultsFrameworkOptions,
    pub(crate) sqlite: &'a SourceSqliteState,
    pub(crate) writer: SourceTabularWriterOwners<'a, 'backend>,
}

// Actual local column arrays/body. RowHead is independently allocated/reused
// by Native thermal/unmet/discomfort helpers and is passed by mutable reference.
struct Columns {
    headings: Vec<Vec<u8>>,
    widths: Vec<i32>,
    body: SourceTabularBody,
}
impl Columns {
    fn allocate(columns: usize, rows: usize) -> Result<Self, Error> {
        let widths = vec![10; columns];
        let headings = vec![Vec::new(); columns];
        let body = SourceTabularBody::allocate(columns, rows)?;
        Ok(Self {
            headings,
            widths,
            body,
        })
    }
    fn headings(&mut self, headings: &[&str]) -> Result<(), Error> {
        if self.headings.len() != headings.len() {
            return Err(Error::Unavailable("actual resilience heading array extent"));
        }
        for (target, text) in self.headings.iter_mut().zip(headings) {
            *target = text.as_bytes().to_vec();
        }
        Ok(())
    }
}

impl SourceTabularReports {
    fn resilience_real(&self, pass: usize, value: f64) -> Result<Vec<u8>, Error> {
        super::beps_report::source_report_real(
            self.input_state.tabular_report_passes[pass].format_reals,
            value,
            2,
        )
    }
    fn resilience_subtitle(&mut self, pass: usize, title: &[u8]) -> Result<(), Error> {
        if self.input_state.tabular_report_passes[pass].produce_tabular {
            self.write_source_subtitle(title)?;
        }
        Ok(())
    }
    fn resilience_emit(
        &mut self,
        pass: usize,
        title: &[u8],
        rows: &[Vec<u8>],
        cols: &mut Columns,
        o: &mut SourceResilienceTableOwners<'_, '_>,
    ) -> Result<(), Error> {
        if self.input_state.tabular_report_passes[pass].produce_tabular {
            self.write_source_table(
                &cols.body,
                rows,
                &cols.headings,
                &mut cols.widths,
                false,
                b"",
                &mut o.writer,
            )?;
        }
        if self.input_state.tabular_report_passes[pass].produce_sqlite && o.sqlite.present() {
            return Err(Error::Unavailable(
                "actual resilience createSQLiteTabularDataRecords after physical table",
            ));
        }
        if self.input_state.tabular_report_passes[pass].produce_json
            && o.results.time_series_and_tabular_enabled
        {
            return Err(Error::Unavailable(
                "actual resilience TabularReportsCollection.addReportTable after SQLite",
            ));
        }
        let _ = title;
        Ok(())
    }
    // Native12884-12915. These are actual PreDef numeric writes, not physical
    // tables here; the later sole WritePredefinedTables consumes these entries.
    fn resilience_predefined(
        &mut self,
        columns: &[&str],
        field: Field,
        o: &mut SourceResilienceTableOwners<'_, '_>,
    ) -> Result<(), Error> {
        let mut ids = Vec::with_capacity(columns.len());
        for name in columns {
            ids.push(
                *o.registry
                    .named_indices
                    .get(*name)
                    .ok_or(Error::Unavailable(
                        "actual resilience predefined column owner",
                    ))?,
            );
        }
        let mut max = vec![0.0; columns.len()];
        let mut min = vec![0.0; columns.len()];
        let mut sum = vec![0.0; columns.len()];
        for j in 0..columns.len() {
            min[j] = value(o.zones, 1, field, j)? * 1.0;
        }
        for zone in 1..=o.globals.num_of_zones {
            let name = zone_name(o.zones, zone)?;
            for j in 0..columns.len() {
                let current = value(o.zones, zone, field, j)? * 1.0;
                if current > max[j] {
                    max[j] = current;
                }
                if current < min[j] {
                    min[j] = current;
                }
                sum[j] += current;
                o.table.fixed_real_entry_2(ids[j], name, current)?;
            }
        }
        for j in 0..columns.len() {
            o.table.fixed_real_entry_2(ids[j], "Min", min[j])?;
            o.table.fixed_real_entry_2(ids[j], "Max", max[j])?;
            o.table.fixed_real_entry_2(
                ids[j],
                "Average",
                sum[j] / f64::from(o.globals.num_of_zones),
            )?;
            o.table.fixed_real_entry_2(ids[j], "Sum", sum[j])?;
        }
        Ok(())
    }
    pub(crate) fn write_source_co2_resilience_tables(
        &mut self,
        o: &mut SourceResilienceTableOwners<'_, '_>,
    ) -> Result<(), Error> {
        if o.globals.num_of_zones <= 0 {
            return Ok(());
        }
        self.resilience_predefined(
            &["pdchCO2HourSafe", "pdchCO2HourCaution", "pdchCO2HourHazard"],
            Field::Co2,
            o,
        )?;
        self.resilience_predefined(
            &[
                "pdchCO2OccuHourSafe",
                "pdchCO2OccuHourCaution",
                "pdchCO2OccuHourHazard",
            ],
            Field::Co2Occu,
            o,
        )?;
        self.resilience_predefined(
            &[
                "pdchCO2OccupiedHourSafe",
                "pdchCO2OccupiedHourCaution",
                "pdchCO2OccupiedHourHazard",
            ],
            Field::Co2Occupied,
            o,
        )?;
        Ok(())
    }
    pub(crate) fn write_source_visual_resilience_tables(
        &mut self,
        o: &mut SourceResilienceTableOwners<'_, '_>,
    ) -> Result<(), Error> {
        // The genuine per-Zone daylight array is allocated with Zone at Native
        // HeatBalanceManager1854, even when no Daylighting Controls are present.
        for zone in 1..=o.globals.num_of_zones {
            let daylight = o
                .zones
                .zone_daylight
                .as_ref()
                .and_then(|rows| index(zone).ok().and_then(|i| rows.get(i)))
                .ok_or(Error::Unavailable(
                    "actual WriteVisualResilience13758 ZoneDaylight row",
                ))?;
            if daylight.total_reference_points == 0
                && self.display_visual_resilience_summary_explicitly
            {
                o.writer.backend.show_warning_error(&format!("Writing Annual Visual Resilience Summary - Lighting Level Hours reports: Zone Average Daylighting Reference Point Illuminance output is required, but no Daylighting Controls are defined in Zone:{}",zone_name(o.zones,zone)?))?;
            }
        }
        if o.globals.num_of_zones <= 0 {
            return Ok(());
        }
        self.resilience_predefined(
            &[
                "pdchIllumHourDark",
                "pdchIllumHourDim",
                "pdchIllumHourAdequate",
                "pdchIllumHourBright",
            ],
            Field::Lighting,
            o,
        )?;
        self.resilience_predefined(
            &[
                "pdchIllumOccuHourDark",
                "pdchIllumOccuHourDim",
                "pdchIllumOccuHourAdequate",
                "pdchIllumOccuHourBright",
            ],
            Field::LightingOccu,
            o,
        )?;
        self.resilience_predefined(
            &[
                "pdchIllumOccupiedHourDark",
                "pdchIllumOccupiedHourDim",
                "pdchIllumOccupiedHourAdequate",
                "pdchIllumOccupiedHourBright",
            ],
            Field::LightingOccupied,
            o,
        )?;
        Ok(())
    }
}
