//! Source A authored SpaceList grammar; no sizing or runtime state.
use super::{field, number};
use ep_raw_model::{ObjectType, RawModel, RawObject, RawValue};
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
    let spaces = raw.objects.get(&ObjectType("Space".into()));
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
            .chain(spaces.into_iter().flat_map(|rows| rows.keys()))
            .any(|other| other.0.eq_ignore_ascii_case(&name.0))
        {
            return Err("SpaceList name collides with an actual Zone or authored Space".into());
        }
        let members = match field(row, "spaces") {
            None => continue,
            Some(RawValue::Array(members)) => members,
            Some(_) => return Err("Source SpaceList spaces must be an actual array".into()),
        };
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
                .ok_or("SpaceList member requires actual authored Space objects")?
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

// Exact Native Zone -> Space -> ZoneList -> SpaceList precedence. Only a real
// empty list can erase a gain's member loop; a missing target never does.
pub(super) fn empty_gain_target(raw: &RawModel, object: &RawObject) -> bool {
    let Some(RawValue::String(reference)) =
        field(object, "zone_or_zonelist_or_space_or_spacelist_name")
    else {
        return false;
    };
    for family in ["Zone", "Space", "ZoneList", "SpaceList"] {
        if let Some((_, row)) = raw
            .objects
            .get(&ObjectType(family.into()))
            .and_then(|rows| {
                rows.iter()
                    .find(|(name, _)| name.0.eq_ignore_ascii_case(reference))
            })
        {
            return family == "SpaceList"
                && match field(row, "spaces") {
                    None => true,
                    Some(RawValue::Array(members)) => members.is_empty(),
                    _ => false,
                };
        }
    }
    false
}

pub(super) fn family_has_instances(raw: &RawModel, family: &str) -> bool {
    raw.objects
        .get(&ObjectType(family.into()))
        .is_some_and(|rows| rows.values().any(|row| !empty_gain_target(raw, row)))
}

// InputProcessor validates the schema before any IHG family-total gate.
// Required strings have no minLength: a present empty schedule is structural
// input, while lookup is conditional on whether the whole family has members.
pub(super) fn validate_empty_gain_schema(family: &str, row: &RawObject) -> Result<(), String> {
    let required: &[&str] = if family == "People" {
        &[
            "zone_or_zonelist_or_space_or_spacelist_name",
            "number_of_people_schedule_name",
            "activity_level_schedule_name",
        ]
    } else {
        &[
            "zone_or_zonelist_or_space_or_spacelist_name",
            "schedule_name",
        ]
    };
    if required
        .iter()
        .any(|name| !matches!(field(row, name), Some(RawValue::String(_))))
    {
        return Err(format!(
            "empty-target {family} requires every actual schema string"
        ));
    }
    for (name, value) in &row.fields {
        let valid = if family == "People" {
            !super::people::known_field(&name.0) || super::people::admitted_field(&name.0, value)
        } else if family == "ElectricEquipment" {
            super::electric::admitted_field(&name.0, value)
        } else {
            match name.0.as_str() {
                "design_level" | "power_per_floor_area" | "power_per_person" => {
                    number(value).is_some()
                }
                "fraction_latent" | "fraction_radiant" | "fraction_lost" => {
                    number(value).is_some_and(|n| (0.0..=1.0).contains(&n))
                }
                "carbon_dioxide_generation_rate" => {
                    number(value).is_some_and(|n| (0.0..=4e-7).contains(&n))
                }
                "zone_or_zonelist_or_space_or_spacelist_name"
                | "schedule_name"
                | "end_use_subcategory" => {
                    matches!(value, RawValue::String(_))
                }
                "fuel_type" | "design_level_calculation_method" => {
                    matches!(value, RawValue::String(_))
                        && super::admitted_field(family, &name.0, value, true)
                }
                _ => true,
            }
        };
        if !valid {
            return Err(format!(
                "empty-target {family}.{} violates the actual schema",
                name.0
            ));
        }
    }
    Ok(())
}
