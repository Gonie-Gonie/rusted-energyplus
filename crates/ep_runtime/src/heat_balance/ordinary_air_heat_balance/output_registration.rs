//! Actual twenty-six first InitZoneAir output registration calls.
use super::environment::SourceZoneFanHeatBalanceArrays;
use super::predictor_corrector::{
    SourceSystemMoistureDemand, SourceSystemSensibleDemand, SourceZoneAirDemand,
    SourceZoneAirRegistrationChildren, SourceZoneListGroupLoadReports,
};
use super::{SourceAirHeatBalance, SourceZoneSpaceHeatBalanceData};
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::radiation::ordinary::{
    OrdinaryRadiationError, OrdinaryRadiationUnavailable,
};
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::heat_balance::selected_input_processor::SelectedInputProcessor;
use crate::heat_balance::source_output_prescan::SourceOutputPreScan;
use crate::heat_balance::source_output_registration::{
    SourceAirOutputField as Field, SourceEnergyTransferMeterArguments, SourceIntegerOutputBinding,
    SourceIntegerOutputDeclaration, SourceIntegerOutputOwners, SourceIntegerOutputTarget,
    SourceMeterEndUse, SourceOutputDictionaryHost, SourceOutputFileHost, SourceOutputRegistration,
    SourceOutputStoreType as Store, SourceOutputUnits as Units, SourceRealOutputBinding,
    SourceRealOutputDeclaration, SourceRealOutputOwners, SourceRealOutputTarget,
    SourceRealRegistrationOptions, SourceReportVariableInput, TimeStepKind,
};
use crate::weather::day::WeatherGlobalState;
use ep_model::ReportInputs;
use std::io::Write;
mod bindings;
pub(crate) use bindings::SourceAirOutputOwners;
use bindings::{Record, Row};

