//! Initial source closure and source-ordered signed volume; no polygon repair.

use super::{ZoneVolumeDiagnostics, ZoneVolumeEdge, ZoneVolumeFace};
use ep_model::{Point3, SurfaceId, SurfaceType};

fn almost_equal(a: Point3, b: Point3) -> bool {
    (a.x_m - b.x_m).abs() < 0.01 && (a.y_m - b.y_m).abs() < 0.01 && (a.z_m - b.z_m).abs() < 0.01
}

struct Edge {
    start: usize,
    end: usize,
    first: SurfaceId,
    others: Vec<SurfaceId>,
    count: u32,
    direction_balance: i32,
}

pub(crate) fn initial_enclosure(faces: &[ZoneVolumeFace]) -> ZoneVolumeDiagnostics {
    // makeListOfUniqueVertices preserves first occurrence under the original
    // strict per-coordinate OneCentimeter test, before any repair.
    let mut unique = Vec::new();
    for face in faces {
        for point in &face.vertices {
            if !unique.iter().any(|other| almost_equal(*point, *other)) {
                unique.push(*point);
            }
        }
    }
    let mut edges: Vec<Edge> = Vec::new();
    let mut data = ZoneVolumeDiagnostics {
        initial_unique_vertex_count: unique.len(),
        ..ZoneVolumeDiagnostics::default()
    };
    for face in faces {
        // edgesNotTwo visits last->first, then each retained successive point.
        for (index, current) in face.vertices.iter().enumerate() {
            let previous = face.vertices[(index + face.vertices.len() - 1) % face.vertices.len()];
            let start = unique.iter().position(|p| almost_equal(*p, previous));
            let end = unique.iter().position(|p| almost_equal(*p, *current));
            if let (Some(start), Some(end)) = (start, end) {
                if let Some(edge) = edges
                    .iter_mut()
                    .find(|e| e.start == start && e.end == end || e.start == end && e.end == start)
                {
                    edge.count += 1;
                    edge.others.push(face.surface_id);
                    edge.direction_balance += if edge.start == start && edge.end == end {
                        1
                    } else {
                        -1
                    };
                } else {
                    edges.push(Edge {
                        start,
                        end,
                        first: face.surface_id,
                        others: Vec::new(),
                        count: 1,
                        direction_balance: 1,
                    });
                }
            }
        }
    }
    data.initial_edges_not_used_twice = edges
        .iter()
        .filter(|edge| edge.count != 2)
        .map(|edge| ZoneVolumeEdge {
            surface_id: edge.first,
            start_m: unique[edge.start],
            end_m: unique[edge.end],
            count: edge.count,
            other_surface_ids: edge.others.clone(),
        })
        .collect();
    data.initially_closed = !edges.is_empty() && data.initial_edges_not_used_twice.is_empty();
    data.edges_winding_consistent = !edges.is_empty()
        && edges
            .iter()
            .all(|edge| edge.count == 2 && edge.direction_balance == 0);
    data
}

pub(crate) fn surface_orientation(faces: &[ZoneVolumeFace]) -> (bool, bool, bool) {
    let (mut floor, mut roof, mut walls) = (true, true, true);
    for face in faces {
        match face.surface_type {
            SurfaceType::Floor => {
                if (face.tilt_deg - 180.0).abs() > 1.0 {
                    floor = false;
                }
            }
            SurfaceType::Roof | SurfaceType::Ceiling => {
                if face.tilt_deg.abs() > 1.0 {
                    roof = false;
                }
            }
            SurfaceType::Wall => {
                if (face.tilt_deg - 90.0).abs() > 1.0 {
                    walls = false;
                }
            }
        }
    }
    (floor, roof, walls)
}

pub(crate) fn wall_heights_same(faces: &[ZoneVolumeFace]) -> bool {
    let mut wall_height: Option<f64> = None;
    for face in faces {
        if face.surface_type != SurfaceType::Wall {
            continue;
        }
        // Original Constant::BigNumber is numeric_limits<Real64>::max().
        let mut max_z = -f64::MAX;
        for point in &face.vertices {
            if point.z_m > max_z {
                max_z = point.z_m;
            }
        }
        if let Some(previous) = wall_height {
            if (max_z - previous).abs() > 0.02 {
                return false;
            }
        } else {
            wall_height = Some(max_z);
        }
    }
    true
}

pub(crate) fn polyhedron_volume(faces: &[ZoneVolumeFace], p0: Point3) -> Option<f64> {
    let mut signed_volume = 0.0;
    let mut has_signed_volume = true;
    for face in faces {
        // Vectors.cc523-526: second face point, ordered dot, each divide/add.
        if let Some(point) = face.vertices.get(1) {
            let area = face.newell_area_vector_m2;
            let pyramid = area[0] * (point.x_m - p0.x_m)
                + area[1] * (point.y_m - p0.y_m)
                + area[2] * (point.z_m - p0.z_m);
            signed_volume += pyramid / 3.0;
        } else {
            has_signed_volume = false;
        }
    }
    (has_signed_volume && signed_volume.is_finite()).then_some(signed_volume)
}

pub(crate) fn bounded_topology_rejection(
    faces: &[ZoneVolumeFace],
    data: &ZoneVolumeDiagnostics,
) -> Option<String> {
    let wall_count = faces
        .iter()
        .filter(|f| f.surface_type == SurfaceType::Wall)
        .count();
    let floor_count = faces
        .iter()
        .filter(|f| f.surface_type == SurfaceType::Floor)
        .count();
    let roof_count = faces
        .iter()
        .filter(|f| matches!(f.surface_type, SurfaceType::Roof | SurfaceType::Ceiling))
        .count();
    let signed_volume = data.signed_polyhedron_volume_m3;
    if faces.len() != 6
        || wall_count != 4
        || floor_count != 1
        || roof_count != 1
        || faces.iter().any(|f| f.vertices.len() != 4)
    {
        Some("automatic volume requires the selected six base quadrilaterals (four walls, one floor and one roof)".into())
    } else if !data.initially_closed {
        Some("initial base-face edges are not used exactly twice; source polygon repair and open-zone fallback are unpaired".into())
    } else if !data.edges_winding_consistent {
        Some("base-face edge winding is inconsistent".into())
    } else if signed_volume.is_none() {
        Some("signed polyhedron volume is not finite".into())
    } else if signed_volume.is_some_and(|v| v <= 0.0) {
        Some("signed polyhedron volume is nonpositive; source-only10m3 fallback is unpaired".into())
    } else {
        None
    }
}

pub(super) fn inspect(faces: &[ZoneVolumeFace], p0: Point3) -> ZoneVolumeDiagnostics {
    let mut data = initial_enclosure(faces);
    (
        data.floor_horizontal,
        data.roof_horizontal,
        data.walls_vertical,
    ) = surface_orientation(faces);
    data.same_wall_height = wall_heights_same(faces);
    data.signed_polyhedron_volume_m3 = polyhedron_volume(faces, p0);
    data.topology_rejection = bounded_topology_rejection(faces, &data);
    data
}
