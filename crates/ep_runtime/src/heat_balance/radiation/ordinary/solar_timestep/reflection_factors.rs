//! Actual opaque SolarReflectionManager582--1364 factors on retained ray/Surface owners.
use super::array::SourceSolarArray;
use super::pierce_surface::pierce_surface;
use super::reflection::{SourceSolReflRecSurfData, dot, subtract};
use super::*;
use crate::heat_balance::selected_heat_balance_input::shadowing::SourceShadowingInput;
use crate::heat_balance::selected_heat_balance_input::surface::FinalSurface;

const ALT_ANGLE_STEPS: usize = 10; // DataSurfaces309
const AZIM_ANGLE_STEPS: usize = 9; // DataSurfaces310
const PI: f64 = std::f64::consts::PI;

fn extent(n: i32) -> Result<usize, Error> {
    usize::try_from(n).map_err(|_| Error::Unavailable("actual reflection nonnegative extent"))
}

fn receiving(solar: &OrdinarySolarRadiation, n: usize) -> Result<&SourceSolReflRecSurfData, Error> {
    solar
        .solar_reflection
        .sol_refl_rec_surf
        .as_ref()
        .and_then(|rows| rows.get(n - 1))
        .ok_or(Error::Unavailable("same allocated SolReflRecSurf row"))
}

fn final_surface(geometry: &SurfaceGeometryInputState, n: usize) -> Result<&FinalSurface, Error> {
    geometry
        .final_surfaces
        .as_ref()
        .and_then(|rows| rows.get(n - 1))
        .ok_or(Error::Unavailable("same final reflection SurfaceData"))
}

fn shelf(geometry: &SurfaceGeometryInputState, n: usize) -> Result<i32, Error> {
    geometry
        .parallel_arrays
        .as_ref()
        .and_then(|rows| rows.surf_daylighting_shelf_ind.get(n - 1))
        .copied()
        .ok_or(Error::Unavailable("actual SurfDaylightingShelfInd"))
}

fn add_1(array: &mut SourceSolarArray<f64, 1>, n: usize, value: f64) -> Result<(), Error> {
    let old = array.read([n])?;
    array.set([n], old + value)
}

fn add_2(
    array: &mut SourceSolarArray<f64, 2>,
    hour: usize,
    n: usize,
    value: f64,
) -> Result<(), Error> {
    let old = array.read([hour, n])?;
    array.set([hour, n], old + value)
}

impl OrdinarySolarRadiation {
    pub(crate) fn source_calc_beam_solar_diffuse_reflection_factors(
        &mut self,
        geometry: &mut SurfaceGeometryInputState,
        _control: &SelectedHeatBalanceInputState,
        globals: &WeatherGlobalState,
        _environment: &WeatherEnvironmentState,
        shadowing: &SourceShadowingInput,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        // Actual596--613: periodic and detailed-hour initialization are distinct.
        if !shadowing.detailed_solar_timestep_integration {
            backend.display_string(if globals.begin_sim_flag {
                "Calculating Beam-to-Diffuse Exterior Solar Reflection Factors"
            } else {
                "Updating Beam-to-Diffuse Exterior Solar Reflection Factors"
            })?;
            let windows = geometry
                .window_arrays
                .as_mut()
                .ok_or(Error::Unavailable("actual reflection factor arrays"))?;
            windows.surf_refl_fac_bm_to_diff_sol_obs.fill(0.0)?;
            windows.surf_refl_fac_bm_to_diff_sol_gnd.fill(0.0)?;
            for hour in 1..=24 {
                self.source_figure_beam_solar_diffuse_reflection_factors(geometry, hour)?;
            }
        } else {
            let hour = ordinal(globals.hour_of_day)?;
            let count = surface_count(geometry)?;
            let windows = geometry
                .window_arrays
                .as_mut()
                .ok_or(Error::Unavailable("actual reflection factor arrays"))?;
            for number in 1..=count {
                windows
                    .surf_refl_fac_bm_to_diff_sol_obs
                    .set([hour, number], 0.0)?;
            }
            for number in 1..=count {
                windows
                    .surf_refl_fac_bm_to_diff_sol_gnd
                    .set([hour, number], 0.0)?;
            }
            self.source_figure_beam_solar_diffuse_reflection_factors(geometry, hour)?;
        }
        Ok(())
    }

