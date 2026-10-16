//! Sole persistent DataSolarShading/DataHeatBalance solar arrays.
use super::array::SourceSolarArray;

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceSolarShadingArrays {
    /// SolarShading.cc887: SurfSunCosTheta.
    pub(crate) surf_sun_cos_theta: SourceSolarArray<f64, 1>,
    /// SolarShading.cc888: SurfSunlitArea.
    pub(crate) surf_sunlit_area: SourceSolarArray<f64, 1>,
    /// SolarShading.cc890: SurfWinIntBeamAbsByShadFac.
    pub(crate) surf_win_int_beam_abs_by_shad_fac: SourceSolarArray<f64, 1>,
    /// SolarShading.cc891: SurfWinExtBeamAbsByShadFac.
    pub(crate) surf_win_ext_beam_abs_by_shad_fac: SourceSolarArray<f64, 1>,
    /// SolarShading.cc892: SurfWinTransBmSolar.
    pub(crate) surf_win_trans_bm_solar: SourceSolarArray<f64, 1>,
    /// SolarShading.cc893: SurfWinTransDifSolar.
    pub(crate) surf_win_trans_dif_solar: SourceSolarArray<f64, 1>,
    /// SolarShading.cc894: SurfWinTransDifSolarGnd.
    pub(crate) surf_win_trans_dif_solar_gnd: SourceSolarArray<f64, 1>,
    /// SolarShading.cc895: SurfWinTransDifSolarSky.
    pub(crate) surf_win_trans_dif_solar_sky: SourceSolarArray<f64, 1>,
    /// SolarShading.cc896: SurfWinTransBmBmSolar.
    pub(crate) surf_win_trans_bm_bm_solar: SourceSolarArray<f64, 1>,
    /// SolarShading.cc897: SurfWinTransBmDifSolar.
    pub(crate) surf_win_trans_bm_dif_solar: SourceSolarArray<f64, 1>,
    /// SolarShading.cc899: SurfAnisoSkyMult.
    pub(crate) surf_aniso_sky_mult: SourceSolarArray<f64, 1>,
    /// SolarShading.cc900: SurfIntAbsFac.
    pub(crate) surf_int_abs_fac: SourceSolarArray<f64, 1>,
    /// SolarShading.cc914: SurfMultIsoSky.
    pub(crate) surf_mult_iso_sky: SourceSolarArray<f64, 1>,
    /// SolarShading.cc915: SurfMultCircumSolar.
    pub(crate) surf_mult_circum_solar: SourceSolarArray<f64, 1>,
    /// SolarShading.cc916: SurfMultHorizonZenith.
    pub(crate) surf_mult_horizon_zenith: SourceSolarArray<f64, 1>,
    /// SolarShading.cc917: SurfWinRevealStatus.
    pub(crate) surf_win_reveal_status: SourceSolarArray<i32, 3>,
    /// SolarShading.cc922: XTEMP.
    pub(crate) xtemp: SourceSolarArray<f64, 1>,
    /// SolarShading.cc923: YTEMP.
    pub(crate) ytemp: SourceSolarArray<f64, 1>,
    /// SolarShading.cc924: XVC.
    pub(crate) xvc: SourceSolarArray<f64, 1>,
    /// SolarShading.cc925: XVS.
    pub(crate) xvs: SourceSolarArray<f64, 1>,
    /// SolarShading.cc926: YVC.
    pub(crate) yvc: SourceSolarArray<f64, 1>,
    /// SolarShading.cc927: YVS.
    pub(crate) yvs: SourceSolarArray<f64, 1>,
    /// SolarShading.cc928: ZVC.
    pub(crate) zvc: SourceSolarArray<f64, 1>,
    /// SolarShading.cc931: ATEMP.
    pub(crate) atemp: SourceSolarArray<f64, 1>,
    /// SolarShading.cc932: BTEMP.
    pub(crate) btemp: SourceSolarArray<f64, 1>,
    /// SolarShading.cc933: CTEMP.
    pub(crate) ctemp: SourceSolarArray<f64, 1>,
    /// SolarShading.cc934: XTEMP1.
    pub(crate) xtemp1: SourceSolarArray<f64, 1>,
    /// SolarShading.cc935: YTEMP1.
    pub(crate) ytemp1: SourceSolarArray<f64, 1>,
    pub(crate) surf_with_shdg_iso_sky: SourceSolarArray<f64, 1>,
    pub(crate) surf_wo_shdg_iso_sky: SourceSolarArray<f64, 1>,
    pub(crate) surf_with_shdg_horiz: SourceSolarArray<f64, 1>,
    pub(crate) surf_wo_shdg_horiz: SourceSolarArray<f64, 1>,
    pub(crate) surf_dif_shdg_ratio_iso_sky: SourceSolarArray<f64, 1>,
    pub(crate) surf_dif_shdg_ratio_horiz: SourceSolarArray<f64, 1>,
    pub(crate) surf_cur_dif_shdg_ratio_iso_sky: SourceSolarArray<f64, 1>,
    pub(crate) surf_dif_shdg_ratio_iso_sky_hrts: SourceSolarArray<f64, 3>,
    pub(crate) surf_dif_shdg_ratio_horiz_hrts: SourceSolarArray<f64, 3>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceSolarHeatBalanceArrays {
    /// SolarShading.cc974: SurfSunlitFracHR.
    pub(crate) surf_sunlit_frac_hr: SourceSolarArray<f64, 2>,
    /// SolarShading.cc975: SurfSunlitFrac.
    pub(crate) surf_sunlit_frac: SourceSolarArray<f64, 3>,
    /// SolarShading.cc976: SurfSunlitFracWithoutReveal.
    pub(crate) surf_sunlit_frac_without_reveal: SourceSolarArray<f64, 3>,
    /// SolarShading.cc977: SurfWinBackSurfaces.
    pub(crate) surf_win_back_surfaces: SourceSolarArray<i32, 4>,
    /// SolarShading.cc979: SurfWinOverlapAreas.
    pub(crate) surf_win_overlap_areas: SourceSolarArray<f64, 4>,
    /// SolarShading.cc981: SurfCosIncAngHR.
    pub(crate) surf_cos_inc_ang_hr: SourceSolarArray<f64, 2>,
    /// SolarShading.cc982: SurfCosIncAng.
    pub(crate) surf_cos_inc_ang: SourceSolarArray<f64, 3>,
    /// SolarShading.cc984: ZoneTransSolar.
    pub(crate) zone_trans_solar: SourceSolarArray<f64, 1>,
    /// SolarShading.cc985: ZoneBmSolFrExtWinsRep.
    pub(crate) zone_bm_sol_fr_ext_wins_rep: SourceSolarArray<f64, 1>,
    /// SolarShading.cc986: ZoneBmSolFrIntWinsRep.
    pub(crate) zone_bm_sol_fr_int_wins_rep: SourceSolarArray<f64, 1>,
    /// SolarShading.cc987: EnclSolInitialDifSolReflW.
    pub(crate) encl_sol_initial_dif_sol_refl_w: SourceSolarArray<f64, 1>,
    /// SolarShading.cc988: ZoneDifSolFrExtWinsRep.
    pub(crate) zone_dif_sol_fr_ext_wins_rep: SourceSolarArray<f64, 1>,
    /// SolarShading.cc989: ZoneDifSolFrIntWinsRep.
    pub(crate) zone_dif_sol_fr_int_wins_rep: SourceSolarArray<f64, 1>,
    /// SolarShading.cc990: ZoneWinHeatGain.
    pub(crate) zone_win_heat_gain: SourceSolarArray<f64, 1>,
    /// SolarShading.cc991: ZoneWinHeatGainRep.
    pub(crate) zone_win_heat_gain_rep: SourceSolarArray<f64, 1>,
    /// SolarShading.cc992: ZoneWinHeatLossRep.
    pub(crate) zone_win_heat_loss_rep: SourceSolarArray<f64, 1>,
    /// SolarShading.cc993: ZoneOpaqSurfInsFaceCond.
    pub(crate) zone_opaq_surf_ins_face_cond: SourceSolarArray<f64, 1>,
    /// SolarShading.cc994: ZoneOpaqSurfInsFaceCondGainRep.
    pub(crate) zone_opaq_surf_ins_face_cond_gain_rep: SourceSolarArray<f64, 1>,
    /// SolarShading.cc995: ZoneOpaqSurfInsFaceCondLossRep.
    pub(crate) zone_opaq_surf_ins_face_cond_loss_rep: SourceSolarArray<f64, 1>,
    /// SolarShading.cc996: ZoneOpaqSurfExtFaceCond.
    pub(crate) zone_opaq_surf_ext_face_cond: SourceSolarArray<f64, 1>,
    /// SolarShading.cc997: ZoneOpaqSurfExtFaceCondGainRep.
    pub(crate) zone_opaq_surf_ext_face_cond_gain_rep: SourceSolarArray<f64, 1>,
    /// SolarShading.cc998: ZoneOpaqSurfExtFaceCondLossRep.
    pub(crate) zone_opaq_surf_ext_face_cond_loss_rep: SourceSolarArray<f64, 1>,
    /// SolarShading.cc1000: SurfQRadSWOutIncident.
    pub(crate) surf_q_rad_sw_out_incident: SourceSolarArray<f64, 1>,
    /// SolarShading.cc1001: SurfQRadSWOutIncidentBeam.
    pub(crate) surf_q_rad_sw_out_incident_beam: SourceSolarArray<f64, 1>,
    /// SolarShading.cc1002: SurfBmIncInsSurfIntensRep.
    pub(crate) surf_bm_inc_ins_surf_intens_rep: SourceSolarArray<f64, 1>,
    /// SolarShading.cc1003: SurfBmIncInsSurfAmountRep.
    pub(crate) surf_bm_inc_ins_surf_amount_rep: SourceSolarArray<f64, 1>,
    /// SolarShading.cc1008: SurfIntBmIncInsSurfIntensRep.
    pub(crate) surf_int_bm_inc_ins_surf_intens_rep: SourceSolarArray<f64, 1>,
    /// SolarShading.cc1009: SurfIntBmIncInsSurfAmountRep.
    pub(crate) surf_int_bm_inc_ins_surf_amount_rep: SourceSolarArray<f64, 1>,
    /// SolarShading.cc1014: SurfQRadSWOutIncidentSkyDiffuse.
    pub(crate) surf_q_rad_sw_out_incident_sky_diffuse: SourceSolarArray<f64, 1>,
    /// SolarShading.cc1015: SurfQRadSWOutIncidentGndDiffuse.
    pub(crate) surf_q_rad_sw_out_incident_gnd_diffuse: SourceSolarArray<f64, 1>,
    /// SolarShading.cc1016: SurfQRadSWOutIncBmToDiffReflGnd.
    pub(crate) surf_q_rad_sw_out_inc_bm_to_diff_refl_gnd: SourceSolarArray<f64, 1>,
    /// SolarShading.cc1017: SurfQRadSWOutIncSkyDiffReflGnd.
    pub(crate) surf_q_rad_sw_out_inc_sky_diff_refl_gnd: SourceSolarArray<f64, 1>,
    /// SolarShading.cc1018: SurfQRadSWOutIncBmToBmReflObs.
    pub(crate) surf_q_rad_sw_out_inc_bm_to_bm_refl_obs: SourceSolarArray<f64, 1>,
    /// SolarShading.cc1019: SurfQRadSWOutIncBmToDiffReflObs.
    pub(crate) surf_q_rad_sw_out_inc_bm_to_diff_refl_obs: SourceSolarArray<f64, 1>,
    /// SolarShading.cc1020: SurfQRadSWOutIncSkyDiffReflObs.
    pub(crate) surf_q_rad_sw_out_inc_sky_diff_refl_obs: SourceSolarArray<f64, 1>,
    /// SolarShading.cc1021: SurfCosIncidenceAngle.
    pub(crate) surf_cos_incidence_angle: SourceSolarArray<f64, 1>,
    /// SolarShading.cc1023: SurfWinBSDFBeamDirectionRep.
    pub(crate) surf_win_bsdf_beam_direction_rep: SourceSolarArray<i32, 1>,
    /// SolarShading.cc1024: SurfWinBSDFBeamThetaRep.
    pub(crate) surf_win_bsdf_beam_theta_rep: SourceSolarArray<f64, 1>,
    /// SolarShading.cc1025: SurfWinBSDFBeamPhiRep.
    pub(crate) surf_win_bsdf_beam_phi_rep: SourceSolarArray<f64, 1>,
    /// SolarShading.cc1026: SurfWinQRadSWwinAbsTot.
    pub(crate) surf_win_q_rad_s_wwin_abs_tot: SourceSolarArray<f64, 1>,
    /// SolarShading.cc1027: SurfWinQRadSWwinAbsLayer.
    pub(crate) surf_win_q_rad_s_wwin_abs_layer: SourceSolarArray<f64, 2>,
    /// SolarShading.cc1028: SurfWinFenLaySurfTempFront.
    pub(crate) surf_win_fen_lay_surf_temp_front: SourceSolarArray<f64, 2>,
    /// SolarShading.cc1029: SurfWinFenLaySurfTempBack.
    pub(crate) surf_win_fen_lay_surf_temp_back: SourceSolarArray<f64, 2>,
    /// SolarShading.cc1031: SurfWinSWwinAbsTotalReport.
    pub(crate) surf_win_s_wwin_abs_total_report: SourceSolarArray<f64, 1>,
    /// SolarShading.cc1032: SurfInitialDifSolInAbsReport.
    pub(crate) surf_initial_dif_sol_in_abs_report: SourceSolarArray<f64, 1>,
    /// SolarShading.cc1033: SurfWinInitialDifSolInTransReport.
    pub(crate) surf_win_initial_dif_sol_in_trans_report: SourceSolarArray<f64, 1>,
    /// SolarShading.cc1034: SurfSWInAbsTotalReport.
    pub(crate) surf_sw_in_abs_total_report: SourceSolarArray<f64, 1>,
    /// SolarShading.cc1049: ZoneTransSolarEnergy.
    pub(crate) zone_trans_solar_energy: SourceSolarArray<f64, 1>,
    /// SolarShading.cc1050: ZoneBmSolFrExtWinsRepEnergy.
    pub(crate) zone_bm_sol_fr_ext_wins_rep_energy: SourceSolarArray<f64, 1>,
    /// SolarShading.cc1051: ZoneBmSolFrIntWinsRepEnergy.
    pub(crate) zone_bm_sol_fr_int_wins_rep_energy: SourceSolarArray<f64, 1>,
    /// SolarShading.cc1052: ZoneDifSolFrExtWinsRepEnergy.
    pub(crate) zone_dif_sol_fr_ext_wins_rep_energy: SourceSolarArray<f64, 1>,
    /// SolarShading.cc1053: ZoneDifSolFrIntWinsRepEnergy.
    pub(crate) zone_dif_sol_fr_int_wins_rep_energy: SourceSolarArray<f64, 1>,
    /// SolarShading.cc1054: ZoneWinHeatGainRepEnergy.
    pub(crate) zone_win_heat_gain_rep_energy: SourceSolarArray<f64, 1>,
    /// SolarShading.cc1055: ZoneWinHeatLossRepEnergy.
    pub(crate) zone_win_heat_loss_rep_energy: SourceSolarArray<f64, 1>,
    /// SolarShading.cc1057: ZnOpqSurfInsFaceCondGnRepEnrg.
    pub(crate) zn_opq_surf_ins_face_cond_gn_rep_enrg: SourceSolarArray<f64, 1>,
    /// SolarShading.cc1058: ZnOpqSurfInsFaceCondLsRepEnrg.
    pub(crate) zn_opq_surf_ins_face_cond_ls_rep_enrg: SourceSolarArray<f64, 1>,
    /// SolarShading.cc1059: ZnOpqSurfExtFaceCondGnRepEnrg.
    pub(crate) zn_opq_surf_ext_face_cond_gn_rep_enrg: SourceSolarArray<f64, 1>,
    /// SolarShading.cc1060: ZnOpqSurfExtFaceCondLsRepEnrg.
    pub(crate) zn_opq_surf_ext_face_cond_ls_rep_enrg: SourceSolarArray<f64, 1>,
    /// SolarShading.cc1063: SurfBmIncInsSurfAmountRepEnergy.
    pub(crate) surf_bm_inc_ins_surf_amount_rep_energy: SourceSolarArray<f64, 1>,
    /// SolarShading.cc1064: SurfIntBmIncInsSurfAmountRepEnergy.
    pub(crate) surf_int_bm_inc_ins_surf_amount_rep_energy: SourceSolarArray<f64, 1>,
    /// SolarShading.cc1067: SurfWinQRadSWwinAbsTotEnergy.
    pub(crate) surf_win_q_rad_s_wwin_abs_tot_energy: SourceSolarArray<f64, 1>,
    pub(crate) encl_sol_qs_dif_sol: SourceSolarArray<f64, 1>,
    pub(crate) encl_sol_qd: SourceSolarArray<f64, 1>,
    pub(crate) encl_sol_q_dfor_daylight: SourceSolarArray<f64, 1>,
    pub(crate) encl_sol_qsw_rad: SourceSolarArray<f64, 1>,
    pub(crate) encl_sol_qsw_rad_lights: SourceSolarArray<f64, 1>,
    pub(crate) encl_sol_db: SourceSolarArray<f64, 1>,
    pub(crate) encl_sol_dbssg: SourceSolarArray<f64, 1>,
    pub(crate) encl_sol_db_int_win: SourceSolarArray<f64, 1>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceShadowCombination {
    pub(crate) use_this_surface: bool,
    pub(crate) num_gen_surface: i32,
    pub(crate) num_back_surface: i32,
    pub(crate) num_sub_surface: i32,
    /// Source lower bound0; None is constructor-unallocated, Some([0]) is allocated0..0.
    pub(crate) general: Option<Vec<i32>>,
    pub(crate) back: Option<Vec<i32>>,
    pub(crate) sub: Option<Vec<i32>>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceSolarTimestepState {
    pub(crate) shading: SourceSolarShadingArrays,
    pub(crate) heat_balance: SourceSolarHeatBalanceArrays,
    pub(crate) shadow_combinations: Option<Vec<SourceShadowCombination>>,
    /// Actual DetermineShadowingCombinations local retained across its report child;
    /// deallocated at5727, never a second global casting-surface owner.
    pub(crate) casting_surface: Option<Vec<bool>>,
    pub(crate) penumbra_ids: Vec<u32>,
    /// Same actual DataBSDFWindow and WindowComplexManager numerical owner.
    pub(crate) complex: SourceComplexSolarState,
    pub(crate) max_hcv: i32,
    pub(crate) max_hc_array_bounds: i32,
    pub(crate) max_hc_array_increment: i32,
    pub(crate) max_gss: i32,
    pub(crate) max_bks: i32,
    pub(crate) max_sbs: i32,
    pub(crate) max_dim: i32,
    pub(crate) ngsshc: i32,
    pub(crate) nbkshc: i32,
    pub(crate) nrvlhc: i32,
    pub(crate) nsbshc: i32,
    pub(crate) lochca: i32,
    pub(crate) calc_sky_dif_shading: bool,
    pub(crate) shadowing_days_left: i32,
    pub(crate) get_input_flag: bool,
    pub(crate) first_time: bool,
    pub(crate) init_complex_once: bool,
    pub(crate) shadow_one_time_flag: bool,
    /// Actual SolarShadingData SUNCOS(3) calls Vector3 uniform-value constructor.
    pub(crate) sun_cosines: [Option<f64>; 3],
    /// The actual BSDF SUNCOSTS owner is borrowed from its own child, not duplicated here.
    pub(crate) sin_phi: Vec<f64>,
    pub(crate) cos_phi: Vec<f64>,
    pub(crate) sin_theta: Vec<f64>,
    pub(crate) cos_theta: Vec<f64>,
    pub(crate) x_vrt: SourceSolarArray<f64, 1>,
    pub(crate) y_vrt: SourceSolarArray<f64, 1>,
    pub(crate) z_vrt: SourceSolarArray<f64, 1>,
    pub(crate) hc_a: SourceSolarArray<i64, 2>,
    pub(crate) hc_b: SourceSolarArray<i64, 2>,
    pub(crate) hc_c: SourceSolarArray<i64, 2>,
    pub(crate) hc_x: SourceSolarArray<i64, 2>,
    pub(crate) hc_y: SourceSolarArray<i64, 2>,
    pub(crate) hc_area: SourceSolarArray<f64, 1>,
    pub(crate) hc_ns: SourceSolarArray<i32, 1>,
    pub(crate) hc_nv: SourceSolarArray<i32, 1>,
    pub(crate) hc_t: SourceSolarArray<f64, 1>,
}
impl Default for SourceSolarTimestepState {
    fn default() -> Self {
        Self {
            shading: SourceSolarShadingArrays::default(), heat_balance: SourceSolarHeatBalanceArrays::default(),
            shadow_combinations: None, casting_surface: None, penumbra_ids: Vec::new(), complex: SourceComplexSolarState::default(), max_hcv: 15,
            max_hc_array_bounds: 0, max_hc_array_increment: 0,
            max_gss: 50, max_bks: 50, max_sbs: 50, max_dim: 0,
            ngsshc: 0, nbkshc: 0, nrvlhc: 0, nsbshc: 0, lochca: 0,
            calc_sky_dif_shading: false, shadowing_days_left: 0,
            get_input_flag: true, first_time: true, init_complex_once: true, shadow_one_time_flag: true,
            sun_cosines: [Some(3.0); 3], sin_phi: Vec::new(), cos_phi: Vec::new(),
            sin_theta: Vec::new(), cos_theta: Vec::new(),
            x_vrt: SourceSolarArray::default(), y_vrt: SourceSolarArray::default(), z_vrt: SourceSolarArray::default(),
            hc_a: SourceSolarArray::default(), hc_b: SourceSolarArray::default(), hc_c: SourceSolarArray::default(),
            hc_x: SourceSolarArray::default(), hc_y: SourceSolarArray::default(), hc_area: SourceSolarArray::default(),
            hc_ns: SourceSolarArray::default(), hc_nv: SourceSolarArray::default(), hc_t: SourceSolarArray::default(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceComplexSolarState {
    pub(crate) total_complex_fenestration_states: i32,
    pub(crate) first_bsdf: i32,
    pub(crate) max_back_surface: i32,
    pub(crate) total_thermal_models: i32,
    pub(crate) sun_costs: SourceSolarArray<[f64; 3], 2>,
    pub(crate) number_complex_windows: i32,
    pub(crate) init_complex_windows_once: bool,
}
impl Default for SourceComplexSolarState {
    fn default() -> Self {
        Self { total_complex_fenestration_states: 0, first_bsdf: 0,
            max_back_surface: 20, total_thermal_models: 0,
            sun_costs: SourceSolarArray { dimensions: Some([60,24]), values: vec![None; 60 * 24] },
            number_complex_windows: 0, init_complex_windows_once: true }
    }
}
