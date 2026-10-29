//! Source GetZoneData1913-1988 and ZoneList's real shared getObjectItem.
//! Raw extensible members and physical IDF metadata stay on the original IP.
use super::{HeatBalanceInputError as Error, SelectedZoneInputOwners, ZoneGetterOwners};
use crate::heat_balance::error_backend::SourceDiagnosticFlags;
use crate::heat_balance::selected_input_processor::{
    SelectedInputProcessor, SourceInputProcessorShortCut, SourceInputStatisticsFields,
    SourceInputStatisticsValue as Value,
};
use ep_model::{TypedModel, ZoneListId};
use std::collections::BTreeMap;

impl SelectedZoneInputOwners {
    pub(super) fn get_zone_lists(
        &mut self,
        source_is_epjson: bool,
        model: &TypedModel,
        owners: &mut ZoneGetterOwners<'_, '_>,
        errors_found: &mut bool,
    ) -> Result<(), Error> {
        owners.shortcut.current_module_object = "ZoneList".into();
        self.num_zone_lists = super::signed(owners.input.get_num_objects_found("ZoneList"))?;
        if self.num_zone_lists == 0 {
            return Ok(());
        }
        let count = usize::try_from(self.num_zone_lists)
            .map_err(|_| Error::Unavailable("negative actual ZoneList count"))?;
        self.zone_lists = Some(vec![super::storage::SelectedZoneListData::default(); count]);
        for index in 0..count {
            let num_alphas =
                get_zone_list_object_item(index, source_is_epjson, owners.input, owners.shortcut)?;
            let zones = self
                .zones
                .as_deref()
                .ok_or(Error::Unavailable("ZoneList before actual Zone allocation"))?;
            let list = self
                .zone_lists
                .as_mut()
                .and_then(|rows| rows.get_mut(index))
                .ok_or(Error::Unavailable("actual ZoneList allocation"))?;
            list.name.clone_from(&owners.shortcut.alpha_args[0]);
            if zones
                .iter()
                .any(|zone| zone.name.eq_ignore_ascii_case(&list.name))
            {
                owners.backend.show_source_warning_error(
                    &format!(
                        "GetZoneData: ZoneList=\"{}\":  is a duplicate of a zone name.",
                        list.name
                    ),
                    &diagnostic_flags(owners.globals, owners.developer, owners.clock)?,
                )?;
                owners.backend.show_continue_error(
                    "This could be a problem in places where either a Zone Name or a Zone List can be used.",
                )?;
            }
            list.num_of_zones = super::signed(num_alphas)?
                .checked_sub(1)
                .ok_or(Error::Unavailable("native signed ZoneList member count"))?;
            if list.num_of_zones < 1 {
                owners.backend.show_source_severe_error(
                    &format!(
                        "GetZoneData: ZoneList=\"{}\":  No zones specified.",
                        list.name
                    ),
                    &diagnostic_flags(owners.globals, owners.developer, owners.clock)?,
                )?;
                *errors_found = true;
            } else {
                let members = usize::try_from(list.num_of_zones)
                    .map_err(|_| Error::Unavailable("negative actual ZoneList member count"))?;
                list.zone = Some(vec![0; members]);
                for member in 0..members {
                    let name = &owners.shortcut.alpha_args[member + 1];
                    list.max_zone_name_length = list.max_zone_name_length.max(name.len());
                    let number = zones
                        .iter()
                        .position(|zone| zone.name.eq_ignore_ascii_case(name))
                        .map(|n| super::signed(n + 1))
                        .transpose()?
                        .unwrap_or(0);
                    let pointers = list
                        .zone
                        .as_mut()
                        .ok_or(Error::Unavailable("actual ZoneList member allocation"))?;
                    pointers[member] = number;
                    if number == 0 {
                        owners.backend.show_source_severe_error(
                            &format!(
                                "GetZoneData: ZoneList=\"{}\":  {} {} not found.",
                                list.name,
                                owners.shortcut.alpha_field_names[member + 1],
                                name
                            ),
                            &diagnostic_flags(owners.globals, owners.developer, owners.clock)?,
                        )?;
                        *errors_found = true;
                    }
                    for &prior in pointers.iter().take(member) {
                        if number == prior {
                            owners.backend.show_source_severe_error(
                                &format!("GetZoneData: ZoneList=\"{}\":  {} {} appears more than once in list.",
                                    list.name, owners.shortcut.alpha_field_names[member + 1], name),
                                &diagnostic_flags(owners.globals, owners.developer, owners.clock)?,
                            )?;
                            *errors_found = true;
                        }
                    }
                }
            }
        }
        // Typed linking is a boundary proof after the real getter/diagnostics;
        // it never substitutes compiled members for the actual Source list.
        if !*errors_found {
            let ids = self.source_zone_list_ids(model)?;
            for (list, id) in self
                .zone_lists
                .as_mut()
                .ok_or(Error::Unavailable("actual ZoneList allocation"))?
                .iter_mut()
                .zip(ids)
            {
                list.typed_zone_list_id = Some(id);
            }
        }
        Ok(())
    }