    fn source_figure_beam_solar_diffuse_reflection_factors(
        &self,
        geometry: &mut SurfaceGeometryInputState,
        hour: usize,
    ) -> Result<(), Error> {
        let max = extent(geometry.max_rec_pts)?;
        let mut refl_obs = SourceSolarArray::<f64, 1>::default();
        let mut refl_gnd = SourceSolarArray::<f64, 1>::default();
        refl_obs.dimension([max], None)?;
        refl_gnd.dimension([max], None)?;
        refl_obs.fill(0.0)?;
        refl_gnd.fill(0.0)?;
        let sun_vec = geometry
            .window_arrays
            .as_ref()
            .ok_or(Error::Unavailable("actual SurfSunCosHourly"))?
            .surf_sun_cos_hourly
            .read([hour])?;
        let mut obs_hit_pt = [0.0; 3];
        let count = surface_count(geometry)?;
        for rec_surf_num in 1..=extent(self.solar_reflection.tot_sol_refl_rec_surf)? {
            let row = receiving(self, rec_surf_num)?;
            let surf_num = ordinal(row.surf_num)?;
            for rec_pt_num in 1..=extent(row.num_rec_pts)? {
                refl_obs.set([rec_pt_num], 0.0)?;
                refl_gnd.set([rec_pt_num], 0.0)?;
                for ray_num in 1..=extent(row.num_refl_rays)? {
                    let hit_pt_surf_num = row.hit_pt_surf_num.read([ray_num, rec_pt_num])?;
                    if hit_pt_surf_num == 0 {
                        continue;
                    }
                    if hit_pt_surf_num > 0 {
                        let hit_number = ordinal(hit_pt_surf_num)?;
                        if shelf(geometry, hit_number)? > 0 {
                            continue;
                        }
                        let class = source_surface(geometry, hit_number)?.class;
                        if class == SurfaceClass::Window || class == SurfaceClass::GlassDoor {
                            continue;
                        }
                        // Actual681: first timestep of the hour, even in the detailed branch.
                        let sunlit = self
                            .solar_timestep
                            .heat_balance
                            .surf_sunlit_frac
                            .read([hour, 1, hit_number])?;
                        if sunlit < 0.01 {
                            continue;
                        }
                    }
                    let origin = row.hit_pt.read([ray_num, rec_pt_num])?;
                    let cos_inc_bm = dot(row.hit_pt_norm_vec.read([ray_num, rec_pt_num])?, sun_vec);
                    if cos_inc_bm <= 0.0 {
                        continue;
                    }
                    // Actual717--735 mirrored shade-side rejection.
                    if hit_pt_surf_num > 0 {
                        let hit_number = ordinal(hit_pt_surf_num)?;
                        if source_surface(geometry, hit_number)?.is_shadowing
                            && hit_number + 1 < count
                        {
                            let mirror = source_surface(geometry, hit_number + 1)?;
                            if mirror.is_shadowing && mirror.mirrored_surface {
                                let cosine = dot(
                                    geometry_properties(geometry, hit_number + 1)?.out_norm,
                                    sun_vec,
                                );
                                if cosine >= 0.0 {
                                    continue;
                                }
                            }
                        }
                    }
                    let mut hit = false;
                    // Actual748 uses every Surface, not the receiver's PossibleObs vector.
                    for obs_surf_num in 1..=count {
                        if hit_pt_surf_num > 0
                            && source_surface(geometry, ordinal(hit_pt_surf_num)?)?.mirrored_surface
                            && i32::try_from(obs_surf_num).ok() == Some(hit_pt_surf_num - 1)
                        {
                            continue;
                        }
                        if i32::try_from(obs_surf_num).ok() == Some(hit_pt_surf_num) {
                            continue;
                        }
                        let obstruction = source_surface(geometry, obs_surf_num)?;
                        if obstruction.mirrored_surface || obstruction.exterior_boundary >= 1 {
                            continue;
                        }
                        hit = pierce_surface(
                            final_surface(geometry, obs_surf_num)?,
                            origin,
                            sun_vec,
                            &mut obs_hit_pt,
                        )?;
                        if hit {
                            break;
                        }
                    }
                    if hit {
                        continue;
                    }
                    let radiance = cos_inc_bm * row.hit_pt_sol_refl.read([ray_num, rec_pt_num])?;
                    if radiance > 0.0 {
                        if hit_pt_surf_num > 0 {
                            let delta = radiance
                                * row.d_omega_ray.read([ray_num])?
                                * row.cos_inc_ang_ray.read([ray_num])?
                                / PI;
                            add_1(&mut refl_obs, rec_pt_num, delta)?;
                        } else {
                            let delta = row.d_omega_ray.read([ray_num])?
                                * row.cos_inc_ang_ray.read([ray_num])?
                                / PI;
                            add_1(&mut refl_gnd, rec_pt_num, delta)?;
                        }
                    }
                }
            }
            let windows = geometry
                .window_arrays
                .as_mut()
                .ok_or(Error::Unavailable("actual beam diffuse factors"))?;
            windows
                .surf_refl_fac_bm_to_diff_sol_obs
                .set([hour, surf_num], 0.0)?;
            windows
                .surf_refl_fac_bm_to_diff_sol_gnd
                .set([hour, surf_num], 0.0)?;
            let num_rec_pts = extent(row.num_rec_pts)?;
            for rec_pt_num in 1..=num_rec_pts {
                add_2(
                    &mut windows.surf_refl_fac_bm_to_diff_sol_obs,
                    hour,
                    surf_num,
                    refl_obs.read([rec_pt_num])?,
                )?;
                add_2(
                    &mut windows.surf_refl_fac_bm_to_diff_sol_gnd,
                    hour,
                    surf_num,
                    refl_gnd.read([rec_pt_num])?,
                )?;
            }
            let value = windows
                .surf_refl_fac_bm_to_diff_sol_obs
                .read([hour, surf_num])?
                / num_rec_pts as f64;
            windows
                .surf_refl_fac_bm_to_diff_sol_obs
                .set([hour, surf_num], value)?;
            let value = windows
                .surf_refl_fac_bm_to_diff_sol_gnd
                .read([hour, surf_num])?
                / num_rec_pts as f64;
            windows
                .surf_refl_fac_bm_to_diff_sol_gnd
                .set([hour, surf_num], value)?;
            // Objexx Fmath min(limit,value), selecting its second operand
            // when equal or unordered, as imported by EnergyPlus.hh166.
            let limit = 0.5 * (1.0 - geometry_properties(geometry, surf_num)?.cos_tilt);
            let windows = geometry
                .window_arrays
                .as_mut()
                .ok_or(Error::Unavailable("actual ground reflection factor"))?;
            let value = windows
                .surf_refl_fac_bm_to_diff_sol_gnd
                .read([hour, surf_num])?;
            windows
                .surf_refl_fac_bm_to_diff_sol_gnd
                .set([hour, surf_num], if limit < value { limit } else { value })?;
        }
        Ok(())
    }

