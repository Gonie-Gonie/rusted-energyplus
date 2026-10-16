//! Original BEPS8071-8564 three complete site/source/factor/area table operations.
use super::helpers::predef_bytes;
use super::prepare::SourceBepsPass;
use super::{Error, SourceBepsReportOwners, SourceTabularBody, SourceTabularReports};

impl SourceTabularReports {
    pub(super) fn source_beps_site_tables(
        &mut self,
        pass: usize,
        p: &SourceBepsPass,
        o: &mut SourceBepsReportOwners<'_, '_>,
    ) -> Result<(f64, f64), Error> {
        let mut rows = vec![Vec::new(); 4];
        let mut cols = vec![Vec::new(); 3];
        let mut widths = vec![14; 3];
        let mut body = SourceTabularBody::allocate(3, 4)?;
        let labels: [&[u8]; 3] = match self.input_state.tabular_report_passes[pass].units_style {
            1 => [
                b"Total Energy [kWh]",
                b"Energy Per Total Building Area [kWh/m2]",
                b"Energy Per Conditioned Building Area [kWh/m2]",
            ],
            4 | 5 => [
                b"Total Energy [kBtu]",
                b"Energy Per Total Building Area [kBtu/ft2]",
                b"Energy Per Conditioned Building Area [kBtu/ft2]",
            ],
            _ => [
                b"Total Energy [GJ]",
                b"Energy Per Total Building Area [MJ/m2]",
                b"Energy Per Conditioned Building Area [MJ/m2]",
            ],
        };
        for i in 0..3 {
            cols[i] = labels[i].to_vec();
        }
        rows[0] = b"Total Site Energy".to_vec();
        rows[1] = b"Net Site Energy".to_vec();
        rows[2] = b"Total Source Energy".to_vec();
        rows[3] = b"Net Source Energy".to_vec();
        let s = &self.input_state;
        let total_site = (s.gather_totals_beps[0]
            + s.gather_totals_beps[1]
            + s.gather_totals_beps[2]
            + s.gather_totals_beps[3]
            + s.gather_totals_beps[4]
            + s.gather_totals_beps[5]
            + s.gather_totals_beps[7]
            + s.gather_totals_beps[8]
            + s.gather_totals_beps[9]
            + s.gather_totals_beps[10]
            + s.gather_totals_beps[11]
            + s.gather_totals_beps[12]
            + s.gather_totals_beps[13])
            / p.large;
        let net_electric = s.gather_electric_purchased - s.gather_electric_surplus_sold;
        let net_site = net_electric
            + (s.gather_totals_beps[1]
                + s.gather_totals_beps[2]
                + s.gather_totals_beps[3]
                + s.gather_totals_beps[4]
                + s.gather_totals_beps[5]
                + s.gather_totals_beps[7]
                + s.gather_totals_beps[8]
                + s.gather_totals_beps[9]
                + s.gather_totals_beps[10]
                + s.gather_totals_beps[11]
                + s.gather_totals_beps[12]
                + s.gather_totals_beps[13])
                / p.large;
        if self.input_state.efficiency_district_cooling == 0.0 {
            self.input_state.efficiency_district_cooling = 1.0;
        }
        if self.input_state.efficiency_district_heating_water == 0.0 {
            self.input_state.efficiency_district_heating_water = 1.0;
        }
        let s = &self.input_state;
        let mut total_source = 0.0;
        // Ten actual fuel-factor branches retain their exact selected-resource order.
        if s.fuel_factors_used[0] {
            total_source += s.gather_totals_source[0];
        } else {
            total_source += s.gather_totals_beps[0] * s.source_factor_electric;
        }
        if s.fuel_factors_used[1] {
            total_source += s.gather_totals_source[1];
        } else {
            total_source += s.gather_totals_beps[1] * s.source_factor_natural_gas;
        }
        if s.fuel_factors_used[2] {
            total_source += s.gather_totals_source[2];
        } else {
            total_source += s.gather_totals_beps[5] * s.source_factor_gasoline;
        }
        if s.fuel_factors_used[3] {
            total_source += s.gather_totals_source[3];
        } else {
            total_source += s.gather_totals_beps[7] * s.source_factor_diesel;
        }
        if s.fuel_factors_used[4] {
            total_source += s.gather_totals_source[4];
        } else {
            total_source += s.gather_totals_beps[8] * s.source_factor_coal;
        }
        if s.fuel_factors_used[5] {
            total_source += s.gather_totals_source[5];
        } else {
            total_source += s.gather_totals_beps[9] * s.source_factor_fuel_oil1;
        }
        if s.fuel_factors_used[6] {
            total_source += s.gather_totals_source[6];
        } else {
            total_source += s.gather_totals_beps[10] * s.source_factor_fuel_oil2;
        }
        if s.fuel_factors_used[7] {
            total_source += s.gather_totals_source[7];
        } else {
            total_source += s.gather_totals_beps[11] * s.source_factor_propane;
        }
        if s.fuel_factors_used[10] {
            total_source += s.gather_totals_source[10];
        } else {
            total_source += s.gather_totals_beps[12] * s.source_factor_other_fuel1;
        }
        if s.fuel_factors_used[11] {
            total_source += s.gather_totals_source[11];
        } else {
            total_source += s.gather_totals_beps[13] * s.source_factor_other_fuel2;
        }
        total_source = (total_source
            + s.gather_totals_beps[2] * s.source_factor_electric / s.efficiency_district_cooling
            + s.gather_totals_beps[3] * s.source_factor_natural_gas
                / s.efficiency_district_heating_water
            + s.gather_totals_beps[4] * s.source_factor_district_heating_steam)
            / p.large;
        let net_source_electric = if s.fuel_factors_used[0] {
            s.gather_totals_source[8] - s.gather_totals_source[9]
        } else {
            net_electric * s.source_factor_electric * p.large
        };
        let mut net_source = 0.0;
        if s.fuel_factors_used[1] {
            net_source += s.gather_totals_source[1];
        } else {
            net_source += s.gather_totals_beps[1] * s.source_factor_natural_gas;
        }
        if s.fuel_factors_used[2] {
            net_source += s.gather_totals_source[2];
        } else {
            net_source += s.gather_totals_beps[5] * s.source_factor_gasoline;
        }
        if s.fuel_factors_used[3] {
            net_source += s.gather_totals_source[3];
        } else {
            net_source += s.gather_totals_beps[7] * s.source_factor_diesel;
        }
        if s.fuel_factors_used[4] {
            net_source += s.gather_totals_source[4];
        } else {
            net_source += s.gather_totals_beps[8] * s.source_factor_coal;
        }
        if s.fuel_factors_used[5] {
            net_source += s.gather_totals_source[5];
        } else {
            net_source += s.gather_totals_beps[9] * s.source_factor_fuel_oil1;
        }
        if s.fuel_factors_used[6] {
            net_source += s.gather_totals_source[6];
        } else {
            net_source += s.gather_totals_beps[10] * s.source_factor_fuel_oil2;
        }
        if s.fuel_factors_used[7] {
            net_source += s.gather_totals_source[7];
        } else {
            net_source += s.gather_totals_beps[11] * s.source_factor_propane;
        }
        if s.fuel_factors_used[10] {
            net_source += s.gather_totals_source[10];
        } else {
            net_source += s.gather_totals_beps[12] * s.source_factor_other_fuel1;
        }
        if s.fuel_factors_used[11] {
            net_source += s.gather_totals_source[11];
        } else {
            net_source += s.gather_totals_beps[13] * s.source_factor_other_fuel2;
        }
        net_source = (net_source
            + net_source_electric
            + s.gather_totals_beps[2] * s.source_factor_electric / s.efficiency_district_cooling
            + s.gather_totals_beps[3] * s.source_factor_natural_gas
                / s.efficiency_district_heating_water
            + s.gather_totals_beps[4] * s.source_factor_district_heating_steam)
            / p.large;
        body.set(1, 1, self.beps_real(pass, total_site, 2)?)?;
        body.set(1, 2, self.beps_real(pass, net_site, 2)?)?;
        body.set(1, 3, self.beps_real(pass, total_source, 2)?)?;
        body.set(1, 4, self.beps_real(pass, net_source, 2)?)?;
        let gross = self.input_state.building_gross_floor_area / p.area;
        let conditioned = self.input_state.building_conditioned_floor_area / p.area;
        if gross > 0.0 {
            body.set(2, 1, self.beps_real(pass, total_site * p.k / gross, 2)?)?;
            body.set(2, 2, self.beps_real(pass, net_site * p.k / gross, 2)?)?;
            body.set(2, 3, self.beps_real(pass, total_source * p.k / gross, 2)?)?;
            body.set(2, 4, self.beps_real(pass, net_source * p.k / gross, 2)?)?;
        }
        if conditioned > 0.0 {
            body.set(
                3,
                1,
                self.beps_real(pass, total_site * p.k / conditioned, 2)?,
            )?;
            body.set(3, 2, self.beps_real(pass, net_site * p.k / conditioned, 2)?)?;
            body.set(
                3,
                3,
                self.beps_real(pass, total_source * p.k / conditioned, 2)?,
            )?;
            body.set(
                3,
                4,
                self.beps_real(pass, net_source * p.k / conditioned, 2)?,
            )?;
        }
        self.emit_source_beps_table(
            pass,
            b"Site and Source Energy",
            &body,
            &rows,
            &cols,
            &mut widths,
            b"",
            o,
        )?;
        self.source_beps_factor_table(pass, o)?;
        self.source_beps_area_table(pass, gross, conditioned, o)?;
        Ok((gross, conditioned))
    }
    fn source_beps_factor_value(
        &self,
        pass: usize,
        resource: usize,
        o: &SourceBepsReportOwners<'_, '_>,
    ) -> Result<Vec<u8>, Error> {
        let s = &self.input_state;
        if !s.ff_schedule_used[resource - 1] {
            let factor = match resource {
                1 => s.source_factor_electric,
                2 => s.source_factor_natural_gas,
                6 => s.source_factor_gasoline,
                8 => s.source_factor_diesel,
                9 => s.source_factor_coal,
                10 => s.source_factor_fuel_oil1,
                11 => s.source_factor_fuel_oil2,
                12 => s.source_factor_propane,
                13 => s.source_factor_other_fuel1,
                14 => s.source_factor_other_fuel2,
                _ => return Err(Error::Unavailable("actual BEPS selected factor resource")),
            };
            self.beps_real(pass, factor, 3)
        } else if s.gather_totals_beps[resource - 1] > 1.0e-14 {
            let mut text = b"Effective Factor = ".to_vec();
            text.extend_from_slice(&self.beps_real(
                pass,
                s.gather_totals_by_source_beps[resource - 1] / s.gather_totals_beps[resource - 1],
                3,
            )?);
            text.extend_from_slice(b" (calculated using schedule \"");
            let schedule = s.ff_schedules[resource - 1].ok_or(Error::Unavailable(
                "actual BEPS ffScheds null Name dereference",
            ))?;
            text.extend_from_slice(
                o.schedules
                    .schedules
                    .get(schedule.0)
                    .ok_or(Error::Unavailable("same actual BEPS ffScheds Name row"))?
                    .name
                    .as_bytes(),
            );
            text.extend_from_slice(b"\")");
            Ok(text)
        } else {
            Ok(b"N/A".to_vec())
        }
    }
    fn source_beps_factor_table(
        &mut self,
        pass: usize,
        o: &mut SourceBepsReportOwners<'_, '_>,
    ) -> Result<(), Error> {
        let rows: [&[u8]; 13] = [
            b"Electricity",
            b"Natural Gas",
            b"District Cooling",
            b"District Heating Water",
            b"District Heating Steam",
            b"Gasoline",
            b"Diesel",
            b"Coal",
            b"Fuel Oil No 1",
            b"Fuel Oil No 2",
            b"Propane",
            b"Other Fuel 1",
            b"Other Fuel 2",
        ];
        let rows: Vec<_> = rows.into_iter().map(<[u8]>::to_vec).collect();
        let cols = vec![b"Site=>Source Conversion Factor".to_vec()];
        let mut widths = vec![50];
        let mut body = SourceTabularBody::allocate(1, 13)?;
        body.set(1, 1, self.source_beps_factor_value(pass, 1, o)?)?;
        body.set(1, 2, self.source_beps_factor_value(pass, 2, o)?)?;
        body.set(
            1,
            3,
            self.beps_real(
                pass,
                self.input_state.source_factor_electric
                    / self.input_state.efficiency_district_cooling,
                3,
            )?,
        )?;
        body.set(
            1,
            4,
            self.beps_real(
                pass,
                self.input_state.source_factor_natural_gas
                    / self.input_state.efficiency_district_heating_water,
                3,
            )?,
        )?;
        body.set(
            1,
            5,
            self.beps_real(
                pass,
                self.input_state.source_factor_district_heating_steam,
                3,
            )?,
        )?;
        body.set(1, 6, self.source_beps_factor_value(pass, 6, o)?)?;
        body.set(1, 7, self.source_beps_factor_value(pass, 8, o)?)?;
        body.set(1, 8, self.source_beps_factor_value(pass, 9, o)?)?;
        body.set(1, 9, self.source_beps_factor_value(pass, 10, o)?)?;
        body.set(1, 10, self.source_beps_factor_value(pass, 11, o)?)?;
        body.set(1, 11, self.source_beps_factor_value(pass, 12, o)?)?;
        body.set(1, 12, self.source_beps_factor_value(pass, 13, o)?)?;
        body.set(1, 13, self.source_beps_factor_value(pass, 14, o)?)?;
        self.emit_source_beps_table(
            pass,
            b"Site to Source Energy Conversion Factors",
            &body,
            &rows,
            &cols,
            &mut widths,
            b"",
            o,
        )
    }
    fn source_beps_area_table(
        &mut self,
        pass: usize,
        gross: f64,
        conditioned: f64,
        o: &mut SourceBepsReportOwners<'_, '_>,
    ) -> Result<(), Error> {
        let mut rows = vec![Vec::new(); 3];
        let mut cols = vec![Vec::new(); 1];
        let mut widths = vec![14];
        let mut body = SourceTabularBody::allocate(1, 3)?;
        let ip = matches!(
            self.input_state.tabular_report_passes[pass].units_style,
            4 | 5
        );
        cols[0] = if ip {
            b"Area [ft2]".to_vec()
        } else {
            b"Area [m2]".to_vec()
        };
        rows[0] = b"Total Building Area".to_vec();
        rows[1] = b"Net Conditioned Building Area".to_vec();
        rows[2] = b"Unconditioned Building Area".to_vec();
        body.set(1, 1, self.beps_real(pass, gross, 2)?)?;
        if ip {
            if self.input_state.tabular_report_passes[pass].produce_tabular {
                predef_bytes(
                    o,
                    "pdchLeedGenData",
                    b"Total gross floor area [ft2]",
                    &self.beps_real(pass, gross, 2)?,
                )?;
            }
        } else if self.input_state.tabular_report_passes[pass].produce_tabular {
            predef_bytes(
                o,
                "pdchLeedGenData",
                b"Total gross floor area [m2]",
                &self.beps_real(pass, gross, 2)?,
            )?;
        }
        body.set(1, 2, self.beps_real(pass, conditioned, 2)?)?;
        body.set(1, 3, self.beps_real(pass, gross - conditioned, 2)?)?;
        self.emit_source_beps_table(
            pass,
            b"Building Area",
            &body,
            &rows,
            &cols,
            &mut widths,
            b"",
            o,
        )
    }
}
