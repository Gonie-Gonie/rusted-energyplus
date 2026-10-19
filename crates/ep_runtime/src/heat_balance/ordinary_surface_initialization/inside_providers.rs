//! Concrete live inside convection and actual guarded temperature diagnostics.
use super::{SourceInsideCtfProviders, SourceSurfaceHeatBalanceArrays};
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::inside_convection_dispatcher::{
    InsideSurfaceTemperatureOwner, source::SourceInsideConvection,
};
use crate::heat_balance::ordinary_air_heat_balance::SourceAirHeatBalance;
use crate::heat_balance::ordinary_equipment::OrdinaryEquipment;
use crate::heat_balance::report_scan::ReportScanState;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::{
    HeatBalanceInputError as Error, SelectedHeatBalanceInputState,
    surface::SurfaceGeometryInputState, zone::SelectedZoneInputOwners,
};
use crate::heat_balance::selected_input_processor::SelectedInputProcessor;
use crate::weather::day::WeatherGlobalState;
use ep_model::ReportInputs;

/// The same controller used by InitSurface473 is borrowed after that call.
/// Constructor limits/clock/globals remain in the caller's actual live owners.
pub(crate) struct SourceOrdinaryInsideCtfHost<'a> {
    pub(crate) convection: &'a mut SourceInsideConvection,
    pub(crate) equipment: &'a mut OrdinaryEquipment,
    pub(crate) constructions: &'a SelectedHeatBalanceInputState,
    pub(crate) input: &'a SelectedInputProcessor,
    pub(crate) reports: &'a ReportInputs,
    pub(crate) scan: &'a mut ReportScanState,
    pub(crate) globals: &'a SelectedHeatBalanceGlobals,
    pub(crate) clock: &'a WeatherGlobalState,
    pub(crate) environment: &'a crate::weather::day::WeatherEnvironmentState,
    pub(crate) hvac: &'a crate::heat_balance::selected_simulation_startup::SelectedHvacGlobals,
    pub(crate) recurring_errors: &'a mut crate::heat_balance::source_initialization::SourceRecurringErrorState,
}
fn index(number: i32) -> Result<usize, Error> {
    number
        .checked_sub(1)
        .and_then(|n| usize::try_from(n).ok())
        .ok_or(Error::Unavailable(
            "actual Source inside diagnostic ordinal",
        ))
}
impl SourceInsideCtfProviders for SourceOrdinaryInsideCtfHost<'_> {
    fn initialize_inside_convection(
        &mut self,
        geometry: &mut SurfaceGeometryInputState,
        arrays: &mut SourceSurfaceHeatBalanceArrays,
        zones: &SelectedZoneInputOwners,
        air: &SourceAirHeatBalance,
        globals: &SelectedHeatBalanceGlobals,
        clock: &WeatherGlobalState,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        self.convection.initialize(
            InsideSurfaceTemperatureOwner::SurfTempIn,
            geometry,
            arrays,
            zones,
            air,
            self.constructions,
            globals,
            clock,
            self.equipment,
            self.input,
            self.reports,
            self.scan,
            backend,
        )
    }

    fn test_surface_temperature(
        &mut self,
        temperature: f64,
        surface: i32,
        zone: i32,
        geometry: &SurfaceGeometryInputState,
        zones: &mut SelectedZoneInputOwners,
        warmup_error_count: &mut i32,
        _backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        // Actual9289 makes a local copy before both temperature predicates.
        let _surface_name = geometry
            .final_surfaces
            .as_deref()
            .and_then(|rows| index(surface).ok().and_then(|i| rows.get(i)))
            .ok_or(Error::Unavailable("actual diagnostic Surface Name"))?
            .row
            .name
            .clone();
        let _zone = zones
            .zones
            .as_mut()
            .and_then(|rows| index(zone).ok().and_then(|i| rows.get_mut(i)))
            .ok_or(Error::Unavailable("actual diagnostic Zone reference"))?;
        // Source takes WarmupSurfTemp by value; increment never writes the caller.
        let mut local_warmup_count = *warmup_error_count;
        if temperature > self.constructions.max_surface_temperature_limit || temperature < -100.0 {
            if self.clock.warmup_flag {
                local_warmup_count = local_warmup_count.checked_add(1).ok_or(
                    Error::Unavailable("undefined Source local WarmupSurfTemp increment"),
                )?;
            }
            if !self.clock.warmup_flag
                || local_warmup_count > 10
                || self.globals.display_extra_warnings
            {
                return Err(Error::Unavailable(
                    "actual positive TestSurfTemp diagnostic Surface counters/recurring child9296",
                ));
            }
        }
        if temperature > self.constructions.max_surface_temperature_before_fatal
            || temperature < -250.0
        {
            if !self.clock.warmup_flag {
                return Err(Error::Unavailable(
                    "actual positive TestSurfTemp fatal diagnostic9420",
                ));
            } else if temperature < -10000.0 || temperature > 10000.0 {
                return Err(Error::Unavailable(
                    "actual extreme warmup TestSurfTemp diagnostic9459",
                ));
            }
        }
        Ok(())
    }

    fn warn_nonconvergence(
        &mut self,
        maximum_difference: f64,
        maximum_allowed_cond_fd: f64,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        source_inside_nonconvergence_warning(maximum_difference, maximum_allowed_cond_fd,
            self.clock, self.globals, self.environment, self.hvac, backend)
    }

    fn recurring_nonconvergence(
        &mut self,
        maximum_difference: f64,
        error_pointer: &mut i32,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        source_inside_nonconvergence_recurring(maximum_difference, error_pointer,
            self.recurring_errors, self.clock, self.globals, backend)
    }
}

