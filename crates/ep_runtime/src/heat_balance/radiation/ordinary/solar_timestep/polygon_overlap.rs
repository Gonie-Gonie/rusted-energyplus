//! Pinned SolarShading.cc3796-4884 polygon children; same persistent HC owners.
//! All Native tuned HC linear loops are logical [figure, vertex] here, because
//! Native Array2 is second-index-fastest and SourceSolarArray has another layout.
use super::array::SourceSolarArray;
use super::shadow_driver::source_hc_nint64 as polygon_nint64;
use super::*;
use crate::heat_balance::error_backend::SourceDiagnosticFlags;
use crate::heat_balance::selected_heat_balance_input::shadowing::ShadowingSystemSettings;

pub(super) const NO_OVERLAP: i32 = 1;
pub(super) const FIRST_SURF_WITHIN_SECOND: i32 = 2;
pub(super) const SECOND_SURF_WITHIN_FIRST: i32 = 3;
pub(super) const PARTIAL_OVERLAP: i32 = 4;
pub(super) const TOO_MANY_VERTICES: i32 = 5;
pub(super) const TOO_MANY_FIGURES: i32 = 6;

fn r<T: Copy>(a: &SourceSolarArray<T, 1>, i: i32) -> Result<T, Error> {
    a.read([ordinal(i)?])
}
fn w<T: Copy>(a: &mut SourceSolarArray<T, 1>, i: i32, v: T) -> Result<(), Error> {
    a.set([ordinal(i)?], v)
}
fn h(a: &SourceSolarArray<i64, 2>, figure: usize, vertex: i32) -> Result<i64, Error> {
    a.read([figure, ordinal(vertex)?])
}
fn hw(a: &mut SourceSolarArray<i64, 2>, figure: usize, vertex: i32, v: i64) -> Result<(), Error> {
    a.set([figure, ordinal(vertex)?], v)
}
fn integer_add(a: i64, b: i64) -> Result<i64, Error> {
    a.checked_add(b).ok_or(Error::Unavailable(
        "Native polygon signed i64 addition domain",
    ))
}
fn integer_sub(a: i64, b: i64) -> Result<i64, Error> {
    a.checked_sub(b).ok_or(Error::Unavailable(
        "Native polygon signed i64 subtraction domain",
    ))
}
fn integer_mul(a: i64, b: i64) -> Result<i64, Error> {
    a.checked_mul(b).ok_or(Error::Unavailable(
        "Native polygon signed i64 multiplication domain",
    ))
}
fn integer_cross(a: i64, b: i64, c: i64, d: i64) -> Result<i64, Error> {
    integer_sub(integer_mul(a, b)?, integer_mul(c, d)?)
}
fn integer_hfunct(x: i64, a: i64, y: i64, b: i64, c: i64) -> Result<i64, Error> {
    integer_add(integer_add(integer_mul(x, a)?, integer_mul(y, b)?)?, c)
}
// General.cc904-915, sign(SMALL,b) compares b>=0, including negative zero.
fn safe_divide(a: f64, b: f64) -> f64 {
    if b.abs() >= 1.0e-10 {
        a / b
    } else {
        a / if b >= 0.0 { 1.0e-10 } else { -1.0e-10 }
    }
}
fn redimension(a: &mut SourceSolarArray<f64, 1>, n: usize) -> Result<(), Error> {
    // Array1D.hh1088: size==0 (including constructor-unallocated) dimensions
    // with the supplied initializer; existing elements otherwise stay intact.
    a.values.resize(n, Some(0.0));
    a.dimensions = Some([n]);
    Ok(())
}
fn size(a: &SourceSolarArray<f64, 1>) -> Result<usize, Error> {
    Ok(a.dimensions.map_or(0, |dimensions| dimensions[0]))
}
fn neq(a: f64, b: f64) -> bool {
    (a - b).abs() > 2.0
}
fn d_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < 2.0
}

// SolarShading.cc3987-4080: rejection preserves any swap performed before it.
fn clipline(
    x0: &mut f64,
    x1: &mut f64,
    y0: &mut f64,
    y1: &mut f64,
    max_x: f64,
    min_x: f64,
    max_y: f64,
    min_y: f64,
    visible: &mut bool,
) {
    let mut rev = false;
    if *x0 > *x1 {
        std::mem::swap(x0, x1);
        std::mem::swap(y0, y1);
        rev = true;
    }
    if *x0 > max_x || *x1 < min_x {
        return;
    }
    let dx = *x1 - *x0;
    let dy = *y1 - *y0;
    let q1 = *x0 - min_x;
    let q2 = max_x - *x0;
    let q3 = *y0 - min_y;
    let q4 = max_y - *y0;
    let mut u1 = 0.0;
    let mut u2 = 1.0;
    if (dx == 0.0 && (q1 < 0.0 || q2 < 0.0)) || (dy == 0.0 && (q3 < 0.0 || q4 < 0.0)) {
        return;
    }
    if dx != 0.0 {
        let r1 = q1 / -dx;
        if r1 > u1 {
            u1 = r1;
        }
        let r2 = q2 / dx;
        if r2 < u2 {
            u2 = r2;
        }
    }
    if dy != 0.0 {
        let r3 = q3 / -dy;
        let r4 = q4 / dy;
        if dy > 0.0 {
            if r3 > u1 {
                u1 = r3;
            }
            if r4 < u2 {
                u2 = r4;
            }
        } else {
            if r4 > u1 {
                u1 = r4;
            }
            if r3 < u2 {
                u2 = r3;
            }
        }
    }
    if u1 > u2 {
        return;
    }
    *visible = true;
    let xn0 = *x0 + dx * u1;
    let yn0 = *y0 + dy * u1;
    let xn1 = *x0 + dx * u2;
    let yn1 = *y0 + dy * u2;
    if rev {
        *x0 = xn1;
        *y0 = yn1;
        *x1 = xn0;
        *y1 = yn0;
    } else {
        *x0 = xn0;
        *y0 = yn0;
        *x1 = xn1;
        *y1 = yn1;
    }
}

