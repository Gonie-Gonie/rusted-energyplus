//! Genuine finite positive regular Thermostat input, Native332-807.
//! Later environment/current-setpoint/predefined bodies keep their own bounds.
use super::*;
use super::super::controls::SourceTemperatureControlledZone;
mod getter;

/// Exact regular NeededControlTypes local290-296; this pinned regular path has
/// no MustHave/DidHave writers. The comfort local is a distinct Native owner.
#[derive(Clone, Default)]
struct SourceNeededControlTypes {
    must_have: [bool; 5],
    did_have: [bool; 5],
}
#[derive(Default)]
pub(super) struct SourceThermostatInputLocals {
    control_types: Option<Vec<SourceNeededControlTypes>>,
    control_schedule_map_to_zone: Option<Vec<i32>>,
}

fn control_kind(name: &str) -> Option<usize> {
    match name {
        "UNCONTROLLED" => Some(0),
        "THERMOSTATSETPOINT:SINGLEHEATING" => Some(1),
        "THERMOSTATSETPOINT:SINGLECOOLING" => Some(2),
        "THERMOSTATSETPOINT:SINGLEHEATINGORCOOLING" => Some(3),
        "THERMOSTATSETPOINT:DUALSETPOINT" => Some(4),
        _ => None,
    }
}
fn position(number: i32) -> Result<usize, Error> {
    usize::try_from(number).ok().and_then(|number| number.checked_sub(1))
        .ok_or(Error::Unavailable("actual thermostat positive native index"))
}

