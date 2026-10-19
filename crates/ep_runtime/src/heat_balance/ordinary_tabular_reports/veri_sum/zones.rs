//! Native11744-11970 complete selected Zone Summary and retained totals.
use super::helpers::{
    Table, count, gain_value, gain_zone, label, mult, people_label, plus, predef_real, predef_zero,
    zone,
};
use super::{Error, Locals, SourceTabularReports, SourceVeriSumOwners};
impl SourceTabularReports {
    // Keep Original declaration and branch-assignment order at this owner boundary.
    #[allow(clippy::needless_late_init)]
    pub(super) fn veri_zones(
        &mut self,
        pass: usize,
        l: &Locals,
        o: &mut SourceVeriSumOwners<'_, '_>,
    ) -> Result<(), Error> {
        let mut t = Table::new(12, plus(o.globals.num_of_zones, 4)?)?;
        t.cols[0] = label(b"Area ", &self.input_state.m2_unit_name);
        t.cols[1] = b"Conditioned (Y/N)".to_vec();
        t.cols[2] = b"Part of Total Floor Area (Y/N)".to_vec();
        t.cols[3] = label(b"Volume ", &self.input_state.m3_unit_name);
        t.cols[4] = b"Multipliers".to_vec();
        t.cols[5] = label(
            b"Above Ground Gross Wall Area ",
            &self.input_state.m2_unit_name,
        );
        t.cols[6] = label(
            b"Underground Gross Wall Area ",
            &self.input_state.m2_unit_name,
        );
        t.cols[7] = label(b"Window Glass Area ", &self.input_state.m2_unit_name);
        t.cols[8] = label(b"Opening Area ", &self.input_state.m2_unit_name);
        t.cols[9] = label(b"Lighting ", &self.input_state.wm2_unit_name);
        t.cols[10] = people_label(&self.input_state.m2_unit_name)?;
        t.cols[11] = label(b"Plug and Process ", &self.input_state.wm2_unit_name);
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
            let at = plus(o.globals.num_of_zones, index)?
                .checked_sub(1)
                .ok_or(Error::Unavailable("actual VeriSum total row"))?;
            *t.rows
                .get_mut(at)
                .ok_or(Error::Unavailable("actual VeriSum total row subscript"))? = name.to_vec();
        }
        t.body.fill(b"");
        for number in 1..=o.globals.num_of_zones {
            let z = zone(o.zones, number)?;
            let m = mult(z)?;
            t.rows[(number - 1) as usize] = z.name.as_bytes().to_vec();
            let conditioned;
            if z.system_zone_node_number > 0 {
                t.body.set(2, number as usize, b"Yes".to_vec())?;
                conditioned = true;
            } else {
                t.body.set(2, number as usize, b"No".to_vec())?;
                conditioned = false;
            }
            let use_area;
            if z.is_part_of_total_area {
                t.body.set(3, number as usize, b"Yes".to_vec())?;
                use_area = true;
            } else {
                t.body.set(3, number as usize, b"No".to_vec())?;
                use_area = false;
            }
            t.body.set(
                1,
                number as usize,
                self.veri_real(pass, z.floor_area * self.input_state.m2_unit_conv_wvst, 2)?,
            )?;
            t.body.set(
                4,
                number as usize,
                self.veri_real(pass, z.volume * self.input_state.m3_unit_conv, 2)?,
            )?;
            if self.input_state.tabular_report_passes[pass].produce_tabular {
                predef_real(
                    o.registry,
                    o.table,
                    "pdchLeedSutSpArea",
                    &z.name,
                    z.floor_area,
                )?;
            }
            if conditioned {
                if self.input_state.tabular_report_passes[pass].produce_tabular {
                    predef_real(
                        o.registry,
                        o.table,
                        "pdchLeedSutOcArea",
                        &z.name,
                        z.floor_area,
                    )?;
                    predef_zero(o.registry, o.table, "pdchLeedSutUnArea", &z.name)?;
                }
            } else {
                if self.input_state.tabular_report_passes[pass].produce_tabular {
                    predef_zero(o.registry, o.table, "pdchLeedSutOcArea", &z.name)?;
                    predef_real(
                        o.registry,
                        o.table,
                        "pdchLeedSutUnArea",
                        &z.name,
                        z.floor_area,
                    )?;
                }
            }
            t.body
                .set(5, number as usize, self.veri_real(pass, m, 2)?)?;
            t.body.set(
                6,
                number as usize,
                self.veri_real(
                    pass,
                    z.exterior_gross_wall_area * self.input_state.m2_unit_conv_wvst,
                    2,
                )?,
            )?;
            t.body.set(
                7,
                number as usize,
                self.veri_real(
                    pass,
                    z.exterior_gross_ground_wall_area * self.input_state.m2_unit_conv_wvst,
                    2,
                )?,
            )?;
            t.body.set(
                8,
                number as usize,
                self.veri_real(
                    pass,
                    l.glass.read([number as usize])? * self.input_state.m2_unit_conv_wvst,
                    2,
                )?,
            )?;
            t.body.set(
                9,
                number as usize,
                self.veri_real(
                    pass,
                    l.opening.read([number as usize])? * self.input_state.m2_unit_conv_wvst,
                    2,
                )?,
            )?;
            let mut light = 0.0;
            for gain in 1..=o.gains.totals[1] {
                if number == gain_zone(o, 1, gain)? {
                    light += gain_value(o, 1, gain)?;
                }
            }
            if z.floor_area > 0.0 {
                t.body.set(
                    10,
                    number as usize,
                    self.veri_real(
                        pass,
                        self.input_state.wm2_unit_conv * light / z.floor_area,
                        4,
                    )?,
                )?;
            }
            let mut people = 0.0;
            for gain in 1..=o.gains.totals[0] {
                if number == gain_zone(o, 0, gain)? {
                    people += gain_value(o, 0, gain)?;
                }
            }
            if people > 0.0 {
                t.body.set(
                    11,
                    number as usize,
                    self.veri_real(
                        pass,
                        z.floor_area * self.input_state.m2_unit_conv_wvst / people,
                        2,
                    )?,
                )?;
            }
            let mut plug = 0.0;
            // Native Zone summary deliberately omits SteamEquipment here.
            for family in [2, 3, 6, 4, 7] {
                for gain in 1..=o.gains.totals[family] {
                    if number == gain_zone(o, family, gain)? {
                        plug += gain_value(o, family, gain)?;
                    }
                }
            }
            if z.floor_area > 0.0 {
                t.body.set(
                    12,
                    number as usize,
                    self.veri_real(
                        pass,
                        plug * self.input_state.wm2_unit_conv / z.floor_area,
                        4,
                    )?,
                )?;
            }
            if !use_area {
                let at = count(self.input_state.notpart_total)?;
                self.input_state.zst_area.set(
                    [at],
                    self.input_state.zst_area.read([at])? + m * z.floor_area,
                )?;
                self.input_state
                    .zst_volume
                    .set([at], self.input_state.zst_volume.read([at])? + m * z.volume)?;
                self.input_state.zst_wall_area.set(
                    [at],
                    self.input_state.zst_wall_area.read([at])? + m * z.exterior_gross_wall_area,
                )?;
                self.input_state.zst_und_wall_area.set(
                    [at],
                    self.input_state.zst_und_wall_area.read([at])?
                        + m * z.exterior_gross_ground_wall_area,
                )?;
                self.input_state.zst_window_area.set(
                    [at],
                    self.input_state.zst_window_area.read([at])?
                        + m * l.glass.read([number as usize])?,
                )?;
                self.input_state.zst_opening_area.set(
                    [at],
                    self.input_state.zst_opening_area.read([at])?
                        + m * l.opening.read([number as usize])?,
                )?;
                self.input_state
                    .zst_light
                    .set([at], self.input_state.zst_light.read([at])? + m * light)?;
                self.input_state
                    .zst_people
                    .set([at], self.input_state.zst_people.read([at])? + m * people)?;
                self.input_state
                    .zst_plug
                    .set([at], self.input_state.zst_plug.read([at])? + m * plug)?;
            } else {
                let at = count(self.input_state.grand_total)?;
                self.input_state.zst_area.set(
                    [at],
                    self.input_state.zst_area.read([at])? + m * z.floor_area,
                )?;
                self.input_state
                    .zst_volume
                    .set([at], self.input_state.zst_volume.read([at])? + m * z.volume)?;
                self.input_state.zst_wall_area.set(
                    [at],
                    self.input_state.zst_wall_area.read([at])? + m * z.exterior_gross_wall_area,
                )?;
                self.input_state.zst_und_wall_area.set(
                    [at],
                    self.input_state.zst_und_wall_area.read([at])?
                        + m * z.exterior_gross_ground_wall_area,
                )?;
                self.input_state.zst_window_area.set(
                    [at],
                    self.input_state.zst_window_area.read([at])?
                        + m * l.glass.read([number as usize])?,
                )?;
                self.input_state.zst_opening_area.set(
                    [at],
                    self.input_state.zst_opening_area.read([at])?
                        + m * l.opening.read([number as usize])?,
                )?;
                self.input_state
                    .zst_light
                    .set([at], self.input_state.zst_light.read([at])? + m * light)?;
                self.input_state
                    .zst_people
                    .set([at], self.input_state.zst_people.read([at])? + m * people)?;
                self.input_state
                    .zst_plug
                    .set([at], self.input_state.zst_plug.read([at])? + m * plug)?;
                let at = count(if conditioned {
                    self.input_state.cond_total
                } else {
                    self.input_state.uncond_total
                })?;
                self.input_state.zst_area.set(
                    [at],
                    self.input_state.zst_area.read([at])? + m * z.floor_area,
                )?;
                self.input_state
                    .zst_volume
                    .set([at], self.input_state.zst_volume.read([at])? + m * z.volume)?;
                self.input_state.zst_wall_area.set(
                    [at],
                    self.input_state.zst_wall_area.read([at])? + m * z.exterior_gross_wall_area,
                )?;
                self.input_state.zst_und_wall_area.set(
                    [at],
                    self.input_state.zst_und_wall_area.read([at])?
                        + m * z.exterior_gross_ground_wall_area,
                )?;
                self.input_state.zst_window_area.set(
                    [at],
                    self.input_state.zst_window_area.read([at])?
                        + m * l.glass.read([number as usize])?,
                )?;
                self.input_state.zst_opening_area.set(
                    [at],
                    self.input_state.zst_opening_area.read([at])?
                        + m * l.opening.read([number as usize])?,
                )?;
                self.input_state
                    .zst_light
                    .set([at], self.input_state.zst_light.read([at])? + m * light)?;
                self.input_state
                    .zst_people
                    .set([at], self.input_state.zst_people.read([at])? + m * people)?;
                self.input_state
                    .zst_plug
                    .set([at], self.input_state.zst_plug.read([at])? + m * plug)?;
            }
        }
        for total in 1..=4 {
            let row = plus(o.globals.num_of_zones, total as i32)?;
            t.body.set(
                1,
                row,
                self.veri_real(
                    pass,
                    self.input_state.zst_area.read([total])? * self.input_state.m2_unit_conv_wvst,
                    2,
                )?,
            )?;
            t.body.set(
                4,
                row,
                self.veri_real(
                    pass,
                    self.input_state.zst_volume.read([total])? * self.input_state.m3_unit_conv,
                    2,
                )?,
            )?;
            t.body.set(
                6,
                row,
                self.veri_real(
                    pass,
                    self.input_state.zst_wall_area.read([total])?
                        * self.input_state.m2_unit_conv_wvst,
                    2,
                )?,
            )?;
            t.body.set(
                7,
                row,
                self.veri_real(
                    pass,
                    self.input_state.zst_und_wall_area.read([total])?
                        * self.input_state.m2_unit_conv_wvst,
                    2,
                )?,
            )?;
            t.body.set(
                8,
                row,
                self.veri_real(
                    pass,
                    self.input_state.zst_window_area.read([total])?
                        * self.input_state.m2_unit_conv_wvst,
                    2,
                )?,
            )?;
            t.body.set(
                9,
                row,
                self.veri_real(
                    pass,
                    self.input_state.zst_opening_area.read([total])?
                        * self.input_state.m2_unit_conv_wvst,
                    2,
                )?,
            )?;
            if self.input_state.zst_area.read([total])? != 0.0 {
                t.body.set(
                    10,
                    row,
                    self.veri_real(
                        pass,
                        self.input_state.zst_light.read([total])? * self.input_state.wm2_unit_conv
                            / self.input_state.zst_area.read([total])?,
                        4,
                    )?,
                )?;
                t.body.set(
                    12,
                    row,
                    self.veri_real(
                        pass,
                        self.input_state.zst_plug.read([total])? * self.input_state.wm2_unit_conv
                            / self.input_state.zst_area.read([total])?,
                        4,
                    )?,
                )?;
            }
            if self.input_state.zst_people.read([total])? != 0.0 {
                t.body.set(
                    11,
                    row,
                    self.veri_real(
                        pass,
                        self.input_state.zst_area.read([total])?
                            * self.input_state.m2_unit_conv_wvst
                            / self.input_state.zst_people.read([total])?,
                        2,
                    )?,
                )?;
            }
        }
        if self.input_state.tabular_report_passes[pass].produce_tabular {
            predef_real(
                o.registry,
                o.table,
                "pdchLeedSutSpArea",
                "Totals",
                self.input_state
                    .zst_area
                    .read([count(self.input_state.grand_total)?])?,
            )?;
            predef_real(
                o.registry,
                o.table,
                "pdchLeedSutOcArea",
                "Totals",
                self.input_state
                    .zst_area
                    .read([count(self.input_state.cond_total)?])?,
            )?;
            predef_real(
                o.registry,
                o.table,
                "pdchLeedSutUnArea",
                "Totals",
                self.input_state
                    .zst_area
                    .read([count(self.input_state.uncond_total)?])?,
            )?;
        }
        self.veri_emit(pass, b"Zone Summary", &mut t, o)
    }
}
