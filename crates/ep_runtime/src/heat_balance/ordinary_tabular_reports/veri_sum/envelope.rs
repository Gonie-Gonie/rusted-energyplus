//! Literal opaque surface classification, ratios and Source wall consistency.
use super::helpers::{Table, label, mult, safe_divide, zone};
use super::{Error, Locals, SourceTabularReports, SourceVeriSumOwners};
use crate::heat_balance::selected_heat_balance_input::surface::SurfaceClass;
pub(super) struct Walls {
    wall: [f64; 4],
}
fn total(a: &[f64; 4]) -> f64 {
    a[0] + a[1] + a[2] + a[3]
}
impl SourceTabularReports {
    // Preserve Original ordered bounds; range negation can change floating-point NaN behavior.
    #[allow(clippy::manual_range_contains)]
    pub(super) fn veri_envelope(
        &mut self,
        pass: usize,
        l: &mut Locals,
        o: &mut SourceVeriSumOwners<'_, '_>,
    ) -> Result<Walls, Error> {
        if self.input_state.tabular_report_passes[pass].produce_tabular {
            self.write_source_text_line(b"ENVELOPE", true)?;
        }
        let mut t = self.veri_wwr_table()?;
        let mut wall = [0.0; 4];
        let mut above = [0.0; 4];
        let window = [0.0; 4];
        let mut wall_cond = [0.0; 4];
        let mut above_cond = [0.0; 4];
        let window_cond = [0.0; 4];
        let mut roof = 0.0;
        let skylight = 0.0;
        let detailed = o
            .input
            .get_num_sections_found("DETAILEDWWR_DEBUG")
            .map_err(|_| Error::Unavailable("actual VeriSum getNumSectionsFound detailedWWR"))?
            > 0;
        if detailed && self.input_state.tabular_report_passes[pass].produce_tabular {
            return Err(Error::Unavailable(
                "actual VeriSum first DetailedWWR debug print11331",
            ));
        }
        for number in 1..=o.geometry.total_surfaces {
            let surface = o
                .geometry
                .final_surfaces
                .as_ref()
                .and_then(|r| r.get((number - 1) as usize))
                .ok_or(Error::Unavailable("actual VeriSum Surface row11338"))?;
            let row = &surface.row;
            if !row.heat_transfer_surface {
                continue;
            }
            let is_above = row.exterior_boundary == 0 || row.exterior_boundary == -4;
            if is_above
                || row.exterior_boundary == -1
                || row.exterior_boundary == -5
                || row.exterior_boundary == -6
            {
                let orientation =
                    row.vertex_progress
                        .orientation_return
                        .as_ref()
                        .ok_or(Error::Unavailable(
                            "actual VeriSum retained Surface.Azimuth11345",
                        ))?;
                let mut azimuth = orientation.azimuth_deg;
                azimuth = (azimuth * 100.0).round() / 100.0;
                let area = row.gross_area;
                if row.frame_divider != 0 {
                    return Err(Error::Unavailable(
                        "actual VeriSum FrameDivider.FrameWidth11350 first read",
                    ));
                }
                let zone_number = row.zone;
                let conditioned =
                    zone_number > 0 && zone(o.zones, zone_number)?.system_zone_node_number > 0;
                if orientation.tilt_deg >= 60.0 && orientation.tilt_deg <= 120.0 {
                    match row.class {
                        SurfaceClass::Wall | SurfaceClass::Floor | SurfaceClass::Roof => {
                            let z = zone(o.zones, zone_number)?;
                            let m = mult(z)?;
                            let axis = if azimuth >= 315.0 || azimuth < 45.0 {
                                Some(0)
                            } else if azimuth >= 45.0 && azimuth < 135.0 {
                                Some(2)
                            } else if azimuth >= 135.0 && azimuth < 225.0 {
                                Some(1)
                            } else if azimuth >= 225.0 && azimuth < 315.0 {
                                Some(3)
                            } else {
                                None
                            };
                            if let Some(axis) = axis {
                                wall[axis] += area * m;
                                if conditioned {
                                    wall_cond[axis] += area * m;
                                }
                                if is_above {
                                    above[axis] += area * m;
                                    if conditioned {
                                        above_cond[axis] += area * m;
                                    }
                                }
                            }
                            if detailed
                                && self.input_state.tabular_report_passes[pass].produce_tabular
                            {
                                return Err(Error::Unavailable(
                                    "actual VeriSum DetailedWWR Wall debug print11413",
                                ));
                            }
                        }
                        SurfaceClass::Window | SurfaceClass::TddDome => {
                            let z = zone(o.zones, zone_number)?;
                            let _m = mult(z)?;
                            return Err(Error::Unavailable(
                                "actual VeriSum window Surface.Multiplier11420 first read",
                            ));
                        }
                        _ => {}
                    }
                } else if orientation.tilt_deg < 60.0 {
                    match row.class {
                        SurfaceClass::Wall | SurfaceClass::Floor | SurfaceClass::Roof => {
                            let z = zone(o.zones, zone_number)?;
                            let m = mult(z)?;
                            roof += area * m;
                            if detailed
                                && self.input_state.tabular_report_passes[pass].produce_tabular
                            {
                                return Err(Error::Unavailable(
                                    "actual VeriSum DetailedWWR Roof debug print11463",
                                ));
                            }
                        }
                        SurfaceClass::Window | SurfaceClass::TddDome => {
                            let z = zone(o.zones, zone_number)?;
                            let _m = mult(z)?;
                            return Err(Error::Unavailable(
                                "actual VeriSum skylight Surface.Multiplier11470 first read",
                            ));
                        }
                        _ => {}
                    }
                }
            }
        }
        let wall_total = total(&wall);
        let above_total = total(&above);
        let window_total = total(&window);
        if detailed && self.input_state.tabular_report_passes[pass].produce_tabular {
            return Err(Error::Unavailable(
                "actual VeriSum DetailedWWR total debug print11492",
            ));
        }
        t.body.fill(b"");
        self.veri_wwr_values(
            pass,
            &mut t,
            &wall,
            &above,
            &window,
            wall_total,
            above_total,
            window_total,
        )?;
        self.veri_emit(pass, b"Window-Wall Ratio", &mut t, o)?;
        t = self.veri_wwr_table()?;
        let wall_total = total(&wall_cond);
        let above_total = total(&above_cond);
        let window_total = total(&window_cond);
        t.body.fill(b"");
        self.veri_wwr_values(
            pass,
            &mut t,
            &wall_cond,
            &above_cond,
            &window_cond,
            wall_total,
            above_total,
            window_total,
        )?;
        self.veri_emit(pass, b"Conditioned Window-Wall Ratio", &mut t, o)?;
        let mut t = Table::new(1, 3)?;
        t.cols[0] = b"Total".to_vec();
        t.rows[0] = label(b"Gross Roof Area ", &self.input_state.m2_unit_name);
        t.rows[1] = label(b"Skylight Area ", &self.input_state.m2_unit_name);
        t.rows[2] = b"Skylight-Roof Ratio [%]".to_vec();
        if detailed && self.input_state.tabular_report_passes[pass].produce_tabular {
            return Err(Error::Unavailable(
                "actual VeriSum DetailedWWR roof total debug print11642",
            ));
        }
        t.body.set(
            1,
            1,
            self.veri_real(pass, roof * self.input_state.m2_unit_conv_wvst, 2)?,
        )?;
        t.body.set(
            1,
            2,
            self.veri_real(pass, skylight * self.input_state.m2_unit_conv_wvst, 2)?,
        )?;
        t.body.set(
            1,
            3,
            self.veri_real(pass, 100.0 * safe_divide(skylight, roof), 2)?,
        )?;
        self.veri_emit(pass, b"Skylight-Roof Ratio", &mut t, o)?;
        // Actual local opening/glass arrays are unchanged on the opaque branch;
        // they remain the same arrays consumed by the subsequent Zone table.
        let _ = l;
        Ok(Walls { wall })
    }
    fn veri_wwr_table(&self) -> Result<Table, Error> {
        let mut t = Table::new(5, 5)?;
        for (i, s) in [
            "Total",
            "North (315 to 45 deg)",
            "East (45 to 135 deg)",
            "South (135 to 225 deg)",
            "West (225 to 315 deg)",
        ]
        .iter()
        .enumerate()
        {
            t.cols[i] = s.as_bytes().to_vec();
        }
        t.rows[0] = label(b"Gross Wall Area ", &self.input_state.m2_unit_name);
        t.rows[1] = label(b"Above Ground Wall Area ", &self.input_state.m2_unit_name);
        t.rows[2] = label(b"Window Opening Area ", &self.input_state.m2_unit_name);
        t.rows[3] = b"Gross Window-Wall Ratio [%]".to_vec();
        t.rows[4] = b"Above Ground Window-Wall Ratio [%]".to_vec();
        Ok(t)
    }
    fn veri_wwr_values(
        &self,
        pass: usize,
        t: &mut Table,
        w: &[f64; 4],
        a: &[f64; 4],
        g: &[f64; 4],
        wt: f64,
        at: f64,
        gt: f64,
    ) -> Result<(), Error> {
        for (row, values, sum) in [(1, w, wt), (2, a, at), (3, g, gt)] {
            for (axis, col) in [(0, 2), (1, 4), (2, 3), (3, 5)] {
                t.body.set(
                    col,
                    row,
                    self.veri_real(pass, values[axis] * self.input_state.m2_unit_conv_wvst, 2)?,
                )?;
            }
            t.body.set(
                1,
                row,
                self.veri_real(pass, sum * self.input_state.m2_unit_conv_wvst, 2)?,
            )?;
        }
        for (row, denominator, sum) in [(4, w, wt), (5, a, at)] {
            for (axis, col) in [(0, 2), (1, 4), (2, 3), (3, 5)] {
                t.body.set(
                    col,
                    row,
                    self.veri_real(pass, 100.0 * safe_divide(g[axis], denominator[axis]), 2)?,
                )?;
            }
            t.body.set(
                1,
                row,
                self.veri_real(pass, 100.0 * safe_divide(gt, sum), 2)?,
            )?;
        }
        Ok(())
    }
    pub(super) fn veri_wall_area_warning(
        &mut self,
        pass: usize,
        walls: &Walls,
        o: &mut SourceVeriSumOwners<'_, '_>,
    ) -> Result<(), Error> {
        let rows = o.zones.zones.as_ref().ok_or(Error::Unavailable(
            "actual VeriSum Zone whole-array sum11712",
        ))?;
        let mut above = 0.0;
        for z in rows {
            above += z.exterior_gross_wall_area_multiplied;
        }
        let mut ground = 0.0;
        for z in rows {
            ground += z.exterior_gross_ground_wall_area_multiplied;
        }
        if above > 0.0 || ground > 0.0 {
            let diff = (total(&walls.wall) - (above + ground)).abs() / (above + ground);
            if diff > 0.019 && self.input_state.tabular_report_passes[pass].produce_tabular {
                o.writer.backend.show_source_warning_error("WriteVeriSumTable: InputVerificationsAndResultsSummary: Wall area based on [>=60,<=120] degrees (tilt) as walls",&o.writer.flags)?;
                // The preceding live Warning rejects a positive error callback
                // at its first Native position, so these continuations cannot
                // reach a compatibility callback with copied diagnostic flags.
                let value = crate::heat_balance::source_round_format::round(diff * 100.0, 1)
                    .map_err(|error| Error::Format(format!("actual VeriSum .1R: {error:?}")))?;
                o.writer.backend.show_continue_error(&format!("differs ~{value}% from user entered Wall class surfaces. Degree calculation based on ASHRAE 90.1 wall definitions."))?;
                for (prefix, value) in [
                    (
                        "Total wall area by ASHRAE 90.1 definition=",
                        total(&walls.wall),
                    ),
                    ("Total exterior wall area from user entered classes=", above),
                    (
                        "Total ground contact wall area from user entered classes=",
                        ground,
                    ),
                ] {
                    let text = self.veri_real(pass, value, 3)?;
                    let text = super::super::writers::source_stripped_report_text(&text);
                    let text = std::str::from_utf8(text).map_err(|_| {
                        Error::Unavailable("actual VeriSum formatted continue ASCII string")
                    })?;
                    o.writer
                        .backend
                        .show_continue_error(&format!("{prefix}{text} m2."))?;
                }
            }
        }
        Ok(())
    }
}
