use std::collections::BTreeSet;

use ep_model::{
    AutoOrNumber, NormalizedName, Space, SpaceId, SpaceList, SpaceListId, SpaceOrigin, SpaceTypeId,
    TypedModel,
};
use ep_raw_model::{RawObject, RawValue};

use super::{Compiler, field_value};

const SPACE_OBJECT_TYPE: &str = "Space";
const SPACE_LIST_OBJECT_TYPE: &str = "SpaceList";
const GENERAL_SPACE_TYPE: &str = "General";

impl Compiler<'_> {
    pub(super) fn parse_space_data(&mut self, model: &mut TypedModel) {
        self.parse_authored_spaces(model);
        self.parse_space_lists(model);
        self.generate_default_spaces(model);
    }

    fn parse_authored_spaces(&mut self, model: &mut TypedModel) {
        for (name, object) in self.objects(SPACE_OBJECT_TYPE) {
            let diagnostics_before_fields = self.diagnostics.len();
            if name.trim().is_empty() {
                self.error(
                    "MissingRequiredField",
                    SPACE_OBJECT_TYPE,
                    Some(&name),
                    Some("name"),
                    "Space requires a non-blank name".to_string(),
                );
            }

            let zone_name = self.required_string(SPACE_OBJECT_TYPE, &name, &object, "zone_name");
            let zone = zone_name.as_deref().and_then(|zone_name| {
                self.resolve_name(
                    &model.zone_names,
                    SPACE_OBJECT_TYPE,
                    &name,
                    "zone_name",
                    zone_name,
                    "Zone",
                )
            });
            let ceiling_height = self.auto_default(
                SPACE_OBJECT_TYPE,
                &name,
                &object,
                "ceiling_height",
                AutoOrNumber::AutoCalculate,
                "Autocalculate",
            );
            let volume = self.auto_default(
                SPACE_OBJECT_TYPE,
                &name,
                &object,
                "volume",
                AutoOrNumber::AutoCalculate,
                "Autocalculate",
            );
            let floor_area = self.auto_default(
                SPACE_OBJECT_TYPE,
                &name,
                &object,
                "floor_area",
                AutoOrNumber::AutoCalculate,
                "Autocalculate",
            );
            let space_type = self.space_type_label(&name, &object);
            let tags = self.space_tags(&name, &object);

            if self.diagnostics.len() != diagnostics_before_fields {
                continue;
            }
            let Some(zone) = zone else {
                continue;
            };
            let Some(space_type) = space_type else {
                continue;
            };
            let Some(tags) = tags else {
                continue;
            };
            if model.authored_space_names.resolve(&name).is_some() {
                self.duplicate_name(SPACE_OBJECT_TYPE, &name);
                continue;
            }
            let Some(zone_index) = self.space_zone_index(model, &name, zone.0) else {
                continue;
            };
            let Some(id_value) = self.checked_id(SPACE_OBJECT_TYPE, &name, model.spaces.len())
            else {
                continue;
            };
            let Some(space_type_id) = self.register_space_type(model, &name, &space_type) else {
                continue;
            };

            let id = SpaceId(id_value);
            let existing = model.authored_space_names.insert(&name, id);
            debug_assert!(existing.is_none());
            model.spaces.push(Space {
                id,
                name: NormalizedName::new(&name),
                zone,
                ceiling_height,
                volume,
                floor_area,
                space_type: NormalizedName::new(&space_type),
                space_type_id,
                tags,
                origin: SpaceOrigin::Authored,
            });
            model.zones[zone_index].spaces.push(id);
        }
    }

    fn parse_space_lists(&mut self, model: &mut TypedModel) {
        for (name, object) in self.objects(SPACE_LIST_OBJECT_TYPE) {
            let diagnostics_before_fields = self.diagnostics.len();
            if name.trim().is_empty() {
                self.error(
                    "MissingRequiredField",
                    SPACE_LIST_OBJECT_TYPE,
                    Some(&name),
                    Some("name"),
                    "SpaceList requires a non-blank name".to_string(),
                );
            }

            if model.zone_names.resolve(&name).is_some() {
                self.error(
                    "SpaceListNameMatchesZone",
                    SPACE_LIST_OBJECT_TYPE,
                    Some(&name),
                    Some("name"),
                    format!("SpaceList/{name} duplicates a Zone name"),
                );
            }
            if model.authored_space_names.resolve(&name).is_some() {
                self.error(
                    "SpaceListNameMatchesSpace",
                    SPACE_LIST_OBJECT_TYPE,
                    Some(&name),
                    Some("name"),
                    format!("SpaceList/{name} duplicates a Space name"),
                );
            }

            let members = self.space_list_members(model, &name, &object);
            if self.diagnostics.len() != diagnostics_before_fields {
                continue;
            }
            let Some((spaces, max_space_name_length)) = members else {
                continue;
            };
            if model.space_list_names.resolve(&name).is_some() {
                self.duplicate_name(SPACE_LIST_OBJECT_TYPE, &name);
                continue;
            }
            let Some(id_value) =
                self.checked_id(SPACE_LIST_OBJECT_TYPE, &name, model.space_lists.len())
            else {
                continue;
            };
            let id = SpaceListId(id_value);
            let existing = model.space_list_names.insert(&name, id);
            debug_assert!(existing.is_none());
            model.space_lists.push(SpaceList {
                id,
                name: NormalizedName::new(&name),
                spaces,
                max_space_name_length,
            });
        }
    }

    fn generate_default_spaces(&mut self, model: &mut TypedModel) {
        for zone_index in 0..model.zones.len() {
            if !model.zones[zone_index].spaces.is_empty() {
                continue;
            }

            let zone = model.zones[zone_index].id;
            let name = model.zones[zone_index].name.0.clone();
            let Some(id_value) = self.checked_id(SPACE_OBJECT_TYPE, &name, model.spaces.len())
            else {
                continue;
            };
            let Some(space_type_id) = self.register_space_type(model, &name, GENERAL_SPACE_TYPE)
            else {
                continue;
            };
            let id = SpaceId(id_value);
            model.spaces.push(Space {
                id,
                name: NormalizedName::new(&name),
                zone,
                ceiling_height: AutoOrNumber::AutoCalculate,
                volume: AutoOrNumber::AutoCalculate,
                floor_area: AutoOrNumber::AutoCalculate,
                space_type: NormalizedName::new(GENERAL_SPACE_TYPE),
                space_type_id,
                tags: Vec::new(),
                origin: SpaceOrigin::AutoZoneDefault,
            });
            model.zones[zone_index].spaces.push(id);
        }
    }

    fn space_type_label(&mut self, object_name: &str, object: &RawObject) -> Option<String> {
        match field_value(object, "space_type") {
            Some(RawValue::String(value)) if !value.trim().is_empty() => Some(value.clone()),
            Some(RawValue::String(_)) => Some(GENERAL_SPACE_TYPE.to_string()),
            Some(_) => {
                self.invalid_field_type(SPACE_OBJECT_TYPE, object_name, "space_type", "string");
                None
            }
            None => {
                self.record_default(
                    SPACE_OBJECT_TYPE,
                    object_name,
                    "space_type",
                    GENERAL_SPACE_TYPE,
                );
                Some(GENERAL_SPACE_TYPE.to_string())
            }
        }
    }

    fn space_tags(&mut self, object_name: &str, object: &RawObject) -> Option<Vec<NormalizedName>> {
        let Some(value) = field_value(object, "tags") else {
            return Some(Vec::new());
        };
        let RawValue::Array(values) = value else {
            self.invalid_field_type(SPACE_OBJECT_TYPE, object_name, "tags", "array");
            return None;
        };

        let diagnostics_before_tags = self.diagnostics.len();
        let mut tags = Vec::with_capacity(values.len());
        for (index, value) in values.iter().enumerate() {
            let RawValue::Object(fields) = value else {
                self.error(
                    "InvalidFieldType",
                    SPACE_OBJECT_TYPE,
                    Some(object_name),
                    Some("tags"),
                    format!("Space/{object_name} tag entry {index} must be an object"),
                );
                continue;
            };
            let entry_object = RawObject {
                fields: fields.clone(),
                source_span: None,
            };
            match field_value(&entry_object, "tag") {
                Some(RawValue::String(tag)) => tags.push(NormalizedName::new(tag)),
                None => tags.push(NormalizedName::new("")),
                Some(_) => self.invalid_field_type(
                    SPACE_OBJECT_TYPE,
                    &format!("{object_name}[{index}]"),
                    "tag",
                    "string",
                ),
            }
        }

        if self.diagnostics.len() != diagnostics_before_tags {
            None
        } else {
            Some(tags)
        }
    }

    fn space_list_members(
        &mut self,
        model: &TypedModel,
        object_name: &str,
        object: &RawObject,
    ) -> Option<(Vec<SpaceId>, usize)> {
        let Some(value) = field_value(object, "spaces") else {
            return Some((Vec::new(), 0));
        };
        let RawValue::Array(values) = value else {
            self.invalid_field_type(SPACE_LIST_OBJECT_TYPE, object_name, "spaces", "array");
            return None;
        };

        let diagnostics_before_members = self.diagnostics.len();
        let mut spaces = Vec::with_capacity(values.len());
        let mut seen = BTreeSet::new();
        let mut max_space_name_length = 0;
        for (index, value) in values.iter().enumerate() {
            let RawValue::Object(fields) = value else {
                self.error(
                    "InvalidFieldType",
                    SPACE_LIST_OBJECT_TYPE,
                    Some(object_name),
                    Some("spaces"),
                    format!("SpaceList/{object_name} space entry {index} must be an object"),
                );
                continue;
            };
            let entry_object = RawObject {
                fields: fields.clone(),
                source_span: None,
            };
            let entry_name = format!("{object_name}[{index}]");
            let Some(space_name) = self.required_string(
                SPACE_LIST_OBJECT_TYPE,
                &entry_name,
                &entry_object,
                "space_name",
            ) else {
                continue;
            };
            max_space_name_length = max_space_name_length.max(space_name.len());
            let Some(space) = self.resolve_name(
                &model.authored_space_names,
                SPACE_LIST_OBJECT_TYPE,
                object_name,
                "space_name",
                &space_name,
                SPACE_OBJECT_TYPE,
            ) else {
                continue;
            };
            if !seen.insert(space) {
                self.error(
                    "DuplicateSpaceListMember",
                    SPACE_LIST_OBJECT_TYPE,
                    Some(object_name),
                    Some("space_name"),
                    format!(
                        "SpaceList/{object_name} includes Space '{}' more than once",
                        NormalizedName::new(&space_name).0
                    ),
                );
                continue;
            }
            spaces.push(space);
        }

        if self.diagnostics.len() != diagnostics_before_members {
            None
        } else {
            Some((spaces, max_space_name_length))
        }
    }

    fn register_space_type(
        &mut self,
        model: &mut TypedModel,
        object_name: &str,
        space_type: &str,
    ) -> Option<SpaceTypeId> {
        if let Some(id) = model.space_type_names.resolve(space_type) {
            return Some(id);
        }
        let id_value =
            self.checked_id(SPACE_OBJECT_TYPE, object_name, model.space_type_names.len())?;
        let id = SpaceTypeId(id_value);
        let existing = model.space_type_names.insert(space_type, id);
        debug_assert!(existing.is_none());
        Some(id)
    }

    fn space_zone_index(
        &mut self,
        model: &TypedModel,
        object_name: &str,
        zone_id: u32,
    ) -> Option<usize> {
        let Ok(zone_index) = usize::try_from(zone_id) else {
            self.internal_space_reference_error(object_name, "Zone", zone_id);
            return None;
        };
        if model.zones.get(zone_index).is_none() {
            self.internal_space_reference_error(object_name, "Zone", zone_id);
            return None;
        }
        Some(zone_index)
    }

    fn internal_space_reference_error(
        &mut self,
        object_name: &str,
        target_type: &str,
        target_id: u32,
    ) {
        self.error(
            "InternalReferenceError",
            SPACE_OBJECT_TYPE,
            Some(object_name),
            None,
            format!("Space/{object_name} resolved an unavailable {target_type} id {target_id}"),
        );
    }
}
