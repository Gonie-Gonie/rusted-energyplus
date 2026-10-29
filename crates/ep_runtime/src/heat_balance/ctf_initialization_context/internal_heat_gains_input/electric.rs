//! Genuine IHG1263-1387 ZoneElectric input, borrowing the retained misc owner.
use super::getter::{SourceGainInputOwners, sizing_severe, sizing_warning};
use super::{E, SelectedInternalHeatGains, SourceMiscGainObject, index, unavailable};
use crate::heat_balance::selected_input_processor::{
    SourceInputFamily, SourceInputProcessorShortCut,
};
use crate::heat_balance::selected_project_data::ProjectInputValue;

impl SelectedInternalHeatGains {
    pub(super) fn read_electric(
        &mut self,
        local: &mut SourceInputProcessorShortCut,
        owners: &mut SourceGainInputOwners<'_, '_>,
    ) -> Result<(), E> {
        let statements = self
            .electric_objects
            .as_ref()
            .ok_or_else(|| unavailable("actual retained zoneElectricObjects"))?
            .len();
        let mut count = 0;
        for ordinal in 0..statements {
            let values = owners.raw.get_object_item::<5, 6>(
                "ElectricEquipment",
                ordinal,
                owners.input,
                local,
            )?;
            local.alpha_args[..5].clone_from_slice(&values.alphas);
            local.numeric_args[..6].copy_from_slice(&values.numbers);
            let schedule = owners.schedules.get_source_schedule(&values.alphas[2])?;
            let invalid_schedule = if local.alpha_field_blanks[2] {
                Some(format!("{} cannot be empty.", local.alpha_field_names[2]))
            } else if schedule.is_none() {
                Some(format!(
                    "{} = {}, item not found.",
                    local.alpha_field_names[2], values.alphas[2]
                ))
            } else if !match owners.clock {
                Some(clock) => owners.schedules.source_check_min_with_clock(
                    schedule.ok_or_else(|| unavailable("actual electric schedule"))?,
                    0.0,
                    clock,
                )?,
                None => owners.schedules.source_check_min(
                    schedule.ok_or_else(|| unavailable("actual electric schedule"))?,
                    0.0,
                )?,
            } {
                Some(format!(
                    "{} = {}, schedule contains values that are < 0",
                    local.alpha_field_names[2], values.alphas[2]
                ))
            } else {
                None
            };
            if let Some(message) = invalid_schedule {
                owners.backend.show_severe_error(&format!(
                    "GetInternalHeatGains: ElectricEquipment = {}",
                    values.alphas[0]
                ))?;
                owners.backend.show_continue_error(&message)?;
                self.errors_found = true;
            }
            let field = match values.alphas[3].as_str() {
                "EQUIPMENTLEVEL" => 0,
                "WATTS/AREA" => 1,
                "WATTS/PERSON" => 2,
                _ => return Err(unavailable("ElectricEquipment official sizing method")),
            };
            let level_value = values.numbers[field];
            let level_blank = local.numeric_field_blanks[field];
            let level_name = local.numeric_field_names[field].clone();
            let object = self
                .electric_objects
                .as_ref()
                .and_then(|rows| rows.get(ordinal))
                .ok_or_else(|| unavailable("same retained electric statement"))?;
            let object_name = object.name.clone();
            let num_spaces = object.space_nums.len();
            for position in 0..num_spaces {
                let object = self
                    .electric_objects
                    .as_ref()
                    .and_then(|rows| rows.get(ordinal))
                    .ok_or_else(|| unavailable("same retained electric member"))?;
                let space_number = object.space_nums[position];
                let name = object.names[position].clone();
                let space = owners
                    .zones
                    .spaces
                    .as_ref()
                    .and_then(|rows| index(space_number).ok().and_then(|i| rows.get(i)))
                    .ok_or_else(|| unavailable("same actual ElectricEquipment Space"))?;
                let zone_number = space.zone_num;
                let row = self
                    .allocations
                    .electric_equipment
                    .as_mut()
                    .and_then(|rows| rows.get_mut(count))
                    .ok_or_else(|| unavailable("actual allocated ZoneElectric row"))?;
                row.name = name;
                row.space_index = space_number;
                row.zone_ptr = zone_number;
                row.schedule = schedule;
                // Shared setDesignLevel3431-3565. No selected EquipmentLevel
                // sign check exists here: unconditional schema minima precede
                // Native stored-number projection and are not new runtime math.
                let mut design_level = 0.0;
                if level_blank {
                    sizing_warning(
                        owners,
                        &format!(
                            "GetInternalHeatGains: ElectricEquipment=\"{}\", specifies {}, but that field is blank.  0 ElectricEquipment will result.",
                            object_name, level_name
                        ),
                    )?;
                } else {
                    if (field == 1 && level_value < 0.0) || (field == 2 && level_value <= 0.0) {
                        let number =
                            crate::heat_balance::source_round_format::round(level_value, 3)
                                .map_err(|error| {
                                    E::Format(format!("IHG design level .3R: {error:?}"))
                                })?;
                        sizing_severe(
                            owners,
                            &format!(
                                "GetInternalHeatGains: ElectricEquipment=\"{}\", invalid {}, value  [{}]={number}",
                                object_name,
                                level_name,
                                if field == 1 { "<0.0" } else { "<=0.0" }
                            ),
                        )?;
                        self.errors_found = true;
                    }
                    if !self.errors_found {
                        if field == 0 {
                            // Every admitted member is its Zone's sole Space;
                            // lists apply the complete amount to each member.
                            design_level = level_value * 1.0;
                        } else if field == 1 {
                            let space = owners
                                .zones
                                .spaces
                                .as_ref()
                                .and_then(|rows| index(space_number).ok().and_then(|i| rows.get(i)))
                                .ok_or_else(|| unavailable("same final electric Space floor"))?;
                            design_level = level_value * space.floor_area;
                            if space.floor_area <= 0.0 && !space.is_remainder_space {
                                sizing_warning(
                                    owners,
                                    &format!(
                                        "GetInternalHeatGains: ElectricEquipment=\"{}\", specifies {}, but Space Floor Area = 0.  0 ElectricEquipment will result.",
                                        object_name, level_name
                                    ),
                                )?;
                            }
                        } else {
                            let occupants = owners
                                .zones
                                .spaces
                                .as_ref()
                                .and_then(|rows| index(space_number).ok().and_then(|i| rows.get(i)))
                                .ok_or_else(|| {
                                    unavailable("same actual electric Space design People")
                                })?
                                .total_occupants;
                            design_level = level_value * occupants;
                            if occupants <= 0.0 {
                                sizing_warning(
                                    owners,
                                    &format!(
                                        "GetInternalHeatGains: ElectricEquipment=\"{}\", specifies {}, but Total Occupants = 0.  0 ElectricEquipment will result.",
                                        object_name, level_name
                                    ),
                                )?;
                            }
                        }
                    }
                }
                row.design_level = design_level;
                let schedule = schedule.ok_or_else(|| {
                    unavailable("electric schedule before actual pointer dereference")
                })?;
                row.nominal_min_design_level = row.design_level
                    * match owners.clock {
                        Some(clock) => owners.schedules.source_min_with_clock(schedule, clock)?,
                        None => owners.schedules.source_min(schedule)?,
                    };
                row.nominal_max_design_level = row.design_level
                    * match owners.clock {
                        Some(clock) => owners.schedules.source_max_with_clock(schedule, clock)?,
                        None => owners.schedules.source_max(schedule)?,
                    };
                row.fraction_latent = values.numbers[3];
                row.fraction_radiant = values.numbers[4];
                row.fraction_lost = values.numbers[5];
                row.fraction_convected =
                    1.0 - (row.fraction_latent + row.fraction_radiant + row.fraction_lost);
                if row.fraction_convected.abs() <= 0.001 {
                    row.fraction_convected = 0.0;
                }
                if row.fraction_convected < 0.0 {
                    owners.backend.show_severe_error(&format!(
                        "GetInternalHeatGains: ElectricEquipment=\"{}\", Sum of Fractions > 1.0",
                        object_name
                    ))?;
                    self.errors_found = true;
                }
                row.end_use_subcategory = if values.num_alphas > 4 {
                    values.alphas[4].clone()
                } else {
                    "General".into()
                };
                if owners.globals.any_energy_management_system_in_model {
                    return Err(unavailable(
                        "positive ElectricEquipment EMS actuator registration",
                    ));
                }
                if !self.errors_found {
                    self.setup_electric_device(space_number, count, owners.backend)?;
                }
                count += 1;
            }
        }
        Ok(())
    }
}

