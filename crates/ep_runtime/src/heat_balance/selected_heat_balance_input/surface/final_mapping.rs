//! Bounded final opaque Surface copy/order and later centroid producers.
//!
//! This is not the complete GetSurfaceData/SetupZoneGeometry return. Parallel
//! window arrays, later getter/enclosure/report closure remain separately held.
use super::{
    Error, SourceErrorBackend, SurfaceClass, SurfaceGeometryInputState, SurfaceGetterOwners,
    SurfaceTmpInput,
};
use crate::geometry::SurfaceGeometryProperties;
use ep_model::{
    OutsideBoundaryCondition, Point3, SunExposure, SurfaceId, SurfaceType, WindExposure,
};

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct FinalSurface {
    pub(crate) row: SurfaceTmpInput,
    pub(crate) original_class: SurfaceClass,
    // SurfaceData constructor explicitly initializes Centroid(0,0,0).
    pub(crate) centroid: [f64; 3],
    pub(crate) typed_id: Option<SurfaceId>,
}

fn index(value: i32) -> Result<usize, Error> {
    usize::try_from(
        value
            .checked_sub(1)
            .ok_or(Error::Unavailable("positive Source surface association"))?,
    )
    .map_err(|_| Error::Unavailable("positive Source surface association"))
}
fn vertices(row: &SurfaceTmpInput) -> Result<Vec<Point3>, Error> {
    row.vertex
        .as_ref()
        .ok_or(Error::Unavailable("actual final Vertex allocation"))?
        .iter()
        .map(|v| {
            let [x_m, y_m, z_m] = v.ok_or(Error::Unavailable("actual final written Vertex"))?;
            Ok(Point3 { x_m, y_m, z_m })
        })
        .collect()
}

