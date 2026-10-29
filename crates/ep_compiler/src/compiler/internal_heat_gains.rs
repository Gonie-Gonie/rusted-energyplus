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

    // Genuine singleton Space identity, independent of Space/Zone vector order.
    // Source preserves authored whitespace; generated defaults retain raw Zone spelling.
    fn source_whole_zone_gain_space_name(
        &mut self,
        model: &TypedModel,
        family: &str,
        object_name: &str,
        zone_id: ep_model::ZoneId,
    ) -> Option<String> {
        let zone = model.zones.iter().find(|row| row.id == zone_id)?;
        let spaces = model
            .spaces
            .iter()
            .filter(|row| row.zone == zone_id)
            .collect::<Vec<_>>();
        if spaces.len() != 1
            || zone.spaces.as_slice() != [spaces[0].id]
            || !matches!(
                spaces[0].origin,
                ep_model::SpaceOrigin::Authored | ep_model::SpaceOrigin::AutoZoneDefault
            )
        {
            self.error(
                &format!("Source{family}SpaceUnavailable"),
                family,
                Some(object_name),
                Some("zone_or_zonelist_or_space_or_spacelist_name"),
                format!("Source {family} requires exactly one whole-Zone Space and no remainder"),
            );
            return None;
        }
        let space = spaces[0];
        let raw_family = if space.origin == ep_model::SpaceOrigin::Authored {
            "Space"
        } else {
            "Zone"
        };
        let names = self
            .raw_model
            .objects
            .get(&ep_raw_model::ObjectType(raw_family.into()))
            .map(|rows| {
                rows.keys()
                    .filter(|name| {
                        if raw_family == "Space" {
                            model.authored_space_names.resolve(&name.0) == Some(space.id)
                        } else {
                            model.zone_names.resolve(&name.0) == Some(zone_id)
                        }
                    })
                    .map(|name| name.0.clone())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        if names.len() != 1
            || (if raw_family == "Space" {
                space.name != model.authored_space_names.normalized_name(&names[0])
            } else {
                space.name != ep_model::NormalizedName::new(&names[0])
            })
        {
            self.error(
                &format!("Source{family}SpaceIdentityUnavailable"),
                family,
                Some(object_name),
                Some("zone_or_zonelist_or_space_or_spacelist_name"),
                format!("Source {family} needs a unique raw/typed whole-Zone Space identity"),
            );
            return None;
        }
        Some(names[0].to_ascii_uppercase())
    }

    pub(super) fn internal_gain_input_targets(
        &mut self,
        model: &TypedModel,
        family: &str,
        name: &str,
        object: &RawObject,
    ) -> Option<(
        ep_model::SourceGainInputTarget,
        Vec<(ep_model::ZoneId, String)>,
    )> {
        const FIELD: &str = "zone_or_zonelist_or_space_or_spacelist_name";
        if !self.source_file_declarations {
            let reference = self.required_string(family, name, object, FIELD)?;
            let zone =
                self.resolve_name(&model.zone_names, family, name, FIELD, &reference, "Zone")?;
            return Some((
                ep_model::SourceGainInputTarget::Zone(zone),
                vec![(zone, name.into())],
            ));
        }
        let reference = self.source_reference_name(family, name, object, FIELD)?;
        // Native IHG3342-3394: Zone precedes Space, ZoneList, then SpaceList.
        // One-member ZoneLists still prefix every actual singleton Space name.
        if let Some(raw_zone) = self.source_raw_object_name("Zone", &reference) {
            let zone =
                self.resolve_name(&model.zone_names, family, name, FIELD, &raw_zone, "Zone")?;
            self.source_whole_zone_gain_space_name(model, family, name, zone)?;
            return Some((
                ep_model::SourceGainInputTarget::Zone(zone),
                vec![(zone, name.into())],
            ));
        }
        if let Some(raw_space) = self.source_raw_object_name("Space", &reference) {
            let id = self.resolve_name(
                &model.authored_space_names,
                family,
                name,
                FIELD,
                &raw_space,
                "Space",
            )?;
            let space = model.spaces.iter().find(|row| row.id == id)?;
            let actual = self.source_whole_zone_gain_space_name(model, family, name, space.zone)?;
            if actual != raw_space.to_ascii_uppercase() {
                self.error(
                    "InternalReferenceError",
                    family,
                    Some(name),
                    Some(FIELD),
                    "Direct Source Space identity differs".into(),
                );
                return None;
            }
            return Some((
                ep_model::SourceGainInputTarget::Space(id),
                vec![(space.zone, name.into())],
            ));
        }
        if let Some(raw_list) = self.source_raw_object_name("ZoneList", &reference) {
            let id = self.resolve_name(
                &model.zone_list_names,
                family,
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
                    family,
                    Some(name),
                    Some(FIELD),
                    format!("Source {family} ZoneList typed member association is unavailable"),
                );
                return None;
            };
            let mut targets = Vec::with_capacity(list.zones.len());
            for &zone in &list.zones {
                let space_name =
                    self.source_whole_zone_gain_space_name(model, family, name, zone)?;
                targets.push((zone, format!("{space_name} {}", name.to_ascii_uppercase())));
            }
            return Some((ep_model::SourceGainInputTarget::ZoneList(id), targets));
        }
        if let Some(raw_list) = self.source_raw_object_name("SpaceList", &reference) {
            let id = self.resolve_name(
                &model.space_list_names,
                family,
                name,
                FIELD,
                &raw_list,
                "SpaceList",
            )?;
            let Some(list) = model
                .space_lists
                .iter()
                .find(|list| list.id == id)
                .filter(|list| !list.spaces.is_empty())
            else {
                self.error(
                    &format!("Source{family}SpaceListUnavailable"),
                    family,
                    Some(name),
                    Some(FIELD),
                    format!("Source {family} requires a positive actual SpaceList member sequence"),
                );
                return None;
            };
            let mut targets = Vec::with_capacity(list.spaces.len());
            for &space_id in &list.spaces {
                let Some(space) = model.spaces.iter().find(|space| {
                    space.id == space_id && space.origin == ep_model::SpaceOrigin::Authored
                }) else {
                    self.error(
                        "InternalReferenceError",
                        family,
                        Some(name),
                        Some(FIELD),
                        format!(
                            "Source {family} SpaceList member must be an actual authored Space"
                        ),
                    );
                    return None;
                };
                let space_name =
                    self.source_whole_zone_gain_space_name(model, family, name, space.zone)?;
                if space_name != space.name.0 {
                    self.error(
                        "InternalReferenceError",
                        family,
                        Some(name),
                        Some(FIELD),
                        format!("Source {family} SpaceList member identity differs"),
                    );
                    return None;
                }
                // Native3411-3413 prefixes every member, including singleton lists.
                // Authored count/level operands stay whole for each genuine row.
                targets.push((
                    space.zone,
                    format!("{space_name} {}", name.to_ascii_uppercase()),
                ));
            }
            return Some((ep_model::SourceGainInputTarget::SpaceList(id), targets));
        }
        self.error(
            "MissingReference",
            family,
            Some(name),
            Some(FIELD),
            format!(
                "{family}/{name} references missing exact Source Zone, Space, ZoneList or SpaceList '{reference}'"
            ),
        );
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
    /// This pass projects the admitted Zone and ordered ZoneList `People` and
    /// `OtherEquipment` inputs; legacy compilation retains direct-Zone lookup. A pre-existing error suppresses the entire pass,
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
