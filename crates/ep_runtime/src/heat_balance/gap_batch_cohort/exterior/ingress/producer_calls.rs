//! Real full producer calls on the getter-created owners; no expected inputs.
use super::*;
use hb::radiation::ordinary::{RadiantExchangeSource, RadiantInitializationOutput};

impl GainFactory {
    pub(crate) fn prepare_actual_beam(
        &mut self,
        primary: &Value,
        shading_path: &std::path::Path,
    ) -> Result {
        self.allocate_actual_surface_arrays()?;
        let day = int(member(primary, "day_of_year")?)?;
        let weekday = int(member(primary, "day_of_week")?)?;
        let hour = int(member(primary, "hour")?)?;
        let step = int(member(primary, "timestep")?)?;
        if !(1..=366).contains(&day)
            || !(1..=7).contains(&weekday)
            || !(1..=24).contains(&hour)
            || !(1..=self.weather.global.time_steps_in_hour).contains(&step)
        {
            return Err(invalid("named primary setup calendar/time bounds").into());
        }
        self.weather.environment.day_of_year_schedule = day;
        self.weather.environment.day_of_year = day;
        self.weather.environment.day_of_week = weekday;
        self.weather.global.hour_of_day = hour;
        self.weather.global.time_step = step;
        let globals = self
            .reports
            .selected_heat_balance_globals
            .as_ref()
            .ok_or("same globals")?;
        let zones = self
            .reports
            .selected_zone_input
            .as_ref()
            .ok_or("same real Zone/Space")?;
        let shadowing = self
            .reports
            .shadowing_input
            .as_ref()
            .ok_or("real shadowing getter owner")?;
        let control = &self.reports.heat_balance_input_state;
        let project = self
            .reports
            .selected_project_data
            .as_ref()
            .ok_or("same Project")?;
        let mut dictionary = hb::source_output_registration::SourceOutputDictionaryStreams {
            eso: Some(&mut self.eso),
            sqlite_present: self.reports.source_sqlite.present(),
            results_framework_output_variables: &mut self.reports.source_results_output_variables,
        };
        let mut children = hb::radiation::ordinary::source_children::SourceSolarChildOwners {
            shading_stream: &mut self.shading_stream,
            shading_path,
            report_flags: &mut self.reports.source_report_flags,
            globals,
            clock: &self.weather.global,
            control,
            zones,
            shadowing,
            daylighting: &mut self.daylighting,
            gain_modules: &self.modules,
            input: self
                .reports
                .selected_input_processor
                .as_mut()
                .ok_or("same IP")?,
            shortcut: &mut self.reports.input_processor_shortcut,
            report_inputs: &self.model.report_inputs,
            minimum_report_frequency: &project.system.minimum_report_frequency,
            developer: &project.system.developer_flag,
            registration: &mut self.reports.output_registration,
            prescan: &self.reports.output_prescan,
            output_host: &mut self.files,
            dictionary_host: &mut dictionary,
            eio: &mut self.eio,
        };
        let mut backend = error_backend(&mut self.error_state, &mut self.errors, &mut self.console);
        self.radiation
            .source_init_solar_calculations(
                &mut self.geometry,
                zones,
                control,
                &self.weather.global,
                &self.weather.environment,
                shadowing,
                &mut self.arrays,
                &mut children,
                &mut backend,
            )
            .map_err(|error| {
                invalid(&format!(
                    "prepare_actual_beam phase=source_init_solar_calculations: {error:?}"
                ))
            })?;
        // Literal original CalcPerSolarBeam caller phase flag, not a getter or
        // shadow-combination readiness replacement. The true SkyDif runs whole.
        self.radiation.solar_timestep.calc_sky_dif_shading = true;
        let transmittance_varies = self.geometry.shading_transmittance_varies;
        self.radiation
            .source_sky_diffuse_solar_shading(
                &mut self.geometry,
                control,
                &self.weather.global,
                shadowing.detailed_sky_diffuse_algorithm,
                transmittance_varies,
                &mut children,
                &mut backend,
            )
            .map_err(|error| {
                invalid(&format!(
                    "prepare_actual_beam phase=source_sky_diffuse_solar_shading: {error:?}"
                ))
            })?;
        self.radiation.solar_timestep.calc_sky_dif_shading = false;
        if self.radiation.solar_timestep.get_input_flag || self.radiation.solar_timestep.first_time
        {
            return Err(invalid(
                "actual full Source Solar init did not reach its own latch writers",
            )
            .into());
        }
        self.weather.global.begin_sim_flag = false;
        self.weather.global.begin_envrn_flag = false;
        Ok(())
    }

