//! Native current regular controls3303-3460 and EMS override6460-6510.
//! Every value comes from the same retained control, schedule, fan or module.
use super::super::controls::SourceTemperatureControlMode;
use super::*;

impl SourceZoneAirPredictorCorrector {
    pub(super) fn calculate_source_current_temperature_controls(
        &mut self,
        owners: &mut SourceZoneAirOwners<'_, '_>,
    ) -> Result<(), Error> {
        for number in 1..=self.controls.number_temperature_zones {
            let row = self
                .controls
                .temperature_controlled_zones
                .as_mut()
                .and_then(|rows| rows.get_mut(index(number).ok()?))
                .ok_or(Error::Unavailable("actual current TempControlledZone3300"))?;
            let actual_zone_number = row.actual_zone_number;
            let at = index(actual_zone_number)?;
            let schedule = row.control_type_schedule.ok_or(Error::Unavailable(
                "actual current control schedule pointer3304",
            ))?;
            let current_mode = owners.schedules.source_current_schedule_value(schedule)?;
            if !current_mode.is_finite()
                || current_mode < f64::from(i32::MIN)
                || current_mode > f64::from(i32::MAX)
            {
                return Err(Error::Unavailable(
                    "actual current control enum conversion domain3304",
                ));
            }
            let mode = current_mode as i32; // Native static_cast, never rounded/clamped.
            *owners
                .fan
                .thermostat
                .temp_control_type
                .as_mut()
                .and_then(|rows| rows.get_mut(at))
                .ok_or(Error::Unavailable("actual current TempControlType3304"))? = mode;
            *owners
                .fan
                .thermostat
                .temp_control_type_report
                .as_mut()
                .and_then(|rows| rows.get_mut(at))
                .ok_or(Error::Unavailable("actual current TempControlTypeRpt3305"))? = mode;
            let setpoint = owners
                .fan
                .thermostat
                .zone_tstat_setpoints
                .as_mut()
                .and_then(|rows| rows.get_mut(at))
                .ok_or(Error::Unavailable("actual current zoneTstatSetpts3308"))?;
            match mode {
                0 => {} // Native Uncontrolled leaves the retained setpoints alone.
                1 => {
                    if row.setpoints[1].is_used {
                        return Err(Error::Unavailable("actual SingleHeat current schedule3316"));
                    }
                }
                2 => return Err(Error::Unavailable("actual SingleCool current schedule3325")),
                3 => {
                    return Err(Error::Unavailable(
                        "actual SingleHeatCool current schedule3342",
                    ));
                }
                4 => {
                    let cool = row.setpoints[4].cool_schedule.ok_or(Error::Unavailable(
                        "actual Dual current cool schedule pointer3373",
                    ))?;
                    setpoint.setpt_hi = owners.schedules.source_current_schedule_value(cool)?;
                    row.thermostat_setpoint_hi = setpoint.setpt_hi;
                    if row.adaptive_comfort_temperature_control {
                        return Err(Error::Unavailable(
                            "actual AdjustOperativeSetPointsforAdapComfort3379",
                        ));
                    }
                    // Actual AdjustAirSetPointsforOpTempCntrl5877 returns before
                    // reading per-row Invalid when AnyOpTempControl is false.
                    if self.controls.any_operative_control
                        && row.operative_control != SourceTemperatureControlMode::None
                    {
                        return Err(Error::Unavailable(
                            "actual operative fraction/MRT adjustment5886",
                        ));
                    }
                    let heat = row.setpoints[4].heat_schedule.ok_or(Error::Unavailable(
                        "actual Dual current heat schedule pointer3385",
                    ))?;
                    setpoint.setpt_lo = owners.schedules.source_current_schedule_value(heat)?;
                    row.thermostat_setpoint_lo = setpoint.setpt_lo;
                    if self.controls.any_operative_control
                        && row.operative_control != SourceTemperatureControlMode::None
                    {
                        return Err(Error::Unavailable(
                            "actual operative fraction/MRT adjustment5886",
                        ));
                    }
                    if let Some(optimum_start) = owners.availability.optimum_start.as_ref() {
                        let optimum = optimum_start.get(at).ok_or(Error::Unavailable(
                            "actual OptStart same actual-Zone row3392",
                        ))?;
                        if optimum.actual_zone_number == actual_zone_number {
                            // This genuine positive optional child needs the
                            // Native day-value getter; do not synthesize samples.
                            return Err(Error::Unavailable(
                                "actual Dual OptStart CEILING/getDayVals3394",
                            ));
                        }
                        if optimum.start_flag {
                            setpoint.setpt_hi = self
                                .controls
                                .occupied_cooling_setpoints
                                .as_ref()
                                .and_then(|rows| rows.get(at))
                                .copied()
                                .flatten()
                                .ok_or(Error::Unavailable(
                                    "actual occupied cooling setpoint3404",
                                ))?;
                            setpoint.setpt_lo = self
                                .controls
                                .occupied_heating_setpoints
                                .as_ref()
                                .and_then(|rows| rows.get(at))
                                .copied()
                                .flatten()
                                .ok_or(Error::Unavailable(
                                    "actual occupied heating setpoint3405",
                                ))?;
                        }
                    }
                    // Actual humidity helper6434 returns before OvercoolCtrl.
                    if self.controls.any_temperature_humidity_control
                        && row.overcool_control != SourceTemperatureControlMode::None
                    {
                        return Err(Error::Unavailable(
                            "actual temperature/humidity overcool-range6441",
                        ));
                    }
                    // Native mode4 deliberately does not write scalar setpt.
                }
                _ => {
                    return Err(Error::Unavailable(
                        "actual illegal control type ShowSevereError3414",
                    ));
                }
            }
            if *owners.faults.source_number_faulty_thermostat() > 0
                && !owners.clock.warmup_flag
                && !owners.globals.doing_sizing
                && !owners.globals.kickoff_simulation
            {
                return Err(Error::Unavailable(
                    "actual FaultsThermostatOffset name row3429",
                ));
            }
        }
        Ok(())
    }

