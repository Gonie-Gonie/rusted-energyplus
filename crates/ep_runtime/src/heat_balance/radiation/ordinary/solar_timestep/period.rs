//! Actual MinimalShadowing combination, SHADOW and period SUN3/SUN4 producers.
use super::state::SourceShadowCombination;
use super::*;
use crate::heat_balance::selected_heat_balance_input::shadowing::SourceShadowingInput;
use std::f64::consts::PI;

fn sun3(day: i32) -> (f64, f64) {
    const DECLINATION: [f64; 9] = [
        0.00561800,
        0.0657911,
        -0.392779,
        0.00064440,
        -0.00618495,
        -0.00010101,
        -0.00007951,
        -0.00011691,
        0.00002096,
    ];
    const EQUATION: [f64; 9] = [
        0.00021971,
        -0.122649,
        0.00762856,
        -0.156308,
        -0.0530028,
        -0.00388702,
        -0.00123978,
        -0.00270502,
        -0.00167992,
    ];
    let x = 0.017167 * f64::from(day);
    let sine = x.sin();
    let cosine = x.cos();
    // Original9466-9478 expression grouping retained; no different-period
    // weather solar helper or a trigonometric recurrence replaces SUN3.
    let evaluate = |c: [f64; 9]| {
        c[0] + c[1] * sine
            + c[2] * cosine
            + c[3] * (sine * cosine * 2.0)
            + c[4] * (cosine * cosine - sine * sine)
            + c[5] * (sine * (cosine * cosine - sine * sine) + cosine * (sine * cosine * 2.0))
            + c[6] * (cosine * (cosine * cosine - sine * sine) - sine * (sine * cosine * 2.0))
            + c[7] * (2.0 * (sine * cosine * 2.0) * (cosine * cosine - sine * sine))
            + c[8]
                * ((cosine * cosine - sine * sine) * (cosine * cosine - sine * sine)
                    - (sine * cosine * 2.0) * (sine * cosine * 2.0))
    };
    (evaluate(DECLINATION), evaluate(EQUATION))
}