/// Clock/IP/backend/EIO are supplied by each actual caller, allowing disjoint borrows
/// of the same real owners through the complete InitZoneAir body.
pub(crate) struct SourceInitAirOutputHost<'a, H, D> {
    pub(crate) registration: &'a mut SourceOutputRegistration,
    pub(crate) prescan: &'a SourceOutputPreScan,
    pub(crate) reports: &'a ReportInputs,
    pub(crate) minimum_report_frequency: &'a str,
    pub(crate) host: &'a mut H,
    pub(crate) dictionary_host: &'a mut D,
}
struct Declaration<'a> {
    field: Field,
    name: &'a str,
    key: &'a str,
    units: Units,
    store: Store,
    options: SourceRealRegistrationOptions<'a>,
}
impl<H: SourceOutputFileHost, D: SourceOutputDictionaryHost> SourceInitAirOutputHost<'_, H, D> {
    fn declare(
        &mut self,
        row: &Row<'_>,
        declaration: Declaration<'_>,
        clock: &WeatherGlobalState,
        reporting_diagnostic_flags: Option<
            crate::heat_balance::error_backend::SourceDiagnosticFlags<'_>,
        >,
        input: &mut SelectedInputProcessor,
        backend: &mut SourceErrorBackend<'_>,
        eio: &mut dyn Write,
    ) -> Result<(), Error> {
        let binding = SourceRealOutputBinding::new(
            SourceRealOutputTarget::Air {
                field: declaration.field,
                index: row.number,
                space: row.space,
            },
            row,
        )
        .map_err(Error::Output)?;
        let mut report_input = SourceReportVariableInput::new(
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
                &mut report_input,
                self.prescan,
                SourceRealOutputDeclaration {
                    name: declaration.name,
                    key: declaration.key,
                    units: declaration.units,
                    store_type: declaration.store,
                },
                clock.minutes_in_time_step,
                self.host,
                eio,
                binding,
                self.dictionary_host,
                declaration.options,
            )
            .map_err(Error::Output)
    }
}
impl<H: SourceOutputFileHost, D: SourceOutputDictionaryHost> SourceZoneAirRegistrationChildren
    for SourceInitAirOutputHost<'_, H, D>
{
    fn register_zone_list(
        &mut self,
        number: i32,
        name: &str,
        reports: &SourceZoneListGroupLoadReports,
        clock: &WeatherGlobalState,
        reporting_diagnostic_flags: Option<
            crate::heat_balance::error_backend::SourceDiagnosticFlags<'_>,
        >,
        input: &mut SelectedInputProcessor,
        backend: &mut SourceErrorBackend<'_>,
        eio: &mut dyn Write,
    ) -> Result<(), Error> {
        let row = Row {
            number,
            space: false,
            record: Record::ZoneList(reports),
        };
        for (field, variable, units, store) in [
            (
                Field::ZoneListSensibleHeatingEnergy,
                "Zone List Sensible Heating Energy",
                Units::J,
                Store::Sum,
            ),
            (
                Field::ZoneListSensibleCoolingEnergy,
                "Zone List Sensible Cooling Energy",
                Units::J,
                Store::Sum,
            ),
            (
                Field::ZoneListSensibleHeatingRate,
                "Zone List Sensible Heating Rate",
                Units::W,
                Store::Average,
            ),
            (
                Field::ZoneListSensibleCoolingRate,
                "Zone List Sensible Cooling Rate",
                Units::W,
                Store::Average,
            ),
        ] {
            self.declare(
                &row,
                Declaration {
                    field,
                    name: variable,
                    key: name,
                    units,
                    store,
                    options: SourceRealRegistrationOptions::unmetered(TimeStepKind::System),
                },
                clock,
                reporting_diagnostic_flags,
                input,
                backend,
                eio,
            )?;
        }
        Ok(())
    }
    fn register_air(
        &mut self,
        number: i32,
        space: bool,
        name: &str,
        air: &SourceZoneSpaceHeatBalanceData,
        clock: &WeatherGlobalState,
        reporting_diagnostic_flags: Option<
            crate::heat_balance::error_backend::SourceDiagnosticFlags<'_>,
        >,
        input: &mut SelectedInputProcessor,
        backend: &mut SourceErrorBackend<'_>,
        eio: &mut dyn Write,
    ) -> Result<(), Error> {
        let row = Row {
            number,
            space,
            record: Record::Air(air),
        };
        let prefix = if space { "Space" } else { "Zone" };
        for (field, suffix, units, time_step) in [
            (
                Field::AirTemperature,
                "Air Temperature",
                Units::C,
                TimeStepKind::System,
            ),
            (
                Field::AirHumidityRatio,
                "Air Humidity Ratio",
                Units::Dimensionless,
                TimeStepKind::System,
            ),
            (
                Field::AirRelativeHumidity,
                "Air Relative Humidity",
                Units::Percentage,
                TimeStepKind::System,
            ),
            (
                Field::MeanRadiantTemperature,
                "Mean Radiant Temperature",
                Units::C,
                TimeStepKind::Zone,
            ),
        ] {
            self.declare(
                &row,
                Declaration {
                    field,
                    name: &format!("{prefix} {suffix}"),
                    key: name,
                    units,
                    store: Store::Average,
                    options: SourceRealRegistrationOptions::unmetered(time_step),
                },
                clock,
                reporting_diagnostic_flags,
                input,
                backend,
                eio,
            )?;
        }
        Ok(())
    }
    fn register_sensible_demand(
        &mut self,
        number: i32,
        space: bool,
        name: &str,
        staged: bool,
        attach_meters: bool,
        zone_multiplier: i32,
        list_multiplier: i32,
        demand: &SourceSystemSensibleDemand,
        clock: &WeatherGlobalState,
        reporting_diagnostic_flags: Option<
            crate::heat_balance::error_backend::SourceDiagnosticFlags<'_>,
        >,
        input: &mut SelectedInputProcessor,
        backend: &mut SourceErrorBackend<'_>,
        eio: &mut dyn Write,
    ) -> Result<(), Error> {
        let row = Row {
            number,
            space,
            record: Record::Sensible(demand),
        };
        let prefix = if space { "Space" } else { "Zone" };
        for (field, suffix, units, store, end_use) in [
            (
                Field::SensibleHeatingEnergy,
                "Air System Sensible Heating Energy",
                Units::J,
                Store::Sum,
                Some(SourceMeterEndUse::Heating),
            ),
            (
                Field::SensibleCoolingEnergy,
                "Air System Sensible Cooling Energy",
                Units::J,
                Store::Sum,
                Some(SourceMeterEndUse::Cooling),
            ),
            (
                Field::SensibleHeatingRate,
                "Air System Sensible Heating Rate",
                Units::W,
                Store::Average,
                None,
            ),
            (
                Field::SensibleCoolingRate,
                "Air System Sensible Cooling Rate",
                Units::W,
                Store::Average,
                None,
            ),
            (
                Field::PredictedSensibleRate,
                "Predicted Sensible Load to Setpoint Heat Transfer Rate",
                Units::W,
                Store::Average,
                None,
            ),
            (
                Field::PredictedHeatingSetpointRate,
                "Predicted Sensible Load to Heating Setpoint Heat Transfer Rate",
                Units::W,
                Store::Average,
                None,
            ),
            (
                Field::PredictedCoolingSetpointRate,
                "Predicted Sensible Load to Cooling Setpoint Heat Transfer Rate",
                Units::W,
                Store::Average,
                None,
            ),
            (
                Field::SystemPredictedSensibleRate,
                "System Predicted Sensible Load to Setpoint Heat Transfer Rate",
                Units::W,
                Store::Average,
                None,
            ),
            (
                Field::SystemPredictedHeatingSetpointRate,
                "System Predicted Sensible Load to Heating Setpoint Heat Transfer Rate",
                Units::W,
                Store::Average,
                None,
            ),
            (
                Field::SystemPredictedCoolingSetpointRate,
                "System Predicted Sensible Load to Cooling Setpoint Heat Transfer Rate",
                Units::W,
                Store::Average,
                None,
            ),
        ] {
            let meter = if attach_meters {
                end_use.map(|end_use| SourceEnergyTransferMeterArguments {
                    end_use,
                    end_use_subcategory: "",
                    zone_name: name,
                    zone_multiplier,
                    zone_list_multiplier: list_multiplier,
                })
            } else {
                None
            };
            self.declare(
                &row,
                Declaration {
                    field,
                    name: &format!("{prefix} {suffix}"),
                    key: name,
                    units,
                    store,
                    options: SourceRealRegistrationOptions {
                        time_step: TimeStepKind::System,
                        meter,
                    },
                },
                clock,
                reporting_diagnostic_flags,
                input,
                backend,
                eio,
            )?;
        }
        // Actual staged Integer/System call is after the ten normal rows.
        if staged {
            return Err(Error::Unavailable("ZoneSystemSensibleDemand staged output"));
        }
        Ok(())
    }
    fn register_moisture_demand(
        &mut self,
        number: i32,
        space: bool,
        name: &str,
        latent_sizing: bool,
        demand: &SourceSystemMoistureDemand,
        clock: &WeatherGlobalState,
        reporting_diagnostic_flags: Option<
            crate::heat_balance::error_backend::SourceDiagnosticFlags<'_>,
        >,
        input: &mut SelectedInputProcessor,
        backend: &mut SourceErrorBackend<'_>,
        eio: &mut dyn Write,
    ) -> Result<(), Error> {
        // Actual latent-sizing block precedes the normal six predicted rows.
        if latent_sizing {
            return Err(Error::Unavailable(
                "ZoneSystemMoistureDemand latent-sizing outputs",
            ));
        }
        let row = Row {
            number,
            space,
            record: Record::Moisture(demand),
        };
        let prefix = if space { "Space" } else { "Zone" };
        for (field, suffix) in [
            (
                Field::PredictedMoistureRate,
                "Predicted Moisture Load Moisture Transfer Rate",
            ),
            (
                Field::PredictedHumidifyingSetpointRate,
                "Predicted Moisture Load to Humidifying Setpoint Moisture Transfer Rate",
            ),
            (
                Field::PredictedDehumidifyingSetpointRate,
                "Predicted Moisture Load to Dehumidifying Setpoint Moisture Transfer Rate",
            ),
            (
                Field::SystemPredictedMoistureRate,
                "System Predicted Moisture Load Moisture Transfer Rate",
            ),
            (
                Field::SystemPredictedHumidifyingSetpointRate,
                "System Predicted Moisture Load to Humidifying Setpoint Moisture Transfer Rate",
            ),
            (
                Field::SystemPredictedDehumidifyingSetpointRate,
                "System Predicted Moisture Load to Dehumidifying Setpoint Moisture Transfer Rate",
            ),
        ] {
            self.declare(
                &row,
                Declaration {
                    field,
                    name: &format!("{prefix} {suffix}"),
                    key: name,
                    units: Units::KgWaterPerSecond,
                    store: Store::Average,
                    options: SourceRealRegistrationOptions::unmetered(TimeStepKind::System),
                },
                clock,
                reporting_diagnostic_flags,
                input,
                backend,
                eio,
            )?;
        }
        Ok(())
    }
    fn register_thermostat(
        &mut self,
        number: i32,
        name: &str,
        fan: &SourceZoneFanHeatBalanceArrays,
        clock: &WeatherGlobalState,
        reporting_diagnostic_flags: Option<
            crate::heat_balance::error_backend::SourceDiagnosticFlags<'_>,
        >,
        input: &mut SelectedInputProcessor,
        backend: &mut SourceErrorBackend<'_>,
        eio: &mut dyn Write,
    ) -> Result<(), Error> {
        let row = Row {
            number,
            space: false,
            record: Record::Thermostat(fan),
        };
        self.declare(
            &row,
            Declaration {
                field: Field::ThermostatAirTemperature,
                name: "Zone Thermostat Air Temperature",
                key: name,
                units: Units::C,
                store: Store::Average,
                options: SourceRealRegistrationOptions::unmetered(TimeStepKind::System),
            },
            clock,
            reporting_diagnostic_flags,
            input,
            backend,
            eio,
        )?;
        let binding = SourceIntegerOutputBinding::new(
            SourceIntegerOutputTarget::ZoneThermostatControlType { index: number },
            &row,
        )
        .map_err(Error::Output)?;
        let mut report_input = SourceReportVariableInput::new(
            self.reports,
            input,
            crate::heat_balance::source_output_registration::SourceReportVariableEnvironment::new(
                self.minimum_report_frequency,
                reporting_diagnostic_flags,
            ),
            backend,
        );
        self.registration
            .setup_bound_zone_integer(
                &mut report_input,
                self.prescan,
                SourceIntegerOutputDeclaration {
                    name: "Zone Thermostat Control Type",
                    key: name,
                    units: Units::Dimensionless,
                    store_type: Store::Average,
                },
                clock.minutes_in_time_step,
                self.host,
                eio,
                binding,
                self.dictionary_host,
            )
            .map_err(Error::Output)?;
        for (field, output_name, units, time_step) in [
            (
                Field::ThermostatHeatingSetpoint,
                "Zone Thermostat Heating Setpoint Temperature",
                Units::C,
                TimeStepKind::System,
            ),
            (
                Field::ThermostatCoolingSetpoint,
                "Zone Thermostat Cooling Setpoint Temperature",
                Units::C,
                TimeStepKind::System,
            ),
            (
                Field::AdaptiveComfortCoolingSetpoint,
                "Zone Adaptive Comfort Operative Temperature Set Point",
                Units::C,
                TimeStepKind::Zone,
            ),
            (
                Field::LoadCorrectionFactor,
                "Zone Predicted Sensible Load Room Air Correction Factor",
                Units::Dimensionless,
                TimeStepKind::System,
            ),
        ] {
            self.declare(
                &row,
                Declaration {
                    field,
                    name: output_name,
                    key: name,
                    units,
                    store: Store::Average,
                    options: SourceRealRegistrationOptions::unmetered(time_step),
                },
                clock,
                reporting_diagnostic_flags,
                input,
                backend,
                eio,
            )?;
        }
        Ok(())
    }
}
