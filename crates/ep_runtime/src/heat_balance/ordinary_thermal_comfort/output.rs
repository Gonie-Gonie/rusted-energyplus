//! Actual Zone declarations and borrowed same-owner comfort semantic targets.
use super::*;
use crate::heat_balance::radiation::ordinary::{
    OrdinaryRadiationError, OrdinaryRadiationUnavailable,
};
use crate::heat_balance::source_output_registration::{
    SourceComfortIntegerOutputField as IntegerField, SourceComfortOutputField as Field,
    SourceIntegerOutputBinding, SourceIntegerOutputDeclaration, SourceIntegerOutputOwners,
    SourceIntegerOutputTarget, SourceOutputStoreType, SourceOutputUnits, SourceRealOutputBinding,
    SourceRealOutputDeclaration, SourceRealOutputOwners, SourceRealOutputTarget,
    SourceRealRegistrationOptions, SourceReportVariableInput, TimeStepKind,
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
            PeopleAdaptiveAsh55RunningMeanOutdoorTemperature => {
                Ok(&people()?.ashrae55_running_mean_outdoor_temp)
            }
            PeopleAdaptiveAsh55ComfortTemperature => Ok(&people()?.t_comf_ash55),
            PeopleAdaptiveCen15251RunningMeanOutdoorTemperature => {
                Ok(&people()?.cen15251_running_mean_outdoor_temp)
            }
            PeopleAdaptiveCen15251ComfortTemperature => Ok(&people()?.t_comf_cen15251),
            PeopleFangerPmv => Ok(&people()?.fanger_pmv),
            PeopleAnkleDraftPpd => Ok(&people()?.ankle_draft_ppdash55),
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
    pub(super) fn register_integer<H: SourceOutputFileHost, D: SourceOutputDictionaryHost>(
        &self,
        o: &mut SourceThermalComfortOwners<'_, '_, H, D>,
        field: IntegerField,
        number: i32,
        name: &str,
        key: &str,
    ) -> Result<(), Error> {
        let binding = SourceIntegerOutputBinding::new(
            SourceIntegerOutputTarget::Comfort {
                field,
                index: number,
            },
            self,
        )
        .map_err(Error::Output)?;
        let mut input = SourceReportVariableInput::new(
            o.report_inputs,
            o.input,
            crate::heat_balance::source_output_registration::SourceReportVariableEnvironment::from_owners(
                o.minimum_report_frequency, o.globals, o.developer, Some(o.clock),
            ),
            o.backend,
        );
        o.registration
            .setup_bound_zone_integer(
                &mut input,
                o.prescan,
                SourceIntegerOutputDeclaration {
                    name,
                    key,
                    units: SourceOutputUnits::Dimensionless,
                    store_type: SourceOutputStoreType::Average,
                },
                o.clock.minutes_in_time_step,
                o.host,
                o.eio,
                binding,
                o.dictionary_host,
            )
            .map_err(Error::Output)
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
            Field::PeopleFangerPmv | Field::PeopleAnkleDraftPpd => (
                SourceOutputUnits::Dimensionless,
                SourceOutputStoreType::Average,
            ),
            Field::PeopleFangerPpd => (
                SourceOutputUnits::Percentage,
                SourceOutputStoreType::Average,
            ),
            Field::PeopleClothingSurfaceTemperature
            | Field::PeopleMeanRadiantTemperature
            | Field::PeopleOperativeTemperature
            | Field::PeopleAdaptiveAsh55RunningMeanOutdoorTemperature
            | Field::PeopleAdaptiveAsh55ComfortTemperature
            | Field::PeopleAdaptiveCen15251RunningMeanOutdoorTemperature
            | Field::PeopleAdaptiveCen15251ComfortTemperature => {
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
            SourceReportVariableInput::new(o.report_inputs, o.input, crate::heat_balance::source_output_registration::SourceReportVariableEnvironment::from_owners(o.minimum_report_frequency, o.globals, o.developer, Some(o.clock)), o.backend);
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

impl SourceIntegerOutputOwners for SourceThermalComfort {
    fn integer_output(
        &self,
        target: SourceIntegerOutputTarget,
    ) -> Result<&i32, OrdinaryRadiationError> {
        let SourceIntegerOutputTarget::Comfort {
            field,
            index: number,
        } = target
        else {
            return Err(OrdinaryRadiationUnavailable::StorageDomain.into());
        };
        let row = self
            .thermal_comfort_data
            .as_ref()
            .and_then(|rows| rows.get(index(number).ok()?))
            .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?;
        Ok(match field {
            IntegerField::PeopleAdaptiveAsh5590 => &row.thermal_comfort_adaptive_ash5590,
            IntegerField::PeopleAdaptiveAsh5580 => &row.thermal_comfort_adaptive_ash5580,
            IntegerField::PeopleAdaptiveCen15251CatI => {
                &row.thermal_comfort_adaptive_cen15251_cat_i
            }
            IntegerField::PeopleAdaptiveCen15251CatII => {
                &row.thermal_comfort_adaptive_cen15251_cat_ii
            }
            IntegerField::PeopleAdaptiveCen15251CatIII => {
                &row.thermal_comfort_adaptive_cen15251_cat_iii
            }
        })
    }
}
