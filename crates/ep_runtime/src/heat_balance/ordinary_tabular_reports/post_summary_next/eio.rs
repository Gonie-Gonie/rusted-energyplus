//! Actual EIO-table parser14621-14755 and same-byte splitCommaString14800-14809.
use super::{Error, SourcePostSummaryReportOwners, SourceTabularBody, SourceTabularReports};
use crate::heat_balance::ordinary_tabular_reports::writers::source_stripped_report_text;
use crate::heat_balance::source_initialization::SourceInitializationFiles;
use crate::heat_balance::source_output_registration::SourceOutputStoreType;

fn split_comma_string(input: &[u8]) -> Vec<Vec<u8>> {
    if input.is_empty() {
        return Vec::new();
    }
    let mut fields: Vec<_> = input
        .split(|byte| *byte == b',')
        .map(|field| source_stripped_report_text(field).to_vec())
        .collect();
    // std::getline does not append a new empty field after a terminal delimiter.
    if input.last() == Some(&b',') {
        fields.pop();
    }
    fields
}

fn units_from_heading(heading: &mut Vec<u8>, units: i32) -> Result<i32, Error> {
    // Actual overloaded14781 helper; no missing lookup is replaced by factor0.
    match units {
        4 => {
            return Err(Error::Unavailable(
                "actual Eio14786 LookupSItoIP heading operation",
            ));
        }
        5 => {
            return Err(Error::Unavailable(
                "actual Eio14788 LookupSItoIP heading operation",
            ));
        }
        1 => {
            return Err(Error::Unavailable(
                "actual Eio14790 LookupJtokWH heading operation",
            ));
        }
        _ => {}
    }
    let cur_heading = heading.clone();
    *heading = cur_heading;
    Ok(0)
}

impl SourceTabularReports {
    /// Read the same already-open EIO handle and emit actual SI initialization tables.
    pub(crate) fn write_source_eio_tables(
        &mut self,
        files: &mut SourceInitializationFiles,
        o: &mut SourcePostSummaryReportOwners<'_, '_>,
    ) -> Result<(), Error> {
        if !self.display_eio_summary {
            return Ok(());
        }
        self.write_source_report_headers(
            b"Initialization Summary",
            b"Entire Facility",
            SourceOutputStoreType::Average,
        )?;
        let mut header_lines = Vec::new();
        let mut body_lines = Vec::new();
        for line in files.source_eio_lines()? {
            let first = *line.first().ok_or(Error::Unavailable(
                "actual Eio14642 std::string::at(0) on an empty EIO line",
            ))?;
            if first == b'!' {
                header_lines.push(line);
            } else if first == b' ' {
                body_lines.push(line[1..].to_vec());
            } else {
                body_lines.push(line);
            }
        }
        for pass in 0..self.input_state.tabular_report_passes.len() {
            for header_line in &header_lines {
                let header_fields = split_comma_string(header_line);
                let with_signs = header_fields.first().ok_or(Error::Unavailable(
                    "actual Eio14658 headerFields.at(0) domain",
                ))?;
                if with_signs.len() < 3 {
                    return Err(Error::Unavailable(
                        "actual Eio14660 std::string::substr(3) position domain",
                    ));
                }
                // Native size_t subtraction wraps for length3; substr clamps count.
                let count = with_signs.len().wrapping_sub(4);
                let end = 3usize.saturating_add(count).min(with_signs.len());
                let table_name = with_signs[3..end].to_vec();
                let mut matching = 0i32;
                for body_line in &body_lines {
                    if body_line.len() > table_name.len() {
                        let mut prefix = table_name.clone();
                        prefix.push(b',');
                        if body_line[..table_name.len() + 1] == prefix {
                            matching = matching.checked_add(1).ok_or(Error::Unavailable(
                                "actual Eio14667 countOfMatchingLines defined int domain",
                            ))?;
                        }
                    }
                }
                let num_rows = matching;
                let num_cols = i32::try_from(header_fields.len() - 1).map_err(|_| {
                    Error::Unavailable("actual Eio14672 numCols representable native int domain")
                })?;
                if num_rows >= 1 {
                    let mut row_head = vec![Vec::new(); num_rows as usize];
                    let mut column_head = vec![Vec::new(); num_cols as usize];
                    let mut column_width = vec![14; num_cols as usize];
                    let mut table_body =
                        SourceTabularBody::allocate(num_cols as usize, num_rows as usize)?;
                    table_body.fill(b"");
                    let mut col_unit_conv = vec![None; num_cols as usize];
                    for col in 1..=num_cols {
                        column_head[col as usize - 1] = header_fields[col as usize].clone();
                        col_unit_conv[col as usize - 1] = Some(units_from_heading(
                            &mut column_head[col as usize - 1],
                            self.input_state.tabular_report_passes[pass].units_style,
                        )?);
                    }
                    let mut row_num = 0i32;
                    for body_line in &body_lines {
                        if body_line.len() > table_name.len() {
                            let mut prefix = table_name.clone();
                            prefix.push(b',');
                            if body_line[..table_name.len() + 1] == prefix {
                                row_num = row_num.checked_add(1).ok_or(Error::Unavailable(
                                    "actual Eio14696 rowNum defined int domain",
                                ))?;
                                if row_num > matching {
                                    break;
                                }
                                let fields = split_comma_string(body_line);
                                row_head[row_num as usize - 1] = row_num.to_string().into_bytes();
                                let field_count = i32::try_from(fields.len()).map_err(|_| {
                                    Error::Unavailable(
                                        "actual Eio14702 dataFields.size native int domain",
                                    )
                                })?;
                                let mut col = 1i32;
                                while col <= num_cols && col < field_count {
                                    if matches!(
                                        self.input_state.tabular_report_passes[pass].units_style,
                                        4 | 5 | 1
                                    ) {
                                        // Reached lookup styles stop in the genuine heading helper first.
                                        return Err(Error::Unavailable(
                                            "actual Eio14706 isNumber conversion operation",
                                        ));
                                    } else {
                                        table_body.set(
                                            col as usize,
                                            row_num as usize,
                                            fields[col as usize].clone(),
                                        )?;
                                    }
                                    col = col.checked_add(1).ok_or(Error::Unavailable(
                                        "actual Eio14702 iCol increment defined int domain",
                                    ))?;
                                }
                            }
                        }
                    }
                    if self.input_state.tabular_report_passes[pass].produce_tabular {
                        self.write_source_subtitle(&table_name)?;
                        let footnote = Vec::new();
                        self.write_source_table(
                            &table_body,
                            &row_head,
                            &column_head,
                            &mut column_width,
                            false,
                            &footnote,
                            &mut o.writer,
                        )?;
                    }
                    if self.input_state.tabular_report_passes[pass].produce_json {
                        if o.results.time_series_and_tabular_enabled {
                            return Err(Error::Unavailable(
                                "actual Eio14740 TabularReportsCollection.addReportTable",
                            ));
                        }
                    }
                    if self.input_state.tabular_report_passes[pass].produce_sqlite {
                        if o.sqlite.present() {
                            return Err(Error::Unavailable(
                                "actual Eio14746 createSQLiteTabularDataRecords",
                            ));
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
