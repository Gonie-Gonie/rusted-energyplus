//! Actual CalcZoneComponentLoadSums5414-5677, opaque uncontrolled Zone report.
use super::*;
impl SourceZoneSpaceHeatBalanceData {
    pub(super) fn calculate_source_component_loads(
        &self,
        number: i32,
        air_owners: &SourceAirHeatBalance,
        controller: &mut SourceZoneAirPredictorCorrector,
        owners: &mut SourceZoneAirOwners<'_, '_>,
    ) -> Result<(), Error> {
        let report = controller
            .reports
            .zones
            .as_mut()
            .and_then(|rows| rows.get_mut(index(number).ok()?))
            .ok_or(Error::Unavailable("same actual ZnAirRpt component row5440"))?;
        report.sum_int_gains = 0.0;
        report.sum_had_tsurfs = 0.0;
        report.sum_m_cp_d_tzones = 0.0;
        report.sum_m_cp_dt_infil = 0.0;
        report.sum_m_cp_d_tsystem = 0.0;
        report.sum_non_air_system = 0.0;
        report.czd_tdt = 0.0;
        report.im_balance = 0.0;
        report.sum_enthalpy_m = 0.0;
        report.sum_enthalpy_h = 0.0;
        let seconds = owners.hvac.time_step_sys_seconds;
        report.sum_int_gains =
            owners
                .gains
                .source_zone_convective_gain(owners.zones, number, false)?;
        if zone(owners, number)?.no_heat_to_return_air {
            report.sum_int_gains +=
                owners
                    .gains
                    .source_zone_convective_gain(owners.zones, number, true)?;
        }
        report.sum_m_cp_d_tzones = self.mcptm - self.mcpm * self.mat;
        report.sum_m_cp_dt_infil = (self.mcpti - self.mcpi * self.mat)
            + (self.mcptv - self.mcpv * self.mat)
            + (self.mcpte - self.mcpe * self.mat)
            + (self.mcptc - self.mcpc * self.mat)
            + (self.m_dot_cpoa * zone(owners, number)?.dry_bulb - self.m_dot_cpoa * self.mat);
        if controller
            .subsystems
            .airflow_network
            .replaces_simple_multizone()
        {
            return Err(Error::Unavailable(
                "positive component AFN exchangeData5479",
            ));
        }
        if zone(owners, number)?.is_controlled {
            return Err(Error::Unavailable("positive component ZoneEquip inlet5488"));
        } else if zone(owners, number)?.is_return_plenum {
            return Err(Error::Unavailable("positive component return plenum5510"));
        } else if zone(owners, number)?.is_supply_plenum {
            return Err(Error::Unavailable("positive component supply plenum5534"));
        }
        if !zone(owners, number)?
            .leakage_parallel_piu_numbers
            .is_empty()
        {
            return Err(Error::Unavailable("positive component PIU leak5543"));
        }
        report.sum_non_air_system = self.non_air_system_response
            + value(&owners.fan.sum_conv_ht_rad_sys, index(number)?)?
            + value(&owners.fan.sum_conv_pool, index(number)?)?;
        for &space_number in &zone(owners, number)?.space_indexes {
            let space = owners
                .zones
                .spaces
                .as_deref()
                .and_then(|rows| rows.get(index(space_number).ok()?))
                .ok_or(Error::Unavailable("actual component Space5562"))?;
            for surface_number in space.ht_surface_first..=space.ht_surface_last {
                let i = index(surface_number)?;
                let surface = owners
                    .geometry
                    .final_surfaces
                    .as_deref()
                    .and_then(|rows| rows.get(i))
                    .ok_or(Error::Unavailable("actual component Surface5565"))?;
                let area = surface.row.area;
                let air_reference = *owners
                    .geometry
                    .parallel_arrays
                    .as_ref()
                    .and_then(|a| a.surf_t_air_ref.get(i))
                    .ok_or(Error::Unavailable("actual component SurfTAirRef"))?;
                let reference = match air_reference {
                    0 => *air_owners.source_space_mat(surface.row.space)?,
                    1 => value(&owners.surface_arrays.surf_temp_eff_bulk_air, i)?,
                    2 => {
                        return Err(Error::Unavailable(
                            "positive component getInsideAirTemperature supply5566",
                        ));
                    }
                    _ => *air_owners.source_space_mat(surface.row.space)?,
                };
                if surface.row.class == SurfaceClass::Window {
                    return Err(Error::Unavailable("positive component Window shading5571"));
                }
                report.sum_had_tsurfs += value(&owners.surface_arrays.surf_h_conv_int, i)?
                    * area
                    * (value(&owners.surface_arrays.surf_temp_in_tmp, i)? - reference);
                // Real selected Surface algorithm enum admits only CTF/Invalid;
                // the positive CondFD getter/body is excluded, never approximated.
            }
        }
        let cp = energyplus_psy_cp_air_fn_w(self.air_hum_rat);
        let rho = energyplus_psy_rho_air_fn_pb_tdb_w(
            owners.environment.current_weather.out_baro_press,
            self.mat,
            self.air_hum_rat,
        );
        if rho < 0.0 {
            return Err(Error::Unavailable(
                "actual component negative-density diagnostic5630",
            ));
        }
        match owners.control.zone_air_solution {
            SelectedZoneAirSolution::ThirdOrder => {
                report.czd_tdt = rho
                    * cp
                    * zone(owners, number)?.volume
                    * zone(owners, number)?.sensible_capacity_multiplier
                    * (self.mat - self.ztm[0])
                    / seconds
            }
            SelectedZoneAirSolution::Euler => {
                return Err(Error::Unavailable("non-ThirdOrder component storage5640"));
            }
        }
        if owners.globals.display_zone_air_heat_balance_off_balance {
            report.im_balance = report.sum_int_gains
                + report.sum_had_tsurfs
                + report.sum_m_cp_d_tzones
                + report.sum_m_cp_dt_infil
                + report.sum_m_cp_d_tsystem
                + report.sum_non_air_system
                - report.czd_tdt;
            let threshold = 0.2
                * (report.sum_int_gains * report.sum_int_gains
                    + report.sum_had_tsurfs * report.sum_had_tsurfs
                    + report.sum_m_cp_d_tzones * report.sum_m_cp_d_tzones
                    + report.sum_m_cp_dt_infil * report.sum_m_cp_dt_infil
                    + report.sum_m_cp_d_tsystem * report.sum_m_cp_d_tsystem
                    + report.sum_non_air_system * report.sum_non_air_system
                    + report.czd_tdt * report.czd_tdt)
                    .sqrt();
            if report.im_balance.abs() > threshold
                && !owners.clock.warmup_flag
                && !owners.globals.doing_sizing
            {
                if zone(owners, number)?.air_heat_balance_imbalance_error_index == 0 {
                    return Err(Error::Unavailable(
                        "positive air imbalance ShowWarningMessage5659",
                    ));
                }
                return Err(Error::Unavailable("positive air imbalance recurring5667"));
            }
        }
        Ok(())
    }
}
