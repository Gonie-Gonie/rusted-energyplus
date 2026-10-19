//! Actual InitSolarCalculations and opaque floor absorption producer.
use super::*;
use crate::heat_balance::selected_heat_balance_input::shadowing::{
    SourceShadingMethod, SourceShadowingInput,
};

impl OrdinarySolarRadiation {
    pub(crate) fn source_init_solar_calculations(
        &mut self,
        geometry: &mut SurfaceGeometryInputState,
        zones: &SelectedZoneInputOwners,
        control: &SelectedHeatBalanceInputState,
        globals: &WeatherGlobalState,
        environment: &WeatherEnvironmentState,
        shadowing: &SourceShadowingInput,
        arrays: &mut SourceSurfaceHeatBalanceArrays,
        children: &mut dyn SourceSolarChildren,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        if globals.begin_sim_flag {
            children.open_shading_stream(shadowing.output_control_shd, backend)?;
            if self.solar_timestep.get_input_flag {
                // Genuine checkShadingSurfaceSchedules386 range, before process
                // input and before clearing the same actual Solar GetInputFlag.
                // Preserve empty-range behavior, checked first-row access and the first unsupported positive-child failure after prior writes.
                #[allow(clippy::never_loop)]
                for _ in geometry.setup_shading.first..=geometry.setup_shading.last {
                    return Err(Error::Unavailable(
                        "checkShadingSurfaceSchedules actual positive shading surface",
                    ));
                }
                // Selected getter admits the real PolygonClipping enum only.
                // PixelCounting/Imported are never represented as this variant.
                match shadowing.shading_method {
                    SourceShadingMethod::PolygonClipping => {}
                }
                if shadowing.disable_group_self_shading {
                    if shadowing.shading_groups_num > 0 {
                        return Err(Error::Unavailable(
                            "processShadowingInput genuine shading-group child779",
                        ));
                    }
                    return match backend.show_fatal_error(
                        "No Shading groups are defined when disabling grouped self shading.",
                    ) {
                        Ok(never) => match never {},
                        Err(error) => Err(Error::Fatal(error)),
                    };
                }
                self.solar_timestep.get_input_flag = false;
                let vertices = geometry.coordinate.max_vertices_per_surface;
                self.solar_timestep.max_hcv =
                    i32::try_from((((15_usize.max(vertices) + 16) / 16) * 16) - 1)
                        .map_err(|_| Error::Unavailable("actual aligned MaxHCV"))?;
            }
            if self.solar_timestep.first_time {
                backend.display_string("Allocate Solar Module Arrays")?;
            }
            self.allocate_source_solar_module_arrays(
                geometry, zones, control, globals, arrays, children, backend,
            )?;
            if control.solar_distribution != SelectedSolarDistribution::FullInteriorExterior {
                if self.solar_timestep.first_time {
                    backend.display_string("Computing Interior Solar Absorption Factors")?;
                }
                self.source_compute_interior_solar_absorption(geometry, zones, control, backend)?;
            }
            if self.solar_timestep.first_time {
                backend.display_string("Determining Shadowing Combinations")?;
            }
            self.source_determine_minimal_shadowing_combinations(
                geometry,
                control,
                environment,
                shadowing,
            )?;
            // Actual IsConvex state and report order must come from the real
            // earlier geometry caller. The host cannot replace this with an
            // empty writer or assume constructortrue from a fixture shape.
            if !environment.ignore_solar_radiation {
                children.write_shadow_combinations(self, geometry, control, backend)?;
                self.solar_timestep.casting_surface = None; // Actual5727 deallocation after report effects.
            }
            children.reset_shading_stream();
            if self.solar_timestep.first_time {
                backend.display_string("Computing Window Shade Absorption Factors")?;
            }
            let spaces = zones
                .spaces
                .as_ref()
                .ok_or(Error::Unavailable("actual Space allocation"))?;
            for zone in zones
                .zones
                .as_ref()
                .ok_or(Error::Unavailable("actual Zone allocation"))?
            {
                for &space in &zone.space_indexes {
                    let space = spaces
                        .get(ordinal(space)? - 1)
                        .ok_or(Error::Unavailable("actual Source Space association"))?;
                    for number in space.window_first..=space.window_last {
                        let surface = &geometry
                            .final_surfaces
                            .as_ref()
                            .ok_or(Error::Unavailable("same Source Surface"))?
                            [ordinal(number)? - 1]
                            .row;
                        if surface.class == SurfaceClass::Window && surface.has_shade_control {
                            return Err(Error::Unavailable(
                                "ComputeWinShadeAbsorpFactors actual shade child11840",
                            ));
                        }
                    }
                }
            }
            if control.calculate_solar_reflections {
                backend.display_string("Initializing Solar Reflection Factors")?;
                return Err(Error::Unavailable(
                    "InitSolarCalculations actual InitSolReflRecSurf",
                ));
            }
            if self.solar_timestep.first_time {
                backend.display_string("Proceeding with Initializing Solar Calculations")?;
            }
        }
        if globals.begin_envrn_flag {
            self.source_reset_solar_environment(geometry, zones, arrays)?;
        }
        self.append_source_sky_patch_angles();
        Ok(())
    }

