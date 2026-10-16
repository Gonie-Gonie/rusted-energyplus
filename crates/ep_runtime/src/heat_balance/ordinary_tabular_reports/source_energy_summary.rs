//! Literal Source Energy End Use Components Summary9926-10321.
mod columns;
use super::beps_report::source_report_real;
use super::{Error, SourceBepsReportOwners, SourceTabularBody, SourceTabularReports};
use crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray;
use crate::heat_balance::source_output_registration::SourceOutputStoreType;

fn allocated<const N: usize>(shape: [usize; N]) -> Result<SourceSolarArray<f64, N>, Error> {
    let mut values = SourceSolarArray::default();
    values.dimension(shape, None)?;
    Ok(values)
}
impl SourceTabularReports {
    /// Actual source-energy table entry on the existing gathered TAB state.
    pub(crate) fn write_source_energy_end_use_summary(
        &mut self,
        o: &mut SourceBepsReportOwners<'_, '_>,
    ) -> Result<(), Error> {
        if !self.display_source_energy_end_use_summary {
            return Ok(());
        }
        // Native local string/integer arrays are unallocated at declaration.
        let mut column_head: Vec<Vec<u8>>;
        let mut column_width: Vec<i32>;
        let mut row_head: Vec<Vec<u8>>;
        let mut table_body: SourceTabularBody;
        // Four actual local numeric allocations, including the unused fourth.
        let mut use_val = allocated([14, 15])?;
        let mut collapsed_total = allocated([14])?;
        let mut collapsed_end_use = allocated([14, 14])?;
        let subcategories =
            usize::try_from(o.registration.max_num_subcategories).map_err(|_| {
                Error::Unavailable("actual SourceEnergy9957 nonnegative subcategory extent")
            })?;
        let _collapsed_end_use_sub = allocated([subcategories, 14, 14])?;
        self.write_source_report_headers(
            b"Source Energy End Use Components Summary",
            b"Entire Facility",
            SourceOutputStoreType::Average,
        )?;
        let mut line = b"Values gathered over ".to_vec();
        line.extend_from_slice(&source_report_real(
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
        self.determine_source_building_floor_area_from_owners(o.globals, o.zones)?;
        for pass in 0..self.input_state.tabular_report_passes.len() {
            // Exactly the fourteen Native source-resource assignments per row.
            const SOURCE: [usize; 14] = [1, 2, 6, 8, 9, 10, 11, 12, 13, 14, 3, 4, 5, 7];
            for end_use in 1..=14 {
                for (resource, &source) in SOURCE.iter().enumerate() {
                    collapsed_end_use.set(
                        [resource + 1, end_use],
                        self.input_state
                            .gather_end_use_by_source_beps
                            .read([source, end_use])?,
                    )?;
                }
            }
            for (resource, &source) in SOURCE.iter().enumerate() {
                collapsed_total.set(
                    [resource + 1],
                    self.input_state.gather_totals_by_source_beps[source - 1],
                )?;
            }
            let mut large_conversion_factor;
            let area_conversion_factor;
            let ip_electricity_conversion_factor = 1.0;
            match self.input_state.tabular_report_passes[pass].units_style {
                1 => {
                    large_conversion_factor = 3600000.0;
                    area_conversion_factor = 1.0;
                }
                4 => {
                    return Err(Error::Unavailable(
                        "actual SourceEnergy10016 getSpecificUnitDivider J/kBtu operation",
                    ));
                }
                5 => {
                    return Err(Error::Unavailable(
                        "actual SourceEnergy10020 getSpecificUnitDivider J/kBtu operation",
                    ));
                }
                _ => {
                    large_conversion_factor = 1000000.0;
                    area_conversion_factor = 1.0;
                }
            }
            for resource in 1..=13 {
                let ip_electricity_adjust = if resource == 1 {
                    ip_electricity_conversion_factor
                } else {
                    1.0
                };
                for end_use in 1..=14 {
                    let value = collapsed_end_use.read([resource, end_use])?
                        / (large_conversion_factor / ip_electricity_adjust);
                    collapsed_end_use.set([resource, end_use], value)?;
                }
                let value = collapsed_total.read([resource])?
                    / (large_conversion_factor / ip_electricity_adjust);
                collapsed_total.set([resource], value)?;
            }
            row_head = vec![Vec::new(); 16];
            column_head = vec![Vec::new(); 13];
            column_width = Vec::with_capacity(13);
            column_width.resize(13, 10);
            table_body = SourceTabularBody::allocate(13, 16)?;
            for resource in 1..=14 {
                for end_use in 0..14 {
                    use_val.set(
                        [resource, end_use + 1],
                        collapsed_end_use.read([resource, end_use + 1])?,
                    )?;
                }
                use_val.set([resource, 15], collapsed_total.read([resource])?)?;
            }
            for (i, label) in columns::END_USE_ROWS.iter().enumerate() {
                row_head[i] = label.to_vec();
            }
            row_head[14] = Vec::new();
            row_head[15] = b"Total Source Energy End Use Components".to_vec();
            large_conversion_factor = 1.0;
            columns::assign(
                &mut column_head,
                self.input_state.tabular_report_passes[pass].units_style,
                false,
            );
            if self.input_state.tabular_report_passes[pass].units_style != 1 {
                // Actual default10130 MJ-to-GJ factor follows column13.
                large_conversion_factor = 1000.0;
            }
            table_body.fill(b"");
            for resource in 1..=13 {
                for end_use in 1..=14 {
                    table_body.set(
                        resource,
                        end_use,
                        source_report_real(
                            self.input_state.tabular_report_passes[pass].format_reals,
                            use_val.read([resource, end_use])? / large_conversion_factor,
                            2,
                        )?,
                    )?;
                }
                table_body.set(
                    resource,
                    16,
                    source_report_real(
                        self.input_state.tabular_report_passes[pass].format_reals,
                        use_val.read([resource, 15])? / large_conversion_factor,
                        2,
                    )?,
                )?;
            }
            self.emit_source_energy_table(
                pass,
                b"Source Energy End Use Components Summary",
                &table_body,
                &row_head,
                &column_head,
                &mut column_width,
                o,
                0,
            )?;
            columns::assign(
                &mut column_head,
                self.input_state.tabular_report_passes[pass].units_style,
                true,
            );
            table_body.fill(b"");
            let converted_conditioned_area =
                self.input_state.building_conditioned_floor_area / area_conversion_factor;
            if converted_conditioned_area > 0.0 {
                for resource in 1..=13 {
                    for end_use in 1..=14 {
                        table_body.set(
                            resource,
                            end_use,
                            source_report_real(
                                self.input_state.tabular_report_passes[pass].format_reals,
                                use_val.read([resource, end_use])? / converted_conditioned_area,
                                2,
                            )?,
                        )?;
                    }
                    table_body.set(
                        resource,
                        16,
                        source_report_real(
                            self.input_state.tabular_report_passes[pass].format_reals,
                            use_val.read([resource, 15])? / converted_conditioned_area,
                            2,
                        )?,
                    )?;
                }
            }
            if self.input_state.tabular_report_passes[pass].produce_tabular {
                self.write_source_text_line(b"Normalized Metrics", true)?;
            }
            self.emit_source_energy_table(
                pass,
                b"Source Energy End Use Components Per Conditioned Floor Area",
                &table_body,
                &row_head,
                &column_head,
                &mut column_width,
                o,
                1,
            )?;
            table_body.fill(b"");
            let converted_gross_area =
                self.input_state.building_gross_floor_area / area_conversion_factor;
            if converted_gross_area > 0.0 {
                for resource in 1..=13 {
                    for end_use in 1..=14 {
                        table_body.set(
                            resource,
                            end_use,
                            source_report_real(
                                self.input_state.tabular_report_passes[pass].format_reals,
                                use_val.read([resource, end_use])? / converted_gross_area,
                                2,
                            )?,
                        )?;
                    }
                    table_body.set(
                        resource,
                        16,
                        source_report_real(
                            self.input_state.tabular_report_passes[pass].format_reals,
                            use_val.read([resource, 15])? / converted_gross_area,
                            2,
                        )?,
                    )?;
                }
            }
            self.emit_source_energy_table(
                pass,
                b"Source Energy End Use Components Per Total Floor Area",
                &table_body,
                &row_head,
                &column_head,
                &mut column_width,
                o,
                2,
            )?;
        }
        Ok(())
    }
    fn emit_source_energy_table(
        &mut self,
        pass: usize,
        title: &[u8],
        body: &SourceTabularBody,
        rows: &[Vec<u8>],
        columns: &[Vec<u8>],
        widths: &mut [i32],
        o: &mut SourceBepsReportOwners<'_, '_>,
        table: usize,
    ) -> Result<(), Error> {
        if self.input_state.tabular_report_passes[pass].produce_tabular {
            self.write_source_subtitle(title)?;
            self.write_source_table(body, rows, columns, widths, false, b"", &mut o.writer)?;
        }
        if self.input_state.tabular_report_passes[pass].produce_sqlite {
            if o.sqlite.present() {
                return Err(Error::Unavailable(
                    [
                        "actual SourceEnergy10151 createSQLiteTabularDataRecords",
                        "actual SourceEnergy10258 createSQLiteTabularDataRecords",
                        "actual SourceEnergy10300 createSQLiteTabularDataRecords",
                    ][table],
                ));
            }
        }
        if self.input_state.tabular_report_passes[pass].produce_json {
            if o.results.time_series_and_tabular_enabled {
                return Err(Error::Unavailable(
                    [
                        "actual SourceEnergy10161 TabularReportsCollection.addReportTable",
                        "actual SourceEnergy10268 TabularReportsCollection.addReportTable",
                        "actual SourceEnergy10310 TabularReportsCollection.addReportTable",
                    ][table],
                ));
            }
        }
        Ok(())
    }
}
