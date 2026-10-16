//! Exact same-model raw operands for positive Table:Style and Summary getters.
use super::*;
use crate::heat_balance::selected_input_processor::{
    SelectedInputProcessor, SourceInputProcessorShortCut,
};
use crate::heat_balance::selected_project_data::{ProjectInputObject, ProjectInputValue};
use crate::heat_balance::selected_sizing::SelectedSizingInput;
/// Actual input format and ordered Style scalar fields; no report flags/defaults.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SelectedTabularInput {
    /// Same real input-loader format as the caller's permanent Source IP.
    pub source_is_epjson: bool,
    /// Actual getObjectItem order, exact case-sensitive object keys and field presence.
    pub table_style_objects: Vec<ProjectInputObject>,
}
impl SelectedTabularInput {
    pub(super) fn style_values(
        &self,
        input: &mut SelectedInputProcessor,
        shortcut: &mut SourceInputProcessorShortCut,
    ) -> Result<([String; 3], usize), Error> {
        let family = "OutputControl:Table:Style";
        let object = self
            .table_style_objects
            .first()
            .ok_or(Error::Unavailable("same actual Table:Style raw object"))?;
        if !input.has_object(family, &object.object_name) {
            return Err(Error::Unavailable(
                "same actual Table:Style raw/IP identity",
            ));
        }
        let fields = [
            ("column_separator", "Column Separator", "COMMA"),
            ("unit_conversion", "Unit Conversion", "NONE"),
            ("format_numeric_values", "Format Numeric Values", "YES"),
        ];
        let max_fields = if self.source_is_epjson {
            fields.iter().enumerate().fold(0, |n, (i, (json, _, _))| {
                if object.fields.contains_key(*json) {
                    n.max(i + 1)
                } else {
                    n
                }
            })
        } else {
            object
                .idf_max_fields
                .ok_or(Error::Unavailable("real staged Table:Style idf_max_fields"))?
        };
        if shortcut.alpha_field_blanks.len() < 3 || shortcut.alpha_field_names.len() < 3 {
            return Err(Error::Unavailable(
                "same actual Table:Style shortcut allocation",
            ));
        }
        let mut values = std::array::from_fn(|_| String::new());
        shortcut.numeric_field_blanks.fill(true);
        shortcut.alpha_field_blanks.fill(true);
        shortcut.alpha_field_names.fill(String::new());
        shortcut.numeric_field_names.fill(String::new());
        input.mark_object_as_used(family, &object.object_name);
        let mut count = 0;
        for (i, (json, legacy, default)) in fields.into_iter().enumerate() {
            let (value, blank) = match object.fields.get(json) {
                None => (default, true),
                Some(ProjectInputValue::String(v)) if v.is_empty() => (default, true),
                Some(ProjectInputValue::String(v)) => (v.as_str(), false),
                _ => return Err(Error::Unavailable("actual Table:Style alpha JSON value")),
            };
            values[i] = value.to_ascii_uppercase();
            shortcut.alpha_field_blanks[i] = blank;
            shortcut.alpha_field_names[i] =
                if self.source_is_epjson { json } else { legacy }.into();
            if i < max_fields {
                count = i + 1;
            }
        }
        Ok((values, count))
    }
    pub(super) fn summary_values(
        &self,
        raw: &SelectedSizingInput,
        input: &mut SelectedInputProcessor,
        maximum: usize,
    ) -> Result<Vec<String>, Error> {
        let family = "Output:Table:SummaryReports";
        let object = raw
            .summary_report_objects
            .first()
            .ok_or(Error::Unavailable(
                "same existing actual SummaryReports raw owner",
            ))?;
        if !input.has_object(family, &object.object_name) {
            return Err(Error::Unavailable(
                "same actual SummaryReports raw/IP identity",
            ));
        }
        let count = if self.source_is_epjson {
            object.reports.len()
        } else {
            object.idf_max_extensible_fields.ok_or(Error::Unavailable(
                "real staged SummaryReports idf_max_extensible_fields",
            ))?
        };
        if !self.source_is_epjson && object.idf_max_fields.is_none() {
            return Err(Error::Unavailable(
                "real staged SummaryReports idf_max_fields",
            ));
        }
        if count > maximum || count != object.reports.len() {
            return Err(Error::Unavailable(
                "actual SummaryReports extension/count/getter array domain",
            ));
        }
        // This Native call supplies local alpha/number arrays only: do not reset
        // the unrelated shared shortcut blanks/names/currentModuleObject here.
        input.mark_object_as_used(family, &object.object_name);
        Ok(object
            .reports
            .iter()
            .map(|v| v.to_ascii_uppercase())
            .collect())
    }
}
