//! Distinct ElectricEquipment raw/typed ownership on Source A, no Native IDs.
use ep_model::{
    ElectricEquipmentDesignLevelCalculationMethod as Method,
    InternalGainScheduleReference as Schedule, SourceGainInputTarget as Target, Space, SpaceOrigin,
    TypedModel, ZoneId,
};
use ep_raw_model::{FieldName, ObjectType, RawModel, RawObject, RawValue};

fn field<'a>(row: &'a RawObject, name: &str) -> Option<&'a RawValue> {
    row.fields.get(&FieldName(name.into()))
}
fn actual<'a>(raw: &'a RawModel, family: &str, reference: &str) -> Option<&'a str> {
    raw.objects
        .get(&ObjectType(family.into()))?
        .keys()
        .find(|name| name.0.eq_ignore_ascii_case(reference))
        .map(|name| name.0.as_str())
}
fn whole_space(model: &TypedModel, zone: ZoneId) -> Result<&Space, String> {
    let rows = model
        .spaces
        .iter()
        .filter(|space| space.zone == zone)
        .collect::<Vec<_>>();
    let owner = model
        .zones
        .iter()
        .find(|row| row.id == zone)
        .ok_or("ElectricEquipment Zone owner missing")?;
    if rows.len() != 1
        || owner.spaces.as_slice() != [rows[0].id]
        || !matches!(
            rows[0].origin,
            SpaceOrigin::Authored | SpaceOrigin::AutoZoneDefault
        )
    {
        return Err("ElectricEquipment requires its actual sole whole-Zone Space".into());
    }
    Ok(rows[0])
}
fn prefix(raw: &RawModel, model: &TypedModel, space: &Space) -> Result<String, String> {
    let family = if space.origin == SpaceOrigin::Authored {
        "Space"
    } else {
        "Zone"
    };
    let names = raw
        .objects
        .get(&ObjectType(family.into()))
        .ok_or("ElectricEquipment raw Space identity missing")?
        .keys()
        .filter(|name| {
            if family == "Space" {
                model.authored_space_names.resolve(&name.0) == Some(space.id)
            } else {
                model.zone_names.resolve(&name.0) == Some(space.zone)
            }
        })
        .map(|name| name.0.to_ascii_uppercase())
        .collect::<Vec<_>>();
    if names.len() != 1 {
        return Err("ElectricEquipment Space/raw identity is ambiguous".into());
    }
    Ok(names[0].clone())
}
fn expected(
    raw: &RawModel,
    model: &TypedModel,
    name: &str,
    row: &RawObject,
) -> Result<(Target, Vec<(ZoneId, String)>), String> {
    let Some(RawValue::String(reference)) =
        field(row, "zone_or_zonelist_or_space_or_spacelist_name")
    else {
        return Err("ElectricEquipment target must be an actual string".into());
    };
    let direct = name.to_ascii_uppercase();
    if let Some(raw_name) = actual(raw, "Zone", reference) {
        let id = model
            .zone_names
            .resolve(raw_name)
            .ok_or("ElectricEquipment exact Zone identity missing")?;
        whole_space(model, id)?;
        return Ok((Target::Zone(id), vec![(id, direct)]));
    }
    if let Some(raw_name) = actual(raw, "Space", reference) {
        let id = model
            .authored_space_names
            .resolve(raw_name)
            .ok_or("ElectricEquipment exact Space identity missing")?;
        let space = model
            .spaces
            .iter()
            .find(|space| space.id == id && space.origin == SpaceOrigin::Authored)
            .ok_or("ElectricEquipment actual authored Space missing")?;
        if whole_space(model, space.zone)?.id != id {
            return Err("ElectricEquipment direct Space owner differs".into());
        }
        return Ok((Target::Space(id), vec![(space.zone, direct)]));
    }
    if let Some(raw_name) = actual(raw, "ZoneList", reference) {
        let id = model
            .zone_list_names
            .resolve(raw_name)
            .ok_or("ElectricEquipment exact ZoneList identity missing")?;
        let list = model
            .zone_lists
            .iter()
            .find(|list| list.id == id)
            .ok_or("ElectricEquipment ZoneList owner missing")?;
        let list_row = &raw.objects[&ObjectType("ZoneList".into())]
            [&ep_raw_model::ObjectName(raw_name.into())];
        let Some(RawValue::Array(members)) = field(list_row, "zones") else {
            return Err("ElectricEquipment actual ZoneList members missing".into());
        };
        let mut ids = Vec::new();
        let mut instances = Vec::new();
        for member in members {
            let RawValue::Object(fields) = member else {
                return Err("ElectricEquipment ZoneList member shape".into());
            };
            let Some(RawValue::String(reference)) = fields.get(&FieldName("zone_name".into()))
            else {
                return Err("ElectricEquipment ZoneList Zone reference missing".into());
            };
            let raw_zone = actual(raw, "Zone", reference)
                .ok_or("ElectricEquipment missing exact member Zone")?;
            let zone = model
                .zone_names
                .resolve(raw_zone)
                .ok_or("ElectricEquipment member Zone identity missing")?;
            let space = whole_space(model, zone)?;
            ids.push(zone);
            instances.push((zone, format!("{} {direct}", prefix(raw, model, space)?)));
        }
        if ids.is_empty() || list.zones != ids {
            return Err("ElectricEquipment ordered ZoneList identity differs".into());
        }
        return Ok((Target::ZoneList(id), instances));
    }
    let raw_name =
        actual(raw, "SpaceList", reference).ok_or("ElectricEquipment missing exact target")?;
    let id = model
        .space_list_names
        .resolve(raw_name)
        .ok_or("ElectricEquipment exact SpaceList identity missing")?;
    let list = model
        .space_lists
        .iter()
        .find(|list| list.id == id)
        .ok_or("ElectricEquipment SpaceList owner missing")?;
    let list_row =
        &raw.objects[&ObjectType("SpaceList".into())][&ep_raw_model::ObjectName(raw_name.into())];
    let members = match field(list_row, "spaces") {
        None => &[][..],
        Some(RawValue::Array(members)) => members.as_slice(),
        _ => return Err("ElectricEquipment actual SpaceList members missing".into()),
    };
    let mut ids = Vec::new();
    let mut instances = Vec::new();
    for member in members {
        let RawValue::Object(fields) = member else {
            return Err("ElectricEquipment SpaceList member shape".into());
        };
        let Some(RawValue::String(reference)) = fields.get(&FieldName("space_name".into())) else {
            return Err("ElectricEquipment SpaceList Space reference missing".into());
        };
        let raw_space = actual(raw, "Space", reference)
            .ok_or("ElectricEquipment missing exact member Space")?;
        let sid = model
            .authored_space_names
            .resolve(raw_space)
            .ok_or("ElectricEquipment member Space identity missing")?;
        let space = model
            .spaces
            .iter()
            .find(|space| space.id == sid && space.origin == SpaceOrigin::Authored)
            .ok_or("ElectricEquipment actual Space member missing")?;
        if whole_space(model, space.zone)?.id != sid {
            return Err("ElectricEquipment member whole-Space association differs".into());
        }
        ids.push(sid);
        instances.push((
            space.zone,
            format!("{} {direct}", prefix(raw, model, space)?),
        ));
    }
    if list.spaces != ids {
        return Err("ElectricEquipment ordered SpaceList identity differs".into());
    }
    Ok((Target::SpaceList(id), instances))
}