impl OrdinarySolarRadiation {
    fn source_init_complex_windows(&mut self) -> Result<(), Error> {
        if self.solar_timestep.complex.init_complex_windows_once {
            self.solar_timestep.complex.init_complex_windows_once = false;
            // Actual InitBSDFWindows104-144 constructs local V(3) before its
            // genuine TotComplexFenStates<=0 early return.
            let _v = [None::<f64>; 3];
            if self
                .solar_timestep
                .complex
                .total_complex_fenestration_states
                > 0
            {
                return Err(Error::Unavailable(
                    "InitBSDFWindows actual positive complex states",
                ));
            }
            // Actual CalcStaticProperties924 iterates NumComplexWind, with no
            // unconditional writes before that loop.
            // Preserve empty-range behavior, checked first-row access and the first unsupported positive-child failure after prior writes.
            #[allow(clippy::never_loop)]
            for _ in 1..=self.solar_timestep.complex.number_complex_windows {
                return Err(Error::Unavailable(
                    "CalcStaticProperties actual positive complex window",
                ));
            }
        }
        Ok(())
    }
    /// Actual5297-5619 Minimal branch. The real SHD report child follows this
    /// numerical child at5621; it must finish before its stream is reset.
    pub(crate) fn source_determine_minimal_shadowing_combinations(
        &mut self,
        geometry: &SurfaceGeometryInputState,
        control: &SelectedHeatBalanceInputState,
        environment: &WeatherEnvironmentState,
        shadowing: &SourceShadowingInput,
    ) -> Result<(), Error> {
        let count = surface_count(geometry)?;
        self.solar_timestep.shadow_combinations =
            Some(vec![SourceShadowCombination::default(); count]);
        self.solar_timestep.casting_surface = Some(vec![false; count]);
        let figures = 2 * shadowing.max_hcs;
        let vertices = usize::try_from(self.solar_timestep.max_hcv + 1)
            .map_err(|_| Error::Unavailable("actual MaxHCV+1"))?;
        self.solar_timestep
            .hc_a
            .dimension([figures, vertices], Some(0))?;
        self.solar_timestep
            .hc_b
            .dimension([figures, vertices], Some(0))?;
        self.solar_timestep
            .hc_c
            .dimension([figures, vertices], Some(0))?;
        self.solar_timestep
            .hc_x
            .dimension([figures, vertices], Some(0))?;
        self.solar_timestep
            .hc_y
            .dimension([figures, vertices], Some(0))?;
        self.solar_timestep
            .hc_area
            .dimension([figures], Some(0.0))?;
        self.solar_timestep.hc_ns.dimension([figures], Some(0))?;
        self.solar_timestep.hc_nv.dimension([figures], Some(0))?;
        self.solar_timestep.hc_t.dimension([figures], Some(0.0))?;
        let mut general = vec![0_i32; ordinal(self.solar_timestep.max_gss)?];
        let _back = vec![0_i32; ordinal(self.solar_timestep.max_gss)?];
        let _sub = vec![0_i32; ordinal(self.solar_timestep.max_gss)?];
        self.solar_timestep.penumbra_ids.clear();
        if environment.ignore_solar_radiation {
            self.solar_timestep.casting_surface = None;
            return Ok(());
        }
        let surfaces = geometry
            .final_surfaces
            .as_ref()
            .ok_or(Error::Unavailable("actual final Surfaces"))?;
        for number in 1..=count {
            let surface = &surfaces[number - 1].row;
            let shadowing = surface.is_shadowing;
            if !shadowing && !surface.heat_transfer_surface {
                continue;
            }
            if !shadowing && !surface.exterior_solar {
                continue;
            }
            if !shadowing && ordinal(surface.base_surface)? != number {
                continue;
            }
            let vertex = surface
                .vertex
                .as_ref()
                .ok_or(Error::Unavailable("same actual Source Vertex"))?;
            let mut z_min = vertex
                .first()
                .copied()
                .flatten()
                .ok_or(Error::Unavailable("actual written Vertex1"))?[2];
            for value in vertex
                .iter()
                .skip(1)
                .take(ordinal(surface.sides)?.saturating_sub(1))
            {
                let z = value.ok_or(Error::Unavailable("actual written Vertex"))?[2];
                if z < z_min {
                    z_min = z;
                }
            }
            let general_count = self.source_general_shadow_candidates(
                geometry,
                control,
                number,
                shadowing,
                z_min,
                &mut general,
            )?;
            // Source5518: inspect every real row; an actually reached subsurface
            // requires CHKSBS, so it cannot be silently treated as count0.
            for candidate in 1..=count {
                let sub = &surfaces[candidate - 1].row;
                if !sub.heat_transfer_surface
                    || candidate == number
                    || ordinal(sub.base_surface)? != number
                {
                    continue;
                }
                return Err(Error::Unavailable(
                    "DetermineShadowingCombinations CHKSBS reached subsurface",
                ));
            }
            let combination = &mut self
                .solar_timestep
                .shadow_combinations
                .as_mut()
                .ok_or(Error::Unavailable("actual retained ShadowComb allocation"))?[number - 1];
            combination.use_this_surface = true;
            combination.num_gen_surface = i32::try_from(general_count)
                .map_err(|_| Error::Unavailable("actual NGSS signed storage"))?;
            combination.num_back_surface = 0;
            combination.num_sub_surface = 0;
            self.solar_timestep.max_dim =
                self.solar_timestep.max_dim.max(combination.num_gen_surface);
            let mut retained_general = vec![0; general_count + 1];
            retained_general[1..].copy_from_slice(&general[..general_count]);
            combination.general = Some(retained_general);
            combination.back = Some(vec![0]);
            combination.sub = Some(vec![0]);
        }
        // The three genuine temporary GSS/SBS/BKS vectors are dropped here,
        // matching Source5617-5619 deallocation before report effects.
        Ok(())
    }

