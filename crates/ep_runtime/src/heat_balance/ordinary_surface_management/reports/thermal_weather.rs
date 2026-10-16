//! ReportThermalResilience5870-6386 selected weather route on the sole real owners.
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
impl SourceSurfaceManagement {
    pub(super) fn report_source_thermal_weather(
        &mut self,
        o: &mut SourceResilienceReportOwners<'_, '_>,
    ) -> Result<(), Error> {
        // Called only at the original RunPeriodWeather && !Warmup arm. Earlier
        // actual period guards preserve the zero-period traversal here.
        let _cold_temp_thresh = 15.56;
        let _heat_temp_thresh = 30.0;
        let value_not_init = -999.0;
        let _near_threshold = 1.0;
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
        // Native5881 enters the People traversal only for the same positive
        // TotPeople. Its first ZonePtr record operation is not represented by
        // this selected weather route. All preceding zone resets are retained.
        if o.gains.totals[0] > 0 {
            return Err(Error::Unavailable(
                "actual positive ReportThermalResilience People.ZonePtr5882",
            ));
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
