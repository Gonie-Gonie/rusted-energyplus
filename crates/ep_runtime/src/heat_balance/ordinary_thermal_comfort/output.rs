//! Actual Hr/Sum/Zone declarations and borrowed same-owner semantic targets.
use super::*;
use crate::heat_balance::radiation::ordinary::{
    OrdinaryRadiationError, OrdinaryRadiationUnavailable,
};
use crate::heat_balance::source_output_registration::{
    SourceComfortOutputField as Field, SourceOutputStoreType, SourceOutputUnits,
    SourceRealOutputBinding, SourceRealOutputDeclaration, SourceRealOutputOwners,
    SourceRealOutputTarget, SourceRealRegistrationOptions, SourceReportVariableInput, TimeStepKind,
};
impl SourceThermalComfort {
    pub(crate) fn real_output(&self, field: Field, number: i32) -> Result<&f64, Error> {
        use Field::*;
        let simple = || {
            self.thermal_comfort_in_ash55
                .as_ref()
                .and_then(|v| v.get(index(number).ok()?))
                .ok_or(Error::Unavailable(
                    "actual allocated comfort simple Zone target",
                ))
        };
        let setpoint = || {
            self.thermal_comfort_setpoint
                .as_ref()
                .and_then(|v| v.get(index(number).ok()?))
                .ok_or(Error::Unavailable(
                    "actual allocated comfort setpoint Zone target",
                ))
        };
        match field {
            ZoneSimpleSummer => Ok(&simple()?.time_not_summer),
            ZoneSimpleWinter => Ok(&simple()?.time_not_winter),
            ZoneSimpleEither => Ok(&simple()?.time_not_either),
            ZoneNotMetHeating => Ok(&setpoint()?.not_met_heating),
            ZoneNotMetHeatingOccupied => Ok(&setpoint()?.not_met_heating_occupied),
            ZoneNotMetCooling => Ok(&setpoint()?.not_met_cooling),
            ZoneNotMetCoolingOccupied => Ok(&setpoint()?.not_met_cooling_occupied),
            FacilitySimpleSummer if number == 0 => Ok(&self.any_zone_time_not_simple_ash55_summer),
            FacilitySimpleWinter if number == 0 => Ok(&self.any_zone_time_not_simple_ash55_winter),
            FacilitySimpleEither if number == 0 => Ok(&self.any_zone_time_not_simple_ash55_either),
            FacilityNotMetHeating if number == 0 => Ok(&self.any_zone_not_met_heating),
            FacilityNotMetCooling if number == 0 => Ok(&self.any_zone_not_met_cooling),
            FacilityNotMetHeatingOccupied if number == 0 => {
                Ok(&self.any_zone_not_met_heating_occupied)
            }
            FacilityNotMetCoolingOccupied if number == 0 => {
                Ok(&self.any_zone_not_met_cooling_occupied)
            }
            _ => Err(Error::Unavailable(
                "actual comfort Facility target has no Zone ordinal",
            )),
        }
    }
    pub(super) fn register<H: SourceOutputFileHost, D: SourceOutputDictionaryHost>(
        &self,
        o: &mut SourceThermalComfortOwners<'_, '_, H, D>,
        field: Field,
        number: i32,
        name: &str,
        key: &str,
    ) -> Result<(), Error> {
        let binding = SourceRealOutputBinding::new(
            SourceRealOutputTarget::Comfort {
                field,
                index: number,
            },
            self,
        )
        .map_err(Error::Output)?;
        let mut input =
            SourceReportVariableInput::new(o.report_inputs, o.input, o.minimum_report_frequency);
        o.registration
            .setup_bound_real(
                &mut input,
                o.prescan,
                SourceRealOutputDeclaration {
                    name,
                    key,
                    units: SourceOutputUnits::Hr,
                    store_type: SourceOutputStoreType::Sum,
                },
                o.clock.minutes_in_time_step,
                o.host,
                o.eio,
                binding,
                o.dictionary_host,
                SourceRealRegistrationOptions::unmetered(TimeStepKind::Zone),
                o.backend,
            )
            .map_err(Error::Output)
    }
}
impl SourceRealOutputOwners for SourceThermalComfort {
    fn real_output(&self, target: SourceRealOutputTarget) -> Result<&f64, OrdinaryRadiationError> {
        match target {
            SourceRealOutputTarget::Comfort { field, index } => self
                .real_output(field, index)
                .map_err(|e| OrdinaryRadiationError::Input(Box::new(e))),
            _ => Err(OrdinaryRadiationUnavailable::StorageDomain.into()),
        }
    }
}
