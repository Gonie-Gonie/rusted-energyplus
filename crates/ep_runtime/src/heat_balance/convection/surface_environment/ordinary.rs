//! Actual retained surface rows and the bounded ordinary A manager call.
//! No Native guard text comparison or complete surface-manager port is claimed.
use super::{
    SurfaceOutdoorEnvironmentParameters, SurfaceOutdoorEnvironmentRow, SurfaceOutdoorWeather,
};
use crate::error::RuntimeError;
use crate::heat_balance::ctf_initialization_context::CtfInitializationContext;
use crate::heat_balance::error_backend::{
    SourceErrorBackend, SourceErrorBackendFailure, SourceFatalStop,
};
use crate::heat_balance::state::{HeatBalanceState, SurfaceHeatBalanceState};
use crate::heat_balance::weather_driver::HeatBalanceWeatherDriver;

mod source;
use crate::weather::{HeatBalanceWeatherContext, day::WeatherDayPhase};
use ep_model::{SimulationModel, WindExposure};
use std::fmt::{Display, Formatter};

/// Real formatting/backend/fatal cause; an unavailable prerequisite is separate.
#[derive(Debug)]
pub enum SurfaceOutdoorEnvironmentError {
    /// Actual ordinary owner or admitted caller is absent or changed.
    Unavailable(&'static str),
    /// Selected .0R producer has no defined result for this actual height.
    Format(String),
    /// Real error sink/database/counter failure before the fatal boundary.
    Backend(SourceErrorBackendFailure),
    /// Reached actual fatal effects or a failure during those effects.
    Fatal(SourceFatalStop),
}
impl Display for SurfaceOutdoorEnvironmentError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unavailable(reason) => f.write_str(reason),
            Self::Format(reason) => f.write_str(reason),
            Self::Backend(error) => Display::fmt(error, f),
            Self::Fatal(error) => Display::fmt(error, f),
        }
    }
}
impl std::error::Error for SurfaceOutdoorEnvironmentError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Backend(error) => Some(error),
            Self::Fatal(error) => Some(error),
            _ => None,
        }
    }
}
impl PartialEq for SurfaceOutdoorEnvironmentError {
    fn eq(&self, other: &Self) -> bool {
        // RuntimeError's existing equality is diagnostic; retained errors remain
        // the original values and Error::source exposes each actual IO cause.
        match (self, other) {
            (Self::Unavailable(a), Self::Unavailable(b)) => a == b,
            (Self::Format(a), Self::Format(b)) => a == b,
            (Self::Backend(a), Self::Backend(b)) => backend_equal(a, b),
            (
                Self::Fatal(SourceFatalStop::Backend(a)),
                Self::Fatal(SourceFatalStop::Backend(b)),
            ) => backend_equal(a, b),
            (Self::Fatal(SourceFatalStop::Fatal(a)), Self::Fatal(SourceFatalStop::Fatal(b))) => {
                a == b
            }
            _ => false,
        }
    }
}
fn backend_equal(a: &SourceErrorBackendFailure, b: &SourceErrorBackendFailure) -> bool {
    match (a, b) {
        (
            SourceErrorBackendFailure::Effect {
                operation: a,
                source: x,
            },
            SourceErrorBackendFailure::Effect {
                operation: b,
                source: y,
            },
        ) => {
            a == b
                && x.kind() == y.kind()
                && x.raw_os_error() == y.raw_os_error()
                && x.to_string() == y.to_string()
        }
        (
            SourceErrorBackendFailure::UndefinedCounterIncrement { counter: a },
            SourceErrorBackendFailure::UndefinedCounterIncrement { counter: b },
        ) => a == b,
        _ => false,
    }
}
impl From<SourceErrorBackendFailure> for SurfaceOutdoorEnvironmentError {
    fn from(error: SourceErrorBackendFailure) -> Self {
        Self::Backend(error)
    }
}
impl From<SurfaceOutdoorEnvironmentError> for RuntimeError {
    fn from(error: SurfaceOutdoorEnvironmentError) -> Self {
        Self::SurfaceOutdoorEnvironment(Box::new(error))
    }
}

