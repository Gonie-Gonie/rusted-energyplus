//! Literal DD initialization and ordinary CEN15251 body2963-3191.
//! Weather-file history and non-Mixing RoomAir remain genuine child boundaries.
use super::*;

impl SourceThermalComfort {
    /// Native initiate=true,wthrsim=false. Retain firstDaySet, useEpwDataCEN
    /// and People accumulators; the actual WeatherManager supplies its DD mean.
    pub(crate) fn initialize_source_adaptive_cen15251_design_day(&mut self, average_dry_bulb: f64) {
        self.avg_dry_bulb_cen = 0.0;
        self.running_average_cen = 0.0;
        let in_average_dry_bulb = average_dry_bulb;
        self.running_average_cen = in_average_dry_bulb;
        self.avg_dry_bulb_cen = 0.0;
    }

    pub(super) fn calc_source_adaptive_cen15251<H, D>(
        &mut self,
        o: &mut SourceThermalComfortOwners<'_, '_, H, D>,
    ) -> Result<(), Error> {
        let system_time_elapsed = o.hvac.system_time_elapsed;
        const ALPHA: f64 = 0.8;
        if o.clock.begin_day_flag && !self.first_day_set {
            self.running_average_cen =
                ALPHA * self.running_average_cen + (1.0 - ALPHA) * self.avg_dry_bulb_cen;
            self.avg_dry_bulb_cen = 0.0;
        }
        self.first_day_set = false;
        if o.clock.begin_hour_flag {
            self.avg_dry_bulb_cen += o.environment.current_weather.out_dry_bulb_temp / 24.0;
        }
        self.people_num = 1;
        while self.people_num <= o.gains.totals[0] {
            let at = index(self.people_num)?;
            let people = o
                .gains
                .allocations
                .people
                .get(at)
                .ok_or(Error::Unavailable("actual AdaptiveCEN15251 People row"))?;
            if people.adaptive_cen15251 {
                self.thermal_comfort_data
                    .as_ref()
                    .and_then(|rows| rows.get(at))
                    .ok_or(Error::Unavailable("actual AdaptiveCEN15251 comfort row"))?;
                self.zone_num = people.zone_ptr;
                self.air_temp = o
                    .air
                    .zones
                    .as_ref()
                    .and_then(|rows| rows.get(index(self.zone_num).ok()?))
                    .ok_or(Error::Unavailable("actual AdaptiveCEN15251 Zone AirHB"))?
                    .air
                    .ztav_comf;
                if o.controller.subsystems.room_air.any_non_mixing {
                    return Err(Error::Unavailable(
                        "actual AdaptiveCEN15251 non-Mixing RoomAir TCMF3128",
                    ));
                }
                self.rad_temp = self.calc_source_fanger_rad_temp(o, at)?;
                self.op_temp = (self.air_temp + self.rad_temp) / 2.0;
                let comfort = self
                    .thermal_comfort_data
                    .as_mut()
                    .and_then(|rows| rows.get_mut(at))
                    .ok_or(Error::Unavailable(
                        "actual mutable AdaptiveCEN15251 comfort row",
                    ))?;
                comfort.thermal_comfort_op_temp = self.op_temp;
                comfort.cen15251_running_mean_outdoor_temp = self.running_average_cen;
                if (10.0..=30.0).contains(&self.running_average_cen) {
                    let people = o
                        .gains
                        .allocations
                        .people
                        .get(at)
                        .ok_or(Error::Unavailable(
                            "actual AdaptiveCEN15251 occupancy People",
                        ))?;
                    let number_occupants = people.number_of_people
                        * o.schedules
                            .source_current_schedule_value(people.sched.ok_or(
                                Error::Unavailable(
                                    "actual AdaptiveCEN15251 occupancy schedule pointer",
                                ),
                            )?)?;
                    let comfort_temperature = 0.33 * self.running_average_cen + 18.8;
                    comfort.t_comf_cen15251 = comfort_temperature;
                    if number_occupants > 0.0 {
                        let comfort_temperature_low = if self.running_average_cen < 15.0 {
                            23.75
                        } else {
                            comfort_temperature
                        };
                        if self.op_temp < comfort_temperature + 2.0
                            && self.op_temp > comfort_temperature_low - 2.0
                        {
                            comfort.thermal_comfort_adaptive_cen15251_cat_i = 1;
                            comfort.thermal_comfort_adaptive_cen15251_cat_ii = 1;
                            comfort.thermal_comfort_adaptive_cen15251_cat_iii = 1;
                        } else if self.op_temp < comfort_temperature + 3.0
                            && self.op_temp > comfort_temperature_low - 3.0
                        {
                            comfort.thermal_comfort_adaptive_cen15251_cat_i = 0;
                            comfort.thermal_comfort_adaptive_cen15251_cat_ii = 1;
                            comfort.thermal_comfort_adaptive_cen15251_cat_iii = 1;
                            o.gains.allocations.people.rows[at].time_not_met_cen15251_cat_i +=
                                system_time_elapsed;
                        } else if self.op_temp < comfort_temperature + 4.0
                            && self.op_temp > comfort_temperature_low - 4.0
                        {
                            comfort.thermal_comfort_adaptive_cen15251_cat_i = 0;
                            comfort.thermal_comfort_adaptive_cen15251_cat_ii = 0;
                            comfort.thermal_comfort_adaptive_cen15251_cat_iii = 1;
                            o.gains.allocations.people.rows[at].time_not_met_cen15251_cat_i +=
                                system_time_elapsed;
                            o.gains.allocations.people.rows[at].time_not_met_cen15251_cat_ii +=
                                system_time_elapsed;
                        } else {
                            comfort.thermal_comfort_adaptive_cen15251_cat_i = 0;
                            comfort.thermal_comfort_adaptive_cen15251_cat_ii = 0;
                            comfort.thermal_comfort_adaptive_cen15251_cat_iii = 0;
                            o.gains.allocations.people.rows[at].time_not_met_cen15251_cat_i +=
                                system_time_elapsed;
                            o.gains.allocations.people.rows[at].time_not_met_cen15251_cat_ii +=
                                system_time_elapsed;
                            o.gains.allocations.people.rows[at].time_not_met_cen15251_cat_iii +=
                                system_time_elapsed;
                        }
                    } else {
                        comfort.thermal_comfort_adaptive_cen15251_cat_i = -1;
                        comfort.thermal_comfort_adaptive_cen15251_cat_ii = -1;
                        comfort.thermal_comfort_adaptive_cen15251_cat_iii = -1;
                    }
                } else {
                    comfort.thermal_comfort_adaptive_cen15251_cat_i = -1;
                    comfort.thermal_comfort_adaptive_cen15251_cat_ii = -1;
                    comfort.thermal_comfort_adaptive_cen15251_cat_iii = -1;
                    comfort.t_comf_cen15251 = -1.0;
                }
            }
            self.people_num = self.people_num.checked_add(1).ok_or(Error::Unavailable(
                "actual AdaptiveCEN15251 PeopleNum increment",
            ))?;
        }
        Ok(())
    }
}
