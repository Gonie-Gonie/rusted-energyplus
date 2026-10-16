//! Actual AllocateSurfaceArrays804–929 and the one outdoor-owner transfer.
//! Separate fields preserve each Source array; no second mutable outdoor copy.
use super::{Error, SurfaceGeometryInputState};
use crate::heat_balance::convection::surface_environment::SurfaceOutdoorEnvironmentState;
use crate::heat_balance::state::SurfaceHeatBalanceState;
use ep_model::TypedModel;

#[allow(dead_code)] // Exact retained constructors before later geometry stages.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct ShadingVertex {
    pub(crate) n_vert: i32,
    pub(crate) xv: Option<Vec<f64>>,
    pub(crate) yv: Option<Vec<f64>>,
    pub(crate) zv: Option<Vec<f64>>,
}
#[allow(dead_code)]
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct MovableInsulation {
    pub(crate) present: bool,
    pub(crate) present_previous_time_step: bool,
    pub(crate) h: f64,
    pub(crate) material_num: i32,
    pub(crate) schedule: Option<std::convert::Infallible>,
}
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct InteriorConvection {
    pub(crate) class: i32,
    pub(crate) class_report: i32,
    pub(crate) model: i32,
    pub(crate) user_model_num: i32,
    pub(crate) model_equation: i32,
    pub(crate) model_equation_report: i32,
    pub(crate) user_curve_num: i32,
    pub(crate) zone_wall_height: f64,
    pub(crate) zone_perimeter: f64,
    pub(crate) zone_horizontal_hydraulic_diameter: f64,
    pub(crate) window_wall_ratio: f64,
    pub(crate) window_location: i32,
    pub(crate) gets_radiant_heat: bool,
    pub(crate) has_active_in_it: bool,
}
impl Default for InteriorConvection {
    fn default() -> Self {
        Self {
            class: -1,
            class_report: -1,
            model: 2,
            user_model_num: 0,
            model_equation: -1,
            model_equation_report: -1,
            user_curve_num: 0,
            zone_wall_height: 0.0,
            zone_perimeter: 0.0,
            zone_horizontal_hydraulic_diameter: 0.0,
            window_wall_ratio: 0.0,
            window_location: 0,
            gets_radiant_heat: false,
            has_active_in_it: false,
        }
    }
}
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ExteriorConvection {
    pub(crate) class: i32,
    pub(crate) class_report: i32,
    pub(crate) model: i32,
    pub(crate) user_model_num: i32,
    pub(crate) forced_model_equation: i32,
    pub(crate) forced_model_report: i32,
    pub(crate) forced_user_curve_num: i32,
    pub(crate) natural_model_equation: i32,
    pub(crate) natural_model_report: i32,
    pub(crate) natural_user_curve_num: i32,
    pub(crate) face_area: f64,
    pub(crate) face_perimeter: f64,
    pub(crate) face_height: f64,
}
impl Default for ExteriorConvection {
    fn default() -> Self {
        Self {
            class: -1,
            class_report: -1,
            model: 2,
            user_model_num: 0,
            forced_model_equation: -1,
            forced_model_report: -1,
            forced_user_curve_num: 0,
            natural_model_equation: -1,
            natural_model_report: -1,
            natural_user_curve_num: 0,
            face_area: 0.0,
            face_perimeter: 0.0,
            face_height: 0.0,
        }
    }
}
#[allow(dead_code)] // Genuine allocated owners, later consumers remain held.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SurfaceParallelArrays {
    pub(crate) shade_vertices: Vec<ShadingVertex>,
    pub(crate) x0: Vec<f64>,
    pub(crate) y0: Vec<f64>,
    pub(crate) z0: Vec<f64>,
    /// Before physical assembly these are the sole four Source outdoor arrays.
    pub(crate) outdoor: Option<Vec<SurfaceOutdoorEnvironmentState>>,
    /// Rust move-custody permutation only; not another Source scientific owner.
    pub(crate) outdoor_to_model_swaps: Vec<(usize, usize)>,
    pub(crate) surf_ems_construction_override_on: Vec<bool>,
    pub(crate) surf_ems_construction_override_value: Vec<i32>,
    pub(crate) surf_ems_override_int_conv_coef: Vec<bool>,
    pub(crate) surf_ems_value_for_int_conv_coef: Vec<f64>,
    pub(crate) surf_ems_override_ext_conv_coef: Vec<bool>,
    pub(crate) surf_ems_value_for_ext_conv_coef: Vec<f64>,
    pub(crate) surf_out_dry_bulb_temp_ems_override_on: Vec<bool>,
    pub(crate) surf_out_dry_bulb_temp_ems_override_value: Vec<f64>,
    pub(crate) surf_out_wet_bulb_temp_ems_override_on: Vec<bool>,
    pub(crate) surf_out_wet_bulb_temp_ems_override_value: Vec<f64>,
    pub(crate) surf_wind_speed_ems_override_on: Vec<bool>,
    pub(crate) surf_wind_speed_ems_override_value: Vec<f64>,
    pub(crate) surf_view_factor_ground_ems_override_on: Vec<bool>,
    pub(crate) surf_view_factor_ground_ems_override_value: Vec<f64>,
    pub(crate) surf_wind_dir_ems_override_on: Vec<bool>,
    pub(crate) surf_wind_dir_ems_override_value: Vec<f64>,
    pub(crate) surf_generic_contam: Vec<f64>,
    pub(crate) surf_penumbra_id: Vec<i32>,
    pub(crate) surf_air_sky_rad_split: Vec<f64>,
    pub(crate) surf_shadow_rec_surf_num: Vec<i32>,
    pub(crate) surf_shadow_disabled_zone_list: Vec<Vec<i32>>,
    pub(crate) surf_shadow_diffuse_sol_refl: Vec<f64>,
    pub(crate) surf_shadow_diffuse_vis_refl: Vec<f64>,
    pub(crate) surf_shadow_glazing_frac: Vec<f64>,
    pub(crate) surf_shadow_glazing_construct: Vec<i32>,
    pub(crate) surf_ext_eco_roof: Vec<bool>,
    pub(crate) surf_ext_cavity_present: Vec<bool>,
    pub(crate) surf_ext_cav_num: Vec<i32>,
    pub(crate) surf_is_pv: Vec<bool>,
    pub(crate) surf_is_ics: Vec<bool>,
    pub(crate) surf_is_pool: Vec<bool>,
    pub(crate) surf_ics_ptr: Vec<i32>,
    pub(crate) surf_is_rad_surf_or_vent_slab_or_pool: Vec<bool>,
    pub(crate) surf_daylighting_shelf_ind: Vec<i32>,
    pub(crate) surf_low_temp_err_count: Vec<i32>,
    pub(crate) surf_high_temp_err_count: Vec<i32>,
    pub(crate) surf_t_air_ref: Vec<i32>,
    pub(crate) surf_t_air_ref_rpt: Vec<i32>,
    pub(crate) interior_convection: Vec<InteriorConvection>,
    pub(crate) exterior_convection: Vec<ExteriorConvection>,
    pub(crate) interior_movable_insulation: Vec<MovableInsulation>,
    pub(crate) exterior_movable_insulation: Vec<MovableInsulation>,
}
impl SurfaceParallelArrays {
    pub(crate) fn allocate(total: usize) -> Self {
        Self {
            shade_vertices: vec![ShadingVertex::default(); total],
            x0: vec![0.0; total],
            y0: vec![0.0; total],
            z0: vec![0.0; total],
            // Geometry/exposure metadata is attached only after actual final binding.
            outdoor: Some(
                (0..total)
                    .map(|_| SurfaceOutdoorEnvironmentState::allocate(0.0, false))
                    .collect(),
            ),
            outdoor_to_model_swaps: Vec::new(),
            surf_ems_construction_override_on: vec![false; total],
            surf_ems_construction_override_value: vec![0; total],
            surf_ems_override_int_conv_coef: vec![false; total],
            surf_ems_value_for_int_conv_coef: vec![0.0; total],
            surf_ems_override_ext_conv_coef: vec![false; total],
            surf_ems_value_for_ext_conv_coef: vec![0.0; total],
            surf_out_dry_bulb_temp_ems_override_on: vec![false; total],
            surf_out_dry_bulb_temp_ems_override_value: vec![0.0; total],
            surf_out_wet_bulb_temp_ems_override_on: vec![false; total],
            surf_out_wet_bulb_temp_ems_override_value: vec![0.0; total],
            surf_wind_speed_ems_override_on: vec![false; total],
            surf_wind_speed_ems_override_value: vec![0.0; total],
            surf_view_factor_ground_ems_override_on: vec![false; total],
            surf_view_factor_ground_ems_override_value: vec![0.0; total],
            surf_wind_dir_ems_override_on: vec![false; total],
            surf_wind_dir_ems_override_value: vec![0.0; total],
            surf_generic_contam: vec![0.0; total],
            surf_penumbra_id: vec![-1; total],
            surf_air_sky_rad_split: vec![0.0; total],
            surf_shadow_rec_surf_num: vec![0; total],
            surf_shadow_disabled_zone_list: vec![Vec::new(); total],
            surf_shadow_diffuse_sol_refl: vec![0.0; total],
            surf_shadow_diffuse_vis_refl: vec![0.0; total],
            surf_shadow_glazing_frac: vec![0.0; total],
            surf_shadow_glazing_construct: vec![0; total],
            surf_ext_eco_roof: vec![false; total],
            surf_ext_cavity_present: vec![false; total],
            surf_ext_cav_num: vec![0; total],
            surf_is_pv: vec![false; total],
            surf_is_ics: vec![false; total],
            surf_is_pool: vec![false; total],
            surf_ics_ptr: vec![0; total],
            surf_is_rad_surf_or_vent_slab_or_pool: vec![false; total],
            surf_daylighting_shelf_ind: vec![0; total],
            surf_low_temp_err_count: vec![0; total],
            surf_high_temp_err_count: vec![0; total],
            surf_t_air_ref: vec![-1; total],
            surf_t_air_ref_rpt: vec![-1; total],
            interior_convection: vec![InteriorConvection::default(); total],
            exterior_convection: vec![ExteriorConvection::default(); total],
            interior_movable_insulation: vec![MovableInsulation::default(); total],
            exterior_movable_insulation: vec![MovableInsulation::default(); total],
        }
    }
}
impl SurfaceGeometryInputState {
    /// Attach later final geometry metadata without replacing any allocated value.
    pub(crate) fn bind_outdoor_geometry(&mut self) -> Result<(), Error> {
        let final_rows = self
            .final_surfaces
            .as_ref()
            .ok_or(Error::Unavailable("final Surface outdoor binding"))?;
        let rows = self
            .parallel_arrays
            .as_mut()
            .and_then(|a| a.outdoor.as_mut())
            .ok_or(Error::Unavailable(
                "Source outdoor arrays before physical move",
            ))?;
        if final_rows.len() != rows.len() {
            return Err(Error::Unavailable("Source outdoor array shape"));
        }
        for (final_row, row) in final_rows.iter().zip(rows) {
            row.row.centroid_height_m = final_row.centroid[2];
            row.row.wind_exposed = final_row.row.exterior_wind;
        }
        Ok(())
    }