    pub(crate) fn source_compute_interior_solar_absorption(
        &mut self,
        geometry: &SurfaceGeometryInputState,
        zones: &SelectedZoneInputOwners,
        control: &SelectedHeatBalanceInputState,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        let zone_rows = zones
            .zones
            .as_ref()
            .ok_or(Error::Unavailable("same actual Zone allocation"))?;
        for enclosure_number in 1..=usize::try_from(self.num_solar_enclosures)
            .map_err(|_| Error::Unavailable("actual NumOfSolarEnclosures"))?
        {
            let enclosure = self
                .solar
                .as_mut()
                .and_then(|rows| rows.get_mut(enclosure_number - 1))
                .ok_or(Error::Unavailable("same Solar enclosure allocation"))?;
            let mut area_sum = 0.0;
            let mut fraction_sum = 0.0;
            let pointers = enclosure
                .source_surface_ptr
                .as_deref()
                .ok_or(Error::Unavailable("actual Solar SurfacePtr"))?;
            for &number in pointers {
                let number = ordinal(number)?;
                let surface = &geometry
                    .final_surfaces
                    .as_ref()
                    .ok_or(Error::Unavailable("same Source final Surface"))?[number - 1]
                    .row;
                let zone = zone_rows
                    .get(ordinal(surface.zone)? - 1)
                    .ok_or(Error::Unavailable("actual Surface Zone ordinal"))?;
                if zone.of_type == 1 && geometry_properties(geometry, number)?.cos_tilt < -0.5 {
                    area_sum += surface.area;
                }
            }
            let horizontal_area = area_sum;
            if enclosure.floor_area <= 0.0 && horizontal_area > 0.0 {
                enclosure.floor_area = horizontal_area;
                backend.show_warning_error("ComputeIntSolarAbsorpFactors: Solar distribution model is set to place solar gains on the zone floor,")?;
                backend.show_continue_error(&format!(
                    "...Enclosure=\"{}\" has no floor, but has approximate horizontal surfaces.",
                    enclosure.name
                ))?;
                return Err(Error::Unavailable(
                    "ComputeIntSolarAbsorpFactors Source {:.2R} horizontal-area diagnostic3339",
                ));
            }
            for &number in pointers {
                let number = ordinal(number)?;
                let surface = &geometry.final_surfaces.as_ref().ok_or(Error::Unavailable(
                    "same actual final Source Surface allocation",
                ))?[number - 1]
                    .row;
                let zone = zone_rows
                    .get(ordinal(surface.zone)? - 1)
                    .ok_or(Error::Unavailable("actual Surface Zone ordinal"))?;
                if (zone.of_type != 1 || geometry_properties(geometry, number)?.cos_tilt < -0.5)
                    && (zone.of_type == 1 || surface.exterior_boundary > 0)
                {
                    let construction_number = *geometry
                        .window_arrays
                        .as_ref()
                        .ok_or(Error::Unavailable("actual active Construction arrays"))?
                        .surf_active_construction
                        .get(number - 1)
                        .ok_or(Error::Unavailable("actual active Construction bounds"))?;
                    let construction = control
                        .construction_usage
                        .get(ordinal(construction_number)? - 1)
                        .ok_or(Error::Unavailable("same Source Construction"))?;
                    if construction.trans_diff <= 0.0 {
                        if area_sum > 0.0 {
                            self.solar_timestep.shading.surf_int_abs_fac.set(
                                [number],
                                surface.area * construction.inside_absorp_solar / area_sum,
                            )?;
                        }
                    } else {
                        return Err(Error::Unavailable(
                            "ComputeIntSolarAbsorpFactors actual transmitting floor3358",
                        ));
                    }
                }
                fraction_sum += self
                    .solar_timestep
                    .shading
                    .surf_int_abs_fac
                    .read([number])?;
            }
            if fraction_sum <= 0.0 && enclosure.exterior_window_area > 0.0 {
                backend.show_severe_error("ComputeIntSolarAbsorpFactors: Solar distribution model is set to place solar gains on the zone floor,")?;
                if enclosure.floor_area <= 0.0 {
                    backend.show_continue_error(&format!(
                        "but Zone or Enclosure =\"{}\" does not appear to have any floor surfaces.",
                        enclosure.name
                    ))?;
                    backend.show_continue_error("Solar gains will be spread evenly on all surfaces in the zone, and the simulation continues...")?;
                } else {
                    backend.show_continue_error(&format!(
                        "but Zone or Enclosure =\"{}\" floor cannot absorb any solar gains. ",
                        enclosure.name
                    ))?;
                    backend.show_continue_error("Check the solar absorptance of the inside layer of the floor surface construction/material.")?;
                    backend.show_continue_error("Solar gains will be spread evenly on all surfaces in the zone, and the simulation continues...")?;
                }
                area_sum = 0.0;
                for &number in pointers {
                    area_sum += source_surface(geometry, ordinal(number)?)?.area;
                }
                for &number in pointers {
                    let number = ordinal(number)?;
                    let construction_number = *geometry
                        .window_arrays
                        .as_ref()
                        .ok_or(Error::Unavailable("actual active Construction arrays"))?
                        .surf_active_construction
                        .get(number - 1)
                        .ok_or(Error::Unavailable("same active Construction"))?;
                    let construction = control
                        .construction_usage
                        .get(ordinal(construction_number)? - 1)
                        .ok_or(Error::Unavailable("same Source Construction"))?;
                    if construction.trans_diff <= 0.0 {
                        if area_sum > 0.0 {
                            self.solar_timestep.shading.surf_int_abs_fac.set(
                                [number],
                                source_surface(geometry, number)?.area
                                    * construction.inside_absorp_solar
                                    / area_sum,
                            )?;
                        }
                    } else {
                        return Err(Error::Unavailable(
                            "ComputeIntSolarAbsorpFactors actual fallback window3410",
                        ));
                    }
                }
            }
        }
        Ok(())
    }
}
