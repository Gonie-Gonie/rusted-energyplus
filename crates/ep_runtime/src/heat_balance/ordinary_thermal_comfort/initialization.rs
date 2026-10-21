//! InitThermalComfort166-497 / GetAngleFactorList1937-2073.
use super::state::{
    SourceAngleFactorData, SourceThermalComfortData, SourceThermalComfortInAsh55,
    SourceThermalComfortSetpoint,
};
use super::*;
use crate::heat_balance::selected_input_processor::SourceInputStatisticsValue as V;
use crate::heat_balance::source_output_registration::SourceComfortOutputField as Field;
impl SourceThermalComfort {
    pub(super) fn initialize<H: SourceOutputFileHost, D: SourceOutputDictionaryHost>(
        &mut self,
        o: &mut SourceThermalComfortOwners<'_, '_, H, D>,
    ) -> Result<(), Error> {
        self.thermal_comfort_data = Some(vec![
            SourceThermalComfortData::default();
            count(o.gains.totals[0])?
        ]);
        for number in 1..=o.gains.totals[0] {
            let (
                name,
                fanger,
                pierce,
                ksu,
                adaptive_ash55,
                adaptive_cen15251,
                cooling_effect_ash55,
                ankle_draft_ash55,
            ) = {
                let p = o
                    .gains
                    .allocations
                    .people
                    .get(index(number)?)
                    .ok_or(Error::Unavailable("actual InitThermalComfort People"))?;
                (
                    p.name.clone(),
                    p.fanger,
                    p.pierce,
                    p.ksu,
                    p.adaptive_ash55,
                    p.adaptive_cen15251,
                    p.cooling_effect_ash55,
                    p.ankle_draft_ash55,
                )
            };
            if fanger {
                self.register(
                    o,
                    Field::PeopleFangerPmv,
                    number,
                    "Zone Thermal Comfort Fanger Model PMV",
                    &name,
                )?;
                self.register(
                    o,
                    Field::PeopleFangerPpd,
                    number,
                    "Zone Thermal Comfort Fanger Model PPD",
                    &name,
                )?;
                self.register(
                    o,
                    Field::PeopleClothingSurfaceTemperature,
                    number,
                    "Zone Thermal Comfort Clothing Surface Temperature",
                    &name,
                )?;
            }
            if pierce {
                return Err(Error::Unavailable("actual positive Pierce registration209"));
            }
            if ksu {
                return Err(Error::Unavailable("actual positive KSU registration249"));
            }
            if fanger || pierce || ksu {
                self.register(
                    o,
                    Field::PeopleMeanRadiantTemperature,
                    number,
                    "Zone Thermal Comfort Mean Radiant Temperature",
                    &name,
                )?;
                self.register(
                    o,
                    Field::PeopleOperativeTemperature,
                    number,
                    "Zone Thermal Comfort Operative Temperature",
                    &name,
                )?;
                self.register(
                    o,
                    Field::PeopleClothingValue,
                    number,
                    "Zone Thermal Comfort Clothing Value",
                    &name,
                )?;
            }
            if adaptive_ash55 {
                return Err(Error::Unavailable(
                    "actual positive adaptiveASH55 registration284",
                ));
            }
            if adaptive_cen15251 {
                return Err(Error::Unavailable(
                    "actual positive adaptiveCEN registration317",
                ));
            }
            if cooling_effect_ash55 {
                return Err(Error::Unavailable(
                    "actual positive CoolingEffect registration357",
                ));
            }
            if ankle_draft_ash55 {
                self.register(
                    o,
                    Field::PeopleAnkleDraftPpd,
                    number,
                    "Zone Thermal Comfort ASHRAE 55 Ankle Draft PPD",
                    &name,
                )?;
            }
        }
        self.thermal_comfort_in_ash55 = Some(vec![
            SourceThermalComfortInAsh55::default();
            count(o.globals.num_of_zones)?
        ]);
        for number in 1..=o.gains.totals[0] {
            let p = o
                .gains
                .allocations
                .people
                .get(index(number)?)
                .ok_or(Error::Unavailable("actual Show55Warning People"))?;
            if p.show55_warning {
                self.thermal_comfort_in_ash55
                    .as_mut()
                    .and_then(|v| v.get_mut(index(p.zone_ptr).ok()?))
                    .ok_or(Error::Unavailable("actual Show55Warning Zone"))?
                    .enable55_warning = true;
            }
        }
        for number in 1..=o.globals.num_of_zones {
            let key = zone_name(o.zones, number)?;
            self.register(
                o,
                Field::ZoneSimpleSummer,
                number,
                "Zone Thermal Comfort ASHRAE 55 Simple Model Summer Clothes Not Comfortable Time",
                key,
            )?;
            self.register(
                o,
                Field::ZoneSimpleWinter,
                number,
                "Zone Thermal Comfort ASHRAE 55 Simple Model Winter Clothes Not Comfortable Time",
                key,
            )?;
            self.register(o,Field::ZoneSimpleEither,number,"Zone Thermal Comfort ASHRAE 55 Simple Model Summer or Winter Clothes Not Comfortable Time",key)?;
        }
        self.register(
            o,
            Field::FacilitySimpleSummer,
            0,
            "Facility Thermal Comfort ASHRAE 55 Simple Model Summer Clothes Not Comfortable Time",
            "Facility",
        )?;
        self.register(
            o,
            Field::FacilitySimpleWinter,
            0,
            "Facility Thermal Comfort ASHRAE 55 Simple Model Winter Clothes Not Comfortable Time",
            "Facility",
        )?;
        self.register(o,Field::FacilitySimpleEither,0,"Facility Thermal Comfort ASHRAE 55 Simple Model Summer or Winter Clothes Not Comfortable Time","Facility")?;
        self.thermal_comfort_setpoint = Some(vec![
            SourceThermalComfortSetpoint::default();
            count(o.globals.num_of_zones)?
        ]);
        for number in 1..=o.globals.num_of_zones {
            let key = zone_name(o.zones, number)?;
            self.register(
                o,
                Field::ZoneNotMetHeating,
                number,
                "Zone Heating Setpoint Not Met Time",
                key,
            )?;
            self.register(
                o,
                Field::ZoneNotMetHeatingOccupied,
                number,
                "Zone Heating Setpoint Not Met While Occupied Time",
                key,
            )?;
            self.register(
                o,
                Field::ZoneNotMetCooling,
                number,
                "Zone Cooling Setpoint Not Met Time",
                key,
            )?;
            self.register(
                o,
                Field::ZoneNotMetCoolingOccupied,
                number,
                "Zone Cooling Setpoint Not Met While Occupied Time",
                key,
            )?;
        }
        self.register(
            o,
            Field::FacilityNotMetHeating,
            0,
            "Facility Heating Setpoint Not Met Time",
            "Facility",
        )?;
        self.register(
            o,
            Field::FacilityNotMetCooling,
            0,
            "Facility Cooling Setpoint Not Met Time",
            "Facility",
        )?;
        self.register(
            o,
            Field::FacilityNotMetHeatingOccupied,
            0,
            "Facility Heating Setpoint Not Met While Occupied Time",
            "Facility",
        )?;
        self.register(
            o,
            Field::FacilityNotMetCoolingOccupied,
            0,
            "Facility Cooling Setpoint Not Met While Occupied Time",
            "Facility",
        )?;
        self.get_source_angle_factor_list(o)?;
        self.zone_occ_hrs = Some(vec![0.0; count(o.globals.num_of_zones)?]);
        Ok(())
    }
    /// Actual InputProcessor getObjectItem932-1132 for this extensible family.
    /// The already allocated six shared arrays are reset, never resized here.
    pub(crate) fn get_source_angle_factor_object_item(
        input: &mut SelectedInputProcessor,
        shortcut: &mut SourceInputProcessorShortCut,
        ordinal: usize,
        source_is_epjson: bool,
    ) -> Result<(usize, usize, i32), Error> {
        const FAMILY: &str = "ComfortViewFactorAngles";
        let names = input
            .source_raw_object_names(FAMILY)
            .map_err(|_| Error::Unavailable("actual AngleFactor raw declaration order"))?;
        if names.len() != input.get_num_objects_found(FAMILY) {
            return Err(Error::Unavailable("actual AngleFactor raw/order count"));
        }
        let name = names.get(ordinal).cloned().ok_or(Error::Unavailable(
            "actual AngleFactor getObjectItem ordinal",
        ))?;
        // findMaxFields occurs before the shared output resets and usage erase.
        let fields = input
            .source_raw_object_fields(FAMILY, &name)
            .map_err(|_| Error::Unavailable("actual AngleFactor raw fields"))?;
        let rows = match fields.get("angles") {
            None => &[][..],
            Some(V::Array(rows)) => rows.as_slice(),
            _ => return Err(Error::Unavailable("actual AngleFactor schema array")),
        };
        let metadata = |key: &str| -> Result<usize, Error> {
            match fields.get(key) {
                None => Ok(0), // Native MaxFields ctor0 when metadata is absent.
                Some(V::Number(text)) => text
                    .parse()
                    .map_err(|_| Error::Unavailable("actual AngleFactor IDF field-count integer")),
                _ => Err(Error::Unavailable("actual AngleFactor IDF count operand")),
            }
        };
        let max_extensible_fields = if source_is_epjson {
            rows.len()
                .checked_mul(2)
                .ok_or(Error::Unavailable("actual AngleFactor extensible count"))?
        } else {
            // F and E are independent Native parser cells, notably at true EOF.
            // The only fixed field is Name, whose special case always counts1.
            let _max_fields = metadata("idf_max_fields")?;
            metadata("idf_max_extensible_fields")?
        };
        shortcut.alpha_args.fill(String::new());
        shortcut.numeric_args.fill(0.0);
        shortcut.numeric_field_blanks.fill(true);
        shortcut.alpha_field_blanks.fill(true);
        shortcut.alpha_field_names.fill(String::new());
        shortcut.numeric_field_names.fill(String::new());
        input.mark_object_as_used(FAMILY, &name);
        let fields = input
            .source_raw_object_fields(FAMILY, &name)
            .map_err(|_| Error::Unavailable("same AngleFactor fields after usage erase"))?;
        let rows = match fields.get("angles") {
            None => &[][..],
            Some(V::Array(rows)) => rows.as_slice(),
            _ => return Err(Error::Unavailable("same AngleFactor retained array")),
        };
        *shortcut
            .alpha_args
            .get_mut(0)
            .ok_or(Error::Unavailable("actual AngleFactor shared Name extent"))? =
            name.to_ascii_uppercase();
        *shortcut
            .alpha_field_blanks
            .get_mut(0)
            .ok_or(Error::Unavailable(
                "actual AngleFactor shared Name blank extent",
            ))? = name.is_empty();
        *shortcut
            .alpha_field_names
            .get_mut(0)
            .ok_or(Error::Unavailable(
                "actual AngleFactor shared Name label extent",
            ))? = if source_is_epjson { "name" } else { "Name" }.into();
        let mut num_alphas = 1;
        let mut num_numbers = 0;
        let mut extensible_count = 0;
        for (at, value) in rows.iter().enumerate() {
            let V::Object(row) = value else {
                return Err(Error::Unavailable(
                    "actual AngleFactor schema extensible row",
                ));
            };
            let (alpha, blank) = match row.get("surface_name") {
                None => (String::new(), true),
                Some(V::String(text)) => (text.to_ascii_uppercase(), text.is_empty()),
                _ => {
                    return Err(Error::Unavailable(
                        "actual AngleFactor schema surface string",
                    ));
                }
            };
            let a = at
                .checked_add(1)
                .ok_or(Error::Unavailable("actual AngleFactor alpha index"))?;
            *shortcut
                .alpha_args
                .get_mut(a)
                .ok_or(Error::Unavailable("actual AngleFactor shared alpha extent"))? = alpha;
            *shortcut
                .alpha_field_blanks
                .get_mut(a)
                .ok_or(Error::Unavailable(
                    "actual AngleFactor shared alpha blank extent",
                ))? = blank;
            if extensible_count < max_extensible_fields {
                num_alphas = a + 1;
            }
            *shortcut
                .alpha_field_names
                .get_mut(a)
                .ok_or(Error::Unavailable(
                    "actual AngleFactor shared alpha label extent",
                ))? = if source_is_epjson {
                "surface_name"
            } else {
                "Surface Name"
            }
            .into();
            extensible_count += 1;
            let (number, blank) = match row.get("angle_factor") {
                None => (0.0, true), // No schema default; the reset value remains.
                Some(V::Number(text)) => (
                    ep_model::classify_source_json_number(text)
                        .map_err(|_| Error::Unavailable("actual AngleFactor JSON numeric origin"))?
                        .as_f64(),
                    false,
                ),
                _ => return Err(Error::Unavailable("actual AngleFactor schema number")),
            };
            *shortcut.numeric_args.get_mut(at).ok_or(Error::Unavailable(
                "actual AngleFactor shared numeric extent",
            ))? = number;
            *shortcut
                .numeric_field_blanks
                .get_mut(at)
                .ok_or(Error::Unavailable(
                    "actual AngleFactor shared numeric blank extent",
                ))? = blank;
            if extensible_count < max_extensible_fields {
                num_numbers = at + 1;
            }
            *shortcut
                .numeric_field_names
                .get_mut(at)
                .ok_or(Error::Unavailable(
                    "actual AngleFactor shared numeric label extent",
                ))? = if source_is_epjson {
                "angle_factor"
            } else {
                "Angle Factor"
            }
            .into();
            extensible_count += 1;
        }
        i32::try_from(num_alphas)
            .map_err(|_| Error::Unavailable("actual AngleFactor signed alpha count"))?;
        i32::try_from(num_numbers)
            .map_err(|_| Error::Unavailable("actual AngleFactor signed numeric count"))?;
        Ok((num_alphas, num_numbers, 1)) // Actual successful IOStatus.
    }

