//! Actual Zone declarations and borrowed same-owner comfort semantic targets.
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
        let people = || {
            self.thermal_comfort_data
                .as_ref()
                .and_then(|rows| rows.get(index(number).ok()?))
                .ok_or(Error::Unavailable("actual allocated comfort People target"))
        };
        match field {
            PeopleFangerPmv => Ok(&people()?.fanger_pmv),
            PeopleFangerPpd => Ok(&people()?.fanger_ppd),
            PeopleClothingSurfaceTemperature => Ok(&people()?.clo_surf_temp),
            PeopleMeanRadiantTemperature => Ok(&people()?.thermal_comfort_mrt),
            PeopleOperativeTemperature => Ok(&people()?.thermal_comfort_op_temp),
            PeopleClothingValue => Ok(&people()?.clothing_value),
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
        let (units, store_type) = match field {
            Field::PeopleFangerPmv => (
                SourceOutputUnits::Dimensionless,
                SourceOutputStoreType::Average,
            ),
            Field::PeopleFangerPpd => (
                SourceOutputUnits::Percentage,
                SourceOutputStoreType::Average,
            ),
            Field::PeopleClothingSurfaceTemperature
            | Field::PeopleMeanRadiantTemperature
            | Field::PeopleOperativeTemperature => {
                (SourceOutputUnits::C, SourceOutputStoreType::Average)
            }
            Field::PeopleClothingValue => (SourceOutputUnits::Clo, SourceOutputStoreType::Average),
            _ => (SourceOutputUnits::Hr, SourceOutputStoreType::Sum),
        };
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
                    units,
                    store_type,
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
