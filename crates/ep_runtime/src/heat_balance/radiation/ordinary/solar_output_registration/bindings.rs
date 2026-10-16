//! Resolves actual Source Solar cells on every registration/reporting read.
use super::{OutputError, SourceSolarOutputOwners, Unavailable};
use crate::heat_balance::source_output_registration::{
    SourceRealOutputOwners, SourceRealOutputTarget, SourceSolarOutputField as Field,
};

impl SourceRealOutputOwners for SourceSolarOutputOwners<'_> {
    fn real_output(&self, target: SourceRealOutputTarget) -> Result<&f64, OutputError> {
        let SourceRealOutputTarget::Solar { field, index } = target else {
            return Err(Unavailable::StorageDomain.into());
        };
        let number = usize::try_from(index)
            .ok()
            .filter(|&n| n > 0)
            .ok_or(Unavailable::StorageDomain)?;
        let i = number - 1;
        Ok(match field {
            Field::ZoneTransSolar => self
                .radiation
                .solar_timestep
                .heat_balance
                .zone_trans_solar
                .read_ref([number])
                .map_err(|error| OutputError::Input(Box::new(error)))?,
            Field::ZoneBmSolFrExtWinsRep => self
                .radiation
                .solar_timestep
                .heat_balance
                .zone_bm_sol_fr_ext_wins_rep
                .read_ref([number])
                .map_err(|error| OutputError::Input(Box::new(error)))?,
            Field::ZoneBmSolFrIntWinsRep => self
                .radiation
                .solar_timestep
                .heat_balance
                .zone_bm_sol_fr_int_wins_rep
                .read_ref([number])
                .map_err(|error| OutputError::Input(Box::new(error)))?,
            Field::ZoneDifSolFrExtWinsRep => self
                .radiation
                .solar_timestep
                .heat_balance
                .zone_dif_sol_fr_ext_wins_rep
                .read_ref([number])
                .map_err(|error| OutputError::Input(Box::new(error)))?,
            Field::ZoneDifSolFrIntWinsRep => self
                .radiation
                .solar_timestep
                .heat_balance
                .zone_dif_sol_fr_int_wins_rep
                .read_ref([number])
                .map_err(|error| OutputError::Input(Box::new(error)))?,
            Field::ZoneTransSolarEnergy => self
                .radiation
                .solar_timestep
                .heat_balance
                .zone_trans_solar_energy
                .read_ref([number])
                .map_err(|error| OutputError::Input(Box::new(error)))?,
            Field::ZoneBmSolFrExtWinsRepEnergy => self
                .radiation
                .solar_timestep
                .heat_balance
                .zone_bm_sol_fr_ext_wins_rep_energy
                .read_ref([number])
                .map_err(|error| OutputError::Input(Box::new(error)))?,
            Field::ZoneBmSolFrIntWinsRepEnergy => self
                .radiation
                .solar_timestep
                .heat_balance
                .zone_bm_sol_fr_int_wins_rep_energy
                .read_ref([number])
                .map_err(|error| OutputError::Input(Box::new(error)))?,
            Field::ZoneDifSolFrExtWinsRepEnergy => self
                .radiation
                .solar_timestep
                .heat_balance
                .zone_dif_sol_fr_ext_wins_rep_energy
                .read_ref([number])
                .map_err(|error| OutputError::Input(Box::new(error)))?,
            Field::ZoneDifSolFrIntWinsRepEnergy => self
                .radiation
                .solar_timestep
                .heat_balance
                .zone_dif_sol_fr_int_wins_rep_energy
                .read_ref([number])
                .map_err(|error| OutputError::Input(Box::new(error)))?,
            Field::ZoneWinHeatGainRep => self
                .radiation
                .solar_timestep
                .heat_balance
                .zone_win_heat_gain_rep
                .read_ref([number])
                .map_err(|error| OutputError::Input(Box::new(error)))?,
            Field::ZoneWinHeatLossRep => self
                .radiation
                .solar_timestep
                .heat_balance
                .zone_win_heat_loss_rep
                .read_ref([number])
                .map_err(|error| OutputError::Input(Box::new(error)))?,
            Field::ZoneWinHeatGainRepEnergy => self
                .radiation
                .solar_timestep
                .heat_balance
                .zone_win_heat_gain_rep_energy
                .read_ref([number])
                .map_err(|error| OutputError::Input(Box::new(error)))?,
            Field::ZoneWinHeatLossRepEnergy => self
                .radiation
                .solar_timestep
                .heat_balance
                .zone_win_heat_loss_rep_energy
                .read_ref([number])
                .map_err(|error| OutputError::Input(Box::new(error)))?,
            Field::ZoneOpaqSurfInsFaceCond => self
                .radiation
                .solar_timestep
                .heat_balance
                .zone_opaq_surf_ins_face_cond
                .read_ref([number])
                .map_err(|error| OutputError::Input(Box::new(error)))?,
            Field::ZoneOpaqSurfInsFaceCondGainRep => self
                .radiation
                .solar_timestep
                .heat_balance
                .zone_opaq_surf_ins_face_cond_gain_rep
                .read_ref([number])
                .map_err(|error| OutputError::Input(Box::new(error)))?,
            Field::ZoneOpaqSurfInsFaceCondLossRep => self
                .radiation
                .solar_timestep
                .heat_balance
                .zone_opaq_surf_ins_face_cond_loss_rep
                .read_ref([number])
                .map_err(|error| OutputError::Input(Box::new(error)))?,
            Field::ZoneOpaqSurfExtFaceCond => self
                .radiation
                .solar_timestep
                .heat_balance
                .zone_opaq_surf_ext_face_cond
                .read_ref([number])
                .map_err(|error| OutputError::Input(Box::new(error)))?,
            Field::ZoneOpaqSurfExtFaceCondGainRep => self
                .radiation
                .solar_timestep
                .heat_balance
                .zone_opaq_surf_ext_face_cond_gain_rep
                .read_ref([number])
                .map_err(|error| OutputError::Input(Box::new(error)))?,
            Field::ZoneOpaqSurfExtFaceCondLossRep => self
                .radiation
                .solar_timestep
                .heat_balance
                .zone_opaq_surf_ext_face_cond_loss_rep
                .read_ref([number])
                .map_err(|error| OutputError::Input(Box::new(error)))?,
            Field::ZnOpqSurfInsFaceCondGnRepEnrg => self
                .radiation
                .solar_timestep
                .heat_balance
                .zn_opq_surf_ins_face_cond_gn_rep_enrg
                .read_ref([number])
                .map_err(|error| OutputError::Input(Box::new(error)))?,
            Field::ZnOpqSurfInsFaceCondLsRepEnrg => self
                .radiation
                .solar_timestep
                .heat_balance
                .zn_opq_surf_ins_face_cond_ls_rep_enrg
                .read_ref([number])
                .map_err(|error| OutputError::Input(Box::new(error)))?,
            Field::ZnOpqSurfExtFaceCondGnRepEnrg => self
                .radiation
                .solar_timestep
                .heat_balance
                .zn_opq_surf_ext_face_cond_gn_rep_enrg
                .read_ref([number])
                .map_err(|error| OutputError::Input(Box::new(error)))?,
            Field::ZnOpqSurfExtFaceCondLsRepEnrg => self
                .radiation
                .solar_timestep
                .heat_balance
                .zn_opq_surf_ext_face_cond_ls_rep_enrg
                .read_ref([number])
                .map_err(|error| OutputError::Input(Box::new(error)))?,
            Field::SurfaceAzimuth => {
                &self
                    .geometry
                    .final_surfaces
                    .as_deref()
                    .and_then(|rows| rows.get(i))
                    .and_then(|row| row.row.vertex_progress.properties.as_ref())
                    .ok_or(Unavailable::StorageDomain)?
                    .azimuth_deg
            }
            Field::SurfaceSunlitArea => self
                .geometry
                .window_arrays
                .as_ref()
                .ok_or(Unavailable::StorageDomain)?
                .surf_sunlit_area
                .read_ref([number])
                .map_err(|error| OutputError::Input(Box::new(error)))?,
            Field::SurfaceSunlitFrac => self
                .geometry
                .window_arrays
                .as_ref()
                .ok_or(Unavailable::StorageDomain)?
                .surf_sunlit_frac
                .read_ref([number])
                .map_err(|error| OutputError::Input(Box::new(error)))?,
            Field::SurfQRadSWOutIncident => self
                .radiation
                .solar_timestep
                .heat_balance
                .surf_q_rad_sw_out_incident
                .read_ref([number])
                .map_err(|error| OutputError::Input(Box::new(error)))?,
            Field::SurfQRadSWOutIncidentBeam => self
                .radiation
                .solar_timestep
                .heat_balance
                .surf_q_rad_sw_out_incident_beam
                .read_ref([number])
                .map_err(|error| OutputError::Input(Box::new(error)))?,
            Field::SurfQRadSWOutIncidentSkyDiffuse => self
                .radiation
                .solar_timestep
                .heat_balance
                .surf_q_rad_sw_out_incident_sky_diffuse
                .read_ref([number])
                .map_err(|error| OutputError::Input(Box::new(error)))?,
            Field::SurfQRadSWOutIncidentGndDiffuse => self
                .radiation
                .solar_timestep
                .heat_balance
                .surf_q_rad_sw_out_incident_gnd_diffuse
                .read_ref([number])
                .map_err(|error| OutputError::Input(Box::new(error)))?,
            Field::SurfCosIncidenceAngle => self
                .radiation
                .solar_timestep
                .heat_balance
                .surf_cos_incidence_angle
                .read_ref([number])
                .map_err(|error| OutputError::Input(Box::new(error)))?,
            Field::SurfQRadSWOutIncSkyDiffReflGnd => self
                .radiation
                .solar_timestep
                .heat_balance
                .surf_q_rad_sw_out_inc_sky_diff_refl_gnd
                .read_ref([number])
                .map_err(|error| OutputError::Input(Box::new(error)))?,
            Field::SurfQRadSWOutIncSkyDiffReflObs => self
                .radiation
                .solar_timestep
                .heat_balance
                .surf_q_rad_sw_out_inc_sky_diff_refl_obs
                .read_ref([number])
                .map_err(|error| OutputError::Input(Box::new(error)))?,
            Field::SurfQRadSWOutIncBmToBmReflObs => self
                .radiation
                .solar_timestep
                .heat_balance
                .surf_q_rad_sw_out_inc_bm_to_bm_refl_obs
                .read_ref([number])
                .map_err(|error| OutputError::Input(Box::new(error)))?,
            Field::SurfQRadSWOutIncBmToDiffReflObs => self
                .radiation
                .solar_timestep
                .heat_balance
                .surf_q_rad_sw_out_inc_bm_to_diff_refl_obs
                .read_ref([number])
                .map_err(|error| OutputError::Input(Box::new(error)))?,
            Field::SurfQRadSWOutIncBmToDiffReflGnd => self
                .radiation
                .solar_timestep
                .heat_balance
                .surf_q_rad_sw_out_inc_bm_to_diff_refl_gnd
                .read_ref([number])
                .map_err(|error| OutputError::Input(Box::new(error)))?,
            Field::SurfAnisoSkyMult => self
                .radiation
                .solar_timestep
                .shading
                .surf_aniso_sky_mult
                .read_ref([number])
                .map_err(|error| OutputError::Input(Box::new(error)))?,
            Field::SurfWinBSDFBeamThetaRep => self
                .radiation
                .solar_timestep
                .heat_balance
                .surf_win_bsdf_beam_theta_rep
                .read_ref([number])
                .map_err(|error| OutputError::Input(Box::new(error)))?,
            Field::SurfWinBSDFBeamPhiRep => self
                .radiation
                .solar_timestep
                .heat_balance
                .surf_win_bsdf_beam_phi_rep
                .read_ref([number])
                .map_err(|error| OutputError::Input(Box::new(error)))?,
            Field::SurfBmIncInsSurfIntensRep => self
                .radiation
                .solar_timestep
                .heat_balance
                .surf_bm_inc_ins_surf_intens_rep
                .read_ref([number])
                .map_err(|error| OutputError::Input(Box::new(error)))?,
            Field::SurfBmIncInsSurfAmountRep => self
                .radiation
                .solar_timestep
                .heat_balance
                .surf_bm_inc_ins_surf_amount_rep
                .read_ref([number])
                .map_err(|error| OutputError::Input(Box::new(error)))?,
            Field::SurfIntBmIncInsSurfIntensRep => self
                .radiation
                .solar_timestep
                .heat_balance
                .surf_int_bm_inc_ins_surf_intens_rep
                .read_ref([number])
                .map_err(|error| OutputError::Input(Box::new(error)))?,
            Field::SurfIntBmIncInsSurfAmountRep => self
                .radiation
                .solar_timestep
                .heat_balance
                .surf_int_bm_inc_ins_surf_amount_rep
                .read_ref([number])
                .map_err(|error| OutputError::Input(Box::new(error)))?,
            Field::SurfInitialDifSolInAbsReport => self
                .radiation
                .solar_timestep
                .heat_balance
                .surf_initial_dif_sol_in_abs_report
                .read_ref([number])
                .map_err(|error| OutputError::Input(Box::new(error)))?,
            Field::SurfWinInitialDifSolInTransReport => self
                .radiation
                .solar_timestep
                .heat_balance
                .surf_win_initial_dif_sol_in_trans_report
                .read_ref([number])
                .map_err(|error| OutputError::Input(Box::new(error)))?,
            Field::SurfSWInAbsTotalReport => self
                .radiation
                .solar_timestep
                .heat_balance
                .surf_sw_in_abs_total_report
                .read_ref([number])
                .map_err(|error| OutputError::Input(Box::new(error)))?,
            Field::SurfBmIncInsSurfAmountRepEnergy => self
                .radiation
                .solar_timestep
                .heat_balance
                .surf_bm_inc_ins_surf_amount_rep_energy
                .read_ref([number])
                .map_err(|error| OutputError::Input(Box::new(error)))?,
            Field::SurfIntBmIncInsSurfAmountRepEnergy => self
                .radiation
                .solar_timestep
                .heat_balance
                .surf_int_bm_inc_ins_surf_amount_rep_energy
                .read_ref([number])
                .map_err(|error| OutputError::Input(Box::new(error)))?,
            Field::SurfCurDifShdgRatioIsoSky => self
                .radiation
                .solar_timestep
                .shading
                .surf_cur_dif_shdg_ratio_iso_sky
                .read_ref([number])
                .map_err(|error| OutputError::Input(Box::new(error)))?,
            Field::SurfDifShdgRatioIsoSky => self
                .radiation
                .solar_timestep
                .shading
                .surf_dif_shdg_ratio_iso_sky
                .read_ref([number])
                .map_err(|error| OutputError::Input(Box::new(error)))?,
            Field::SurfDifShdgRatioHoriz => self
                .radiation
                .solar_timestep
                .shading
                .surf_dif_shdg_ratio_horiz
                .read_ref([number])
                .map_err(|error| OutputError::Input(Box::new(error)))?,
            Field::SurfWithShdgIsoSky => self
                .radiation
                .solar_timestep
                .shading
                .surf_with_shdg_iso_sky
                .read_ref([number])
                .map_err(|error| OutputError::Input(Box::new(error)))?,
            Field::SurfWoShdgIsoSky => self
                .radiation
                .solar_timestep
                .shading
                .surf_wo_shdg_iso_sky
                .read_ref([number])
                .map_err(|error| OutputError::Input(Box::new(error)))?,
        })
    }
}
