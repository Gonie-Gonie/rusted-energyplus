//! Actual backend-aware property calls over the sole psychrometric state.
//! Numeric-only callers retain their established API without a diagnostic claim.
use super::SourcePsychrometricAccess;
use super::psy02_state::{
    SourcePsatLookup, SourceTsatEnthalpyLookup, SourceTsatPressureLookup, SourceTwbLookup,
    with_state,
};
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::heat_balance::selected_simulation_startup::SelectedHvacGlobals;
use crate::heat_balance::source_initialization::SourceRecurringErrorState;
use crate::weather::day::{WeatherEnvironmentState, WeatherGlobalState};

// Exact zero-based PsychCacheData.hh PsychrometricFunction identities.
const RELATIVE_HUMIDITY: usize = 1;
const WET_BULB_TEMPERATURE: usize = 2;
const TSAT_FROM_PRESSURE: usize = 10;
const WET_BULB_HUMIDITY: usize = 13;
const WET_BULB_ITERATIONS: usize = 14;
const TSAT_PRESSURE_ITERATIONS: usize = 16;
const W_FROM_DEW_POINT: usize = 4;
const W_FROM_ENTHALPY: usize = 5;
const W_FROM_WET_BULB: usize = 6;
const W_FROM_RELATIVE_HUMIDITY: usize = 7;
const PSAT: usize = 8;
const TSAT_FROM_ENTHALPY: usize = 9;
const W_FROM_WET_BULB_HUMIDITY: usize = 15;

/// Temporary live references to actual phase, timestamp and diagnostic owners.
/// This creates no phase flags, recurrence rows, shadow cache or error backend.
pub(crate) struct SourcePsychrometricOwners<'a, 'backend> {
    pub(crate) clock: &'a WeatherGlobalState,
    pub(crate) globals: &'a SelectedHeatBalanceGlobals,
    pub(crate) environment: &'a WeatherEnvironmentState,
    pub(crate) hvac: &'a SelectedHvacGlobals,
    pub(crate) recurring: &'a mut SourceRecurringErrorState,
    pub(crate) backend: &'a mut SourceErrorBackend<'backend>,
}
fn trimmed(value: f64, precision: usize) -> Result<String, Error> {
    crate::heat_balance::source_round_format::truncate(value, precision)
        .map_err(|error| Error::Format(format!("psychrometric .{precision}T: {error:?}")))
}
fn set_string(value: String) {
    with_state(|state| state.source_string = value);
}
fn string() -> String {
    with_state(|state| state.source_string.clone())
}
fn first(index: usize) -> bool {
    with_state(|state| state.source_error_indices[index] == 0)
}

impl SourcePsychrometricOwners<'_, '_> {
    fn timestamp(&mut self, called_from: &str) -> Result<(), Error> {
        self.backend.show_source_continue_error_time_stamp(
            &format!(
                " Routine={},",
                if called_from.is_empty() {
                    "Unknown"
                } else {
                    called_from
                }
            ),
            self.clock,
            self.globals,
            self.environment,
            self.hvac,
        )
    }
    // Native Psychrometrics.cc606-637. The outer caller deliberately reaches
    // this function at equality; this inner test remains strictly less than.
    fn w_from_enthalpy_error(
        &mut self,
        temperature: f64,
        enthalpy: f64,
        humidity: f64,
        called_from: &str,
    ) -> Result<(), Error> {
        if humidity < -0.0001 && !self.clock.warmup_flag {
            if first(W_FROM_ENTHALPY) {
                set_string(format!(
                    " Dry-Bulb= {} Enthalpy= {}",
                    trimmed(temperature, 2)?,
                    trimmed(enthalpy, 3)?,
                ));
                self.backend
                    .show_warning_message("Calculated Humidity Ratio invalid (PsyWFnTdbH)")?;
                // Unlike Psat, the nonempty caller has no trailing comma.
                let routine = if called_from.is_empty() {
                    " Routine=Unknown,".into()
                } else {
                    format!(" Routine={called_from}")
                };
                self.backend.show_source_continue_error_time_stamp(
                    &routine,
                    self.clock,
                    self.globals,
                    self.environment,
                    self.hvac,
                )?;
                self.backend.show_continue_error(&string())?;
                set_string(format!(
                    "Calculated Humidity Ratio= {}",
                    trimmed(humidity, 4)?,
                ));
                self.backend.show_continue_error(&format!(
                    "{} ... Humidity Ratio set to .00001",
                    string(),
                ))?;
            }
            self.recurring(
                W_FROM_ENTHALPY,
                "Calculated Humidity Ratio invalid (PsyWFnTdbH)",
                humidity,
                "[]",
            )?;
        }
        Ok(())
    }

