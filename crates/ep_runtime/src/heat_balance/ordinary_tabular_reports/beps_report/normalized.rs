//! Original normalized-table calculations9042-9227, including the 13-column loop.
use super::prepare::{SourceBepsLocals, SourceBepsPass};
use super::{Error, SourceBepsReportOwners, SourceTabularBody, SourceTabularReports};
impl SourceTabularReports {
    pub(super) fn source_beps_normalized_tables(
        &mut self,
        pass: usize,
        p: &SourceBepsPass,
        l: &mut SourceBepsLocals,
        gross: f64,
        conditioned: f64,
        o: &mut SourceBepsReportOwners<'_, '_>,
    ) -> Result<(), Error> {
        let mut rows = vec![Vec::new(); 4];
        let mut cols = vec![Vec::new(); 14];
        let mut widths = vec![7; 14];
        let mut body = SourceTabularBody::allocate(14, 4)?;
        for resource in 1..=14 {
            l.normal_val.set(
                [resource, 1],
                l.collapsed_end_use.read([resource, 3])?
                    + l.collapsed_end_use.read([resource, 4])?,
            )?;
            l.normal_val.set(
                [resource, 2],
                l.collapsed_end_use.read([resource, 7])?
                    + l.collapsed_end_use.read([resource, 8])?
                    + l.collapsed_end_use.read([resource, 1])?
                    + l.collapsed_end_use.read([resource, 2])?
                    + l.collapsed_end_use.read([resource, 9])?
                    + l.collapsed_end_use.read([resource, 10])?
                    + l.collapsed_end_use.read([resource, 12])?,
            )?;
            l.normal_val.set(
                [resource, 3],
                l.collapsed_end_use.read([resource, 5])?
                    + l.collapsed_end_use.read([resource, 6])?
                    + l.collapsed_end_use.read([resource, 14])?
                    + l.collapsed_end_use.read([resource, 11])?
                    + l.collapsed_end_use.read([resource, 13])?,
            )?;
            l.normal_val
                .set([resource, 4], l.collapsed_total.read([resource])?)?;
        }
        for resource in 1..=13 {
            for end_use in 1..=4 {
                let index = [resource, end_use];
                let value = l.normal_val.read(index)? * p.k;
                l.normal_val.set(index, value)?;
            }
        }
        rows[0] = b"Lighting".to_vec();
        rows[1] = b"HVAC".to_vec();
        rows[2] = b"Other".to_vec();
        rows[3] = b"Total".to_vec();
        let unit = match self.input_state.tabular_report_passes[pass].units_style {
            1 => "kWh/m2",
            4 | 5 => "kBtu/ft2",
            _ => "MJ/m2",
        };
        let resources = [
            "Electricity",
            "Natural Gas",
            "Gasoline",
            "Diesel",
            "Coal",
            "Fuel Oil No 1",
            "Fuel Oil No 2",
            "Propane",
            "Other Fuel 1",
            "Other Fuel 2",
            "District Cooling",
            "District Heating Water",
            "District Heating Steam",
        ];
        for i in 0..13 {
            let this_unit =
                if i == 0 && self.input_state.tabular_report_passes[pass].units_style == 5 {
                    "kWh/ft2"
                } else {
                    unit
                };
            cols[i] = format!("{} Intensity [{}]", resources[i], this_unit).into_bytes();
        }
        cols[13] = if matches!(
            self.input_state.tabular_report_passes[pass].units_style,
            4 | 5
        ) {
            b"Water Intensity [gal/ft2]".to_vec()
        } else {
            b"Water Intensity [m3/m2]".to_vec()
        };
        if self.input_state.tabular_report_passes[pass].produce_tabular {
            self.write_source_text_line(b"Normalized Metrics", true)?;
        }
        body.fill(b"");
        if conditioned > 0.0 {
            for resource in 1..=14 {
                for end_use in 1..=4 {
                    body.set(
                        resource,
                        end_use,
                        self.beps_real(
                            pass,
                            l.normal_val.read([resource, end_use])? / conditioned,
                            2,
                        )?,
                    )?;
                }
            }
        }
        self.emit_source_beps_table(
            pass,
            b"Utility Use Per Conditioned Floor Area",
            &body,
            &rows,
            &cols,
            &mut widths,
            b"",
            o,
        )?;
        body.fill(b"");
        if gross > 0.0 {
            for resource in 1..=13 {
                for end_use in 1..=4 {
                    body.set(
                        resource,
                        end_use,
                        self.beps_real(pass, l.normal_val.read([resource, end_use])? / gross, 2)?,
                    )?;
                }
            }
        }
        self.emit_source_beps_table(
            pass,
            b"Utility Use Per Total Floor Area",
            &body,
            &rows,
            &cols,
            &mut widths,
            b"",
            o,
        )
    }
}
