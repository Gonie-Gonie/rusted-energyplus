//! Genuine uncontrolled ThirdOrder correctAirTemp3863-4164 on live AirHB.
use super::*;
impl SourceZoneSpaceHeatBalanceData {
    pub(super) fn correct_source_zone_temperature(
        &mut self,
        number: i32,
        use_zone_time_step_history: bool,
        controller: &mut SourceZoneAirPredictorCorrector,
        owners: &mut SourceZoneAirOwners<'_, '_>,
    ) -> Result<f64, Error> {
        let mut temperature_change = 0.0;
        let _zone_multiplier = f64::from(
            zone(owners, number)?
                .multiplier
                .checked_mul(zone(owners, number)?.list_multiplier)
                .ok_or(Error::Unavailable("defined Source integer ZoneMult3889"))?,
        ); //3889
        let seconds = owners.hvac.time_step_sys_seconds;
        if !use_zone_time_step_history {
            self.ztm = self.dsxmat;
            self.w_prev_zone_ts_temp = self.dsw_prev_zone_ts;
        } else {
            self.ztm = self.xmat;
            self.w_prev_zone_ts_temp = self.w_prev_zone_ts;
        }
        let volume = zone(owners, number)?.volume;
        let rho = energyplus_psy_rho_air_fn_pb_tdb_w(
            owners.environment.current_weather.out_baro_press,
            self.mat,
            self.air_hum_rat,
        );
        if rho < 0.0 {
            return Err(Error::Unavailable(
                "actual correctAirTemp negative-density diagnostic3910",
            ));
        }
        self.air_power_cap = volume
            * zone(owners, number)?.sensible_capacity_multiplier
            * rho
            * energyplus_psy_cp_air_fn_w(self.air_hum_rat)
            / seconds;
        controller.subsystems.room_air.manage_source_air_model(
            number,
            owners.input,
            owners.shortcut,
            owners.backend,
        )?;
        self.calculate_source_zone_sums(true, number, controller, owners)?;
        if owners.globals.hybrid_model_people_count {
            return Err(Error::Unavailable(
                "positive hybrid convection-except-people3923",
            ));
        }
        let node = zone(owners, number)?.system_zone_node_number;
        if node > 0 {
            return Err(Error::Unavailable(
                "positive controlled Zone SystemNode3935",
            ));
        }
        self.temp_dep_coef = self.sum_ha + self.sum_m_cp;
        self.temp_ind_coef =
            self.sum_int_gain + self.sum_ha_tsurf - self.sum_ha_tref + self.sum_m_cp_t;
        if controller.subsystems.airflow_network.distribution_simulated {
            return Err(Error::Unavailable("positive AFN TotalSen4058"));
        }
        if controller.subsystems.duct_loss.simulated {
            return Err(Error::Unavailable("positive DuctLoss ZoneSen4061"));
        }
        match owners.control.zone_air_solution {
            SelectedZoneAirSolution::ThirdOrder => {
                self.zt =
                    crate::heat_balance::zone_predictor_corrector::source_third_order_air_value(
                        self.temp_ind_coef,
                        self.air_power_cap,
                        &self.ztm,
                        self.temp_dep_coef,
                    )
            }
            SelectedZoneAirSolution::Euler => {
                return Err(Error::Unavailable("non-ThirdOrder Source air solve4079"));
            }
        }
        if controller.subsystems.room_air.any_non_mixing
            && controller.subsystems.room_air.model(number)? == 7
        {
            return Err(Error::Unavailable("positive RoomAFN ControlAirNode4089"));
        }
        let sensible_load = 0.0; //4094 actual uncontrolled branch
        if controller.subsystems.hybrid_model.active {
            return Err(Error::Unavailable("positive hybrid temperature zone4099"));
        }
        self.mat = self.zt;
        controller
            .demand
            .zones
            .as_mut()
            .and_then(|rows| rows.get_mut(index(number).ok()?))
            .ok_or(Error::Unavailable("same actual ZoneSysEnergyDemand4124"))?
            .report_source_sensible_load(sensible_load, seconds);
        self.correct_source_zone_humidity(number, controller, owners)?;
        self.air_hum_rat = self.air_hum_rat_temp;
        self.air_rel_hum = 100.0
            * relative_humidity(
                owners.psychrometrics,
                self.zt,
                self.air_hum_rat,
                owners.environment.current_weather.out_baro_press,
            )?;
        if controller.subsystems.room_air.any_non_mixing {
            return Err(Error::Unavailable("positive nonmixing isMixed flags4137"));
        }
        match owners.control.zone_air_solution {
            SelectedZoneAirSolution::ThirdOrder => {
                temperature_change = cpp_max(temperature_change, (self.zt - self.ztm[0]).abs())
            }
            SelectedZoneAirSolution::Euler => {
                return Err(Error::Unavailable("non-ThirdOrder temperature change4150"));
            }
        }
        Ok(temperature_change)
    }
}