impl SourceSolarTimestepState {
    fn polygon_remove_near_duplicate(&self, nv: &mut i32, previous: i32) -> Result<(), Error> {
        if previous != 0 {
            let x = r(&self.shading.xtemp, *nv)?;
            let y = r(&self.shading.ytemp, *nv)?;
            for k in 1..=previous {
                if (x - r(&self.shading.xtemp, k)?).abs() > 2.0 {
                    continue;
                }
                if (y - r(&self.shading.ytemp, k)?).abs() > 2.0 {
                    continue;
                }
                *nv = previous;
                break;
            }
        }
        Ok(())
    }

    // SolarShading.cc3796-3871: enclosed count increments before duplicate check.
    fn source_inclos(
        &mut self,
        n1: usize,
        n1_vertices: i32,
        n2: usize,
        n2_vertices: i32,
        overlap_vertices: &mut i32,
        nin: &mut i32,
    ) -> Result<(), Error> {
        *nin = 0;
        for n in 1..=n1_vertices {
            let mut cycle = false;
            for m in 1..=n2_vertices {
                let hfunct = integer_hfunct(
                    h(&self.hc_x, n1, n)?,
                    h(&self.hc_a, n2, m)?,
                    h(&self.hc_y, n1, n)?,
                    h(&self.hc_b, n2, m)?,
                    h(&self.hc_c, n2, m)?,
                )? as f64;
                if hfunct > 0.0 {
                    cycle = true;
                    break;
                }
            }
            if cycle {
                continue;
            }
            *nin += 1;
            if *overlap_vertices != 0 {
                for k in 1..=*overlap_vertices {
                    if r(&self.shading.xtemp, k)? == h(&self.hc_x, n1, n)? as f64
                        && r(&self.shading.ytemp, k)? == h(&self.hc_y, n1, n)? as f64
                    {
                        cycle = true;
                        break;
                    }
                }
                if cycle {
                    continue;
                }
            }
            *overlap_vertices += 1;
            w(
                &mut self.shading.xtemp,
                *overlap_vertices,
                h(&self.hc_x, n1, n)? as f64,
            )?;
            w(
                &mut self.shading.ytemp,
                *overlap_vertices,
                h(&self.hc_y, n1, n)? as f64,
            )?;
        }
        Ok(())
    }

    // SolarShading.cc3873-3974: integer numerators convert once, then division.
    fn source_intcpt(
        &mut self,
        nv1: i32,
        nv2: i32,
        nv3: &mut i32,
        ns1: usize,
        ns2: usize,
    ) -> Result<(), Error> {
        for n in 1..=nv1 {
            for m in 1..=nv2 {
                let i1 = integer_hfunct(
                    h(&self.hc_a, ns1, n)?,
                    h(&self.hc_x, ns2, m)?,
                    h(&self.hc_b, ns1, n)?,
                    h(&self.hc_y, ns2, m)?,
                    h(&self.hc_c, ns1, n)?,
                )?;
                let i2 = integer_hfunct(
                    h(&self.hc_a, ns1, n)?,
                    h(&self.hc_x, ns2, m + 1)?,
                    h(&self.hc_b, ns1, n)?,
                    h(&self.hc_y, ns2, m + 1)?,
                    h(&self.hc_c, ns1, n)?,
                )?;
                if i1 >= 0 && i2 >= 0 {
                    continue;
                }
                if i1 <= 0 && i2 <= 0 {
                    continue;
                }
                let i1 = integer_hfunct(
                    h(&self.hc_a, ns2, m)?,
                    h(&self.hc_x, ns1, n)?,
                    h(&self.hc_b, ns2, m)?,
                    h(&self.hc_y, ns1, n)?,
                    h(&self.hc_c, ns2, m)?,
                )?;
                let i2 = integer_hfunct(
                    h(&self.hc_a, ns2, m)?,
                    h(&self.hc_x, ns1, n + 1)?,
                    h(&self.hc_b, ns2, m)?,
                    h(&self.hc_y, ns1, n + 1)?,
                    h(&self.hc_c, ns2, m)?,
                )?;
                if i1 >= 0 && i2 >= 0 {
                    continue;
                }
                if i1 <= 0 && i2 <= 0 {
                    continue;
                }
                let kk = *nv3;
                *nv3 += 1;
                let normal = integer_cross(
                    h(&self.hc_b, ns2, m)?,
                    h(&self.hc_a, ns1, n)?,
                    h(&self.hc_a, ns2, m)?,
                    h(&self.hc_b, ns1, n)?,
                )? as f64;
                let x_untrunc = integer_cross(
                    h(&self.hc_c, ns2, m)?,
                    h(&self.hc_b, ns1, n)?,
                    h(&self.hc_b, ns2, m)?,
                    h(&self.hc_c, ns1, n)?,
                )? as f64
                    / normal;
                let y_untrunc = integer_cross(
                    h(&self.hc_a, ns2, m)?,
                    h(&self.hc_c, ns1, n)?,
                    h(&self.hc_c, ns2, m)?,
                    h(&self.hc_a, ns1, n)?,
                )? as f64
                    / normal;
                if ordinal(*nv3)? > size(&self.shading.xtemp)? {
                    let new_size = size(&self.shading.xtemp)? + 10;
                    redimension(&mut self.shading.xtemp, new_size)?;
                    redimension(&mut self.shading.ytemp, new_size)?;
                }
                w(
                    &mut self.shading.xtemp,
                    *nv3,
                    polygon_nint64(x_untrunc)? as f64,
                )?;
                w(
                    &mut self.shading.ytemp,
                    *nv3,
                    polygon_nint64(y_untrunc)? as f64,
                )?;
                self.polygon_remove_near_duplicate(nv3, kk)?;
            }
        }
        Ok(())
    }

