//! Source A input associations only; runtime Group/ZoneList numbers stay separate.
use ep_model::TypedModel;
use ep_raw_model::{FieldName, ObjectType, RawModel, RawValue};
use std::collections::{BTreeMap, BTreeSet};

pub(super) fn validate(raw: &RawModel, model: &TypedModel, violations: &mut Vec<String>) {
    let Ok(rows) = raw.source_ordered_instances("ZoneGroup") else {
        violations.push("Source ZoneGroup declaration order is unavailable".into());
        return;
    };
    if rows.len() != model.zone_groups.len()
        || model.zone_group_names.len() != model.zone_groups.len()
    {
        violations.push("Source ZoneGroup raw/typed identity count differs".into());
        return;
    }
    let mut grouped_lists = BTreeSet::new();
    let mut assignments = BTreeMap::new();
    for (index, ((name, row), group)) in rows.iter().zip(&model.zone_groups).enumerate() {
        let target = match row.fields.get(&FieldName("zone_list_name".into())) {
            Some(RawValue::String(value)) => Some(value.as_str()),
            _ => None,
        };
        let factor = match row.fields.get(&FieldName("zone_list_multiplier".into())) {
            None => Some(1),
            Some(RawValue::Number(value)) => match ep_model::classify_source_json_number(value) {
                Ok(ep_model::SourceJsonNumber::Integer(value)) => i32::try_from(value)
                    .ok()
                    .filter(|value| *value >= 1)
                    .map(|value| value as u32),
                _ => None,
            },
            _ => None,
        };
        if usize::try_from(group.id.0).ok() != Some(index)
            || group.name.0 != name.0.to_ascii_uppercase()
            || model.zone_group_names.resolve(&name.0) != Some(group.id)
            || target.and_then(|target| model.zone_list_names.resolve(target))
                != Some(group.zone_list)
            || factor != Some(group.multiplier)
        {
            violations.push(
                "Source ZoneGroup exact ordered identity/reference/multiplier differs".into(),
            );
        }
        let Some(list) = model
            .zone_lists
            .iter()
            .find(|list| list.id == group.zone_list)
        else {
            violations.push("Source ZoneGroup actual typed ZoneList is unavailable".into());
            continue;
        };
        let raw_list = raw
            .objects
            .get(&ObjectType("ZoneList".into()))
            .and_then(|lists| {
                lists
                    .keys()
                    .find(|name| target.is_some_and(|target| name.0.eq_ignore_ascii_case(target)))
            });
        if raw_list.is_none_or(|name| name.0.to_ascii_uppercase() != list.name.0)
            || list.zones.is_empty()
            || !grouped_lists.insert(list.id)
        {
            violations
                .push("Source ZoneGroup requires a unique exact positive grouped ZoneList".into());
        }
        for &zone_id in &list.zones {
            if !model.zones.iter().any(|zone| zone.id == zone_id)
                || assignments
                    .insert(zone_id, (list.id, group.multiplier))
                    .is_some()
            {
                violations
                    .push("Source ZoneGroup assigns an unavailable or already grouped Zone".into());
            }
        }
    }
    for zone in &model.zones {
        let (list, factor) = assignments
            .get(&zone.id)
            .map_or((None, 1), |(list, factor)| (Some(*list), *factor));
        // Native Zone.ListGroup is the referenced ZoneList number, never the
        // ordinal of the Group that supplied the independent list multiplier.
        if zone.list_group != list || zone.list_multiplier != factor {
            violations.push(
                "Source ZoneGroup/list assignment differs from actual typed grouped membership"
                    .into(),
            );
        }
    }
}
