//! Source A ZoneGroup grammar over real positive ZoneLists, without runtime state.
use super::field;
use ep_raw_model::{FieldName, ObjectType, RawModel, RawObject, RawValue};
use std::collections::BTreeSet;

pub(super) fn positive(raw: &RawModel) -> bool {
    raw.objects
        .get(&ObjectType("ZoneGroup".into()))
        .is_some_and(|rows| !rows.is_empty())
}

// The schema is integer/minimum1/default1 without a maximum. This bounded
// Source lane additionally preserves defined conversion to the Native int.
pub(super) fn multiplier_value(value: &RawValue) -> Option<u32> {
    let RawValue::Number(text) = value else {
        return None;
    };
    let ep_model::SourceJsonNumber::Integer(value) =
        ep_model::classify_source_json_number(text).ok()?
    else {
        return None;
    };
    i32::try_from(value)
        .ok()
        .filter(|value| *value >= 1)
        .map(|value| value as u32)
}

fn multiplier(row: &RawObject) -> Option<u32> {
    field(row, "zone_list_multiplier").map_or(Some(1), multiplier_value)
}

pub(super) fn validate(raw: &RawModel) -> Result<(), String> {
    let lists = raw
        .objects
        .get(&ObjectType("ZoneList".into()))
        .ok_or("Source ZoneGroup requires actual positive ZoneLists")?;
    let zones = raw
        .objects
        .get(&ObjectType("Zone".into()))
        .ok_or("Source ZoneGroup requires actual Zones")?;
    if zones.values().any(|zone| {
        field(zone, "multiplier").is_some_and(|value| multiplier_value(value).is_none())
    }) {
        return Err(
            "Source grouped input requires genuine positive Native integer Zone multipliers".into(),
        );
    }
    let mut names = BTreeSet::new();
    let mut grouped_lists = BTreeSet::new();
    let mut grouped_zones = BTreeSet::new();
    // getObjectItem declaration order is retained, including staged IDF.
    for (name, row) in raw.source_ordered_instances("ZoneGroup")? {
        if name.0.contains('\n')
            || name
                .0
                .bytes()
                .all(|byte| matches!(byte, b'\t' | b'\n' | 0x0c | b'\r' | b' '))
            || !names.insert(name.0.to_ascii_uppercase())
        {
            return Err("Source ZoneGroup requires a unique exact schema name".into());
        }
        let Some(RawValue::String(reference)) = field(row, "zone_list_name") else {
            return Err("Source ZoneGroup requires its actual zone_list_name string".into());
        };
        let (list_name, list) = lists
            .iter()
            .find(|(name, _)| name.0.eq_ignore_ascii_case(reference))
            .ok_or("ZoneGroup references missing exact Source ZoneList")?;
        let factor = multiplier(row).ok_or(
            "ZoneGroup multiplier must be a genuine integer within the positive Native int domain",
        )?;
        if !grouped_lists.insert(list_name.0.to_ascii_uppercase()) {
            return Err("Source ZoneGroup repeats an already grouped ZoneList".into());
        }
        let Some(RawValue::Array(members)) = field(list, "zones") else {
            return Err("Source ZoneGroup requires its actual positive ZoneList members".into());
        };
        if members.is_empty() {
            return Err("Source ZoneGroup requires a positive ZoneList".into());
        }
        for member in members {
            let RawValue::Object(fields) = member else {
                return Err("ZoneGroup actual ZoneList member must be an object".into());
            };
            let Some(RawValue::String(reference)) = fields.get(&FieldName("zone_name".into()))
            else {
                return Err("ZoneGroup actual ZoneList member requires zone_name".into());
            };
            let (zone_name, zone) = zones
                .iter()
                .find(|(name, _)| name.0.eq_ignore_ascii_case(reference))
                .ok_or("ZoneGroup actual ZoneList references missing exact Zone")?;
            if !grouped_zones.insert(zone_name.0.to_ascii_uppercase()) {
                return Err(
                    "Source ZoneGroup assigns a Zone through multiple grouped ZoneLists".into(),
                );
            }
            let zone_factor = field(zone, "multiplier")
                .map_or(Some(1), multiplier_value)
                .ok_or("Zone multiplier must fit the positive Native int domain")?;
            if zone_factor
                .checked_mul(factor)
                .is_none_or(|product| product > i32::MAX as u32)
            {
                return Err(
                    "Source Zone/group multiplier product must fit a positive Native int".into(),
                );
            }
        }
    }
    Ok(())
}