    // SolarShading.cc4082-4309. arrx/arry are the Native fixed 20-value locals.
    fn source_cliprect(&mut self, ns2: usize, nv1: i32, nv3: &mut i32) -> Result<(), Error> {
        let mut intflag = false;
        let (max_x, min_x) = if h(&self.hc_x, ns2, 1)? > h(&self.hc_x, ns2, 3)? {
            (h(&self.hc_x, ns2, 1)? as f64, h(&self.hc_x, ns2, 3)? as f64)
        } else {
            (h(&self.hc_x, ns2, 3)? as f64, h(&self.hc_x, ns2, 1)? as f64)
        };
        let (max_y, min_y) = if h(&self.hc_y, ns2, 1)? > h(&self.hc_y, ns2, 3)? {
            (h(&self.hc_y, ns2, 1)? as f64, h(&self.hc_y, ns2, 3)? as f64)
        } else {
            (h(&self.hc_y, ns2, 3)? as f64, h(&self.hc_y, ns2, 1)? as f64)
        };
        let mut arrx = [None; 20];
        let mut arry = [None; 20];
        let mut arrc = 0_usize;
        for j in 0..nv1 {
            let mut x_1 = r(&self.shading.xtemp, j + 1)?;
            let mut y_1 = r(&self.shading.ytemp, j + 1)?;
            let mut x_2 = r(&self.shading.xtemp, (j + 1) % nv1 + 1)?;
            let mut y_2 = r(&self.shading.ytemp, (j + 1) % nv1 + 1)?;
            let (x1, x2, y1, y2) = (x_1, x_2, y_1, y_2);
            let mut visible = false;
            clipline(
                &mut x_1,
                &mut x_2,
                &mut y_1,
                &mut y_2,
                max_x,
                min_x,
                max_y,
                min_y,
                &mut visible,
            );
            if visible {
                if (x_1 != x1 || y_1 != y1) || (x_2 != x2 || y_2 != y2) {
                    intflag = true;
                }
                if arrc == 0
                    || ((neq(local_read(&arrx, arrc - 1)?, x_1)
                        || neq(local_read(&arry, arrc - 1)?, y_1))
                        && (neq(local_read(&arrx, 0)?, x_1) || neq(local_read(&arry, 0)?, y_1)))
                {
                    local_write(&mut arrx, arrc, x_1)?;
                    local_write(&mut arry, arrc, y_1)?;
                    arrc += 1;
                    if (neq(x_1, x_2) || neq(y_1, y_2))
                        && (neq(local_read(&arrx, 0)?, x_2) || neq(local_read(&arry, 0)?, y_2))
                    {
                        local_write(&mut arrx, arrc, x_2)?;
                        local_write(&mut arry, arrc, y_2)?;
                        arrc += 1;
                    }
                } else if (neq(local_read(&arrx, arrc - 1)?, x_2)
                    || neq(local_read(&arry, arrc - 1)?, y_2))
                    && (neq(local_read(&arrx, 0)?, x_2) || neq(local_read(&arry, 0)?, y_2))
                {
                    local_write(&mut arrx, arrc, x_2)?;
                    local_write(&mut arry, arrc, y_2)?;
                    arrc += 1;
                }
            }
        }
        *nv3 = arrc as i32;
        if *nv3 > 1 {
            let mut last_edge_index = -1_i32;
            let mut incr = 0_i32;
            let corner_xs = [min_x, min_x, max_x, max_x];
            let corner_ys = [min_y, max_y, max_y, min_y];
            let edges = [min_x, max_y, max_x, min_y];
            let mut last_edge = None;
            for i in 0..=arrc {
                let k = i % arrc;
                let curr_x = local_read(&arrx, k)?;
                let curr_y = local_read(&arry, k)?;
                let mut edge_count = 0;
                let mut edge_index = -1_i32;
                for (m, &edge) in edges.iter().enumerate() {
                    if (m % 2 == 0 && d_eq(curr_x, edge)) || (m % 2 == 1 && d_eq(curr_y, edge)) {
                        edge_count += 1;
                        edge_index = m as i32;
                    }
                }
                if edge_count == 0 {
                    if i != arrc {
                        w(&mut self.shading.xtemp, incr + 1, curr_x)?;
                        w(&mut self.shading.ytemp, incr + 1, curr_y)?;
                        incr += 1;
                    }
                    continue;
                }
                if edge_count > 1 {
                    edge_index = if d_eq(curr_x, min_x) {
                        if d_eq(curr_y, min_y) { 3 } else { 0 }
                    } else if d_eq(curr_y, max_y) {
                        1
                    } else {
                        2
                    };
                }
                if last_edge_index > -1 && edge_index > -1 && last_edge_index != edge_index {
                    let mut jump_count = 0;
                    if (edge_index == 0 && last_edge_index == 3)
                        || edge_index - last_edge_index == 1
                    {
                        jump_count = 1;
                    } else if edge_index % 2 == last_edge_index % 2 {
                        jump_count = 2;
                    } else if (edge_index == 3 && last_edge_index == 0)
                        || last_edge_index - edge_index == 1
                    {
                        jump_count = 3;
                    }
                    if jump_count > 0 {
                        let start_index = (last_edge_index + 1) % 4;
                        let mut added = 0;
                        let (last_edge_x, last_edge_y) = last_edge.ok_or(Error::Unavailable(
                            "Native CLIPRECT LastEdge initialized domain",
                        ))?;
                        for j1 in 0..jump_count {
                            let i1 = ((start_index + j1) % 4) as usize;
                            let corner_x = corner_xs[i1];
                            let corner_y = corner_ys[i1];
                            if corner_x == last_edge_x && corner_y == last_edge_y {
                                continue;
                            }
                            let mut inside = true;
                            for j in 0..nv1 {
                                if r(&self.shading.atemp, j + 1)? * corner_x
                                    + corner_y * r(&self.shading.btemp, j + 1)?
                                    + r(&self.shading.ctemp, j + 1)?
                                    > 0.0
                                {
                                    inside = false;
                                    break;
                                }
                            }
                            if inside
                                && (incr == 0
                                    || ((neq(corner_x, r(&self.shading.xtemp, incr)?)
                                        || neq(corner_y, r(&self.shading.ytemp, incr)?))
                                        && (neq(corner_x, r(&self.shading.xtemp, 1)?)
                                            || neq(corner_y, r(&self.shading.ytemp, 1)?))))
                            {
                                w(&mut self.shading.xtemp, incr + 1, corner_x)?;
                                w(&mut self.shading.ytemp, incr + 1, corner_y)?;
                                incr += 1;
                                added += 1;
                            }
                        }
                        if jump_count > 2 && added == jump_count && edge_count == 1 {
                            if i != arrc {
                                w(&mut self.shading.xtemp, incr + 1, curr_x)?;
                                w(&mut self.shading.ytemp, incr + 1, curr_y)?;
                                incr += 1;
                            }
                            break;
                        }
                    }
                }
                if i != arrc {
                    w(&mut self.shading.xtemp, incr + 1, curr_x)?;
                    w(&mut self.shading.ytemp, incr + 1, curr_y)?;
                    incr += 1;
                }
                last_edge_index = edge_index;
                last_edge = Some((curr_x, curr_y));
            }
            *nv3 = incr;
        } else {
            if *nv3 == 1 {
                w(&mut self.shading.xtemp, 1, local_read(&arrx, 0)?)?;
                w(&mut self.shading.ytemp, 1, local_read(&arry, 0)?)?;
            }
            if *nv3 == 0 {
                let corner_xs = [min_x, min_x, max_x, max_x];
                let corner_ys = [min_y, max_y, max_y, min_y];
                let corner_x = corner_xs[0];
                let corner_y = corner_ys[0];
                let mut inside = true;
                for j in 0..nv1 {
                    if r(&self.shading.atemp, j + 1)? * corner_x
                        + corner_y * r(&self.shading.btemp, j + 1)?
                        + r(&self.shading.ctemp, j + 1)?
                        >= 0.0
                    {
                        inside = false;
                        break;
                    }
                }
                if inside {
                    for i1 in 0..4 {
                        w(&mut self.shading.xtemp, i1 as i32 + 1, corner_xs[i1])?;
                        w(&mut self.shading.ytemp, i1 as i32 + 1, corner_ys[i1])?;
                    }
                    *nv3 = 4;
                    intflag = true;
                }
            }
        }
        if *nv3 > 0 {
            let x_0 = r(&self.shading.xtemp, 1)?;
            let y_0 = r(&self.shading.ytemp, 1)?;
            let mut xp_0 = x_0;
            let mut yp_0 = y_0;
            let mut next = None;
            for p in 0..*nv3 - 1 {
                let xp_1 = r(&self.shading.xtemp, p + 2)?;
                let yp_1 = r(&self.shading.ytemp, p + 2)?;
                w(&mut self.shading.atemp, p + 1, yp_0 - yp_1)?;
                w(&mut self.shading.btemp, p + 1, xp_1 - xp_0)?;
                w(&mut self.shading.ctemp, p + 1, xp_0 * yp_1 - yp_0 * xp_1)?;
                xp_0 = xp_1;
                yp_0 = yp_1;
                next = Some((xp_1, yp_1));
            }
            let (xp_1, yp_1) = next.ok_or(Error::Unavailable(
                "Native CLIPRECT NV3=1 reads uninitialized XP_1/YP_1",
            ))?;
            w(&mut self.shading.atemp, *nv3, yp_1 - y_0)?;
            w(&mut self.shading.btemp, *nv3, x_0 - xp_1)?;
            w(&mut self.shading.ctemp, *nv3, xp_1 * y_0 - yp_1 * x_0)?;
        }
        if *nv3 < 3 {
            self.overlap_status = Some(NO_OVERLAP);
        } else if !intflag {
            self.overlap_status = Some(FIRST_SURF_WITHIN_SECOND);
        }
        Ok(())
    }