impl SourceZoneAirPredictorCorrector {
    pub(super) fn get_source_thermostat_objects(
        &mut self, owners: &mut SourceZoneAirOwners<'_, '_>, locals: &mut SourceThermostatInputLocals,
    ) -> Result<(), Error> {
        // Native prescan329-370: same TStat array was allocated by the parent.
        for item in 1..=self.controls.number_thermostat_statements {
            let _ = getter::thermostat(owners.input, owners.shortcut, item, *owners.source_is_epjson)?;
            let tstat = self.controls.thermostat_objects.as_mut().and_then(|rows| rows.get_mut(item - 1))
                .ok_or(Error::Unavailable("actual TStatObjects prescan allocation"))?;
            tstat.name = owners.shortcut.alpha_args[0].clone();
            let zone_at = owners.zones.zones.as_ref()
                .ok_or(Error::Unavailable("actual Zone array in thermostat prescan"))?
                .iter().position(|row| row.name == owners.shortcut.alpha_args[1]);
            if let Some(zone_at) = zone_at {
                tstat.temperature_zone_start = self.controls.number_temperature_zones.checked_add(1)
                    .ok_or(Error::Unavailable("actual thermostat start native int domain"))?;
                self.controls.number_temperature_zones = self.controls.number_temperature_zones.checked_add(1)
                    .ok_or(Error::Unavailable("actual thermostat zone-count native int domain"))?;
                tstat.number_zones = 1;
                tstat.zone_list_active = false;
                tstat.zone_or_list_pointer = i32::try_from(zone_at + 1)
                    .map_err(|_| Error::Unavailable("actual thermostat Zone pointer native int domain"))?;
            } else if owners.zones.num_zone_lists > 0 {
                return Err(Error::Unavailable("actual thermostat prescan ZoneList FindItem351"));
            } else {
                return Err(Error::Unavailable("actual thermostat prescan ShowSevereItemNotFound367"));
            }
        }
        if self.controls.number_temperature_zones > 0 {
            let count = usize::try_from(self.controls.number_temperature_zones)
                .map_err(|_| Error::Unavailable("actual NumTempControlledZones allocation domain"))?;
            if self.controls.temperature_controlled_zones.is_some() {
                return Err(Error::Unavailable("repeated actual TempControlledZone allocation379"));
            }
            self.controls.temperature_controlled_zones = Some(vec![SourceTemperatureControlledZone::default(); count]);
            locals.control_types = Some(vec![SourceNeededControlTypes::default(); count]);
            locals.control_schedule_map_to_zone = Some(vec![0; count]);
            let mut controlled_zone_number: i32 = 0;
            self.flags.number_on_off_zones = 0;
            for item in 1..=self.controls.number_thermostat_statements {
                let (mut num_alphas, num_numbers) = getter::thermostat(
                    owners.input, owners.shortcut, item, *owners.source_is_epjson)?;
                let (number_zones, zone_list_active) = self.controls.thermostat_objects.as_ref()
                    .and_then(|rows| rows.get(item - 1)).map(|row| (row.number_zones, row.zone_list_active))
                    .ok_or(Error::Unavailable("actual TStatObjects second-pass association"))?;
                for item_zone in 1..=number_zones {
                    controlled_zone_number = controlled_zone_number.checked_add(1)
                        .ok_or(Error::Unavailable("actual thermostat second-pass native int domain"))?;
                    let at = position(controlled_zone_number)?;
                    if zone_list_active {
                        return Err(Error::Unavailable("actual thermostat ZoneList alpha rewrite406"));
                    }
                    let rows = self.controls.temperature_controlled_zones.as_mut()
                        .ok_or(Error::Unavailable("actual TempControlledZone second-pass array"))?;
                    let assigned = rows.get(..at)
                        .ok_or(Error::Unavailable("actual thermostat preceding Zone count"))?
                        .iter().position(|row| row.zone_name == owners.shortcut.alpha_args[1]);
                    let row = rows.get_mut(at).ok_or(Error::Unavailable("actual TempControlledZone second-pass row"))?;
                    row.zone_name = owners.shortcut.alpha_args[1].clone();
                    if assigned.is_some() {
                        return Err(Error::Unavailable("actual thermostat ShowSevereDuplicateAssignment423"));
                    }
                    let actual_zone = owners.zones.zones.as_ref()
                        .ok_or(Error::Unavailable("actual thermostat Zone lookup array"))?
                        .iter().position(|zone| zone.name == owners.shortcut.alpha_args[1]);
                    row.actual_zone_number = match actual_zone {
                        Some(at) => i32::try_from(at + 1)
                            .map_err(|_| Error::Unavailable("actual thermostat actual-Zone integer domain"))?,
                        None => 0, // Actual FindItemInList missing-name return.
                    };
                    if row.actual_zone_number == 0 {
                        return Err(Error::Unavailable("actual thermostat ShowSevereItemNotFound416"));
                    }
                    owners.zones.zones.as_mut().and_then(|zones| zones.get_mut(position(row.actual_zone_number).ok()?))
                        .ok_or(Error::Unavailable("actual thermostat same Zone index writer419"))?
                        .temperature_controlled_zone_index = controlled_zone_number;
                    row.name = owners.shortcut.alpha_args[0].clone();
                    row.control_type_schedule = owners.schedules.get_source_schedule(&owners.shortcut.alpha_args[2])?;
                    if item_zone == 1 {
                        let schedule = row.control_type_schedule
                            .ok_or(Error::Unavailable("actual thermostat control schedule ShowSevereItemNotFound451"))?;
                        if !owners.schedules.source_check_min_max_inclusive(schedule, 0.0, 4.0)? {
                            return Err(Error::Unavailable("actual thermostat control schedule ShowSevereBadMinMax454"));
                        }
                    }
                    if owners.shortcut.alpha_field_blanks[6] {
                        num_alphas = 5;
                    } else if owners.shortcut.alpha_field_blanks[8] {
                        num_alphas = 7;
                    } else if owners.shortcut.alpha_field_blanks[10] {
                        num_alphas = 9;
                    }
                    // Literal Native nint((NumAlphas - 3.0) / 2.0), using
                    // ObjexxFCL Fmath.hh588-591 sign*0.5 then int truncation.
                    let setpoint_type_count = (f64::from(num_alphas) - 3.0) / 2.0;
                    let sign = if setpoint_type_count >= 0.0 { 1.0 } else { -1.0 };
                    let setpoint_type_count = (setpoint_type_count + sign * 0.5) as i32;
                    for item_setpoint in 1..=setpoint_type_count {
                        let alpha_number = 2 * item_setpoint - 1 + 3;
                        let alpha_at = position(alpha_number)?;
                        if *owners.shortcut.alpha_field_blanks.get(alpha_at)
                            .ok_or(Error::Unavailable("actual thermostat setpoint alpha slot"))?
                        {
                            return Err(Error::Unavailable("actual thermostat ShowSevereEmptyField476"));
                        }
                        let kind = owners.shortcut.alpha_args.get(alpha_at).and_then(|name| control_kind(name))
                            .ok_or(Error::Unavailable("actual thermostat ShowSevereInvalidKey482"))?;
                        row.setpoints[kind].name = owners.shortcut.alpha_args.get(alpha_at + 1)
                            .ok_or(Error::Unavailable("actual thermostat setpoint name slot"))?.clone();
                        row.setpoints[kind].is_used = true;
                    }
                    if num_numbers > 0 {
                        if owners.shortcut.numeric_args[0] >= 0.0 {
                            row.delta_temperature_cutout_setpoint = owners.shortcut.numeric_args[0];
                            if owners.shortcut.numeric_args[0] > 0.0 {
                                self.flags.number_on_off_zones = self.flags.number_on_off_zones.checked_add(1)
                                    .ok_or(Error::Unavailable("actual NumOnOffCtrZone int domain"))?;
                            }
                        } else {
                            return Err(Error::Unavailable("actual thermostat negative cutout ShowSevereError498"));
                        }
                    }
                    if row.delta_temperature_cutout_setpoint > 0.0 && !row.setpoints[3].name.is_empty() {
                        return Err(Error::Unavailable("actual thermostat SingleHeatCool cutout ShowWarningError510"));
                    }
                }
            }
        }
        Ok(())
    }

