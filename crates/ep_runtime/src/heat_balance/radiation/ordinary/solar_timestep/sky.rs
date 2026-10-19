//! Actual Source Perez components and 6x24 diffuse sky integration.
use super::*;
use std::f64::consts::PI;

const EPS: f64 = 1.0e-10;
const D_PHI: f64 = (PI / 2.0) / 6.0;
const D_THETA: f64 = 2.0 * PI / 24.0;
const D_THETA_D_PHI: f64 = D_THETA * D_PHI;

impl OrdinarySolarRadiation {
    /// InitSolarCalculations364-377. Called on each actual manager invocation,
    /// including BeginEnvrn calls; Source uses push_back, not replace/clear.
    pub(crate) fn append_source_sky_patch_angles(&mut self) {
        for phi in 1..=6 {
            let angle = 0.5 * D_PHI + f64::from(phi - 1) * D_PHI;
            self.solar_timestep.sin_phi.push(angle.sin());
            self.solar_timestep.cos_phi.push(angle.cos());
        }
        for theta in 1..=24 {
            let angle = f64::from(theta - 1) * D_THETA;
            self.solar_timestep.sin_theta.push(angle.sin());
            self.solar_timestep.cos_theta.push(angle.cos());
        }
        self.solar_timestep.first_time = false;
    }