    // SolarShading.cc4311-4560: real HC edge snapshots and reciprocal products.
    fn source_clippoly(
        &mut self,
        clipping: &ShadowingSystemSettings,
        max_vertices_per_surface: usize,
        ns1: usize,
        ns2: usize,
        nv1: i32,
        nv2: i32,
        nv3: &mut i32,
    ) -> Result<(), Error> {
        for j in 1..=nv1 {
            w(&mut self.shading.xtemp, j, h(&self.hc_x, ns1, j)? as f64)?;
            w(&mut self.shading.ytemp, j, h(&self.hc_y, ns1, j)? as f64)?;
            w(&mut self.shading.atemp, j, h(&self.hc_a, ns1, j)? as f64)?;
            w(&mut self.shading.btemp, j, h(&self.hc_b, ns1, j)? as f64)?;
            w(&mut self.shading.ctemp, j, h(&self.hc_c, ns1, j)? as f64)?;
        }
        let mut nvout = nv1;
        let mut intflag = false;
        let mut nvtemp = 0_i32;
        if clipping.slater_barsky {
            let rect = nv2 == 4
                && ((h(&self.hc_x, ns2, 1)? == h(&self.hc_x, ns2, 2)?
                    && h(&self.hc_y, ns2, 1)? != h(&self.hc_y, ns2, 2)?
                    && h(&self.hc_y, ns2, 3)? == h(&self.hc_y, ns2, 2)?
                    && h(&self.hc_y, ns2, 4)? == h(&self.hc_y, ns2, 1)?
                    && h(&self.hc_x, ns2, 3)? == h(&self.hc_x, ns2, 4)?)
                    || (h(&self.hc_y, ns2, 1)? == h(&self.hc_y, ns2, 2)?
                        && h(&self.hc_x, ns2, 1)? != h(&self.hc_x, ns2, 2)?
                        && h(&self.hc_x, ns2, 3)? == h(&self.hc_x, ns2, 2)?
                        && h(&self.hc_x, ns2, 4)? == h(&self.hc_x, ns2, 1)?
                        && h(&self.hc_y, ns2, 3)? == h(&self.hc_y, ns2, 4)?));
            if rect {
                return self.source_cliprect(ns2, nv1, nv3);
            }
        }
        for e in 1..=nv2 {
            for p in 1..=nvout {
                w(&mut self.shading.xtemp1, p, r(&self.shading.xtemp, p)?)?;
                w(&mut self.shading.ytemp1, p, r(&self.shading.ytemp, p)?)?;
            }
            let mut s = nvout;
            let hca_e = h(&self.hc_a, ns2, e)? as f64;
            let hcb_e = h(&self.hc_b, ns2, e)? as f64;
            let hcc_e = h(&self.hc_c, ns2, e)? as f64;
            let mut xtemp1_s = r(&self.shading.xtemp1, s)?;
            let mut ytemp1_s = r(&self.shading.ytemp1, s)?;
            for p in 1..=nvout {
                let xtemp1_p = r(&self.shading.xtemp1, p)?;
                let ytemp1_p = r(&self.shading.ytemp1, p)?;
                let hfunct = xtemp1_p * hca_e + ytemp1_p * hcb_e + hcc_e;
                if hfunct <= 0.0 {
                    let hfunct = xtemp1_s * hca_e + ytemp1_s * hcb_e + hcc_e;
                    if hfunct > 0.0 {
                        let kk = nvtemp;
                        nvtemp += 1;
                        self.polygon_clip_intersection(nvtemp, s, hca_e, hcb_e, hcc_e)?;
                        intflag = true;
                        if e == nv2 {
                            self.polygon_remove_near_duplicate(&mut nvtemp, kk)?;
                        }
                    }
                    let kk = nvtemp;
                    nvtemp += 1;
                    if nvtemp > self.max_hc_array_bounds {
                        let new_bounds = self
                            .max_hc_array_bounds
                            .checked_add(self.max_hc_array_increment)
                            .ok_or(Error::Unavailable(
                                "Native MAXHCArrayBounds signed increment domain",
                            ))?;
                        let n = usize::try_from(new_bounds)
                            .map_err(|_| Error::Unavailable("Native MAXHCArrayBounds extent"))?;
                        redimension(&mut self.shading.xtemp, n)?;
                        redimension(&mut self.shading.ytemp, n)?;
                        redimension(&mut self.shading.xtemp1, n)?;
                        redimension(&mut self.shading.ytemp1, n)?;
                        redimension(&mut self.shading.atemp, n)?;
                        redimension(&mut self.shading.btemp, n)?;
                        redimension(&mut self.shading.ctemp, n)?;
                        self.max_hc_array_bounds = new_bounds;
                    }
                    w(&mut self.shading.xtemp, nvtemp, xtemp1_p)?;
                    w(&mut self.shading.ytemp, nvtemp, ytemp1_p)?;
                    if e == nv2 {
                        self.polygon_remove_near_duplicate(&mut nvtemp, kk)?;
                    }
                } else {
                    let hfunct = xtemp1_s * hca_e + ytemp1_s * hcb_e + hcc_e;
                    if hfunct <= 0.0 {
                        let bound = i32::try_from(max_vertices_per_surface)
                            .ok()
                            .and_then(|v| v.checked_add(1))
                            .and_then(|v| v.checked_mul(2))
                            .ok_or(Error::Unavailable(
                                "Native MaxVerticesPerSurface clipping bound",
                            ))?;
                        if nvtemp < bound {
                            let kk = nvtemp;
                            nvtemp += 1;
                            self.polygon_clip_intersection(nvtemp, s, hca_e, hcb_e, hcc_e)?;
                            intflag = true;
                            if e == nv2 {
                                self.polygon_remove_near_duplicate(&mut nvtemp, kk)?;
                            }
                        }
                    }
                }
                s = p;
                xtemp1_s = xtemp1_p;
                ytemp1_s = ytemp1_p;
            }
            nvout = nvtemp;
            if nvout == 0 {
                break;
            }
            nvtemp = 0;
            if e != nv2 && nvout > 2 {
                let x_1 = r(&self.shading.xtemp, 1)?;
                let y_1 = r(&self.shading.ytemp, 1)?;
                let mut x_p = x_1;
                let mut y_p = y_1;
                let mut last = None;
                for p in 1..nvout {
                    let x_p1 = r(&self.shading.xtemp, p + 1)?;
                    let y_p1 = r(&self.shading.ytemp, p + 1)?;
                    w(&mut self.shading.atemp, p, y_p - y_p1)?;
                    w(&mut self.shading.btemp, p, x_p1 - x_p)?;
                    w(&mut self.shading.ctemp, p, x_p * y_p1 - y_p * x_p1)?;
                    x_p = x_p1;
                    y_p = y_p1;
                    last = Some((x_p1, y_p1));
                }
                let (x_p1, y_p1) =
                    last.ok_or(Error::Unavailable("Native CLIPPOLY output edge last point"))?;
                w(&mut self.shading.atemp, nvout, y_p1 - y_1)?;
                w(&mut self.shading.btemp, nvout, x_1 - x_p1)?;
                w(&mut self.shading.ctemp, nvout, x_p1 * y_1 - y_p1 * x_1)?;
            }
        }
        *nv3 = nvout;
        if *nv3 < 3 {
            self.overlap_status = Some(NO_OVERLAP);
        } else if !intflag {
            self.overlap_status = Some(FIRST_SURF_WITHIN_SECOND);
        }
        Ok(())
    }

