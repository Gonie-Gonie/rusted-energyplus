//! HBSM2252-2322 surface-only BeginEnvrn prefix. The genuine caller must
//! reconstruct the real Zone/Space AirHB owners and MRT before invoking it.
//! The accepted sole normal/master-history initializer follows this prefix.
use super::OrdinarySurfaceInitialization;
use super::prefix::index;
use crate::heat_balance::ctf_thermal_history::CtfThermalHistoryState;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::heat_balance::selected_heat_balance_input::surface::SurfaceGeometryInputState;
use crate::heat_balance::selected_heat_balance_input::zone::SelectedZoneInputOwners;

impl OrdinarySurfaceInitialization {
    pub(crate) fn initialize_source_surface_environment_bulk(
        &mut self,
        geometry: &SurfaceGeometryInputState,
        zones: &SelectedZoneInputOwners,
        globals: &SelectedHeatBalanceGlobals,
        history: &mut CtfThermalHistoryState,
    ) -> Result<(), Error> {
        let zone_rows = zones
            .zones
            .as_ref()
            .ok_or(Error::Unavailable("same Zone allocation"))?;
        let spaces = zones
            .spaces
            .as_ref()
            .ok_or(Error::Unavailable("same Space allocation"))?;
        let surfaces = geometry
            .final_surfaces
            .as_ref()
            .ok_or(Error::Unavailable("same final Surface allocation"))?;
        let count = usize::try_from(globals.num_of_zones)
            .map_err(|_| Error::Unavailable("actual NumOfZones"))?;
        if zone_rows.len() != count {
            return Err(Error::Unavailable("actual Zone count association"));
        }
        for zone in zone_rows {
            for &space_number in &zone.space_indexes {
                let space = spaces
                    .get(index(space_number)?)
                    .ok_or(Error::Unavailable("same Source Space ordinal"))?;
                for number in space.ht_surface_first..=space.ht_surface_last {
                    let i = index(number)?;
                    *self
                        .arrays
                        .surf_temp_eff_bulk_air
                        .as_mut()
                        .and_then(|values| values.get_mut(i))
                        .ok_or(Error::Unavailable(
                            "SurfTempEffBulkAir BeginEnvrn allocation",
                        ))? = 23.0;
                    *self
                        .arrays
                        .surf_temp_in
                        .as_mut()
                        .and_then(|values| values.get_mut(i))
                        .ok_or(Error::Unavailable("SurfTempIn BeginEnvrn allocation"))? = 23.0;
                    *self
                        .arrays
                        .surf_temp_in_tmp
                        .as_mut()
                        .and_then(|values| values.get_mut(i))
                        .ok_or(Error::Unavailable("SurfTempInTmp BeginEnvrn allocation"))? = 23.0;
                    *self
                        .arrays
                        .surf_h_conv_int
                        .as_mut()
                        .and_then(|values| values.get_mut(i))
                        .ok_or(Error::Unavailable("SurfHConvInt BeginEnvrn allocation"))? = 3.076;
                    *self
                        .arrays
                        .surf_h_conv_ext
                        .as_mut()
                        .and_then(|values| values.get_mut(i))
                        .ok_or(Error::Unavailable("SurfHConvExt BeginEnvrn allocation"))? = 0.0;
                    *self
                        .arrays
                        .surf_h_air_ext
                        .as_mut()
                        .and_then(|values| values.get_mut(i))
                        .ok_or(Error::Unavailable("SurfHAirExt BeginEnvrn allocation"))? = 0.0;
                    *self
                        .arrays
                        .surf_h_sky_ext
                        .as_mut()
                        .and_then(|values| values.get_mut(i))
                        .ok_or(Error::Unavailable("SurfHSkyExt BeginEnvrn allocation"))? = 0.0;
                    *self
                        .arrays
                        .surf_h_grd_ext
                        .as_mut()
                        .and_then(|values| values.get_mut(i))
                        .ok_or(Error::Unavailable("SurfHGrdExt BeginEnvrn allocation"))? = 0.0;
                    *self
                        .arrays
                        .surf_h_srd_surf_ext
                        .as_mut()
                        .and_then(|values| values.get_mut(i))
                        .ok_or(Error::Unavailable("SurfHSrdSurfExt BeginEnvrn allocation"))? = 0.0;
                    *self
                        .arrays
                        .surf_temp_out
                        .as_mut()
                        .and_then(|values| values.get_mut(i))
                        .ok_or(Error::Unavailable("SurfTempOut BeginEnvrn allocation"))? = 0.0;
                    *self
                        .arrays
                        .surf_temp_in_mov_ins_rep
                        .as_mut()
                        .and_then(|values| values.get_mut(i))
                        .ok_or(Error::Unavailable(
                            "SurfTempInMovInsRep BeginEnvrn allocation",
                        ))? = 0.0;
                    *self
                        .arrays
                        .surf_q_conv_in_rep
                        .as_mut()
                        .and_then(|values| values.get_mut(i))
                        .ok_or(Error::Unavailable("SurfQConvInRep BeginEnvrn allocation"))? = 0.0;
                    *self
                        .arrays
                        .surf_qdot_conv_in_rep
                        .as_mut()
                        .and_then(|values| values.get_mut(i))
                        .ok_or(Error::Unavailable(
                            "SurfQdotConvInRep BeginEnvrn allocation",
                        ))? = 0.0;
                    *self
                        .arrays
                        .surf_qdot_conv_in_per_area
                        .as_mut()
                        .and_then(|values| values.get_mut(i))
                        .ok_or(Error::Unavailable(
                            "SurfQdotConvInPerArea BeginEnvrn allocation",
                        ))? = 0.0;
                    *self
                        .arrays
                        .surf_q_rad_net_surf_in_rep
                        .as_mut()
                        .and_then(|values| values.get_mut(i))
                        .ok_or(Error::Unavailable(
                            "SurfQRadNetSurfInRep BeginEnvrn allocation",
                        ))? = 0.0;
                    *self
                        .arrays
                        .surf_qdot_rad_net_surf_in_rep
                        .as_mut()
                        .and_then(|values| values.get_mut(i))
                        .ok_or(Error::Unavailable(
                            "SurfQdotRadNetSurfInRep BeginEnvrn allocation",
                        ))? = 0.0;
                    *self
                        .arrays
                        .surf_q_rad_solar_in_rep
                        .as_mut()
                        .and_then(|values| values.get_mut(i))
                        .ok_or(Error::Unavailable(
                            "SurfQRadSolarInRep BeginEnvrn allocation",
                        ))? = 0.0;
                    *self
                        .arrays
                        .surf_qdot_rad_solar_in_rep
                        .as_mut()
                        .and_then(|values| values.get_mut(i))
                        .ok_or(Error::Unavailable(
                            "SurfQdotRadSolarInRep BeginEnvrn allocation",
                        ))? = 0.0;
                    *self
                        .arrays
                        .surf_qdot_rad_solar_in_rep_per_area
                        .as_mut()
                        .and_then(|values| values.get_mut(i))
                        .ok_or(Error::Unavailable(
                            "SurfQdotRadSolarInRepPerArea BeginEnvrn allocation",
                        ))? = 0.0;
                    *self
                        .arrays
                        .surf_q_rad_lights_in_rep
                        .as_mut()
                        .and_then(|values| values.get_mut(i))
                        .ok_or(Error::Unavailable(
                            "SurfQRadLightsInRep BeginEnvrn allocation",
                        ))? = 0.0;
                    *self
                        .arrays
                        .surf_qdot_rad_lights_in_rep
                        .as_mut()
                        .and_then(|values| values.get_mut(i))
                        .ok_or(Error::Unavailable(
                            "SurfQdotRadLightsInRep BeginEnvrn allocation",
                        ))? = 0.0;
                    *self
                        .arrays
                        .surf_q_rad_int_gains_in_rep
                        .as_mut()
                        .and_then(|values| values.get_mut(i))
                        .ok_or(Error::Unavailable(
                            "SurfQRadIntGainsInRep BeginEnvrn allocation",
                        ))? = 0.0;
                    *self
                        .arrays
                        .surf_qdot_rad_int_gains_in_rep
                        .as_mut()
                        .and_then(|values| values.get_mut(i))
                        .ok_or(Error::Unavailable(
                            "SurfQdotRadIntGainsInRep BeginEnvrn allocation",
                        ))? = 0.0;
                    *self
                        .arrays
                        .surf_q_rad_hvac_in_rep
                        .as_mut()
                        .and_then(|values| values.get_mut(i))
                        .ok_or(Error::Unavailable(
                            "SurfQRadHVACInRep BeginEnvrn allocation",
                        ))? = 0.0;
                    *self
                        .arrays
                        .surf_qdot_rad_hvac_in_rep
                        .as_mut()
                        .and_then(|values| values.get_mut(i))
                        .ok_or(Error::Unavailable(
                            "SurfQdotRadHVACInRep BeginEnvrn allocation",
                        ))? = 0.0;
                    *self
                        .arrays
                        .surf_qdot_rad_hvac_in_per_area
                        .as_mut()
                        .and_then(|values| values.get_mut(i))
                        .ok_or(Error::Unavailable(
                            "SurfQdotRadHVACInPerArea BeginEnvrn allocation",
                        ))? = 0.0;
                    *self
                        .arrays
                        .surf_q_conv_out_report
                        .as_mut()
                        .and_then(|values| values.get_mut(i))
                        .ok_or(Error::Unavailable(
                            "SurfQConvOutReport BeginEnvrn allocation",
                        ))? = 0.0;
                    *self
                        .arrays
                        .surf_qdot_conv_out_rep
                        .as_mut()
                        .and_then(|values| values.get_mut(i))
                        .ok_or(Error::Unavailable(
                            "SurfQdotConvOutRep BeginEnvrn allocation",
                        ))? = 0.0;
                    *self
                        .arrays
                        .surf_qdot_conv_out_per_area
                        .as_mut()
                        .and_then(|values| values.get_mut(i))
                        .ok_or(Error::Unavailable(
                            "SurfQdotConvOutPerArea BeginEnvrn allocation",
                        ))? = 0.0;
                    *self
                        .arrays
                        .surf_q_rad_out_report
                        .as_mut()
                        .and_then(|values| values.get_mut(i))
                        .ok_or(Error::Unavailable(
                            "SurfQRadOutReport BeginEnvrn allocation",
                        ))? = 0.0;
                    *self
                        .arrays
                        .surf_qdot_rad_out_rep
                        .as_mut()
                        .and_then(|values| values.get_mut(i))
                        .ok_or(Error::Unavailable(
                            "SurfQdotRadOutRep BeginEnvrn allocation",
                        ))? = 0.0;
                    *self
                        .arrays
                        .surf_qdot_rad_out_rep_per_area
                        .as_mut()
                        .and_then(|values| values.get_mut(i))
                        .ok_or(Error::Unavailable(
                            "SurfQdotRadOutRepPerArea BeginEnvrn allocation",
                        ))? = 0.0;
                    *self
                        .arrays
                        .surf_q_air_ext_report
                        .as_mut()
                        .and_then(|values| values.get_mut(i))
                        .ok_or(Error::Unavailable(
                            "SurfQAirExtReport BeginEnvrn allocation",
                        ))? = 0.0;
                    *self
                        .arrays
                        .surf_q_heat_emi_report
                        .as_mut()
                        .and_then(|values| values.get_mut(i))
                        .ok_or(Error::Unavailable(
                            "SurfQHeatEmiReport BeginEnvrn allocation",
                        ))? = 0.0;
                }
                if space.opaque_or_intmass_first >= 0 {
                    for number in space.opaque_or_intmass_first..=space.opaque_or_intmass_last {
                        let i = index(number)?;
                        let id = surfaces
                            .get(i)
                            .and_then(|row| row.typed_id)
                            .ok_or(Error::Unavailable("actual opaque Surface custody"))?;
                        let h = history
                            .surface_ids
                            .iter()
                            .position(|candidate| *candidate == id)
                            .ok_or(Error::Unavailable("same conduction-report Surface owner"))?;
                        *history.flux_reports.inside_conduction.get_mut(h).ok_or(
                            Error::Unavailable("SurfOpaqInsFaceCond BeginEnvrn allocation"),
                        )? = 0.0;
                        *history.flux_reports.inside_flux.get_mut(h).ok_or(
                            Error::Unavailable("SurfOpaqInsFaceCondFlux BeginEnvrn allocation"),
                        )? = 0.0;
                        *self
                            .arrays
                            .surf_opaq_ins_face_cond_energy
                            .as_mut()
                            .and_then(|values| values.get_mut(i))
                            .ok_or(Error::Unavailable(
                                "SurfOpaqInsFaceCondEnergy BeginEnvrn allocation",
                            ))? = 0.0;
                        *self
                            .arrays
                            .surf_opaq_ins_face_beam_sol_absorbed
                            .as_mut()
                            .and_then(|values| values.get_mut(i))
                            .ok_or(Error::Unavailable(
                                "SurfOpaqInsFaceBeamSolAbsorbed BeginEnvrn allocation",
                            ))? = 0.0;
                    }
                }
                if space.window_first >= 0 {
                    // Preserve empty-range behavior, checked first-row access and the first unsupported positive-child failure after prior writes.
                    #[allow(clippy::never_loop)]
                    for _number in space.window_first..=space.window_last {
                        return Err(Error::Unavailable(
                            "positive InitThermalAndFluxHistories window-frame prefix2306",
                        ));
                    }
                }
            }
        }
        Ok(())
    }

    /// HBSM2378. This real write occurs after the accepted normal/master
    /// history reset and optional internal-source histories, before boundary flux.
    pub(crate) fn reset_source_cond_fd_relaxation_factor(&mut self) {
        self.cond_fd_relaxation_factor = self.cond_fd_relaxation_factor_input;
    }
}
