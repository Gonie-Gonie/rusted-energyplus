//! Actual OutputProcessor3245?3744 System/Zone record keeping.
use super::bound_real::SourceOutputVariableType;
use super::report_streams::{SourceOutputReportStreams, SourceReportStream};
use super::*;
use crate::heat_balance::ordinary_output_control::SourceResultsFrameworkOptions;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::heat_balance::selected_project_data::SelectedProjectData;

/// Disjoint actual environment fields: leaves other weather Which operands
/// available for simultaneous immutable borrows. No whole-environment factory.
pub(crate) struct SourceOutputUpdateEnvironment<'a> {
    pub(crate) month: &'a i32,
    pub(crate) day_of_month: &'a i32,
    pub(crate) day_of_week: &'a i32,
    pub(crate) holiday_index: &'a i32,
    pub(crate) dst_indicator: &'a i32,
    pub(crate) end_month_flag: &'a mut bool,
    pub(crate) end_year_flag: &'a bool,
}
/// Reborrow the actual caller owners on this call; no target/value mirror.
pub(crate) struct SourceOutputUpdateOwners<'a> {
    pub(crate) clock: &'a mut WeatherGlobalState,
    pub(crate) environment: SourceOutputUpdateEnvironment<'a>,
    pub(crate) hvac: &'a SelectedHvacGlobals,
    pub(crate) project: &'a SelectedProjectData,
    pub(crate) results: &'a mut SourceResultsFrameworkOptions,
    pub(crate) standard_output_record_count: &'a mut i64,
    pub(crate) standard_meter_record_count: &'a mut i64,
    pub(crate) real: &'a dyn SourceRealOutputOwners,
    pub(crate) integers: &'a dyn SourceIntegerOutputOwners,
}
/// Defined-domain transport only; Native's conversion outside int range is undefined.
pub(super) fn source_i32(value: f64) -> Result<i32, Error> {
    if !value.is_finite()
        || value.trunc() < f64::from(i32::MIN)
        || value.trunc() > f64::from(i32::MAX)
    {
        return Err(Error::Unavailable(
            "actual OutputProcessor int conversion domain",
        ));
    }
    Ok(value.trunc() as i32)
}
pub(super) fn increment_record(value: &mut i64) -> Result<(), Error> {
    *value = value
        .checked_add(1)
        .ok_or(Error::Unavailable("actual Int64 output count domain"))?;
    Ok(())
}
fn encode_mdhm(month: i32, day: i32, hour: i32, minute: i32) -> Result<i32, Error> {
    month
        .checked_mul(100)
        .and_then(|n| n.checked_add(day))
        .and_then(|n| n.checked_mul(100))
        .and_then(|n| n.checked_add(hour))
        .and_then(|n| n.checked_mul(100))
        .and_then(|n| n.checked_add(minute))
        .ok_or(Error::Unavailable("actual EncodeMonDayHrMin int domain"))
}
fn no_variable_frame(options: &SourceResultsFrameworkOptions) -> Result<(), Error> {
    if options.time_series_enabled {
        return Err(Error::Unavailable(
            "actual ResultsFramework variable frame initialize/newRow/push child",
        ));
    }
    Ok(())
}
impl SourceOutputRegistration {
    fn current_minute(&self, kind: TimeStepKind) -> f64 {
        match kind {
            TimeStepKind::Zone => self.time_pointers.zone.current_minute,
            TimeStepKind::System => self.time_pointers.system.current_minute,
        }
    }
    fn print_variable_stamp(
        &mut self,
        end: f64,
        start: f64,
        time_print: &mut bool,
        owners: &SourceOutputUpdateOwners<'_>,
        streams: &mut impl SourceOutputReportStreams,
    ) -> Result<(), Error> {
        if *time_print {
            if self.last_hour_printed != owners.clock.hour_of_day
                || (self.last_start_minute - start).abs() > 0.001
                || (self.last_end_minute - end).abs() > 0.001
            {
                super::report_writers::write_source_timestamp(
                    streams,
                    SourceReportStream::Eso,
                    SourceReportFrequency::EachCall,
                    self.frequency_stamp_report_numbers[SourceReportFrequency::TimeStep as usize],
                    true,
                    start,
                    end,
                    owners,
                )?;
                self.last_hour_printed = owners.clock.hour_of_day;
                self.last_start_minute = start;
                self.last_end_minute = end;
            }
            *time_print = false;
        }
        Ok(())
    }
    pub(crate) fn update_source_data_and_report(
        &mut self,
        kind: TimeStepKind,
        owners: &mut SourceOutputUpdateOwners<'_>,
        streams: &mut impl SourceOutputReportStreams,
    ) -> Result<(), Error> {
        // Kind is the genuine Zone/System enum: invalid Native discriminator is unrepresentable.
        let start_minute = self.current_minute(kind);
        let time_step = match kind {
            TimeStepKind::Zone
                if self.time_pointers.zone.target == Some(TimeStepTarget::GlobalZoneTimeStep) =>
            {
                owners.clock.time_step_zone
            }
            TimeStepKind::System
                if self.time_pointers.system.target == Some(TimeStepTarget::HvacSystemTimeStep) =>
            {
                owners.hvac.time_step_sys
            }
            _ => {
                return Err(Error::Unavailable(
                    "actual OutputProcessor timestep pointer association",
                ));
            }
        };
        match kind {
            TimeStepKind::Zone => self.time_pointers.zone.current_minute += time_step * 60.0,
            TimeStepKind::System => self.time_pointers.system.current_minute += time_step * 60.0,
        }
        let end_time_step = kind == TimeStepKind::Zone
            || self.time_pointers.system.current_minute == self.time_pointers.zone.current_minute;
        let minute_now = self.current_minute(kind);
        let mdhm = encode_mdhm(
            *owners.environment.month,
            *owners.environment.day_of_month,
            owners.clock.hour_of_day,
            source_i32(minute_now)?,
        )?;
        let mut time_print = true;
        let rx_time = (minute_now - start_minute) / f64::from(owners.clock.minutes_in_time_step);
        no_variable_frame(owners.results)?; // first actual initializeTSDataFrame3295, before outVar writes
        for number in 0..self.out_vars.len() {
            if self.out_vars[number].time_step_type != kind {
                continue;
            }
            let var = &self.out_vars[number];
            let value = match var.variable_type {
                SourceOutputVariableType::Real => *owners
                    .real
                    .real_output(
                        var.which
                            .ok_or(Error::Unavailable("actual OutVarReal Which"))?,
                    )
                    .map_err(Error::Output)?,
                SourceOutputVariableType::Integer => f64::from(
                    *owners
                        .integers
                        .integer_output(
                            var.integer_which
                                .ok_or(Error::Unavailable("actual OutVarInt Which"))?,
                        )
                        .map_err(Error::Output)?,
                ),
            };
            let var = &mut self.out_vars[number];
            var.stored = true;
            if var.store_type == SourceOutputStoreType::Average {
                let current_value = value * rx_time;
                let extremum_value = if var.variable_type == SourceOutputVariableType::Real {
                    value
                } else {
                    current_value
                };
                if extremum_value > var.max_value {
                    var.max_value = extremum_value;
                    var.max_value_date = mdhm;
                }
                if extremum_value < var.min_value {
                    var.min_value = extremum_value;
                    var.min_value_date = mdhm;
                }
                var.time_step_value += current_value;
                var.external_interface_time_step_value = var.time_step_value;
            } else {
                if value > var.max_value {
                    var.max_value = value;
                    var.max_value_date = mdhm;
                }
                if value < var.min_value {
                    var.min_value = value;
                    var.min_value_date = mdhm;
                }
                var.time_step_value += value;
                var.external_interface_time_step_value = var.time_step_value;
            }
            if !var.report {
                continue;
            }
            if var.schedule.is_some() {
                return Err(Error::Unavailable(
                    "actual OutVar schedule getCurrentVal3360",
                ));
            }
            var.time_step_stored = true;
            if !var.this_time_step_stored {
                var.this_time_step_count = var
                    .this_time_step_count
                    .checked_add(1)
                    .ok_or(Error::Unavailable("actual OutVar thisTSCount domain"))?;
                var.this_time_step_stored = true;
            }
            if var.frequency != SourceReportFrequency::EachCall {
                continue;
            }
            let report_id = var.report_id;
            self.print_variable_stamp(minute_now, start_minute, &mut time_print, owners, streams)?;
            super::report_writers::write_source_numeric(streams, report_id, value, owners)?;
            increment_record(owners.standard_output_record_count)?;
        }
        if kind == TimeStepKind::System {
            return Ok(());
        }
        if end_time_step {
            no_variable_frame(owners.results)?;
            // Deliberately capacity, not length, matching Native3427.
            if self.meter_values.capacity() > 0 {
                for (number, meter) in self.meters.meters.iter().enumerate() {
                    if meter.meter_type != meters::SourceMeterType::Normal {
                        return Err(Error::Unavailable(
                            "actual custom/decrement meter source iteration",
                        ));
                    }
                    for &source_number in &meter.source_variable_numbers {
                        let var = self
                            .out_vars
                            .get(source_number)
                            .ok_or(Error::Unavailable("actual meter srcVarNums association"))?;
                        let target =
                            self.meter_values.get_mut(number).ok_or(Error::Unavailable(
                                "actual UpdateMeterReporting meterValues allocation",
                            ))?;
                        *target += var.time_step_value
                            * f64::from(var.zone_multiplier)
                            * f64::from(var.zone_list_multiplier);
                    }
                }
            }
            for number in 0..self.out_vars.len() {
                let var = &mut self.out_vars[number];
                if var.schedule.is_some() {
                    return Err(Error::Unavailable(
                        "actual OutVar schedule getCurrentVal3472",
                    ));
                }
                let report_now = true; // literal local; no schedule pointer admitted beyond preceding real boundary
                if !report_now || !var.report {
                    var.time_step_value = 0.0;
                }
                var.value += var.time_step_value;
                if !report_now || !var.report {
                    continue;
                }
                if var.frequency == SourceReportFrequency::TimeStep {
                    let report_id = var.report_id;
                    let value = var.time_step_value;
                    let kind = var.time_step_type;
                    let end = self.current_minute(kind);
                    self.print_variable_stamp(end, start_minute, &mut time_print, owners, streams)?;
                    super::report_writers::write_source_numeric(streams, report_id, value, owners)?;
                    increment_record(owners.standard_output_record_count)?;
                }
                self.out_vars[number].time_step_value = 0.0;
                self.out_vars[number].this_time_step_stored = false;
            }
            self.update_source_meters(mdhm, owners.clock.warmup_flag)?;
            let print_sql = time_print;
            self.report_source_time_step_meters(
                start_minute,
                self.time_pointers.zone.current_minute,
                &mut time_print,
                print_sql,
                owners,
                streams,
            )?;
        }
        if owners.clock.end_hour_flag {
            if self.frequency_tracking_variables[SourceReportFrequency::Hour as usize] {
                super::report_writers::write_source_timestamp(
                    streams,
                    SourceReportStream::Eso,
                    SourceReportFrequency::Hour,
                    self.frequency_stamp_report_numbers[SourceReportFrequency::TimeStep as usize],
                    true,
                    -1.0,
                    -1.0,
                    owners,
                )?;
                time_print = false;
            }
            no_variable_frame(owners.results)?;
            self.time_pointers.zone.current_minute = 0.0;
            self.time_pointers.system.current_minute = 0.0;
            for var in &mut self.out_vars {
                if var.time_step_stored {
                    if var.store_type == SourceOutputStoreType::Average {
                        var.value /= f64::from(var.this_time_step_count);
                    }
                    if var.report && var.frequency == SourceReportFrequency::Hour && var.stored {
                        super::report_writers::write_source_numeric(
                            streams,
                            var.report_id,
                            var.value,
                            owners,
                        )?;
                        increment_record(owners.standard_output_record_count)?;
                        var.stored = false;
                    }
                    var.store_value += var.value;
                    var.num_stored += 1.0;
                }
                var.time_step_stored = false;
                var.this_time_step_stored = false;
                var.this_time_step_count = 0;
                var.value = 0.0;
            }
            self.report_source_meters(SourceReportFrequency::Hour, time_print, owners, streams)?;
        }
        if !owners.clock.end_hour_flag {
            return Ok(());
        }
        if owners.clock.end_day_flag {
            self.print_period_variable_stamp(
                SourceReportFrequency::Day,
                &mut time_print,
                owners,
                streams,
            )?;
            no_variable_frame(owners.results)?;
            self.number_hours_in_month = self
                .number_hours_in_month
                .checked_add(24)
                .ok_or(Error::Unavailable("actual NumHoursInMonth domain"))?;
            for var in &mut self.out_vars {
                super::report_writers::write_source_variable_output(
                    var,
                    SourceReportFrequency::Day,
                    owners,
                    streams,
                )?;
            }
            self.report_source_meters(SourceReportFrequency::Day, time_print, owners, streams)?;
        }
        if !owners.clock.end_day_flag {
            return Ok(());
        }
        if *owners.environment.end_month_flag || owners.clock.end_envrn_flag {
            self.print_period_variable_stamp(
                SourceReportFrequency::Month,
                &mut time_print,
                owners,
                streams,
            )?;
            no_variable_frame(owners.results)?;
            self.number_hours_in_simulation = self
                .number_hours_in_simulation
                .checked_add(self.number_hours_in_month)
                .ok_or(Error::Unavailable("actual NumHoursInSim domain"))?;
            *owners.environment.end_month_flag = false;
            for var in &mut self.out_vars {
                super::report_writers::write_source_variable_output(
                    var,
                    SourceReportFrequency::Month,
                    owners,
                    streams,
                )?;
            }
            self.report_source_meters(SourceReportFrequency::Month, time_print, owners, streams)?;
            self.number_hours_in_month = 0;
        }
        if owners.clock.end_envrn_flag {
            self.print_period_variable_stamp(
                SourceReportFrequency::Simulation,
                &mut time_print,
                owners,
                streams,
            )?;
            no_variable_frame(owners.results)?;
            for var in &mut self.out_vars {
                super::report_writers::write_source_variable_output(
                    var,
                    SourceReportFrequency::Simulation,
                    owners,
                    streams,
                )?;
            }
            self.report_source_meters(
                SourceReportFrequency::Simulation,
                time_print,
                owners,
                streams,
            )?;
            self.number_hours_in_simulation = 0;
        }
        if *owners.environment.end_year_flag {
            self.print_period_variable_stamp(
                SourceReportFrequency::Year,
                &mut time_print,
                owners,
                streams,
            )?;
            no_variable_frame(owners.results)?;
            for var in &mut self.out_vars {
                super::report_writers::write_source_variable_output(
                    var,
                    SourceReportFrequency::Year,
                    owners,
                    streams,
                )?;
            }
            self.report_source_meters(SourceReportFrequency::Year, time_print, owners, streams)?;
            owners.clock.calendar_year = owners
                .clock
                .calendar_year
                .checked_add(1)
                .ok_or(Error::Unavailable("actual CalendarYear domain"))?;
            owners.clock.calendar_year_chr = owners.clock.calendar_year.to_string();
        }
        Ok(())
    }
    fn print_period_variable_stamp(
        &self,
        frequency: SourceReportFrequency,
        time_print: &mut bool,
        owners: &SourceOutputUpdateOwners<'_>,
        streams: &mut impl SourceOutputReportStreams,
    ) -> Result<(), Error> {
        if self.frequency_tracking_variables[frequency as usize] {
            super::report_writers::write_source_timestamp(
                streams,
                SourceReportStream::Eso,
                frequency,
                self.frequency_stamp_report_numbers[frequency as usize],
                true,
                -1.0,
                -1.0,
                owners,
            )?;
            *time_print = false;
        }
        Ok(())
    }
}