    pub(crate) fn source_anisotropic_sky_view_factors(
        &mut self,
        geometry: &SurfaceGeometryInputState,
        control: &SelectedHeatBalanceInputState,
        globals: &WeatherGlobalState,
        environment: &WeatherEnvironmentState,
        detailed_sky_diffuse: bool,
        shading_transmittance_varies: bool,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        const EPSILON_LIMIT: [f64; 7] = [1.065, 1.23, 1.5, 1.95, 2.8, 4.5, 6.2];
        const F11: [f64; 8] = [
            -0.0083117, 0.1299457, 0.3296958, 0.5682053, 0.8730280, 1.1326077, 1.0601591, 0.6777470,
        ];
        const F12: [f64; 8] = [
            0.5877285, 0.6825954, 0.4868735, 0.1874525, -0.3920403, -1.2367284, -1.5999137,
            -0.3272588,
        ];
        const F13: [f64; 8] = [
            -0.0620636, -0.1513752, -0.2210958, -0.2951290, -0.3616149, -0.4118494, -0.3589221,
            -0.2504286,
        ];
        const F21: [f64; 8] = [
            -0.0596012, -0.0189325, 0.0554140, 0.1088631, 0.2255647, 0.2877813, 0.2642124,
            0.1561313,
        ];
        const F22: [f64; 8] = [
            0.0721249, 0.0659650, -0.0639588, -0.1519229, -0.4620442, -0.8230357, -1.1272340,
            -1.3765031,
        ];
        const F23: [f64; 8] = [
            -0.0220216, -0.0288748, -0.0260542, -0.0139754, 0.0012448, 0.0558651, 0.1310694,
            0.2506212,
        ];
        let weather = &environment.current_weather;
        let cos_zenith = weather.solar_direction_cosines[2];
        let zenith = cos_zenith.acos();
        let zenith_degrees = zenith / (PI / 180.0);
        self.solar_timestep.shading.surf_aniso_sky_mult.fill(0.0)?;
        let air_mass_h = 1.0 - 0.1 * environment.elevation / 1000.0;
        let air_mass = if zenith_degrees <= 75.0 {
            air_mass_h / cos_zenith
        } else {
            air_mass_h / (cos_zenith + 0.15 * (93.9 - zenith_degrees).powf(-1.253))
        };
        let kappa_z3 = 1.041 * (zenith * zenith * zenith);
        let epsilon = ((weather.beam_solar_rad + weather.dif_solar_rad) / weather.dif_solar_rad
            + kappa_z3)
            / (1.0 + kappa_z3);
        let delta = weather.dif_solar_rad * air_mass / 1353.0;
        let mut bin = 0;
        while bin < 8 {
            if bin == 7 || epsilon < EPSILON_LIMIT[bin] {
                break;
            }
            bin += 1;
        }
        let i = bin.clamp(0, F11.len() - 1);
        // Match C++ max(first,second): a generated NaN second operand retains
        // the first operand. No post-hoc finite filter is added to Source math.
        let f1_value = F11[i] + F12[i] * delta + F13[i] * zenith;
        let f1 = if 0.0 < f1_value { f1_value } else { 0.0 };
        let f2 = F21[i] + F22[i] * delta + F23[i] * zenith;
        let hour = ordinal(globals.hour_of_day)?;
        let step = ordinal(globals.time_step)?;
        for &surface_number in &geometry.surface_lists.all_exterior_solar {
            let number = ordinal(surface_number)?;
            let properties = geometry_properties(geometry, number)?;
            let mut cosine = weather.solar_direction_cosines[0] * properties.out_norm[0]
                + weather.solar_direction_cosines[1] * properties.out_norm[1]
                + weather.solar_direction_cosines[2] * properties.out_norm[2];
            if cosine > 1.0 {
                if cosine > 1.0 + 0.0001 {
                    backend.show_severe_error(
                        "Cosine of incident angle of beam solar on surface out of range...too high",
                    )?;
                    backend.show_continue_error("This is a diagnostic error that should not be encountered under normal circumstances")?;
                    let name = &geometry
                        .final_surfaces
                        .as_ref()
                        .ok_or(Error::Unavailable("actual final Surface"))?[number - 1]
                        .row
                        .name;
                    backend.show_continue_error(&format!("Occurs on surface: {name}"))?;
                    // Preserve the actual reached prefix. The fmt {} double
                    // transport is a separate unsupported diagnostic child.
                    return Err(Error::Unavailable(
                        "AnisoSkyViewFactors Source format current value then fatal",
                    ));
                }
                cosine = 1.0;
            } else if cosine < -1.0 {
                if cosine < -1.0 - 0.0001 {
                    backend.show_severe_error(
                        "Cosine of incident angle of beam solar on surface out of range...too low",
                    )?;
                    backend.show_continue_error("This is a diagnostic error that should not be encountered under normal circumstances")?;
                    let name = &geometry
                        .final_surfaces
                        .as_ref()
                        .ok_or(Error::Unavailable("actual final Surface"))?[number - 1]
                        .row
                        .name;
                    backend.show_continue_error(&format!("Occurs on surface: {name}"))?;
                    return Err(Error::Unavailable(
                        "AnisoSkyViewFactors Source format current value then fatal",
                    ));
                }
                cosine = -1.0;
            }
            let sky_view = geometry
                .final_surfaces
                .as_ref()
                .ok_or(Error::Unavailable("actual final Surface"))?[number - 1]
                .row
                .vertex_progress
                .view_factor_sky;
            let isotropic = sky_view * (1.0 - f1);
            self.solar_timestep
                .shading
                .surf_mult_iso_sky
                .set([number], isotropic)?;
            let numerator = if 0.0 < cosine { cosine } else { 0.0 };
            let denominator = if 0.0871557 < cos_zenith {
                cos_zenith
            } else {
                0.0871557
            };
            let mut circumsolar = numerator / denominator;
            if circumsolar > 0.0 && cos_zenith < 0.0871557 && properties.tilt_deg < 2.0 {
                circumsolar = 1.0;
            }
            self.solar_timestep
                .shading
                .surf_mult_circum_solar
                .set([number], f1 * circumsolar)?;
            self.solar_timestep
                .shading
                .surf_mult_horizon_zenith
                .set([number], f2 * properties.sin_tilt)?;
            let sunlit = self
                .solar_timestep
                .heat_balance
                .surf_sunlit_frac
                .read([hour, step, number])?;
            if !detailed_sky_diffuse
                || !shading_transmittance_varies
                || control.solar_distribution == SelectedSolarDistribution::Minimal
            {
                let value = isotropic
                    * self
                        .solar_timestep
                        .shading
                        .surf_dif_shdg_ratio_iso_sky
                        .read([number])?
                    + (f1 * circumsolar) * sunlit
                    + (f2 * properties.sin_tilt)
                        * self
                            .solar_timestep
                            .shading
                            .surf_dif_shdg_ratio_horiz
                            .read([number])?;
                self.solar_timestep
                    .shading
                    .surf_aniso_sky_mult
                    .set([number], value)?;
            } else {
                let value = isotropic
                    * self
                        .solar_timestep
                        .shading
                        .surf_dif_shdg_ratio_iso_sky_hrts
                        .read([step, hour, number])?
                    + (f1 * circumsolar) * sunlit
                    + (f2 * properties.sin_tilt)
                        * self
                            .solar_timestep
                            .shading
                            .surf_dif_shdg_ratio_horiz_hrts
                            .read([step, hour, number])?;
                self.solar_timestep
                    .shading
                    .surf_aniso_sky_mult
                    .set([number], value)?;
                let current_ratio = self
                    .solar_timestep
                    .shading
                    .surf_dif_shdg_ratio_iso_sky_hrts
                    .read([step, hour, number])?;
                self.solar_timestep
                    .shading
                    .surf_cur_dif_shdg_ratio_iso_sky
                    .set([number], current_ratio)?;
            }
            let actual = self
                .solar_timestep
                .shading
                .surf_aniso_sky_mult
                .read([number])?;
            self.solar_timestep
                .shading
                .surf_aniso_sky_mult
                .set([number], if 0.0 < actual { actual } else { 0.0 })?;
        }
        Ok(())
    }

