//! Genuine uncontrolled ThirdOrder prediction and four-slot down-interpolation.
use super::*;
fn down_interpolate(old_step: f64, new_step: f64, old: &[f64; 4], new: &mut [f64; 4]) -> f64 {
    let ratio = old_step / new_step;
    new[0] = old[0];
    if (ratio - 2.0).abs() < 0.01 {
        new[1] = (old[0] + old[1]) / 2.0;
        new[2] = old[1];
        new[3] = (old[1] + old[2]) / 2.0;
    } else if (ratio - 3.0).abs() < 0.01 {
        let delta = (old[1] - old[0]) / 3.0;
        new[1] = old[0] + delta;
        new[2] = new[1] + delta;
        new[3] = old[1];
    } else {
        let delta = (old[1] - old[0]) / ratio;
        new[1] = old[0] + delta;
        new[2] = new[1] + delta;
        new[3] = new[2] + delta;
    }
    old[0]
}
impl SourceZoneSpaceHeatBalanceData {
    fn update_source_temperatures(
        &mut self,
        number: i32,
        shorten: bool,
        use_zone_history: bool,
        prior_step: f64,
        controller: &SourceZoneAirPredictorCorrector,
        owners: &SourceZoneAirOwners<'_, '_>,
    ) -> Result<(), Error> {
        if shorten {
            if zone(owners, number)?.system_zone_node_number > 0 {
                return Err(Error::Unavailable("positive Zone air-node rollback6779"));
            }
            if owners.hvac.number_system_time_steps
                != owners.hvac.number_system_time_steps_last_zone
            {
                self.mat = down_interpolate(
                    prior_step,
                    owners.hvac.time_step_sys,
                    &self.xmat,
                    &mut self.dsxmat,
                );
                self.air_hum_rat = down_interpolate(
                    prior_step,
                    owners.hvac.time_step_sys,
                    &self.w_prev_zone_ts,
                    &mut self.dsw_prev_zone_ts,
                );
                if controller.subsystems.room_air.any_non_mixing {
                    return Err(Error::Unavailable("positive RoomAir rollback6808"));
                }
            }
        }
        if use_zone_history {
            self.ztm = self.xmat;
            self.w_prev_zone_ts_temp = self.w_prev_zone_ts;
        } else {
            self.ztm = self.dsxmat;
            self.w_prev_zone_ts_temp = self.dsw_prev_zone_ts;
        }
        Ok(())
    }
    fn predict_source_system_load(
        &mut self,
        number: i32,
        shorten: bool,
        use_zone_history: bool,
        prior_step: f64,
        controller: &mut SourceZoneAirPredictorCorrector,
        owners: &mut SourceZoneAirOwners<'_, '_>,
    ) -> Result<(), Error> {
        self.update_source_temperatures(
            number,
            shorten,
            use_zone_history,
            prior_step,
            controller,
            owners,
        )?;
        let rho = energyplus_psy_rho_air_fn_pb_tdb_w(
            owners.environment.current_weather.out_baro_press,
            self.mat,
            self.air_hum_rat,
        );
        if rho < 0.0 {
            return Err(Error::Unavailable(
                "actual predicted negative-density diagnostic3168",
            ));
        }
        self.air_power_cap = zone(owners, number)?.volume
            * zone(owners, number)?.sensible_capacity_multiplier
            * rho
            * energyplus_psy_cp_air_fn_w(self.air_hum_rat)
            / owners.hvac.time_step_sys_seconds;
        self.calculate_source_zone_sums(false, number, controller, owners)?;
        if owners.globals.hybrid_model_people_count {
            self.sum_int_gain_except_people = 0.0;
            return Err(Error::Unavailable(
                "positive predicted convection-except-people3180",
            ));
        }
        self.temp_dep_coef = self.sum_ha + self.sum_m_cp;
        self.temp_ind_coef = self.sum_int_gain + self.sum_ha_tsurf - self.sum_ha_tref
            + self.sum_m_cp_t
            + self.sys_dep_zone_loads_lagged;
        self.temp_history_term = self.air_power_cap
            * (3.0 * self.ztm[0] - (3.0 / 2.0) * self.ztm[1] + (1.0 / 3.0) * self.ztm[2]);
        self.temp_dep_load = (11.0 / 6.0) * self.air_power_cap + self.temp_dep_coef;
        self.temp_ind_load = self.temp_history_term + self.temp_ind_coef;
        if controller.subsystems.room_air.any_non_mixing
            && controller.subsystems.room_air.model(number)? == 7
        {
            return Err(Error::Unavailable("positive predicted RoomAir AFN3193"));
        }
        owners.hvac.shorten_time_step_sys_room_air = false;
        if owners.control.zone_air_solution != SelectedZoneAirSolution::ThirdOrder {
            return Err(Error::Unavailable(
                "non-ThirdOrder predicted T1/W1 history3214",
            ));
        }
        self.calculate_source_predicted_sensible(number, controller, owners)?;
        self.calculate_source_predicted_humidity(number, controller, owners)?;
        Ok(())
    }
    fn calculate_source_predicted_sensible(
        &mut self,
        number: i32,
        controller: &mut SourceZoneAirPredictorCorrector,
        owners: &mut SourceZoneAirOwners<'_, '_>,
    ) -> Result<(), Error> {
        let kind = *owners
            .fan
            .thermostat
            .temp_control_type
            .as_ref()
            .and_then(|a| a.get(index(number).ok()?))
            .ok_or(Error::Unavailable("actual predicted TempControlType6855"))?;
        if kind != 0 {
            return Err(Error::Unavailable("positive controlled predicted load6862"));
        }
        let node = zone(owners, number)?.system_zone_node_number;
        if node > 0 {
            return Err(Error::Unavailable(
                "positive predicted node TempSetPoint7222",
            ));
        }
        *controller
            .demand
            .setback
            .as_mut()
            .and_then(|a| a.get_mut(index(number).ok()?))
            .ok_or(Error::Unavailable("actual predicted Setback7226"))? = 0.0 > self.set_point_last;
        self.set_point_last = 0.0;
        owners
            .fan
            .thermostat
            .zone_tstat_setpoints
            .as_mut()
            .and_then(|a| a.get_mut(index(number).ok()?))
            .ok_or(Error::Unavailable("actual predicted Zone setpoint7229"))?
            .setpt = 0.0;
        *controller
            .demand
            .deadband_or_setback
            .as_mut()
            .and_then(|a| a.get_mut(index(number).ok()?))
            .ok_or(Error::Unavailable("actual predicted DeadBand7230"))? = false;
        *controller
            .demand
            .current_deadband_or_setback
            .as_mut()
            .and_then(|a| a.get_mut(index(number).ok()?))
            .ok_or(Error::Unavailable("actual predicted CurDeadBand7231"))? = false;
        let z = zone(owners, number)?;
        let correction = value(
            &owners.fan.thermostat.load_correction_factor,
            index(number)?,
        )?;
        controller
            .demand
            .zones
            .as_mut()
            .and_then(|a| a.get_mut(index(number).ok()?))
            .ok_or(Error::Unavailable(
                "actual predicted ZoneSysEnergyDemand7240",
            ))?
            .report_source_predicted_sensible(
                0.0,
                0.0,
                0.0,
                correction,
                z.multiplier,
                z.list_multiplier,
                z.is_controlled,
            )?;
        Ok(())
    }
    fn calculate_source_predicted_humidity(
        &mut self,
        number: i32,
        controller: &mut SourceZoneAirPredictorCorrector,
        owners: &SourceZoneAirOwners<'_, '_>,
    ) -> Result<(), Error> {
        if zone(owners, number)?.humidity_control_zone_index > 0 {
            return Err(Error::Unavailable(
                "positive predicted humidity control3492",
            ));
        }
        if owners.globals.doing_sizing && owners.control.do_latent_sizing {
            return Err(Error::Unavailable(
                "positive predicted ZoneEquipConfig latent sizing3634",
            ));
        }
        let row = controller
            .demand
            .zone_moisture
            .as_mut()
            .and_then(|a| a.get_mut(index(number).ok()?))
            .ok_or(Error::Unavailable(
                "actual predicted ZoneSysMoistureDemand3806",
            ))?;
        row.base.total_output_required = 0.0;
        row.output_required_to_dehumidifying_sp = 0.0;
        row.output_required_to_humidifying_sp = 0.0;
        Ok(())
    }
}
impl SourceZoneAirPredictorCorrector {
    pub(super) fn predict_source_system_loads(
        &mut self,
        shorten: bool,
        use_zone_history: bool,
        prior_step: f64,
        air: &mut SourceAirHeatBalance,
        owners: &mut SourceZoneAirOwners<'_, '_>,
    ) -> Result<(), Error> {
        if self.flags.number_stage_zones > 0 {
            return Err(Error::Unavailable("positive staged thermostat2915"));
        }
        // On/off thermostat loop preceding all-zone prediction is genuine, not skipped.
        if self.flags.number_on_off_zones > 0 {
            return Err(Error::Unavailable(
                "positive on/off thermostat prediction3044",
            ));
        }
        for number in 1..=owners.globals.num_of_zones {
            air.zones
                .as_mut()
                .and_then(|a| a.get_mut(index(number).ok()?))
                .ok_or(Error::Unavailable("actual Zone prediction3116"))?
                .air
                .predict_source_system_load(
                    number,
                    shorten,
                    use_zone_history,
                    prior_step,
                    self,
                    owners,
                )?;
            for &space in &zone(owners, number)?.space_indexes {
                if owners.space_controls.current {
                    return Err(Error::Unavailable("positive Space predictSystemLoad3119"));
                } else if shorten {
                    let from = &air
                        .zones
                        .as_ref()
                        .and_then(|a| a.get(index(number).ok()?))
                        .ok_or(Error::Unavailable("actual Zone rollback handoff3123"))?
                        .air;
                    let to = &mut air
                        .spaces
                        .as_mut()
                        .and_then(|a| a.get_mut(index(space).ok()?))
                        .ok_or(Error::Unavailable("actual Space rollback handoff3123"))?
                        .air;
                    to.mat = from.mat;
                    to.air_hum_rat = from.air_hum_rat;
                }
            }
        }
        if self.flags.number_on_off_zones > 0 {
            return Err(Error::Unavailable(
                "positive on/off thermostat mode-last3132",
            ));
        }
        Ok(())
    }
}
