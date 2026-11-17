//! WindowManager.cc7213-7410/7666-8347 and4237-4365, actual Blind producer.
//! Persistent cells are the same MaterialBlind row; local arrays use Native
//! one-based indices. Matrix f[col][row] represents Native F(row,col), so its
//! first Native index varies fastest, including y(j,i)=tmp(i) in the inverse.
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError;
use crate::heat_balance::selected_material_data::{
    SOURCE_BLIND_MAX_PROFILE_ANGLES, SOURCE_BLIND_MAX_SLAT_ANGLES, SelectedMaterialData,
    SourceBlindMaterial, SourceMaterialGroup, unavailable,
};
use ep_platform_math::MathProviderError;
use std::f64::consts::PI;

// Error transport only: no numeric/provider lookup, cache, fallback or Native IO.
fn provider_error(operation: &str, error: MathProviderError) -> HeatBalanceInputError {
    HeatBalanceInputError::SourceInput(Box::new(unavailable(&format!(
        "WindowManager Blind {operation}: {error}"
    ))))
}

/// Native CalcWindowBlindProperties on the actual ordered material map rows.
#[allow(
    clippy::needless_range_loop,
    reason = "Preserve Native one-based column-major index and operation order"
)]
pub(crate) fn calc_window_blind_properties(
    materials: &mut SelectedMaterialData,
    backend: &mut SourceErrorBackend<'_>,
) -> Result<(), HeatBalanceInputError> {
    // Native local15/16/37 slots, plus unused index0 for direct index mapping.
    let mut bld_pr = [0.0; 16];
    let mut st_lay = [0.0; 17];
    let mut sun_el_deg = [0.0; 38];
    for index in 0..materials.materials.len() {
        let typed_id = materials.materials[index];
        if materials.material_groups.get(&typed_id) != Some(&SourceMaterialGroup::Blind) {
            continue;
        }
        let source_number = i32::try_from(index + 1)
            .map_err(|_| HeatBalanceInputError::Unavailable("actual Blind material ordinal"))?;
        let blind = materials.source_blind_material_mut(source_number, typed_id)?;
        bld_pr[2] = blind.slat_width;
        bld_pr[3] = blind.slat_separation;
        for i_sol_vis in 1..=2 {
            if i_sol_vis == 1 {
                bld_pr[4] = 0.0;
                bld_pr[5] = 0.0;
                bld_pr[6] = 0.0;
                bld_pr[7] = blind.slat_tar.sol.ft.bm[0].df_tra;
                bld_pr[8] = blind.slat_tar.sol.ft.bm[0].df_ref;
                bld_pr[9] = blind.slat_tar.sol.bk.bm[0].df_ref;
                bld_pr[10] = blind.slat_tar.sol.ft.df.tra;
                bld_pr[11] = blind.slat_tar.sol.ft.df.refl;
                bld_pr[12] = blind.slat_tar.sol.bk.df.refl;
            } else {
                bld_pr[4] = 0.0;
                bld_pr[5] = 0.0;
                bld_pr[6] = 0.0;
                bld_pr[7] = blind.slat_tar.vis.ft.bm[0].df_tra;
                bld_pr[8] = blind.slat_tar.vis.ft.bm[0].df_ref;
                bld_pr[9] = blind.slat_tar.vis.bk.bm[0].df_ref;
                bld_pr[10] = blind.slat_tar.vis.ft.df.tra;
                bld_pr[11] = blind.slat_tar.vis.ft.df.refl;
                bld_pr[12] = blind.slat_tar.vis.bk.df.refl;
            }
            bld_pr[13] = blind.slat_tar.ir.ft.tra;
            bld_pr[14] = blind.slat_tar.ir.ft.emi;
            bld_pr[15] = blind.slat_tar.ir.bk.emi;
            for i_slat_ang in 0..SOURCE_BLIND_MAX_SLAT_ANGLES {
                st_lay.fill(0.0);
                let bld_el = (PI / 180.0) * i_slat_ang as f64;
                blind_optics_diffuse(blind, i_sol_vis, &bld_pr, bld_el, &mut st_lay, backend)?;
                {
                    let btar = &mut blind.tars[i_slat_ang];
                    if i_sol_vis == 1 {
                        btar.sol.ft.df.tra = st_lay[9];
                        btar.sol.ft.df.refl = st_lay[10];
                        btar.sol.bk.df.tra = st_lay[11];
                        btar.sol.bk.df.refl = st_lay[12];
                        let value = 1.0 - st_lay[9] - st_lay[10];
                        btar.sol.ft.df.abs = if 0.0 < value { value } else { 0.0 };
                        let value = 1.0 - st_lay[11] - st_lay[12];
                        btar.sol.bk.df.abs = if 0.0 < value { value } else { 0.0 };
                        btar.ir.ft.tra = st_lay[13];
                        btar.ir.ft.emi = st_lay[14];
                        btar.ir.bk.tra = st_lay[13];
                        btar.ir.bk.emi = st_lay[15];
                    } else {
                        btar.vis.ft.df.tra = st_lay[9];
                        btar.vis.ft.df.refl = st_lay[10];
                        btar.vis.bk.df.tra = st_lay[11];
                        btar.vis.bk.df.refl = st_lay[12];
                    }
                }
                for i_prof_ang in 1..=SOURCE_BLIND_MAX_PROFILE_ANGLES {
                    let sun_el = -PI / 2.0 + (PI / 36.0) * (i_prof_ang - 1) as f64;
                    sun_el_deg[i_prof_ang] = 57.2958 * sun_el;
                    blind_optics_beam(blind, &bld_pr, bld_el, sun_el, &mut st_lay, backend)?;
                    let btar = &mut blind.tars[i_slat_ang];
                    if i_sol_vis == 1 {
                        btar.sol.ft.bm[i_prof_ang].bm_tra = st_lay[1];
                        btar.sol.ft.bm[i_prof_ang].bm_ref = st_lay[2];
                        btar.sol.bk.bm[i_prof_ang].bm_tra = st_lay[3];
                        btar.sol.bk.bm[i_prof_ang].bm_ref = st_lay[4];
                        btar.sol.ft.bm[i_prof_ang].df_tra = st_lay[5];
                        btar.sol.ft.bm[i_prof_ang].df_ref = st_lay[6];
                        btar.sol.bk.bm[i_prof_ang].df_tra = st_lay[7];
                        btar.sol.bk.bm[i_prof_ang].df_ref = st_lay[8];
                        let value = 1.0 - st_lay[6] - st_lay[1] - st_lay[5];
                        btar.sol.ft.bm[i_prof_ang].abs = if 0.0 < value { value } else { 0.0 };
                        let value = 1.0 - st_lay[7] - st_lay[3] - st_lay[8];
                        btar.sol.bk.bm[i_prof_ang].abs = if 0.0 < value { value } else { 0.0 };
                    } else {
                        btar.vis.ft.bm[i_prof_ang].bm_tra = st_lay[1];
                        btar.vis.ft.bm[i_prof_ang].bm_ref = st_lay[2];
                        btar.vis.bk.bm[i_prof_ang].bm_tra = st_lay[3];
                        btar.vis.bk.bm[i_prof_ang].bm_ref = st_lay[4];
                        btar.vis.ft.bm[i_prof_ang].df_tra = st_lay[5];
                        btar.vis.ft.bm[i_prof_ang].df_ref = st_lay[6];
                        btar.vis.bk.bm[i_prof_ang].df_tra = st_lay[7];
                        btar.vis.bk.bm[i_prof_ang].df_ref = st_lay[8];
                    }
                }
            }
            if i_sol_vis == 1 {
                for i_slat_ang in 0..SOURCE_BLIND_MAX_SLAT_ANGLES {
                    let btar = &mut blind.tars[i_slat_ang];
                    let mut sum_denom = 0.0;
                    let mut sum_tra1 = 0.0;
                    let mut sum_tra2 = 0.0;
                    let mut sum_ref = 0.0;
                    let mut sum_abs = 0.0;
                    for i_phi in 1..=18 {
                        let btargs = &btar.sol.ft.bm[i_phi];
                        let btargs1 = &btar.sol.ft.bm[i_phi + 1];
                        let denom = (PI / 36.0)
                            * ep_platform_math::cos(-PI / 2.0 + (i_phi as f64 - 0.5) * (PI / 36.0))
                                .map_err(|error| {
                                    provider_error("CalcWindowBlindProperties cos(ground)", error)
                                })?;
                        sum_denom += denom;
                        sum_tra1 += denom * (btargs.bm_tra + btargs1.bm_tra) * 0.5;
                        sum_tra2 += denom * (btargs.df_tra + btargs1.df_tra) * 0.5;
                        sum_ref += denom * (btargs.df_ref + btargs1.df_ref) * 0.5;
                        sum_abs += denom * (btargs.abs + btargs1.abs) * 0.5;
                    }
                    let value1 = sum_tra1 / sum_denom;
                    let value2 = sum_tra2 / sum_denom;
                    btar.sol.ft.df.tra_gnd = (if 0.0 < value1 { value1 } else { 0.0 })
                        + (if 0.0 < value2 { value2 } else { 0.0 });
                    let value = sum_ref / sum_denom;
                    btar.sol.ft.df.refl_gnd = if 0.0 < value { value } else { 0.0 };
                    let value = sum_abs / sum_denom;
                    btar.sol.ft.df.abs_gnd = if 0.0 < value { value } else { 0.0 };
                    sum_abs = 0.0;
                    sum_ref = 0.0;
                    sum_tra2 = 0.0;
                    sum_tra1 = 0.0;
                    sum_denom = 0.0;
                    for i_phi in 19..=SOURCE_BLIND_MAX_PROFILE_ANGLES - 1 {
                        let btargs = &btar.sol.ft.bm[i_phi];
                        let btargs1 = &btar.sol.ft.bm[i_phi + 1];
                        let denom = (PI / 36.0)
                            * ep_platform_math::cos(-PI / 2.0 + (i_phi as f64 - 0.5) * (PI / 36.0))
                                .map_err(|error| {
                                    provider_error("CalcWindowBlindProperties cos(sky)", error)
                                })?;
                        sum_denom += denom;
                        sum_tra1 += denom * (btargs.bm_tra + btargs1.bm_tra) * 0.5;
                        sum_tra2 += denom * (btargs.df_tra + btargs1.df_tra) * 0.5;
                        sum_ref += denom * (btargs.df_ref + btargs1.df_ref) * 0.5;
                        sum_abs += denom * (btargs.abs + btargs1.abs) * 0.5;
                    }
                    let value1 = sum_tra1 / sum_denom;
                    let value2 = sum_tra2 / sum_denom;
                    btar.sol.ft.df.tra_sky = (if 0.0 < value1 { value1 } else { 0.0 })
                        + (if 0.0 < value2 { value2 } else { 0.0 });
                    let value = sum_ref / sum_denom;
                    btar.sol.ft.df.refl_sky = if 0.0 < value { value } else { 0.0 };
                    let value = sum_abs / sum_denom;
                    btar.sol.ft.df.abs_sky = if 0.0 < value { value } else { 0.0 };
                }
            }
        }
    }
    Ok(())
}