    pub(crate) fn source_calc_beam_solar_specular_reflection_factors(
        &mut self,
        geometry: &mut SurfaceGeometryInputState,
        _control: &SelectedHeatBalanceInputState,
        globals: &WeatherGlobalState,
        _environment: &WeatherEnvironmentState,
        shadowing: &SourceShadowingInput,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        if !shadowing.detailed_solar_timestep_integration {
            backend.display_string(if globals.begin_sim_flag {
                "Calculating Beam-to-Beam Exterior Solar Reflection Factors"
            } else {
                "Updating Beam-to-Beam Exterior Solar Reflection Factors"
            })?;
            let windows = geometry
                .window_arrays
                .as_mut()
                .ok_or(Error::Unavailable("actual beam specular factors"))?;
            windows.surf_refl_fac_bm_to_bm_sol_obs.fill(0.0)?;
            windows.surf_cos_inc_ave_bm_to_bm_sol_obs.fill(0.0)?;
            for hour in 1..=24 {
                self.source_figure_beam_solar_specular_reflection_factors(geometry, hour)?;
            }
        } else {
            let hour = ordinal(globals.hour_of_day)?;
            let count = surface_count(geometry)?;
            let windows = geometry
                .window_arrays
                .as_mut()
                .ok_or(Error::Unavailable("actual beam specular factors"))?;
            for number in 1..=count {
                windows
                    .surf_refl_fac_bm_to_bm_sol_obs
                    .set([hour, number], 0.0)?;
            }
            for number in 1..=count {
                windows
                    .surf_cos_inc_ave_bm_to_bm_sol_obs
                    .set([hour, number], 0.0)?;
            }
            self.source_figure_beam_solar_specular_reflection_factors(geometry, hour)?;
        }
        Ok(())
    }

