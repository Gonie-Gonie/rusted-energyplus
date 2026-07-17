use super::{Compiler, INTERNAL_HEAT_SOURCE_OBJECT_TYPE};
use ep_model::{
    ConstructionInternalHeatSource, ConstructionInternalHeatSourceDimensions, NormalizedName,
    TypedModel,
};

impl Compiler<'_> {
    pub(super) fn parse_construction_internal_heat_sources(&mut self, model: &mut TypedModel) {
        const CONSTRUCTION_FIELD: &str = "construction_name";
        const SOURCE_LAYER_FIELD: &str = "thermal_source_present_after_layer_number";
        const TEMPERATURE_LAYER_FIELD: &str =
            "temperature_calculation_requested_after_layer_number";
        const DIMENSIONS_FIELD: &str = "dimensions_for_the_ctf_calculation";
        const TUBE_SPACING_FIELD: &str = "tube_spacing";
        const TEMPERATURE_POSITION_FIELD: &str = "two_dimensional_temperature_calculation_position";

        for (name, object) in self.objects(INTERNAL_HEAT_SOURCE_OBJECT_TYPE) {
            let diagnostics_before_fields = self.diagnostics.len();
            if name.trim().is_empty() {
                self.error(
                    "MissingRequiredField",
                    INTERNAL_HEAT_SOURCE_OBJECT_TYPE,
                    Some(&name),
                    Some("name"),
                    format!("{INTERNAL_HEAT_SOURCE_OBJECT_TYPE} requires a nonblank object name"),
                );
            }

            let construction_name = self.required_string(
                INTERNAL_HEAT_SOURCE_OBJECT_TYPE,
                &name,
                &object,
                CONSTRUCTION_FIELD,
            );
            let source_after_layer = self.required_positive_u32(
                INTERNAL_HEAT_SOURCE_OBJECT_TYPE,
                &name,
                &object,
                SOURCE_LAYER_FIELD,
            );
            let temperature_after_layer = self.required_positive_u32(
                INTERNAL_HEAT_SOURCE_OBJECT_TYPE,
                &name,
                &object,
                TEMPERATURE_LAYER_FIELD,
            );
            let ctf_dimensions = self
                .required_positive_u32(
                    INTERNAL_HEAT_SOURCE_OBJECT_TYPE,
                    &name,
                    &object,
                    DIMENSIONS_FIELD,
                )
                .and_then(|dimensions| match dimensions {
                    1 => Some(ConstructionInternalHeatSourceDimensions::OneDimensional),
                    2 => Some(ConstructionInternalHeatSourceDimensions::TwoDimensional),
                    _ => {
                        self.error(
                            "InvalidInternalHeatSourceDimensions",
                            INTERNAL_HEAT_SOURCE_OBJECT_TYPE,
                            Some(&name),
                            Some(DIMENSIONS_FIELD),
                            format!(
                                "{INTERNAL_HEAT_SOURCE_OBJECT_TYPE}/{name} field {DIMENSIONS_FIELD} must be 1 or 2, got {dimensions}"
                            ),
                        );
                        None
                    }
                });
            let tube_spacing_m = self.required_number_range(
                INTERNAL_HEAT_SOURCE_OBJECT_TYPE,
                &name,
                &object,
                TUBE_SPACING_FIELD,
                0.01..=1.0,
            );
            let temperature_location_perpendicular = self.number_bounded_blank_default(
                INTERNAL_HEAT_SOURCE_OBJECT_TYPE,
                &name,
                &object,
                TEMPERATURE_POSITION_FIELD,
                0.0,
                (0.0, true),
                (1.0, true),
            );

            if self.diagnostics.len() != diagnostics_before_fields {
                continue;
            }
            let (
                Some(construction_name),
                Some(source_after_layer),
                Some(temperature_after_layer),
                Some(ctf_dimensions),
                Some(tube_spacing_m),
            ) = (
                construction_name,
                source_after_layer,
                temperature_after_layer,
                ctf_dimensions,
                tube_spacing_m,
            )
            else {
                continue;
            };

            let Some(construction_id) = self.resolve_name(
                &model.construction_names,
                INTERNAL_HEAT_SOURCE_OBJECT_TYPE,
                &name,
                CONSTRUCTION_FIELD,
                &construction_name,
                "Construction",
            ) else {
                continue;
            };
            let Ok(construction_index) = usize::try_from(construction_id.0) else {
                self.error(
                    "InvalidReference",
                    INTERNAL_HEAT_SOURCE_OBJECT_TYPE,
                    Some(&name),
                    Some(CONSTRUCTION_FIELD),
                    format!(
                        "{INTERNAL_HEAT_SOURCE_OBJECT_TYPE}/{name} resolved construction '{construction_name}' outside the platform index range"
                    ),
                );
                continue;
            };
            let Some(construction) = model.constructions.get(construction_index) else {
                self.error(
                    "InvalidReference",
                    INTERNAL_HEAT_SOURCE_OBJECT_TYPE,
                    Some(&name),
                    Some(CONSTRUCTION_FIELD),
                    format!(
                        "{INTERNAL_HEAT_SOURCE_OBJECT_TYPE}/{name} resolved construction '{construction_name}' outside the construction arena"
                    ),
                );
                continue;
            };
            if !construction.is_ordinary_opaque() {
                self.error(
                    "InvalidInternalHeatSourceConstruction",
                    INTERNAL_HEAT_SOURCE_OBJECT_TYPE,
                    Some(&name),
                    Some(CONSTRUCTION_FIELD),
                    format!(
                        "{INTERNAL_HEAT_SOURCE_OBJECT_TYPE}/{name} requires an ordinary opaque Construction target; '{construction_name}' has kind {}",
                        construction.kind.id()
                    ),
                );
                continue;
            }

            let layer_count = construction.effective_layers().len();
            if layer_count < 2 {
                self.error(
                    "InvalidInternalHeatSourceConstruction",
                    INTERNAL_HEAT_SOURCE_OBJECT_TYPE,
                    Some(&name),
                    Some(CONSTRUCTION_FIELD),
                    format!(
                        "{INTERNAL_HEAT_SOURCE_OBJECT_TYPE}/{name} target '{construction_name}' requires at least two material layers, found {layer_count}"
                    ),
                );
                continue;
            }

            let mut layer_positions_valid = true;
            for (field, position) in [
                (SOURCE_LAYER_FIELD, source_after_layer),
                (TEMPERATURE_LAYER_FIELD, temperature_after_layer),
            ] {
                let position_is_invalid = match usize::try_from(position) {
                    Ok(position) => position >= layer_count,
                    Err(_) => true,
                };
                if position_is_invalid {
                    self.error(
                        "InvalidInternalHeatSourceLayerPosition",
                        INTERNAL_HEAT_SOURCE_OBJECT_TYPE,
                        Some(&name),
                        Some(field),
                        format!(
                            "{INTERNAL_HEAT_SOURCE_OBJECT_TYPE}/{name} field {field} must be between 1 and {}, got {position}",
                            layer_count - 1
                        ),
                    );
                    layer_positions_valid = false;
                }
            }
            if !layer_positions_valid {
                continue;
            }
            if construction.has_internal_heat_source() {
                self.error(
                    "DuplicateInternalHeatSourceConstruction",
                    INTERNAL_HEAT_SOURCE_OBJECT_TYPE,
                    Some(&name),
                    Some(CONSTRUCTION_FIELD),
                    format!(
                        "{INTERNAL_HEAT_SOURCE_OBJECT_TYPE}/{name} repeats construction '{construction_name}', which already has an internal heat source"
                    ),
                );
                continue;
            }

            let internal_heat_source = ConstructionInternalHeatSource {
                name: NormalizedName::new(&name),
                source_after_layer,
                temperature_after_layer,
                ctf_dimensions,
                tube_spacing_m,
                half_tube_spacing_m: tube_spacing_m / 2.0,
                temperature_location_perpendicular,
            };
            let Some(construction) = model.constructions.get_mut(construction_index) else {
                continue;
            };
            construction.internal_heat_source = Some(internal_heat_source);
        }
    }
}