    /// Return values come from the real Source Weather methods, never fixture
    /// outputs. The unit caller then stores the true result in the actual BSDF
    /// SUNCOSTS slot before the same shared production Figure invocation.
    pub(crate) fn beam_weather_handoff(&mut self, primary: &Value) -> Result<([f64; 7], [f64; 3])> {
        self.weather.global.hour_of_day = int(member(primary, "hour")?)?;
        self.weather.global.time_step = int(member(primary, "timestep")?)?;
        self.weather.global.begin_envrn_flag = boolean(member(primary, "begin_environment")?)?;
        self.weather.environment.day_of_year_schedule = int(member(primary, "day_of_year")?)?;
        self.weather.environment.day_of_week = int(member(primary, "day_of_week")?)?;
        self.reports
            .selected_schedule_manager
            .as_mut()
            .ok_or("same schedule owner")?
            .update_source_schedule_values(&self.weather.global, &self.weather.environment)?;
        let daily = self
            .weather
            .native_gap_daily_solar_coefficients(int(member(primary, "day_of_year")?)?);
        let direction = self
            .weather
            .native_gap_sun_direction_cosines(real(member(primary, "time")?)?);
        self.radiation.solar_timestep.complex.sun_costs.set(
            [
                usize::try_from(self.weather.global.time_step)?,
                usize::try_from(self.weather.global.hour_of_day)?,
            ],
            direction,
        )?;
        Ok((daily, direction))
    }

    pub(crate) fn actual_figure(&mut self) -> Result {
        self.radiation.source_figure_solar_beam_at_timestep(
            &mut self.geometry,
            &self.reports.heat_balance_input_state,
            &self.weather.global,
            self.reports
                .shadowing_input
                .as_ref()
                .ok_or("same shadowing input owner")?,
            usize::try_from(self.weather.global.hour_of_day)?,
            usize::try_from(self.weather.global.time_step)?,
        )?;
        Ok(())
    }

    pub(crate) fn allocate_actual_surface_arrays(&mut self) -> Result {
        let surfaces = self
            .geometry
            .final_surfaces
            .as_ref()
            .ok_or("actual final Surface allocation")?;
        let ids = surfaces
            .iter()
            .map(|s| s.typed_id.ok_or("actual getter-bound Surface identity"))
            .collect::<std::result::Result<Vec<_>, _>>()?;
        if surfaces.len() != usize::try_from(self.geometry.total_surfaces)? {
            return Err(invalid("actual allocated Surface count association").into());
        }
        self.history =
            Some(hb::ctf_thermal_history::CtfThermalHistoryState::new_source_unallocated(ids));
        // Actual DataHeatBalance.hh1870 / 1871 constructor true / zero. No CTF
        // calculation is invoked or claimed by this original direct unit route.
        let globals = self
            .reports
            .selected_heat_balance_globals
            .as_ref()
            .ok_or("same globals")?;
        self.arrays.allocate(
            surfaces.len(),
            self.reports
                .heat_balance_input_state
                .any_internal_heat_source_in_input,
            self.public_ctf
                .aggregate_after_returned_calls
                .simple_ctf_only,
            globals.any_energy_management_system_in_model,
            self.history.as_mut().ok_or("same retained history owner")?,
        )?;
        Ok(())
    }

