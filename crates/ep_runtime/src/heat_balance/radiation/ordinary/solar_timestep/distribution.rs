//! Actual opaque CalcInteriorSolarDistribution and InitIntSolarDistribution.
use super::*;
use crate::heat_balance::ctf_initialization_context::internal_heat_gains_input::SelectedInternalHeatGains;
use crate::heat_balance::selected_gain_modules::SourceGainModules;

impl OrdinarySolarRadiation {
    pub(crate) fn source_calc_interior_solar_distribution(
        &mut self, geometry: &mut SurfaceGeometryInputState,
        zones: &SelectedZoneInputOwners, control: &SelectedHeatBalanceInputState,
        globals: &WeatherGlobalState, environment: &WeatherEnvironmentState,
        arrays: &mut SourceSurfaceHeatBalanceArrays, modules: &SourceGainModules,
    ) -> Result<(), Error> {
        let zone_rows = zones.zones.as_ref().ok_or(Error::Unavailable("actual Zone allocation"))?;
        let spaces = zones.spaces.as_ref().ok_or(Error::Unavailable("actual Space allocation"))?;
        for zone in zone_rows { for &space_number in &zone.space_indexes {
            let space = spaces.get(ordinal(space_number)?-1).ok_or(Error::Unavailable("actual Space association"))?;
            for _ in space.window_first..=space.window_last { return Err(Error::Unavailable("CalcInteriorSolarDistribution actual window factor6565")); }
            for number in space.opaque_or_intmass_first..=space.opaque_or_intmass_last {
                let number = ordinal(number)?;
                let windows = geometry.window_arrays.as_mut().ok_or(Error::Unavailable("actual DataSurface Solar arrays"))?;
                windows.surf_opaq_ai.set([number],0.0)?;
                windows.surf_opaq_ao.set([number],0.0)?;
            }
        } }
        if !modules.tdd_pipe.as_deref().unwrap_or_default().is_empty() { return Err(Error::Unavailable("CalcInteriorSolarDistribution actual TDD factor6581")); }
        let hour = ordinal(globals.hour_of_day)?;
        let step = ordinal(globals.time_step)?;
        for enclosure_number in 1..=usize::try_from(self.num_solar_enclosures).map_err(|_| Error::Unavailable("actual Solar enclosure count"))? {
            // Actual locals are initialized here, after the true opaque resets.
            let total_beam = 0.0;
            let absorbed_beam = 0.0;
            self.solar_timestep.heat_balance.encl_sol_db.set([enclosure_number],0.0)?;
            self.solar_timestep.heat_balance.encl_sol_dbssg.set([enclosure_number],0.0)?;
            self.solar_timestep.heat_balance.encl_sol_db_int_win.set([enclosure_number],0.0)?;
            if control.any_bsdf {
                geometry.window_arrays.as_mut().ok_or(Error::Unavailable("same SurfWinACFOverlap"))?.surf_win_acf_overlap.fill(0.0);
            }
            let enclosure = self.solar.as_ref().and_then(|rows| rows.get(enclosure_number-1)).ok_or(Error::Unavailable("same Source Solar enclosure"))?;
            let source_pointers = enclosure.source_surface_ptr.as_deref().ok_or(Error::Unavailable("actual InitSolar SurfacePtr"))?;
            for &number in source_pointers {
                let number = ordinal(number)?;
                let surface = &geometry.final_surfaces.as_ref().ok_or(Error::Unavailable("same Source Surface"))?[number-1].row;
                if surface.class != SurfaceClass::Window && surface.class != SurfaceClass::TddDome {
                    if !surface.heat_transfer_surface || !surface.exterior_solar { continue; }
                    let windows = geometry.window_arrays.as_mut().ok_or(Error::Unavailable("same DataSurface Solar factors"))?;
                    let construction = *windows.surf_active_construction.get(number-1).ok_or(Error::Unavailable("actual active Construction"))?;
                    let construction = control.construction_usage.get(ordinal(construction)?-1).ok_or(Error::Unavailable("same Construction owner"))?;
                    let cosine = self.solar_timestep.heat_balance.surf_cos_inc_ang.read([hour,step,number])?;
                    let sunlit = self.solar_timestep.heat_balance.surf_sunlit_frac.read([hour,step,number])?;
                    windows.surf_opaq_ao.set([number],construction.outside_absorp_solar * cosine * sunlit)?;
                }
            }
            for &number in source_pointers {
                let number = ordinal(number)?;
                let surface = &geometry.final_surfaces.as_ref().unwrap()[number-1];
                if surface.row.class != SurfaceClass::Window && surface.row.class != SurfaceClass::TddDome { continue; }
                if !surface.row.exterior_solar && surface.original_class != SurfaceClass::TddDiffuser { continue; }
                return Err(Error::Unavailable("CalcInteriorSolarDistribution actual exterior window/TDD6639"));
            }
            let absorbed_scheduled_beam = 0.0;
            let total_scheduled_beam = 0.0;
            if geometry.total_incident_solar_scheduled_gains > 0 { return Err(Error::Unavailable("CalcInteriorSolarDistribution actual scheduled gains8442")); }
            let hb = &mut self.solar_timestep.heat_balance;
            hb.encl_sol_dbssg.set([enclosure_number],total_scheduled_beam - absorbed_scheduled_beam)?;
            hb.encl_sol_db.set([enclosure_number],total_beam - absorbed_beam)?;
            if hb.encl_sol_db.read([enclosure_number])? < 0.0 { hb.encl_sol_db.set([enclosure_number],0.0)?; }
            for &number in source_pointers {
                let number = ordinal(number)?;
                let surface = &geometry.final_surfaces.as_ref().unwrap()[number-1];
                let beam = environment.current_weather.beam_solar_rad * surface.incident_solar_multiplier;
                let _diffuse = environment.current_weather.dif_solar_rad * surface.incident_solar_multiplier;
                let _ground = environment.current_weather.gnd_solar_rad * surface.incident_solar_multiplier;
                let hb = &mut self.solar_timestep.heat_balance;
                if control.solar_distribution == SelectedSolarDistribution::FullInteriorExterior {
                    let amount = hb.surf_bm_inc_ins_surf_amount_rep.read([number])? * beam;
                    hb.surf_bm_inc_ins_surf_amount_rep.set([number],amount)?;
                    hb.surf_bm_inc_ins_surf_amount_rep_energy.set([number],amount * globals.time_step_zone_sec)?;
                    let divider = *geometry.window_arrays.as_ref().ok_or(Error::Unavailable("actual divider area"))?.surf_win_divider_area.get(number-1).ok_or(Error::Unavailable("actual divider area bounds"))?;
                    hb.surf_bm_inc_ins_surf_intens_rep.set([number],amount / (surface.row.area + divider))?;
                } else {
                    if self.solar_timestep.shading.surf_int_abs_fac.read([number])? > 0.0 && surface.row.heat_transfer_surface {
                        let intensity = if enclosure.floor_area > 0.0 { beam * total_beam / enclosure.floor_area }
                            else if enclosure.total_surface_area > 0.0 { beam * total_beam / enclosure.total_surface_area }
                            else { 0.0 };
                        hb.surf_bm_inc_ins_surf_intens_rep.set([number],intensity)?;
                    }
                    let amount = surface.row.area * hb.surf_bm_inc_ins_surf_intens_rep.read([number])?;
                    hb.surf_bm_inc_ins_surf_amount_rep.set([number],amount)?;
                    hb.surf_bm_inc_ins_surf_amount_rep_energy.set([number],amount * globals.time_step_zone_sec)?;
                }
                if surface.row.class == SurfaceClass::Window || surface.row.class == SurfaceClass::TddDome {
                    return Err(Error::Unavailable("CalcInteriorSolarDistribution actual transmitted-window reports8494"));
                }
            }
        }
        for number in 1..=usize::try_from(self.num_solar_enclosures).map_err(|_| Error::Unavailable("actual enclosure count"))? {
            let hb = &mut self.solar_timestep.heat_balance;
            hb.encl_sol_db.set([number],hb.encl_sol_db.read([number])? + hb.encl_sol_db_int_win.read([number])?)?;
            let beam = hb.encl_sol_db_int_win.read([number])? * environment.current_weather.beam_solar_rad;
            hb.zone_bm_sol_fr_int_wins_rep.set([number],beam)?;
            hb.zone_bm_sol_fr_int_wins_rep_energy.set([number],beam * globals.time_step_zone_sec)?;
        }
        self.source_calc_window_transmitted_diffuse_initial_distribution(geometry,arrays)
    }

