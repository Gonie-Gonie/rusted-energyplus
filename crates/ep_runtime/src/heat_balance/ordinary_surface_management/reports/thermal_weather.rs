//! ReportThermalResilience5870-6386 no-optional-model People route on real owners.
//! The actual People/report-period/SET flags decide the bounded branches.
use super::{Error, SourceResilienceReportOwners, SourceSurfaceManagement};

fn index(number: i32) -> Result<usize, Error> {
    number
        .checked_sub(1)
        .and_then(|n| usize::try_from(n).ok())
        .ok_or(Error::Unavailable(
            "actual positive thermal resilience Zone ordinal",
        ))
}
// General1041-1064; the fourth C++ int argument truncates the actual clock expression.
fn threshold_time(
    environment: &crate::weather::day::WeatherEnvironmentState,
    clock: &crate::weather::day::WeatherGlobalState,
) -> Result<f64, Error> {
    let step = clock.time_step.checked_sub(1).ok_or(Error::Unavailable(
        "actual safety threshold TimeStep-1 int domain",
    ))?;
    let minute = clock.time_step_zone * f64::from(step) * 60.0;
    if !minute.is_finite()
        || minute.trunc() < f64::from(i32::MIN)
        || minute.trunc() > f64::from(i32::MAX)
    {
        return Err(Error::Unavailable(
            "actual safety threshold minute int conversion",
        ));
    }
    let encoded = environment
        .month
        .checked_mul(100)
        .and_then(|n| n.checked_add(environment.day_of_month))
        .and_then(|n| n.checked_mul(100))
        .and_then(|n| n.checked_add(clock.hour_of_day))
        .and_then(|n| n.checked_mul(100))
        .and_then(|n| n.checked_add(minute.trunc() as i32))
        .ok_or(Error::Unavailable(
            "actual EncodeMonDayHrMin signed int domain",
        ))?;
    Ok(f64::from(encoded))
}
impl SourceSurfaceManagement {
    pub(crate) fn report_source_thermal_weather(
        &mut self,
        o: &mut SourceResilienceReportOwners<'_, '_>,
    ) -> Result<(), Error> {
        // Called only at the original RunPeriodWeather && !Warmup arm. Earlier
        // actual period guards preserve the zero-period traversal here.
        let value_not_init = -999.0;
        let near_threshold = 1.0;
        for zone_num in 1..=o.globals.num_of_zones {
            let row = o
                .zones
                .resilience
                .as_mut()
                .and_then(|rows| rows.get_mut(index(zone_num).ok()?))
                .ok_or(Error::Unavailable("actual Zone resilience owner5878"))?;
            row.pierce_set = value_not_init;
            row.pmv = value_not_init;
        }
        for person in 1..=o.gains.totals[0] {
            let at = index(person)?;
            let people = o
                .gains
                .allocations
                .people
                .get(at)
                .ok_or(Error::Unavailable("actual thermal resilience People5882"))?;
            let zone = index(people.zone_ptr)?;
            let row = o
                .zones
                .resilience
                .as_mut()
                .and_then(|v| v.get_mut(zone))
                .ok_or(Error::Unavailable(
                    "actual thermal resilience People Zone5883",
                ))?;
            // The primary overwrites this cell per People object; it is not a sum.
            row.zone_num_occ = people.number_of_people
                * o.schedules.source_current_schedule_value(
                    people
                        .sched
                        .ok_or(Error::Unavailable("actual resilience People schedule5884"))?,
                )?;
            row.zone_pierce_set_last_step = row.zone_pierce_set;
            if people.pierce {
                return Err(Error::Unavailable(
                    "actual positive thermal resilience PierceSET5887",
                ));
            }
            row.zone_pierce_set = -1.0;
            let occupants = row.zone_num_occ;
            let temperature = o
                .air
                .zones
                .as_ref()
                .and_then(|v| v.get(zone))
                .ok_or(Error::Unavailable("actual resilience Zone ZTAV5893"))?
                .air
                .ztav;
            if temperature > people.cold_stress_temp_thresh {
                if !row.crossed_cold_thresh {
                    row.zone_cold_hour_of_safety_bins[0] += o.clock.time_step_zone;
                }
            } else {
                row.zone_cold_hour_of_safety_bins[2] += o.clock.time_step_zone;
                row.zone_cold_hour_of_safety_bins[3] += occupants * o.clock.time_step_zone;
                row.zone_cold_hour_of_safety_bins[4] +=
                    f64::from(u8::from(occupants > 0.0)) * o.clock.time_step_zone;
                if !row.crossed_cold_thresh {
                    row.zone_cold_hour_of_safety_bins[1] = threshold_time(o.environment, o.clock)?;
                    row.crossed_cold_thresh = true;
                }
            }
            if temperature < people.heat_stress_temp_thresh {
                if !row.crossed_heat_thresh {
                    row.zone_heat_hour_of_safety_bins[0] += o.clock.time_step_zone;
                }
            } else {
                row.zone_heat_hour_of_safety_bins[2] += o.clock.time_step_zone;
                row.zone_heat_hour_of_safety_bins[3] += occupants * o.clock.time_step_zone;
                row.zone_heat_hour_of_safety_bins[4] +=
                    f64::from(u8::from(occupants > 0.0)) * o.clock.time_step_zone;
                if !row.crossed_heat_thresh {
                    row.zone_heat_hour_of_safety_bins[1] = threshold_time(o.environment, o.clock)?;
                    row.crossed_heat_thresh = true;
                }
            }
            // This read is unconditional, including the genuine ctor0 when Fanger=false.
            let pmv = o
                .comfort
                .thermal_comfort_data
                .as_ref()
                .and_then(|v| v.get(at))
                .ok_or(Error::Unavailable(
                    "actual resilience ThermalComfortData FangerPMV5955",
                ))?
                .fanger_pmv;
            for (bin, exceeds, delta) in [
                (0, pmv < -3.0, -3.0 - pmv),
                (1, pmv < -0.7, -0.7 - pmv),
                (2, pmv > 0.7, pmv - 0.7),
                (3, pmv > 3.0, pmv - 3.0),
            ] {
                if exceeds {
                    row.zone_discomfort_wt_exceed_occu_hour_bins[bin] +=
                        delta * occupants * o.clock.time_step_zone;
                    row.zone_discomfort_wt_exceed_occupied_hour_bins[bin] +=
                        delta * f64::from(u8::from(occupants > 0.0)) * o.clock.time_step_zone;
                }
            }
            if row.pierce_set < value_not_init + near_threshold {
                row.pierce_set = row.zone_pierce_set;
            } else if row.pierce_set != row.zone_pierce_set {
                return Err(Error::Unavailable(
                    "actual recurring multiple People PierceSET warning5986",
                ));
            }
            if row.pmv < value_not_init + near_threshold {
                row.pmv = pmv;
            } else if row.pmv != pmv {
                return Err(Error::Unavailable(
                    "actual recurring multiple People PMV warning5997",
                ));
            }
            // Positive report periods stopped at their actual earlier flags owner.
        }
        for zone_num in 1..=o.globals.num_of_zones {
            let at = index(zone_num)?;
            let row = o
                .zones
                .resilience
                .as_mut()
                .and_then(|rows| rows.get_mut(at))
                .ok_or(Error::Unavailable("actual Zone resilience owner6087"))?;
            let hi = row.zone_heat_index;
            let humidex = row.zone_humidex;

            let num_occ = row.zone_num_occ;
            if hi <= 26.7 {
                row.zone_heat_index_hour_bins[0] += o.clock.time_step_zone;
                row.zone_heat_index_occu_hour_bins[0] += num_occ * o.clock.time_step_zone;
                row.zone_heat_index_occupied_hour_bins[0] +=
                    f64::from(u8::from(num_occ > 0.0)) * o.clock.time_step_zone;
            } else if hi > 26.7 && hi <= 32.2 {
                row.zone_heat_index_hour_bins[1] += o.clock.time_step_zone;
                row.zone_heat_index_occu_hour_bins[1] += num_occ * o.clock.time_step_zone;
                row.zone_heat_index_occupied_hour_bins[1] +=
                    f64::from(u8::from(num_occ > 0.0)) * o.clock.time_step_zone;
            } else if hi > 32.2 && hi <= 39.4 {
                row.zone_heat_index_hour_bins[2] += o.clock.time_step_zone;
                row.zone_heat_index_occu_hour_bins[2] += num_occ * o.clock.time_step_zone;
                row.zone_heat_index_occupied_hour_bins[2] +=
                    f64::from(u8::from(num_occ > 0.0)) * o.clock.time_step_zone;
            } else if hi > 39.4 && hi <= 51.7 {
                row.zone_heat_index_hour_bins[3] += o.clock.time_step_zone;
                row.zone_heat_index_occu_hour_bins[3] += num_occ * o.clock.time_step_zone;
                row.zone_heat_index_occupied_hour_bins[3] +=
                    f64::from(u8::from(num_occ > 0.0)) * o.clock.time_step_zone;
            } else {
                row.zone_heat_index_hour_bins[4] += o.clock.time_step_zone;
                row.zone_heat_index_occu_hour_bins[4] += num_occ * o.clock.time_step_zone;
                row.zone_heat_index_occupied_hour_bins[4] +=
                    f64::from(u8::from(num_occ > 0.0)) * o.clock.time_step_zone;
            }

            if humidex <= 29.0 {
                row.zone_humidex_hour_bins[0] += o.clock.time_step_zone;
                row.zone_humidex_occu_hour_bins[0] += num_occ * o.clock.time_step_zone;
                row.zone_humidex_occupied_hour_bins[0] +=
                    f64::from(u8::from(num_occ > 0.0)) * o.clock.time_step_zone;
            } else if humidex > 29.0 && humidex <= 40.0 {
                row.zone_humidex_hour_bins[1] += o.clock.time_step_zone;
                row.zone_humidex_occu_hour_bins[1] += num_occ * o.clock.time_step_zone;
                row.zone_humidex_occupied_hour_bins[1] +=
                    f64::from(u8::from(num_occ > 0.0)) * o.clock.time_step_zone;
            } else if humidex > 40.0 && humidex <= 45.0 {
                row.zone_humidex_hour_bins[2] += o.clock.time_step_zone;
                row.zone_humidex_occu_hour_bins[2] += num_occ * o.clock.time_step_zone;
                row.zone_humidex_occupied_hour_bins[2] +=
                    f64::from(u8::from(num_occ > 0.0)) * o.clock.time_step_zone;
            } else if humidex > 45.0 && humidex <= 50.0 {
                row.zone_humidex_hour_bins[3] += o.clock.time_step_zone;
                row.zone_humidex_occu_hour_bins[3] += num_occ * o.clock.time_step_zone;
                row.zone_humidex_occupied_hour_bins[3] +=
                    f64::from(u8::from(num_occ > 0.0)) * o.clock.time_step_zone;
            } else {
                row.zone_humidex_hour_bins[4] += o.clock.time_step_zone;
                row.zone_humidex_occu_hour_bins[4] += num_occ * o.clock.time_step_zone;
                row.zone_humidex_occupied_hour_bins[4] +=
                    f64::from(u8::from(num_occ > 0.0)) * o.clock.time_step_zone;
            }

            let temperature = o
                .air
                .zones
                .as_ref()
                .and_then(|rows| rows.get(at))
                .ok_or(Error::Unavailable("actual zoneHeatBalance ZTAV6145"))?
                .air
                .ztav;
            let zone_tstat_setpt = o
                .fan
                .thermostat
                .zone_tstat_setpoints
                .as_ref()
                .and_then(|rows| rows.get(at))
                .ok_or(Error::Unavailable("actual zoneTstatSetpts6146"))?;
            let cooling_setpoint = zone_tstat_setpt.setpt_hi;
            let heating_setpoint = zone_tstat_setpt.setpt_lo;

            if cooling_setpoint > 0.0 && temperature > cooling_setpoint {
                row.zone_unmet_degree_hour_bins[0] +=
                    (temperature - cooling_setpoint) * o.clock.time_step_zone;
                row.zone_unmet_degree_hour_bins[1] +=
                    num_occ * (temperature - cooling_setpoint) * o.clock.time_step_zone;
                row.zone_unmet_degree_hour_bins[2] += f64::from(u8::from(num_occ > 0.0))
                    * (temperature - cooling_setpoint)
                    * o.clock.time_step_zone;
            }
            if heating_setpoint > 0.0 && temperature < heating_setpoint {
                row.zone_unmet_degree_hour_bins[3] +=
                    (heating_setpoint - temperature) * o.clock.time_step_zone;
                row.zone_unmet_degree_hour_bins[4] +=
                    num_occ * (heating_setpoint - temperature) * o.clock.time_step_zone;
                row.zone_unmet_degree_hour_bins[5] += f64::from(u8::from(num_occ > 0.0))
                    * (heating_setpoint - temperature)
                    * o.clock.time_step_zone;
            }

            if self.has_pierce_set {
                return Err(Error::Unavailable(
                    "actual positive zone PierceSET accumulation6166",
                ));
            }
            // Native6220-6386 loops over the same TotThermalReportPers. Its
            // positive domain already stops at the original5807/5812 guards;
            // no period flag, matrix value or occupancy is invented here.
        }
        Ok(())
    }
}