    fn polygon_clip_intersection(
        &mut self,
        output: i32,
        s: i32,
        hca_e: f64,
        hcb_e: f64,
        hcc_e: f64,
    ) -> Result<(), Error> {
        let atemp_s = r(&self.shading.atemp, s)?;
        let btemp_s = r(&self.shading.btemp, s)?;
        let ctemp_s = r(&self.shading.ctemp, s)?;
        let normal = hcb_e * atemp_s - hca_e * btemp_s;
        if normal != 0.0 {
            let normal_inv = 1.0 / normal;
            w(
                &mut self.shading.xtemp,
                output,
                polygon_nint64((hcc_e * btemp_s - hcb_e * ctemp_s) * normal_inv)? as f64,
            )?;
            w(
                &mut self.shading.ytemp,
                output,
                polygon_nint64((hca_e * ctemp_s - hcc_e * atemp_s) * normal_inv)? as f64,
            )?;
        } else {
            w(
                &mut self.shading.xtemp,
                output,
                safe_divide(hcc_e * btemp_s - hcb_e * ctemp_s, normal),
            )?;
            w(
                &mut self.shading.ytemp,
                output,
                safe_divide(hca_e * ctemp_s - hcc_e * atemp_s, normal),
            )?;
        }
        Ok(())
    }

