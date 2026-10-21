//! Literal ordinary-reporting Fanger499-815 and bounded CalcRadTemp2232.
//! Other clothing/MRT/RoomAir and comfort-thermostat routes keep explicit bounds.
use super::*;

// ObjexxFCL::pow_4 and root_4 group as square-of-square and sqrt-of-sqrt.
fn pow4(value: f64) -> f64 {
    let square = value * value;
    square * square
}
fn root4(value: f64) -> f64 {
    value.sqrt().sqrt()
}
fn fan_value(rows: &Option<Vec<f64>>, at: usize) -> Result<f64, Error> {
    rows.as_ref()
        .and_then(|rows| rows.get(at))
        .copied()
        .ok_or(Error::Unavailable(
            "actual Fanger radiant HVAC-to-person array",
        ))
}

impl SourceThermalComfort {
    pub(super) fn calc_source_fanger<H, D>(
        &mut self,
        o: &mut SourceThermalComfortOwners<'_, '_, H, D>,
    ) -> Result<(), Error> {
        self.people_num = 1;
        while self.people_num <= o.gains.totals[0] {
            let at = index(self.people_num)?;
            let (fanger, zone, activity, work, clothing_type, method, clothing, velocity) = {
                let people = o
                    .gains
                    .allocations
                    .people
                    .get(at)
                    .ok_or(Error::Unavailable("actual Fanger People owner"))?;
                (
                    people.fanger,
                    people.zone_ptr,
                    people.activity_level_sched,
                    people.work_eff_sched,
                    people.clothing_type,
                    people.clothing_method_sched,
                    people.clothing_sched,
                    people.air_velocity_sched,
                )
            };
            // Native takes this row reference before the false-Fanger continue.
            self.thermal_comfort_data
                .as_ref()
                .and_then(|rows| rows.get(at))
                .ok_or(Error::Unavailable("actual Fanger ThermalComfortData"))?;
            if fanger {
                self.zone_num = zone;
                let air = &o
                    .air
                    .zones
                    .as_ref()
                    .and_then(|rows| rows.get(index(zone).ok()?))
                    .ok_or(Error::Unavailable("actual Fanger Zone AirHB"))?
                    .air;
                self.air_temp = air.ztav_comf;
                if o.controller.subsystems.room_air.any_non_mixing {
                    return Err(Error::Unavailable(
                        "positive Fanger non-Mixing RoomAir TCMF/cross-vent branch554",
                    ));
                }
                self.rad_temp = self.calc_source_fanger_rad_temp(o, at)?;
                let humidity = o
                    .air
                    .zones
                    .as_ref()
                    .and_then(|rows| rows.get(index(zone).ok()?))
                    .ok_or(Error::Unavailable(
                        "actual Fanger comfort-averaged humidity",
                    ))?
                    .air
                    .air_hum_rat_avg_comf;
                self.rel_hum = o.psychrometrics.source_relative_humidity(
                    self.air_temp,
                    humidity,
                    o.environment.current_weather.out_baro_press,
                    "",
                    &mut SourcePsychrometricOwners {
                        clock: o.clock,
                        globals: o.globals,
                        environment: o.environment,
                        hvac: o.hvac,
                        recurring: o.recurring_errors,
                        backend: o.backend,
                    },
                )?;
                let people =
                    o.gains
                        .allocations
                        .people
                        .rows
                        .get_mut(at)
                        .ok_or(Error::Unavailable(
                            "actual mutable Fanger People T/RH owner",
                        ))?;
                people.temperature_in_zone = self.air_temp;
                people.relative_humidity_in_zone = self.rel_hum * 100.0;
                self.act_level = o.schedules.source_current_schedule_value(
                    activity.ok_or(Error::Unavailable("actual Fanger activity schedule"))?,
                )? / 1.8;
                self.work_eff = o.schedules.source_current_schedule_value(
                    work.ok_or(Error::Unavailable("actual Fanger work efficiency schedule"))?,
                )? * self.act_level;
                match clothing_type {
                    0 => {
                        self.clo_unit =
                            o.schedules
                                .source_current_schedule_value(clothing.ok_or(
                                    Error::Unavailable("actual Fanger clothing schedule"),
                                )?)?;
                        self.thermal_comfort_data
                            .as_mut()
                            .and_then(|rows| rows.get_mut(at))
                            .ok_or(Error::Unavailable("actual Fanger clothing output owner"))?
                            .clothing_value = self.clo_unit;
                    }
                    1 => {
                        let comfort = self
                            .thermal_comfort_data
                            .as_mut()
                            .and_then(|rows| rows.get_mut(at))
                            .ok_or(Error::Unavailable("actual dynamic clothing retained row"))?;
                        comfort.thermal_comfort_op_temp = (self.rad_temp + self.air_temp) / 2.0;
                        comfort.clothing_value = self.clo_unit;
                        self.calc_source_dynamic_clothing()?;
                        self.clo_unit = self
                            .thermal_comfort_data
                            .as_ref()
                            .and_then(|rows| rows.get(at))
                            .ok_or(Error::Unavailable("actual dynamic clothing result"))?
                            .clothing_value;
                    }
                    2 => {
                        let intermediate =
                            o.schedules.source_current_schedule_value(method.ok_or(
                                Error::Unavailable("actual clothing calculation method schedule"),
                            )?)?;
                        if intermediate == 1.0 {
                            self.clo_unit =
                                o.schedules.source_current_schedule_value(clothing.ok_or(
                                    Error::Unavailable("actual method1 clothing schedule"),
                                )?)?;
                            self.thermal_comfort_data
                                .as_mut()
                                .and_then(|rows| rows.get_mut(at))
                                .ok_or(Error::Unavailable("actual method1 clothing retained row"))?
                                .clothing_value = self.clo_unit;
                        } else if intermediate == 2.0 {
                            let comfort = self
                                .thermal_comfort_data
                                .as_mut()
                                .and_then(|rows| rows.get_mut(at))
                                .ok_or(Error::Unavailable(
                                    "actual method2 clothing retained row",
                                ))?;
                            comfort.thermal_comfort_op_temp = (self.rad_temp + self.air_temp) / 2.0;
                            comfort.clothing_value = self.clo_unit;
                            self.calc_source_dynamic_clothing()?;
                            self.clo_unit = self
                                .thermal_comfort_data
                                .as_ref()
                                .and_then(|rows| rows.get(at))
                                .ok_or(Error::Unavailable("actual method2 clothing result"))?
                                .clothing_value;
                        } else {
                            // Native reads the schedule, warns on every such call, and
                            // does not assign comfort.ClothingValue on this fallback.
                            self.clo_unit =
                                o.schedules.source_current_schedule_value(clothing.ok_or(
                                    Error::Unavailable("actual fallback clothing schedule"),
                                )?)?;
                            let name = &o
                                .gains
                                .allocations
                                .people
                                .get(at)
                                .ok_or(Error::Unavailable("actual fallback People name"))?
                                .name;
                            o.backend.show_warning_error(&format!("PEOPLE=\"{name}\", Scheduled clothing value will be used rather than clothing calculation method."))?;
                        }
                    }
                    _ => return Err(Error::Unavailable("actual bounded Fanger clothing enum")),
                }
                self.air_vel = o.schedules.source_current_schedule_value(
                    velocity.ok_or(Error::Unavailable("actual Fanger air speed schedule"))?,
                )?;
                // Ordinary reporting has no PNum/Tset: Native's comfort-control
                // air-speed warning633 and return-PMV argument are not selected.
                let pmv = self.calc_source_fanger_pmv(
                    self.air_temp,
                    self.rad_temp,
                    self.rel_hum,
                    self.air_vel,
                    self.act_level,
                    self.clo_unit,
                    self.work_eff,
                    o.psychrometrics,
                    &mut SourcePsychrometricOwners {
                        clock: o.clock,
                        globals: o.globals,
                        environment: o.environment,
                        hvac: o.hvac,
                        recurring: o.recurring_errors,
                        backend: o.backend,
                    },
                )?;
                let comfort = self
                    .thermal_comfort_data
                    .as_mut()
                    .and_then(|rows| rows.get_mut(at))
                    .ok_or(Error::Unavailable("actual Fanger retained results"))?;
                comfort.fanger_pmv = pmv;
                comfort.thermal_comfort_mrt = self.rad_temp;
                comfort.thermal_comfort_op_temp = (self.rad_temp + self.air_temp) / 2.0;
                comfort.clo_surf_temp = self.clo_surf_temp;
                comfort.fanger_ppd = Self::calc_source_fanger_ppd(pmv);
            }
            // There is no occupancy guard in Native's reporting Fanger loop.
            self.people_num = self
                .people_num
                .checked_add(1)
                .ok_or(Error::Unavailable("actual Fanger PeopleNum increment"))?;
        }
        Ok(())
    }

