//! Native PierceSurface.hh99--368 on the same retained computed SurfaceData.
use super::Error;
use crate::heat_balance::selected_heat_balance_input::surface::{
    FinalSurface,
    computed_geometry::{
        SourceComputedShapeCategory, SourceComputedSurface2D, SourceComputedSurfaceSlab,
    },
};

// DataSurfaces.hh468: dispatch threshold, not an authored geometry restriction.
const N_VERTICES_BIG: usize = 20;

fn vertex(s2d: &SourceComputedSurface2D, at: usize) -> Result<[f64; 2], Error> {
    s2d.vertices
        .as_deref()
        .and_then(|values| values.get(at))
        .copied()
        .ok_or(Error::Unavailable("PierceSurface actual Surface2D vertex"))
}

fn edge(s2d: &SourceComputedSurface2D, at: usize) -> Result<[f64; 2], Error> {
    s2d.edges
        .as_deref()
        .and_then(|values| values.get(at))
        .copied()
        .ok_or(Error::Unavailable("PierceSurface actual Surface2D edge"))
}

fn cross_at(s2d: &SourceComputedSurface2D, h2d: [f64; 2], at: usize) -> Result<f64, Error> {
    let e = edge(s2d, at)?;
    let v = vertex(s2d, at)?;
    let difference = [h2d[0] - v[0], h2d[1] - v[1]];
    Ok((e[0] * difference[1]) - (e[1] * difference[0]))
}

// Native99--126: three edge tests in ascending order, accepting zero crosses.
fn triangular(s2d: &SourceComputedSurface2D, h2d: [f64; 2]) -> Result<bool, Error> {
    for at in 0..3 {
        if cross_at(s2d, h2d, at)? < 0.0 {
            return Ok(false);
        }
    }
    Ok(true)
}

// Native128--189: switch fallthrough checks3..8 in descending edge order.
fn convex(s2d: &SourceComputedSurface2D, h2d: [f64; 2]) -> Result<bool, Error> {
    let n = s2d.vertices.as_ref().map_or(0, Vec::len);
    if n < 3 {
        return Err(Error::Unavailable(
            "PierceSurface active convex n>=3 domain",
        ));
    }
    if n <= 8 {
        for at in (0..n).rev() {
            if cross_at(s2d, h2d, at)? < 0.0 {
                return Ok(false);
            }
        }
    } else {
        for at in 0..n {
            if cross_at(s2d, h2d, at)? < 0.0 {
                return Ok(false);
            }
        }
    }
    Ok(true)
}

// Native223--269 repeats this exact edge-index/slope/vertex/edge-read sequence.
fn slab_edge_x(
    s2d: &SourceComputedSurface2D,
    slab: &SourceComputedSurfaceSlab,
    at: usize,
    y_hit: f64,
) -> Result<f64, Error> {
    let i = *slab
        .edges
        .get(at)
        .ok_or(Error::Unavailable("PierceSurface actual slab edge index"))?;
    let slope = *slab
        .edges_xy
        .get(at)
        .ok_or(Error::Unavailable("PierceSurface actual slab edge slope"))?;
    let v = vertex(s2d, i)?;
    let _e = edge(s2d, i)?;
    Ok(v[0] + (y_hit - v[1]) * slope)
}

