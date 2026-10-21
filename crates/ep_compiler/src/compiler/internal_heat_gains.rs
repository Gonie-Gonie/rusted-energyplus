//! Bounded internal-heat-gain input sequencing.

use super::{Compiler, DiagnosticSeverity};
use ep_model::{PeopleScheduleReference, TypedModel};
use ep_raw_model::RawObject;

impl Compiler<'_> {
    /// Preserve the actual built-in identity without manufacturing a user schedule.
    /// Native creates these names before reading user schedules, so a user name
    /// collision is a duplicate error rather than an alternate built-in value.
    pub(super) fn people_schedule_reference(
        &mut self,
        model: &TypedModel,
        name: &str,
        object: &RawObject,
    ) -> Option<PeopleScheduleReference> {
        const FIELD: &str = "number_of_people_schedule_name";
        let reference = self.optional_string("People", name, object, FIELD)?;
        let built_in = if reference.eq_ignore_ascii_case("Constant-0.0") {
            Some(PeopleScheduleReference::ConstantZero)
        } else if reference.eq_ignore_ascii_case("Constant-1.0") {
            Some(PeopleScheduleReference::ConstantOne)
        } else {
            None
        };
        if let Some(built_in) = built_in {
            // Native keys retain whitespace. Default/legacy typed maps trim
            // names; this raw scan preserves built-in collisions for both policies.
            let exact_collision = self.raw_model.objects.iter().any(|(family, objects)| {
                match family.0.as_str() {
                    "Schedule:Constant"
                    | "Schedule:Compact"
                    | "Schedule:Year"
                    | "Schedule:File"
                    | "ExternalInterface:Schedule"
                    | "ExternalInterface:FunctionalMockupUnitImport:To:Schedule" => objects
                        .keys()
                        .any(|name| name.0.eq_ignore_ascii_case(&reference)),
                    "ExternalInterface:FunctionalMockupUnitExport:To:Schedule" => {
                        objects.values().any(|object| {
                            matches!(
                                super::field_value(object, "schedule_name"),
                                Some(ep_raw_model::RawValue::String(name))
                                    if name.eq_ignore_ascii_case(&reference)
                            )
                        })
                    }
                    // File:Shading names always have the generated _shading
                    // suffix and cannot equal either reserved built-in name.
                    _ => false,
                }
            });
            if exact_collision {
                self.error(
                    "AmbiguousPeopleSchedule",
                    "People",
                    Some(name),
                    Some(FIELD),
                    format!("People/{name} schedule reference '{reference}' collides with an EnergyPlus built-in schedule"),
                );
                None
            } else {
                Some(built_in)
            }
        } else {
            self.resolve_name(
                &model.schedule_names,
                "People",
                name,
                FIELD,
                &reference,
                "Schedule",
            )
            .map(PeopleScheduleReference::User)
        }
    }

    /// Parses the currently typed internal-gain families in source order.
    ///
    /// This bounded pass only projects direct-Zone `People` and
    /// `OtherEquipment` inputs. A pre-existing error suppresses the entire pass,
    /// while an error raised by `People` does not suppress the subsequent
    /// `OtherEquipment` scan.
    pub(super) fn parse_bounded_internal_heat_gains_input(&mut self, model: &mut TypedModel) {
        if self
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity == DiagnosticSeverity::Error)
        {
            return;
        }

        self.parse_people(model);
        // Deferred here: Lights, Electric/Gas/HotWater/Steam equipment.
        self.parse_other_equipment(model);
        // Deferred here: ITE, outdoor-temperature baseboard, and CO2 gains.
    }
}
