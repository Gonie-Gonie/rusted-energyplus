//! Literal reached nominal EIO and Zone.InternalHeatGains writer.
use super::getter::SourceGainInputOwners;
use super::{E, SelectedInternalHeatGains, index, unavailable};
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
            zone.internal_heat_gains = 0.0 + 0.0 + 0.0 + other_total + 0.0 + 0.0;
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
                for value in [0.0, 0.0, 0.0, other_total, 0.0, 0.0] {
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
        // People/Lights/electric/gas/hot-water/steam/ITE/baseboard nominal loops
        // are actually empty; their headers are conditionally emitted in-loop.
        Ok(())
    }
}
