//! Literal DD initialization and ordinary AdaptiveASH55 body2717-2961.
//! STAT/EPW history and non-Mixing RoomAir retain genuine child boundaries.
use super::*;

impl SourceThermalComfort {
    /// Native initiate=true,wthrsim=false. The caller supplies the actual
    /// WeatherManager796-800 DesDayInput mean after its warning has succeeded.
    pub(crate) fn initialize_source_adaptive_ash55_design_day(&mut self, average_dry_bulb: f64) {
        self.avg_dry_bulb_ash = 0.0;
        self.running_average_ash = 0.0;
        self.monthly_temp.fill(0.0);
        let in_average_dry_bulb = average_dry_bulb;
        self.running_average_ash = in_average_dry_bulb;
        self.monthly_temp.fill(in_average_dry_bulb);
        self.avg_dry_bulb_ash = 0.0;
    }

    pub(super) fn calc_source_adaptive_ash55<H, D>(
        &mut self,
        o: &mut SourceThermalComfortOwners<'_, '_, H, D>,
    ) -> Result<(), Error> {
        let system_time_elapsed = o.hvac.system_time_elapsed;
        if o.clock.begin_day_flag && self.use_epw_data {
            return Err(Error::Unavailable(
                "actual AdaptiveASH55 EPW running-average history2882",
            ));
        }
        if o.clock.begin_day_flag && self.use_stat_data {
            return Err(Error::Unavailable(
                "actual AdaptiveASH55 STAT monthly history2898",
            ));
        }
        if o.clock.begin_hour_flag {
            self.avg_dry_bulb_ash += o.environment.current_weather.out_dry_bulb_temp / 24.0;
        }
        self.people_num = 1;
        while self.people_num <= o.gains.totals[0] {
            let at = index(self.people_num)?;
            let people = o
                .gains
                .allocations
                .people
                .get(at)
                .ok_or(Error::Unavailable("actual AdaptiveASH55 People row"))?;
            if people.adaptive_ash55 {
                // Native takes this row before the Zone/Air/MRT writes.
                self.thermal_comfort_data
                    .as_ref()
                    .and_then(|rows| rows.get(at))
                    .ok_or(Error::Unavailable("actual AdaptiveASH55 comfort row"))?;
                self.zone_num = people.zone_ptr;
                self.air_temp = o
                    .air
                    .zones
                    .as_ref()
                    .and_then(|rows| rows.get(index(self.zone_num).ok()?))
                    .ok_or(Error::Unavailable("actual AdaptiveASH55 Zone AirHB"))?
                    .air
                    .ztav_comf;
                if o.controller.subsystems.room_air.any_non_mixing {
                    return Err(Error::Unavailable(
                        "actual AdaptiveASH55 non-Mixing RoomAir TCMF2914",
                    ));
                }
                self.rad_temp = self.calc_source_fanger_rad_temp(o, at)?;
                self.op_temp = (self.air_temp + self.rad_temp) / 2.0;
                let comfort = self
                    .thermal_comfort_data
                    .as_mut()
                    .and_then(|rows| rows.get_mut(at))
                    .ok_or(Error::Unavailable(
                        "actual mutable AdaptiveASH55 comfort row",
                    ))?;
                comfort.thermal_comfort_op_temp = self.op_temp;
                comfort.ashrae55_running_mean_outdoor_temp = self.running_average_ash;
                if (10.0..=33.5).contains(&self.running_average_ash) {
                    let people = o
                        .gains
                        .allocations
                        .people
                        .get(at)
                        .ok_or(Error::Unavailable("actual AdaptiveASH55 occupancy People"))?;
                    let number_occupants = people.number_of_people
                        * o.schedules
                            .source_current_schedule_value(people.sched.ok_or(
                                Error::Unavailable(
                                    "actual AdaptiveASH55 occupancy schedule pointer",
                                ),
                            )?)?;
                    let comfort_temperature = 0.31 * self.running_average_ash + 17.8;
                    comfort.t_comf_ash55 = comfort_temperature;
                    if number_occupants > 0.0 {
                        if self.op_temp < comfort_temperature + 2.5
                            && self.op_temp > comfort_temperature - 2.5
                        {
                            comfort.thermal_comfort_adaptive_ash5590 = 1;
                            comfort.thermal_comfort_adaptive_ash5580 = 1;
                        } else if self.op_temp < comfort_temperature + 3.5
                            && self.op_temp > comfort_temperature - 3.5
                        {
                            comfort.thermal_comfort_adaptive_ash5590 = 0;
                            comfort.thermal_comfort_adaptive_ash5580 = 1;
                            o.gains.allocations.people.rows[at].time_not_met_ash5590 +=
                                system_time_elapsed;
                        } else {
                            comfort.thermal_comfort_adaptive_ash5590 = 0;
                            comfort.thermal_comfort_adaptive_ash5580 = 0;
                            o.gains.allocations.people.rows[at].time_not_met_ash5580 +=
                                system_time_elapsed;
                            o.gains.allocations.people.rows[at].time_not_met_ash5590 +=
                                system_time_elapsed;
                        }
                    } else {
                        comfort.thermal_comfort_adaptive_ash5590 = -1;
                        comfort.thermal_comfort_adaptive_ash5580 = -1;
                    }
                } else {
                    comfort.thermal_comfort_adaptive_ash5590 = -1;
                    comfort.thermal_comfort_adaptive_ash5580 = -1;
                    comfort.t_comf_ash55 = -1.0;
                }
            }
            self.people_num = self.people_num.checked_add(1).ok_or(Error::Unavailable(
                "actual AdaptiveASH55 PeopleNum increment",
            ))?;
        }
        Ok(())
    }
}