    // SolarShading.cc4608-4724. Preserve in-place compaction and bubble swaps.
    fn source_order(
        &mut self,
        nv3: i32,
        ns3: usize,
        max_vertices_per_surface: usize,
    ) -> Result<(), Error> {
        if self.order_first_time_flag {
            let vertices = i32::try_from(max_vertices_per_surface)
                .ok()
                .and_then(|v| v.checked_add(1))
                .ok_or(Error::Unavailable(
                    "Native ORDER MaxVerticesPerSurface+1 signed domain",
                ))?;
            self.slope
                .dimension([10_usize.max(ordinal(vertices)?)], None)?;
            self.order_first_time_flag = false;
        }
        let mut xmin = r(&self.shading.xtemp, 1)?;
        let mut yxmin = r(&self.shading.ytemp, 1)?;
        for n in 2..=nv3 {
            if r(&self.shading.xtemp, n)? >= xmin {
                continue;
            }
            xmin = r(&self.shading.xtemp, n)?;
            yxmin = r(&self.shading.ytemp, n)?;
        }
        let mut p = 1_i32;
        let mut m = 0_i32;
        for n in 1..=nv3 {
            let delta_x = r(&self.shading.xtemp, n)? - xmin;
            let delta_y = r(&self.shading.ytemp, n)? - yxmin;
            if delta_x.abs() > 0.5 {
                m += 1;
                w(&mut self.slope, m, delta_y / delta_x)?;
                let x_n = r(&self.shading.xtemp, n)?;
                w(&mut self.shading.xtemp, m, x_n)?;
                let y_n = r(&self.shading.ytemp, n)?;
                w(&mut self.shading.ytemp, m, y_n)?;
            } else if delta_y > 0.5 {
                p = 2;
                hw(
                    &mut self.hc_x,
                    ns3,
                    2,
                    polygon_nint64(r(&self.shading.xtemp, n)?)?,
                )?;
                hw(
                    &mut self.hc_y,
                    ns3,
                    2,
                    polygon_nint64(r(&self.shading.ytemp, n)?)?,
                )?;
            } else if delta_y < -0.5 {
                hw(
                    &mut self.hc_x,
                    ns3,
                    nv3,
                    polygon_nint64(r(&self.shading.xtemp, n)?)?,
                )?;
                hw(
                    &mut self.hc_y,
                    ns3,
                    nv3,
                    polygon_nint64(r(&self.shading.ytemp, n)?)?,
                )?;
            } else {
                hw(&mut self.hc_x, ns3, 1, polygon_nint64(xmin)?)?;
                hw(&mut self.hc_y, ns3, 1, polygon_nint64(yxmin)?)?;
            }
        }
        if m != 1 {
            for i in 2..=m {
                for j in 1..i {
                    if r(&self.slope, i)? <= r(&self.slope, j)? {
                        continue;
                    }
                    let save_x = r(&self.shading.xtemp, i)?;
                    let save_y = r(&self.shading.ytemp, i)?;
                    let save_s = r(&self.slope, i)?;
                    let x_j = r(&self.shading.xtemp, j)?;
                    w(&mut self.shading.xtemp, i, x_j)?;
                    let y_j = r(&self.shading.ytemp, j)?;
                    w(&mut self.shading.ytemp, i, y_j)?;
                    let slope_j = r(&self.slope, j)?;
                    w(&mut self.slope, i, slope_j)?;
                    w(&mut self.shading.xtemp, j, save_x)?;
                    w(&mut self.shading.ytemp, j, save_y)?;
                    w(&mut self.slope, j, save_s)?;
                }
            }
        }
        for n in 1..=m {
            hw(
                &mut self.hc_x,
                ns3,
                n + p,
                polygon_nint64(r(&self.shading.xtemp, n)?)?,
            )?;
            hw(
                &mut self.hc_y,
                ns3,
                n + p,
                polygon_nint64(r(&self.shading.ytemp, n)?)?,
            )?;
        }
        Ok(())
    }

