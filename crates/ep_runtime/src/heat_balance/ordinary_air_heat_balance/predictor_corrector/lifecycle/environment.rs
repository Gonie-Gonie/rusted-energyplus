//! Native environment/day initialization and regular thermostat demand controls.
use super::*;
impl SourceZoneAirPredictorCorrector {
    pub(super) fn initialize_source_air_environment(
        &mut self,
        air: &mut SourceAirHeatBalance,
        owners: &mut SourceZoneAirOwners<'_, '_>,
    ) -> Result<(), Error> {
        if self.flags.my_environment && owners.clock.begin_envrn_flag {
            for row in air.zones.as_mut().ok_or(Error::Unavailable(
                "actual zoneHeatBalance beginEnvironment2635",
            ))? {
                row.air
                    .begin_source_environment(owners.environment.current_weather.out_hum_rat);
            }
            if owners.space_controls.current {
                for row in air.spaces.as_mut().ok_or(Error::Unavailable(
                    "actual spaceHeatBalance beginEnvironment2639",
                ))? {
                    row.air
                        .begin_source_environment(owners.environment.current_weather.out_hum_rat);
                }
            }
            let fan = &mut owners.fan.thermostat;
            for row in fan.zone_tstat_setpoints.as_mut().ok_or(Error::Unavailable(
                "actual zoneTstatSetpts beginEnvironment2644",
            ))? {
                row.setpt_hi = 0.0;
                row.setpt_lo = 0.0;
                row.setpt_adap_comfort_cool = 0.0;
                row.setpt = 0.0;
            }
            fan.load_correction_factor
                .as_mut()
                .ok_or(Error::Unavailable("actual LoadCorrectionFactor2648"))?
                .fill(1.0);
            fan.temp_control_type
                .as_mut()
                .ok_or(Error::Unavailable("actual TempControlType2649"))?
                .fill(0);
            for row in self
                .demand
                .zones
                .as_mut()
                .ok_or(Error::Unavailable("actual ZoneSysEnergyDemand2650"))?
            {
                row.begin_source_environment()?;
            }
            for row in self
                .demand
                .zone_moisture
                .as_mut()
                .ok_or(Error::Unavailable("actual ZoneSysMoistureDemand2653"))?
            {
                row.begin_source_environment()?;
            }
            if owners.space_controls.current {
                for row in self
                    .demand
                    .spaces
                    .as_mut()
                    .ok_or(Error::Unavailable("actual spaceSysEnergyDemand2657"))?
                {
                    row.begin_source_environment()?;
                }
                for row in self
                    .demand
                    .space_moisture
                    .as_mut()
                    .ok_or(Error::Unavailable("actual spaceSysMoistureDemand2660"))?
                {
                    row.begin_source_environment()?;
                }
            }
            self.demand
                .deadband_or_setback
                .as_mut()
                .ok_or(Error::Unavailable("actual DeadBandOrSetback2665"))?
                .fill(false);
            for row in owners
                .zones
                .zones
                .as_mut()
                .ok_or(Error::Unavailable("actual Zone NoHeatToReturnAir2667"))?
            {
                row.no_heat_to_return_air = false;
            }
            fan.previous_measured_temperature_1
                .as_mut()
                .ok_or(Error::Unavailable("actual PreviousMeasuredZT1"))?
                .fill(0.0);
            fan.previous_measured_temperature_2
                .as_mut()
                .ok_or(Error::Unavailable("actual PreviousMeasuredZT2"))?
                .fill(0.0);
            fan.previous_measured_temperature_3
                .as_mut()
                .ok_or(Error::Unavailable("actual PreviousMeasuredZT3"))?
                .fill(0.0);
            fan.previous_measured_humidity_1
                .as_mut()
                .ok_or(Error::Unavailable("actual PreviousMeasuredHumRat1"))?
                .fill(0.0);
            fan.previous_measured_humidity_2
                .as_mut()
                .ok_or(Error::Unavailable("actual PreviousMeasuredHumRat2"))?
                .fill(0.0);
            fan.previous_measured_humidity_3
                .as_mut()
                .ok_or(Error::Unavailable("actual PreviousMeasuredHumRat3"))?
                .fill(0.0);
            self.flags.my_environment = false;
        }
        if !owners.clock.begin_envrn_flag {
            self.flags.my_environment = true;
        }
        if self.flags.my_day && owners.clock.begin_day_flag {
            self.flags.my_day = false;
        }
        if !owners.clock.begin_day_flag {
            self.flags.my_day = true;
        }
        for number in 1..=self.controls.number_temperature_zones {
            let row = self
                .controls
                .temperature_controlled_zones
                .as_ref()
                .and_then(|rows| rows.get(index(number).ok()?))
                .ok_or(Error::Unavailable("actual Init TempControlledZone2682"))?;
            if *owners.equipment.source_equipment_inputs_filled()
                && !self.flags.controlled_zones_checked
            {
                if !owners
                    .equipment
                    .verify_source_controlled_zone_for_thermostat(&row.zone_name)
                {
                    return Err(Error::Unavailable(
                        "actual uncontrolled thermostat ShowSevereError2685",
                    ));
                }
            }
            if row.manage_demand {
                let at = index(row.actual_zone_number)?;
                let setpoint = owners
                    .fan
                    .thermostat
                    .zone_tstat_setpoints
                    .as_mut()
                    .and_then(|rows| rows.get_mut(at))
                    .ok_or(Error::Unavailable("actual demand zoneTstatSetpts2697"))?;
                let mode = *owners
                    .fan
                    .thermostat
                    .temp_control_type
                    .as_ref()
                    .and_then(|rows| rows.get(at))
                    .ok_or(Error::Unavailable("actual demand TempControlType2699"))?;
                match mode {
                    1 => {
                        if setpoint.setpt > row.heating_reset_limit {
                            setpoint.setpt = row.heating_reset_limit;
                            setpoint.setpt_lo = row.heating_reset_limit;
                        }
                    }
                    2 => {
                        if setpoint.setpt < row.cooling_reset_limit {
                            setpoint.setpt = row.cooling_reset_limit;
                            setpoint.setpt_hi = row.cooling_reset_limit;
                        }
                    }
                    3 => {
                        if setpoint.setpt > row.heating_reset_limit
                            || setpoint.setpt < row.cooling_reset_limit
                        {
                            *owners
                                .fan
                                .thermostat
                                .temp_control_type
                                .as_mut()
                                .and_then(|rows| rows.get_mut(at))
                                .ok_or(Error::Unavailable("actual demand TempControlType2715"))? =
                                4;
                            *owners
                                .fan
                                .thermostat
                                .temp_control_type_report
                                .as_mut()
                                .and_then(|rows| rows.get_mut(at))
                                .ok_or(Error::Unavailable(
                                    "actual demand TempControlTypeRpt2716",
                                ))? = 4;
                            setpoint.setpt_lo = setpoint.setpt;
                            setpoint.setpt_hi = setpoint.setpt;
                            if setpoint.setpt_lo > row.heating_reset_limit {
                                setpoint.setpt_lo = row.heating_reset_limit;
                            }
                            if setpoint.setpt_hi < row.cooling_reset_limit {
                                setpoint.setpt_hi = row.cooling_reset_limit;
                            }
                        }
                    }
                    4 => {
                        if setpoint.setpt_lo > row.heating_reset_limit {
                            setpoint.setpt_lo = row.heating_reset_limit;
                        }
                        if setpoint.setpt_hi < row.cooling_reset_limit {
                            setpoint.setpt_hi = row.cooling_reset_limit;
                        }
                    }
                    _ => {}
                }
            }
        }
        if self.controls.number_comfort_zones > 0 {
            return Err(Error::Unavailable(
                "positive actual ComfortControlledZone2754",
            ));
        }
        if self.flags.errors_found {
            return match owners.backend.show_fatal_error(
                "InitZoneAirSetpoints - program terminates due to previous condition.",
            ) {
                Err(e) => Err(Error::Fatal(e)),
                Ok(never) => match never {},
            };
        }
        if *owners.equipment.source_equipment_inputs_filled() {
            self.flags.controlled_zones_checked = true;
        }
        Ok(())
    }
}
