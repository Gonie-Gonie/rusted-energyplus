//! Geometry summary and polygon helper functions.

pub(crate) mod centroid_precision;
mod inside_convection_owners;
pub mod production_trace;
mod source_geometry;
mod zone_volume;
pub mod zone_volume_trace;
pub use inside_convection_owners::{
    GeometryOwnerUnavailable, InitializedGeometryOwners, InitializedZoneGeometry,
    OpaqueGeometryOwners, OpaqueSurfaceDimensions, OpaqueZoneGeometry, opaque_geometry_owners,
    opaque_surface_dimensions,
};
pub(crate) use inside_convection_owners::{
    source_bound_max, source_bound_min, source_max, source_min,
};
pub(crate) use source_geometry::{
    GetVerticesProperties, Orientation, SourceCentroidScratch, SourceNormalArea,
    SourceProcessShape, area_vector as source_newell_area_vector, finish_get_vertices_properties,
    magnitude as source_vector_length, orientation_from_normal, source_normal_area,
    source_process_coplanarity, source_process_distance, source_process_is_rectangle,
    source_process_shift, source_process_sky_rad_split, source_process_translation,
    source_surface_centroid,
};
pub use source_geometry::{
    SurfaceGeometryError, SurfaceGeometryProperties, source_triangle_centroid,
    surface_geometry_properties,
};
pub use zone_volume::{
    ZONE_GEOMETRY_AUTO_CALCULATE, ZoneGeometryProperties, ZoneVolumeDiagnostics, ZoneVolumeEdge,
    ZoneVolumeError, ZoneVolumeFace, calculate_zone_volume, prepare_zone_height,
    zone_geometry_input_value, zone_geometry_properties,
};
// Retain the narrow owner reexport used by gated scientific bridges or the explicit Native owner API.
#[allow(unused_imports)]
pub(crate) use zone_volume::{
    bounded_topology_rejection, calculated_zone_height, initial_enclosure, polyhedron_volume,
    surface_orientation, wall_heights_same,
};
/// Actual original product precision observed for the bounded centroid owner.
pub const SOURCE_CENTROID_PRODUCT_PRECISION_BITS: u32 =
    centroid_precision::SOURCE_PRODUCT_PRECISION_BITS;
/// Binary64 third promoted by the original long-double centroid expression.
pub const SOURCE_CENTROID_THIRD_BITS: u64 = centroid_precision::SOURCE_THIRD_BITS;

use crate::first_zone::{SurfaceGeometrySummary, ZoneGeometrySummary};
use ep_model::{AutoOrNumber, OutsideBoundaryCondition, Point3, SurfaceType, TypedModel, Zone};

/// Builds per-zone geometry summaries from the typed model.
#[must_use]
pub fn zone_geometry_summaries(model: &TypedModel) -> Vec<ZoneGeometrySummary> {
    model
        .zones
        .iter()
        .map(|zone| ZoneGeometrySummary {
            zone_id: zone.id,
            zone_name: zone.name.0.clone(),
            surface_count: model
                .surfaces
                .iter()
                .filter(|surface| surface.zone == zone.id)
                .count(),
            floor_area_m2: zone_floor_area_m2(model, zone),
            volume_m3: zone_volume_m3(model, zone),
            exterior_wall_area_m2: exterior_wall_area_m2(model, zone),
        })
        .collect()
}

/// Builds per-surface geometry summaries from the typed model.
#[must_use]
pub fn surface_geometry_summaries(model: &TypedModel) -> Vec<SurfaceGeometrySummary> {
    model
        .surfaces
        .iter()
        .map(|surface| {
            let zone_name = model
                .zones
                .iter()
                .find(|zone| zone.id == surface.zone)
                .map(|zone| zone.name.0.clone())
                .unwrap_or_else(|| "UNKNOWN".to_string());

            SurfaceGeometrySummary {
                surface_id: surface.id,
                surface_name: surface.name.0.clone(),
                zone_name,
                surface_type: surface.surface_type,
                area_m2: surface_area_m2(&surface.vertices),
                azimuth_deg: surface_azimuth_deg(&surface.vertices),
                tilt_deg: surface_tilt_deg(surface.surface_type, &surface.vertices),
            }
        })
        .collect()
}

pub(crate) fn zone_floor_area_m2(model: &TypedModel, zone: &Zone) -> f64 {
    if let AutoOrNumber::Value(floor_area_m2) = zone.floor_area
        && floor_area_m2 > 0.0
    {
        return floor_area_m2;
    }

    model
        .surfaces
        .iter()
        .filter(|surface| surface.zone == zone.id && surface.surface_type == SurfaceType::Floor)
        .map(|surface| surface_area_m2(&surface.vertices))
        .sum()
}

fn exterior_wall_area_m2(model: &TypedModel, zone: &Zone) -> f64 {
    model
        .surfaces
        .iter()
        .filter(|surface| {
            surface.zone == zone.id
                && surface.surface_type == SurfaceType::Wall
                && surface.outside_boundary_condition == OutsideBoundaryCondition::Outdoors
        })
        .map(|surface| surface_area_m2(&surface.vertices))
        .sum()
}

pub(crate) fn zone_volume_m3(model: &TypedModel, zone: &Zone) -> Option<f64> {
    zone_geometry_properties(model, zone)
        .ok()
        .map(|state| state.volume_m3)
}

/// Calculates a polygon surface area from 3D vertices in square meters.
#[must_use]
pub fn surface_area_m2(vertices: &[Point3]) -> f64 {
    source_geometry::gross_area(vertices)
}

pub(crate) fn surface_azimuth_deg(vertices: &[Point3]) -> f64 {
    source_geometry::orientation(vertices).map_or(0.0, |value| value.azimuth_deg)
}

pub(crate) fn surface_tilt_deg(_surface_type: SurfaceType, vertices: &[Point3]) -> f64 {
    source_geometry::orientation(vertices).map_or(0.0, |value| value.tilt_deg)
}