    // SolarShading.cc4726-4884. Native limits use MaxHCS (not 2*MaxHCS).
    #[allow(clippy::too_many_arguments)]
    pub(super) fn source_determine_polygon_overlap(
        &mut self,
        clipping: &ShadowingSystemSettings,
        max_hcs: usize,
        max_vertices_per_surface: usize,
        display_extra_warnings: bool,
        backend: &mut SourceErrorBackend<'_>,
        flags: &SourceDiagnosticFlags<'_>,
        ns1: usize,
        ns2: usize,
        ns3: usize,
    ) -> Result<(), Error> {
        if ns3 > max_hcs {
            self.overlap_status = Some(TOO_MANY_FIGURES);
            if !self.too_many_figures_message && !display_extra_warnings {
                backend.show_source_warning_error(&format!("DeterminePolygonOverlap: Too many figures [>{max_hcs}]  detected in an overlap calculation. Use Output:Diagnostics,DisplayExtraWarnings; for more details."), flags)?;
                self.too_many_figures_message = true;
            }
            if display_extra_warnings {
                self.num_too_many_figures =
                    self.num_too_many_figures
                        .checked_add(1)
                        .ok_or(Error::Unavailable(
                            "Native NumTooManyFigures signed increment domain",
                        ))?;
                track(
                    &mut self.track_too_many_figures,
                    self.num_too_many_figures,
                    self.current_shadowing_surface,
                    self.current_surface_being_shadowed,
                )?;
            }
            return Ok(());
        }
        self.overlap_status = Some(PARTIAL_OVERLAP);
        let nv1 = self.hc_nv.read([ns1])?;
        let nv2 = self.hc_nv.read([ns2])?;
        let mut nv3 = 0;
        if !clipping.sutherland_hodgman {
            let mut nin1 = 0;
            self.source_inclos(ns1, nv1, ns2, nv2, &mut nv3, &mut nin1)?;
            if nin1 >= nv1 {
                self.overlap_status = Some(FIRST_SURF_WITHIN_SECOND);
            } else {
                let mut nin2 = 0;
                self.source_inclos(ns2, nv2, ns1, nv1, &mut nv3, &mut nin2)?;
                if nin2 >= nv2 {
                    self.overlap_status = Some(SECOND_SURF_WITHIN_FIRST);
                } else {
                    self.source_intcpt(nv1, nv2, &mut nv3, ns1, ns2)?;
                    if nv3 < 3 {
                        self.overlap_status = Some(NO_OVERLAP);
                        return Ok(());
                    }
                }
            }
        } else {
            self.source_clippoly(
                clipping,
                max_vertices_per_surface,
                ns1,
                ns2,
                nv1,
                nv2,
                &mut nv3,
            )?;
        }
        if nv3 < self.max_hcv {
            if !clipping.sutherland_hodgman {
                self.source_order(nv3, ns3, max_vertices_per_surface)?;
            } else {
                for n in 1..=nv3 {
                    hw(
                        &mut self.hc_x,
                        ns3,
                        n,
                        polygon_nint64(r(&self.shading.xtemp, n)?)?,
                    )?;
                    hw(
                        &mut self.hc_y,
                        ns3,
                        n,
                        polygon_nint64(r(&self.shading.ytemp, n)?)?,
                    )?;
                }
            }
            self.source_htrans0(ns3, nv3, max_hcs, backend, flags)?;
            if self.hc_area.read([ns3])?.abs() * 100000.0 < self.hc_area.read([ns1])?.abs() {
                self.overlap_status = Some(NO_OVERLAP);
            } else {
                if self.hc_area.read([ns1])? * self.hc_area.read([ns2])? > 0.0 {
                    self.hc_area.set([ns3], -self.hc_area.read([ns3])?)?;
                }
                let hct_1 = self.hc_t.read([ns1])?;
                let hct_2 = self.hc_t.read([ns2])?;
                if hct_2 == 1.0 || hct_1 == 1.0 {
                    self.hc_t.set([ns3], hct_1 * hct_2)?;
                } else {
                    self.hc_t.set([ns3], (hct_1 + hct_2) - hct_1 * hct_2)?;
                }
            }
        } else {
            self.overlap_status = Some(TOO_MANY_VERTICES);
            if !self.too_many_vertices_message && !display_extra_warnings {
                backend.show_source_warning_error(&format!("DeterminePolygonOverlap: Too many vertices [>{}] detected in an overlap calculation. Use Output:Diagnostics,DisplayExtraWarnings; for more details.", self.max_hcv), flags)?;
                self.too_many_vertices_message = true;
            }
            if display_extra_warnings {
                self.num_too_many_vertices =
                    self.num_too_many_vertices
                        .checked_add(1)
                        .ok_or(Error::Unavailable(
                            "Native NumTooManyVertices signed increment domain",
                        ))?;
                track(
                    &mut self.track_too_many_vertices,
                    self.num_too_many_vertices,
                    self.current_shadowing_surface,
                    self.current_surface_being_shadowed,
                )?;
            }
        }
        Ok(())
    }