    // These three Native error children use no comma for a nonempty caller;
    // the empty caller remains the literal " Routine=Unknown,".
    fn timestamp_without_comma(&mut self, called_from: &str) -> Result<(), Error> {
        let routine = if called_from.is_empty() {
            " Routine=Unknown,".into()
        } else {
            format!(" Routine={called_from}")
        };
        self.backend.show_source_continue_error_time_stamp(
            &routine,
            self.clock,
            self.globals,
            self.environment,
            self.hvac,
        )
    }

    // Native Psychrometrics.cc786-818; the original input wet bulb is retained
    // until this complete error child returns, before the outer clamp.
    fn w_from_wet_bulb_temperature_error(
        &mut self,
        temperature: f64,
        wet: f64,
        pressure: f64,
        called_from: &str,
    ) -> Result<(), Error> {
        if wet > temperature + 0.01
            && with_state(|state| state.source_report_errors)
            && !self.clock.warmup_flag
        {
            if first(W_FROM_WET_BULB) {
                set_string(format!(
                    " Dry-Bulb= {} Pressure= {}",
                    trimmed(temperature, 2)?,
                    trimmed(pressure, 2)?,
                ));
                self.backend
                    .show_warning_message("Given Wet Bulb Temperature invalid (PsyWFnTdbTwbPb)")?;
                self.timestamp_without_comma(called_from)?;
                self.backend.show_continue_error(&string())?;
                set_string(format!("Calculated Wet-Bulb= {}", trimmed(wet, 2)?));
                self.backend.show_continue_error(&format!(
                    "{} ... Since Dry Bulb < Wet Bulb, Wet Bulb set = to Dry Bulb",
                    string(),
                ))?;
            }
            self.recurring(
                W_FROM_WET_BULB,
                "Given Wet Bulb Temperature invalid (PsyWFnTdbTwbPb)",
                wet,
                "C",
            )?;
        }
        Ok(())
    }

    // Native Psychrometrics.cc822-857. ReportErrors controls this error child
    // only; the actual nested Psat owner has its own original phase gate.
    fn w_from_wet_bulb_humidity_error(
        &mut self,
        temperature: f64,
        wet: f64,
        pressure: f64,
        humidity: f64,
        called_from: &str,
    ) -> Result<(), Error> {
        if humidity < 0.0
            && with_state(|state| state.source_report_errors)
            && !self.clock.warmup_flag
        {
            if first(W_FROM_WET_BULB_HUMIDITY) {
                set_string(format!(
                    " Dry-Bulb= {} Wet-Bulb= {} Pressure= {}",
                    trimmed(temperature, 2)?,
                    trimmed(wet, 2)?,
                    trimmed(pressure, 2)?,
                ));
                self.backend
                    .show_warning_message("Calculated Humidity Ratio Invalid (PsyWFnTdbTwbPb)")?;
                self.timestamp_without_comma(called_from)?;
                self.backend.show_continue_error(&string())?;
                set_string(format!(
                    "Calculated Humidity Ratio= {}, will recalculate Humidity Ratio",
                    trimmed(humidity, 4)?,
                ));
                self.backend.show_continue_error(&format!(
                    "{} using Relative Humidity .01% (and Dry-Bulb and Pressure as shown)",
                    string(),
                ))?;
            }
            self.recurring(
                W_FROM_WET_BULB_HUMIDITY,
                "Calculated Humidity Ratio Invalid (PsyWFnTdbTwbPb)",
                humidity,
                "[]",
            )?;
        }
        Ok(())
    }

    // Native Psychrometrics.cc1228-1266, including the original differently
    // named recurring message. This child does NOT consult ReportErrors.
    fn w_from_relative_humidity_error(
        &mut self,
        temperature: f64,
        relative: f64,
        pressure: f64,
        humidity: f64,
        called_from: &str,
    ) -> Result<(), Error> {
        if humidity <= -0.0001 && !self.clock.warmup_flag {
            if first(W_FROM_RELATIVE_HUMIDITY) {
                set_string(format!(
                    " Dry-Bulb= {} Relative Humidity [%]= {} Pressure= {}",
                    trimmed(temperature, 2)?,
                    trimmed(relative * 100.0, 2)?,
                    trimmed(pressure, 2)?,
                ));
                self.backend
                    .show_warning_message("Calculated Humidity Ratio is invalid (PsyWFnTdbRhPb)")?;
                self.timestamp_without_comma(called_from)?;
                self.backend.show_continue_error(&string())?;
                set_string(format!(
                    "Calculated Humidity Ratio= {}",
                    trimmed(humidity, 4)?
                ));
                self.backend.show_continue_error(&format!(
                    "{} ... Humidity Ratio set to .00001",
                    string(),
                ))?;
            }
            self.recurring(
                W_FROM_RELATIVE_HUMIDITY,
                "Calculated Humidity Ratio Invalid (PsyWFnTdbTwbPb)",
                humidity,
                "[]",
            )?;
        }
        Ok(())
    }

