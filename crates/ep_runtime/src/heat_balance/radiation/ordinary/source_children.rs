//! Concrete Solar children borrowing the real caller's stream/output/Dayltg owners.
use super::OrdinarySolarRadiation;
use super::shading_report::{SourceShadingStream, write_source_shadow_combinations};
use super::solar_output_registration::{
    SourceSolarOutputOwners, SourceSolarOutputRegistrationContext,
    register_source_sky_diffuse_debug_outputs, register_source_solar_module_outputs,
};
use super::solar_timestep::SourceSolarChildren;
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::ordinary_daylighting::SourceDaylighting;
use crate::heat_balance::selected_gain_modules::SourceGainModules;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::{
    HeatBalanceInputError as Error, SelectedHeatBalanceInputState, shadowing::SourceShadowingInput,
    surface::SurfaceGeometryInputState, zone::SelectedZoneInputOwners,
};
use crate::heat_balance::selected_input_processor::{
    SelectedInputProcessor, SourceInputProcessorShortCut,
};
use crate::heat_balance::source_output_prescan::SourceOutputPreScan;
use crate::heat_balance::source_output_registration::{
    SourceOutputDictionaryHost, SourceOutputFileHost, SourceOutputRegistration,
    SourceReportVariableInput,
};
use crate::weather::day::{WeatherEnvironmentState, WeatherGlobalState};
use ep_model::ReportInputs;
use std::io::Write;
use std::path::Path;

/// Selected sole DataReportingFlags.hh63-67 members with genuine native defaults.
/// The caller retains this owner across Solar and the later progress consumer.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceSolarReportFlags {
    /// Actual ReportFlagData NumOfWarmupDays constructor0, reset per environment419.
    pub(crate) number_of_warmup_days: i32,
    /// Actual ReportFlagData cWarmupDay constructor-empty, producer Simulation457.
    pub(crate) warmup_day: String,
    pub(crate) display_performance_simulation_flag: bool,
    pub(crate) print_end_data_dictionary: bool,
    /// Actual DoWeatherInitReporting constructor false, written at SimulationManager204.
    pub(crate) do_weather_init_reporting: bool,
}

impl SourceSolarReportFlags {
    /// Actual SimulationManager419 resets only this integer, leaving cWarmupDay retained.
    pub(crate) fn reset_source_warmup_day_count(&mut self) {
        self.number_of_warmup_days = 0;
    }
    /// Actual SimulationManager456--457; the real WarmupFlag parent owns its guard.
    pub(crate) fn advance_source_warmup_day(&mut self) -> Result<(), Error> {
        let next = self
            .number_of_warmup_days
            .checked_add(1)
            .ok_or(Error::Unavailable(
                "actual NumOfWarmupDays signed-int increment outside defined Native domain456",
            ))?;
        self.number_of_warmup_days = next;
        self.warmup_day = self.number_of_warmup_days.to_string();
        Ok(())
    }
}

/// References only; each real operation acts on its original persistent owner.
pub(crate) struct SourceSolarChildOwners<'a, H, D> {
    pub(crate) shading_stream: &'a mut SourceShadingStream,
    pub(crate) shading_path: &'a Path,
    pub(crate) report_flags: &'a mut SourceSolarReportFlags,
    pub(crate) globals: &'a SelectedHeatBalanceGlobals,
    pub(crate) clock: &'a WeatherGlobalState,
    pub(crate) control: &'a SelectedHeatBalanceInputState,
    pub(crate) zones: &'a SelectedZoneInputOwners,
    #[allow(
        dead_code,
        reason = "The InitSolar child view retains its actual ShadowCalculation owner borrow; the selected child paths do not read this member directly."
    )]
    pub(crate) shadowing: &'a SourceShadowingInput,
    pub(crate) daylighting: &'a mut SourceDaylighting,
    pub(crate) gain_modules: &'a SourceGainModules,
    pub(crate) input: &'a mut SelectedInputProcessor,
    pub(crate) shortcut: &'a mut SourceInputProcessorShortCut,
    pub(crate) report_inputs: &'a ReportInputs,
    pub(crate) minimum_report_frequency: &'a str,
    pub(crate) registration: &'a mut SourceOutputRegistration,
    pub(crate) prescan: &'a SourceOutputPreScan,
    pub(crate) output_host: &'a mut H,
    pub(crate) dictionary_host: &'a mut D,
    pub(crate) eio: &'a mut dyn Write,
}

