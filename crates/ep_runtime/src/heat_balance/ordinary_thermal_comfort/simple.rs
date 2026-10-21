//! CalcThermalComfortSimpleASH552307-2515, occupied Mixing-zone polygon and report tail.
use super::*;
fn rounded(value: f64) -> Result<String, Error> {
    crate::heat_balance::source_round_format::round(value, 1)
        .map_err(|e| Error::Format(format!("ThermalComfort .1R: {e:?}")))
}
// OutputReportTabular18509-18567, including its boundary-product behavior.
fn in_triangle(q: (f64, f64), a: (f64, f64), b: (f64, f64), c: (f64, f64)) -> bool {
    let f_ab = (q.1 - a.1) * (b.0 - a.0) - (q.0 - a.0) * (b.1 - a.1);
    let f_ca = (q.1 - c.1) * (a.0 - c.0) - (q.0 - c.0) * (a.1 - c.1);
    let f_bc = (q.1 - b.1) * (c.0 - b.0) - (q.0 - b.0) * (c.1 - b.1);
    f_ab * f_bc >= 0.0 && f_bc * f_ca >= 0.0
}
fn in_quadrilateral(q: (f64, f64), vertices: [(f64, f64); 4]) -> bool {
    let [a, b, c, d] = vertices;
    let in_abc = in_triangle(q, a, b, c);
    let in_cda = in_triangle(q, c, d, a);
    in_abc || in_cda
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
        for people in o.gains.allocations.people.iter() {
            self.zone_num = people.zone_ptr;
            let occupants = people.number_of_people
                * o.schedules
                    .source_current_schedule_value(people.sched.ok_or(Error::Unavailable(
                        "actual SimpleASH55 People schedule pointer2344",
                    ))?)?;
            if occupants > 0.0 {
                self.thermal_comfort_in_ash55
                    .as_mut()
                    .and_then(|v| v.get_mut(index(self.zone_num).ok()?))
                    .ok_or(Error::Unavailable(
                        "actual occupied SimpleASH55 People Zone",
                    ))?
                    .zone_is_occupied = true;
            }
        }
        for number in 1..=o.globals.num_of_zones {
            let at = index(number)?;
            let row = self
                .thermal_comfort_in_ash55
                .as_mut()
                .and_then(|v| v.get_mut(at))
                .ok_or(Error::Unavailable("actual simple comfort Zone"))?;
            if row.zone_is_occupied {
                let air = &o
                    .air
                    .zones
                    .as_ref()
                    .and_then(|v| v.get(at))
                    .ok_or(Error::Unavailable(
                        "actual SimpleASH55 averaged Zone AirHB2348",
                    ))?
                    .air;
                *self
                    .zone_occ_hrs
                    .as_mut()
                    .and_then(|v| v.get_mut(at))
                    .ok_or(Error::Unavailable("actual ZoneOccHrs ordinal"))? +=
                    o.clock.time_step_zone;
                let air_temperature = air.ztav_comf;
                if o.controller.subsystems.room_air.any_non_mixing {
                    return Err(Error::Unavailable(
                        "actual SimpleASH55 IsZoneDispVent3Node/IsZoneUFAD/TCMF2353",
                    ));
                }
                let operative = air_temperature * 0.5 + air.mrt * 0.5;
                let point = (operative, air.air_hum_rat_avg_comf);
                let summer = in_quadrilateral(
                    point,
                    [(25.1, 0.0), (23.6, 0.012), (26.8, 0.012), (28.3, 0.0)],
                );
                let winter = in_quadrilateral(
                    point,
                    [(21.7, 0.0), (19.6, 0.012), (23.9, 0.012), (26.3, 0.0)],
                );
                if summer {
                    row.time_not_summer = 0.0;
                } else {
                    row.time_not_summer = o.clock.time_step_zone;
                    row.total_time_not_summer += o.clock.time_step_zone;
                    self.any_zone_time_not_simple_ash55_summer = o.clock.time_step_zone;
                }
                if winter {
                    row.time_not_winter = 0.0;
                } else {
                    row.time_not_winter = o.clock.time_step_zone;
                    row.total_time_not_winter += o.clock.time_step_zone;
                    self.any_zone_time_not_simple_ash55_winter = o.clock.time_step_zone;
                }
                if summer || winter {
                    row.time_not_either = 0.0;
                } else {
                    row.time_not_either = o.clock.time_step_zone;
                    row.total_time_not_either += o.clock.time_step_zone;
                    self.any_zone_time_not_simple_ash55_either = o.clock.time_step_zone;
                }
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
                let flags = SourceDiagnosticFlags {
                    kickoff: &o.globals.kickoff_simulation,
                    developer: o.developer,
                    warmup: &o.clock.warmup_flag,
                    doing_sizing: &o.globals.doing_sizing,
                };
                o.backend.show_source_warning_error(
                    &format!(
                        "More than 4% of time ({} hours) uncomfortable in one or more zones ",
                        rounded(allowed_hours)?
                    ),
                    &flags,
                )?;
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
