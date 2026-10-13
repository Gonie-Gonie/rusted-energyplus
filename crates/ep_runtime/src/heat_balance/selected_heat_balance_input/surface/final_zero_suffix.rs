//! Literal selected final-surface2494-2598 effects around centroid2591.
//! This does not complete the later lists, enclosures, fatal or Setup return.
use super::{
    Error, SourceErrorBackend, SurfaceClass, SurfaceGeometryInputState, SurfaceGetterOwners, signed,
};

fn index(value: i32) -> Result<usize, Error> {
    value
        .checked_sub(1)
        .and_then(|value| usize::try_from(value).ok())
        .ok_or(Error::Unavailable("actual positive Surface/Space index"))
}

impl SurfaceGeometryInputState {
    /// Real false-shade branches plus Zone count traversal before centroid.
    pub(crate) fn check_opaque_window_shading_and_zones(
        &mut self,
        owners: &SurfaceGetterOwners<'_>,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        let rows = self.final_surfaces.as_ref().ok_or(Error::Unavailable(
            "actual final Surface before shade/Zone checks",
        ))?;
        if !self.surface_error {
            // Actual2499-2544. Opaque raw SurfaceData constructor has false;
            // the admitted empty WindowShadingControl producer never sets it.
            for surface in rows {
                if !surface.row.has_shade_control {
                    continue;
                }
                return Err(Error::Unavailable(
                    "positive final blind material association",
                ));
            }
            // FinalAssociate10258 loops the actual stored control count, not
            // TotWindows or a typed empty list. Zero leaves its array untouched.
            if self.total_window_shading_control != 0 {
                return Err(Error::Unavailable(
                    "positive final WindowShadingControl association",
                ));
            }
            // CheckWindow10285 traverses every final row independently.
            for surface in rows {
                if surface.row.has_shade_control
                    && surface.row.window_shading_control_list.len() > 1
                {
                    return Err(Error::Unavailable("positive window-shading similarity"));
                }
            }
        }
        let zones = owners
            .zones
            .zones
            .as_ref()
            .ok_or(Error::Unavailable("same actual Zone count-check owner"))?;
        let spaces = owners
            .zones
            .spaces
            .as_ref()
            .ok_or(Error::Unavailable("same actual Space count-check owner"))?;
        for zone in zones {
            // Original2553-2555 literal local initial values: opaque, withWin,
            // internalMass. Supported class domain has no Window/IntMass rows.
            let mut counts = [0_i32; 3];
            for &space_number in &zone.space_indexes {
                let space = spaces
                    .get(index(space_number)?)
                    .ok_or(Error::Unavailable("actual Zone/Space range association"))?;
                if space.ht_surface_first == 0 {
                    continue;
                }
                for number in space.ht_surface_first..=space.ht_surface_last {
                    let surface = rows
                        .get(index(number)?)
                        .ok_or(Error::Unavailable("actual final HT surface range"))?;
                    match surface.row.class {
                        SurfaceClass::Wall | SurfaceClass::Floor | SurfaceClass::Roof => {
                            counts[0] = counts[0]
                                .checked_add(1)
                                .ok_or(Error::Unavailable("Source signed opaque count"))?;
                        }
                        _ => return Err(Error::Unavailable("nonopaque final Zone count domain")),
                    }
                }
            }
            if counts[1] == 1 && counts[0] == 1 && counts[2] == 0 {
                // Literal2582 prefix precedes the real backend operations.
                self.surface_error = true;
                backend.show_severe_error(&format!(
                    "GetSurfaceData: Zone {} has only one floor, wall or roof, and this surface has a window.",
                    zone.name,
                ))?;
                backend.show_continue_error(
                    "Add more floors, walls or roofs, or an internal mass surface.",
                )?;
            }
        }
        Ok(())
    }

    // SetupShadeSurfacesForSolarCalcs14681-14767: the complete selected empty
    // caller still queries all four families and owns its three local arrays.
    fn setup_empty_shade_surface_solar_inputs(
        &self,
        owners: &mut SurfaceGetterOwners<'_>,
    ) -> Result<(), Error> {
        let families = [
            "SolarCollector:FlatPlate:Water",
            "SolarCollector:FlatPlate:PhotovoltaicThermal",
            "Generator:Photovoltaic",
            "SolarCollector:IntegralCollectorStorage",
        ];
        let mut counts = [0_i32; 4];
        for (slot, family) in families.iter().enumerate() {
            owners.shortcut.current_module_object = (*family).into();
            counts[slot] = signed(owners.input.get_num_objects_found(family))?;
        }
        if counts.iter().any(|count| *count != 0) {
            return Err(Error::Unavailable(
                "positive solar-component surface getter",
            ));
        }
        let candidates = counts
            .iter()
            .try_fold(0_i32, |total, value| total.checked_add(*value))
            .ok_or(Error::Unavailable("Source signed solar candidate count"))?;
        let collectors = counts[0]
            .checked_add(counts[3])
            .ok_or(Error::Unavailable("Source signed solar collector count"))?;
        let candidate_names = vec![
            String::new();
            usize::try_from(candidates).map_err(|_| Error::Unavailable(
                "actual solar candidate allocation"
            ))?
        ];
        let collector_count = usize::try_from(collectors)
            .map_err(|_| Error::Unavailable("actual collector local allocation"))?;
        let _candidate_ics_surface_names = vec![String::new(); collector_count];
        let _candidate_ics_boundary_types = vec![String::new(); collector_count];
        for surface in self
            .final_surfaces
            .as_ref()
            .ok_or(Error::Unavailable("actual final Surface solar scan"))?
        {
            // FindItemInList at14745 has actual size0, so Found0; no writes to
            // ExtSolar/ExtWind/ViewFactorGround/SurfIsICS/SurfICSPtr are reached.
            if candidate_names
                .iter()
                .any(|name| name.eq_ignore_ascii_case(&surface.row.name))
            {
                return Err(Error::Unavailable(
                    "positive solar-component surface assignment",
                ));
            }
        }
        Ok(())
    }

    /// After the real centroid, consume the empty renewables/movable callers
    /// and the sole Building getter's actual CalcSolRefl predicate2597.
    pub(crate) fn read_empty_surface_solar_inputs(
        &self,
        owners: &mut SurfaceGetterOwners<'_>,
    ) -> Result<(), Error> {
        self.setup_empty_shade_surface_solar_inputs(owners)?;
        owners.shortcut.current_module_object = "SurfaceControl:MovableInsulation".into();
        if owners
            .input
            .get_num_objects_found(&owners.shortcut.current_module_object)
            != 0
        {
            return Err(Error::Unavailable("positive movable-insulation getter"));
        }
        // At zero11780-12007 has no argument reset, material mutation or list
        // assignment. Source literal constructor insulation flags stay retained.
        if owners.control.calculate_solar_reflections {
            return Err(Error::Unavailable(
                "positive shading-surface reflectance getter",
            ));
        }
        Ok(())
    }
}