fn number(row: &RawObject, name: &str) -> Result<f64, String> {
    match field(row, name) {
        None => Ok(0.0),
        Some(RawValue::Number(text)) => ep_model::classify_source_json_number(text)
            .map(ep_model::SourceJsonNumber::as_f64)
            .map_err(|_| format!("ElectricEquipment invalid stored number {name}")),
        _ => Err(format!("ElectricEquipment nonnumeric {name}")),
    }
}
fn method(row: &RawObject) -> Result<Method, String> {
    match field(row, "design_level_calculation_method") {
        None => Ok(Method::EquipmentLevel),
        Some(RawValue::String(value))
            if value.is_empty() || value.eq_ignore_ascii_case("EquipmentLevel") =>
        {
            Ok(Method::EquipmentLevel)
        }
        Some(RawValue::String(value)) if value.eq_ignore_ascii_case("Watts/Area") => {
            Ok(Method::WattsPerZoneFloorArea)
        }
        Some(RawValue::String(value)) if value.eq_ignore_ascii_case("Watts/Person") => {
            Ok(Method::WattsPerPerson)
        }
        _ => Err("ElectricEquipment actual method differs".into()),
    }
}
fn schedule(model: &TypedModel, row: &RawObject) -> Result<Option<Schedule>, String> {
    let Some(RawValue::String(value)) = field(row, "schedule_name") else {
        return Err("ElectricEquipment actual schedule string missing".into());
    };
    if value.eq_ignore_ascii_case("Constant-0.0") {
        Ok(Some(Schedule::ConstantZero))
    } else if value.eq_ignore_ascii_case("Constant-1.0") {
        Ok(Some(Schedule::ConstantOne))
    } else {
        model
            .schedule_names
            .resolve(value)
            .map(|id| Some(Schedule::User(id)))
            .ok_or("ElectricEquipment actual schedule identity missing".into())
    }
}

