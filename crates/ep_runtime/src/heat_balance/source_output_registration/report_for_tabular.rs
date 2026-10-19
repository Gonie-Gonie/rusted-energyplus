//! Actual OutputProcessor ReportForTabularReports2002-2125 on retained meters.
//! Uses the same final-year period and predefined table, without a report cache.
use super::SourceOutputRegistration;
use super::meters::SourceMeterIpUnits;
use crate::heat_balance::ctf_reports::PredefinedStringTable;
use crate::heat_balance::predefined_tables::PredefinedRegistry;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;

fn column(registry: &PredefinedRegistry, name: &str) -> Result<i32, Error> {
    registry
        .named_indices
        .get(name)
        .copied()
        .ok_or(Error::Unavailable("actual energy-meter predefined column"))
}

// Native DateToStringWithMonth2127-2215; General1004-1039 decodes by
// truncating integer division and Objexx mod, retaining the invalid-date guards.
fn date_to_string_with_month(coded_date: i32) -> String {
    if coded_date == 0 {
        return "-".into();
    }
    let month = coded_date / 1_000_000;
    let rest = coded_date - month * 1_000_000;
    let day = rest / 10_000;
    let rest = rest - day * 10_000;
    let mut hour = rest / 100;
    let mut minute = rest % 100;
    if !(1..=12).contains(&month) {
        return "-".into();
    }
    if !(1..=31).contains(&day) {
        return "-".into();
    }
    if !(1..=24).contains(&hour) {
        return "-".into();
    }
    if !(0..=60).contains(&minute) {
        return "-".into();
    }
    hour -= 1;
    if minute == 60 {
        hour += 1;
        minute = 0;
    }
    let month_name = match month {
        1 => "JAN",
        2 => "FEB",
        3 => "MAR",
        4 => "APR",
        5 => "MAY",
        6 => "JUN",
        7 => "JUL",
        8 => "AUG",
        9 => "SEP",
        10 => "OCT",
        11 => "NOV",
        12 => "DEC",
        _ => return "-".into(),
    };
    format!("{day:02}-{month_name:3}-{hour:02}:{minute:02}")
}

impl SourceOutputRegistration {
    /// Push each actual final-year meter period to the same predefined table.
    /// The admitted meter constructors produce Electricity, Cooling, OtherJ and OtherKG.
    /// Existing real-entry helpers keep their explicit finite fixed-format
    /// domain; no scientific/nonfinite formatting or completion is invented.
    pub(crate) fn report_source_for_tabular_reports(
        &self,
        registry: &PredefinedRegistry,
        table: &mut PredefinedStringTable,
        time_step_zone_seconds: &f64,
    ) -> Result<(), Error> {
        for meter in &self.meters.meters {
            let period = &meter.period_final_year_simulation;
            // Static column identities perform no owner read ahead of the
            // corresponding PreDefTableEntry call below.
            let names = match meter
                .ip_units
                .ok_or(Error::Unavailable("actual initialized meter RT_forIPUnits"))?
            {
                SourceMeterIpUnits::Electricity => [
                    "pdchEMelecannual",
                    "pdchEMelecminvalue",
                    "pdchEMelecminvaluetime",
                    "pdchEMelecmaxvalue",
                    "pdchEMelecmaxvaluetime",
                ],
                SourceMeterIpUnits::Cooling => [
                    "pdchEMcoolannual",
                    "pdchEMcoolminvalue",
                    "pdchEMcoolminvaluetime",
                    "pdchEMcoolmaxvalue",
                    "pdchEMcoolmaxvaluetime",
                ],
                SourceMeterIpUnits::OtherKG => [
                    "pdchEMotherKGannual",
                    "pdchEMotherKGminvalue",
                    "pdchEMotherKGminvaluetime",
                    "pdchEMotherKGmaxvalue",
                    "pdchEMotherKGmaxvaluetime",
                ],
                SourceMeterIpUnits::OtherJ => [
                    "pdchEMotherJannual",
                    "pdchEMotherJminvalue",
                    "pdchEMotherJminvaluetime",
                    "pdchEMotherJmaxvalue",
                    "pdchEMotherJmaxvaluetime",
                ],
            };
            let annual = match meter.ip_units {
                Some(SourceMeterIpUnits::OtherKG) => period.value,
                Some(SourceMeterIpUnits::Electricity | SourceMeterIpUnits::Cooling | SourceMeterIpUnits::OtherJ) => {
                    period.value * 1.0E-9
                }
                None => return Err(Error::Unavailable("actual initialized meter RT_forIPUnits")),
            };
            table.fixed_real_entry_2(column(registry, names[0])?, &meter.name, annual)?;
            if meter.ip_units == Some(SourceMeterIpUnits::OtherKG) {
                table.fixed_real_entry_3(
                    column(registry, names[1])?,
                    &meter.name,
                    period.minimum / *time_step_zone_seconds,
                )?;
            } else {
                table.fixed_real_entry_2(
                    column(registry, names[1])?,
                    &meter.name,
                    period.minimum / *time_step_zone_seconds,
                )?;
            }
            table.string_entry(
                column(registry, names[2])?,
                &meter.name,
                &date_to_string_with_month(period.minimum_date),
            )?;
            if meter.ip_units == Some(SourceMeterIpUnits::OtherKG) {
                table.fixed_real_entry_3(
                    column(registry, names[3])?,
                    &meter.name,
                    period.maximum / *time_step_zone_seconds,
                )?;
            } else {
                table.fixed_real_entry_2(
                    column(registry, names[3])?,
                    &meter.name,
                    period.maximum / *time_step_zone_seconds,
                )?;
            }
            table.string_entry(
                column(registry, names[4])?,
                &meter.name,
                &date_to_string_with_month(period.maximum_date),
            )?;
        }
        Ok(())
    }
}