    fn calc_source_fanger_rad_temp<H, D>(
        &mut self,
        o: &mut SourceThermalComfortOwners<'_, '_, H, D>,
        people_at: usize,
    ) -> Result<f64, Error> {
        let people = o
            .gains
            .allocations
            .people
            .get(people_at)
            .ok_or(Error::Unavailable("actual CalcRadTemp People"))?;
        match people.mrt_calc_type {
            0 => {
                let enclosure = o
                    .zones
                    .spaces
                    .as_ref()
                    .and_then(|rows| rows.get(index(people.space_index).ok()?))
                    .ok_or(Error::Unavailable("actual CalcRadTemp People Space"))?
                    .radiant_enclosure_num;
                self.rad_temp = o
                    .radiation
                    .radiant_enclosures()
                    .get(index(enclosure)?)
                    .ok_or(Error::Unavailable(
                        "actual CalcRadTemp retained enclosure MRT",
                    ))?
                    .mean_radiant_temperature;
            }
            1 => {
                self.rad_temp = self.calc_source_surface_weighted_mrt(
                    people.surface_ptr,
                    true,
                    o.geometry,
                    o.control,
                    o.radiation,
                    o.history,
                    o.zones,
                    o.air,
                    o.backend,
                )?;
            }
            2 => {
                self.rad_temp = self.calc_source_angle_factor_mrt(
                    people.angle_factor_list_ptr,
                    o.geometry,
                    o.control,
                    o.history,
                )?;
            }
            _ => return Err(Error::Unavailable("actual invalid Fanger MRT selection")),
        }
        let zone = index(self.zone_num)?;
        let radiation_to_person = fan_value(&o.fan.zone_q_ht_rad_sys_to_person, zone)?
            + fan_value(&o.fan.zone_q_cooling_panel_to_person, zone)?
            + fan_value(&o.fan.zone_q_hw_baseboard_to_person, zone)?
            + fan_value(&o.fan.zone_q_steam_baseboard_to_person, zone)?
            + fan_value(&o.fan.zone_q_elec_baseboard_to_person, zone)?;
        *o.fan
            .zone_qdot_rad_hvac_to_person
            .as_mut()
            .and_then(|rows| rows.get_mut(zone))
            .ok_or(Error::Unavailable("actual mutable ZoneQdotRadHVACToPerson"))? =
            radiation_to_person;
        if radiation_to_person > 0.0 {
            self.rad_temp += 273.15;
            self.rad_temp = root4(pow4(self.rad_temp) + (radiation_to_person / 1.8 / 5.6697e-8));
            self.rad_temp -= 273.15;
        }
        Ok(self.rad_temp)
    }

