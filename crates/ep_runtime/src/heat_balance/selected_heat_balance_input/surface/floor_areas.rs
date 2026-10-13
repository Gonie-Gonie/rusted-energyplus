//! GetSurfaceData2304–2455: actual Space traversal then Zone floor adjustment.
//! These are retained Source owners, before area checks and centroid2591.
use super::{
    Error, SourceErrorBackend, SurfaceClass, SurfaceGeometryInputState, SurfaceGetterOwners,
};

fn index(value: i32) -> Result<usize, Error> {
    usize::try_from(
        value
            .checked_sub(1)
            .ok_or(Error::Unavailable("Source floor association"))?,
    )
    .map_err(|_| Error::Unavailable("Source floor association"))
}
fn real(value: f64, decimals: usize) -> Result<String, Error> {
    super::super::rounded(value, decimals)
}

impl SurfaceGeometryInputState {
    pub(crate) fn set_floor_and_roof_areas(
        &self,
        owners: &mut SurfaceGetterOwners<'_>,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        // Actual2307 leaves constructor/current fields untouched on SurfError.
        if self.surface_error {
            return Ok(());
        }
        let surfaces = self
            .final_surfaces
            .as_ref()
            .ok_or(Error::Unavailable("final Surface floor traversal"))?;
        let zones = owners
            .zones
            .zones
            .as_mut()
            .ok_or(Error::Unavailable("same Zone floor owner"))?;
        let spaces = owners
            .zones
            .spaces
            .as_mut()
            .ok_or(Error::Unavailable("same Space floor owner"))?;
        let extra = owners.globals.display_extra_warnings;
        let mut errors = 0_i32;
        for space in &mut *spaces {
            let zone = zones
                .get_mut(index(space.zone_num)?)
                .ok_or(Error::Unavailable("floor Space zoneNum"))?;
            let mut calculated = 0.0;
            for number in space.ht_surface_first..=space.ht_surface_last {
                let surface = surfaces
                    .get(index(number)?)
                    .ok_or(Error::Unavailable("Space HT floor range"))?;
                let area = surface
                    .row
                    .vertex_progress
                    .properties
                    .ok_or(Error::Unavailable("retained Surface.Area"))?
                    .area_m2;
                if surface.row.class == SurfaceClass::Floor {
                    zone.has_floor = true;
                    space.has_floor = true;
                    calculated += area;
                }
                if surface.row.class == SurfaceClass::Roof {
                    zone.ceiling_area += area;
                    zone.has_roof = true;
                }
            }
            if space.user_entered_floor_area != -99999.0 {
                if space.user_entered_floor_area > 0.0 {
                    if calculated > 0.0 {
                        let difference = (calculated - space.user_entered_floor_area).abs()
                            / space.user_entered_floor_area;
                        if difference > 0.05 {
                            errors = errors
                                .checked_add(1)
                                .ok_or(Error::Unavailable("Source signed floor warning count"))?;
                            if errors == 1 && !extra {
                                backend.show_warning_error("GetSurfaceData: Entered Space Floor Area(s) differ more than 5% from calculated Space Floor Area(s).")?;
                                backend.show_continue_error("...use Output:Diagnostics,DisplayExtraWarnings; to show more details on individual Spaces.")?;
                            }
                            if extra {
                                backend.show_warning_error(&format!("GetSurfaceData: Entered Floor Area for Space=\"{}\" is {}% different from the calculated Floor Area.",space.name,real(difference * 100.0,1)?))?;
                                backend.show_continue_error(&format!("Entered Space Floor Area={}, Calculated Space Floor Area={}, entered Floor Area will be used.",real(space.user_entered_floor_area,2)?,real(calculated,2)?))?;
                            }
                        }
                    }
                    space.floor_area = space.user_entered_floor_area;
                    space.has_floor = true;
                }
            } else {
                space.floor_area = calculated;
            }
        }
        errors = 0;
        for zone in zones {
            let mut calculated = 0.0;
            for number in &zone.space_indexes {
                let space = spaces
                    .get(index(*number)?)
                    .ok_or(Error::Unavailable("Zone floor spaceIndexes"))?;
                calculated += space.floor_area;
                zone.has_floor |= space.has_floor;
            }
            if zone.user_entered_floor_area != -99999.0 {
                if zone.user_entered_floor_area > 0.0 {
                    if calculated > 0.0 {
                        let difference = (calculated - zone.user_entered_floor_area).abs()
                            / zone.user_entered_floor_area;
                        if difference > 0.05 {
                            errors = errors.checked_add(1).ok_or(Error::Unavailable(
                                "Source signed Zone floor warning count",
                            ))?;
                            if errors == 1 && !extra {
                                backend.show_warning_error("GetSurfaceData: Entered Zone Floor Area(s) differ more than 5% from the sum of the Space Floor Area(s).")?;
                                backend.show_continue_error("...use Output:Diagnostics,DisplayExtraWarnings; to show more details on individual zones.")?;
                            }
                            if extra {
                                backend.show_warning_error(&format!("GetSurfaceData: Entered Floor Area for Zone=\"{}\" is {}% different from the sum of the Space Floor Area(s).",zone.name,real(difference * 100.0,1)?))?;
                                backend.show_continue_error(&format!(
                                    "Entered Zone Floor Area={}, Sum of Space Floor Area(s)={}",
                                    real(zone.user_entered_floor_area, 2)?,
                                    real(calculated, 2)?
                                ))?;
                                backend.show_continue_error("Entered Zone Floor Area will be used and Space Floor Area(s) will be adjusted proportionately.")?;
                            }
                        }
                    }
                    zone.floor_area = zone.user_entered_floor_area;
                    zone.has_floor = true;
                    if zone.num_spaces == 1 {
                        let number = zone
                            .space_indexes
                            .first()
                            .ok_or(Error::Unavailable("actual single Space index"))?;
                        spaces
                            .get_mut(index(*number)?)
                            .ok_or(Error::Unavailable("single Space floor assignment"))?
                            .floor_area = zone.floor_area;
                    } else if calculated > 0.0 {
                        let ratio = zone.floor_area / calculated;
                        for number in &zone.space_indexes {
                            spaces
                                .get_mut(index(*number)?)
                                .ok_or(Error::Unavailable("Space proportional floor assignment"))?
                                .floor_area *= ratio;
                        }
                    } else if extra {
                        backend.show_warning_error(&format!("GetSurfaceData: Entered Floor Area entered for Zone=\"{}\" significantly different from sum of Space Floor Areas",zone.name))?;
                        backend.show_continue_error("But the sum of the Space Floor Areas is zero and there is more than one Space in the zone.Unable to apportion the zone floor area. Space Floor Areas are zero.")?;
                    }
                } else if calculated > 0.0 {
                    zone.floor_area = calculated;
                }
            } else {
                zone.floor_area = calculated;
            }
            let mut total = 0.0;
            for number in &zone.space_indexes {
                total += spaces
                    .get(index(*number)?)
                    .ok_or(Error::Unavailable("Space floor fraction sum"))?
                    .floor_area;
            }
            if total > 0.0 {
                for number in &zone.space_indexes {
                    let space = spaces
                        .get_mut(index(*number)?)
                        .ok_or(Error::Unavailable("Space floor fraction assignment"))?;
                    space.fraction_zone_floor_area = space.floor_area / total;
                }
            }
        }
        Ok(())
    }
}
