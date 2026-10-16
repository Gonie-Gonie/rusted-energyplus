//! Actual three unconditional Pollution System/Sum/kg live registrations512-542.
use super::*;
use crate::heat_balance::radiation::ordinary::{
    OrdinaryRadiationError, OrdinaryRadiationUnavailable,
};
use crate::heat_balance::source_output_prescan::SourceOutputPreScan;
use crate::heat_balance::source_output_registration::{
    SourceGeneralMeterArguments, SourceGeneralRealRegistrationOptions, SourceMeterEndUse,
    SourceMeterGroup, SourceMeterResource, SourceOutputDictionaryHost, SourceOutputFileHost,
    SourceOutputStoreType, SourceOutputUnits, SourceRealOutputBinding, SourceRealOutputDeclaration,
    SourceRealOutputTarget, SourceReportVariableInput,
};
use ep_model::ReportInputs;
pub(crate) struct SourcePollutionOutputContext<'a, H, D> {
    pub(crate) registration: &'a mut SourceOutputRegistration,
    pub(crate) prescan: &'a SourceOutputPreScan,
    pub(crate) reports: &'a ReportInputs,
    pub(crate) minimum_report_frequency: &'a str,
    pub(crate) host: &'a mut H,
    pub(crate) dictionary_host: &'a mut D,
}
impl SourceRealOutputOwners for SourcePollutionModule {
    fn real_output(&self, target: SourceRealOutputTarget) -> Result<&f64, OrdinaryRadiationError> {
        let SourceRealOutputTarget::Pollution { field } = target else {
            return Err(OrdinaryRadiationUnavailable::StorageDomain.into());
        };
        Ok(match field {
            SourceCarbonEquivalentField::N2o => &self.total_carbon_equivalent_n2o,
            SourceCarbonEquivalentField::Ch4 => &self.total_carbon_equivalent_ch4,
            SourceCarbonEquivalentField::Co2 => &self.total_carbon_equivalent_co2,
        })
    }
}
impl<H: SourceOutputFileHost, D: SourceOutputDictionaryHost>
    SourcePollutionOutputContext<'_, H, D>
{
    pub(super) fn declare(
        &mut self,
        pollution: &SourcePollutionModule,
        field: SourceCarbonEquivalentField,
        clock: &WeatherGlobalState,
        input: &mut SelectedInputProcessor,
        backend: &mut SourceErrorBackend<'_>,
        eio: &mut dyn std::io::Write,
    ) -> Result<(), Error> {
        let name = match field {
            SourceCarbonEquivalentField::N2o => {
                "Environmental Impact Total N2O Emissions Carbon Equivalent Mass"
            }
            SourceCarbonEquivalentField::Ch4 => {
                "Environmental Impact Total CH4 Emissions Carbon Equivalent Mass"
            }
            SourceCarbonEquivalentField::Co2 => {
                "Environmental Impact Total CO2 Emissions Carbon Equivalent Mass"
            }
        };
        let binding =
            SourceRealOutputBinding::new(SourceRealOutputTarget::Pollution { field }, pollution)
                .map_err(Error::Output)?;
        let mut input =
            SourceReportVariableInput::new(self.reports, input, self.minimum_report_frequency);
        self.registration
            .setup_bound_general_real(
                &mut input,
                self.prescan,
                SourceRealOutputDeclaration {
                    name,
                    key: "Site",
                    units: SourceOutputUnits::Kg,
                    store_type: SourceOutputStoreType::Sum,
                },
                clock.minutes_in_time_step,
                self.host,
                eio,
                binding,
                self.dictionary_host,
                SourceGeneralRealRegistrationOptions {
                    time_step: TimeStepKind::System,
                    meter: Some(SourceGeneralMeterArguments {
                        resource: SourceMeterResource::CarbonEquivalent,
                        end_use: SourceMeterEndUse::CarbonEquivalentEmissions,
                        end_use_subcategory: "",
                        group: SourceMeterGroup::Invalid,
                        zone_name: "",
                        space_type: "",
                        zone_multiplier: 1,
                        zone_list_multiplier: 1,
                    }),
                },
                backend,
            )
            .map_err(Error::Output)
    }
}