    pub(crate) fn source_zone_list_ids(
        &self,
        model: &TypedModel,
    ) -> Result<Vec<ZoneListId>, Error> {
        let count = usize::try_from(self.num_zone_lists)
            .map_err(|_| Error::Unavailable("negative actual ZoneList count"))?;
        if count == 0 {
            if !model.zone_lists.is_empty() {
                return Err(Error::Unavailable("raw/typed ZoneList count"));
            }
            return Ok(Vec::new());
        }
        let lists = self
            .zone_lists
            .as_deref()
            .ok_or(Error::Unavailable("actual ZoneList allocation"))?;
        let zones = self
            .zones
            .as_deref()
            .ok_or(Error::Unavailable("actual ZoneList Zone allocation"))?;
        let mut typed = model
            .zone_lists
            .iter()
            .map(|list| (list.name.0.as_str(), list))
            .collect::<BTreeMap<_, _>>();
        if lists.len() != count
            || model.zone_lists.len() != count
            || typed.len() != count
            || model
                .zone_lists
                .iter()
                .map(|list| list.id)
                .collect::<std::collections::BTreeSet<_>>()
                .len()
                != count
        {
            return Err(Error::Unavailable(
                "Source/typed ZoneList identity bijection",
            ));
        }
        let mut result = Vec::with_capacity(count);
        for list in lists {
            let row = typed
                .remove(list.name.as_str())
                .ok_or(Error::Unavailable("Source/typed exact ZoneList name"))?;
            let pointers = list
                .zone
                .as_deref()
                .ok_or(Error::Unavailable("actual ZoneList member allocation"))?;
            if pointers.is_empty()
                || i32::try_from(pointers.len()).ok() != Some(list.num_of_zones)
                || pointers.len() != row.zones.len()
                || row.max_zone_name_length != list.max_zone_name_length
                || pointers
                    .iter()
                    .collect::<std::collections::BTreeSet<_>>()
                    .len()
                    != pointers.len()
                || list.typed_zone_list_id.is_some_and(|id| id != row.id)
            {
                return Err(Error::Unavailable("Source/typed ZoneList member shape"));
            }
            for (&number, &id) in pointers.iter().zip(&row.zones) {
                let index = number
                    .checked_sub(1)
                    .and_then(|n| usize::try_from(n).ok())
                    .ok_or(Error::Unavailable(
                        "positive actual ZoneList member pointer",
                    ))?;
                if zones.get(index).and_then(|zone| zone.typed_zone_id) != Some(id) {
                    return Err(Error::Unavailable(
                        "Source/typed ZoneList member order/identity",
                    ));
                }
            }
            result.push(row.id);
        }
        if !typed.is_empty() {
            return Err(Error::Unavailable(
                "incomplete Source/typed ZoneList bijection",
            ));
        }
        Ok(result)
    }
}

pub(super) fn diagnostic_flags<'a>(
    globals: &'a crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals,
    developer: &'a bool,
    clock: Option<&'a crate::weather::day::WeatherGlobalState>,
) -> Result<SourceDiagnosticFlags<'a>, Error> {
    let clock = clock.ok_or(Error::Unavailable(
        "ZoneList diagnostic actual WeatherGlobal flags",
    ))?;
    Ok(SourceDiagnosticFlags {
        kickoff: &globals.kickoff_simulation,
        developer,
        warmup: &clock.warmup_flag,
        doing_sizing: &globals.doing_sizing,
    })
}

