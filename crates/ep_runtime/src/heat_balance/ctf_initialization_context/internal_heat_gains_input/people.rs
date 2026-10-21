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
            if values.alphas[3] != "PEOPLE" {
                return Err(unavailable("positive People density sizing method"));
            }
            let level_value = values.numbers[0];
            let level_blank = local.numeric_field_blanks[0];
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
                let mut number = 0.0;
                if level_blank {
                    diagnostic(
                        owners,
                        &format!(
                            "GetInternalHeatGains: People=\"{}\", specifies {}, but that field is blank.  0 People will result.",
                            object.name, local.numeric_field_names[0]
                        ),
                        false,
                    )?;
                } else if !self.errors_found {
                    number = level_value * 1.0;
                }
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
                if row.user_spec_sens_frac == -99999.0 {
                    return Err(unavailable(
                        "actual People automatic sensible gain temperature operands",
                    ));
                }
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
                    if values.num_alphas >= 14
                        && values.alphas[13..values.num_alphas.min(20)]
                            .iter()
                            .any(|x| !x.is_empty())
                    {
                        return Err(unavailable("positive optional People comfort model"));
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
