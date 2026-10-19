//! Original WriteTable17452–17816; five styles share the actual files and state.
use super::super::output_files::{SourceTabularOutputStream, escape};
use super::bytes::{currency, element, find, right_sized, same, strip, units};
use super::{Error, SourceTabularReports, SourceTabularWriterOwners, style_count};

/// Local Array2D_string body: Native first argument is column, second is row.
/// String allocation default-constructs empty bytes; this is not a numeric seed.
pub(crate) struct SourceTabularBody {
    columns: usize,
    rows: usize,
    values: Vec<Vec<u8>>,
}
impl SourceTabularBody {
    // Actual local Array2D_string whole-value assignment, same existing cells.
    pub(crate) fn fill(&mut self, value: &[u8]) {
        for cell in &mut self.values {
            cell.clear();
            cell.extend_from_slice(value);
        }
    }
    pub(crate) fn allocate(columns: usize, rows: usize) -> Result<Self, Error> {
        let size = columns.checked_mul(rows).ok_or(Error::Unavailable(
            "actual local tabular string allocation extent",
        ))?;
        Ok(Self {
            columns,
            rows,
            values: vec![Vec::new(); size],
        })
    }
    pub(crate) fn at(&self, col: usize, row: usize) -> Result<&[u8], Error> {
        if col == 0 || row == 0 || col > self.columns || row > self.rows {
            return Err(Error::Unavailable("actual local tabular string ordinal"));
        }
        Ok(&self.values[(col - 1) * self.rows + row - 1])
    }
    pub(crate) fn set(&mut self, col: usize, row: usize, value: Vec<u8>) -> Result<(), Error> {
        if col == 0 || row == 0 || col > self.columns || row > self.rows {
            return Err(Error::Unavailable("actual local tabular string ordinal"));
        }
        self.values[(col - 1) * self.rows + row - 1] = value;
        Ok(())
    }
}
fn as_int(value: usize) -> Result<i32, Error> {
    i32::try_from(value)
        .map_err(|_| Error::Unavailable("actual WriteTable isize/int string-length domain"))
}
fn width(value: i32) -> Result<usize, Error> {
    // Native casts the int to std::string::size_type. Negative sizes request
    // an unrepresentable allocation; do not clamp them into a successful table.
    usize::try_from(value)
        .map_err(|_| Error::Unavailable("actual WriteTable negative string-size domain"))
}
fn cell(
    stream: &mut SourceTabularOutputStream,
    value: &[u8],
    o: &SourceTabularWriterOwners<'_, '_>,
) -> Result<(), Error> {
    stream.bytes(b"    <td align=\"right\">")?;
    if !value.is_empty() {
        stream.bytes(&escape(&currency(value, true, o.monetary)?, false)?)?;
    } else {
        stream.bytes(b"&nbsp;")?;
    }
    stream.bytes(b"</td>\n")
}
fn tag_with_attribute(tag: &[u8], unit: &[u8], transpose: bool) -> Vec<u8> {
    let mut out = b"<".to_vec();
    out.extend_from_slice(tag);
    if !unit.is_empty() {
        out.extend_from_slice(b" units=\"");
        out.extend_from_slice(unit);
        out.extend_from_slice(if transpose { b"\">\n" } else { b"\">" });
    } else {
        out.push(b'>');
    }
    out
}
impl SourceTabularReports {
    pub(crate) fn write_source_table(
        &mut self,
        body: &SourceTabularBody,
        row_labels: &[Vec<u8>],
        column_labels: &[Vec<u8>],
        widths: &mut [i32],
        mut transpose_xml: bool,
        footnote: &[u8],
        o: &mut SourceTabularWriterOwners<'_, '_>,
    ) -> Result<(), Error> {
        let mut rows_body = as_int(body.rows)?;
        let mut cols_body = as_int(body.columns)?;
        let mut rows_row_labels = as_int(row_labels.len())?;
        let mut cols_column_labels = as_int(column_labels.len())?;
        let cols_width = as_int(widths.len())?;
        if rows_body != rows_row_labels {
            o.backend
                .show_source_warning_error("REPORT:TABLE Inconsistent number of rows.", &o.flags)?;
            rows_body = rows_body.min(rows_row_labels);
            rows_row_labels = rows_body;
        }
        if cols_body != cols_column_labels || cols_body != cols_width {
            o.backend.show_source_warning_error(
                "REPORT:TABLE Inconsistent number of columns.",
                &o.flags,
            )?;
            cols_body = cols_body.min(cols_column_labels.min(cols_width));
            cols_column_labels = cols_body;
        }
        let rows = rows_body as usize;
        let cols = cols_body as usize;
        let mut row_tags = vec![Vec::new(); rows];
        let mut col_tags = vec![Vec::new(); cols];
        let mut row_units = vec![Vec::new(); rows];
        let mut col_units = vec![Vec::new(); cols];
        let mut body_esc = SourceTabularBody::allocate(cols, rows)?;
        let mut labels = vec![vec![Vec::new(); 50]; cols_column_labels as usize];
        let mut max_label_rows = 0;
        let s = &mut self.input_state;
        for i in 0..style_count(s.number_of_styles)? {
            let stream = &mut self.output_streams.streams[i];
            let del = s.delimiters[i].as_bytes();
            for col in 0..cols_column_labels as usize {
                let mut n = 0;
                let mut work = column_labels[col].as_slice();
                widths[col] = widths[col].max(as_int(column_labels[col].len())?);
                loop {
                    let bar = work.iter().position(|&c| c == b'|');
                    n += 1;
                    let destination = labels[col].get_mut(n - 1).ok_or(Error::Unavailable(
                        "actual WriteTable colLabelMulti row exceeds 50",
                    ))?;
                    if let Some(bar) = bar {
                        *destination = work[..bar].to_vec();
                        work = &work[bar + 1..];
                    } else {
                        *destination = work.to_vec();
                        break;
                    }
                }
                if n > max_label_rows {
                    max_label_rows = n;
                }
            }
            match s.table_styles[i] {
                0 | 1 => {
                    for row in 0..max_label_rows {
                        let mut line = del.to_vec();
                        for label in labels.iter().take(cols_column_labels as usize) {
                            line.extend_from_slice(del);
                            line.extend_from_slice(strip(&label[row]));
                        }
                        stream.bytes(&currency(&line, false, o.monetary)?)?;
                        stream.bytes(b"\n")?;
                    }
                    // Preserve Original index order and same-row checked accesses across the retained work arrays.
                    #[allow(clippy::needless_range_loop)]
                    for row in 0..rows {
                        let mut line = del.to_vec();
                        line.extend_from_slice(&row_labels[row]);
                        for col in 0..cols {
                            line.extend_from_slice(del);
                            line.extend_from_slice(strip(body.at(col + 1, row + 1)?));
                        }
                        stream.bytes(&currency(&line, false, o.monetary)?)?;
                        stream.bytes(b"\n")?;
                    }
                    if !footnote.is_empty() {
                        stream.bytes(footnote)?;
                        stream.bytes(b"\n")?;
                    }
                    stream.bytes(b"\n\n")?;
                }
                2 => {
                    for col in 0..cols_column_labels as usize {
                        // Preserve Original index order and same-row checked accesses across the retained work arrays.
                        #[allow(clippy::needless_range_loop)]
                        for row in 0..max_label_rows {
                            labels[col][row].truncate(width(widths[col])?);
                        }
                    }
                    let mut max_row_width = 0;
                    for label in row_labels.iter().take(rows_row_labels as usize) {
                        if label.len() > max_row_width {
                            max_row_width = label.len();
                        }
                    }
                    // Preserve Original index order and same-row checked accesses across the retained work arrays.
                    #[allow(clippy::needless_range_loop)]
                    for row in 0..max_label_rows {
                        let mut line = Vec::new();
                        let col1start = (max_row_width + 2).max(3);
                        for col in 0..cols_column_labels as usize {
                            if col == 0 {
                                line = vec![b' '; col1start - 1];
                            }
                            line.extend_from_slice(b"  ");
                            line.extend_from_slice(&right_sized(
                                &labels[col][row],
                                width(widths[col])?,
                            ));
                        }
                        stream.bytes(&currency(&line, false, o.monetary)?)?;
                        stream.bytes(b"\n")?;
                    }
                    // Preserve Original index order and same-row checked accesses across the retained work arrays.
                    #[allow(clippy::needless_range_loop)]
                    for row in 0..rows {
                        let mut line = b"  ".to_vec();
                        line.extend_from_slice(&right_sized(&row_labels[row], max_row_width));
                        // Preserve Original index order and same-row checked accesses across the retained work arrays.
                        #[allow(clippy::needless_range_loop)]
                        for col in 0..cols {
                            line.extend_from_slice(if col == 0 { b"   " } else { b"  " });
                            line.extend_from_slice(&right_sized(
                                body.at(col + 1, row + 1)?,
                                width(widths[col])?,
                            ));
                        }
                        stream.bytes(&currency(&line, false, o.monetary)?)?;
                        stream.bytes(b"\n")?;
                    }
                    if !footnote.is_empty() {
                        stream.bytes(footnote)?;
                        stream.bytes(b"\n")?;
                    }
                    stream.bytes(b"\n\n")?;
                }
                3 => {
                    stream.bytes(b"<table border=\"1\" cellpadding=\"4\" cellspacing=\"0\">\n  <tr><td></td>\n")?;
                    for label in labels.iter().take(cols_column_labels as usize) {
                        let mut line = b"    <td align=\"right\">".to_vec();
                        // Preserve Original index order and same-row checked accesses across the retained work arrays.
                        #[allow(clippy::needless_range_loop)]
                        for row in 0..max_label_rows {
                            line.extend_from_slice(&escape(&label[row], false)?);
                            if row + 1 < max_label_rows {
                                line.extend_from_slice(b"<br>");
                            }
                        }
                        stream.bytes(&currency(&line, true, o.monetary)?)?;
                        stream.bytes(b"</td>\n")?;
                    }
                    stream.bytes(b"  </tr>\n")?;
                    // Preserve Original index order and same-row checked accesses across the retained work arrays.
                    #[allow(clippy::needless_range_loop)]
                    for row in 0..rows {
                        stream.bytes(b"  <tr>\n")?;
                        cell(stream, &row_labels[row], o)?;
                        for col in 0..cols {
                            cell(stream, body.at(col + 1, row + 1)?, o)?;
                        }
                        stream.bytes(b"  </tr>\n")?;
                    }
                    stream.bytes(b"</table>\n")?;
                    if !footnote.is_empty() {
                        stream.bytes(b"<div class=\"footnote\" style=\"font-style: italic;\">")?;
                        stream.bytes(footnote)?;
                        stream.bytes(b"</div>\n")?;
                    }
                    stream.bytes(b"<br><br>\n")?;
                }
                4 => {
                    let mut table_blank = true;
                    for row in 1..=rows {
                        for col in 1..=cols {
                            if !body.at(col, row)?.is_empty() {
                                table_blank = false;
                                break;
                            }
                        }
                        if !table_blank {
                            break;
                        }
                    }
                    if !table_blank {
                        s.active_sub_table_name = element(&s.active_sub_table_name);
                        s.active_report_name_no_space = element(&s.active_report_name);
                        if same(&s.active_sub_table_name, &s.active_report_name_no_space) {
                            s.active_sub_table_name.extend_from_slice(b"Record");
                        }
                        if s.active_sub_table_name.is_empty() {
                            s.active_sub_table_name = s.active_report_name_no_space.clone();
                            s.active_sub_table_name.extend_from_slice(b"Record");
                        }
                        if cols == 1 && rows > 1 {
                            transpose_xml = true;
                        }
                        for row in 0..rows {
                            row_tags[row] = element(&row_labels[row]);
                            if row_tags[row].is_empty() {
                                row_tags[row] = b"none".to_vec();
                            }
                            row_units[row] = units(&row_labels[row]);
                            if same(&row_units[row], b"Invalid/Undefined") {
                                row_units[row].clear();
                            }
                        }
                        for col in 0..cols {
                            col_tags[col] = element(&column_labels[col]);
                            if col_tags[col].is_empty() {
                                col_tags[col] = b"none".to_vec();
                            }
                            col_units[col] = units(&column_labels[col]);
                            if same(&col_units[col], b"Invalid/Undefined") {
                                col_units[col].clear();
                            }
                        }
                        for row in 1..=rows {
                            for col in 1..=cols {
                                body_esc.set(col, row, escape(body.at(col, row)?, true)?)?;
                            }
                        }
                        if !transpose_xml {
                            // Preserve Original index order and same-row checked accesses across the retained work arrays.
                            #[allow(clippy::needless_range_loop)]
                            for row in 0..rows {
                                let mut blank = true;
                                for col in 0..cols {
                                    if !body_esc.at(col + 1, row + 1)?.is_empty() {
                                        blank = false;
                                        break;
                                    }
                                }
                                if !blank {
                                    stream.bytes(b"  <")?;
                                    stream.bytes(&s.active_sub_table_name)?;
                                    stream.bytes(b">\n")?;
                                    if !row_tags[row].is_empty() {
                                        stream.bytes(b"    <name>")?;
                                        stream.bytes(&row_tags[row])?;
                                        stream.bytes(b"</name>\n")?;
                                    }
                                    for col in 0..cols {
                                        let value = strip(body_esc.at(col + 1, row + 1)?);
                                        if !value.is_empty() {
                                            let tag = tag_with_attribute(
                                                &col_tags[col],
                                                &col_units[col],
                                                false,
                                            );
                                            stream.bytes(b"    ")?;
                                            stream.bytes(&tag)?;
                                            stream.bytes(value)?;
                                            stream.bytes(b"</")?;
                                            stream.bytes(&col_tags[col])?;
                                            stream.bytes(b">\n")?;
                                        }
                                    }
                                    stream.bytes(b"  </")?;
                                    stream.bytes(&s.active_sub_table_name)?;
                                    stream.bytes(b">\n")?;
                                }
                            }
                        } else {
                            for col in 0..cols {
                                let mut blank = true;
                                for row in 0..rows {
                                    if !body_esc.at(col + 1, row + 1)?.is_empty() {
                                        blank = false;
                                        break;
                                    }
                                }
                                if !blank {
                                    stream.bytes(b"  <")?;
                                    stream.bytes(&s.active_sub_table_name)?;
                                    stream.bytes(b">\n")?;
                                    if !col_tags[col].is_empty() {
                                        if !col_units[col].is_empty() {
                                            stream.bytes(b"    <name units=\"")?;
                                            stream.bytes(&col_units[col])?;
                                            stream.bytes(b"\">")?;
                                        } else {
                                            stream.bytes(b"    <name>")?;
                                        }
                                        stream.bytes(&col_tags[col])?;
                                        stream.bytes(b"</name>\n")?;
                                    }
                                    for row in 0..rows {
                                        let value = body_esc.at(col + 1, row + 1)?;
                                        if !value.is_empty() {
                                            let tag = tag_with_attribute(
                                                &row_tags[row],
                                                &row_units[row],
                                                true,
                                            );
                                            stream.bytes(b"    ")?;
                                            stream.bytes(&tag)?;
                                            stream.bytes(strip(value))?;
                                            stream.bytes(b"</")?;
                                            stream.bytes(&row_tags[row])?;
                                            stream.bytes(b">\n")?;
                                        }
                                    }
                                    stream.bytes(b"  </")?;
                                    stream.bytes(&s.active_sub_table_name)?;
                                    stream.bytes(b">\n")?;
                                }
                            }
                        }
                        if !footnote.is_empty() {
                            if find(footnote, b"<br").is_some() {
                                stream.bytes(b"  <footnote><![CDATA[")?;
                                stream.bytes(footnote)?;
                                stream.bytes(b"]]></footnote>\n")?;
                            } else {
                                stream.bytes(b"  <footnote>")?;
                                stream.bytes(footnote)?;
                                stream.bytes(b"</footnote>\n")?;
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}