fn metadata(fields: &SourceInputStatisticsFields, name: &str) -> Result<usize, Error> {
    let Some(Value::Number(value)) = fields.get(name) else {
        return Err(Error::Unavailable("ZoneList physical IDF field metadata"));
    };
    let value = value
        .parse::<f64>()
        .map_err(|_| Error::Unavailable("ZoneList IDF field metadata number"))?;
    if !value.is_finite() || value < 0.0 || value > f64::from(i32::MAX) || value.fract() != 0.0 {
        return Err(Error::Unavailable(
            "ZoneList IDF signed field metadata range",
        ));
    }
    Ok(value as usize)
}

fn get_zone_list_object_item(
    index: usize,
    source_is_epjson: bool,
    input: &mut SelectedInputProcessor,
    shortcut: &mut SourceInputProcessorShortCut,
) -> Result<usize, Error> {
    let count = input.get_num_objects_found("ZoneList");
    let names = input
        .source_raw_object_names("ZoneList")
        .map_err(|_| Error::Unavailable("actual original ZoneList order"))?;
    if names.len() != count {
        return Err(Error::Unavailable("ZoneList raw/IP count"));
    }
    let name = names
        .get(index)
        .cloned()
        .ok_or(Error::Unavailable("ZoneList getObjectItem index"))?;
    if !input.has_object("ZoneList", &name) {
        return Err(Error::Unavailable("ZoneList raw/IP exact identity"));
    }
    let fields = input
        .source_raw_object_fields("ZoneList", &name)
        .map_err(|_| Error::Unavailable("actual raw ZoneList fields"))?;
    let values = match fields.get("zones") {
        None => &[][..],
        Some(Value::Array(values)) => values.as_slice(),
        _ => {
            return Err(Error::Unavailable(
                "ZoneList extensible JSON representation",
            ));
        }
    };
    let max_extensible_fields = if source_is_epjson {
        values.len()
    } else {
        let _max_fields = metadata(fields, "idf_max_fields")?;
        metadata(fields, "idf_max_extensible_fields")?
    };
    let mut members = Vec::with_capacity(values.len());
    for value in values {
        let Value::Object(fields) = value else {
            return Err(Error::Unavailable(
                "ZoneList extensible object representation",
            ));
        };
        let (value, blank) = match fields.get("zone_name") {
            None => (String::new(), true),
            Some(Value::String(value)) => (value.to_ascii_uppercase(), value.is_empty()),
            _ => return Err(Error::Unavailable("ZoneList Zone name JSON representation")),
        };
        members.push((value, blank));
    }
    let needed = members
        .len()
        .checked_add(1)
        .ok_or(Error::Unavailable("native ZoneList shared alpha overflow"))?;
    if shortcut.alpha_args.len() < needed
        || shortcut.alpha_field_names.len() < needed
        || shortcut.alpha_field_blanks.len() < needed
    {
        return Err(Error::Unavailable(
            "sole actual shared ZoneList alpha allocation",
        ));
    }
    shortcut.alpha_args.fill(String::new());
    shortcut.numeric_args.fill(0.0);
    shortcut.alpha_field_names.fill(String::new());
    shortcut.numeric_field_names.fill(String::new());
    shortcut.alpha_field_blanks.fill(true);
    shortcut.numeric_field_blanks.fill(true);
    input.mark_object_as_used("ZoneList", &name);
    shortcut.alpha_args[0] = name.to_ascii_uppercase();
    shortcut.alpha_field_names[0] = if source_is_epjson { "name" } else { "Name" }.into();
    shortcut.alpha_field_blanks[0] = name.is_empty();
    for (index, (value, blank)) in members.into_iter().enumerate() {
        shortcut.alpha_args[index + 1] = value;
        shortcut.alpha_field_names[index + 1] = if source_is_epjson {
            "zone_name"
        } else {
            "Zone Name"
        }
        .into();
        shortcut.alpha_field_blanks[index + 1] = blank;
    }
    Ok(1 + (needed - 1).min(max_extensible_fields))
}
