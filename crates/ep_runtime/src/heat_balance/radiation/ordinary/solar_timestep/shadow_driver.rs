//! SolarShading3433-3794,5737-6337: the real geometric shadow driver.
use super::array::SourceSolarArray;
use super::polygon_overlap::{
    FIRST_SURF_WITHIN_SECOND, NO_OVERLAP, PARTIAL_OVERLAP, SECOND_SURF_WITHIN_FIRST,
    TOO_MANY_FIGURES, TOO_MANY_VERTICES,
};
use super::*;
use crate::heat_balance::error_backend::SourceDiagnosticFlags;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::shadowing::{
    ShadowingSystemSettings, SourceShadowingInput,
};

const HC_MULT: f64 = 100000.0;
const SQ_HC_MULT_FAC: f64 = 0.5 / (HC_MULT * HC_MULT);

/// ObjexxFCL Fmath.hh615 nint64, including its binary64 addition before cast.
/// Rust's saturating cast must not silently replace an undefined Native cast.
pub(super) fn source_hc_nint64(value: f64) -> Result<i64, Error> {
    let biased = value + if value < 0.0 { -0.5 } else { 0.5 };
    if !biased.is_finite() || !(-9223372036854775808.0..9223372036854775808.0).contains(&biased) {
        return Err(Error::Unavailable("nint64 actual signed conversion domain"));
    }
    Ok(biased as i64)
}

fn signed(value: usize) -> Result<i32, Error> {
    i32::try_from(value).map_err(|_| Error::Unavailable("actual signed shadow ordinal"))
}

fn shadow_add(first: i32, second: i32) -> Result<i32, Error> {
    first
        .checked_add(second)
        .ok_or(Error::Unavailable("actual signed shadow addition domain"))
}
fn shadow_sub(first: i32, second: i32) -> Result<i32, Error> {
    first.checked_sub(second).ok_or(Error::Unavailable(
        "actual signed shadow subtraction domain",
    ))
}
fn shadow_scratch_extent(geometry: &SurfaceGeometryInputState) -> Result<usize, Error> {
    ordinal(shadow_add(
        signed(geometry.coordinate.max_vertices_per_surface)?,
        1,
    )?)
}
fn shadow_figure_bound(max_hcs: usize) -> Result<usize, Error> {
    usize::try_from(
        signed(max_hcs)?
            .checked_mul(2)
            .ok_or(Error::Unavailable("HTRANS signed figure bound domain"))?,
    )
    .map_err(|_| Error::Unavailable("HTRANS positive figure bound domain"))
}

/// CTRANS3529-3593. Each caller passes its actual distinct retained XYZ arrays.
fn source_ctrans(
    geometry: &SurfaceGeometryInputState,
    ns: usize,
    ngrs: usize,
    xvt: &mut SourceSolarArray<f64, 1>,
    yvt: &mut SourceSolarArray<f64, 1>,
    zvt: &mut SourceSolarArray<f64, 1>,
) -> Result<i32, Error> {
    let surface = source_surface(geometry, ns)?;
    let base = geometry_properties(geometry, ngrs)?;
    let arrays = geometry.parallel_arrays.as_ref().ok_or(Error::Unavailable(
        "CTRANS actual DataSurface parallel arrays",
    ))?;
    let x0 = *arrays
        .x0
        .get(ngrs - 1)
        .ok_or(Error::Unavailable("CTRANS actual X0"))?;
    let y0 = *arrays
        .y0
        .get(ngrs - 1)
        .ok_or(Error::Unavailable("CTRANS actual Y0"))?;
    let z0 = *arrays
        .z0
        .get(ngrs - 1)
        .ok_or(Error::Unavailable("CTRANS actual Z0"))?;
    let nvt = surface.sides;
    for n in 1..=nvt {
        let n = ordinal(n)?;
        let vertex = surface
            .vertex
            .as_ref()
            .and_then(|v| v.get(n - 1))
            .copied()
            .flatten()
            .ok_or(Error::Unavailable("CTRANS actual written Vertex"))?;
        let mut xdif = vertex[0] - x0;
        let mut ydif = vertex[1] - y0;
        let mut zdif = vertex[2] - z0;
        if xdif.abs() <= 1.0e-15 {
            xdif = 0.0;
        }
        if ydif.abs() <= 1.0e-15 {
            ydif = 0.0;
        }
        if zdif.abs() <= 1.0e-15 {
            zdif = 0.0;
        }
        xvt.set(
            [n],
            base.lcsx[0] * xdif + base.lcsx[1] * ydif + base.lcsx[2] * zdif,
        )?;
        yvt.set(
            [n],
            base.lcsy[0] * xdif + base.lcsy[1] * ydif + base.lcsy[2] * zdif,
        )?;
        zvt.set(
            [n],
            base.lcsz[0] * xdif + base.lcsz[1] * ydif + base.lcsz[2] * zdif,
        )?;
    }
    Ok(nvt)
}

