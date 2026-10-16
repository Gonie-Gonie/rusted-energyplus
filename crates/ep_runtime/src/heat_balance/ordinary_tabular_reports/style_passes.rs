//! Literal setTabularReportStyles5297-5364 on the retained TAB and output owners.
use super::SourceTabularReports;
use crate::heat_balance::ordinary_output_control::SourceResultsFrameworkOptions;
use crate::heat_balance::source_initialization::SourceSqliteState;

/// Complete Native tabularReportStyle573-580 value constructor.
#[derive(Clone, Debug, PartialEq)]
pub(super) struct SourceTabularReportStyle {
    pub(super) units_style: i32,
    pub(super) format_reals: bool,
    pub(super) produce_tabular: bool,
    pub(super) produce_sqlite: bool,
    pub(super) produce_json: bool,
}
impl Default for SourceTabularReportStyle {
    fn default() -> Self {
        Self {
            units_style: 0,
            format_reals: true,
            produce_tabular: true,
            produce_sqlite: true,
            produce_json: true,
        }
    }
}
impl SourceTabularReports {
    pub(crate) fn set_source_tabular_report_styles(
        &mut self,
        results: &SourceResultsFrameworkOptions,
        sqlite: &SourceSqliteState,
    ) {
        let mut current_style = SourceTabularReportStyle::default();
        let state = &mut self.input_state;
        state.tabular_report_passes.clear();
        current_style.units_style = state.units_style_tabular;
        current_style.format_reals = state.format_reals_tabular;
        current_style.produce_tabular = true;
        current_style.produce_json = false;
        current_style.produce_sqlite = false;
        let mut need_more_for_json = false;
        let mut need_more_for_sqlite = false;
        if results.time_series_and_tabular_enabled {
            if state.format_reals_json == state.format_reals_tabular
                && state.units_style_json == state.units_style_tabular
            {
                current_style.produce_json = true;
                need_more_for_json = false;
            } else {
                need_more_for_json = true;
            }
        }
        if sqlite.present() {
            if state.format_reals_sqlite == state.format_reals_tabular
                && state.units_style_sqlite == state.units_style_tabular
            {
                current_style.produce_sqlite = true;
                need_more_for_sqlite = false;
            } else {
                need_more_for_sqlite = true;
            }
        }
        state.tabular_report_passes.push(current_style.clone());
        if need_more_for_json {
            current_style.units_style = state.units_style_json;
            current_style.format_reals = state.format_reals_json;
            current_style.produce_tabular = false;
            current_style.produce_json = true;
            current_style.produce_sqlite = false;
            if need_more_for_sqlite {
                if sqlite.present() {
                    if state.format_reals_sqlite == state.format_reals_json
                        && state.units_style_sqlite == state.units_style_json
                    {
                        current_style.produce_sqlite = true;
                        need_more_for_sqlite = false;
                    } else {
                        need_more_for_sqlite = true;
                    }
                }
            }
            state.tabular_report_passes.push(current_style.clone());
        }
        if need_more_for_sqlite {
            current_style.units_style = state.units_style_sqlite;
            current_style.format_reals = state.format_reals_sqlite;
            current_style.produce_tabular = false;
            current_style.produce_json = false;
            current_style.produce_sqlite = true;
            state.tabular_report_passes.push(current_style.clone());
        }
    }
}
