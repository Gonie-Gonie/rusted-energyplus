use super::*;
use crate::psychrometrics::production_trace::{
    output_step, register_environment_axis, register_time_axis, system_call, zone_step,
};
use crate::time_axis::{build_environment_time_axes, build_hourly_time_axis};
use ep_model::{NormalizedName, RunPeriod, RunPeriodId, TypedModel};

fn model() -> TypedModel {
    let mut model = TypedModel::default();
    model.timestep.number_of_timesteps_per_hour = 4;
    model.run_periods.push(RunPeriod {
        id: RunPeriodId(0),
        name: NormalizedName::new("Actual quarter-hour loop"),
        begin_month: 1,
        begin_day_of_month: 1,
        begin_year: Some(2013),
        end_month: 1,
        end_day_of_month: 1,
        end_year: Some(2013),
        day_of_week_for_start_day: None,
        first_hour_interpolation_starting_values: Default::default(),
        use_weather_file_holidays_and_special_days: true,
        use_weather_file_daylight_saving_period: true,
        apply_weekend_holiday_rule: false,
        use_weather_file_rain_indicators: true,
        use_weather_file_snow_indicators: true,
        treat_weather_as_actual: false,
    });
    model
}

#[test]
fn disabled_observer_leaves_result_and_storage_unchanged() -> Result<(), Box<dyn std::error::Error>>
{
    let hourly = build_hourly_time_axis(&model())?;
    let (result, trace) = capture(false, || {
        register_time_axis(&hourly);
        let _scope = zone_step(0, 1, 4, 900.0);
        -0.0_f64
    });
    assert_eq!(result.to_bits(), (-0.0_f64).to_bits());
    assert!(trace.is_none());
    assert!(ACTIVE_CLOCK.with(|active| active.borrow().is_none()));
    Ok(())
}

#[test]
fn physical_hooks_capture_all_zero_kernel_steps_and_keep_validation_separate()
-> Result<(), Box<dyn std::error::Error>> {
    let model = model();
    let hourly = build_hourly_time_axis(&model)?;
    let environment = build_environment_time_axes(&model)?.remove(0);
    let (_, trace) = capture(true, || {
        register_time_axis(&hourly);
        register_environment_axis(&environment);
        for hour in 0..24 {
            for step in 1..=4 {
                let _zone = zone_step(hour, step, 4, 900.0);
                let _system =
                    system_call(hour * 4 + step as usize - 1, hour == 0 && step == 1, 900.0);
            }
        }
        for step in 0..96 {
            let _validation = output_step(step, 4, 900.0);
        }
    });
    let trace = trace.expect("enabled clock capture");
    assert_eq!(trace.total_invocation_count, 96);
    assert_eq!(trace.zone_invocations.len(), 96);
    assert_eq!(trace.prepared_hourly_frames.len(), 24);
    assert_eq!(trace.materialized_environment_index, Some(1));
    for (index, event) in trace.zone_invocations.iter().enumerate() {
        assert_eq!(event.sequence, index as u64 + 1);
        assert_eq!(event.calendar_frame_index, Some(index / 4));
        assert_eq!(event.environment_point_index, Some(index));
        assert_eq!(event.timestep_seconds_bits, 900.0_f64.to_bits());
    }
    assert_eq!(trace.prepared_hourly_frames[0].day_of_week, 3);
    assert!(trace.prepared_environment_points[0].begin_environment);
    assert!(trace.prepared_environment_points[95].end_environment);
    assert!(ACTIVE_CLOCK.with(|active| active.borrow().is_none()));
    Ok(())
}

#[test]
fn bounded_nested_observers_preserve_outer_order_and_report_actual_suffix() {
    let (_, outer) = capture_with_limit(2, || {
        let _first = zone_step(0, 1, 4, 900.0);
        let (_, inner) = capture(true, || {
            let _nested = zone_step(10, 2, 4, 900.0);
        });
        assert_eq!(inner.expect("inner").zone_invocations[0].hour_index, 10);
        for step in 2..=4 {
            let _scope = zone_step(0, step, 4, 900.0);
        }
    });
    let trace = outer.expect("outer");
    assert_eq!(trace.total_invocation_count, 4);
    assert_eq!(trace.zone_invocations.len(), 2);
    assert_eq!(trace.zone_invocations[0].zone_timestep, 1);
    assert_eq!(trace.zone_invocations[1].zone_timestep, 2);
    assert_eq!(trace.zone_invocations[1].calendar_frame_index, None);
    assert_eq!(trace.zone_invocations[1].environment_point_index, None);
}

#[test]
fn unwinding_restores_outer_capture_without_leaking_interrupted_invocations() {
    let (_, outer) = capture(true, || {
        let unwind = std::panic::catch_unwind(|| {
            capture(true, || {
                let _interrupted = zone_step(10, 3, 4, 900.0);
                std::panic::resume_unwind(Box::new("deliberate clock observer unwind"));
            });
        });
        assert!(unwind.is_err());
        let _continued = zone_step(0, 1, 4, 900.0);
    });
    let trace = outer.expect("outer restored");
    assert_eq!(trace.total_invocation_count, 1);
    assert_eq!(trace.zone_invocations[0].hour_index, 0);
    assert!(ACTIVE_CLOCK.with(|active| active.borrow().is_none()));
}