pub(super) fn validate(raw: &RawModel, model: &TypedModel, violations: &mut Vec<String>) {
    let result = (|| -> Result<(), String> {
        let rows = raw
            .source_ordered_instances("ElectricEquipment")
            .map_err(|_| "ElectricEquipment raw order unavailable")?;
        if rows.len() != model.source_electric_equipment_inputs.len()
            || model.electric_equipment_names.names().len() != model.electric_equipment.len()
        {
            return Err("ElectricEquipment distinct raw/arena/name counts differ".into());
        }
        let mut cursor = 0;
        for ((name, row), binding) in rows.iter().zip(&model.source_electric_equipment_inputs) {
            let (target, members) = expected(raw, model, &name.0, row)?;
            let range = binding.instances.clone();
            if binding.object_name != name.0
                || binding.target != target
                || range.start != cursor
                || range.start > range.end
                || range.end > model.electric_equipment.len()
                || range.len() != members.len()
            {
                return Err("ElectricEquipment ordered raw range or target differs".into());
            }
            // Genuine empty lists preserve the raw declaration, not getter rows.
            // The raw boundary has already validated all structural schema fields.
            if !range.is_empty() {
                let method = method(row)?;
                let schedule = schedule(model, row)?;
                let end_use = match field(row, "end_use_subcategory") {
                    Some(RawValue::String(value)) if !value.is_empty() => value.as_str(),
                    _ => "General",
                };
                let values = [
                    number(row, "design_level")?,
                    number(row, "watts_per_floor_area")?,
                    number(row, "watts_per_person")?,
                    number(row, "fraction_latent")?,
                    number(row, "fraction_radiant")?,
                    number(row, "fraction_lost")?,
                ];
                for (offset, (instance, (zone, instance_name))) in model.electric_equipment
                    [range.clone()]
                .iter()
                .zip(&members)
                .enumerate()
                {
                    let actual = [
                        instance.design_level_w,
                        instance.watts_per_floor_area_w_per_m2,
                        instance.watts_per_person_w,
                        instance.fraction_latent,
                        instance.fraction_radiant,
                        instance.fraction_lost,
                    ];
                    if usize::try_from(instance.id.0).ok() != Some(range.start + offset)
                        || instance.zone != *zone
                        || instance.name.0 != *instance_name
                        || model.electric_equipment_names.names()[range.start + offset]
                            != instance.name
                        || instance.design_level_calculation_method != method
                        || instance.schedule != schedule
                        || instance.end_use_subcategory != end_use
                        || actual
                            .iter()
                            .zip(values)
                            .any(|(a, b)| a.to_bits() != b.to_bits())
                    {
                        return Err(
                            "ElectricEquipment dense instance identity or raw operands differ"
                                .into(),
                        );
                    }
                }
            }
            cursor = range.end;
        }
        if cursor != model.electric_equipment.len() {
            return Err("ElectricEquipment raw ranges must cover its entire distinct arena".into());
        }
        Ok(())
    })();
    if let Err(error) = result {
        violations.push(error);
    }
}
