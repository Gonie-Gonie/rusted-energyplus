//! Actual GetZoneData1992-2078: original shared IP, ordered writes and errors.
use super::{HeatBalanceInputError as Error, SelectedZoneInputOwners, ZoneGetterOwners};
use crate::heat_balance::selected_input_processor::{
    SelectedInputProcessor, SourceInputProcessorShortCut, SourceInputStatisticsValue as Value,
};
use ep_model::{TypedModel, ZoneGroupId};
use std::collections::{BTreeMap, BTreeSet};

impl SelectedZoneInputOwners {
    pub(super) fn get_zone_groups(
        &mut self,
        source_is_epjson: bool,
        model: &TypedModel,
        owners: &mut ZoneGetterOwners<'_, '_>,
        errors_found: &mut bool,
    ) -> Result<(), Error> {
        owners.shortcut.current_module_object = "ZoneGroup".into();
        self.num_zone_groups = super::signed(owners.input.get_num_objects_found("ZoneGroup"))?;
        if self.num_zone_groups > 0 {
            let count = usize::try_from(self.num_zone_groups)
                .map_err(|_| Error::Unavailable("actual ZoneGroup count"))?;
            self.zone_groups = Some(vec![
                super::storage::SelectedZoneGroupData::default();
                count
            ]);
            for index in 0..count {
                get_zone_group_object_item(index, source_is_epjson, owners.input, owners.shortcut)?;
                let name = owners.shortcut.alpha_args[0].clone();
                let multiplier = super::source_int(owners.shortcut.numeric_args[0])?;
                {
                    let group = self
                        .zone_groups
                        .as_mut()
                        .and_then(|rows| rows.get_mut(index))
                        .ok_or(Error::Unavailable("actual ZoneGroup allocation"))?;
                    group.name.clone_from(&name);
                    group.multiplier = multiplier;
                }
                let lists = self.zone_lists.as_deref().unwrap_or(&[]);
                let list_num = lists
                    .iter()
                    .position(|list| {
                        list.name
                            .eq_ignore_ascii_case(&owners.shortcut.alpha_args[1])
                    })
                    .map(|n| super::signed(n + 1))
                    .transpose()?
                    .unwrap_or(0);
                let groups = self
                    .zone_groups
                    .as_mut()
                    .ok_or(Error::Unavailable("actual ZoneGroup allocation"))?;
                // Preserve Native Name -> Multiplier -> ZoneList writes even
                // when subsequent reference/duplicate/Zone diagnostics fail.
                groups[index].zone_list = list_num;
                if list_num == 0 {
                    owners.backend.show_source_severe_error(
                        &format!(
                            "GetZoneData: ZoneGroup=\"{}\":  {} named {} not found.",
                            name,
                            owners.shortcut.alpha_field_names[1],
                            owners.shortcut.alpha_args[1]
                        ),
                        &super::zone_lists::diagnostic_flags(
                            owners.globals,
                            owners.developer,
                            owners.clock,
                        )?,
                    )?;
                    *errors_found = true;
                } else {
                    for prior in groups.iter().take(index) {
                        if list_num == prior.zone_list {
                            owners.backend.show_source_severe_error(
                                &format!("GetZoneData: ZoneGroup=\"{}\":  {} already used by ZoneGroup named {}.",
                                    name, owners.shortcut.alpha_field_names[1], prior.name),
                                &super::zone_lists::diagnostic_flags(owners.globals, owners.developer, owners.clock)?,
                            )?;
                            *errors_found = true;
                        }
                    }
                    let list = &lists[usize::try_from(list_num - 1)
                        .map_err(|_| Error::Unavailable("actual grouped ZoneList pointer"))?];
                    let members = usize::try_from(list.num_of_zones)
                        .map_err(|_| Error::Unavailable("actual grouped ZoneList extent"))?;
                    // An empty list already emitted its ZoneList severe. Native
                    // still stores the Group and skips its zero-member loop;
                    // the constructor-unallocated member array is not accessed.
                    let pointers = if members == 0 {
                        &[][..]
                    } else {
                        list.zone
                            .as_deref()
                            .ok_or(Error::Unavailable("actual grouped ZoneList allocation"))?
                    };
                    let zones = self
                        .zones
                        .as_mut()
                        .ok_or(Error::Unavailable("actual grouped Zone allocation"))?;
                    for &number in pointers
                        .get(..members)
                        .ok_or(Error::Unavailable("actual grouped ZoneList member bounds"))?
                    {
                        if number > 0 {
                            let zone = usize::try_from(number - 1)
                                .ok()
                                .and_then(|at| zones.get_mut(at))
                                .ok_or(Error::Unavailable("actual grouped Zone pointer"))?;
                            if zone.list_group == 0 {
                                zone.list_multiplier = multiplier;
                                zone.list_group = list_num; // Native stores ListNum, never GroupNum.
                            } else {
                                owners.backend.show_source_severe_error(
                                    &format!("GetZoneData: ZoneGroup=\"{}\":  Zone {} in ZoneList already exists in ZoneList of another ZoneGroup.", name, zone.name),
                                    &super::zone_lists::diagnostic_flags(owners.globals, owners.developer, owners.clock)?,
                                )?;
                                let previous = zone
                                    .list_group
                                    .checked_sub(1)
                                    .and_then(|n| usize::try_from(n).ok())
                                    .and_then(|at| lists.get(at))
                                    .ok_or(Error::Unavailable(
                                        "actual previous grouped ZoneList pointer",
                                    ))?;
                                owners.backend.show_continue_error(&format!(
                                    "Previous ZoneList={}",
                                    previous.name
                                ))?;
                                *errors_found = true;
                            }
                        }
                    }
                }
            }
        }
        if !*errors_found {
            let ids = self.source_zone_group_ids(model)?;
            for (group, id) in self.zone_groups.as_mut().into_iter().flatten().zip(ids) {
                group.typed_zone_group_id = Some(id);
            }
        }
        Ok(())
    }