// WindowManager7666: small Native scratch is local; the real material is borrowed.
#[allow(
    clippy::needless_range_loop,
    reason = "Preserve Native one-based column-major index and operation order"
)]
fn blind_optics_diffuse(
    blind: &SourceBlindMaterial,
    i_sol_vis: i32,
    c: &[f64; 16],
    b_el: f64,
    p: &mut [f64; 17],
    backend: &mut SourceErrorBackend<'_>,
) -> Result<(), HeatBalanceInputError> {
    let mut f_edge_a = [0.0; 3];
    let mut j = [0.0; 7];
    let mut g = [0.0; 7];
    let mut q = [0.0; 7];
    let mut f = [[0.0; 7]; 7];
    let mut x = [[0.0; 5]; 5];
    let mut xinv = [[0.0; 5]; 5];
    let mut indx = [0; 5];
    view_fac(c[2], c[3], b_el, PI / 2.0, &mut f)?;
    for k in [3, 5] {
        for m in 3..=6 {
            x[k - 2][m - 2] = -c[12] * f[m][k] - c[10] * f[m][k + 1];
            x[k - 1][m - 2] = -c[10] * f[m][k] - c[11] * f[m][k + 1];
        }
    }
    for k in 1..=4 {
        x[k][k] += 1.0;
    }
    indx.fill(0);
    invert_matrix(&mut x, &mut xinv, &mut indx, backend)?;
    q[3] = c[12] * f[1][3] + c[10] * f[1][4];
    q[4] = c[10] * f[1][3] + c[11] * f[1][4];
    q[5] = c[12] * f[1][5] + c[10] * f[1][6];
    q[6] = c[10] * f[1][5] + c[11] * f[1][6];
    j[1] = 1.0;
    j[2] = 0.0;
    for k in 3..=6 {
        j[k] = 0.0;
        for m in 3..=6 {
            j[k] += xinv[k - 2][m - 2] * q[m];
        }
    }
    for k in 1..=6 {
        g[k] = 0.0;
        for m in 1..=6 {
            g[k] += j[m] * f[m][k];
        }
    }
    let mut f_edge_source = [0.0; 10];
    let phib = b_el;
    // Native PiOvr2/10 is nine degrees, deliberately distinct from DiffuseAverage.
    let delphis = (PI / 2.0) / 10.0;
    for i_up_down in 1..=2 {
        for i_phi in 0..10 {
            let mut phis = -(i_phi as f64 + 0.5) * delphis;
            if i_up_down == 2 {
                phis = (i_phi as f64 + 0.5) * delphis;
            }
            f_edge_source[i_phi] = 0.0;
            let mut f_edge1 = 0.0;
            let gamma = phib - phis;
            if ep_platform_math::sin(gamma)
                .map_err(|error| provider_error("BlindOpticsDiffuse sin(gamma) condition", error))?
                .abs()
                > 0.01
            {
                if (phib > 0.0 && phib <= PI / 2.0 && phis <= phib)
                    || (phib > PI / 2.0 && phib <= PI && phis > -(PI - phib))
                {
                    f_edge1 = blind.slat_thickness
                        * ep_platform_math::sin(gamma)
                            .map_err(|error| {
                                provider_error("BlindOpticsDiffuse sin(gamma) numerator", error)
                            })?
                            .abs()
                        / ((blind.slat_separation
                            + blind.slat_thickness
                                / ep_platform_math::sin(phib)
                                    .map_err(|error| {
                                        provider_error("BlindOpticsDiffuse sin(phib)", error)
                                    })?
                                    .abs())
                            * ep_platform_math::cos(phis).map_err(|error| {
                                provider_error("BlindOpticsDiffuse cos(phis)", error)
                            })?);
                }
                let value = f_edge1.abs();
                // Objexx min returns its second operand for ties/unordered.
                f_edge_source[i_phi] = if 1.0 < value { 1.0 } else { value };
            }
        }
        f_edge_a[i_up_down] = diffuse_average(&f_edge_source)?;
    }
    let f_edge = 0.5 * (f_edge_a[1] + f_edge_a[2]);
    p[9] = g[2] * (1.0 - f_edge);
    p[10] = g[1] * (1.0 - f_edge) + f_edge * c[11];
    q[3] = c[12] * f[2][3] + c[10] * f[2][4];
    q[4] = c[10] * f[2][3] + c[11] * f[2][4];
    q[5] = c[12] * f[2][5] + c[10] * f[2][6];
    q[6] = c[10] * f[2][5] + c[11] * f[2][6];
    j[1] = 0.0;
    j[2] = 1.0;
    for k in 3..=6 {
        j[k] = 0.0;
        for m in 3..=6 {
            j[k] += xinv[k - 2][m - 2] * q[m];
        }
    }
    for k in 1..=6 {
        g[k] = 0.0;
        for m in 1..=6 {
            g[k] += j[m] * f[m][k];
        }
    }
    p[11] = g[1] * (1.0 - f_edge);
    p[12] = g[2] * (1.0 - f_edge) + f_edge * c[11];
    if i_sol_vis == 1 {
        let ri = 1.0 - c[13] - c[14];
        let rib = 1.0 - c[13] - c[15];
        for k in [3, 5] {
            for m in 3..=6 {
                x[k - 2][m - 2] = -rib * f[m][k] - c[13] * f[m][k + 1];
                x[k - 1][m - 2] = -c[13] * f[m][k] - ri * f[m][k + 1];
            }
        }
        for k in 1..=4 {
            x[k][k] += 1.0;
        }
        indx.fill(0);
        invert_matrix(&mut x, &mut xinv, &mut indx, backend)?;
        q[3] = rib * f[1][3] + c[13] * f[1][4];
        q[4] = c[13] * f[1][3] + ri * f[1][4];
        q[5] = rib * f[1][5] + c[13] * f[1][6];
        q[6] = c[13] * f[1][5] + ri * f[1][6];
        j[1] = 1.0;
        j[2] = 0.0;
        for k in 3..=6 {
            j[k] = 0.0;
            for m in 3..=6 {
                j[k] += xinv[k - 2][m - 2] * q[m];
            }
        }
        for k in 1..=6 {
            g[k] = 0.0;
            for m in 1..=6 {
                g[k] += j[m] * f[m][k];
            }
        }
        p[13] = g[2] * (1.0 - f_edge);
        let blind_ir_refl_front = g[1] * (1.0 - f_edge) + f_edge * ri;
        let value = 1.0 - p[13] - blind_ir_refl_front;
        p[14] = if 0.0001 < value { value } else { 0.0001 };
        q[3] = rib * f[2][3] + c[13] * f[2][4];
        q[4] = c[13] * f[2][3] + ri * f[2][4];
        q[5] = rib * f[2][5] + c[13] * f[2][6];
        q[6] = c[13] * f[2][5] + ri * f[2][6];
        j[1] = 0.0;
        j[2] = 1.0;
        for k in 3..=6 {
            j[k] = 0.0;
            for m in 3..=6 {
                j[k] += xinv[k - 2][m - 2] * q[m];
            }
        }
        for k in 1..=6 {
            g[k] = 0.0;
            for m in 1..=6 {
                g[k] += j[m] * f[m][k];
            }
        }
        // Native back uses ri, not rib, and the same front p13.
        let blind_ir_refl_back = g[2] * (1.0 - f_edge) + f_edge * ri;
        let value = 1.0 - p[13] - blind_ir_refl_back;
        p[15] = if 0.0001 < value { value } else { 0.0001 };
    }
    Ok(())
}

