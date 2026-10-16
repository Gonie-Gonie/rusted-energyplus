//! Genuine InitSurface conduction-history constants and the actual HVAC reset.
use super::SourceSurfaceHeatBalanceArrays;
use super::prefix::{index, source_groups};
use crate::heat_balance::ctf_public_initialization::OrderedCtfPublicInitialization;
use crate::heat_balance::ctf_thermal_history::CtfThermalHistoryState;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::surface::SurfaceGeometryInputState;
use crate::heat_balance::selected_heat_balance_input::zone::SelectedZoneInputOwners;
use crate::heat_balance::selected_heat_balance_input::{
    HeatBalanceInputError as Error, SelectedHeatBalanceInputState, SelectedHeatTransferModel,
};

impl SourceSurfaceHeatBalanceArrays {
    /// HBSM490-536. Actual Source Zone/Space/opaque order, actual term1..N
    /// expression order, and the same persistent normal history arrays.
    pub(crate) fn compute_source_ctf_constants(
        &mut self,
        geometry: &SurfaceGeometryInputState,
        zones: &SelectedZoneInputOwners,
        constructions: &SelectedHeatBalanceInputState,
        public: &OrderedCtfPublicInitialization,
        history: &CtfThermalHistoryState,
        globals: &SelectedHeatBalanceGlobals,
    ) -> Result<(), Error> {
        let surfaces = geometry
            .final_surfaces
            .as_ref()
            .ok_or(Error::Unavailable("actual final Surface allocation"))?;
        for group in source_groups(zones, globals.num_of_zones, true)? {
            for number in group {
                let i = index(number)?;
                let surface = surfaces
                    .get(i)
                    .ok_or(Error::Unavailable("actual opaque Source Surface ordinal"))?;
                if surface.row.heat_transfer_algorithm != SelectedHeatTransferModel::Ctf {
                    // EMPD is an explicitly unrepresented selector in the current
                    // enum. Other selectors continue exactly at the Source guard.
                    continue;
                }
                let construction = constructions
                    .construction_usage
                    .get(index(surface.row.construction)?)
                    .ok_or(Error::Unavailable("actual Surface Construction owner"))?;
                let id = construction
                    .construction_id
                    .ok_or(Error::Unavailable("actual public Construction identity"))?;
                let coefficients = &public
                    .constructions
                    .iter()
                    .find(|row| row.construction_id == id)
                    .ok_or(Error::Unavailable(
                        "same returned public Construction owner",
                    ))?
                    .result
                    .as_ref()
                    .map_err(|_| Error::Unavailable("returned public Construction storage"))?
                    .storage;
                *self
                    .surf_ctf_const_out_part
                    .as_mut()
                    .and_then(|values| values.get_mut(i))
                    .ok_or(Error::Unavailable("SurfCTFConstOutPart allocation"))? = 0.0;
                *self
                    .surf_ctf_const_in_part
                    .as_mut()
                    .and_then(|values| values.get_mut(i))
                    .ok_or(Error::Unavailable("SurfCTFConstInPart allocation"))? = 0.0;
                if coefficients.num_ctf_terms <= 1 {
                    continue;
                }
                let surface_id = surface
                    .typed_id
                    .ok_or(Error::Unavailable("actual Source Surface typed custody"))?;
                let h = history
                    .surface_ids
                    .iter()
                    .position(|id| *id == surface_id)
                    .ok_or(Error::Unavailable("same normal-history Surface owner"))?;
                for term in 1..=usize::try_from(coefficients.num_ctf_terms)
                    .map_err(|_| Error::Unavailable("actual NumCTFTerms"))?
                {
                    let ctf_cross = *coefficients
                        .cross
                        .get(term)
                        .ok_or(Error::Unavailable("CTFCross fixed capacity"))?;
                    let th11 = *history
                        .normal
                        .outside_temperature
                        .get(term)
                        .and_then(|values| values.get(h))
                        .ok_or(Error::Unavailable("SurfOutsideTempHist term+1"))?;
                    let th12 = *history
                        .normal
                        .inside_temperature
                        .get(term)
                        .and_then(|values| values.get(h))
                        .ok_or(Error::Unavailable("SurfInsideTempHist term+1"))?;
                    let qh11 = *history
                        .normal
                        .outside_flux
                        .get(term)
                        .and_then(|values| values.get(h))
                        .ok_or(Error::Unavailable("SurfOutsideFluxHist term+1"))?;
                    let qh12 = *history
                        .normal
                        .inside_flux
                        .get(term)
                        .and_then(|values| values.get(h))
                        .ok_or(Error::Unavailable("SurfInsideFluxHist term+1"))?;
                    *self
                        .surf_ctf_const_in_part
                        .as_mut()
                        .and_then(|values| values.get_mut(i))
                        .ok_or(Error::Unavailable("SurfCTFConstInPart allocation"))? +=
                        ctf_cross * th11 - coefficients.inside[term] * th12
                            + coefficients.flux[term] * qh12;
                    *self
                        .surf_ctf_const_out_part
                        .as_mut()
                        .and_then(|values| values.get_mut(i))
                        .ok_or(Error::Unavailable("SurfCTFConstOutPart allocation"))? +=
                        coefficients.outside[term] * th11 - ctf_cross * th12
                            + coefficients.flux[term] * qh11;
                }
            }
        }
        Ok(())
    }

