//! Genuine zone-only ThirdOrder correctAirTemp3863-4164 on live AirHB/Node owners.
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
        let zone_multiplier = f64::from(
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
        let sensible_load = if node > 0 {
            let system_node = owners.equipment.source_zone_air_node_mut(node)?;
            self.temp_dep_coef = self.sum_ha + self.sum_m_cp + self.sum_sys_m_cp;
            self.temp_ind_coef = self.sum_int_gain + self.sum_ha_tsurf - self.sum_ha_tref
                + self.sum_m_cp_t
                + self.sum_sys_m_cp_t
                + (self.non_air_system_response / zone_multiplier + self.sys_dep_zone_loads_lagged);
            if controller.subsystems.airflow_network.distribution_simulated {
                return Err(Error::Unavailable("positive controlled AFN TotalSen3943"));
            }
            if controller.subsystems.duct_loss.simulated {
                return Err(Error::Unavailable(
                    "positive controlled DuctLoss ZoneSen3946",
                ));
            }
            match owners.control.zone_air_solution {
                SelectedZoneAirSolution::ThirdOrder => {
                    self.zt =
                        crate::heat_balance::zone_predictor_corrector::source_third_order_air_value(
                            self.temp_ind_coef,
                            self.air_power_cap,
                            &self.ztm,
                            self.temp_dep_coef,
                        );
                }
                SelectedZoneAirSolution::Euler => {
                    return Err(Error::Unavailable(
                        "non-ThirdOrder controlled Source air solve3964",
                    ));
                }
            }
            if !controller.subsystems.room_air.any_non_mixing {
                *system_node.temperature = self.zt;
                *owners
                    .fan
                    .temp_tstat_air
                    .as_mut()
                    .and_then(|rows| rows.get_mut(index(number).ok()?))
                    .ok_or(Error::Unavailable("actual controlled TempTstatAir3976"))? = self.zt;
                *owners
                    .fan
                    .thermostat
                    .load_correction_factor
                    .as_mut()
                    .and_then(|rows| rows.get_mut(index(number).ok()?))
                    .ok_or(Error::Unavailable(
                        "actual controlled LoadCorrectionFactor3978",
                    ))? = 1.0;
            } else {
                let air_model = controller
                    .subsystems
                    .room_air
                    .air_models
                    .as_ref()
                    .and_then(|rows| rows.get(index(number).ok()?))
                    .ok_or(Error::Unavailable("actual controlled RoomAir AirModel3980"))?;
                if air_model.model == 1 || !air_model.simulate_air_model {
                    *system_node.temperature = self.zt;
                    *owners
                        .fan
                        .temp_tstat_air
                        .as_mut()
                        .and_then(|rows| rows.get_mut(index(number).ok()?))
                        .ok_or(Error::Unavailable("actual controlled TempTstatAir3986"))? = self.zt;
                    *owners
                        .fan
                        .thermostat
                        .load_correction_factor
                        .as_mut()
                        .and_then(|rows| rows.get_mut(index(number).ok()?))
                        .ok_or(Error::Unavailable(
                            "actual controlled LoadCorrectionFactor3988",
                        ))? = 1.0;
                } else {
                    return Err(Error::Unavailable(
                        "positive controlled IsZoneDispVent3Node3989",
                    ));
                }
            }
            let cp_air = energyplus_psy_cp_air_fn_w(self.air_hum_rat);
            let zone_enthalpy_in = self.sum_sys_m_cp_t;
            zone_enthalpy_in
                - (*system_node.mass_flow_rate / zone_multiplier)
                    * cp_air
                    * *system_node.temperature
                + self.non_air_system_response / zone_multiplier
                + self.sys_dep_zone_loads_lagged
        } else {
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
            0.0 //4094 actual uncontrolled branch
        };
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
        // Same conservative boundary at the actual RH/Psat invocation, after
        // preceding humidity and Node writes; no diagnostic/cache-hit proxy.
        if self.zt <= -100.0 || self.zt >= 200.0 {
            return Err(Error::Unavailable(
                "actual corrector RH/Psat diagnostic-aware normal-temperature domain4129",
            ));
        }
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

#[cfg(all(test, feature = "native-controlled-air-cohort"))]
impl SourceZoneSpaceHeatBalanceData {
    /// Caller bridge only; the original full member body above owns all math.
    pub(crate) fn correct_native_multiplier_cohort(
        &mut self,
        number: i32,
        use_zone_history: bool,
        controller: &mut SourceZoneAirPredictorCorrector,
        owners: &mut SourceZoneAirOwners<'_, '_>,
    ) -> Result<f64, Error> {
        self.correct_source_zone_temperature(number, use_zone_history, controller, owners)
    }
}