    fn source_figure_beam_solar_specular_reflection_factors(
        &self,
        geometry: &mut SurfaceGeometryInputState,
        hour: usize,
    ) -> Result<(), Error> {
        let max = extent(geometry.max_rec_pts)?;
        // Native keeps this first, unused array, and leaves the beam array unwritten
        // until the positive NumPossibleObs branch is reached.
        let mut unused_diffuse = SourceSolarArray::<f64, 1>::default();
        let mut refl_beam = SourceSolarArray::<f64, 1>::default();
        let mut weighted_sum = SourceSolarArray::<f64, 1>::default();
        unused_diffuse.dimension([max], None)?;
        refl_beam.dimension([max], None)?;
        weighted_sum.dimension([max], None)?;
        unused_diffuse.fill(0.0)?;
        weighted_sum.fill(0.0)?;
        if geometry
            .window_arrays
            .as_ref()
            .ok_or(Error::Unavailable("actual hourly solar cosine"))?
            .surf_sun_cos_hourly
            .read([hour])?[2]
            < 0.00001
        {
            return Ok(());
        }
        let _sun_vect = geometry
            .window_arrays
            .as_ref()
            .ok_or(Error::Unavailable("actual hourly solar cosine"))?
            .surf_sun_cos_hourly
            .read([hour])?;
        for rec_surf_num in 1..=extent(self.solar_reflection.tot_sol_refl_rec_surf)? {
            let row = receiving(self, rec_surf_num)?;
            let surf_num = ordinal(row.surf_num)?;
            if row.num_possible_obs > 0 {
                refl_beam.fill(0.0)?;
                weighted_sum.fill(0.0)?;
                let num_rec_pts = extent(row.num_rec_pts)?;
                for index in 1..=extent(row.num_possible_obs)? {
                    let refl_surf_num = ordinal(row.possible_obs_surf_nums.read([index])?)?;
                    let surface = source_surface(geometry, refl_surf_num)?;
                    // Preserve the Native OR and its short-circuited glazing read.
                    let reflective =
                        if surface.class == SurfaceClass::Window && surface.exterior_solar {
                            true
                        } else {
                            let glazing = *geometry
                                .parallel_arrays
                                .as_ref()
                                .and_then(|arrays| {
                                    arrays.surf_shadow_glazing_frac.get(refl_surf_num - 1)
                                })
                                .ok_or(Error::Unavailable("actual SurfShadowGlazingFrac"))?;
                            glazing > 0.0 && surface.is_shadowing
                        };
                    if reflective {
                        // Genuine first positive window/specular-shade child950.
                        // POLYF/window transport and its dMax Pierce overload remain
                        // unavailable. No blanket opaque zero/no-op replaces this loop.
                        return Err(Error::Unavailable(
                            "FigureBeamSolSpecularReflFactors actual positive window/specular shade950",
                        ));
                    }
                }
                let windows = geometry
                    .window_arrays
                    .as_mut()
                    .ok_or(Error::Unavailable("actual specular average factors"))?;
                for rec_pt_num in 1..=num_rec_pts {
                    let amount = refl_beam.read([rec_pt_num])?;
                    let cos_inc_weighted = if amount != 0.0 {
                        weighted_sum.read([rec_pt_num])? / amount
                    } else {
                        0.0
                    };
                    add_2(
                        &mut windows.surf_cos_inc_ave_bm_to_bm_sol_obs,
                        hour,
                        surf_num,
                        cos_inc_weighted,
                    )?;
                    add_2(
                        &mut windows.surf_refl_fac_bm_to_bm_sol_obs,
                        hour,
                        surf_num,
                        refl_beam.read([rec_pt_num])?,
                    )?;
                }
                let value = windows
                    .surf_refl_fac_bm_to_bm_sol_obs
                    .read([hour, surf_num])?
                    / num_rec_pts as f64;
                windows
                    .surf_refl_fac_bm_to_bm_sol_obs
                    .set([hour, surf_num], value)?;
                let value = windows
                    .surf_cos_inc_ave_bm_to_bm_sol_obs
                    .read([hour, surf_num])?
                    / num_rec_pts as f64;
                windows
                    .surf_cos_inc_ave_bm_to_bm_sol_obs
                    .set([hour, surf_num], value)?;
            }
        }
        Ok(())
    }

