//! Capability registry loading and object-rule matching for arbitrary runs.

use serde::Deserialize;

/// Capability registry path bundled with the repository and release package.
pub const CAPABILITY_REGISTRY_PATH: &str = "specs/capabilities.toml";
const CAPABILITY_REGISTRY_TOML: &str = include_str!("../../../specs/capabilities.toml");

#[derive(Debug, Default, Deserialize)]
pub(crate) struct CapabilityRegistrySpec {
    #[serde(default)]
    pub(crate) capability: Vec<CapabilitySpec>,
    #[serde(default)]
    pub(crate) unsupported_rule: Vec<SupportRuleSpec>,
    #[serde(default)]
    pub(crate) partial_rule: Vec<SupportRuleSpec>,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct CapabilitySpec {
    pub(crate) id: String,
    pub(crate) domain: String,
    pub(crate) support_level: String,
    pub(crate) run_state: String,
    #[serde(default)]
    pub(crate) required_objects: Vec<String>,
    #[serde(default)]
    pub(crate) forbidden_active_features: Vec<String>,
    #[serde(default)]
    pub(crate) algorithms: Vec<String>,
    #[serde(default)]
    pub(crate) evidence_cases: Vec<String>,
    #[serde(default)]
    pub(crate) claim_boundary: String,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct SupportRuleSpec {
    pub(crate) id: String,
    #[serde(default)]
    pub(crate) object_patterns: Vec<String>,
    #[serde(default)]
    pub(crate) except_object_patterns: Vec<String>,
    #[serde(default)]
    pub(crate) reason: String,
}

#[derive(Debug)]
pub(crate) struct LoadedCapabilityRegistry {
    pub(crate) spec: CapabilityRegistrySpec,
    pub(crate) loaded: bool,
    pub(crate) error: Option<String>,
}

pub(crate) fn load_embedded_capability_registry() -> LoadedCapabilityRegistry {
    match toml::from_str::<CapabilityRegistrySpec>(CAPABILITY_REGISTRY_TOML) {
        Ok(spec) => LoadedCapabilityRegistry {
            spec,
            loaded: true,
            error: None,
        },
        Err(error) => LoadedCapabilityRegistry {
            spec: CapabilityRegistrySpec::default(),
            loaded: false,
            error: Some(error.to_string()),
        },
    }
}

pub(crate) fn registry_capability<'a>(
    registry: &'a CapabilityRegistrySpec,
    id: &str,
) -> Option<&'a CapabilitySpec> {
    registry
        .capability
        .iter()
        .find(|capability| capability.id == id)
}

pub(crate) fn registry_capability_ids_and_missing(
    registry: &CapabilityRegistrySpec,
    ids: Vec<String>,
) -> (Vec<String>, Vec<String>) {
    ids.into_iter()
        .partition(|id| registry_capability(registry, id).is_some())
}

pub(crate) fn partial_rule_for_object<'a>(
    registry: &'a CapabilityRegistrySpec,
    object_type: &str,
) -> Option<&'a SupportRuleSpec> {
    support_rule_for_object(&registry.partial_rule, object_type)
}

pub(crate) fn unsupported_rule_for_object<'a>(
    registry: &'a CapabilityRegistrySpec,
    object_type: &str,
) -> Option<&'a SupportRuleSpec> {
    support_rule_for_object(&registry.unsupported_rule, object_type)
}

fn support_rule_for_object<'a>(
    rules: &'a [SupportRuleSpec],
    object_type: &str,
) -> Option<&'a SupportRuleSpec> {
    rules.iter().find(|rule| {
        rule.object_patterns
            .iter()
            .any(|pattern| object_pattern_matches(pattern, object_type))
            && !rule
                .except_object_patterns
                .iter()
                .any(|pattern| object_pattern_matches(pattern, object_type))
    })
}

fn object_pattern_matches(pattern: &str, object_type: &str) -> bool {
    if pattern.contains('*') {
        return wildcard_match(pattern, object_type);
    }
    pattern == object_type
}

fn wildcard_match(pattern: &str, value: &str) -> bool {
    if pattern == "*" {
        return true;
    }

    let parts = pattern
        .split('*')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>();
    if parts.is_empty() {
        return true;
    }

    let mut search_start = 0;
    for (index, part) in parts.iter().enumerate() {
        let Some(position) = value[search_start..].find(part) else {
            return false;
        };
        if index == 0 && !pattern.starts_with('*') && position != 0 {
            return false;
        }
        search_start += position + part.len();
    }

    pattern.ends_with('*')
        || parts
            .last()
            .is_some_and(|last_part| value.ends_with(last_part))
}