    // SolarShading.cc4562-4606, including all prior overlap figures in order.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn source_multol(
        &mut self,
        clipping: &ShadowingSystemSettings,
        max_hcs: usize,
        max_vertices_per_surface: usize,
        display_extra_warnings: bool,
        backend: &mut SourceErrorBackend<'_>,
        flags: &SourceDiagnosticFlags<'_>,
        nnn: usize,
        loc0: i32,
        nrfigs: i32,
    ) -> Result<(), Error> {
        self.max_number_of_figures = self.max_number_of_figures.max(nrfigs);
        for i in 1..=nrfigs {
            let ns1 = ordinal(
                loc0.checked_add(i)
                    .ok_or(Error::Unavailable("Native MULTOL LOC0+I signed domain"))?,
            )?;
            let ns3_signed = self
                .lochca
                .checked_add(1)
                .ok_or(Error::Unavailable("Native MULTOL LOCHCA+1 signed domain"))?;
            let ns3 = ordinal(ns3_signed)?;
            self.source_determine_polygon_overlap(
                clipping,
                max_hcs,
                max_vertices_per_surface,
                display_extra_warnings,
                backend,
                flags,
                ns1,
                nnn,
                ns3,
            )?;
            if self.source_overlap_status()? == NO_OVERLAP {
                continue;
            }
            if self.source_overlap_status()? == TOO_MANY_VERTICES
                || self.source_overlap_status()? == TOO_MANY_FIGURES
            {
                break;
            }
            self.lochca = ns3_signed;
        }
        Ok(())
    }
}

fn local_read(a: &[Option<f64>; 20], i: usize) -> Result<f64, Error> {
    a.get(i).copied().flatten().ok_or(Error::Unavailable(
        "Native CLIPRECT defined 20-value local array element",
    ))
}
fn local_write(a: &mut [Option<f64>; 20], i: usize, value: f64) -> Result<(), Error> {
    *a.get_mut(i).ok_or(Error::Unavailable(
        "Native CLIPRECT defined 20-value local array bounds",
    ))? = Some(value);
    Ok(())
}
fn track(
    a: &mut Option<Vec<(Option<i32>, Option<i32>)>>,
    n: i32,
    first: Option<i32>,
    second: Option<i32>,
) -> Result<(), Error> {
    let n = ordinal(n)?;
    let rows = a.get_or_insert_with(Vec::new);
    rows.resize(n, (None, None));
    rows[n - 1].0 = Some(first.ok_or(Error::Unavailable(
        "Native warning tracker current shadowing surface unwritten",
    ))?);
    rows[n - 1].1 = Some(second.ok_or(Error::Unavailable(
        "Native warning tracker current receiving surface unwritten",
    ))?);
    Ok(())
}
