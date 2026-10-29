//! Literal reached nominal EIO and Zone.InternalHeatGains writer.
use super::getter::SourceGainInputOwners;
use super::{E, SelectedInternalHeatGains, index, unavailable};
use crate::heat_balance::selected_schedule_manager::{
    SelectedScheduleManager, SourceScheduleIndex,
};
use std::io::Write;

fn rounded(value: f64, precision: usize) -> Result<String, E> {
    crate::heat_balance::source_round_format::round(value, precision)
        .map_err(|e| E::Format(format!("InternalHeatGains .{precision}R: {e:?}")))
}
fn divide(eio: &mut dyn Write, numerator: f64, denominator: f64) -> Result<(), E> {
    if denominator > 0.0 {
        write!(eio, "{},", rounded(numerator / denominator, 3)?)?;
    } else {
        write!(eio, "N/A,")?;
    }
    Ok(())
}
fn optional_schedule_name(
    schedules: &SelectedScheduleManager,
    pointer: Option<SourceScheduleIndex>,
) -> Result<&str, E> {
    match pointer {
        None => Ok(""),
        Some(pointer) => schedules
            .schedules
            .get(pointer.0)
            .map(|row| row.name.as_str())
            .ok_or_else(|| unavailable("same optional People nominal schedule owner")),
    }
}