/// Four actual allocated owners in the same actual Surface's ID/order carrier.
/// The marker distinguishes this manager call from an untouched allocation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SurfaceOutdoorEnvironmentState {
    /// Retained values plus the actual stored height/exposure read by setters.
    pub row: SurfaceOutdoorEnvironmentRow,
    /// Rust availability marker; not an invented Original global flag.
    pub manager_invoked: bool,
}
impl SurfaceOutdoorEnvironmentState {
    pub(crate) fn allocate(centroid_height_m: f64, wind_exposed: bool) -> Self {
        Self {
            row: SurfaceOutdoorEnvironmentRow {
                centroid_height_m,
                wind_exposed,
                dry_bulb_c: 0.0,
                wet_bulb_c: 0.0,
                wind_speed_m_per_s: 0.0,
                wind_direction_deg: 0.0,
            },
            manager_invoked: false,
        }
    }
    /// A consumer copies the retained values only after the real manager call.
    pub(crate) fn current(self) -> Option<SurfaceOutdoorWeather> {
        self.manager_invoked.then_some(SurfaceOutdoorWeather {
            dry_bulb_c: self.row.dry_bulb_c,
            wet_bulb_c: self.row.wet_bulb_c,
            wind_speed_m_per_s: self.row.wind_speed_m_per_s,
            wind_direction_deg: self.row.wind_direction_deg,
        })
    }
}

// Literal ObjexxFCL min(double,double,double), including NaNs and tie direction.
fn minimum_three(a: f64, b: f64, c: f64) -> f64 {
    if a < b {
        if a < c { a } else { c }
    } else if b < c {
        b
    } else {
        c
    }
}

/// CheckSurfaceOutBulbTempAt, with cumulative min and the actual current name.
/// This does not invoke either wind setter or reset any of the four owners.
pub fn check_surface_out_bulb_temperatures<'a>(
    rows: impl IntoIterator<Item = (&'a SurfaceOutdoorEnvironmentRow, &'a str)>,
    backend: &mut SourceErrorBackend<'_>,
) -> Result<(), SurfaceOutdoorEnvironmentError> {
    check_out_bulb_temperatures(
        "Surface",
        rows.into_iter()
            .map(|(row, name)| (row.dry_bulb_c, row.wet_bulb_c, row.centroid_height_m, name)),
        backend,
    )
}

// Both Original bulk guards invoke the same DataEnvironment error routine.
fn check_out_bulb_temperatures<'a>(
    kind: &str,
    rows: impl IntoIterator<Item = (f64, f64, f64, &'a str)>,
    backend: &mut SourceErrorBackend<'_>,
) -> Result<(), SurfaceOutdoorEnvironmentError> {
    let mut min_bulb = 0.0;
    for (dry_bulb, wet_bulb, centroid_height, name) in rows {
        min_bulb = minimum_three(min_bulb, dry_bulb, wet_bulb);
        if min_bulb < -100.0 {
            backend.show_severe_error(&format!(
                "SetOutBulbTempAt: {kind} Outdoor Temperatures < -100 C"
            ))?;
            // Source formats this only after ShowSevereError has returned.
            let height = crate::heat_balance::source_round_format::round(centroid_height, 0)
                .map_err(|error| {
                    SurfaceOutdoorEnvironmentError::Format(format!(
                        "{} height .0R unavailable: {error:?}",
                        kind.to_ascii_lowercase()
                    ))
                })?;
            backend.show_continue_error(&format!(
                "...check {kind} Heights - Maximum {kind} Height=[{height}]."
            ))?;
            if centroid_height >= 20000.0 {
                backend.show_continue_error("...according to your maximum Z height, your building is somewhere in the Stratosphere.")?;
                backend.show_continue_error(&format!("...look at {kind} Name= {name}"))?;
            }
            return match backend
                .show_fatal_error("Program terminates due to preceding condition(s).")
            {
                Ok(never) => match never {},
                Err(error) => Err(SurfaceOutdoorEnvironmentError::Fatal(error)),
            };
        }
    }
    Ok(())
}

fn unavailable(reason: &'static str) -> RuntimeError {
    SurfaceOutdoorEnvironmentError::Unavailable(reason).into()
}

/// Bounded ordinary A invocation. Missing actual weather/backend stops before
/// any writes; changed Surface ownership is not repaired by an ID fallback.
pub(crate) fn invoke(
    model: &SimulationModel,
    state: &mut HeatBalanceState,
    weather: Option<HeatBalanceWeatherContext<'_>>,
    output: Option<&mut CtfInitializationContext<'_>>,
) -> Result<(), RuntimeError> {
    let actual = weather
        .and_then(|value| value.owned)
        .ok_or_else(|| unavailable("surface weather requires actual current production weather"))?;
    let clock = actual
        .ctf_history_clock
        .ok_or_else(|| unavailable("surface weather requires actual caller WarmupFlag"))?;
    let output = output
        .ok_or_else(|| unavailable("surface weather requires retained actual error backend"))?;
    let terrain = model
        .typed
        .building
        .as_ref()
        .ok_or_else(|| unavailable("surface weather requires actual compiled Building terrain"))?
        .terrain;
    validate_surfaces(model, state)?;
    output.backend.state.warmup_flag = matches!(clock.phase, WeatherDayPhase::Warmup { .. });
    let parameters = SurfaceOutdoorEnvironmentParameters::default_weather_station(terrain);
    let weather = SurfaceOutdoorWeather {
        dry_bulb_c: actual.current_weather.out_dry_bulb_temp,
        wet_bulb_c: actual.current_weather.out_wet_bulb_temp,
        wind_speed_m_per_s: actual.current_weather.wind_speed,
        wind_direction_deg: actual.current_weather.wind_dir,
    };
    apply_surfaces(
        &mut state.surfaces,
        parameters,
        weather,
        &mut output.backend,
    )
}

