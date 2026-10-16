//! Actual GetNextEnvironment1178–1203 DesignDay environment report continuation.
//! The caller has already borrowed the genuine sole DesDayInput row at1178.
use super::Failure;
use super::run_period::{DAY_NAMES, SKY_NAMES, date};
use crate::heat_balance::radiation::ordinary::source_children::SourceSolarReportFlags;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::weather::day::source_design_day::SourceDesignDayData;
use crate::weather::day::{SourceEnvironmentData, WeatherEnvironmentState};
use std::io::Write;

#[expect(
    clippy::write_literal,
    reason = "Preserve Native EnvNameFormat/EnvDST formats and literal operands in Source order"
)]
pub(super) fn initialize_source_design_day_environment(
    environment_row: &SourceEnvironmentData,
    environment: &mut WeatherEnvironmentState,
    design_day: &SourceDesignDayData,
    reports: &SourceSolarReportFlags,
    eio: &mut dyn Write,
) -> Result<(), Failure> {
    environment.run_period_environment = false;
    let start_date = date(design_day.month, design_day.day_of_month);
    let end_date = start_date.clone();
    if reports.do_weather_init_reporting {
        let day_type = DAY_NAMES
            .get(
                usize::try_from(design_day.day_type)
                    .map_err(|_| Error::Unavailable("actual DesignDay dayTypeNames index1189"))?,
            )
            .ok_or(Error::Unavailable(
                "actual DesignDay dayTypeNames index1189",
            ))?;
        let sky = SKY_NAMES
            .get(
                usize::try_from(environment_row.sky_temperature_model).map_err(|_| {
                    Error::Unavailable("actual DesignDay SkyTempModelNames index1197")
                })?,
            )
            .ok_or(Error::Unavailable(
                "actual DesignDay SkyTempModelNames index1197",
            ))?;
        writeln!(
            eio,
            "Environment,{},{},{},{},{},{},{},{},{},{},{},{},{}",
            environment_row.title,
            "SizingPeriod:DesignDay",
            start_date,
            end_date,
            day_type,
            "1",
            "N/A",
            "N/A",
            "N/A",
            "N/A",
            "N/A",
            "N/A",
            sky
        )
        .map_err(Error::Io)?;
    }
    if design_day.daylight_saving_indicator == 0 && reports.do_weather_init_reporting {
        writeln!(
            eio,
            "Environment:Daylight Saving,No,{}",
            "SizingPeriod:DesignDay"
        )
        .map_err(Error::Io)?;
    } else if reports.do_weather_init_reporting {
        writeln!(
            eio,
            "Environment:Daylight Saving,Yes,{},{},{}",
            "SizingPeriod:DesignDay", start_date, end_date
        )
        .map_err(Error::Io)?;
    }
    Ok(())
}
