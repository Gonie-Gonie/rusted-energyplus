//! Actual timestamp/numeric/OutVar/MeterPeriod writers, preserving stream/count order.
use super::bound_real::SourceOutputVariableType;
use super::meters::SourceMeterPeriod;
use super::milo_dtoa::source_dtoa;
use super::report_streams::{SourceOutputReportStreams, SourceReportStream};
use super::update_data::{SourceOutputUpdateOwners, increment_record};
use super::update_meters::{MAX_SET, MIN_SET};
use super::*;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
fn day_type(owners: &SourceOutputUpdateOwners<'_>) -> Result<&'static str, Error> {
    const NAMES: [&str; 13] = [
        "Unused",
        "Sunday",
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
        "Holiday",
        "SummerDesignDay",
        "WinterDesignDay",
        "CustomDay1",
        "CustomDay2",
    ];
    let current = if *owners.environment.holiday_index > 0 {
        *owners.environment.holiday_index
    } else {
        *owners.environment.day_of_week
    };
    let index = usize::try_from(current)
        .map_err(|_| Error::Unavailable("actual Sched dayTypeNames index"))?;
    NAMES
        .get(index)
        .copied()
        .ok_or(Error::Unavailable("actual Sched dayTypeNames index"))
}
fn sqlite_boundary(streams: &impl SourceOutputReportStreams) -> Result<(), Error> {
    if streams.sqlite_present() {
        return Err(Error::Unavailable("actual OutputProcessor SQLite writer"));
    }
    Ok(())
}
pub(super) fn write_source_timestamp(
    streams: &mut impl SourceOutputReportStreams,
    stream: SourceReportStream,
    frequency: SourceReportFrequency,
    id: i32,
    write_sql: bool,
    start: f64,
    end: f64,
    owners: &SourceOutputUpdateOwners<'_>,
) -> Result<(), Error> {
    if frequency == SourceReportFrequency::Year {
        // Native WriteYearlyTimeStamp has no stream.good guard before print.
        stream.write(
            streams,
            format!("{},{}\n", id, owners.clock.calendar_year_chr).as_bytes(),
        )?;
        if write_sql {
            sqlite_boundary(streams)?;
        }
        return Ok(());
    }
    let report = if id == -1 {
        String::new()
    } else {
        id.to_string()
    };
    if !stream.good(streams) {
        return Ok(());
    }
    let e = &owners.environment;
    let c = &owners.clock;
    let line = match frequency {
        SourceReportFrequency::EachCall | SourceReportFrequency::TimeStep => format!(
            "{},{},{:2},{:2},{:2},{:2},{:5.2},{:5.2},{}\n",
            report,
            c.day_of_sim_chr,
            e.month,
            e.day_of_month,
            e.dst_indicator,
            c.hour_of_day,
            start,
            end,
            day_type(owners)?
        ),
        SourceReportFrequency::Hour => format!(
            "{},{},{:2},{:2},{:2},{:2},{:5.2},{:5.2},{}\n",
            report,
            c.day_of_sim_chr,
            e.month,
            e.day_of_month,
            e.dst_indicator,
            c.hour_of_day,
            0.0,
            60.0,
            day_type(owners)?
        ),
        SourceReportFrequency::Day => format!(
            "{},{},{:2},{:2},{:2},{}\n",
            report,
            c.day_of_sim_chr,
            e.month,
            e.day_of_month,
            e.dst_indicator,
            day_type(owners)?
        ),
        SourceReportFrequency::Month => format!("{},{},{:2}\n", report, c.day_of_sim_chr, e.month),
        SourceReportFrequency::Simulation => format!("{},{}\n", report, c.day_of_sim_chr),
        SourceReportFrequency::Year => {
            return Err(Error::Unavailable(
                "actual yearly timestamp dispatch invariant",
            ));
        }
    };
    stream.write(streams, line.as_bytes())?;
    if write_sql {
        sqlite_boundary(streams)?;
    }
    Ok(())
}
fn warmup_suppressed(owners: &SourceOutputUpdateOwners<'_>) -> bool {
    owners
        .project
        .system
        .update_data_during_warmup_external_interface
        && !owners.project.system.report_during_warmup
}
pub(super) fn write_source_numeric(
    streams: &mut impl SourceOutputReportStreams,
    id: i32,
    value: f64,
    owners: &SourceOutputUpdateOwners<'_>,
) -> Result<(), Error> {
    if warmup_suppressed(owners) {
        return Ok(());
    }
    sqlite_boundary(streams)?;
    if streams.eso_good() {
        let text = source_dtoa(value)?;
        streams.write_eso(format!("{id},{text}\n").as_bytes())?;
    }
    Ok(())
}
fn date_string(date: i32, frequency: SourceReportFrequency) -> String {
    // Native General::DecodeMonDayHrMin uses truncating integer division/mod.
    let month = date / 1000000;
    let rest = date - month * 1000000;
    let day = rest / 10000;
    let rest = rest - day * 10000;
    let hour = rest / 100;
    let minute = rest % 100;
    match frequency {
        SourceReportFrequency::Day => format!("{hour:2},{minute:2}"),
        SourceReportFrequency::Month => format!("{day:2},{hour:2},{minute:2}"),
        SourceReportFrequency::Year | SourceReportFrequency::Simulation => {
            format!("{month:2},{day:2},{hour:2},{minute:2}")
        }
        _ => String::new(),
    }
}
fn write_variable_report(
    var: &SourceOutVarReal,
    owners: &SourceOutputUpdateOwners<'_>,
    streams: &mut impl SourceOutputReportStreams,
) -> Result<(), Error> {
    let value = if var.store_type == SourceOutputStoreType::Average {
        var.store_value / var.num_stored
    } else {
        var.store_value
    };
    if owners.results.time_series_enabled
        && matches!(
            var.frequency,
            SourceReportFrequency::Day
                | SourceReportFrequency::Month
                | SourceReportFrequency::Year
                | SourceReportFrequency::Simulation
        )
    {
        return Err(Error::Unavailable(
            "actual RF variable push before OutVar report data",
        ));
    }
    sqlite_boundary(streams)?;
    if streams.eso_good() {
        let text = if var.variable_type == SourceOutputVariableType::Real {
            source_dtoa(value)?
        } else if value == 0.0 {
            String::from("0.0")
        } else {
            if !value.is_finite() {
                return Err(Error::Unavailable(
                    "actual nonfinite Integer fixed report formatting",
                ));
            }
            format!("{value:.6}")
        };
        let line = if matches!(
            var.frequency,
            SourceReportFrequency::EachCall
                | SourceReportFrequency::TimeStep
                | SourceReportFrequency::Hour
        ) {
            format!("{},{text}\n", var.report_id)
        } else {
            let min = source_dtoa(var.min_value)?;
            let max = source_dtoa(var.max_value)?;
            let min_date = date_string(var.min_value_date, var.frequency);
            let max_date = date_string(var.max_value_date, var.frequency);
            format!(
                "{},{text},{min},{min_date},{max},{max_date}\n",
                var.report_id
            )
        };
        streams.write_eso(line.as_bytes())?;
    }
    Ok(())
}
pub(super) fn write_source_variable_output(
    var: &mut SourceOutVarReal,
    frequency: SourceReportFrequency,
    owners: &mut SourceOutputUpdateOwners<'_>,
    streams: &mut impl SourceOutputReportStreams,
) -> Result<(), Error> {
    if warmup_suppressed(owners) {
        return Ok(());
    }
    if !var.report || var.frequency != frequency || !var.stored {
        return Ok(());
    }
    if var.num_stored > 0.0 {
        write_variable_report(var, owners, streams)?;
        increment_record(owners.standard_output_record_count)?;
    }
    var.store_value = 0.0;
    var.num_stored = 0.0;
    var.min_value = MIN_SET;
    var.max_value = MAX_SET;
    var.stored = false;
    Ok(())
}
pub(super) fn write_source_meter_period(
    period: &SourceMeterPeriod,
    frequency: SourceReportFrequency,
    owners: &mut SourceOutputUpdateOwners<'_>,
    streams: &mut impl SourceOutputReportStreams,
) -> Result<(), Error> {
    let text = if period.value == 0.0 {
        String::from("0.0")
    } else {
        source_dtoa(period.value)?
    };
    sqlite_boundary(streams)?;
    let line = if matches!(
        frequency,
        SourceReportFrequency::EachCall
            | SourceReportFrequency::TimeStep
            | SourceReportFrequency::Hour
    ) {
        format!("{},{text}\n", period.report_number)
    } else {
        let min = source_dtoa(period.minimum)?;
        let max = source_dtoa(period.maximum)?;
        let min_date = date_string(period.minimum_date, frequency);
        let max_date = date_string(period.maximum_date, frequency);
        format!(
            "{},{text},{min},{min_date},{max},{max_date}\n",
            period.report_number
        )
    };
    if streams.mtr_good() {
        streams.write_mtr(line.as_bytes())?;
    }
    increment_record(owners.standard_meter_record_count)?;
    if streams.eso_good() && !period.report_file_only {
        streams.write_eso(line.as_bytes())?;
        increment_record(owners.standard_output_record_count)?;
    }
    Ok(())
}
pub(super) fn write_source_cumulative_meter(
    id: i32,
    value: f64,
    meter_only: bool,
    owners: &mut SourceOutputUpdateOwners<'_>,
    streams: &mut impl SourceOutputReportStreams,
) -> Result<(), Error> {
    let text = if value == 0.0 {
        String::from("0.0")
    } else {
        source_dtoa(value)?
    };
    sqlite_boundary(streams)?;
    let line = format!("{id},{text}\n");
    if streams.mtr_good() {
        streams.write_mtr(line.as_bytes())?;
    }
    increment_record(owners.standard_meter_record_count)?;
    if !meter_only {
        if streams.eso_good() {
            streams.write_eso(line.as_bytes())?;
        }
        // Deliberately outside ESO.good: differs from MeterPeriod::WriteReportData.
        increment_record(owners.standard_output_record_count)?;
    }
    Ok(())
}
