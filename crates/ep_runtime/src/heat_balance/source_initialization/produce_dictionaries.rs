//! Actual ProduceRDDMDD first call uses General's sole retained report cache.
use super::{SourceInitializationError, SourceInitializationFiles};
use crate::RuntimeError;
use crate::heat_balance::ctf_init_postlude::CtfInitPostludeError;
use crate::heat_balance::ctf_initialization_context::{
    CtfInitializationError, CtfInitializationReports,
};
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::report_scan::{
    ReportScanDiagnostic, ReportScanFailure, SourceReportScanInput, try_scan_for_reports,
    try_scan_for_reports_with_input,
};
use ep_model::ReportInputs;

impl CtfInitializationReports {
    /// Actual SimulationManager338 OutputProcessor child on the shared dictionary,
    /// meter, report-cache, file and ResultsFramework owners. No input is replayed
    /// after General has already scanned its cache.
    pub(crate) fn produce_source_rdd_mdd(
        &mut self,
        input: &ReportInputs,
        files: &mut SourceInitializationFiles,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), RuntimeError> {
        let mut option1 = String::new();
        let mut option2 = String::new();
        let mut sort_by_name = false;
        let mut diagnostic = |message| match message {
            ReportScanDiagnostic::Warning(text) => backend.show_warning_error(&text),
            ReportScanDiagnostic::Continue(text) => backend.show_continue_error(&text),
        };
        let result = if self.scan.get_report_input {
            let processor = self.selected_input_processor.as_mut().ok_or(
                SourceInitializationError::Association(
                    "actual ProduceRDDMDD shared InputProcessor",
                ),
            )?;
            let mut source =
                SourceReportScanInput::new(processor, &mut self.input_processor_shortcut, input)
                    .map_err(|value| {
                        CtfInitializationError::Postlude(CtfInitPostludeError::Scan(value))
                    })?;
            try_scan_for_reports_with_input(
                input,
                &mut self.scan,
                "VariableDictionary",
                None,
                Some(&mut option1),
                Some(&mut option2),
                &mut source,
                &mut diagnostic,
            )
        } else {
            try_scan_for_reports(
                input,
                &mut self.scan,
                "VariableDictionary",
                None,
                Some(&mut option1),
                Some(&mut option2),
                &mut diagnostic,
            )
        };
        let do_report = result.map_err(|error| {
            CtfInitializationError::Postlude(match error {
                ReportScanFailure::Unavailable(value) => CtfInitPostludeError::Scan(value),
                ReportScanFailure::Diagnostic(value) => CtfInitPostludeError::Backend(value),
            })
        })?;
        self.output_registration.produce_source_dictionaries(
            do_report,
            &option1,
            &option2,
            &mut sort_by_name,
            &self.source_output_control,
            &mut self.source_results_framework,
            files,
            backend,
        )?;
        Ok(())
    }
}
