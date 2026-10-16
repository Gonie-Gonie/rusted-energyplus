//! Source Solar AllocateModuleArrays887-1088; genuine output child follows.
use super::*;
impl OrdinarySolarRadiation {
    /// Actual HBManager2836-2858 child, immediately after the real Zone and
    /// Space AirHB allocations. No AirHB storage or setup flag is duplicated.
    pub(crate) fn allocate_source_solar_enclosure_arrays(&mut self) -> Result<(), Error> {
        let count = usize::try_from(self.num_solar_enclosures)
            .map_err(|_| Error::Unavailable("actual NumOfSolarEnclosures"))?;
        self.solar_timestep.heat_balance.encl_sol_qs_dif_sol.dimension([count], None)?;
        self.solar_timestep.heat_balance.encl_sol_qd.dimension([count], None)?;
        self.solar_timestep.heat_balance.encl_sol_q_dfor_daylight.dimension([count], None)?;
        self.solar_timestep.heat_balance.encl_sol_db.dimension([count], None)?;
        self.solar_timestep.heat_balance.encl_sol_dbssg.dimension([count], None)?;
        self.solar_timestep.heat_balance.encl_sol_db_int_win.dimension([count], None)?;
        self.solar_timestep.heat_balance.encl_sol_qsw_rad.dimension([count], None)?;
        self.solar_timestep.heat_balance.encl_sol_qsw_rad_lights.dimension([count], None)?;
        for number in 1..=count {
            self.solar_timestep.heat_balance.encl_sol_qs_dif_sol.set([number], 0.0)?;
            self.solar_timestep.heat_balance.encl_sol_qd.set([number], 0.0)?;
            self.solar_timestep.heat_balance.encl_sol_q_dfor_daylight.set([number], 0.0)?;
            self.solar_timestep.heat_balance.encl_sol_qsw_rad.set([number], 0.0)?;
            self.solar_timestep.heat_balance.encl_sol_qsw_rad_lights.set([number], 0.0)?;
            self.solar_timestep.heat_balance.encl_sol_db.set([number], 0.0)?;
            self.solar_timestep.heat_balance.encl_sol_dbssg.set([number], 0.0)?;
            self.solar_timestep.heat_balance.encl_sol_db_int_win.set([number], 0.0)?;
        }
        Ok(())
    }

