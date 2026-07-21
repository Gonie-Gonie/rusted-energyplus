use ep_model::{ConstructionId, ConstructionKind, TypedModel};
use ep_raw_model::{FieldName, RawValue};

use super::{Compiler, DiagnosticSeverity};

const RAW_CONSTRUCTION_REFERENCES: [(&str, &str, bool); 6] = [
    ("Pipe:Indoor", "construction_name", false),
    ("Pipe:Outdoor", "construction_name", false),
    ("Pipe:Underground", "construction_name", false),
    ("GroundHeatExchanger:Surface", "construction_name", true),
    ("DaylightingDevice:Tubular", "construction_name", false),
    (
        "EnergyManagementSystem:ConstructionIndexVariable",
        "construction_object_name",
        true,
    ),
];

impl Compiler<'_> {
    /// Collects positive-only construction-use evidence from the currently retained subset.
    pub(super) fn collect_known_construction_use_evidence(&self, model: &mut TypedModel) {
        if self
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity == DiagnosticSeverity::Error)
        {
            return;
        }

        let mut known_used = model
            .surfaces
            .iter()
            .map(|surface| surface.construction)
            .collect::<Vec<_>>();
        let mut known_ctf_used = Vec::new();

        for (object_type, field, requires_ctf) in RAW_CONSTRUCTION_REFERENCES {
            let Ok(instances) = self.raw_model.ordered_instances(object_type) else {
                continue;
            };
            let field = FieldName(field.to_string());
            for (_, object) in instances {
                let Some(RawValue::String(construction_name)) = object.fields.get(&field) else {
                    continue;
                };
                let Some(construction) = model.construction_names.resolve(construction_name) else {
                    continue;
                };

                known_used.push(construction);
                if requires_ctf && construction_is_non_window(model, construction) {
                    known_ctf_used.push(construction);
                }
            }
        }

        known_used.sort_unstable();
        known_used.dedup();
        known_ctf_used.sort_unstable();
        known_ctf_used.dedup();
        model.known_used_constructions = known_used;
        model.known_ctf_used_constructions = known_ctf_used;
    }
}

fn construction_is_non_window(model: &TypedModel, id: ConstructionId) -> bool {
    model
        .constructions
        .get(id.0 as usize)
        .filter(|construction| construction.id == id)
        .is_some_and(|construction| {
            matches!(
                construction.kind,
                ConstructionKind::Opaque | ConstructionKind::AirBoundary
            )
        })
}