#[allow(
    clippy::needless_range_loop,
    reason = "Preserve Native one-based column-major index and operation order"
)]
fn blind_optics_beam(
    blind: &SourceBlindMaterial,
    c: &[f64; 16],
    b_el: f64,
    s_el: f64,
    p: &mut [f64; 17],
    backend: &mut SourceErrorBackend<'_>,
) -> Result<(), HeatBalanceInputError> {
    let mut j = [0.0; 7];
    let mut g = [0.0; 7];
    let mut q = [0.0; 7];
    let mut f = [[0.0; 7]; 7];
    let mut x = [[0.0; 5]; 5];
    let mut xinv = [[0.0; 5]; 5];
    let mut indx = [0; 5];
    p.fill(0.0); // Native p=0 for every profile; never zero the persistent TARs.
    let phis = s_el;
    let mut phib = b_el;
    for i in [0, 2] {
        if i == 2 {
            phib = PI - phib;
        }
        let mut f_edge = 0.0;
        let mut f_edge1 = 0.0;
        let gamma = phib - phis;
        if ep_platform_math::sin(gamma)
            .map_err(|error| provider_error("BlindOpticsBeam sin(gamma) condition", error))?
            .abs()
            > 0.01
        {
            if (phib > 0.0 && phib <= PI / 2.0 && phis <= phib)
                || (phib > PI / 2.0 && phib <= PI && phis > -(PI - phib))
            {
                f_edge1 = blind.slat_thickness
                    * ep_platform_math::sin(gamma)
                        .map_err(|error| {
                            provider_error("BlindOpticsBeam sin(gamma) numerator", error)
                        })?
                        .abs()
                    / ((blind.slat_separation
                        + blind.slat_thickness
                            / ep_platform_math::sin(phib)
                                .map_err(|error| {
                                    provider_error("BlindOpticsBeam sin(phib)", error)
                                })?
                                .abs())
                        * ep_platform_math::cos(phis)
                            .map_err(|error| provider_error("BlindOpticsBeam cos(phis)", error))?);
            }
            let value = f_edge1.abs();
            f_edge = if 1.0 < value { 1.0 } else { value };
        }
        p[1 + i] = blind
            .beam_beam_trans(phis, phib)
            .map_err(|error| provider_error("BlindOpticsBeam BeamBeamTrans", error))?;
        p[2 + i] = 0.0;
        view_fac(c[2], c[3], phib, phis, &mut f)?;
        for k in [3, 5] {
            for m in 3..=6 {
                x[k - 2][m - 2] = -c[12] * f[m][k] - c[10] * f[m][k + 1];
                x[k - 1][m - 2] = -c[10] * f[m][k] - c[11] * f[m][k + 1];
            }
        }
        for k in 1..=4 {
            x[k][k] += 1.0;
        }
        indx.fill(0);
        invert_matrix(&mut x, &mut xinv, &mut indx, backend)?;
        if (phis - phib).abs() <= PI / 2.0 {
            q[3] = c[4] + c[7];
            q[4] = c[5] + c[8];
        } else {
            q[3] = c[6] + c[9];
            q[4] = c[4] + c[7];
        }
        q[3] *= 1.0 - p[1 + i];
        q[4] *= 1.0 - p[1 + i];
        j[1] = 0.0;
        j[2] = 0.0;
        for k in 3..=6 {
            j[k] = 0.0;
            for m in 3..=4 {
                j[k] += xinv[k - 2][m - 2] * q[m];
            }
        }
        for k in 1..=6 {
            g[k] = 0.0;
            for m in 3..=6 {
                g[k] += j[m] * f[m][k];
            }
        }
        p[5 + i] = g[2] * (1.0 - f_edge);
        p[6 + i] = g[1] * (1.0 - f_edge) + f_edge * c[8];
    }
    Ok(())
}

