//! Output-only bindings to existing runtime axes and real invocation scopes.

use super::{ACTIVE_TRACE, TraceBuffer};
use crate::time_axis::{EnvironmentTimeAxis, TimeAxis};
use std::marker::PhantomData;
use std::rc::Rc;

/// Calendar fields copied from an already-built runtime hourly time point.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CalendarObservation {
    /// Simulation calendar year, not the source EPW record year.
    pub year: u32,
    /// Simulation month.
    pub month: u32,
    /// Simulation day of month.
    pub day_of_month: u32,
    /// Actual one-based runtime day of simulation.
    pub day_of_sim: usize,
    /// Hour ending from the existing hourly axis.
    pub hour_ending: u32,
}

/// Fields copied from the existing materialized Rust environment time point.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct EnvironmentObservation {
    /// Materialized Rust environment index; not the full EnergyPlus Envrn ordinal.
    pub environment_index: usize,
    /// Actual zone point index within that environment.
    pub sample_index: usize,
    /// Actual one-based zone timestep count.
    pub simulation_timestep: usize,
    /// Actual environment-axis zone timestep within the hour.
    pub zone_timestep: u32,
    /// Actual start minute encoded without rounding.
    pub start_minute_bits: u64,
    /// Actual end minute encoded without rounding.
    pub end_minute_bits: u64,
    /// Actual fractional hour encoded without rounding.
    pub current_time_hours_bits: u64,
    /// Existing environment-start flag.
    pub begin_environment: bool,
    /// Existing environment-end flag.
    pub end_environment: bool,
    /// Existing day-start flag.
    pub begin_day: bool,
    /// Existing day-end flag.
    pub end_day: bool,
    /// Existing hour-start flag.
    pub begin_hour: bool,
    /// Existing hour-end flag.
    pub end_hour: bool,
}

/// Actual zone-loop operands or the output cursor being validated afterwards.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ZoneTimestepObservation {
    /// Zero-based hourly loop index.
    pub hour_index: usize,
    /// One-based substep within that hour.
    pub zone_timestep: u32,
    /// Actual number of zone steps used by the loop.
    pub zone_steps_per_hour: u32,
    /// Zero-based zone output index for this invocation or validation cursor.
    pub sample_index: usize,
    /// Actual timestep duration encoded without rounding.
    pub timestep_seconds_bits: u64,
    /// Calendar looked up from the already-built hourly runtime axis, if present.
    pub calendar: Option<CalendarObservation>,
    /// Existing B environment point, if that axis was materialized.
    pub environment: Option<EnvironmentObservation>,
}

/// Existing operands at the actual direct-Zone PurchasedAir coupling call.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SystemCallObservation {
    /// Actual schedule sample index passed into this call.
    pub schedule_sample_index: usize,
    /// Actual environment-start operand passed into this call.
    pub begin_environment: bool,
    /// Actual system duration operand; no adaptive-system ordinal is inferred.
    pub timestep_seconds_bits: u64,
}

/// Active actual Rust invocation or referenced-output context.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ExecutionContext {
    /// Distinguishes execution, actual system coupling and post-run validation.
    pub scope: &'static str,
    /// Existing zone invocation or referenced-output operands.
    pub zone_timestep: Option<ZoneTimestepObservation>,
    /// Existing operands of an actual system coupling call.
    pub system_call: Option<SystemCallObservation>,
}

/// Restores the thread-local context when the actual invocation scope ends.
/// This guard cannot move to a different thread.
#[must_use]
pub struct ExecutionContextGuard {
    previous: Option<Option<ExecutionContext>>,
    collecting_thread: PhantomData<Rc<()>>,
}

impl Drop for ExecutionContextGuard {
    fn drop(&mut self) {
        if let Some(previous) = self.previous {
            ACTIVE_TRACE.with(|active| {
                if let Some(trace) = active.borrow_mut().as_mut() {
                    trace.context = previous;
                }
            });
        }
    }
}

/// Registers an existing runtime axis only while collection is enabled.
/// No simulation date is reconstructed from weather records.
pub fn register_time_axis(axis: &TimeAxis) {
    ACTIVE_TRACE.with(|active| {
        if let Some(trace) = active.borrow_mut().as_mut() {
            trace.hourly_calendar = axis
                .points
                .iter()
                .map(|point| CalendarObservation {
                    year: point.year,
                    month: point.month,
                    day_of_month: point.day_of_month,
                    day_of_sim: point.day_of_sim,
                    hour_ending: point.hour,
                })
                .collect();
        }
    });
}