    /// HBSM581-603. This is called after the internal-source branch; it neither
    /// skips that branch nor clears the shared InitSurface first-time member.
    pub(crate) fn reset_source_radiant_system_terms(
        &mut self,
        zones: &SelectedZoneInputOwners,
        globals: &SelectedHeatBalanceGlobals,
    ) -> Result<(), Error> {
        for group in source_groups(zones, globals.num_of_zones, false)? {
            for number in group {
                let i = index(number)?;
                *self
                    .rad_sys_ti_hb_const_coef
                    .as_mut()
                    .and_then(|values| values.get_mut(i))
                    .ok_or(Error::Unavailable("RadSysTiHBConstCoef allocation"))? = 0.0;
                *self
                    .rad_sys_ti_hb_tout_coef
                    .as_mut()
                    .and_then(|values| values.get_mut(i))
                    .ok_or(Error::Unavailable("RadSysTiHBToutCoef allocation"))? = 0.0;
                *self
                    .rad_sys_ti_hb_qsrc_coef
                    .as_mut()
                    .and_then(|values| values.get_mut(i))
                    .ok_or(Error::Unavailable("RadSysTiHBQsrcCoef allocation"))? = 0.0;
                *self
                    .rad_sys_to_hb_const_coef
                    .as_mut()
                    .and_then(|values| values.get_mut(i))
                    .ok_or(Error::Unavailable("RadSysToHBConstCoef allocation"))? = 0.0;
                *self
                    .rad_sys_to_hb_tin_coef
                    .as_mut()
                    .and_then(|values| values.get_mut(i))
                    .ok_or(Error::Unavailable("RadSysToHBTinCoef allocation"))? = 0.0;
                *self
                    .rad_sys_to_hb_qsrc_coef
                    .as_mut()
                    .and_then(|values| values.get_mut(i))
                    .ok_or(Error::Unavailable("RadSysToHBQsrcCoef allocation"))? = 0.0;
                *self
                    .q_rad_sys_source
                    .as_mut()
                    .and_then(|values| values.get_mut(i))
                    .ok_or(Error::Unavailable("QRadSysSource allocation"))? = 0.0;
                *self
                    .qpv_sys_source
                    .as_mut()
                    .and_then(|values| values.get_mut(i))
                    .ok_or(Error::Unavailable("QPVSysSource allocation"))? = 0.0;
                *self
                    .q_pool_surf_numerator
                    .as_mut()
                    .and_then(|values| values.get_mut(i))
                    .ok_or(Error::Unavailable("QPoolSurfNumerator allocation"))? = 0.0;
                *self
                    .pool_heat_trans_coefs
                    .as_mut()
                    .and_then(|values| values.get_mut(i))
                    .ok_or(Error::Unavailable("PoolHeatTransCoefs allocation"))? = 0.0;
            }
        }
        Ok(())
    }

    /// HBSM605-612. The actual Source list must be supplied; no HT/all-surface
    /// replacement list, initialized allocation assumption or default is used.
    pub(crate) fn reset_source_hvac_radiant_flux(
        &mut self,
        all_gets_radiant_heat_surfaces: &[i32],
    ) -> Result<(), Error> {
        for &number in all_gets_radiant_heat_surfaces {
            let row = self
                .surf_q_rad_from_hvac
                .as_mut()
                .and_then(|values| values.get_mut(index(number).ok()?))
                .ok_or(Error::Unavailable(
                    "actual allGetsRadiantHeatSurfaceList and struct allocation",
                ))?;
            row.ht_rad_sys = Some(0.0);
            row.hw_baseboard = Some(0.0);
            row.steam_baseboard = Some(0.0);
            row.elec_baseboard = Some(0.0);
            row.cooling_panel = Some(0.0);
        }
        Ok(())
    }
}