#[allow(
    clippy::needless_range_loop,
    reason = "Preserve Native one-based column-major index and operation order"
)]
fn view_fac(
    s: f64,
    h: f64,
    phib: f64,
    phis: f64,
    f: &mut [[f64; 7]; 7],
) -> Result<(), HeatBalanceInputError> {
    let mut lengths = [0.0; 7];
    let h2 = h * h;
    let ht = 2.0 * h;
    let mut co =
        ep_platform_math::cos(phis).map_err(|error| provider_error("ViewFac cos(phis)", error))?;
    if co.abs() < 0.001 {
        co = 0.0;
    }
    let mut w = ht;
    if co != 0.0 {
        w = s * ep_platform_math::cos(phib - phis)
            .map_err(|error| provider_error("ViewFac cos(phib-phis)", error))?
            / co;
    }
    let mut l3 = s * h / w.abs();
    if l3 > s {
        l3 = s;
    }
    let l5 = s - l3;
    let a = ht
        * ep_platform_math::cos(phib)
            .map_err(|error| provider_error("ViewFac cos(phib)", error))?;
    let value = s * s + h2 + a * s;
    let d1 = ep_platform_math::sqrt(if 0.0 < value { value } else { 0.0 })
        .map_err(|error| provider_error("ViewFac sqrt(d1)", error))?;
    let value = s * s + h2 - a * s;
    let d2 = ep_platform_math::sqrt(if 0.0 < value { value } else { 0.0 })
        .map_err(|error| provider_error("ViewFac sqrt(d2)", error))?;
    let value = l3 * l3 + h2 + a * l3;
    let d3 = ep_platform_math::sqrt(if 0.0 < value { value } else { 0.0 })
        .map_err(|error| provider_error("ViewFac sqrt(d3)", error))?;
    let value = l3 * l3 + h2 - a * l3;
    let d4 = ep_platform_math::sqrt(if 0.0 < value { value } else { 0.0 })
        .map_err(|error| provider_error("ViewFac sqrt(d4)", error))?;
    let value = l5 * l5 + h2 - a * l5;
    let d5 = ep_platform_math::sqrt(if 0.0 < value { value } else { 0.0 })
        .map_err(|error| provider_error("ViewFac sqrt(d5)", error))?;
    let value = l5 * l5 + h2 + a * l5;
    let d6 = ep_platform_math::sqrt(if 0.0 < value { value } else { 0.0 })
        .map_err(|error| provider_error("ViewFac sqrt(d6)", error))?;
    for i in 1..=6 {
        f[i][i] = 0.0;
    }
    f[1][1] = 0.0;
    f[1][2] = (d1 + d2 - 2.0 * s) / ht;
    f[1][3] = (h + l3 - d3) / ht;
    f[1][4] = (h + l3 - d4) / ht;
    f[1][5] = (l5 + d3 - d1) / ht;
    f[1][6] = (l5 + d4 - d2) / ht;
    f[2][3] = (l3 + d5 - d2) / ht;
    f[2][4] = (l3 + d6 - d1) / ht;
    f[2][5] = (h + l5 - d5) / ht;
    f[2][6] = (h + l5 - d6) / ht;
    f[3][4] = (d3 + d4 - ht) / (2.0 * l3);
    f[3][5] = 0.0;
    f[3][6] = (d2 + h - d4 - d5) / (2.0 * l3);
    f[4][5] = (d1 + h - d3 - d6) / (2.0 * l3);
    f[4][6] = 0.0;
    f[5][6] = 0.0;
    if l5 > 0.0 {
        f[5][6] = (d5 + d6 - ht) / (2.0 * l5);
    }
    lengths[1] = h;
    lengths[2] = h;
    lengths[3] = l3;
    lengths[4] = l3;
    lengths[5] = l5;
    lengths[6] = l5;
    for i in 2..=6 {
        for j in 1..i {
            f[i][j] = 0.0;
            if lengths[i] > 0.0 {
                f[i][j] = f[j][i] * lengths[j] / lengths[i];
            }
        }
    }
    Ok(())
}

