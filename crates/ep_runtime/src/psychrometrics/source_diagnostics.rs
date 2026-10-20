//! Actual backend-aware Psat and dew-point property calls over the sole state.
//! Numeric-only callers retain their established API without a diagnostic claim.
use super::SourcePsychrometricAccess;
use super::psy02_state::{SourcePsatLookup, with_state};
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::heat_balance::selected_simulation_startup::SelectedHvacGlobals;
use crate::heat_balance::source_initialization::SourceRecurringErrorState;
use crate::weather::day::{WeatherEnvironmentState, WeatherGlobalState};

// Exact zero-based PsychCacheData.hh PsychrometricFunction identities.
const W_FROM_DEW_POINT: usize = 4;
const PSAT: usize = 8;

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

#[cfg(test)]
#[path = "source_diagnostics_tests.rs"]
mod tests;
