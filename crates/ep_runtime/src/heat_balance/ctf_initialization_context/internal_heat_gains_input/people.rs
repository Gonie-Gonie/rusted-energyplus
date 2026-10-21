//! Genuine direct People getter over retained IP, schedule and Zone/Space owners.
use super::getter::SourceGainInputOwners;
use super::{E, SelectedInternalHeatGains, SourceMiscGainObject, index, unavailable};
use crate::heat_balance::error_backend::SourceDiagnosticFlags;
use crate::heat_balance::selected_input_processor::SourceInputProcessorShortCut;
use crate::heat_balance::selected_schedule_manager::SourceScheduleIndex;

fn diagnostic(
    owners: &mut SourceGainInputOwners<'_, '_>,
    message: &str,
    severe: bool,
) -> Result<(), E> {
    let clock = owners
        .clock
        .ok_or_else(|| unavailable("actual People diagnostic clock"))?;
    let flags = SourceDiagnosticFlags {
        kickoff: &owners.globals.kickoff_simulation,
        developer: owners.developer,
        warmup: &clock.warmup_flag,
        doing_sizing: &owners.globals.doing_sizing,
    };
    if severe {
        owners.backend.show_source_severe_error(message, &flags)
    } else {
        owners.backend.show_source_warning_error(message, &flags)
    }
}
fn round(value: f64, precision: usize) -> Result<String, E> {
    crate::heat_balance::source_round_format::round(value, precision)
        .map_err(|e| E::Format(format!("IHG People .{precision}R: {e:?}")))
}
// Exact direct/area sizing child, over the selected local numeric operand.
// It writes no People field until every reached diagnostic has returned.
fn design_level(
    errors_found: &mut bool,
    object: &SourceMiscGainObject,
    method: usize,
    space_number: i32,
    value: f64,
    blank: bool,
    field_name: &str,
    owners: &mut SourceGainInputOwners<'_, '_>,
) -> Result<f64, E> {
    if blank {
        diagnostic(
            owners,
            &format!(
                "GetInternalHeatGains: People=\"{}\", specifies {}, but that field is blank.  0 People will result.",
                object.name, field_name
            ),
            false,
        )?;
        return Ok(0.0);
    }
    if (method == 1 && value < 0.0) || (method == 2 && value <= 0.0) {
        diagnostic(
            owners,
            &format!(
                "GetInternalHeatGains: People=\"{}\", invalid {}, value  [{}]={}",
                object.name,
                field_name,
                if method == 1 { "<0.0" } else { "<=0.0" },
                round(value, 3)?
            ),
            true,
        )?;
        *errors_found = true;
    }
    if *errors_found {
        return Ok(0.0);
    }
    if method == 0 {
        // The admitted direct Zone has exactly one real implicit Space.
        return Ok(value * 1.0);
    }
    let space = owners
        .zones
        .spaces
        .as_ref()
        .and_then(|rows| rows.get(index(space_number).ok()?))
        .ok_or_else(|| unavailable("actual People sizing Space area owner"))?;
    let area = space.floor_area;
    let remainder = space.is_remainder_space;
    let result = if method == 1 {
        value * area
    } else {
        area / value
    };
    if area <= 0.0 && !remainder {
        diagnostic(
            owners,
            &format!(
                "GetInternalHeatGains: People=\"{}\", specifies {}, but Space Floor Area = 0.  0 People will result.",
                object.name, field_name
            ),
            false,
        )?;
    }
    Ok(result)
}
impl SelectedInternalHeatGains {
    pub(super) fn read_people(
        &mut self,
        misc: &[SourceMiscGainObject],
        local: &mut SourceInputProcessorShortCut,
        owners: &mut SourceGainInputOwners<'_, '_>,
    ) -> Result<(), E> {
        let clock = owners
            .clock
            .ok_or_else(|| unavailable("actual positive People input clock"))?;
        let mut count = 0;
        for (ordinal, object) in misc.iter().enumerate() {
            let values =
                owners
                    .raw
                    .get_object_item::<21, 8>("People", ordinal, owners.input, local)?;
            local.alpha_args[..21].clone_from_slice(&values.alphas);
            local.numeric_args[..8].copy_from_slice(&values.numbers);
            let schedule = owners.schedules.get_source_schedule(&values.alphas[2])?;
            self.people_schedule_check(schedule, 2, &values.alphas, local, owners, false)?;
            let method = match values.alphas[3].as_str() {
                "PEOPLE" => 0,
                "PEOPLE/AREA" => 1,
                "AREA/PERSON" => 2,
                _ => return Err(unavailable("invalid People sizing method")),
            };
            let level_value = values.numbers[method];
            let level_blank = local.numeric_field_blanks[method];
            for (position, &space_number) in object.space_nums.iter().enumerate() {
                let zone_number = owners
                    .zones
                    .spaces
                    .as_ref()
                    .and_then(|rows| rows.get(index(space_number).ok()?))
                    .ok_or_else(|| unavailable("actual People Space association"))?
                    .zone_num;
                let row = self
                    .allocations
                    .people
                    .rows
                    .get_mut(count)
                    .ok_or_else(|| unavailable("actual allocated People row"))?;
                row.name = object.names[position].clone();
                row.space_index = space_number;
                row.zone_ptr = zone_number;
                row.sched = schedule;
                let number = design_level(
                    &mut self.errors_found,
                    object,
                    method,
                    space_number,
                    level_value,
                    level_blank,
                    &local.numeric_field_names[method],
                    owners,
                )?;
                row.number_of_people = number;
                let schedule = schedule.ok_or_else(|| {
                    unavailable("invalid People schedule before actual dereference")
                })?;
                row.nom_min_number_people =
                    number * owners.schedules.source_min_with_clock(schedule, clock)?;
                row.nom_max_number_people =
                    number * owners.schedules.source_max_with_clock(schedule, clock)?;
                let zone = owners
                    .zones
                    .zones
                    .as_mut()
                    .and_then(|rows| rows.get_mut(index(zone_number).ok()?))
                    .ok_or_else(|| unavailable("actual People Zone totals"))?;
                zone.total_occupants += number;
                zone.min_occupants += row.nom_min_number_people;
                zone.max_occupants += row.nom_max_number_people;
                let space = owners
                    .zones
                    .spaces
                    .as_mut()
                    .and_then(|rows| rows.get_mut(index(space_number).ok()?))
                    .ok_or_else(|| unavailable("actual People Space totals"))?;
                space.total_occupants += number;
                space.min_occupants += row.nom_min_number_people;
                space.max_occupants += row.nom_max_number_people;
                row.fraction_radiant = values.numbers[3];
                row.fraction_convected = 1.0 - row.fraction_radiant;
                if position == 0 && row.fraction_convected < 0.0 {
                    diagnostic(
                        owners,
                        &format!(
                            "GetInternalHeatGains: People=\"{}\", {} < 0.0, value ={}",
                            values.alphas[0],
                            local.numeric_field_names[3],
                            round(values.numbers[3], 2)?
                        ),
                        true,
                    )?;
                    self.errors_found = true;
                }
                row.user_spec_sens_frac =
                    if values.num_numbers >= 5 && !local.numeric_field_blanks[4] {
                        values.numbers[4]
                    } else {
                        -99999.0
                    };
                row.co2_rate_factor = if values.num_numbers >= 6 && !local.numeric_field_blanks[5] {
                    values.numbers[5]
                } else {
                    3.82e-8
                };
                row.cold_stress_temp_thresh =
                    if values.num_numbers >= 7 && !local.numeric_field_blanks[6] {
                        values.numbers[6]
                    } else {
                        15.56
                    };
                row.heat_stress_temp_thresh =
                    if values.num_numbers == 8 && !local.numeric_field_blanks[7] {
                        values.numbers[7]
                    } else {
                        30.0
                    };
                if row.co2_rate_factor < 0.0 {
                    diagnostic(
                        owners,
                        &format!(
                            "GetInternalHeatGains: People=\"{}\", {} < 0.0, value ={}",
                            values.alphas[0],
                            local.numeric_field_names[5],
                            round(values.numbers[5], 2)?
                        ),
                        true,
                    )?;
                    self.errors_found = true;
                }
                row.activity_level_sched =
                    owners.schedules.get_source_schedule(&values.alphas[4])?;
                let activity = row.activity_level_sched;
                if position == 0 {
                    self.people_schedule_check(activity, 4, &values.alphas, local, owners, true)?;
                }
                let row = self
                    .allocations
                    .people
                    .rows
                    .get_mut(count)
                    .ok_or_else(|| unavailable("same allocated People row"))?;
                if values.num_alphas >= 6 {
                    row.show55_warning = match values.alphas[5].as_str() {
                        "YES" => true,
                        "NO" => false,
                        _ => return Err(unavailable("People invalid ASHRAE55 warning key")),
                    };
                }
                // Native543-561 warns before model selection only when the
                // actual alpha count is below the first model slot. Later A21
                // can suppress this warning even when the same schedules exist.
                // No lookup, ctor-field assignment or ErrorsFound write follows
                // these auxiliary operands unless a model is selected.
                if values.num_alphas > 6 {
                    if values.num_alphas < 14
                        && [8, 11, 12].iter().any(|&i| !local.alpha_field_blanks[i])
                    {
                        diagnostic(
                            owners,
                            &format!(
                                "GetInternalHeatGains: People=\"{}\" has comfort related schedules but no thermal comfort model selected.",
                                values.alphas[0]
                            ),
                            false,
                        )?;
                        owners.backend.show_continue_error(
                            "If schedules are specified for air velocity, clothing insulation, and/or work efficiency but no thermal comfort",
                        )?;
                        owners.backend.show_continue_error(
                            "thermal comfort model is selected, the schedules will be listed as unused schedules in the .err file.",
                        )?;
                        owners.backend.show_continue_error(
                            "To avoid these errors, select a valid thermal comfort model or eliminate these schedules in the PEOPLE input.",
                        )?;
                    }
                    for model in values
                        .alphas
                        .iter()
                        .take(values.num_alphas.min(20))
                        .skip(13)
                    {
                        if model == "FANGER" {
                            row.fanger = true;
                        } else if !model.is_empty() {
                            return Err(unavailable("positive unsupported People comfort model"));
                        }
                    }
                    if row.fanger {
                        self.read_fanger_comfort(
                            count,
                            position == 0,
                            &values.alphas,
                            local,
                            owners,
                        )?;
                    }
                }
                count += 1;
            }
        }
        // Native820 installs all devices only AFTER every People row read.
        for person in 0..self.allocations.people.rows.len() {
            if owners.globals.any_energy_management_system_in_model {
                return Err(unavailable("positive People EMS registration"));
            }
            if !self.errors_found {
                let space = self.allocations.people.rows[person].space_index;
                let flags = SourceDiagnosticFlags {
                    kickoff: &owners.globals.kickoff_simulation,
                    developer: owners.developer,
                    warmup: &clock.warmup_flag,
                    doing_sizing: &owners.globals.doing_sizing,
                };
                self.setup_people_device(space, person, owners.backend, &flags)?;
            }
        }
        for zone_index in 0..owners
            .zones
            .zones
            .as_ref()
            .ok_or_else(|| unavailable("People nominal Zone rows"))?
            .len()
        {
            let zone = &owners
                .zones
                .zones
                .as_ref()
                .ok_or_else(|| unavailable("People nominal Zone rows"))?[zone_index];
            let total = zone.total_occupants;
            let area = zone.floor_area;
            let name = zone.name.clone();
            if total > 0.0 {
                if area > 0.0 && area / total < 0.1 {
                    diagnostic(
                        owners,
                        &format!(
                            "GetInternalHeatGains: Zone=\"{name}\" occupant density is extremely high."
                        ),
                        false,
                    )?;
                    owners.backend.show_continue_error(&format!(
                        "Occupant Density=[{}] person/m2.",
                        round(total / area, 0)?
                    ))?;
                    owners.backend.show_continue_error(&format!("Occupant Density=[{}] m2/person. Problems in Temperature Out of Bounds may result.",round(area/total,3)?))?;
                }
                let mut maximum = 0.0;
                let mut option = None;
                for (person, row) in self.allocations.people.rows.iter().enumerate() {
                    if index(row.zone_ptr)? != zone_index {
                        continue;
                    }
                    let current = owners.schedules.source_current_schedule_value(
                        row.sched
                            .ok_or_else(|| unavailable("same People occupancy schedule"))?,
                    )?;
                    if maximum < current * row.number_of_people {
                        maximum = current * row.number_of_people;
                        option = Some(person);
                    }
                }
                if maximum > total && area > 0.0 && area / maximum < 0.1 {
                    diagnostic(
                        owners,
                        &format!(
                            "GetInternalHeatGains: Zone=\"{name}\" occupant density at a maximum schedule value is extremely high."
                        ),
                        false,
                    )?;
                    owners.backend.show_continue_error(&format!(
                        "Occupant Density=[{}] person/m2.",
                        round(maximum / area, 0)?
                    ))?;
                    owners.backend.show_continue_error(&format!("Occupant Density=[{}] m2/person. Problems in Temperature Out of Bounds may result.",round(area/maximum,3)?))?;
                    let row = &self.allocations.people.rows
                        [option.ok_or_else(|| unavailable("actual maximum occupant row"))?];
                    owners.backend.show_continue_error(&format!(
                        "Check values in People={}, Number of People Schedule={}",
                        row.name,
                        owners.schedules.source_current_schedule_value(
                            row.sched
                                .ok_or_else(|| unavailable("same People occupancy schedule"))?
                        )?
                    ))?;
                }
            }
            let zone = &mut owners
                .zones
                .zones
                .as_mut()
                .ok_or_else(|| unavailable("same People nominal Zone rows"))?[zone_index];
            if zone.is_nominal_controlled && total > 0.0 {
                zone.is_nominal_occupied = true;
                for column in ["pdchOaoNomNumOcc1", "pdchOaoNomNumOcc2"] {
                    let id =
                        *owners.registry.named_indices.get(column).ok_or_else(|| {
                            unavailable("actual People predefined occupant column")
                        })?;
                    owners.table.fixed_real_entry_2(id, &name, total)?;
                }
            }
        }
        Ok(())
    }
    /// Actual IHG617-810 additional-input branch for supported Fanger clothing.
    /// Defaults and blank flags come from the genuine schema/IP getter.
    fn read_fanger_comfort(
        &mut self,
        person: usize,
        first_space: bool,
        alphas: &[String; 21],
        local: &SourceInputProcessorShortCut,
        owners: &mut SourceGainInputOwners<'_, '_>,
    ) -> Result<(), E> {
        let clock = owners
            .clock
            .ok_or_else(|| unavailable("actual Fanger input clock"))?;
        let row = self
            .allocations
            .people
            .rows
            .get_mut(person)
            .ok_or_else(|| unavailable("actual Fanger People row"))?;
        row.mrt_calc_type = match alphas[6].as_str() {
            "ENCLOSUREAVERAGED" => 0,
            "SURFACEWEIGHTED" => 1,
            "ANGLEFACTOR" => 2,
            _ => -1,
        };
        if row.mrt_calc_type != 0 {
            return Err(unavailable(
                "positive Fanger non-EnclosureAveraged MRT inputs",
            ));
        }
        if !local.alpha_field_blanks[8] {
            row.work_eff_sched = owners.schedules.get_source_schedule(&alphas[8])?;
        }
        let work = row.work_eff_sched;
        if first_space {
            let message = if local.alpha_field_blanks[8] {
                Some(format!("{} cannot be empty.", local.alpha_field_names[8]))
            } else if work.is_none() {
                Some(format!(
                    "{} = {}, item not found.",
                    local.alpha_field_names[8], alphas[8]
                ))
            } else if !owners.schedules.source_check_min_max_inclusive(
                work.ok_or_else(|| unavailable("actual Fanger work schedule"))?,
                0.0,
                1.0,
                clock,
            )? {
                Some(format!(
                    "{} = {}, schedule contains values that are < 0 and/or > 1",
                    local.alpha_field_names[8], alphas[8]
                ))
            } else {
                None
            };
            if let Some(message) = message {
                self.people_comfort_diagnostic(alphas, &message,
                    local.alpha_field_blanks[8].then_some("It is required when Thermal Comfort Model Type is one of \"Fanger\", \"Pierce\", \"KSU\", \"CoolingEffectASH55\" or \"AnkleDraftASH55\""),
                    true, owners)?;
            }
        }
        let row = self
            .allocations
            .people
            .rows
            .get_mut(person)
            .ok_or_else(|| unavailable("same Fanger clothing People row"))?;
        if !alphas[9].is_empty() {
            row.clothing_type = match alphas[9].as_str() {
                "CLOTHINGINSULATIONSCHEDULE" => 0,
                "DYNAMICCLOTHINGMODELASHRAE55" => 1,
                "CALCULATIONMETHODSCHEDULE" => 2,
                _ => -1,
            };
            match row.clothing_type {
                0 => {
                    row.clothing_sched = owners.schedules.get_source_schedule(&alphas[11])?;
                    let clothing = row.clothing_sched;
                    if first_space {
                        let message = if local.alpha_field_blanks[11] {
                            Some(format!(
                                "{} cannot be empty when {} = {}.",
                                local.alpha_field_names[11], local.alpha_field_names[9], alphas[9]
                            ))
                        } else if clothing.is_none() {
                            Some(format!(
                                "{} = {}, item not found.",
                                local.alpha_field_names[11], alphas[11]
                            ))
                        } else if !owners.schedules.source_check_min_with_clock(
                            clothing
                                .ok_or_else(|| unavailable("actual Fanger clothing schedule"))?,
                            0.0,
                            clock,
                        )? {
                            Some(format!(
                                "{} = {}, schedule contains values that are < 0",
                                local.alpha_field_names[11], alphas[11]
                            ))
                        } else {
                            None
                        };
                        if let Some(message) = message {
                            self.people_comfort_diagnostic(alphas, &message, None, true, owners)?;
                        } else {
                            let max_ok = owners.schedules.source_max_with_clock(
                                clothing
                                    .ok_or_else(|| unavailable("actual Fanger clothing maximum"))?,
                                clock,
                            )? - 2.0
                                <= f64::from(f32::EPSILON);
                            if !max_ok {
                                let message = format!(
                                    "{} = {}, schedule contains values that are > 2",
                                    local.alpha_field_names[11], alphas[11]
                                );
                                self.people_comfort_diagnostic(
                                    alphas, &message, None, false, owners,
                                )?;
                            }
                        }
                    }
                }
                1 => {} // Native DynamicAshrae55 consumes no A11/A12 here.
                2 => {
                    row.clothing_method_sched =
                        owners.schedules.get_source_schedule(&alphas[10])?;
                    let method = row.clothing_method_sched;
                    if first_space && method.is_none() {
                        let message = format!(
                            "{} = {}, item not found.",
                            local.alpha_field_names[10], alphas[10]
                        );
                        self.people_comfort_diagnostic(alphas, &message, None, true, owners)?;
                    }
                    // Native next dereferences the method pointer. A null one
                    // is an explicit undefined-owner boundary after its Severe.
                    let method = method.ok_or_else(|| {
                        unavailable("actual Fanger clothing-method pointer before hasVal")
                    })?;
                    if owners.schedules.source_has_value(method, 1.0, clock)? {
                        let row = self
                            .allocations
                            .people
                            .rows
                            .get_mut(person)
                            .ok_or_else(|| unavailable("same Fanger selected clothing row"))?;
                        row.clothing_sched = owners.schedules.get_source_schedule(&alphas[11])?;
                        if first_space && row.clothing_sched.is_none() {
                            let message = format!(
                                "{} = {}, item not found.",
                                local.alpha_field_names[11], alphas[11]
                            );
                            self.people_comfort_diagnostic(alphas, &message, None, true, owners)?;
                        }
                    }
                    // Native adds no method-range or clothing min/max check.
                }
                _ => return Err(unavailable("invalid Fanger clothing input key")),
            }
        }
        let row = self
            .allocations
            .people
            .rows
            .get_mut(person)
            .ok_or_else(|| unavailable("same Fanger air-speed People row"))?;
        if !local.alpha_field_blanks[12] {
            row.air_velocity_sched = owners.schedules.get_source_schedule(&alphas[12])?;
        }
        let velocity = row.air_velocity_sched;
        if first_space {
            let message = if local.alpha_field_blanks[12] {
                Some(format!("{} cannot be empty.", local.alpha_field_names[12]))
            } else if velocity.is_none() {
                Some(format!(
                    "{} = {}, item not found.",
                    local.alpha_field_names[12], alphas[12]
                ))
            } else if !owners.schedules.source_check_min_with_clock(
                velocity.ok_or_else(|| unavailable("actual Fanger air velocity schedule"))?,
                0.0,
                clock,
            )? {
                Some(format!(
                    "{} = {}, schedule contains values that are < 0",
                    local.alpha_field_names[12], alphas[12]
                ))
            } else {
                None
            };
            if let Some(message) = message {
                self.people_comfort_diagnostic(alphas, &message,
                    local.alpha_field_blanks[12].then_some("Required when Thermal Comfort Model Type is one of \"Fanger\", \"Pierce\", \"KSU\", \"CoolingEffectASH55\" or \"AnkleDraftASH55\""),
                    true, owners)?;
            }
        }
        let row = self
            .allocations
            .people
            .rows
            .get_mut(person)
            .ok_or_else(|| unavailable("same Fanger ankle schedule People row"))?;
        if !alphas[20].is_empty() {
            row.ankle_air_velocity_sched = owners.schedules.get_source_schedule(&alphas[20])?;
        }
        // The selected AnkleDraft flag is false, but Native still validates a
        // nonblank A21 lookup. There is deliberately no minimum check here.
        if first_space && !local.alpha_field_blanks[20] && row.ankle_air_velocity_sched.is_none() {
            let message = format!(
                "{} = {}, item not found.",
                local.alpha_field_names[20], alphas[20]
            );
            self.people_comfort_diagnostic(alphas, &message, None, true, owners)?;
        }
        Ok(())
    }
    fn people_comfort_diagnostic(
        &mut self,
        alphas: &[String; 21],
        message: &str,
        followup: Option<&str>,
        severe: bool,
        owners: &mut SourceGainInputOwners<'_, '_>,
    ) -> Result<(), E> {
        diagnostic(
            owners,
            &format!("GetInternalHeatGains: People = {}", alphas[0]),
            severe,
        )?;
        owners.backend.show_continue_error(message)?;
        if let Some(followup) = followup {
            owners.backend.show_continue_error(followup)?;
        }
        if severe {
            self.errors_found = true;
        }
        Ok(())
    }
    fn people_schedule_check(
        &mut self,
        schedule: Option<SourceScheduleIndex>,
        field: usize,
        alphas: &[String; 21],
        local: &SourceInputProcessorShortCut,
        owners: &mut SourceGainInputOwners<'_, '_>,
        activity: bool,
    ) -> Result<(), E> {
        let clock = owners
            .clock
            .ok_or_else(|| unavailable("same People schedule clock"))?;
        let message = if local.alpha_field_blanks[field] {
            Some(format!(
                "{} cannot be empty.",
                local.alpha_field_names[field]
            ))
        } else if schedule.is_none() {
            Some(format!(
                "{} = {}, item not found.",
                local.alpha_field_names[field], alphas[field]
            ))
        } else if !owners.schedules.source_check_min_with_clock(
            schedule.ok_or_else(|| unavailable("actual People schedule"))?,
            0.0,
            clock,
        )? {
            Some(format!(
                "{} = {}, schedule contains values that are < 0",
                local.alpha_field_names[field], alphas[field]
            ))
        } else {
            None
        };
        if let Some(message) = message {
            diagnostic(
                owners,
                &format!("GetInternalHeatGains: People = {}", alphas[0]),
                true,
            )?;
            owners.backend.show_continue_error(&message)?;
            self.errors_found = true;
        } else if activity
            && !owners.schedules.source_check_min_max_inclusive(
                schedule.ok_or_else(|| unavailable("actual activity schedule"))?,
                70.0,
                1000.0,
                clock,
            )?
        {
            diagnostic(
                owners,
                &format!("GetInternalHeatGains: People = {}", alphas[0]),
                false,
            )?;
            owners.backend.show_continue_error(&format!(
                "{} = {}, schedule contains values that are < 70 and/or > 1000",
                local.alpha_field_names[field], alphas[field]
            ))?;
            owners.backend.show_continue_error(
                "Values fall outside of typical w/person range for thermal comfort reporting.",
            )?;
        }
        Ok(())
    }
}
