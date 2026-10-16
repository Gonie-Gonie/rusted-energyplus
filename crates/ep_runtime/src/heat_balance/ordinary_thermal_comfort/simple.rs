//! CalcThermalComfortSimpleASH552307-2515, genuine no-People route and report tail.
use super::*;
fn rounded(value: f64) -> Result<String, Error> {
    crate::heat_balance::source_round_format::round(value, 1)
        .map_err(|e| Error::Format(format!("ThermalComfort .1R: {e:?}")))
}
impl SourceThermalComfort {
    pub(super) fn calc_source_simple_ash55<H, D>(
        &mut self,
        o: &mut SourceThermalComfortOwners<'_, '_, H, D>,
    ) -> Result<(), Error> {
        self.any_zone_time_not_simple_ash55_summer = 0.0;
        self.any_zone_time_not_simple_ash55_winter = 0.0;
        self.any_zone_time_not_simple_ash55_either = 0.0;
        for row in self
            .thermal_comfort_in_ash55
            .as_mut()
            .ok_or(Error::Unavailable("actual allocated ThermalComfortInASH55"))?
        {
            row.zone_is_occupied = false;
        }
        if let Some(people) = o.gains.allocations.people.first() {
            self.zone_num = people.zone_ptr;
            // Actual next operand is people.sched->getCurrentVal(), not a count or guessed zero.
            return Err(Error::Unavailable(
                "actual positive People schedule current value2344",
            ));
        }
        for number in 1..=o.globals.num_of_zones {
            let at = index(number)?;
            let row = self
                .thermal_comfort_in_ash55
                .as_mut()
                .and_then(|v| v.get_mut(at))
                .ok_or(Error::Unavailable("actual simple comfort Zone"))?;
            if row.zone_is_occupied {
                *self
                    .zone_occ_hrs
                    .as_mut()
                    .and_then(|v| v.get_mut(at))
                    .ok_or(Error::Unavailable("actual ZoneOccHrs ordinal"))? +=
                    o.clock.time_step_zone;
                return Err(Error::Unavailable(
                    "actual occupied SimpleASH55 Air/room-air comfort polygon child",
                ));
            } else {
                row.time_not_summer = 0.0;
                row.time_not_winter = 0.0;
                row.time_not_either = 0.0;
            }
        }
        self.total_any_zone_time_not_simple_ash55_summer +=
            self.any_zone_time_not_simple_ash55_summer;
        self.total_any_zone_time_not_simple_ash55_winter +=
            self.any_zone_time_not_simple_ash55_winter;
        self.total_any_zone_time_not_simple_ash55_either +=
            self.any_zone_time_not_simple_ash55_either;
        if o.clock.end_design_day_envrns_flag {
            let allowed_hours = f64::from(o.clock.num_of_day_in_envrn) * 24.0 * 0.04;
            let mut show_warning = false;
            for number in 1..=o.globals.num_of_zones {
                let row = self
                    .thermal_comfort_in_ash55
                    .as_ref()
                    .and_then(|v| v.get(index(number).ok()?))
                    .ok_or(Error::Unavailable("actual simple comfort warning Zone"))?;
                if row.enable55_warning && row.total_time_not_either > allowed_hours {
                    show_warning = true;
                }
            }
            if show_warning {
                o.backend.show_warning_error(&format!(
                    "More than 4% of time ({} hours) uncomfortable in one or more zones ",
                    rounded(allowed_hours)?
                ))?;
                o.backend
                    .show_continue_error("Based on ASHRAE 55-2004 graph (Section 5.2.1.1)")?;
                if o.environment.run_period_environment {
                    o.backend.show_continue_error(&format!(
                        "During Environment [{}]: {}",
                        o.environment.environment_start_end, o.environment.environment_name
                    ))?;
                } else {
                    o.backend.show_continue_error(&format!(
                        "During SizingPeriod Environment [{}]: {}",
                        o.environment.environment_start_end, o.environment.environment_name
                    ))?;
                }
                for number in 1..=o.globals.num_of_zones {
                    let row = self
                        .thermal_comfort_in_ash55
                        .as_ref()
                        .and_then(|v| v.get(index(number).ok()?))
                        .ok_or(Error::Unavailable("actual simple comfort warning Zone"))?;
                    if row.enable55_warning && row.total_time_not_either > allowed_hours {
                        o.backend.show_continue_error(&format!(
                            "{} hours were uncomfortable in zone: {}",
                            rounded(row.total_time_not_either)?,
                            zone_name(o.zones, number)?
                        ))?;
                    }
                }
            }
            for number in 1..=o.globals.num_of_zones {
                let row = self
                    .thermal_comfort_in_ash55
                    .as_ref()
                    .and_then(|v| v.get(index(number).ok()?))
                    .ok_or(Error::Unavailable("actual simple comfort report Zone"))?;
                let key = zone_name(o.zones, number)?;
                table_entry(
                    o.registry,
                    o.table,
                    "pdchSCwinterClothes",
                    key,
                    row.total_time_not_winter,
                )?;
                table_entry(
                    o.registry,
                    o.table,
                    "pdchSCsummerClothes",
                    key,
                    row.total_time_not_summer,
                )?;
                table_entry(
                    o.registry,
                    o.table,
                    "pdchSCeitherClothes",
                    key,
                    row.total_time_not_either,
                )?;
            }
            table_entry(
                o.registry,
                o.table,
                "pdchSCwinterClothes",
                "Facility",
                self.total_any_zone_time_not_simple_ash55_winter,
            )?;
            table_entry(
                o.registry,
                o.table,
                "pdchSCsummerClothes",
                "Facility",
                self.total_any_zone_time_not_simple_ash55_summer,
            )?;
            table_entry(
                o.registry,
                o.table,
                "pdchSCeitherClothes",
                "Facility",
                self.total_any_zone_time_not_simple_ash55_either,
            )?;
            o.predefined_summary
                .total_time_not_simple_ash55_either_for_abups =
                self.total_any_zone_time_not_simple_ash55_either;
            for number in 1..=o.globals.num_of_zones {
                let row = self
                    .thermal_comfort_in_ash55
                    .as_mut()
                    .and_then(|v| v.get_mut(index(number).ok()?))
                    .ok_or(Error::Unavailable("actual simple comfort reset Zone"))?;
                row.total_time_not_winter = 0.0;
                row.total_time_not_summer = 0.0;
                row.total_time_not_either = 0.0;
            }
            self.total_any_zone_time_not_simple_ash55_winter = 0.0;
            self.total_any_zone_time_not_simple_ash55_summer = 0.0;
            self.total_any_zone_time_not_simple_ash55_either = 0.0;
            footnote(o.registry, o.clock, "pdstSimpleComfort")?;
            for number in 1..=o.globals.num_of_zones {
                let occupied = *self
                    .zone_occ_hrs
                    .as_ref()
                    .and_then(|v| v.get(index(number).ok()?))
                    .ok_or(Error::Unavailable("actual LEED ZoneOccHrs"))?;
                let denominator =
                    o.clock
                        .num_of_day_in_envrn
                        .checked_mul(24)
                        .ok_or(Error::Unavailable(
                            "actual LEED environment day hours integer",
                        ))?;
                table_entry(
                    o.registry,
                    o.table,
                    "pdchLeedSutHrsWeek",
                    zone_name(o.zones, number)?,
                    168.0 * (occupied / f64::from(denominator)),
                )?;
            }
        }
        Ok(())
    }
}
