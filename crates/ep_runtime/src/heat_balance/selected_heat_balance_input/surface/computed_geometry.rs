//! Literal DataSurfaces190--197 /361--470 and actual Surface2D constructor89--187.
//! Existing Source row.Shape/IsConvex/Vertex are the only producer operands.
use super::final_mapping::FinalSurface;
use super::{Error, SurfaceGeometryInputState};
use crate::geometry::SourceProcessShape;
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(i32)]
pub(crate) enum SourceComputedShapeCategory {
    #[default]
    Invalid = -1,
    Triangular = 0,
    Rectangular = 1,
    Convex = 2,
    Nonconvex = 3,
    Num = 4,
}
/// Actual Surface2DSlab ctor: xl/xu0, borrowed constructor yl/yu values, empty vectors.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceComputedSurfaceSlab {
    pub(crate) xl: f64,
    pub(crate) xu: f64,
    pub(crate) yl: f64,
    pub(crate) yu: f64,
    pub(crate) edges: Vec<usize>,
    pub(crate) edges_xy: Vec<f64>,
}
/// Actual Surface2D default ctor: axis0, unallocated Array1D, corners0, s1/s3zero, stdvectors empty.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceComputedSurface2D {
    pub(crate) axis: i32,
    pub(crate) vertices: Option<Vec<[f64; 2]>>,
    pub(crate) vl: [f64; 2],
    pub(crate) vu: [f64; 2],
    pub(crate) edges: Option<Vec<[f64; 2]>>,
    pub(crate) s1: f64,
    pub(crate) s3: f64,
    pub(crate) slab_ys: Vec<f64>,
    pub(crate) slabs: Vec<SourceComputedSurfaceSlab>,
}
// These are QUALIFIED std::min/max (first operand on false), not Objexx double min.
fn std_min(a: f64, b: f64) -> f64 {
    if b < a { b } else { a }
}
fn std_max(a: f64, b: f64) -> f64 {
    if a < b { b } else { a }
}
fn comparison(a: f64, b: f64) -> Ordering {
    if a < b {
        Ordering::Less
    } else if b < a {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}
fn vertex(vertices: &[Option<[f64; 3]>], at: usize) -> Result<[f64; 3], Error> {
    vertices
        .get(at)
        .copied()
        .flatten()
        .ok_or(Error::Unavailable(
            "actual written Surface.Vertex coordinate in computed geometry",
        ))
}
impl FinalSurface {
    pub(super) fn set_source_computed_geometry(&mut self) -> Result<(), Error> {
        let Some(vertices) = self.row.vertex.as_deref() else {
            return Ok(());
        };
        if vertices.len() >= 3 {
            self.shape_cat = match self.row.shape {
                SourceProcessShape::Triangle => SourceComputedShapeCategory::Triangular,
                SourceProcessShape::Rectangle => SourceComputedShapeCategory::Rectangular,
                _ if self.row.is_convex => SourceComputedShapeCategory::Convex,
                _ => SourceComputedShapeCategory::Nonconvex,
            }; //193, before any coordinate read in computed_plane.
            self.plane = computed_plane(vertices)?; //194, only after whole local plane returns.
            self.surface_2d = computed_surface_2d(self.shape_cat, self.plane, vertices)?; //195
        }
        Ok(())
    }
}
impl SurfaceGeometryInputState {
    pub(crate) fn set_source_computed_surface_geometry(&mut self) -> Result<(), Error> {
        // Genuine Array1D traversal, including a native constructor-unallocated/empty array.
        for surface in self.final_surfaces.as_mut().into_iter().flatten() {
            surface.set_source_computed_geometry()?;
        }
        Ok(())
    }
}
fn computed_plane(vertices: &[Option<[f64; 3]>]) -> Result<[f64; 4], Error> {
    let n = vertices.len();
    let mut center = [0.0; 3];
    let (mut a, mut b, mut c) = (0.0, 0.0, 0.0);
    for i in 0..n {
        let v = vertex(vertices, i)?;
        let w = vertex(vertices, (i + 1) % n)?;
        a += (v[1] - w[1]) * (v[2] + w[2]);
        b += (v[2] - w[2]) * (v[0] + w[0]);
        c += (v[0] - w[0]) * (v[1] + w[1]);
        center[0] += v[0];
        center[1] += v[1];
        center[2] += v[2];
    }
    let d = -((center[0] * a + center[1] * b + center[2] * c) / (n as f64));
    Ok([a, b, c, d])
}
fn computed_surface_2d(
    category: SourceComputedShapeCategory,
    plane: [f64; 4],
    vertices: &[Option<[f64; 3]>],
) -> Result<SourceComputedSurface2D, Error> {
    // Actual retained Native DataSurfaces compile row ends -UNDEBUG: this
    // recomputation/equality assertion is live, before projection-axis reads420.
    if plane != computed_plane(vertices)? {
        return Err(Error::Unavailable(
            "actual active assert plane==computed_plane420 domain",
        ));
    }
    let (a, b, c) = (plane[0].abs(), plane[1].abs(), plane[2].abs());
    let axis = if a >= std_max(b, c) {
        0
    } else if b >= std_max(a, c) {
        1
    } else {
        2
    };
    let v0 = vertex(vertices, 0)?;
    let (x, y) = match axis {
        0 => (1, 2),
        1 => (0, 2),
        _ => (0, 1),
    };
    let (mut xl, mut xu, mut yl, mut yu) = (v0[x], v0[x], v0[y], v0[y]);
    let mut projected = Vec::with_capacity(vertices.len());
    for i in 0..vertices.len() {
        let v = vertex(vertices, i)?;
        projected.push([v[x], v[y]]);
        xl = std_min(xl, v[x]);
        xu = std_max(xu, v[x]);
        yl = std_min(yl, v[y]);
        yu = std_max(yu, v[y]);
    }
    SourceComputedSurface2D::construct(category, axis, projected, [xl, yl], [xu, yu])
}
impl SourceComputedSurface2D {
    fn construct(
        category: SourceComputedShapeCategory,
        axis: i32,
        vertices: Vec<[f64; 2]>,
        vl: [f64; 2],
        vu: [f64; 2],
    ) -> Result<Self, Error> {
        let n = vertices.len();
        // Initializer order axis/vertices/vl/vu, with the remaining native defaults.
        let mut result = Self {
            axis,
            vertices: Some(vertices),
            vl,
            vu,
            ..Self::default()
        };
        let vertices = result
            .vertices
            .as_deref_mut()
            .ok_or(Error::Unavailable("actual Surface2D vertices constructor"))?;
        let mut area = 0.0;
        for i in 0..n {
            let v = vertices[i];
            let w = vertices[(i + 1) % n];
            area += (v[0] * w[1]) - (w[0] * v[1]);
        }
        if area < 0.0 {
            vertices[1..].reverse();
        }
        // Array1D reserve(n) sets real allocated data with size0 before pushes.
        let edges = result.edges.insert(Vec::with_capacity(n));
        for i in 0..n {
            let v = vertices[i];
            let w = vertices[(i + 1) % n];
            edges.push([w[0] - v[0], w[1] - v[1]]);
        }
        if category == SourceComputedShapeCategory::Rectangular {
            if n != 4 {
                return Err(Error::Unavailable(
                    "actual rectangular Surface2D assert n==4 defined domain120",
                ));
            }
            result.s1 = (edges[0][0] * edges[0][0]) + (edges[0][1] * edges[0][1]);
            result.s3 = (edges[3][0] * edges[3][0]) + (edges[3][1] * edges[3][1]);
        } else if category == SourceComputedShapeCategory::Nonconvex || n >= 20 {
            if n < 4 {
                return Err(Error::Unavailable(
                    "actual slab Surface2D assert n>=4 defined domain125",
                ));
            }
            result.slab_ys.reserve(n);
            for v in vertices.iter() {
                result.slab_ys.push(v[1]);
            }
            // Native std::sort's tie permutation is implementation-dependent. Equal
            // scalar bits are harmless; mixed +/-0 and unordered values need actual
            // platform permutation authority before any sorted writer is certified.
            let has_positive_zero = result
                .slab_ys
                .iter()
                .any(|v| *v == 0.0 && !v.is_sign_negative());
            let has_negative_zero = result
                .slab_ys
                .iter()
                .any(|v| *v == 0.0 && v.is_sign_negative());
            if result.slab_ys.iter().any(|v| !v.is_finite())
                || (has_positive_zero && has_negative_zero)
            {
                return Err(Error::Unavailable(
                    "actual Surface2D slabYs std::sort unordered/mixed-zero permutation132",
                ));
            }
            result.slab_ys.sort_unstable_by(|a, b| comparison(*a, *b));
            result.slab_ys.dedup_by(|a, b| *a == *b);
            result.slab_ys.shrink_to_fit();
            if result.slab_ys.is_empty() {
                return Err(Error::Unavailable(
                    "actual slabYs.size()-1 unsigned loop domain135",
                ));
            }
            for at in 0..result.slab_ys.len() - 1 {
                let (mut xl, mut xu) = (f64::MAX, f64::MIN);
                let (yl, yu) = (result.slab_ys[at], result.slab_ys[at + 1]);
                result.slabs.push(SourceComputedSurfaceSlab {
                    xl: 0.0,
                    xu: 0.0,
                    yl,
                    yu,
                    edges: Vec::new(),
                    edges_xy: Vec::new(),
                });
                let slab = result
                    .slabs
                    .last_mut()
                    .ok_or(Error::Unavailable("actual appended Surface2DSlab"))?;
                let mut cross_edges = Vec::new();
                for i in 0..n {
                    let v = vertices[i];
                    let w = vertices[(i + 1) % n];
                    if (v[1] <= yl && yu <= w[1]) || (yu <= v[1] && w[1] <= yl) {
                        let e = edges[i];
                        if e[1] == 0.0 {
                            return Err(Error::Unavailable(
                                "actual active crossing-edge assert e.y!=0 before division149 domain",
                            ));
                        }
                        let exy = e[0] / e[1];
                        let xb = v[0] + (yl - v[1]) * exy;
                        let xt = v[0] + (yu - v[1]) * exy;
                        xl = std_min(xl, std_min(xb, xt));
                        xu = std_max(xu, std_max(xb, xt));
                        cross_edges.push((xb, xt, i));
                    }
                }
                slab.xl = xl;
                slab.xu = xu;
                if cross_edges.len() < 2 {
                    return Err(Error::Unavailable(
                        "actual Surface2D crossEdges assert size>=2 defined domain161",
                    ));
                }
                for (i, a) in cross_edges.iter().enumerate() {
                    let am = a.0 + a.1;
                    if !am.is_finite() || cross_edges[..i].iter().any(|b| b.0 + b.1 == am) {
                        return Err(Error::Unavailable(
                            "actual Surface2D crossEdges std::sort tied/unordered midpoint permutation162",
                        ));
                    }
                }
                cross_edges.sort_unstable_by(|a, b| comparison(a.0 + a.1, b.0 + b.1));
                // Native162--176 debug-only order checks are ACTIVE under the
                // authenticated -UNDEBUG baseline. No upstream simplicity is inferred.
                let mut xb = cross_edges[0].0;
                let mut xt = cross_edges[0].1;
                let tol = 1.0e-9 * std_max(xl.abs(), xu.abs());
                for edge in &cross_edges {
                    let xbe = edge.0;
                    let xte = edge.1;
                    if !(xb <= xbe + tol) {
                        return Err(Error::Unavailable(
                            "actual active assert xbot<=next_xbot+tol171 domain",
                        ));
                    }
                    if !(xt <= xte + tol) {
                        return Err(Error::Unavailable(
                            "actual active assert xtop<=next_xtop+tol172 domain",
                        ));
                    }
                    xb = xbe;
                    xt = xte;
                }
                if category != SourceComputedShapeCategory::Nonconvex && cross_edges.len() != 2 {
                    return Err(Error::Unavailable(
                        "actual convex slab crossEdges assert size==2 defined domain177",
                    ));
                }
                for edge in cross_edges {
                    let i_edge = edge.2;
                    slab.edges.push(i_edge);
                    let e = edges[i_edge];
                    if e[1] == 0.0 {
                        return Err(Error::Unavailable(
                            "actual active stored-edge assert e.y!=0 after index push183 domain",
                        ));
                    }
                    slab.edges_xy
                        .push(if e[1] != 0.0 { e[0] / e[1] } else { 0.0 });
                }
                if slab.edges.len() % 2 != 0 {
                    return Err(Error::Unavailable(
                        "actual active assert slab.edges.size even185 domain",
                    ));
                }
                if slab.edges.len() != slab.edges_xy.len() {
                    return Err(Error::Unavailable(
                        "actual active assert edge/slopes sizes equal186 domain",
                    ));
                }
            }
        }
        Ok(result)
    }
}
