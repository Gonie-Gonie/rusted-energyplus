//! Actual WriteReportHeaders17305, Subtitle17362, TextLine17405 and Table17452.
//! All strings are Native char bytes and all output borrows the sole five files.
mod bytes;
mod table;
use super::output_files::{SourceTabularOutputStream, escape};
use super::{Error, SourceTabularReports};
use crate::heat_balance::error_backend::{SourceDiagnosticFlags, SourceErrorBackend};
use crate::heat_balance::ordinary_utility_bills::SourceCostEstimateMonetary;
use crate::heat_balance::source_output_registration::SourceOutputStoreType;
use bytes::{anchor, element};
pub(crate) use table::SourceTabularBody;

/// Constructed only around one actual report call; no phase flag is copied.
pub(crate) struct SourceTabularWriterOwners<'a, 'backend> {
    pub(crate) monetary: &'a SourceCostEstimateMonetary,
    pub(crate) flags: SourceDiagnosticFlags<'a>,
    pub(crate) backend: &'a mut SourceErrorBackend<'backend>,
}
fn style_count(value: i32) -> Result<usize, Error> {
    let count =
        usize::try_from(value).map_err(|_| Error::Unavailable("actual numStyles array bound"))?;
    if count > 5 {
        return Err(Error::Unavailable(
            "actual numStyles exceeds five-member array",
        ));
    }
    Ok(count)
}
fn td(values: &[Option<i32>; 8], index: usize) -> Result<i32, Error> {
    values[index].ok_or(Error::Unavailable(
        "actual date_and_time-written TAB td cell",
    ))
}
fn header_timestamp(
    stream: &mut SourceTabularOutputStream,
    values: &[Option<i32>; 8],
) -> Result<(), Error> {
    stream.integer(td(values, 0)?, 4)?;
    stream.bytes(b"-")?;
    stream.fill = b'0';
    stream.integer(td(values, 1)?, 2)?;
    stream.bytes(b"-")?;
    stream.integer(td(values, 2)?, 2)?;
    stream.bytes(b"\n    ")?;
    stream.integer(td(values, 4)?, 2)?;
    stream.bytes(b":")?;
    stream.integer(td(values, 5)?, 2)?;
    stream.bytes(b":")?;
    stream.integer(td(values, 6)?, 2)?;
    stream.fill = b' ';
    stream.bytes(b"</b></p>\n")
}
impl SourceTabularReports {
    pub(crate) fn write_source_report_headers(
        &mut self,
        report: &[u8],
        object: &[u8],
        store: SourceOutputStoreType,
    ) -> Result<(), Error> {
        let mut modified = report.to_vec();
        if store == SourceOutputStoreType::Sum {
            modified.extend_from_slice(b" per second");
        }
        let s = &mut self.input_state;
        for i in 0..style_count(s.number_of_styles)? {
            let stream = &mut self.output_streams.streams[i];
            let del = s.delimiters[i].as_bytes();
            match s.table_styles[i] {
                0 | 1 | 2 => {
                    stream.bytes(b"----------------------------------------------------------------------------------------------------\n")?;
                    stream.bytes(if s.table_styles[i] == 2 {
                        b"REPORT:      "
                    } else {
                        b"REPORT:"
                    })?;
                    stream.bytes(del)?;
                    stream.bytes(&modified)?;
                    stream.bytes(b"\n")?;
                    stream.bytes(if s.table_styles[i] == 2 {
                        b"FOR:         "
                    } else {
                        b"FOR:"
                    })?;
                    stream.bytes(del)?;
                    stream.bytes(object)?;
                    stream.bytes(b"\n")?;
                }
                3 => {
                    stream.bytes(b"<hr>\n<p><a href=\"#toc\" style=\"float: right\">Table of Contents</a></p>\n<a name=")?;
                    stream.bytes(&anchor(report, object))?;
                    stream.bytes(b"></a>\n<p>Report:<b>")?;
                    stream.bytes(del)?;
                    stream.bytes(&modified)?;
                    stream.bytes(b"</b></p>\n<p>For:<b>")?;
                    stream.bytes(del)?;
                    stream.bytes(object)?;
                    stream.bytes(b"</b></p>\n<p>Timestamp: <b>")?;
                    header_timestamp(stream, &s.td)?;
                }
                4 => {
                    if !s.prev_report_name.is_empty() {
                        stream.bytes(b"</")?;
                        stream.bytes(&s.prev_report_name)?;
                        stream.bytes(b">\n")?;
                    }
                    stream.bytes(b"<")?;
                    stream.bytes(&element(&modified))?;
                    stream.bytes(b">\n  <for>")?;
                    stream.bytes(&escape(object, true)?)?;
                    stream.bytes(b"</for>\n")?;
                    s.prev_report_name = element(&modified);
                }
                _ => {}
            }
        }
        s.active_sub_table_name.clear();
        s.active_report_name = modified;
        s.active_for_name = object.to_vec();
        Ok(())
    }
    pub(crate) fn write_source_subtitle(&mut self, subtitle: &[u8]) -> Result<(), Error> {
        let s = &mut self.input_state;
        for i in 0..style_count(s.number_of_styles)? {
            match s.table_styles[i] {
                0 | 1 | 2 => {
                    let stream = &mut self.output_streams.streams[i];
                    stream.bytes(subtitle)?;
                    stream.bytes(b"\n\n")?;
                }
                3 => {
                    let stream = &mut self.output_streams.streams[i];
                    stream.bytes(b"<b>")?;
                    stream.bytes(subtitle)?;
                    stream.bytes(b"</b><br><br>\n<!-- FullName:")?;
                    stream.bytes(&s.active_report_name)?;
                    stream.bytes(b"_")?;
                    stream.bytes(&s.active_for_name)?;
                    stream.bytes(b"_")?;
                    stream.bytes(subtitle)?;
                    stream.bytes(b"-->\n")?;
                }
                4 => s.active_sub_table_name = subtitle.to_vec(),
                _ => {}
            }
        }
        Ok(())
    }
    pub(crate) fn write_source_text_line(&mut self, line: &[u8], bold: bool) -> Result<(), Error> {
        let s = &self.input_state;
        for i in 0..style_count(s.number_of_styles)? {
            let stream = &mut self.output_streams.streams[i];
            match s.table_styles[i] {
                0 | 1 | 2 => {
                    stream.bytes(line)?;
                    stream.bytes(b"\n")?;
                }
                3 => {
                    if bold {
                        stream.bytes(b"<b>")?;
                    }
                    stream.bytes(line)?;
                    stream.bytes(if bold { b"</b><br><br>\n" } else { b"<br>\n" })?;
                }
                4 => {
                    if !line.is_empty() {
                        stream.bytes(b"<note>")?;
                        stream.bytes(line)?;
                        stream.bytes(b"</note>\n")?;
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}

// Same accepted MakeAnchorName17818-17867 byte kernel, shared by TOC.
pub(super) fn source_anchor(report: &[u8], object: &[u8]) -> Vec<u8> {
    anchor(report, object)
}

// Native stripped byte-string helper; no UTF8 re-encoding or numeric copy.
pub(super) fn source_stripped_report_text(text: &[u8]) -> &[u8] { bytes::strip(text) }

// Actual FillRowHead18171-18183 shared with BEPS/Demand temporary text rows.
pub(super) fn source_fill_row_head(rows: &mut [Vec<u8>]) {
    let mut current = Vec::new();
    for row in rows {
        if row.is_empty() { *row = current.clone(); } else { current = row.clone(); }
    }
}
