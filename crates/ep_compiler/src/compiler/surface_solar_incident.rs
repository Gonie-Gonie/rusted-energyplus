use std::collections::BTreeSet;

use ep_model::{NormalizedName, SurfaceSolarIncident, SurfaceSolarIncidentId, TypedModel};

use super::{Compiler, DiagnosticSeverity};

const OBJECT_TYPE: &str = "SurfaceProperty:SolarIncidentInside";
const SURFACE_FIELD: &str = "surface_name";
const CONSTRUCTION_FIELD: &str = "construction_name";
const SCHEDULE_FIELD: &str = "inside_surface_incident_sun_solar_radiation_schedule_name";

impl Compiler<'_> {
    pub(super) fn parse_surface_solar_incidents(&mut self, model: &mut TypedModel) {
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

            let surface_name = self.required_string(OBJECT_TYPE, &name, &object, SURFACE_FIELD);
            let construction_name =
                self.required_string(OBJECT_TYPE, &name, &object, CONSTRUCTION_FIELD);
            let schedule_name = self.required_string(OBJECT_TYPE, &name, &object, SCHEDULE_FIELD);

            let surface = surface_name.as_deref().and_then(|target_name| {
                self.resolve_name(
                    &model.surface_names,
                    OBJECT_TYPE,
                    &name,
                    SURFACE_FIELD,
                    target_name,
                    "BuildingSurface:Detailed",
                )
            });
            let construction = construction_name.as_deref().and_then(|target_name| {
                self.resolve_name(
                    &model.construction_names,
                    OBJECT_TYPE,
                    &name,
                    CONSTRUCTION_FIELD,
                    target_name,
                    "Construction",
                )
            });
            let schedule = schedule_name.as_deref().and_then(|target_name| {
                self.resolve_name(
                    &model.schedule_names,
                    OBJECT_TYPE,
                    &name,
                    SCHEDULE_FIELD,
                    target_name,
                    "Schedule",
                )
            });

            if self.diagnostics.len() != diagnostics_before_object {
                continue;
            }
            let (Some(surface), Some(construction), Some(schedule)) =
                (surface, construction, schedule)
            else {
                continue;
            };
            let Some(id_value) = self.checked_id(OBJECT_TYPE, &name, pending.len()) else {
                continue;
            };
            if !surface_construction_pairs.insert((surface, construction)) {
                self.error(
                    "DuplicateSurfaceSolarIncidentPair",
                    OBJECT_TYPE,
                    Some(&name),
                    Some(CONSTRUCTION_FIELD),
                    format!(
                        "{OBJECT_TYPE}/{name} repeats an existing normalized surface/construction pair"
                    ),
                );
                continue;
            }
            pending.push(SurfaceSolarIncident {
                id: SurfaceSolarIncidentId(id_value),
                name: NormalizedName::new(&name),
                surface,
                construction,
                schedule,
            });
        }

        let has_errors = self.diagnostics[diagnostics_before_pass..]
            .iter()
            .any(|diagnostic| diagnostic.severity == DiagnosticSeverity::Error);
        if !has_errors {
            model.surface_solar_incidents = pending;
        }
    }
}
