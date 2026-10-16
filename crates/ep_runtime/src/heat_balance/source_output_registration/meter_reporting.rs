//! Genuine OutputProcessor1768?2000 meter reporting, same periods/frames/files.
use super::report_streams::{SourceOutputReportStreams, SourceReportStream};
use super::report_writers::{
    write_source_cumulative_meter, write_source_meter_period, write_source_timestamp,
};
use super::update_data::{SourceOutputUpdateOwners, source_i32};
use super::update_meters::{MAX_SET, MIN_SET};
use super::*;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
impl SourceOutputRegistration {
    pub(crate) fn report_source_time_step_meters(
        &mut self,
        start: f64,
        end: f64,
        print_eso: &mut bool,
        mut print_sql: bool,
        owners: &mut SourceOutputUpdateOwners<'_>,
        streams: &mut impl SourceOutputReportStreams,
    ) -> Result<(), Error> {
        let frequency = SourceReportFrequency::TimeStep;
        let f = frequency as usize;
        if !owners.results.meter_frames[f].enabled {
            owners.results.meter_frames[f].initialize(&self.meters.meters, frequency);
        }
        let mut print_timestamp = true;
        for meter in &mut self.meters.meters {
            let period = &meter.periods[f];
            meter.current_time_step_value = period.value;
            if !period.report && !period.accumulated_report {
                continue;
            }
            if print_timestamp {
                write_source_timestamp(
                    streams,
                    SourceReportStream::Mtr,
                    SourceReportFrequency::EachCall,
                    self.frequency_stamp_report_numbers[f],
                    print_timestamp && print_sql,
                    start,
                    end,
                    owners,
                )?;
                if owners.results.meter_frames[f].enabled {
                    owners.results.meter_frames[f].new_row(
                        *owners.environment.month,
                        *owners.environment.day_of_month,
                        owners.clock.hour_of_day,
                        source_i32(end)?,
                        owners.clock.calendar_year,
                    )?;
                }
                print_timestamp = false;
                print_sql = false;
            }
            if *print_eso && !period.report_file_only && !period.accumulated_file_only {
                write_source_timestamp(
                    streams,
                    SourceReportStream::Eso,
                    SourceReportFrequency::EachCall,
                    self.frequency_stamp_report_numbers[f],
                    print_timestamp && *print_eso && print_sql,
                    start,
                    end,
                    owners,
                )?;
                *print_eso = false;
            }
            if period.report {
                write_source_meter_period(period, frequency, owners, streams)?;
                owners.results.meter_frames[f].push_value(period.report_number, period.value);
            }
            if period.accumulated_report {
                write_source_cumulative_meter(
                    period.accumulated_report_number,
                    period.value,
                    period.accumulated_file_only,
                    owners,
                    streams,
                )?;
                owners.results.meter_frames[f]
                    .push_value(period.accumulated_report_number, period.value);
            }
        }
        for meter in &mut self.meters.meters {
            meter.periods[f].value = 0.0;
        }
        Ok(())
    }
    pub(crate) fn report_source_meters(
        &mut self,
        frequency: SourceReportFrequency,
        mut print_sql: bool,
        owners: &mut SourceOutputUpdateOwners<'_>,
        streams: &mut impl SourceOutputReportStreams,
    ) -> Result<(), Error> {
        if !matches!(
            frequency,
            SourceReportFrequency::Hour
                | SourceReportFrequency::Day
                | SourceReportFrequency::Month
                | SourceReportFrequency::Year
                | SourceReportFrequency::Simulation
        ) {
            return Err(Error::Unavailable(
                "actual ReportMeters frequency assertion",
            ));
        }
        let f = frequency as usize;
        if !owners.results.meter_frames[f].enabled {
            owners.results.meter_frames[f].initialize(&self.meters.meters, frequency);
        }
        let mut print_timestamp = true;
        for meter in &mut self.meters.meters {
            if frequency == SourceReportFrequency::Simulation {
                let period = &meter.periods[f];
                meter.period_last_simulation.value = period.value;
                meter.period_last_simulation.minimum = period.minimum;
                meter.period_last_simulation.minimum_date = period.minimum_date;
                meter.period_last_simulation.maximum = period.maximum;
                meter.period_last_simulation.maximum_date = period.maximum_date;
            }
            let period = &meter.periods[f];
            if !period.report && !period.accumulated_report {
                continue;
            }
            if print_timestamp {
                write_source_timestamp(
                    streams,
                    SourceReportStream::Mtr,
                    frequency,
                    self.frequency_stamp_report_numbers[f],
                    print_timestamp && print_sql,
                    -1.0,
                    -1.0,
                    owners,
                )?;
                if owners.results.meter_frames[f].enabled {
                    owners.results.meter_frames[f].new_row(
                        *owners.environment.month,
                        *owners.environment.day_of_month,
                        owners.clock.hour_of_day,
                        0,
                        owners.clock.calendar_year,
                    )?;
                }
                print_timestamp = false;
                print_sql = false;
            }
            if period.report {
                write_source_meter_period(period, frequency, owners, streams)?;
                owners.results.meter_frames[f].push_value(period.report_number, period.value);
                let period = &mut meter.periods[f];
                period.value = 0.0;
                if frequency != SourceReportFrequency::Hour {
                    period.minimum = MIN_SET;
                    period.maximum = MAX_SET;
                }
            }
            let period = &meter.periods[f];
            if period.accumulated_report {
                // Read this same Simulation cell after any preceding normal report reset.
                let value = meter.periods[SourceReportFrequency::Simulation as usize].value;
                write_source_cumulative_meter(
                    period.accumulated_report_number,
                    value,
                    period.accumulated_file_only,
                    owners,
                    streams,
                )?;
                owners.results.meter_frames[f].push_value(period.accumulated_report_number, value);
            }
        }
        Ok(())
    }
}
