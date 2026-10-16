//! Actual BEPS8566-9040 end uses, all LEED terms and two genuine subtable calls.
use super::helpers::{category, column, predef_bytes, predef_real_id, subcategory};
use super::prepare::{SourceBepsLocals, SourceBepsPass};
use super::{Error, SourceBepsReportOwners, SourceTabularBody, SourceTabularReports};
impl SourceTabularReports {
    pub(super) fn source_beps_end_use_tables(
        &mut self,
        pass: usize,
        p: &SourceBepsPass,
        l: &mut SourceBepsLocals,
        o: &mut SourceBepsReportOwners<'_, '_>,
    ) -> Result<(f64, f64), Error> {
        let mut rows = vec![Vec::new(); 16];
        let mut cols = vec![Vec::new(); 14];
        let mut widths = vec![10; 14];
        let mut body = SourceTabularBody::allocate(14, 16)?;
        for resource in 1..=14 {
            for end_use in 0..14 {
                l.use_val.set(
                    [resource, end_use + 1],
                    l.collapsed_end_use.read([resource, end_use + 1])?,
                )?;
            }
            l.use_val
                .set([resource, 15], l.collapsed_total.read([resource])?)?;
        }
        let end_uses: [&[u8]; 14] = [
            b"Heating",
            b"Cooling",
            b"Interior Lighting",
            b"Exterior Lighting",
            b"Interior Equipment",
            b"Exterior Equipment",
            b"Fans",
            b"Pumps",
            b"Heat Rejection",
            b"Humidification",
            b"Heat Recovery",
            b"Water Systems",
            b"Refrigeration",
            b"Generators",
        ];
        for i in 0..14 {
            rows[i] = end_uses[i].to_vec();
        }
        rows[14] = Vec::new();
        rows[15] = b"Total End Uses".to_vec();
        let names = [
            "Electricity",
            "Natural Gas",
            "Gasoline",
            "Diesel",
            "Coal",
            "Fuel Oil No 1",
            "Fuel Oil No 2",
            "Propane",
            "Other Fuel 1",
            "Other Fuel 2",
            "District Cooling",
            "District Heating Water",
            "District Heating Steam",
        ];
        match self.input_state.tabular_report_passes[pass].units_style {
            1 => {
                for i in 0..13 {
                    cols[i] = format!("{} [kWh]", names[i]).into_bytes();
                }
                cols[13] = b"Water [m3]".to_vec();
            }
            4 => {
                for i in 0..13 {
                    cols[i] = format!("{} [kBtu]", names[i]).into_bytes();
                }
                cols[13] = b"Water [gal]".to_vec();
            }
            5 => {
                cols[0] = b"Electricity [kWh]".to_vec();
                for i in 1..13 {
                    cols[i] = format!("{} [kBtu]", names[i]).into_bytes();
                }
                cols[13] = b"Water [gal]".to_vec();
            }
            _ => {
                for i in 0..13 {
                    cols[i] = format!("{} [GJ]", names[i]).into_bytes();
                }
                cols[13] = b"Water [m3]".to_vec();
            }
        }
        body.fill(b"");
        for resource in 1..=14 {
            for end_use in 1..=14 {
                body.set(
                    resource,
                    end_use,
                    self.beps_real(pass, l.use_val.read([resource, end_use])?, 2)?,
                )?;
            }
            body.set(
                resource,
                16,
                self.beps_real(pass, l.use_val.read([resource, 15])?, 2)?,
            )?;
        }
        for resource in 1..=14 {
            let mut total = 0.0;
            for end_use in 1..=14 {
                total += l.use_val.read([resource, end_use])?;
            }
            if (total - l.collapsed_total.read([resource])?).abs()
                > l.collapsed_total.read([resource])? * 0.001
            {
                let mut message=b"In the Annual Building Utility Performance Summary Report the total row does not match the sum of the column for: ".to_vec();
                message.extend_from_slice(&cols[resource - 1]);
                let message = std::str::from_utf8(&message).map_err(|_| {
                    Error::Unavailable("actual BEPS constant resource diagnostic text domain")
                })?;
                o.writer
                    .backend
                    .show_source_warning_error(message, &o.writer.flags)?;
            }
        }
        let unconvert = p.large / 1_000_000_000.0;
        let unconvert_ip = (p.large / p.ip_electricity) / 1_000_000_000.0;
        self.source_beps_leed_energy(pass, l, unconvert, unconvert_ip, o)?;
        let mut footnote = Vec::new();
        match p.primary_heating {
            1 => {
                if self.input_state.tabular_report_passes[pass].produce_tabular {
                    footnote=b"Note: Electricity appears to be the principal heating source based on energy usage.".to_vec();
                    predef_bytes(
                        o,
                        "pdchLeedGenData",
                        b"Principal Heating Source",
                        b"Electricity",
                    )?;
                }
            }
            2 => {
                if self.input_state.tabular_report_passes[pass].produce_tabular {
                    footnote=b"Note: Natural gas appears to be the principal heating source based on energy usage.".to_vec();
                    predef_bytes(
                        o,
                        "pdchLeedGenData",
                        b"Principal Heating Source",
                        b"Natural Gas",
                    )?;
                }
            }
            3..=10 => {
                if self.input_state.tabular_report_passes[pass].produce_tabular {
                    footnote=b"Note: Additional fuel appears to be the principal heating source based on energy usage.".to_vec();
                    predef_bytes(
                        o,
                        "pdchLeedGenData",
                        b"Principal Heating Source",
                        b"Additional Fuel",
                    )?;
                }
            }
            12 => {
                if self.input_state.tabular_report_passes[pass].produce_tabular {
                    footnote=b"Note: District heating water appears to be the principal heating source based on energy usage.".to_vec();
                    predef_bytes(
                        o,
                        "pdchLeedGenData",
                        b"Principal Heating Source",
                        b"District Heating Water",
                    )?;
                }
            }
            13 => {
                if self.input_state.tabular_report_passes[pass].produce_tabular {
                    footnote=b"Note: District heating steam appears to be the principal heating source based on energy usage.".to_vec();
                    predef_bytes(
                        o,
                        "pdchLeedGenData",
                        b"Principal Heating Source",
                        b"District Heating steam",
                    )?;
                }
            }
            _ => {}
        }
        self.emit_source_beps_table(
            pass,
            b"End Uses",
            &body,
            &rows,
            &cols,
            &mut widths,
            &footnote,
            o,
        )?;
        self.source_beps_subcategory_table(pass, false, l, o)?;
        self.source_beps_subcategory_table(pass, true, l, o)?;
        let map_names = [
            "pdchLeedPerfElEneUse",
            "pdchLeedPerfGasEneUse",
            "pdchLeedPerfGasolineEneUse",
            "pdchLeedPerfDieselEneUse",
            "pdchLeedPerfCoalEneUse",
            "pdchLeedPerfFuelOil1EneUse",
            "pdchLeedPerfFuelOil2EneUse",
            "pdchLeedPerfPropaneEneUse",
            "pdchLeedPerfOtherFuel1EneUse",
            "pdchLeedPerfOtherFuel2EneUse",
            "pdchLeedPerfDisClEneUse",
            "pdchLeedPerfDisHtWtrEneUse",
            "pdchLeedPerfDisHtStEneUse",
        ];
        let mut map = Vec::with_capacity(13);
        for name in map_names {
            map.push(column(o, name)?);
        }
        for resource in 1..=13 {
            let mut row: i32 = 1;
            let local_unconvert = if resource == 1 {
                unconvert_ip
            } else {
                unconvert
            };
            for end_use in 1..=14 {
                if category(o.registration, end_use)?.num_subcategories > 0 {
                    for sub in 1..=category(o.registration, end_use)?.num_subcategories {
                        if self.input_state.tabular_report_passes[pass].produce_tabular {
                            let mut name = category(o.registration, end_use)?
                                .display_name
                                .as_bytes()
                                .to_vec();
                            name.extend_from_slice(b" -- ");
                            name.extend_from_slice(&subcategory(
                                o.registration,
                                end_use,
                                sub as usize,
                                false,
                            )?);
                            predef_real_id(
                                o,
                                map[resource - 1],
                                &name,
                                local_unconvert
                                    * l.collapsed_sub.read([sub as usize, end_use, resource])?,
                                2,
                            )?;
                        }
                        row = row
                            .checked_add(1)
                            .ok_or(Error::Unavailable("actual BEPS9018 local row increment"))?;
                    }
                    if self.input_state.need_other_row_leed45.read([end_use])? {
                        if self.input_state.tabular_report_passes[pass].produce_tabular {
                            let mut name = category(o.registration, end_use)?
                                .display_name
                                .as_bytes()
                                .to_vec();
                            name.extend_from_slice(b" -- Other");
                            predef_real_id(
                                o,
                                map[resource - 1],
                                &name,
                                local_unconvert * l.sub_other.read([resource, end_use])?,
                                2,
                            )?;
                        }
                        row = row
                            .checked_add(1)
                            .ok_or(Error::Unavailable("actual BEPS9028 local row increment"))?;
                    }
                } else {
                    if self.input_state.tabular_report_passes[pass].produce_tabular {
                        let mut name = category(o.registration, end_use)?
                            .display_name
                            .as_bytes()
                            .to_vec();
                        name.extend_from_slice(b" -- Not Subdivided");
                        predef_real_id(
                            o,
                            map[resource - 1],
                            &name,
                            local_unconvert * l.collapsed_end_use.read([resource, end_use])?,
                            2,
                        )?;
                    }
                    row = row
                        .checked_add(1)
                        .ok_or(Error::Unavailable("actual BEPS9037 local row increment"))?;
                }
            }
        }
        Ok((unconvert, unconvert_ip))
    }
}
