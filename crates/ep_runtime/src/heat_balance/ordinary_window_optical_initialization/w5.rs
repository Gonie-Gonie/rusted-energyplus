//! W5InitGlassParameters: genuine opaque Construction and Surface writes.
use super::*;
impl SourceWindowOpticalInitialization {
    pub(super) fn w5_init_glass_parameters(
        &mut self,
        owners: &mut SourceWindowOpticalOwners<'_, '_>,
    ) -> Result<(), Error> {
        let max_layers = usize::try_from(owners.bounds.max_solid_window_layers)
            .map_err(|_| Error::Unavailable("W5 actual MaxSolidWinLayers"))?;
        for number in 0..owners.control.construction_usage.len() {
            let row = &mut owners.control.construction_usage[number];
            if row.from_window5_data_file {
                continue;
            } //1659
            if row.window_type_bsdf {
                continue;
            } //1662
            row.trans_diff = 0.0;
            row.trans_diff_vis = 0.0;
            row.abs_diff_back_shade = 0.0;
            row.shade_absorp_thermal = 0.0;
            row.reflect_sol_diff_back = 0.0;
            row.reflect_sol_diff_front = 0.0;
            row.reflect_vis_diff_front = 0.0;
            row.abs_beam_shade_coef.fill(0.0);
            row.trans_sol_beam_coef.fill(0.0);
            row.refl_sol_beam_front_coef.fill(0.0);
            // Authentic1676 begin()..begin() is empty: NO back-coefficient write.
            row.trans_vis_beam_coef.fill(0.0);
            row.window_arrays
                .abs_diff
                .as_mut()
                .ok_or(Error::Unavailable(
                    "W5 genuine earlier AbsDiff allocation1678",
                ))?
                .fill(Some(0.0));
            row.window_arrays
                .abs_diff_back
                .as_mut()
                .ok_or(Error::Unavailable(
                    "W5 genuine earlier AbsDiffBack allocation1679",
                ))?
                .fill(Some(0.0));
            for layer in 0..max_layers {
                for coefficient in 0..6 {
                    // Source1682-1683 explicitly targets LAST Construct every time.
                    let last = owners
                        .control
                        .total_constructions
                        .checked_sub(1)
                        .and_then(|index| owners.control.construction_usage.get_mut(index))
                        .ok_or(Error::Unavailable("W5 actual LAST Construction1682"))?;
                    last.window_arrays
                        .abs_beam_coef
                        .as_mut()
                        .and_then(|rows| rows.get_mut(layer))
                        .ok_or(Error::Unavailable("W5 LAST AbsBeamCoef allocation1682"))?
                        [coefficient] = Some(0.0);
                    last.window_arrays
                        .abs_beam_back_coef
                        .as_mut()
                        .and_then(|rows| rows.get_mut(layer))
                        .ok_or(Error::Unavailable("W5 LAST AbsBeamBackCoef allocation1683"))?
                        [coefficient] = Some(0.0);
                }
            }
        }
        let count = usize::try_from(owners.geometry.total_surfaces)
            .map_err(|_| Error::Unavailable("W5 actual TotSurfaces"))?;
        for number in 0..count {
            let surface = owners
                .geometry
                .final_surfaces
                .as_deref()
                .and_then(|rows| rows.get(number))
                .ok_or(Error::Unavailable("W5 actual Surface1691"))?;
            if surface.row.frame_divider == 0 {
                continue;
            }
            return Err(Error::Unavailable(
                "W5 positive frame/divider SurfaceWindow1696",
            ));
        }
        // DifOverrideCount1741 is actual local0; the admitted loop has no positive writer.
        for number in 0..count {
            let surface = owners
                .geometry
                .final_surfaces
                .as_deref()
                .and_then(|rows| rows.get(number))
                .ok_or(Error::Unavailable("W5 actual Surface1743"))?;
            *owners
                .geometry
                .window_arrays
                .as_mut()
                .and_then(|arrays| arrays.surf_win_solar_diffusing.get_mut(number))
                .ok_or(Error::Unavailable(
                    "W5 actual SolarDiffusing allocation1744",
                ))? = false;
            if surface.row.class != SurfaceClass::Window || surface.row.exterior_boundary != 0 {
                continue;
            }
            if *owners
                .geometry
                .window_arrays
                .as_ref()
                .and_then(|arrays| arrays.surf_win_storm_win_constr.get(number))
                .ok_or(Error::Unavailable("W5 actual StormWinConstr1745"))?
                != 0
            {
                continue;
            }
            return Err(Error::Unavailable(
                "W5 positive diffusing glazing ConstrNum1749",
            ));
        }
        Ok(())
    }
}
