//! Source A singleton authored Space grammar and raw identity custody only.
//! No geometry, load, floor-area, getter state or numerical outputs are produced here.
use super::{field, number};
use ep_raw_model::{FieldName, ObjectType, RawModel, RawValue};
use std::collections::{BTreeMap, BTreeSet};

pub(super) fn positive(raw: &RawModel) -> bool {
    raw.objects
        .get(&ObjectType("Space".into()))
        .is_some_and(|rows| !rows.is_empty())
}
pub(super) fn known_field(name: &str) -> bool {
    matches!(
        name,
        "zone_name" | "ceiling_height" | "volume" | "floor_area" | "space_type" | "tags"
    )
}
pub(super) fn admitted_field(name: &str, value: &RawValue) -> bool {
    match name {
        "zone_name" | "space_type" => matches!(value, RawValue::String(_)),
        "ceiling_height" | "volume" | "floor_area" => {
            number(value).is_some()
                || matches!(value,RawValue::String(s) if s.is_empty()||s.eq_ignore_ascii_case("Autocalculate"))
        }
        "tags" => matches!(value,RawValue::Array(rows) if rows.iter().all(|row|
            matches!(row,RawValue::Object(fields) if fields.get(&FieldName("tag".into())).is_none_or(|v|matches!(v,RawValue::String(_)))))),
        _ => false,
    }
}
pub(super) fn validate(raw: &RawModel) -> Result<(), String> {
    let Some(spaces) = raw.objects.get(&ObjectType("Space".into())) else {
        return Ok(());
    };
    let zones = raw
        .objects
        .get(&ObjectType("Zone".into()))
        .ok_or("Space requires actual Zone objects")?;
    let mut owners = BTreeMap::new();
    let mut names = BTreeSet::new();
    for (name, row) in spaces {
        if name.0.contains('\n')
            || name
                .0
                .bytes()
                .all(|b| matches!(b, b'\t' | b'\n' | 0x0c | b'\r' | b' '))
            || !names.insert(name.0.to_ascii_uppercase())
        {
            return Err("Source Space requires a unique actual schema name".into());
        }
        let Some(RawValue::String(reference)) = field(row, "zone_name") else {
            return Err("Space requires actual zone_name string".into());
        };
        let zone = zones
            .keys()
            .find(|z| z.0.eq_ignore_ascii_case(reference))
            .ok_or("Space references missing exact Source Zone")?;
        if owners
            .insert(zone.0.to_ascii_uppercase(), name.0.to_ascii_uppercase())
            .is_some()
        {
            return Err("Source A requires exactly one whole-Zone Space per Zone".into());
        }
        for (name, value) in &row.fields {
            if known_field(&name.0) && !admitted_field(&name.0, value) {
                return Err(format!("Source Space known field invalid: {}", name.0));
            }
        }
    }
    // Native CreateMissingSpaces adds a remainder only for mixed assigned/unassigned
    // surfaces. Uniform blank and uniform sole-Space assignments keep one owner.
    let mut explicit = BTreeSet::new();
    let mut blank = BTreeSet::new();
    if let Some(surfaces) = raw
        .objects
        .get(&ObjectType("BuildingSurface:Detailed".into()))
    {
        for row in surfaces.values() {
            let Some(RawValue::String(zone)) = field(row, "zone_name") else {
                return Err("Surface requires actual Zone string".into());
            };
            let zone = zones
                .keys()
                .find(|z| z.0.eq_ignore_ascii_case(zone))
                .ok_or("Surface references missing actual Zone")?
                .0
                .to_ascii_uppercase();
            match field(row, "space_name") {
                None => {
                    blank.insert(zone);
                }
                Some(RawValue::String(value)) if value.is_empty() => {
                    blank.insert(zone);
                }
                Some(RawValue::String(value)) => {
                    let actual = owners.get(&zone).unwrap_or(&zone);
                    if !actual.eq_ignore_ascii_case(value) {
                        return Err(
                            "Surface requires its actual sole Space in the same Zone".into()
                        );
                    }
                    explicit.insert(zone);
                }
                Some(_) => return Err("Surface Space reference requires actual string".into()),
            }
        }
    }
    if explicit.intersection(&blank).next().is_some() {
        return Err("Source A excludes positive remainder-Space construction".into());
    }
    Ok(())
}
