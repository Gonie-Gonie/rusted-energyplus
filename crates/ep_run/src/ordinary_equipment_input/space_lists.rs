//! Source A nonempty authored SpaceList grammar; no sizing or runtime state.
use super::field;
use ep_raw_model::{ObjectType, RawModel, RawValue};
use std::collections::BTreeSet;

pub(super) fn positive(raw: &RawModel) -> bool {
    raw.objects
        .get(&ObjectType("SpaceList".into()))
        .is_some_and(|rows| !rows.is_empty())
}

pub(super) fn validate(raw: &RawModel) -> Result<(), String> {
    let Some(lists) = raw.objects.get(&ObjectType("SpaceList".into())) else {
        return Ok(());
    };
    let spaces = raw
        .objects
        .get(&ObjectType("Space".into()))
        .ok_or("SpaceList requires actual authored Space objects")?;
    let zones = raw
        .objects
        .get(&ObjectType("Zone".into()))
        .ok_or("SpaceList requires actual Zone objects")?;
    let mut list_names = BTreeSet::new();
    for (name, row) in lists {
        if name.0.contains('\n')
            || name
                .0
                .bytes()
                .all(|b| matches!(b, b'\t' | b'\n' | 0x0c | b'\r' | b' '))
            || !list_names.insert(name.0.to_ascii_uppercase())
        {
            return Err("Source SpaceList requires a unique actual schema name".into());
        }
        if zones
            .keys()
            .chain(spaces.keys())
            .any(|other| other.0.eq_ignore_ascii_case(&name.0))
        {
            return Err("SpaceList name collides with an actual Zone or authored Space".into());
        }
        let Some(RawValue::Array(members)) = field(row, "spaces") else {
            return Err("Source SpaceList requires actual spaces array".into());
        };
        // Native schema permits empty lists; their gain-target lifecycle has
        // separate pending coverage and is not claimed by this positive increment.
        if members.is_empty() {
            return Err("Source SpaceList requires positive actual members".into());
        }
        let mut seen = BTreeSet::new();
        for member in members {
            let RawValue::Object(fields) = member else {
                return Err("SpaceList member must be an actual object".into());
            };
            let Some(RawValue::String(reference)) =
                fields.get(&ep_raw_model::FieldName("space_name".into()))
            else {
                return Err("SpaceList member requires actual space_name string".into());
            };
            let actual = spaces
                .keys()
                .find(|space| space.0.eq_ignore_ascii_case(reference))
                .ok_or("SpaceList references missing exact authored Space")?;
            if !seen.insert(actual.0.to_ascii_uppercase()) {
                return Err("SpaceList repeats an actual Space member".into());
            }
        }
    }
    Ok(())
}