impl SourceSolarTimestepState {
    pub(super) fn source_overlap_status(&self) -> Result<i32, Error> {
        self.overlap_status
            .ok_or(Error::Unavailable("actual written OverlapStatus"))
    }
    fn source_nvs(&self) -> Result<i32, Error> {
        self.nvs.ok_or(Error::Unavailable("actual written NVS"))
    }
    fn source_clipped_vertices(&self) -> Result<i32, Error> {
        self.num_vert_in_shadow_or_clipped_surface
            .ok_or(Error::Unavailable("actual written clipped vertex count"))
    }
    fn source_x_projection(&self) -> Result<f64, Error> {
        self.x_shadow_projection
            .ok_or(Error::Unavailable("actual written XShadowProjection"))
    }
    fn source_y_projection(&self) -> Result<f64, Error> {
        self.y_shadow_projection
            .ok_or(Error::Unavailable("actual written YShadowProjection"))
    }
    /// HTRANS0/1 share Native integer side arithmetic and real SUM accumulation.
    /// Native's tuned linear indices advance vertices. Logical [NS,N] is used
    /// here because SourceSolarArray's physical backing has a different stride.
    fn source_htrans_sides(&mut self, ns: usize, num_vertices: i32) -> Result<(), Error> {
        let extra = ordinal(shadow_add(num_vertices, 1)?)?;
        let mut x_m = self.hc_x.read([ns, 1])?;
        self.hc_x.set([ns, extra], x_m)?;
        let mut y_m = self.hc_y.read([ns, 1])?;
        self.hc_y.set([ns, extra], y_m)?;
        let mut sum = 0.0;
        for n in 1..=num_vertices {
            let n = ordinal(n)?;
            let x_l = x_m;
            let y_l = y_m;
            x_m = self.hc_x.read([ns, n + 1])?;
            y_m = self.hc_y.read([ns, n + 1])?;
            let a = y_l
                .checked_sub(y_m)
                .ok_or(Error::Unavailable("HTRANS signed HCA arithmetic"))?;
            self.hc_a.set([ns, n], a)?;
            let b = x_m
                .checked_sub(x_l)
                .ok_or(Error::Unavailable("HTRANS signed HCB arithmetic"))?;
            self.hc_b.set([ns, n], b)?;
            let c = y_m
                .checked_mul(x_l)
                .and_then(|first| {
                    x_m.checked_mul(y_l)
                        .and_then(|second| first.checked_sub(second))
                })
                .ok_or(Error::Unavailable("HTRANS signed HCC arithmetic"))?;
            self.hc_c.set([ns, n], c)?;
            sum += c as f64;
        }
        self.hc_area.set([ns], sum * SQ_HC_MULT_FAC)
    }

    /// SolarShading3699-3743. Fatal bounds precede HCNV, including on failure.
    pub(super) fn source_htrans0(
        &mut self,
        ns: usize,
        num_vertices: i32,
        max_hcs: usize,
        backend: &mut SourceErrorBackend<'_>,
        flags: &SourceDiagnosticFlags<'_>,
    ) -> Result<(), Error> {
        if ns > shadow_figure_bound(max_hcs)? {
            return match backend.show_source_fatal_error(
                &format!("Solar Shading: HTrans0: Too many Figures (>{max_hcs})"),
                flags,
            ) {
                Ok(never) => match never {},
                Err(error) => Err(error),
            };
        }
        self.hc_nv.set([ns], num_vertices)?;
        self.source_htrans_sides(ns, num_vertices)
    }