    fn recurring(
        &mut self,
        kind: usize,
        message: &str,
        value: f64,
        units: &str,
    ) -> Result<(), Error> {
        let mut index = with_state(|state| state.source_error_indices[kind]);
        // This literal backend store has no callback/SQLite/output operation.
        // Publish even a partial index write when its allocation/count stops.
        let result = self.backend.show_source_recurring_warning_at_end(
            message,
            &mut index,
            self.recurring,
            self.clock,
            self.globals,
            Some(value),
            Some(value),
            None,
            units,
            units,
            "",
        );
        with_state(|state| state.source_error_indices[kind] = index);
        result
    }
}

impl SourcePsychrometricAccess {
    /// Actual PsyWFnTdbH, preserving the raw inverse until all diagnostics finish.
    /// The pure numerical API and this backend-aware route share one equation.
    #[track_caller]
    pub(crate) fn source_humidity_ratio_from_enthalpy(
        &mut self,
        temperature: f64,
        enthalpy: f64,
        called_from: &str,
        suppress_warnings: bool,
        owners: &mut SourcePsychrometricOwners<'_, '_>,
    ) -> Result<f64, Error> {
        let humidity = super::raw_psy_w_fn_tdb_h(temperature, enthalpy);
        if humidity <= -0.0001 && !suppress_warnings {
            owners.w_from_enthalpy_error(temperature, enthalpy, humidity, called_from)?;
        }
        Ok(super::finish_psy_w_fn_tdb_h(
            temperature,
            enthalpy,
            humidity,
        ))
    }

    /// Cached Native PsyTsatFnHPb with original-input miss semantics and
    /// genuine range, child-property and conditional convergence diagnostics.
    pub(crate) fn source_saturation_temperature_from_enthalpy(
        &mut self,
        enthalpy: f64,
        pressure: f64,
        called_from: &str,
        developer_flag: &bool,
        owners: &mut SourcePsychrometricOwners<'_, '_>,
    ) -> Result<f64, Error> {
        let index = match with_state(|state| state.begin_source_tsat_enthalpy(enthalpy, pressure)) {
            SourceTsatEnthalpyLookup::Hit(value) => return Ok(value),
            SourceTsatEnthalpyLookup::Miss { index } => index,
        };
        let (shifted_enthalpy, local_enthalpy) = super::tsat_h_pb_enthalpy_values(enthalpy);
        let mut flag_error = false;
        if (shifted_enthalpy <= -4.24e4 || shifted_enthalpy >= 4.5866e7)
            && !owners.clock.warmup_flag
        {
            if first(TSAT_FROM_ENTHALPY) {
                owners
                    .backend
                    .show_warning_message("Enthalpy out of range (PsyTsatFnHPb)")?;
                owners.timestamp(called_from)?;
                // The first warning uses the PRE-CLAMP HH, and the flag is
                // published only after this actual continuation succeeds.
                set_string(format!(
                    " Enthalpy={} Pressure= {}",
                    trimmed(shifted_enthalpy, 5)?,
                    trimmed(pressure, 2)?,
                ));
                owners.backend.show_continue_error(&string())?;
                flag_error = true;
            }
            owners.recurring(
                TSAT_FROM_ENTHALPY,
                "Enthalpy out of range (PsyTsatFnHPb)",
                shifted_enthalpy,
                "J/kg",
            )?;
        }
        let (clamped_enthalpy, seed) = super::tsat_h_pb_polynomial_seed(shifted_enthalpy);
        if flag_error {
            owners.backend.show_continue_error(&format!(
                " Initial Resultant Temperature= {}",
                trimmed(seed, 2)?
            ))?;
        }
        let (temperature, iterations) = super::tsat_h_pb_pressure_correction(
            seed,
            local_enthalpy,
            pressure,
            |temperature, pressure| {
                let humidity = self.source_humidity_ratio_from_wet_bulb(
                    temperature,
                    temperature,
                    pressure,
                    called_from,
                    owners,
                )?;
                Ok::<f64, Error>(super::energyplus_psy_h_fn_tdb_w(temperature, humidity))
            },
        )?;
        // Native retains this unusual first-range-warning-only guard, including
        // a convergence on iteration31. A later range recurrence sets no flag.
        if flag_error && iterations > 30 {
            let flags = crate::heat_balance::error_backend::SourceDiagnosticFlags {
                kickoff: &owners.globals.kickoff_simulation,
                developer: developer_flag,
                warmup: &owners.clock.warmup_flag,
                doing_sizing: &owners.globals.doing_sizing,
            };
            owners
                .backend
                .show_source_severe_error("Temperature did not converge (PsyTsatFnHPb)", &flags)?;
            owners.timestamp_without_comma(called_from)?;
            // Unlike the first range diagnostic, this uses HH after case clamps.
            set_string(format!(
                " Enthalpy={} Pressure= {}",
                trimmed(clamped_enthalpy, 5)?,
                trimmed(pressure, 2)?,
            ));
            owners.backend.show_continue_error(&format!(
                "{} Last T={}",
                string(),
                trimmed(temperature, 2)?
            ))?;
        }
        with_state(|state| state.finish_source_tsat_enthalpy(index, temperature));
        Ok(temperature)
    }