impl SelectedInternalHeatGains {
    pub(super) fn write_nominal(
        &self,
        owners: &mut SourceGainInputOwners<'_, '_>,
    ) -> Result<(), E> {
        writeln!(
            owners.eio,
            "! <Zone Internal Gains Nominal>,Zone Name, Floor Area {{m2}},# Occupants,Area per Occupant {{m2/person}},Occupant per Area {{person/m2}},Interior Lighting {{W/m2}},Electric Load {{W/m2}},Gas Load {{W/m2}},Other Load {{W/m2}},Hot Water Eq {{W/m2}},Steam Equipment {{W/m2}},Sum Loads per Area {{W/m2}},Outdoor Controlled Baseboard Heat"
        )?;
        let zones = owners
            .zones
            .zones
            .as_mut()
            .ok_or_else(|| unavailable("actual nominal Zones"))?;
        for (number, zone) in zones.iter_mut().enumerate() {
            let mut electric_total = 0.0;
            if let Some(rows) = &self.allocations.electric_equipment {
                for row in rows {
                    if index(row.zone_ptr)? == number {
                        electric_total += row.design_level;
                    }
                }
            }
            let mut other_total = 0.0;
            if let Some(rows) = &self.allocations.other_equipment {
                for row in rows {
                    if index(row.zone_ptr)? == number {
                        other_total += row.design_level;
                    }
                }
            }
            // Zero families retain their literal local totals; preserve Source
            // left association instead of substituting a typed gain result.
            zone.internal_heat_gains = 0.0 + electric_total + 0.0 + other_total + 0.0 + 0.0;
            write!(
                owners.eio,
                " Zone Internal Gains Nominal, {},{},{},",
                zone.name,
                rounded(zone.floor_area, 2)?,
                rounded(zone.total_occupants, 1)?
            )?;
            if zone.floor_area > 0.0 {
                divide(owners.eio, zone.floor_area, zone.total_occupants)?;
                write!(
                    owners.eio,
                    "{},",
                    rounded(zone.total_occupants / zone.floor_area, 3)?
                )?;
                for value in [0.0, electric_total, 0.0, other_total, 0.0, 0.0] {
                    write!(owners.eio, "{},", rounded(value / zone.floor_area, 3)?)?;
                }
                writeln!(
                    owners.eio,
                    "{},No",
                    rounded(zone.internal_heat_gains / zone.floor_area, 3)?
                )?;
            } else {
                writeln!(owners.eio, "0.0,N/A,N/A,N/A,N/A,N/A,N/A,N/A,N/A,No")?;
            }
        }
        for people_number in 1..=self.totals[0] {
            let number = index(people_number)?;
            let people = self
                .allocations
                .people
                .get(number)
                .ok_or_else(|| unavailable("actual People nominal row2715"))?;
            if number == 0 {
                write!(
                    owners.eio,
                    "! <People Internal Gains Nominal>,Name,Schedule Name,Zone Name,Zone Floor Area {{m2}},# Zone Occupants,Number of People {{}},People/Floor Area {{person/m2}},Floor Area per person {{m2/person}},Fraction Radiant,Fraction Convected,Sensible Fraction Calculation,Activity level,ASHRAE 55 Warnings,Carbon Dioxide Generation Rate,Minimum Number of People for All Day Types,Maximum Number of People for All Day Types,Minimum Number of People for Weekdays, Maximum Number of People for Weekdays, Minimum Number of People for Weekends/Holidays, Maximum Number of People for Weekends /Holidays,Minimum Number of People for Summer Design Days, Maximum Number of People for Summer Design Days,Minimum Number of People for Winter Design Days, Maximum Number of People for Winter Design Days"
                )?;
                if people.fanger
                    || people.pierce
                    || people.ksu
                    || people.cooling_effect_ash55
                    || people.ankle_draft_ash55
                {
                    writeln!(
                        owners.eio,
                        ",MRT Calculation Type,Work Efficiency, Clothing Insulation Calculation Method,Clothing Insulation Calculation Method Schedule,Clothing,Air Velocity,Fanger Calculation,Pierce Calculation,KSU Calculation,Cooling Effect Calculation,Ankle Draft Calculation"
                    )?;
                } else {
                    writeln!(owners.eio)?;
                }
            }
            if people.zone_ptr == 0 {
                writeln!(
                    owners.eio,
                    " People-Illegal Zone specified, {}",
                    people.name
                )?;
                continue;
            }
            let zone = zones
                .get(index(people.zone_ptr)?)
                .ok_or_else(|| unavailable("actual People nominal Zone"))?;
            let schedule = people
                .sched
                .ok_or_else(|| unavailable("actual People nominal occupancy schedule"))?;
            let schedule_name = owners
                .schedules
                .schedules
                .get(schedule.0)
                .ok_or_else(|| unavailable("actual People nominal schedule owner"))?
                .name
                .as_str();
            write!(
                owners.eio,
                " People Internal Gains Nominal, {},{},{},{},{},",
                people.name,
                schedule_name,
                zone.name,
                rounded(zone.floor_area, 2)?,
                rounded(zone.total_occupants, 1)?
            )?;
            write!(owners.eio, "{},", rounded(people.number_of_people, 1)?)?;
            divide(owners.eio, people.number_of_people, zone.floor_area)?;
            if people.number_of_people > 0.0 {
                divide(owners.eio, zone.floor_area, people.number_of_people)?;
            } else {
                write!(owners.eio, "N/A,")?;
            }
            write!(
                owners.eio,
                "{},{},",
                rounded(people.fraction_radiant, 3)?,
                rounded(people.fraction_convected, 3)?
            )?;
            if people.user_spec_sens_frac == -99999.0 {
                // Native Constant::AutoCalculate.
                write!(owners.eio, "AutoCalculate,")?;
            } else {
                write!(owners.eio, "{},", rounded(people.user_spec_sens_frac, 3)?)?;
            }
            let activity = people
                .activity_level_sched
                .ok_or_else(|| unavailable("actual People nominal activity schedule"))?;
            let activity_name = owners
                .schedules
                .schedules
                .get(activity.0)
                .ok_or_else(|| unavailable("actual People nominal activity schedule owner"))?
                .name
                .as_str();
            write!(
                owners.eio,
                "{},{},{},{},{},",
                activity_name,
                if people.show55_warning { "Yes" } else { "No" },
                rounded(people.co2_rate_factor, 4)?,
                rounded(people.nom_min_number_people, 1)?,
                rounded(people.nom_max_number_people, 1)?
            )?;
            for group in 0..4 {
                let (minimum, maximum) = match owners.clock {
                    Some(clock) => owners
                        .schedules
                        .source_day_group_min_max_with_clock(schedule, group, clock)?,
                    None => owners.schedules.source_day_group_min_max(schedule, group)?,
                };
                write!(
                    owners.eio,
                    "{},{},",
                    rounded(people.number_of_people * minimum, 1)?,
                    rounded(people.number_of_people * maximum, 1)?
                )?;
            }
            if people.fanger
                || people.pierce
                || people.ksu
                || people.cooling_effect_ash55
                || people.ankle_draft_ash55
            {
                if !(people.fanger || people.ankle_draft_ash55)
                    || people.pierce
                    || people.ksu
                    || people.cooling_effect_ash55
                {
                    return Err(unavailable(
                        "actual unsupported People nominal MRT/work/clothing branch2793",
                    ));
                }
                let mrt_type = match people.mrt_calc_type {
                    0 => "Zone Averaged",
                    1 => "Surface Weighted",
                    2 => "Angle Factor",
                    _ => {
                        return Err(unavailable(
                            "actual unsupported People nominal MRT branch2795",
                        ));
                    }
                };
                let clothing_type = match people.clothing_type {
                    0 => "Clothing Insulation Schedule",
                    1 => "Dynamic Clothing Model ASHRAE55",
                    2 => "Calculation Method Schedule",
                    _ => return Err(unavailable("invalid People nominal clothing type")),
                };
                let method_name = if people.clothing_type == 2 {
                    let method = people.clothing_method_sched.ok_or_else(|| {
                        unavailable("actual People nominal clothing-method pointer")
                    })?;
                    optional_schedule_name(owners.schedules, Some(method))?
                } else {
                    "N/A"
                };
                write!(
                    owners.eio,
                    "{},{},{},{},{},{},{},{},{},{},{}",
                    mrt_type,
                    optional_schedule_name(owners.schedules, people.work_eff_sched)?,
                    clothing_type,
                    method_name,
                    optional_schedule_name(owners.schedules, people.clothing_sched)?,
                    optional_schedule_name(owners.schedules, people.air_velocity_sched)?,
                    if people.fanger { "Yes" } else { "No" },
                    if people.pierce { "Yes" } else { "No" },
                    if people.ksu { "Yes" } else { "No" },
                    if people.cooling_effect_ash55 {
                        "Yes"
                    } else {
                        "No"
                    },
                    if people.ankle_draft_ash55 {
                        "Yes"
                    } else {
                        "No"
                    },
                )?;
            }
            writeln!(owners.eio)?;
        }
        if let Some(rows) = &self.allocations.electric_equipment {
            for (number, row) in rows.iter().enumerate() {
                if number == 0 {
                    writeln!(
                        owners.eio,
                        "! <ElectricEquipment Internal Gains Nominal>,Name,Schedule Name,Zone Name,Zone Floor Area {{m2}},# Zone Occupants,Equipment Level {{W}},Equipment/Floor Area {{W/m2}},Equipment per person {{W/person}},Fraction Latent,Fraction Radiant,Fraction Lost,Fraction Convected,End-Use SubCategory,Minimum Equipment Level for All Day Types {{W}},Maximum Equipment Level for All Day Types {{W}},Minimum Equipment Level for Weekdays {{W}}, Maximum Equipment Level for Weekdays {{W}},Minimum Equipment Level for Weekends/Holidays {{W}}, Maximum Equipment Level for Weekends /Holidays {{W}},Minimum Equipment Level for Summer Design Days {{W}}, Maximum Equipment Level for Summer Design Days {{W}},Minimum Equipment Level for Winter Design Days {{W}}, Maximum Equipment Level for Winter Design Days {{W}}"
                    )?;
                }
                if row.zone_ptr == 0 {
                    writeln!(
                        owners.eio,
                        " Electric Equipment-Illegal Zone specified, {}",
                        row.name
                    )?;
                    continue;
                }
                let zone = zones
                    .get(index(row.zone_ptr)?)
                    .ok_or_else(|| unavailable("actual ElectricEquipment nominal Zone"))?;
                let schedule = row
                    .schedule
                    .ok_or_else(|| unavailable("same nominal schedule pointer"))?;
                let name = owners
                    .schedules
                    .schedules
                    .get(schedule.0)
                    .ok_or_else(|| unavailable("same nominal schedule owner"))?
                    .name
                    .as_str();
                write!(
                    owners.eio,
                    " ElectricEquipment Internal Gains Nominal, {},{},{},{},{},",
                    row.name,
                    name,
                    zone.name,
                    rounded(zone.floor_area, 2)?,
                    rounded(zone.total_occupants, 1)?
                )?;
                write!(owners.eio, "{},", rounded(row.design_level, 3)?)?;
                divide(owners.eio, row.design_level, zone.floor_area)?;
                divide(owners.eio, row.design_level, zone.total_occupants)?;
                for value in [
                    row.fraction_latent,
                    row.fraction_radiant,
                    row.fraction_lost,
                    row.fraction_convected,
                ] {
                    write!(owners.eio, "{},", rounded(value, 3)?)?;
                }
                write!(
                    owners.eio,
                    "{},{},{},",
                    row.end_use_subcategory,
                    rounded(row.nominal_min_design_level, 3)?,
                    rounded(row.nominal_max_design_level, 3)?
                )?;
                // Four original day-type calls, each receiving the SAME schedule.
                for group in 0..4 {
                    let (minimum, maximum) = match owners.clock {
                        Some(clock) => owners
                            .schedules
                            .source_day_group_min_max_with_clock(schedule, group, clock)?,
                        None => owners.schedules.source_day_group_min_max(schedule, group)?,
                    };
                    write!(owners.eio, "{},", rounded(row.design_level * minimum, 3)?)?;
                    if group == 3 {
                        writeln!(owners.eio, "{}", rounded(row.design_level * maximum, 3)?)?;
                    } else {
                        write!(owners.eio, "{},", rounded(row.design_level * maximum, 3)?)?;
                    }
                }
            }
        }
        if let Some(rows) = &self.allocations.other_equipment {
            for (number, row) in rows.iter().enumerate() {
                if number == 0 {
                    writeln!(
                        owners.eio,
                        "! <OtherEquipment Internal Gains Nominal>,Name,Schedule Name,Zone Name,Zone Floor Area {{m2}},# Zone Occupants,Equipment Level {{W}},Equipment/Floor Area {{W/m2}},Equipment per person {{W/person}},Fraction Latent,Fraction Radiant,Fraction Lost,Fraction Convected,Minimum Equipment Level for All Day Types {{W}},Maximum Equipment Level for All Day Types {{W}},Minimum Equipment Level for Weekdays {{W}}, Maximum Equipment Level for Weekdays {{W}},Minimum Equipment Level for Weekends/Holidays {{W}}, Maximum Equipment Level for Weekends /Holidays {{W}},Minimum Equipment Level for Summer Design Days {{W}}, Maximum Equipment Level for Summer Design Days {{W}},Minimum Equipment Level for Winter Design Days {{W}}, Maximum Equipment Level for Winter Design Days {{W}}"
                    )?;
                }
                if row.zone_ptr == 0 {
                    writeln!(
                        owners.eio,
                        " Other Equipment-Illegal Zone specified, {}",
                        row.name
                    )?;
                    continue;
                }
                let zone = zones
                    .get(index(row.zone_ptr)?)
                    .ok_or_else(|| unavailable("actual OtherEquipment nominal Zone"))?;
                let schedule = row
                    .schedule
                    .ok_or_else(|| unavailable("same nominal schedule pointer"))?;
                let name = owners
                    .schedules
                    .schedules
                    .get(schedule.0)
                    .ok_or_else(|| unavailable("same nominal schedule owner"))?
                    .name
                    .as_str();
                write!(
                    owners.eio,
                    " OtherEquipment Internal Gains Nominal, {},{},{},{},{},",
                    row.name,
                    name,
                    zone.name,
                    rounded(zone.floor_area, 2)?,
                    rounded(zone.total_occupants, 1)?
                )?;
                write!(owners.eio, "{},", rounded(row.design_level, 3)?)?;
                divide(owners.eio, row.design_level, zone.floor_area)?;
                divide(owners.eio, row.design_level, zone.total_occupants)?;
                for value in [
                    row.fraction_latent,
                    row.fraction_radiant,
                    row.fraction_lost,
                    row.fraction_convected,
                    row.nominal_min_design_level,
                    row.nominal_max_design_level,
                ] {
                    write!(owners.eio, "{},", rounded(value, 3)?)?;
                }
                // Four original day-type calls, each receiving the SAME schedule.
                for group in 0..4 {
                    let (minimum, maximum) = match owners.clock {
                        Some(clock) => owners
                            .schedules
                            .source_day_group_min_max_with_clock(schedule, group, clock)?,
                        None => owners.schedules.source_day_group_min_max(schedule, group)?,
                    };
                    write!(owners.eio, "{},", rounded(row.design_level * minimum, 3)?)?;
                    if group == 3 {
                        writeln!(owners.eio, "{}", rounded(row.design_level * maximum, 3)?)?;
                    } else {
                        write!(owners.eio, "{},", rounded(row.design_level * maximum, 3)?)?;
                    }
                }
            }
        }
        // Lights/gas/hot-water/steam/ITE/baseboard nominal loops
        // are actually empty; their headers are conditionally emitted in-loop.
        Ok(())
    }
}
