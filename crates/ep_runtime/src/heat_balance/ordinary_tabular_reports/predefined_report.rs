//! Literal WritePredefinedTables13948-14197, on the existing registry/table owner.
mod access;
use super::beps_report::source_report_real;
use super::{Error, SourceTabularBody, SourceTabularReports, SourceTabularWriterOwners};
use crate::heat_balance::ctf_reports::PredefinedStringTable;
use crate::heat_balance::ordinary_output_control::SourceResultsFrameworkOptions;
use crate::heat_balance::predefined_tables::PredefinedRegistry;
use crate::heat_balance::source_initialization::SourceSqliteState;
use crate::heat_balance::source_output_registration::SourceOutputStoreType;
use access::{entry, entry_mut, increment, ordinal, row, written};

pub(crate) struct SourcePredefinedReportOwners<'a, 'backend> {
    pub(crate) registry: &'a PredefinedRegistry,
    pub(crate) table: &'a mut PredefinedStringTable,
    pub(crate) results: &'a SourceResultsFrameworkOptions,
    pub(crate) sqlite: &'a SourceSqliteState,
    pub(crate) writer: SourceTabularWriterOwners<'a, 'backend>,
}

impl SourceTabularReports {
    /// Associate the real entries and emit every shown report/subtable in Native order.
    pub(crate) fn write_source_predefined_tables(
        &mut self,
        o: &mut SourcePredefinedReportOwners<'_, '_>,
    ) -> Result<(), Error> {
        // Actual initialization is once per function, not once per table/pass.
        let mut col_current = 0i32;
        let mut row_current = 0i32;
        for pass in 0..self.input_state.tabular_report_passes.len() {
            let extent = usize::try_from(o.table.num_table_entry).map_err(|_| {
                Error::Unavailable(
                    "actual Predefined13991 nonnegative tableEntry allocation extent",
                )
            })?;
            let mut unique_object_name = vec![String::new(); extent];
            let mut use_unique_object_name = vec![None; extent];
            let mut num_unique = 0i32;
            for number in 1..=o.table.num_table_entry {
                let current_column = entry(o.table, number)?.index_column;
                if current_column >= 1 && current_column <= o.registry.column_count {
                    let associated = row(
                        &o.registry.columns,
                        current_column,
                        "actual Predefined13998 columnTag row",
                    )?
                    .subtable_index;
                    entry_mut(o.table, number)?.sub_table_index = associated;
                }
                let name = &entry(o.table, number)?.object_name;
                let mut found = 0i32;
                for unique in 1..=num_unique {
                    if name == &unique_object_name[ordinal(unique)?] {
                        found = unique;
                        break;
                    }
                }
                if found > 0 {
                    entry_mut(o.table, number)?.unique_object_name = found;
                } else {
                    num_unique = increment(
                        num_unique,
                        "actual Predefined14014 numUnqObjName int increment",
                    )?;
                    unique_object_name[ordinal(num_unique)?] = name.clone();
                    entry_mut(o.table, number)?.unique_object_name = num_unique;
                }
            }
            for report_number in 1..=o.registry.report_count {
                if row(
                    &o.registry.reports,
                    report_number,
                    "actual Predefined14021 reportName row",
                )?
                .show
                {
                    if self.input_state.tabular_report_passes[pass].produce_tabular {
                        self.write_source_report_headers(
                            row(
                                &o.registry.reports,
                                report_number,
                                "actual Predefined14024 report pretty name",
                            )?
                            .name_with_spaces
                            .as_bytes(),
                            b"Entire Facility",
                            SourceOutputStoreType::Average,
                        )?;
                    }
                    // Native jSubTable_end is an explicit local at14029.
                    let subtable_end = o.registry.subtable_count;
                    for subtable_number in 1..=subtable_end {
                        if row(
                            &o.registry.subtables,
                            subtable_number,
                            "actual Predefined14030 subTable row",
                        )?
                        .report_index
                            == report_number
                        {
                            let mut num_columns = 0i32;
                            for column in 1..=o.registry.column_count {
                                if row(
                                    &o.registry.columns,
                                    column,
                                    "actual Predefined14034 column association",
                                )?
                                .subtable_index
                                    == subtable_number
                                {
                                    num_columns = increment(
                                        num_columns,
                                        "actual Predefined14035 curNumColumns int increment",
                                    )?;
                                }
                            }
                            use_unique_object_name.fill(Some(false));
                            for number in 1..=o.table.num_table_entry {
                                if entry(o.table, number)?.sub_table_index == subtable_number {
                                    let unique = entry(o.table, number)?.unique_object_name;
                                    *use_unique_object_name.get_mut(ordinal(unique)?).ok_or(
                                        Error::Unavailable(
                                            "actual Predefined14043 unique object index domain",
                                        ),
                                    )? = Some(true);
                                }
                            }
                            let mut num_rows = 0i32;
                            for unique in 1..=num_unique {
                                if written(
                                    &use_unique_object_name,
                                    unique,
                                    "actual Predefined14048 useUniqueObjectName cell",
                                )? {
                                    num_rows = increment(
                                        num_rows,
                                        "actual Predefined14049 curNumRows int increment",
                                    )?;
                                }
                            }
                            if num_rows == 0 {
                                num_rows = 1;
                            }
                            let mut row_head = vec![Vec::new(); num_rows as usize];
                            let mut column_head = vec![Vec::new(); num_columns as usize];
                            let mut column_width = vec![14; num_columns as usize];
                            let mut body = SourceTabularBody::allocate(
                                num_columns as usize,
                                num_rows as usize,
                            )?;
                            // Three actual array assignments at14060-14062.
                            row_head.fill(Vec::new());
                            column_head.fill(Vec::new());
                            body.fill(b"");
                            let mut row_to_unique = vec![None; num_rows as usize];
                            let mut column_to_tag = vec![None; num_columns as usize];
                            let mut col_unit_conv = vec![None; num_columns as usize];
                            let mut count_row = 0i32;
                            row_head[0] = b"None".to_vec();
                            for unique in 1..=num_unique {
                                if written(
                                    &use_unique_object_name,
                                    unique,
                                    "actual Predefined14072 useUniqueObjectName cell",
                                )? {
                                    count_row = increment(
                                        count_row,
                                        "actual Predefined14073 countRow int increment",
                                    )?;
                                    row_head[ordinal(count_row)?] =
                                        unique_object_name[ordinal(unique)?].as_bytes().to_vec();
                                    row_to_unique[ordinal(count_row)?] = Some(unique);
                                }
                            }
                            let mut count_column = 0i32;
                            for column in 1..=o.registry.column_count {
                                if row(
                                    &o.registry.columns,
                                    column,
                                    "actual Predefined14081 column association",
                                )?
                                .subtable_index
                                    == subtable_number
                                {
                                    count_column = increment(
                                        count_column,
                                        "actual Predefined14082 countColumn int increment",
                                    )?;
                                    let heading = &row(
                                        &o.registry.columns,
                                        column,
                                        "actual Predefined14084 column heading",
                                    )?
                                    .heading;
                                    match self.input_state.tabular_report_passes[pass].units_style {
                                        4 => {
                                            return Err(Error::Unavailable(
                                                "actual Predefined14088 LookupSItoIP heading operation",
                                            ));
                                        }
                                        5 => {
                                            return Err(Error::Unavailable(
                                                "actual Predefined14091 LookupSItoIP heading operation",
                                            ));
                                        }
                                        1 => {
                                            return Err(Error::Unavailable(
                                                "actual Predefined14094 LookupJtokWH heading operation",
                                            ));
                                        }
                                        _ => {
                                            // Native curColTag copy precedes colUnitConv0, then columnHead.
                                            let current_tag = heading.as_bytes().to_vec();
                                            col_unit_conv[ordinal(count_column)?] = Some(0i32);
                                            column_head[ordinal(count_column)?] = current_tag;
                                        }
                                    }
                                    column_to_tag[ordinal(count_column)?] = Some(column);
                                }
                            }
                            for number in 1..=o.table.num_table_entry {
                                if entry(o.table, number)?.sub_table_index == subtable_number {
                                    let current_tag = entry(o.table, number)?.index_column;
                                    for column in 1..=num_columns {
                                        if current_tag
                                            == written(
                                                &column_to_tag,
                                                column,
                                                "actual Predefined14111 colHeadToColTag cell",
                                            )?
                                        {
                                            col_current = column;
                                            break;
                                        }
                                    }
                                    let current_unique = entry(o.table, number)?.unique_object_name;
                                    for current_row in 1..=num_rows {
                                        if current_unique
                                            == written(
                                                &row_to_unique,
                                                current_row,
                                                "actual Predefined14119 rowToUnqObjName cell",
                                            )?
                                        {
                                            row_current = current_row;
                                            break;
                                        }
                                    }
                                    if matches!(
                                        self.input_state.tabular_report_passes[pass].units_style,
                                        4 | 5 | 1
                                    ) || !self.input_state.tabular_report_passes[pass]
                                        .format_reals
                                    {
                                        let unit_conv = written(
                                            &col_unit_conv,
                                            col_current,
                                            "actual Predefined14128 colUnitConv cell",
                                        )?;
                                        if row(
                                            &o.registry.subtables,
                                            subtable_number,
                                            "actual Predefined14129 SizingPeriod name",
                                        )?
                                        .name
                                        .eq_ignore_ascii_case("SizingPeriod:DesignDay")
                                            && matches!(
                                                self.input_state.tabular_report_passes[pass]
                                                    .units_style,
                                                4 | 5
                                            )
                                        {
                                            if column_head[ordinal(col_current)?]
                                                .eq_ignore_ascii_case(b"Humidity Value")
                                            {
                                                return Err(Error::Unavailable(
                                                    "actual Predefined14134 LookupSItoIP following humidity-type entry",
                                                ));
                                            }
                                        }
                                        if entry(o.table, number)?.original_entry_is_real
                                            && (unit_conv != 0
                                                || !self.input_state.tabular_report_passes[pass]
                                                    .format_reals)
                                        {
                                            let value = entry(o.table, number)?.original_real_entry;
                                            if unit_conv != 0 {
                                                return Err(Error::Unavailable(
                                                    "actual Predefined14146 ConvertIP original real entry",
                                                ));
                                            }
                                            if self.input_state.tabular_report_passes[pass]
                                                .format_reals
                                            {
                                                body.set(
                                                    col_current as usize,
                                                    row_current as usize,
                                                    source_report_real(
                                                        self.input_state.tabular_report_passes
                                                            [pass]
                                                            .format_reals,
                                                        value,
                                                        entry(o.table, number)?.significant_digits,
                                                    )?,
                                                )?;
                                            } else {
                                                return Err(Error::Unavailable(
                                                    "actual Predefined14154 unrounded EnergyPlus::format real operation",
                                                ));
                                            }
                                        } else {
                                            body.set(
                                                col_current as usize,
                                                row_current as usize,
                                                entry(o.table, number)?.char_entry.clone(),
                                            )?;
                                        }
                                    } else {
                                        body.set(
                                            col_current as usize,
                                            row_current as usize,
                                            entry(o.table, number)?.char_entry.clone(),
                                        )?;
                                    }
                                }
                            }
                            if self.input_state.tabular_report_passes[pass].produce_tabular {
                                self.write_source_subtitle(
                                    row(
                                        &o.registry.subtables,
                                        subtable_number,
                                        "actual Predefined14166 subTable name",
                                    )?
                                    .name
                                    .as_bytes(),
                                )?;
                                self.write_source_table(
                                    &body,
                                    &row_head,
                                    &column_head,
                                    &mut column_width,
                                    false,
                                    row(
                                        &o.registry.subtables,
                                        subtable_number,
                                        "actual Predefined14168 subTable footnote",
                                    )?
                                    .footnote
                                    .as_bytes(),
                                    &mut o.writer,
                                )?;
                            }
                            if self.input_state.tabular_report_passes[pass].produce_sqlite {
                                if o.sqlite.present() {
                                    return Err(Error::Unavailable(
                                        "actual Predefined14172 createSQLiteTabularDataRecords",
                                    ));
                                }
                            }
                            if self.input_state.tabular_report_passes[pass].produce_json {
                                if o.results.time_series_and_tabular_enabled {
                                    return Err(Error::Unavailable(
                                        "actual Predefined14183 TabularReportsCollection.addReportTable",
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