    pub(crate) fn source_sky_diffuse_solar_shading(
        &mut self,
        geometry: &mut SurfaceGeometryInputState,
        control: &SelectedHeatBalanceInputState,
        globals: &WeatherGlobalState,
        detailed_sky_diffuse: bool,
        shading_transmittance_varies: bool,
        children: &mut dyn SourceSolarChildren,
    ) -> Result<(), Error> {
        let count = surface_count(geometry)?;
        let detailed = detailed_sky_diffuse
            && shading_transmittance_varies
            && control.solar_distribution != SelectedSolarDistribution::Minimal;
        self.solar_timestep.shading.surf_sunlit_area.fill(0.0)?;
        self.solar_timestep
            .shading
            .surf_with_shdg_iso_sky
            .dimension([count], Some(0.0))?;
        self.solar_timestep
            .shading
            .surf_wo_shdg_iso_sky
            .dimension([count], Some(0.0))?;
        self.solar_timestep
            .shading
            .surf_with_shdg_horiz
            .dimension([count], Some(0.0))?;
        self.solar_timestep
            .shading
            .surf_wo_shdg_horiz
            .dimension([count], Some(0.0))?;
        self.solar_timestep
            .shading
            .surf_dif_shdg_ratio_iso_sky
            .dimension([count], None)?;
        self.solar_timestep
            .shading
            .surf_dif_shdg_ratio_horiz
            .dimension([count], None)?;
        self.solar_timestep
            .shading
            .surf_dif_shdg_ratio_iso_sky
            .fill(1.0)?;
        self.solar_timestep
            .shading
            .surf_dif_shdg_ratio_horiz
            .fill(1.0)?;
        if detailed {
            let steps = ordinal(globals.time_steps_in_hour)?;
            self.solar_timestep
                .shading
                .surf_cur_dif_shdg_ratio_iso_sky
                .dimension([count], Some(1.0))?;
            self.solar_timestep
                .shading
                .surf_dif_shdg_ratio_iso_sky_hrts
                .dimension([steps, 24, count], None)?;
            self.solar_timestep
                .shading
                .surf_dif_shdg_ratio_iso_sky_hrts
                .fill(1.0)?;
            self.solar_timestep
                .shading
                .surf_dif_shdg_ratio_horiz_hrts
                .dimension([steps, 24, count], None)?;
            self.solar_timestep
                .shading
                .surf_dif_shdg_ratio_horiz_hrts
                .fill(1.0)?;
        }
        children.register_sky_diffuse_outputs(self, geometry, detailed)?;
        for phi in 0..6 {
            self.solar_timestep.sun_cosines[2] = Some(
                *self
                    .solar_timestep
                    .sin_phi
                    .get(phi)
                    .ok_or(Error::Unavailable("actual InitSolar patch sine"))?,
            );
            for theta in 0..24 {
                let cos_phi = *self
                    .solar_timestep
                    .cos_phi
                    .get(phi)
                    .ok_or(Error::Unavailable("actual InitSolar patch cosine"))?;
                self.solar_timestep.sun_cosines[0] =
                    Some(
                        cos_phi
                            * self.solar_timestep.cos_theta.get(theta).copied().ok_or(
                                Error::Unavailable("actual InitSolar patch azimuth cosine"),
                            )?,
                    );
                self.solar_timestep.sun_cosines[1] = Some(
                    cos_phi
                        * self
                            .solar_timestep
                            .sin_theta
                            .get(theta)
                            .copied()
                            .ok_or(Error::Unavailable("actual InitSolar patch azimuth sine"))?,
                );
                for &number in &geometry.surface_lists.all_exterior_solar_and_shading {
                    let number = ordinal(number)?;
                    let normal = geometry_properties(geometry, number)?.out_norm;
                    let cosine = self.solar_timestep.sun_cosines[0]
                        .ok_or(Error::Unavailable("actual Source Sun4 SOLCOS(1)"))?
                        * normal[0]
                        + self.solar_timestep.sun_cosines[1]
                            .ok_or(Error::Unavailable("actual Source Sun4 SOLCOS(2)"))?
                            * normal[1]
                        + self.solar_timestep.sun_cosines[2]
                            .ok_or(Error::Unavailable("actual Source Sun4 SOLCOS(3)"))?
                            * normal[2];
                    self.solar_timestep
                        .shading
                        .surf_sun_cos_theta
                        .set([number], cosine)?;
                }
                self.source_shadow(geometry, 24, 0)?;
                for &number in &geometry.surface_lists.all_exterior_solar_and_shading {
                    let number = ordinal(number)?;
                    let cosine = self
                        .solar_timestep
                        .shading
                        .surf_sun_cos_theta
                        .read([number])?;
                    if cosine < 0.0 {
                        continue;
                    }
                    let without = cos_phi * D_THETA_D_PHI * cosine;
                    let area = source_surface(geometry, number)?.net_area_shadow_calc;
                    let fraction = self
                        .solar_timestep
                        .shading
                        .surf_sunlit_area
                        .read([number])?
                        / if area > EPS { area } else { area + EPS };
                    let with = without * fraction;
                    let shading = &mut self.solar_timestep.shading;
                    shading.surf_with_shdg_iso_sky.set(
                        [number],
                        shading.surf_with_shdg_iso_sky.read([number])? + with,
                    )?;
                    shading.surf_wo_shdg_iso_sky.set(
                        [number],
                        shading.surf_wo_shdg_iso_sky.read([number])? + without,
                    )?;
                    if phi == 0 {
                        shading.surf_with_shdg_horiz.set(
                            [number],
                            shading.surf_with_shdg_horiz.read([number])? + with,
                        )?;
                        shading.surf_wo_shdg_horiz.set(
                            [number],
                            shading.surf_wo_shdg_horiz.read([number])? + without,
                        )?;
                    }
                }
            }
        }
        for &number in &geometry.surface_lists.all_exterior_solar_and_shading {
            let number = ordinal(number)?;
            let shading = &mut self.solar_timestep.shading;
            let without = shading.surf_wo_shdg_iso_sky.read([number])?;
            shading.surf_dif_shdg_ratio_iso_sky.set(
                [number],
                shading.surf_with_shdg_iso_sky.read([number])?
                    / if without.abs() > EPS {
                        without
                    } else {
                        without + EPS
                    },
            )?;
            let without = shading.surf_wo_shdg_horiz.read([number])?;
            shading.surf_dif_shdg_ratio_horiz.set(
                [number],
                shading.surf_with_shdg_horiz.read([number])?
                    / if without.abs() > EPS {
                        without
                    } else {
                        without + EPS
                    },
            )?;
        }
        for number in 1..=count {
            let ratio = if detailed {
                self.solar_timestep
                    .shading
                    .surf_dif_shdg_ratio_iso_sky_hrts
                    .read([1, 1, number])?
            } else {
                self.solar_timestep
                    .shading
                    .surf_dif_shdg_ratio_iso_sky
                    .read([number])?
            };
            let final_surface = &mut geometry
                .final_surfaces
                .as_mut()
                .ok_or(Error::Unavailable("actual final Surface"))?[number - 1];
            let surface = &mut final_surface.row;
            surface.vertex_progress.view_factor_sky_ir *= ratio;
            surface.vertex_progress.view_factor_ground_ir =
                1.0 - surface.vertex_progress.view_factor_sky_ir;
            if final_surface.has_surrounding_surface_property {
                surface.vertex_progress.view_factor_ground_ir = 1.0
                    - surface.vertex_progress.view_factor_sky_ir
                    - final_surface.surrounding_surface_view_factor;
            }
        }
        if detailed {
            for number in 1..=count {
                let isotropic = self
                    .solar_timestep
                    .shading
                    .surf_dif_shdg_ratio_iso_sky
                    .read([number])?;
                for hour in 1..=24 {
                    for step in 1..=ordinal(globals.time_steps_in_hour)? {
                        self.solar_timestep
                            .shading
                            .surf_dif_shdg_ratio_iso_sky_hrts
                            .set([step, hour, number], isotropic)?;
                    }
                }
                let horizon = self
                    .solar_timestep
                    .shading
                    .surf_dif_shdg_ratio_horiz
                    .read([number])?;
                for hour in 1..=24 {
                    for step in 1..=ordinal(globals.time_steps_in_hour)? {
                        self.solar_timestep
                            .shading
                            .surf_dif_shdg_ratio_horiz_hrts
                            .set([step, hour, number], horizon)?;
                    }
                }
            }
        }
        Ok(())
    }
}
