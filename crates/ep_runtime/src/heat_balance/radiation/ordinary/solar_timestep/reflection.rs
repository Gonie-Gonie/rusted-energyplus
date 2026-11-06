//! Actual SolarReflectionManager94-578 receiver/obstruction/ray/hit owners.
//! Numerical operands are the retained final Source Surface and Construction owners.
use super::array::SourceSolarArray;
use super::*;

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceSolarReflectionState {
    pub(crate) tot_sol_refl_rec_surf: i32,
    #[allow(dead_code, reason = "Retain Native unused constructor/clear owner")]
    pub(crate) tot_phi_refl_rays: i32,
    #[allow(dead_code, reason = "Retain Native unused constructor/clear owner")]
    pub(crate) tot_theta_refl_rays: i32,
    pub(crate) sol_refl_rec_surf: Option<Vec<SourceSolReflRecSurfData>>,
}

/// SolarReflectionManager.hh68-98: scalar zero constructors, arrays unallocated.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceSolReflRecSurfData {
    pub(crate) surf_num: i32,
    pub(crate) surf_name: String,
    pub(crate) num_rec_pts: i32,
    pub(crate) rec_pt: SourceSolarArray<[f64; 3], 1>,
    pub(crate) norm_vec: [f64; 3],
    pub(crate) theta_norm_vec: f64,
    pub(crate) phi_norm_vec: f64,
    pub(crate) num_refl_rays: i32,
    pub(crate) ray_vec: SourceSolarArray<[f64; 3], 1>,
    pub(crate) cos_inc_ang_ray: SourceSolarArray<f64, 1>,
    pub(crate) d_omega_ray: SourceSolarArray<f64, 1>,
    pub(crate) hit_pt: SourceSolarArray<[f64; 3], 2>,
    pub(crate) hit_pt_surf_num: SourceSolarArray<i32, 2>,
    pub(crate) hit_pt_sol_refl: SourceSolarArray<f64, 2>,
    pub(crate) rec_pt_hit_pt_dis: SourceSolarArray<f64, 2>,
    pub(crate) hit_pt_norm_vec: SourceSolarArray<[f64; 3], 2>,
    pub(crate) possible_obs_surf_nums: SourceSolarArray<i32, 1>,
    pub(crate) num_possible_obs: i32,
}

pub(super) fn subtract(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
pub(super) fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    (a[0] * b[0]) + (a[1] * b[1]) + (a[2] * b[2])
}
pub(super) fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        (a[1] * b[2]) - (a[2] * b[1]),
        (a[2] * b[0]) - (a[0] * b[2]),
        (a[0] * b[1]) - (a[1] * b[0]),
    ]
}
pub(super) fn magnitude(value: [f64; 3]) -> f64 {
    ((value[0] * value[0]) + (value[1] * value[1]) + (value[2] * value[2])).sqrt()
}
pub(super) fn distance(a: [f64; 3], b: [f64; 3]) -> f64 {
    magnitude(subtract(a, b))
}
pub(super) fn normalize_reciprocal(value: [f64; 3]) -> Result<[f64; 3], Error> {
    let length = magnitude(value);
    if length == 0.0 {
        // Objexx Vector3.normalize asserts length!=0. No zero-normal repair.
        return Err(Error::Unavailable(
            "InitSolReflRecSurf actual hit normal nonzero assertion529",
        ));
    }
    let dilation = 1.0 / length;
    Ok([
        value[0] * dilation,
        value[1] * dilation,
        value[2] * dilation,
    ])
}
fn signed(value: usize) -> Result<i32, Error> {
    i32::try_from(value).map_err(|_| Error::Unavailable("actual reflection signed ordinal"))
}
fn increment(value: &mut i32) -> Result<(), Error> {
    *value = value
        .checked_add(1)
        .ok_or(Error::Unavailable("actual reflection signed count"))?;
    Ok(())
}
fn vertex(
    geometry: &SurfaceGeometryInputState,
    surface: usize,
    number: usize,
) -> Result<[f64; 3], Error> {
    source_surface(geometry, surface)?
        .vertex
        .as_ref()
        .and_then(|rows| rows.get(number - 1))
        .copied()
        .flatten()
        .ok_or(Error::Unavailable(
            "InitSolReflRecSurf actual written final World vertex",
        ))
}
fn sides(geometry: &SurfaceGeometryInputState, surface: usize) -> Result<usize, Error> {
    usize::try_from(source_surface(geometry, surface)?.sides)
        .map_err(|_| Error::Unavailable("actual reflection Surface.Sides"))
}

