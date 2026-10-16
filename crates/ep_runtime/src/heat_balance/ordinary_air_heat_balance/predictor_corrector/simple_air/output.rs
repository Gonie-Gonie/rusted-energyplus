//! DataHeatBalance205-300 / HB Air297-382 declarations and live report pointers.
use super::*;
use crate::heat_balance::radiation::ordinary::{
    OrdinaryRadiationError, OrdinaryRadiationUnavailable,
};
use crate::heat_balance::source_output_prescan::SourceOutputPreScan;
use crate::heat_balance::source_output_registration::{
    SourceAirOutputField as Field, SourceOutputDictionaryHost, SourceOutputFileHost,
    SourceOutputRegistration, SourceOutputStoreType as Store, SourceOutputUnits as Units,
    SourceRealOutputBinding, SourceRealOutputDeclaration, SourceRealOutputOwners,
    SourceRealOutputTarget, SourceRealRegistrationOptions, SourceReportVariableInput, TimeStepKind,
};
use ep_model::ReportInputs;
use std::io::Write;

/// Resolves only the report/site group; final State composes this with its
/// existing live AirHB/demand/thermostat resolver, without copying a number.
pub(crate) struct SourceAirReportOutputOwners<'a> {
    pub(crate) controller: &'a SourceZoneAirPredictorCorrector,
}
impl SourceRealOutputOwners for SourceAirReportOutputOwners<'_> {
    fn real_output(&self, target: SourceRealOutputTarget) -> Result<&f64, OrdinaryRadiationError> {
        let SourceRealOutputTarget::Air {
            field,
            index: number,
            space,
        } = target
        else {
            return Err(OrdinaryRadiationUnavailable::StorageDomain.into());
        };
        match field {
            Field::ZoneTotalExfiltrationHeatLoss | Field::ZoneTotalExhaustHeatLoss => {
                if number != 0 || space {
                    return Err(OrdinaryRadiationUnavailable::StorageDomain.into());
                }
                return Ok(match field {
                    Field::ZoneTotalExfiltrationHeatLoss => {
                        &self.controller.simple_air.zone_total_exfiltration_heat_loss
                    }
                    _ => &self.controller.simple_air.zone_total_exhaust_heat_loss,
                });
            }
            _ => {}
        }
        let i = index(number).map_err(|_| OrdinaryRadiationUnavailable::StorageDomain)?;
        let rows = if space {
            &self.controller.reports.spaces
        } else {
            &self.controller.reports.zones
        };
        let row = rows
            .as_deref()
            .and_then(|v| v.get(i))
            .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?;
        Ok(match field {
            Field::MeanAirTemp => &row.mean_air_temp,
            Field::WetbulbGlobeTemp => &row.wetbulb_globe_temp,
            Field::OperativeTemp => &row.operative_temp,
            Field::MeanAirDewPointTemp => &row.mean_air_dew_point_temp,
            Field::MeanAirHumRat => &row.mean_air_hum_rat,
            Field::SumIntGains => &row.sum_int_gains,
            Field::SumHADTsurfs => &row.sum_had_tsurfs,
            Field::SumMCpDTzones => &row.sum_m_cp_d_tzones,
            Field::SumMCpDtInfil => &row.sum_m_cp_dt_infil,
            Field::SumMCpDTsystem => &row.sum_m_cp_d_tsystem,
            Field::SumNonAirSystem => &row.sum_non_air_system,
            Field::CzdTdt => &row.czd_tdt,
            Field::ImBalance => &row.im_balance,
            Field::SumEnthalpyM => &row.sum_enthalpy_m,
            Field::SumEnthalpyH => &row.sum_enthalpy_h,
            Field::ExfilTotalLoss => &row.exfil_total_loss,
            Field::ExfilSensiLoss => &row.exfil_sensi_loss,
            Field::ExfilLatentLoss => &row.exfil_latent_loss,
            Field::ExhTotalLoss => &row.exh_total_loss,
            Field::ExhSensiLoss => &row.exh_sensi_loss,
            Field::ExhLatentLoss => &row.exh_latent_loss,
            _ => return Err(OrdinaryRadiationUnavailable::StorageDomain.into()),
        })
    }
}

