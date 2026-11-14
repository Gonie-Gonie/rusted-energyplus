//! Native calcPredictedHumidityRatio3461-3815, real normal-mixing producer.
use super::*;
use crate::psychrometrics::SourcePsychrometricOwners;

fn humidity_setpoint(
    temperature: f64,
    rh: f64,
    owners: &mut SourceZoneAirOwners<'_, '_>,
) -> Result<f64, Error> {
    owners
        .psychrometrics
        .source_humidity_ratio_from_relative_humidity(
            temperature,
            rh / 100.0,
            owners.environment.current_weather.out_baro_press,
            "calcPredictedHumidityRatio",
            &mut SourcePsychrometricOwners {
                clock: owners.clock,
                globals: owners.globals,
                environment: owners.environment,
                hvac: &*owners.hvac,
                recurring: &mut *owners.recurring_errors,
                backend: &mut *owners.backend,
            },
        )
}

impl SourceZoneSpaceHeatBalanceData {
    pub(super) fn calculate_source_predicted_humidity(
        &mut self,
        rafn_fraction: f64,
        number: i32,
        controller: &mut SourceZoneAirPredictorCorrector,
        owners: &mut SourceZoneAirOwners<'_, '_>,
    ) -> Result<(), Error> {
        let humidity_index = zone(owners, number)?.humidity_control_zone_index;
        if humidity_index <= 0 {
            // The existing no-controlled-humidity branch remains literal.
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
            return Ok(());
        }
        let (rh_humidifying, rh_dehumidifying, single_setpoint) = {
            let row = controller
                .controls
                .humidity_controlled_zones
                .as_mut()
                .and_then(|a| a.get_mut(index(humidity_index).ok()?))
                .ok_or(Error::Unavailable(
                    "actual positive HumidityControlZone3492",
                ))?;
            if row.actual_zone_number != number {
                return Err(Error::Unavailable(
                    "actual humidity-control Zone identity3493",
                ));
            }
            let mut humidifying =
                owners
                    .schedules
                    .source_current_schedule_value(row.humidifying_schedule.ok_or(
                        Error::Unavailable("actual humidifying schedule pointer3494"),
                    )?)?;
            let mut dehumidifying = owners.schedules.source_current_schedule_value(
                row.dehumidifying_schedule.ok_or(Error::Unavailable(
                    "actual dehumidifying schedule pointer3495",
                ))?,
            )?;
            // These genuine retained constructor fields have no newly admitted
            // EMS writer; ordinary Source EMS ingress remains blocked.
            if row.ems_override_humidify_setpoint_on {
                humidifying = row.ems_override_humidify_setpoint_value;
            }
            if row.ems_override_dehumidify_setpoint_on {
                dehumidifying = row.ems_override_dehumidify_setpoint_value;
            }
            if *owners.faults.source_number_faulty_humidistat() > 0
                && !owners.clock.warmup_flag
                && !owners.globals.doing_sizing
                && !owners.globals.kickoff_simulation
            {
                return Err(Error::Unavailable(
                    "actual positive humidistat fault offset child3509",
                ));
            }
            if humidifying > dehumidifying {
                if row.error_index == 0 {
                    owners.backend.show_warning_message(&format!(
                        "HUMIDISTAT: The humidifying setpoint is above the dehumidifying setpoint in {}",
                        row.control_name,
                    ))?;
                    owners.backend.show_continue_error(
                        "The zone humidifying setpoint is set to the dehumidifying setpoint.",
                    )?;
                    owners.backend.show_source_continue_error_time_stamp(
                        "Occurrence info:",
                        owners.clock,
                        owners.globals,
                        owners.environment,
                        &*owners.hvac,
                    )?;
                }
                owners.backend.show_source_recurring_warning_at_end(
                    "The humidifying setpoint is still above the dehumidifying setpoint",
                    &mut row.error_index,
                    owners.recurring_errors,
                    owners.clock,
                    owners.globals,
                    Some(humidifying),
                    Some(humidifying),
                    None,
                    "",
                    "",
                    "",
                )?;
                humidifying = dehumidifying;
            }
            (humidifying, dehumidifying, humidifying == dehumidifying)
        };
        // Native normal-mixing coefficients: no HVAC moisture output is used to
        // invent demand. Every term is an existing heat-balance owner.
        let latent_gain = self.latent_gain
            + value(&owners.fan.sum_latent_ht_rad_sys, index(number)?)?
            + value(&owners.fan.sum_latent_pool, index(number)?)?;
        let seconds = owners.hvac.time_step_sys_seconds;
        let developer = owners
            .reporting_diagnostic_flags
            .as_ref()
            .map(|flags| flags.developer)
            .ok_or(Error::Unavailable(
                "actual Humidistat psychrometric diagnostic flags3681",
            ))?;
        let rho = owners.psychrometrics.source_air_density(
            owners.environment.current_weather.out_baro_press,
            self.zt,
            self.air_hum_rat,
            "calcPredictedHumidityRatio",
            developer,
            &mut SourcePsychrometricOwners {
                clock: owners.clock,
                globals: owners.globals,
                environment: owners.environment,
                hvac: &*owners.hvac,
                recurring: &mut *owners.recurring_errors,
                backend: &mut *owners.backend,
            },
        )?;
        let vapor_enthalpy = energyplus_psy_hg_air_fn_w_tdb(self.air_hum_rat, self.zt);
        if controller
            .subsystems
            .airflow_network
            .replaces_simple_multizone()
        {
            return Err(Error::Unavailable(
                "actual predicted humidity AFN exchangeData3692",
            ));
        }
        let b = (latent_gain / vapor_enthalpy)
            + ((self.oamfl + self.vamfl + self.ctmfl)
                * owners.environment.current_weather.out_hum_rat)
            + self.eamf_lx_hum_rat
            + self.sum_hm_a_ra_w
            + self.mixing_mass_flow_x_hum_rat
            + self.m_dot_oa * owners.environment.current_weather.out_hum_rat;
        let a = self.oamfl
            + self.vamfl
            + self.eamfl
            + self.ctmfl
            + self.sum_hm_a_ra
            + self.mixing_mass_flow_zone
            + self.m_dot_oa;
        let c =
            rho * zone(owners, number)?.volume * zone(owners, number)?.moisture_capacity_multiplier
                / seconds;
        if controller.subsystems.room_air.model(number)? == 7 {
            return Err(Error::Unavailable(
                "actual predicted humidity RoomAFN coefficient child3711",
            ));
        }
        let w_humidifying = humidity_setpoint(self.zt, rh_humidifying, owners)?;
        let mut exp_700_a_c = 0.0;
        let mut load_humidifying = match owners.control.zone_air_solution {
            SelectedZoneAirSolution::ThirdOrder => {
                ((11.0 / 6.0) * c + a) * w_humidifying
                    - (b + c
                        * (3.0 * self.w_prev_zone_ts_temp[0]
                            - (3.0 / 2.0) * self.w_prev_zone_ts_temp[1]
                            + (1.0 / 3.0) * self.w_prev_zone_ts_temp[2]))
            }
            SelectedZoneAirSolution::Analytical => {
                if a == 0.0 {
                    c * (w_humidifying - self.w1) - b
                } else {
                    exp_700_a_c = cpp_min(700.0, -a / c).exp();
                    a * (w_humidifying - self.w1 * exp_700_a_c) / (1.0 - exp_700_a_c) - b
                }
            }
            SelectedZoneAirSolution::Euler => c * (w_humidifying - self.w1) + a * w_humidifying - b,
        };
        if rafn_fraction > 0.0 {
            load_humidifying /= rafn_fraction;
        }
        let w_dehumidifying = humidity_setpoint(self.zt, rh_dehumidifying, owners)?;
        let mut load_dehumidifying = match owners.control.zone_air_solution {
            SelectedZoneAirSolution::ThirdOrder => {
                ((11.0 / 6.0) * c + a) * w_dehumidifying
                    - (b + c
                        * (3.0 * self.w_prev_zone_ts_temp[0]
                            - (3.0 / 2.0) * self.w_prev_zone_ts_temp[1]
                            + (1.0 / 3.0) * self.w_prev_zone_ts_temp[2]))
            }
            SelectedZoneAirSolution::Analytical => {
                if a == 0.0 {
                    c * (w_dehumidifying - self.w1) - b
                } else {
                    a * (w_dehumidifying - self.w1 * exp_700_a_c) / (1.0 - exp_700_a_c) - b
                }
            }
            SelectedZoneAirSolution::Euler => {
                c * (w_dehumidifying - self.w1) + a * w_dehumidifying - b
            }
        };
        if rafn_fraction > 0.0 {
            load_dehumidifying /= rafn_fraction;
        }
        let total = if single_setpoint || (load_humidifying > 0.0 && load_dehumidifying > 0.0) {
            load_humidifying
        } else if load_humidifying < 0.0 && load_dehumidifying < 0.0 {
            load_dehumidifying
        } else if load_humidifying <= 0.0 && load_dehumidifying >= 0.0 {
            0.0
        } else {
            let zone_name = zone(owners, number)?.name.clone();
            let flags = owners
                .reporting_diagnostic_flags
                .as_ref()
                .ok_or(Error::Unavailable(
                    "actual Humidistat severe/fatal diagnostic flags3780",
                ))?;
            let round = |value, precision| {
                crate::heat_balance::source_round_format::round(value, precision)
                    .map_err(|error| Error::Format(format!("Humidistat .{precision}R: {error:?}")))
            };
            owners.backend.show_source_severe_error(
                "Humidistat: Unanticipated combination of humidifying and dehumidifying loads - report to EnergyPlus Development Team",
                flags,
            )?;
            owners.backend.show_source_continue_error_time_stamp(
                &format!("occurs in Zone = {zone_name}"),
                owners.clock,
                owners.globals,
                owners.environment,
                &*owners.hvac,
            )?;
            owners.backend.show_continue_error(&format!(
                "LoadToHumidifySetPoint={}, LoadToDehumidifySetPoint={}",
                round(load_humidifying, 5)?,
                round(load_dehumidifying, 5)?,
            ))?;
            owners.backend.show_continue_error(&format!(
                "Zone RH Humidifying Set-point={}",
                round(rh_humidifying, 1)?,
            ))?;
            owners.backend.show_continue_error(&format!(
                "Zone RH Dehumidifying Set-point={}",
                round(rh_dehumidifying, 2)?,
            ))?;
            return match owners
                .backend
                .show_source_fatal_error("Program terminates due to above conditions.", flags)
            {
                Ok(never) => match never {},
                Err(error) => Err(error),
            };
        };
        let zone = zone(owners, number)?;
        controller
            .demand
            .zone_moisture
            .as_mut()
            .and_then(|a| a.get_mut(index(number).ok()?))
            .ok_or(Error::Unavailable(
                "actual positive ZoneSysMoistureDemand3806",
            ))?
            .report_source_predicted_moisture(
                total,
                load_humidifying,
                load_dehumidifying,
                zone.multiplier,
                zone.list_multiplier,
                zone.is_controlled,
            )?;
        Ok(())
    }
}
