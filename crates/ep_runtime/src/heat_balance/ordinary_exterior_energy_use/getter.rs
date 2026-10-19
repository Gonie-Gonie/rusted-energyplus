//! Native119-425 getter on genuine ordered raw objects and the sole shared shortcut.
use super::*;
fn count(input: &SelectedInputProcessor, family: &str) -> Result<i32, Error> {
    i32::try_from(input.get_num_objects_found(family))
        .map_err(|_| Error::Unavailable("actual Exterior signed object count"))
}
fn size(number: i32) -> Result<usize, Error> {
    usize::try_from(number).map_err(|_| Error::Unavailable("actual Exterior allocation count"))
}
fn column(registry: &PredefinedRegistry, name: &str) -> Result<i32, Error> {
    let number = *registry
        .named_indices
        .get(name)
        .ok_or(Error::Unavailable("actual Exterior predefined column"))?;
    if number <= 0 || number > registry.column_count {
        return Err(Error::Unavailable(
            "actual Exterior predefined column ordinal",
        ));
    }
    Ok(number)
}
impl SourceExteriorEnergyUse {
    pub(super) fn get_source_input<H: SourceOutputFileHost, D: SourceOutputDictionaryHost>(
        &mut self,
        o: &mut SourceExteriorEnergyOwners<'_, '_, H, D>,
    ) -> Result<(), Error> {
        self.number_lights = count(o.input, "Exterior:Lights")?;
        self.lights = Some(vec![
            SourceExteriorLightUsage::default();
            size(self.number_lights)?
        ]);
        let fuel_count = count(o.input, "Exterior:FuelEquipment")?;
        let water_count = count(o.input, "Exterior:WaterEquipment")?;
        let equipment_count = fuel_count
            .checked_add(water_count)
            .ok_or(Error::Unavailable("actual Exterior equipment count sum128"))?;
        self.equipment = Some(vec![
            SourceExteriorEquipmentUsage::default();
            size(equipment_count)?
        ]);
        self.unique_equipment_names.reserve(size(equipment_count)?);
        self.get_input = false;
        self.number_equipment = 0;
        // Native cCurrentModuleObject is function-local, not shortcut.current_module_object.
        for number in 1..=self.number_lights {
            let counts = o.raw.get_shared_source_object_item(
                "Exterior:Lights",
                index(number)?,
                o.input,
                o.shortcut,
            )?;
            self.light_mut(number)?
                .name
                .clone_from(&o.shortcut.alpha_args[0]);
            if o.shortcut.alpha_field_blanks[1] {
                return Err(Error::Unavailable(
                    "actual Exterior blank Schedule Name severe155",
                ));
            }
            let pointer = o.schedules.get_source_schedule(&o.shortcut.alpha_args[1])?;
            self.light_mut(number)?.schedule = pointer;
            let pointer = pointer.ok_or(Error::Unavailable(
                "actual Exterior Schedule not-found severe158",
            ))?;
            let minimum = o.schedules.source_min(pointer)?;
            // Native explicitly initializes int SchMin from the Real64 getMinVal result.
            let integer_minimum = minimum.trunc();
            if !integer_minimum.is_finite()
                || integer_minimum < f64::from(i32::MIN)
                || integer_minimum > f64::from(i32::MAX)
            {
                return Err(Error::Unavailable(
                    "actual Exterior getMinVal-to-int undefined domain160",
                ));
            }
            if (integer_minimum as i32) < 0 {
                return Err(Error::Unavailable(
                    "actual Exterior negative int SchMin severe161",
                ));
            }
            self.light_mut(number)?.control = if o.shortcut.alpha_field_blanks[2]
                || o.shortcut.alpha_args[2].eq_ignore_ascii_case("ScheduleNameOnly")
            {
                SourceLightControlType::ScheduleOnly
            } else if o.shortcut.alpha_args[2].eq_ignore_ascii_case("AstronomicalClock") {
                SourceLightControlType::AstroClockOverride
            } else {
                return Err(Error::Unavailable(
                    "actual Exterior invalid Control Option severe176",
                ));
            };
            let subcategory = if counts.num_alphas > 3 {
                o.shortcut.alpha_args[3].clone()
            } else {
                "General".into()
            };
            self.light_mut(number)?.design_level = o.shortcut.numeric_args[0];
            if o.globals.any_energy_management_system_in_model {
                return Err(Error::Unavailable("actual Exterior SetupEMSActuator187"));
            }
            self.register_source_light(number, &subcategory, o)?;
            o.table
                .fixed_real_entry_2(
                    column(o.registry, "pdchExLtPower")?,
                    &self.light(number)?.name,
                    self.light(number)?.design_level,
                )
                .map_err(Error::from)?;
            self.sum_design_level += self.light(number)?.design_level;
            if self.light(number)?.control == SourceLightControlType::AstroClockOverride {
                o.table
                    .string_entry(
                        column(o.registry, "pdchExLtClock")?,
                        &self.light(number)?.name,
                        "AstronomicalClock",
                    )
                    .map_err(Error::from)?;
                o.table
                    .string_entry(
                        column(o.registry, "pdchExLtSchd")?,
                        &self.light(number)?.name,
                        "-",
                    )
                    .map_err(Error::from)?;
            } else {
                o.table
                    .string_entry(
                        column(o.registry, "pdchExLtClock")?,
                        &self.light(number)?.name,
                        "Schedule",
                    )
                    .map_err(Error::from)?;
                let schedule_name = &o
                    .schedules
                    .schedules
                    .get(pointer.0)
                    .ok_or(Error::Unavailable(
                        "actual Exterior predefined Schedule Name233",
                    ))?
                    .name;
                o.table
                    .string_entry(
                        column(o.registry, "pdchExLtSchd")?,
                        &self.light(number)?.name,
                        schedule_name,
                    )
                    .map_err(Error::from)?;
            }
        }
        o.table
            .fixed_real_entry_2(
                column(o.registry, "pdchExLtPower")?,
                "Exterior Lighting Total",
                self.sum_design_level,
            )
            .map_err(Error::from)?;
        if fuel_count > 0 {
            return Err(Error::Unavailable(
                "actual Exterior:FuelEquipment first getObjectItem242",
            ));
        }
        if water_count > 0 {
            return Err(Error::Unavailable(
                "actual Exterior:WaterEquipment first getObjectItem344",
            ));
        }
        // Both actual equipment record loops are zero. ErrorsFound(false) had no writer.
        Ok(())
    }
}
