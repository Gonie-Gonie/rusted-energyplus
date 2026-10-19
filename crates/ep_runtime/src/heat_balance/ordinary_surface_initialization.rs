//! Actual InitSurfaceHeatBalance manager constructor and allocated scalar arrays.
//! Normal/master CTF histories and four conduction reports remain in their one
//! existing CtfThermalHistoryState. All new arrays use final Source ordinal order.
//! This HELD owner does not activate the compatibility timestep or claim its callers.

use super::ctf_thermal_history::{
    CtfThermalHistoryArrays, CtfThermalHistoryMasters, CtfThermalHistoryState,
};
use super::selected_heat_balance_input::HeatBalanceInputError as Error;

mod environment;
mod inside_ctf;
mod inside_driver;
mod inside_entry;
pub(crate) mod inside_providers;
mod manager;
pub(crate) mod provider_host;
pub(crate) use inside_driver::{SourceInsideCtfOutcome, SourceInsideCtfProviders};
pub use manager::SourceInitSurfaceError;
pub(crate) use manager::{SourceInitSurfaceOwners, SourceInitSurfaceProviders};
pub(crate) mod output;
pub(crate) mod outside_driver;
mod prefix;
#[cfg(all(test, feature = "native-controlled-air-cohort"))]
pub(crate) use prefix::check_glazing_shading_status_change as native_gap_check_glazing_shading_status_change;
mod radiant_handoff;
mod registration;
pub(crate) mod report;
mod suffix;
mod thermal_history;
pub(crate) use thermal_history::SourceThermalHistoryOwners;

/// Actual SurfQRadFromHVACData has no member initializers. Allocation therefore
/// retains unwritten cells; the genuine InitSurface suffix writes all five.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceHvacRadiantFlux {
    pub(crate) ht_rad_sys: Option<f64>,
    pub(crate) hw_baseboard: Option<f64>,
    pub(crate) steam_baseboard: Option<f64>,
    pub(crate) elec_baseboard: Option<f64>,
    pub(crate) cooling_panel: Option<f64>,
}

