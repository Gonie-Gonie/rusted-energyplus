//! Genuine GetSpaceData2422-2567: lexical authored rows, then Zone defaults.
//! Allocation capacity is not active numSpaces and must remain retained.
use super::{
    HeatBalanceInputError as Error, SelectedZoneInputOwners, ZoneGetterOwners, signed, storage,
};
use crate::heat_balance::selected_input_processor::{
    SourceInputFamily, SourceInputStatisticsFields, SourceInputStatisticsValue as Value,
};
use ep_model::{AutoOrNumber, SpaceOrigin, TypedModel};

fn alpha(fields: &SourceInputStatisticsFields, name: &str, default: &str) -> Result<String, Error> {
    // Native getAlphaFieldValue621 uses json.empty(): a present string,
    // including "", is nonempty JSON and is preserved (ASCII-fold only).
    match fields.get(name) {
        None => Ok(default.to_ascii_uppercase()),
        Some(Value::String(value)) => Ok(value.to_ascii_uppercase()),
        _ => Err(Error::Unavailable("actual Space alpha JSON kind")),
    }
}
fn real(fields: &SourceInputStatisticsFields, name: &str) -> Result<f64, Error> {
    // All three official Space real defaults are Autocalculate. Native651
    // converts JSON unsigned integers to signed i64 before Real64.
    match fields.get(name) {
        None | Some(Value::String(_)) => Ok(-99999.0),
        Some(Value::Number(token)) => ep_model::classify_source_json_number(token)
            .map(|number| number.as_f64())
            .map_err(|_| Error::Unavailable("actual Space real JSON number")),
        _ => Err(Error::Unavailable("actual Space real JSON kind")),
    }
}
fn selector(value: AutoOrNumber) -> f64 {
    match value {
        AutoOrNumber::AutoCalculate => -99999.0,
        AutoOrNumber::Value(value) => value,
    }
}
fn type_number(types: &mut [String], active: &mut i32, name: &str) -> Result<i32, Error> {
    let count = usize::try_from(*active).map_err(|_| Error::Unavailable("actual numSpaceTypes"))?;
    let rows = types
        .get(..count)
        .ok_or(Error::Unavailable("actual spaceTypes active extent"))?;
    if let Some(index) = rows
        .iter()
        .position(|value| value.eq_ignore_ascii_case(name))
    {
        return signed(index + 1);
    }
    *active = active
        .checked_add(1)
        .ok_or(Error::Unavailable("native signed Space type overflow"))?;
    *types
        .get_mut(count)
        .ok_or(Error::Unavailable("spaceTypes bounds"))? = name.into();
    Ok(*active)
}

