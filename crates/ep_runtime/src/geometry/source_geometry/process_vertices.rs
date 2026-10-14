//! Literal selected ProcessSurfaceVertices point operations; pairing pending.
use super::{DEG_TO_RAD, Vector, dot, magnitude, normalize, subtract, vector};
use ep_model::Point3;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(i32)]
pub(crate) enum SourceProcessShape {
    #[default]
    None = 0,
    Triangle = 1,
    Quadrilateral = 2,
    Rectangle = 3,
    Polygonal = 10,
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(dead_code)] // Actual callee locals are inspected by the cfg(test) reference consumer.
pub(crate) struct SourceCoplanarity {
    pub(crate) plane: [f64; 4],
    pub(crate) plane_error: bool,
    pub(crate) is_coplanar: bool,
    pub(crate) max_distance: f64,
    pub(crate) error_vertex: usize,
}

pub(crate) fn source_process_coplanarity(vertices: &[Point3]) -> SourceCoplanarity {
    // Vectors287-326 plus genuine PlaneEq constructor0, not a missing-value fallback.
    let mut normal = [0.0; 3];
    let mut reference = [0.0; 3];
    for (i, current) in vertices.iter().enumerate() {
        let u = vector(*current);
        let v = vector(vertices[(i + 1) % vertices.len()]);
        normal[0] += (u[1] - v[1]) * (u[2] + v[2]);
        normal[1] += (u[2] - v[2]) * (u[0] + v[0]);
        normal[2] += (u[0] - v[0]) * (u[1] + v[1]);
        for j in 0..3 {
            reference[j] += u[j];
        }
    }
    let mut plane = [0.0; 4];
    let mut length = magnitude(normal);
    let plane_error = length == 0.0;
    if length != 0.0 {
        plane[0] = normal[0] / length;
        plane[1] = normal[1] / length;
        plane[2] = normal[2] / length;
        length *= vertices.len() as f64;
        plane[3] = -dot(reference, normal) / length;
    }
    // Actual CalcCoPlanarNess ignores plerror and uses this local plane.
    let mut max_distance = 0.0;
    let mut error_vertex = 0;
    for (i, vertex) in vertices.iter().enumerate() {
        let p = vector(*vertex);
        let distance = plane[0] * p[0] + plane[1] * p[1] + plane[2] * p[2] + plane[3];
        if distance.abs() > max_distance {
            max_distance = distance.abs();
            error_vertex = i + 1;
        }
    }
    SourceCoplanarity {
        plane,
        plane_error,
        is_coplanar: max_distance.abs() <= 1.0e-4,
        max_distance,
        error_vertex,
    }
}

pub(crate) fn source_process_distance(a: Point3, b: Point3) -> f64 {
    magnitude(subtract(vector(a), vector(b)))
}

pub(crate) fn source_process_is_rectangle(vertices: &[Point3; 4]) -> bool {
    let first = source_process_distance(vertices[0], vertices[2]);
    let second = source_process_distance(vertices[1], vertices[3]);
    if (first - second).abs() < 0.020 {
        let a = normalize(subtract(vector(vertices[2]), vector(vertices[1])));
        let b = normalize(subtract(vector(vertices[1]), vector(vertices[0])));
        return dot(a, b).abs() <= (89.0 * DEG_TO_RAD).cos();
    }
    false
}

pub(crate) fn source_process_translation(vertices: &[Point3]) -> Option<Vector> {
    let second = vector(vertices[1]);
    let x21 = subtract(second, vector(vertices[0]));
    let x23 = subtract(second, vector(vertices[2]));
    if dot(x23, x23) <= 1.0e-6 {
        // Caller retains the preceding ShadeV writes, then stops at the
        // still-unported diagnostic/optional-ESO/fatal branch13433-13440.
        return None;
    }
    let gamma = dot(x21, x23) / dot(x23, x23);
    let edge = subtract(vector(vertices[2]), second);
    Some([
        second[0] + gamma * edge[0],
        second[1] + gamma * edge[1],
        second[2] + gamma * edge[2],
    ])
}

pub(crate) fn source_process_shift(axis: Vector, second: Point3, origin: Vector) -> f64 {
    dot(axis, subtract(vector(second), origin))
}

pub(crate) fn source_process_sky_rad_split(cos_tilt: f64) -> f64 {
    (0.5 * (1.0 + cos_tilt)).sqrt()
}
