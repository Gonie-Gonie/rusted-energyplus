//! Ordered original opaque-surface geometry, separate from coordinate conversion.
//!
//! Mirrors Vectors.cc and the selected GetVertices/CalcSurfaceCentroid producers.
//! Legacy pure bundles remain available. The selected raw caller owns actual
//! diagnostics, counters and later centroid scratch separately.

use super::centroid_precision::multiply_by_source_third;
mod process_vertices;
pub(crate) use process_vertices::{
    SourceProcessShape, source_process_coplanarity, source_process_distance,
    source_process_is_rectangle, source_process_shift, source_process_sky_rad_split,
    source_process_translation,
};

use ep_model::Point3;
use std::fmt::{Display, Formatter};

const DEG_TO_RAD: f64 = std::f64::consts::PI / 180.0;
type Vector = [f64; 3];

/// Actual derived quantities assigned together at heat-balance initialization.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SurfaceGeometryProperties {
    /// Surface area without subtractions in this opaque input boundary.
    pub area_m2: f64,
    /// Gross area from the magnitude of the ordered Newell area vector.
    pub gross_area_m2: f64,
    /// Opaque area before any excluded subsurface subtraction.
    pub net_area_shadow_m2: f64,
    /// Original degree azimuth including the original cleanup branches.
    pub azimuth_deg: f64,
    /// Original degree tilt from the cyclic normalized Newell vector.
    pub tilt_deg: f64,
    /// Ordered fan cross sum divided by two.
    pub newell_area_vector_m2: Vector,
    /// Cyclic Newell normal normalized with ordered unfused arithmetic.
    pub newell_normal: Vector,
    /// Independently component-snapped outward normal, without renormalization.
    pub out_norm: Vector,
    /// Original area-weighted quad centroid.
    pub centroid_m: Point3,
    /// Normalized vertex-three minus vertex-two local axis.
    pub lcsx: Vector,
    /// Cross product of local Z and local X.
    pub lcsy: Vector,
    /// Cyclic normalized Newell normal used for local Z.
    pub lcsz: Vector,
    /// Sine of stored azimuth times the source degree-to-radian constant.
    pub sin_azimuth: f64,
    /// Cosine of stored azimuth times the source degree-to-radian constant.
    pub cos_azimuth: f64,
    /// Sine of stored tilt times the source degree-to-radian constant.
    pub sin_tilt: f64,
    /// Cosine of stored tilt times the source degree-to-radian constant.
    pub cos_tilt: f64,
}

/// Geometry that cannot initialize a numerical heat-transfer surface.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SurfaceGeometryError {
    /// Fewer than three actual vertices.
    TooFewVertices,
    /// Nonfinite input coordinates or derived geometry.
    Nonfinite,
    /// Exact zero area or cyclic normal.
    Degenerate,
}
impl Display for SurfaceGeometryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::TooFewVertices => "surface requires at least three vertices",
            Self::Nonfinite => "surface coordinates and derived geometry must be finite",
            Self::Degenerate => "surface has zero area or a zero Newell normal",
        })
    }
}

