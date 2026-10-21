//! Literal GetThermalComfortInputsASHRAE826-886 and AnkleDraft1352-1421.
//! Uses the same actual comfort/schedule/property/radiant/diagnostic owners.
use super::*;

impl SourceThermalComfort {
    /// The ASHRAE input route is distinct from ordinary Fanger: its body area,
    /// fallback warning and retained row writes must not be substituted.
    fn get_source_ashrae_inputs<H, D>(
        &mut self,
        o: &mut SourceThermalComfortOwners<'_, '_, H, D>,
    ) -> Result<(), Error> {
        let at = index(self.people_num)?;
        let (zone, activity, work, clothing_type, method, clothing, velocity) = {
            let people = o
                .gains
                .allocations
                .people
                .get(at)
                .ok_or(Error::Unavailable("actual ASHRAE input People"))?;
            (
                people.zone_ptr,
                people.activity_level_sched,
                people.work_eff_sched,
                people.clothing_type,
                people.clothing_method_sched,
                people.clothing_sched,
                people.air_velocity_sched,
            )
        };
        self.thermal_comfort_data
            .as_ref()
            .and_then(|rows| rows.get(at))
            .ok_or(Error::Unavailable("actual ASHRAE input comfort row"))?;
        self.zone_num = zone;
        self.air_temp = o
            .air
            .zones
            .as_ref()
            .and_then(|zones| zones.get(index(zone).ok()?))
            .ok_or(Error::Unavailable("actual ASHRAE averaged Zone AirHB"))?
            .air
            .ztav_comf;
        if o.controller.subsystems.room_air.any_non_mixing {
            return Err(Error::Unavailable(
                "positive ASHRAE non-Mixing RoomAir TCMF",
            ));
        }
        self.rad_temp = self.calc_source_fanger_rad_temp(o, at)?;
        let humidity = o
            .air
            .zones
            .as_ref()
            .and_then(|zones| zones.get(index(zone).ok()?))
            .ok_or(Error::Unavailable("actual ASHRAE averaged humidity"))?
            .air
            .air_hum_rat_avg_comf;
        self.rel_hum = o.psychrometrics.source_relative_humidity(
            self.air_temp,
            humidity,
            o.environment.current_weather.out_baro_press,
            "",
            &mut SourcePsychrometricOwners {
                clock: o.clock,
                globals: o.globals,
                environment: o.environment,
                hvac: o.hvac,
                recurring: o.recurring_errors,
                backend: o.backend,
            },
        )?;
        // Native BodySurfAreaPierce, not ordinary Fanger's BodySurfArea1.8.
        self.act_level = o.schedules.source_current_schedule_value(
            activity.ok_or(Error::Unavailable("actual ASHRAE activity schedule"))?,
        )? / 1.8258;
        self.work_eff = o.schedules.source_current_schedule_value(
            work.ok_or(Error::Unavailable("actual ASHRAE work schedule"))?,
        )? * self.act_level;
        match clothing_type {
            0 => {
                self.clo_unit = o.schedules.source_current_schedule_value(
                    clothing.ok_or(Error::Unavailable("actual ASHRAE clothing schedule"))?,
                )?;
                self.thermal_comfort_data
                    .as_mut()
                    .and_then(|rows| rows.get_mut(at))
                    .ok_or(Error::Unavailable("actual ASHRAE clothing row"))?
                    .clothing_value = self.clo_unit;
            }
            1 => {
                let comfort = self
                    .thermal_comfort_data
                    .as_mut()
                    .and_then(|rows| rows.get_mut(at))
                    .ok_or(Error::Unavailable("actual ASHRAE dynamic clothing row"))?;
                comfort.thermal_comfort_op_temp = (self.rad_temp + self.air_temp) / 2.0;
                comfort.clothing_value = self.clo_unit;
                self.calc_source_dynamic_clothing()?;
                self.clo_unit = self
                    .thermal_comfort_data
                    .as_ref()
                    .and_then(|rows| rows.get(at))
                    .ok_or(Error::Unavailable("actual ASHRAE dynamic result"))?
                    .clothing_value;
            }
            2 => {
                let intermediate =
                    o.schedules
                        .source_current_schedule_value(method.ok_or(Error::Unavailable(
                            "actual ASHRAE clothing method schedule",
                        ))?)?;
                if intermediate == 1.0 {
                    self.clo_unit = o.schedules.source_current_schedule_value(
                        clothing.ok_or(Error::Unavailable("actual ASHRAE method1 clothing"))?,
                    )?;
                    self.thermal_comfort_data
                        .as_mut()
                        .and_then(|rows| rows.get_mut(at))
                        .ok_or(Error::Unavailable("actual ASHRAE method1 row"))?
                        .clothing_value = self.clo_unit;
                } else if intermediate == 2.0 {
                    let comfort = self
                        .thermal_comfort_data
                        .as_mut()
                        .and_then(|rows| rows.get_mut(at))
                        .ok_or(Error::Unavailable("actual ASHRAE method2 clothing row"))?;
                    comfort.thermal_comfort_op_temp = (self.rad_temp + self.air_temp) / 2.0;
                    comfort.clothing_value = self.clo_unit;
                    self.calc_source_dynamic_clothing()?;
                    self.clo_unit = self
                        .thermal_comfort_data
                        .as_ref()
                        .and_then(|rows| rows.get(at))
                        .ok_or(Error::Unavailable("actual ASHRAE method2 result"))?
                        .clothing_value;
                } else {
                    self.clo_unit = o.schedules.source_current_schedule_value(clothing.ok_or(
                        Error::Unavailable("actual ASHRAE fallback clothing schedule"),
                    )?)?;
                    o.backend.show_warning_error("Scheduled clothing value will be used rather than clothing calculation method.")?;
                }
            }
            _ => o.backend.show_severe_error("Incorrect Clothing Type")?,
        }
        self.air_vel = o.schedules.source_current_schedule_value(
            velocity.ok_or(Error::Unavailable("actual ASHRAE air speed schedule"))?,
        )?;
        self.act_met = self.act_level / 58.2;
        Ok(())
    }