/// Observes the already-materialized first B environment axis, without adding
/// missing sizing/design environments or changing the schedule cache.
pub fn register_environment_axis(axis: &EnvironmentTimeAxis) {
    ACTIVE_TRACE.with(|active| {
        if let Some(trace) = active.borrow_mut().as_mut() {
            trace.environment_points = axis
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

fn bind(make_context: impl FnOnce(&TraceBuffer) -> ExecutionContext) -> ExecutionContextGuard {
    let previous = ACTIVE_TRACE.with(|active| {
        let mut active = active.borrow_mut();
        active.as_mut().map(|trace| {
            let context = make_context(trace);
            trace.context.replace(context)
        })
    });
    ExecutionContextGuard {
        previous,
        collecting_thread: PhantomData,
    }
}

fn zone_context(
    trace: &TraceBuffer,
    hour_index: usize,
    substep: u32,
    steps: u32,
    seconds: f64,
) -> ZoneTimestepObservation {
    let sample_index = hour_index * steps as usize + substep.saturating_sub(1) as usize;
    ZoneTimestepObservation {
        hour_index,
        zone_timestep: substep,
        zone_steps_per_hour: steps,
        sample_index,
        timestep_seconds_bits: seconds.to_bits(),
        calendar: trace.hourly_calendar.get(hour_index).copied(),
        environment: trace.environment_points.get(sample_index).copied(),
    }
}

/// Binds the real hourly/substep loop operands for this zone execution scope.
pub fn zone_step(
    hour_index: usize,
    substep: u32,
    steps: u32,
    seconds: f64,
) -> ExecutionContextGuard {
    bind(|trace| ExecutionContext {
        scope: "zone_timestep_execution",
        zone_timestep: Some(zone_context(trace, hour_index, substep, steps, seconds)),
        system_call: None,
    })
}

/// Binds the actual output-validation cursor after physical timesteps finished.
/// It references the output's zone interval, not the time of another execution.
pub fn output_step(sample_index: usize, steps: u32, seconds: f64) -> ExecutionContextGuard {
    bind(|trace| ExecutionContext {
        scope: "coupled_output_validation",
        zone_timestep: if steps == 0 {
            None
        } else {
            Some(zone_context(
                trace,
                sample_index / steps as usize,
                (sample_index % steps as usize) as u32 + 1,
                steps,
                seconds,
            ))
        },
        system_call: None,
    })
}

/// Binds existing arguments only for the actual system coupling invocation.
pub fn system_call(
    sample_index: usize,
    begin_environment: bool,
    seconds: f64,
) -> ExecutionContextGuard {
    bind(|trace| ExecutionContext {
        scope: "direct_zone_purchased_air_system_call",
        zone_timestep: trace.context.and_then(|context| context.zone_timestep),
        system_call: Some(SystemCallObservation {
            schedule_sample_index: sample_index,
            begin_environment,
            timestep_seconds_bits: seconds.to_bits(),
        }),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::psychrometrics::production_trace::{capture, record};

    #[test]
    fn actual_axis_lookup_system_scope_and_validation_scope_restore() {
        let calendar = CalendarObservation {
            year: 2024,
            month: 7,
            day_of_month: 15,
            day_of_sim: 1,
            hour_ending: 1,
        };
        let (_, trace) = capture(true, || {
            ACTIVE_TRACE.with(|active| {
                active
                    .borrow_mut()
                    .as_mut()
                    .expect("capture")
                    .hourly_calendar
                    .push(calendar);
            });
            {
                let _zone = zone_step(0, 2, 4, 900.0);
                {
                    let _system = system_call(1, false, 900.0);
                    record("system", &[1.0], 1.0);
                }
                record("zone", &[1.0], 1.0);
            }
            {
                let _validation = output_step(1, 4, 900.0);
                record("validation", &[1.0], 1.0);
            }
            record("outside", &[1.0], 1.0);
        });
        let trace = trace.expect("capture");
        let system = trace.calls[0].context.expect("system context");
        assert_eq!(system.scope, "direct_zone_purchased_air_system_call");
        assert_eq!(system.system_call.expect("system").schedule_sample_index, 1);
        let zone = system.zone_timestep.expect("zone");
        assert_eq!(zone.sample_index, 1);
        assert_eq!(zone.zone_timestep, 2);
        assert_eq!(zone.calendar, Some(calendar));
        assert_eq!(zone.environment, None);
        assert_eq!(
            trace.calls[1].context.expect("zone context").system_call,
            None
        );
        assert_eq!(
            trace.calls[2].context.expect("validation").zone_timestep,
            Some(zone)
        );
        assert_eq!(trace.calls[3].context, None);
    }
}
