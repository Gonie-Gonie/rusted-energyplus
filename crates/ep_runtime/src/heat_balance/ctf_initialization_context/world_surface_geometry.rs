//! Actual raw row producers, canonical World output and retained numerical properties.
//!
//! The boolean selects this bounded prefix only. It is not a completed
//! SetupZoneGeometry/HeatBalanceInput return or readiness flag. The admitted
//! GetSurfaceData body return is retained as its own bounded diagnostic only.
use super::{CtfInitializationContext, hb_input_error};
use crate::error::RuntimeError;
use crate::geometry::SurfaceGeometryProperties;
use crate::heat_balance::selected_heat_balance_input::surface::{
    SurfaceGetterOwners, SurfaceInputPrefix, SurfaceRowPrefix,
};
use crate::heat_balance::selected_heat_balance_input::{
    HeatBalanceInputError as E, SelectedHeatBalanceInput,
};
use crate::heat_balance::selected_project_data::ProjectInputValue;
use ep_model::{SurfaceId, TypedModel};

fn selects_source_geometry(raw: &SelectedHeatBalanceInput) -> bool {
    let [object] = raw.surface.geometry_rules.as_slice() else {
        return false;
    };
    let field = |key| match object.fields.get(key) {
        Some(ProjectInputValue::String(value)) => Some(value.as_str()),
        _ => None,
    };
    field("starting_vertex_position").is_some_and(|v| {
        [
            "UpperLeftCorner",
            "LowerLeftCorner",
            "LowerRightCorner",
            "UpperRightCorner",
        ]
        .iter()
        .any(|corner| v.eq_ignore_ascii_case(corner))
    }) && field("vertex_entry_direction").is_some_and(|v| {
        ["Counterclockwise", "CCW", "Clockwise", "CW"]
            .iter()
            .any(|direction| v.eq_ignore_ascii_case(direction))
    }) && field("coordinate_system").is_some_and(|v| {
        ["World", "Absolute", "Relative"]
            .iter()
            .any(|system| v.eq_ignore_ascii_case(system))
    })
}