    pub(crate) fn initialize_actual_radiant(&mut self) -> Result {
        let globals = self
            .reports
            .selected_heat_balance_globals
            .as_ref()
            .ok_or("same globals")?;
        let zones = self
            .reports
            .selected_zone_input
            .as_ref()
            .ok_or("same actual Zone/Space")?;
        let project = self
            .reports
            .selected_project_data
            .as_ref()
            .ok_or("same Project")?;
        let source = RadiantExchangeSource {
            geometry: &self.geometry,
            zones,
            constructions: &self.reports.heat_balance_input_state,
            carroll_method: project.carroll_method,
            any_inside_shelf: globals.any_inside_shelf,
        };
        let mut backend = error_backend(&mut self.error_state, &mut self.errors, &mut self.console);
        let mut output = RadiantInitializationOutput {
            input: self
                .reports
                .selected_input_processor
                .as_mut()
                .ok_or("same IP")?,
            shortcut: &mut self.reports.input_processor_shortcut,
            scan: &mut self.reports.scan,
            report_inputs: &self.model.report_inputs,
            backend: &mut backend,
            eio: &mut self.eio,
            debug: None, // This gated legacy owner has no actual debug stream.
            advanced_report_variables: globals.display_advanced_report_variables,
        };
        self.radiation
            .initialize_source_radiant(&source, &mut output)?;
        Ok(())
    }

    pub(crate) fn prepare_actual_surface_gains(&mut self) -> Result {
        self.allocate_actual_surface_arrays()?;
        self.initialize_actual_radiant()?;
        let zones = self
            .reports
            .selected_zone_input
            .as_ref()
            .ok_or("same actual Zone/Space")?;
        let globals = self
            .reports
            .selected_heat_balance_globals
            .as_ref()
            .ok_or("same globals")?;
        let rows = zones.zones.as_ref().ok_or("actual Zone allocation")?;
        let spaces = zones.spaces.as_ref().ok_or("actual Space allocation")?;
        let active = &self
            .geometry
            .window_arrays
            .as_ref()
            .ok_or("actual active Construction array")?
            .surf_active_construction;
        // Literal metadata-only HBSM365-366 caller handoff. Both source values
        // come from the real input/getter Construction owner, never the oracle.
        for zone in rows.iter().take(usize::try_from(globals.num_of_zones)?) {
            for &space_number in &zone.space_indexes {
                let space = spaces
                    .get(index(space_number)?)
                    .ok_or("actual Space ordinal")?;
                for number in space.ht_surface_first..=space.ht_surface_last {
                    let i = index(number)?;
                    let construction = self
                        .reports
                        .heat_balance_input_state
                        .construction_usage
                        .get(index(
                            *active.get(i).ok_or("actual active Construction ordinal")?,
                        )?)
                        .ok_or("actual Construction owner")?;
                    *self
                        .arrays
                        .surf_abs_solar_int
                        .as_mut()
                        .and_then(|v| v.get_mut(i))
                        .ok_or("actual solar absorptance cell")? = construction.inside_absorp_solar;
                    *self
                        .arrays
                        .surf_abs_thermal_int
                        .as_mut()
                        .and_then(|v| v.get_mut(i))
                        .ok_or("actual thermal absorptance cell")? =
                        construction.inside_absorp_thermal;
                }
            }
        }
        hb::ordinary_surface_initialization::native_gap_check_glazing_shading_status_change(
            &self.geometry,
            zones,
            &mut self.reports.heat_balance_input_state,
            globals,
            &self.weather.global,
            &mut self.radiation,
        )?;
        let project = self
            .reports
            .selected_project_data
            .as_ref()
            .ok_or("same Project")?;
        let source = RadiantExchangeSource {
            geometry: &self.geometry,
            zones,
            constructions: &self.reports.heat_balance_input_state,
            carroll_method: project.carroll_method,
            any_inside_shelf: globals.any_inside_shelf,
        };
        self.radiation.compute_source_thermal_absorption(
            &source,
            self.arrays
                .surf_abs_thermal_int
                .as_deref_mut()
                .ok_or("actual thermal absorptance")?,
            self.geometry.any_movable_slat,
        )?;
        let mut backend = error_backend(&mut self.error_state, &mut self.errors, &mut self.console);
        self.radiation.compute_source_shortwave_absorption(
            &source,
            self.arrays
                .surf_abs_solar_int
                .as_deref()
                .ok_or("actual solar absorptance")?,
            &mut backend,
        )?;
        self.weather.global.begin_sim_flag = false;
        self.weather.global.begin_envrn_flag = false;
        Ok(())
    }
}
