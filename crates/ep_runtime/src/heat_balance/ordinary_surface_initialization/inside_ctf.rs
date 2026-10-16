//! Source CTF-only cached terms and direct NetLW consumer, without a second solve.
use super::SourceSurfaceHeatBalanceArrays;
use super::prefix::{index, source_groups};
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::heat_balance::selected_heat_balance_input::surface::SurfaceGeometryInputState;
use crate::heat_balance::selected_heat_balance_input::zone::SelectedZoneInputOwners;

pub(super) fn value(values: &Option<Vec<f64>>, i: usize) -> Result<f64, Error> {
    values
        .as_ref()
        .and_then(|values| values.get(i))
        .copied()
        .ok_or(Error::Unavailable(
            "actual allocated Source inside-CTF scalar",
        ))
}
fn flag(values: &Option<Vec<i32>>, i: usize) -> Result<i32, Error> {
    values
        .as_ref()
        .and_then(|values| values.get(i))
        .copied()
        .ok_or(Error::Unavailable(
            "actual allocated Source inside-CTF flag",
        ))
}
pub(super) fn set(values: &mut Option<Vec<f64>>, i: usize, value: f64) -> Result<(), Error> {
    *values
        .as_mut()
        .and_then(|values| values.get_mut(i))
        .ok_or(Error::Unavailable(
            "actual allocated Source inside-CTF output",
        ))? = value;
    Ok(())
}

impl SourceSurfaceHeatBalanceArrays {
    /// HBSM8788-8802 and repeated8838-8852; the genuine caller invokes this
    /// after initial/30-iteration InitIntConvCoeff at the two exact Source sites.
    pub(crate) fn precalculate_source_inside_terms(
        &mut self,
        geometry: &SurfaceGeometryInputState,
        zones: &SelectedZoneInputOwners,
        timestep_zone_seconds: f64,
        globals: &SelectedHeatBalanceGlobals,
    ) -> Result<(), Error> {
        let surfaces = geometry
            .final_surfaces
            .as_ref()
            .ok_or(Error::Unavailable("inside actual final Surface allocation"))?;
        for group in source_groups(zones, globals.num_of_zones, true)? {
            for number in group {
                let i = index(number)?;
                let surface = surfaces
                    .get(i)
                    .ok_or(Error::Unavailable("inside actual Source opaque ordinal"))?;
                if geometry.use_representative_surface_calculations
                    && number != surface.representative_calc_surface_num
                {
                    continue;
                }
                self.precalculate_source_inside_terms_at(i, timestep_zone_seconds)?;
            }
        }
        Ok(())
    }

    /// HBSM8913-8933. The next surface solve reads the same array written by
    /// exchange_source_inside_iteration. No compatibility longwave probe,
    /// copied Surface radiation field, outside solve or zone correction runs.
    pub(crate) fn solve_source_inside_opaque_ctf(
        &mut self,
        geometry: &SurfaceGeometryInputState,
        zones: &SelectedZoneInputOwners,
        globals: &SelectedHeatBalanceGlobals,
    ) -> Result<(), Error> {
        for group in source_groups(zones, globals.num_of_zones, true)? {
            for number in group {
                let i = index(number)?;
                self.solve_source_inside_opaque_ctf_at(i)?;
            }
        }
        // The later movable-insulation/window/history/convergence suffix is
        // owned by the complete CTF-only caller and is not certified here.
        let _ = geometry;
        Ok(())
    }

    /// Same scalar Source8788-8800 expression, shared by actual per-Space caller.
    pub(super) fn precalculate_source_inside_terms_at(
        &mut self,
        i: usize,
        timestep_zone_seconds: f64,
    ) -> Result<(), Error> {
        let term = value(&self.surf_ctf_const_in_part, i)?
            + value(&self.surf_qdot_rad_int_gains_in_per_area, i)?
            + value(&self.surf_opaq_q_rad_sw_in_abs, i)?
            + value(&self.surf_q_additional_heat_source_inside, i)?
            + value(&self.surf_h_conv_int, i)? * value(&self.ref_air_temp, i)?
            + value(&self.surf_qdot_rad_hvac_in_per_area, i)?
            + (value(&self.q_rad_surf_afn_duct, i)? / timestep_zone_seconds);
        set(&mut self.surf_temp_term, i, term)?;
        let is_pool = flag(&self.surf_is_operating_pool, i)?;
        let divisor = 1.0
            / (value(&self.surf_ctf_inside0, i)?
                - f64::from(flag(&self.surf_is_adiabatic, i)?) * value(&self.surf_ctf_cross0, i)?
                + f64::from(is_pool) * value(&self.pool_heat_trans_coefs, i)?
                + (if is_pool == 0 { 1.0 } else { 0.0 }) * value(&self.surf_h_conv_int, i)?
                + 5.0);
        set(&mut self.surf_temp_div, i, divisor)?;
        Ok(())
    }

    /// Same scalar Source8913-8933 expression, shared without a second solve.
    pub(super) fn solve_source_inside_opaque_ctf_at(&mut self, i: usize) -> Result<(), Error> {
        let is_pool = flag(&self.surf_is_operating_pool, i)?;
        let is_source = flag(&self.surf_is_source_or_sink, i)?;
        let is_adiabatic = flag(&self.surf_is_adiabatic, i)?;
        let solved = ((if is_pool == 0 { 1.0 } else { 0.0 })
            * (value(&self.surf_temp_term, i)?
                + value(&self.surf_qdot_rad_net_lw_in_per_area, i)?)
            + f64::from(is_source)
                * value(&self.surf_ctf_source_in0, i)?
                * value(&self.surf_q_source_sink_hist, i)?
            + f64::from(is_pool) * value(&self.surf_ctf_const_in_part, i)?
            + f64::from(is_pool) * value(&self.q_pool_surf_numerator, i)?
            + 5.0 * value(&self.surf_temp_ins_old, i)?
            + (if is_adiabatic == 0 { 1.0 } else { 0.0 })
                * value(&self.surf_ctf_cross0, i)?
                * value(&self.surf_temp_out_hist, i)?)
            * value(&self.surf_temp_div, i)?;
        set(&mut self.surf_temp_in_tmp, i, solved)?;
        // Source8933 explicitly reads the just-written Tmp owner.
        let copy = value(&self.surf_temp_in_tmp, i)?;
        set(&mut self.surf_temp_in, i, copy)?;
        Ok(())
    }
}