    /// Actual PsyWFnTdbTwbPb over the single Psat cache and live diagnostics.
    /// No result is produced if an earlier Native diagnostic backend fails.
    pub(crate) fn source_humidity_ratio_from_wet_bulb(
        &mut self,
        temperature: f64,
        wet: f64,
        pressure: f64,
        called_from: &str,
        owners: &mut SourcePsychrometricOwners<'_, '_>,
    ) -> Result<f64, Error> {
        let mut wet = wet;
        if wet > temperature {
            if wet > temperature + 0.01 {
                owners.w_from_wet_bulb_temperature_error(
                    temperature,
                    wet,
                    pressure,
                    called_from,
                )?;
            }
            wet = temperature;
        }
        let nested_from = if called_from.is_empty() {
            "PsyWFnTdbTwbPb"
        } else {
            called_from
        };
        let saturation = self.source_saturation_pressure(wet, nested_from, owners)?;
        let humidity = super::raw_psy_w_fn_tdb_twb_pb(temperature, wet, pressure, saturation);
        if humidity < 0.0 {
            owners.w_from_wet_bulb_humidity_error(
                temperature,
                wet,
                pressure,
                humidity,
                called_from,
            )?;
            return self.source_humidity_ratio_from_relative_humidity(
                temperature,
                0.0001,
                pressure,
                called_from,
                owners,
            );
        }
        Ok(humidity)
    }

    // Complete Native PsyWFnTdbRhPb child, also retaining the RH=0.0001
    // fallback's separate nested Psat call/caller name and final floor.
    pub(crate) fn source_humidity_ratio_from_relative_humidity(
        &mut self,
        temperature: f64,
        relative: f64,
        pressure: f64,
        called_from: &str,
        owners: &mut SourcePsychrometricOwners<'_, '_>,
    ) -> Result<f64, Error> {
        let nested_from = if called_from.is_empty() {
            "PsyWFnTdbRhPb"
        } else {
            called_from
        };
        let saturation = self.source_saturation_pressure(temperature, nested_from, owners)?;
        let humidity = super::raw_psy_w_fn_tdb_rh_pb(relative, pressure, saturation);
        if humidity < 1.0e-5 {
            if humidity <= -0.0001 {
                owners.w_from_relative_humidity_error(
                    temperature,
                    relative,
                    pressure,
                    humidity,
                    called_from,
                )?;
            }
            return Ok(1.0e-5);
        }
        Ok(humidity)
    }

    /// Actual stateful PsyRhoAirFnPbTdbW; negative density reaches the genuine
    /// severe/continue/timestamp/fatal backend in Original198-222 order.
    pub(crate) fn source_air_density(
        &mut self,
        pressure: f64,
        temperature: f64,
        humidity: f64,
        called_from: &str,
        developer_flag: &bool,
        owners: &mut SourcePsychrometricOwners<'_, '_>,
    ) -> Result<f64, Error> {
        let density = super::energyplus_psy_rho_air_fn_pb_tdb_w(pressure, temperature, humidity);
        if density < 0.0 {
            let flags = crate::heat_balance::error_backend::SourceDiagnosticFlags {
                kickoff: &owners.globals.kickoff_simulation,
                developer: developer_flag,
                warmup: &owners.clock.warmup_flag,
                doing_sizing: &owners.globals.doing_sizing,
            };
            let round = |value, precision| {
                crate::heat_balance::source_round_format::round(value, precision).map_err(|error| {
                    Error::Format(format!("PsyRhoAirFnPbTdbW .{precision}R: {error:?}"))
                })
            };
            owners.backend.show_source_severe_error(
                &format!(
                    "PsyRhoAirFnPbTdbW: RhoAir (Density of Air) is calculated <= 0 [{}].",
                    round(density, 5)?
                ),
                &flags,
            )?;
            owners.backend.show_continue_error(&format!(
                "pb =[{}], tdb=[{}], w=[{}].",
                round(pressure, 2)?,
                round(temperature, 2)?,
                round(humidity, 7)?
            ))?;
            // The density error's nonempty caller uses NO trailing comma;
            // the empty case is the literal Unknown, spelling in Original218.
            let routine = if called_from.is_empty() {
                " Routine=Unknown,".into()
            } else {
                format!(" Routine={called_from}")
            };
            owners.backend.show_source_continue_error_time_stamp(
                &routine,
                owners.clock,
                owners.globals,
                owners.environment,
                owners.hvac,
            )?;
            return match owners
                .backend
                .show_source_fatal_error("Program terminates due to preceding condition.", &flags)
            {
                Ok(never) => match never {},
                Err(error) => Err(error),
            };
        }
        Ok(density)
    }

