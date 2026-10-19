//! Native SolarShading8624-8667, full ordinary opaque consumer.
//! Its external-window-model parent is separately admitted; this function does
//! not replace the different CalcInteriorSolarDistribution reset/whole body.
use super::*;

impl OrdinarySolarRadiation {
    #[allow(
        dead_code,
        reason = "Retain the actual SolarShading CalcAbsorbedOnExteriorOpaqueSurfaces consumer entry even where the selected parent does not invoke that separate original child."
    )]
    pub(crate) fn source_calc_absorbed_on_exterior_opaque(
        &self,
        number_zones: i32,
        geometry: &mut SurfaceGeometryInputState,
        zones: &SelectedZoneInputOwners,
        control: &SelectedHeatBalanceInputState,
        clock: &WeatherGlobalState,
    ) -> Result<(), Error> {
        for zone_index in 0..number_zones {
            let zone = zones
                .zones
                .as_ref()
                .and_then(|rows| usize::try_from(zone_index).ok().and_then(|i| rows.get(i)))
                .ok_or(Error::Unavailable(
                    "CalcAbsorbedOnExteriorOpaqueSurfaces actual Zone row",
                ))?;
            for &space_number in &zone.space_indexes {
                let space = zones
                    .spaces
                    .as_ref()
                    .and_then(|rows| ordinal(space_number).ok().and_then(|i| rows.get(i - 1)))
                    .ok_or(Error::Unavailable(
                        "CalcAbsorbedOnExteriorOpaqueSurfaces actual Space row",
                    ))?;
                for number in space.ht_surface_first..=space.ht_surface_last {
                    let n = ordinal(number)?;
                    let surface = geometry
                        .final_surfaces
                        .as_ref()
                        .and_then(|rows| rows.get(n - 1))
                        .ok_or(Error::Unavailable(
                            "CalcAbsorbedOnExteriorOpaqueSurfaces actual Surface row",
                        ))?;
                    if !surface.row.exterior_solar
                        && surface.original_class != SurfaceClass::TddDiffuser
                    {
                        continue;
                    }
                    let windows = geometry.window_arrays.as_mut().ok_or(Error::Unavailable(
                        "CalcAbsorbedOnExteriorOpaqueSurfaces actual DataSurface arrays",
                    ))?;
                    let construction =
                        *windows
                            .surf_active_construction
                            .get(n - 1)
                            .ok_or(Error::Unavailable(
                                "CalcAbsorbedOnExteriorOpaqueSurfaces actual active Construction",
                            ))?;
                    if surface.original_class == SurfaceClass::TddDiffuser {
                        return Err(Error::Unavailable(
                            "CalcAbsorbedOnExteriorOpaqueSurfaces actual SurfWinTDDPipeNum then TDDPipe.Dome8648",
                        ));
                    }
                    let hour = ordinal(clock.hour_of_day)?;
                    let step = ordinal(clock.time_step)?;
                    let cosine = self
                        .solar_timestep
                        .heat_balance
                        .surf_cos_inc_ang
                        .read([hour, step, n])?;
                    let sunlit = self
                        .solar_timestep
                        .heat_balance
                        .surf_sunlit_frac
                        .read([hour, step, n])?;
                    if sunlit > 0.0 {
                        let construction = control
                            .construction_usage
                            .get(ordinal(construction)? - 1)
                            .ok_or(Error::Unavailable(
                                "CalcAbsorbedOnExteriorOpaqueSurfaces actual Construction row",
                            ))?;
                        if construction.trans_diff <= 0.0 {
                            windows
                                .surf_opaq_ao
                                .set([n], construction.outside_absorp_solar * cosine * sunlit)?;
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
