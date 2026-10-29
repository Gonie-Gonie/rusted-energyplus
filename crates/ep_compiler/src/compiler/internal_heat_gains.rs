//! Bounded internal-heat-gain input sequencing.

use super::{Compiler, DiagnosticSeverity};
use ep_model::{InternalGainScheduleReference, PeopleScheduleReference, TypedModel};
use ep_raw_model::RawObject;

impl Compiler<'_> {
    /// Exact raw Source reference spelling, before any legacy typed lookup.
    pub(super) fn source_reference_name(
        &mut self,
        object_type: &str,
        name: &str,
        object: &RawObject,
        field: &str,
    ) -> Option<String> {
        if self.source_file_declarations
            && let Some(ep_raw_model::RawValue::String(value)) = super::field_value(object, field)
            && !value.is_empty()
        {
            return Some(value.clone());
        }
        self.required_string(object_type, name, object, field)
    }

    pub(super) fn source_raw_object_name(&self, family: &str, reference: &str) -> Option<String> {
        self.raw_model
            .objects
            .get(&ep_raw_model::ObjectType(family.into()))?
            .keys()
            .find(|name| name.0.eq_ignore_ascii_case(reference))
            .map(|name| name.0.clone())
    }

    // Native implicit GetSpaceData copies the actual Zone.Name verbatim. The
    // typed Space is real, but its legacy NormalizedName is not Source spelling.
    fn source_implicit_space_name(
        &mut self,
        model: &TypedModel,
        object_name: &str,
        zone_id: ep_model::ZoneId,
    ) -> Option<String> {
        let Some(zone) = model.zones.iter().find(|zone| zone.id == zone_id) else {
            self.error(
                "InternalReferenceError",
                "OtherEquipment",
                Some(object_name),
                None,
                "Source OtherEquipment member Zone identity is unavailable".into(),
            );
            return None;
        };
        let spaces = model
            .spaces
            .iter()
            .filter(|space| space.zone == zone_id)
            .collect::<Vec<_>>();
        if spaces.len() != 1
            || spaces[0].origin != ep_model::SpaceOrigin::AutoZoneDefault
            || zone.spaces.as_slice() != [spaces[0].id]
        {
            self.error("SourceOtherEquipmentSpaceUnavailable", "OtherEquipment", Some(object_name),
                Some("zone_or_zonelist_or_space_or_spacelist_name"),
                "Source OtherEquipment requires the actual sole implicit Space for each member Zone".into());
            return None;
        }
        let names = self
            .raw_model
            .objects
            .get(&ep_raw_model::ObjectType("Zone".into()))
            .map(|rows| {
                rows.keys()
                    .filter(|name| model.zone_names.resolve(&name.0) == Some(zone_id))
                    .map(|name| name.0.clone())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        if names.len() != 1 || spaces[0].name != ep_model::NormalizedName::new(&names[0]) {
            self.error("SourceOtherEquipmentZoneIdentityUnavailable", "OtherEquipment", Some(object_name),
                Some("zone_or_zonelist_or_space_or_spacelist_name"),
                "Source OtherEquipment member needs a unique real raw Zone/typed implicit-Space association".into());
            return None;
        }
        Some(names[0].to_ascii_uppercase())
    }

    pub(super) fn other_equipment_input_targets(
        &mut self,
        model: &TypedModel,
        name: &str,
        object: &RawObject,
    ) -> Option<(
        ep_model::SourceOtherEquipmentInputTarget,
        Vec<(ep_model::ZoneId, String)>,
    )> {
        const FIELD: &str = "zone_or_zonelist_or_space_or_spacelist_name";
        if !self.source_file_declarations {
            let reference = self.required_string("OtherEquipment", name, object, FIELD)?;
            let zone = self.resolve_name(
                &model.zone_names,
                "OtherEquipment",
                name,
                FIELD,
                &reference,
                "Zone",
            )?;
            return Some((
                ep_model::SourceOtherEquipmentInputTarget::Zone(zone),
                vec![(zone, name.into())],
            ));
        }
        let reference = self.source_reference_name("OtherEquipment", name, object, FIELD)?;
        // Native IHG3342-3394: Zone wins even when a ZoneList has the same name;
        // an authored Space wins over a list but remains outside this stage.
        if let Some(raw_zone) = self.source_raw_object_name("Zone", &reference) {
            let zone = self.resolve_name(
                &model.zone_names,
                "OtherEquipment",
                name,
                FIELD,
                &raw_zone,
                "Zone",
            )?;
            return Some((
                ep_model::SourceOtherEquipmentInputTarget::Zone(zone),
                vec![(zone, name.into())],
            ));
        }
        if self.source_raw_object_name("Space", &reference).is_some() {
            self.error(
                "SourceOtherEquipmentSpaceUnavailable",
                "OtherEquipment",
                Some(name),
                Some(FIELD),
                "Direct authored Space OtherEquipment remains outside the Source list stage".into(),
            );
            return None;
        }
        if let Some(raw_list) = self.source_raw_object_name("ZoneList", &reference) {
            let id = self.resolve_name(
                &model.zone_list_names,
                "OtherEquipment",
                name,
                FIELD,
                &raw_list,
                "ZoneList",
            )?;
            let Some(list) = model
                .zone_lists
                .iter()
                .find(|list| list.id == id)
                .filter(|list| !list.zones.is_empty())
            else {
                self.error(
                    "InternalReferenceError",
                    "OtherEquipment",
                    Some(name),
                    Some(FIELD),
                    "Source OtherEquipment ZoneList typed member association is unavailable".into(),
                );
                return None;
            };
            let mut targets = Vec::with_capacity(list.zones.len());
            for &zone in &list.zones {
                let space_name = self.source_implicit_space_name(model, name, zone)?;
                targets.push((zone, format!("{space_name} {}", name.to_ascii_uppercase())));
            }
            return Some((
                ep_model::SourceOtherEquipmentInputTarget::ZoneList(id),
                targets,
            ));
        }
        if self
            .source_raw_object_name("SpaceList", &reference)
            .is_some()
        {
            self.error(
                "SourceOtherEquipmentSpaceListUnavailable",
                "OtherEquipment",
                Some(name),
                Some(FIELD),
                "SpaceList OtherEquipment remains outside the Source list stage".into(),
            );
            return None;
        }
        self.error("MissingReference", "OtherEquipment", Some(name), Some(FIELD),
            format!("OtherEquipment/{name} references missing exact Source Zone or ZoneList '{reference}'"));
        None
    }

    /// Preserve the actual built-in identity without manufacturing a user schedule.
    /// Native creates these names before reading user schedules, so a user name
    /// collision is a duplicate error rather than an alternate built-in value.
    fn internal_gain_schedule_reference(
        &mut self,
        model: &TypedModel,
        object_type: &str,
        name: &str,
        object: &RawObject,
        field: &str,
        ambiguous_code: &str,
    ) -> Option<InternalGainScheduleReference> {
        let reference = self.optional_schedule_name(object_type, name, object, field)?;
        let built_in = if reference.eq_ignore_ascii_case("Constant-0.0") {
            Some(InternalGainScheduleReference::ConstantZero)
        } else if reference.eq_ignore_ascii_case("Constant-1.0") {
            Some(InternalGainScheduleReference::ConstantOne)
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
                    ambiguous_code,
                    object_type,
                    Some(name),
                    Some(field),
                    format!("{object_type}/{name} schedule reference '{reference}' collides with an EnergyPlus built-in schedule"),
                );
                None
            } else {
                Some(built_in)
            }
        } else {
            self.resolve_name(
                &model.schedule_names,
                object_type,
                name,
                field,
                &reference,
                "Schedule",
            )
            .map(InternalGainScheduleReference::User)
        }
    }

    pub(super) fn people_schedule_reference(
        &mut self,
        model: &TypedModel,
        name: &str,
        object: &RawObject,
    ) -> Option<PeopleScheduleReference> {
        self.internal_gain_schedule_reference(
            model,
            "People",
            name,
            object,
            "number_of_people_schedule_name",
            "AmbiguousPeopleSchedule",
        )
    }

    pub(super) fn other_equipment_schedule_reference(
        &mut self,
        model: &TypedModel,
        name: &str,
        object: &RawObject,
    ) -> Option<InternalGainScheduleReference> {
        if self.source_file_declarations {
            return self.internal_gain_schedule_reference(
                model,
                "OtherEquipment",
                name,
                object,
                "schedule_name",
                "AmbiguousOtherEquipmentSchedule",
            );
        }
        // Unscoped compilation keeps the old authored-schedule-only boundary.
        let reference =
            self.optional_schedule_name("OtherEquipment", name, object, "schedule_name")?;
        self.resolve_name(
            &model.schedule_names,
            "OtherEquipment",
            name,
            "schedule_name",
            &reference,
            "Schedule",
        )
        .map(InternalGainScheduleReference::User)
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