    /// Validate every association before taking any actual Source array value.
    pub(crate) fn take_outdoor_in_model_order(
        &mut self,
        model: &TypedModel,
    ) -> Result<Vec<SurfaceOutdoorEnvironmentState>, Error> {
        let final_rows = self
            .final_surfaces
            .as_ref()
            .ok_or(Error::Unavailable("final Surface outdoor identity"))?;
        let arrays = self
            .parallel_arrays
            .as_mut()
            .ok_or(Error::Unavailable("Source surface arrays"))?;
        let source = arrays
            .outdoor
            .as_ref()
            .ok_or(Error::Unavailable("Source outdoor owners already moved"))?;
        if source.len() != final_rows.len() || source.len() != model.surfaces.len() {
            return Err(Error::Unavailable("Source outdoor total before sole move"));
        }
        let order = model
            .surfaces
            .iter()
            .map(|surface| {
                final_rows
                    .iter()
                    .position(|row| row.typed_id == Some(surface.id))
                    .ok_or(Error::Unavailable(
                        "Source outdoor SurfaceId before sole move",
                    ))
            })
            .collect::<Result<Vec<_>, _>>()?;
        if order
            .iter()
            .copied()
            .collect::<std::collections::BTreeSet<_>>()
            .len()
            != source.len()
        {
            return Err(Error::Unavailable(
                "Source outdoor move is a complete bijection",
            ));
        }
        let mut positions = (0..source.len()).collect::<Vec<_>>();
        let mut swaps = Vec::with_capacity(order.len());
        for (destination, original) in order.into_iter().enumerate() {
            let from = positions
                .iter()
                .position(|index| *index == original)
                .ok_or(Error::Unavailable("complete Source outdoor permutation"))?;
            positions.swap(destination, from);
            swaps.push((destination, from));
        }
        // All fallible guards finished. Swap moves the complete vector without
        // cloning, dropping rows, or a partial lookup after the sole take.
        let Some(mut source) = arrays.outdoor.take() else {
            return Err(Error::Unavailable("Source outdoor custody"));
        };
        for &(destination, from) in &swaps {
            source.swap(destination, from);
        }
        arrays.outdoor_to_model_swaps = swaps;
        Ok(source)
    }

