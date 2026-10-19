//! Actual zone-only correctHumRat4433-4619; same live humidity/latent/Node fields.
use super::*;
impl SourceZoneSpaceHeatBalanceData {
    pub(super) fn correct_source_zone_humidity(
        &mut self,
        number: i32,
        controller: &SourceZoneAirPredictorCorrector,
        owners: &mut SourceZoneAirOwners<'_, '_>,
    ) -> Result<(), Error> {
        let mut moisture_mass_flow_rate = 0.0;
        let mut zone_mass_flow_rate = 0.0;
        let z = zone(owners, number)?;
        let zone_multiplier = z
            .multiplier
            .checked_mul(z.list_multiplier)
            .ok_or(Error::Unavailable("defined Source humidity ZoneMult4447"))?;
        if z.is_controlled {
            let config = owners.equipment.source_zone_equipment_config(number)?;
            for at in 0..*config.number_inlet_nodes() {
                let node_number = *config
                    .inlet_node_numbers()
                    .and_then(|nodes| nodes.get(at))
                    .ok_or(Error::Unavailable(
                        "actual humidity controlled InletNode4456",
                    ))?;
                let ordinal = i32::try_from(node_number).map_err(|_| {
                    Error::Unavailable("actual humidity InletNode native integer ordinal")
                })?;
                let inlet = owners.equipment.source_zone_air_node(ordinal)?;
                moisture_mass_flow_rate +=
                    (*inlet.mass_flow_rate * *inlet.humidity_ratio) / f64::from(zone_multiplier);
                zone_mass_flow_rate += *inlet.mass_flow_rate / f64::from(zone_multiplier);
            }
        } else if z.is_return_plenum {
            return Err(Error::Unavailable("positive humidity return plenum4463"));
        } else if z.is_supply_plenum {
            return Err(Error::Unavailable("positive humidity supply plenum4487"));
        }
        if !z.leakage_parallel_piu_numbers.is_empty() {
            return Err(Error::Unavailable("positive humidity parallel PIU4494"));
        }
        let latent_gain = self.latent_gain
            + value(&owners.fan.sum_latent_ht_rad_sys, index(number)?)?
            + value(&owners.fan.sum_latent_pool, index(number)?)?;
        let seconds = owners.hvac.time_step_sys_seconds;
        let rho = energyplus_psy_rho_air_fn_pb_tdb_w(
            owners.environment.current_weather.out_baro_press,
            self.zt,
            self.air_hum_rat,
        );
        if rho < 0.0 {
            return Err(Error::Unavailable(
                "actual humidity negative-density diagnostic4512",
            ));
        }
        let vaporization = energyplus_psy_hg_air_fn_w_tdb(self.air_hum_rat, self.zt);
        let b = (latent_gain / vaporization)
            + ((self.oamfl + self.vamfl + self.ctmfl)
                * owners.environment.current_weather.out_hum_rat)
            + self.eamf_lx_hum_rat
            + moisture_mass_flow_rate
            + self.sum_hm_a_ra_w
            + self.mixing_mass_flow_x_hum_rat
            + self.m_dot_oa * owners.environment.current_weather.out_hum_rat;
        let a = zone_mass_flow_rate
            + self.oamfl
            + self.vamfl
            + self.eamfl
            + self.ctmfl
            + self.sum_hm_a_ra
            + self.mixing_mass_flow_zone
            + self.m_dot_oa;
        if controller
            .subsystems
            .airflow_network
            .replaces_simple_multizone()
        {
            return Err(Error::Unavailable("positive humidity AFN exchangeData4522"));
        }
        let c =
            rho * zone(owners, number)?.volume * zone(owners, number)?.moisture_capacity_multiplier
                / seconds;
        if controller.subsystems.airflow_network.distribution_simulated {
            return Err(Error::Unavailable("positive humidity AFN TotalLat4530"));
        }
        if controller.subsystems.duct_loss.simulated {
            return Err(Error::Unavailable("positive humidity DuctLoss ZoneLat4533"));
        }
        match owners.control.zone_air_solution {
            SelectedZoneAirSolution::ThirdOrder => {
                self.air_hum_rat_temp =
                    crate::heat_balance::zone_predictor_corrector::source_third_order_air_value(
                        b,
                        c,
                        &self.w_prev_zone_ts_temp,
                        a,
                    )
            }
            SelectedZoneAirSolution::Euler => {
                return Err(Error::Unavailable("non-ThirdOrder humidity solve4554"));
            }
        }
        if self.air_hum_rat_temp < 0.0 {
            self.air_hum_rat_temp = 0.0;
        }
        // Conservative raw-temperature admission before the actual cached
        // Psat-backed call; diagnostic/CalledFrom transport is not yet ported.
        if self.zt <= -100.0 || self.zt >= 200.0 {
            return Err(Error::Unavailable(
                "actual humidity WFromRh/Psat diagnostic-aware normal-temperature domain4568",
            ));
        }
        let saturation = owners
            .psychrometrics
            .evaluate(
                EnergyPlusPsychrometricFunction::WFromRh,
                &[
                    self.zt,
                    1.0,
                    owners.environment.current_weather.out_baro_press,
                ],
            )
            .map_err(|_| Error::Unavailable("actual humidity saturation instance/cache4568"))?
            .result;
        if self.air_hum_rat_temp > saturation {
            self.air_hum_rat_temp = saturation;
        }
        if controller.subsystems.room_air.model(number)? == 7 {
            return Err(Error::Unavailable("positive humidity RoomAFN4575"));
        }
        if controller.subsystems.hybrid_model.active {
            return Err(Error::Unavailable("positive hybrid humidity zone4581"));
        }
        let node_number = zone(owners, number)?.system_zone_node_number;
        if node_number > 0 {
            let node = owners.equipment.source_zone_air_node_mut(node_number)?;
            *node.humidity_ratio = self.air_hum_rat_temp;
            *node.enthalpy = energyplus_psy_h_fn_tdb_w(self.zt, self.air_hum_rat_temp);
        }
        if owners.control.do_latent_sizing {
            return Err(Error::Unavailable("positive latent sizing4603"));
        }
        Ok(())
    }
}