/// Shared actual production warning/formatter/timestamp operations. No full
/// convection host is constructed by the specified suffix unit.
pub(crate) fn source_inside_nonconvergence_warning(
    maximum_difference: f64,
    maximum_allowed_cond_fd: f64,
    clock: &WeatherGlobalState,
    globals: &SelectedHeatBalanceGlobals,
    environment: &crate::weather::day::WeatherEnvironmentState,
    hvac: &crate::heat_balance::selected_simulation_startup::SelectedHvacGlobals,
    backend: &mut SourceErrorBackend<'_>,
) -> Result<(), Error> {
    let difference = crate::heat_balance::source_round_format::round(maximum_difference, 3)
        .map_err(|error| {
            Error::Format(format!(
                "inside maximum difference .3R unavailable: {error:?}"
            ))
        })?;
    let allowed = crate::heat_balance::source_round_format::round(maximum_allowed_cond_fd, 6)
        .map_err(|error| {
            Error::Format(format!(
                "inside allowed difference .6R unavailable: {error:?}"
            ))
        })?;
    backend.show_inside_warning_error(&format!(
            "Inside surface heat balance did not converge with Max Temp Difference [C] ={difference} vs Max Allowed Temp Diff [C] ={allowed}"
        ), clock, globals)?;
    backend.show_source_continue_error_time_stamp("", clock, globals, environment, hvac)
}
/// Same original recurring message/optional operands; shared by the live host
/// and the explicitly selected suffix unit, on their real persistent rows.
pub(crate) fn source_inside_nonconvergence_recurring(
    maximum_difference: f64,
    error_pointer: &mut i32,
    recurring_errors: &mut crate::heat_balance::source_initialization::SourceRecurringErrorState,
    clock: &WeatherGlobalState,
    globals: &SelectedHeatBalanceGlobals,
    backend: &mut SourceErrorBackend<'_>,
) -> Result<(), Error> {
    backend.show_source_recurring_warning_at_end(
        "Inside surface heat balance convergence problem continues",
        error_pointer,
        recurring_errors,
        clock,
        globals,
        Some(maximum_difference),
        Some(maximum_difference),
        None,
        "[C]",
        "[C]",
        "",
    )
}
