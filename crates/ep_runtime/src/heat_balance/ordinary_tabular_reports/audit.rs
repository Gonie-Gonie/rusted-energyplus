//! Actual unconditional WriteTabularReports5278-5294 audit suffix.
use super::{Error, SourceTabularReports};
use crate::heat_balance::ctf_reports::PredefinedStringTable;
use crate::heat_balance::ordinary_output_control::SourceOutputControl;
use crate::heat_balance::predefined_tables::PredefinedRegistry;
use crate::heat_balance::source_initialization::SourceInitializationFiles;

impl SourceTabularReports {
    /// This suffix is outside the WriteTabularFiles block; the actual caller
    /// must reach it even when that flag is false, after the weather reports.
    pub(crate) fn write_source_tabular_audit(
        &self,
        files: &mut SourceInitializationFiles,
        control: &SourceOutputControl,
        registry: &PredefinedRegistry,
        table: &PredefinedStringTable,
    ) -> Result<(), Error> {
        let audit = files.ensure_source_reporting_audit(control)?;
        // Read the sole retained cells in Native print order, no inferred
        // lengths, allocator capacities, defaults or cloned report snapshot.
        writeln!(
            audit,
            " MonthlyInputCount={:12}",
            self.input_state.monthly_input_count
        )?;
        writeln!(
            audit,
            " sizeMonthlyInput={:12}",
            self.input_state.size_monthly_input
        )?;
        writeln!(
            audit,
            " MonthlyFieldSetInputCount={:12}",
            self.input_state.monthly_field_set_input_count
        )?;
        writeln!(
            audit,
            " sizeMonthlyFieldSetInput={:12}",
            self.input_state.size_monthly_field_set_input
        )?;
        writeln!(
            audit,
            " MonthlyTablesCount={:12}",
            self.input_state.monthly_tables_count
        )?;
        writeln!(
            audit,
            " MonthlyColumnsCount={:12}",
            self.input_state.monthly_columns_count
        )?;
        writeln!(audit, " sizeReportName={:12}", registry.report_capacity)?;
        writeln!(audit, " numReportName={:12}", registry.report_count)?;
        writeln!(audit, " sizeSubTable={:12}", registry.subtable_capacity)?;
        writeln!(audit, " numSubTable={:12}", registry.subtable_count)?;
        writeln!(audit, " sizeColumnTag={:12}", registry.column_capacity)?;
        writeln!(audit, " numColumnTag={:12}", registry.column_count)?;
        writeln!(audit, " sizeTableEntry={:12}", table.size_table_entry)?;
        writeln!(audit, " numTableEntry={:12}", table.num_table_entry)?;
        writeln!(
            audit,
            " sizeCompSizeTableEntry={:12}",
            registry.component_sizing_capacity
        )?;
        writeln!(
            audit,
            " numCompSizeTableEntry={:12}",
            registry.component_sizing_count
        )?;
        Ok(())
    }

    pub(crate) fn source_max_unique_key_count(&self) -> &i32 {
        &self.input_state.max_unique_key_count
    }
    pub(crate) fn source_monthly_field_set_input_count(&self) -> &i32 {
        &self.input_state.monthly_field_set_input_count
    }
}
