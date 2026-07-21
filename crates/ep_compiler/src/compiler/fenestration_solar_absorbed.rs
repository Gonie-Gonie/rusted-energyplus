use std::collections::BTreeSet;

use ep_model::{
    ConstructionKind, FenestrationSolarAbsorbedRequest, FenestrationSolarAbsorbedRequestId,
    NormalizedName, TypedModel,
};

use super::{Compiler, DiagnosticSeverity, field_value};

const OBJECT_TYPE: &str = "ComplexFenestrationProperty:SolarAbsorbedLayers";
const SURFACE_FIELD: &str = "fenestration_surface";
const CONSTRUCTION_FIELD: &str = "construction_name";
const MAX_SCHEDULED_LAYERS: usize = 5;

fn schedule_field(layer: usize) -> String {
    format!("layer_{layer}_solar_radiation_absorbed_schedule_name")
}

impl Compiler<'_> {
    pub(super) fn parse_fenestration_solar_absorbed_requests(&mut self, model: &mut TypedModel) {
        let diagnostics_before_pass = self.diagnostics.len();
        let mut pending = Vec::new();
        let mut surface_construction_pairs = BTreeSet::new();

        for (name, object) in self.objects(OBJECT_TYPE) {
            let diagnostics_before_object = self.diagnostics.len();
            if name.trim().is_empty() {
                self.error(
                    "MissingRequiredField",
                    OBJECT_TYPE,
                    Some(&name),
                    Some("name"),
                    format!("{OBJECT_TYPE} requires a nonblank object name"),
                );
            }

            let fenestration_surface_name =
                self.required_string(OBJECT_TYPE, &name, &object, SURFACE_FIELD);
            let construction_name =
                self.required_string(OBJECT_TYPE, &name, &object, CONSTRUCTION_FIELD);
            let construction = construction_name.as_deref().and_then(|target_name| {
                self.resolve_name(
                    &model.construction_names,
                    OBJECT_TYPE,
                    &name,
                    CONSTRUCTION_FIELD,
                    target_name,
                    "Construction:ComplexFenestrationState",
                )
            });

            let expected_layer_count = construction.and_then(|construction_id| {
                let Some(construction) = model.constructions.get(construction_id.0 as usize) else {
                    self.error(
                        "InvalidFenestrationSolarAbsorbedConstruction",
                        OBJECT_TYPE,
                        Some(&name),
                        Some(CONSTRUCTION_FIELD),
                        format!(
                            "{OBJECT_TYPE}/{name} resolved an invalid construction identity"
                        ),
                    );
                    return None;
                };
                let Some(metadata) = construction.complex_fenestration.as_ref() else {
                    self.error(
                        "InvalidFenestrationSolarAbsorbedConstruction",
                        OBJECT_TYPE,
                        Some(&name),
                        Some(CONSTRUCTION_FIELD),
                        format!(
                            "{OBJECT_TYPE}/{name} field {CONSTRUCTION_FIELD} must reference a typed Construction:ComplexFenestrationState"
                        ),
                    );
                    return None;
                };
                if !matches!(construction.kind, ConstructionKind::ComplexFenestration) {
                    self.error(
                        "InvalidFenestrationSolarAbsorbedConstruction",
                        OBJECT_TYPE,
                        Some(&name),
                        Some(CONSTRUCTION_FIELD),
                        format!(
                            "{OBJECT_TYPE}/{name} field {CONSTRUCTION_FIELD} resolved inconsistent complex-fenestration metadata"
                        ),
                    );
                    return None;
                }
                let layer_count = metadata.optical_layers.len();
                if !(1..=MAX_SCHEDULED_LAYERS).contains(&layer_count) {
                    self.error(
                        "InvalidFenestrationSolarAbsorbedLayerCount",
                        OBJECT_TYPE,
                        Some(&name),
                        Some(CONSTRUCTION_FIELD),
                        format!(
                            "{OBJECT_TYPE}/{name} references {layer_count} solid layers, outside the public one-to-five schedule field surface"
                        ),
                    );
                    return None;
                }
                Some(layer_count)
            });

            let mut layer_schedules = Vec::new();
            if let Some(layer_count) = expected_layer_count {
                for layer in 1..=layer_count {
                    let field = schedule_field(layer);
                    let schedule_name = self.required_string(OBJECT_TYPE, &name, &object, &field);
                    let schedule = schedule_name.as_deref().and_then(|target_name| {
                        self.resolve_name(
                            &model.schedule_names,
                            OBJECT_TYPE,
                            &name,
                            &field,
                            target_name,
                            "Schedule",
                        )
                    });
                    if let Some(schedule) = schedule {
                        layer_schedules.push(schedule);
                    }
                }
                for layer in (layer_count + 1)..=MAX_SCHEDULED_LAYERS {
                    let field = schedule_field(layer);
                    if field_value(&object, &field).is_some() {
                        self.error(
                            "UnexpectedFenestrationSolarAbsorbedLayerSchedule",
                            OBJECT_TYPE,
                            Some(&name),
                            Some(&field),
                            format!(
                                "{OBJECT_TYPE}/{name} supplies {field}, but the referenced construction has only {layer_count} solid layer(s)"
                            ),
                        );
                    }
                }
            } else {
                // Keep schema diagnostics independent when construction validation
                // prevents deriving the source-effective solid-layer count.
                for layer in 1..=MAX_SCHEDULED_LAYERS {
                    let field = schedule_field(layer);
                    if layer == 1 || field_value(&object, &field).is_some() {
                        let schedule_name =
                            self.required_string(OBJECT_TYPE, &name, &object, &field);
                        if let Some(target_name) = schedule_name.as_deref() {
                            let _ = self.resolve_name(
                                &model.schedule_names,
                                OBJECT_TYPE,
                                &name,
                                &field,
                                target_name,
                                "Schedule",
                            );
                        }
                    }
                }
            }

            if self.diagnostics.len() != diagnostics_before_object {
                continue;
            }
            let (Some(fenestration_surface_name), Some(construction)) =
                (fenestration_surface_name, construction)
            else {
                continue;
            };
            let fenestration_surface_name = NormalizedName::new(&fenestration_surface_name);
            let Some(id_value) = self.checked_id(OBJECT_TYPE, &name, pending.len()) else {
                continue;
            };
            if !surface_construction_pairs.insert((fenestration_surface_name.clone(), construction))
            {
                self.error(
                    "DuplicateFenestrationSolarAbsorbedPair",
                    OBJECT_TYPE,
                    Some(&name),
                    Some(CONSTRUCTION_FIELD),
                    format!(
                        "{OBJECT_TYPE}/{name} repeats an existing normalized fenestration-surface/construction pair"
                    ),
                );
                continue;
            }
            pending.push(FenestrationSolarAbsorbedRequest {
                id: FenestrationSolarAbsorbedRequestId(id_value),
                name: NormalizedName::new(&name),
                fenestration_surface_name,
                construction,
                layer_schedules,
            });
        }

        let has_errors = self.diagnostics[diagnostics_before_pass..]
            .iter()
            .any(|diagnostic| diagnostic.severity == DiagnosticSeverity::Error);
        if !has_errors {
            model.fenestration_solar_absorbed_requests = pending;
        }
    }
}
