//! Actual raw World row producers and required retained numerical properties.
//!
//! The boolean selects this bounded prefix only. It is not a completed
//! GetSurfaceData/SetupZoneGeometry/HeatBalanceInput return or readiness flag.
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

fn selects_world(raw: &SelectedHeatBalanceInput) -> bool {
    let [object] = raw.surface.geometry_rules.as_slice() else {
        return false;
    };
    let field = |key| match object.fields.get(key) {
        Some(ProjectInputValue::String(value)) => Some(value.as_str()),
        _ => None,
    };
    field("starting_vertex_position").is_some_and(|v| v.eq_ignore_ascii_case("UpperLeftCorner"))
        && field("vertex_entry_direction").is_some_and(|v| {
            v.eq_ignore_ascii_case("Counterclockwise") || v.eq_ignore_ascii_case("CCW")
        })
        && field("coordinate_system")
            .is_some_and(|v| v.eq_ignore_ascii_case("World") || v.eq_ignore_ascii_case("Absolute"))
}

impl CtfInitializationContext<'_> {
    /// Consume actual source raw rows on the explicitly selected World route.
    /// Missing or different raw GGR preserves the established legacy route and
    /// leaves the actual GetSurfaceData entered guard untouched.
    pub(crate) fn prepare_selected_world_surface_geometry(
        &mut self,
        model: &TypedModel,
        environment: &crate::weather::day::WeatherEnvironmentState,
    ) -> Result<bool, RuntimeError> {
        let reports = &mut *self.reports;
        let Some(raw) = reports.heat_balance_input.as_ref() else {
            return Ok(false);
        };
        if !selects_world(raw) {
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
                SurfaceRowPrefix::BeforeVertices { slot, sides } => state
                    .get_world_vertices(slot, sides, &mut owners, &mut self.backend)
                    .map_err(hb_input_error)?,
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
            .copy_final_opaque_surfaces(
                &mut owners,
                &mut self.backend,
                &mut reports.heat_balance_input_errors_found,
            )
            .map_err(hb_input_error)?;
        state
            .check_final_surface_areas(&mut self.backend)
            .map_err(hb_input_error)?;
        state
            .calculate_final_centroids(&mut self.backend, environment)
            .map_err(hb_input_error)?;
        state.bind_typed_surfaces(&owners).map_err(hb_input_error)?;
        // Do not clear SetupGeometry trig arrays here: Original310's caller
        // boundary has not been reached by this bounded numerical-prefix port.
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
}