impl<H: SourceOutputFileHost, D: SourceOutputDictionaryHost> SourceSolarChildren
    for SourceSolarChildOwners<'_, H, D>
{
    fn open_shading_stream(
        &mut self,
        output_shd: bool,
        _backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        self.shading_stream.open(output_shd, self.shading_path);
        Ok(())
    }

    fn write_shadow_combinations(
        &mut self,
        solar: &mut OrdinarySolarRadiation,
        geometry: &SurfaceGeometryInputState,
        control: &SelectedHeatBalanceInputState,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        write_source_shadow_combinations(
            self.shading_stream,
            solar,
            geometry,
            control,
            self.globals,
            backend,
        )
    }

    fn reset_shading_stream(&mut self) {
        self.shading_stream.reset();
    }

    fn display_shadowing_update(
        &mut self,
        clock: &WeatherGlobalState,
        environment: &WeatherEnvironmentState,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        // DataGlobalConstants.hh566: actual RunPeriodWeather=3.
        let date = if clock.kind_of_sim == 3 {
            &environment.current_month_day_year
        } else {
            &environment.current_month_day
        };
        backend.display_string(&format!(
            "Updating Shadowing Calculations, Start Date={date}"
        ))?;
        // SolarShading9028 is after the reached display call, never before it.
        self.report_flags.display_performance_simulation_flag = true;
        Ok(())
    }

    fn set_display_performance_simulation_flag(&mut self) -> Result<(), Error> {
        self.report_flags.display_performance_simulation_flag = true;
        Ok(())
    }

    fn register_solar_module_outputs(
        &mut self,
        solar: &OrdinarySolarRadiation,
        geometry: &SurfaceGeometryInputState,
        zones: &SelectedZoneInputOwners,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        let live = SourceSolarOutputOwners {
            radiation: solar,
            geometry,
        };
        let mut requests = SourceReportVariableInput::new(
            self.report_inputs,
            self.input,
            self.minimum_report_frequency,
        );
        let mut output = SourceSolarOutputRegistrationContext {
            registration: self.registration,
            input: &mut requests,
            prescan: self.prescan,
            minutes_in_time_step: self.clock.minutes_in_time_step,
            host: self.output_host,
            eio: self.eio,
            dictionary_host: self.dictionary_host,
        };
        register_source_solar_module_outputs(&live, zones, self.globals, &mut output, backend)
    }

    fn register_sky_diffuse_outputs(
        &mut self,
        solar: &OrdinarySolarRadiation,
        geometry: &SurfaceGeometryInputState,
        detailed_shading: bool,
    ) -> Result<(), Error> {
        let live = SourceSolarOutputOwners {
            radiation: solar,
            geometry,
        };
        let mut requests = SourceReportVariableInput::new(
            self.report_inputs,
            self.input,
            self.minimum_report_frequency,
        );
        let mut output = SourceSolarOutputRegistrationContext {
            registration: self.registration,
            input: &mut requests,
            prescan: self.prescan,
            minutes_in_time_step: self.clock.minutes_in_time_step,
            host: self.output_host,
            eio: self.eio,
            dictionary_host: self.dictionary_host,
        };
        register_source_sky_diffuse_debug_outputs(&live, detailed_shading, &mut output)
    }

    fn calc_daylight_coefficients(
        &mut self,
        solar: &mut OrdinarySolarRadiation,
        geometry: &mut SurfaceGeometryInputState,
        _clock: &WeatherGlobalState,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        self.daylighting.calculate_daylighting_coefficients(
            self.input,
            self.shortcut,
            self.zones,
            solar,
            geometry,
            self.control,
            self.gain_modules,
            backend,
        )
    }
}