    pub(super) fn get_source_dual_setpoint_schedules(
        &mut self, owners: &mut SourceZoneAirOwners<'_, '_>,
    ) -> Result<(), Error> {
        for number in 1..=self.flags.number_temperature_controls[4] {
            getter::dual(owners.input, owners.shortcut, number, *owners.source_is_epjson)?;
            let row = self.flags.temperature_setpoint_schedules[4].as_mut()
                .and_then(|rows| rows.get_mut(number - 1))
                .ok_or(Error::Unavailable("actual Dual tempSetptScheds row652"))?;
            row.name = owners.shortcut.alpha_args[0].clone();
            if owners.shortcut.alpha_field_blanks[1] {
                return Err(Error::Unavailable("actual Dual heat ShowSevereEmptyField656"));
            }
            row.heat_schedule = owners.schedules.get_source_schedule(&owners.shortcut.alpha_args[1])?;
            if row.heat_schedule.is_none() {
                return Err(Error::Unavailable("actual Dual heat ShowSevereItemNotFound659"));
            }
            if owners.shortcut.alpha_field_blanks[2] {
                return Err(Error::Unavailable("actual Dual cool ShowSevereEmptyField664"));
            }
            row.cool_schedule = owners.schedules.get_source_schedule(&owners.shortcut.alpha_args[2])?;
            if row.cool_schedule.is_none() {
                return Err(Error::Unavailable("actual Dual cool ShowSevereItemNotFound667"));
            }
        }
        Ok(())
    }