    /// SolarShading3745-3794; nint64 is applied before each actual cell write.
    pub(super) fn source_htrans1(
        &mut self,
        ns: usize,
        num_vertices: i32,
        max_hcs: usize,
        backend: &mut SourceErrorBackend<'_>,
        flags: &SourceDiagnosticFlags<'_>,
    ) -> Result<(), Error> {
        if ns > shadow_figure_bound(max_hcs)? {
            return match backend.show_source_fatal_error(
                &format!("Solar Shading: HTrans1: Too many Figures (>{max_hcs})"),
                flags,
            ) {
                Ok(never) => match never {},
                Err(error) => Err(error),
            };
        }
        self.hc_nv.set([ns], num_vertices)?;
        for n in 1..=num_vertices {
            let n = ordinal(n)?;
            self.hc_x.set(
                [ns, n],
                source_hc_nint64(self.shading.xvs.read([n])? * HC_MULT)?,
            )?;
            self.hc_y.set(
                [ns, n],
                source_hc_nint64(self.shading.yvs.read([n])? * HC_MULT)?,
            )?;
        }
        self.source_htrans_sides(ns, num_vertices)
    }

    /// CLIP3433-3527. The supplied XVert/YVert/ZVert arrays are retained owners,
    /// including the closing vertex writes performed only on a partial clip.
    fn source_clip_general_caster(&mut self, nvt: i32) -> Result<(), Error> {
        let mut nabove = 0;
        let mut next = 0;
        let mut non = 0;
        let mut xin = 0.0;
        let mut xout = 0.0;
        let mut yin = 0.0;
        let mut yout = 0.0;
        self.num_vert_in_shadow_or_clipped_surface = self.nvs;
        for n in 1..=nvt {
            let z = self.z_vert.read([ordinal(n)?])?;
            if z > 0.0 {
                nabove = shadow_add(nabove, 1)?;
            } else if z == 0.0 {
                non = shadow_add(non, 1)?;
            }
        }
        if shadow_add(nabove, non)? == nvt {
            self.nvs = Some(nvt);
            self.num_vert_in_shadow_or_clipped_surface = Some(nvt);
            for n in 1..=nvt {
                let n = ordinal(n)?;
                self.shading.xvc.set([n], self.x_vert.read([n])?)?;
                self.shading.yvc.set([n], self.y_vert.read([n])?)?;
                self.shading.zvc.set([n], self.z_vert.read([n])?)?;
            }
        } else if nabove == 0 {
            self.nvs = Some(0);
            self.num_vert_in_shadow_or_clipped_surface = Some(0);
        } else {
            self.nvs = Some(shadow_add(nabove, 2)?);
            self.num_vert_in_shadow_or_clipped_surface = Some(shadow_add(nabove, 2)?);
            let mut z_p = self.z_vert.read([1])?;
            let extra = ordinal(shadow_add(nvt, 1)?)?;
            self.x_vert.set([extra], self.x_vert.read([1])?)?;
            self.y_vert.set([extra], self.y_vert.read([1])?)?;
            self.z_vert.set([extra], z_p)?;
            for n in 1..=nvt {
                let p = ordinal(shadow_add(n, 1)?)?;
                let z_n = z_p;
                z_p = self.z_vert.read([p])?;
                let n = ordinal(n)?;
                if z_n >= 0.0 && z_p < 0.0 {
                    let factor = 1.0 / (z_p - z_n);
                    xin = (z_p * self.x_vert.read([n])? - z_n * self.x_vert.read([p])?) * factor;
                    yin = (z_p * self.y_vert.read([n])? - z_n * self.y_vert.read([p])?) * factor;
                } else if z_n <= 0.0 && z_p > 0.0 {
                    next = shadow_add(signed(n)?, 1)?;
                    let factor = 1.0 / (z_p - z_n);
                    xout = (z_p * self.x_vert.read([n])? - z_n * self.x_vert.read([p])?) * factor;
                    yout = (z_p * self.y_vert.read([n])? - z_n * self.y_vert.read([p])?) * factor;
                }
            }
            self.shading.xvc.set([1], xout)?;
            self.shading.yvc.set([1], yout)?;
            self.shading.zvc.set([1], 0.0)?;
            let last = ordinal(self.source_nvs()?)?;
            self.shading.xvc.set([last], xin)?;
            self.shading.yvc.set([last], yin)?;
            self.shading.zvc.set([last], 0.0)?;
            for n in 1..=nabove {
                if next > nvt {
                    next = 1;
                }
                let at = ordinal(shadow_add(n, 1)?)?;
                let from = ordinal(next)?;
                self.shading.xvc.set([at], self.x_vert.read([from])?)?;
                self.shading.yvc.set([at], self.y_vert.read([from])?)?;
                self.shading.zvc.set([at], self.z_vert.read([from])?)?;
                next = shadow_add(next, 1)?;
            }
        }
        Ok(())
    }

