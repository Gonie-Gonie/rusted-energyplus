//! Actual Setup550-639 retained centroid, bounds, adjacency and count writes.
use super::{Error, SourceErrorBackend, SurfaceClass, SurfaceGeometryInputState};
use crate::geometry::{source_bound_max, source_bound_min, source_max, source_min};
use crate::heat_balance::selected_heat_balance_input::zone::SelectedZoneInputOwners;

fn index(number: i32) -> Result<usize, Error> {
    usize::try_from(number)
        .ok()
        .and_then(|number| number.checked_sub(1))
        .ok_or(Error::Unavailable(
            "positive Source centroid/bounds association",
        ))
}

fn increment(value: &mut i32) -> Result<(), Error> {
    *value = value.checked_add(1).ok_or(Error::Unavailable(
        "Source signed Zone surface count overflow",
    ))?;
    Ok(())
}

impl SurfaceGeometryInputState {
    pub(crate) fn prepare_opaque_setup_zone_bounds(
        &mut self,
        owners: &mut SelectedZoneInputOwners,
        num_of_zones: i32,
        backend: &mut SourceErrorBackend<'_>,
        errors_found: &mut bool,
    ) -> Result<(), Error> {
        let surfaces = self.final_surfaces.as_ref().ok_or(Error::Unavailable(
            "same final centroid/bounds Surface owner",
        ))?;
        let zones = owners
            .zones
            .as_mut()
            .ok_or(Error::Unavailable("same centroid/bounds Zone allocation"))?;
        let spaces = owners
            .spaces
            .as_ref()
            .ok_or(Error::Unavailable("same centroid/bounds Space allocation"))?;
        for zone_number in 1..=num_of_zones {
            let zone = zones
                .get_mut(index(zone_number)?)
                .ok_or(Error::Unavailable("actual centroid Zone association"))?;
            let (mut non_internal_mass, mut internal_mass) = (false, false);
            let mut total_surface_area = 0.0;
            let mut centroid = [0.0; 3];
            zone.centroid_m = Some(centroid); // Actual557, before first-vertex reads.
            if zone.all_surface_first > 0 {
                let first = surfaces
                    .get(index(zone.all_surface_first)?)
                    .ok_or(Error::Unavailable("actual centroid first Surface"))?;
                if first.row.sides > 0 {
                    let vertex = first
                        .row
                        .vertex
                        .as_ref()
                        .and_then(|rows| rows.first())
                        .copied()
                        .flatten()
                        .ok_or(Error::Unavailable("actual first Surface Vertex1"))?;
                    zone.bounds_m = [
                        vertex[0], vertex[0], vertex[1], vertex[1], vertex[2], vertex[2],
                    ];
                }
            }
            for &space_number in &zone.space_indexes {
                let space = spaces
                    .get(index(space_number)?)
                    .ok_or(Error::Unavailable("actual centroid Space association"))?;
                for surface_number in space.all_surface_first..=space.all_surface_last {
                    let surface = surfaces
                        .get(index(surface_number)?)
                        .ok_or(Error::Unavailable("actual centroid Space Surface range"))?;
                    let row = &surface.row;
                    if row.class == SurfaceClass::IntMass {
                        internal_mass = true;
                        continue;
                    }
                    if !row.is_air_boundary_surface {
                        non_internal_mass = true;
                    }
                    if matches!(
                        row.class,
                        SurfaceClass::Wall | SurfaceClass::Roof | SurfaceClass::Floor
                    ) {
                        let gross_area = row
                            .vertex_progress
                            .normal_area
                            .ok_or(Error::Unavailable("actual centroid Surface.GrossArea"))?
                            .gross_area_m2;
                        centroid[0] += surface.centroid[0] * gross_area;
                        centroid[1] += surface.centroid[1] * gross_area;
                        centroid[2] += surface.centroid[2] * gross_area;
                        total_surface_area += gross_area;
                        zone.centroid_m = Some(centroid);
                    }
                    // Bounds follow each row's weighted centroid writes, even
                    // for a reached nonbase/non-internal-mass class.
                    let vertices = row
                        .vertex
                        .as_ref()
                        .ok_or(Error::Unavailable("actual bounds Vertex allocation"))?;
                    if vertices.is_empty()
                        || usize::try_from(row.sides).ok() != Some(vertices.len())
                    {
                        return Err(Error::Unavailable(
                            "complete selected bounds Vertex storage",
                        ));
                    }
                    for axis in 0..3 {
                        let first = vertices[0]
                            .ok_or(Error::Unavailable("actual written bounds Vertex"))?[axis];
                        let (mut minimum, mut maximum) = (first, first);
                        for vertex in vertices {
                            let value = vertex
                                .ok_or(Error::Unavailable("actual written bounds Vertex"))?[axis];
                            minimum = source_min(minimum, value);
                            maximum = source_max(maximum, value);
                        }
                        zone.bounds_m[axis * 2] =
                            source_bound_min(zone.bounds_m[axis * 2], minimum);
                        zone.bounds_m[axis * 2 + 1] =
                            source_bound_max(zone.bounds_m[axis * 2 + 1], maximum);
                    }
                }
            }
            if total_surface_area > 0.0 {
                centroid[0] /= total_surface_area;
                centroid[1] /= total_surface_area;
                centroid[2] /= total_surface_area;
                zone.centroid_m = Some(centroid);
            }
            if internal_mass && !non_internal_mass {
                backend.show_severe_error(&format!(
                    "SetUpZoneGeometry: Zone=\"{}\" has only internal mass surfaces.  Need at least one other surface.", zone.name
                ))?;
                *errors_found = true;
            }
        }
        let total = usize::try_from(self.total_surfaces)
            .map_err(|_| Error::Unavailable("actual signed TotSurfaces for adjacency"))?;
        let adjacent = self.adjacent_zone.insert(vec![0; total]); // Actual607 dimension.
        for (number, target) in adjacent.iter_mut().enumerate() {
            let surface = surfaces
                .get(number)
                .ok_or(Error::Unavailable("actual adjacency Surface"))?;
            if surface.row.exterior_boundary <= 0 {
                continue;
            }
            *target = surfaces
                .get(index(surface.row.exterior_boundary)?)
                .ok_or(Error::Unavailable("actual adjacent Surface association"))?
                .row
                .zone;
        }
        for zone_number in 1..=num_of_zones {
            let zone = zones
                .get_mut(index(zone_number)?)
                .ok_or(Error::Unavailable("actual count Zone association"))?;
            for number in 0..total {
                let row = &surfaces
                    .get(number)
                    .ok_or(Error::Unavailable("actual count Surface association"))?
                    .row;
                // Exact name predicate precedes the numeric Zone mismatch skip.
                if !row.heat_transfer_surface && row.zone_name == zone.name {
                    increment(&mut zone.num_shading_surfaces)?;
                }
                if row.zone != zone_number {
                    continue;
                }
                if row.heat_transfer_surface
                    && matches!(
                        row.class,
                        SurfaceClass::Wall | SurfaceClass::Roof | SurfaceClass::Floor
                    )
                {
                    increment(&mut zone.num_surfaces)?;
                }
                if row.heat_transfer_surface
                    && matches!(
                        row.class,
                        SurfaceClass::Window
                            | SurfaceClass::GlassDoor
                            | SurfaceClass::Door
                            | SurfaceClass::TddDome
                            | SurfaceClass::TddDiffuser
                    )
                {
                    increment(&mut zone.num_sub_surfaces)?;
                }
            }
        }
        Ok(()) // Actual639; no641 NominalU/report body is substituted.
    }
}
