//! Input-only ordered ZoneList grammar and exact actual references.
use ep_raw_model::{FieldName, ObjectType, RawModel, RawValue};
use std::collections::BTreeSet;

pub(super) fn positive(raw: &RawModel) -> bool {
    raw.objects
        .get(&ObjectType("ZoneList".into()))
        .is_some_and(|rows| !rows.is_empty())
}

pub(super) fn validate(raw: &RawModel) -> Result<(), String> {
    let Some(lists) = raw.objects.get(&ObjectType("ZoneList".into())) else {
        return Ok(());
    };
    let zones = raw
        .objects
        .get(&ObjectType("Zone".into()))
        .ok_or("ZoneList requires actual Zone objects")?;
    for (name, row) in lists {
        if name.0.contains('\n')
            || name
                .0
                .bytes()
                .all(|b| matches!(b, b'\t' | b'\n' | 0x0c | b'\r' | b' '))
        {
            return Err("Source ZoneList name does not match the actual schema pattern".into());
        }
        let Some(RawValue::Array(members)) = row.fields.get(&FieldName("zones".into())) else {
            return Err("Source ZoneList requires its actual zones array".into());
        };
        if members.is_empty() {
            return Err("Source ZoneList requires positive actual members".into());
        }
        let mut seen = BTreeSet::new();
        for member in members {
            let RawValue::Object(fields) = member else {
                return Err("ZoneList member must be an actual object".into());
            };
            let Some(RawValue::String(reference)) = fields.get(&FieldName("zone_name".into()))
            else {
                return Err("ZoneList member requires actual zone_name string".into());
            };
            let Some(actual) = zones
                .keys()
                .find(|zone| zone.0.eq_ignore_ascii_case(reference))
            else {
                return Err(format!(
                    "ZoneList/{} references missing exact Zone '{reference}'",
                    name.0
                ));
            };
            if !seen.insert(actual.0.to_ascii_uppercase()) {
                return Err("ZoneList repeats an actual Zone member".into());
            }
        }
    }
    Ok(())
}

pub(super) fn gain_target(raw: &RawModel, reference: &str) -> bool {
    // Native Zone lookup precedes Space and ZoneList. Authored Space families
    // remain rejected by the enclosing stage; no trimmed alias is invented.
    raw.objects
        .get(&ObjectType("Zone".into()))
        .is_some_and(|rows| {
            rows.keys()
                .any(|name| name.0.eq_ignore_ascii_case(reference))
        })
        || raw
            .objects
            .get(&ObjectType("ZoneList".into()))
            .is_some_and(|rows| {
                rows.keys()
                    .any(|name| name.0.eq_ignore_ascii_case(reference))
            })
}