    /// Actual PsyRhFnTdbWPb over the same Psat state and diagnostic owners.
    /// Preserve Psat-before-W-floor and the original saturation grouping.
    pub(crate) fn source_relative_humidity(
        &mut self,
        temperature: f64,
        humidity: f64,
        pressure: f64,
        called_from: &str,
        owners: &mut SourcePsychrometricOwners<'_, '_>,
    ) -> Result<f64, Error> {
        let nested_from = if called_from.is_empty() {
            "PsyRhFnTdbWPb"
        } else {
            called_from
        };
        let saturation = self.source_saturation_pressure(temperature, nested_from, owners)?;
        // std::max(dW, floor) retains a NaN first argument. f64::max and
        // range.contains would change that Original comparison behavior.
        let humidity = if humidity < 1.0e-5 { 1.0e-5 } else { humidity };
        let degree = humidity / (0.62198 * saturation / (pressure - saturation));
        let relative = degree / (1.0 - (1.0 - degree) * (saturation / pressure));
        let correction = if relative < 0.0 {
            Some((0.01, relative < -0.05, "1%"))
        } else if relative > 1.0 {
            Some((1.0, relative > 1.01, "100.0%"))
        } else {
            None
        };
        if let Some((bounded, warn, reset)) = correction {
            if warn && !owners.clock.warmup_flag {
                if first(RELATIVE_HUMIDITY) {
                    set_string(format!(
                        " Dry-Bulb= {} Humidity Ratio= {} Calculated Relative Humidity [%]= {}",
                        trimmed(temperature, 2)?,
                        trimmed(humidity, 3)?,
                        trimmed(relative * 100.0, 2)?
                    ));
                    owners.backend.show_warning_message(
                        "Calculated Relative Humidity out of range (PsyRhFnTdbWPb) ",
                    )?;
                    let routine = if called_from.is_empty() {
                        " Routine=Unknown,".into()
                    } else {
                        format!(" Routine={called_from}")
                    };
                    owners.backend.show_source_continue_error_time_stamp(
                        &routine,
                        owners.clock,
                        owners.globals,
                        owners.environment,
                        owners.hvac,
                    )?;
                    owners.backend.show_continue_error(&string())?;
                    owners.backend.show_continue_error(&format!(
                        "Relative Humidity being reset to {reset}"
                    ))?;
                }
                owners.recurring(
                    RELATIVE_HUMIDITY,
                    "Calculated Relative Humidity out of range (PsyRhFnTdbWPb)",
                    relative * 100.0,
                    "%",
                )?;
            }
            return Ok(bounded);
        }
        Ok(relative)
    }

    /// Cached PsyPsatFnTemp with actual miss-local raw diagnostics and effects.
    /// A backend failure retains the already written tag and old numeric value.
    pub(crate) fn source_saturation_pressure(
        &mut self,
        temperature: f64,
        called_from: &str,
        owners: &mut SourcePsychrometricOwners<'_, '_>,
    ) -> Result<f64, Error> {
        let lookup = with_state(|state| state.begin_source_psat(temperature));
        let (index, representative) = match lookup {
            SourcePsatLookup::Hit(value) => return Ok(value),
            SourcePsatLookup::Miss {
                index,
                representative,
            } => (index, representative),
        };
        // Psychrometrics.cc700: raw receives the truncated CACHE representative,
        // and diagnostics occur only on a miss, after the tag write.
        if !owners.clock.warmup_flag && (representative <= -100.0 || representative >= 200.0) {
            if first(PSAT) {
                owners.backend.show_warning_message(
                    "Temperature out of range [-100. to 200.] (PsyPsatFnTemp)",
                )?;
                owners.timestamp(called_from)?;
                owners.backend.show_continue_error(&format!(
                    " Input Temperature={}",
                    trimmed(representative, 2)?
                ))?;
            }
            owners.recurring(
                PSAT,
                "Temperature out of range [-100. to 200.] (PsyPsatFnTemp)",
                representative,
                "C",
            )?;
        }
        let value = super::energyplus_psy_psat_fn_temp_raw(representative);
        with_state(|state| state.finish_source_psat(index, value));
        Ok(value)
    }

