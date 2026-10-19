//! Actual opaque ComputeIntThermalAbsorpFactors / ComputeIntSWAbsorpFactors.
use super::{Error, OrdinarySolarRadiation, RadiantExchangeSource, Unavailable};
use super::{backend_error, index, value};
use crate::heat_balance::error_backend::SourceErrorBackend;

// DataSurfaces.hh224-241: these are Source enum identities, not typed IDs.
const SWITCHABLE_GLAZING: i32 = 3;
const INTERIOR_SHADE: i32 = 2;
const INTERIOR_BLIND: i32 = 6;
const SUSPENDED_DIVIDER: i32 = 1;

impl OrdinarySolarRadiation {
    pub(crate) fn compute_source_thermal_absorption(
        &mut self,
        source: &RadiantExchangeSource<'_>,
        inside_thermal_absorptance: &mut [f64],
        any_movable_slat: bool,
    ) -> Result<(), Error> {
        for enclosure in &self.radiant {
            if !enclosure.rad_recalc {
                continue;
            }
            for &space_number in &enclosure.space_nums {
                let spaces = source
                    .zones
                    .spaces
                    .as_ref()
                    .ok_or(Unavailable::StorageDomain)?;
                let space = spaces
                    .get(index(space_number)?)
                    .ok_or(Unavailable::StorageDomain)?;
                for surface_number in space.window_first..=space.window_last {
                    let windows = source
                        .geometry
                        .window_arrays
                        .as_ref()
                        .ok_or(Unavailable::StorageDomain)?;
                    let at = index(surface_number)?;
                    let shade = *windows
                        .surf_win_shading_flag
                        .get(at)
                        .ok_or(Unavailable::StorageDomain)?;
                    if shade == INTERIOR_SHADE || shade == INTERIOR_BLIND {
                        return Err(Unavailable::InputBranch.into());
                    }
                    let construction_number = *windows
                        .surf_active_construction
                        .get(at)
                        .ok_or(Unavailable::StorageDomain)?;
                    let construction = source
                        .constructions
                        .construction_usage
                        .get(index(construction_number)?)
                        .ok_or(Unavailable::StorageDomain)?;
                    *inside_thermal_absorptance
                        .get_mut(at)
                        .ok_or(Unavailable::StorageDomain)? = construction.inside_absorp_thermal;
                }
            }
        }
        if any_movable_slat {
            return Err(Unavailable::InputBranch.into());
        }
        for enclosure in &mut self.radiant {
            if !enclosure.rad_recalc {
                continue;
            }
            let mut sum = 0.0;
            for &surface_number in enclosure.source_surface_ptr.as_deref().unwrap_or_default() {
                let surfaces = source
                    .geometry
                    .final_surfaces
                    .as_ref()
                    .ok_or(Unavailable::StorageDomain)?;
                let windows = source
                    .geometry
                    .window_arrays
                    .as_ref()
                    .ok_or(Unavailable::StorageDomain)?;
                let at = index(surface_number)?;
                let surface = surfaces.get(at).ok_or(Unavailable::StorageDomain)?;
                let construction = source
                    .constructions
                    .construction_usage
                    .get(index(surface.row.construction)?)
                    .ok_or(Unavailable::StorageDomain)?;
                let shade = *windows
                    .surf_win_shading_flag
                    .get(at)
                    .ok_or(Unavailable::StorageDomain)?;
                if shade == SWITCHABLE_GLAZING {
                    return Err(Unavailable::InputBranch.into());
                }
                let area = surface.row.area;
                sum += area * value(inside_thermal_absorptance, surface_number)?;
                let frame_area = *windows
                    .surf_win_frame_area
                    .get(at)
                    .ok_or(Unavailable::StorageDomain)?;
                if frame_area > 0.0 {
                    sum += frame_area
                        * (1.0
                            + 0.5
                                * windows
                                    .surf_win_proj_corr_fr_in
                                    .get(at)
                                    .copied()
                                    .ok_or(Unavailable::StorageDomain)?)
                        * windows
                            .surf_win_frame_emis
                            .get(at)
                            .copied()
                            .ok_or(Unavailable::StorageDomain)?;
                }
                let divider_area = *windows
                    .surf_win_divider_area
                    .get(at)
                    .ok_or(Unavailable::StorageDomain)?;
                if divider_area > 0.0 {
                    let mut divider_absorption = *windows
                        .surf_win_divider_emis
                        .get(at)
                        .ok_or(Unavailable::StorageDomain)?;
                    if *windows
                        .surf_win_divider_type
                        .get(at)
                        .ok_or(Unavailable::StorageDomain)?
                        == SUSPENDED_DIVIDER
                    {
                        divider_absorption = construction.inside_absorp_thermal;
                    }
                    if (shade == INTERIOR_SHADE || shade == INTERIOR_BLIND)
                        && *windows
                            .surf_win_has_shade_or_blind_layer
                            .get(at)
                            .ok_or(Unavailable::StorageDomain)?
                    {
                        return Err(Unavailable::InputBranch.into());
                    }
                    sum += divider_area
                        * (1.0
                            + windows
                                .surf_win_proj_corr_div_in
                                .get(at)
                                .copied()
                                .ok_or(Unavailable::StorageDomain)?)
                        * divider_absorption;
                }
            }
            // Original has no Smallest/clamp/nonfinite replacement here.
            enclosure.rad_therm_abs_mult = 1.0 / sum;
        }
        Ok(())
    }

    pub(crate) fn compute_source_shortwave_absorption(
        &mut self,
        source: &RadiantExchangeSource<'_>,
        inside_solar_absorptance: &[f64],
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        // Source iterates the allocated EnclSolInfo owner, not only its separate count.
        let Some(solar) = self.solar.as_mut() else {
            return Ok(());
        };
        for enclosure in solar {
            if !enclosure.rad_recalc {
                continue;
            }
            let mut sum = 0.0;
            for &surface_number in enclosure.source_surface_ptr.as_deref().unwrap_or_default() {
                let surfaces = source
                    .geometry
                    .final_surfaces
                    .as_ref()
                    .ok_or(Unavailable::StorageDomain)?;
                let windows = source
                    .geometry
                    .window_arrays
                    .as_ref()
                    .ok_or(Unavailable::StorageDomain)?;
                let at = index(surface_number)?;
                let surface = surfaces.get(at).ok_or(Unavailable::StorageDomain)?;
                let construction_number = *windows
                    .surf_active_construction
                    .get(at)
                    .ok_or(Unavailable::StorageDomain)?;
                let construction = source
                    .constructions
                    .construction_usage
                    .get(index(construction_number)?)
                    .ok_or(Unavailable::StorageDomain)?;
                if construction.trans_diff <= 0.0 {
                    let absorption = value(inside_solar_absorptance, surface_number)?;
                    let area = surface.row.area;
                    sum += area * absorption;
                } else {
                    // Actual positive window route follows; no opaque substitution.
                    return Err(Unavailable::InputBranch.into());
                }
            }
            if sum > 0.01 {
                enclosure.sol_vmult = 1.0 / sum;
            } else {
                if enclosure.sol_abs_first_calc {
                    backend.show_warning_error(&format!(
                        "ComputeIntSWAbsorbFactors: Sum of area times inside solar absorption for all surfaces is zero in Enclosure: {}",
                        enclosure.name,
                    )).map_err(backend_error)?;
                    enclosure.sol_abs_first_calc = false;
                }
                enclosure.sol_vmult = 0.0;
            }
        }
        Ok(())
    }
}
