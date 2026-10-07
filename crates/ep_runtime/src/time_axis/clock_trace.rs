//! Optional copies of prepared calendars and actual zone-loop invocations.
//!
//! This collector never advances a clock or supplies calculation operands. It
//! observes one thread and does not model EnergyPlus global environment flags.

use super::{EnvironmentTimeAxis, TimeAxis, TimePoint, energyplus_weekday_number};
use crate::psychrometrics::production_trace::EnvironmentObservation;
use std::cell::RefCell;

const INVOCATION_LIMIT: usize = 100_000;

/// Direct unit forwarding to the existing Gregorian predicate; no new policy.
#[must_use]
pub fn existing_is_leap_year(year: u32) -> bool {
    super::is_leap_year(year)
}

/// Direct unit forwarding to the existing ordinal helper using its year shape.
/// Invalid calendar dates remain `None`; no source error sentinel is invented.
#[must_use]
pub fn existing_day_of_year(month: u32, day: u32, leap_year: bool) -> Option<u32> {
    super::day_of_year(
        if leap_year {
            super::DEFAULT_LEAP_RUN_PERIOD_YEAR
        } else {
            super::DEFAULT_RUN_PERIOD_YEAR
        },
        month,
        day,
    )
}

thread_local! {
    static ACTIVE_CLOCK: RefCell<Option<ClockTrace>> = RefCell::default();
}

/// Calendar values copied from an existing hourly point before reporting.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CalendarFrame {
    /// Existing hourly index.
    pub hourly_sample_index: usize,
    /// Existing day count within the selected run period.
    pub day_of_sim: usize,
    /// Civil calendar year, separate from the EPW record year.
    pub year: u32,
    /// Existing month.
    pub month: u32,
    /// Existing day of month.
    pub day_of_month: u32,
    /// Existing Gregorian ordinal.
    pub gregorian_day_of_year: u32,
    /// Existing weather-effective ordinal.
    pub weather_day_of_year: u32,
    /// Existing always-leap schedule ordinal.
    pub schedule_day_of_year: u32,
    /// Gregorian weekday, using the existing Sunday=1 mapping.
    pub gregorian_day_of_week: i32,
    /// Simulation weekday, using the existing Sunday=1 mapping.
    pub day_of_week: i32,
    /// Existing schedule day-type index.
    pub day_type: u32,
    /// Existing schedule day-type label.
    pub day_type_label: &'static str,
    /// Existing Gregorian leap-year state.
    pub gregorian_year_is_leap_year: bool,
    /// Existing weather-effective leap-year state.
    pub weather_effective_year_is_leap_year: bool,
    /// Existing weather leap-year addition.
    pub leap_year_add: u32,
    /// Existing effective daylight-saving state.
    pub dst: bool,
    /// Existing special-day override, when present.
    pub special_day_type: Option<u32>,
    /// Existing hour ending.
    pub hour_ending: u32,
}

impl From<&TimePoint> for CalendarFrame {
    fn from(point: &TimePoint) -> Self {
        Self {
            hourly_sample_index: point.sample_index,
            day_of_sim: point.day_of_sim,
            year: point.year,
            month: point.month,
            day_of_month: point.day_of_month,
            gregorian_day_of_year: point.gregorian_day_of_year,
            weather_day_of_year: point.day_of_year,
            schedule_day_of_year: point.schedule_day_of_year,
            gregorian_day_of_week: energyplus_weekday_number(point.gregorian_day_of_week),
            day_of_week: energyplus_weekday_number(point.day_of_week),
            day_type: point.day_type.energyplus_index(),
            day_type_label: point.day_type.label(),
            gregorian_year_is_leap_year: point.gregorian_year_is_leap_year,
            weather_effective_year_is_leap_year: point.weather_effective_year_is_leap_year,
            leap_year_add: point.leap_year_add,
            dst: point.dst,
            special_day_type: point.special_day_type.map(|kind| kind.energyplus_index()),
            hour_ending: point.hour,
        }
    }
}

/// One actual invocation of the existing physical zone-loop hook.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ZoneClockInvocation {
    /// One-based order on the collecting execution thread.
    pub sequence: u64,
    /// Actual hourly loop operand.
    pub hour_index: usize,
    /// Actual one-based zone substep operand.
    pub zone_timestep: u32,
    /// Actual loop subdivision operand.
    pub zone_steps_per_hour: u32,
    /// Actual duration operand, with no decimal rounding.
    pub timestep_seconds_bits: u64,
    /// Existing hourly frame referenced by this invocation, if registered.
    pub calendar_frame_index: Option<usize>,
    /// Existing B environment point referenced by this invocation, if registered.
    pub environment_point_index: Option<usize>,
}