    /// Actual Group/List/Zone numbers are independently checked against typed IDs.
    pub(crate) fn source_zone_group_ids(
        &self,
        model: &TypedModel,
    ) -> Result<Vec<ZoneGroupId>, Error> {
        let count = usize::try_from(self.num_zone_groups)
            .map_err(|_| Error::Unavailable("actual signed ZoneGroup count"))?;
        let groups = self.zone_groups.as_deref().unwrap_or(&[]);
        let lists = self.zone_lists.as_deref().unwrap_or(&[]);
        let zones = self.zones.as_deref().unwrap_or(&[]);
        let list_ids = self.source_zone_list_ids(model)?;
        let mut typed = model
            .zone_groups
            .iter()
            .map(|g| (g.name.0.as_str(), g))
            .collect::<BTreeMap<_, _>>();
        let ids = model
            .zone_groups
            .iter()
            .map(|g| g.id)
            .collect::<BTreeSet<_>>();
        if groups.len() != count
            || model.zone_groups.len() != count
            || typed.len() != count
            || ids.len() != count
            || ids
                .iter()
                .enumerate()
                .any(|(index, id)| u32::try_from(index).ok() != Some(id.0))
            || (count == 0 && self.zone_groups.is_some())
        {
            return Err(Error::Unavailable(
                "actual/typed ZoneGroup allocation/name/ID bijection",
            ));
        }
        let mut assigned = BTreeMap::new();
        let mut grouped_lists = BTreeSet::new();
        let mut result = Vec::with_capacity(count);
        for group in groups {
            let row = typed
                .remove(group.name.as_str())
                .ok_or(Error::Unavailable("actual/typed exact ZoneGroup name"))?;
            let at = group
                .zone_list
                .checked_sub(1)
                .and_then(|n| usize::try_from(n).ok())
                .ok_or(Error::Unavailable("actual positive Group ZoneList pointer"))?;
            let list = lists
                .get(at)
                .ok_or(Error::Unavailable("actual Group ZoneList bounds"))?;
            if list_ids.get(at) != Some(&row.zone_list)
                || list.typed_zone_list_id != Some(row.zone_list)
                || group.multiplier < 1
                || u32::try_from(group.multiplier).ok() != Some(row.multiplier)
                || group.typed_zone_group_id.is_some_and(|id| id != row.id)
                || !grouped_lists.insert(group.zone_list)
            {
                return Err(Error::Unavailable(
                    "actual/typed Group list/factor/identity",
                ));
            }
            for &number in list
                .zone
                .as_deref()
                .ok_or(Error::Unavailable("actual Group list allocation"))?
            {
                if assigned
                    .insert(number, (group.zone_list, group.multiplier, row.zone_list))
                    .is_some()
                {
                    return Err(Error::Unavailable(
                        "actual Group overlapping Zone assignment",
                    ));
                }
            }
            result.push(row.id);
        }
        let typed_zones = model
            .zones
            .iter()
            .map(|z| (z.id, z))
            .collect::<BTreeMap<_, _>>();
        for (at, zone) in zones.iter().enumerate() {
            let number = super::signed(at + 1)?;
            let row = zone
                .typed_zone_id
                .and_then(|id| typed_zones.get(&id).copied())
                .ok_or(Error::Unavailable("actual Group Zone typed identity"))?;
            let expected = assigned.remove(&number);
            let (list, factor, typed_list) = expected
                .map_or((0, 1, None), |(list, factor, typed)| {
                    (list, factor, Some(typed))
                });
            if zone.list_group != list
                || zone.list_multiplier != factor
                || row.list_group != typed_list
                || u32::try_from(factor).ok() != Some(row.list_multiplier)
                || zone.multiplier < 1
                || u32::try_from(zone.multiplier).ok() != Some(row.multiplier)
                || zone.multiplier.checked_mul(factor).is_none()
            {
                return Err(Error::Unavailable(
                    "actual/typed Zone ListGroup/ListMultiplier/defined product",
                ));
            }
        }
        if !typed.is_empty() || !assigned.is_empty() || typed_zones.len() != zones.len() {
            return Err(Error::Unavailable(
                "incomplete actual/typed Group Zone bijection",
            ));
        }
        Ok(result)
    }
}

