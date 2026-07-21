use std::collections::BTreeSet;

use ep_model::TypedModel;

use super::{Compiler, DiagnosticSeverity};

const ZONE_OBJECT_TYPE: &str = "Zone";
const WARNING_CODE: &str = "IncompleteScheduledSurfaceGainsTypedSubset";

impl Compiler<'_> {
    /// Warns only when the retained typed opaque-surface subset proves that a Zone mixes
    /// exactly scheduled and unscheduled current surface/construction pairs.
    pub(super) fn check_scheduled_surface_gains_typed_subset(
        &mut self,
        model: &TypedModel,
        diagnostics_before_surface_inputs: usize,
    ) {
        if self.diagnostics[diagnostics_before_surface_inputs..]
            .iter()
            .any(|diagnostic| diagnostic.severity == DiagnosticSeverity::Error)
        {
            return;
        }

        let scheduled_pairs = model
            .surface_solar_incidents
            .iter()
            .map(|incident| (incident.surface, incident.construction))
            .collect::<BTreeSet<_>>();
        if scheduled_pairs.is_empty() {
            return;
        }

        for zone in &model.zones {
            let zone_spaces = zone.spaces.iter().copied().collect::<BTreeSet<_>>();
            let mut has_exact_scheduled_pair = false;
            let mut known_unscheduled_surface_names = Vec::new();

            for surface in &model.surfaces {
                if surface.zone != zone.id || !zone_spaces.contains(&surface.space) {
                    continue;
                }
                if scheduled_pairs.contains(&(surface.id, surface.construction)) {
                    has_exact_scheduled_pair = true;
                } else {
                    known_unscheduled_surface_names.push(surface.name.0.as_str());
                }
            }

            if has_exact_scheduled_pair && !known_unscheduled_surface_names.is_empty() {
                self.warning(
                    WARNING_CODE,
                    ZONE_OBJECT_TYPE,
                    Some(&zone.name.0),
                    None,
                    format!(
                        "Zone/{} has a mixed retained typed BuildingSurface:Detailed subset: at least one surface has an exact SurfaceProperty:SolarIncidentInside surface/construction pair, while these known-unscheduled surfaces remain in typed arena order: {}",
                        zone.name.0,
                        known_unscheduled_surface_names.join(", ")
                    ),
                );
            }
        }
    }
}
