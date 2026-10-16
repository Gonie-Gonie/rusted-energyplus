//! Exact initDaylighting/manageDaylighting empty-control traversal and real resets.
use super::*;
impl SourceDaylighting {
    pub(crate) fn initialize_daylighting(
        &mut self,
        first_time: bool,
        geometry: &mut SurfaceGeometryInputState,
        zones: &SelectedZoneInputOwners,
        radiation: &mut OrdinarySolarRadiation,
        gains: &mut SelectedInternalHeatGains,
        modules: &SourceGainModules,
        globals: &SelectedHeatBalanceGlobals,
        clock: &WeatherGlobalState,
        environment: &WeatherEnvironmentState,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        for &number in &geometry.surface_lists.all_exterior_solar_window {
            let window = geometry
                .surface_windows
                .as_mut()
                .and_then(|rows| rows.get_mut(index(number).ok()?))
                .ok_or(Error::Unavailable(
                    "same Source SurfaceWindow at initDaylighting",
                ))?;
            if !empty(&window.reference_points) {
                return Err(Error::Unavailable(
                    "initDaylighting actual positive SurfaceWindowRefPt reset5635",
                ));
            }
        }
        gains.reset_source_space_power_reduction_factors(globals.num_spaces)?;
        if !empty(&self.daylight_controls) {
            return Err(Error::Unavailable(
                "initDaylighting actual positive control PowerReductionFactor5645",
            ));
        }
        if environment.current_weather.sun_is_up {
            for number in 1..=radiation.num_solar_enclosures() {
                let enclosure = solar(radiation, number)?;
                if enclosure.total_enclosure_daylight_reference_points == 0
                    || enclosure.has_interzone_window
                {
                    continue;
                }
                return Err(Error::Unavailable(
                    "initDaylighting actual positive enclosure daylight window report5674",
                ));
            }
        }
        if environment.current_weather.sun_is_up && !empty(&modules.tdd_pipe) {
            if first_time {
                backend
                    .display_string("Computing Interior Daylighting Illumination for TDD pipes")?;
            }
            return Err(Error::Unavailable(
                "initDaylighting actual DayltgInteriorTDDIllum5710",
            ));
        }
        if !empty(&self.daylight_controls) {
            return Err(Error::Unavailable(
                "initDaylighting actual positive DElight control loop5713",
            ));
        }
        if environment.current_weather.sun_is_up && !globals.doing_sizing {
            self.interior_map_illuminance(clock)?;
        }
        let spaces = zones.spaces.as_deref().ok_or(Error::Unavailable(
            "same Space allocation in daylight window reset",
        ))?;
        let zone_rows = zones.zones.as_deref().ok_or(Error::Unavailable(
            "same Zone allocation in daylight window reset",
        ))?;
        for zone_number in 1..=globals.num_of_zones {
            for &space in &zone_rows
                .get(index(zone_number)?)
                .ok_or(Error::Unavailable("Source Dayltg Zone ordinal"))?
                .space_indexes
            {
                let row = spaces
                    .get(index(space)?)
                    .ok_or(Error::Unavailable("Source Dayltg Space ordinal"))?;
                for number in row.window_first..=row.window_last {
                    let i = index(number)?;
                    let windows = geometry
                        .window_arrays
                        .as_mut()
                        .ok_or(Error::Unavailable("actual Source window shading arrays"))?;
                    let report = windows
                        .surf_win_frac_time_shading_device_on
                        .get_mut(i)
                        .ok_or(Error::Unavailable("actual window shading report ordinal"))?;
                    *report = 0.0;
                    let flag = *windows
                        .surf_win_shading_flag
                        .get(i)
                        .ok_or(Error::Unavailable("actual Source WinShadingType"))?;
                    // Native IS_SHADED == !(NoShade0 || ShadeOff1), including Invalid.
                    *report = if flag != 0 && flag != 1 { 1.0 } else { 0.0 };
                }
            }
        }
        Ok(())
    }
    fn interior_map_illuminance(&mut self, clock: &WeatherGlobalState) -> Result<(), Error> {
        if clock.warmup_flag {
            return Ok(());
        }
        // Genuine MaxMapRefPoints2500 allocation without element writes.
        let _daylight_illum = vec![None::<f64>; 2500];
        if !empty(&self.illuminance_maps) {
            return Err(Error::Unavailable(
                "DayltgInteriorMapIllum actual positive map enclosure8957",
            ));
        }
        Ok(())
    }
    pub(crate) fn manage_daylighting(
        &mut self,
        radiation: &mut OrdinarySolarRadiation,
        clock: &WeatherGlobalState,
        environment: &WeatherEnvironmentState,
    ) -> Result<(), Error> {
        let weather = &environment.current_weather;
        if weather.sun_is_up
            && (weather.beam_solar_rad + weather.gnd_solar_rad + weather.dif_solar_rad > 0.0)
        {
            for number in 1..=radiation.num_solar_enclosures() {
                let enclosure = solar(radiation, number)?;
                if enclosure.total_enclosure_daylight_reference_points == 0
                    || !enclosure.has_interzone_window
                {
                    continue;
                }
                return Err(Error::Unavailable(
                    "manageDaylighting actual DayltgInterReflIllFrIntWins5846",
                ));
            }
            // DayltgElecLightingControl6929 truly returns BEFORE any power
            // factor reset when this same constructor/input array is empty.
            if empty(&self.daylight_controls) {
                return Ok(());
            }
            return Err(Error::Unavailable(
                "DayltgElecLightingControl actual positive control body",
            ));
        } else if self.map_results_to_report && clock.time_step == clock.time_steps_in_hour {
            if !empty(&self.illuminance_maps) {
                return Err(Error::Unavailable(
                    "manageDaylighting actual positive ReportIllumMap5852",
                ));
            }
            self.map_results_to_report = false;
        }
        Ok(())
    }
}
