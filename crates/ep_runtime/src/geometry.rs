//! Geometry summary and polygon helper functions.

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
    if let AutoOrNumber::Value(volume_m3) = zone.volume
        && volume_m3 > 0.0
    {
        return Some(volume_m3);
    }

    if let Some(volume_m3) = bounding_box_volume_m3(model, zone)
        && volume_m3 > 0.0
    {
        return Some(volume_m3);
    }

    let AutoOrNumber::Value(ceiling_height_m) = zone.ceiling_height else {
        return None;
    };
    if ceiling_height_m <= 0.0 {
        return None;
    }
    let floor_area_m2 = zone_floor_area_m2(model, zone);
    if floor_area_m2 > 0.0 {
        Some(floor_area_m2 * ceiling_height_m)
    } else {
        None
    }
}

fn bounding_box_volume_m3(model: &TypedModel, zone: &Zone) -> Option<f64> {
    let mut bounds: Option<(f64, f64, f64, f64, f64, f64)> = None;
    for surface in model
        .surfaces
        .iter()
        .filter(|surface| surface.zone == zone.id)
    {
        for vertex in &surface.vertices {
            let x = vertex.x_m + zone.origin.x_m;
            let y = vertex.y_m + zone.origin.y_m;
            let z = vertex.z_m + zone.origin.z_m;
            bounds = Some(match bounds {
                Some((min_x, max_x, min_y, max_y, min_z, max_z)) => (
                    min_x.min(x),
                    max_x.max(x),
                    min_y.min(y),
                    max_y.max(y),
                    min_z.min(z),
                    max_z.max(z),
                ),
                None => (x, x, y, y, z, z),
            });
        }
    }

    let (min_x, max_x, min_y, max_y, min_z, max_z) = bounds?;
    let volume_m3 = (max_x - min_x) * (max_y - min_y) * (max_z - min_z);
    if volume_m3 > 0.0 {
        Some(volume_m3)
    } else {
        None
    }
}

/// Calculates a polygon surface area from 3D vertices in square meters.
#[must_use]
pub fn surface_area_m2(vertices: &[Point3]) -> f64 {
    if vertices.len() < 3 {
        return 0.0;
    }

    let origin = vertices[0];
    vertices[1..]
        .windows(2)
        .map(|window| {
            let first = vector_between(origin, window[0]);
            let second = vector_between(origin, window[1]);
            cross(first, second).magnitude() * 0.5
        })
        .sum()
}

pub(crate) fn surface_azimuth_deg(vertices: &[Point3]) -> f64 {
    let Some(normal) = polygon_normal(vertices) else {
        return 0.0;
    };

    let horizontal_magnitude = normal.x.hypot(normal.y);
    if horizontal_magnitude > 1.0e-12 {
        return normalize_degrees(normal.x.atan2(normal.y).to_degrees());
    }

    // EnergyPlus DetermineAzimuthAndTilt defines the local x axis from vertex 2 to
    // vertex 3 for horizontal surfaces, whose normal cannot define an azimuth.
    let edge = vector_between(vertices[1], vertices[2]);
    if edge.x.hypot(edge.y) <= 1.0e-12 {
        return 0.0;
    }

    normalize_degrees(180.0 - edge.y.atan2(edge.x).to_degrees())
}

pub(crate) fn surface_tilt_deg(surface_type: SurfaceType, vertices: &[Point3]) -> f64 {
    let Some(normal) = polygon_normal(vertices) else {
        return 0.0;
    };
    let magnitude = normal.magnitude();
    if magnitude <= 1.0e-12 {
        return 0.0;
    }
    if (normal.z.abs() / magnitude) > 1.0 - 1.0e-12 {
        return match surface_type {
            SurfaceType::Floor => 180.0,
            SurfaceType::Roof | SurfaceType::Ceiling => 0.0,
            SurfaceType::Wall => 90.0,
        };
    }

    (-normal.z / magnitude).clamp(-1.0, 1.0).acos().to_degrees()
}

fn polygon_normal(vertices: &[Point3]) -> Option<Vector3> {
    if vertices.len() < 3 {
        return None;
    }

    let origin = vertices[0];
    let mut normal = Vector3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    for window in vertices[1..].windows(2) {
        let first = vector_between(origin, window[0]);
        let second = vector_between(origin, window[1]);
        let triangle_normal = cross(first, second);
        normal.x += triangle_normal.x;
        normal.y += triangle_normal.y;
        normal.z += triangle_normal.z;
    }

    if normal.magnitude() > 1.0e-12 {
        Some(normal)
    } else {
        None
    }
}

fn normalize_degrees(value: f64) -> f64 {
    value.rem_euclid(360.0)
}

#[derive(Clone, Copy)]
struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3 {
    fn magnitude(self) -> f64 {
        (self
            .x
            .mul_add(self.x, self.y.mul_add(self.y, self.z * self.z)))
        .sqrt()
    }
}

fn vector_between(origin: Point3, point: Point3) -> Vector3 {
    Vector3 {
        x: point.x_m - origin.x_m,
        y: point.y_m - origin.y_m,
        z: point.z_m - origin.z_m,
    }
}

fn cross(left: Vector3, right: Vector3) -> Vector3 {
    Vector3 {
        x: left.y * right.z - left.z * right.y,
        y: left.z * right.x - left.x * right.z,
        z: left.x * right.y - left.y * right.x,
    }
}