fn get_zone_group_object_item(
    index: usize,
    source_is_epjson: bool,
    input: &mut SelectedInputProcessor,
    shortcut: &mut SourceInputProcessorShortCut,
) -> Result<(), Error> {
    let names = input
        .source_raw_object_names("ZoneGroup")
        .map_err(|_| Error::Unavailable("actual original ZoneGroup order"))?;
    if names.len() != input.get_num_objects_found("ZoneGroup") {
        return Err(Error::Unavailable("actual ZoneGroup raw/IP count"));
    }
    let name = names
        .get(index)
        .cloned()
        .ok_or(Error::Unavailable("actual ZoneGroup getObjectItem index"))?;
    if !input.has_object("ZoneGroup", &name) {
        return Err(Error::Unavailable("actual ZoneGroup raw/IP identity"));
    }
    let fields = input
        .source_raw_object_fields("ZoneGroup", &name)
        .map_err(|_| Error::Unavailable("actual original ZoneGroup fields"))?;
    let reference = match fields.get("zone_list_name") {
        None => String::new(),
        Some(Value::String(value)) => value.to_ascii_uppercase(),
        _ => return Err(Error::Unavailable("actual ZoneGroup alpha representation")),
    };
    let (multiplier, blank) = match fields.get("zone_list_multiplier") {
        None => (1, true),
        Some(Value::Number(token)) => {
            // Strong primary JSON-schema integer storage, before Native int conversion.
            if token.is_empty()
                || !token.bytes().all(|b| b.is_ascii_digit())
                || token.starts_with('0')
            {
                return Err(Error::Unavailable(
                    "actual ZoneGroup genuine positive JSON integer",
                ));
            }
            let value = token
                .parse::<i32>()
                .map_err(|_| Error::Unavailable("actual ZoneGroup defined signed integer"))?;
            (value, false)
        }
        _ => {
            return Err(Error::Unavailable(
                "actual ZoneGroup integer JSON representation",
            ));
        }
    };
    if shortcut.alpha_args.len() < 2
        || shortcut.alpha_field_names.len() < 2
        || shortcut.alpha_field_blanks.len() < 2
        || shortcut.numeric_args.is_empty()
        || shortcut.numeric_field_names.is_empty()
        || shortcut.numeric_field_blanks.is_empty()
    {
        return Err(Error::Unavailable(
            "sole actual ZoneGroup shared scratch allocation",
        ));
    }
    shortcut.alpha_args.fill(String::new());
    shortcut.numeric_args.fill(0.0);
    shortcut.alpha_field_names.fill(String::new());
    shortcut.numeric_field_names.fill(String::new());
    shortcut.alpha_field_blanks.fill(true);
    shortcut.numeric_field_blanks.fill(true);
    input.mark_object_as_used("ZoneGroup", &name);
    shortcut.alpha_args[0] = name.to_ascii_uppercase();
    shortcut.alpha_field_blanks[0] = name.is_empty();
    shortcut.alpha_args[1] = reference;
    shortcut.alpha_field_blanks[1] = shortcut.alpha_args[1].is_empty();
    shortcut.alpha_field_names[0] = if source_is_epjson { "name" } else { "Name" }.into();
    shortcut.alpha_field_names[1] = if source_is_epjson {
        "zone_list_name"
    } else {
        "Zone List Name"
    }
    .into();
    shortcut.numeric_args[0] = f64::from(multiplier);
    shortcut.numeric_field_blanks[0] = blank;
    shortcut.numeric_field_names[0] = if source_is_epjson {
        "zone_list_multiplier"
    } else {
        "Zone List Multiplier"
    }
    .into();
    Ok(())
}