    pub(super) fn calc_source_ankle_draft<H, D>(
        &mut self,
        o: &mut SourceThermalComfortOwners<'_, '_, H, D>,
    ) -> Result<(), Error> {
        // This is a distinct Native loop, not Fanger's post-loop PeopleNum.
        self.people_num = 1;
        while self.people_num <= o.gains.totals[0] {
            let at = index(self.people_num)?;
            let people = o
                .gains
                .allocations
                .people
                .get(at)
                .ok_or(Error::Unavailable("actual AnkleDraft People"))?;
            if people.ankle_draft_ash55 {
                let ankle_schedule = people.ankle_air_velocity_sched;
                self.thermal_comfort_data
                    .as_ref()
                    .and_then(|rows| rows.get(at))
                    .ok_or(Error::Unavailable("actual AnkleDraft comfort row"))?;
                self.get_source_ashrae_inputs(o)?;
                let relative_air_velocity = if self.act_met > 1.0 {
                    self.air_vel + 0.3 * (self.act_met - 1.0)
                } else {
                    self.air_vel
                };
                let mut ppd_ad = -1.0;
                if self.act_met < 1.3 && self.clo_unit < 0.7 && relative_air_velocity < 0.2 {
                    let ankle_air_velocity = o.schedules.source_current_schedule_value(
                        ankle_schedule
                            .ok_or(Error::Unavailable("actual ankle air speed schedule"))?,
                    )?;
                    let pmv = self.calc_source_fanger_pmv(
                        self.air_temp,
                        self.rad_temp,
                        self.rel_hum,
                        relative_air_velocity,
                        self.act_level,
                        self.clo_unit,
                        self.work_eff,
                        o.psychrometrics,
                        &mut SourcePsychrometricOwners {
                            clock: o.clock,
                            globals: o.globals,
                            environment: o.environment,
                            hvac: o.hvac,
                            recurring: o.recurring_errors,
                            backend: o.backend,
                        },
                    )?;
                    ppd_ad = ((-2.58 + 3.05 * ankle_air_velocity - 1.06 * pmv).exp()
                        / (1.0 + (-2.58 + 3.05 * ankle_air_velocity - 1.06 * pmv).exp()))
                        * 100.0;
                } else if o.globals.display_extra_warnings {
                    if relative_air_velocity >= 0.2 {
                        o.backend.show_source_recurring_warning_at_end(
                            "Relative air velocity is above 0.2 m/s in Ankle draft PPD calculations. PPD at ankle draft will be set to -1.0.",
                            &mut self.ankle_draft_air_vel_warning_ind, o.recurring_errors,
                            o.clock, o.globals, Some(relative_air_velocity), Some(relative_air_velocity), None,
                            "[m/s]", "[m/s]", "",
                        )?;
                    }
                    if self.act_met >= 1.3 {
                        o.backend.show_source_recurring_warning_at_end(
                            "Metabolic rate is above 1.3 met in Ankle draft PPD calculations. PPD at ankle draft will be set to -1.0.",
                            &mut self.ankle_draft_act_met_warning_ind, o.recurring_errors,
                            o.clock, o.globals, Some(self.act_met), Some(self.act_met), None,
                            "[m/s]", "[m/s]", "",
                        )?;
                    }
                    if self.clo_unit >= 0.7 {
                        o.backend.show_source_recurring_warning_at_end(
                            "Clothing unit is above 0.7 in Ankle draft PPD calculations. PPD at ankle draft will be set to -1.0.",
                            &mut self.ankle_draft_clo_unit_warning_ind, o.recurring_errors,
                            o.clock, o.globals, Some(self.clo_unit), Some(self.clo_unit), None,
                            "[m/s]", "[m/s]", "",
                        )?;
                    }
                }
                self.thermal_comfort_data
                    .as_mut()
                    .and_then(|rows| rows.get_mut(at))
                    .ok_or(Error::Unavailable("actual AnkleDraft retained output"))?
                    .ankle_draft_ppdash55 = ppd_ad;
            }
            // No occupancy condition exists in the Native ankle loop.
            self.people_num = self
                .people_num
                .checked_add(1)
                .ok_or(Error::Unavailable("actual AnkleDraft PeopleNum increment"))?;
        }
        Ok(())
    }
}