impl OrdinarySolarRadiation {
    pub(crate) fn source_init_solar_reflection_receivers(
        &mut self,
        geometry: &mut SurfaceGeometryInputState,
        control: &mut SelectedHeatBalanceInputState,
        environment: &WeatherEnvironmentState,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        // Native179-221: receiver count, no-receiver warning/reset, then IgnoreSolar.
        self.solar_reflection.tot_sol_refl_rec_surf = 0;
        let total = surface_count(geometry)?;
        for number in 1..=total {
            if source_surface(geometry, number)?.exterior_solar {
                increment(&mut self.solar_reflection.tot_sol_refl_rec_surf)?;
            }
        }
        if self.solar_reflection.tot_sol_refl_rec_surf == 0 {
            backend.show_warning_error(
                "Calculation of solar reflected from obstructions has been requested but there",
            )?;
            backend.show_continue_error(
                "are no building surfaces that can receive reflected solar. Calculation will not be done.",
            )?;
            control.calculate_solar_reflections = false;
            return Ok(());
        }
        if environment.ignore_solar_radiation {
            self.solar_reflection.tot_sol_refl_rec_surf = 0;
            control.calculate_solar_reflections = false;
            return Ok(());
        }

        // Native223-230: receiver allocation precedes all six factor dimensions.
        let receiver_count = ordinal(self.solar_reflection.tot_sol_refl_rec_surf)?;
        // Objexx Array1D.allocate calls dimension_real: the existing elements
        // survive an unchanged extent; a changed extent default-constructs rows.
        if self
            .solar_reflection
            .sol_refl_rec_surf
            .as_ref()
            .map(Vec::len)
            != Some(receiver_count)
        {
            self.solar_reflection.sol_refl_rec_surf =
                Some(vec![SourceSolReflRecSurfData::default(); receiver_count]);
        }
        let factors = geometry
            .window_arrays
            .as_mut()
            .ok_or(Error::Unavailable("actual reflection Surface arrays"))?;
        factors
            .surf_refl_fac_bm_to_diff_sol_obs
            .dimension([24, total], Some(0.0))?;
        factors
            .surf_refl_fac_bm_to_diff_sol_gnd
            .dimension([24, total], Some(0.0))?;
        factors
            .surf_refl_fac_bm_to_bm_sol_obs
            .dimension([24, total], Some(0.0))?;
        factors
            .surf_refl_fac_sky_sol_obs
            .dimension([total], Some(0.0))?;
        factors
            .surf_refl_fac_sky_sol_gnd
            .dimension([total], Some(0.0))?;
        factors
            .surf_cos_inc_ave_bm_to_bm_sol_obs
            .dimension([24, total], Some(0.0))?;

        let receivers = self
            .solar_reflection
            .sol_refl_rec_surf
            .as_mut()
            .ok_or(Error::Unavailable("actual reflection receiver allocation"))?;
        let mut receiver_number = 0_i32;
        for number in 1..=total {
            *geometry
                .parallel_arrays
                .as_mut()
                .and_then(|rows| rows.surf_shadow_rec_surf_num.get_mut(number - 1))
                .ok_or(Error::Unavailable("actual SurfShadowRecSurfNum bounds"))? = 0;
            if !source_surface(geometry, number)?.exterior_solar {
                continue;
            }
            increment(&mut receiver_number)?;
            let receiver = &mut receivers[ordinal(receiver_number)? - 1];
            receiver.surf_num = signed(number)?;
            receiver.surf_name = source_surface(geometry, number)?.name.clone();
            *geometry
                .parallel_arrays
                .as_mut()
                .and_then(|rows| rows.surf_shadow_rec_surf_num.get_mut(number - 1))
                .ok_or(Error::Unavailable("actual SurfShadowRecSurfNum bounds"))? = receiver_number;
            for at in 1..=sides(geometry, number)? {
                if vertex(geometry, number, at)?[2] < geometry.ground_level_z {
                    backend.show_warning_error(&format!(
                        "Calculation of reflected solar onto surface={} may be inaccurate",
                        source_surface(geometry, number)?.name,
                    ))?;
                    backend.show_continue_error(
                        "because it has one or more vertices below ground level.",
                    )?;
                    break;
                }
            }
        }

        geometry.max_rec_pts = 1;
        for receiver in receivers.iter_mut() {
            receiver.num_rec_pts = source_surface(geometry, ordinal(receiver.surf_num)?)?.sides;
            if receiver.num_rec_pts > geometry.max_rec_pts {
                geometry.max_rec_pts = receiver.num_rec_pts;
            }
        }
        // DataSurfaces.hh309-310: literal altitude10 and azimuth9 step counts.
        geometry.max_refl_rays = 10 * 9;
        let max_points = ordinal(geometry.max_rec_pts)?;
        let max_rays = ordinal(geometry.max_refl_rays)?;
        for receiver in receivers.iter_mut() {
            receiver.norm_vec = [0.0; 3];
            receiver.rec_pt.dimension([max_points], Some([0.0; 3]))?;
            receiver.ray_vec.dimension([max_rays], Some([0.0; 3]))?;
            receiver.cos_inc_ang_ray.dimension([max_rays], Some(0.0))?;
            receiver.d_omega_ray.dimension([max_rays], Some(0.0))?;
            receiver
                .hit_pt
                .dimension([max_rays, max_points], Some([0.0; 3]))?;
            receiver
                .hit_pt_surf_num
                .dimension([max_rays, max_points], Some(0))?;
            receiver
                .hit_pt_sol_refl
                .dimension([max_rays, max_points], Some(0.0))?;
            receiver
                .rec_pt_hit_pt_dis
                .dimension([max_rays, max_points], Some(0.0))?;
            receiver
                .hit_pt_norm_vec
                .dimension([max_rays, max_points], Some([0.0; 3]))?;
            receiver
                .possible_obs_surf_nums
                .dimension([total], Some(0))?;
        }

        for receiver in receivers.iter_mut() {
            let number = ordinal(receiver.surf_num)?;
            receiver.norm_vec = geometry_properties(geometry, number)?.out_norm;
            let rec_vec = vertex(geometry, number, 1)?;
            receiver.num_possible_obs = 0;
            for obstruction in 1..=total {
                let candidate = source_surface(geometry, obstruction)?;
                if obstruction == number
                    || signed(obstruction)? == source_surface(geometry, number)?.base_surface
                {
                    continue;
                }
                if candidate.heat_transfer_surface && candidate.exterior_boundary != 0 {
                    continue;
                }
                let mut behind = true;
                for at in 1..=sides(geometry, obstruction)? {
                    let obs_vec = vertex(geometry, obstruction, at)?;
                    if dot(receiver.norm_vec, subtract(obs_vec, rec_vec)) > 0.000001 {
                        behind = false;
                        break;
                    }
                }
                if behind {
                    continue;
                }
                if candidate.heat_transfer_surface {
                    let mut has_view = false;
                    for a in 1..=sides(geometry, number)? {
                        for b in 1..=sides(geometry, obstruction)? {
                            let ab = subtract(
                                vertex(geometry, obstruction, b)?,
                                vertex(geometry, number, a)?,
                            );
                            if dot(ab, receiver.norm_vec) > 0.0
                                && dot(ab, geometry_properties(geometry, obstruction)?.out_norm)
                                    < 0.0
                            {
                                has_view = true;
                                break;
                            }
                        }
                        if has_view {
                            break;
                        }
                    }
                    if !has_view {
                        continue;
                    }
                }
                increment(&mut receiver.num_possible_obs)?;
                receiver
                    .possible_obs_surf_nums
                    .set([ordinal(receiver.num_possible_obs)?], signed(obstruction)?)?;
            }

            let num_points = ordinal(receiver.num_rec_pts)?;
            for j in 1..=num_points {
                receiver.rec_pt.set([j], [0.0; 3])?;
                for k in 1..=num_points {
                    let weight = if num_points == 3 {
                        if k == j { 0.6 } else { 0.2 }
                    } else if k == j {
                        (f64::from(receiver.num_rec_pts) + 1.0)
                            / (2.0 * f64::from(receiver.num_rec_pts))
                    } else {
                        1.0 / (2.0 * f64::from(receiver.num_rec_pts))
                    };
                    let v = vertex(geometry, number, k)?;
                    let mut point = receiver.rec_pt.read([j])?;
                    point[0] += weight * v[0];
                    point[1] += weight * v[1];
                    point[2] += weight * v[2];
                    receiver.rec_pt.set([j], point)?;
                }
            }

            let phi_surf = receiver.norm_vec[2].asin();
            let tan_phi_surf = phi_surf.tan();
            let sin_phi_surf = phi_surf.sin();
            let cos_phi_surf = phi_surf.cos();
            let theta_surf =
                if receiver.norm_vec[0].abs() > 1.0e-5 || receiver.norm_vec[1].abs() > 1.0e-5 {
                    receiver.norm_vec[1].atan2(receiver.norm_vec[0])
                } else {
                    0.0
                };
            receiver.phi_norm_vec = phi_surf;
            receiver.theta_norm_vec = theta_surf;
            let pi = std::f64::consts::PI;
            let pi_over_two = std::f64::consts::FRAC_PI_2;
            // EnergyPlus.hh imports Objexx Fmath max/min. Keep their
            // operand selection on equality/unordered values literally.
            let lower = phi_surf - pi_over_two;
            let phi_min = if -pi_over_two < lower {
                lower
            } else {
                -pi_over_two
            };
            let upper = phi_surf + pi_over_two;
            let phi_max = if pi_over_two < upper {
                pi_over_two
            } else {
                upper
            };
            let d_phi = (phi_max - phi_min) / 10.0;
            let mut ray_number = 0_i32;
            for i_phi in 1..=10 {
                let phi = phi_min + (f64::from(i_phi) - 0.5) * d_phi;
                let s_phi = phi.sin();
                let c_phi = phi.cos();
                let (theta_min, theta_max) = if phi_surf >= 0.0 {
                    if phi >= pi_over_two - phi_surf {
                        (-pi, pi)
                    } else {
                        let acos_tan_tan = (-phi.tan() * tan_phi_surf).acos();
                        (
                            theta_surf - acos_tan_tan.abs(),
                            theta_surf + acos_tan_tan.abs(),
                        )
                    }
                } else if phi <= -phi_surf - pi_over_two {
                    (-pi, pi)
                } else {
                    let acos_tan_tan = (-phi.tan() * tan_phi_surf).acos();
                    (
                        theta_surf - acos_tan_tan.abs(),
                        theta_surf + acos_tan_tan.abs(),
                    )
                };
                let d_theta = (theta_max - theta_min) / 9.0;
                let d_omega = c_phi * d_theta * d_phi;
                for i_theta in 1..=9 {
                    let theta = theta_min + (f64::from(i_theta) - 0.5) * d_theta;
                    let ray = [c_phi * theta.cos(), c_phi * theta.sin(), s_phi];
                    let cosine =
                        s_phi * sin_phi_surf + c_phi * cos_phi_surf * (theta - theta_surf).cos();
                    if cosine < 0.0 {
                        continue;
                    }
                    increment(&mut ray_number)?;
                    let at = ordinal(ray_number)?;
                    receiver.ray_vec.set([at], ray)?;
                    receiver.cos_inc_ang_ray.set([at], cosine)?;
                    receiver.d_omega_ray.set([at], d_omega)?;
                }
            }
            receiver.num_refl_rays = ray_number;
        }

        // Native464-578: receiving surface -> receiving point -> ray -> obstruction.
        // Native NearestHitPt is one invocation-local Vector3, not reset per ray.
        let mut nearest_point = None;
        for receiver in receivers.iter_mut() {
            for point_number in 1..=ordinal(receiver.num_rec_pts)? {
                let rec_pt = receiver.rec_pt.read([point_number])?;
                for ray_number in 1..=ordinal(receiver.num_refl_rays)? {
                    let mut total_hits = 0_i32;
                    let mut nearest_surface = 0_i32;
                    let mut nearest_distance = 1.0e8;
                    let mut skip = 0_i32;
                    let ray = receiver.ray_vec.read([ray_number])?;
                    let possible = usize::try_from(receiver.num_possible_obs)
                        .map_err(|_| Error::Unavailable("actual possible obstruction count"))?;
                    for at in 1..=possible {
                        let obstruction_number = receiver.possible_obs_surf_nums.read([at])?;
                        if obstruction_number == skip {
                            continue;
                        }
                        let obstruction = ordinal(obstruction_number)?;
                        let surface = geometry
                            .final_surfaces
                            .as_ref()
                            .and_then(|rows| rows.get(obstruction - 1))
                            .ok_or(Error::Unavailable("actual PierceSurface final Surface"))?;
                        let mut hit_point = [0.0; 3];
                        if !super::pierce_surface::pierce_surface(
                            surface,
                            rec_pt,
                            ray,
                            &mut hit_point,
                        )? {
                            continue;
                        }
                        if surface.row.class == SurfaceClass::Window {
                            skip = surface.row.base_surface;
                        }
                        if surface.row.class == SurfaceClass::Window
                            && surface.row.base_surface == nearest_surface
                        {
                            nearest_surface = obstruction_number;
                        } else {
                            increment(&mut total_hits)?;
                            let hit_distance = distance(hit_point, rec_pt);
                            if hit_distance < nearest_distance {
                                nearest_distance = hit_distance;
                                nearest_surface = obstruction_number;
                                nearest_point = Some(hit_point);
                            } else if hit_distance == nearest_distance
                                && dot(geometry_properties(geometry, obstruction)?.out_norm, ray)
                                    <= 0.0
                            {
                                nearest_surface = obstruction_number;
                            }
                        }
                    }
                    let indices = [ray_number, point_number];
                    if total_hits > 0 {
                        receiver.hit_pt_surf_num.set(indices, nearest_surface)?;
                        receiver.rec_pt_hit_pt_dis.set(indices, nearest_distance)?;
                        receiver.hit_pt.set(
                            indices,
                            nearest_point.ok_or(Error::Unavailable(
                                "InitSolReflRecSurf actual written nearest hit point523",
                            ))?,
                        )?;
                        let nearest = ordinal(nearest_surface)?;
                        let v1 =
                            subtract(vertex(geometry, nearest, 1)?, vertex(geometry, nearest, 3)?);
                        let v2 =
                            subtract(vertex(geometry, nearest, 2)?, vertex(geometry, nearest, 3)?);
                        let mut normal = normalize_reciprocal(cross(v1, v2))?;
                        if dot(normal, [-ray[0], -ray[1], -ray[2]]) < 0.0 {
                            normal = [-normal[0], -normal[1], -normal[2]];
                        }
                        receiver.hit_pt_norm_vec.set(indices, normal)?;
                        let construction = source_surface(geometry, nearest)?.construction;
                        let reflectance = if construction > 0 {
                            let construction = control
                                .construction_usage
                                .get(ordinal(construction)? - 1)
                                .ok_or(Error::Unavailable(
                                    "actual hit-point Construction ordinal",
                                ))?;
                            if !construction.type_is_window {
                                1.0 - construction.outside_absorp_solar
                            } else {
                                0.0
                            }
                        } else {
                            *geometry
                                .parallel_arrays
                                .as_ref()
                                .and_then(|rows| rows.surf_shadow_diffuse_sol_refl.get(nearest - 1))
                                .ok_or(Error::Unavailable("actual shading diffuse reflectance"))?
                        };
                        receiver.hit_pt_sol_refl.set(indices, reflectance)?;
                    } else {
                        receiver.hit_pt_surf_num.set(indices, 0)?;
                        if ray[2] < 0.0 && rec_pt[2] > geometry.ground_level_z {
                            let alfa = (-ray[2]).acos();
                            let beta = ray[1].atan2(ray[0]);
                            let horizontal_distance =
                                (rec_pt[2] - geometry.ground_level_z) * alfa.tan();
                            let ground_point = [
                                rec_pt[0] + horizontal_distance * beta.cos(),
                                rec_pt[1] + horizontal_distance * beta.sin(),
                                geometry.ground_level_z,
                            ];
                            receiver.hit_pt.set(indices, ground_point)?;
                            receiver.hit_pt_surf_num.set(indices, -1)?;
                            receiver
                                .rec_pt_hit_pt_dis
                                .set(indices, (rec_pt[2] - geometry.ground_level_z) / (-ray[2]))?;
                            receiver
                                .hit_pt_sol_refl
                                .set(indices, environment.current_weather.gnd_reflectance)?;
                            receiver.hit_pt_norm_vec.set(indices, [0.0, 0.0, 1.0])?;
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
