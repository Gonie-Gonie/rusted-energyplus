use std::collections::BTreeSet;

use ep_model::{NormalizedName, TypedModel, ZoneGroup, ZoneGroupId, ZoneId, ZoneList, ZoneListId};
use ep_raw_model::{RawObject, RawValue};

use super::{Compiler, field_value};

const ZONE_LIST_OBJECT_TYPE: &str = "ZoneList";
const ZONE_GROUP_OBJECT_TYPE: &str = "ZoneGroup";
const ZONE_EQUIPMENT_CONNECTIONS_OBJECT_TYPE: &str = "ZoneHVAC:EquipmentConnections";

impl Compiler<'_> {
    pub(super) fn mark_nominal_controlled_zones(&mut self, model: &mut TypedModel) {
        let controlled_zone_names = self
            .objects(ZONE_EQUIPMENT_CONNECTIONS_OBJECT_TYPE)
            .into_iter()
            .filter_map(|(_name, object)| match field_value(&object, "zone_name") {
                Some(RawValue::String(zone_name)) if !zone_name.trim().is_empty() => {
                    Some(NormalizedName::new(zone_name))
                }
                _ => None,
            })
            .collect::<BTreeSet<_>>();

        for zone in &mut model.zones {
            zone.is_nominal_controlled = controlled_zone_names.contains(&zone.name);
        }
    }

    pub(super) fn parse_zone_lists(&mut self, model: &mut TypedModel) {
        for (name, object) in self.objects(ZONE_LIST_OBJECT_TYPE) {
            let diagnostics_before_fields = self.diagnostics.len();
            if name.trim().is_empty() {
                self.error(
                    "MissingRequiredField",
                    ZONE_LIST_OBJECT_TYPE,
                    Some(&name),
                    Some("name"),
                    "ZoneList requires a non-blank name".to_string(),
                );
            }

            let members = self.zone_list_members(model, &name, &object);
            if self.diagnostics.len() != diagnostics_before_fields {
                continue;
            }
            let Some((zones, max_zone_name_length)) = members else {
                continue;
            };
            if model.zone_list_names.resolve(&name).is_some() {
                self.duplicate_name(ZONE_LIST_OBJECT_TYPE, &name);
                continue;
            }
            let Some(id_value) =
                self.checked_id(ZONE_LIST_OBJECT_TYPE, &name, model.zone_lists.len())
            else {
                continue;
            };
            let id = ZoneListId(id_value);

            if model.zone_names.resolve(&name).is_some() {
                self.warning(
                    "ZoneListNameMatchesZone",
                    ZONE_LIST_OBJECT_TYPE,
                    Some(&name),
                    Some("name"),
                    format!(
                        "ZoneList/{name} duplicates a Zone name and may be ambiguous where either name is accepted"
                    ),
                );
            }

            let existing = model.zone_list_names.insert(&name, id);
            debug_assert!(existing.is_none());
            model.zone_lists.push(ZoneList {
                id,
                name: NormalizedName::new(&name),
                zones,
                max_zone_name_length,
            });
        }
    }

    pub(super) fn parse_zone_groups(&mut self, model: &mut TypedModel) {
        for (name, object) in self.objects(ZONE_GROUP_OBJECT_TYPE) {
            let diagnostics_before_fields = self.diagnostics.len();
            if name.trim().is_empty() {
                self.error(
                    "MissingRequiredField",
                    ZONE_GROUP_OBJECT_TYPE,
                    Some(&name),
                    Some("name"),
                    "ZoneGroup requires a non-blank name".to_string(),
                );
            }

            let zone_list_name =
                self.required_string(ZONE_GROUP_OBJECT_TYPE, &name, &object, "zone_list_name");
            let zone_list = zone_list_name.as_deref().and_then(|zone_list_name| {
                self.resolve_name(
                    &model.zone_list_names,
                    ZONE_GROUP_OBJECT_TYPE,
                    &name,
                    "zone_list_name",
                    zone_list_name,
                    ZONE_LIST_OBJECT_TYPE,
                )
            });
            let multiplier = self.u32_default(
                ZONE_GROUP_OBJECT_TYPE,
                &name,
                &object,
                "zone_list_multiplier",
                1,
            );
            if multiplier == 0 || multiplier > i32::MAX as u32 {
                self.error(
                    "InvalidNumericRange",
                    ZONE_GROUP_OBJECT_TYPE,
                    Some(&name),
                    Some("zone_list_multiplier"),
                    format!(
                        "ZoneGroup/{name} field zone_list_multiplier must be between 1 and {}, got {multiplier}",
                        i32::MAX
                    ),
                );
            }

            if self.diagnostics.len() != diagnostics_before_fields {
                continue;
            }
            let Some(zone_list) = zone_list else {
                continue;
            };
            if model.zone_group_names.resolve(&name).is_some() {
                self.duplicate_name(ZONE_GROUP_OBJECT_TYPE, &name);
                continue;
            }
            if let Some(existing_group) = model
                .zone_groups
                .iter()
                .find(|group| group.zone_list == zone_list)
            {
                self.error(
                    "DuplicateZoneGroupList",
                    ZONE_GROUP_OBJECT_TYPE,
                    Some(&name),
                    Some("zone_list_name"),
                    format!(
                        "ZoneGroup/{name} reuses the ZoneList already assigned by ZoneGroup/{}",
                        existing_group.name.0
                    ),
                );
                continue;
            }

            let Ok(zone_list_index) = usize::try_from(zone_list.0) else {
                self.error(
                    "InternalReferenceError",
                    ZONE_GROUP_OBJECT_TYPE,
                    Some(&name),
                    Some("zone_list_name"),
                    format!(
                        "ZoneGroup/{name} resolved an unavailable ZoneList id {}",
                        zone_list.0
                    ),
                );
                continue;
            };
            let Some(zone_list_record) = model.zone_lists.get(zone_list_index) else {
                self.error(
                    "InternalReferenceError",
                    ZONE_GROUP_OBJECT_TYPE,
                    Some(&name),
                    Some("zone_list_name"),
                    format!(
                        "ZoneGroup/{name} resolved an unavailable ZoneList id {}",
                        zone_list.0
                    ),
                );
                continue;
            };
            let zone_ids = zone_list_record.zones.clone();
            let mut zone_indexes = Vec::with_capacity(zone_ids.len());
            let mut overlap_found = false;
            for zone_id in &zone_ids {
                let Ok(zone_index) = usize::try_from(zone_id.0) else {
                    self.error(
                        "InternalReferenceError",
                        ZONE_GROUP_OBJECT_TYPE,
                        Some(&name),
                        Some("zone_list_name"),
                        format!(
                            "ZoneGroup/{name} resolved an unavailable Zone id {}",
                            zone_id.0
                        ),
                    );
                    overlap_found = true;
                    continue;
                };
                let Some(zone) = model.zones.get(zone_index) else {
                    self.error(
                        "InternalReferenceError",
                        ZONE_GROUP_OBJECT_TYPE,
                        Some(&name),
                        Some("zone_list_name"),
                        format!(
                            "ZoneGroup/{name} resolved an unavailable Zone id {}",
                            zone_id.0
                        ),
                    );
                    overlap_found = true;
                    continue;
                };
                zone_indexes.push(zone_index);
                if let Some(previous_list) = zone.list_group {
                    self.error(
                        "ZoneInMultipleGroups",
                        ZONE_GROUP_OBJECT_TYPE,
                        Some(&name),
                        Some("zone_list_name"),
                        format!(
                            "ZoneGroup/{name} assigns Zone '{}' through more than one grouped ZoneList (previous list id {})",
                            zone.name.0, previous_list.0
                        ),
                    );
                    overlap_found = true;
                }
            }
            if overlap_found {
                continue;
            }

            let Some(id_value) =
                self.checked_id(ZONE_GROUP_OBJECT_TYPE, &name, model.zone_groups.len())
            else {
                continue;
            };
            let id = ZoneGroupId(id_value);
            let existing = model.zone_group_names.insert(&name, id);
            debug_assert!(existing.is_none());
            for zone_index in zone_indexes {
                let zone = &mut model.zones[zone_index];
                zone.list_multiplier = multiplier;
                zone.list_group = Some(zone_list);
            }
            model.zone_groups.push(ZoneGroup {
                id,
                name: NormalizedName::new(&name),
                zone_list,
                multiplier,
            });
        }
    }

    fn zone_list_members(
        &mut self,
        model: &TypedModel,
        object_name: &str,
        object: &RawObject,
    ) -> Option<(Vec<ZoneId>, usize)> {
        let Some(value) = field_value(object, "zones") else {
            self.error(
                "MissingZoneListMember",
                ZONE_LIST_OBJECT_TYPE,
                Some(object_name),
                Some("zones"),
                format!("ZoneList/{object_name} has no zones specified"),
            );
            return None;
        };
        let RawValue::Array(values) = value else {
            self.invalid_field_type(ZONE_LIST_OBJECT_TYPE, object_name, "zones", "array");
            return None;
        };
        if values.is_empty() {
            self.error(
                "MissingZoneListMember",
                ZONE_LIST_OBJECT_TYPE,
                Some(object_name),
                Some("zones"),
                format!("ZoneList/{object_name} has no zones specified"),
            );
            return None;
        }

        let diagnostics_before_members = self.diagnostics.len();
        let mut zones = Vec::with_capacity(values.len());
        let mut seen = BTreeSet::new();
        let mut max_zone_name_length = 0;
        for (index, value) in values.iter().enumerate() {
            let RawValue::Object(fields) = value else {
                self.error(
                    "InvalidFieldType",
                    ZONE_LIST_OBJECT_TYPE,
                    Some(object_name),
                    Some("zones"),
                    format!("ZoneList/{object_name} zone entry {index} must be an object"),
                );
                continue;
            };
            let entry_object = RawObject {
                fields: fields.clone(),
                source_span: None,
            };
            let entry_name = format!("{object_name}[{index}]");
            let Some(zone_name) = self.required_string(
                ZONE_LIST_OBJECT_TYPE,
                &entry_name,
                &entry_object,
                "zone_name",
            ) else {
                continue;
            };
            max_zone_name_length = max_zone_name_length.max(zone_name.len());
            let Some(zone) = self.resolve_name(
                &model.zone_names,
                ZONE_LIST_OBJECT_TYPE,
                object_name,
                "zone_name",
                &zone_name,
                "Zone",
            ) else {
                continue;
            };
            if !seen.insert(zone) {
                self.error(
                    "DuplicateZoneListMember",
                    ZONE_LIST_OBJECT_TYPE,
                    Some(object_name),
                    Some("zone_name"),
                    format!(
                        "ZoneList/{object_name} includes Zone '{}' more than once",
                        NormalizedName::new(&zone_name).0
                    ),
                );
                continue;
            }
            zones.push(zone);
        }

        if self.diagnostics.len() != diagnostics_before_members {
            return None;
        }
        Some((zones, max_zone_name_length))
    }
}
