//! Actual WriteTableOfContents3211-3390 on retained report selections and rows.
use super::output_files::SourceTabularOutputStream;
use super::writers::source_anchor;
use super::{Error, SourceTabularReports};
use crate::heat_balance::ordinary_life_cycle_cost::SourceEconomicLifeCycleCost;
use crate::heat_balance::ordinary_utility_bills::SourceCostEstimateMonetary;
use crate::heat_balance::predefined_tables::PredefinedRegistry;
use crate::heat_balance::selected_schedule_manager::SelectedScheduleManager;
use crate::weather::day::{WeatherGlobalState, WeatherOwnerState};

pub(crate) struct SourceTabularTocOwners<'a> {
    pub(crate) registry: &'a PredefinedRegistry,
    pub(crate) life_cycle_cost: &'a SourceEconomicLifeCycleCost,
    pub(crate) monetary: &'a SourceCostEstimateMonetary,
    pub(crate) clock: &'a WeatherGlobalState,
    pub(crate) weather: &'a WeatherOwnerState,
    pub(crate) schedules: &'a SelectedScheduleManager,
}

fn report_link(stream: &mut SourceTabularOutputStream, name: &[u8]) -> Result<(), Error> {
    stream.bytes(b"<br><a href=\"#")?;
    stream.bytes(&source_anchor(name, b"Entire Facility"))?;
    stream.bytes(b"\">")?;
    stream.bytes(name)?;
    stream.bytes(b"</a>\n")
}
fn entry_link(
    stream: &mut SourceTabularOutputStream,
    section: &[u8],
    report: &[u8],
) -> Result<(), Error> {
    stream.bytes(b"<a href=\"#")?;
    stream.bytes(&source_anchor(section, report))?;
    stream.bytes(b"\">")?;
    stream.bytes(report)?;
    stream.bytes(b"</a>   |  \n")
}
impl SourceTabularReports {
    pub(crate) fn write_source_table_of_contents(
        &mut self,
        o: &SourceTabularTocOwners<'_>,
    ) -> Result<(), Error> {
        if self.display_life_cycle_cost_report && *o.life_cycle_cost.source_parameters_present() {
            self.add_source_table_of_contents_entry("Life-Cycle Cost Report", "Entire Facility")?;
        }
        for ordinal in 1..=self.input_state.number_of_styles {
            let at = usize::try_from(ordinal - 1)
                .map_err(|_| Error::Unavailable("actual TOC TableStyle ordinal"))?;
            if *self
                .input_state
                .table_styles
                .get(at)
                .ok_or(Error::Unavailable("actual TOC TableStyle array read3256"))?
                != 3
            {
                continue;
            }
            let stream = self
                .output_streams
                .streams
                .get_mut(at)
                .ok_or(Error::Unavailable(
                    "actual TOC TabularOutputFile pointer3257",
                ))?;
            stream.bytes(b"<hr>\n")?;
            stream.bytes(b"<a name=toc></a>\n")?;
            stream.bytes(b"<p><b>Table of Contents</b></p>\n")?;
            stream.bytes(b"<a href=\"#top\">Top</a>\n")?;
            if self.display_tabular_beps {
                report_link(stream, b"Annual Building Utility Performance Summary")?;
            }
            if self.display_tabular_veri_sum {
                report_link(stream, b"Input Verification and Results Summary")?;
            }
            if self.display_demand_end_use {
                report_link(stream, b"Demand End Use Components Summary")?;
            }
            if self.display_source_energy_end_use_summary {
                report_link(stream, b"Source Energy End Use Components Summary")?;
            }
            if *o.monetary.source_do_cost_estimate() {
                report_link(stream, b"Component Cost Economics Summary")?;
            }
            if self.display_component_sizing {
                report_link(stream, b"Component Sizing Summary")?;
            }
            if self.display_surface_shadowing {
                report_link(stream, b"Surface Shadowing Summary")?;
            }
            if self.display_adaptive_comfort {
                report_link(stream, b"Adaptive Comfort Summary")?;
            }
            if self.display_eio_summary {
                report_link(stream, b"Initialization Summary")?;
            }
            if self.display_heat_emissions_summary {
                report_link(stream, b"Annual Heat Emissions Summary")?;
            }
            if self.display_thermal_resilience_summary {
                report_link(stream, b"Annual Thermal Resilience Summary")?;
            }
            // Native3302-3309 CO2/Visual report links are commented out.
            for number in 1..=o.registry.report_count {
                let row = o
                    .registry
                    .reports
                    .as_ref()
                    .and_then(|rows| rows.get((number - 1) as usize))
                    .ok_or(Error::Unavailable("actual TOC reportName row3311"))?;
                if row.show {
                    report_link(stream, row.name_with_spaces.as_bytes())?;
                }
            }
            if o.clock.do_weath_sim {
                // Preserve empty-range behavior, checked first-row access and the first unsupported positive-child failure after prior writes.
                #[allow(clippy::never_loop)]
                for _number in 1..=self.input_state.monthly_input_count {
                    return Err(Error::Unavailable(
                        "actual TOC MonthlyInput.numTables first read3318",
                    ));
                }
                for number in 1..=self.input_state.output_table_binned_count {
                    let row = self
                        .input_state
                        .output_table_binned
                        .as_ref()
                        .and_then(|rows| rows.get((number - 1) as usize))
                        .ok_or(Error::Unavailable("actual TOC OutputTableBinned row3328"))?;
                    if row.number_of_tables > 0 {
                        stream.bytes(b"<p><b>")?;
                        stream.bytes(row.variable_or_meter.as_bytes())?;
                        if let Some(index) = row.schedule {
                            stream.bytes(b" [")?;
                            let schedule = o
                                .schedules
                                .schedules
                                .get(index.0)
                                .ok_or(Error::Unavailable("actual TOC Binned sched Name3332"))?;
                            stream.bytes(schedule.name.as_bytes())?;
                            stream.bytes(b"]</b></p> |\n")?;
                        } else {
                            stream.bytes(b"</b></p> |\n")?;
                        }
                        // Preserve empty-range behavior, checked first-row access and the first unsupported positive-child failure after prior writes.
                        #[allow(clippy::never_loop)]
                        for table in 1..=row.number_of_tables {
                            let _cur_table = row.result_index.checked_add(table - 1).ok_or(
                                Error::Unavailable("actual TOC curTable3336 defined int domain"),
                            )?;
                            // Actual empty curName is local; the next operation
                            // reads unitNames and, for IP, invokes LookupSItoIP.
                            if self.input_state.units_style_tabular == 4
                                || self.input_state.units_style_tabular == 5
                            {
                                return Err(Error::Unavailable(
                                    "actual TOC Binned unitNames/LookupSItoIP3339 first operation",
                                ));
                            }
                            return Err(Error::Unavailable(
                                "actual TOC Binned unitNames3345 first operation",
                            ));
                        }
                    }
                }
                if !self.input_state.annual_tables.is_empty() {
                    return Err(Error::Unavailable(
                        "actual AnnualTable.addTableOfContents m_name1251 first operation",
                    ));
                }
            }
            for number in 1..=self.input_state.toc_entries_count {
                let at = (number - 1) as usize;
                let count = self.input_state.toc_entries_count as usize;
                let rows = self
                    .input_state
                    .toc_entries
                    .as_mut()
                    .and_then(|rows| rows.get_mut(at..count))
                    .ok_or(Error::Unavailable("actual TOCEntries row3365"))?;
                let (first, rest) = rows
                    .split_first_mut()
                    .ok_or(Error::Unavailable("actual TOCEntries current row3365"))?;
                if !first.is_written {
                    let cur_section = &first.section_name;
                    stream.bytes(b"<p><b>")?;
                    stream.bytes(cur_section.as_bytes())?;
                    stream.bytes(b"</b></p> |\n")?;
                    // Split field/range borrows retain the Native const string
                    // reference while writing only each row's isWritten cell.
                    if !first.is_written && first.section_name == *cur_section {
                        entry_link(
                            stream,
                            first.section_name.as_bytes(),
                            first.report_name.as_bytes(),
                        )?;
                        first.is_written = true;
                    }
                    for row in rest {
                        if !row.is_written && row.section_name == *cur_section {
                            entry_link(
                                stream,
                                row.section_name.as_bytes(),
                                row.report_name.as_bytes(),
                            )?;
                            row.is_written = true;
                        }
                    }
                }
            }
            if o.weather.tot_report_periods > 0 {
                report_link(stream, b"Reporting Period Summary")?;
                // Preserve empty-range behavior, checked first-row access and the first unsupported positive-child failure after prior writes.
                #[allow(clippy::never_loop)]
                for _period in 1..=o.weather.tot_thermal_report_periods {
                    return Err(Error::Unavailable(
                        "actual Thermal AddTOCReportPeriod title3402 first read",
                    ));
                }
                // Preserve empty-range behavior, checked first-row access and the first unsupported positive-child failure after prior writes.
                #[allow(clippy::never_loop)]
                for _period in 1..=o.weather.tot_co2_report_periods {
                    return Err(Error::Unavailable(
                        "actual CO2 AddTOCReportPeriod title3402 first read",
                    ));
                }
                // Preserve empty-range behavior, checked first-row access and the first unsupported positive-child failure after prior writes.
                #[allow(clippy::never_loop)]
                for _period in 1..=o.weather.tot_visual_report_periods {
                    return Err(Error::Unavailable(
                        "actual Visual AddTOCReportPeriod title3402 first read",
                    ));
                }
            }
        }
        Ok(())
    }
}