#[allow(
    clippy::needless_range_loop,
    reason = "Preserve Native one-based column-major index and operation order"
)]
fn invert_matrix(
    a: &mut [[f64; 5]; 5],
    y: &mut [[f64; 5]; 5],
    indx: &mut [usize; 5],
    backend: &mut SourceErrorBackend<'_>,
) -> Result<(), HeatBalanceInputError> {
    let mut tmp = [0.0; 5];
    let mut d = 0;
    for column in y.iter_mut().take(5).skip(1) {
        for value in column.iter_mut().take(5).skip(1) {
            *value = 0.0;
        }
    }
    for i in 1..=4 {
        y[i][i] = 1.0;
    }
    indx.fill(0);
    lu_decomposition(a, indx, &mut d, backend)?;
    for j in 1..=4 {
        tmp.fill(0.0);
        tmp[j] = 1.0;
        lu_solution(a, indx, &mut tmp);
        for i in 1..=4 {
            y[i][j] = tmp[i];
        } // Native y(j,i)=tmp(i), not y(i,j).
    }
    Ok(())
}

#[allow(
    clippy::needless_range_loop,
    reason = "Preserve Native one-based column-major index and operation order"
)]
fn lu_decomposition(
    ajac: &mut [[f64; 5]; 5],
    indx: &mut [usize; 5],
    d: &mut i32,
    backend: &mut SourceErrorBackend<'_>,
) -> Result<(), HeatBalanceInputError> {
    // Native imax persists across columns; it is undefined before first selection.
    // Source handles only that undefined use with the existing unavailable cause.
    let mut imax: Option<usize> = None;
    let mut vv = [0.0; 10];
    *d = 1;
    for i in 1..=4 {
        let mut aamax = 0.0;
        for j in 1..=4 {
            if ajac[i][j].abs() > aamax {
                aamax = ajac[i][j].abs();
            }
        }
        if aamax == 0.0 {
            return match backend
                .show_fatal_error("Singular matrix in LUdecomposition, window calculations")
            {
                Ok(never) => match never {},
                Err(stop) => Err(HeatBalanceInputError::Fatal(stop)),
            };
        }
        vv[i - 1] = 1.0 / aamax;
    }
    for j in 1..=4 {
        for i in 1..j {
            let mut sum = ajac[i][j];
            for k in 1..i {
                sum -= ajac[i][k] * ajac[k][j];
            }
            ajac[i][j] = sum;
        }
        let mut aamax = 0.0;
        for i in j..=4 {
            let mut sum = ajac[i][j];
            for k in 1..j {
                sum -= ajac[i][k] * ajac[k][j];
            }
            ajac[i][j] = sum;
            let dum = vv[i - 1] * sum.abs();
            if dum >= aamax {
                imax = Some(i);
                aamax = dum;
            }
        }
        let selected = imax.ok_or_else(|| {
            HeatBalanceInputError::SourceInput(Box::new(unavailable(
                "Native uninitialized LU pivot: no dum>=aamax selection before first imax use",
            )))
        })?;
        if j != selected {
            // Native ajac(k,imax) is ajac[imax][k]: swap OUTER columns.
            // All four actual cells are exchanged without FP operations.
            // The fifth padding cell(index0) is identically zero in both columns.
            ajac.swap(selected, j);
            *d = -*d;
            vv[selected - 1] = vv[j - 1];
        }
        indx[j] = selected;
        if ajac[j][j] == 0.0 {
            ajac[j][j] = f64::EPSILON;
        }
        if j != 4 {
            let dum = 1.0 / ajac[j][j];
            for i in j + 1..=4 {
                ajac[i][j] *= dum;
            }
        }
    }
    Ok(())
}