/// Prepared calendar copies and a bounded ordered prefix of real invocations.
#[derive(Clone, Debug)]
pub struct ClockTrace {
    /// Existing selected run-period name.
    pub run_period_name: Option<String>,
    /// Existing prepared hourly calendar values; these are not executed events.
    pub prepared_hourly_frames: Vec<CalendarFrame>,
    /// Existing B materialized environment index; A does not create this axis.
    pub materialized_environment_index: Option<usize>,
    /// Existing B environment fields; these are not EnergyPlus global flags.
    pub prepared_environment_points: Vec<EnvironmentObservation>,
    /// Actual parsed-input design-day declarations, separate from EP native state.
    pub parsed_input_design_day_declaration_count: Option<usize>,
    /// Maximum stored invocation observations.
    pub invocation_limit: usize,
    /// All actual observed zone hook invocations, including any omitted suffix.
    pub total_invocation_count: u64,
    /// Actual observations retained in invocation order.
    pub zone_invocations: Vec<ZoneClockInvocation>,
}

struct CaptureGuard {
    previous: Option<ClockTrace>,
}

impl Drop for CaptureGuard {
    fn drop(&mut self) {
        ACTIVE_CLOCK.with(|active| *active.borrow_mut() = self.previous.take());
    }
}

/// Runs unchanged. Disabled captures create no calendar copies or event storage.
/// Nested captures restore the prior observer even if the operation panics.
pub fn capture<R>(enabled: bool, execute: impl FnOnce() -> R) -> (R, Option<ClockTrace>) {
    if !enabled {
        return (execute(), None);
    }
    capture_with_limit(INVOCATION_LIMIT, execute)
}

fn capture_with_limit<R>(limit: usize, execute: impl FnOnce() -> R) -> (R, Option<ClockTrace>) {
    let previous = ACTIVE_CLOCK.with(|active| {
        active.borrow_mut().replace(ClockTrace {
            run_period_name: None,
            prepared_hourly_frames: Vec::new(),
            materialized_environment_index: None,
            prepared_environment_points: Vec::new(),
            parsed_input_design_day_declaration_count: None,
            invocation_limit: limit,
            total_invocation_count: 0,
            zone_invocations: Vec::new(),
        })
    });
    let _guard = CaptureGuard { previous };
    let result = execute();
    let trace = ACTIVE_CLOCK.with(|active| active.borrow_mut().take());
    (result, trace)
}

/// Copies the axis actually prepared for the runtime only when enabled.
pub fn register_time_axis(axis: &TimeAxis) {
    ACTIVE_CLOCK.with(|active| {
        if let Some(trace) = active.borrow_mut().as_mut() {
            trace.run_period_name = Some(axis.run_period_name.clone());
            trace.prepared_hourly_frames = axis.points.iter().map(CalendarFrame::from).collect();
        }
    });
}

/// Copies an already-materialized B axis without inventing an A environment.
pub fn register_environment_axis(axis: &EnvironmentTimeAxis) {
    ACTIVE_CLOCK.with(|active| {
        if let Some(trace) = active.borrow_mut().as_mut() {
            trace.materialized_environment_index = Some(axis.environment_index);
            trace.prepared_environment_points = axis
                .points
                .iter()
                .map(|point| EnvironmentObservation {
                    environment_index: point.environment_index,
                    sample_index: point.sample_index,
                    simulation_timestep: point.simulation_timestep,
                    zone_timestep: point.zone_timestep,
                    start_minute_bits: point.start_minute.to_bits(),
                    end_minute_bits: point.end_minute.to_bits(),
                    current_time_hours_bits: point.current_time_hours.to_bits(),
                    begin_environment: point.begin_environment,
                    end_environment: point.end_environment,
                    begin_day: point.begin_day,
                    end_day: point.end_day,
                    begin_hour: point.begin_hour,
                    end_hour: point.end_hour,
                })
                .collect();
        }
    });
}

/// Observes a real parsed object count; it is not a native EP environment number.
pub fn register_design_day_declaration_count(count: usize) {
    ACTIVE_CLOCK.with(|active| {
        if let Some(trace) = active.borrow_mut().as_mut() {
            trace.parsed_input_design_day_declaration_count = Some(count);
        }
    });
}

/// Observes only the existing physical zone hook, independent of kernel calls.
pub(crate) fn record_zone_step(hour_index: usize, substep: u32, steps: u32, seconds: f64) {
    ACTIVE_CLOCK.with(|active| {
        let mut active = active.borrow_mut();
        let Some(trace) = active.as_mut() else {
            return;
        };
        trace.total_invocation_count += 1;
        if trace.zone_invocations.len() == trace.invocation_limit {
            return;
        }
        let sample_index = hour_index * steps as usize + substep.saturating_sub(1) as usize;
        trace.zone_invocations.push(ZoneClockInvocation {
            sequence: trace.total_invocation_count,
            hour_index,
            zone_timestep: substep,
            zone_steps_per_hour: steps,
            timestep_seconds_bits: seconds.to_bits(),
            calendar_frame_index: trace
                .prepared_hourly_frames
                .get(hour_index)
                .map(|_| hour_index),
            environment_point_index: trace
                .prepared_environment_points
                .get(sample_index)
                .map(|_| sample_index),
        });
    });
}

#[cfg(test)]
mod tests;