impl SurfaceGeometryInputState {
    /// Actual selected CreateMissingSpaces and final class-copy/remapping.
    /// No output or full-getter completion flag is fabricated by this boundary.
    pub(crate) fn copy_final_opaque_surfaces(
        &mut self,
        owners: &mut SurfaceGetterOwners<'_>,
        backend: &mut SourceErrorBackend<'_>,
        errors: &mut bool,
    ) -> Result<(), Error> {
        if *errors {
            // Original1215, before actual Surface allocation.
            return match backend
                .show_fatal_error("GetSurfaceData: Errors discovered, program terminates.")
            {
                Err(error) => Err(Error::Fatal(error)),
                Ok(never) => match never {},
            };
        }
        if self.pending_vertices.is_some()
            || self.surfaces_read != self.next_object
            || super::signed(self.next_object)? != self.total_surfaces
        {
            return Err(Error::Unavailable(
                "complete actual opaque GetVertices rows before final copy",
            ));
        }
        let total = usize::try_from(self.total_surfaces)
            .map_err(|_| Error::Unavailable("actual final Surface count"))?;
        self.final_surfaces = Some(vec![FinalSurface::default(); total]); // selected Surface members,1219
        self.class_moved = vec![false; total]; // actual1439-1440
        self.moved_surfaces = 0;
        self.all_surface_list_report_order.reserve(total);
        let tmp = self
            .surface_tmp
            .as_mut()
            .ok_or(Error::Unavailable("actual SurfaceTmp before final copy"))?;
        let zones = owners
            .zones
            .zones
            .as_ref()
            .ok_or(Error::Unavailable("actual Zone owner before final copy"))?;
        let spaces = owners
            .zones
            .spaces
            .as_ref()
            .ok_or(Error::Unavailable("actual Space owner before final copy"))?;
        let mut without_space = vec![false; zones.len()];
        let mut with_space = vec![false; zones.len()];
        for row in &*tmp {
            // CreateMissingSpaces2928-2946, before any assignment.
            if !row.heat_transfer_surface {
                continue;
            }
            let zone = index(row.zone)?;
            if row.space == 0 {
                *without_space
                    .get_mut(zone)
                    .ok_or(Error::Unavailable("raw Zone association"))? = true;
            } else {
                *with_space
                    .get_mut(zone)
                    .ok_or(Error::Unavailable("raw Zone association"))? = true;
            }
        }
        if without_space.iter().zip(&with_space).any(|(a, b)| *a && *b) {
            return Err(Error::Unavailable(
                "actual positive remainder-Space constructor",
            ));
        }
        for row in &mut *tmp {
            // Actual2997-3002; last real Space, never Zone alias.
            if !row.heat_transfer_surface || row.space != 0 {
                continue;
            }
            let zone = zones
                .get(index(row.zone)?)
                .ok_or(Error::Unavailable("actual missing-Space Zone"))?;
            let last = zone
                .space_indexes
                .last()
                .copied()
                .ok_or(Error::Unavailable("actual implicit Space identity"))?;
            if spaces
                .get(index(last)?)
                .is_none_or(|space| space.zone_num != row.zone)
            {
                return Err(Error::Unavailable("actual implicit Space/Zone association"));
            }
            row.space = last;
        }
        self.old_to_new = vec![-1; total]; // retained actual local mapping1448-1449
        for zone in zones {
            for &space in &zone.space_indexes {
                for class in [SurfaceClass::Wall, SurfaceClass::Floor, SurfaceClass::Roof] {
                    for (old, row) in tmp.iter().enumerate() {
                        if self.class_moved[old]
                            || row.zone == 0
                            || row.space != space
                            || row.class != class
                        {
                            continue;
                        }
                        let new = self.moved_surfaces;
                        self.moved_surfaces += 1;
                        self.final_surfaces
                            .as_mut()
                            .ok_or(Error::Unavailable("final Surface allocation"))?[new]
                            .row = row.clone();
                        self.old_to_new[old] = super::signed(new + 1)?;
                        self.class_moved[old] = true;
                        self.all_surface_list_report_order
                            .push(super::signed(old + 1)?);
                    }
                }
            }
        }
        if self.moved_surfaces != total || self.class_moved.iter().any(|v| !v) {
            return Err(Error::Unavailable(
                "actual opaque Surface class-copy bijection",
            ));
        }
        let final_rows = self
            .final_surfaces
            .as_mut()
            .ok_or(Error::Unavailable("actual sorted Surface"))?;
        for (i, surface) in final_rows.iter_mut().enumerate() {
            // Actual1710-1732
            if surface.row.base_surface > 0 {
                surface.row.base_surface =
                    *self
                        .old_to_new
                        .get(index(surface.row.base_surface)?)
                        .ok_or(Error::Unavailable("actual BaseSurf old-to-new mapping"))?;
                if surface.row.base_surface < 1 {
                    return Err(Error::Unavailable("missing actual BaseSurf mapping"));
                }
            }
            let old = self.all_surface_list_report_order[i];
            if old > 0 {
                self.all_surface_list_report_order[i] = self.old_to_new[index(old)?];
            }
        }
        self.surface_tmp = None; // Actual1734, after all final copies/remaps.
        let spaces = owners
            .zones
            .spaces
            .as_mut()
            .ok_or(Error::Unavailable("actual Space surface-list owner"))?;
        for (i, surface) in final_rows.iter().enumerate() {
            // createSpaceSurfaceLists3006-3023
            if surface.row.space > 0 {
                spaces
                    .get_mut(index(surface.row.space)?)
                    .ok_or(Error::Unavailable("actual final Space association"))?
                    .surfaces
                    .push(super::signed(i + 1)?);
            }
        }
        for space in spaces.iter() {
            if space.surfaces.is_empty() {
                backend.show_warning_error(&format!(
                    "createSpaceSurfaceLists: Space={} has no surfaces.",
                    space.name
                ))?;
            }
        }
        for (i, surface) in final_rows.iter_mut().enumerate() {
            // selected self-Adiabatic reconciliation1785-1794
            if surface.row.exterior_boundary == -999 {
                if surface.row.exterior_boundary_name != surface.row.name {
                    return Err(Error::Unavailable(
                        "actual non-self interzone reconciliation",
                    ));
                }
                surface.row.exterior_boundary = super::signed(i + 1)?;
            }
        }
        for zone in owners
            .zones
            .zones
            .as_mut()
            .ok_or(Error::Unavailable("actual Zone ranges"))?
        {
            for &space_num in &zone.space_indexes {
                let space = spaces
                    .get_mut(index(space_num)?)
                    .ok_or(Error::Unavailable("actual Space ranges"))?;
                for &number in &space.surfaces {
                    if space.all_surface_first == 0 {
                        space.all_surface_first = number;
                    }
                    space.all_surface_last = number;
                    if space.ht_surface_first == 0 {
                        space.ht_surface_first = number;
                        space.opaque_or_intmass_first = number;
                        space.opaque_or_window_first = number;
                    }
                    space.ht_surface_last = number;
                    space.opaque_or_intmass_last = number;
                    space.opaque_or_window_last = number;
                }
                zone.all_surface_last = space.all_surface_last;
            }
            let first = zone
                .space_indexes
                .first()
                .copied()
                .ok_or(Error::Unavailable("actual first Space"))?;
            zone.all_surface_first = spaces
                .get(index(first)?)
                .ok_or(Error::Unavailable("actual first Space range"))?
                .all_surface_first;
        }
        Ok(())
    }

