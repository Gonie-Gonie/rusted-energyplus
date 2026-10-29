//! Source-only ElectricEquipment identities and retained raw operand projection.
use super::{Compiler, RawValue, field_value};
use ep_model::{
    ElectricEquipment, ElectricEquipmentDesignLevelCalculationMethod as Method,
    ElectricEquipmentId, SourceElectricEquipmentInput, TypedModel,
};

fn method(value: &str) -> Option<Method> {
    if value.eq_ignore_ascii_case("EquipmentLevel") {
        Some(Method::EquipmentLevel)
    } else if value.eq_ignore_ascii_case("Watts/Area") {
        Some(Method::WattsPerZoneFloorArea)
    } else if value.eq_ignore_ascii_case("Watts/Person") {
        Some(Method::WattsPerPerson)
    } else {
        None
    }
}

impl Compiler<'_> {
    pub(super) fn parse_electric_equipment(&mut self, model: &mut TypedModel) {
        if !self.source_file_declarations {
            // Legacy compilation keeps ElectricEquipment RawOnly and unprojected.
            return;
        }
        const FAMILY: &str = "ElectricEquipment";
        let objects = self.objects(FAMILY);
        // IP schema checks all three numeric minima even when unused or when
        // the entire family has zero members. This precedes getter references.
        for (name, object) in &objects {
            self.source_empty_gain_schema(FAMILY, name, object);
        }
        let mut targets = objects
            .iter()
            .map(|(name, object)| self.internal_gain_input_targets(model, FAMILY, name, object))
            .collect::<Vec<_>>();
        let has_instances = targets.iter().flatten().any(|(_, rows)| !rows.is_empty());
        'inputs: for (index, (name, object)) in objects.into_iter().enumerate() {
            if name.contains('\n')
                || name
                    .bytes()
                    .all(|b| matches!(b, b'\t' | b'\n' | 0x0c | b'\r' | b' '))
            {
                self.error(
                    "InvalidName",
                    FAMILY,
                    Some(&name),
                    None,
                    "ElectricEquipment name must match the actual epJSON name pattern".into(),
                );
                continue;
            }
            let Some((target, members)) = targets[index].take() else {
                continue;
            };
            if has_instances
                && matches!(field_value(&object,"schedule_name"),Some(RawValue::String(v))if v.is_empty())
            {
                self.error(
                    "MissingRequiredField",
                    FAMILY,
                    Some(&name),
                    Some("schedule_name"),
                    "A nonzero ElectricEquipment family consumes every raw schedule".into(),
                );
            }
            if members.is_empty() {
                if has_instances {
                    // Native still consumes each raw schedule/method when any
                    // real ElectricEquipment instance exists in the family.
                    let _ = self.electric_equipment_schedule_reference(model, &name, &object);
                    self.enum_default(
                        FAMILY,
                        &name,
                        (&object, "design_level_calculation_method"),
                        Method::EquipmentLevel,
                        "EquipmentLevel",
                        method,
                    );
                }
                let cursor = model.electric_equipment.len();
                model
                    .source_electric_equipment_inputs
                    .push(SourceElectricEquipmentInput {
                        object_name: name,
                        target,
                        instances: cursor..cursor,
                    });
                continue;
            }
            let schedule = self.electric_equipment_schedule_reference(model, &name, &object);
            let selected_method = self.enum_default(
                FAMILY,
                &name,
                (&object, "design_level_calculation_method"),
                Method::EquipmentLevel,
                "EquipmentLevel",
                method,
            );
            let first = model.electric_equipment.len();
            let mut instances = Vec::with_capacity(members.len());
            for (zone, instance_name) in members {
                let Some(value) = self.checked_id(FAMILY, &name, first + instances.len()) else {
                    continue 'inputs;
                };
                let id = ElectricEquipmentId(value);
                // Generated public aliases never erase a distinct real owner.
                model
                    .electric_equipment_names
                    .insert_instance(&instance_name, id);
                instances.push((id, zone, instance_name));
            }
            // Preserve Native IP stored-number-kind conversion after the
            // independent authored-schema minimum checks above. Missing values
            // remain missing in raw HB/IP custody; typed defaults supply no input.
            let design_level_w = self.number_default(FAMILY, &name, &object, "design_level", 0.0);
            let watts_per_floor_area_w_per_m2 =
                self.number_default(FAMILY, &name, &object, "watts_per_floor_area", 0.0);
            let watts_per_person_w =
                self.number_default(FAMILY, &name, &object, "watts_per_person", 0.0);
            let fraction_latent = self.number_range_default(
                FAMILY,
                &name,
                &object,
                "fraction_latent",
                0.0,
                0.0..=1.0,
            );
            let fraction_radiant = self.number_range_default(
                FAMILY,
                &name,
                &object,
                "fraction_radiant",
                0.0,
                0.0..=1.0,
            );
            let fraction_lost =
                self.number_range_default(FAMILY, &name, &object, "fraction_lost", 0.0, 0.0..=1.0);
            let sum = fraction_latent + fraction_radiant + fraction_lost;
            let mut convected = 1.0 - sum;
            if convected.abs() <= 0.001 {
                convected = 0.0;
            }
            if convected < 0.0 {
                self.error(
                    "InvalidElectricEquipmentFractionSum",
                    FAMILY,
                    Some(&name),
                    Some("fraction_latent+fraction_radiant+fraction_lost"),
                    format!("ElectricEquipment/{name} Sum of Fractions > 1.0, got {sum}"),
                );
            }
            let end_use_subcategory = match field_value(&object, "end_use_subcategory") {
                Some(RawValue::String(value)) if !value.is_empty() => value.clone(),
                _ => {
                    self.record_default(FAMILY, &name, "end_use_subcategory", "General");
                    "General".into()
                }
            };
            for (id, zone, instance_name) in instances {
                model.electric_equipment.push(ElectricEquipment {
                    id,
                    name: model
                        .electric_equipment_names
                        .normalized_name(&instance_name),
                    zone,
                    schedule,
                    design_level_calculation_method: selected_method,
                    design_level_w,
                    watts_per_floor_area_w_per_m2,
                    watts_per_person_w,
                    fraction_latent,
                    fraction_radiant,
                    fraction_lost,
                    end_use_subcategory: end_use_subcategory.clone(),
                });
            }
            model
                .source_electric_equipment_inputs
                .push(SourceElectricEquipmentInput {
                    object_name: name,
                    target,
                    instances: first..model.electric_equipment.len(),
                });
        }
    }
}