impl SelectedZoneInputOwners {
    pub(super) fn get_space_data(
        &mut self,
        model: &TypedModel,
        owners: &mut ZoneGetterOwners<'_, '_>,
        errors_found: &mut bool,
    ) -> Result<(), Error> {
        // Literal find, no count query, null insertion, shortcut reset or IDF
        // ordinal conversion. Original traverses the epJSON ordered map here.
        let family = owners.input.find_exact_family("Space").cloned();
        let mut names = match &family {
            Some(SourceInputFamily::Objects(rows)) => rows
                .iter()
                .map(|row| row.object_name.clone())
                .collect::<Vec<_>>(),
            Some(SourceInputFamily::Null) | None => Vec::new(),
        };
        names.sort();
        let zone_count = usize::try_from(owners.globals.num_of_zones)
            .map_err(|_| Error::Unavailable("negative actual NumOfZones"))?;
        let capacity = if family.is_some() {
            names
                .len()
                .checked_add(zone_count)
                .ok_or(Error::Unavailable("actual Space allocation overflow"))?
        } else {
            zone_count
        };
        self.spaces = Some(vec![storage::SelectedSpaceData::default(); capacity]);
        self.space_types = Some(vec![
            String::new();
            if family.is_some() { names.len() + 1 } else { 1 }
        ]);
        let zones = self
            .zones
            .as_mut()
            .ok_or(Error::Unavailable("Space before Zone allocation"))?;
        let spaces = self
            .spaces
            .as_mut()
            .ok_or(Error::Unavailable("actual Space allocation"))?;
        let types = self
            .space_types
            .as_mut()
            .ok_or(Error::Unavailable("actual spaceTypes allocation"))?;
        let mut space_num = 0_i32;
        for name in names {
            space_num = space_num
                .checked_add(1)
                .ok_or(Error::Unavailable("native signed Space overflow"))?;
            let fields = owners
                .input
                .source_raw_object_fields("Space", &name)
                .map_err(|_| Error::Unavailable("actual retained Space JSON fields"))?
                .clone();
            let space = spaces
                .get_mut(
                    usize::try_from(space_num - 1)
                        .map_err(|_| Error::Unavailable("actual Space index"))?,
                )
                .ok_or(Error::Unavailable("actual Space allocation bounds"))?;
            space.name = name.to_ascii_uppercase();
            owners.input.mark_object_as_used("Space", &name);
            let zone_name = alpha(&fields, "zone_name", "")?;
            space.ceiling_height = real(&fields, "ceiling_height")?;
            space.volume = real(&fields, "volume")?;
            space.user_entered_floor_area = real(&fields, "floor_area")?;
            if let Some(index) = zones
                .iter()
                .position(|zone| zone.name.eq_ignore_ascii_case(&zone_name))
            {
                space.zone_num = signed(index + 1)?;
                zones[index].space_indexes.push(space_num);
                zones[index].num_spaces += 1;
            } else {
                let flags = super::zone_lists::diagnostic_flags(
                    owners.globals,
                    owners.developer,
                    owners.clock,
                )?;
                owners.backend.show_source_severe_error(
                    &format!("GetSpaceData: Space={}", space.name),
                    &flags,
                )?;
                owners
                    .backend
                    .show_continue_error(&format!("Zone Name ={zone_name}not found."))?;
                *errors_found = true;
            }
            space.space_type = alpha(&fields, "space_type", "General")?;
            space.space_type_num = type_number(
                types,
                &mut owners.globals.num_space_types,
                &space.space_type,
            )?;
            if let Some(tags) = fields.get("tags") {
                let Value::Array(tags) = tags else {
                    return Err(Error::Unavailable("actual Space tags array"));
                };
                for tag in tags {
                    let Value::Object(tag) = tag else {
                        return Err(Error::Unavailable("actual Space tag object"));
                    };
                    space.tags.push(alpha(tag, "tag", "")?);
                }
            }
            // This association observes typed identity only after real Native
            // writes. It never substitutes typed container order for SpaceNum.
            if space.zone_num > 0 {
                let zone = &zones[usize::try_from(space.zone_num - 1)
                    .map_err(|_| Error::Unavailable("Space actual Zone index"))?];
                let matching = model
                    .spaces
                    .iter()
                    .filter(|typed| {
                        typed.origin == SpaceOrigin::Authored && typed.name.0 == space.name
                    })
                    .collect::<Vec<_>>();
                if matching.len() != 1
                    || Some(matching[0].zone) != zone.typed_zone_id
                    || selector(matching[0].ceiling_height).to_bits()
                        != space.ceiling_height.to_bits()
                    || selector(matching[0].volume).to_bits() != space.volume.to_bits()
                    || selector(matching[0].floor_area).to_bits()
                        != space.user_entered_floor_area.to_bits()
                    || matching[0].space_type.0 != space.space_type
                    || matching[0]
                        .tags
                        .iter()
                        .map(|tag| tag.0.as_str())
                        .ne(space.tags.iter().map(String::as_str))
                {
                    return Err(Error::Unavailable(
                        "actual authored Space/typed association",
                    ));
                }
                space.typed_space_id = Some(matching[0].id);
            }
        }
        if family.is_some() {
            owners.globals.num_spaces = space_num;
        }
        // Positive SpaceList retains its separate unavailable branch, after
        // the real preceding Space allocation and field/usage writes.
        if let Some(family) = owners.input.find_exact_family("SpaceList") {
            if matches!(family,SourceInputFamily::Objects(rows) if !rows.is_empty()) {
                return Err(Error::Unavailable("positive SpaceList"));
            }
            self.space_lists = Some(Vec::new());
        }
        for (zone_index, zone) in zones.iter_mut().enumerate() {
            if !zone.space_indexes.is_empty() {
                continue;
            }
            owners.globals.num_spaces = owners
                .globals
                .num_spaces
                .checked_add(1)
                .ok_or(Error::Unavailable("native signed Space overflow"))?;
            let at = usize::try_from(owners.globals.num_spaces - 1)
                .map_err(|_| Error::Unavailable("Space index"))?;
            let space = spaces
                .get_mut(at)
                .ok_or(Error::Unavailable("native Space allocation bounds"))?;
            space.zone_num = signed(zone_index + 1)?;
            space.name.clone_from(&zone.name);
            space.space_type = "GENERAL".into();
            space.space_type_num =
                type_number(types, &mut owners.globals.num_space_types, "GENERAL")?;
            zone.space_indexes.push(owners.globals.num_spaces);
            zone.num_spaces += 1;
            let matching = model
                .spaces
                .iter()
                .filter(|typed| Some(typed.zone) == zone.typed_zone_id)
                .collect::<Vec<_>>();
            if matching.len() != 1
                || matching[0].name != ep_model::NormalizedName::new(&space.name)
                || matching[0].origin != SpaceOrigin::AutoZoneDefault
            {
                return Err(Error::Unavailable(
                    "actual implicit Space/typed association",
                ));
            }
            space.typed_space_id = Some(matching[0].id);
        }
        Ok(())
    }
}