    /// Actual selected area diagnostics and OriginalClass write2458-2492.
    /// SurfError stays distinct from ErrorsFound until the unported final fatal.
    pub(crate) fn check_final_surface_areas(
        &mut self,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        let rows = self
            .final_surfaces
            .as_mut()
            .ok_or(Error::Unavailable("final Surface before area checks"))?;
        for surface in rows.iter() {
            let area = surface
                .row
                .vertex_progress
                .normal_area
                .ok_or(Error::Unavailable("actual stored Surface area"))?
                .gross_area_m2;
            if area < 1.0e-6 {
                backend.show_severe_error(&format!(
                    "GetSurfaceData: Zero or negative surface area[{}], Surface={}",
                    super::super::rounded(area, 5)?,
                    surface.row.name
                ))?;
                self.surface_error = true;
            }
            if (1.0e-6..0.001).contains(&area) {
                backend.show_warning_error(&format!(
                    "GetSurfaceData: Very small surface area[{}], Surface={}",
                    super::super::rounded(area, 5)?,
                    surface.row.name
                ))?;
            }
        }
        for surface in rows {
            surface.original_class = surface.row.class;
        }
        Ok(())
    }

    /// Later CalcSurfaceCentroid2591, using each retained final vertex/area.
    /// This bounded producer does not imply the intervening or later whole
    /// GetSurfaceData/SetupZoneGeometry owner/report lifecycle has been ported.
    pub(crate) fn calculate_final_centroids(
        &mut self,
        backend: &mut SourceErrorBackend<'_>,
        environment: &crate::weather::day::WeatherEnvironmentState,
    ) -> Result<(), Error> {
        let rows = self
            .final_surfaces
            .as_mut()
            .ok_or(Error::Unavailable("actual final Surface before centroid"))?;
        let mut negative = 0_i32;
        for surface in rows {
            let points = vertices(&surface.row)?;
            let quantities = surface
                .row
                .vertex_progress
                .properties
                .ok_or(Error::Unavailable("actual GetVertices quantity return"))?;
            if points.len() == 4 && quantities.gross_area_m2 <= 0.0 {
                self.centroid_scratch.triangle1 =
                    [Some(points[0]), Some(points[1]), Some(points[2])];
                self.centroid_scratch.triangle2 =
                    [Some(points[0]), Some(points[2]), Some(points[3])];
                backend.show_warning_error(&format!(
                    "CalcSurfaceCentroid: zero area surface, for surface={}",
                    surface.row.name
                ))?;
                continue; // actual14559 retains that Surface constructor centroid.
            }
            let centroid = crate::geometry::source_surface_centroid(
                &points,
                quantities.gross_area_m2,
                &mut self.centroid_scratch,
            );
            surface.centroid = [centroid.x_m, centroid.y_m, centroid.z_m]; // actual14635, before warning suffix.
            if centroid.z_m < 0.0
                && (surface.row.exterior_wind || surface.row.exterior_boundary == 0)
            {
                negative = negative
                    .checked_add(1)
                    .ok_or(Error::Unavailable("Original signed negative-Z count"))?;
            }
        }
        if negative > 0 {
            backend.show_warning_error(&format!(
                "CalcSurfaceCentroid: {negative} Surfaces have the Z coordinate < 0."
            ))?;
            backend.show_continue_error(
                "...in any calculations, Wind Speed will be 0.0 for these surfaces.",
            )?;
            backend.show_continue_error(&format!("...in any calculations, Outside temperatures will be the outside temperature + {} for these surfaces.",
                super::super::rounded(environment.outdoor_environment_parameters.weather_file_temperature_modifier_c,3)?))?;
            backend.show_continue_error(
                "...that is, these surfaces will have conditions as though at ground level.",
            )?;
        }
        Ok(())
    }