    pub(crate) fn allocate_source_solar_module_arrays(
        &mut self, geometry: &mut SurfaceGeometryInputState, zones: &SelectedZoneInputOwners,
        control: &SelectedHeatBalanceInputState, globals: &WeatherGlobalState,
        arrays: &mut SourceSurfaceHeatBalanceArrays,
        children: &mut dyn SourceSolarChildren, backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        let surfaces = surface_count(geometry)?;
        let steps = usize::try_from(globals.time_steps_in_hour)
            .map_err(|_| Error::Unavailable("actual TimeStepsInHour"))?;
        let enclosures = usize::try_from(self.num_solar_enclosures)
            .map_err(|_| Error::Unavailable("actual NumOfSolarEnclosures"))?;
        let zones_count = zones.zones.as_ref().ok_or(Error::Unavailable("actual Zone allocation"))?.len();
        let vertices = geometry.coordinate.max_vertices_per_surface;
        let max_back = usize::try_from(self.solar_timestep.complex.max_back_surface)
            .map_err(|_| Error::Unavailable("actual DataBSDFWindow MaxBkSurf"))?;
        let layers = geometry.window_arrays.as_ref().ok_or(Error::Unavailable("actual Surface window allocation"))?.solid_window_columns;
        let _ = control.window_thermal_model.ok_or(Error::Unavailable("actual Window thermal model object"))?;
        let _ = control.window_optical_model.ok_or(Error::Unavailable("actual Window optical model object"))?;
        self.solar_timestep.shading.surf_sun_cos_theta.dimension([surfaces], Some(0.0))?; // 887
        self.solar_timestep.shading.surf_sunlit_area.dimension([surfaces], Some(0.0))?; // 888
        self.solar_timestep.shading.surf_win_int_beam_abs_by_shad_fac.dimension([surfaces], None)?; // 890
        self.solar_timestep.shading.surf_win_ext_beam_abs_by_shad_fac.dimension([surfaces], None)?; // 891
        self.solar_timestep.shading.surf_win_trans_bm_solar.dimension([surfaces], None)?; // 892
        self.solar_timestep.shading.surf_win_trans_dif_solar.dimension([surfaces], None)?; // 893
        self.solar_timestep.shading.surf_win_trans_dif_solar_gnd.dimension([surfaces], None)?; // 894
        self.solar_timestep.shading.surf_win_trans_dif_solar_sky.dimension([surfaces], None)?; // 895
        self.solar_timestep.shading.surf_win_trans_bm_bm_solar.dimension([surfaces], None)?; // 896
        self.solar_timestep.shading.surf_win_trans_bm_dif_solar.dimension([surfaces], None)?; // 897
        self.solar_timestep.shading.surf_aniso_sky_mult.dimension([surfaces], Some(1.0))?; // 899
        self.solar_timestep.shading.surf_int_abs_fac.dimension([surfaces], Some(0.0))?; // 900
        self.solar_timestep.shading.surf_mult_iso_sky.dimension([surfaces], Some(0.0))?; // 914
        self.solar_timestep.shading.surf_mult_circum_solar.dimension([surfaces], Some(0.0))?; // 915
        self.solar_timestep.shading.surf_mult_horizon_zenith.dimension([surfaces], Some(0.0))?; // 916
        self.solar_timestep.shading.surf_win_reveal_status.dimension([24, steps, surfaces], Some(0))?; // 917
        self.solar_timestep.max_hc_array_bounds = i32::try_from(2 * (vertices + 1)).map_err(|_| Error::Unavailable("actual MAXHCArrayBounds"))?;
        self.solar_timestep.max_hc_array_increment = i32::try_from(vertices + 1).map_err(|_| Error::Unavailable("actual MAXHCArrayIncrement"))?;
        self.solar_timestep.shading.xtemp.dimension([2 * (vertices + 1)], Some(0.0))?; // 922
        self.solar_timestep.shading.ytemp.dimension([2 * (vertices + 1)], Some(0.0))?; // 923
        self.solar_timestep.shading.xvc.dimension([vertices + 1], Some(0.0))?; // 924
        self.solar_timestep.shading.xvs.dimension([vertices + 1], Some(0.0))?; // 925
        self.solar_timestep.shading.yvc.dimension([vertices + 1], Some(0.0))?; // 926
        self.solar_timestep.shading.yvs.dimension([vertices + 1], Some(0.0))?; // 927
        self.solar_timestep.shading.zvc.dimension([vertices + 1], Some(0.0))?; // 928
        self.solar_timestep.shading.atemp.dimension([2 * (vertices + 1)], Some(0.0))?; // 931
        self.solar_timestep.shading.btemp.dimension([2 * (vertices + 1)], Some(0.0))?; // 932
        self.solar_timestep.shading.ctemp.dimension([2 * (vertices + 1)], Some(0.0))?; // 933
        self.solar_timestep.shading.xtemp1.dimension([2 * (vertices + 1)], Some(0.0))?; // 934
        self.solar_timestep.shading.ytemp1.dimension([2 * (vertices + 1)], Some(0.0))?; // 935
        geometry.window_arrays.as_mut().ok_or(Error::Unavailable("same DataSurface arrays"))?.surf_sun_cos_hourly.dimension([24], None)?; // 937
        for hour in 1..=24 { geometry.window_arrays.as_mut().unwrap().surf_sun_cos_hourly.set([hour], [0.0; 3])?; }
        geometry.window_arrays.as_mut().ok_or(Error::Unavailable("same DataSurface arrays"))?.surf_sunlit_area.dimension([surfaces], Some(0.0))?; // 941
        geometry.window_arrays.as_mut().ok_or(Error::Unavailable("same DataSurface arrays"))?.surf_sunlit_frac.dimension([surfaces], Some(0.0))?; // 942
        arrays.surf_sky_solar_inc = Some(vec![0.0; surfaces]); // 943, same later HBSM field
        arrays.surf_gnd_solar_inc = Some(vec![0.0; surfaces]); // 944, same later HBSM field
        geometry.window_arrays.as_mut().ok_or(Error::Unavailable("same DataSurface arrays"))?.surf_bm_to_bm_refl_fac_obs.dimension([surfaces], Some(0.0))?; // 945
        geometry.window_arrays.as_mut().ok_or(Error::Unavailable("same DataSurface arrays"))?.surf_bm_to_diff_refl_fac_obs.dimension([surfaces], Some(0.0))?; // 946
        geometry.window_arrays.as_mut().ok_or(Error::Unavailable("same DataSurface arrays"))?.surf_bm_to_diff_refl_fac_gnd.dimension([surfaces], Some(0.0))?; // 947
        geometry.window_arrays.as_mut().ok_or(Error::Unavailable("same DataSurface arrays"))?.surf_sky_diff_refl_fac_gnd.dimension([surfaces], Some(0.0))?; // 948
        geometry.window_arrays.as_mut().ok_or(Error::Unavailable("same DataSurface arrays"))?.surf_opaq_ai.dimension([surfaces], Some(0.0))?; // 949
        geometry.window_arrays.as_mut().ok_or(Error::Unavailable("same DataSurface arrays"))?.surf_opaq_ao.dimension([surfaces], Some(0.0))?; // 950
        geometry.window_arrays.as_mut().ok_or(Error::Unavailable("same DataSurface arrays"))?.surf_win_trans_solar.dimension([surfaces], Some(0.0))?; // 951
        geometry.window_arrays.as_mut().ok_or(Error::Unavailable("same DataSurface arrays"))?.surf_win_bm_solar.dimension([surfaces], Some(0.0))?; // 952
        geometry.window_arrays.as_mut().ok_or(Error::Unavailable("same DataSurface arrays"))?.surf_win_bm_bm_solar.dimension([surfaces], Some(0.0))?; // 953
        geometry.window_arrays.as_mut().ok_or(Error::Unavailable("same DataSurface arrays"))?.surf_win_bm_dif_solar.dimension([surfaces], Some(0.0))?; // 954
        geometry.window_arrays.as_mut().ok_or(Error::Unavailable("same DataSurface arrays"))?.surf_win_dif_solar.dimension([surfaces], Some(0.0))?; // 955
        geometry.window_arrays.as_mut().ok_or(Error::Unavailable("same DataSurface arrays"))?.surf_win_heat_gain.dimension([surfaces], Some(0.0))?; // 956
        geometry.window_arrays.as_mut().ok_or(Error::Unavailable("same DataSurface arrays"))?.surf_win_heat_gain_rep.dimension([surfaces], Some(0.0))?; // 957
        geometry.window_arrays.as_mut().ok_or(Error::Unavailable("same DataSurface arrays"))?.surf_win_heat_loss_rep.dimension([surfaces], Some(0.0))?; // 958
        geometry.window_arrays.as_mut().ok_or(Error::Unavailable("same DataSurface arrays"))?.surf_win_gain_conv_glaz_to_zone_rep.dimension([surfaces], Some(0.0))?; // 959
        geometry.window_arrays.as_mut().ok_or(Error::Unavailable("same DataSurface arrays"))?.surf_win_gain_ir_glaz_to_zone_rep.dimension([surfaces], Some(0.0))?; // 960
        geometry.window_arrays.as_mut().ok_or(Error::Unavailable("same DataSurface arrays"))?.surf_win_loss_sw_zone_to_out_win_rep.dimension([surfaces], Some(0.0))?; // 961
        geometry.window_arrays.as_mut().ok_or(Error::Unavailable("same DataSurface arrays"))?.surf_win_gain_frame_divider_to_zone_rep.dimension([surfaces], Some(0.0))?; // 962
        geometry.window_arrays.as_mut().ok_or(Error::Unavailable("same DataSurface arrays"))?.surf_win_gain_conv_shade_to_zone_rep.dimension([surfaces], Some(0.0))?; // 963
        geometry.window_arrays.as_mut().ok_or(Error::Unavailable("same DataSurface arrays"))?.surf_win_gain_ir_shade_to_zone_rep.dimension([surfaces], Some(0.0))?; // 964
        geometry.window_arrays.as_mut().ok_or(Error::Unavailable("same DataSurface arrays"))?.surf_win_gap_conv_ht_flow_rep.dimension([surfaces], Some(0.0))?; // 965
        geometry.window_arrays.as_mut().ok_or(Error::Unavailable("same DataSurface arrays"))?.surf_win_shading_absorbed_solar.dimension([surfaces], Some(0.0))?; // 966
        geometry.window_arrays.as_mut().ok_or(Error::Unavailable("same DataSurface arrays"))?.surf_win_sys_sol_transmittance.dimension([surfaces], Some(0.0))?; // 967
        geometry.window_arrays.as_mut().ok_or(Error::Unavailable("same DataSurface arrays"))?.surf_win_sys_sol_reflectance.dimension([surfaces], Some(0.0))?; // 968
        geometry.window_arrays.as_mut().ok_or(Error::Unavailable("same DataSurface arrays"))?.surf_win_sys_sol_absorptance.dimension([surfaces], Some(0.0))?; // 969
        geometry.window_arrays.as_mut().ok_or(Error::Unavailable("same DataSurface arrays"))?.surf_win_inside_glass_condensation_flag.dimension([surfaces], Some(0))?; // 970
        geometry.window_arrays.as_mut().ok_or(Error::Unavailable("same DataSurface arrays"))?.surf_win_inside_frame_condensation_flag.dimension([surfaces], Some(0))?; // 971
        geometry.window_arrays.as_mut().ok_or(Error::Unavailable("same DataSurface arrays"))?.surf_win_inside_divider_condensation_flag.dimension([surfaces], Some(0))?; // 972
        self.solar_timestep.heat_balance.surf_sunlit_frac_hr.dimension([24, surfaces], Some(0.0))?; // 974
        self.solar_timestep.heat_balance.surf_sunlit_frac.dimension([24, steps, surfaces], Some(0.0))?; // 975
        self.solar_timestep.heat_balance.surf_sunlit_frac_without_reveal.dimension([24, steps, surfaces], Some(0.0))?; // 976
        self.solar_timestep.heat_balance.surf_win_back_surfaces.dimension([24, steps, max_back, surfaces], Some(0))?; // 977
        self.solar_timestep.heat_balance.surf_win_overlap_areas.dimension([24, steps, max_back, surfaces], Some(0.0))?; // 979
        self.solar_timestep.heat_balance.surf_cos_inc_ang_hr.dimension([24, surfaces], Some(0.0))?; // 981
        self.solar_timestep.heat_balance.surf_cos_inc_ang.dimension([24, steps, surfaces], Some(0.0))?; // 982
        self.solar_timestep.heat_balance.zone_trans_solar.dimension([enclosures], Some(0.0))?; // 984
        self.solar_timestep.heat_balance.zone_bm_sol_fr_ext_wins_rep.dimension([enclosures], Some(0.0))?; // 985
        self.solar_timestep.heat_balance.zone_bm_sol_fr_int_wins_rep.dimension([enclosures], Some(0.0))?; // 986
        self.solar_timestep.heat_balance.encl_sol_initial_dif_sol_refl_w.dimension([enclosures], Some(0.0))?; // 987
        self.solar_timestep.heat_balance.zone_dif_sol_fr_ext_wins_rep.dimension([enclosures], Some(0.0))?; // 988
        self.solar_timestep.heat_balance.zone_dif_sol_fr_int_wins_rep.dimension([enclosures], Some(0.0))?; // 989
        self.solar_timestep.heat_balance.zone_win_heat_gain.dimension([zones_count], Some(0.0))?; // 990
        self.solar_timestep.heat_balance.zone_win_heat_gain_rep.dimension([zones_count], Some(0.0))?; // 991
        self.solar_timestep.heat_balance.zone_win_heat_loss_rep.dimension([zones_count], Some(0.0))?; // 992
        self.solar_timestep.heat_balance.zone_opaq_surf_ins_face_cond.dimension([zones_count], Some(0.0))?; // 993
        self.solar_timestep.heat_balance.zone_opaq_surf_ins_face_cond_gain_rep.dimension([zones_count], Some(0.0))?; // 994
        self.solar_timestep.heat_balance.zone_opaq_surf_ins_face_cond_loss_rep.dimension([zones_count], Some(0.0))?; // 995
        self.solar_timestep.heat_balance.zone_opaq_surf_ext_face_cond.dimension([zones_count], Some(0.0))?; // 996
        self.solar_timestep.heat_balance.zone_opaq_surf_ext_face_cond_gain_rep.dimension([zones_count], Some(0.0))?; // 997
        self.solar_timestep.heat_balance.zone_opaq_surf_ext_face_cond_loss_rep.dimension([zones_count], Some(0.0))?; // 998
        self.solar_timestep.heat_balance.surf_q_rad_sw_out_incident.dimension([surfaces], Some(0.0))?; // 1000
        self.solar_timestep.heat_balance.surf_q_rad_sw_out_incident_beam.dimension([surfaces], Some(0.0))?; // 1001
        self.solar_timestep.heat_balance.surf_bm_inc_ins_surf_intens_rep.dimension([surfaces], Some(0.0))?; // 1002
        self.solar_timestep.heat_balance.surf_bm_inc_ins_surf_amount_rep.dimension([surfaces], Some(0.0))?; // 1003
        self.solar_timestep.heat_balance.surf_int_bm_inc_ins_surf_intens_rep.dimension([surfaces], Some(0.0))?; // 1008
        self.solar_timestep.heat_balance.surf_int_bm_inc_ins_surf_amount_rep.dimension([surfaces], Some(0.0))?; // 1009
        self.solar_timestep.heat_balance.surf_q_rad_sw_out_incident_sky_diffuse.dimension([surfaces], Some(0.0))?; // 1014
        self.solar_timestep.heat_balance.surf_q_rad_sw_out_incident_gnd_diffuse.dimension([surfaces], Some(0.0))?; // 1015
        self.solar_timestep.heat_balance.surf_q_rad_sw_out_inc_bm_to_diff_refl_gnd.dimension([surfaces], Some(0.0))?; // 1016
        self.solar_timestep.heat_balance.surf_q_rad_sw_out_inc_sky_diff_refl_gnd.dimension([surfaces], Some(0.0))?; // 1017
        self.solar_timestep.heat_balance.surf_q_rad_sw_out_inc_bm_to_bm_refl_obs.dimension([surfaces], Some(0.0))?; // 1018
        self.solar_timestep.heat_balance.surf_q_rad_sw_out_inc_bm_to_diff_refl_obs.dimension([surfaces], Some(0.0))?; // 1019
        self.solar_timestep.heat_balance.surf_q_rad_sw_out_inc_sky_diff_refl_obs.dimension([surfaces], Some(0.0))?; // 1020
        self.solar_timestep.heat_balance.surf_cos_incidence_angle.dimension([surfaces], Some(0.0))?; // 1021
        self.solar_timestep.heat_balance.surf_win_bsdf_beam_direction_rep.dimension([surfaces], Some(0))?; // 1023
        self.solar_timestep.heat_balance.surf_win_bsdf_beam_theta_rep.dimension([surfaces], Some(0.0))?; // 1024
        self.solar_timestep.heat_balance.surf_win_bsdf_beam_phi_rep.dimension([surfaces], Some(0.0))?; // 1025
        self.solar_timestep.heat_balance.surf_win_q_rad_s_wwin_abs_tot.dimension([surfaces], Some(0.0))?; // 1026
        self.solar_timestep.heat_balance.surf_win_q_rad_s_wwin_abs_layer.dimension([surfaces, layers], Some(0.0))?; // 1027
        self.solar_timestep.heat_balance.surf_win_fen_lay_surf_temp_front.dimension([surfaces, layers], Some(0.0))?; // 1028
        self.solar_timestep.heat_balance.surf_win_fen_lay_surf_temp_back.dimension([surfaces, layers], Some(0.0))?; // 1029
        self.solar_timestep.heat_balance.surf_win_s_wwin_abs_total_report.dimension([surfaces], Some(0.0))?; // 1031
        self.solar_timestep.heat_balance.surf_initial_dif_sol_in_abs_report.dimension([surfaces], Some(0.0))?; // 1032
        self.solar_timestep.heat_balance.surf_win_initial_dif_sol_in_trans_report.dimension([surfaces], Some(0.0))?; // 1033
        self.solar_timestep.heat_balance.surf_sw_in_abs_total_report.dimension([surfaces], Some(0.0))?; // 1034
        geometry.window_arrays.as_mut().ok_or(Error::Unavailable("same DataSurface arrays"))?.surf_win_trans_solar_energy.dimension([surfaces], Some(0.0))?; // 1037
        geometry.window_arrays.as_mut().ok_or(Error::Unavailable("same DataSurface arrays"))?.surf_win_bm_solar_energy.dimension([surfaces], Some(0.0))?; // 1038
        geometry.window_arrays.as_mut().ok_or(Error::Unavailable("same DataSurface arrays"))?.surf_win_bm_bm_solar_energy.dimension([surfaces], Some(0.0))?; // 1039
        geometry.window_arrays.as_mut().ok_or(Error::Unavailable("same DataSurface arrays"))?.surf_win_bm_dif_solar_energy.dimension([surfaces], Some(0.0))?; // 1040
        geometry.window_arrays.as_mut().ok_or(Error::Unavailable("same DataSurface arrays"))?.surf_win_dif_solar_energy.dimension([surfaces], Some(0.0))?; // 1042
        geometry.window_arrays.as_mut().ok_or(Error::Unavailable("same DataSurface arrays"))?.surf_win_heat_gain_rep_energy.dimension([surfaces], Some(0.0))?; // 1043
        geometry.window_arrays.as_mut().ok_or(Error::Unavailable("same DataSurface arrays"))?.surf_win_heat_loss_rep_energy.dimension([surfaces], Some(0.0))?; // 1044
        geometry.window_arrays.as_mut().ok_or(Error::Unavailable("same DataSurface arrays"))?.surf_win_gap_conv_ht_flow_rep_energy.dimension([surfaces], Some(0.0))?; // 1045
        geometry.window_arrays.as_mut().ok_or(Error::Unavailable("same DataSurface arrays"))?.surf_win_heat_transfer_rep_energy.dimension([surfaces], Some(0.0))?; // 1046
        geometry.window_arrays.as_mut().ok_or(Error::Unavailable("same DataSurface arrays"))?.surf_win_shading_absorbed_solar_energy.dimension([surfaces], Some(0.0))?; // 1047
        self.solar_timestep.heat_balance.zone_trans_solar_energy.dimension([enclosures], Some(0.0))?; // 1049
        self.solar_timestep.heat_balance.zone_bm_sol_fr_ext_wins_rep_energy.dimension([enclosures], Some(0.0))?; // 1050
        self.solar_timestep.heat_balance.zone_bm_sol_fr_int_wins_rep_energy.dimension([enclosures], Some(0.0))?; // 1051
        self.solar_timestep.heat_balance.zone_dif_sol_fr_ext_wins_rep_energy.dimension([enclosures], Some(0.0))?; // 1052
        self.solar_timestep.heat_balance.zone_dif_sol_fr_int_wins_rep_energy.dimension([enclosures], Some(0.0))?; // 1053
        self.solar_timestep.heat_balance.zone_win_heat_gain_rep_energy.dimension([zones_count], Some(0.0))?; // 1054
        self.solar_timestep.heat_balance.zone_win_heat_loss_rep_energy.dimension([zones_count], Some(0.0))?; // 1055
        self.solar_timestep.heat_balance.zn_opq_surf_ins_face_cond_gn_rep_enrg.dimension([zones_count], Some(0.0))?; // 1057
        self.solar_timestep.heat_balance.zn_opq_surf_ins_face_cond_ls_rep_enrg.dimension([zones_count], Some(0.0))?; // 1058
        self.solar_timestep.heat_balance.zn_opq_surf_ext_face_cond_gn_rep_enrg.dimension([zones_count], Some(0.0))?; // 1059
        self.solar_timestep.heat_balance.zn_opq_surf_ext_face_cond_ls_rep_enrg.dimension([zones_count], Some(0.0))?; // 1060
        self.solar_timestep.heat_balance.surf_bm_inc_ins_surf_amount_rep_energy.dimension([surfaces], Some(0.0))?; // 1063
        self.solar_timestep.heat_balance.surf_int_bm_inc_ins_surf_amount_rep_energy.dimension([surfaces], Some(0.0))?; // 1064
        self.solar_timestep.heat_balance.surf_win_q_rad_s_wwin_abs_tot_energy.dimension([surfaces], Some(0.0))?; // 1067
        for number in 1..=surfaces {
            let windows = geometry.window_arrays.as_mut().ok_or(Error::Unavailable("same DataSurface reveal arrays"))?;
            *windows.surf_win_bm_sol_absd_outs_reveal.get_mut(number - 1).ok_or(Error::Unavailable("actual reveal bounds"))? = 0.0;
            *windows.surf_win_bm_sol_refld_outs_reveal_report.get_mut(number - 1).ok_or(Error::Unavailable("actual reveal bounds"))? = 0.0;
            *windows.surf_win_bm_sol_absd_ins_reveal.get_mut(number - 1).ok_or(Error::Unavailable("actual reveal bounds"))? = 0.0;
            *windows.surf_win_bm_sol_refld_ins_reveal.get_mut(number - 1).ok_or(Error::Unavailable("actual reveal bounds"))? = 0.0;
            *windows.surf_win_bm_sol_refld_ins_reveal_report.get_mut(number - 1).ok_or(Error::Unavailable("actual reveal bounds"))? = 0.0;
            *windows.surf_win_outs_reveal_diff_onto_glazing.get_mut(number - 1).ok_or(Error::Unavailable("actual reveal bounds"))? = 0.0;
            *windows.surf_win_ins_reveal_diff_onto_glazing.get_mut(number - 1).ok_or(Error::Unavailable("actual reveal bounds"))? = 0.0;
            *windows.surf_win_ins_reveal_diff_into_zone.get_mut(number - 1).ok_or(Error::Unavailable("actual reveal bounds"))? = 0.0;
            *windows.surf_win_outs_reveal_diff_onto_frame.get_mut(number - 1).ok_or(Error::Unavailable("actual reveal bounds"))? = 0.0;
            *windows.surf_win_ins_reveal_diff_onto_frame.get_mut(number - 1).ok_or(Error::Unavailable("actual reveal bounds"))? = 0.0;
        }
        for number in 1..=surfaces {
            let windows = geometry.window_arrays.as_mut().ok_or(Error::Unavailable("same DataSurface reveal report arrays"))?;
            *windows.surf_win_ins_reveal_diff_onto_glazing_report.get_mut(number - 1).ok_or(Error::Unavailable("actual reveal report bounds"))? = 0.0;
            *windows.surf_win_ins_reveal_diff_into_zone_report.get_mut(number - 1).ok_or(Error::Unavailable("actual reveal report bounds"))? = 0.0;
            *windows.surf_win_ins_reveal_diff_onto_frame_report.get_mut(number - 1).ok_or(Error::Unavailable("actual reveal report bounds"))? = 0.0;
            *windows.surf_win_bm_sol_absd_ins_reveal_report.get_mut(number - 1).ok_or(Error::Unavailable("actual reveal report bounds"))? = 0.0;
        }
        // Source1090-2600: mandatory real registration child, not an inferred
        // successful unrequested-output shortcut.
        children.register_solar_module_outputs(self, geometry, zones, backend)
    }
}
