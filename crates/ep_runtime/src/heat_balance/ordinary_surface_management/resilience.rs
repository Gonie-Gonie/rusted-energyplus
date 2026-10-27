//! CalcThermalResilience5707-5799 on the actual Zone/Air/Psychrometrics owners.
use super::*;
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::ordinary_air_heat_balance::SourceAirHeatBalance;
use crate::heat_balance::radiation::ordinary::{
    OrdinaryRadiationError, OrdinaryRadiationUnavailable,
};
use crate::heat_balance::selected_heat_balance_input::zone::SelectedZoneInputOwners;
use crate::heat_balance::selected_input_processor::SelectedInputProcessor;
use crate::heat_balance::source_output_prescan::SourceOutputPreScan;
use crate::heat_balance::source_output_registration::{
    SourceAirOutputField as Field, SourceOutputDictionaryHost, SourceOutputFileHost,
    SourceOutputRegistration, SourceOutputStoreType, SourceOutputUnits, SourceRealOutputBinding,
    SourceRealOutputDeclaration, SourceRealOutputOwners, SourceRealOutputTarget,
    SourceRealRegistrationOptions, SourceReportVariableInput, TimeStepKind,
};
use crate::psychrometrics::{EnergyPlusPsychrometricFunction as Psy, SourcePsychrometricAccess};
use crate::weather::day::{WeatherEnvironmentState, WeatherGlobalState};
use ep_model::ReportInputs;
use std::io::Write;