    /// Literal PsyWFnTdpPb, never an RH=1 alias or a humidity/pressure floor.
    /// The same actual Psat cache is visited for every whole-degree retry.
    pub(crate) fn source_dew_point_humidity_ratio(
        &mut self,
        temperature: f64,
        pressure: f64,
        called_from: &str,
        owners: &mut SourcePsychrometricOwners<'_, '_>,
    ) -> Result<f64, Error> {
        let nested_from = if called_from.is_empty() {
            "PsyWFnTdpPb"
        } else {
            called_from
        };
        let dew = self.source_saturation_pressure(temperature, nested_from, owners)?;
        let w = dew * 0.62198 / (pressure - dew);
        if w < 0.0 {
            let mut delta_t = 0.0;
            let mut adjusted = dew;
            while adjusted >= pressure {
                delta_t += 1.0;
                adjusted =
                    self.source_saturation_pressure(temperature - delta_t, nested_from, owners)?;
            }
            let w1 = adjusted * 0.62198 / (pressure - adjusted);
            if w <= -0.0001 && !owners.clock.warmup_flag {
                if first(W_FROM_DEW_POINT) {
                    set_string(format!(
                        " Dew-Point= {} Barometric Pressure= {}",
                        trimmed(temperature, 2)?,
                        trimmed(pressure, 2)?
                    ));
                    owners.backend.show_warning_message("Calculated partial vapor pressure is greater than the barometric pressure, so that calculated humidity ratio is invalid (PsyWFnTdpPb).")?;
                    owners.timestamp(called_from)?;
                    owners.backend.show_continue_error(&string())?;
                    // This Source cast occurs only while formatting the actual
                    // first warning; reject its genuinely undefined int domain.
                    if !delta_t.is_finite() || delta_t.trunc() > f64::from(i32::MAX) {
                        return Err(Error::Unavailable(
                            "PsyWFnTdpPb warning DeltaT int conversion",
                        ));
                    }
                    set_string(format!(
                        "Instead, calculated Humidity Ratio at {} ({} degree less) = {}",
                        trimmed(temperature - delta_t, 1)?,
                        delta_t as i32,
                        trimmed(w1, 4)?
                    ));
                    owners.backend.show_continue_error(&format!(
                        "{} will be used. Simulation continues.",
                        string()
                    ))?;
                }
                owners.recurring(
                    W_FROM_DEW_POINT,
                    "Entered Humidity Ratio invalid (PsyWFnTdpPb)",
                    w1,
                    "[]",
                )?;
            }
            return Ok(w1);
        }
        Ok(w)
    }
}

impl SourcePsychrometricOwners<'_, '_> {
    fn recurring_without_values(&mut self, kind: usize, message: &str) -> Result<(), Error> {
        let mut index = with_state(|state| state.source_error_indices[kind]);
        let result = self.backend.show_source_recurring_warning_at_end(
            message,
            &mut index,
            self.recurring,
            self.clock,
            self.globals,
            None,
            None,
            None,
            "",
            "",
            "",
        );
        with_state(|state| state.source_error_indices[kind] = index);
        result
    }
}

impl SourcePsychrometricAccess {
    /// Native unsigned-tag Twb cache: representatives on miss, tags before raw.
    pub(crate) fn source_wet_bulb_temperature(
        &mut self,
        temperature: f64,
        humidity: f64,
        pressure: f64,
        called_from: &str,
        owners: &mut SourcePsychrometricOwners<'_, '_>,
    ) -> Result<f64, Error> {
        let (index, temperature, humidity, pressure) =
            match with_state(|state| state.begin_source_twb(temperature, humidity, pressure)) {
                SourceTwbLookup::Hit(value) => return Ok(value),
                SourceTwbLookup::Miss {
                    index,
                    temperature,
                    humidity,
                    pressure,
                } => (index, temperature, humidity, pressure),
            };
        let value = self.source_wet_bulb_temperature_raw(
            temperature,
            humidity,
            pressure,
            called_from,
            owners,
        )?;
        with_state(|state| state.finish_source_twb(index, value));
        Ok(value)
    }

