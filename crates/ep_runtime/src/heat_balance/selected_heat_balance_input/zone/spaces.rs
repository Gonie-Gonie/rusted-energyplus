//! Genuine GetSpaceData2422-2567: epJSON-map authored rows, then Zone defaults.
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
        // ordinal conversion. These family rows retain the epJSON map's pinned
        // doj::alphanum_less order, independently of the IDF declaration overlay.
        let family = owners.input.find_exact_family("Space").cloned();
        let names = match &family {
            Some(SourceInputFamily::Objects(rows)) => rows
                .iter()
                .map(|row| row.object_name.clone())
                .collect::<Vec<_>>(),
            Some(SourceInputFamily::Null) | None => Vec::new(),
        };
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
        // Literal GetSpaceData2489-2550. Resolve members before the following
        // implicit defaults exist, and retain allocation/append/diagnostic order.
        if let Some(family) = owners.input.find_exact_family("SpaceList").cloned() {
            let names = match family {
                SourceInputFamily::Objects(rows) => rows
                    .into_iter()
                    .map(|row| row.object_name)
                    .collect::<Vec<_>>(),
                SourceInputFamily::Null => Vec::new(),
            };
            self.space_lists = Some(vec![storage::SelectedSpaceListData::default(); names.len()]);
            for (at, name) in names.into_iter().enumerate() {
                let fields = owners
                    .input
                    .source_raw_object_fields("SpaceList", &name)
                    .map_err(|_| Error::Unavailable("actual retained SpaceList JSON fields"))?
                    .clone();
                let list = &mut self
                    .space_lists
                    .as_mut()
                    .ok_or(Error::Unavailable("actual SpaceList allocation"))?[at];
                list.name = name.to_ascii_uppercase();
                owners.input.mark_object_as_used("SpaceList", &name);
                for (collision, label) in [
                    (
                        zones
                            .iter()
                            .any(|zone| zone.name.eq_ignore_ascii_case(&list.name)),
                        "zone",
                    ),
                    (
                        spaces
                            .iter()
                            .any(|space| space.name.eq_ignore_ascii_case(&list.name)),
                        "space",
                    ),
                ] {
                    if collision {
                        owners.backend.show_source_severe_error(
                            &format!("GetSpaceData: SpaceList=\"{}\":  is a duplicate of a {label} name.", list.name),
                            &super::zone_lists::diagnostic_flags(owners.globals, owners.developer, owners.clock)?,
                        )?;
                        *errors_found = true;
                    }
                }
                list.num_list_spaces = 0;
                if let Some(members) = fields.get("spaces") {
                    let Value::Array(members) = members else {
                        return Err(Error::Unavailable("actual SpaceList spaces array"));
                    };
                    for member in members {
                        let Value::Object(member) = member else {
                            return Err(Error::Unavailable("actual SpaceList member object"));
                        };
                        let member_name = alpha(member, "space_name", "")?;
                        let number = spaces
                            .iter()
                            .position(|space| space.name.eq_ignore_ascii_case(&member_name))
                            .map(|index| signed(index + 1))
                            .transpose()?
                            .unwrap_or(0);
                        if number > 0 {
                            list.spaces.push(number);
                            list.num_list_spaces =
                                list.num_list_spaces
                                    .checked_add(1)
                                    .ok_or(Error::Unavailable(
                                        "native signed SpaceList member count",
                                    ))?;
                        } else {
                            owners.backend.show_source_severe_error(
                                &format!("GetSpaceData: SpaceList={}", list.name),
                                &super::zone_lists::diagnostic_flags(
                                    owners.globals,
                                    owners.developer,
                                    owners.clock,
                                )?,
                            )?;
                            owners.backend.show_continue_error(&format!(
                                "Space Name={member_name} not found."
                            ))?;
                            *errors_found = true;
                        }
                        list.max_space_name_length =
                            list.max_space_name_length.max(member_name.len());
                        // Native compares every preceding retained successful member,
                        // after append/count and max-length writes, with no early break.
                        for &prior in list.spaces.iter().take(list.spaces.len().saturating_sub(1)) {
                            if number == prior {
                                owners.backend.show_source_severe_error(
                                    &format!("GetSpaceData: SpaceList=\"{}\":  Space Name {member_name} appears more than once in list.", list.name),
                                    &super::zone_lists::diagnostic_flags(owners.globals, owners.developer, owners.clock)?,
                                )?;
                                *errors_found = true;
                            }
                        }
                    }
                }
            }
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
        // Rust identity proof follows all genuine getter writes; it never
        // substitutes typed members for the actual pre-default resolution.
        if !*errors_found {
            let ids = self.source_space_list_ids(model)?;
            if let Some(lists) = self.space_lists.as_mut() {
                for (list, id) in lists.iter_mut().zip(ids) {
                    list.typed_space_list_id = Some(id);
                }
            }
        }
        Ok(())
    }
}