    /// SHDGSS6058-6337, with real independent caster sun threshold int(1e-5)=0.
    fn source_shdgss(
        &mut self,
        geometry: &SurfaceGeometryInputState,
        shadowing: &SourceShadowingInput,
        clipping: &ShadowingSystemSettings,
        hb_globals: &SelectedHeatBalanceGlobals,
        ngrs: usize,
        hour: usize,
        _step: usize,
        current_surface: usize,
        ngss: i32,
        hts: usize,
        backend: &mut SourceErrorBackend<'_>,
        flags: &SourceDiagnosticFlags<'_>,
    ) -> Result<(), Error> {
        if self.shdgss_one_time_flag {
            let vertices = shadow_scratch_extent(geometry)?;
            self.x_vert.dimension([vertices], Some(0.0))?;
            self.y_vert.dimension([vertices], Some(0.0))?;
            self.z_vert.dimension([vertices], Some(0.0))?;
            self.shdgss_one_time_flag = false;
        }
        self.fgsshc = shadow_add(self.lochca, 1)?;
        self.overlap_status = Some(NO_OVERLAP);
        if ngss <= 0 {
            self.shading
                .surf_sunlit_area
                .set([hts], self.hc_area.read([1])?)?;
        } else {
            let mut exit_loop_status = -1;
            for i in 1..=ngss {
                let gssnr = ordinal(
                    *self
                        .shadow_combinations
                        .as_ref()
                        .and_then(|rows| rows.get(current_surface - 1))
                        .and_then(|row| row.general.as_ref())
                        .and_then(|rows| rows.get(ordinal(i).ok()?))
                        .ok_or(Error::Unavailable("SHDGSS actual GenSurf cell"))?,
                )?;
                // Native declares int const sunIsUp = DataEnvironment::SunIsUpValue.
                if self.shading.surf_sun_cos_theta.read([gssnr])? > 0.0 {
                    continue;
                }
                let surface = source_surface(geometry, gssnr)?;
                if !surface.heat_transfer_surface {
                    // Positive authored shade getters, IsTransparent and schedule
                    // pointers have not been ported into this SurfaceData owner.
                    return Err(Error::Unavailable(
                        "SHDGSS authored shade transparency/schedule owners",
                    ));
                }
                if shadowing.disable_all_self_shading {
                    if surface.zone != 0 {
                        continue;
                    }
                } else if shadowing.disable_group_self_shading {
                    let disabled = geometry
                        .parallel_arrays
                        .as_ref()
                        .and_then(|arrays| {
                            arrays
                                .surf_shadow_disabled_zone_list
                                .get(current_surface - 1)
                        })
                        .ok_or(Error::Unavailable("SHDGSS actual disabled-zone list"))?;
                    if disabled.contains(&surface.zone) {
                        continue;
                    }
                }
                // General opaque caster: CTRANS then actual receiver-plane CLIP.
                let nvt = source_ctrans(
                    geometry,
                    gssnr,
                    ngrs,
                    &mut self.x_vert,
                    &mut self.y_vert,
                    &mut self.z_vert,
                )?;
                self.source_clip_general_caster(nvt)?;
                if self.source_clipped_vertices()? <= 2 {
                    continue;
                }
                for n in 1..=self.source_clipped_vertices()? {
                    let n = ordinal(n)?;
                    self.shading.xvs.set(
                        [n],
                        self.shading.xvc.read([n])?
                            - self.source_x_projection()? * self.shading.zvc.read([n])?,
                    )?;
                    self.shading.yvs.set(
                        [n],
                        self.shading.yvc.read([n])?
                            - self.source_y_projection()? * self.shading.zvc.read([n])?,
                    )?;
                }
                let ns3 = ordinal(shadow_add(self.lochca, 1)?)?;
                self.source_htrans1(ns3, self.source_nvs()?, shadowing.max_hcs, backend, flags)?;
                let nvr = self.hc_nv.read([1])?;
                for n in 1..=self.source_clipped_vertices()? {
                    let n = ordinal(n)?;
                    let x_n = self.hc_x.read([ns3, n])?;
                    let y_n = self.hc_y.read([ns3, n])?;
                    for l in 1..=nvr {
                        let l = ordinal(l)?;
                        // Int64 subtraction/abs precede Real64 conversion in Native.
                        let del_x = self
                            .hc_x
                            .read([1, l])?
                            .checked_sub(x_n)
                            .and_then(i64::checked_abs)
                            .ok_or(Error::Unavailable("SHDGSS signed snap X arithmetic"))?
                            as f64;
                        if del_x > 6.0 {
                            continue;
                        }
                        let del_y = self
                            .hc_y
                            .read([1, l])?
                            .checked_sub(y_n)
                            .and_then(i64::checked_abs)
                            .ok_or(Error::Unavailable("SHDGSS signed snap Y arithmetic"))?
                            as f64;
                        if del_y > 6.0 {
                            continue;
                        }
                        if del_x > 0.0 {
                            self.hc_x.set([ns3, n], self.hc_x.read([1, l])?)?;
                        }
                        if del_y > 0.0 {
                            self.hc_y.set([ns3, n], self.hc_y.read([1, l])?)?;
                        }
                        break;
                    }
                }
                self.source_htrans0(
                    ns3,
                    self.source_clipped_vertices()?,
                    shadowing.max_hcs,
                    backend,
                    flags,
                )?;
                // For real opaque HeatTransSurf, shadowSurfSched is constructor
                // nullptr; the actual row's SchedMinValue remains the operand.
                let sch_value = surface.sched_min_value;
                self.hc_t.set([ns3], sch_value)?;
                self.current_shadowing_surface = Some(i);
                self.current_surface_being_shadowed = Some(signed(gssnr)?);
                self.source_determine_polygon_overlap(
                    clipping,
                    shadowing.max_hcs,
                    geometry.coordinate.max_vertices_per_surface,
                    hb_globals.display_extra_warnings,
                    backend,
                    flags,
                    1,
                    ns3,
                    ns3,
                )?;
                if self.source_overlap_status()? == FIRST_SURF_WITHIN_SECOND && sch_value > 0.0 {
                    self.overlap_status = Some(PARTIAL_OVERLAP);
                }
                let main_overlap_status = self.source_overlap_status()?;
                exit_loop_status = main_overlap_status;
                if main_overlap_status == NO_OVERLAP {
                } else if main_overlap_status == FIRST_SURF_WITHIN_SECOND
                    || main_overlap_status == TOO_MANY_VERTICES
                    || main_overlap_status == TOO_MANY_FIGURES
                {
                    break;
                } else if main_overlap_status == SECOND_SURF_WITHIN_FIRST
                    || main_overlap_status == PARTIAL_OVERLAP
                {
                    self.lochca = signed(ns3)?;
                    self.ngsshc = shadow_add(shadow_sub(self.lochca, self.fgsshc)?, 1)?;
                    if self.ngsshc > 1 {
                        self.source_multol(
                            clipping,
                            shadowing.max_hcs,
                            geometry.coordinate.max_vertices_per_surface,
                            hb_globals.display_extra_warnings,
                            backend,
                            flags,
                            ordinal(self.lochca)?,
                            shadow_sub(self.fgsshc, 1)?,
                            shadow_sub(self.ngsshc, 1)?,
                        )?;
                    }
                } else {
                    break;
                }
                exit_loop_status = -1;
            }
            if exit_loop_status == FIRST_SURF_WITHIN_SECOND {
                self.shading.surf_sunlit_area.set([hts], 0.0)?;
                self.lochca = self.fgsshc;
            } else if exit_loop_status == TOO_MANY_VERTICES || exit_loop_status == TOO_MANY_FIGURES
            {
                // This estimate is Native6297, with the Native status/warning path.
                self.shading
                    .surf_sunlit_area
                    .set([hts], 0.25 * self.hc_area.read([1])?)?;
            } else {
                self.ngsshc = shadow_add(shadow_sub(self.lochca, self.fgsshc)?, 1)?;
                if self.ngsshc <= 0 {
                    self.shading
                        .surf_sunlit_area
                        .set([hts], self.hc_area.read([1])?)?;
                } else {
                    let mut area = self.hc_area.read([1])?;
                    for i in self.fgsshc..=shadow_sub(shadow_add(self.fgsshc, self.ngsshc)?, 1)? {
                        let i = ordinal(i)?;
                        area += self.hc_area.read([i])? * (1.0 - self.hc_t.read([i])?);
                    }
                    self.shading.surf_sunlit_area.set([hts], area)?;
                    if self.shading.surf_sunlit_area.read([hts])? <= 0.0 {
                        self.shading.surf_sunlit_area.set([hts], 0.0)?;
                        self.lochca = self.fgsshc;
                    }
                }
            }
        }
        self.ngsshc = shadow_add(shadow_sub(self.lochca, self.fgsshc)?, 1)?;
        let _ = hour; // Authored schedule value branches remain an explicit fence.
        Ok(())
    }