pub(super) fn check_electric_input_association(
    misc: &[SourceMiscGainObject],
    owners: &SourceGainInputOwners<'_, '_>,
) -> Result<(), E> {
    let bindings = &owners.model.source_electric_equipment_inputs;
    if bindings.len() != misc.len() {
        return Err(unavailable(
            "ElectricEquipment raw/typed object association count",
        ));
    }
    let zones = owners
        .zones
        .zones
        .as_deref()
        .ok_or_else(|| unavailable("ElectricEquipment association actual Zones"))?;
    let spaces = owners
        .zones
        .spaces
        .as_deref()
        .ok_or_else(|| unavailable("ElectricEquipment association actual Spaces"))?;
    let mut seen = vec![false; owners.model.electric_equipment.len()];
    let mut cursor = 0;
    let raw_rows = owners
        .raw
        .objects
        .get("ElectricEquipment")
        .ok_or_else(|| unavailable("ElectricEquipment retained raw family identity"))?;
    if raw_rows.len() != misc.len() {
        return Err(unavailable("ElectricEquipment retained raw ordinal count"));
    }
    // Native starts are written in JSON-map order before getIDFObjNum places
    // each misc row. Prove that real prefix independently of typed IDF ranges;
    // zero-member lists keep total+1, including repeated starts and empty tails.
    let Some(SourceInputFamily::Objects(json_rows)) =
        owners.input.find_exact_family("ElectricEquipment")
    else {
        return Err(unavailable(
            "ElectricEquipment actual JSON-map prefix owner",
        ));
    };
    let mut native_cursor = 0_i32;
    for shape in json_rows {
        let ordinal = raw_rows
            .iter()
            .position(|row| row.object_name == shape.object_name)
            .ok_or_else(|| unavailable("ElectricEquipment exact JSON/IDF ordinal"))?;
        let object = misc
            .get(ordinal)
            .ok_or_else(|| unavailable("ElectricEquipment actual misc ordinal"))?;
        if Some(object.space_start_ptr) != native_cursor.checked_add(1) {
            return Err(unavailable(
                "ElectricEquipment actual Native instance start",
            ));
        }
        native_cursor = native_cursor
            .checked_add(object.num_spaces)
            .ok_or_else(|| unavailable("ElectricEquipment actual Native instance prefix"))?;
    }
    for (ordinal, object) in misc.iter().enumerate() {
        let raw = &raw_rows[ordinal];
        let matching = bindings
            .iter()
            .filter(|binding| binding.object_name == raw.object_name)
            .collect::<Vec<_>>();
        if matching.len() != 1 {
            return Err(unavailable(
                "ElectricEquipment exact raw object association",
            ));
        }
        let binding = matching[0];
        if binding.instances.start != cursor {
            return Err(unavailable("ElectricEquipment dense raw instance cursor"));
        }
        if raw.object_name.to_ascii_uppercase() != object.name {
            return Err(unavailable(
                "ElectricEquipment retained raw/Native name association",
            ));
        }
        let target = match raw
            .fields
            .get("zone_or_zonelist_or_space_or_spacelist_name")
        {
            Some(ProjectInputValue::String(value)) => value.to_ascii_uppercase(),
            _ => {
                return Err(unavailable(
                    "ElectricEquipment retained raw target identity",
                ));
            }
        };
        let rows = owners
            .model
            .electric_equipment
            .get(binding.instances.clone())
            .ok_or_else(|| unavailable("ElectricEquipment actual dense typed instance range"))?;
        if rows.len() != object.space_nums.len()
            || rows.len() != object.names.len()
            || usize::try_from(object.num_spaces).ok() != Some(rows.len())
        {
            return Err(unavailable(
                "ElectricEquipment raw/typed expanded instance count",
            ));
        }
        match binding.target {
            ep_model::SourceElectricEquipmentInputTarget::Zone(id) => {
                if object.zone_list_active
                    || object.space_list_active
                    || object.space_or_space_list_ptr != 0
                    || rows.len() != 1
                    || rows[0].zone != id
                    || !zones
                        .iter()
                        .any(|zone| zone.typed_zone_id == Some(id) && zone.name == target)
                {
                    return Err(unavailable(
                        "ElectricEquipment direct Zone precedence association",
                    ));
                }
            }
            ep_model::SourceElectricEquipmentInputTarget::Space(id) => {
                let matching = spaces
                    .iter()
                    .enumerate()
                    .filter(|(_, space)| space.typed_space_id == Some(id))
                    .collect::<Vec<_>>();
                if object.zone_list_active
                    || object.space_list_active
                    || rows.len() != 1
                    || matching.len() != 1
                    || matching[0].1.name != target
                    || zones.iter().any(|zone| zone.name == target)
                    || object.space_nums.as_slice()
                        != [i32::try_from(matching[0].0 + 1).map_err(|_| {
                            unavailable("ElectricEquipment direct Space actual index")
                        })?]
                    || object.space_or_space_list_ptr != object.space_nums[0]
                {
                    return Err(unavailable(
                        "ElectricEquipment direct Space precedence association",
                    ));
                }
            }
            ep_model::SourceElectricEquipmentInputTarget::ZoneList(id) => {
                let lists =
                    owners.zones.zone_lists.as_deref().ok_or_else(|| {
                        unavailable("ElectricEquipment actual ZoneList allocation")
                    })?;
                let lists = lists
                    .iter()
                    .filter(|list| list.typed_zone_list_id == Some(id))
                    .collect::<Vec<_>>();
                let actual_zones = object
                    .space_nums
                    .iter()
                    .map(|&number| {
                        spaces
                            .get(index(number)?)
                            .map(|space| space.zone_num)
                            .ok_or_else(|| {
                                unavailable("ElectricEquipment list member Space/Zone owner")
                            })
                    })
                    .collect::<Result<Vec<_>, E>>()?;
                if !object.zone_list_active
                    || object.space_list_active
                    || object.space_or_space_list_ptr != 0
                    || lists.len() != 1
                    || lists[0].name != target
                    || lists[0].zone.as_deref() != Some(actual_zones.as_slice())
                {
                    return Err(unavailable(
                        "ElectricEquipment actual ordered ZoneList association",
                    ));
                }
            }
            ep_model::SourceElectricEquipmentInputTarget::SpaceList(id) => {
                // Recheck the real typed/actual list bijection, including member
                // order, before any allocated ElectricEquipment instance is written.
                owners.zones.source_space_list_ids(owners.model)?;
                let lists =
                    owners.zones.space_lists.as_deref().ok_or_else(|| {
                        unavailable("ElectricEquipment actual SpaceList allocation")
                    })?;
                let matching = lists
                    .iter()
                    .enumerate()
                    .filter(|(_, list)| list.typed_space_list_id == Some(id))
                    .collect::<Vec<_>>();
                if object.zone_list_active
                    || !object.space_list_active
                    || matching.len() != 1
                    || matching[0].1.name != target
                    || matching[0].1.spaces != object.space_nums
                    || matching[0].1.num_list_spaces != object.num_spaces
                    || object.space_or_space_list_ptr
                        != i32::try_from(matching[0].0 + 1).map_err(|_| {
                            unavailable("ElectricEquipment actual SpaceList signed index")
                        })?
                    || zones.iter().any(|zone| zone.name == target)
                    || spaces.iter().any(|space| space.name == target)
                    || owners
                        .zones
                        .zone_lists
                        .as_deref()
                        .unwrap_or(&[])
                        .iter()
                        .any(|list| list.name == target)
                {
                    return Err(unavailable(
                        "ElectricEquipment actual ordered SpaceList precedence association",
                    ));
                }
            }
        }
        // Only the actual empty SpaceList branch above proves a zero-member
        // statement. It retains its real pointer/flag/start without a fake row.
        if rows.is_empty()
            && !matches!(
                binding.target,
                ep_model::SourceElectricEquipmentInputTarget::SpaceList(_)
            )
        {
            return Err(unavailable(
                "ElectricEquipment empty non-SpaceList instance range",
            ));
        }
        cursor = binding.instances.end;
        for (position, row) in rows.iter().enumerate() {
            let at = binding.instances.start + position;
            let slot = seen
                .get_mut(at)
                .ok_or_else(|| unavailable("ElectricEquipment typed range coverage"))?;
            let space = spaces
                .get(index(object.space_nums[position])?)
                .ok_or_else(|| unavailable("ElectricEquipment actual expanded Space"))?;
            let zone = zones
                .get(index(space.zone_num)?)
                .ok_or_else(|| unavailable("ElectricEquipment actual expanded Zone"))?;
            let typed_space = owners
                .model
                .spaces
                .iter()
                .find(|typed| Some(typed.id) == space.typed_space_id)
                .ok_or_else(|| unavailable("ElectricEquipment genuine typed implicit Space"))?;
            if *slot
                || usize::try_from(row.id.0).ok() != Some(at)
                || row.name.0 != object.names[position]
                || zone.typed_zone_id != Some(row.zone)
                || typed_space.zone != row.zone
                || typed_space.origin == ep_model::SpaceOrigin::AutoZoneRemainder
                || (object.space_list_active
                    && typed_space.origin != ep_model::SpaceOrigin::Authored)
                || match typed_space.origin {
                    ep_model::SpaceOrigin::Authored => typed_space.name.0 != space.name,
                    ep_model::SpaceOrigin::AutoZoneDefault => {
                        typed_space.name != ep_model::NormalizedName::new(&space.name)
                            || space.name != zone.name
                    }
                    ep_model::SpaceOrigin::AutoZoneRemainder => true,
                }
                || zone.num_spaces != 1
                || zone.space_indexes.as_slice() != [object.space_nums[position]]
            {
                return Err(unavailable(
                    "ElectricEquipment typed instance/name/Zone/implicit-Space bijection",
                ));
            }
            *slot = true;
        }
    }
    if cursor != owners.model.electric_equipment.len() || seen.iter().any(|seen| !seen) {
        return Err(unavailable(
            "ElectricEquipment complete typed instance coverage",
        ));
    }
    Ok(())
}