pub(crate) struct SourceResilienceOwners<'a, 'backend, H, D> {
    pub(crate) zones: &'a mut SelectedZoneInputOwners,
    pub(crate) globals: &'a SelectedHeatBalanceGlobals,
    pub(crate) control: &'a SelectedHeatBalanceInputState,
    pub(crate) tabular: &'a SourceTabularReports,
    pub(crate) air: &'a SourceAirHeatBalance,
    pub(crate) psychrometrics: &'a mut SourcePsychrometricAccess,
    pub(crate) environment: &'a WeatherEnvironmentState,
    pub(crate) clock: &'a WeatherGlobalState,
    pub(crate) input: &'a mut SelectedInputProcessor,
    pub(crate) registration: &'a mut SourceOutputRegistration,
    pub(crate) prescan: &'a SourceOutputPreScan,
    pub(crate) report_inputs: &'a ReportInputs,
    pub(crate) minimum_report_frequency: &'a str,
    pub(crate) developer: &'a bool,
    pub(crate) host: &'a mut H,
    pub(crate) dictionary_host: &'a mut D,
    pub(crate) eio: &'a mut dyn Write,
    pub(crate) backend: &'a mut SourceErrorBackend<'backend>,
}
pub(crate) struct SourceResilienceOutputOwners<'a> {
    pub(crate) zones: &'a SelectedZoneInputOwners,
}
fn index(number: i32) -> Result<usize, Error> {
    number
        .checked_sub(1)
        .and_then(|n| usize::try_from(n).ok())
        .ok_or(Error::Unavailable(
            "positive actual resilience Zone ordinal",
        ))
}
impl SourceRealOutputOwners for SourceResilienceOutputOwners<'_> {
    fn real_output(&self, target: SourceRealOutputTarget) -> Result<&f64, OrdinaryRadiationError> {
        let SourceRealOutputTarget::Air {
            field,
            index: number,
            space: false,
        } = target
        else {
            return Err(OrdinaryRadiationUnavailable::StorageDomain.into());
        };
        let at = index(number).map_err(|e| OrdinaryRadiationError::Input(Box::new(e)))?;
        let row = self
            .zones
            .resilience
            .as_ref()
            .and_then(|v| v.get(at))
            .ok_or_else(|| {
                OrdinaryRadiationError::Input(Box::new(Error::Unavailable(
                    "same actual allocated ZoneResilience output owner",
                )))
            })?;
        match field {
            Field::ZoneHeatIndex => Ok(&row.zone_heat_index),
            Field::ZoneHumidex => Ok(&row.zone_humidex),
            _ => Err(OrdinaryRadiationUnavailable::StorageDomain.into()),
        }
    }
}
fn register<H: SourceOutputFileHost, D: SourceOutputDictionaryHost>(
    o: &mut SourceResilienceOwners<'_, '_, H, D>,
    number: i32,
    field: Field,
    name: &str,
    units: SourceOutputUnits,
) -> Result<(), Error> {
    let key = o
        .zones
        .zones
        .as_ref()
        .and_then(|v| index(number).ok().and_then(|n| v.get(n)))
        .map(|z| z.name.as_str())
        .ok_or(Error::Unavailable("same actual resilience Zone Name"))?;
    let output_owners = SourceResilienceOutputOwners { zones: o.zones };
    let binding = SourceRealOutputBinding::new(
        SourceRealOutputTarget::Air {
            field,
            index: number,
            space: false,
        },
        &output_owners,
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
                store_type: SourceOutputStoreType::Average,
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
/// Native default-build cache calls, not the legacy stateless RH/dewpoint helpers.
/// Diagnostic-positive RH stops after the real Psat call and algebra, before its
/// unported native error child. EP_psych_stats is outside this selected build.
// Preserve Original ordered bounds; range negation can change floating-point NaN behavior.
#[allow(clippy::manual_range_contains)]
fn relative_humidity(
    psy: &mut SourcePsychrometricAccess,
    tdb: f64,
    dw: f64,
    pb: f64,
) -> Result<f64, Error> {
    let pws = psy
        .evaluate(Psy::Psat, &[tdb])
        .map_err(Error::Unavailable)?
        .result;
    let w = if dw < 1.0e-5 { 1.0e-5 } else { dw };
    let u = w / (0.62198 * pws / (pb - pws));
    let rh = u / (1.0 - (1.0 - u) * (pws / pb));
    if rh < 0.0 || rh > 1.0 {
        if rh < -0.05 || rh > 1.01 {
            return Err(Error::Unavailable(
                "actual PsyRhFnTdbWPb_error1263 diagnostic child",
            ));
        }
        let lower = if rh < 0.01 { 0.01 } else { rh };
        return Ok(if lower < 1.0 { lower } else { 1.0 });
    }
    Ok(rh)
}
fn dew_point(psy: &mut SourcePsychrometricAccess, w: f64, pb: f64) -> Result<f64, Error> {
    let w0 = if w < 1.0e-5 { 1.0e-5 } else { w };
    let pdew = pb * w0 / (0.62198 + w0);
    Ok(psy
        .evaluate(Psy::TsatPb, &[pdew])
        .map_err(Error::Unavailable)?
        .result)
}
impl SourceSurfaceManagement {
    pub(crate) fn calculate_source_thermal_resilience<
        H: SourceOutputFileHost,
        D: SourceOutputDictionaryHost,
    >(
        &mut self,
        o: &mut SourceResilienceOwners<'_, '_, H, D>,
    ) -> Result<(), Error> {
        if self.manage_surface_heat_balance_first_time {
            for number in 1..=o.globals.num_of_zones {
                register(
                    o,
                    number,
                    Field::ZoneHeatIndex,
                    "Zone Heat Index",
                    SourceOutputUnits::C,
                )?;
                register(
                    o,
                    number,
                    Field::ZoneHumidex,
                    "Zone Humidity Index",
                    SourceOutputUnits::Dimensionless,
                )?;
            }
            for req in &o.registration.requested_variables {
                if req.name == "Zone Heat Index" {
                    self.report_var_heat_index = true;
                } else if req.name == "Zone Humidity Index" {
                    self.report_var_humidex = true;
                }
            }
        }
        if self.report_var_heat_index || o.tabular.display_thermal_resilience_summary {
            for number in 1..=o.globals.num_of_zones {
                let at = index(number)?;
                let air = o
                    .air
                    .zones
                    .as_ref()
                    .and_then(|v| v.get(at))
                    .ok_or(Error::Unavailable("actual live resilience Zone Air owner"))?;
                let zone_t = air.air.ztav;
                let zone_w = air.air.air_hum_rat_avg;
                let zone_rh = relative_humidity(
                    o.psychrometrics,
                    zone_t,
                    zone_w,
                    o.environment.current_weather.out_baro_press,
                )? * 100.0;
                let zone_tf = zone_t * (9.0 / 5.0) + 32.0;
                if o.control.heat_index_method == 0 {
                    let mut hi;
                    if zone_tf < 80.0 {
                        hi = 0.5 * (zone_tf + 61.0 + (zone_tf - 68.0) * 1.2 + (zone_rh * 0.094));
                    } else {
                        hi = -42.379
                            + 2.04901523 * zone_tf
                            + 10.14333127 * zone_rh
                            + -0.22475541 * zone_tf * zone_rh
                            + -0.00683783 * zone_tf * zone_tf
                            + -0.05481717 * zone_rh * zone_rh
                            + 0.00122874 * zone_tf * zone_tf * zone_rh
                            + 0.00085282 * zone_tf * zone_rh * zone_rh
                            + -0.00000199 * zone_tf * zone_tf * zone_rh * zone_rh;
                        if zone_rh < 13.0 && zone_tf < 112.0 {
                            hi -= (13.0 - zone_rh) / 4.0
                                * ((17.0 - (zone_tf - 95.0).abs()) / 17.0).sqrt();
                        } else if zone_rh > 85.0 && zone_tf < 87.0 {
                            hi += (zone_rh - 85.0) / 10.0 * (87.0 - zone_tf) / 5.0;
                        }
                    }
                    hi = (hi - 32.0) * (5.0 / 9.0);
                    o.zones
                        .resilience
                        .as_mut()
                        .and_then(|v| v.get_mut(at))
                        .ok_or(Error::Unavailable("actual live ZoneHeatIndex writer"))?
                        .zone_heat_index = hi;
                } else {
                    return Err(Error::Unavailable("actual ExtendedHI::heatindex5784 child"));
                }
            }
        }
        if self.report_var_humidex || o.tabular.display_thermal_resilience_summary {
            for number in 1..=o.globals.num_of_zones {
                let at = index(number)?;
                let air = o
                    .air
                    .zones
                    .as_ref()
                    .and_then(|v| v.get(at))
                    .ok_or(Error::Unavailable("actual live resilience Zone Air owner"))?;
                let zone_w = air.air.air_hum_rat_avg;
                let zone_t = air.air.ztav;
                let t_dew_point_k = dew_point(
                    o.psychrometrics,
                    zone_w,
                    o.environment.current_weather.out_baro_press,
                )? + 273.15;
                let e = 6.11 * (5417.7530 * ((1.0 / 273.16) - (1.0 / t_dew_point_k))).exp();
                let h = 5.0 / 9.0 * (e - 10.0);
                let humidex = zone_t + h;
                o.zones
                    .resilience
                    .as_mut()
                    .and_then(|v| v.get_mut(at))
                    .ok_or(Error::Unavailable("actual live ZoneHumidex writer"))?
                    .zone_humidex = humidex;
            }
        }
        Ok(())
    }
}
