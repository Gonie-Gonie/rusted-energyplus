//! Actual ReportZoneMeanAirTemp and calcMeanAirTemps, borrowing sole live owners.
use super::super::demand::SourceAirReport;
use super::*;
use crate::heat_balance::ordinary_air_heat_balance::SourceAirHeatBalance;
use crate::heat_balance::selected_simulation_startup::SelectedRuntimeLanguage;
use crate::heat_balance::source_output_registration::SourceOutputRegistration;
use crate::psychrometrics::{EnergyPlusPsychrometricFunction as Psy, SourcePsychrometricAccess};
use crate::weather::day::WeatherEnvironmentState;

/// The actual supplied Source Air/Zone/report request/runtime language/PSY owners.
/// No scalar report snapshot, constructor/reset or per-step clone is stored here.
pub(crate) struct SourceMeanAirReportOwners<'a> {
    pub(crate) air: &'a SourceAirHeatBalance,
    pub(crate) zones: &'a SelectedZoneInputOwners,
    pub(crate) globals: &'a SelectedHeatBalanceGlobals,
    pub(crate) space_controls: &'a SelectedHeatBalanceSpaceControls,
    pub(crate) environment: &'a WeatherEnvironmentState,
    pub(crate) registration: &'a SourceOutputRegistration,
    pub(crate) runtime_language: &'a SelectedRuntimeLanguage,
    pub(crate) psychrometrics: &'a mut SourcePsychrometricAccess,
}
fn report_mut(
    reports: &mut SourceZoneAirReports,
    space: bool,
    number: i32,
) -> Result<&mut SourceAirReport, Error> {
    let rows = if space {
        &mut reports.spaces
    } else {
        &mut reports.zones
    };
    rows.as_mut()
        .and_then(|rows| rows.get_mut(index(number).ok()?))
        .ok_or(Error::Unavailable(
            "actual ReportZoneMeanAirTemp AirReport ordinal",
        ))
}
fn find_zone(zones: &SelectedZoneInputOwners, key: &str) -> Option<i32> {
    // Actual FindItemInList is exact equality after makeUPPER at the request call.
    zones
        .zones
        .as_deref()
        .unwrap_or(&[])
        .iter()
        .position(|row| row.name == key)
        .and_then(|i| i32::try_from(i + 1).ok())
}
fn find_space(zones: &SelectedZoneInputOwners, key: &str) -> Option<i32> {
    zones
        .spaces
        .as_deref()
        .unwrap_or(&[])
        .iter()
        .position(|row| row.name == key)
        .and_then(|i| i32::try_from(i + 1).ok())
}
/// Literal PsyTdpFnWPb1550-1552. The same instance's actual TsatPb cache,
/// nested Psat cache and saved scalar state execute through its existing route.
fn source_dew_point(
    psy: &mut SourcePsychrometricAccess,
    humidity: f64,
    pressure: f64,
) -> Result<f64, Error> {
    // Actual ObjexxFCL double max(W,1e-5), preserving first-operand NaN/-0 rules.
    let limited_humidity = if humidity < 1.0e-5 { 1.0e-5 } else { humidity };
    let dew_pressure = pressure * limited_humidity / (0.62198 + limited_humidity);
    Ok(psy
        .evaluate(Psy::TsatPb, &[dew_pressure])
        .map_err(Error::Unavailable)?
        .result)
}
fn calculate_mean_air_temperatures(
    average_temperature: f64,
    average_humidity: f64,
    mean_radiant_temperature: f64,
    report: &mut SourceAirReport,
    any_operative_control: bool,
    environment: &WeatherEnvironmentState,
    psy: &mut SourcePsychrometricAccess,
) -> Result<(), Error> {
    report.mean_air_temp = average_temperature;
    report.mean_air_hum_rat = average_humidity;
    report.operative_temp = 0.5 * (average_temperature + mean_radiant_temperature);
    report.mean_air_dew_point_temp = source_dew_point(
        psy,
        report.mean_air_hum_rat,
        environment.current_weather.out_baro_press,
    )?;
    if any_operative_control {
        // The actual next operation reads Source Zone.TempControlledZoneIndex.
        // That positive thermostat child/field is not represented by this route.
        return Err(Error::Unavailable(
            "positive actual operative Zone index read4708",
        ));
    }
    if report.report_wbgt {
        report.wetbulb_globe_temp = 0.7
            * psy
                .evaluate(
                    Psy::Twb,
                    &[
                        average_temperature,
                        average_humidity,
                        environment.current_weather.out_baro_press,
                    ],
                )
                .map_err(Error::Unavailable)?
                .result
            + 0.3 * report.operative_temp;
    }
    Ok(())
}
impl SourceZoneAirPredictorCorrector {
    /// Actual first request pass4623-4654, real zero-sensor traversal4657,
    /// successful one-time clear4674, then Zone-first/conditional Space reports.
    pub(crate) fn report_source_zone_mean_air_temp(
        &mut self,
        o: &mut SourceMeanAirReportOwners<'_>,
    ) -> Result<(), Error> {
        if self.simple_air.calc_extra_report_var_one_time_flag {
            for request in &o.registration.requested_variables {
                if request.name == "ZONE WETBULB GLOBE TEMPERATURE" {
                    if request.key.is_empty() {
                        for number in 1..=o.globals.num_of_zones {
                            report_mut(&mut self.reports, false, number)?.report_wbgt = true;
                        }
                    } else {
                        let key = request.key.to_ascii_uppercase();
                        if let Some(number) = find_zone(o.zones, &key) {
                            report_mut(&mut self.reports, false, number)?.report_wbgt = true;
                        }
                    }
                } else if request.name == "SPACE WETBULB GLOBE TEMPERATURE"
                    && o.space_controls.simulation
                {
                    if request.key.is_empty() {
                        for number in 1..=o.globals.num_spaces {
                            report_mut(&mut self.reports, true, number)?.report_wbgt = true;
                        }
                    } else {
                        let key = request.key.to_ascii_uppercase();
                        if let Some(number) = find_space(o.zones, &key) {
                            report_mut(&mut self.reports, true, number)?.report_wbgt = true;
                        }
                    }
                }
            }
            if o.runtime_language.number_of_sensors > 0 {
                return Err(Error::Unavailable(
                    "positive actual RuntimeLanguage Sensor(1) read4658",
                ));
            }
            // Native count0 leaves its unallocated Sensor array untouched.
            self.simple_air.calc_extra_report_var_one_time_flag = false;
        }
        for number in 1..=o.globals.num_of_zones {
            let i = index(number)?;
            let zone_air = &o
                .air
                .zones
                .as_ref()
                .and_then(|rows| rows.get(i))
                .ok_or(Error::Unavailable("actual report Zone AirHB4677"))?
                .air;
            calculate_mean_air_temperatures(
                zone_air.ztav,
                zone_air.air_hum_rat_avg,
                zone_air.mrt,
                report_mut(&mut self.reports, false, number)?,
                self.controls.any_operative_control,
                o.environment,
                o.psychrometrics,
            )?;
            if o.space_controls.simulation {
                let zone = o
                    .zones
                    .zones
                    .as_ref()
                    .and_then(|rows| rows.get(i))
                    .ok_or(Error::Unavailable("actual report Zone spaceIndexes4680"))?;
                for &space in &zone.space_indexes {
                    let space_air = &o
                        .air
                        .spaces
                        .as_ref()
                        .and_then(|rows| rows.get(index(space).ok()?))
                        .ok_or(Error::Unavailable("actual report Space AirHB4681"))?
                        .air;
                    calculate_mean_air_temperatures(
                        space_air.ztav,
                        space_air.air_hum_rat_avg,
                        space_air.mrt,
                        report_mut(&mut self.reports, true, space)?,
                        self.controls.any_operative_control,
                        o.environment,
                        o.psychrometrics,
                    )?;
                }
            }
        }
        Ok(())
    }
}
