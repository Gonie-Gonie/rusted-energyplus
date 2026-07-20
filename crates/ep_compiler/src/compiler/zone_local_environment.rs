use ep_model::{NodeId, NormalizedName, TypedModel, ZoneLocalEnvironment, ZoneLocalEnvironmentId};

use super::Compiler;

const OBJECT_TYPE: &str = "ZoneProperty:LocalEnvironment";

enum NodeCandidate {
    Blank,
    Existing(NodeId),
    New(String),
}

impl Compiler<'_> {
    pub(super) fn parse_zone_local_environments(&mut self, model: &mut TypedModel) {
        for (name, object) in self.objects(OBJECT_TYPE) {
            let diagnostics_before_fields = self.diagnostics.len();
            if name.trim().is_empty() {
                self.error(
                    "MissingRequiredField",
                    OBJECT_TYPE,
                    Some(&name),
                    Some("name"),
                    "ZoneProperty:LocalEnvironment requires a non-blank name".to_string(),
                );
            }

            let zone_name = self.required_string(OBJECT_TYPE, &name, &object, "zone_name");
            let zone = zone_name.as_deref().and_then(|zone_name| {
                self.resolve_name(
                    &model.zone_names,
                    OBJECT_TYPE,
                    &name,
                    "zone_name",
                    zone_name,
                    "Zone",
                )
            });
            let node_candidate = self.zone_local_environment_node_candidate(model, &name, &object);

            if self.diagnostics.len() != diagnostics_before_fields {
                continue;
            }
            let Some(zone) = zone else {
                continue;
            };
            let Some(node_candidate) = node_candidate else {
                continue;
            };
            if model.zone_local_environment_names.resolve(&name).is_some() {
                self.duplicate_name(OBJECT_TYPE, &name);
                continue;
            }
            let Ok(zone_index) = usize::try_from(zone.0) else {
                self.internal_zone_local_environment_reference_error(&name, "Zone", zone.0);
                continue;
            };
            if model.zones.get(zone_index).is_none() {
                self.internal_zone_local_environment_reference_error(&name, "Zone", zone.0);
                continue;
            }
            let Some(id_value) =
                self.checked_id(OBJECT_TYPE, &name, model.zone_local_environments.len())
            else {
                continue;
            };

            let outdoor_air_node = match node_candidate {
                NodeCandidate::Blank => None,
                NodeCandidate::Existing(node) => Some(node),
                NodeCandidate::New(node_name) => {
                    let diagnostics_before_node = self.diagnostics.len();
                    let node = self.register_node(model, &node_name);
                    if self.diagnostics.len() != diagnostics_before_node {
                        continue;
                    }
                    let Some(node) = node else {
                        continue;
                    };
                    Some(node)
                }
            };

            let id = ZoneLocalEnvironmentId(id_value);
            let existing = model.zone_local_environment_names.insert(&name, id);
            debug_assert!(existing.is_none());
            model.zone_local_environments.push(ZoneLocalEnvironment {
                id,
                name: NormalizedName::new(&name),
                zone,
                outdoor_air_node,
            });
            if let Some(outdoor_air_node) = outdoor_air_node {
                model.zones[zone_index].linked_outdoor_air_node = Some(outdoor_air_node);
            }
        }
    }

    fn zone_local_environment_node_candidate(
        &mut self,
        model: &TypedModel,
        object_name: &str,
        object: &ep_raw_model::RawObject,
    ) -> Option<NodeCandidate> {
        let Some(node_name) =
            self.optional_string(OBJECT_TYPE, object_name, object, "outdoor_air_node_name")
        else {
            return Some(NodeCandidate::Blank);
        };

        if let Some(node_list) = model.node_list_names.resolve(&node_name) {
            let Ok(node_list_index) = usize::try_from(node_list.0) else {
                self.internal_zone_local_environment_reference_error(
                    object_name,
                    "NodeList",
                    node_list.0,
                );
                return None;
            };
            let Some(node_list_record) = model.node_lists.get(node_list_index) else {
                self.internal_zone_local_environment_reference_error(
                    object_name,
                    "NodeList",
                    node_list.0,
                );
                return None;
            };
            if node_list_record.nodes.len() != 1 {
                self.error(
                    "InvalidSingleNodeReference",
                    OBJECT_TYPE,
                    Some(object_name),
                    Some("outdoor_air_node_name"),
                    format!(
                        "ZoneProperty:LocalEnvironment/{object_name} requires one node, but NodeList '{}' has {} members",
                        node_list_record.name.0,
                        node_list_record.nodes.len()
                    ),
                );
                return None;
            }
            return Some(NodeCandidate::Existing(node_list_record.nodes[0]));
        }

        if let Some(node) = model.node_names.resolve(&node_name) {
            Some(NodeCandidate::Existing(node))
        } else {
            Some(NodeCandidate::New(node_name))
        }
    }

    fn internal_zone_local_environment_reference_error(
        &mut self,
        object_name: &str,
        target_type: &str,
        target_id: u32,
    ) {
        self.error(
            "InternalReferenceError",
            OBJECT_TYPE,
            Some(object_name),
            None,
            format!(
                "ZoneProperty:LocalEnvironment/{object_name} resolved an unavailable {target_type} id {target_id}"
            ),
        );
    }
}
