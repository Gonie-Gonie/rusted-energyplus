//! Actual548 selected closed-zone call; return precedes550 centroid/bounds.
use super::{Error, SelectedInputProcessor, SourceErrorBackend, SurfaceGeometryInputState};
use crate::geometry::{
    InitializedZoneGeometry, ZoneGeometryProperties, bounded_topology_rejection, initial_enclosure,
    polyhedron_volume, surface_orientation, wall_heights_same,
};
use crate::heat_balance::selected_heat_balance_input::zone::SelectedZoneInputOwners;

fn index(number: i32) -> Result<usize, Error> {
    usize::try_from(number)
        .ok()
        .and_then(|n| n.checked_sub(1))
        .ok_or(Error::Unavailable("positive Source volume association"))
}

impl SurfaceGeometryInputState {
    pub(crate) fn calculate_opaque_setup_volumes(
        &mut self,
        input: &SelectedInputProcessor,
        owners: &mut SelectedZoneInputOwners,
        display_extra_warnings: bool,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<Vec<InitializedZoneGeometry>, Error> {
        let show_zone_surfaces = input
            .get_num_sections_found("SHOWZONESURFACES_DEBUG")
            .map_err(|_| Error::Unavailable("actual SHOWZONESURFACES section query"))?
            > 0;
        let mut initmsg = true;
        let zones = owners
            .zones
            .as_mut()
            .ok_or(Error::Unavailable("same Source volume Zone owner"))?;
        let spaces = owners
            .spaces
            .as_mut()
            .ok_or(Error::Unavailable("same Source volume Space owner"))?;
        let mut returned = Vec::new();
        for (zone_index, zone) in zones.iter_mut().enumerate() {
            if !zone.has_floor {
                backend.show_warning_error(&format!("No floor exists in Zone=\"{}\", zone floor area is zero. All values for this zone that are entered per floor area will be zero.", zone.name))?;
            }
            let zone_number = super::signed(zone_index + 1)?;
            let (faces, sum_areas) = self.copy_source_zone_volume_faces(
                zone_number,
                zone.all_surface_first,
                zone.all_surface_last,
            )?;
            // Genuine selected pure calls, in their Original12087/91/94 order.
            let orientation = surface_orientation(&faces);
            let same_height = wall_heights_same(&faces);
            let mut diagnostics = initial_enclosure(&faces);
            (
                diagnostics.floor_horizontal,
                diagnostics.roof_horizontal,
                diagnostics.walls_vertical,
            ) = orientation;
            diagnostics.same_wall_height = same_height;
            diagnostics.sum_face_areas_m2 = sum_areas;
            let floor_for_volume = if zone.floor_area > 0.0 {
                zone.floor_area
            } else {
                zone.geometric_floor_area
            };
            let _ceiling_for_volume = if zone.ceiling_area > 0.0 {
                zone.ceiling_area
            } else {
                zone.geometric_ceiling_area
            };
            if !diagnostics.initially_closed {
                return Err(Error::Unavailable(
                    "Source volume polygon-repair/open-zone branch",
                ));
            }
            let mut numeric = ZoneGeometryProperties {
                volume_m3: zone.volume,
                ceiling_height_m: zone.ceiling_height,
                ceiling_height_entered: zone.ceiling_height_entered,
                floor_area_m2: zone.floor_area,
                user_entered_floor_area_m2: zone.user_entered_floor_area,
                geometric_floor_area_m2: zone.geometric_floor_area,
                ceiling_area_m2: zone.ceiling_area,
                geometric_ceiling_area_m2: zone.geometric_ceiling_area,
                has_floor: zone.has_floor,
                has_roof: zone.has_roof,
                faces,
                ..Default::default()
            };
            diagnostics.signed_polyhedron_volume_m3 =
                polyhedron_volume(&numeric.faces, numeric.p0_m);
            diagnostics.topology_rejection =
                bounded_topology_rejection(&numeric.faces, &diagnostics);
            if diagnostics.topology_rejection.is_some() {
                // Six faces and consistent winding/positive signed volume are
                // conservative Rust admission, not extra Native enclosure tests.
                return Err(Error::Unavailable(
                    "selected initially-closed six-quad volume domain",
                ));
            }
            let calculated = diagnostics
                .signed_polyhedron_volume_m3
                .ok_or(Error::Unavailable("finite actual closed polyhedron volume"))?;
            if zone.volume > 0.0 {
                if calculated > 0.0 && (calculated - zone.volume).abs() / zone.volume > 0.05 {
                    diagnostics.volume_differs_by_more_than_five_percent = true;
                    self.err_count5 = self
                        .err_count5
                        .checked_add(1)
                        .ok_or(Error::Unavailable("Source signed ErrCount5 overflow"))?;
                    if self.err_count5 == 1 && !display_extra_warnings {
                        if initmsg {
                            backend.show_message("Note that the following warning(s) may/will occur if you have not enclosed your zone completely.")?;
                            initmsg = false;
                        }
                        backend.show_warning_error(
                            "Entered Zone Volumes differ from calculated zone volume(s).",
                        )?;
                        backend.show_continue_error("...use Output:Diagnostics,DisplayExtraWarnings; to show more details on individual zones.")?;
                    }
                    if display_extra_warnings {
                        if initmsg {
                            backend.show_message("Note that the following warning(s) may/will occur if you have not enclosed your zone completely.")?;
                            initmsg = false;
                        }
                        backend.show_warning_error(&format!("Entered Volume entered for Zone=\"{}\" significantly different from calculated Volume",zone.name))?;
                        let entered = super::super::rounded(zone.volume, 2)?;
                        let calculated_text = super::super::rounded(calculated, 2)?;
                        backend.show_continue_error(&format!("Entered Zone Volume value={entered}, Calculated Zone Volume value={calculated_text}, entered volume will be used in calculations."))?;
                    }
                }
            } else if zone.ceiling_height_entered && floor_for_volume > 0.0 {
                zone.volume = floor_for_volume * zone.ceiling_height;
            } else {
                zone.volume = calculated;
            }
            numeric.volume_m3 = zone.volume;
            numeric.diagnostics = diagnostics;
            if zone.volume <= 0.0 || !zone.volume.is_finite() {
                return Err(Error::Unavailable(
                    "Source nonpositive10m3/nonfinite volume fallback",
                ));
            }
            for &number in &zone.space_indexes {
                let space = spaces
                    .get_mut(index(number)?)
                    .ok_or(Error::Unavailable("actual volume Space association"))?;
                if space.zone_num != zone_number {
                    return Err(Error::Unavailable("same volume Space Zone association"));
                }
                if space.volume > 0.0 {
                    continue;
                }
                if zone.num_spaces == 1 {
                    space.volume = zone.volume;
                } else if zone.geometric_floor_area > 0.0 {
                    space.volume = zone.volume * space.floor_area / zone.geometric_floor_area;
                }
            }
            let mut total_space_volume = 0.0;
            for &number in &zone.space_indexes {
                total_space_volume += spaces
                    .get(index(number)?)
                    .ok_or(Error::Unavailable("actual Space volume sum"))?
                    .volume;
            }
            if total_space_volume > 0.0 {
                for &number in &zone.space_indexes {
                    let space = spaces
                        .get_mut(index(number)?)
                        .ok_or(Error::Unavailable("actual Space volume fraction"))?;
                    space.fraction_zone_volume = space.volume / total_space_volume;
                }
            }
            if show_zone_surfaces {
                // First debug print follows all preceding writes. No debug
                // header flag is cleared and no successful sink is invented.
                return Err(Error::Unavailable(
                    "positive SHOWZONESURFACES debug OutputFile",
                ));
            }
            numeric.volume_calculation_count += 1;
            returned.push(InitializedZoneGeometry {
                zone_id: zone
                    .typed_zone_id
                    .ok_or(Error::Unavailable("actual bound volume ZoneId"))?,
                returned: numeric,
            });
        }
        // Every admitted zone was initially enclosed; the12311 summary branch
        // is genuinely false. Faces remain in the existing Rust numeric return;
        // this projection does not claim Native local-array deallocation parity.
        Ok(returned)
    }
}
