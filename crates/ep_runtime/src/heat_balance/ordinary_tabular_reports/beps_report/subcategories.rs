//! Literal writeBEPSEndUseBySubCatOrSpaceType9672-9925 on the same local arrays.
use super::helpers::{category, count, subcategory};
use super::prepare::SourceBepsLocals;
use super::{Error, SourceBepsReportOwners, SourceTabularBody, SourceTabularReports};
fn next_row(row: &mut usize) -> Result<(), Error> {
    let native =
        i32::try_from(*row).map_err(|_| Error::Unavailable("actual BEPS row signed-int domain"))?;
    *row = usize::try_from(
        native
            .checked_add(1)
            .ok_or(Error::Unavailable("actual BEPS row signed increment"))?,
    )
    .map_err(|_| Error::Unavailable("actual BEPS row ordinal domain"))?;
    Ok(())
}
impl SourceTabularReports {
    // Keep Original constructor/local initialization and later assignment order, including unconsumed suffix writes.
    #[allow(unused_assignments)]
    pub(super) fn source_beps_subcategory_table(
        &mut self,
        pass: usize,
        by_space: bool,
        l: &mut SourceBepsLocals,
        o: &mut SourceBepsReportOwners<'_, '_>,
    ) -> Result<(), Error> {
        let mut cols = vec![Vec::new(); 15];
        let mut widths = vec![10; 15];
        let names = [
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
        match self.input_state.tabular_report_passes[pass].units_style {
            1 => {
                for i in 0..13 {
                    cols[i + 1] = format!("{} [kWh]", names[i]).into_bytes();
                }
                cols[14] = b"Water [m3]".to_vec();
            }
            4 => {
                for i in 0..13 {
                    cols[i + 1] = format!("{} [kBtu]", names[i]).into_bytes();
                }
                cols[14] = b"Water [gal]".to_vec();
            }
            5 => {
                cols[1] = b"Electricity [kWh]".to_vec();
                for i in 1..13 {
                    cols[i + 1] = format!("{} [kBtu]", names[i]).into_bytes();
                }
                cols[14] = b"Water [gal]".to_vec();
            }
            _ => {
                for i in 0..13 {
                    cols[i + 1] = format!("{} [GJ]", names[i]).into_bytes();
                }
                cols[14] = b"Water [m3]".to_vec();
            }
        }
        let mut number = 0;
        let mut num_rows: i32 = 0;
        cols[0] = if by_space {
            b"Space Type".to_vec()
        } else {
            b"Subcategory".to_vec()
        };
        // The two invocations deliberately reuse l.sub_other. The later LEED
        // loop reads the values left by the space-type invocation, as Native.
        let need = if by_space {
            &mut self.input_state.need_other_row_end_use
        } else {
            &mut self.input_state.need_other_row_leed45
        };
        for end_use in 1..=14 {
            need.set([end_use], false)?;
        }
        for resource in 1..=14 {
            for end_use in 1..=14 {
                number = if by_space {
                    category(o.registration, end_use)?.num_space_types
                } else {
                    category(o.registration, end_use)?.num_subcategories
                };
                if number > 0 {
                    l.sub_other.set(
                        [resource, end_use],
                        l.collapsed_end_use.read([resource, end_use])?,
                    )?;
                    for sub in 1..=number {
                        let term = if by_space {
                            l.collapsed_space.read([sub as usize, end_use, resource])?
                        } else {
                            l.collapsed_sub.read([sub as usize, end_use, resource])?
                        };
                        let value = l.sub_other.read([resource, end_use])? - term;
                        l.sub_other.set([resource, end_use], value)?;
                    }
                    if l.sub_other.read([resource, end_use])?.abs() > 0.01 {
                        need.set([end_use], true)?;
                    } else {
                        l.sub_other.set([resource, end_use], 0.0)?;
                    }
                } else {
                    l.sub_other.set([resource, end_use], 0.0)?;
                }
            }
        }
        for end_use in 1..=14 {
            number = if by_space {
                category(o.registration, end_use)?.num_space_types
            } else {
                category(o.registration, end_use)?.num_subcategories
            };
            if number > 0 {
                for _sub in 1..=number {
                    num_rows = num_rows.checked_add(1).ok_or(Error::Unavailable(
                        "actual BEPS9804 numRows signed increment",
                    ))?;
                }
                if need.read([end_use])? {
                    num_rows = num_rows.checked_add(1).ok_or(Error::Unavailable(
                        "actual BEPS9807 numRows signed increment",
                    ))?;
                }
            } else {
                num_rows = num_rows.checked_add(1).ok_or(Error::Unavailable(
                    "actual BEPS9810 numRows signed increment",
                ))?;
            }
        }
        let num_rows = count(num_rows)?;
        let mut rows = vec![Vec::new(); num_rows];
        let mut body = SourceTabularBody::allocate(15, num_rows)?;
        for col in 1..=15 {
            for row in 1..=num_rows {
                rows[row - 1].clear();
                body.set(col, row, Vec::new())?;
            }
        }
        {
            let mut row = 1;
            for end_use in 1..=14 {
                number = if by_space {
                    category(o.registration, end_use)?.num_space_types
                } else {
                    category(o.registration, end_use)?.num_subcategories
                };
                rows[row - 1] = category(o.registration, end_use)?
                    .display_name
                    .as_bytes()
                    .to_vec();
                if number > 0 {
                    for sub in 1..=number {
                        body.set(
                            1,
                            row,
                            subcategory(o.registration, end_use, sub as usize, by_space)?,
                        )?;
                        next_row(&mut row)?;
                    }
                    if need.read([end_use])? {
                        body.set(1, row, b"Other".to_vec())?;
                        next_row(&mut row)?;
                    }
                } else {
                    body.set(
                        1,
                        row,
                        if by_space {
                            b"Unassigned".to_vec()
                        } else {
                            b"General".to_vec()
                        },
                    )?;
                    next_row(&mut row)?;
                }
            }
        }
        for resource in 1..=14 {
            let mut row = 1;
            for end_use in 1..=14 {
                number = if by_space {
                    category(o.registration, end_use)?.num_space_types
                } else {
                    category(o.registration, end_use)?.num_subcategories
                };
                if number > 0 {
                    for sub in 1..=number {
                        let value = if by_space {
                            l.collapsed_space.read([sub as usize, end_use, resource])?
                        } else {
                            l.collapsed_sub.read([sub as usize, end_use, resource])?
                        };
                        // This function cannot retain the need-array borrow
                        // across write_source_table; actual flag reads precede it.
                        body.set(
                            resource + 1,
                            row,
                            super::helpers::real(
                                self.input_state.tabular_report_passes[pass].format_reals,
                                value,
                                2,
                            )?,
                        )?;
                        next_row(&mut row)?;
                    }
                    if need.read([end_use])? {
                        body.set(
                            resource + 1,
                            row,
                            super::helpers::real(
                                self.input_state.tabular_report_passes[pass].format_reals,
                                l.sub_other.read([resource, end_use])?,
                                2,
                            )?,
                        )?;
                        next_row(&mut row)?;
                    }
                } else {
                    body.set(
                        resource + 1,
                        row,
                        super::helpers::real(
                            self.input_state.tabular_report_passes[pass].format_reals,
                            l.collapsed_end_use.read([resource, end_use])?,
                            2,
                        )?,
                    )?;
                    next_row(&mut row)?;
                }
            }
        }
        if self.display_tabular_beps {
            let title: &[u8] = if by_space {
                b"End Uses By Space Type"
            } else {
                b"End Uses By Subcategory"
            };
            if self.input_state.tabular_report_passes[pass].produce_tabular {
                self.write_source_subtitle(title)?;
                self.write_source_table(
                    &body,
                    &rows,
                    &cols,
                    &mut widths,
                    false,
                    b"",
                    &mut o.writer,
                )?;
            }
            let mut row_temp = rows.clone();
            super::super::writers::source_fill_row_head(&mut row_temp);
            for row in 1..=num_rows {
                row_temp[row - 1].push(b':');
                row_temp[row - 1].extend_from_slice(body.at(1, row)?);
            }
            let mut body_temp = SourceTabularBody::allocate(14, num_rows)?;
            for col in 1..=14 {
                for row in 1..=num_rows {
                    body_temp.set(col, row, body.at(col + 1, row)?.to_vec())?;
                }
            }
            let col_temp = cols[1..].to_vec();
            if self.input_state.tabular_report_passes[pass].produce_sqlite {
                if o.sqlite.present() {
                    return Err(Error::Unavailable(
                        "actual BEPS9911 SQLite subcategory/space-type table",
                    ));
                }
            }
            if self.input_state.tabular_report_passes[pass].produce_json {
                if o.results.time_series_and_tabular_enabled {
                    return Err(Error::Unavailable(
                        "actual BEPS9917 JSON subcategory/space-type table",
                    ));
                }
            }
            drop(row_temp);
            drop(body_temp);
            drop(col_temp);
        }
        Ok(())
    }
}