    /// Native PsyTwbFnTdbWPb_raw: genuine diagnostics interleave with properties.
    pub(super) fn source_wet_bulb_temperature_raw(
        &mut self,
        temperature: f64,
        humidity: f64,
        pressure: f64,
        called_from: &str,
        owners: &mut SourcePsychrometricOwners<'_, '_>,
    ) -> Result<f64, Error> {
        let mut flag_error = false;
        if (temperature <= -100.0 || temperature >= 200.0) && !owners.clock.warmup_flag {
            if first(WET_BULB_TEMPERATURE) {
                owners.backend.show_warning_message(
                    "Temperature out of range [-100. to 200.] (PsyTwbFnTdbWPb)",
                )?;
                owners.timestamp_without_comma(called_from)?;
                owners.backend.show_continue_error(&format!(
                    " Input Temperature={}",
                    trimmed(temperature, 2)?
                ))?;
                flag_error = true;
            }
            owners.recurring(
                WET_BULB_TEMPERATURE,
                "Temperature out of range [-100. to 200.] (PsyTwbFnTdbWPb)",
                temperature,
                "C",
            )?;
        }
        let mut humidity = humidity;
        if humidity < 0.0 {
            if humidity <= -0.0001 && !owners.clock.warmup_flag {
                if first(WET_BULB_HUMIDITY) {
                    set_string(format!(
                        " Dry-Bulb= {} Humidity Ratio= {} Pressure= {}",
                        trimmed(temperature, 2)?,
                        trimmed(humidity, 3)?,
                        trimmed(pressure, 2)?
                    ));
                    owners
                        .backend
                        .show_warning_message("Entered Humidity Ratio invalid (PsyTwbFnTdbWPb)")?;
                    owners.timestamp_without_comma(called_from)?;
                    owners.backend.show_continue_error(&string())?;
                    set_string(format!("Humidity Ratio= {}", trimmed(humidity, 4)?));
                    owners.backend.show_continue_error(&format!(
                        "{} ... Humidity Ratio set to .00001",
                        string()
                    ))?;
                }
                owners.recurring(
                    WET_BULB_HUMIDITY,
                    "Entered Humidity Ratio invalid (PsyTwbFnTdbWPb)",
                    humidity,
                    "[]",
                )?;
            }
            humidity = 1.0e-5;
        }
        let nested_from = if called_from.is_empty() {
            "PsyTwbFnTdbWPb"
        } else {
            called_from
        };
        let boiling = if pressure != with_state(|state| state.source_scalar_state().last_patm) {
            let boiling =
                self.source_saturation_temperature_from_pressure(pressure, nested_from, owners)?;
            with_state(|state| {
                let parameters = state.source_scalar_state();
                parameters.last_patm = pressure;
                parameters.last_t_boil = boiling;
            });
            boiling
        } else {
            with_state(|state| state.source_scalar_state().last_t_boil)
        };
        let (mut wet, mut x1, mut y1) = (temperature, 0.0, 0.0);
        let mut iterations = super::ENERGYPLUS_WET_BULB_MAX_ITERATIONS + 1;
        for iteration in 1..=super::ENERGYPLUS_WET_BULB_MAX_ITERATIONS {
            if wet >= boiling - 0.09 {
                wet = boiling - 0.1;
            }
            let saturation = self.source_saturation_pressure(wet, nested_from, owners)?;
            let star = 0.62198 * saturation / (pressure - saturation);
            let new_humidity = if wet >= 0.0 {
                ((2501.0 - 2.326 * wet) * star - 1.006 * (temperature - wet))
                    / (2501.0 + 1.86 * temperature - 4.186 * wet)
            } else {
                ((2830.0 - 0.24 * wet) * star - 1.006 * (temperature - wet))
                    / (2830.0 + 1.86 * temperature - 2.1 * wet)
            };
            let tolerance = with_state(|state| state.source_scalar_state().iconv_tol);
            let (next, converged) = super::energyplus_general_iterate(
                wet,
                humidity - new_humidity,
                &mut x1,
                &mut y1,
                iteration,
                tolerance,
            );
            wet = next;
            if converged {
                iterations = iteration;
                break;
            }
        }
        if iterations > super::ENERGYPLUS_WET_BULB_MAX_ITERATIONS && !owners.clock.warmup_flag {
            if first(WET_BULB_ITERATIONS) {
                owners.backend.show_warning_message(&format!(
                    "WetBulb not converged after {iterations} iterations(PsyTwbFnTdbWPb)"
                ))?;
                owners.timestamp(called_from)?;
                owners.backend.show_continue_error(&format!(
                    " Input Temperature = {}",
                    trimmed(temperature, 2)?
                ))?;
                owners.backend.show_continue_error(&format!(
                    " Input Humidity Ratio= {}",
                    trimmed(humidity, 6)?
                ))?;
                owners
                    .backend
                    .show_continue_error(&format!(" Input Pressure = {}", trimmed(pressure, 2)?))?;
                flag_error = true;
            }
            owners.recurring_without_values(
                WET_BULB_ITERATIONS,
                "WetBulb not converged after max iterations(PsyTwbFnTdbWPb)",
            )?;
        }
        if flag_error {
            owners
                .backend
                .show_continue_error(&format!(" Resultant Temperature= {}", trimmed(wet, 2)?))?;
        }
        Ok(if wet > temperature { temperature } else { wet })
    }

