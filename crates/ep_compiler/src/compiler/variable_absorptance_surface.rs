use ep_model::{
    MaterialId, MaterialVariableAbsorptanceId, OutsideBoundaryCondition, TypedModel,
    VariableAbsorptanceSurfaceBinding,
};

use super::{Compiler, DiagnosticSeverity};

const SURFACE_OBJECT_TYPE: &str = "BuildingSurface:Detailed";
const CONSTRUCTION_OBJECT_TYPE: &str = "Construction";
const OVERLAY_OBJECT_TYPE: &str = "MaterialProperty:VariableAbsorptance";

#[derive(Debug)]
struct PendingWarning {
    code: &'static str,
    object_type: &'static str,
    object_name: String,
    field: String,
    message: String,
}

impl Compiler<'_> {
    /// Finalizes the bounded `GetVariableAbsorptanceSurfaceList` selection after detailed surfaces
    /// have been parsed successfully.
    pub(super) fn build_variable_absorptance_surface_list(
        &mut self,
        model: &mut TypedModel,
        diagnostics_before_surfaces: usize,
    ) {
        if self.diagnostics[diagnostics_before_surfaces..]
            .iter()
            .any(|diagnostic| diagnostic.severity == DiagnosticSeverity::Error)
        {
            return;
        }

        let mut overlay_by_material = vec![None; model.materials.len()];
        for (overlay_index, overlay) in model.material_variable_absorptances.iter().enumerate() {
            if usize::try_from(overlay.id.0).ok() != Some(overlay_index) {
                self.variable_absorptance_internal_error(
                    OVERLAY_OBJECT_TYPE,
                    &overlay.name.0,
                    format!(
                        "typed overlay id {} does not match arena index {overlay_index}",
                        overlay.id.0
                    ),
                );
                return;
            }
            let Some(material_index) = self.variable_absorptance_material_index(
                model,
                overlay.reference_material,
                OVERLAY_OBJECT_TYPE,
                &overlay.name.0,
            ) else {
                return;
            };
            if overlay_by_material[material_index]
                .replace(overlay.id)
                .is_some()
            {
                self.variable_absorptance_internal_error(
                    OVERLAY_OBJECT_TYPE,
                    &overlay.name.0,
                    format!(
                        "duplicates the typed overlay target Material id {}",
                        overlay.reference_material.0
                    ),
                );
                return;
            }
        }
        if model.material_variable_absorptances.is_empty() {
            model.variable_absorptance_surface_bindings.clear();
            return;
        }

        let mut bindings = Vec::new();
        let mut warnings = Vec::new();
        for (surface_index, surface) in model.surfaces.iter().enumerate() {
            if usize::try_from(surface.id.0).ok() != Some(surface_index) {
                self.variable_absorptance_internal_error(
                    SURFACE_OBJECT_TYPE,
                    &surface.name.0,
                    format!(
                        "typed Surface id {} does not match arena index {surface_index}",
                        surface.id.0
                    ),
                );
                return;
            }
            let Ok(construction_index) = usize::try_from(surface.construction.0) else {
                self.variable_absorptance_internal_error(
                    SURFACE_OBJECT_TYPE,
                    &surface.name.0,
                    format!(
                        "references unavailable Construction id {}",
                        surface.construction.0
                    ),
                );
                return;
            };
            let Some(construction) = model.constructions.get(construction_index) else {
                self.variable_absorptance_internal_error(
                    SURFACE_OBJECT_TYPE,
                    &surface.name.0,
                    format!(
                        "references unavailable Construction id {}",
                        surface.construction.0
                    ),
                );
                return;
            };
            if construction.id != surface.construction {
                self.variable_absorptance_internal_error(
                    SURFACE_OBJECT_TYPE,
                    &surface.name.0,
                    format!(
                        "references Construction id {} stored at a mismatched arena index",
                        surface.construction.0
                    ),
                );
                return;
            }
            let Some(outside_material) = construction.effective_layers().first().copied() else {
                continue;
            };
            let Some(material_index) = self.variable_absorptance_material_index(
                model,
                outside_material,
                SURFACE_OBJECT_TYPE,
                &surface.name.0,
            ) else {
                return;
            };
            let Some(variable_absorptance) = overlay_by_material[material_index] else {
                continue;
            };
            let Some(overlay_index) = self.variable_absorptance_overlay_index(
                model,
                variable_absorptance,
                SURFACE_OBJECT_TYPE,
                &surface.name.0,
            ) else {
                return;
            };

            if surface.outside_boundary_condition == OutsideBoundaryCondition::Outdoors {
                bindings.push(VariableAbsorptanceSurfaceBinding {
                    surface: surface.id,
                    variable_absorptance,
                });
            } else {
                let overlay = &model.material_variable_absorptances[overlay_index];
                let material = &model.materials[material_index];
                warnings.push(PendingWarning {
                    code: "VariableAbsorptanceIgnoredOnNonOutdoorSurface",
                    object_type: SURFACE_OBJECT_TYPE,
                    object_name: surface.name.0.clone(),
                    field: "outside_boundary_condition".to_string(),
                    message: format!(
                        "{OVERLAY_OBJECT_TYPE}/{} on outside-layer material '{}' is ignored for non-Outdoors {SURFACE_OBJECT_TYPE}/{}",
                        overlay.name.0, material.name.0, surface.name.0
                    ),
                });
            }
        }

        for (construction_index, construction) in model.constructions.iter().enumerate() {
            if usize::try_from(construction.id.0).ok() != Some(construction_index) {
                self.variable_absorptance_internal_error(
                    CONSTRUCTION_OBJECT_TYPE,
                    &construction.name.0,
                    format!(
                        "typed Construction id {} does not match arena index {construction_index}",
                        construction.id.0
                    ),
                );
                return;
            }
            for (layer_index, material_id) in
                construction.effective_layers().iter().enumerate().skip(1)
            {
                let Some(material_index) = self.variable_absorptance_material_index(
                    model,
                    *material_id,
                    CONSTRUCTION_OBJECT_TYPE,
                    &construction.name.0,
                ) else {
                    return;
                };
                let Some(variable_absorptance) = overlay_by_material[material_index] else {
                    continue;
                };
                let Some(overlay_index) = self.variable_absorptance_overlay_index(
                    model,
                    variable_absorptance,
                    CONSTRUCTION_OBJECT_TYPE,
                    &construction.name.0,
                ) else {
                    return;
                };
                let overlay = &model.material_variable_absorptances[overlay_index];
                let material = &model.materials[material_index];
                let one_based_layer = layer_index + 1;
                warnings.push(PendingWarning {
                    code: "VariableAbsorptanceIgnoredOnInsideConstructionLayer",
                    object_type: CONSTRUCTION_OBJECT_TYPE,
                    object_name: construction.name.0.clone(),
                    field: format!("layer_{one_based_layer}"),
                    message: format!(
                        "{OVERLAY_OBJECT_TYPE}/{} on material '{}' is ignored at inside layer {one_based_layer} of {CONSTRUCTION_OBJECT_TYPE}/{}",
                        overlay.name.0, material.name.0, construction.name.0
                    ),
                });
            }
        }

        // Publish once so a partially scanned selection is never observable.
        model.variable_absorptance_surface_bindings = bindings;
        for warning in warnings {
            self.warning(
                warning.code,
                warning.object_type,
                Some(&warning.object_name),
                Some(&warning.field),
                warning.message,
            );
        }
    }

    fn variable_absorptance_material_index(
        &mut self,
        model: &TypedModel,
        material: MaterialId,
        object_type: &str,
        object_name: &str,
    ) -> Option<usize> {
        let Ok(material_index) = usize::try_from(material.0) else {
            self.variable_absorptance_internal_error(
                object_type,
                object_name,
                format!("references unavailable Material id {}", material.0),
            );
            return None;
        };
        let Some(candidate) = model.materials.get(material_index) else {
            self.variable_absorptance_internal_error(
                object_type,
                object_name,
                format!("references unavailable Material id {}", material.0),
            );
            return None;
        };
        if candidate.id != material {
            self.variable_absorptance_internal_error(
                object_type,
                object_name,
                format!(
                    "references Material id {} stored at a mismatched arena index",
                    material.0
                ),
            );
            return None;
        }
        Some(material_index)
    }

    fn variable_absorptance_overlay_index(
        &mut self,
        model: &TypedModel,
        variable_absorptance: MaterialVariableAbsorptanceId,
        object_type: &str,
        object_name: &str,
    ) -> Option<usize> {
        let Ok(overlay_index) = usize::try_from(variable_absorptance.0) else {
            self.variable_absorptance_internal_error(
                object_type,
                object_name,
                format!(
                    "resolves unavailable MaterialVariableAbsorptance id {}",
                    variable_absorptance.0
                ),
            );
            return None;
        };
        let Some(candidate) = model.material_variable_absorptances.get(overlay_index) else {
            self.variable_absorptance_internal_error(
                object_type,
                object_name,
                format!(
                    "resolves unavailable MaterialVariableAbsorptance id {}",
                    variable_absorptance.0
                ),
            );
            return None;
        };
        if candidate.id != variable_absorptance {
            self.variable_absorptance_internal_error(
                object_type,
                object_name,
                format!(
                    "resolves MaterialVariableAbsorptance id {} stored at a mismatched arena index",
                    variable_absorptance.0
                ),
            );
            return None;
        }
        Some(overlay_index)
    }

    fn variable_absorptance_internal_error(
        &mut self,
        object_type: &str,
        object_name: &str,
        detail: String,
    ) {
        self.error(
            "InternalReferenceError",
            object_type,
            Some(object_name),
            None,
            format!("{object_type}/{object_name} {detail}"),
        );
    }
}