    pub(crate) fn source_shadow(
        &mut self,
        geometry: &SurfaceGeometryInputState,
        _hour: usize,
        _step: usize,
    ) -> Result<(), Error> {
        if self.solar_timestep.shadow_one_time_flag {
            let vertices = geometry.coordinate.max_vertices_per_surface + 1;
            self.solar_timestep.x_vrt.dimension([vertices], None)?;
            self.solar_timestep.y_vrt.dimension([vertices], None)?;
            self.solar_timestep.z_vrt.dimension([vertices], None)?;
            self.solar_timestep.x_vrt.fill(0.0)?;
            self.solar_timestep.y_vrt.fill(0.0)?;
            self.solar_timestep.z_vrt.fill(0.0)?;
            self.solar_timestep.shadow_one_time_flag = false;
        }
        self.solar_timestep.shading.surf_sunlit_area.fill(0.0)?;
        for number in 1..=surface_count(geometry)? {
            let combination = self
                .solar_timestep
                .shadow_combinations
                .as_ref()
                .and_then(|rows| rows.get(number - 1))
                .ok_or(Error::Unavailable("actual ShadowComb allocation"))?;
            if !combination.use_this_surface {
                continue;
            }
            self.solar_timestep
                .shading
                .surf_sunlit_area
                .set([number], 0.0)?;
            let general = combination.num_gen_surface;
            self.solar_timestep.ngsshc = 0;
            let back = combination.num_back_surface;
            self.solar_timestep.nbkshc = 0;
            let sub = combination.num_sub_surface;
            self.solar_timestep.nrvlhc = 0;
            self.solar_timestep.nsbshc = 0;
            self.solar_timestep.lochca = 1;
            if self
                .solar_timestep
                .shading
                .surf_sun_cos_theta
                .read([number])?
                < 0.00001
            {
                self.solar_timestep
                    .shading
                    .surf_sunlit_area
                    .set([number], 0.0)?;
                // Actual SHDSBS9302 does nothing iff NSBS<=0. Neither a clock
                // nor an admission boolean is used as the subsurface count.
                if sub > 0 {
                    return Err(Error::Unavailable("SHDSBS reached positive NSBS"));
                }
                let _ = back;
            } else if general <= 0 && sub <= 0 {
                self.solar_timestep.shading.surf_sunlit_area.set(
                    [number],
                    source_surface(geometry, number)?.net_area_shadow_calc,
                )?;
            } else {
                return Err(Error::Unavailable(
                    "SHADOW reached polygon or subsurface child",
                ));
            }
        }
        Ok(())
    }

    fn source_sun4(
        &mut self,
        time: f64,
        equation: f64,
        sin_declination: f64,
        cos_declination: f64,
        environment: &WeatherEnvironmentState,
    ) {
        let hour_angle = 15.0 * (12.0 - (time + equation))
            + (environment.time_zone_meridian - environment.longitude);
        let h = hour_angle * (PI / 180.0);
        let z = sin_declination * environment.sin_latitude
            + cos_declination * environment.cos_latitude * h.cos();
        self.solar_timestep.sun_cosines[2] = Some(z);
        self.solar_timestep.sun_cosines[1] = Some(0.0);
        self.solar_timestep.sun_cosines[0] = Some(0.0);
        if z < 0.00001 {
            return;
        }
        self.solar_timestep.sun_cosines[1] = Some(
            sin_declination * environment.cos_latitude
                - cos_declination * environment.sin_latitude * h.cos(),
        );
        self.solar_timestep.sun_cosines[0] = Some(cos_declination * h.sin());
    }