    /// Literal CalcAngleFactorMRT2075-2102, in retained list row order.
    /// Final Source Surface ordinals map through typed IDs to real term1 history.
    fn calc_source_angle_factor_mrt(
        &self,
        number: i32,
        geometry: &SurfaceGeometryInputState,
        control: &SelectedHeatBalanceInputState,
        history: &CtfThermalHistoryState,
    ) -> Result<f64, Error> {
        let list = self
            .angle_factor_list
            .as_ref()
            .and_then(|lists| lists.get(index(number).ok()?))
            .ok_or(Error::Unavailable(
                "actual CalcAngleFactorMRT retained list",
            ))?;
        let surfaces = geometry
            .final_surfaces
            .as_deref()
            .ok_or(Error::Unavailable(
                "actual CalcAngleFactorMRT final Surfaces",
            ))?;
        let mut surface_temp_emiss_angle_fac_summed = 0.0;
        let mut sum_surface_emiss_angle_factor = 0.0;
        for at in 0..count(list.total_angle_factor_surfaces)? {
            let number = *list
                .surface_ptr
                .get(at)
                .ok_or(Error::Unavailable("actual CalcAngleFactorMRT SurfacePtr"))?;
            let surface = surfaces
                .get(index(number)?)
                .ok_or(Error::Unavailable("actual CalcAngleFactorMRT Surface"))?;
            let id = surface.typed_id.ok_or(Error::Unavailable(
                "actual CalcAngleFactorMRT typed Surface ID",
            ))?;
            let history_at = history
                .surface_ids
                .iter()
                .position(|candidate| *candidate == id)
                .ok_or(Error::Unavailable(
                    "actual CalcAngleFactorMRT same history mapping",
                ))?;
            let surface_temp =
                *history.normal.inside_temperature[0]
                    .get(history_at)
                    .ok_or(Error::Unavailable(
                        "actual CalcAngleFactorMRT SurfInsideTempHist term1",
                    ))?
                    + 273.15;
            let emissivity = control
                .construction_usage
                .get(index(surface.row.construction)?)
                .ok_or(Error::Unavailable("actual CalcAngleFactorMRT construction"))?
                .inside_absorp_thermal;
            let surface_eaf = emissivity
                * *list.angle_factor.get(at).ok_or(Error::Unavailable(
                    "actual CalcAngleFactorMRT angle operand",
                ))?;
            surface_temp_emiss_angle_fac_summed += surface_eaf * pow4(surface_temp);
            sum_surface_emiss_angle_factor += surface_eaf;
        }
        Ok(root4(surface_temp_emiss_angle_fac_summed / sum_surface_emiss_angle_factor) - 273.15)
    }