    /// SHDBKS5928-6056, including the allocation before the zero-NBKS return.
    fn source_shdbks(
        &mut self,
        geometry: &SurfaceGeometryInputState,
        shadowing: &SourceShadowingInput,
        clipping: &ShadowingSystemSettings,
        hb_globals: &SelectedHeatBalanceGlobals,
        ngrs: i32,
        current_surface: usize,
        nbks: i32,
        hts: usize,
        backend: &mut SourceErrorBackend<'_>,
        flags: &SourceDiagnosticFlags<'_>,
    ) -> Result<(), Error> {
        if self.shdbks_one_time_flag {
            let vertices = shadow_scratch_extent(geometry)?;
            self.x_vrtx.dimension([vertices], None)?;
            self.y_vrtx.dimension([vertices], None)?;
            self.z_vrtx.dimension([vertices], None)?;
            self.x_vrtx.fill(0.0)?;
            self.y_vrtx.fill(0.0)?;
            self.z_vrtx.fill(0.0)?;
            self.shdbks_one_time_flag = false;
        }
        if nbks <= 0
            || self.shading.surf_sunlit_area.read([hts])? <= 0.0
            || self.source_overlap_status()? == TOO_MANY_VERTICES
            || self.source_overlap_status()? == TOO_MANY_FIGURES
        {
            return Ok(());
        }
        self.fbkshc = shadow_add(self.lochca, 1)?;
        for i in 1..=nbks {
            let back = ordinal(
                *self
                    .shadow_combinations
                    .as_ref()
                    .and_then(|rows| rows.get(current_surface - 1))
                    .and_then(|row| row.back.as_ref())
                    .and_then(|rows| rows.get(ordinal(i).ok()?))
                    .ok_or(Error::Unavailable("SHDBKS actual BackSurf cell"))?,
            )?;
            if self.shading.surf_sun_cos_theta.read([back])? > -0.00001 {
                continue;
            }
            let nvt = source_ctrans(
                geometry,
                back,
                ordinal(ngrs)?,
                &mut self.x_vrtx,
                &mut self.y_vrtx,
                &mut self.z_vrtx,
            )?;
            for n in 1..=nvt {
                let n = ordinal(n)?;
                self.shading.xvs.set(
                    [n],
                    self.x_vrtx.read([n])? - self.source_x_projection()? * self.z_vrtx.read([n])?,
                )?;
                self.shading.yvs.set(
                    [n],
                    self.y_vrtx.read([n])? - self.source_y_projection()? * self.z_vrtx.read([n])?,
                )?;
            }
            let ns3 = ordinal(shadow_add(self.lochca, 1)?)?;
            self.hc_t.set([ns3], 0.0)?;
            self.source_htrans1(ns3, nvt, shadowing.max_hcs, backend, flags)?;
            let nvr = self.hc_nv.read([1])?;
            for n in 1..=nvt {
                let n = ordinal(n)?;
                // SHDBKS uses Real64 x3/y3; its subtraction order differs from SHDGSS.
                let x3 = self.hc_x.read([ns3, n])? as f64;
                let y3 = self.hc_y.read([ns3, n])? as f64;
                for m in 1..=nvr {
                    let m = ordinal(m)?;
                    if (self.hc_x.read([1, m])? as f64 - x3).abs() > 6.0 {
                        continue;
                    }
                    if (self.hc_y.read([1, m])? as f64 - y3).abs() > 6.0 {
                        continue;
                    }
                    self.hc_x.set([ns3, n], self.hc_x.read([1, m])?)?;
                    self.hc_y.set([ns3, n], self.hc_y.read([1, m])?)?;
                    break;
                }
            }
            self.source_htrans0(ns3, nvt, shadowing.max_hcs, backend, flags)?;
            self.hc_t.set([ns3], 1.0)?;
            self.source_determine_polygon_overlap(
                clipping,
                shadowing.max_hcs,
                geometry.coordinate.max_vertices_per_surface,
                hb_globals.display_extra_warnings,
                backend,
                flags,
                1,
                ns3,
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
            self.lochca = signed(ns3)?;
            self.hc_ns.set([ordinal(self.lochca)?], signed(back)?)?;
            self.hc_area.set(
                [ordinal(self.lochca)?],
                -self.hc_area.read([ordinal(self.lochca)?])?,
            )?;
            self.nbkshc = shadow_add(shadow_sub(self.lochca, self.fbkshc)?, 1)?;
        }
        Ok(())
    }
}

impl OrdinarySolarRadiation {
    /// SHADOW5737-5926. No case identity or expected sunlit-area trajectory enters.
    pub(crate) fn source_shadow(
        &mut self,
        geometry: &SurfaceGeometryInputState,
        shadowing: &SourceShadowingInput,
        clipping: &ShadowingSystemSettings,
        hb_globals: &SelectedHeatBalanceGlobals,
        hour: usize,
        step: usize,
        backend: &mut SourceErrorBackend<'_>,
        flags: &SourceDiagnosticFlags<'_>,
    ) -> Result<(), Error> {
        let state = &mut self.solar_timestep;
        if state.shadow_one_time_flag {
            let vertices = shadow_scratch_extent(geometry)?;
            state.x_vrt.dimension([vertices], None)?;
            state.y_vrt.dimension([vertices], None)?;
            state.z_vrt.dimension([vertices], None)?;
            state.x_vrt.fill(0.0)?;
            state.y_vrt.fill(0.0)?;
            state.z_vrt.fill(0.0)?;
            state.shadow_one_time_flag = false;
        }
        state.shading.surf_sunlit_area.fill(0.0)?;
        for grsnr in 1..=surface_count(geometry)? {
            let combination = state
                .shadow_combinations
                .as_ref()
                .and_then(|rows| rows.get(grsnr - 1))
                .ok_or(Error::Unavailable("actual ShadowComb allocation"))?;
            if !combination.use_this_surface {
                continue;
            }
            state.shading.surf_sunlit_area.set([grsnr], 0.0)?;
            let ngss = combination.num_gen_surface;
            state.ngsshc = 0;
            let nbks = combination.num_back_surface;
            state.nbkshc = 0;
            let nsbs = combination.num_sub_surface;
            state.nrvlhc = 0;
            state.nsbshc = 0;
            state.lochca = 1;
            let hts = grsnr;
            if state.shading.surf_sun_cos_theta.read([grsnr])? < 0.00001 {
                state.shading.surf_sunlit_area.set([hts], 0.0)?;
                if nsbs > 0 {
                    return Err(Error::Unavailable("SHDSBS reached positive NSBS"));
                }
            } else if ngss <= 0 && nsbs <= 0 {
                state
                    .shading
                    .surf_sunlit_area
                    .set([hts], source_surface(geometry, grsnr)?.net_area_shadow_calc)?;
            } else {
                let receiving = source_surface(geometry, grsnr)?;
                let ngrs = if receiving.is_shadowing {
                    grsnr
                } else {
                    ordinal(receiving.base_surface)?
                };
                let base = geometry_properties(geometry, ngrs)?;
                let sun = [
                    state.sun_cosines[0].ok_or(Error::Unavailable("SHADOW actual SUNCOS1"))?,
                    state.sun_cosines[1].ok_or(Error::Unavailable("SHADOW actual SUNCOS2"))?,
                    state.sun_cosines[2].ok_or(Error::Unavailable("SHADOW actual SUNCOS3"))?,
                ];
                let xs = base.lcsx[0] * sun[0] + base.lcsx[1] * sun[1] + base.lcsx[2] * sun[2];
                let ys = base.lcsy[0] * sun[0] + base.lcsy[1] * sun[1] + base.lcsy[2] * sun[2];
                let zs = base.lcsz[0] * sun[0] + base.lcsz[1] * sun[1] + base.lcsz[2] * sun[2];
                if zs.abs() > 1.0e-4 {
                    // Constant::SmallDistance
                    state.x_shadow_projection = Some(xs / zs);
                    state.y_shadow_projection = Some(ys / zs);
                    if state.source_x_projection()?.abs() < 1.0e-8 {
                        state.x_shadow_projection = Some(0.0);
                    }
                    if state.source_y_projection()?.abs() < 1.0e-8 {
                        state.y_shadow_projection = Some(0.0);
                    }
                } else {
                    state.x_shadow_projection = Some(0.0);
                    state.y_shadow_projection = Some(0.0);
                }
                let nvt = source_ctrans(
                    geometry,
                    grsnr,
                    ngrs,
                    &mut state.x_vrt,
                    &mut state.y_vrt,
                    &mut state.z_vrt,
                )?;
                for n in 1..=nvt {
                    let from = ordinal(shadow_sub(shadow_add(nvt, 1)?, n)?)?;
                    let n = ordinal(n)?;
                    state.shading.xvs.set([n], state.x_vrt.read([from])?)?;
                    state.shading.yvs.set([n], state.y_vrt.read([from])?)?;
                }
                state.source_htrans1(1, nvt, shadowing.max_hcs, backend, flags)?;
                state.hc_area.set([1], -state.hc_area.read([1])?)?;
                state.hc_t.set([1], 1.0)?;
                state.source_shdgss(
                    geometry, shadowing, clipping, hb_globals, ngrs, hour, step, grsnr, ngss, hts,
                    backend, flags,
                )?;
                if !state.calc_sky_dif_shading {
                    state.source_shdbks(
                        geometry,
                        shadowing,
                        clipping,
                        hb_globals,
                        receiving.base_surface,
                        grsnr,
                        nbks,
                        hts,
                        backend,
                        flags,
                    )?;
                }
                if nsbs > 0 {
                    return Err(Error::Unavailable("SHDSBS reached positive NSBS"));
                }
                let area = source_surface(geometry, grsnr)?.net_area_shadow_calc;
                // Native max/min retain their sequential writes.
                let sunlit = state.shading.surf_sunlit_area.read([hts])?;
                state
                    .shading
                    .surf_sunlit_area
                    .set([hts], if 0.0 < sunlit { sunlit } else { 0.0 })?;
                let sunlit = state.shading.surf_sunlit_area.read([hts])?;
                state
                    .shading
                    .surf_sunlit_area
                    .set([hts], if area < sunlit { area } else { sunlit })?;
            }
        }
        Ok(())
    }
}