#[allow(
    clippy::needless_range_loop,
    reason = "Preserve Native one-based column-major index and operation order"
)]
fn lu_solution(a: &[[f64; 5]; 5], indx: &[usize; 5], b: &mut [f64; 5]) {
    let mut ii = 0;
    for i in 1..=4 {
        let ll = indx[i];
        let mut sum = b[ll];
        b[ll] = b[i];
        if ii != 0 {
            for j in ii..i {
                sum -= a[i][j] * b[j];
            }
        } else if sum != 0.0 {
            ii = i;
        }
        b[i] = sum;
    }
    for i in (1..=4).rev() {
        let mut sum = b[i];
        for j in i + 1..=4 {
            sum -= a[i][j] * b[j];
        }
        b[i] = sum / a[i][i];
    }
}

// Native WindowManager5608: ten 0..90-degree samples, not the9-degree edge step.
#[allow(
    clippy::needless_range_loop,
    reason = "Preserve Native neighboring-angle indices and provider/accumulation order"
)]
fn diffuse_average(props: &[f64; 10]) -> Result<f64, HeatBalanceInputError> {
    let d_phi_r = 10.0 * (PI / 180.0);
    let mut avg = 0.0;
    for i_phi in 0..9 {
        avg += 0.5
            * d_phi_r
            * (props[i_phi]
                * ep_platform_math::sin(2.0 * i_phi as f64 * d_phi_r)
                    .map_err(|error| provider_error("DiffuseAverage sin(lower)", error))?
                + props[i_phi + 1]
                    * ep_platform_math::sin(2.0 * (i_phi + 1) as f64 * d_phi_r)
                        .map_err(|error| provider_error("DiffuseAverage sin(upper)", error))?);
    }
    Ok(if avg < 0.0 { 0.0 } else { avg })
}
