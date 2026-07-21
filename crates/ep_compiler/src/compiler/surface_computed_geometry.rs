//! Bounded projection of `SurfaceData::set_computed_geometry`.

use super::{Compiler, DiagnosticSeverity};
use ep_model::{
    Point3, SurfaceComputedGeometry, SurfaceProjectedPoint, SurfaceProjectionAxis,
    SurfaceShapeCategory, TypedModel,
};

const RECTANGLE_DIAGONAL_DIFFERENCE_M: f64 = 0.020;
// Additional bounded admission gate: every vertex must remain within 1e-9 of
// the model-coordinate scale from its Newell plane. This intentionally omits
// non-planar geometry instead of promoting the source warning/recovery paths.
const COPLANAR_DISTANCE_RELATIVE_TOLERANCE: f64 = 1.0e-9;

impl Compiler<'_> {
    /// Attaches bounded source-shaped geometry to admitted detailed surfaces.
    ///
    /// The projection is positive-only: finite, nondegenerate triangles and
    /// conservatively recognized coplanar rectangles are attached, while all
    /// other shapes are left without derived state and without a diagnostic.
    pub(super) fn set_bounded_surface_computed_geometry(&mut self, model: &mut TypedModel) {
        if self
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity == DiagnosticSeverity::Error)
        {
            return;
        }

        for surface in &mut model.surfaces {
            surface.computed_geometry = computed_geometry(&surface.vertices);
        }
    }
}

fn computed_geometry(vertices: &[Point3]) -> Option<SurfaceComputedGeometry> {
    let shape_category = match vertices.len() {
        3 => SurfaceShapeCategory::Triangular,
        4 => SurfaceShapeCategory::Rectangular,
        _ => return None,
    };
    if !vertices.iter().all(point_is_finite) {
        return None;
    }

    let plane = newell_plane(vertices)?;
    if !is_conservatively_coplanar(vertices, plane) {
        return None;
    }
    if shape_category == SurfaceShapeCategory::Rectangular && !is_source_rectangle(vertices) {
        return None;
    }

    projected_geometry(vertices, shape_category, plane)
}

fn point_is_finite(point: &Point3) -> bool {
    point.x_m.is_finite() && point.y_m.is_finite() && point.z_m.is_finite()
}

fn newell_plane(vertices: &[Point3]) -> Option<[f64; 4]> {
    let mut a = 0.0;
    let mut b = 0.0;
    let mut c = 0.0;
    let mut center_x = 0.0;
    let mut center_y = 0.0;
    let mut center_z = 0.0;

    for (index, vertex) in vertices.iter().enumerate() {
        let next = &vertices[(index + 1) % vertices.len()];
        a += (vertex.y_m - next.y_m) * (vertex.z_m + next.z_m);
        b += (vertex.z_m - next.z_m) * (vertex.x_m + next.x_m);
        c += (vertex.x_m - next.x_m) * (vertex.y_m + next.y_m);
        center_x += vertex.x_m;
        center_y += vertex.y_m;
        center_z += vertex.z_m;
    }

    let d = -((center_x * a + center_y * b + center_z * c) / vertices.len() as f64);
    let plane = [a, b, c, d];
    let normal_magnitude = a.hypot(b).hypot(c);
    if plane.iter().all(|coefficient| coefficient.is_finite())
        && normal_magnitude.is_finite()
        && normal_magnitude > 0.0
    {
        Some(plane)
    } else {
        None
    }
}

fn is_conservatively_coplanar(vertices: &[Point3], plane: [f64; 4]) -> bool {
    let [a, b, c, d] = plane;
    let normal_magnitude = a.hypot(b).hypot(c);
    let coordinate_scale = vertices.iter().fold(1.0_f64, |scale, vertex| {
        scale
            .max(vertex.x_m.abs())
            .max(vertex.y_m.abs())
            .max(vertex.z_m.abs())
    });
    let distance_tolerance = COPLANAR_DISTANCE_RELATIVE_TOLERANCE * coordinate_scale;
    distance_tolerance.is_finite()
        && vertices.iter().all(|vertex| {
            let residual = a * vertex.x_m + b * vertex.y_m + c * vertex.z_m + d;
            residual.is_finite() && residual.abs() / normal_magnitude <= distance_tolerance
        })
}

fn is_source_rectangle(vertices: &[Point3]) -> bool {
    debug_assert_eq!(vertices.len(), 4);

    // The source predicate normalizes edges 3->2 and 2->1. This bounded
    // projection additionally rejects zero-length edges before normalization.
    if (0..vertices.len()).any(|index| {
        vector_length(vertices[index], vertices[(index + 1) % vertices.len()])
            .is_none_or(|length| length == 0.0)
    }) {
        return false;
    }

    let diagonal_1 = vector_length(vertices[0], vertices[2]);
    let diagonal_2 = vector_length(vertices[1], vertices[3]);
    let (Some(diagonal_1), Some(diagonal_2)) = (diagonal_1, diagonal_2) else {
        return false;
    };
    if (diagonal_1 - diagonal_2).abs() >= RECTANGLE_DIAGONAL_DIFFERENCE_M {
        return false;
    }

    let edge_32 = normalized_vector(vertices[1], vertices[2]);
    let edge_21 = normalized_vector(vertices[0], vertices[1]);
    let (Some(edge_32), Some(edge_21)) = (edge_32, edge_21) else {
        return false;
    };
    let dot = edge_32[0] * edge_21[0] + edge_32[1] * edge_21[1] + edge_32[2] * edge_21[2];
    source_rectangle_right_angle_is_admitted(dot)
}