    /// Native signed-tag cache; raw receives original pressure, not representative.
    pub(crate) fn source_saturation_temperature_from_pressure(
        &mut self,
        pressure: f64,
        called_from: &str,
        owners: &mut SourcePsychrometricOwners<'_, '_>,
    ) -> Result<f64, Error> {
        let index = match with_state(|state| state.begin_source_tsat_pressure(pressure)) {
            SourceTsatPressureLookup::Hit(value) => return Ok(value),
            SourceTsatPressureLookup::Miss { index } => index,
        };
        let value =
            self.source_saturation_temperature_from_pressure_raw(pressure, called_from, owners)?;
        with_state(|state| state.finish_source_tsat_pressure(index, value));
        Ok(value)
    }

    pub(super) fn source_saturation_temperature_from_pressure_raw(
        &mut self,
        pressure: f64,
        called_from: &str,
        owners: &mut SourcePsychrometricOwners<'_, '_>,
    ) -> Result<f64, Error> {
        let mut flag_error = false;
        if !owners.clock.warmup_flag && (pressure <= 0.0017 || pressure >= 1555000.0) {
            if first(TSAT_FROM_PRESSURE) {
                owners
                    .backend
                    .show_warning_message("Pressure out of range (PsyTsatFnPb)")?;
                owners.timestamp_without_comma(called_from)?;
                owners
                    .backend
                    .show_continue_error(&format!(" Input Pressure= {}", trimmed(pressure, 2)?))?;
                flag_error = true;
            }
            owners.recurring(
                TSAT_FROM_PRESSURE,
                "Pressure out of range (PsyTsatFnPb)",
                pressure,
                "Pa",
            )?;
        }
        if pressure == with_state(|state| state.source_scalar_state().press_save) {
            return Ok(with_state(|state| state.source_scalar_state().t_sat_save));
        }
        with_state(|state| state.source_scalar_state().press_save = pressure);
        if with_state(|state| state.source_scalar_state().use_interpolation) {
            return Err(Error::Unavailable(
                "actual Source PsyTsatFnPb interpolation branch",
            ));
        }
        let mut iterations = 0;
        let temperature = if pressure >= 1555000.0 {
            200.0
        } else if pressure <= 0.0017 {
            -100.0
        } else if pressure > 611.0 && pressure < 611.25 {
            0.0
        } else {
            let (mut temperature, mut x1, mut y1) = (100.0, 0.0, 0.0);
            let nested_from = if called_from.is_empty() {
                "PsyTsatFnPb"
            } else {
                called_from
            };
            iterations = super::ENERGYPLUS_TSAT_PRESSURE_MAX_ITERATIONS + 1;
            for iteration in 1..=super::ENERGYPLUS_TSAT_PRESSURE_MAX_ITERATIONS {
                let saturation =
                    self.source_saturation_pressure(temperature, nested_from, owners)?;
                let (next, converged) = super::energyplus_general_iterate(
                    temperature,
                    pressure - saturation,
                    &mut x1,
                    &mut y1,
                    iteration,
                    super::ENERGYPLUS_PSYCHROMETRIC_ITERATION_TOLERANCE,
                );
                temperature = next;
                if converged {
                    iterations = iteration;
                    break;
                }
            }
            temperature
        };
        if iterations > super::ENERGYPLUS_TSAT_PRESSURE_MAX_ITERATIONS && !owners.clock.warmup_flag
        {
            if first(TSAT_PRESSURE_ITERATIONS) {
                owners.backend.show_warning_message(&format!("Saturation Temperature not converged after {iterations} iterations (PsyTsatFnPb)"))?;
                owners.timestamp_without_comma(called_from)?;
                owners
                    .backend
                    .show_continue_error(&format!(" Input Pressure= {}", trimmed(pressure, 2)?))?;
                flag_error = true;
            }
            owners.recurring(
                TSAT_PRESSURE_ITERATIONS,
                "Saturation Temperature not converged after max iterations (PsyTsatFnPb)",
                temperature,
                "C",
            )?;
        }
        with_state(|state| state.source_scalar_state().t_sat_save = temperature);
        if flag_error {
            owners.backend.show_continue_error(&format!(
                " Resultant Temperature= {}",
                trimmed(temperature, 2)?
            ))?;
        }
        Ok(temperature)
    }
}

#[cfg(test)]
#[path = "source_diagnostics_tests.rs"]
mod tests;
