//! Actual opaque InitSolarHeatGains2515-3776 on the sole live Solar arrays.
use super::*;
use crate::heat_balance::selected_gain_modules::SourceGainModules;

impl OrdinarySolarRadiation {
    pub(crate) fn source_init_solar_heat_gains(
        &mut self, geometry: &mut SurfaceGeometryInputState,
        zones: &SelectedZoneInputOwners, control: &SelectedHeatBalanceInputState,
        globals: &WeatherGlobalState, environment: &mut WeatherEnvironmentState,
        arrays: &mut SourceSurfaceHeatBalanceArrays, modules: &SourceGainModules,
        init_surface_first_time: bool, interzone_window: bool,
    ) -> Result<(), Error> {
        let zone_rows = zones.zones.as_ref().ok_or(Error::Unavailable("actual Zone allocation"))?;
        let spaces = zones.spaces.as_ref().ok_or(Error::Unavailable("actual Space allocation"))?;
        let count = surface_count(geometry)?;
        for number in 1..=zone_rows.len() {
            let hb = &mut self.solar_timestep.heat_balance;
            hb.zone_win_heat_gain_rep_energy.set([number],0.0)?;
            hb.zone_win_heat_loss_rep_energy.set([number],0.0)?;
            hb.zn_opq_surf_ins_face_cond_gn_rep_enrg.set([number],0.0)?;
            hb.zn_opq_surf_ins_face_cond_ls_rep_enrg.set([number],0.0)?;
            hb.zn_opq_surf_ext_face_cond_gn_rep_enrg.set([number],0.0)?;
            hb.zn_opq_surf_ext_face_cond_ls_rep_enrg.set([number],0.0)?;
            hb.zone_win_heat_gain.set([number],0.0)?;
            hb.zone_win_heat_gain_rep.set([number],0.0)?;
            hb.zone_win_heat_loss_rep.set([number],0.0)?;
            hb.zone_opaq_surf_ins_face_cond.set([number],0.0)?;
            hb.zone_opaq_surf_ins_face_cond_gain_rep.set([number],0.0)?;
            hb.zone_opaq_surf_ins_face_cond_loss_rep.set([number],0.0)?;
            hb.zone_opaq_surf_ext_face_cond.set([number],0.0)?;
            hb.zone_opaq_surf_ext_face_cond_gain_rep.set([number],0.0)?;
            hb.zone_opaq_surf_ext_face_cond_loss_rep.set([number],0.0)?;
        }
        for enclosure in 1..=usize::try_from(self.num_solar_enclosures).map_err(|_| Error::Unavailable("actual Solar enclosure count"))? {
            self.solar_timestep.heat_balance.encl_sol_initial_dif_sol_refl_w.set([enclosure],0.0)?;
        }
        for zone in zone_rows { for &space_number in &zone.space_indexes {
            let space = spaces.get(ordinal(space_number)?-1).ok_or(Error::Unavailable("actual Source Space association"))?;
            for number in space.opaque_or_intmass_first..=space.opaque_or_intmass_last {
                let number = ordinal(number)?;
                set(&mut arrays.surf_opaq_ins_face_cond_gain_rep,number,0.0)?;
                set(&mut arrays.surf_opaq_ins_face_cond_loss_rep,number,0.0)?;
                set(&mut arrays.surf_opaq_q_rad_sw_in_abs,number,0.0)?;
                set(&mut arrays.surf_qdot_rad_lights_in_per_area,number,0.0)?;
                set(&mut arrays.surf_opaq_q_rad_sw_out_abs,number,0.0)?;
                set(&mut arrays.surf_opaq_initial_dif_sol_in_abs,number,0.0)?;
                set(&mut arrays.surf_opaq_ins_face_beam_sol_absorbed,number,0.0)?;
                set(&mut arrays.surf_opaq_sw_out_abs_total_report,number,0.0)?;
                set(&mut arrays.surf_opaq_sw_out_abs_energy_report,number,0.0)?;
            }
            // Actual empty Source window ranges are inspected at their first
            // Original2598 boundary; no later positive-window writes are skipped.
            for _ in space.window_first..=space.window_last {
                return Err(Error::Unavailable("InitSolarHeatGains actual window reset2598"));
            }
        } }
        if init_surface_first_time {
            for number in 1..=count {
                let ground = geometry.final_surfaces.as_ref().ok_or(Error::Unavailable("actual final Source Surface"))?[number-1].row.view_factor_ground;
                let windows = geometry.window_arrays.as_mut().ok_or(Error::Unavailable("same Solar DataSurface arrays"))?;
                windows.surf_bm_to_diff_refl_fac_gnd.set([number],ground)?;
                windows.surf_sky_diff_refl_fac_gnd.set([number],ground)?;
            }
        }
        let weather = &environment.current_weather;
        let current_positive = weather.sun_is_up && (weather.beam_solar_rad + weather.gnd_solar_rad + weather.dif_solar_rad > 0.0);
        let sunset = !current_positive && environment.previous_sol_rad_positive;
        let sun_up_no_rad = weather.sun_is_up && !current_positive;
        let reset_solar = globals.begin_envrn_flag || sun_up_no_rad || sunset;
        // Sole DataEnvironment flag, written at genuine2651, before resets.
        environment.previous_sol_rad_positive = current_positive;
        if current_positive || reset_solar {
            for number in 1..=count {
                let hb = &mut self.solar_timestep.heat_balance;
                hb.surf_bm_inc_ins_surf_intens_rep.set([number],0.0)?;
                hb.surf_bm_inc_ins_surf_amount_rep.set([number],0.0)?;
                hb.surf_int_bm_inc_ins_surf_intens_rep.set([number],0.0)?;
                hb.surf_int_bm_inc_ins_surf_amount_rep.set([number],0.0)?;
                hb.surf_int_bm_inc_ins_surf_amount_rep_energy.set([number],0.0)?;
                hb.surf_q_rad_sw_out_incident.set([number],0.0)?;
                hb.surf_q_rad_sw_out_incident_beam.set([number],0.0)?;
                hb.surf_q_rad_sw_out_incident_sky_diffuse.set([number],0.0)?;
                hb.surf_q_rad_sw_out_incident_gnd_diffuse.set([number],0.0)?;
                hb.surf_q_rad_sw_out_inc_bm_to_diff_refl_gnd.set([number],0.0)?;
                hb.surf_q_rad_sw_out_inc_sky_diff_refl_gnd.set([number],0.0)?;
                hb.surf_q_rad_sw_out_inc_bm_to_bm_refl_obs.set([number],0.0)?;
                hb.surf_q_rad_sw_out_inc_bm_to_diff_refl_obs.set([number],0.0)?;
                hb.surf_q_rad_sw_out_inc_sky_diff_refl_obs.set([number],0.0)?;
                set(&mut arrays.surf_sky_solar_inc,number,0.0)?;
                set(&mut arrays.surf_gnd_solar_inc,number,0.0)?;
            }
            for number in 1..=usize::try_from(self.num_solar_enclosures).map_err(|_| Error::Unavailable("actual enclosure count"))? {
                let hb = &mut self.solar_timestep.heat_balance;
                hb.zone_trans_solar.set([number],0.0)?;
                hb.zone_bm_sol_fr_ext_wins_rep.set([number],0.0)?;
                hb.zone_bm_sol_fr_int_wins_rep.set([number],0.0)?;
                hb.zone_dif_sol_fr_ext_wins_rep.set([number],0.0)?;
                hb.zone_dif_sol_fr_int_wins_rep.set([number],0.0)?;
                hb.zone_trans_solar_energy.set([number],0.0)?;
                hb.zone_bm_sol_fr_ext_wins_rep_energy.set([number],0.0)?;
                hb.zone_bm_sol_fr_int_wins_rep_energy.set([number],0.0)?;
                hb.zone_dif_sol_fr_ext_wins_rep_energy.set([number],0.0)?;
                hb.zone_dif_sol_fr_int_wins_rep_energy.set([number],0.0)?;
            }
        }
        if reset_solar {
            for number in 1..=usize::try_from(self.num_solar_enclosures).map_err(|_| Error::Unavailable("actual enclosure count"))? {
                self.solar_timestep.heat_balance.encl_sol_qd.set([number],0.0)?;
                self.solar_timestep.heat_balance.encl_sol_q_dfor_daylight.set([number],0.0)?;
            }
            if !modules.tdd_pipe.as_deref().unwrap_or_default().is_empty() {
                return Err(Error::Unavailable("InitSolarHeatGains actual TDD reset2777"));
            }
            if control.calculate_solar_reflections {
                let windows = geometry.window_arrays.as_mut().ok_or(Error::Unavailable("actual Solar reflection arrays"))?;
                for number in 1..=count {
                    windows.surf_bm_to_bm_refl_fac_obs.set([number],0.0)?;
                    windows.surf_bm_to_diff_refl_fac_obs.set([number],0.0)?;
                    windows.surf_bm_to_diff_refl_fac_gnd.set([number],0.0)?;
                }
            }
            for number in 1..=count {
                let hb = &mut self.solar_timestep.heat_balance;
                hb.surf_initial_dif_sol_in_abs_report.set([number],0.0)?;
                hb.surf_cos_incidence_angle.set([number],0.0)?;
                hb.surf_sw_in_abs_total_report.set([number],0.0)?;
                let windows = geometry.window_arrays.as_mut().ok_or(Error::Unavailable("same DataSurface reset arrays"))?;
                *windows.surf_win_profile_ang_hor.get_mut(number-1).ok_or(Error::Unavailable("actual window angle bounds"))? = 0.0;
                *windows.surf_win_profile_ang_vert.get_mut(number-1).ok_or(Error::Unavailable("actual window angle bounds"))? = 0.0;
                windows.surf_win_sys_sol_reflectance.set([number],0.0)?;
                windows.surf_win_sys_sol_absorptance.set([number],0.0)?;
            }
        }
        if !current_positive { return Ok(()); }
        // Actual reflection arrays remain unallocated on the no-reflection
        // branch, so the two Source equal_dimensions assertions hold there.
        let ground_solar_incident = environment.current_weather.beam_solar_rad * environment.current_weather.solar_direction_cosines[2]
            + environment.current_weather.dif_solar_rad;
        let _ground_solar_incident = if ground_solar_incident < 0.0 { 0.0 } else { ground_solar_incident };
        for number in 1..=count {
            let surface = &mut geometry.final_surfaces.as_mut().ok_or(Error::Unavailable("actual final Surface"))?[number-1];
            // Genuine GetSurfIncidentSolarMultiplier9882-9892: its constructor
            // flag is the actual early-return predicate.
            if surface.has_incident_solar_multiplier { return Err(Error::Unavailable("GetSurfIncidentSolarMultiplier positive scalar/schedule owner")); }
            surface.incident_solar_multiplier = 1.0;
        }
        for number in 1..=count {
            let surface = &geometry.final_surfaces.as_ref().ok_or(Error::Unavailable("actual final Surface"))?[number-1];
            let multiplier = surface.incident_solar_multiplier;
            set(&mut arrays.surf_sky_solar_inc,number,environment.current_weather.dif_solar_rad * multiplier * self.solar_timestep.shading.surf_aniso_sky_mult.read([number])?)?;
            if surface.use_ground_surface_reflectance { return Err(Error::Unavailable("InitSolarHeatGains actual GroundSurfsProperty reflectance2826")); }
            let ground = environment.current_weather.gnd_solar_rad * multiplier * surface.row.view_factor_ground;
            set(&mut arrays.surf_gnd_solar_inc,number,ground)?;
            let windows = geometry.window_arrays.as_mut().ok_or(Error::Unavailable("actual window solar arrays"))?;
            *windows.surf_win_sky_gnd_solar_inc.get_mut(number-1).ok_or(Error::Unavailable("actual window ground bounds"))? = ground;
            *windows.surf_win_bm_gnd_solar_inc.get_mut(number-1).ok_or(Error::Unavailable("actual window ground bounds"))? = 0.0;
        }
        if control.calculate_solar_reflections { return Err(Error::Unavailable("InitSolarHeatGains actual reflection interpolation2837")); }
        // Actual CalcWindowProfileAngles10729 performs these local weather
        // operations even with no window rows, then traverses the same ranges.
        let elevation = std::f64::consts::PI / 2.0 - environment.current_weather.solar_direction_cosines[2].acos();
        let _azimuth = environment.current_weather.solar_direction_cosines[0].atan2(environment.current_weather.solar_direction_cosines[1]);
        let _cos_elevation = elevation.cos();
        let _sin_elevation = elevation.sin();
        for zone in zone_rows { for &space_number in &zone.space_indexes {
            let space = &spaces[ordinal(space_number)?-1];
            for _ in space.window_first..=space.window_last { return Err(Error::Unavailable("CalcWindowProfileAngles actual positive window")); }
        } }
        if control.calculate_window_reveal_reflection { return Err(Error::Unavailable("InitSolarHeatGains actual CalcBeamSolarOnWinRevealSurface")); }
        self.source_calc_interior_solar_distribution(geometry,zones,control,globals,environment,arrays,modules)?;
        for number in 1..=usize::try_from(self.num_solar_enclosures).map_err(|_| Error::Unavailable("actual enclosure count"))? {
            let hb = &mut self.solar_timestep.heat_balance;
            let beam = environment.current_weather.beam_solar_rad;
            hb.encl_sol_q_dfor_daylight.set([number],(hb.encl_sol_db.read([number])? - hb.encl_sol_db_int_win.read([number])?) * beam
                + hb.encl_sol_dbssg.read([number])? + hb.encl_sol_initial_dif_sol_refl_w.read([number])?)?;
            hb.encl_sol_qd.set([number],hb.encl_sol_db.read([number])? * beam + hb.encl_sol_dbssg.read([number])?
                + hb.encl_sol_initial_dif_sol_refl_w.read([number])?)?;
        }
        for number in 1..=usize::try_from(self.num_solar_enclosures).map_err(|_| Error::Unavailable("actual enclosure count"))? {
            let hb = &mut self.solar_timestep.heat_balance;
            hb.encl_sol_qs_dif_sol.set([number],hb.encl_sol_q_dfor_daylight.read([number])?)?;
        }
        if interzone_window { return Err(Error::Unavailable("InitSolarHeatGains actual interzone distribution2933")); }
        for number in 1..=usize::try_from(self.num_solar_enclosures).map_err(|_| Error::Unavailable("actual enclosure count"))? {
            let multiplier = self.solar.as_ref().and_then(|rows| rows.get(number-1)).ok_or(Error::Unavailable("same solar enclosure"))?.sol_vmult;
            let hb = &mut self.solar_timestep.heat_balance;
            hb.encl_sol_qs_dif_sol.set([number],hb.encl_sol_qs_dif_sol.read([number])? * multiplier)?;
        }
        // Positive shading inputs are excluded by the selected raw getters;
        // retained shadow rows still prevent silently skipping a reached branch.
        if geometry.final_surfaces.as_ref().unwrap().iter().any(|row| row.row.is_shadowing) {
            return Err(Error::Unavailable("InitSolarHeatGains actual shading-surface incident reports2974"));
        }
        let hour = ordinal(globals.hour_of_day)?;
        let step = ordinal(globals.time_step)?;
        let mut current_beam_solar = vec![None::<f64>;count];
        for &number in &geometry.surface_lists.all_exterior_solar {
            let number = ordinal(number)?;
            let surface = &geometry.final_surfaces.as_ref().unwrap()[number-1];
            let weather = &environment.current_weather;
            let beam = weather.beam_solar_rad * surface.incident_solar_multiplier;
            current_beam_solar[number-1] = Some(beam);
            let diffuse = weather.dif_solar_rad * surface.incident_solar_multiplier;
            let hb = &mut self.solar_timestep.heat_balance;
            let cosine = hb.surf_cos_inc_ang.read([hour,step,number])?;
            hb.surf_cos_incidence_angle.set([number],cosine)?;
            hb.surf_q_rad_sw_out_incident_beam.set([number],beam * hb.surf_sunlit_frac.read([hour,step,number])? * cosine)?;
            if surface.use_ground_surface_reflectance { return Err(Error::Unavailable("InitSolarHeatGains actual ground report reflectance3030")); }
            hb.surf_q_rad_sw_out_incident_sky_diffuse.set([number],diffuse * self.solar_timestep.shading.surf_aniso_sky_mult.read([number])?)?;
            hb.surf_q_rad_sw_out_incident_gnd_diffuse.set([number],get(&arrays.surf_gnd_solar_inc,number)?)?;
            let windows = geometry.window_arrays.as_ref().ok_or(Error::Unavailable("actual reflection factors"))?;
            hb.surf_q_rad_sw_out_inc_bm_to_diff_refl_gnd.set([number],beam * weather.solar_direction_cosines[2] * weather.gnd_reflectance * windows.surf_bm_to_diff_refl_fac_gnd.read([number])?)?;
            hb.surf_q_rad_sw_out_inc_sky_diff_refl_gnd.set([number],diffuse * weather.gnd_reflectance * windows.surf_sky_diff_refl_fac_gnd.read([number])?)?;
            hb.surf_q_rad_sw_out_incident.set([number],hb.surf_q_rad_sw_out_incident_beam.read([number])?
                + hb.surf_q_rad_sw_out_incident_sky_diffuse.read([number])? + hb.surf_q_rad_sw_out_inc_bm_to_diff_refl_gnd.read([number])?
                + hb.surf_q_rad_sw_out_inc_sky_diff_refl_gnd.read([number])?)?;
        }
        if !modules.tdd_pipe.as_deref().unwrap_or_default().is_empty() { return Err(Error::Unavailable("InitSolarHeatGains actual TDD reports3069")); }
        for zone in zone_rows { for &space_number in &zone.space_indexes {
            let space = &spaces[ordinal(space_number)?-1];
            for number in space.opaque_or_intmass_first..=space.opaque_or_intmass_last {
                let number = ordinal(number)?;
                let surface = &geometry.final_surfaces.as_ref().unwrap()[number-1];
                let construction = surface.row.construction;
                let beam = environment.current_weather.beam_solar_rad * surface.incident_solar_multiplier;
                current_beam_solar[number-1] = Some(beam);
                let windows = geometry.window_arrays.as_ref().ok_or(Error::Unavailable("same opaque solar factors"))?;
                if surface.row.exterior_solar {
                    set(&mut arrays.surf_opaq_q_rad_sw_out_abs,number,windows.surf_opaq_ao.read([number])? * beam
                        + get(&arrays.surf_abs_solar_ext,number)? * (get(&arrays.surf_sky_solar_inc,number)? + get(&arrays.surf_gnd_solar_inc,number)?))?;
                }
                if construction > 0 {
                    // Actual SurfaceScheduledSolarInc8936 iterates the retained
                    // count, and returns0 only when that real loop is empty.
                    if geometry.total_incident_solar_scheduled_gains > 0 { return Err(Error::Unavailable("SurfaceScheduledSolarInc actual positive schedule owner")); }
                    let construct = control.construction_usage.get(ordinal(construction)?-1).ok_or(Error::Unavailable("same Construction owner"))?;
                    if construct.trans_diff <= 0.0 {
                        let shelf = *geometry.parallel_arrays.as_ref().ok_or(Error::Unavailable("actual parallel surface arrays"))?.surf_daylighting_shelf_ind.get(number-1).ok_or(Error::Unavailable("actual shelf association"))?;
                        if shelf > 0 { return Err(Error::Unavailable("InitSolarHeatGains actual inside Shelf3138")); }
                        let absorbed = windows.surf_opaq_ai.read([number])? * beam;
                        add(&mut arrays.surf_opaq_q_rad_sw_in_abs,number,absorbed)?;
                        set(&mut arrays.surf_opaq_ins_face_beam_sol_absorbed,number,absorbed * source_surface(geometry,number)?.area)?;
                    }
                }
            }
            for _ in space.window_first..=space.window_last { return Err(Error::Unavailable("InitSolarHeatGains actual window solve3172")); }
        } }
        if !modules.tdd_pipe.as_deref().unwrap_or_default().is_empty() { return Err(Error::Unavailable("InitSolarHeatGains actual TDD absorption3682")); }
        if geometry.use_representative_surface_calculations { return Err(Error::Unavailable("InitSolarHeatGains actual representative-surface absorption")); }
        Ok(())
    }
}