    pub(crate) fn source_calc_sky_solar_diffuse_reflection_factors(
        &mut self,
        geometry: &mut SurfaceGeometryInputState,
        control: &SelectedHeatBalanceInputState,
        _globals: &WeatherGlobalState,
        _environment: &WeatherEnvironmentState,
        shadowing: &SourceShadowingInput,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        let max = extent(geometry.max_rec_pts)?;
        let mut refl_obs = SourceSolarArray::<f64, 1>::default();
        let mut refl_gnd = SourceSolarArray::<f64, 1>::default();
        refl_obs.dimension([max], None)?;
        refl_gnd.dimension([max], None)?;
        let d_phi = std::f64::consts::FRAC_PI_2 / (ALT_ANGLE_STEPS as f64 / 2.0);
        let d_theta = 2.0 * PI / (2.0 * AZIM_ANGLE_STEPS as f64);
        let mut sin_phi = vec![-1.0];
        let mut cos_phi = vec![-1.0];
        for i_phi in 1..=ALT_ANGLE_STEPS / 2 {
            let phi = (i_phi as f64 - 0.5) * d_phi;
            sin_phi.push(phi.sin());
            cos_phi.push(phi.cos());
        }
        let mut sin_theta = vec![-1.0];
        let mut cos_theta = vec![-1.0];
        for i_theta in 1..=2 * AZIM_ANGLE_STEPS {
            let theta = (i_theta as f64 - 0.5) * d_theta;
            sin_theta.push(theta.sin());
            cos_theta.push(theta.cos());
        }
        backend.display_string("Calculating Sky Diffuse Exterior Solar Reflection Factors")?;
        refl_obs.fill(0.0)?;
        refl_gnd.fill(0.0)?;
        let count = surface_count(geometry)?;
        let mut hit_pnt_obs = [0.0; 3];
        for rec_surf_num in 1..=extent(self.solar_reflection.tot_sol_refl_rec_surf)? {
            let row = receiving(self, rec_surf_num)?;
            let surf_num = ordinal(row.surf_num)?;
            for rec_pt_num in 1..=extent(row.num_rec_pts)? {
                refl_obs.set([rec_pt_num], 0.0)?;
                refl_gnd.set([rec_pt_num], 0.0)?;
                for ray_num in 1..=extent(row.num_refl_rays)? {
                    let hit_number_signed = row.hit_pt_surf_num.read([ray_num, rec_pt_num])?;
                    if hit_number_signed == 0 {
                        continue;
                    }
                    let hit_pnt_refl = row.hit_pt.read([ray_num, rec_pt_num])?;
                    if hit_number_signed > 0 {
                        let hit_number = ordinal(hit_number_signed)?;
                        if shelf(geometry, hit_number)? > 0 {
                            continue;
                        }
                        let mut hit_pt_surf_num_x = hit_number;
                        if source_surface(geometry, hit_number)?.is_shadowing
                            && dot(
                                row.ray_vec.read([ray_num])?,
                                geometry_properties(geometry, hit_number)?.out_norm,
                            ) > 0.0
                        {
                            if hit_number + 1 < count {
                                hit_pt_surf_num_x = hit_number + 1;
                            }
                            if shelf(geometry, hit_pt_surf_num_x)? > 0 {
                                continue;
                            }
                        }
                        let view_sky = source_surface(geometry, hit_pt_surf_num_x)?
                            .vertex_progress
                            .view_factor_sky;
                        let sky_radiance = if !shadowing.detailed_sky_diffuse_algorithm
                            || !geometry.shading_transmittance_varies
                            || control.solar_distribution == SelectedSolarDistribution::Minimal
                        {
                            view_sky
                                * self
                                    .solar_timestep
                                    .shading
                                    .surf_dif_shdg_ratio_iso_sky
                                    .read([hit_pt_surf_num_x])?
                                * row.hit_pt_sol_refl.read([ray_num, rec_pt_num])?
                        } else {
                            // Actual literal first timestep/hour, not current clock.
                            view_sky
                                * self
                                    .solar_timestep
                                    .shading
                                    .surf_dif_shdg_ratio_iso_sky_hrts
                                    .read([1, 1, hit_pt_surf_num_x])?
                                * row.hit_pt_sol_refl.read([ray_num, rec_pt_num])?
                        };
                        let delta = sky_radiance
                            * row.d_omega_ray.read([ray_num])?
                            * row.cos_inc_ang_ray.read([ray_num])?
                            / PI;
                        add_1(&mut refl_obs, rec_pt_num, delta)?;
                    } else {
                        // Actual1276--1338: integrate unobstructed upward ground rays.
                        let mut d_refl_sky_gnd = 0.0;
                        let mut u_ray = [0.0; 3];
                        for i_phi in 1..=ALT_ANGLE_STEPS / 2 {
                            u_ray[2] = sin_phi[i_phi];
                            let d_omega = cos_phi[i_phi] * d_theta * d_phi;
                            let cos_inc_to_sky = sin_phi[i_phi];
                            for i_theta in 1..=2 * AZIM_ANGLE_STEPS {
                                u_ray[0] = cos_phi[i_phi] * cos_theta[i_theta];
                                u_ray[1] = cos_phi[i_phi] * sin_theta[i_theta];
                                let mut hit_obs = false;
                                for &obstruction_number in
                                    &geometry.surface_lists.all_shadow_possible_obstructions
                                {
                                    let obs_surf_num = ordinal(obstruction_number)?;
                                    let properties = geometry_properties(geometry, obs_surf_num)?;
                                    if properties.tilt_deg < 5.0 {
                                        continue;
                                    }
                                    if !source_surface(geometry, obs_surf_num)?.is_shadowing {
                                        if dot(u_ray, properties.out_norm) >= 0.0 {
                                            continue;
                                        }
                                        if properties.tilt_deg > 89.0 && properties.tilt_deg < 91.0
                                        {
                                            let surf_vert = source_surface(geometry, obs_surf_num)?
                                                .vertex
                                                .as_ref()
                                                .and_then(|vertices| vertices.get(1))
                                                .copied()
                                                .flatten()
                                                .ok_or(Error::Unavailable(
                                                    "actual reflection obstruction Vertex2",
                                                ))?;
                                            let to_ground = subtract(hit_pnt_refl, surf_vert);
                                            if dot(to_ground, properties.out_norm) < 0.0 {
                                                continue;
                                            }
                                        }
                                    }
                                    hit_obs = pierce_surface(
                                        final_surface(geometry, obs_surf_num)?,
                                        hit_pnt_refl,
                                        u_ray,
                                        &mut hit_pnt_obs,
                                    )?;
                                    if hit_obs {
                                        break;
                                    }
                                }
                                if hit_obs {
                                    continue;
                                }
                                d_refl_sky_gnd += cos_inc_to_sky * d_omega / PI;
                            }
                        }
                        let delta = d_refl_sky_gnd
                            * row.d_omega_ray.read([ray_num])?
                            * row.cos_inc_ang_ray.read([ray_num])?
                            / PI;
                        add_1(&mut refl_gnd, rec_pt_num, delta)?;
                    }
                }
            }
            let windows = geometry
                .window_arrays
                .as_mut()
                .ok_or(Error::Unavailable("actual sky reflection factors"))?;
            windows.surf_refl_fac_sky_sol_obs.set([surf_num], 0.0)?;
            windows.surf_refl_fac_sky_sol_gnd.set([surf_num], 0.0)?;
            let num_rec_pts = extent(row.num_rec_pts)?;
            for rec_pt_num in 1..=num_rec_pts {
                add_1(
                    &mut windows.surf_refl_fac_sky_sol_obs,
                    surf_num,
                    refl_obs.read([rec_pt_num])?,
                )?;
                add_1(
                    &mut windows.surf_refl_fac_sky_sol_gnd,
                    surf_num,
                    refl_gnd.read([rec_pt_num])?,
                )?;
            }
            let value = windows.surf_refl_fac_sky_sol_obs.read([surf_num])? / num_rec_pts as f64;
            windows.surf_refl_fac_sky_sol_obs.set([surf_num], value)?;
            let value = windows.surf_refl_fac_sky_sol_gnd.read([surf_num])? / num_rec_pts as f64;
            windows.surf_refl_fac_sky_sol_gnd.set([surf_num], value)?;
            let limit = 0.5 * (1.0 - geometry_properties(geometry, surf_num)?.cos_tilt);
            let windows = geometry
                .window_arrays
                .as_mut()
                .ok_or(Error::Unavailable("actual sky ground reflection factor"))?;
            let value = windows.surf_refl_fac_sky_sol_gnd.read([surf_num])?;
            windows
                .surf_refl_fac_sky_sol_gnd
                .set([surf_num], if limit < value { limit } else { value })?;
        }
        Ok(())
    }
}
