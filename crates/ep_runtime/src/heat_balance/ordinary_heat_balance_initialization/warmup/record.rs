//! Literal RecKeepHeatBalance2971-3057 on the sole live manager/Air/demand/report owners.
use super::*;
impl SourceHeatBalanceManager {
    pub(crate) fn record_source_heat_balance(
        &mut self,
        o: &mut SourceWarmupRecordOwners<'_>,
    ) -> Result<(), Error> {
        for number in 1..=o.globals.num_of_zones {
            let i = index(number)?;
            let zone = o
                .air
                .zones
                .as_ref()
                .and_then(|a| a.get(i))
                .ok_or(Error::Unavailable("RecKeep actual zoneHeatBalance"))?;
            let demand = o
                .demand
                .zones
                .as_ref()
                .and_then(|a| a.get(i))
                .ok_or(Error::Unavailable("RecKeep actual ZoneSysEnergyDemand"))?;
            if zone.air.ztav > get(&self.max_temp_zone, i)? {
                set(&mut self.max_temp_zone, i, zone.air.ztav)?;
            }
            if zone.air.ztav < get(&self.min_temp_zone, i)? {
                set(&mut self.min_temp_zone, i, zone.air.ztav)?;
            }
            if demand.air_sys_heat_rate > get(&self.max_heat_load_zone, i)? {
                set(&mut self.max_heat_load_zone, i, demand.air_sys_heat_rate)?;
            }
            if demand.air_sys_cool_rate > get(&self.max_cool_load_zone, i)? {
                set(&mut self.max_cool_load_zone, i, demand.air_sys_cool_rate)?;
            }
            set(
                &mut self.temp_zone_sec_prev_day,
                i,
                get(&self.temp_zone_prev_day, i)?,
            )?;
            set(
                &mut self.load_zone_sec_prev_day,
                i,
                get(&self.load_zone_prev_day, i)?,
            )?;
            set(&mut self.temp_zone_prev_day, i, get(&self.temp_zone, i)?)?;
            set(&mut self.load_zone_prev_day, i, get(&self.load_zone, i)?)?;
            set(&mut self.temp_zone, i, zone.air.ztav)?;
            set(
                &mut self.load_zone,
                i,
                cpp_max(demand.air_sys_heat_rate, demand.air_sys_cool_rate.abs()),
            )?;
            if !o.clock.warmup_flag
                && o.clock.day_of_sim == 1
                && (!o.globals.doing_sizing || o.globals.do_pure_load_calc)
            {
                report_array(&self.report, 0)?;
                *self
                    .report
                    .temperature_difference
                    .get_mut(i)
                    .ok_or(Error::Unavailable("actual WarmupTempDiff cell"))? =
                    (get(&self.temp_zone_sec_prev_day, i)? - get(&self.temp_zone_prev_day, i)?)
                        .abs();
                report_array(&self.report, 1)?;
                *self
                    .report
                    .load_difference
                    .get_mut(i)
                    .ok_or(Error::Unavailable("actual WarmupLoadDiff cell"))? =
                    (get(&self.load_zone_sec_prev_day, i)? - get(&self.load_zone_prev_day, i)?)
                        .abs();
                if number == 1 {
                    self.report.count = self
                        .report
                        .count
                        .checked_add(1)
                        .ok_or(Error::Unavailable("actual CountWarmupDayPoints increment"))?;
                }
                let point = self.report.count.checked_sub(1).ok_or(Error::Unavailable(
                    "actual CountWarmupDayPoints positive ordinal",
                ))?;
                report_array(&self.report, 2)?;
                *self
                    .report
                    .temperature
                    .get_mut(point)
                    .and_then(|a| a.get_mut(i))
                    .ok_or(Error::Unavailable("actual TempZoneRpt cell"))? =
                    self.report.temperature_difference[i];
                report_array(&self.report, 3)?;
                *self
                    .report
                    .load
                    .get_mut(point)
                    .and_then(|a| a.get_mut(i))
                    .ok_or(Error::Unavailable("actual LoadZoneRpt cell"))? =
                    self.report.load_difference[i];
                report_array(&self.report, 4)?;
                *self
                    .report
                    .maximum_load
                    .get_mut(point)
                    .and_then(|a| a.get_mut(i))
                    .ok_or(Error::Unavailable("actual MaxLoadZoneRpt cell"))? =
                    get(&self.load_zone, i)?;
                if o.project.system.report_detailed_warmup_convergence {
                    if self.report.first_detailed_write {
                        o.eio.write_all(DETAILED_HEADER.as_bytes())?;
                        self.report.first_detailed_write = false;
                    }
                    writeln!(
                        o.eio,
                        " Warmup Convergence Information - Detailed, {},{},{},{},{}",
                        zone_name(o.zones, i)?,
                        o.clock.time_step,
                        o.clock.hour_of_day,
                        round(self.report.temperature_difference[i], 10)?,
                        round(self.report.load_difference[i], 10)?
                    )?;
                }
            }
        }
        if o.geometry.any_movable_insulation {
            return Err(Error::Unavailable(
                "RecKeep actual intMovInsulSurfNums list3050",
            ));
        }
        // Genuine UpdateWindowFaceTempsNonBSDFWin3303, including real empty-list path.
        for &number in &o.geometry.surface_lists.all_ht_window {
            let i = index(number)?;
            let surface = o
                .geometry
                .final_surfaces
                .as_ref()
                .and_then(|a| a.get(i))
                .ok_or(Error::Unavailable("actual window-report Surface"))?;
            let construction = o
                .control
                .construction_usage
                .get(index(surface.row.construction)?)
                .ok_or(Error::Unavailable("actual window-report Construction"))?;
            if construction.window_type_bsdf {
                continue;
            }
            let id = surface
                .typed_id
                .ok_or(Error::Unavailable("actual window-report Surface identity"))?;
            let h =
                o.history
                    .surface_ids
                    .iter()
                    .position(|a| *a == id)
                    .ok_or(Error::Unavailable(
                        "same window-report normal history owner",
                    ))?;
            let number = usize::try_from(number)
                .map_err(|_| Error::Unavailable("actual window-report positive Surface"))?;
            let outside = *o.history.normal.outside_temperature[0]
                .get(h)
                .ok_or(Error::Unavailable("actual window-report outside term1"))?;
            o.radiation
                .solar_timestep
                .heat_balance
                .surf_win_fen_lay_surf_temp_front
                .set([number, 1], outside)?;
            let inside = *o.history.normal.inside_temperature[0]
                .get(h)
                .ok_or(Error::Unavailable("actual window-report inside term1"))?;
            let layer = usize::try_from(construction.total_layers)
                .map_err(|_| Error::Unavailable("actual window-report TotLayers"))?;
            o.radiation
                .solar_timestep
                .heat_balance
                .surf_win_fen_lay_surf_temp_back
                .set([number, layer], inside)?;
        }
        Ok(())
    }
}
