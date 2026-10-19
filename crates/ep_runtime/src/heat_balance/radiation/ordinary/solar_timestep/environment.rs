//! Exact InitSolarCalculations237-362 BeginEnvrn writes.
use super::*;
impl OrdinarySolarRadiation {
    pub(super) fn source_reset_solar_environment(
        &mut self,
        geometry: &mut SurfaceGeometryInputState,
        zones: &SelectedZoneInputOwners,
        _arrays: &mut SourceSurfaceHeatBalanceArrays,
    ) -> Result<(), Error> {
        self.solar_timestep.shading.surf_sun_cos_theta.fill(0.0)?; // Source240
        self.solar_timestep.shading.surf_sunlit_area.fill(0.0)?; // Source241
        geometry
            .window_arrays
            .as_mut()
            .ok_or(Error::Unavailable("same DataSurface Solar allocation"))?
            .surf_sunlit_area
            .fill(0.0)?; // Source242
        geometry
            .window_arrays
            .as_mut()
            .ok_or(Error::Unavailable("same DataSurface Solar allocation"))?
            .surf_sunlit_frac
            .fill(0.0)?; // Source243
        self.solar_timestep
            .heat_balance
            .surf_sunlit_frac_hr
            .fill(0.0)?; // Source244
        self.solar_timestep
            .heat_balance
            .surf_sunlit_frac
            .fill(0.0)?; // Source245
        self.solar_timestep
            .heat_balance
            .surf_sunlit_frac_without_reveal
            .fill(0.0)?; // Source246
        self.solar_timestep
            .heat_balance
            .surf_win_back_surfaces
            .fill(0)?; // Source247
        self.solar_timestep
            .heat_balance
            .surf_win_overlap_areas
            .fill(0.0)?; // Source248
        self.solar_timestep
            .heat_balance
            .surf_cos_inc_ang_hr
            .fill(0.0)?; // Source249
        self.solar_timestep
            .heat_balance
            .surf_cos_inc_ang
            .fill(0.0)?; // Source250
        self.solar_timestep.shading.surf_aniso_sky_mult.fill(1.0)?; // Source251
        self.solar_timestep.shading.surf_mult_iso_sky.fill(0.0)?; // Source258
        self.solar_timestep
            .shading
            .surf_mult_circum_solar
            .fill(0.0)?; // Source259
        self.solar_timestep
            .shading
            .surf_mult_horizon_zenith
            .fill(0.0)?; // Source260
        self.solar_timestep.shading.surf_win_reveal_status.fill(0)?; // Source261
        for number in 1..=zones
            .zones
            .as_ref()
            .ok_or(Error::Unavailable("actual Zone allocation"))?
            .len()
        {
            self.solar_timestep
                .heat_balance
                .zone_win_heat_gain
                .set([number], 0.0)?; // Source264
            self.solar_timestep
                .heat_balance
                .zone_win_heat_gain_rep
                .set([number], 0.0)?; // Source265
            self.solar_timestep
                .heat_balance
                .zone_win_heat_loss_rep
                .set([number], 0.0)?; // Source266
            self.solar_timestep
                .heat_balance
                .zone_win_heat_gain_rep_energy
                .set([number], 0.0)?; // Source267
            self.solar_timestep
                .heat_balance
                .zone_win_heat_loss_rep_energy
                .set([number], 0.0)?; // Source268
            self.solar_timestep
                .heat_balance
                .zone_opaq_surf_ins_face_cond
                .set([number], 0.0)?; // Source269
            self.solar_timestep
                .heat_balance
                .zone_opaq_surf_ins_face_cond_gain_rep
                .set([number], 0.0)?; // Source270
            self.solar_timestep
                .heat_balance
                .zone_opaq_surf_ins_face_cond_loss_rep
                .set([number], 0.0)?; // Source271
            self.solar_timestep
                .heat_balance
                .zn_opq_surf_ins_face_cond_gn_rep_enrg
                .set([number], 0.0)?; // Source272
            self.solar_timestep
                .heat_balance
                .zn_opq_surf_ins_face_cond_ls_rep_enrg
                .set([number], 0.0)?; // Source273
        }
        for number in 1..=usize::try_from(self.num_solar_enclosures)
            .map_err(|_| Error::Unavailable("actual Solar enclosure count"))?
        {
            self.solar_timestep
                .heat_balance
                .zone_trans_solar
                .set([number], 0.0)?; // Source276
            self.solar_timestep
                .heat_balance
                .zone_bm_sol_fr_ext_wins_rep
                .set([number], 0.0)?; // Source277
            self.solar_timestep
                .heat_balance
                .zone_bm_sol_fr_int_wins_rep
                .set([number], 0.0)?; // Source278
            self.solar_timestep
                .heat_balance
                .encl_sol_initial_dif_sol_refl_w
                .set([number], 0.0)?; // Source279
            self.solar_timestep
                .heat_balance
                .zone_dif_sol_fr_ext_wins_rep
                .set([number], 0.0)?; // Source280
            self.solar_timestep
                .heat_balance
                .zone_dif_sol_fr_int_wins_rep
                .set([number], 0.0)?; // Source281
            self.solar_timestep
                .heat_balance
                .zone_trans_solar_energy
                .set([number], 0.0)?; // Source282
            self.solar_timestep
                .heat_balance
                .zone_bm_sol_fr_ext_wins_rep_energy
                .set([number], 0.0)?; // Source283
            self.solar_timestep
                .heat_balance
                .zone_bm_sol_fr_int_wins_rep_energy
                .set([number], 0.0)?; // Source284
            self.solar_timestep
                .heat_balance
                .zone_dif_sol_fr_ext_wins_rep_energy
                .set([number], 0.0)?; // Source285
            self.solar_timestep
                .heat_balance
                .zone_dif_sol_fr_int_wins_rep_energy
                .set([number], 0.0)?; // Source286
        }
        for number in 1..=surface_count(geometry)? {
            self.solar_timestep
                .heat_balance
                .surf_q_rad_sw_out_incident
                .set([number], 0.0)?; // Source289
            self.solar_timestep
                .heat_balance
                .surf_q_rad_sw_out_incident_beam
                .set([number], 0.0)?; // Source290
            self.solar_timestep
                .heat_balance
                .surf_bm_inc_ins_surf_intens_rep
                .set([number], 0.0)?; // Source291
            self.solar_timestep
                .heat_balance
                .surf_bm_inc_ins_surf_amount_rep
                .set([number], 0.0)?; // Source292
            self.solar_timestep
                .heat_balance
                .surf_int_bm_inc_ins_surf_intens_rep
                .set([number], 0.0)?; // Source293
            self.solar_timestep
                .heat_balance
                .surf_int_bm_inc_ins_surf_amount_rep
                .set([number], 0.0)?; // Source294
            self.solar_timestep
                .heat_balance
                .surf_q_rad_sw_out_incident_sky_diffuse
                .set([number], 0.0)?; // Source295
            self.solar_timestep
                .heat_balance
                .surf_q_rad_sw_out_incident_gnd_diffuse
                .set([number], 0.0)?; // Source296
            self.solar_timestep
                .heat_balance
                .surf_q_rad_sw_out_inc_bm_to_diff_refl_gnd
                .set([number], 0.0)?; // Source297
            self.solar_timestep
                .heat_balance
                .surf_q_rad_sw_out_inc_sky_diff_refl_gnd
                .set([number], 0.0)?; // Source298
            self.solar_timestep
                .heat_balance
                .surf_q_rad_sw_out_inc_bm_to_bm_refl_obs
                .set([number], 0.0)?; // Source299
            self.solar_timestep
                .heat_balance
                .surf_q_rad_sw_out_inc_bm_to_diff_refl_obs
                .set([number], 0.0)?; // Source300
            self.solar_timestep
                .heat_balance
                .surf_q_rad_sw_out_inc_sky_diff_refl_obs
                .set([number], 0.0)?; // Source301
            self.solar_timestep
                .heat_balance
                .surf_cos_incidence_angle
                .set([number], 0.0)?; // Source302
            self.solar_timestep
                .heat_balance
                .surf_sw_in_abs_total_report
                .set([number], 0.0)?; // Source303
            self.solar_timestep
                .heat_balance
                .surf_bm_inc_ins_surf_amount_rep_energy
                .set([number], 0.0)?; // Source304
            self.solar_timestep
                .heat_balance
                .surf_int_bm_inc_ins_surf_amount_rep_energy
                .set([number], 0.0)?; // Source305
            self.solar_timestep
                .heat_balance
                .surf_initial_dif_sol_in_abs_report
                .set([number], 0.0)?; // Source306
        }
        let spaces = zones
            .spaces
            .as_ref()
            .ok_or(Error::Unavailable("same Space allocation"))?;
        for zone in zones
            .zones
            .as_ref()
            .ok_or(Error::Unavailable("same Zone allocation"))?
        {
            for &space in &zone.space_indexes {
                let space = spaces
                    .get(ordinal(space)? - 1)
                    .ok_or(Error::Unavailable("actual Source Space association"))?;
                // Preserve empty-range behavior, checked first-row access and the first unsupported positive-child failure after prior writes.
                #[allow(clippy::never_loop)]
                for _ in space.window_first..=space.window_last {
                    return Err(Error::Unavailable(
                        "InitSolarCalculations actual BeginEnvrn window reset313",
                    ));
                }
            }
        }
        Ok(())
    }
}