    /// Validate every physical ID against the actual final name/class/owners
    /// and full transformed vertex bits. The compiler eager projection is only
    /// an independent typed association; it is not the producer of this result.
    pub(crate) fn bind_typed_surfaces(
        &mut self,
        owners: &SurfaceGetterOwners<'_>,
    ) -> Result<(), Error> {
        if self.surface_error {
            return Err(Error::Unavailable(
                "retained SurfError requires the later complete GetSurfaceData fatal closure",
            ));
        }
        let rows = self
            .final_surfaces
            .as_mut()
            .ok_or(Error::Unavailable("final Surface before typed binding"))?;
        if rows.len() != owners.model.surfaces.len() {
            return Err(Error::Unavailable("actual final/typed Surface count"));
        }
        let mut ids = std::collections::BTreeSet::new();
        for surface in rows {
            let row = &surface.row;
            let mut matches = owners
                .model
                .surfaces
                .iter()
                .filter(|s| s.name.0.eq_ignore_ascii_case(&row.name));
            let typed = matches
                .next()
                .ok_or(Error::Unavailable("actual final Surface name identity"))?;
            if matches.next().is_some() || !ids.insert(typed.id) {
                return Err(Error::Unavailable(
                    "actual typed Surface identity bijection",
                ));
            }
            let class = match typed.surface_type {
                SurfaceType::Wall => SurfaceClass::Wall,
                SurfaceType::Floor => SurfaceClass::Floor,
                SurfaceType::Roof | SurfaceType::Ceiling => SurfaceClass::Roof,
            };
            let zone = owners
                .zones
                .zones
                .as_ref()
                .and_then(|z| z.get(index(row.zone).ok()?))
                .ok_or(Error::Unavailable("actual final Zone association"))?;
            let space = owners
                .zones
                .spaces
                .as_ref()
                .and_then(|s| s.get(index(row.space).ok()?))
                .ok_or(Error::Unavailable("actual final Space association"))?;
            let construction = owners
                .control
                .construction_order
                .get(index(row.construction)?)
                .ok_or(Error::Unavailable("actual final Construction association"))?;
            let boundary = if row.exterior_boundary == 0 {
                OutsideBoundaryCondition::Outdoors
            } else if row.exterior_boundary == row.base_surface {
                OutsideBoundaryCondition::Adiabatic
            } else {
                return Err(Error::Unavailable("actual supported final boundary"));
            };
            if class != row.class
                || zone.typed_zone_id != Some(typed.zone)
                || space.typed_space_id != Some(typed.space)
                || space.zone_num != row.zone
                || *construction != typed.construction
                || boundary != typed.outside_boundary_condition
                || row.exterior_solar
                    != (typed.sun_exposure == SunExposure::SunExposed
                        && typed.outside_boundary_condition == OutsideBoundaryCondition::Outdoors)
                || row.exterior_wind != (typed.wind_exposure == WindExposure::WindExposed)
            {
                return Err(Error::Unavailable(
                    "actual Surface name/class/Zone/Space/Construction/boundary identity",
                ));
            }
            let points = vertices(row)?;
            if points.len() != typed.vertices.len()
                || points.iter().zip(&typed.vertices).any(|(a, b)| {
                    [a.x_m, a.y_m, a.z_m].map(f64::to_bits)
                        != [b.x_m, b.y_m, b.z_m].map(f64::to_bits)
                })
            {
                return Err(Error::Unavailable(
                    "actual raw-to-World vertex bits differ from typed physical geometry",
                ));
            }
            if surface.centroid.iter().any(|v| !v.is_finite()) {
                return Err(Error::Unavailable(
                    "nonfinite actual centroid cannot initialize physical surface",
                ));
            }
            surface.typed_id = Some(typed.id);
        }
        Ok(())
    }

    pub(crate) fn required_properties(
        &self,
        id: SurfaceId,
    ) -> Result<SurfaceGeometryProperties, Error> {
        let surface = self
            .final_surfaces
            .as_ref()
            .and_then(|rows| rows.iter().find(|row| row.typed_id == Some(id)))
            .ok_or(Error::Unavailable(
                "missing required retained selected World Surface return",
            ))?;
        let quantities = surface
            .row
            .vertex_progress
            .properties
            .ok_or(Error::Unavailable(
                "missing retained selected World quantities",
            ))?;
        let [x_m, y_m, z_m] = surface.centroid;
        Ok(quantities.with_centroid(Point3 { x_m, y_m, z_m }))
    }
}
