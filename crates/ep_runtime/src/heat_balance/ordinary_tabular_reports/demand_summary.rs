//! Actual WriteDemandEndUseSummary10323-10820, finite nonIP reporting route.
mod helpers;
use super::{Error, SourceTabularBody, SourceTabularReports, SourceTabularWriterOwners};
use crate::heat_balance::ctf_reports::PredefinedStringTable;
use crate::heat_balance::ordinary_output_control::SourceResultsFrameworkOptions;
use crate::heat_balance::predefined_tables::PredefinedRegistry;
use crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray;
use crate::heat_balance::source_initialization::SourceSqliteState;
use crate::heat_balance::source_output_registration::{
    SourceOutputRegistration, SourceOutputStoreType,
};
use helpers::{Table, category, count, increment, real, subcategory};
pub(crate) struct SourceDemandReportOwners<'a, 'backend> {
    pub(crate) registration: &'a SourceOutputRegistration,
    pub(crate) registry: &'a PredefinedRegistry,
    pub(crate) table: &'a mut PredefinedStringTable,
    pub(crate) results: &'a SourceResultsFrameworkOptions,
    pub(crate) sqlite: &'a SourceSqliteState,
    pub(crate) writer: SourceTabularWriterOwners<'a, 'backend>,
}
const RESOURCES: [usize; 14] = [1, 2, 6, 8, 9, 10, 11, 12, 13, 14, 3, 4, 5, 7];
struct Locals {
    use_val: SourceSolarArray<f64, 2>,
    total: SourceSolarArray<f64, 1>,
    end: SourceSolarArray<f64, 2>,
    ind: SourceSolarArray<f64, 2>,
    time: SourceSolarArray<i32, 1>,
    sub: SourceSolarArray<f64, 3>,
    ind_sub: SourceSolarArray<f64, 3>,
    other: SourceSolarArray<f64, 2>,
}
fn allocate<T: Copy, const N: usize>(shape: [usize; N]) -> Result<SourceSolarArray<T, N>, Error> {
    let mut a = SourceSolarArray::default();
    a.dimension(shape, None)?;
    Ok(a)
}
impl SourceTabularReports {
    pub(crate) fn write_source_demand_end_use_summary(
        &mut self,
        o: &mut SourceDemandReportOwners<'_, '_>,
    ) -> Result<(), Error> {
        if !self.display_demand_end_use {
            return Ok(());
        }
        let use_val = allocate([14, 15])?;
        let total = allocate([14])?;
        let end = allocate([14, 14])?;
        let ind = allocate([14, 14])?;
        let time = allocate([14])?;
        let sub = allocate([count(o.registration.max_num_subcategories)?, 14, 14])?;
        let ind_sub = allocate([count(o.registration.max_num_subcategories)?, 14, 14])?;
        let other = allocate([14, 14])?;
        let mut l = Locals {
            use_val,
            total,
            end,
            ind,
            time,
            sub,
            ind_sub,
            other,
        };
        self.write_source_report_headers(
            b"Demand End Use Components Summary",
            b"Entire Facility",
            SourceOutputStoreType::Average,
        )?;
        let mut ip_electricity = 1.0;
        for pass in 0..self.input_state.tabular_report_passes.len() {
            for resource in 1..=14 {
                l.total.set([resource], 0.0)?;
            }
            for (at, &source) in RESOURCES.iter().enumerate() {
                l.total
                    .set([at + 1], self.input_state.gather_demand_total[source - 1])?;
                l.time.set(
                    [at + 1],
                    self.input_state.gather_demand_time_stamp[source - 1],
                )?;
            }
            let power = 1.0;
            let flow = 1.0;
            ip_electricity = 1.0;
            if self.input_state.tabular_report_passes[pass].units_style == 4
                || self.input_state.tabular_report_passes[pass].units_style == 5
            {
                return Err(Error::Unavailable(
                    "actual Demand getSpecificUnitMultiplier10407/10410 first operation",
                ));
            }
            for resource in 1..=14 {
                for end_use in 1..=14 {
                    l.end.set([resource, end_use], 0.0)?;
                }
            }
            for end_use in 1..=14 {
                for (at, &source) in RESOURCES.iter().enumerate() {
                    let factor = if at == 0 {
                        power / ip_electricity
                    } else if at == 13 {
                        flow
                    } else {
                        power
                    };
                    l.end.set(
                        [at + 1, end_use],
                        self.input_state
                            .gather_demand_end_use
                            .read([source, end_use])?
                            * factor,
                    )?;
                }
            }
            for end_use in 1..=14 {
                for sub in 1..=category(o.registration, end_use)?.num_subcategories {
                    for (at, &source) in RESOURCES.iter().enumerate() {
                        let factor = if at == 0 {
                            power / ip_electricity
                        } else if at == 13 {
                            flow
                        } else {
                            power
                        };
                        l.sub.set(
                            [sub as usize, end_use, at + 1],
                            self.input_state.gather_demand_end_use_sub.read([
                                sub as usize,
                                end_use,
                                source,
                            ])? * factor,
                        )?;
                    }
                }
            }
            for resource in 1..=14 {
                for end_use in 1..=14 {
                    l.ind.set([resource, end_use], 0.0)?;
                }
            }
            for end_use in 1..=14 {
                for (at, &source) in RESOURCES.iter().enumerate() {
                    let value = self
                        .input_state
                        .gather_demand_ind_end_use
                        .read([source, end_use])?;
                    l.ind.set(
                        [at + 1, end_use],
                        if at == 0 {
                            value * (1.0 / ip_electricity)
                        } else {
                            value
                        },
                    )?;
                }
            }
            for end_use in 1..=14 {
                for sub in 1..=category(o.registration, end_use)?.num_subcategories {
                    for (at, &source) in RESOURCES.iter().enumerate() {
                        let value = self.input_state.gather_demand_ind_end_use_sub.read([
                            sub as usize,
                            end_use,
                            source,
                        ])?;
                        l.ind_sub.set(
                            [sub as usize, end_use, at + 1],
                            if at == 0 {
                                value * (1.0 / ip_electricity)
                            } else {
                                value
                            },
                        )?;
                    }
                }
            }
            for resource in 1..=14 {
                let factor = if resource == 1 {
                    power / ip_electricity
                } else if resource == 14 {
                    flow
                } else {
                    power
                };
                l.total
                    .set([resource], l.total.read([resource])? * factor)?;
            }
            let mut t = Table::new(14, 17)?;
            for resource in 1..=14 {
                for end_use in 1..=14 {
                    l.use_val
                        .set([resource, end_use], l.end.read([resource, end_use])?)?;
                }
                l.use_val.set([resource, 15], l.total.read([resource])?)?;
            }
            t.rows[0] = b"Time of Peak".to_vec();
            for (at, name) in helpers::END_USES.iter().enumerate() {
                t.rows[at + 1] = name.as_bytes().to_vec();
            }
            t.rows[15] = Vec::new();
            t.rows[16] = b"Total End Uses".to_vec();
            for (at, name) in helpers::RESOURCE_NAMES.iter().enumerate() {
                t.cols[at] = format!("{name} [W]").into_bytes();
            }
            t.cols[13] = b"Water [m3/s]".to_vec();
            t.body.fill(b"");
            for resource in 1..=14 {
                for end_use in 1..=14 {
                    t.body.set(
                        resource,
                        1 + end_use,
                        self.demand_real(pass, l.use_val.read([resource, end_use])?, 2)?,
                    )?;
                }
                t.body.set(
                    resource,
                    1,
                    super::remaining::source_date_to_string(l.time.read([resource])?)?.into_bytes(),
                )?;
                t.body.set(
                    resource,
                    17,
                    self.demand_real(pass, l.total.read([resource])?, 2)?,
                )?;
            }
            self.demand_emit(pass, b"End Uses", &mut t, o)?;
            self.demand_subcategory_table(pass, &l, o)?;
        }
        // Actual post-style LEED individual peaks consume the final local pass.
        for resource in 1..=14 {
            for end_use in 1..=14 {
                if self.input_state.need_other_row_leed45.read([end_use])? {
                    if category(o.registration, end_use)?.num_subcategories == 0 {
                        l.other
                            .set([resource, end_use], l.ind.read([resource, end_use])?)?;
                    } else {
                        let mut sum = 0.0;
                        for sub in 1..=category(o.registration, end_use)?.num_subcategories {
                            sum += l.ind_sub.read([sub as usize, end_use, resource])?;
                        }
                        l.other
                            .set([resource, end_use], l.ind.read([resource, end_use])? - sum)?;
                        if l.other.read([resource, end_use])? < 0.0 {
                            l.other.set([resource, end_use], 0.0)?;
                        }
                    }
                }
            }
        }
        let columns = [
            "pdchLeedPerfElDem",
            "pdchLeedPerfGasDem",
            "pdchLeedPerfGasolineDem",
            "pdchLeedPerfDieselDem",
            "pdchLeedPerfCoalDem",
            "pdchLeedPerfFuelOil1Dem",
            "pdchLeedPerfFuelOil2Dem",
            "pdchLeedPerfPropaneDem",
            "pdchLeedPerfOtherFuel1Dem",
            "pdchLeedPerfOtherFuel2Dem",
            "pdchLeedPerfDisClDem",
            "pdchLeedPerfDisHtWtrDem",
            "pdchLeedPerfDisHtStDem",
        ];
        let mut ids = SourceSolarArray::default();
        ids.dimension([13], None)?;
        for (at, name) in columns.iter().enumerate() {
            ids.set(
                [at + 1],
                *o.registry
                    .named_indices
                    .get(*name)
                    .ok_or(Error::Unavailable(
                        "actual Demand resource_entry_map predefined index",
                    ))?,
            )?;
        }
        for resource in 1..=13 {
            let mut row: i32 = 1;
            let unconvert = if resource == 1 { ip_electricity } else { 1.0 };
            for end_use in 1..=14 {
                if category(o.registration, end_use)?.num_subcategories > 0 {
                    for sub in 1..=category(o.registration, end_use)?.num_subcategories {
                        let mut object = category(o.registration, end_use)?
                            .display_name
                            .as_bytes()
                            .to_vec();
                        object.extend_from_slice(b" -- ");
                        object.extend_from_slice(&subcategory(
                            o.registration,
                            end_use,
                            sub as usize,
                        )?);
                        helpers::predef(
                            o,
                            ids.read([resource])?,
                            &object,
                            unconvert * l.ind_sub.read([sub as usize, end_use, resource])?,
                        )?;
                        increment(&mut row)?;
                    }
                    if self.input_state.need_other_row_leed45.read([end_use])? {
                        let mut object = category(o.registration, end_use)?
                            .display_name
                            .as_bytes()
                            .to_vec();
                        object.extend_from_slice(b" -- Other");
                        helpers::predef(
                            o,
                            ids.read([resource])?,
                            &object,
                            unconvert * l.other.read([resource, end_use])?,
                        )?;
                        increment(&mut row)?;
                    }
                } else {
                    let mut object = category(o.registration, end_use)?
                        .display_name
                        .as_bytes()
                        .to_vec();
                    object.extend_from_slice(b" -- Not Subdivided");
                    helpers::predef(
                        o,
                        ids.read([resource])?,
                        &object,
                        unconvert * l.ind.read([resource, end_use])?,
                    )?;
                    increment(&mut row)?;
                }
            }
        }
        Ok(())
    }
    fn demand_real(&self, pass: usize, value: f64, digits: i32) -> Result<Vec<u8>, Error> {
        real(
            self.input_state.tabular_report_passes[pass].format_reals,
            value,
            digits,
        )
    }
    fn demand_emit(
        &mut self,
        pass: usize,
        title: &[u8],
        t: &mut Table,
        o: &mut SourceDemandReportOwners<'_, '_>,
    ) -> Result<(), Error> {
        if self.input_state.tabular_report_passes[pass].produce_tabular {
            self.write_source_subtitle(title)?;
            // Immutable char metadata has no Native writer anywhere in this
            // function/module; short local text owns only the Rust borrow bridge.
            let footnote = self.input_state.footnote.clone();
            self.write_source_table(
                &t.body,
                &t.rows,
                &t.cols,
                &mut t.widths,
                false,
                &footnote,
                &mut o.writer,
            )?;
        }
        if self.input_state.tabular_report_passes[pass].produce_sqlite && o.sqlite.present() {
            return Err(Error::Unavailable(
                "actual Demand SQLite first createRecords",
            ));
        }
        if self.input_state.tabular_report_passes[pass].produce_json
            && o.results.time_series_and_tabular_enabled
        {
            return Err(Error::Unavailable(
                "actual Demand JSON first addReportTable",
            ));
        }
        Ok(())
    }
    fn demand_subcategory_table(
        &mut self,
        pass: usize,
        l: &Locals,
        o: &mut SourceDemandReportOwners<'_, '_>,
    ) -> Result<(), Error> {
        let mut rows: i32 = 0;
        for end_use in 1..=14 {
            if category(o.registration, end_use)?.num_subcategories > 0 {
                for _sub in 1..=category(o.registration, end_use)?.num_subcategories {
                    increment(&mut rows)?;
                }
            } else {
                increment(&mut rows)?;
            }
        }
        let mut t = Table::new(15, count(rows)?)?;
        t.body.fill(b"");
        let mut row: i32 = 1;
        for end_use in 1..=14 {
            t.rows[count(row - 1)?] = category(o.registration, end_use)?
                .display_name
                .as_bytes()
                .to_vec();
            if category(o.registration, end_use)?.num_subcategories > 0 {
                for sub in 1..=category(o.registration, end_use)?.num_subcategories {
                    t.body.set(
                        1,
                        count(row)?,
                        subcategory(o.registration, end_use, sub as usize)?,
                    )?;
                    increment(&mut row)?;
                }
            } else {
                t.body.set(1, count(row)?, b"General".to_vec())?;
                increment(&mut row)?;
            }
        }
        t.cols[0] = b"Subcategory".to_vec();
        for (at, name) in helpers::RESOURCE_NAMES.iter().enumerate() {
            t.cols[at + 1] = format!("{name} [W]").into_bytes();
        }
        t.cols[14] = b"Water [m3/s]".to_vec();
        for resource in 1..=14 {
            let mut row: i32 = 1;
            for end_use in 1..=14 {
                if category(o.registration, end_use)?.num_subcategories > 0 {
                    for sub in 1..=category(o.registration, end_use)?.num_subcategories {
                        t.body.set(
                            resource + 1,
                            count(row)?,
                            self.demand_real(
                                pass,
                                l.sub.read([sub as usize, end_use, resource])?,
                                2,
                            )?,
                        )?;
                        increment(&mut row)?;
                    }
                } else {
                    t.body.set(
                        resource + 1,
                        count(row)?,
                        self.demand_real(pass, l.end.read([resource, end_use])?, 2)?,
                    )?;
                    increment(&mut row)?;
                }
            }
        }
        if self.input_state.tabular_report_passes[pass].produce_tabular {
            self.write_source_subtitle(b"End Uses By Subcategory")?;
            let footnote = self.input_state.footnote.clone();
            self.write_source_table(
                &t.body,
                &t.rows,
                &t.cols,
                &mut t.widths,
                false,
                &footnote,
                &mut o.writer,
            )?;
        }
        let mut row_temp = t.rows.clone();
        super::writers::source_fill_row_head(&mut row_temp);
        for row in 1..=count(rows)? {
            row_temp[row - 1].push(b':');
            row_temp[row - 1].extend_from_slice(t.body.at(1, row)?);
        }
        let mut body_temp = SourceTabularBody::allocate(14, count(rows)?)?;
        for col in 1..=14 {
            for row in 1..=count(rows)? {
                body_temp.set(col, row, t.body.at(col + 1, row)?.to_vec())?;
            }
        }
        let column_temp = t.cols[1..].to_vec();
        if self.input_state.tabular_report_passes[pass].produce_sqlite && o.sqlite.present() {
            return Err(Error::Unavailable(
                "actual Demand subcategory SQLite first createRecords10739",
            ));
        }
        if self.input_state.tabular_report_passes[pass].produce_json
            && o.results.time_series_and_tabular_enabled
        {
            return Err(Error::Unavailable(
                "actual Demand subcategory JSON first addReportTable10746",
            ));
        }
        drop(row_temp);
        drop(body_temp);
        drop(column_temp);
        Ok(())
    }
}