    pub(crate) fn source_period_solar_beam(
        &mut self,
        geometry: &mut SurfaceGeometryInputState,
        control: &SelectedHeatBalanceInputState,
        globals: &WeatherGlobalState,
        environment: &WeatherEnvironmentState,
        shadowing: &SourceShadowingInput,
        kickoff_sizing: bool,
        kickoff_simulation: bool,
        equation: f64,
        sin_declination: f64,
        cos_declination: f64,
    ) -> Result<(), Error> {
        if self.solar_timestep.init_complex_once {
            self.source_init_complex_windows()?;
        }
        self.solar_timestep.init_complex_once = false;
        if kickoff_sizing || kickoff_simulation {
            return Ok(());
        }
        let steps = ordinal(globals.time_steps_in_hour)?;
        let count = surface_count(geometry)?;
        let detailed = shadowing.detailed_solar_timestep_integration;
        let hours: Vec<usize> = if detailed {
            vec![ordinal(globals.hour_of_day)?]
        } else {
            (1..=24).collect()
        };
        let time_steps: Vec<usize> = if detailed {
            vec![ordinal(globals.time_step)?]
        } else {
            (1..=steps).collect()
        };
        if !detailed {
            for number in 1..=count {
                let windows = geometry
                    .window_arrays
                    .as_mut()
                    .ok_or(Error::Unavailable("actual Solar DataSurface allocation"))?;
                windows.surf_opaq_ao.set([number], 0.0)?;
                self.solar_timestep
                    .shading
                    .surf_sun_cos_theta
                    .set([number], 0.0)?;
                let window = geometry
                    .surface_windows
                    .as_mut()
                    .and_then(|rows| rows.get_mut(number - 1))
                    .ok_or(Error::Unavailable(
                        "same SurfaceWindow row including opaque",
                    ))?;
                for &hour in &hours {
                    window.outside_projection_sunlit_multiplier[hour] = 1.0;
                    window.inside_outside_projection_sunlit_multiplier[hour] = 1.0;
                }
            }
            for &hour in &hours {
                for number in 1..=count {
                    self.solar_timestep
                        .heat_balance
                        .surf_sunlit_frac_hr
                        .set([hour, number], 0.0)?;
                    self.solar_timestep
                        .heat_balance
                        .surf_cos_inc_ang_hr
                        .set([hour, number], 0.0)?;
                }
            }
            for &hour in &hours {
                for &step in &time_steps {
                    for number in 1..=count {
                        self.solar_timestep
                            .heat_balance
                            .surf_sunlit_frac
                            .set([hour, step, number], 0.0)?;
                        self.solar_timestep
                            .heat_balance
                            .surf_cos_inc_ang
                            .set([hour, step, number], 0.0)?;
                        self.solar_timestep
                            .heat_balance
                            .surf_sunlit_frac_without_reveal
                            .set([hour, step, number], 0.0)?;
                    }
                }
            }
            let max_back = self
                .solar_timestep
                .heat_balance
                .surf_win_back_surfaces
                .dimensions
                .ok_or(Error::Unavailable("same actual MaxBkSurf allocation"))?[2];
            for &hour in &hours {
                for &step in &time_steps {
                    for back in 1..=max_back {
                        for number in 1..=count {
                            self.solar_timestep
                                .heat_balance
                                .surf_win_back_surfaces
                                .set([hour, step, back, number], 0)?;
                            self.solar_timestep
                                .heat_balance
                                .surf_win_overlap_areas
                                .set([hour, step, back, number], 0.0)?;
                        }
                    }
                }
            }
        } else {
            let hour = ordinal(globals.hour_of_day)?;
            let step = ordinal(globals.time_step)?;
            for number in 1..=count {
                self.solar_timestep
                    .shading
                    .surf_sun_cos_theta
                    .set([number], 0.0)?;
                geometry
                    .window_arrays
                    .as_mut()
                    .ok_or(Error::Unavailable("actual Solar DataSurface allocation"))?
                    .surf_opaq_ao
                    .set([number], 0.0)?;
                let window = geometry
                    .surface_windows
                    .as_mut()
                    .and_then(|rows| rows.get_mut(number - 1))
                    .ok_or(Error::Unavailable(
                        "same SurfaceWindow row including opaque",
                    ))?;
                window.outside_projection_sunlit_multiplier[hour] = 1.0;
                window.inside_outside_projection_sunlit_multiplier[hour] = 1.0;
                self.solar_timestep
                    .heat_balance
                    .surf_sunlit_frac_hr
                    .set([hour, number], 0.0)?;
                self.solar_timestep
                    .heat_balance
                    .surf_cos_inc_ang_hr
                    .set([hour, number], 0.0)?;
                self.solar_timestep
                    .heat_balance
                    .surf_sunlit_frac
                    .set([hour, step, number], 0.0)?;
                self.solar_timestep
                    .heat_balance
                    .surf_sunlit_frac_without_reveal
                    .set([hour, step, number], 0.0)?;
                self.solar_timestep
                    .heat_balance
                    .surf_cos_inc_ang
                    .set([hour, step, number], 0.0)?;
            }
            let max_back = self
                .solar_timestep
                .heat_balance
                .surf_win_back_surfaces
                .dimensions
                .ok_or(Error::Unavailable("same actual MaxBkSurf allocation"))?[2];
            for back in 1..=max_back {
                for number in 1..=count {
                    self.solar_timestep
                        .heat_balance
                        .surf_win_back_surfaces
                        .set([hour, step, back, number], 0)?;
                    self.solar_timestep
                        .heat_balance
                        .surf_win_overlap_areas
                        .set([hour, step, back, number], 0.0)?;
                }
            }
        }
        for &hour in &hours {
            for &step in &time_steps {
                let time = if steps != 1 {
                    (hour - 1) as f64 + step as f64 * globals.time_step_zone
                } else {
                    hour as f64 + environment.one_step_solar_time_offset
                };
                self.source_sun4(
                    time,
                    equation,
                    sin_declination,
                    cos_declination,
                    environment,
                );
                let sun = [
                    self.solar_timestep.sun_cosines[0]
                        .ok_or(Error::Unavailable("actual Source Sun4 SOLCOS(1)"))?,
                    self.solar_timestep.sun_cosines[1]
                        .ok_or(Error::Unavailable("actual Source Sun4 SOLCOS(2)"))?,
                    self.solar_timestep.sun_cosines[2]
                        .ok_or(Error::Unavailable("actual Source Sun4 SOLCOS(3)"))?,
                ];
                if step == steps || detailed {
                    geometry
                        .window_arrays
                        .as_mut()
                        .ok_or(Error::Unavailable("actual SurfSunCosHourly"))?
                        .surf_sun_cos_hourly
                        .set([hour], sun)?;
                }
                self.solar_timestep
                    .complex
                    .sun_costs
                    .set([step, hour], sun)?;
            }
        }
        // Actual UpdateComplexWindows542 returns first on NumComplexWind==0,
        // before kickoff flags. No owner/flag is cleared on that return.
        if self.solar_timestep.complex.number_complex_windows != 0
            && !(kickoff_sizing || kickoff_simulation)
        {
            return Err(Error::Unavailable(
                "UpdateComplexWindows actual positive complex windows",
            ));
        }
        for &hour in &hours {
            for &step in &time_steps {
                self.source_figure_solar_beam_at_timestep(
                    geometry, control, globals, shadowing, hour, step,
                )?;
            }
        }
        Ok(())
    }