    pub(super) fn override_source_air_setpoints_for_ems(
        &self,
        owners: &mut SourceZoneAirOwners<'_, '_>,
    ) -> Result<(), Error> {
        for number in 1..=self.controls.number_temperature_zones {
            let row = self
                .controls
                .temperature_controlled_zones
                .as_ref()
                .and_then(|rows| rows.get(index(number).ok()?))
                .ok_or(Error::Unavailable("actual EMS TempControlledZone6472"))?;
            if row.ems_override_heating_on {
                let at = index(row.actual_zone_number)?;
                let setpoint = owners
                    .fan
                    .thermostat
                    .zone_tstat_setpoints
                    .as_mut()
                    .and_then(|rows| rows.get_mut(at))
                    .ok_or(Error::Unavailable("actual EMS heating zoneTstatSetpts6475"))?;
                let mode = *owners
                    .fan
                    .thermostat
                    .temp_control_type
                    .as_ref()
                    .and_then(|rows| rows.get(at))
                    .ok_or(Error::Unavailable("actual EMS heating TempControlType6477"))?;
                match mode {
                    1 | 3 => {
                        setpoint.setpt = row.ems_override_heating_value;
                        setpoint.setpt_lo = row.ems_override_heating_value;
                    }
                    4 => setpoint.setpt_lo = row.ems_override_heating_value,
                    _ => {}
                }
            }
            if row.ems_override_cooling_on {
                let at = index(row.actual_zone_number)?;
                let setpoint = owners
                    .fan
                    .thermostat
                    .zone_tstat_setpoints
                    .as_mut()
                    .and_then(|rows| rows.get_mut(at))
                    .ok_or(Error::Unavailable("actual EMS cooling zoneTstatSetpts6494"))?;
                let mode = *owners
                    .fan
                    .thermostat
                    .temp_control_type
                    .as_ref()
                    .and_then(|rows| rows.get(at))
                    .ok_or(Error::Unavailable("actual EMS cooling TempControlType6496"))?;
                match mode {
                    2 | 3 => {
                        setpoint.setpt = row.ems_override_cooling_value;
                        setpoint.setpt_hi = row.ems_override_cooling_value;
                    }
                    4 => setpoint.setpt_hi = row.ems_override_cooling_value,
                    _ => {}
                }
            }
        }
        Ok(())
    }
}
