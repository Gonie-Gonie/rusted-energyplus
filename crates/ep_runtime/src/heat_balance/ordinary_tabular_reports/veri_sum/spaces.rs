//! Actual required writeVeriSumSpaceTables11974-12354, per current style.
use super::helpers::{
    Table, count, gain_space, gain_value, index, label, people_label, plus, zone,
};
use super::{Error, SourceSolarArray, SourceTabularReports, SourceVeriSumOwners};
fn allocated(n: i32) -> Result<SourceSolarArray<f64, 1>, Error> {
    let mut a = SourceSolarArray::default();
    a.dimension([count(n)?], None)?;
    Ok(a)
}
impl SourceTabularReports {
    // Keep Original declaration and branch-assignment order at this owner boundary.
    // Keep Original constructor/local initialization and later assignment order, including unconsumed suffix writes.
    #[allow(clippy::needless_late_init, unused_assignments)]
    pub(super) fn veri_spaces(
        &mut self,
        pass: usize,
        o: &mut SourceVeriSumOwners<'_, '_>,
    ) -> Result<(), Error> {
        let mut t = Table::new(11, plus(o.globals.num_spaces, 4)?)?;
        t.cols[0] = label(b"Area ", &self.input_state.m2_unit_name);
        t.cols[1] = b"Conditioned (Y/N)".to_vec();
        t.cols[2] = b"Part of Total Floor Area (Y/N)".to_vec();
        t.cols[3] = b"Multipliers".to_vec();
        t.cols[4] = b"Zone Name".to_vec();
        t.cols[5] = b"Space Type".to_vec();
        t.cols[6] = b"Radiant/Solar Enclosure Name".to_vec();
        t.cols[7] = label(b"Lighting ", &self.input_state.wm2_unit_name);
        t.cols[8] = people_label(&self.input_state.m2_unit_name)?;
        t.cols[9] = label(b"Plug and Process ", &self.input_state.wm2_unit_name);
        t.cols[10] = b"Tags".to_vec();
        for (index, name) in [
            (self.input_state.grand_total, b"Total".as_slice()),
            (self.input_state.cond_total, b"Conditioned Total".as_slice()),
            (
                self.input_state.uncond_total,
                b"Unconditioned Total".as_slice(),
            ),
            (
                self.input_state.notpart_total,
                b"Not Part of Total".as_slice(),
            ),
        ] {
            let at = plus(o.globals.num_spaces, index)?
                .checked_sub(1)
                .ok_or(Error::Unavailable("actual VeriSum space total row"))?;
            *t.rows.get_mut(at).ok_or(Error::Unavailable(
                "actual VeriSum space total row subscript",
            ))? = name.to_vec();
        }
        let mut space_light = allocated(o.globals.num_spaces)?;
        let mut space_people = allocated(o.globals.num_spaces)?;
        let mut space_plug = allocated(o.globals.num_spaces)?;
        for at in 1..=count(o.globals.num_spaces)? {
            space_light.set([at], 0.0)?;
            space_people.set([at], 0.0)?;
            space_plug.set([at], 0.0)?;
        }
        let mut types = Table::new(7, plus(o.globals.num_space_types, 1)?)?;
        types.cols[0] = label(b"Total Area ", &self.input_state.m2_unit_name);
        types.cols[1] = label(b"Conditioned Area ", &self.input_state.m2_unit_name);
        types.cols[2] = label(b"Unconditioned Area ", &self.input_state.m2_unit_name);
        types.cols[3] = label(b"Not Part of Total Area ", &self.input_state.m2_unit_name);
        types.cols[4] = label(b"Lighting ", &self.input_state.wm2_unit_name);
        types.cols[5] = people_label(&self.input_state.m2_unit_name)?;
        types.cols[6] = label(b"Plug and Process ", &self.input_state.wm2_unit_name);
        let at = plus(o.globals.num_space_types, self.input_state.grand_total)?
            .checked_sub(1)
            .ok_or(Error::Unavailable(
                "actual VeriSum space type total name row",
            ))?;
        *types.rows.get_mut(at).ok_or(Error::Unavailable(
            "actual VeriSum space type total name subscript",
        ))? = b"Total".to_vec();
        let mut type_area = allocated(o.globals.num_space_types)?;
        let mut type_cond = allocated(o.globals.num_space_types)?;
        let mut type_uncond = allocated(o.globals.num_space_types)?;
        let mut type_not = allocated(o.globals.num_space_types)?;
        let mut type_light = allocated(o.globals.num_space_types)?;
        let mut type_people = allocated(o.globals.num_space_types)?;
        let mut type_plug = allocated(o.globals.num_space_types)?;
        for at in 1..=count(o.globals.num_space_types)? {
            type_area.set([at], 0.0)?;
            type_cond.set([at], 0.0)?;
            type_uncond.set([at], 0.0)?;
            type_not.set([at], 0.0)?;
            type_light.set([at], 0.0)?;
            type_people.set([at], 0.0)?;
            type_plug.set([at], 0.0)?;
        }
        for family in [0, 1, 2, 3, 6, 4, 5, 7] {
            for number in 1..=o.gains.totals[family] {
                let space = count(gain_space(o, family, number)?)?;
                let value = gain_value(o, family, number)?;
                let (spaces, types) = if family == 0 {
                    (&mut space_people, &mut type_people)
                } else if family == 1 {
                    (&mut space_light, &mut type_light)
                } else {
                    (&mut space_plug, &mut type_plug)
                };
                spaces.set([space], spaces.read([space])? + value)?;
                let row = o
                    .zones
                    .spaces
                    .as_ref()
                    .and_then(|r| r.get(space.wrapping_sub(1)))
                    .ok_or(Error::Unavailable(
                        "actual VeriSum gain space.spaceTypeNum row",
                    ))?;
                let kind = count(row.space_type_num)?;
                types.set([kind], types.read([kind])? + value)?;
            }
        }
        for at in 1..=4 {
            self.input_state.zst_area.set([at], 0.0)?;
            self.input_state.zst_light.set([at], 0.0)?;
            self.input_state.zst_plug.set([at], 0.0)?;
            self.input_state.zst_people.set([at], 0.0)?;
        }
        let mut row_number: i32 = 0;
        for number in 1..=o.globals.num_of_zones {
            let z = zone(o.zones, number)?;
            let m = f64::from(z.multiplier) * f64::from(z.list_multiplier);
            for &space_number in &z.space_indexes {
                let space_at = index(space_number)?;
                let space = o
                    .zones
                    .spaces
                    .as_ref()
                    .and_then(|r| r.get(space_at))
                    .ok_or(Error::Unavailable("actual VeriSum curSpace row12159"))?;
                let mut conditioned = false;
                let mut use_area = false;
                row_number = row_number.checked_add(1).ok_or(Error::Unavailable(
                    "actual VeriSum spaceTableRowNum signed increment",
                ))?;
                let row = count(row_number)?;
                let sp = count(space_number)?;
                *t.rows
                    .get_mut(row.wrapping_sub(1))
                    .ok_or(Error::Unavailable(
                        "actual VeriSum spaceTable row subscript",
                    ))? = space.name.as_bytes().to_vec();
                t.body.set(5, row, z.name.as_bytes().to_vec())?;
                t.body.set(6, row, space.space_type.as_bytes().to_vec())?;
                let enclosure_at = index(space.solar_enclosure_num)?;
                let enclosure = o
                    .radiation
                    .enclosure_storage()
                    .and_then(|r| r.get(enclosure_at))
                    .ok_or(Error::Unavailable("actual VeriSum EnclSolInfo.Name12166"))?;
                t.body.set(7, row, enclosure.name.as_bytes().to_vec())?;
                t.body.set(
                    1,
                    row,
                    self.veri_real(
                        pass,
                        space.floor_area * self.input_state.m2_unit_conv_wvst,
                        2,
                    )?,
                )?;
                if z.system_zone_node_number > 0 {
                    t.body.set(2, row, b"Yes".to_vec())?;
                    conditioned = true;
                } else {
                    t.body.set(2, row, b"No".to_vec())?;
                    conditioned = false;
                }
                if z.is_part_of_total_area {
                    t.body.set(3, row, b"Yes".to_vec())?;
                    use_area = true;
                } else {
                    t.body.set(3, row, b"No".to_vec())?;
                    use_area = false;
                }
                t.body.set(4, row, self.veri_real(pass, m, 2)?)?;
                if space.floor_area > 0.0 {
                    t.body.set(
                        8,
                        row,
                        self.veri_real(
                            pass,
                            self.input_state.wm2_unit_conv * space_light.read([sp])?
                                / space.floor_area,
                            4,
                        )?,
                    )?;
                } else {
                    t.body.set(8, row, self.veri_real(pass, 0.0, 4)?)?;
                }
                if space_people.read([sp])? > 0.0 {
                    t.body.set(
                        9,
                        row,
                        self.veri_real(
                            pass,
                            space.floor_area * self.input_state.m2_unit_conv_wvst
                                / space_people.read([sp])?,
                            2,
                        )?,
                    )?;
                } else {
                    t.body.set(9, row, self.veri_real(pass, 0.0, 2)?)?;
                }
                if space.floor_area > 0.0 {
                    t.body.set(
                        10,
                        row,
                        self.veri_real(
                            pass,
                            space_plug.read([sp])? * self.input_state.wm2_unit_conv
                                / space.floor_area,
                            4,
                        )?,
                    )?;
                } else {
                    t.body.set(10, row, self.veri_real(pass, 0.0, 4)?)?;
                }
                t.body.set(11, row, space.tags.join(", ").into_bytes())?;
                if !use_area {
                    let kind = count(space.space_type_num)?;
                    type_not.set([kind], type_not.read([kind])? + space.floor_area * m)?;
                    let at = count(self.input_state.notpart_total)?;
                    self.input_state.zst_area.set(
                        [at],
                        self.input_state.zst_area.read([at])? + m * space.floor_area,
                    )?;
                    self.input_state.zst_light.set(
                        [at],
                        self.input_state.zst_light.read([at])? + m * space_light.read([sp])?,
                    )?;
                    self.input_state.zst_people.set(
                        [at],
                        self.input_state.zst_people.read([at])? + m * space_people.read([sp])?,
                    )?;
                    self.input_state.zst_plug.set(
                        [at],
                        self.input_state.zst_plug.read([at])? + m * space_plug.read([sp])?,
                    )?;
                } else {
                    let kind = count(space.space_type_num)?;
                    type_area.set([kind], type_area.read([kind])? + space.floor_area * m)?;
                    let at = count(self.input_state.grand_total)?;
                    self.input_state.zst_area.set(
                        [at],
                        self.input_state.zst_area.read([at])? + m * space.floor_area,
                    )?;
                    self.input_state.zst_light.set(
                        [at],
                        self.input_state.zst_light.read([at])? + m * space_light.read([sp])?,
                    )?;
                    self.input_state.zst_people.set(
                        [at],
                        self.input_state.zst_people.read([at])? + m * space_people.read([sp])?,
                    )?;
                    self.input_state.zst_plug.set(
                        [at],
                        self.input_state.zst_plug.read([at])? + m * space_plug.read([sp])?,
                    )?;
                    let at;
                    if conditioned {
                        let kind = count(space.space_type_num)?;
                        type_cond.set([kind], type_cond.read([kind])? + space.floor_area * m)?;
                        at = count(self.input_state.cond_total)?;
                    } else {
                        let kind = count(space.space_type_num)?;
                        type_uncond
                            .set([kind], type_uncond.read([kind])? + space.floor_area * m)?;
                        at = count(self.input_state.uncond_total)?;
                    }
                    self.input_state.zst_area.set(
                        [at],
                        self.input_state.zst_area.read([at])? + m * space.floor_area,
                    )?;
                    self.input_state.zst_light.set(
                        [at],
                        self.input_state.zst_light.read([at])? + m * space_light.read([sp])?,
                    )?;
                    self.input_state.zst_people.set(
                        [at],
                        self.input_state.zst_people.read([at])? + m * space_people.read([sp])?,
                    )?;
                    self.input_state.zst_plug.set(
                        [at],
                        self.input_state.zst_plug.read([at])? + m * space_plug.read([sp])?,
                    )?;
                }
            }
        }
        for total in 1..=4 {
            let row = plus(o.globals.num_spaces, total as i32)?;
            t.body.set(
                1,
                row,
                self.veri_real(
                    pass,
                    self.input_state.zst_area.read([total])? * self.input_state.m2_unit_conv_wvst,
                    2,
                )?,
            )?;
            if self.input_state.zst_area.read([total])? != 0.0 {
                t.body.set(
                    8,
                    row,
                    self.veri_real(
                        pass,
                        self.input_state.zst_light.read([total])? * self.input_state.wm2_unit_conv
                            / self.input_state.zst_area.read([total])?,
                        4,
                    )?,
                )?;
                t.body.set(
                    10,
                    row,
                    self.veri_real(
                        pass,
                        self.input_state.zst_plug.read([total])? * self.input_state.wm2_unit_conv
                            / self.input_state.zst_area.read([total])?,
                        4,
                    )?,
                )?;
            } else {
                t.body.set(8, row, self.veri_real(pass, 0.0, 4)?)?;
                t.body.set(10, row, self.veri_real(pass, 0.0, 4)?)?;
            }
            if self.input_state.zst_people.read([total])? != 0.0 {
                t.body.set(
                    9,
                    row,
                    self.veri_real(
                        pass,
                        self.input_state.zst_area.read([total])?
                            * self.input_state.m2_unit_conv_wvst
                            / self.input_state.zst_people.read([total])?,
                        2,
                    )?,
                )?;
            } else {
                t.body.set(9, row, self.veri_real(pass, 0.0, 2)?)?;
            }
        }
        let mut total_area = 0.0;
        let mut conditioned_area = 0.0;
        let mut unconditioned_area = 0.0;
        let mut not_total_area = 0.0;
        for kind in 1..=count(o.globals.num_space_types)? {
            types.rows[kind - 1] = o
                .zones
                .space_types
                .as_ref()
                .and_then(|r| r.get(kind - 1))
                .ok_or(Error::Unavailable("actual VeriSum spaceTypes row12275"))?
                .as_bytes()
                .to_vec();
            types.body.set(
                1,
                kind,
                self.veri_real(
                    pass,
                    type_area.read([kind])? * self.input_state.m2_unit_conv_wvst,
                    2,
                )?,
            )?;
            types.body.set(
                2,
                kind,
                self.veri_real(
                    pass,
                    type_cond.read([kind])? * self.input_state.m2_unit_conv_wvst,
                    2,
                )?,
            )?;
            types.body.set(
                3,
                kind,
                self.veri_real(
                    pass,
                    type_uncond.read([kind])? * self.input_state.m2_unit_conv_wvst,
                    2,
                )?,
            )?;
            types.body.set(
                4,
                kind,
                self.veri_real(
                    pass,
                    type_not.read([kind])? * self.input_state.m2_unit_conv_wvst,
                    2,
                )?,
            )?;
            total_area += type_area.read([kind])?;
            conditioned_area += type_cond.read([kind])?;
            unconditioned_area += type_uncond.read([kind])?;
            not_total_area += type_not.read([kind])?;
            if type_area.read([kind])? > 0.0 {
                types.body.set(
                    5,
                    kind,
                    self.veri_real(
                        pass,
                        self.input_state.wm2_unit_conv * type_light.read([kind])?
                            / type_area.read([kind])?,
                        4,
                    )?,
                )?;
            } else {
                types.body.set(5, kind, self.veri_real(pass, 0.0, 4)?)?;
            }
            if type_people.read([kind])? > 0.0 {
                types.body.set(
                    6,
                    kind,
                    self.veri_real(
                        pass,
                        type_area.read([kind])? * self.input_state.m2_unit_conv_wvst
                            / type_people.read([kind])?,
                        2,
                    )?,
                )?;
            } else {
                types.body.set(6, kind, self.veri_real(pass, 0.0, 2)?)?;
            }
            if type_area.read([kind])? > 0.0 {
                types.body.set(
                    7,
                    kind,
                    self.veri_real(
                        pass,
                        type_plug.read([kind])? * self.input_state.wm2_unit_conv
                            / type_area.read([kind])?,
                        4,
                    )?,
                )?;
            } else {
                types.body.set(7, kind, self.veri_real(pass, 0.0, 4)?)?;
            }
        }
        let row = plus(o.globals.num_space_types, 1)?;
        types.body.set(
            1,
            row,
            self.veri_real(pass, total_area * self.input_state.m2_unit_conv_wvst, 2)?,
        )?;
        types.body.set(
            2,
            row,
            self.veri_real(
                pass,
                conditioned_area * self.input_state.m2_unit_conv_wvst,
                2,
            )?,
        )?;
        types.body.set(
            3,
            row,
            self.veri_real(
                pass,
                unconditioned_area * self.input_state.m2_unit_conv_wvst,
                2,
            )?,
        )?;
        types.body.set(
            4,
            row,
            self.veri_real(pass, not_total_area * self.input_state.m2_unit_conv_wvst, 2)?,
        )?;
        // Native emits BOTH physical tables, then BOTH JSON calls, then BOTH SQL.
        if self.input_state.tabular_report_passes[pass].produce_tabular {
            self.write_source_subtitle(b"Space Summary")?;
            self.write_source_table(
                &t.body,
                &t.rows,
                &t.cols,
                &mut t.widths,
                false,
                b"",
                &mut o.writer,
            )?;
            self.write_source_subtitle(b"Space Type Summary")?;
            self.write_source_table(
                &types.body,
                &types.rows,
                &types.cols,
                &mut types.widths,
                false,
                b"",
                &mut o.writer,
            )?;
        }
        if self.input_state.tabular_report_passes[pass].produce_json
            && o.results.time_series_and_tabular_enabled
        {
            return Err(Error::Unavailable(
                "actual VeriSum space JSON first addReportTable12329",
            ));
        }
        if self.input_state.tabular_report_passes[pass].produce_sqlite && o.sqlite.present() {
            return Err(Error::Unavailable(
                "actual VeriSum space SQLite first createRecords12343",
            ));
        }
        Ok(())
    }
}