    /// SolarShading5070-5245. The same admitted Figure body is used by the
    /// genuine CalcPerSolarBeam caller and by a direct unit caller. It reads the
    /// already-produced SUNCOSTS cell; it never computes or seeds that value.
    pub(crate) fn source_figure_solar_beam_at_timestep(
        &mut self,
        geometry: &mut SurfaceGeometryInputState,
        control: &SelectedHeatBalanceInputState,
        globals: &WeatherGlobalState,
        shadowing: &SourceShadowingInput,
        hour: usize,
        step: usize,
    ) -> Result<(), Error> {
        let count = surface_count(geometry)?;
        let steps = ordinal(globals.time_steps_in_hour)?;
        let detailed = shadowing.detailed_solar_timestep_integration;
        let sun = self.solar_timestep.complex.sun_costs.read([step, hour])?;
        self.solar_timestep.sun_cosines = sun.map(Some);
        self.solar_timestep.shading.surf_sun_cos_theta.fill(0.0)?;
        if sun[2] < 0.00001 {
            return Ok(());
        }
        for number in 1..=count {
            let normal = geometry_properties(geometry, number)?.out_norm;
            let cosine = sun[0] * normal[0] + sun[1] * normal[1] + sun[2] * normal[2];
            self.solar_timestep
                .shading
                .surf_sun_cos_theta
                .set([number], cosine)?;
            if step == steps || detailed {
                self.solar_timestep
                    .heat_balance
                    .surf_cos_inc_ang_hr
                    .set([hour, number], cosine)?;
            }
            self.solar_timestep
                .heat_balance
                .surf_cos_inc_ang
                .set([hour, step, number], cosine)?;
        }
        // SourceShadingMethod currently has exactly the real admitted
        // PolygonClipping variant; Scheduled/Imported remain input-bound.
        self.source_shadow(geometry, hour, step)?;
        for number in 1..=count {
            let surface = source_surface(geometry, number)?;
            if surface.area >= 1.0e-10 {
                let fraction = self
                    .solar_timestep
                    .shading
                    .surf_sunlit_area
                    .read([number])?
                    / surface.net_area_shadow_calc;
                if step == steps || detailed {
                    self.solar_timestep
                        .heat_balance
                        .surf_sunlit_frac_hr
                        .set([hour, number], fraction)?;
                }
                self.solar_timestep
                    .heat_balance
                    .surf_sunlit_frac
                    .set([hour, step, number], fraction)?;
                if fraction < 1.0e-5 {
                    self.solar_timestep
                        .heat_balance
                        .surf_sunlit_frac
                        .set([hour, step, number], 0.0)?;
                }
            }
            if self
                .solar_timestep
                .heat_balance
                .surf_sunlit_frac
                .read([hour, step, number])?
                > 1.0
            {
                self.solar_timestep
                    .heat_balance
                    .surf_sunlit_frac
                    .set([hour, step, number], 1.0)?;
            }
        }
        if shadowing.detailed_sky_diffuse_algorithm
            && geometry.shading_transmittance_varies
            && control.solar_distribution != SelectedSolarDistribution::Minimal
        {
            return Err(Error::Unavailable(
                "FigureSolarBeamAtTimestep actual detailed diffuse shading child",
            ));
        }
        for &number in &geometry.surface_lists.all_exterior_solar_window_with_frame {
            if self.solar_timestep.heat_balance.surf_sunlit_frac.read([
                hour,
                step,
                ordinal(number)?,
            ])? > 0.0
            {
                return Err(Error::Unavailable(
                    "FigureSolarBeamAtTimestep actual CalcFrameDividerShadow",
                ));
            }
        }
        Ok(())
    }

