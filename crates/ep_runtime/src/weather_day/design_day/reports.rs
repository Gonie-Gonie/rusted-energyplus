//! Actual selected six GetDesignDayData predefined report writes, in getter order.
use super::{DesignDayHumidity, DesignDayInput};
use crate::heat_balance::ctf_initialization_context::CtfInitializationReports;
use crate::heat_balance::ctf_reports::CtfReportUnavailable;
use crate::weather::day::WeatherEnvironmentConfiguration;

fn column(reports: &CtfInitializationReports, key: &str) -> Result<i32, CtfReportUnavailable> {
    let column = *reports
        .registry
        .named_indices
        .get(key)
        .ok_or(CtfReportUnavailable::TableOwner)?;
    if column <= 0 || column > reports.registry.column_count {
        return Err(CtfReportUnavailable::TableOwner);
    }
    Ok(column)
}

impl DesignDayInput {
    fn register_predefined_reports(
        &self,
        reports: &mut CtfInitializationReports,
    ) -> Result<(), CtfReportUnavailable> {
        let input = &self.parameters;
        // Each operation uses the same real increment/allocation/table owner.
        // A failed later operation retains the already reached cell prefix.
        reports.table.fixed_real_entry_2(
            column(reports, "pdchDDmaxDB")?,
            &input.name,
            input.maximum_dry_bulb,
        )?;
        reports.table.fixed_real_entry_2(
            column(reports, "pdchDDrange")?,
            &input.name,
            input.daily_dry_bulb_range,
        )?;
        reports.table.fixed_real_entry_2(
            column(reports, "pdchDDhumid")?,
            &input.name,
            input.humidity_indicator,
        )?;
        let humidity_type = match input.humidity_type {
            DesignDayHumidity::WetBulb => "Wetbulb [C]",
            DesignDayHumidity::DewPoint => "Dewpoint [C]",
        };
        reports
            .table
            .string_entry(column(reports, "pdchDDhumTyp")?, &input.name, humidity_type)?;
        reports.table.fixed_real_entry_2(
            column(reports, "pdchDDwindSp")?,
            &input.name,
            input.wind_speed,
        )?;
        reports.table.fixed_real_entry_2(
            column(reports, "pdchDDwindDr")?,
            &input.name,
            input.wind_direction,
        )?;
        Ok(())
    }
}

impl WeatherEnvironmentConfiguration {
    /// Reached once by the actual weather-input getter after SetPredefinedTables.
    /// The same real report registry/table is borrowed, never allocated a second time.
    pub(crate) fn register_design_day_predefined_reports(
        &self,
        reports: &mut CtfInitializationReports,
    ) -> Result<(), CtfReportUnavailable> {
        if i32::try_from(self.design_days.len()).ok() != Some(self.design_day_count) {
            return Err(CtfReportUnavailable::Association);
        }
        for getter_index in 0..self.design_days.len() {
            let stored_index = if self.reverse_design_days && getter_index < 2 {
                1 - getter_index
            } else {
                getter_index
            };
            self.design_days
                .get(stored_index)
                .ok_or(CtfReportUnavailable::Association)?
                .register_predefined_reports(reports)?;
        }
        Ok(())
    }
}