    /// Literal CalcSurfaceWeightedMRT2105-2198 over the same Surface fields,
    /// enclosure order and term1 histories. No CalculateZoneMRT array is reused.
    pub(crate) fn calc_source_surface_weighted_mrt(
        &mut self,
        surface_number: i32,
        average_with_surface: bool,
        geometry: &mut SurfaceGeometryInputState,
        control: &SelectedHeatBalanceInputState,
        radiation: &OrdinarySolarRadiation,
        history: &CtfThermalHistoryState,
        zones: &SelectedZoneInputOwners,
        air: &SourceAirHeatBalance,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<f64, Error> {
        let surfaces = geometry
            .final_surfaces
            .as_deref_mut()
            .ok_or(Error::Unavailable(
                "actual SurfaceWeighted final Surface allocation",
            ))?;
        if self.first_time_surface_weighted_flag {
            self.first_time_surface_weighted_error = true;
            self.first_time_surface_weighted_flag = false;
            for enclosure in radiation.radiant_enclosures() {
                let pointers =
                    enclosure
                        .source_surface_ptr
                        .as_deref()
                        .ok_or(Error::Unavailable(
                            "actual SurfaceWeighted enclosure SurfacePtr",
                        ))?;
                for &number in pointers {
                    let surface = &mut surfaces
                        .get_mut(index(number)?)
                        .ok_or(Error::Unavailable("actual SurfaceWeighted initial Surface"))?
                        .row;
                    let emissivity = control
                        .construction_usage
                        .get(index(surface.construction)?)
                        .ok_or(Error::Unavailable("actual SurfaceWeighted construction"))?
                        .inside_absorp_thermal;
                    surface.ae = surface.area * emissivity;
                }
                for &number in pointers {
                    surfaces
                        .get_mut(index(number)?)
                        .ok_or(Error::Unavailable("actual SurfaceWeighted target"))?
                        .row
                        .enclosure_ae_sum = 0.0;
                    for &other in pointers {
                        if other == number {
                            continue;
                        }
                        let ae = surfaces
                            .get(index(other)?)
                            .ok_or(Error::Unavailable("actual SurfaceWeighted other AE"))?
                            .row
                            .ae;
                        surfaces
                            .get_mut(index(number)?)
                            .ok_or(Error::Unavailable("actual SurfaceWeighted sum target"))?
                            .row
                            .enclosure_ae_sum += ae;
                    }
                }
            }
        }
        let selected = index(surface_number)?;
        let enclosure_number = surfaces
            .get(selected)
            .ok_or(Error::Unavailable(
                "actual SurfaceWeighted selected Surface",
            ))?
            .row
            .rad_encl_index;
        let enclosure = radiation
            .radiant_enclosures()
            .get(index(enclosure_number)?)
            .ok_or(Error::Unavailable(
                "actual SurfaceWeighted selected radiant enclosure",
            ))?;
        let pointers = enclosure
            .source_surface_ptr
            .as_deref()
            .ok_or(Error::Unavailable(
                "actual SurfaceWeighted selected SurfacePtr",
            ))?;
        if enclosure.rad_recalc {
            surfaces
                .get_mut(selected)
                .ok_or(Error::Unavailable(
                    "actual SurfaceWeighted recalculation target",
                ))?
                .row
                .enclosure_ae_sum = 0.0;
            for &other in pointers {
                if other == surface_number {
                    continue;
                }
                let surface = &mut surfaces
                    .get_mut(index(other)?)
                    .ok_or(Error::Unavailable("actual SurfaceWeighted recalc other"))?
                    .row;
                let emissivity = control
                    .construction_usage
                    .get(index(surface.construction)?)
                    .ok_or(Error::Unavailable(
                        "actual SurfaceWeighted recalc construction",
                    ))?
                    .inside_absorp_thermal;
                surface.ae = surface.area * emissivity;
                let ae = surface.ae;
                surfaces
                    .get_mut(selected)
                    .ok_or(Error::Unavailable("actual SurfaceWeighted recalc sum"))?
                    .row
                    .enclosure_ae_sum += ae;
            }
        }
        let temperature = |number: i32| -> Result<f64, Error> {
            let id = surfaces
                .get(index(number)?)
                .and_then(|surface| surface.typed_id)
                .ok_or(Error::Unavailable(
                    "actual SurfaceWeighted typed Surface identity",
                ))?;
            let at = history
                .surface_ids
                .iter()
                .position(|candidate| *candidate == id)
                .ok_or(Error::Unavailable(
                    "actual SurfaceWeighted same history mapping",
                ))?;
            history.normal.inside_temperature[0]
                .get(at)
                .copied()
                .ok_or(Error::Unavailable(
                    "actual SurfaceWeighted SurfInsideTempHist term1",
                ))
        };
        let mut sum_aet = 0.0;
        for &other in pointers {
            if other == surface_number {
                continue;
            }
            sum_aet += surfaces
                .get(index(other)?)
                .ok_or(Error::Unavailable("actual SurfaceWeighted AET surface"))?
                .row
                .ae
                * temperature(other)?;
        }
        let this_temperature = temperature(surface_number)?;
        let surface = &surfaces
            .get(selected)
            .ok_or(Error::Unavailable("actual SurfaceWeighted retained target"))?
            .row;
        let mut result = 0.0;
        if surface.enclosure_ae_sum > 0.01 {
            result = sum_aet / surface.enclosure_ae_sum;
            if average_with_surface {
                result = 0.5 * (this_temperature + result);
            }
        } else if self.first_time_surface_weighted_error {
            backend.show_warning_error(&format!("CalcSurfaceWeightedMRT: Areas*Inside surface emissivities are summing to zero for Enclosure=\"{}\"", enclosure.name))?;
            let space = index(surface.space)?;
            let name = &zones
                .spaces
                .as_deref()
                .and_then(|spaces| spaces.get(space))
                .ok_or(Error::Unavailable("actual SurfaceWeighted Space name"))?
                .name;
            backend.show_continue_error(&format!("As a result, the MAT for Space={} will be used for MRT when calculating the surface weighted MRT.", name))?;
            backend.show_continue_error(&format!("for Surface={}", surface.name))?;
            self.first_time_surface_weighted_error = false;
            result = air
                .spaces
                .as_deref()
                .and_then(|spaces| spaces.get(space))
                .ok_or(Error::Unavailable(
                    "actual SurfaceWeighted spaceHeatBalance MAT",
                ))?
                .air
                .mat;
            if average_with_surface {
                result = 0.5 * (this_temperature + result);
            }
        }
        Ok(result)
    }

    /// Native DynamicClothingModel3190-3211 writes only the current retained
    /// row. ManageThermalComfort owns the actual six-AM weather state update.
    #[allow(
        clippy::manual_range_contains,
        reason = "Preserve Native ordered bounds and NaN no-assignment path"
    )]
    pub(crate) fn calc_source_dynamic_clothing(&mut self) -> Result<(), Error> {
        let temperature = self.temporary_six_am_temperature;
        let comfort = self
            .thermal_comfort_data
            .as_mut()
            .and_then(|rows| rows.get_mut(index(self.people_num).ok()?))
            .ok_or(Error::Unavailable("actual DynamicClothingModel People row"))?;
        if temperature < -5.0 {
            comfort.clothing_value = 1.0;
        } else if temperature >= -5.0 && temperature < 5.0 {
            comfort.clothing_value = 0.818 - 0.0364 * temperature;
        } else if temperature >= 5.0 && temperature < 26.0 {
            let temporary = -0.1635 - 0.0066 * temperature;
            comfort.clothing_value = 10.0_f64.powf(temporary);
        } else if temperature >= 26.0 {
            comfort.clothing_value = 0.46;
        }
        Ok(())
    }

    /// Same scalar child as Native CalcFangerPMV676, over the real property state.
    /// Public within the crate for the existing direct scalar-owner check.
    pub(crate) fn calc_source_fanger_pmv(
        &mut self,
        air_temp: f64,
        rad_temp: f64,
        rel_hum: f64,
        air_vel: f64,
        act_level: f64,
        clo_unit: f64,
        work_eff: f64,
        psychrometrics: &mut SourcePsychrometricAccess,
        owners: &mut SourcePsychrometricOwners<'_, '_>,
    ) -> Result<f64, Error> {
        self.vap_press = psychrometrics.source_saturation_pressure(air_temp, "", owners)?;
        self.vap_press *= rel_hum;
        self.int_heat_prod = act_level - work_eff;
        let std_icl = 0.155 * clo_unit;
        self.clo_body_rat = if std_icl < 0.078 {
            1.0 + 1.29 * std_icl
        } else {
            1.05 + 0.645 * std_icl
        };
        self.abs_rad_temp = rad_temp + 273.15;
        self.abs_air_temp = air_temp + 273.15;
        self.clo_insul = std_icl * self.clo_body_rat;
        let p2 = self.clo_insul * 3.96;
        let p3 = self.clo_insul * 100.0;
        let p1 = self.clo_insul * self.abs_air_temp;
        let p4 = 308.7 - 0.028 * self.int_heat_prod + p2 * pow4(self.abs_rad_temp / 100.0);
        self.abs_clo_surf_temp = self.abs_air_temp + (35.5 - air_temp) / (3.5 * std_icl + 0.1);
        let mut xn = self.abs_clo_surf_temp / 100.0;
        self.hc_for = 12.1 * air_vel.sqrt();
        self.iter_num = 0;
        let mut xf = xn;
        while (((xn - xf).abs() > 0.00015) || self.iter_num == 0) && self.iter_num < 150 {
            xf = (xf + xn) / 2.0;
            self.hc_nat = 2.38 * root4((100.0 * xf - self.abs_air_temp).abs());
            // Native max(forced,natural) retains a NaN first argument.
            self.hc = if self.hc_for < self.hc_nat {
                self.hc_nat
            } else {
                self.hc_for
            };
            xn = (p4 + p1 * self.hc - p2 * pow4(xf)) / (100.0 + p3 * self.hc);
            self.iter_num += 1;
            // Preserve Native's exact >MaxIter predicate inside the <MaxIter
            // loop; do not invent a warning at equality150.
            if self.iter_num > 150 {
                owners
                    .backend
                    .show_warning_error("Max iteration exceeded in CalcThermalFanger")?;
            }
        }
        self.abs_clo_surf_temp = 100.0 * xn;
        self.clo_surf_temp = self.abs_clo_surf_temp - 273.15;
        self.rad_heat_loss = 3.96
            * self.clo_body_rat
            * (pow4(self.abs_clo_surf_temp * 0.01) - pow4(self.abs_rad_temp * 0.01));
        self.conv_heat_loss = self.clo_body_rat * self.hc * (self.clo_surf_temp - air_temp);
        self.dry_heat_loss = self.rad_heat_loss + self.conv_heat_loss;
        self.evap_heat_loss_reg_comf = 0.0;
        if self.int_heat_prod > 58.2 {
            self.evap_heat_loss_reg_comf = 0.42 * (self.int_heat_prod - 58.2);
        }
        self.evap_heat_loss_diff =
            3.05 * 0.001 * (5733.0 - 6.99 * self.int_heat_prod - self.vap_press);
        self.evap_heat_loss = self.evap_heat_loss_reg_comf + self.evap_heat_loss_diff;
        self.lat_resp_heat_loss = 1.7 * 0.00001 * act_level * (5867.0 - self.vap_press);
        self.dry_resp_heat_loss = 0.0014 * act_level * (34.0 - air_temp);
        self.resp_heat_loss = self.lat_resp_heat_loss + self.dry_resp_heat_loss;
        self.therm_sens_trans_coef = 0.303 * (-0.036 * act_level).exp() + 0.028;
        Ok(self.therm_sens_trans_coef
            * (self.int_heat_prod - self.evap_heat_loss - self.resp_heat_loss - self.dry_heat_loss))
    }

    #[allow(
        clippy::manual_clamp,
        reason = "Retain Original810-814 predicate order and NaN pass-through"
    )]
    fn calc_source_fanger_ppd(pmv: f64) -> f64 {
        let exp_test = -0.03353 * pow4(pmv) - 0.2179 * (pmv * pmv);
        let mut ppd = if exp_test > -20.0 {
            100.0 - 95.0 * exp_test.exp()
        } else {
            100.0
        };
        if ppd < 0.0 {
            ppd = 0.0;
        } else if ppd > 100.0 {
            ppd = 100.0;
        }
        ppd
    }
}
