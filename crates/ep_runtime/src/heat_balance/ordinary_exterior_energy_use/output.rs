//! Two actual Zone declarations with live ExteriorLight Power/CurrentUse bindings.
use super::*;
use crate::heat_balance::radiation::ordinary::{
    OrdinaryRadiationError, OrdinaryRadiationUnavailable,
};
use crate::heat_balance::source_output_registration::{
    SourceExteriorLightOutputField as Field, SourceGeneralMeterArguments,
    SourceGeneralRealRegistrationOptions, SourceMeterEndUse, SourceMeterGroup, SourceMeterResource,
    SourceOutputStoreType, SourceOutputUnits, SourceRealOutputBinding, SourceRealOutputDeclaration,
    SourceRealOutputOwners, SourceRealOutputTarget, SourceRealRegistrationOptions,
    SourceReportVariableInput, TimeStepKind,
};
impl SourceRealOutputOwners for SourceExteriorEnergyUse {
    fn real_output(&self, target: SourceRealOutputTarget) -> Result<&f64, OrdinaryRadiationError> {
        match target {
            SourceRealOutputTarget::ExteriorLight { field, index } => {
                let row = self
                    .light(index)
                    .map_err(|e| OrdinaryRadiationError::Input(Box::new(e)))?;
                Ok(match field {
                    Field::Power => &row.power,
                    Field::CurrentUse => &row.current_use,
                })
            }
            _ => Err(OrdinaryRadiationUnavailable::StorageDomain.into()),
        }
    }
}
impl SourceExteriorEnergyUse {
    pub(super) fn register_source_light<H: SourceOutputFileHost, D: SourceOutputDictionaryHost>(
        &self,
        number: i32,
        subcategory: &str,
        o: &mut SourceExteriorEnergyOwners<'_, '_, H, D>,
    ) -> Result<(), Error> {
        let row = self.light(number)?;
        let binding = SourceRealOutputBinding::new(
            SourceRealOutputTarget::ExteriorLight {
                field: Field::Power,
                index: number,
            },
            self,
        )
        .map_err(Error::Output)?;
        let mut input =
            SourceReportVariableInput::new(o.reports, o.input, o.minimum_report_frequency);
        o.registration
            .setup_bound_real(
                &mut input,
                o.prescan,
                SourceRealOutputDeclaration {
                    name: "Exterior Lights Electricity Rate",
                    key: &row.name,
                    units: SourceOutputUnits::W,
                    store_type: SourceOutputStoreType::Average,
                },
                o.clock.minutes_in_time_step,
                o.host,
                o.eio,
                binding,
                o.dictionary_host,
                SourceRealRegistrationOptions::unmetered(TimeStepKind::Zone),
                o.backend,
            )
            .map_err(Error::Output)?;
        let binding = SourceRealOutputBinding::new(
            SourceRealOutputTarget::ExteriorLight {
                field: Field::CurrentUse,
                index: number,
            },
            self,
        )
        .map_err(Error::Output)?;
        let mut input =
            SourceReportVariableInput::new(o.reports, o.input, o.minimum_report_frequency);
        o.registration
            .setup_bound_general_real(
                &mut input,
                o.prescan,
                SourceRealOutputDeclaration {
                    name: "Exterior Lights Electricity Energy",
                    key: &row.name,
                    units: SourceOutputUnits::J,
                    store_type: SourceOutputStoreType::Sum,
                },
                o.clock.minutes_in_time_step,
                o.host,
                o.eio,
                binding,
                o.dictionary_host,
                SourceGeneralRealRegistrationOptions {
                    time_step: TimeStepKind::Zone,
                    meter: Some(SourceGeneralMeterArguments {
                        resource: SourceMeterResource::Electricity,
                        end_use: SourceMeterEndUse::ExteriorLights,
                        end_use_subcategory: subcategory,
                        group: SourceMeterGroup::Invalid,
                        zone_name: "",
                        space_type: "",
                        zone_multiplier: 1,
                        zone_list_multiplier: 1,
                    }),
                },
                o.backend,
            )
            .map_err(Error::Output)
    }
}