fn vector(point: Point3) -> Vector {
    [point.x_m, point.y_m, point.z_m]
}
fn point(value: Vector) -> Point3 {
    Point3 {
        x_m: value[0],
        y_m: value[1],
        z_m: value[2],
    }
}
fn subtract(a: Vector, b: Vector) -> Vector {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
fn cross(a: Vector, b: Vector) -> Vector {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
fn dot(a: Vector, b: Vector) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn magnitude(v: Vector) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}
fn normalize(v: Vector) -> Vector {
    let length = magnitude(v);
    if length != 0.0 {
        [v[0] / length, v[1] / length, v[2] / length]
    } else {
        [0.0; 3]
    }
}
fn add_to(target: &mut Vector, value: Vector) {
    for i in 0..3 {
        target[i] += value[i];
    }
}
fn multiply(value: Vector, factor: f64) -> Vector {
    [value[0] * factor, value[1] * factor, value[2] * factor]
}
fn area_vector(vertices: &[Point3]) -> Vector {
    if vertices.len() < 3 {
        return [0.0; 3];
    }
    let origin = vector(vertices[0]);
    let mut first = subtract(vector(vertices[1]), origin);
    let mut area = [0.0; 3];
    for vertex in &vertices[2..] {
        let second = subtract(vector(*vertex), origin);
        add_to(&mut area, cross(first, second));
        first = second;
    }
    multiply(area, 1.0 / 2.0)
}
fn cyclic_normal(vertices: &[Point3]) -> Vector {
    let mut value = [0.0; 3];
    for (index, current) in vertices.iter().enumerate() {
        let next = vertices[(index + 1) % vertices.len()];
        value[0] += (current.y_m - next.y_m) * (current.z_m + next.z_m);
        value[1] += (current.z_m - next.z_m) * (current.x_m + next.x_m);
        value[2] += (current.x_m - next.x_m) * (current.y_m + next.y_m);
    }
    normalize(value)
}

pub(super) fn gross_area(vertices: &[Point3]) -> f64 {
    magnitude(area_vector(vertices))
}

/// Original three-point ordered sum and the bounded source extended product.
/// This is the same numerical path used by the stored triangle/quad centroid.
#[must_use]
pub fn source_triangle_centroid(a: Point3, b: Point3, c: Point3) -> Point3 {
    point([
        multiply_by_source_third(a.x_m + b.x_m + c.x_m),
        multiply_by_source_third(a.y_m + b.y_m + c.y_m),
        multiply_by_source_third(a.z_m + b.z_m + c.z_m),
    ])
}

fn polygon_area(vertices: &[Point3; 3]) -> f64 {
    let points = vertices.map(vector);
    let normal = normalize(cross(
        subtract(points[1], points[0]),
        subtract(points[2], points[0]),
    ));
    let mut sum = [0.0; 3];
    add_to(&mut sum, cross(points[0], points[1]));
    add_to(&mut sum, cross(points[1], points[2]));
    add_to(&mut sum, cross(points[2], points[0]));
    0.5 * dot(normal, sum).abs()
}
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceCentroidScratch {
    // Original constructor allocates three Vector elements without a value.
    pub(crate) triangle1: [Option<Point3>; 3],
    pub(crate) triangle2: [Option<Point3>; 3],
}

fn centroid(
    vertices: &[Point3],
    gross_area: f64,
    mut scratch: Option<&mut SourceCentroidScratch>,
) -> Point3 {
    if vertices.len() == 3 {
        return source_triangle_centroid(vertices[0], vertices[1], vertices[2]);
    }
    if vertices.len() == 4 {
        let mut first = [vertices[0], vertices[1], vertices[2]];
        let mut second = [vertices[0], vertices[2], vertices[3]];
        if let Some(scratch) = scratch.as_deref_mut() {
            scratch.triangle1 = first.map(Some);
            scratch.triangle2 = second.map(Some);
        }
        let mut first_fraction = polygon_area(&first) / gross_area;
        let mut second_fraction = polygon_area(&second) / gross_area;
        if (first_fraction + second_fraction) > 1.05 {
            first = [vertices[0], vertices[1], vertices[3]];
            second = [vertices[1], vertices[2], vertices[3]];
            if let Some(scratch) = scratch {
                scratch.triangle1 = first.map(Some);
                scratch.triangle2 = second.map(Some);
            }
            let first_area = polygon_area(&first);
            let second_area = polygon_area(&second);
            let total_area = first_area + second_area;
            first_fraction = first_area / total_area;
            second_fraction = second_area / total_area;
        }
        let first = source_triangle_centroid(first[0], first[1], first[2]);
        let second = source_triangle_centroid(second[0], second[1], second[2]);
        let mut value = multiply(vector(first), first_fraction);
        add_to(&mut value, multiply(vector(second), second_fraction));
        return point(value);
    }
    let mut value = [0.0; 3];
    for vertex in vertices {
        add_to(&mut value, vector(*vertex));
    }
    point(multiply(value, 1.0 / vertices.len() as f64))
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Orientation {
    pub(crate) azimuth_deg: f64,
    pub(crate) tilt_deg: f64,
    pub(crate) lcsx: Vector,
    pub(crate) lcsy: Vector,
    pub(crate) lcsz: Vector,
}
pub(super) fn orientation(vertices: &[Point3]) -> Option<Orientation> {
    if vertices.len() < 3 {
        return None;
    }
    Some(orientation_from_normal(vertices, cyclic_normal(vertices)))
}

pub(crate) fn orientation_from_normal(vertices: &[Point3], normal: Vector) -> Orientation {
    let lcsx = normalize(subtract(vector(vertices[2]), vector(vertices[1])));
    let lcsz = normal;
    let lcsy = cross(lcsz, lcsx);
    let costheta = dot(lcsz, [0.0, 0.0, 1.0]);
    let rotang_0 = if costheta.abs() < 1.0 - 1.12e-16 {
        let x2 = cross([0.0, 0.0, 1.0], lcsz);
        dot(x2, [0.0, 1.0, 0.0]).atan2(dot(x2, [1.0, 0.0, 0.0]))
    } else {
        dot(lcsx, [0.0, 1.0, 0.0]).atan2(dot(lcsx, [1.0, 0.0, 0.0]))
    };
    let mut tilt = normal[2].acos() / DEG_TO_RAD;
    let mut azimuth = rotang_0 / DEG_TO_RAD;
    azimuth = (450.0 - azimuth) % 360.0;
    azimuth += 90.0;
    if azimuth < 0.0 {
        azimuth += 360.0;
    }
    azimuth %= 360.0;
    if (azimuth - 360.0).abs() < 0.001 {
        azimuth = 0.0;
    } else if (azimuth - 180.0).abs() < 1.0e-6 {
        azimuth = 180.0;
    }
    if (tilt - 180.0).abs() < 1.0e-6 {
        tilt = 180.0;
    }
    Orientation {
        azimuth_deg: azimuth,
        tilt_deg: tilt,
        lcsx,
        lcsy,
        lcsz,
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct GetVerticesProperties {
    /// Surface area without subtractions in this opaque input boundary.
    pub(crate) area_m2: f64,
    /// Gross area from the magnitude of the ordered Newell area vector.
    pub(crate) gross_area_m2: f64,
    /// Opaque area before any excluded subsurface subtraction.
    pub(crate) net_area_shadow_m2: f64,
    /// Original degree azimuth including the original cleanup branches.
    pub(crate) azimuth_deg: f64,
    /// Original degree tilt from the cyclic normalized Newell vector.
    pub(crate) tilt_deg: f64,
    /// Ordered fan cross sum divided by two.
    pub(crate) newell_area_vector_m2: Vector,
    /// Cyclic Newell normal normalized with ordered unfused arithmetic.
    pub(crate) newell_normal: Vector,
    /// Independently component-snapped outward normal, without renormalization.
    pub(crate) out_norm: Vector,
    /// Normalized vertex-three minus vertex-two local axis.
    pub(crate) lcsx: Vector,
    /// Cross product of local Z and local X.
    pub(crate) lcsy: Vector,
    /// Cyclic normalized Newell normal used for local Z.
    pub(crate) lcsz: Vector,
    /// Sine of stored azimuth times the source degree-to-radian constant.
    pub(crate) sin_azimuth: f64,
    /// Cosine of stored azimuth times the source degree-to-radian constant.
    pub(crate) cos_azimuth: f64,
    /// Sine of stored tilt times the source degree-to-radian constant.
    pub(crate) sin_tilt: f64,
    /// Cosine of stored tilt times the source degree-to-radian constant.
    pub(crate) cos_tilt: f64,
}

/// Ordered numerical GetVertices quantities, excluding later centroid work.
/// The actual SurfaceTmp consumer retains reached raw writes and diagnostics.
/// Legacy nonfinite/degenerate numerical admission remains unchanged.
/// Quantities assigned at actual GetVertices9383-9388, before diagnostics.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct SourceNormalArea {
    pub(crate) newell_normal: Vector,
    pub(crate) newell_area_vector_m2: Vector,
    pub(crate) gross_area_m2: f64,
}

pub(crate) fn source_normal_area(vertices: &[Point3]) -> SourceNormalArea {
    let newell_normal = cyclic_normal(vertices);
    let newell_area_vector_m2 = area_vector(vertices);
    let gross_area_m2 = magnitude(newell_area_vector_m2);
    SourceNormalArea {
        newell_normal,
        newell_area_vector_m2,
        gross_area_m2,
    }
}

/// Later trig/outward-normal numerical work, after real orientation diagnostics.
pub(crate) fn finish_get_vertices_properties(
    normal_area: SourceNormalArea,
    orientation: Orientation,
) -> GetVerticesProperties {
    let SourceNormalArea {
        newell_normal,
        newell_area_vector_m2,
        gross_area_m2,
    } = normal_area;
    let Orientation {
        azimuth_deg,
        tilt_deg,
        lcsx,
        lcsy,
        lcsz,
    } = orientation;
    let sin_azimuth = (azimuth_deg * DEG_TO_RAD).sin();
    let cos_azimuth = (azimuth_deg * DEG_TO_RAD).cos();
    let sin_tilt = (tilt_deg * DEG_TO_RAD).sin();
    let cos_tilt = (tilt_deg * DEG_TO_RAD).cos();
    let mut out_norm = newell_normal;
    for value in &mut out_norm {
        if (*value - 1.0).abs() < 1.0e-6 {
            *value = 1.0;
        }
        if (*value + 1.0).abs() < 1.0e-6 {
            *value = -1.0;
        }
        if value.abs() < 1.0e-6 {
            *value = 0.0;
        }
    }
    GetVerticesProperties {
        area_m2: gross_area_m2,
        gross_area_m2,
        net_area_shadow_m2: gross_area_m2,
        azimuth_deg,
        tilt_deg,
        newell_area_vector_m2,
        newell_normal,
        out_norm,
        lcsx,
        lcsy,
        lcsz,
        sin_azimuth,
        cos_azimuth,
        sin_tilt,
        cos_tilt,
    }
}

/// Existing legacy numerical admission, composed from the same ordered pieces.
pub(crate) fn get_vertices_properties(
    vertices: &[Point3],
) -> Result<GetVerticesProperties, SurfaceGeometryError> {
    if vertices.len() < 3 {
        return Err(SurfaceGeometryError::TooFewVertices);
    }
    if vertices.iter().any(|value| {
        vector(*value)
            .iter()
            .any(|component| !component.is_finite())
    }) {
        return Err(SurfaceGeometryError::Nonfinite);
    }
    let normal_area = source_normal_area(vertices);
    let orientation = orientation_from_normal(vertices, normal_area.newell_normal);
    if normal_area.gross_area_m2 == 0.0 || normal_area.newell_normal == [0.0; 3] {
        return Err(SurfaceGeometryError::Degenerate);
    }
    let returned = finish_get_vertices_properties(normal_area, orientation);
    if !returned.is_finite() {
        return Err(SurfaceGeometryError::Nonfinite);
    }
    Ok(returned)
}

impl GetVerticesProperties {
    pub(crate) fn is_finite(self) -> bool {
        ![
            self.gross_area_m2,
            self.azimuth_deg,
            self.tilt_deg,
            self.sin_azimuth,
            self.cos_azimuth,
            self.sin_tilt,
            self.cos_tilt,
        ]
        .into_iter()
        .chain(self.newell_normal)
        .chain(self.out_norm)
        .chain(self.lcsx)
        .chain(self.lcsy)
        .chain(self.lcsz)
        .any(|value| !value.is_finite())
    }
    /// Move the same quantities into the complete owner at CalcSurfaceCentroid.
    pub(crate) fn with_centroid(self, centroid_m: Point3) -> SurfaceGeometryProperties {
        SurfaceGeometryProperties {
            area_m2: self.area_m2,
            gross_area_m2: self.gross_area_m2,
            net_area_shadow_m2: self.net_area_shadow_m2,
            azimuth_deg: self.azimuth_deg,
            tilt_deg: self.tilt_deg,
            newell_area_vector_m2: self.newell_area_vector_m2,
            newell_normal: self.newell_normal,
            out_norm: self.out_norm,
            lcsx: self.lcsx,
            lcsy: self.lcsy,
            lcsz: self.lcsz,
            sin_azimuth: self.sin_azimuth,
            cos_azimuth: self.cos_azimuth,
            sin_tilt: self.sin_tilt,
            cos_tilt: self.cos_tilt,
            centroid_m,
        }
    }
}

/// Actual later centroid numerical producer. The caller owns warning output and
/// stores the returned centroid before checking the supported finite domain.
pub(crate) fn source_surface_centroid(
    vertices: &[Point3],
    gross_area_m2: f64,
    scratch: &mut SourceCentroidScratch,
) -> Point3 {
    if vertices.len() >= 5 {
        let mut value = [0.0; 3];
        for vertex in vertices {
            add_to(&mut value, vector(*vertex));
        }
        let sides = vertices.len() as f64;
        // Literal Original14618 division, distinct from legacy reciprocal math.
        for component in &mut value {
            *component /= sides;
        }
        point(value)
    } else {
        centroid(vertices, gross_area_m2, Some(scratch))
    }
}

/// Legacy bundle composed from the same existing numerical operations.
/// The actual selected raw caller invokes the two boundaries separately.
pub fn surface_geometry_properties(
    vertices: &[Point3],
) -> Result<SurfaceGeometryProperties, SurfaceGeometryError> {
    let returned = get_vertices_properties(vertices)?;
    let centroid_m = centroid(vertices, returned.gross_area_m2, None);
    if vector(centroid_m).iter().any(|value| !value.is_finite()) {
        return Err(SurfaceGeometryError::Nonfinite);
    }
    Ok(returned.with_centroid(centroid_m))
}

#[cfg(test)]
mod tests;