impl CtfInitializationContext<'_> {
    /// Consume the admitted raw GGR through actual GetVertices to canonical World rows.
    /// Missing or different raw GGR preserves the established legacy route and
    /// leaves the actual GetSurfaceData entered guard untouched.
    pub(crate) fn prepare_selected_world_surface_geometry(
        &mut self,
        model: &TypedModel,
        environment: &crate::weather::day::WeatherEnvironmentState,
        radiation_input: Option<crate::heat_balance::radiation::ordinary::OrdinaryRadiationInput>,
        minutes_in_time_step: i32,
        host: &mut dyn crate::heat_balance::source_output_registration::SourceOutputFileHost,
    ) -> Result<bool, RuntimeError> {
        let reports = &mut *self.reports;
        let Some(raw) = reports.heat_balance_input.as_ref() else {
            return Ok(false);
        };
        if !selects_source_geometry(raw) {
            return Ok(false);
        }
        if reports.surface_geometry_input.entered_once {
            // Original's entered-once state is never reset/replayed on error.
            // Only an already bound complete numerical prefix can be reused.
            for surface in &model.surfaces {
                reports
                    .surface_geometry_input
                    .required_properties(surface.id)
                    .map_err(hb_input_error)?;
            }
            return Ok(true);
        }
        // Actual preconstruction owner supplies the unconditional window bound.
        let max_solid_window_layers = reports
            .selected_simulation_startup
            .as_ref()
            .ok_or_else(|| hb_input_error(E::Unavailable("actual startup construction bounds")))?
            .construction_bounds
            .max_solid_window_layers;
        let mut owners = SurfaceGetterOwners {
            model,
            environment,
            input: reports
                .selected_input_processor
                .as_mut()
                .ok_or_else(|| hb_input_error(E::Unavailable("actual surface IP owner")))?,
            shortcut: &mut reports.input_processor_shortcut,
            zones: reports
                .selected_zone_input
                .as_mut()
                .ok_or_else(|| hb_input_error(E::Unavailable("actual surface Zone/Space owner")))?,
            control: &mut reports.heat_balance_input_state,
            globals: reports
                .selected_heat_balance_globals
                .as_ref()
                .ok_or_else(|| hb_input_error(E::Unavailable("actual surface global owner")))?,
        };
        let state = &mut reports.surface_geometry_input;
        state
            .initialize_coordinate_owners(&owners)
            .map_err(hb_input_error)?;
        let count = match state
            .begin_input(
                raw,
                &mut owners,
                &mut self.backend,
                self.eio,
                &mut reports.heat_balance_input_errors_found,
            )
            .map_err(hb_input_error)?
        {
            SurfaceInputPrefix::OpaqueRows { count } => count,
            SurfaceInputPrefix::AlreadyEntered => {
                return Err(hb_input_error(E::Unavailable(
                    "unexpected repeated raw geometry entry",
                )));
            }
        };
        for index in 0..count {
            match state
                .read_opaque_row(
                    raw,
                    index,
                    &mut owners,
                    &mut self.backend,
                    &mut reports.heat_balance_input_errors_found,
                )
                .map_err(hb_input_error)?
            {
                SurfaceRowPrefix::Continue => {}
                SurfaceRowPrefix::BeforeVertices { slot, sides } => {
                    state
                        .get_world_vertices(slot, sides, &mut owners, &mut self.backend)
                        .map_err(hb_input_error)?;
                    // Actual GetHTSurfaceData4339-4359, before the next raw row.
                    state
                        .finish_opaque_world_row_after_vertices(
                            slot,
                            &owners,
                            reports
                                .selected_project_data
                                .as_ref()
                                .map(|p| &p.system.shadowing_clipping),
                            reports.shadowing_input.as_ref(),
                            &mut self.backend,
                            &mut reports.heat_balance_input_errors_found,
                        )
                        .map_err(hb_input_error)?;
                }
            }
        }
        state
            .finish_empty_detailed_types(
                owners.input,
                owners.shortcut,
                &mut self.backend,
                &mut reports.heat_balance_input_errors_found,
            )
            .map_err(hb_input_error)?;
        state
            .finish_opaque_input_getters(
                owners.input,
                owners.shortcut,
                &mut self.backend,
                &mut reports.heat_balance_input_errors_found,
            )
            .map_err(hb_input_error)?;
        state
            .copy_final_opaque_surfaces(
                &mut owners,
                &mut self.backend,
                &mut reports.heat_balance_input_errors_found,
                max_solid_window_layers,
            )
            .map_err(hb_input_error)?;
        state
            .set_floor_and_roof_areas(&mut owners, &mut self.backend)
            .map_err(hb_input_error)?;
        state
            .check_final_surface_areas(&mut self.backend)
            .map_err(hb_input_error)?;
        state
            .check_opaque_window_shading_and_zones(&owners, &mut self.backend)
            .map_err(hb_input_error)?;
        state
            .calculate_final_centroids(&mut self.backend, environment)
            .map_err(hb_input_error)?;
        state
            .read_empty_surface_solar_inputs(&mut owners)
            .map_err(hb_input_error)?;
        state
            .build_opaque_surface_lists(
                &mut owners,
                &mut self.backend,
                reports.heat_balance_input_errors_found,
            )
            .map_err(hb_input_error)?;
        state
            .report_surface_input_diagnostics(&mut self.backend)
            .map_err(hb_input_error)?;
        state
            .read_empty_surface_boundary_inputs(
                &mut owners,
                &mut self.backend,
                self.eio,
                &mut reports.heat_balance_input_errors_found,
            )
            .map_err(hb_input_error)?;
        // Actual2857: create and retain the SAME container used by later Solar.
        // No new input count query or typed enclosure surrogate is introduced.
        let radiation_input = radiation_input.ok_or_else(|| {
            hb_input_error(E::Unavailable(
                "actual radiation input for sole early container",
            ))
        })?;
        let radiation = reports.ordinary_solar_radiation.get_or_insert_with(|| {
            crate::heat_balance::radiation::ordinary::OrdinarySolarRadiation::for_surface_input(
                radiation_input,
            )
        });
        let mut output = crate::heat_balance::radiation::ordinary::EnclosureOutputOwners {
            output: &mut reports.output_registration,
            prescan: &reports.output_prescan,
            minimum_report_frequency: &reports
                .selected_project_data
                .as_ref()
                .ok_or_else(|| hb_input_error(E::Unavailable("same enclosure process settings")))?
                .system
                .minimum_report_frequency,
            minutes_in_time_step,
            host,
            eio: self.eio,
        };
        radiation
            .setup_radiant_enclosures(state, &mut owners, &mut output)
            .map_err(hb_input_error)?;
        // Actual2859-2915, after the same radiant producer and before caller binding.
        state
            .finish_selected_surface_input(
                &mut owners,
                &mut self.backend,
                &mut reports.heat_balance_input_errors_found,
            )
            .map_err(hb_input_error)?;
        state.bind_typed_surfaces(&owners).map_err(hb_input_error)?;
        state.bind_outdoor_geometry().map_err(hb_input_error)?;
        // Retain Zone trig arrays through this GetSurfaceData return. The
        // separate Source Setup continuation owns actual310/311 deallocation.
        Ok(true)
    }

    /// Require the same retained raw numerical return by physical SurfaceId.
    /// No missing-value fallback repeats a property kernel on typed vertices.
    pub(crate) fn selected_world_surface_properties(
        &self,
        id: SurfaceId,
    ) -> Result<SurfaceGeometryProperties, RuntimeError> {
        self.reports
            .surface_geometry_input
            .required_properties(id)
            .map_err(hb_input_error)
    }

    pub(crate) fn take_selected_world_outdoor_rows(
        &mut self,
        model: &TypedModel,
    ) -> Result<
        Vec<crate::heat_balance::convection::surface_environment::SurfaceOutdoorEnvironmentState>,
        RuntimeError,
    > {
        self.reports
            .surface_geometry_input
            .take_outdoor_in_model_order(model)
            .map_err(hb_input_error)
    }

    pub(crate) fn restore_selected_world_outdoor_rows(
        &mut self,
        surfaces: Vec<crate::heat_balance::state::SurfaceHeatBalanceState>,
    ) {
        self.reports
            .surface_geometry_input
            .restore_outdoor_from_surfaces(surfaces);
    }

    /// Move the actual early548 return into the existing initializer consumer.
    /// This invokes no geometry, getter or second numerical volume calculation.
    pub(crate) fn selected_world_zone_volume(
        &self,
        id: ep_model::ZoneId,
        returned: &mut Vec<crate::geometry::InitializedZoneGeometry>,
    ) -> Result<crate::geometry::ZoneGeometryProperties, RuntimeError> {
        let zone = self
            .reports
            .selected_zone_input
            .as_ref()
            .and_then(|owner| owner.zones.as_ref())
            .and_then(|zones| zones.iter().find(|zone| zone.typed_zone_id == Some(id)))
            .ok_or_else(|| hb_input_error(E::Unavailable("same actual Source548 Zone owner")))?;
        let index = returned
            .iter()
            .position(|row| row.zone_id == id)
            .ok_or_else(|| hb_input_error(E::Unavailable("actual carried Source548 return")))?;
        let result = Ok(returned.remove(index).returned);
        crate::geometry::zone_volume_trace::record(id, &zone.name, &result);
        result.map_err(|error: crate::geometry::ZoneVolumeError| {
            RuntimeError::UnsupportedZoneVolumeGeometry {
                zone_name: zone.name.clone(),
                reason: error.to_string(),
            }
        })
    }
}
