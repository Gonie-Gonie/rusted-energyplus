//! Four actual unconditional Facility System/Sum/hr declarations2161–2191.
use super::*;
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::radiation::ordinary::{
    OrdinaryRadiationError, OrdinaryRadiationUnavailable,
};
use crate::heat_balance::selected_input_processor::SelectedInputProcessor;
use crate::heat_balance::source_output_prescan::SourceOutputPreScan;
use crate::heat_balance::source_output_registration::{
    SourceGeneralRealRegistrationOptions, SourceOutputDictionaryHost, SourceOutputFileHost,
    SourceOutputRegistration, SourceOutputStoreType, SourceOutputUnits, SourceRealOutputBinding,
    SourceRealOutputDeclaration, SourceRealOutputOwners, SourceRealOutputTarget as Target,
    SourceReportVariableInput, SourceSystemReportsOutputField as Field, TimeStepKind,
};
use ep_model::ReportInputs;
/// Per-call streams/metadata; the sole registration/IP/backend are separate borrows.
pub(crate) struct SourceSystemReportsOutputContext<'a, H, D> {
    pub(crate) prescan: &'a SourceOutputPreScan,
    pub(crate) reports: &'a ReportInputs,
    pub(crate) minimum_report_frequency: &'a str,
    pub(crate) minutes_in_time_step: i32,
    pub(crate) host: &'a mut H,
    pub(crate) eio: &'a mut dyn std::io::Write,
    pub(crate) dictionary_host: &'a mut D,
}
impl SourceRealOutputOwners for SourceSystemReports {
    fn real_output(&self, target: Target) -> Result<&f64, OrdinaryRadiationError> {
        let Target::SystemReports { field } = target else {
            return Err(OrdinaryRadiationUnavailable::StorageDomain.into());
        };
        Ok(match field {
            Field::AnyZoneBelowTargetTime => &self.any_zone_time_below_voz_dyn,
            Field::AllZonesAtTargetTime => &self.all_zones_time_at_voz_dyn,
            Field::AnyZoneAboveTargetTime => &self.any_zone_time_above_voz_dyn,
            Field::AnyZoneUnoccupiedVentilationTime => &self.any_zone_time_vent_unocc,
        })
    }
}
impl SourceSystemReports {
    pub(super) fn register_source_facility_ventilation_outputs<
        H: SourceOutputFileHost,
        D: SourceOutputDictionaryHost,
    >(
        &self,
        registration: &mut SourceOutputRegistration,
        input: &mut SelectedInputProcessor,
        backend: &mut SourceErrorBackend<'_>,
        o: &mut SourceSystemReportsOutputContext<'_, H, D>,
    ) -> Result<(), Error> {
        for (field, name) in [
            (
                Field::AnyZoneBelowTargetTime,
                "Facility Any Zone Ventilation Below Target Voz Time",
            ),
            (
                Field::AllZonesAtTargetTime,
                "Facility All Zones Ventilation At Target Voz Time",
            ),
            (
                Field::AnyZoneAboveTargetTime,
                "Facility Any Zone Ventilation Above Target Voz Time",
            ),
            (
                Field::AnyZoneUnoccupiedVentilationTime,
                "Facility Any Zone Ventilation When Unoccupied Time",
            ),
        ] {
            let binding = SourceRealOutputBinding::new(Target::SystemReports { field }, self)
                .map_err(Error::Output)?;
            let mut report_input =
                SourceReportVariableInput::new(o.reports, input, o.minimum_report_frequency);
            registration
                .setup_bound_general_real(
                    &mut report_input,
                    o.prescan,
                    SourceRealOutputDeclaration {
                        name,
                        key: "Facility",
                        units: SourceOutputUnits::Hr,
                        store_type: SourceOutputStoreType::Sum,
                    },
                    o.minutes_in_time_step,
                    o.host,
                    o.eio,
                    binding,
                    o.dictionary_host,
                    SourceGeneralRealRegistrationOptions {
                        time_step: TimeStepKind::System,
                        meter: None,
                    },
                    backend,
                )
                .map_err(Error::Output)?;
        }
        Ok(())
    }
}
