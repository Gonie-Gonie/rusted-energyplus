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
        state.bind_typed_surfaces(&owners).map_err(hb_input_error)?;
        state.bind_outdoor_geometry().map_err(hb_input_error)?;
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

    /// Qualified existing numeric handoff, supplied by genuine retained faces.
    /// This is not the complete Source SetupZoneGeometry height/volume suffix.
    pub(crate) fn selected_world_zone_volume(
        &mut self,
        id: ep_model::ZoneId,
    ) -> Result<crate::geometry::ZoneGeometryProperties, RuntimeError> {
        let reports = &mut *self.reports;
        let zones = reports
            .selected_zone_input
            .as_mut()
            .and_then(|owner| owner.zones.as_mut())
            .ok_or_else(|| hb_input_error(E::Unavailable("same selected Zone volume owner")))?;
        let number = zones
            .iter()
            .position(|zone| zone.typed_zone_id == Some(id))
            .ok_or_else(|| hb_input_error(E::Unavailable("actual selected volume ZoneId")))?;
        let source_number = i32::try_from(number)
            .ok()
            .and_then(|value| value.checked_add(1))
            .ok_or_else(|| hb_input_error(E::Unavailable("Source signed volume Zone index")))?;
        let faces = reports
            .surface_geometry_input
            .retained_zone_volume_faces(source_number)
            .map_err(hb_input_error)?;
        let zone = &mut zones[number];
        let mut numeric = crate::geometry::ZoneGeometryProperties {
            volume_m3: zone.volume,
            ceiling_height_m: zone.ceiling_height,
            ceiling_height_entered: zone.ceiling_height_entered,
            floor_area_m2: zone.floor_area,
            user_entered_floor_area_m2: zone.user_entered_floor_area,
            ceiling_area_m2: zone.ceiling_area,
            has_floor: zone.has_floor,
            has_roof: zone.has_roof,
            faces,
            ..Default::default()
        };
        // Temporary numeric denominators only. No second persistent Zone owner,
        // inferred multiplier or Source geometric-area producer is installed.
        for face in &numeric.faces {
            match face.surface_type {
                ep_model::SurfaceType::Floor => {
                    numeric.geometric_floor_area_m2 += face.gross_area_m2
                }
                ep_model::SurfaceType::Roof | ep_model::SurfaceType::Ceiling => {
                    numeric.geometric_ceiling_area_m2 += face.gross_area_m2
                }
                ep_model::SurfaceType::Wall => {}
            }
        }
        let result = (|| {
            let height = crate::geometry::prepare_zone_height(&mut numeric);
            // Preserve partial helper writes before either numerical error.
            zone.ceiling_height = numeric.ceiling_height_m;
            zone.ceiling_height_entered = numeric.ceiling_height_entered;
            height?;
            let volume = crate::geometry::calculate_zone_volume(&mut numeric);
            zone.volume = numeric.volume_m3;
            volume?;
            Ok(numeric)
        })();
        crate::geometry::zone_volume_trace::record(id, &zone.name, &result);
        result.map_err(|error: crate::geometry::ZoneVolumeError| {
            RuntimeError::UnsupportedZoneVolumeGeometry {
                zone_name: zone.name.clone(),
                reason: error.to_string(),
            }
        })
    }
}