    /// Actual PerformSolarCalculations9001-9069. The display/report-flag child
    /// is required at9022 before SUN3 and consumes the real calendar strings.
    // Keep the Original explicit counter and checked row-access order.
    #[allow(clippy::explicit_counter_loop)]
    pub(crate) fn source_perform_solar_calculations(
        &mut self,
        geometry: &mut SurfaceGeometryInputState,
        control: &SelectedHeatBalanceInputState,
        globals: &WeatherGlobalState,
        environment: &WeatherEnvironmentState,
        shadowing: &SourceShadowingInput,
        kickoff_sizing: bool,
        kickoff_simulation: bool,
        children: &mut dyn SourceSolarChildren,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        if globals.begin_sim_flag {
            self.solar_timestep.calc_sky_dif_shading = true;
            let transmittance_varies = geometry.shading_transmittance_varies;
            self.source_sky_diffuse_solar_shading(
                geometry,
                control,
                globals,
                shadowing.detailed_sky_diffuse_algorithm,
                transmittance_varies,
                children,
                backend,
            )?;
            self.solar_timestep.calc_sky_dif_shading = false;
        }
        if globals.begin_envrn_flag {
            self.solar_timestep.shadowing_days_left = 0;
        }
        if self.solar_timestep.shadowing_days_left <= 0
            || shadowing.detailed_solar_timestep_integration
        {
            let (sin_declination, cos_declination, equation) = if !shadowing
                .detailed_solar_timestep_integration
            {
                self.solar_timestep.shadowing_days_left = shadowing.calculation_frequency;
                if globals.day_of_sim + self.solar_timestep.shadowing_days_left
                    > globals.num_of_day_in_envrn
                {
                    self.solar_timestep.shadowing_days_left =
                        globals.num_of_day_in_envrn - globals.day_of_sim + 1;
                }
                if !globals.warmup_flag {
                    children.display_shadowing_update(globals, environment, backend)?;
                }
                let mut day = environment.day_of_year;
                let mut sum_declination = 0.0;
                let mut sum_equation = 0.0;
                for _ in 1..=self.solar_timestep.shadowing_days_left {
                    let (sine, equation) = sun3(day);
                    sum_declination += sine;
                    sum_equation += equation;
                    day += 1;
                }
                let sine = sum_declination / f64::from(self.solar_timestep.shadowing_days_left);
                (
                    sine,
                    (1.0 - sine * sine).sqrt(),
                    sum_equation / f64::from(self.solar_timestep.shadowing_days_left),
                )
            } else {
                let (sine, equation) = sun3(environment.day_of_year);
                if !globals.warmup_flag && globals.begin_day_flag && globals.day_of_sim % 14 == 0 {
                    children.set_display_performance_simulation_flag()?;
                }
                (sine, (1.0 - sine * sine).sqrt(), equation)
            };
            self.source_period_solar_beam(
                geometry,
                control,
                globals,
                environment,
                shadowing,
                kickoff_sizing,
                kickoff_simulation,
                equation,
                sin_declination,
                cos_declination,
            )?;
            if control.calculate_solar_reflections {
                return Err(Error::Unavailable(
                    "PerformSolarCalculations actual reflected-solar children",
                ));
            }
            children.calc_daylight_coefficients(self, geometry, globals, backend)?;
        }
        if !globals.warmup_flag {
            self.solar_timestep.shadowing_days_left -= 1;
        }
        if geometry.total_storm_windows > 0 {
            return Err(Error::Unavailable(
                "PerformSolarCalculations actual storm/daylight recalc predicate owners",
            ));
        }
        Ok(())
    }
}