    fn source_calc_window_transmitted_diffuse_initial_distribution(
        &mut self, geometry: &SurfaceGeometryInputState, arrays: &mut SourceSurfaceHeatBalanceArrays,
    ) -> Result<(), Error> {
        // Actual11878-11968 locals allocate four2x(CFSMAXNL+1) matrices without
        // numeric writes, even when no exterior transmitting surfaces exist.
        let _beam = [[None::<f64>;7];2];
        let _diffuse = [[None::<f64>;7];2];
        let _beam_back = [[None::<f64>;7];2];
        let _diffuse_back = [[None::<f64>;7];2];
        arrays.surf_opaq_initial_dif_sol_in_abs.as_mut().ok_or(Error::Unavailable("actual initial diffuse opaque allocation"))?.fill(0.0);
        for row in arrays.surf_win_initial_dif_solwin_abs.as_mut().ok_or(Error::Unavailable("same initial diffuse layer allocation"))? { row.fill(0.0); }
        self.solar_timestep.heat_balance.encl_sol_initial_dif_sol_refl_w.fill(0.0)?;
        arrays.surf_win_initial_dif_sol_in_trans.as_mut().ok_or(Error::Unavailable("same initial diffuse window allocation"))?.fill(0.0);
        for number in 1..=usize::try_from(self.num_radiant_enclosures).map_err(|_| Error::Unavailable("actual radiant enclosure count"))? {
            // Original11976 intentionally traverses NumOfRadiantEnclosures but
            // reads EnclSolInfo; retain that exact bound rather than repair it.
            let enclosure = self.solar.as_ref().and_then(|rows| rows.get(number-1)).ok_or(Error::Unavailable("actual Solar enclosure at radiant ordinal"))?;
            for &number in enclosure.source_surface_ptr.as_deref().ok_or(Error::Unavailable("actual Source Solar SurfacePtr"))? {
                let surface = &geometry.final_surfaces.as_ref().ok_or(Error::Unavailable("same final Surface"))?[ordinal(number)?-1];
                if surface.row.exterior_boundary != 0 && surface.row.exterior_boundary != -4 && surface.original_class != SurfaceClass::TddDiffuser { continue; }
                if surface.row.class != SurfaceClass::Window && surface.original_class != SurfaceClass::TddDiffuser { continue; }
                return Err(Error::Unavailable("CalcWinTransDifSolInitialDistribution actual transmitting surface"));
            }
        }
        Ok(())
    }

