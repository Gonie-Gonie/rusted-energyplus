//! Native finishCoilSummaryReportTable126-131 and its three same-vector children.
use super::SelectedCoilSelectionReport;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;

impl SelectedCoilSelectionReport {
    pub(crate) fn finish_source_coil_summary_report_table(&mut self) -> Result<(), Error> {
        self.final_source_coil_data_processing()?;
        self.write_source_coil_selection_output()?;
        self.write_source_coil_selection_output2()
    }
    fn final_source_coil_data_processing(&mut self) -> Result<(), Error> {
        // Preserve empty-range behavior, checked first-row access and the first unsupported positive-child failure after prior writes.
        #[allow(clippy::never_loop)]
        for _coil in &mut self.coil_selection_data {
            return Err(Error::Unavailable(
                "actual doFinalProcessingOfCoilData639 CoilSelectionData.zoneEqNum read",
            ));
        }
        Ok(())
    }
    fn write_source_coil_selection_output(&mut self) -> Result<(), Error> {
        // Preserve empty-range behavior, checked first-row access and the first unsupported positive-child failure after prior writes.
        #[allow(clippy::never_loop)]
        for _coil in &mut self.coil_selection_data {
            return Err(Error::Unavailable(
                "actual writeCoilSelectionOutput139 first CoilSelectionData/PreDefTableEntry operation",
            ));
        }
        Ok(())
    }
    fn write_source_coil_selection_output2(&mut self) -> Result<(), Error> {
        // Preserve empty-range behavior, checked first-row access and the first unsupported positive-child failure after prior writes.
        #[allow(clippy::never_loop)]
        for _coil in &mut self.coil_selection_data {
            return Err(Error::Unavailable(
                "actual writeCoilSelectionOutput2437 first CoilSelectionData/PreDefTableEntry operation",
            ));
        }
        Ok(())
    }
}
