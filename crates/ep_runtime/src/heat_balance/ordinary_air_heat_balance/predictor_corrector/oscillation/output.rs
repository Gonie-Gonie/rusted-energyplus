//! Six actual System/hr/Sum registrations, bound to the same detector cells.
use super::*;
use crate::heat_balance::source_output_prescan::SourceOutputPreScan;
use crate::heat_balance::source_output_registration::{
    SourceOutputDictionaryHost, SourceOutputFileHost, SourceOutputRegistration,
    SourceOutputStoreType, SourceOutputUnits, SourceRealOutputBinding, SourceRealOutputDeclaration,
    SourceRealRegistrationOptions, SourceReportVariableInput, TimeStepKind,
};
use ep_model::ReportInputs;
pub(crate) struct SourceOscillationOutputContext<'a, H, D> {
    pub(crate) registration: &'a mut SourceOutputRegistration,
    pub(crate) prescan: &'a SourceOutputPreScan,
    pub(crate) reports: &'a ReportInputs,
    pub(crate) minimum_report_frequency: &'a str,
    pub(crate) host: &'a mut H,
    pub(crate) dictionary_host: &'a mut D,
}
impl<H: SourceOutputFileHost, D: SourceOutputDictionaryHost>
    SourceOscillationOutputContext<'_, H, D>
{
    pub(super) fn declare(
        &mut self,
        detector: &SourceZoneTemperatureOscillation,
        field: SourceOscillationOutputField,
        number: i32,
        name: &str,
        key: &str,
        clock: &WeatherGlobalState,
        reporting_diagnostic_flags: Option<
            crate::heat_balance::error_backend::SourceDiagnosticFlags<'_>,
        >,
        input: &mut SelectedInputProcessor,
        backend: &mut SourceErrorBackend<'_>,
        eio: &mut dyn std::io::Write,
    ) -> Result<(), Error> {
        let binding = SourceRealOutputBinding::new(
            SourceRealOutputTarget::Oscillation {
                field,
                index: number,
            },
            detector,
        )
        .map_err(Error::Output)?;
        let mut input = SourceReportVariableInput::new(
            self.reports,
            input,
            crate::heat_balance::source_output_registration::SourceReportVariableEnvironment::new(
                self.minimum_report_frequency,
                reporting_diagnostic_flags,
            ),
            backend,
        );
        self.registration
            .setup_bound_real(
                &mut input,
                self.prescan,
                SourceRealOutputDeclaration {
                    name,
                    key,
                    units: SourceOutputUnits::Hr,
                    store_type: SourceOutputStoreType::Sum,
                },
                clock.minutes_in_time_step,
                self.host,
                eio,
                binding,
                self.dictionary_host,
                SourceRealRegistrationOptions::unmetered(TimeStepKind::System),
            )
            .map_err(Error::Output)
    }
}