    pub(super) fn link_source_thermostat_schedules(
        &mut self, owners: &mut SourceZoneAirOwners<'_, '_>, locals: &mut SourceThermostatInputLocals,
    ) -> Result<(), Error> {
        for number in 1..=self.controls.number_temperature_zones {
            let row = self.controls.temperature_controlled_zones.as_mut()
                .and_then(|rows| rows.get_mut(position(number).ok()?))
                .ok_or(Error::Unavailable("actual thermostat link TempControlledZone675"))?;
            for kind in 1..5 {
                if !row.setpoints[kind].is_used { continue; }
                let setpoint = self.flags.temperature_setpoint_schedules[kind].as_ref()
                    .and_then(|rows| rows.iter().find(|setpoint| setpoint.name == row.setpoints[kind].name)
                        .or_else(|| rows.iter().find(|setpoint| setpoint.name.eq_ignore_ascii_case(&row.setpoints[kind].name))))
                    .ok_or(Error::Unavailable("actual thermostat missing setpoint ShowSevereError685"))?;
                if kind == 1 || kind == 3 || kind == 4 { row.setpoints[kind].heat_schedule = setpoint.heat_schedule; }
                if kind == 2 || kind == 3 || kind == 4 { row.setpoints[kind].cool_schedule = setpoint.cool_schedule; }
            }
        }
        for number in 1..=self.controls.number_temperature_zones {
            let row = self.controls.temperature_controlled_zones.as_ref()
                .and_then(|rows| rows.get(position(number).ok()?))
                .ok_or(Error::Unavailable("actual thermostat schedule-validation row715"))?;
            let Some(schedule) = row.control_type_schedule else { continue; };
            let minimum = owners.schedules.source_min(schedule)?;
            let maximum = owners.schedules.source_max(schedule)?;
            // Prior real inclusive0..4 validation establishes this defined int
            // conversion domain; do not replace these casts by rounded values.
            let schedule_minimum = minimum as i32;
            let schedule_maximum = maximum as i32;
            if schedule_minimum == 0 && schedule_maximum == 0 {
                let native_number = owners.schedules.schedules.get(schedule.0)
                    .ok_or(Error::Unavailable("actual thermostat schedule Num pointer"))?.number.0;
                let schedule_number = i32::try_from(native_number)
                    .map_err(|_| Error::Unavailable("actual thermostat schedule Num integer domain"))?;
                let map = locals.control_schedule_map_to_zone.as_ref()
                    .ok_or(Error::Unavailable("actual CTSchedMapToControlledZone local"))?;
                if !map.contains(&schedule_number) {
                    return Err(Error::Unavailable("actual thermostat all-zero schedule ShowSevereError726"));
                }
                *locals.control_schedule_map_to_zone.as_mut()
                    .and_then(|map| map.get_mut(position(number).ok()?))
                    .ok_or(Error::Unavailable("actual CTSchedMapToControlledZone row730"))? = schedule_number;
            }
            for kind in 1..5 {
                let setpoint = &row.setpoints[kind];
                if !setpoint.is_used {
                    if owners.schedules.source_has_value(schedule, kind as f64)? {
                        return Err(Error::Unavailable("actual unused thermostat control ShowSevereError739"));
                    }
                    continue;
                }
                if setpoint.heat_schedule.is_none() && (kind == 1 || kind == 3 || kind == 4)
                    && owners.schedules.source_has_value(schedule, kind as f64)?
                {
                    return Err(Error::Unavailable("actual thermostat missing heat ShowSevereError755"));
                }
                if setpoint.cool_schedule.is_none() && (kind == 2 || kind == 3 || kind == 4)
                    && owners.schedules.source_has_value(schedule, kind as f64)?
                {
                    return Err(Error::Unavailable("actual thermostat missing cool ShowSevereError769"));
                }
            }
        }
        for number in 1..=self.controls.number_temperature_zones {
            let at = position(number)?;
            let row = self.controls.temperature_controlled_zones.as_ref().and_then(|rows| rows.get(at))
                .ok_or(Error::Unavailable("actual thermostat NeededControlTypes row782"))?;
            if row.control_type_schedule.is_none() { continue; }
            let needed = locals.control_types.as_ref().and_then(|rows| rows.get(at))
                .ok_or(Error::Unavailable("actual regular NeededControlTypes local row"))?;
            for kind in 1..5 {
                if needed.must_have[kind] && needed.did_have[kind] { continue; }
                if !needed.must_have[kind] { continue; }
                return Err(Error::Unavailable("actual thermostat missing control ShowWarningError798"));
            }
        }
        if locals.control_types.is_some() { locals.control_types = None; }
        Ok(())
    }
}
