//! Actual BEPS8685-8917 LEED writes, cost fractions and same TAB accumulators.
use super::helpers::{column, predef_real};
use super::prepare::SourceBepsLocals;
use super::{Error, SourceBepsReportOwners, SourceTabularReports};
impl SourceTabularReports {
    pub(super) fn source_beps_leed_energy(
        &mut self,
        pass: usize,
        l: &SourceBepsLocals,
        unconvert: f64,
        unconvert_ip_except_elec: f64,
        o: &mut SourceBepsReportOwners<'_, '_>,
    ) -> Result<(), Error> {
        if self.input_state.building_gross_floor_area > 0.0 {
            if self.input_state.tabular_report_passes[pass].produce_tabular {
                predef_real(
                    o,
                    "pdchLeedEuiElec",
                    b"Interior Lighting (All)",
                    unconvert_ip_except_elec * 1000.0 * l.use_val.read([1, 3])?
                        / self.input_state.building_gross_floor_area,
                    2,
                )?;
                predef_real(
                    o,
                    "pdchLeedEuiElec",
                    b"Space Heating",
                    unconvert_ip_except_elec * 1000.0 * l.use_val.read([1, 1])?
                        / self.input_state.building_gross_floor_area,
                    2,
                )?;
                predef_real(
                    o,
                    "pdchLeedEuiElec",
                    b"Space Cooling",
                    unconvert_ip_except_elec * 1000.0 * l.use_val.read([1, 2])?
                        / self.input_state.building_gross_floor_area,
                    2,
                )?;
                predef_real(
                    o,
                    "pdchLeedEuiElec",
                    b"Fans (All)",
                    unconvert_ip_except_elec * 1000.0 * l.use_val.read([1, 7])?
                        / self.input_state.building_gross_floor_area,
                    2,
                )?;
                predef_real(
                    o,
                    "pdchLeedEuiElec",
                    b"Service Water Heating",
                    unconvert_ip_except_elec * 1000.0 * l.use_val.read([1, 12])?
                        / self.input_state.building_gross_floor_area,
                    2,
                )?;
                predef_real(
                    o,
                    "pdchLeedEuiElec",
                    b"Receptacle Equipment",
                    unconvert_ip_except_elec * 1000.0 * l.use_val.read([1, 5])?
                        / self.input_state.building_gross_floor_area,
                    2,
                )?;
                predef_real(
                    o,
                    "pdchLeedEuiElec",
                    b"Miscellaneous (All)",
                    unconvert_ip_except_elec * 1000.0 * (l.use_val.read([1, 15])?)
                        / self.input_state.building_gross_floor_area,
                    2,
                )?;
                predef_real(
                    o,
                    "pdchLeedEuiElec",
                    b"Subtotal",
                    unconvert_ip_except_elec * 1000.0 * l.use_val.read([1, 15])?
                        / self.input_state.building_gross_floor_area,
                    2,
                )?;
            }
        }
        if self.input_state.tabular_report_passes[pass].produce_tabular {
            predef_real(
                o,
                "pdchLeedEusTotal",
                b"Electricity",
                unconvert_ip_except_elec * l.use_val.read([1, 15])?,
                2,
            )?;
            predef_real(
                o,
                "pdchLeedEusProc",
                b"Electricity",
                unconvert_ip_except_elec * (l.use_val.read([1, 5])? + l.use_val.read([1, 13])?),
                2,
            )?;
        }
        let mut process_elec_cost: f64 = 0.0;
        if l.use_val.read([1, 15])? != 0.0 {
            let mut process_fraction: f64 =
                (l.use_val.read([1, 5])? + l.use_val.read([1, 13])?) / l.use_val.read([1, 15])?;
            process_elec_cost = o.predefined_summary.leed_electric_cost_total * process_fraction;
        }
        if self.input_state.tabular_report_passes[pass].produce_tabular {
            predef_real(o, "pdchLeedEcsProc", b"Electricity", process_elec_cost, 2)?;
            let subtable = column(o, "pdstLeedEneCostSum")?;
            o.registry.add_footnote(
                subtable,
                "Process energy cost based on ratio of process to total energy.",
            )?;
        }
        if self.input_state.building_gross_floor_area > 0.0 {
            if self.input_state.tabular_report_passes[pass].produce_tabular {
                predef_real(
                    o,
                    "pdchLeedEuiNatG",
                    b"Space Heating",
                    unconvert * 1000.0 * l.use_val.read([2, 1])?
                        / self.input_state.building_gross_floor_area,
                    2,
                )?;
                predef_real(
                    o,
                    "pdchLeedEuiNatG",
                    b"Service Water Heating",
                    unconvert * 1000.0 * l.use_val.read([2, 12])?
                        / self.input_state.building_gross_floor_area,
                    2,
                )?;
                predef_real(
                    o,
                    "pdchLeedEuiNatG",
                    b"Miscellaneous (All)",
                    unconvert * 1000.0 * l.use_val.read([2, 15])?
                        / self.input_state.building_gross_floor_area,
                    2,
                )?;
                predef_real(
                    o,
                    "pdchLeedEuiNatG",
                    b"Subtotal",
                    unconvert * 1000.0 * l.use_val.read([2, 15])?
                        / self.input_state.building_gross_floor_area,
                    2,
                )?;
            }
        }
        if self.input_state.tabular_report_passes[pass].produce_tabular {
            predef_real(
                o,
                "pdchLeedEusTotal",
                b"Natural Gas",
                unconvert * l.use_val.read([2, 15])?,
                2,
            )?;
            predef_real(
                o,
                "pdchLeedEusProc",
                b"Natural Gas",
                unconvert * (l.use_val.read([2, 5])? + l.use_val.read([2, 13])?),
                2,
            )?;
        }
        let mut process_gas_cost: f64 = 0.0;
        if l.use_val.read([2, 15])? != 0.0 {
            let mut process_fraction: f64 =
                (l.use_val.read([2, 5])? + l.use_val.read([2, 13])?) / l.use_val.read([2, 15])?;
            process_gas_cost = o.predefined_summary.leed_gas_cost_total * process_fraction;
        }
        if self.input_state.tabular_report_passes[pass].produce_tabular {
            predef_real(o, "pdchLeedEcsProc", b"Natural Gas", process_gas_cost, 2)?;
        }
        let mut use_val_col_add_fuel15: f64 = l.use_val.read([3, 15])?
            + l.use_val.read([4, 15])?
            + l.use_val.read([5, 15])?
            + l.use_val.read([6, 15])?
            + l.use_val.read([7, 15])?
            + l.use_val.read([8, 15])?
            + l.use_val.read([9, 15])?
            + l.use_val.read([10, 15])?;
        let mut use_val_col_add_fuel5: f64 = l.use_val.read([3, 5])?
            + l.use_val.read([4, 5])?
            + l.use_val.read([5, 5])?
            + l.use_val.read([6, 5])?
            + l.use_val.read([7, 5])?
            + l.use_val.read([8, 5])?
            + l.use_val.read([9, 5])?
            + l.use_val.read([10, 5])?;
        let mut use_val_col_add_fuel13: f64 = l.use_val.read([3, 13])?
            + l.use_val.read([4, 13])?
            + l.use_val.read([5, 13])?
            + l.use_val.read([6, 13])?
            + l.use_val.read([7, 13])?
            + l.use_val.read([8, 13])?
            + l.use_val.read([9, 13])?
            + l.use_val.read([10, 13])?;
        if self.input_state.building_gross_floor_area > 0.0 {
            if self.input_state.tabular_report_passes[pass].produce_tabular {
                predef_real(
                    o,
                    "pdchLeedEuiOthr",
                    b"Miscellaneous",
                    unconvert * 1000.0 * use_val_col_add_fuel15
                        / self.input_state.building_gross_floor_area,
                    2,
                )?;
                predef_real(
                    o,
                    "pdchLeedEuiOthr",
                    b"Subtotal",
                    unconvert * 1000.0 * use_val_col_add_fuel15
                        / self.input_state.building_gross_floor_area,
                    2,
                )?;
            }
        }
        if self.input_state.tabular_report_passes[pass].produce_tabular {
            predef_real(
                o,
                "pdchLeedEusTotal",
                b"Additional",
                (use_val_col_add_fuel15
                    + l.use_val.read([11, 15])?
                    + l.use_val.read([12, 15])?
                    + l.use_val.read([13, 15])?),
                2,
            )?;
            predef_real(
                o,
                "pdchLeedEusProc",
                b"Additional",
                unconvert
                    * (use_val_col_add_fuel5
                        + use_val_col_add_fuel13
                        + l.use_val.read([11, 5])?
                        + l.use_val.read([11, 13])?
                        + l.use_val.read([12, 5])?
                        + l.use_val.read([12, 13])?
                        + l.use_val.read([13, 5])?
                        + l.use_val.read([13, 13])?),
                2,
            )?;
        }
        let mut process_othr_cost: f64 = 0.0;
        if (use_val_col_add_fuel15
            + l.use_val.read([11, 15])?
            + l.use_val.read([12, 15])?
            + l.use_val.read([13, 15])?)
            > 0.001
        {
            let mut process_fraction: f64 = (use_val_col_add_fuel5
                + use_val_col_add_fuel13
                + l.use_val.read([11, 5])?
                + l.use_val.read([11, 13])?
                + l.use_val.read([12, 5])?
                + l.use_val.read([12, 13])?
                + l.use_val.read([13, 5])?
                + l.use_val.read([13, 13])?)
                / (use_val_col_add_fuel15
                    + l.use_val.read([11, 15])?
                    + l.use_val.read([12, 15])?
                    + l.use_val.read([13, 15])?);
            process_othr_cost = o.predefined_summary.leed_other_cost_total * process_fraction;
        }
        if self.input_state.tabular_report_passes[pass].produce_tabular {
            predef_real(o, "pdchLeedEcsProc", b"Additional", process_othr_cost, 2)?;
            predef_real(
                o,
                "pdchLeedEcsProc",
                b"Total",
                process_elec_cost + process_gas_cost + process_othr_cost,
                2,
            )?;
        }
        self.input_state.leed_site_int_lite = 0.0;
        self.input_state.leed_site_sp_heat = 0.0;
        self.input_state.leed_site_sp_cool = 0.0;
        self.input_state.leed_site_fan_int = 0.0;
        self.input_state.leed_site_srv_watr = 0.0;
        self.input_state.leed_site_recept = 0.0;
        self.input_state.leed_site_total = 0.0;
        for i_resource in 1..=13 {
            self.input_state.leed_site_int_lite += l.use_val.read([i_resource, 3])?;
            self.input_state.leed_site_sp_heat += l.use_val.read([i_resource, 1])?;
            self.input_state.leed_site_sp_cool += l.use_val.read([i_resource, 2])?;
            self.input_state.leed_site_fan_int += l.use_val.read([i_resource, 7])?;
            self.input_state.leed_site_srv_watr += l.use_val.read([i_resource, 12])?;
            self.input_state.leed_site_recept += l.use_val.read([i_resource, 5])?;
            self.input_state.leed_site_total += l.use_val.read([i_resource, 15])?;
        }
        if self.input_state.leed_site_total != 0.0 {
            if self.input_state.tabular_report_passes[pass].produce_tabular {
                predef_real(
                    o,
                    "pdchLeedEupPerc",
                    b"Interior Lighting (All)",
                    100.0 * self.input_state.leed_site_int_lite / self.input_state.leed_site_total,
                    2,
                )?;
                predef_real(
                    o,
                    "pdchLeedEupPerc",
                    b"Space Heating",
                    100.0 * self.input_state.leed_site_sp_heat / self.input_state.leed_site_total,
                    2,
                )?;
                predef_real(
                    o,
                    "pdchLeedEupPerc",
                    b"Space Cooling",
                    100.0 * self.input_state.leed_site_sp_cool / self.input_state.leed_site_total,
                    2,
                )?;
                predef_real(
                    o,
                    "pdchLeedEupPerc",
                    b"Fans (All)",
                    100.0 * self.input_state.leed_site_fan_int / self.input_state.leed_site_total,
                    2,
                )?;
                predef_real(
                    o,
                    "pdchLeedEupPerc",
                    b"Service Water Heating",
                    100.0 * self.input_state.leed_site_srv_watr / self.input_state.leed_site_total,
                    2,
                )?;
                predef_real(
                    o,
                    "pdchLeedEupPerc",
                    b"Receptacle Equipment",
                    100.0 * self.input_state.leed_site_recept / self.input_state.leed_site_total,
                    2,
                )?;
                predef_real(
                    o,
                    "pdchLeedEupPerc",
                    b"Miscellaneous",
                    100.0
                        * (self.input_state.leed_site_total
                            - (self.input_state.leed_site_int_lite
                                + self.input_state.leed_site_sp_heat
                                + self.input_state.leed_site_sp_cool
                                + self.input_state.leed_site_fan_int
                                + self.input_state.leed_site_srv_watr
                                + self.input_state.leed_site_recept))
                        / self.input_state.leed_site_total,
                    2,
                )?;
            }
        }
        if self.input_state.tabular_report_passes[pass].produce_tabular {
            predef_real(
                o,
                "pdchLeedEusTotal",
                b"Total",
                unconvert_ip_except_elec * l.use_val.read([1, 15])?
                    + unconvert
                        * (use_val_col_add_fuel15
                            + l.use_val.read([11, 15])?
                            + l.use_val.read([12, 15])?
                            + l.use_val.read([13, 15])?
                            + l.use_val.read([2, 15])?),
                2,
            )?;
            predef_real(
                o,
                "pdchLeedEusProc",
                b"Total",
                unconvert_ip_except_elec * (l.use_val.read([1, 5])? + l.use_val.read([1, 13])?)
                    + unconvert
                        * (use_val_col_add_fuel5
                            + use_val_col_add_fuel13
                            + l.use_val.read([11, 5])?
                            + l.use_val.read([11, 13])?
                            + l.use_val.read([12, 5])?
                            + l.use_val.read([12, 13])?
                            + l.use_val.read([13, 5])?
                            + l.use_val.read([13, 13])?
                            + l.use_val.read([2, 5])?
                            + l.use_val.read([2, 13])?),
                2,
            )?;
        }
        Ok(())
    }
}