    fn get_source_angle_factor_list<H, D>(
        &mut self,
        o: &mut SourceThermalComfortOwners<'_, '_, H, D>,
    ) -> Result<(), Error> {
        const FAMILY: &str = "ComfortViewFactorAngles";
        o.shortcut.current_module_object = FAMILY.into();
        let number = o.input.get_num_objects_found(FAMILY);
        i32::try_from(number)
            .map_err(|_| Error::Unavailable("actual AngleFactor signed list count"))?;
        self.angle_factor_list = Some(vec![SourceAngleFactorData::default(); number]);
        let mut errors_found = false;
        for item in 0..number {
            let (_num_alphas, num_numbers, _status) = Self::get_source_angle_factor_object_item(
                o.input,
                o.shortcut,
                item,
                *o.source_is_epjson,
            )?;
            let list = self
                .angle_factor_list
                .as_mut()
                .and_then(|lists| lists.get_mut(item))
                .ok_or(Error::Unavailable("actual AngleFactor allocated list"))?;
            list.name = o.shortcut.alpha_args[0].clone();
            list.total_angle_factor_surfaces = i32::try_from(num_numbers)
                .map_err(|_| Error::Unavailable("actual AngleFactor member count"))?;
            list.surface_name = vec![String::new(); num_numbers];
            list.surface_ptr = vec![0; num_numbers];
            list.angle_factor = vec![0.0; num_numbers];
            let mut all_angle_fac_summed = 0.0;
            for surf_num in 0..num_numbers {
                list.surface_name[surf_num] = o.shortcut.alpha_args[surf_num + 1].clone();
                let surfaces = o
                    .geometry
                    .final_surfaces
                    .as_deref()
                    .ok_or(Error::Unavailable(
                        "actual AngleFactor final Surface allocation",
                    ))?;
                list.surface_ptr[surf_num] = surfaces
                    .iter()
                    .position(|surface| surface.row.name == o.shortcut.alpha_args[surf_num + 1])
                    .map(|at| {
                        i32::try_from(at + 1)
                            .map_err(|_| Error::Unavailable("actual AngleFactor Surface ordinal"))
                    })
                    .transpose()?
                    .unwrap_or(0);
                list.angle_factor[surf_num] = o.shortcut.numeric_args[surf_num];
                if list.surface_ptr[surf_num] == 0 {
                    o.backend.show_severe_error(&format!(
                        "{}: invalid {}, entered value={}",
                        FAMILY,
                        o.shortcut.alpha_field_names[surf_num + 1],
                        o.shortcut.alpha_args[surf_num + 1]
                    ))?;
                    o.backend.show_continue_error(&format!(
                        "ref {}={} not found in {}={}",
                        o.shortcut.alpha_field_names[0],
                        o.shortcut.alpha_args[0],
                        o.shortcut.alpha_field_names[1],
                        o.shortcut.alpha_args[1]
                    ))?;
                    errors_found = true;
                } else {
                    let surface = &surfaces[index(list.surface_ptr[surf_num])?].row;
                    if surf_num == 0 {
                        list.enclosure_ptr = surface.rad_encl_index;
                    }
                    if list.enclosure_ptr != surface.rad_encl_index {
                        // routineName includes ': '; Native adds the second ':'.
                        o.backend.show_warning_error(&format!("GetAngleFactorList: : For {}=\"{}\", surfaces are not all in the same radiant enclosure.", FAMILY, list.name))?;
                        let first =
                            surfaces
                                .get(index(list.surface_ptr[0])?)
                                .ok_or(Error::Unavailable(
                                    "actual AngleFactor first Surface diagnostic",
                                ))?;
                        let enclosures = o.radiation.radiant_enclosures();
                        let first_name = &enclosures
                            .get(index(list.enclosure_ptr)?)
                            .ok_or(Error::Unavailable(
                                "actual AngleFactor first enclosure diagnostic",
                            ))?
                            .name;
                        o.backend.show_continue_error(&format!(
                            "... Surface=\"{}\" is in enclosure=\"{}\"",
                            first.row.name, first_name
                        ))?;
                        let this_name = &enclosures
                            .get(index(surface.rad_encl_index)?)
                            .ok_or(Error::Unavailable(
                                "actual AngleFactor other enclosure diagnostic",
                            ))?
                            .name;
                        o.backend.show_continue_error(&format!(
                            "... Surface=\"{}\" is in enclosure=\"{}\"",
                            surface.name, this_name
                        ))?;
                    }
                }
                all_angle_fac_summed += list.angle_factor[surf_num];
            }
            if (all_angle_fac_summed - 1.0).abs() > 0.01 {
                o.backend.show_severe_error(&format!(
                    "{}=\"{}\", invalid - Sum[AngleFactors]",
                    FAMILY, o.shortcut.alpha_args[0]
                ))?;
                let sum = crate::heat_balance::source_round_format::round(all_angle_fac_summed, 3)
                    .map_err(|error| Error::Format(format!("AngleFactor sum .3R: {error:?}")))?;
                let limit = crate::heat_balance::source_round_format::round(0.01, 3)
                    .map_err(|error| Error::Format(format!("AngleFactor limit .3R: {error:?}")))?;
                o.backend.show_continue_error(&format!("...Sum of Angle Factors [{}] should not deviate from expected sum [1.0] by more than limit [{}].", sum, limit))?;
                errors_found = true;
            }
        }
        if errors_found {
            return match o
                .backend
                .show_fatal_error("GetAngleFactorList: Program terminated due to preceding errors.")
            {
                Err(error) => Err(Error::Fatal(error)),
                Ok(never) => match never {},
            };
        }
        for item in 0..count(o.gains.totals[0])? {
            let people =
                o.gains
                    .allocations
                    .people
                    .rows
                    .get_mut(item)
                    .ok_or(Error::Unavailable(
                        "actual AngleFactor People association row",
                    ))?;
            if people.mrt_calc_type != 2 {
                continue;
            }
            let lists = self
                .angle_factor_list
                .as_ref()
                .ok_or(Error::Unavailable("actual AngleFactor association lists"))?;
            // Native container FindItemInList compares already-uppercase names
            // exactly, including blank/whitespace and first case-variant match.
            people.angle_factor_list_ptr = lists
                .iter()
                .position(|list| list.name == people.angle_factor_list_name)
                .map(|at| {
                    i32::try_from(at + 1)
                        .map_err(|_| Error::Unavailable("actual AngleFactor list ordinal"))
                })
                .transpose()?
                .unwrap_or(0);
            let needs_list = people.fanger || people.pierce || people.ksu;
            if people.angle_factor_list_ptr == 0 && needs_list {
                o.backend.show_severe_error(&format!(
                    "GetAngleFactorList: {}=\"{}\", invalid",
                    FAMILY, people.angle_factor_list_name
                ))?;
                o.backend.show_continue_error(&format!(
                    "... Angle Factor List Name not found for PEOPLE=\"{}\"",
                    people.name
                ))?;
                errors_found = true;
            } else {
                let list = lists
                    .get(index(people.angle_factor_list_ptr)?)
                    .ok_or(Error::Unavailable("actual AngleFactor associated list"))?;
                let people_enclosure = o
                    .zones
                    .spaces
                    .as_deref()
                    .and_then(|spaces| spaces.get(index(people.space_index).ok()?))
                    .ok_or(Error::Unavailable("actual AngleFactor People Space"))?
                    .radiant_enclosure_num;
                if people_enclosure != list.enclosure_ptr && needs_list {
                    o.backend.show_warning_error(&format!(
                        "GetAngleFactorList: {}=\"{}\", radiant enclosure mismatch.",
                        FAMILY, list.name
                    ))?;
                    let enclosures = o.radiation.radiant_enclosures();
                    let list_name = &enclosures
                        .get(index(list.enclosure_ptr)?)
                        .ok_or(Error::Unavailable(
                            "actual AngleFactor associated enclosure name",
                        ))?
                        .name;
                    let people_name = &enclosures
                        .get(index(people_enclosure)?)
                        .ok_or(Error::Unavailable(
                            "actual AngleFactor People enclosure name",
                        ))?
                        .name;
                    o.backend.show_continue_error(&format!(
                        "...Enclosure=\"{}\" doe not match enclosure=\"{}\" for PEOPLE=\"{}\"",
                        list_name, people_name, people.name
                    ))?;
                }
            }
        }
        if errors_found {
            return match o
                .backend
                .show_fatal_error("GetAngleFactorList: Program terminated due to preceding errors.")
            {
                Err(error) => Err(Error::Fatal(error)),
                Ok(never) => match never {},
            };
        }
        Ok(())
    }
}