// Native191--283, using the reached producer's own slabs, never recomputing them.
#[allow(
    clippy::manual_midpoint,
    reason = "Retain Native's explicit unsigned (l+u)/2 expression."
)]
#[allow(
    clippy::manual_range_contains,
    reason = "Retain Native comparison order and unordered-coordinate branches."
)]
fn nonconvex(s2d: &SourceComputedSurface2D, h2d: [f64; 2]) -> Result<bool, Error> {
    if s2d.vertices.as_ref().map_or(0, Vec::len) < 3 {
        return Err(Error::Unavailable("PierceSurface active slab n>=3 domain"));
    }
    let slab_ys = &s2d.slab_ys;
    if slab_ys.is_empty() {
        return Err(Error::Unavailable(
            "PierceSurface active nonempty slabYs domain",
        ));
    }
    let y_hit = h2d[1];
    // std::lower_bound compares each stored ordinate < yHit.
    let i_hit = slab_ys.partition_point(|value| *value < y_hit);
    if !(y_hit >= slab_ys[0] && y_hit <= slab_ys[slab_ys.len() - 1]) {
        return Err(Error::Unavailable(
            "PierceSurface active slab y-range domain",
        ));
    }
    if i_hit == slab_ys.len() {
        return Err(Error::Unavailable(
            "PierceSurface active lower_bound!=end domain",
        ));
    }
    // Native decrements the iterator before subtracting begin. At begin that
    // operation is undefined; do not invent a clamped bottom-slab selection.
    let preceding = i_hit.checked_sub(1).ok_or(Error::Unavailable(
        "PierceSurface undefined lower_bound-before-begin slab selection",
    ))?;
    let i_slab = preceding.min(s2d.slabs.len());
    let slab = s2d
        .slabs
        .get(i_slab)
        .ok_or(Error::Unavailable("PierceSurface actual selected slab"))?;
    let x_hit = h2d[0];
    if x_hit < slab.xl || x_hit > slab.xu {
        return Ok(false);
    }
    let n_edges = slab.edges.len();
    if n_edges < 2 {
        return Err(Error::Unavailable(
            "PierceSurface active slab edges>=2 domain",
        ));
    }
    if n_edges == 2 {
        let x0 = slab_edge_x(s2d, slab, 0, y_hit)?;
        if x_hit < x0 {
            return Ok(false);
        }
        let x1 = slab_edge_x(s2d, slab, 1, y_hit)?;
        if x1 < x_hit {
            return Ok(false);
        }
    } else {
        if n_edges < 4 {
            return Err(Error::Unavailable(
                "PierceSurface active slab edges>=4 domain",
            ));
        }
        if !n_edges.is_multiple_of(2) {
            return Err(Error::Unavailable(
                "PierceSurface active even slab edge count domain",
            ));
        }
        let mut lower = 0;
        let mut upper = n_edges - 1;
        let xl = slab_edge_x(s2d, slab, lower, y_hit)?;
        if x_hit < xl {
            return Ok(false);
        }
        let xu = slab_edge_x(s2d, slab, upper, y_hit)?;
        if xu < x_hit {
            return Ok(false);
        }
        while upper - lower > 1 {
            let middle = (lower + upper) / 2;
            let xm = slab_edge_x(s2d, slab, middle, y_hit)?;
            if x_hit <= xm {
                upper = middle;
            } else {
                lower = middle;
            }
        }
        if upper - lower != 1 {
            return Err(Error::Unavailable(
                "PierceSurface active adjacent slab edges domain",
            ));
        }
        if upper.is_multiple_of(2) {
            return Ok(false);
        }
    }
    Ok(true)
}

// Native286--332: bounding box precedes shape dispatch and every polygon read.
#[allow(
    clippy::manual_range_contains,
    reason = "Native's ordered disjunctions differ from range contains for NaN."
)]
fn polygon(surface: &FinalSurface, hit_pt: [f64; 3]) -> Result<bool, Error> {
    let s2d = &surface.surface_2d;
    let axis = s2d.axis;
    let h2d = [
        if axis == 0 { hit_pt[1] } else { hit_pt[0] },
        if axis == 2 { hit_pt[1] } else { hit_pt[2] },
    ];
    if h2d[0] < s2d.vl[0] || s2d.vu[0] < h2d[0] || h2d[1] < s2d.vl[1] || s2d.vu[1] < h2d[1] {
        return Ok(false);
    }
    if surface.shape_cat == SourceComputedShapeCategory::Rectangular {
        let v0 = vertex(s2d, 0)?;
        let v0h = [h2d[0] - v0[0], h2d[1] - v0[1]];
        let e0 = edge(s2d, 0)?;
        let he1 = (v0h[0] * e0[0]) + (v0h[1] * e0[1]);
        if he1 < 0.0 || he1 > s2d.s1 {
            return Ok(false);
        }
        let e3 = edge(s2d, 3)?;
        let he3 = -((v0h[0] * e3[0]) + (v0h[1] * e3[1]));
        if he3 < 0.0 || he3 > s2d.s3 {
            return Ok(false);
        }
        return Ok(true);
    }
    if surface.shape_cat == SourceComputedShapeCategory::Triangular {
        return triangular(s2d, h2d);
    }
    if surface.shape_cat == SourceComputedShapeCategory::Nonconvex
        || s2d.vertices.as_ref().map_or(0, Vec::len) >= N_VERTICES_BIG
    {
        return nonconvex(s2d, h2d);
    }
    if surface.shape_cat == SourceComputedShapeCategory::Convex {
        return convex(s2d, h2d);
    }
    Ok(false)
}

// Native335--368: false plane tests leave hitPt untouched. Polygon misses retain
// the newly written plane intersection, in x/y/z statement order.
pub(super) fn pierce_surface(
    surface: &FinalSurface,
    ray_ori: [f64; 3],
    ray_dir: [f64; 3],
    hit_pt: &mut [f64; 3],
) -> Result<bool, Error> {
    let plane = surface.plane;
    let den = (plane[0] * ray_dir[0]) + (plane[1] * ray_dir[1]) + (plane[2] * ray_dir[2]);
    if den == 0.0 {
        return Ok(false);
    }
    let num =
        -((plane[0] * ray_ori[0]) + (plane[1] * ray_ori[1]) + (plane[2] * ray_ori[2]) + plane[3]);
    if num * den <= 0.0 {
        return Ok(false);
    }
    let t = num / den;
    hit_pt[0] = ray_ori[0] + (t * ray_dir[0]);
    hit_pt[1] = ray_ori[1] + (t * ray_dir[1]);
    hit_pt[2] = ray_ori[2] + (t * ray_dir[2]);
    polygon(surface, *hit_pt)
}
