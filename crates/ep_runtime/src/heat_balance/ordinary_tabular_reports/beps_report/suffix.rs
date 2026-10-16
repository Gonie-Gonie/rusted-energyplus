//! Actual BEPS9042-9660 normalized, electric, thermal, water and comfort suffix.
use super::helpers::{predef_bytes, predef_real};
use super::prepare::{SourceBepsLocals, SourceBepsPass};
use super::{Error, SourceBepsReportOwners, SourceTabularBody, SourceTabularReports};
impl SourceTabularReports {
    pub(super) fn source_beps_electric_table(
        &mut self,
        pass: usize,
        p: &SourceBepsPass,
        l: &SourceBepsLocals,
        unconvert_ip: f64,
        o: &mut SourceBepsReportOwners<'_, '_>,
    ) -> Result<(), Error> {
        let mut rows = vec![Vec::new(); 14];
        let mut cols = vec![Vec::new(); 2];
        let mut widths = vec![14; 2];
        let mut body = SourceTabularBody::allocate(2, 14)?;
        cols[0] = match self.input_state.tabular_report_passes[pass].units_style {
            1 => b"Electricity [kWh]".to_vec(),
            4 | 5 => b"Electricity [kBtu]".to_vec(),
            _ => b"Electricity [GJ]".to_vec(),
        };
        cols[1] = b"Percent Electricity [%]".to_vec();
        rows[0] = b"Fuel-Fired Power Generation".to_vec();
        rows[1] = b"High Temperature Geothermal*".to_vec();
        rows[2] = b"Photovoltaic Power".to_vec();
        rows[3] = b"Wind Power".to_vec();
        rows[4] = b"Power Conversion".to_vec();
        rows[5] = b"Net Decrease in On-Site Storage".to_vec();
        rows[6] = b"Total On-Site Electric Sources".to_vec();
        rows[7] = b"".to_vec();
        rows[8] = b"Electricity Coming From Utility".to_vec();
        rows[9] = b"Surplus Electricity Going To Utility".to_vec();
        rows[10] = b"Net Electricity From Utility".to_vec();
        rows[11] = b"".to_vec();
        rows[12] = b"Total On-Site and Utility Electric Sources".to_vec();
        rows[13] = b"Total Electricity End Uses".to_vec();
        if self.input_state.tabular_report_passes[pass].units_style == 5 {
            cols[0] = b"Electricity [kWh]".to_vec();
        }
        body.set(
            1,
            1,
            self.beps_real(pass, self.input_state.gather_power_fuel_fire_gen, 3)?,
        )?;
        body.set(
            1,
            2,
            self.beps_real(pass, self.input_state.gather_power_ht_geothermal, 3)?,
        )?;
        body.set(
            1,
            3,
            self.beps_real(pass, self.input_state.gather_power_pv, 3)?,
        )?;
        if self.input_state.tabular_report_passes[pass].produce_tabular {
            predef_real(
                o,
                "pdchLeedRenAnGen",
                b"Photovoltaic",
                unconvert_ip * self.input_state.gather_power_pv,
                2,
            )?;
        }
        body.set(
            1,
            4,
            self.beps_real(pass, self.input_state.gather_power_wind, 3)?,
        )?;
        if self.input_state.tabular_report_passes[pass].produce_tabular {
            predef_real(
                o,
                "pdchLeedRenAnGen",
                b"Wind",
                unconvert_ip * self.input_state.gather_power_wind,
                2,
            )?;
        }
        body.set(
            1,
            5,
            self.beps_real(pass, self.input_state.gather_power_conversion, 3)?,
        )?;
        body.set(
            1,
            6,
            self.beps_real(pass, self.input_state.overall_net_energy_from_storage, 3)?,
        )?;
        body.set(
            1,
            7,
            self.beps_real(pass, self.input_state.gather_electric_produced, 3)?,
        )?;
        body.set(
            1,
            9,
            self.beps_real(pass, self.input_state.gather_electric_purchased, 3)?,
        )?;
        body.set(
            1,
            10,
            self.beps_real(pass, self.input_state.gather_electric_surplus_sold, 3)?,
        )?;
        body.set(
            1,
            11,
            self.beps_real(
                pass,
                self.input_state.gather_electric_purchased
                    - self.input_state.gather_electric_surplus_sold,
                3,
            )?,
        )?;
        body.set(
            1,
            13,
            self.beps_real(
                pass,
                self.input_state.gather_electric_produced
                    + (self.input_state.gather_electric_purchased
                        - self.input_state.gather_electric_surplus_sold),
                3,
            )?,
        )?;
        body.set(
            1,
            14,
            self.beps_real(pass, l.collapsed_total.read([1])?, 3)?,
        )?;
        if l.collapsed_total.read([1])? > 0.0 {
            body.set(
                2,
                1,
                self.beps_real(
                    pass,
                    100.0 * self.input_state.gather_power_fuel_fire_gen
                        / l.collapsed_total.read([1])?,
                    2,
                )?,
            )?;
            body.set(
                2,
                2,
                self.beps_real(
                    pass,
                    100.0 * self.input_state.gather_power_ht_geothermal
                        / l.collapsed_total.read([1])?,
                    2,
                )?,
            )?;
            body.set(
                2,
                3,
                self.beps_real(
                    pass,
                    100.0 * self.input_state.gather_power_pv / l.collapsed_total.read([1])?,
                    2,
                )?,
            )?;
            body.set(
                2,
                4,
                self.beps_real(
                    pass,
                    100.0 * self.input_state.gather_power_wind / l.collapsed_total.read([1])?,
                    2,
                )?,
            )?;
            body.set(
                2,
                5,
                self.beps_real(
                    pass,
                    100.0 * self.input_state.gather_power_conversion
                        / l.collapsed_total.read([1])?,
                    2,
                )?,
            )?;
            body.set(
                2,
                6,
                self.beps_real(
                    pass,
                    100.0 * self.input_state.overall_net_energy_from_storage
                        / l.collapsed_total.read([1])?,
                    2,
                )?,
            )?;
            body.set(
                2,
                7,
                self.beps_real(
                    pass,
                    100.0 * self.input_state.gather_electric_produced
                        / l.collapsed_total.read([1])?,
                    2,
                )?,
            )?;
            body.set(
                2,
                9,
                self.beps_real(
                    pass,
                    100.0 * self.input_state.gather_electric_purchased
                        / l.collapsed_total.read([1])?,
                    2,
                )?,
            )?;
            body.set(
                2,
                10,
                self.beps_real(
                    pass,
                    100.0 * self.input_state.gather_electric_surplus_sold
                        / l.collapsed_total.read([1])?,
                    2,
                )?,
            )?;
            body.set(
                2,
                11,
                self.beps_real(
                    pass,
                    100.0
                        * (self.input_state.gather_electric_purchased
                            - self.input_state.gather_electric_surplus_sold)
                        / l.collapsed_total.read([1])?,
                    2,
                )?,
            )?;
            body.set(
                2,
                13,
                self.beps_real(
                    pass,
                    100.0
                        * (self.input_state.gather_electric_produced
                            + (self.input_state.gather_electric_purchased
                                - self.input_state.gather_electric_surplus_sold))
                        / l.collapsed_total.read([1])?,
                    2,
                )?,
            )?;
            body.set(2, 14, self.beps_real(pass, 100.0, 2)?)?;
        }
        self.emit_source_beps_table(
            pass,
            b"Electric Loads Satisfied",
            &body,
            &rows,
            &cols,
            &mut widths,
            b"",
            o,
        )
    }
    pub(super) fn source_beps_thermal_table(
        &mut self,
        pass: usize,
        first: bool,
        p: &SourceBepsPass,
        l: &mut SourceBepsLocals,
        o: &mut SourceBepsReportOwners<'_, '_>,
    ) -> Result<(), Error> {
        let mut rows = vec![Vec::new(); 7];
        let mut cols = vec![Vec::new(); 2];
        let mut widths = vec![14; 2];
        let mut body = SourceTabularBody::allocate(2, 7)?;
        cols[0] = match self.input_state.tabular_report_passes[pass].units_style {
            1 => b"Heat [kWh]".to_vec(),
            4 | 5 => b"Heat [kBtu]".to_vec(),
            _ => b"Heat [GJ]".to_vec(),
        };
        cols[1] = b"Percent Heat [%]".to_vec();
        rows[0] = b"Water-Side Heat Recovery".to_vec();
        rows[1] = b"Air to Air Heat Recovery for Cooling".to_vec();
        rows[2] = b"Air to Air Heat Recovery for Heating".to_vec();
        rows[3] = b"High-Temperature Geothermal*".to_vec();
        rows[4] = b"Solar Water Thermal".to_vec();
        rows[5] = b"Solar Air Thermal".to_vec();
        rows[6] = b"Total On-Site Thermal Sources".to_vec();
        if first {
            l.gt_water_heat_recovery = self.input_state.gather_water_heat_recovery;
            l.gt_air_heat_recovery_cool = self.input_state.gather_air_heat_recovery_cool;
            l.gt_air_heat_recovery_heat = self.input_state.gather_air_heat_recovery_heat;
            l.gt_heat_ht_geothermal = self.input_state.gather_heat_ht_geothermal;
            l.gt_heat_solar_water = self.input_state.gather_heat_solar_water;
            l.gt_heat_solar_air = self.input_state.gather_heat_solar_air;
        } else {
            self.input_state.gather_water_heat_recovery = l.gt_water_heat_recovery;
            self.input_state.gather_air_heat_recovery_cool = l.gt_air_heat_recovery_cool;
            self.input_state.gather_air_heat_recovery_heat = l.gt_air_heat_recovery_heat;
            self.input_state.gather_heat_ht_geothermal = l.gt_heat_ht_geothermal;
            self.input_state.gather_heat_solar_water = l.gt_heat_solar_water;
            self.input_state.gather_heat_solar_air = l.gt_heat_solar_air;
        }
        self.input_state.gather_water_heat_recovery /= p.large;
        self.input_state.gather_air_heat_recovery_cool /= p.large;
        self.input_state.gather_air_heat_recovery_heat /= p.large;
        self.input_state.gather_heat_ht_geothermal /= p.large;
        self.input_state.gather_heat_solar_water /= p.large;
        self.input_state.gather_heat_solar_air /= p.large;
        let totalOnsiteHeat = self.input_state.gather_water_heat_recovery
            + self.input_state.gather_air_heat_recovery_cool
            + self.input_state.gather_air_heat_recovery_heat
            + self.input_state.gather_heat_ht_geothermal
            + self.input_state.gather_heat_solar_water
            + self.input_state.gather_heat_solar_air;
        body.set(
            1,
            1,
            self.beps_real(pass, self.input_state.gather_water_heat_recovery, 2)?,
        )?;
        body.set(
            1,
            2,
            self.beps_real(pass, self.input_state.gather_air_heat_recovery_cool, 2)?,
        )?;
        body.set(
            1,
            3,
            self.beps_real(pass, self.input_state.gather_air_heat_recovery_heat, 2)?,
        )?;
        body.set(
            1,
            4,
            self.beps_real(pass, self.input_state.gather_heat_ht_geothermal, 2)?,
        )?;
        body.set(
            1,
            5,
            self.beps_real(pass, self.input_state.gather_heat_solar_water, 2)?,
        )?;
        body.set(
            1,
            6,
            self.beps_real(pass, self.input_state.gather_heat_solar_air, 2)?,
        )?;
        body.set(1, 7, self.beps_real(pass, totalOnsiteHeat, 2)?)?;
        if totalOnsiteHeat > 0.0 {
            body.set(
                2,
                1,
                self.beps_real(
                    pass,
                    100.0 * self.input_state.gather_water_heat_recovery / totalOnsiteHeat,
                    2,
                )?,
            )?;
            body.set(
                2,
                2,
                self.beps_real(
                    pass,
                    100.0 * self.input_state.gather_air_heat_recovery_cool / totalOnsiteHeat,
                    2,
                )?,
            )?;
            body.set(
                2,
                3,
                self.beps_real(
                    pass,
                    100.0 * self.input_state.gather_air_heat_recovery_heat / totalOnsiteHeat,
                    2,
                )?,
            )?;
            body.set(
                2,
                4,
                self.beps_real(
                    pass,
                    100.0 * self.input_state.gather_heat_ht_geothermal / totalOnsiteHeat,
                    2,
                )?,
            )?;
            body.set(
                2,
                5,
                self.beps_real(
                    pass,
                    100.0 * self.input_state.gather_heat_solar_water / totalOnsiteHeat,
                    2,
                )?,
            )?;
            body.set(
                2,
                6,
                self.beps_real(
                    pass,
                    100.0 * self.input_state.gather_heat_solar_air / totalOnsiteHeat,
                    2,
                )?,
            )?;
            body.set(2, 7, self.beps_real(pass, 100.0, 2)?)?;
        }
        self.emit_source_beps_table(
            pass,
            b"On-Site Thermal Sources",
            &body,
            &rows,
            &cols,
            &mut widths,
            b"",
            o,
        )
    }
    pub(super) fn source_beps_water_table(
        &mut self,
        pass: usize,
        p: &SourceBepsPass,
        o: &mut SourceBepsReportOwners<'_, '_>,
    ) -> Result<(), Error> {
        let mut rows = vec![Vec::new(); 13];
        let mut cols = vec![Vec::new(); 2];
        let mut widths = vec![14; 2];
        let mut body = SourceTabularBody::allocate(2, 13)?;
        cols[0] = match self.input_state.tabular_report_passes[pass].units_style {
            1 => b"Water [m3]".to_vec(),
            4 | 5 => b"Water [gal]".to_vec(),
            _ => b"Water [m3]".to_vec(),
        };
        cols[1] = b"Percent Water [%]".to_vec();
        rows[0] = b"Rainwater Collection".to_vec();
        rows[1] = b"Condensate Collection".to_vec();
        rows[2] = b"Groundwater Well".to_vec();
        rows[3] = b"Total On Site Water Sources".to_vec();
        rows[4] = b"-".to_vec();
        rows[5] = b"Initial Storage".to_vec();
        rows[6] = b"Final Storage".to_vec();
        rows[7] = b"Change in Storage".to_vec();
        rows[8] = b"-".to_vec();
        rows[9] = b"Water Supplied by Utility".to_vec();
        rows[10] = b"-".to_vec();
        rows[11] = b"Total On Site, Change in Storage, and Utility Water Sources".to_vec();
        rows[12] = b"Total Water End Uses".to_vec();
        for col in 1..=2 {
            for row in 1..=13 {
                body.set(col, row, b"-".to_vec())?;
            }
        }
        let totalOnsiteWater = self.input_state.gather_rain_water
            + self.input_state.gather_condensate
            + self.input_state.gather_wellwater;
        body.set(
            1,
            1,
            self.beps_real(pass, self.input_state.gather_rain_water / p.water, 2)?,
        )?;
        body.set(
            1,
            2,
            self.beps_real(pass, self.input_state.gather_condensate / p.water, 2)?,
        )?;
        body.set(
            1,
            3,
            self.beps_real(pass, self.input_state.gather_wellwater / p.water, 2)?,
        )?;
        body.set(1, 4, self.beps_real(pass, totalOnsiteWater / p.water, 2)?)?;
        let initialStorage = 0.0;
        let finalStorage = 0.0;
        let mut StorageChange = 0.0;
        if let Some(rows) = &o.water.storage {
            if !rows.is_empty() {
                return Err(Error::Unavailable(
                    "actual BEPS9486 sum WaterStorage.InitialVolume first row",
                ));
            }
            // Real sum over the actual allocated empty array has the Native additive identity.
            StorageChange = initialStorage - finalStorage;
        }
        body.set(1, 6, self.beps_real(pass, initialStorage / p.water, 2)?)?;
        body.set(1, 7, self.beps_real(pass, finalStorage / p.water, 2)?)?;
        body.set(1, 8, self.beps_real(pass, StorageChange / p.water, 2)?)?;
        let totalWater = totalOnsiteWater + self.input_state.gather_mains + StorageChange;
        body.set(
            1,
            10,
            self.beps_real(pass, self.input_state.gather_mains / p.water, 2)?,
        )?;
        body.set(1, 12, self.beps_real(pass, totalWater / p.water, 2)?)?;
        body.set(
            1,
            13,
            self.beps_real(
                pass,
                self.input_state.gather_water_end_use_total / p.water,
                2,
            )?,
        )?;
        if self.input_state.gather_water_end_use_total > 0.0 {
            body.set(
                2,
                1,
                self.beps_real(
                    pass,
                    100.0 * self.input_state.gather_rain_water
                        / self.input_state.gather_water_end_use_total,
                    2,
                )?,
            )?;
            body.set(
                2,
                2,
                self.beps_real(
                    pass,
                    100.0 * self.input_state.gather_condensate
                        / self.input_state.gather_water_end_use_total,
                    2,
                )?,
            )?;
            body.set(
                2,
                3,
                self.beps_real(
                    pass,
                    100.0 * self.input_state.gather_wellwater
                        / self.input_state.gather_water_end_use_total,
                    2,
                )?,
            )?;
            body.set(
                2,
                4,
                self.beps_real(
                    pass,
                    100.0 * totalOnsiteWater / self.input_state.gather_water_end_use_total,
                    2,
                )?,
            )?;
            body.set(
                2,
                6,
                self.beps_real(
                    pass,
                    100.0 * initialStorage / self.input_state.gather_water_end_use_total,
                    2,
                )?,
            )?;
            body.set(
                2,
                7,
                self.beps_real(
                    pass,
                    100.0 * finalStorage / self.input_state.gather_water_end_use_total,
                    2,
                )?,
            )?;
            body.set(
                2,
                8,
                self.beps_real(
                    pass,
                    100.0 * StorageChange / self.input_state.gather_water_end_use_total,
                    2,
                )?,
            )?;
            body.set(
                2,
                10,
                self.beps_real(
                    pass,
                    100.0 * self.input_state.gather_mains
                        / self.input_state.gather_water_end_use_total,
                    2,
                )?,
            )?;
            body.set(
                2,
                12,
                self.beps_real(
                    pass,
                    100.0 * totalWater / self.input_state.gather_water_end_use_total,
                    2,
                )?,
            )?;
            body.set(2, 13, self.beps_real(pass, 100.0, 2)?)?;
        }
        self.emit_source_beps_table(
            pass,
            b"Water Source Summary",
            &body,
            &rows,
            &cols,
            &mut widths,
            b"",
            o,
        )
    }
    pub(super) fn source_beps_comfort_tables(
        &mut self,
        pass: usize,
        o: &mut SourceBepsReportOwners<'_, '_>,
    ) -> Result<(), Error> {
        if self.display_tabular_beps {
            let rows = vec![
                b"Tolerance for Zone Heating Setpoint Not Met Time".to_vec(),
                b"Tolerance for Zone Cooling Setpoint Not Met Time".to_vec(),
            ];
            let mut cols = vec![Vec::new(); 1];
            let mut widths = vec![14];
            let mut body = SourceTabularBody::allocate(1, 2)?;
            if self.input_state.tabular_report_passes[pass].produce_tabular {
                self.write_source_subtitle(b"Setpoint Not Met Criteria")?;
            }
            let name = b"Degrees [deltaC]";
            if matches!(
                self.input_state.tabular_report_passes[pass].units_style,
                4 | 5
            ) {
                return Err(Error::Unavailable(
                    "actual BEPS9549 LookupSItoIP deltaC operation",
                ));
            } else {
                cols[0] = name.to_vec();
            }
            body.set(
                1,
                1,
                self.beps_real(pass, o.hvac.deviation_from_setpoint_heating.abs(), 2)?,
            )?;
            body.set(
                1,
                2,
                self.beps_real(pass, o.hvac.deviation_from_setpoint_cooling, 2)?,
            )?;
            if self.input_state.tabular_report_passes[pass].produce_tabular {
                self.write_source_table(
                    &body,
                    &rows,
                    &cols,
                    &mut widths,
                    false,
                    b"",
                    &mut o.writer,
                )?;
            }
            if self.input_state.tabular_report_passes[pass].produce_sqlite {
                if o.sqlite.present() {
                    return Err(Error::Unavailable(
                        "actual BEPS9579 SQLite Setpoint Not Met Criteria",
                    ));
                }
            }
            if self.input_state.tabular_report_passes[pass].produce_json {
                if o.results.time_series_and_tabular_enabled {
                    return Err(Error::Unavailable(
                        "actual BEPS9585 JSON Setpoint Not Met Criteria",
                    ));
                }
            }
        }
        let mut rows = vec![Vec::new(); 3];
        let mut cols = vec![Vec::new(); 1];
        let mut widths = vec![14];
        let mut body = SourceTabularBody::allocate(1, 3)?;
        if self.display_tabular_beps {
            if self.input_state.tabular_report_passes[pass].produce_tabular {
                self.write_source_subtitle(b"Comfort and Setpoint Not Met Summary")?;
            }
        }
        cols[0] = b"Facility [Hours]".to_vec();
        rows[0] = b"Time Setpoint Not Met During Occupied Heating".to_vec();
        rows[1] = b"Time Setpoint Not Met During Occupied Cooling".to_vec();
        rows[2] = b"Time Not Comfortable Based on Simple ASHRAE 55-2004".to_vec();
        body.set(
            1,
            1,
            self.beps_real(
                pass,
                o.predefined_summary
                    .total_not_met_heating_occupied_for_abups,
                2,
            )?,
        )?;
        body.set(
            1,
            2,
            self.beps_real(
                pass,
                o.predefined_summary
                    .total_not_met_cooling_occupied_for_abups,
                2,
            )?,
        )?;
        if self.input_state.tabular_report_passes[pass].produce_tabular {
            let text = self.beps_real(
                pass,
                o.predefined_summary
                    .total_not_met_heating_occupied_for_abups,
                2,
            )?;
            predef_bytes(
                o,
                "pdchLeedAmData",
                b"Number of hours heating loads not met",
                &text,
            )?;
            let text = self.beps_real(
                pass,
                o.predefined_summary
                    .total_not_met_cooling_occupied_for_abups,
                2,
            )?;
            predef_bytes(
                o,
                "pdchLeedAmData",
                b"Number of hours cooling loads not met",
                &text,
            )?;
            let text = self.beps_real(
                pass,
                o.predefined_summary.total_not_met_occupied_for_abups,
                2,
            )?;
            predef_bytes(o, "pdchLeedAmData", b"Number of hours not met", &text)?;
        }
        body.set(
            1,
            3,
            self.beps_real(
                pass,
                o.predefined_summary
                    .total_time_not_simple_ash55_either_for_abups,
                2,
            )?,
        )?;
        if self.display_tabular_beps {
            if self.input_state.tabular_report_passes[pass].produce_tabular {
                self.write_source_table(
                    &body,
                    &rows,
                    &cols,
                    &mut widths,
                    false,
                    b"",
                    &mut o.writer,
                )?;
            }
            if self.input_state.tabular_report_passes[pass].produce_sqlite {
                if o.sqlite.present() {
                    return Err(Error::Unavailable(
                        "actual BEPS9633 SQLite Comfort and Setpoint Not Met Summary",
                    ));
                }
            }
            if self.input_state.tabular_report_passes[pass].produce_json {
                if o.results.time_series_and_tabular_enabled {
                    return Err(Error::Unavailable(
                        "actual BEPS9643 JSON Comfort and Setpoint Not Met Summary",
                    ));
                }
            }
        }
        Ok(())
    }
}