pub(crate) struct SourceAirReportRegistrationContext<'a, H, D> {
    pub(crate) registration: &'a mut SourceOutputRegistration,
    pub(crate) prescan: &'a SourceOutputPreScan,
    pub(crate) reports: &'a ReportInputs,
    pub(crate) minimum_report_frequency: &'a str,
    pub(crate) minutes_in_time_step: i32,
    pub(crate) host: &'a mut H,
    pub(crate) eio: &'a mut dyn Write,
    pub(crate) dictionary_host: &'a mut D,
}
struct Declaration<'a> {
    field: Field,
    number: i32,
    space: bool,
    name: &'a str,
    key: &'a str,
    units: Units,
    time_step: TimeStepKind,
    store: Store,
}
struct ReportRecord<'a> {
    number: i32,
    space: bool,
    prefix: &'a str,
    key: &'a str,
    advanced: bool,
}
impl<H: SourceOutputFileHost, D: SourceOutputDictionaryHost>
    SourceAirReportRegistrationContext<'_, H, D>
{
    fn declare(
        &mut self,
        controller: &SourceZoneAirPredictorCorrector,
        input: &mut SelectedInputProcessor,
        backend: &mut SourceErrorBackend<'_>,
        declaration: Declaration<'_>,
    ) -> Result<(), Error> {
        let Declaration {
            field,
            number,
            space,
            name,
            key,
            units,
            time_step,
            store,
        } = declaration;
        let owners = SourceAirReportOutputOwners { controller };
        let binding = SourceRealOutputBinding::new(
            SourceRealOutputTarget::Air {
                field,
                index: number,
                space,
            },
            &owners,
        )
        .map_err(Error::Output)?;
        // Borrow the same input only for this actual call. The following nine
        // definition queries need the same owner after this borrow ends.
        let mut report_input =
            SourceReportVariableInput::new(self.reports, input, self.minimum_report_frequency);
        self.registration
            .setup_bound_real(
                &mut report_input,
                self.prescan,
                SourceRealOutputDeclaration {
                    name,
                    key,
                    units,
                    store_type: store,
                },
                self.minutes_in_time_step,
                self.host,
                self.eio,
                binding,
                self.dictionary_host,
                SourceRealRegistrationOptions::unmetered(time_step),
                backend,
            )
            .map_err(Error::Output)
    }
    fn register_report_record(
        &mut self,
        controller: &SourceZoneAirPredictorCorrector,
        input: &mut SelectedInputProcessor,
        backend: &mut SourceErrorBackend<'_>,
        record: ReportRecord<'_>,
    ) -> Result<(), Error> {
        let ReportRecord {
            number,
            space,
            prefix,
            key,
            advanced,
        } = record;
        use TimeStepKind::{System, Zone};
        for (suffix, field, units, step) in [
            ("Mean Air Temperature", Field::MeanAirTemp, Units::C, Zone),
            (
                "Wetbulb Globe Temperature",
                Field::WetbulbGlobeTemp,
                Units::C,
                Zone,
            ),
            (
                "Operative Temperature",
                Field::OperativeTemp,
                Units::C,
                Zone,
            ),
            (
                "Mean Air Dewpoint Temperature",
                Field::MeanAirDewPointTemp,
                Units::C,
                Zone,
            ),
            (
                "Mean Air Humidity Ratio",
                Field::MeanAirHumRat,
                Units::KgWaterPerKgDryAir,
                Zone,
            ),
            (
                "Air Heat Balance Internal Convective Heat Gain Rate",
                Field::SumIntGains,
                Units::W,
                System,
            ),
            (
                "Air Heat Balance Surface Convection Rate",
                Field::SumHADTsurfs,
                Units::W,
                System,
            ),
            (
                "Air Heat Balance Interzone Air Transfer Rate",
                Field::SumMCpDTzones,
                Units::W,
                System,
            ),
            (
                "Air Heat Balance Outdoor Air Transfer Rate",
                Field::SumMCpDtInfil,
                Units::W,
                System,
            ),
            (
                "Air Heat Balance System Air Transfer Rate",
                Field::SumMCpDTsystem,
                Units::W,
                System,
            ),
            (
                "Air Heat Balance System Convective Heat Gain Rate",
                Field::SumNonAirSystem,
                Units::W,
                System,
            ),
            (
                "Air Heat Balance Air Energy Storage Rate",
                Field::CzdTdt,
                Units::W,
                System,
            ),
        ] {
            self.declare(
                controller,
                input,
                backend,
                Declaration {
                    field,
                    number,
                    space,
                    name: &format!("{prefix} {suffix}"),
                    key,
                    units,
                    time_step: step,
                    store: Store::Average,
                },
            )?;
        }
        if advanced {
            self.declare(
                controller,
                input,
                backend,
                Declaration {
                    field: Field::ImBalance,
                    number,
                    space,
                    name: &format!("{prefix} Air Heat Balance Deviation Rate"),
                    key,
                    units: Units::W,
                    time_step: System,
                    store: Store::Average,
                },
            )?;
        }
        Ok(())
    }
    pub(super) fn register_source_air_reports(
        &mut self,
        controller: &SourceZoneAirPredictorCorrector,
        owners: &mut SourceSimpleAirInputOwners<'_, '_>,
    ) -> Result<(), Error> {
        use TimeStepKind::{System, Zone};
        for number in 1..=owners.globals.num_of_zones {
            let zone = owners
                .zones
                .zones
                .as_deref()
                .and_then(|v| v.get(index(number).ok()?))
                .ok_or(Error::Unavailable("actual Air report Zone298"))?;
            self.register_report_record(
                controller,
                owners.input,
                owners.backend,
                ReportRecord {
                    number,
                    space: false,
                    prefix: "Zone",
                    key: &zone.name,
                    advanced: owners.globals.display_advanced_report_variables,
                },
            )?;
            if owners.space_controls.simulation {
                for &space_number in &zone.space_indexes {
                    let space = owners
                        .zones
                        .spaces
                        .as_deref()
                        .and_then(|v| v.get(index(space_number).ok()?))
                        .ok_or(Error::Unavailable("actual Air report Space304"))?;
                    self.register_report_record(
                        controller,
                        owners.input,
                        owners.backend,
                        ReportRecord {
                            number: space_number,
                            space: true,
                            prefix: "Space",
                            key: &space.name,
                            advanced: owners.globals.display_advanced_report_variables,
                        },
                    )?;
                }
            }
            if owners.globals.display_advanced_report_variables {
                self.declare(
                    controller,
                    owners.input,
                    owners.backend,
                    Declaration {
                        field: Field::SumEnthalpyM,
                        number,
                        space: false,
                        name: "Zone Phase Change Material Melting Enthalpy",
                        key: &zone.name,
                        units: Units::JPerKg,
                        time_step: Zone,
                        store: Store::Average,
                    },
                )?;
                self.declare(
                    controller,
                    owners.input,
                    owners.backend,
                    Declaration {
                        field: Field::SumEnthalpyH,
                        number,
                        space: false,
                        name: "Zone Phase Change Material Freezing Enthalpy",
                        key: &zone.name,
                        units: Units::JPerKg,
                        time_step: Zone,
                        store: Store::Average,
                    },
                )?;
            }
            for (name, field) in [
                (
                    "Zone Exfiltration Heat Transfer Rate",
                    Field::ExfilTotalLoss,
                ),
                (
                    "Zone Exfiltration Sensible Heat Transfer Rate",
                    Field::ExfilSensiLoss,
                ),
                (
                    "Zone Exfiltration Latent Heat Transfer Rate",
                    Field::ExfilLatentLoss,
                ),
                ("Zone Exhaust Air Heat Transfer Rate", Field::ExhTotalLoss),
                (
                    "Zone Exhaust Air Sensible Heat Transfer Rate",
                    Field::ExhSensiLoss,
                ),
                (
                    "Zone Exhaust Air Latent Heat Transfer Rate",
                    Field::ExhLatentLoss,
                ),
            ] {
                self.declare(
                    controller,
                    owners.input,
                    owners.backend,
                    Declaration {
                        field,
                        number,
                        space: false,
                        name,
                        key: &zone.name,
                        units: Units::W,
                        time_step: System,
                        store: Store::Average,
                    },
                )?;
            }
        }
        self.declare(
            controller,
            owners.input,
            owners.backend,
            Declaration {
                field: Field::ZoneTotalExfiltrationHeatLoss,
                number: 0,
                space: false,
                name: "Site Total Zone Exfiltration Heat Loss",
                key: "Environment",
                units: Units::J,
                time_step: System,
                store: Store::Sum,
            },
        )?;
        self.declare(
            controller,
            owners.input,
            owners.backend,
            Declaration {
                field: Field::ZoneTotalExhaustHeatLoss,
                number: 0,
                space: false,
                name: "Site Total Zone Exhaust Air Heat Loss",
                key: "Environment",
                units: Units::J,
                time_step: System,
                store: Store::Sum,
            },
        )?;
        Ok(())
    }
}