/// Same HeatBalanceSurfaceManagerData owner, not a radiation readiness wrapper.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct OrdinarySurfaceInitialization {
    pub(crate) first_time: bool,
    pub(crate) inside_entry_first_time: bool,
    pub(crate) inside_entry_environment_flag: bool,
    pub(crate) ctf_only_first_time: bool,
    pub(crate) ctf_inside_error_count: i32,
    pub(crate) ctf_inside_error_pointer: i32,
    pub(crate) ctf_inside_warmup_error_count: i32,
    pub(crate) interzone_window: bool,
    pub(crate) inside_surface_iterations: i32,
    pub(crate) minimum_inside_surface_iterations: i32,
    pub(crate) maximum_allowed_temperature_difference: f64,
    pub(crate) maximum_allowed_temperature_difference_cond_fd: f64,
    pub(crate) cond_fd_relaxation_factor: f64,
    pub(crate) cond_fd_relaxation_factor_input: f64,
    pub(crate) sum_surface_heat_emission: f64,
    /// Actual HeatBalSurfMgr.hh242 constructortrue, separate from inside/Init first flags.
    pub(crate) calculate_zone_mrt_first_time: bool,
    pub(crate) arrays: SourceSurfaceHeatBalanceArrays,
}
impl Default for OrdinarySurfaceInitialization {
    fn default() -> Self {
        Self {
            first_time: true,
            inside_entry_first_time: true,
            inside_entry_environment_flag: true,
            ctf_only_first_time: true,
            ctf_inside_error_count: 0,
            ctf_inside_error_pointer: 0,
            ctf_inside_warmup_error_count: 0,
            interzone_window: false,
            inside_surface_iterations: 0,
            minimum_inside_surface_iterations: 1,
            maximum_allowed_temperature_difference: 0.002,
            maximum_allowed_temperature_difference_cond_fd: 0.002,
            cond_fd_relaxation_factor: 1.0,
            cond_fd_relaxation_factor_input: 1.0,
            sum_surface_heat_emission: 0.0,
            calculate_zone_mrt_first_time: true,
            arrays: SourceSurfaceHeatBalanceArrays::default(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceSurfaceHeatBalanceArrays {
    /// Actual manager SurfaceAE/ZoneAESum constructor-unallocated; MRT5596-5599
    /// allocates both unwritten before the two literal whole-array zero writes.
    pub(crate) surface_ae:
        crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray<f64, 1>,
    pub(crate) zone_ae_sum:
        crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray<f64, 1>,
    /// DataHeatBalSurf constructor-unallocated; genuine WindowManager144 owns dimension1.
    /// The later AllocateSurfaceHeatBalArrays leaves this same early array intact.
    pub(crate) surf_win_coeff_adj_ratio: Option<Vec<f64>>,
    /// HBSM1417: dataHeatBalSurf::SurfCTFConstInPart.
    pub(crate) surf_ctf_const_in_part: Option<Vec<f64>>,
    /// HBSM1418: dataHeatBalSurf::SurfCTFConstOutPart.
    pub(crate) surf_ctf_const_out_part: Option<Vec<f64>>,
    /// HBSM1419: dataHeatBalSurf::SurfCTFCross0.
    pub(crate) surf_ctf_cross0: Option<Vec<f64>>,
    /// HBSM1420: dataHeatBalSurf::SurfCTFInside0.
    pub(crate) surf_ctf_inside0: Option<Vec<f64>>,
    /// HBSM1421: dataHeatBalSurf::SurfTempOutHist.
    pub(crate) surf_temp_out_hist: Option<Vec<f64>>,
    /// HBSM1422: dataHeatBalSurf::SurfCTFSourceIn0.
    pub(crate) surf_ctf_source_in0: Option<Vec<f64>>,
    /// HBSM1423: dataHeatBalSurf::SurfQSourceSinkHist.
    pub(crate) surf_q_source_sink_hist: Option<Vec<f64>>,
    /// HBSM1424: dataHeatBalSurf::SurfIsAdiabatic.
    pub(crate) surf_is_adiabatic: Option<Vec<i32>>,
    /// HBSM1425: dataHeatBalSurf::SurfIsSourceOrSink.
    pub(crate) surf_is_source_or_sink: Option<Vec<i32>>,
    /// HBSM1426: dataHeatBalSurf::SurfIsOperatingPool.
    pub(crate) surf_is_operating_pool: Option<Vec<i32>>,
    /// HBSM1427: dataHeatBalSurf::SurfTempTerm.
    pub(crate) surf_temp_term: Option<Vec<f64>>,
    /// HBSM1428: dataHeatBalSurf::SurfTempDiv.
    pub(crate) surf_temp_div: Option<Vec<f64>>,
    /// HBSM1430: dataHeatBalFanSys::CTFTsrcConstPart.
    pub(crate) ctf_tsrc_const_part: Option<Vec<f64>>,
    /// HBSM1431: dataHeatBalFanSys::CTFTuserConstPart.
    pub(crate) ctf_tuser_const_part: Option<Vec<f64>>,
    /// HBSM1434: dataHeatBal::SurfTempEffBulkAir.
    pub(crate) surf_temp_eff_bulk_air: Option<Vec<f64>>,
    /// HBSM1435: dataHeatBalSurf::SurfHConvInt.
    pub(crate) surf_h_conv_int: Option<Vec<f64>>,
    /// HBSM1436: dataHeatBalSurf::SurfHConvExt.
    pub(crate) surf_h_conv_ext: Option<Vec<f64>>,
    /// HBSM1437: dataHeatBalSurf::SurfHAirExt.
    pub(crate) surf_h_air_ext: Option<Vec<f64>>,
    /// HBSM1438: dataHeatBalSurf::SurfHSkyExt.
    pub(crate) surf_h_sky_ext: Option<Vec<f64>>,
    /// HBSM1439: dataHeatBalSurf::SurfHGrdExt.
    pub(crate) surf_h_grd_ext: Option<Vec<f64>>,
    /// HBSM1440: dataHeatBalSurf::SurfHSrdSurfExt.
    pub(crate) surf_h_srd_surf_ext: Option<Vec<f64>>,
    /// HBSM1442: dataHeatBalSurf::SurfTempIn.
    pub(crate) surf_temp_in: Option<Vec<f64>>,
    /// HBSM1443: dataHeatBalSurf::SurfTempInsOld.
    pub(crate) surf_temp_ins_old: Option<Vec<f64>>,
    /// HBSM1444: dataHeatBalSurf::SurfTempInTmp.
    pub(crate) surf_temp_in_tmp: Option<Vec<f64>>,
    /// HBSM1445: dataHeatBalSurfMgr::RefAirTemp.
    pub(crate) ref_air_temp: Option<Vec<f64>>,
    /// HBSM1446: dataHeatBalSurf::SurfQRadLWOutSrdSurfs.
    pub(crate) surf_q_rad_lw_out_srd_surfs: Option<Vec<f64>>,
    /// HBSM1448: dataHeatBal::SurfWinQRadSWwinAbs.
    pub(crate) surf_win_q_rad_s_wwin_abs: Option<Vec<Vec<f64>>>,
    /// HBSM1449: dataHeatBal::SurfWinInitialDifSolwinAbs.
    pub(crate) surf_win_initial_dif_solwin_abs: Option<Vec<Vec<f64>>>,
    /// HBSM1450: dataHeatBalSurf::SurfQRadSWOutMvIns.
    pub(crate) surf_q_rad_sw_out_mv_ins: Option<Vec<f64>>,
    /// HBSM1451: dataHeatBal::SurfQdotRadIntGainsInPerArea.
    pub(crate) surf_qdot_rad_int_gains_in_per_area: Option<Vec<f64>>,
    /// HBSM1452: dataHeatBalSurf::SurfQAdditionalHeatSourceOutside.
    pub(crate) surf_q_additional_heat_source_outside: Option<Vec<f64>>,
    /// HBSM1453: dataHeatBalSurf::SurfQAdditionalHeatSourceInside.
    pub(crate) surf_q_additional_heat_source_inside: Option<Vec<f64>>,
    /// HBSM1482: dataHeatBalSurf::SurfTempOut.
    pub(crate) surf_temp_out: Option<Vec<f64>>,
    /// HBSM1483: dataHeatBalSurf::SurfTempInMovInsRep.
    pub(crate) surf_temp_in_mov_ins_rep: Option<Vec<f64>>,
    /// HBSM1484: dataHeatBalSurf::SurfQConvInRep.
    pub(crate) surf_q_conv_in_rep: Option<Vec<f64>>,
    /// HBSM1485: dataHeatBalSurf::SurfQdotConvInPerArea.
    pub(crate) surf_qdot_conv_in_per_area: Option<Vec<f64>>,
    /// HBSM1486: dataHeatBalSurf::SurfQdotConvInRep.
    pub(crate) surf_qdot_conv_in_rep: Option<Vec<f64>>,
    /// HBSM1488: dataHeatBalSurf::SurfQRadNetSurfInRep.
    pub(crate) surf_q_rad_net_surf_in_rep: Option<Vec<f64>>,
    /// HBSM1489: dataHeatBalSurf::SurfQdotRadNetSurfInRep.
    pub(crate) surf_qdot_rad_net_surf_in_rep: Option<Vec<f64>>,
    /// HBSM1491: dataHeatBalSurf::SurfQRadSolarInRep.
    pub(crate) surf_q_rad_solar_in_rep: Option<Vec<f64>>,
    /// HBSM1492: dataHeatBalSurf::SurfQdotRadSolarInRep.
    pub(crate) surf_qdot_rad_solar_in_rep: Option<Vec<f64>>,
    /// HBSM1493: dataHeatBalSurf::SurfQdotRadSolarInRepPerArea.
    pub(crate) surf_qdot_rad_solar_in_rep_per_area: Option<Vec<f64>>,
    /// HBSM1495: dataHeatBalSurf::SurfQRadLightsInRep.
    pub(crate) surf_q_rad_lights_in_rep: Option<Vec<f64>>,
    /// HBSM1496: dataHeatBalSurf::SurfQdotRadLightsInRep.
    pub(crate) surf_qdot_rad_lights_in_rep: Option<Vec<f64>>,
    /// HBSM1498: dataHeatBalSurf::SurfQRadIntGainsInRep.
    pub(crate) surf_q_rad_int_gains_in_rep: Option<Vec<f64>>,
    /// HBSM1499: dataHeatBalSurf::SurfQdotRadIntGainsInRep.
    pub(crate) surf_qdot_rad_int_gains_in_rep: Option<Vec<f64>>,
    /// HBSM1501: dataHeatBalSurf::SurfQRadHVACInRep.
    pub(crate) surf_q_rad_hvac_in_rep: Option<Vec<f64>>,
    /// HBSM1502: dataHeatBalSurf::SurfQdotRadHVACInRep.
    pub(crate) surf_qdot_rad_hvac_in_rep: Option<Vec<f64>>,
    /// HBSM1503: dataHeatBalSurf::SurfQdotRadHVACInPerArea.
    pub(crate) surf_qdot_rad_hvac_in_per_area: Option<Vec<f64>>,
    /// HBSM1505: dataHeatBalSurf::SurfQConvOutReport.
    pub(crate) surf_q_conv_out_report: Option<Vec<f64>>,
    /// HBSM1506: dataHeatBalSurf::SurfQdotConvOutPerArea.
    pub(crate) surf_qdot_conv_out_per_area: Option<Vec<f64>>,
    /// HBSM1507: dataHeatBalSurf::SurfQdotConvOutRep.
    pub(crate) surf_qdot_conv_out_rep: Option<Vec<f64>>,
    /// HBSM1509: dataHeatBalSurf::SurfQdotRadOutRep.
    pub(crate) surf_qdot_rad_out_rep: Option<Vec<f64>>,
    /// HBSM1510: dataHeatBalSurf::SurfQdotRadOutRepPerArea.
    pub(crate) surf_qdot_rad_out_rep_per_area: Option<Vec<f64>>,
    /// HBSM1511: dataHeatBalSurf::SurfQRadOutReport.
    pub(crate) surf_q_rad_out_report: Option<Vec<f64>>,
    /// HBSM1513: dataHeatBalSurf::SurfQAirExtReport.
    pub(crate) surf_q_air_ext_report: Option<Vec<f64>>,
    /// HBSM1514: dataHeatBalSurf::SurfQHeatEmiReport.
    pub(crate) surf_q_heat_emi_report: Option<Vec<f64>>,
    /// HBSM1516: dataHeatBal::SurfOpaqSWOutAbsTotalReport.
    pub(crate) surf_opaq_sw_out_abs_total_report: Option<Vec<f64>>,
    /// HBSM1517: dataHeatBal::SurfOpaqSWOutAbsEnergyReport.
    pub(crate) surf_opaq_sw_out_abs_energy_report: Option<Vec<f64>>,
    /// HBSM1521: dataHeatBalSurf::SurfOpaqInsFaceCondGainRep.
    pub(crate) surf_opaq_ins_face_cond_gain_rep: Option<Vec<f64>>,
    /// HBSM1522: dataHeatBalSurf::SurfOpaqInsFaceCondLossRep.
    pub(crate) surf_opaq_ins_face_cond_loss_rep: Option<Vec<f64>>,
    /// HBSM1523: dataHeatBalSurf::SurfOpaqInsFaceCondEnergy.
    pub(crate) surf_opaq_ins_face_cond_energy: Option<Vec<f64>>,
    /// HBSM1527: dataHeatBalSurf::SurfOpaqExtFaceCondGainRep.
    pub(crate) surf_opaq_ext_face_cond_gain_rep: Option<Vec<f64>>,
    /// HBSM1528: dataHeatBalSurf::SurfOpaqExtFaceCondLossRep.
    pub(crate) surf_opaq_ext_face_cond_loss_rep: Option<Vec<f64>>,
    /// HBSM1529: dataHeatBalSurf::SurfOpaqOutFaceCondEnergy.
    pub(crate) surf_opaq_out_face_cond_energy: Option<Vec<f64>>,
    /// HBSM1531: dataHeatBalSurf::SurfOpaqAvgFaceCondGainRep.
    pub(crate) surf_opaq_avg_face_cond_gain_rep: Option<Vec<f64>>,
    /// HBSM1532: dataHeatBalSurf::SurfOpaqAvgFaceCondLossRep.
    pub(crate) surf_opaq_avg_face_cond_loss_rep: Option<Vec<f64>>,
    /// HBSM1533: dataHeatBalSurf::SurfOpaqAvgFaceCond.
    pub(crate) surf_opaq_avg_face_cond: Option<Vec<f64>>,
    /// HBSM1534: dataHeatBalSurf::SurfOpaqAvgFaceCondFlux.
    pub(crate) surf_opaq_avg_face_cond_flux: Option<Vec<f64>>,
    /// HBSM1535: dataHeatBalSurf::SurfOpaqAvgFaceCondEnergy.
    pub(crate) surf_opaq_avg_face_cond_energy: Option<Vec<f64>>,
    /// HBSM1537: dataHeatBalSurf::SurfOpaqStorageCondGainRep.
    pub(crate) surf_opaq_storage_cond_gain_rep: Option<Vec<f64>>,
    /// HBSM1538: dataHeatBalSurf::SurfOpaqStorageCondLossRep.
    pub(crate) surf_opaq_storage_cond_loss_rep: Option<Vec<f64>>,
    /// HBSM1539: dataHeatBalSurf::SurfOpaqStorageCond.
    pub(crate) surf_opaq_storage_cond: Option<Vec<f64>>,
    /// HBSM1540: dataHeatBalSurf::SurfOpaqStorageCondFlux.
    pub(crate) surf_opaq_storage_cond_flux: Option<Vec<f64>>,
    /// HBSM1541: dataHeatBalSurf::SurfOpaqStorageCondEnergy.
    pub(crate) surf_opaq_storage_cond_energy: Option<Vec<f64>>,
    /// HBSM1543: dataHeatBalSurf::SurfOpaqInsFaceBeamSolAbsorbed.
    pub(crate) surf_opaq_ins_face_beam_sol_absorbed: Option<Vec<f64>>,
    /// HBSM1545: dataHeatBalSurf::SurfOpaqQRadSWOutAbs.
    pub(crate) surf_opaq_q_rad_sw_out_abs: Option<Vec<f64>>,
    /// HBSM1546: dataHeatBalSurf::SurfOpaqQRadSWInAbs.
    pub(crate) surf_opaq_q_rad_sw_in_abs: Option<Vec<f64>>,
    /// HBSM1548: dataHeatBalSurf::SurfOpaqInitialDifSolInAbs.
    pub(crate) surf_opaq_initial_dif_sol_in_abs: Option<Vec<f64>>,
    /// HBSM1549: dataHeatBalSurf::SurfWinInitialDifSolInTrans.
    pub(crate) surf_win_initial_dif_sol_in_trans: Option<Vec<f64>>,
    /// HBSM1550: dataHeatBalSurf::SurfWinInitialBeamSolInTrans.
    pub(crate) surf_win_initial_beam_sol_in_trans: Option<Vec<f64>>,
    /// HBSM1552: dataHeatBalSurf::SurfQdotRadNetLWInPerArea.
    pub(crate) surf_qdot_rad_net_lw_in_per_area: Option<Vec<f64>>,
    /// HBSM1553: dataHeatBalSurf::SurfQdotRadLightsInPerArea.
    pub(crate) surf_qdot_rad_lights_in_per_area: Option<Vec<f64>>,
    /// HBSM1556: dataHeatBalSurf::SurfTempSource.
    pub(crate) surf_temp_source: Option<Vec<f64>>,
    /// HBSM1557: dataHeatBalSurf::SurfTempUserLoc.
    pub(crate) surf_temp_user_loc: Option<Vec<f64>>,
    /// HBSM1558: dataHeatBalSurf::SurfTsrcHist.
    pub(crate) surf_tsrc_hist: Option<Vec<Vec<f64>>>,
    /// HBSM1559: dataHeatBalSurf::SurfTuserHist.
    pub(crate) surf_tuser_hist: Option<Vec<Vec<f64>>>,
    /// HBSM1560: dataHeatBalSurf::SurfQsrcHist.
    pub(crate) surf_qsrc_hist: Option<Vec<Vec<f64>>>,
    /// HBSM1561: dataHeatBalSurf::SurfTsrcHistM.
    pub(crate) surf_tsrc_hist_m: Option<Vec<Vec<f64>>>,
    /// HBSM1562: dataHeatBalSurf::SurfTuserHistM.
    pub(crate) surf_tuser_hist_m: Option<Vec<Vec<f64>>>,
    /// HBSM1563: dataHeatBalSurf::SurfQsrcHistM.
    pub(crate) surf_qsrc_hist_m: Option<Vec<Vec<f64>>>,
    /// HBSM1566: dataHeatBalFanSys::RadSysTiHBConstCoef.
    pub(crate) rad_sys_ti_hb_const_coef: Option<Vec<f64>>,
    /// HBSM1567: dataHeatBalFanSys::RadSysTiHBToutCoef.
    pub(crate) rad_sys_ti_hb_tout_coef: Option<Vec<f64>>,
    /// HBSM1568: dataHeatBalFanSys::RadSysTiHBQsrcCoef.
    pub(crate) rad_sys_ti_hb_qsrc_coef: Option<Vec<f64>>,
    /// HBSM1569: dataHeatBalFanSys::RadSysToHBConstCoef.
    pub(crate) rad_sys_to_hb_const_coef: Option<Vec<f64>>,
    /// HBSM1570: dataHeatBalFanSys::RadSysToHBTinCoef.
    pub(crate) rad_sys_to_hb_tin_coef: Option<Vec<f64>>,
    /// HBSM1571: dataHeatBalFanSys::RadSysToHBQsrcCoef.
    pub(crate) rad_sys_to_hb_qsrc_coef: Option<Vec<f64>>,
    /// HBSM1572: dataHeatBalFanSys::QRadSysSource.
    pub(crate) q_rad_sys_source: Option<Vec<f64>>,
    /// HBSM1573: dataHeatBalFanSys::TCondFDSourceNode.
    pub(crate) t_cond_fd_source_node: Option<Vec<f64>>,
    /// HBSM1575: dataHeatBalFanSys::QRadSurfAFNDuct.
    pub(crate) q_rad_surf_afn_duct: Option<Vec<f64>>,
    /// HBSM1578: dataHeatBalFanSys::QPoolSurfNumerator.
    pub(crate) q_pool_surf_numerator: Option<Vec<f64>>,
    /// HBSM1579: dataHeatBalFanSys::PoolHeatTransCoefs.
    pub(crate) pool_heat_trans_coefs: Option<Vec<f64>>,
    /// HBSM1582: dataHeatBalFanSys::QPVSysSource.
    pub(crate) qpv_sys_source: Option<Vec<f64>>,
    /// HBSM1585: dataMstBal::TempOutsideAirFD.
    pub(crate) temp_outside_air_fd: Option<Vec<f64>>,
    /// HBSM1586: dataMstBal::RhoVaporAirOut.
    pub(crate) rho_vapor_air_out: Option<Vec<f64>>,
    /// HBSM1587: dataMstBal::RhoVaporSurfIn.
    pub(crate) rho_vapor_surf_in: Option<Vec<f64>>,
    /// HBSM1588: dataMstBal::RhoVaporAirIn.
    pub(crate) rho_vapor_air_in: Option<Vec<f64>>,
    /// HBSM1589: dataMstBal::HConvExtFD.
    pub(crate) h_conv_ext_fd: Option<Vec<f64>>,
    /// HBSM1590: dataMstBal::HMassConvExtFD.
    pub(crate) h_mass_conv_ext_fd: Option<Vec<f64>>,
    /// HBSM1591: dataMstBal::HConvInFD.
    pub(crate) h_conv_in_fd: Option<Vec<f64>>,
    /// HBSM1592: dataMstBal::HMassConvInFD.
    pub(crate) h_mass_conv_in_fd: Option<Vec<f64>>,
    /// HBSM1593: dataMstBal::HSkyFD.
    pub(crate) h_sky_fd: Option<Vec<f64>>,
    /// HBSM1594: dataMstBal::HGrndFD.
    pub(crate) h_grnd_fd: Option<Vec<f64>>,
    /// HBSM1595: dataMstBal::HAirFD.
    pub(crate) h_air_fd: Option<Vec<f64>>,
    /// HBSM1596: dataMstBal::HSurrFD.
    pub(crate) h_surr_fd: Option<Vec<f64>>,
    /// HBSM1598: dataSurface::SurfSkySolarInc.
    pub(crate) surf_sky_solar_inc: Option<Vec<f64>>,
    /// HBSM1599: dataSurface::SurfGndSolarInc.
    pub(crate) surf_gnd_solar_inc: Option<Vec<f64>>,
    /// HBSM1601: dataHeatBalSurf::SurfAbsSolarExt.
    pub(crate) surf_abs_solar_ext: Option<Vec<f64>>,
    /// HBSM1602: dataHeatBalSurf::SurfAbsThermalExt.
    pub(crate) surf_abs_thermal_ext: Option<Vec<f64>>,
    /// HBSM1603: dataHeatBalSurf::SurfRoughnessExt.
    pub(crate) surf_roughness_ext: Option<Vec<Option<ep_model::MaterialSurfaceRoughness>>>,
    /// HBSM1604: dataHeatBalSurf::SurfAbsSolarInt.
    pub(crate) surf_abs_solar_int: Option<Vec<f64>>,
    /// HBSM1605: dataHeatBalSurf::SurfAbsThermalInt.
    pub(crate) surf_abs_thermal_int: Option<Vec<f64>>,
    /// HBSM1574: allocated struct members remain unwritten until the suffix.
    pub(crate) surf_q_rad_from_hvac: Option<Vec<SourceHvacRadiantFlux>>,
    /// Actual DataHeatBalSurface.hh81 std::vector ctor empty; inside first-call resize.
    pub(crate) zone_has_mixed_ht_models: Vec<bool>,
}

fn reset_history_arrays(arrays: &mut CtfThermalHistoryArrays, count: usize) {
    for term in 0..19 {
        arrays.inside_temperature[term] = vec![0.0; count];
        arrays.outside_temperature[term] = vec![0.0; count];
        arrays.inside_flux[term] = vec![0.0; count];
        arrays.outside_flux[term] = vec![0.0; count];
    }
}

impl SourceSurfaceHeatBalanceArrays {
    /// Actual scalar allocation prefix1406-1605. The same history/report owner
    /// is dimensioned at its Source position; no second mutable history exists.
    pub(crate) fn allocate(
        &mut self,
        count: usize,
        any_internal_heat_source: bool,
        simple_ctf_only: bool,
        any_ems: bool,
        existing_history: &mut CtfThermalHistoryState,
    ) -> Result<(), Error> {
        if existing_history.surface_ids.len() != count {
            return Err(Error::Unavailable(
                "actual full CTF history Surface identity count",
            ));
        }
        self.surf_ctf_const_in_part = Some(vec![0.0; count]);
        self.surf_ctf_const_out_part = Some(vec![0.0; count]);
        self.surf_ctf_cross0 = Some(vec![0.0; count]);
        self.surf_ctf_inside0 = Some(vec![0.0; count]);
        self.surf_temp_out_hist = Some(vec![0.0; count]);
        self.surf_ctf_source_in0 = Some(vec![0.0; count]);
        self.surf_q_source_sink_hist = Some(vec![0.0; count]);
        self.surf_is_adiabatic = Some(vec![0; count]);
        self.surf_is_source_or_sink = Some(vec![0; count]);
        self.surf_is_operating_pool = Some(vec![0; count]);
        self.surf_temp_term = Some(vec![0.0; count]);
        self.surf_temp_div = Some(vec![0.0; count]);
        if any_internal_heat_source {
            return Err(Error::Unavailable(
                "AllocateSurfaceHeatBalArrays positive internal-source allocation1429",
            ));
        }
        self.surf_temp_eff_bulk_air = Some(vec![23.0; count]);
        self.surf_h_conv_int = Some(vec![0.0; count]);
        self.surf_h_conv_ext = Some(vec![0.0; count]);
        self.surf_h_air_ext = Some(vec![0.0; count]);
        self.surf_h_sky_ext = Some(vec![0.0; count]);
        self.surf_h_grd_ext = Some(vec![0.0; count]);
        self.surf_h_srd_surf_ext = Some(vec![0.0; count]);
        self.surf_temp_in = Some(vec![0.0; count]);
        self.surf_temp_ins_old = Some(vec![0.0; count]);
        self.surf_temp_in_tmp = Some(vec![0.0; count]);
        self.ref_air_temp = Some(vec![0.0; count]);
        self.surf_q_rad_lw_out_srd_surfs = Some(vec![0.0; count]);
        self.surf_win_q_rad_s_wwin_abs = Some(vec![vec![0.0; 7]; count]);
        self.surf_win_initial_dif_solwin_abs = Some(vec![vec![0.0; 6]; count]);
        self.surf_q_rad_sw_out_mv_ins = Some(vec![0.0; count]);
        self.surf_qdot_rad_int_gains_in_per_area = Some(vec![0.0; count]);
        self.surf_q_additional_heat_source_outside = Some(vec![0.0; count]);
        self.surf_q_additional_heat_source_inside = Some(vec![0.0; count]);
        reset_history_arrays(&mut existing_history.normal, count);
        if !simple_ctf_only || any_ems {
            existing_history.masters = Some(CtfThermalHistoryMasters {
                arrays: CtfThermalHistoryArrays {
                    inside_temperature: std::array::from_fn(|_| Vec::new()),
                    outside_temperature: std::array::from_fn(|_| Vec::new()),
                    inside_flux: std::array::from_fn(|_| Vec::new()),
                    outside_flux: std::array::from_fn(|_| Vec::new()),
                },
                current_num_histories: vec![0; count],
            });
            let masters = existing_history.masters.as_mut().ok_or(Error::Unavailable(
                "actual allocated master thermal histories",
            ))?;
            reset_history_arrays(&mut masters.arrays, count);
        }
        self.surf_temp_out = Some(vec![0.0; count]);
        self.surf_temp_in_mov_ins_rep = Some(vec![0.0; count]);
        self.surf_q_conv_in_rep = Some(vec![0.0; count]);
        self.surf_qdot_conv_in_per_area = Some(vec![0.0; count]);
        self.surf_qdot_conv_in_rep = Some(vec![0.0; count]);
        self.surf_q_rad_net_surf_in_rep = Some(vec![0.0; count]);
        self.surf_qdot_rad_net_surf_in_rep = Some(vec![0.0; count]);
        self.surf_q_rad_solar_in_rep = Some(vec![0.0; count]);
        self.surf_qdot_rad_solar_in_rep = Some(vec![0.0; count]);
        self.surf_qdot_rad_solar_in_rep_per_area = Some(vec![0.0; count]);
        self.surf_q_rad_lights_in_rep = Some(vec![0.0; count]);
        self.surf_qdot_rad_lights_in_rep = Some(vec![0.0; count]);
        self.surf_q_rad_int_gains_in_rep = Some(vec![0.0; count]);
        self.surf_qdot_rad_int_gains_in_rep = Some(vec![0.0; count]);
        self.surf_q_rad_hvac_in_rep = Some(vec![0.0; count]);
        self.surf_qdot_rad_hvac_in_rep = Some(vec![0.0; count]);
        self.surf_qdot_rad_hvac_in_per_area = Some(vec![0.0; count]);
        self.surf_q_conv_out_report = Some(vec![0.0; count]);
        self.surf_qdot_conv_out_per_area = Some(vec![0.0; count]);
        self.surf_qdot_conv_out_rep = Some(vec![0.0; count]);
        self.surf_qdot_rad_out_rep = Some(vec![0.0; count]);
        self.surf_qdot_rad_out_rep_per_area = Some(vec![0.0; count]);
        self.surf_q_rad_out_report = Some(vec![0.0; count]);
        self.surf_q_air_ext_report = Some(vec![0.0; count]);
        self.surf_q_heat_emi_report = Some(vec![0.0; count]);
        self.surf_opaq_sw_out_abs_total_report = Some(vec![0.0; count]);
        self.surf_opaq_sw_out_abs_energy_report = Some(vec![0.0; count]);
        existing_history.flux_reports.inside_conduction = vec![0.0; count];
        existing_history.flux_reports.inside_flux = vec![0.0; count];
        self.surf_opaq_ins_face_cond_gain_rep = Some(vec![0.0; count]);
        self.surf_opaq_ins_face_cond_loss_rep = Some(vec![0.0; count]);
        self.surf_opaq_ins_face_cond_energy = Some(vec![0.0; count]);
        existing_history.flux_reports.outside_conduction = vec![0.0; count];
        existing_history.flux_reports.outside_flux = vec![0.0; count];
        self.surf_opaq_ext_face_cond_gain_rep = Some(vec![0.0; count]);
        self.surf_opaq_ext_face_cond_loss_rep = Some(vec![0.0; count]);
        self.surf_opaq_out_face_cond_energy = Some(vec![0.0; count]);
        self.surf_opaq_avg_face_cond_gain_rep = Some(vec![0.0; count]);
        self.surf_opaq_avg_face_cond_loss_rep = Some(vec![0.0; count]);
        self.surf_opaq_avg_face_cond = Some(vec![0.0; count]);
        self.surf_opaq_avg_face_cond_flux = Some(vec![0.0; count]);
        self.surf_opaq_avg_face_cond_energy = Some(vec![0.0; count]);
        self.surf_opaq_storage_cond_gain_rep = Some(vec![0.0; count]);
        self.surf_opaq_storage_cond_loss_rep = Some(vec![0.0; count]);
        self.surf_opaq_storage_cond = Some(vec![0.0; count]);
        self.surf_opaq_storage_cond_flux = Some(vec![0.0; count]);
        self.surf_opaq_storage_cond_energy = Some(vec![0.0; count]);
        self.surf_opaq_ins_face_beam_sol_absorbed = Some(vec![0.0; count]);
        self.surf_opaq_q_rad_sw_out_abs = Some(vec![0.0; count]);
        self.surf_opaq_q_rad_sw_in_abs = Some(vec![0.0; count]);
        self.surf_opaq_initial_dif_sol_in_abs = Some(vec![0.0; count]);
        self.surf_win_initial_dif_sol_in_trans = Some(vec![0.0; count]);
        self.surf_win_initial_beam_sol_in_trans = Some(vec![0.0; count]);
        self.surf_qdot_rad_net_lw_in_per_area = Some(vec![0.0; count]);
        self.surf_qdot_rad_lights_in_per_area = Some(vec![0.0; count]);
        self.rad_sys_ti_hb_const_coef = Some(vec![0.0; count]);
        self.rad_sys_ti_hb_tout_coef = Some(vec![0.0; count]);
        self.rad_sys_ti_hb_qsrc_coef = Some(vec![0.0; count]);
        self.rad_sys_to_hb_const_coef = Some(vec![0.0; count]);
        self.rad_sys_to_hb_tin_coef = Some(vec![0.0; count]);
        self.rad_sys_to_hb_qsrc_coef = Some(vec![0.0; count]);
        self.q_rad_sys_source = Some(vec![0.0; count]);
        self.t_cond_fd_source_node = Some(vec![15.0; count]);
        self.surf_q_rad_from_hvac = Some(vec![SourceHvacRadiantFlux::default(); count]);
        self.q_rad_surf_afn_duct = Some(vec![0.0; count]);
        self.q_pool_surf_numerator = Some(vec![0.0; count]);
        self.pool_heat_trans_coefs = Some(vec![0.0; count]);
        self.qpv_sys_source = Some(vec![0.0; count]);
        self.temp_outside_air_fd = Some(vec![0.0; count]);
        self.rho_vapor_air_out = Some(vec![0.0; count]);
        self.rho_vapor_surf_in = Some(vec![0.0; count]);
        self.rho_vapor_air_in = Some(vec![0.0; count]);
        self.h_conv_ext_fd = Some(vec![0.0; count]);
        self.h_mass_conv_ext_fd = Some(vec![0.0; count]);
        self.h_conv_in_fd = Some(vec![0.0; count]);
        self.h_mass_conv_in_fd = Some(vec![0.0; count]);
        self.h_sky_fd = Some(vec![0.0; count]);
        self.h_grnd_fd = Some(vec![0.0; count]);
        self.h_air_fd = Some(vec![0.0; count]);
        self.h_surr_fd = Some(vec![0.0; count]);
        self.surf_sky_solar_inc = Some(vec![0.0; count]);
        self.surf_gnd_solar_inc = Some(vec![0.0; count]);
        self.surf_abs_solar_ext = Some(vec![0.0; count]);
        self.surf_abs_thermal_ext = Some(vec![0.0; count]);
        self.surf_roughness_ext = Some(vec![None; count]);
        self.surf_abs_solar_int = Some(vec![0.0; count]);
        self.surf_abs_thermal_int = Some(vec![0.0; count]);
        Ok(())
    }
}