pub(super) fn source_rectangle_right_angle_is_admitted(dot: f64) -> bool {
    dot.is_finite() && dot.abs() <= 89.0_f64.to_radians().cos()
}

fn vector_length(from: Point3, to: Point3) -> Option<f64> {
    let dx = to.x_m - from.x_m;
    let dy = to.y_m - from.y_m;
    let dz = to.z_m - from.z_m;
    // Preserve the source VecSquaredLength multiplication/addition order for
    // both diagonal comparison and VecNormalize at their inclusive boundaries.
    let length = (dx * dx + dy * dy + dz * dz).sqrt();
    length.is_finite().then_some(length)
}

fn normalized_vector(from: Point3, to: Point3) -> Option<[f64; 3]> {
    let length = vector_length(from, to)?;
    if length == 0.0 {
        return None;
    }
    Some([
        (to.x_m - from.x_m) / length,
        (to.y_m - from.y_m) / length,
        (to.z_m - from.z_m) / length,
    ])
}

fn projected_geometry(
    vertices: &[Point3],
    shape_category: SurfaceShapeCategory,
    plane: [f64; 4],
) -> Option<SurfaceComputedGeometry> {
    let [a, b, c, _] = plane;
    let projection_axis = if a.abs() >= b.abs().max(c.abs()) {
        SurfaceProjectionAxis::X
    } else if b.abs() >= a.abs().max(c.abs()) {
        SurfaceProjectionAxis::Y
    } else {
        SurfaceProjectionAxis::Z
    };
    let mut projected_vertices = vertices
        .iter()
        .map(|vertex| project_point(*vertex, projection_axis))
        .collect::<Vec<_>>();

    let first = *projected_vertices.first()?;
    let (mut lower, mut upper) = (first, first);
    for vertex in &projected_vertices {
        lower.x_m = source_min(lower.x_m, vertex.x_m);
        lower.y_m = source_min(lower.y_m, vertex.y_m);
        upper.x_m = source_max(upper.x_m, vertex.x_m);
        upper.y_m = source_max(upper.y_m, vertex.y_m);
    }

    let mut signed_area_twice = 0.0;
    for index in 0..projected_vertices.len() {
        let vertex = projected_vertices[index];
        let next = projected_vertices[(index + 1) % projected_vertices.len()];
        signed_area_twice += vertex.x_m * next.y_m - next.x_m * vertex.y_m;
    }
    if !signed_area_twice.is_finite() || signed_area_twice == 0.0 {
        return None;
    }
    if signed_area_twice < 0.0 {
        projected_vertices[1..].reverse();
    }

    let projected_edges = (0..projected_vertices.len())
        .map(|index| {
            let vertex = projected_vertices[index];
            let next = projected_vertices[(index + 1) % projected_vertices.len()];
            SurfaceProjectedPoint {
                x_m: next.x_m - vertex.x_m,
                y_m: next.y_m - vertex.y_m,
            }
        })
        .collect::<Vec<_>>();
    if !projected_edges
        .iter()
        .all(|edge| edge.x_m.is_finite() && edge.y_m.is_finite())
    {
        return None;
    }

    let (rectangle_side_1_squared_m2, rectangle_side_3_squared_m2) =
        if shape_category == SurfaceShapeCategory::Rectangular {
            (
                squared_length(projected_edges[0])?,
                squared_length(projected_edges[3])?,
            )
        } else {
            (0.0, 0.0)
        };

    Some(SurfaceComputedGeometry {
        shape_category,
        plane,
        projection_axis,
        projected_vertices,
        projected_lower_bound: lower,
        projected_upper_bound: upper,
        projected_edges,
        rectangle_side_1_squared_m2,
        rectangle_side_3_squared_m2,
    })
}

fn source_min(current: f64, candidate: f64) -> f64 {
    if candidate < current {
        candidate
    } else {
        current
    }
}

fn source_max(current: f64, candidate: f64) -> f64 {
    if current < candidate {
        candidate
    } else {
        current
    }
}

fn project_point(point: Point3, axis: SurfaceProjectionAxis) -> SurfaceProjectedPoint {
    match axis {
        SurfaceProjectionAxis::X => SurfaceProjectedPoint {
            x_m: point.y_m,
            y_m: point.z_m,
        },
        SurfaceProjectionAxis::Y => SurfaceProjectedPoint {
            x_m: point.x_m,
            y_m: point.z_m,
        },
        SurfaceProjectionAxis::Z => SurfaceProjectedPoint {
            x_m: point.x_m,
            y_m: point.y_m,
        },
    }
}

fn squared_length(vector: SurfaceProjectedPoint) -> Option<f64> {
    let squared = vector.x_m * vector.x_m + vector.y_m * vector.y_m;
    (squared.is_finite() && squared > 0.0).then_some(squared)
}