    pub(crate) fn source_init_interior_solar_distribution(
        &mut self, geometry: &mut SurfaceGeometryInputState,
        zones: &SelectedZoneInputOwners, control: &SelectedHeatBalanceInputState,
        globals: &WeatherGlobalState, environment: &WeatherEnvironmentState,
        arrays: &mut SourceSurfaceHeatBalanceArrays, gains: &SelectedInternalHeatGains,
        modules: &SourceGainModules, interzone_window: bool,
    ) -> Result<(), Error> {
        let space_gains = gains.allocations.space_int_gain.as_deref().ok_or(Error::Unavailable("same actual allocated spaceIntGain"))?;
        for number in 1..=usize::try_from(self.num_solar_enclosures).map_err(|_| Error::Unavailable("actual enclosure count"))? {
            let enclosure = self.solar.as_ref().and_then(|rows| rows.get(number-1)).ok_or(Error::Unavailable("same Source Solar enclosure"))?;
            let mut sum_space_shortwave = 0.0;
            for &space in &enclosure.space_nums {
                sum_space_shortwave += space_gains.get(ordinal(space)?-1).ok_or(Error::Unavailable("same actual spaceIntGain QLTSW ordinal"))?[1];
            }
            let hb = &mut self.solar_timestep.heat_balance;
            hb.encl_sol_qsw_rad.set([number],hb.encl_sol_qd.read([number])? + sum_space_shortwave)?;
            hb.encl_sol_qsw_rad_lights.set([number],sum_space_shortwave)?;
        }
        if interzone_window { return Err(Error::Unavailable("InitIntSolarDistribution actual interzone branch3817")); }
        if environment.current_weather.sun_is_up {
            for &number in &geometry.surface_lists.all_ht {
                let number = ordinal(number)?;
                let surface = &geometry.final_surfaces.as_ref().ok_or(Error::Unavailable("same actual Surface"))?[number-1];
                if surface.row.class == SurfaceClass::Shading { continue; }
                let enclosure_number = ordinal(surface.row.solar_encl_index)?;
                let enclosure = self.solar.as_ref().and_then(|rows| rows.get(enclosure_number-1)).ok_or(Error::Unavailable("same Source Solar enclosure"))?;
                let hb = &mut self.solar_timestep.heat_balance;
                let intensity = hb.zone_bm_sol_fr_int_wins_rep.read([enclosure_number])? / enclosure.total_surface_area;
                hb.surf_int_bm_inc_ins_surf_intens_rep.set([number],intensity)?;
                let divider = *geometry.window_arrays.as_ref().ok_or(Error::Unavailable("same divider area"))?.surf_win_divider_area.get(number-1).ok_or(Error::Unavailable("actual divider bounds"))?;
                let amount = intensity * (surface.row.area + divider);
                hb.surf_int_bm_inc_ins_surf_amount_rep.set([number],amount)?;
                hb.surf_int_bm_inc_ins_surf_amount_rep_energy.set([number],amount * globals.time_step_zone_sec)?;
            }
        }
        for number in 1..=usize::try_from(self.num_solar_enclosures).map_err(|_| Error::Unavailable("actual enclosure count"))? {
            let multiplier = self.solar.as_ref().and_then(|rows| rows.get(number-1)).ok_or(Error::Unavailable("same Source Solar enclosure"))?.sol_vmult;
            let hb = &mut self.solar_timestep.heat_balance;
            hb.encl_sol_qsw_rad.set([number],hb.encl_sol_qsw_rad.read([number])? * multiplier)?;
            hb.encl_sol_qsw_rad_lights.set([number],hb.encl_sol_qsw_rad_lights.read([number])? * multiplier)?;
        }
        let zone_rows = zones.zones.as_ref().ok_or(Error::Unavailable("actual Zone allocation"))?;
        let spaces = zones.spaces.as_ref().ok_or(Error::Unavailable("actual Space allocation"))?;
        for zone in zone_rows { for &space_number in &zone.space_indexes {
            let space = spaces.get(ordinal(space_number)?-1).ok_or(Error::Unavailable("actual Source Space association"))?;
            for number in space.opaque_or_intmass_first..=space.opaque_or_intmass_last {
                let number = ordinal(number)?;
                let surface = &geometry.final_surfaces.as_ref().unwrap()[number-1].row;
                let enclosure = ordinal(surface.solar_encl_index)?;
                let construction = control.construction_usage.get(ordinal(surface.construction)?-1).ok_or(Error::Unavailable("same Source Construction"))?;
                let absorption = get(&arrays.surf_abs_solar_int,number)?;
                // Original3894 deliberately uses InsideAbsorpSolar for the
                // visible lights operand. Do not replace it with AbsorpVis.
                let visible_absorption = construction.inside_absorp_solar;
                add(&mut arrays.surf_opaq_q_rad_sw_in_abs,number,self.solar_timestep.heat_balance.encl_sol_qsw_rad.read([enclosure])? * absorption)?;
                add(&mut arrays.surf_qdot_rad_lights_in_per_area,number,self.solar_timestep.heat_balance.encl_sol_qsw_rad_lights.read([enclosure])? * visible_absorption)?;
                if geometry.any_movable_insulation { return Err(Error::Unavailable("InitIntSolarDistribution actual movable-insulation child3900")); }
                let initial = get(&arrays.surf_opaq_initial_dif_sol_in_abs,number)?;
                add(&mut arrays.surf_opaq_q_rad_sw_in_abs,number,initial)?;
            }
            for _ in space.window_first..=space.window_last { return Err(Error::Unavailable("InitIntSolarDistribution actual window radiant branch3927")); }
        } }
        // DaylightingDevices1506-1564 has no unconditional prefix writes. The
        // genuine TDDPipe.size() loop is empty only on its sole actual owner.
        for _ in modules.tdd_pipe.as_deref().unwrap_or_default() {
            return Err(Error::Unavailable("DistributeTDDAbsorbedSolar actual positive TDDPipe"));
        }
        Ok(())
    }
}