    /// Restore actual moved values before a later initialization error escapes.
    /// The caller consumes its whole staged vector, so no mutable duplicate lives.
    pub(crate) fn restore_outdoor_from_surfaces(&mut self, surfaces: Vec<SurfaceHeatBalanceState>) {
        // The staged constructor preserves model order/cardinality; neither
        // Solar constructor nor its input method can edit this Vec's shape.
        let mut source = surfaces
            .into_iter()
            .map(|surface| surface.outdoor_environment)
            .collect::<Vec<_>>();
        if let Some(arrays) = &mut self.parallel_arrays {
            for &(destination, from) in arrays.outdoor_to_model_swaps.iter().rev() {
                source.swap(destination, from);
            }
            arrays.outdoor = Some(source);
            arrays.outdoor_to_model_swaps.clear();
        }
    }
}

impl SurfaceGeometryInputState {
    /// Move the sole actual AllocateSurfaceArrays outdoor allocation without
    /// constructing the old physical CTF/temperature/cache state. Numeric rows
    /// stay in actual final Surface order and are never cloned or initialized here.
    pub(crate) fn take_source_outdoor_in_surface_order(
        &mut self,
    ) -> Result<Vec<crate::heat_balance::convection::surface_environment::source_rows::SourceSurfaceOutdoorState>, Error>{
        use crate::heat_balance::convection::surface_environment::source_rows::SourceSurfaceOutdoorState;
        let final_rows = self.final_surfaces.as_ref().ok_or(Error::Unavailable(
            "actual final Surface before Source outdoor move",
        ))?;
        let total = usize::try_from(self.total_surfaces)
            .map_err(|_| Error::Unavailable("actual Source outdoor total"))?;
        let arrays = self.parallel_arrays.as_mut().ok_or(Error::Unavailable(
            "actual AllocateSurfaceArrays before Source outdoor move",
        ))?;
        let source = arrays.outdoor.as_ref().ok_or(Error::Unavailable(
            "sole Source outdoor owners already moved",
        ))?;
        if final_rows.len() != total
            || source.len() != total
            || !arrays.outdoor_to_model_swaps.is_empty()
        {
            return Err(Error::Unavailable(
                "actual Source outdoor order/shape before sole move",
            ));
        }
        let ids = final_rows
            .iter()
            .map(|row| {
                row.typed_id.ok_or(Error::Unavailable(
                    "actual final Surface identity before Source outdoor move",
                ))
            })
            .collect::<Result<Vec<_>, _>>()?;
        if ids
            .iter()
            .copied()
            .collect::<std::collections::BTreeSet<_>>()
            .len()
            != total
        {
            return Err(Error::Unavailable(
                "actual unique full Source Surface identity",
            ));
        }
        // All fallible association checks precede the sole ownership transfer.
        let source = arrays
            .outdoor
            .take()
            .ok_or(Error::Unavailable("sole Source outdoor allocation custody"))?;
        Ok(ids
            .into_iter()
            .zip(source)
            .map(
                |(surface_id, outdoor_environment)| SourceSurfaceOutdoorState {
                    surface_id,
                    outdoor_environment,
                },
            )
            .collect())
    }
}