fn validate_surfaces(
    model: &SimulationModel,
    state: &HeatBalanceState,
) -> Result<(), RuntimeError> {
    if state.surfaces.len() != model.typed.surfaces.len()
        || state
            .surfaces
            .iter()
            .zip(&model.typed.surfaces)
            .any(|(actual, declared)| {
                actual.surface_id != declared.id
                    || actual.surface_name != declared.name.0
                    || actual.outdoor_environment.row.centroid_height_m.to_bits()
                        != actual.geometry.centroid_m.z_m.to_bits()
                    || actual.outdoor_environment.row.wind_exposed
                        != (declared.wind_exposure == WindExposure::WindExposed)
            })
        || state
            .surfaces
            .iter()
            .map(|surface| surface.surface_id)
            .collect::<std::collections::BTreeSet<_>>()
            .len()
            != state.surfaces.len()
    {
        return Err(unavailable(
            "surface weather ID/order/height/exposure owner changed",
        ));
    }
    Ok(())
}

fn apply_surfaces(
    surfaces: &mut [SurfaceHeatBalanceState],
    parameters: SurfaceOutdoorEnvironmentParameters,
    weather: SurfaceOutdoorWeather,
    backend: &mut SourceErrorBackend<'_>,
) -> Result<(), RuntimeError> {
    for surface in surfaces.iter_mut() {
        surface.outdoor_environment.manager_invoked = true;
    }
    super::set_surface_out_bulb_temperature_rows(
        parameters,
        weather,
        surfaces
            .iter_mut()
            .map(|surface| &mut surface.outdoor_environment.row),
    );
    check_surface_out_bulb_temperatures(
        surfaces.iter().map(|surface| {
            (
                &surface.outdoor_environment.row,
                surface.surface_name.as_str(),
            )
        }),
        backend,
    )?;
    super::set_surface_wind_speed_rows(
        parameters,
        weather.wind_speed_m_per_s,
        surfaces
            .iter_mut()
            .map(|surface| &mut surface.outdoor_environment.row),
    );
    super::set_surface_wind_direction_rows(
        weather.wind_direction_deg,
        surfaces
            .iter_mut()
            .map(|surface| &mut surface.outdoor_environment.row),
    );
    Ok(())
}

/// Rust failure custody for a moved Source-connected State. This is not an
/// Original readiness flag or a second live simulation owner.
pub(crate) fn retain_failed_state(
    state: HeatBalanceState,
    context: Option<&mut CtfInitializationContext<'_>>,
) {
    if (state.selected_zone_input.is_some() || state.selected_heat_balance_globals.is_some())
        && let Some(context) = context
    {
        context.reports.failed_heat_balance_state = Some(Box::new(state));
    }
}

/// Source-connected callers borrow the existing production session. Legacy
/// callers without either Source owner retain the surface-only invocation.
pub(crate) fn invoke_from_driver(
    model: &SimulationModel,
    state: &mut HeatBalanceState,
    weather: Option<HeatBalanceWeatherContext<'_>>,
    driver: Option<HeatBalanceWeatherDriver<'_>>,
    output: Option<&mut CtfInitializationContext<'_>>,
) -> Result<(), RuntimeError> {
    match (
        state.selected_zone_input.is_some(),
        state.selected_heat_balance_globals.is_some(),
    ) {
        (false, false) => invoke(model, state, weather, output),
        (true, true) => source::invoke(model, state, weather, driver, output),
        _ => Err(unavailable("partial Source Zone/global outdoor ownership")),
    }
}

pub(crate) fn invoke_if_active(
    model: &SimulationModel,
    state: &mut HeatBalanceState,
    weather: Option<HeatBalanceWeatherContext<'_>>,
    driver: Option<HeatBalanceWeatherDriver<'_>>,
    output: Option<&mut CtfInitializationContext<'_>>,
) -> Result<(), RuntimeError> {
    if state
        .surfaces
        .iter()
        .any(|surface| surface.outdoor_environment.manager_invoked)
    {
        if state
            .surfaces
            .iter()
            .any(|surface| !surface.outdoor_environment.manager_invoked)
        {
            return Err(unavailable("partial ordinary surface weather activation"));
        }
        invoke_from_driver(model, state, weather, driver, output)?;
    }
    Ok(())
}

#[cfg(test)]
#[path = "ordinary_tests.rs"]
mod tests;
