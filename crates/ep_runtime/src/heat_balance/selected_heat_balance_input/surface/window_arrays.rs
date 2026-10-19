//! Literal AllocateSurfaceWindows117–249 storage for every allocated row.
//! Opaque admission does not skip these arrays. Future window solvers are absent.
use super::Error;

// Retained arrays are consumed by later Original stages, not a Ready marker.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SurfaceWindowArrays {
    /// SolarShading.cc937: SurfSunCosHourly, unallocated until Solar AllocateModuleArrays.
    pub(crate) surf_sun_cos_hourly:
        crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray<
            [f64; 3],
            1,
        >,
    /// SolarShading.cc941: SurfSunlitArea, unallocated until Solar AllocateModuleArrays.
    pub(crate) surf_sunlit_area:
        crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray<f64, 1>,
    /// SolarShading.cc942: SurfSunlitFrac, unallocated until Solar AllocateModuleArrays.
    pub(crate) surf_sunlit_frac:
        crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray<f64, 1>,
    /// SolarShading.cc945: SurfBmToBmReflFacObs, unallocated until Solar AllocateModuleArrays.
    pub(crate) surf_bm_to_bm_refl_fac_obs:
        crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray<f64, 1>,
    /// SolarShading.cc946: SurfBmToDiffReflFacObs, unallocated until Solar AllocateModuleArrays.
    pub(crate) surf_bm_to_diff_refl_fac_obs:
        crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray<f64, 1>,
    /// SolarShading.cc947: SurfBmToDiffReflFacGnd, unallocated until Solar AllocateModuleArrays.
    pub(crate) surf_bm_to_diff_refl_fac_gnd:
        crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray<f64, 1>,
    /// SolarShading.cc948: SurfSkyDiffReflFacGnd, unallocated until Solar AllocateModuleArrays.
    pub(crate) surf_sky_diff_refl_fac_gnd:
        crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray<f64, 1>,
    /// SolarShading.cc949: SurfOpaqAI, unallocated until Solar AllocateModuleArrays.
    pub(crate) surf_opaq_ai:
        crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray<f64, 1>,
    /// SolarShading.cc950: SurfOpaqAO, unallocated until Solar AllocateModuleArrays.
    pub(crate) surf_opaq_ao:
        crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray<f64, 1>,
    /// SolarShading.cc951: SurfWinTransSolar, unallocated until Solar AllocateModuleArrays.
    pub(crate) surf_win_trans_solar:
        crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray<f64, 1>,
    /// SolarShading.cc952: SurfWinBmSolar, unallocated until Solar AllocateModuleArrays.
    pub(crate) surf_win_bm_solar:
        crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray<f64, 1>,
    /// SolarShading.cc953: SurfWinBmBmSolar, unallocated until Solar AllocateModuleArrays.
    pub(crate) surf_win_bm_bm_solar:
        crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray<f64, 1>,
    /// SolarShading.cc954: SurfWinBmDifSolar, unallocated until Solar AllocateModuleArrays.
    pub(crate) surf_win_bm_dif_solar:
        crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray<f64, 1>,
    /// SolarShading.cc955: SurfWinDifSolar, unallocated until Solar AllocateModuleArrays.
    pub(crate) surf_win_dif_solar:
        crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray<f64, 1>,
    /// SolarShading.cc956: SurfWinHeatGain, unallocated until Solar AllocateModuleArrays.
    pub(crate) surf_win_heat_gain:
        crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray<f64, 1>,
    /// SolarShading.cc957: SurfWinHeatGainRep, unallocated until Solar AllocateModuleArrays.
    pub(crate) surf_win_heat_gain_rep:
        crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray<f64, 1>,
    /// SolarShading.cc958: SurfWinHeatLossRep, unallocated until Solar AllocateModuleArrays.
    pub(crate) surf_win_heat_loss_rep:
        crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray<f64, 1>,
    /// SolarShading.cc959: SurfWinGainConvGlazToZoneRep, unallocated until Solar AllocateModuleArrays.
    pub(crate) surf_win_gain_conv_glaz_to_zone_rep:
        crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray<f64, 1>,
    /// SolarShading.cc960: SurfWinGainIRGlazToZoneRep, unallocated until Solar AllocateModuleArrays.
    pub(crate) surf_win_gain_ir_glaz_to_zone_rep:
        crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray<f64, 1>,
    /// SolarShading.cc961: SurfWinLossSWZoneToOutWinRep, unallocated until Solar AllocateModuleArrays.
    pub(crate) surf_win_loss_sw_zone_to_out_win_rep:
        crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray<f64, 1>,
    /// SolarShading.cc962: SurfWinGainFrameDividerToZoneRep, unallocated until Solar AllocateModuleArrays.
    pub(crate) surf_win_gain_frame_divider_to_zone_rep:
        crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray<f64, 1>,
    /// SolarShading.cc963: SurfWinGainConvShadeToZoneRep, unallocated until Solar AllocateModuleArrays.
    pub(crate) surf_win_gain_conv_shade_to_zone_rep:
        crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray<f64, 1>,
    /// SolarShading.cc964: SurfWinGainIRShadeToZoneRep, unallocated until Solar AllocateModuleArrays.
    pub(crate) surf_win_gain_ir_shade_to_zone_rep:
        crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray<f64, 1>,
    /// SolarShading.cc965: SurfWinGapConvHtFlowRep, unallocated until Solar AllocateModuleArrays.
    pub(crate) surf_win_gap_conv_ht_flow_rep:
        crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray<f64, 1>,
    /// SolarShading.cc966: SurfWinShadingAbsorbedSolar, unallocated until Solar AllocateModuleArrays.
    pub(crate) surf_win_shading_absorbed_solar:
        crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray<f64, 1>,
    /// SolarShading.cc967: SurfWinSysSolTransmittance, unallocated until Solar AllocateModuleArrays.
    pub(crate) surf_win_sys_sol_transmittance:
        crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray<f64, 1>,
    /// SolarShading.cc968: SurfWinSysSolReflectance, unallocated until Solar AllocateModuleArrays.
    pub(crate) surf_win_sys_sol_reflectance:
        crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray<f64, 1>,
    /// SolarShading.cc969: SurfWinSysSolAbsorptance, unallocated until Solar AllocateModuleArrays.
    pub(crate) surf_win_sys_sol_absorptance:
        crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray<f64, 1>,
    /// SolarShading.cc970: SurfWinInsideGlassCondensationFlag, unallocated until Solar AllocateModuleArrays.
    pub(crate) surf_win_inside_glass_condensation_flag:
        crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray<i32, 1>,
    /// SolarShading.cc971: SurfWinInsideFrameCondensationFlag, unallocated until Solar AllocateModuleArrays.
    pub(crate) surf_win_inside_frame_condensation_flag:
        crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray<i32, 1>,
    /// SolarShading.cc972: SurfWinInsideDividerCondensationFlag, unallocated until Solar AllocateModuleArrays.
    pub(crate) surf_win_inside_divider_condensation_flag:
        crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray<i32, 1>,
    /// SolarShading.cc1037: SurfWinTransSolarEnergy, unallocated until Solar AllocateModuleArrays.
    pub(crate) surf_win_trans_solar_energy:
        crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray<f64, 1>,
    /// SolarShading.cc1038: SurfWinBmSolarEnergy, unallocated until Solar AllocateModuleArrays.
    pub(crate) surf_win_bm_solar_energy:
        crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray<f64, 1>,
    /// SolarShading.cc1039: SurfWinBmBmSolarEnergy, unallocated until Solar AllocateModuleArrays.
    pub(crate) surf_win_bm_bm_solar_energy:
        crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray<f64, 1>,
    /// SolarShading.cc1040: SurfWinBmDifSolarEnergy, unallocated until Solar AllocateModuleArrays.
    pub(crate) surf_win_bm_dif_solar_energy:
        crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray<f64, 1>,
    /// SolarShading.cc1042: SurfWinDifSolarEnergy, unallocated until Solar AllocateModuleArrays.
    pub(crate) surf_win_dif_solar_energy:
        crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray<f64, 1>,
    /// SolarShading.cc1043: SurfWinHeatGainRepEnergy, unallocated until Solar AllocateModuleArrays.
    pub(crate) surf_win_heat_gain_rep_energy:
        crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray<f64, 1>,
    /// SolarShading.cc1044: SurfWinHeatLossRepEnergy, unallocated until Solar AllocateModuleArrays.
    pub(crate) surf_win_heat_loss_rep_energy:
        crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray<f64, 1>,
    /// SolarShading.cc1045: SurfWinGapConvHtFlowRepEnergy, unallocated until Solar AllocateModuleArrays.
    pub(crate) surf_win_gap_conv_ht_flow_rep_energy:
        crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray<f64, 1>,
    /// SolarShading.cc1046: SurfWinHeatTransferRepEnergy, unallocated until Solar AllocateModuleArrays.
    pub(crate) surf_win_heat_transfer_rep_energy:
        crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray<f64, 1>,
    /// SolarShading.cc1047: SurfWinShadingAbsorbedSolarEnergy, unallocated until Solar AllocateModuleArrays.
    pub(crate) surf_win_shading_absorbed_solar_energy:
        crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray<f64, 1>,
    pub(crate) rows: usize,
    pub(crate) equivalent_layer_columns: usize,
    pub(crate) solid_window_columns: usize,
    pub(crate) surf_win_a: Vec<f64>,
    pub(crate) surf_win_a_diff_front: Vec<f64>,
    pub(crate) surf_win_acf_overlap: Vec<f64>,
    pub(crate) surf_win_frame_q_rad_out_abs: Vec<f64>, // SurfWinFrameQRadOutAbs
    pub(crate) surf_win_frame_q_rad_in_abs: Vec<f64>,  // SurfWinFrameQRadInAbs
    pub(crate) surf_win_divider_q_rad_out_abs: Vec<f64>, // SurfWinDividerQRadOutAbs
    pub(crate) surf_win_divider_q_rad_in_abs: Vec<f64>, // SurfWinDividerQRadInAbs
    pub(crate) surf_win_ext_beam_abs_by_shade: Vec<f64>, // SurfWinExtBeamAbsByShade
    pub(crate) surf_win_ext_diff_abs_by_shade: Vec<f64>, // SurfWinExtDiffAbsByShade
    pub(crate) surf_win_int_beam_abs_by_shade: Vec<f64>, // SurfWinIntBeamAbsByShade
    pub(crate) surf_win_int_sw_abs_by_shade: Vec<f64>, // SurfWinIntSWAbsByShade
    pub(crate) surf_win_initial_dif_sol_abs_by_shade: Vec<f64>, // SurfWinInitialDifSolAbsByShade
    pub(crate) surf_win_int_lw_abs_by_shade: Vec<f64>, // SurfWinIntLWAbsByShade
    pub(crate) surf_win_conv_heat_flow_natural: Vec<f64>, // SurfWinConvHeatFlowNatural
    pub(crate) surf_win_conv_heat_gain_to_zone_air: Vec<f64>, // SurfWinConvHeatGainToZoneAir
    pub(crate) surf_win_ret_heat_gain_to_zone_air: Vec<f64>, // SurfWinRetHeatGainToZoneAir
    pub(crate) surf_win_divider_heat_gain: Vec<f64>,   // SurfWinDividerHeatGain
    pub(crate) surf_win_bl_tsol_bm_bm: Vec<f64>,       // SurfWinBlTsolBmBm
    pub(crate) surf_win_bl_tsol_bm_dif: Vec<f64>,      // SurfWinBlTsolBmDif
    pub(crate) surf_win_bl_tsol_dif_dif: Vec<f64>,     // SurfWinBlTsolDifDif
    pub(crate) surf_win_bl_gl_sys_tsol_bm_bm: Vec<f64>, // SurfWinBlGlSysTsolBmBm
    pub(crate) surf_win_bl_gl_sys_tsol_dif_dif: Vec<f64>, // SurfWinBlGlSysTsolDifDif
    pub(crate) surf_win_sc_tsol_bm_bm: Vec<f64>,       // SurfWinScTsolBmBm
    pub(crate) surf_win_sc_tsol_bm_dif: Vec<f64>,      // SurfWinScTsolBmDif
    pub(crate) surf_win_sc_tsol_dif_dif: Vec<f64>,     // SurfWinScTsolDifDif
    pub(crate) surf_win_sc_gl_sys_tsol_bm_bm: Vec<f64>, // SurfWinScGlSysTsolBmBm
    pub(crate) surf_win_sc_gl_sys_tsol_dif_dif: Vec<f64>, // SurfWinScGlSysTsolDifDif
    pub(crate) surf_win_gl_tsol_bm_bm: Vec<f64>,       // SurfWinGlTsolBmBm
    pub(crate) surf_win_gl_tsol_bm_dif: Vec<f64>,      // SurfWinGlTsolBmDif
    pub(crate) surf_win_gl_tsol_dif_dif: Vec<f64>,     // SurfWinGlTsolDifDif
    pub(crate) surf_win_bm_sol_trans_thru_int_win_rep: Vec<f64>, // SurfWinBmSolTransThruIntWinRep
    pub(crate) surf_win_bm_sol_absd_outs_reveal: Vec<f64>, // SurfWinBmSolAbsdOutsReveal
    pub(crate) surf_win_bm_sol_refld_outs_reveal_report: Vec<f64>, // SurfWinBmSolRefldOutsRevealReport
    pub(crate) surf_win_bm_sol_absd_ins_reveal: Vec<f64>,          // SurfWinBmSolAbsdInsReveal
    pub(crate) surf_win_bm_sol_refld_ins_reveal: Vec<f64>,         // SurfWinBmSolRefldInsReveal
    pub(crate) surf_win_bm_sol_refld_ins_reveal_report: Vec<f64>, // SurfWinBmSolRefldInsRevealReport
    pub(crate) surf_win_outs_reveal_diff_onto_glazing: Vec<f64>, // SurfWinOutsRevealDiffOntoGlazing
    pub(crate) surf_win_ins_reveal_diff_onto_glazing: Vec<f64>,  // SurfWinInsRevealDiffOntoGlazing
    pub(crate) surf_win_ins_reveal_diff_into_zone: Vec<f64>,     // SurfWinInsRevealDiffIntoZone
    pub(crate) surf_win_outs_reveal_diff_onto_frame: Vec<f64>,   // SurfWinOutsRevealDiffOntoFrame
    pub(crate) surf_win_ins_reveal_diff_onto_frame: Vec<f64>,    // SurfWinInsRevealDiffOntoFrame
    pub(crate) surf_win_ins_reveal_diff_onto_glazing_report: Vec<f64>, // SurfWinInsRevealDiffOntoGlazingReport
    pub(crate) surf_win_ins_reveal_diff_into_zone_report: Vec<f64>, // SurfWinInsRevealDiffIntoZoneReport
    pub(crate) surf_win_ins_reveal_diff_onto_frame_report: Vec<f64>, // SurfWinInsRevealDiffOntoFrameReport
    pub(crate) surf_win_bm_sol_absd_ins_reveal_report: Vec<f64>, // SurfWinBmSolAbsdInsRevealReport
    pub(crate) surf_win_bm_sol_trans_thru_int_win_rep_energy: Vec<f64>, // SurfWinBmSolTransThruIntWinRepEnergy
    pub(crate) surf_win_bm_sol_refld_outs_reveal_rep_energy: Vec<f64>, // SurfWinBmSolRefldOutsRevealRepEnergy
    pub(crate) surf_win_bm_sol_refld_ins_reveal_rep_energy: Vec<f64>, // SurfWinBmSolRefldInsRevealRepEnergy
    pub(crate) surf_win_profile_ang_hor: Vec<f64>,                    // SurfWinProfileAngHor
    pub(crate) surf_win_profile_ang_vert: Vec<f64>,                   // SurfWinProfileAngVert
    pub(crate) surf_win_shading_flag: Vec<i32>,                       // SurfWinShadingFlag
    pub(crate) surf_win_shading_flag_ems_on: Vec<bool>,               // SurfWinShadingFlagEMSOn
    pub(crate) surf_win_shading_flag_ems_value: Vec<i32>,             // SurfWinShadingFlagEMSValue
    pub(crate) surf_win_storm_win_flag: Vec<i32>,                     // SurfWinStormWinFlag
    pub(crate) surf_win_storm_win_flag_prev_day: Vec<i32>,            // SurfWinStormWinFlagPrevDay
    pub(crate) surf_win_frac_time_shading_device_on: Vec<f64>, // SurfWinFracTimeShadingDeviceOn
    pub(crate) surf_win_ext_int_shade_prev_ts: Vec<i32>,       // SurfWinExtIntShadePrevTS
    pub(crate) surf_win_has_shade_or_blind_layer: Vec<bool>,   // SurfWinHasShadeOrBlindLayer
    pub(crate) surf_win_surf_day_light_init: Vec<bool>,        // SurfWinSurfDayLightInit
    pub(crate) surf_win_dayl_fac_point: Vec<i32>,              // SurfWinDaylFacPoint
    pub(crate) surf_win_vis_trans_selected: Vec<f64>,          // SurfWinVisTransSelected
    pub(crate) surf_win_switching_factor: Vec<f64>,            // SurfWinSwitchingFactor
    pub(crate) surf_win_vis_trans_ratio: Vec<f64>,             // SurfWinVisTransRatio
    pub(crate) surf_win_i_rfrom_parent_zone: Vec<f64>,         // SurfWinIRfromParentZone
    pub(crate) surf_win_frame_area: Vec<f64>,                  // SurfWinFrameArea
    pub(crate) surf_win_frame_conductance: Vec<f64>,           // SurfWinFrameConductance
    pub(crate) surf_win_frame_sol_absorp: Vec<f64>,            // SurfWinFrameSolAbsorp
    pub(crate) surf_win_frame_vis_absorp: Vec<f64>,            // SurfWinFrameVisAbsorp
    pub(crate) surf_win_frame_emis: Vec<f64>,                  // SurfWinFrameEmis
    pub(crate) surf_win_fr_edge_to_center_gl_cond_ratio: Vec<f64>, // SurfWinFrEdgeToCenterGlCondRatio
    pub(crate) surf_win_frame_edge_area: Vec<f64>,                 // SurfWinFrameEdgeArea
    pub(crate) surf_win_frame_temp_in: Vec<f64>,                   // SurfWinFrameTempIn
    pub(crate) surf_win_frame_temp_in_old: Vec<f64>,               // SurfWinFrameTempInOld
    pub(crate) surf_win_frame_temp_surf_out: Vec<f64>,             // SurfWinFrameTempSurfOut
    pub(crate) surf_win_proj_corr_fr_out: Vec<f64>,                // SurfWinProjCorrFrOut
    pub(crate) surf_win_proj_corr_fr_in: Vec<f64>,                 // SurfWinProjCorrFrIn
    pub(crate) surf_win_divider_type: Vec<i32>,                    // SurfWinDividerType
    pub(crate) surf_win_divider_area: Vec<f64>,                    // SurfWinDividerArea
    pub(crate) surf_win_divider_conductance: Vec<f64>,             // SurfWinDividerConductance
    pub(crate) surf_win_divider_sol_absorp: Vec<f64>,              // SurfWinDividerSolAbsorp
    pub(crate) surf_win_divider_vis_absorp: Vec<f64>,              // SurfWinDividerVisAbsorp
    pub(crate) surf_win_divider_emis: Vec<f64>,                    // SurfWinDividerEmis
    pub(crate) surf_win_div_edge_to_center_gl_cond_ratio: Vec<f64>, // SurfWinDivEdgeToCenterGlCondRatio
    pub(crate) surf_win_divider_edge_area: Vec<f64>,                // SurfWinDividerEdgeArea
    pub(crate) surf_win_divider_temp_in: Vec<f64>,                  // SurfWinDividerTempIn
    pub(crate) surf_win_divider_temp_in_old: Vec<f64>,              // SurfWinDividerTempInOld
    pub(crate) surf_win_divider_temp_surf_out: Vec<f64>,            // SurfWinDividerTempSurfOut
    pub(crate) surf_win_proj_corr_div_out: Vec<f64>,                // SurfWinProjCorrDivOut
    pub(crate) surf_win_proj_corr_div_in: Vec<f64>,                 // SurfWinProjCorrDivIn
    pub(crate) surf_win_shade_abs_fac_face1: Vec<f64>,              // SurfWinShadeAbsFacFace1
    pub(crate) surf_win_shade_abs_fac_face2: Vec<f64>,              // SurfWinShadeAbsFacFace2
    pub(crate) surf_win_conv_coeff_with_shade: Vec<f64>,            // SurfWinConvCoeffWithShade
    pub(crate) surf_win_other_conv_heat_gain: Vec<f64>,             // SurfWinOtherConvHeatGain
    pub(crate) surf_win_eff_ins_surf_temp: Vec<f64>,                // SurfWinEffInsSurfTemp
    pub(crate) surf_win_tot_glazing_thickness: Vec<f64>,            // SurfWinTotGlazingThickness
    pub(crate) surf_win_tan_profile_ang_hor: Vec<f64>,              // SurfWinTanProfileAngHor
    pub(crate) surf_win_tan_profile_ang_vert: Vec<f64>,             // SurfWinTanProfileAngVert
    pub(crate) surf_win_inside_sill_depth: Vec<f64>,                // SurfWinInsideSillDepth
    pub(crate) surf_win_inside_reveal: Vec<f64>,                    // SurfWinInsideReveal
    pub(crate) surf_win_inside_sill_sol_abs: Vec<f64>,              // SurfWinInsideSillSolAbs
    pub(crate) surf_win_inside_reveal_sol_abs: Vec<f64>,            // SurfWinInsideRevealSolAbs
    pub(crate) surf_win_outside_reveal_sol_abs: Vec<f64>,           // SurfWinOutsideRevealSolAbs
    pub(crate) surf_win_airflow_source: Vec<i32>,                   // SurfWinAirflowSource
    pub(crate) surf_win_airflow_destination: Vec<i32>,              // SurfWinAirflowDestination
    pub(crate) surf_win_airflow_return_node_ptr: Vec<i32>,          // SurfWinAirflowReturnNodePtr
    pub(crate) surf_win_max_airflow: Vec<f64>,                      // SurfWinMaxAirflow
    pub(crate) surf_win_airflow_control_type: Vec<i32>,             // SurfWinAirflowControlType
    pub(crate) surf_win_airflow_has_schedule: Vec<bool>,            // SurfWinAirflowHasSchedule
    pub(crate) surf_win_airflow_scheds: Vec<Option<std::convert::Infallible>>, // SurfWinAirflowScheds
    pub(crate) surf_win_airflow_this_ts: Vec<f64>, // SurfWinAirflowThisTS
    pub(crate) surf_win_t_airflow_gap_outlet: Vec<f64>, // SurfWinTAirflowGapOutlet
    pub(crate) surf_win_window_calc_iterations_rep: Vec<i32>, // SurfWinWindowCalcIterationsRep
    pub(crate) surf_win_venting_open_factor_mult_rep: Vec<f64>, // SurfWinVentingOpenFactorMultRep
    pub(crate) surf_win_inside_temp_for_venting_rep: Vec<f64>, // SurfWinInsideTempForVentingRep
    pub(crate) surf_win_venting_availability_rep: Vec<f64>, // SurfWinVentingAvailabilityRep
    pub(crate) surf_win_sky_gnd_solar_inc: Vec<f64>, // SurfWinSkyGndSolarInc
    pub(crate) surf_win_bm_gnd_solar_inc: Vec<f64>, // SurfWinBmGndSolarInc
    pub(crate) surf_win_solar_diffusing: Vec<bool>, // SurfWinSolarDiffusing
    pub(crate) surf_win_frame_heat_gain: Vec<f64>, // SurfWinFrameHeatGain
    pub(crate) surf_win_frame_heat_loss: Vec<f64>, // SurfWinFrameHeatLoss
    pub(crate) surf_win_divider_heat_loss: Vec<f64>, // SurfWinDividerHeatLoss
    pub(crate) surf_win_tc_layer_temp: Vec<f64>,   // SurfWinTCLayerTemp
    pub(crate) surf_win_spec_temp: Vec<f64>,       // SurfWinSpecTemp
    pub(crate) surf_win_window_model_type: Vec<i32>, // SurfWinWindowModelType
    pub(crate) surf_win_tdd_pipe_num: Vec<f64>,    // SurfWinTDDPipeNum
    pub(crate) surf_win_storm_win_constr: Vec<i32>, // SurfWinStormWinConstr
    pub(crate) surf_active_construction: Vec<i32>, // SurfActiveConstruction
    pub(crate) surf_win_active_shaded_construction: Vec<i32>, // SurfWinActiveShadedConstruction
}
impl SurfaceWindowArrays {
    pub(crate) fn allocate(rows: usize, max_solid_window_layers: i32) -> Result<Self, Error> {
        // Borrow actual SetPreConstructionInputParameters, never assume seven.
        let solid_window_columns = usize::try_from(max_solid_window_layers)
            .map_err(|_| Error::Unavailable("actual MaxSolidWinLayers allocation bound"))?;
        let equivalent_layer_columns = 6_usize + 1; // CFSMAXNL=6, actual header67.
        let eql_slots = rows
            .checked_mul(equivalent_layer_columns)
            .ok_or(Error::Unavailable("Source window matrix storage size"))?;
        let solid_slots = rows
            .checked_mul(solid_window_columns)
            .ok_or(Error::Unavailable(
                "Source solid-window matrix storage size",
            ))?;
        Ok(Self {
            surf_sun_cos_hourly: crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray::default(),
            surf_sunlit_area: crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray::default(),
            surf_sunlit_frac: crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray::default(),
            surf_bm_to_bm_refl_fac_obs: crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray::default(),
            surf_bm_to_diff_refl_fac_obs: crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray::default(),
            surf_bm_to_diff_refl_fac_gnd: crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray::default(),
            surf_sky_diff_refl_fac_gnd: crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray::default(),
            surf_opaq_ai: crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray::default(),
            surf_opaq_ao: crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray::default(),
            surf_win_trans_solar: crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray::default(),
            surf_win_bm_solar: crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray::default(),
            surf_win_bm_bm_solar: crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray::default(),
            surf_win_bm_dif_solar: crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray::default(),
            surf_win_dif_solar: crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray::default(),
            surf_win_heat_gain: crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray::default(),
            surf_win_heat_gain_rep: crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray::default(),
            surf_win_heat_loss_rep: crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray::default(),
            surf_win_gain_conv_glaz_to_zone_rep: crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray::default(),
            surf_win_gain_ir_glaz_to_zone_rep: crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray::default(),
            surf_win_loss_sw_zone_to_out_win_rep: crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray::default(),
            surf_win_gain_frame_divider_to_zone_rep: crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray::default(),
            surf_win_gain_conv_shade_to_zone_rep: crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray::default(),
            surf_win_gain_ir_shade_to_zone_rep: crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray::default(),
            surf_win_gap_conv_ht_flow_rep: crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray::default(),
            surf_win_shading_absorbed_solar: crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray::default(),
            surf_win_sys_sol_transmittance: crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray::default(),
            surf_win_sys_sol_reflectance: crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray::default(),
            surf_win_sys_sol_absorptance: crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray::default(),
            surf_win_inside_glass_condensation_flag: crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray::default(),
            surf_win_inside_frame_condensation_flag: crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray::default(),
            surf_win_inside_divider_condensation_flag: crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray::default(),
            surf_win_trans_solar_energy: crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray::default(),
            surf_win_bm_solar_energy: crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray::default(),
            surf_win_bm_bm_solar_energy: crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray::default(),
            surf_win_bm_dif_solar_energy: crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray::default(),
            surf_win_dif_solar_energy: crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray::default(),
            surf_win_heat_gain_rep_energy: crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray::default(),
            surf_win_heat_loss_rep_energy: crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray::default(),
            surf_win_gap_conv_ht_flow_rep_energy: crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray::default(),
            surf_win_heat_transfer_rep_energy: crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray::default(),
            surf_win_shading_absorbed_solar_energy: crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray::default(),
            rows,
            equivalent_layer_columns,
            solid_window_columns,
            surf_win_a: vec![0.0; eql_slots],
            surf_win_a_diff_front: vec![0.0; eql_slots],
            surf_win_acf_overlap: vec![0.0; solid_slots],
            surf_win_frame_q_rad_out_abs: vec![0.0; rows], // SurfWinFrameQRadOutAbs: 0
            surf_win_frame_q_rad_in_abs: vec![0.0; rows],  // SurfWinFrameQRadInAbs: 0
            surf_win_divider_q_rad_out_abs: vec![0.0; rows], // SurfWinDividerQRadOutAbs: 0
            surf_win_divider_q_rad_in_abs: vec![0.0; rows], // SurfWinDividerQRadInAbs: 0
            surf_win_ext_beam_abs_by_shade: vec![0.0; rows], // SurfWinExtBeamAbsByShade: 0
            surf_win_ext_diff_abs_by_shade: vec![0.0; rows], // SurfWinExtDiffAbsByShade: 0
            surf_win_int_beam_abs_by_shade: vec![0.0; rows], // SurfWinIntBeamAbsByShade: 0
            surf_win_int_sw_abs_by_shade: vec![0.0; rows], // SurfWinIntSWAbsByShade: 0
            surf_win_initial_dif_sol_abs_by_shade: vec![0.0; rows], // SurfWinInitialDifSolAbsByShade: 0
            surf_win_int_lw_abs_by_shade: vec![0.0; rows],          // SurfWinIntLWAbsByShade: 0
            surf_win_conv_heat_flow_natural: vec![0.0; rows],       // SurfWinConvHeatFlowNatural: 0
            surf_win_conv_heat_gain_to_zone_air: vec![0.0; rows], // SurfWinConvHeatGainToZoneAir: 0
            surf_win_ret_heat_gain_to_zone_air: vec![0.0; rows],  // SurfWinRetHeatGainToZoneAir: 0
            surf_win_divider_heat_gain: vec![0.0; rows],          // SurfWinDividerHeatGain: 0
            surf_win_bl_tsol_bm_bm: vec![0.0; rows],              // SurfWinBlTsolBmBm: 0
            surf_win_bl_tsol_bm_dif: vec![0.0; rows],             // SurfWinBlTsolBmDif: 0
            surf_win_bl_tsol_dif_dif: vec![0.0; rows],            // SurfWinBlTsolDifDif: 0
            surf_win_bl_gl_sys_tsol_bm_bm: vec![0.0; rows],       // SurfWinBlGlSysTsolBmBm: 0
            surf_win_bl_gl_sys_tsol_dif_dif: vec![0.0; rows],     // SurfWinBlGlSysTsolDifDif: 0
            surf_win_sc_tsol_bm_bm: vec![0.0; rows],              // SurfWinScTsolBmBm: 0
            surf_win_sc_tsol_bm_dif: vec![0.0; rows],             // SurfWinScTsolBmDif: 0
            surf_win_sc_tsol_dif_dif: vec![0.0; rows],            // SurfWinScTsolDifDif: 0
            surf_win_sc_gl_sys_tsol_bm_bm: vec![0.0; rows],       // SurfWinScGlSysTsolBmBm: 0
            surf_win_sc_gl_sys_tsol_dif_dif: vec![0.0; rows],     // SurfWinScGlSysTsolDifDif: 0
            surf_win_gl_tsol_bm_bm: vec![0.0; rows],              // SurfWinGlTsolBmBm: 0
            surf_win_gl_tsol_bm_dif: vec![0.0; rows],             // SurfWinGlTsolBmDif: 0
            surf_win_gl_tsol_dif_dif: vec![0.0; rows],            // SurfWinGlTsolDifDif: 0
            surf_win_bm_sol_trans_thru_int_win_rep: vec![0.0; rows], // SurfWinBmSolTransThruIntWinRep: 0
            surf_win_bm_sol_absd_outs_reveal: vec![0.0; rows], // SurfWinBmSolAbsdOutsReveal: 0
            surf_win_bm_sol_refld_outs_reveal_report: vec![0.0; rows], // SurfWinBmSolRefldOutsRevealReport: 0
            surf_win_bm_sol_absd_ins_reveal: vec![0.0; rows], // SurfWinBmSolAbsdInsReveal: 0
            surf_win_bm_sol_refld_ins_reveal: vec![0.0; rows], // SurfWinBmSolRefldInsReveal: 0
            surf_win_bm_sol_refld_ins_reveal_report: vec![0.0; rows], // SurfWinBmSolRefldInsRevealReport: 0
            surf_win_outs_reveal_diff_onto_glazing: vec![0.0; rows], // SurfWinOutsRevealDiffOntoGlazing: 0
            surf_win_ins_reveal_diff_onto_glazing: vec![0.0; rows], // SurfWinInsRevealDiffOntoGlazing: 0
            surf_win_ins_reveal_diff_into_zone: vec![0.0; rows], // SurfWinInsRevealDiffIntoZone: 0
            surf_win_outs_reveal_diff_onto_frame: vec![0.0; rows], // SurfWinOutsRevealDiffOntoFrame: 0
            surf_win_ins_reveal_diff_onto_frame: vec![0.0; rows], // SurfWinInsRevealDiffOntoFrame: 0
            surf_win_ins_reveal_diff_onto_glazing_report: vec![0.0; rows], // SurfWinInsRevealDiffOntoGlazingReport: 0
            surf_win_ins_reveal_diff_into_zone_report: vec![0.0; rows], // SurfWinInsRevealDiffIntoZoneReport: 0
            surf_win_ins_reveal_diff_onto_frame_report: vec![0.0; rows], // SurfWinInsRevealDiffOntoFrameReport: 0
            surf_win_bm_sol_absd_ins_reveal_report: vec![0.0; rows], // SurfWinBmSolAbsdInsRevealReport: 0
            surf_win_bm_sol_trans_thru_int_win_rep_energy: vec![0.0; rows], // SurfWinBmSolTransThruIntWinRepEnergy: 0
            surf_win_bm_sol_refld_outs_reveal_rep_energy: vec![0.0; rows], // SurfWinBmSolRefldOutsRevealRepEnergy: 0
            surf_win_bm_sol_refld_ins_reveal_rep_energy: vec![0.0; rows], // SurfWinBmSolRefldInsRevealRepEnergy: 0
            surf_win_profile_ang_hor: vec![0.0; rows],                    // SurfWinProfileAngHor: 0
            surf_win_profile_ang_vert: vec![0.0; rows], // SurfWinProfileAngVert: 0
            surf_win_shading_flag: vec![1; rows], // SurfWinShadingFlag: DataSurfaces::WinShadingType::ShadeOff
            surf_win_shading_flag_ems_on: vec![false; rows], // SurfWinShadingFlagEMSOn: false
            surf_win_shading_flag_ems_value: vec![0; rows], // SurfWinShadingFlagEMSValue: 0.0
            surf_win_storm_win_flag: vec![0; rows], // SurfWinStormWinFlag: 0
            surf_win_storm_win_flag_prev_day: vec![0; rows], // SurfWinStormWinFlagPrevDay: 0
            surf_win_frac_time_shading_device_on: vec![0.0; rows], // SurfWinFracTimeShadingDeviceOn: 0
            surf_win_ext_int_shade_prev_ts: vec![1; rows], // SurfWinExtIntShadePrevTS: DataSurfaces::WinShadingType::ShadeOff
            surf_win_has_shade_or_blind_layer: vec![false; rows], // SurfWinHasShadeOrBlindLayer: false
            surf_win_surf_day_light_init: vec![false; rows],      // SurfWinSurfDayLightInit: false
            surf_win_dayl_fac_point: vec![0; rows],               // SurfWinDaylFacPoint: 0
            surf_win_vis_trans_selected: vec![0.0; rows],         // SurfWinVisTransSelected: 0
            surf_win_switching_factor: vec![0.0; rows],           // SurfWinSwitchingFactor: 0
            surf_win_vis_trans_ratio: vec![0.0; rows],            // SurfWinVisTransRatio: 0
            surf_win_i_rfrom_parent_zone: vec![0.0; rows],        // SurfWinIRfromParentZone: 0
            surf_win_frame_area: vec![0.0; rows],                 // SurfWinFrameArea: 0
            surf_win_frame_conductance: vec![0.0; rows],          // SurfWinFrameConductance: 0
            surf_win_frame_sol_absorp: vec![0.0; rows],           // SurfWinFrameSolAbsorp: 0
            surf_win_frame_vis_absorp: vec![0.0; rows],           // SurfWinFrameVisAbsorp: 0
            surf_win_frame_emis: vec![0.0; rows],                 // SurfWinFrameEmis: 0
            surf_win_fr_edge_to_center_gl_cond_ratio: vec![1.0; rows], // SurfWinFrEdgeToCenterGlCondRatio: 1.0
            surf_win_frame_edge_area: vec![0.0; rows],                 // SurfWinFrameEdgeArea: 0
            surf_win_frame_temp_in: vec![23.0; rows],                  // SurfWinFrameTempIn: 23.0
            surf_win_frame_temp_in_old: vec![23.0; rows], // SurfWinFrameTempInOld: 23.0
            surf_win_frame_temp_surf_out: vec![23.0; rows], // SurfWinFrameTempSurfOut: 23.0
            surf_win_proj_corr_fr_out: vec![0.0; rows],   // SurfWinProjCorrFrOut: 0
            surf_win_proj_corr_fr_in: vec![0.0; rows],    // SurfWinProjCorrFrIn: 0
            surf_win_divider_type: vec![0; rows], // SurfWinDividerType: DataSurfaces::FrameDividerType::DividedLite
            surf_win_divider_area: vec![0.0; rows], // SurfWinDividerArea: 0
            surf_win_divider_conductance: vec![0.0; rows], // SurfWinDividerConductance: 0
            surf_win_divider_sol_absorp: vec![0.0; rows], // SurfWinDividerSolAbsorp: 0
            surf_win_divider_vis_absorp: vec![0.0; rows], // SurfWinDividerVisAbsorp: 0
            surf_win_divider_emis: vec![0.0; rows], // SurfWinDividerEmis: 0
            surf_win_div_edge_to_center_gl_cond_ratio: vec![1.0; rows], // SurfWinDivEdgeToCenterGlCondRatio: 1
            surf_win_divider_edge_area: vec![0.0; rows],                // SurfWinDividerEdgeArea: 0
            surf_win_divider_temp_in: vec![23.0; rows], // SurfWinDividerTempIn: 23.0
            surf_win_divider_temp_in_old: vec![23.0; rows], // SurfWinDividerTempInOld: 23.0
            surf_win_divider_temp_surf_out: vec![23.0; rows], // SurfWinDividerTempSurfOut: 23.0
            surf_win_proj_corr_div_out: vec![0.0; rows], // SurfWinProjCorrDivOut: 0
            surf_win_proj_corr_div_in: vec![0.0; rows], // SurfWinProjCorrDivIn: 0
            surf_win_shade_abs_fac_face1: vec![0.5; rows], // SurfWinShadeAbsFacFace1: 0.5
            surf_win_shade_abs_fac_face2: vec![0.5; rows], // SurfWinShadeAbsFacFace2: 0.5
            surf_win_conv_coeff_with_shade: vec![0.0; rows], // SurfWinConvCoeffWithShade: 0
            surf_win_other_conv_heat_gain: vec![0.0; rows], // SurfWinOtherConvHeatGain: 0
            surf_win_eff_ins_surf_temp: vec![23.0; rows], // SurfWinEffInsSurfTemp: 23.0
            surf_win_tot_glazing_thickness: vec![0.0; rows], // SurfWinTotGlazingThickness: 0
            surf_win_tan_profile_ang_hor: vec![0.0; rows], // SurfWinTanProfileAngHor: 0
            surf_win_tan_profile_ang_vert: vec![0.0; rows], // SurfWinTanProfileAngVert: 0
            surf_win_inside_sill_depth: vec![0.0; rows], // SurfWinInsideSillDepth: 0
            surf_win_inside_reveal: vec![0.0; rows],    // SurfWinInsideReveal: 0
            surf_win_inside_sill_sol_abs: vec![0.0; rows], // SurfWinInsideSillSolAbs: 0
            surf_win_inside_reveal_sol_abs: vec![0.0; rows], // SurfWinInsideRevealSolAbs: 0
            surf_win_outside_reveal_sol_abs: vec![0.0; rows], // SurfWinOutsideRevealSolAbs: 0
            surf_win_airflow_source: vec![-1; rows], // SurfWinAirflowSource: DataSurfaces::WindowAirFlowSource::Invalid
            surf_win_airflow_destination: vec![-1; rows], // SurfWinAirflowDestination: DataSurfaces::WindowAirFlowDestination::Invalid
            surf_win_airflow_return_node_ptr: vec![0; rows], // SurfWinAirflowReturnNodePtr: 0
            surf_win_max_airflow: vec![0.0; rows],        // SurfWinMaxAirflow: 0
            surf_win_airflow_control_type: vec![-1; rows], // SurfWinAirflowControlType: DataSurfaces::WindowAirFlowControlType::Invalid
            surf_win_airflow_has_schedule: vec![false; rows], // SurfWinAirflowHasSchedule: false
            surf_win_airflow_scheds: vec![None; rows],     // SurfWinAirflowScheds: nullptr
            surf_win_airflow_this_ts: vec![0.0; rows],     // SurfWinAirflowThisTS: 0
            surf_win_t_airflow_gap_outlet: vec![0.0; rows], // SurfWinTAirflowGapOutlet: 0
            surf_win_window_calc_iterations_rep: vec![0; rows], // SurfWinWindowCalcIterationsRep: 0
            surf_win_venting_open_factor_mult_rep: vec![0.0; rows], // SurfWinVentingOpenFactorMultRep: 0
            surf_win_inside_temp_for_venting_rep: vec![0.0; rows], // SurfWinInsideTempForVentingRep: 0
            surf_win_venting_availability_rep: vec![0.0; rows], // SurfWinVentingAvailabilityRep: 0
            surf_win_sky_gnd_solar_inc: vec![0.0; rows],        // SurfWinSkyGndSolarInc: 0
            surf_win_bm_gnd_solar_inc: vec![0.0; rows],         // SurfWinBmGndSolarInc: 0
            surf_win_solar_diffusing: vec![false; rows],        // SurfWinSolarDiffusing: false
            surf_win_frame_heat_gain: vec![0.0; rows],          // SurfWinFrameHeatGain: 0
            surf_win_frame_heat_loss: vec![0.0; rows],          // SurfWinFrameHeatLoss: 0
            surf_win_divider_heat_loss: vec![0.0; rows],        // SurfWinDividerHeatLoss: 0
            surf_win_tc_layer_temp: vec![0.0; rows],            // SurfWinTCLayerTemp: 0
            surf_win_spec_temp: vec![0.0; rows],                // SurfWinSpecTemp: 0
            surf_win_window_model_type: vec![0; rows], // SurfWinWindowModelType: DataSurfaces::WindowModel::Detailed
            surf_win_tdd_pipe_num: vec![0.0; rows],    // SurfWinTDDPipeNum: 0
            surf_win_storm_win_constr: vec![0; rows],  // SurfWinStormWinConstr: 0
            surf_active_construction: vec![0; rows],   // SurfActiveConstruction: 0
            surf_win_active_shaded_construction: vec![0; rows], // SurfWinActiveShadedConstruction: 0
        })
    }
}
