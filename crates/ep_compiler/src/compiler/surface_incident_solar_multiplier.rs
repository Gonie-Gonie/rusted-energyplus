use std::collections::BTreeSet;

use ep_model::{
    NormalizedName, SurfaceIncidentSolarMultiplierRequest, SurfaceIncidentSolarMultiplierRequestId,
    TypedModel,
};

use super::{Compiler, DiagnosticSeverity};

const OBJECT_TYPE: &str = "SurfaceProperty:IncidentSolarMultiplier";

impl Compiler<'_> {
    pub(super) fn parse_surface_incident_solar_multiplier_requests(
        &mut self,
        model: &mut TypedModel,
    ) {
        let diagnostics_before_requests = self.diagnostics.len();
        let mut pending = Vec::new();
        let mut target_surface_names = BTreeSet::new();

        for (declaration_name, object) in self.objects(OBJECT_TYPE) {
            let diagnostics_before_fields = self.diagnostics.len();
            let surface_name =
                self.required_string(OBJECT_TYPE, &declaration_name, &object, "surface_name");
            let multiplier = self.number_bounded_blank_default(
                OBJECT_TYPE,
                &declaration_name,
                &object,
                "incident_solar_multiplier",
                1.0,
                (0.0, true),
                (1.0, true),
            );
            let schedule = match self.optional_reference_name_checked(
                OBJECT_TYPE,
                &declaration_name,
                &object,
                "incident_solar_multiplier_schedule_name",
            ) {
                Some(Some(schedule_name)) => self
                    .resolve_name(
                        &model.schedule_names,
                        OBJECT_TYPE,
                        &declaration_name,
                        "incident_solar_multiplier_schedule_name",
                        &schedule_name,
                        "Schedule",
                    )
                    .map(Some),
                Some(None) => Some(None),
                None => None,
            };

            if self.diagnostics.len() != diagnostics_before_fields {
                continue;
            }
            let Some(surface_name) = surface_name.as_deref().map(NormalizedName::new) else {
                continue;
            };
            let Some(schedule) = schedule else {
                continue;
            };
            if !target_surface_names.insert(surface_name.clone()) {
                self.error(
                    "DuplicateIncidentSolarMultiplierSurface",
                    OBJECT_TYPE,
                    Some(&declaration_name),
                    Some("surface_name"),
                    format!(
                        "{OBJECT_TYPE}/{declaration_name} repeats target surface '{}'; source-order last-wins surface mutation remains deferred, so duplicate targets are rejected",
                        surface_name.0
                    ),
                );
                continue;
            }
            let Some(id_value) = self.checked_id(OBJECT_TYPE, &declaration_name, pending.len())
            else {
                continue;
            };
            pending.push(SurfaceIncidentSolarMultiplierRequest {
                id: SurfaceIncidentSolarMultiplierRequestId(id_value),
                declaration_name: NormalizedName::new(&declaration_name),
                surface_name,
                multiplier,
                schedule,
            });
        }

        let has_errors = self.diagnostics[diagnostics_before_requests..]
            .iter()
            .any(|diagnostic| diagnostic.severity == DiagnosticSeverity::Error);
        if !has_errors {
            model.surface_incident_solar_multiplier_requests = pending;
        }
    }
}
