//! CalcIfSetPointMet2533-2696, same thermostat/demand/Air/Zone/report owners.
use super::*;
fn temp(values: &Option<Vec<f64>>, at: usize) -> Result<f64, Error> {
    values
        .as_ref()
        .and_then(|v| v.get(at))
        .copied()
        .ok_or(Error::Unavailable(
            "actual ThermalComfort thermostat scalar",
        ))
}
impl SourceThermalComfort {
    pub(super) fn calc_source_if_setpoint_met<H, D>(
        &mut self,
        o: &mut SourceThermalComfortOwners<'_, '_, H, D>,
    ) -> Result<(), Error> {
        let cooling_threshold = o.hvac.deviation_from_setpoint_cooling;
        let heating_threshold = o.hvac.deviation_from_setpoint_heating;
        self.any_zone_not_met_heating = 0.0;
        self.any_zone_not_met_cooling = 0.0;
        self.any_zone_not_met_occupied = 0.0;
        self.any_zone_not_met_heating_occupied = 0.0;
        self.any_zone_not_met_cooling_occupied = 0.0;
        for number in 1..=o.globals.num_of_zones {
            let at = index(number)?;
            let setpoint = o
                .fan
                .thermostat
                .zone_tstat_setpoints
                .as_ref()
                .and_then(|v| v.get(at))
                .ok_or(Error::Unavailable("actual same Zone thermostat setpoint"))?;
            let load = o
                .controller
                .demand
                .zones
                .as_ref()
                .and_then(|v| v.get(at))
                .ok_or(Error::Unavailable("actual same ZoneSysEnergyDemand"))?
                .base
                .total_output_required;
            let row = self
                .thermal_comfort_setpoint
                .as_mut()
                .and_then(|v| v.get_mut(at))
                .ok_or(Error::Unavailable("actual ThermalComfortSetPoint Zone"))?;
            row.not_met_cooling = 0.0;
            row.not_met_heating = 0.0;
            row.not_met_cooling_occupied = 0.0;
            row.not_met_heating_occupied = 0.0;
            let test_heating = *o
                .fan
                .thermostat
                .temp_control_type
                .as_ref()
                .and_then(|v| v.get(at))
                .ok_or(Error::Unavailable("actual TempControlType Zone"))?
                != 2; // actual SingleCool2
            let test_cooling = *o
                .fan
                .thermostat
                .temp_control_type
                .as_ref()
                .and_then(|v| v.get(at))
                .ok_or(Error::Unavailable("actual TempControlType Zone"))?
                != 1; // actual SingleHeat1
            if test_heating && load > 0.0 {
                let delta = if o.controller.subsystems.room_air.model(number)? != 1 {
                    temp(&o.fan.temp_tstat_air, at)? - setpoint.setpt_lo
                } else {
                    let air = o
                        .air
                        .zones
                        .as_ref()
                        .and_then(|v| v.get(at))
                        .ok_or(Error::Unavailable("same actual comfort zoneHeatBalance"))?;
                    if o.controller.flags.number_on_off_zones > 0 {
                        air.air.ztav - setpoint.setpt_lo_average
                    } else {
                        air.air.ztav - setpoint.setpt_lo
                    }
                };
                if delta < heating_threshold {
                    row.not_met_heating = o.clock.time_step_zone;
                    row.total_not_met_heating += o.clock.time_step_zone;
                    if self.any_zone_not_met_heating == 0.0 {
                        self.any_zone_not_met_heating = o.clock.time_step_zone;
                    }
                    let occupied = self
                        .thermal_comfort_in_ash55
                        .as_ref()
                        .and_then(|v| v.get(at))
                        .ok_or(Error::Unavailable("actual same comfort ZoneIsOccupied"))?
                        .zone_is_occupied;
                    if occupied {
                        row.not_met_heating_occupied = o.clock.time_step_zone;
                        row.total_not_met_heating_occupied += o.clock.time_step_zone;
                        if self.any_zone_not_met_heating_occupied == 0.0 {
                            self.any_zone_not_met_heating_occupied = o.clock.time_step_zone;
                        }
                        if self.any_zone_not_met_occupied == 0.0 {
                            self.any_zone_not_met_occupied = o.clock.time_step_zone;
                        }
                    }
                }
            } else if test_cooling && load < 0.0 {
                let mut delta = if o.controller.subsystems.room_air.model(number)? != 1 {
                    temp(&o.fan.temp_tstat_air, at)? - setpoint.setpt_hi
                } else {
                    let air = o
                        .air
                        .zones
                        .as_ref()
                        .and_then(|v| v.get(at))
                        .ok_or(Error::Unavailable("same actual comfort zoneHeatBalance"))?;
                    if o.controller.flags.number_on_off_zones > 0 {
                        air.air.ztav - setpoint.setpt_hi_average
                    } else {
                        air.air.ztav - setpoint.setpt_hi
                    }
                };
                let zone = o
                    .zones
                    .zones
                    .as_ref()
                    .and_then(|v| v.get(at))
                    .ok_or(Error::Unavailable("actual comfort adjusted ITE Zone"))?;
                if zone.has_adjusted_return_temperature_by_ite {
                    delta =
                        temp(&o.fan.temp_tstat_air, at)? - zone.adjusted_return_temperature_by_ite;
                }
                if delta > cooling_threshold {
                    row.not_met_cooling = o.clock.time_step_zone;
                    row.total_not_met_cooling += o.clock.time_step_zone;
                    if self.any_zone_not_met_cooling == 0.0 {
                        self.any_zone_not_met_cooling = o.clock.time_step_zone;
                    }
                    let occupied = self
                        .thermal_comfort_in_ash55
                        .as_ref()
                        .and_then(|v| v.get(at))
                        .ok_or(Error::Unavailable("actual same comfort ZoneIsOccupied"))?
                        .zone_is_occupied;
                    if occupied {
                        row.not_met_cooling_occupied = o.clock.time_step_zone;
                        row.total_not_met_cooling_occupied += o.clock.time_step_zone;
                        if self.any_zone_not_met_cooling_occupied == 0.0 {
                            self.any_zone_not_met_cooling_occupied = o.clock.time_step_zone;
                        }
                        if self.any_zone_not_met_occupied == 0.0 {
                            self.any_zone_not_met_occupied = o.clock.time_step_zone;
                        }
                    }
                }
            }
        }
        self.total_any_zone_not_met_heating += self.any_zone_not_met_heating;
        self.total_any_zone_not_met_cooling += self.any_zone_not_met_cooling;
        self.total_any_zone_not_met_heating_occupied += self.any_zone_not_met_heating_occupied;
        self.total_any_zone_not_met_cooling_occupied += self.any_zone_not_met_cooling_occupied;
        self.total_any_zone_not_met_occupied += self.any_zone_not_met_occupied;
        if o.clock.end_design_day_envrns_flag {
            for number in 1..=o.globals.num_of_zones {
                let row = self
                    .thermal_comfort_setpoint
                    .as_ref()
                    .and_then(|v| v.get(index(number).ok()?))
                    .ok_or(Error::Unavailable("actual comfort setpoint report Zone"))?;
                let key = zone_name(o.zones, number)?;
                table_entry(
                    o.registry,
                    o.table,
                    "pdchULnotMetHeat",
                    key,
                    row.total_not_met_heating,
                )?;
                table_entry(
                    o.registry,
                    o.table,
                    "pdchULnotMetCool",
                    key,
                    row.total_not_met_cooling,
                )?;
                table_entry(
                    o.registry,
                    o.table,
                    "pdchULnotMetHeatOcc",
                    key,
                    row.total_not_met_heating_occupied,
                )?;
                table_entry(
                    o.registry,
                    o.table,
                    "pdchULnotMetCoolOcc",
                    key,
                    row.total_not_met_cooling_occupied,
                )?;
            }
            table_entry(
                o.registry,
                o.table,
                "pdchULnotMetHeat",
                "Facility",
                self.total_any_zone_not_met_heating,
            )?;
            table_entry(
                o.registry,
                o.table,
                "pdchULnotMetCool",
                "Facility",
                self.total_any_zone_not_met_cooling,
            )?;
            table_entry(
                o.registry,
                o.table,
                "pdchULnotMetHeatOcc",
                "Facility",
                self.total_any_zone_not_met_heating_occupied,
            )?;
            table_entry(
                o.registry,
                o.table,
                "pdchULnotMetCoolOcc",
                "Facility",
                self.total_any_zone_not_met_cooling_occupied,
            )?;
            o.predefined_summary
                .total_not_met_heating_occupied_for_abups =
                self.total_any_zone_not_met_heating_occupied;
            o.predefined_summary
                .total_not_met_cooling_occupied_for_abups =
                self.total_any_zone_not_met_cooling_occupied;
            o.predefined_summary.total_not_met_occupied_for_abups =
                self.total_any_zone_not_met_occupied;
            for number in 1..=o.globals.num_of_zones {
                let row = self
                    .thermal_comfort_setpoint
                    .as_mut()
                    .and_then(|v| v.get_mut(index(number).ok()?))
                    .ok_or(Error::Unavailable("actual comfort setpoint reset Zone"))?;
                row.total_not_met_heating = 0.0;
                row.total_not_met_cooling = 0.0;
                row.total_not_met_heating_occupied = 0.0;
                row.total_not_met_cooling_occupied = 0.0;
            }
            self.total_any_zone_not_met_heating = 0.0;
            self.total_any_zone_not_met_cooling = 0.0;
            self.total_any_zone_not_met_heating_occupied = 0.0;
            self.total_any_zone_not_met_cooling_occupied = 0.0;
            self.total_any_zone_not_met_occupied = 0.0;
            footnote(o.registry, o.clock, "pdstUnmetLoads")?;
        }
        Ok(())
    }
}
