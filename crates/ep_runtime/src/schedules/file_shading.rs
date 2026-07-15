//! Immutable runtime lookup for generated `Schedule:File:Shading` columns.

use super::{ScheduleSeriesKind, ScheduleTrace, ScheduleValueSeries};
use crate::time_axis::{EnvironmentTimeAxis, TimeAxis};
use ep_model::{ScheduleFileShading, ScheduleFileShadingColumn};

pub(super) fn file_shading_series_for_time_axis(
    schedule: &ScheduleFileShading,
    time_axis: &TimeAxis,
) -> Vec<ScheduleValueSeries> {
    let axis_timesteps_per_hour = time_axis.zone_timestep.timesteps_per_hour;
    schedule
        .columns
        .iter()
        .map(|column| {
            let values = time_axis
                .points
                .iter()
                .map(|point| {
                    file_shading_hourly_average(
                        schedule,
                        column,
                        axis_timesteps_per_hour,
                        point.schedule_day_of_year,
                        point.hour,
                    )
                    .unwrap_or(f64::NAN)
                })
                .collect();
            file_shading_trace(schedule, column, values)
        })
        .collect()
}

pub(super) fn file_shading_series_for_environment_time_axis(
    schedule: &ScheduleFileShading,
    time_axis: &EnvironmentTimeAxis,
) -> Vec<ScheduleValueSeries> {
    let axis_timesteps_per_hour = time_axis.zone_timestep.timesteps_per_hour;
    schedule
        .columns
        .iter()
        .map(|column| {
            file_shading_column_series(
                schedule,
                column,
                axis_timesteps_per_hour,
                time_axis
                    .points
                    .iter()
                    .map(|point| (point.schedule_day_of_year, point.hour, point.zone_timestep)),
            )
        })
        .collect()
}

fn file_shading_column_series(
    schedule: &ScheduleFileShading,
    column: &ScheduleFileShadingColumn,
    axis_timesteps_per_hour: u32,
    points: impl IntoIterator<Item = (u32, u32, u32)>,
) -> ScheduleValueSeries {
    let values = points
        .into_iter()
        .map(|(schedule_day_of_year, hour_ending, zone_timestep)| {
            file_shading_zone_timestep_value(
                schedule,
                column,
                axis_timesteps_per_hour,
                schedule_day_of_year,
                hour_ending,
                zone_timestep,
            )
            .unwrap_or(f64::NAN)
        })
        .collect();

    file_shading_trace(schedule, column, values)
}

fn file_shading_trace(
    schedule: &ScheduleFileShading,
    column: &ScheduleFileShadingColumn,
    values: Vec<f64>,
) -> ScheduleValueSeries {
    ScheduleTrace {
        schedule_id: column.id,
        schedule_name: column.schedule_name.0.clone(),
        kind: ScheduleSeriesKind::FileShadingZoneTimestep {
            source_day_count: schedule.source_day_count,
            timesteps_per_hour: schedule.timesteps_per_hour,
            source_value_count: column.values.len(),
        },
        values,
    }
}

fn file_shading_hourly_average(
    schedule: &ScheduleFileShading,
    column: &ScheduleFileShadingColumn,
    axis_timesteps_per_hour: u32,
    schedule_day_of_year: u32,
    hour_ending: u32,
) -> Option<f64> {
    let timesteps_per_hour = schedule.timesteps_per_hour;
    if timesteps_per_hour == 0 || axis_timesteps_per_hour != timesteps_per_hour {
        return None;
    }

    let sum = (1..=timesteps_per_hour).try_fold(0.0, |sum, zone_timestep| {
        file_shading_zone_timestep_value(
            schedule,
            column,
            axis_timesteps_per_hour,
            schedule_day_of_year,
            hour_ending,
            zone_timestep,
        )
        .map(|value| sum + value)
    })?;
    Some(sum / f64::from(timesteps_per_hour))
}

fn file_shading_zone_timestep_value(
    schedule: &ScheduleFileShading,
    column: &ScheduleFileShadingColumn,
    axis_timesteps_per_hour: u32,
    schedule_day_of_year: u32,
    hour_ending: u32,
    zone_timestep: u32,
) -> Option<f64> {
    let timesteps_per_hour = schedule.timesteps_per_hour;
    if timesteps_per_hour == 0 || axis_timesteps_per_hour != timesteps_per_hour {
        return None;
    }
    if !(1..=366).contains(&schedule_day_of_year)
        || !(1..=24).contains(&hour_ending)
        || !(1..=timesteps_per_hour).contains(&zone_timestep)
    {
        return None;
    }

    let source_day = match schedule.source_day_count {
        365 => match schedule_day_of_year {
            60 => 59,
            day if day > 60 => day - 1,
            day => day,
        },
        366 => schedule_day_of_year,
        _ => return None,
    };
    let source_index = (source_day - 1)
        .checked_mul(24)?
        .checked_add(hour_ending - 1)?
        .checked_mul(timesteps_per_hour)?
        .checked_add(zone_timestep - 1)?;
    column.values.get(source_index as usize).copied()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::RuntimeError;
    use crate::time_axis::{
        build_environment_time_axis_for_run_period_with_zone_timesteps, build_hourly_time_axis,
    };
    use crate::{ExecutionStep, RuntimeOutputRegistry, RuntimeOutputRequest, build_execution_plan};
    use ep_model::{
        DayOfWeek, FirstHourInterpolationStartingValues, InternalGainId, NormalizedName,
        OtherEquipment, OtherEquipmentDesignLevelCalculationMethod, RunPeriod, RunPeriodId,
        ScheduleConstant, ScheduleId, SimulationModel, TimestepConfig, TypedModel, ZoneId,
    };

    fn shading_schedule(
        source_day_count: u32,
        timesteps_per_hour: u32,
        id: u32,
    ) -> ScheduleFileShading {
        let value_count = source_day_count * 24 * timesteps_per_hour;
        ScheduleFileShading {
            file_name: "shading.csv".to_string(),
            timesteps_per_hour,
            source_day_count,
            columns: vec![ScheduleFileShadingColumn {
                id: ScheduleId(id),
                surface_header: "South Wall".to_string(),
                schedule_name: NormalizedName::new("South Wall_shading"),
                values: (0..value_count).map(f64::from).collect(),
            }],
        }
    }

    fn february_run_period() -> RunPeriod {
        RunPeriod {
            id: RunPeriodId(0),
            name: NormalizedName::new("February"),
            begin_month: 2,
            begin_day_of_month: 28,
            begin_year: Some(2016),
            end_month: 3,
            end_day_of_month: 1,
            end_year: Some(2016),
            day_of_week_for_start_day: Some(DayOfWeek::Sunday),
            first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues::Hour24,
            use_weather_file_holidays_and_special_days: false,
            use_weather_file_daylight_saving_period: false,
            apply_weekend_holiday_rule: false,
            use_weather_file_rain_indicators: false,
            use_weather_file_snow_indicators: false,
            treat_weather_as_actual: false,
        }
    }

    #[test]
    fn common_year_lookup_repeats_day_59_and_preserves_nonzero_generated_id() {
        let schedule = shading_schedule(365, 2, 37);
        let series = file_shading_column_series(
            &schedule,
            &schedule.columns[0],
            2,
            [(59, 24, 2), (60, 24, 2), (61, 1, 1)],
        );

        assert_eq!(series.schedule_id, ScheduleId(37));
        assert_eq!(series.schedule_name, "SOUTH WALL_SHADING");
        assert_eq!(series.values[0], series.values[1]);
        assert_eq!(series.values[0], f64::from(((58 * 24 + 23) * 2) + 1));
        assert_eq!(series.values[2], f64::from((59 * 24) * 2));
        assert!(matches!(
            series.kind,
            ScheduleSeriesKind::FileShadingZoneTimestep {
                source_day_count: 365,
                timesteps_per_hour: 2,
                source_value_count
            } if source_value_count == 365 * 24 * 2
        ));
    }

    #[test]
    fn public_precompute_emits_generated_columns_first_on_both_axes() {
        let hourly_model = TypedModel {
            timestep: TimestepConfig {
                number_of_timesteps_per_hour: 1,
            },
            file_shading_schedule: Some(shading_schedule(365, 1, 37)),
            schedules: vec![ScheduleConstant {
                id: ScheduleId(38),
                name: NormalizedName::new("Constant"),
                schedule_type_limits: None,
                hourly_value: 0.5,
            }],
            ..TypedModel::default()
        };
        let hourly_axis =
            build_hourly_time_axis(&hourly_model).expect("default hourly axis should build");
        let hourly_series = super::super::precompute_schedule_value_series_for_time_axis(
            &hourly_model,
            &hourly_axis,
        );
        assert_eq!(
            hourly_series
                .iter()
                .map(|series| series.schedule_id)
                .collect::<Vec<_>>(),
            [ScheduleId(37), ScheduleId(38)]
        );
        assert!(
            hourly_series[0]
                .values
                .iter()
                .all(|value| value.is_finite())
        );

        let run_period = february_run_period();
        let environment_axis =
            build_environment_time_axis_for_run_period_with_zone_timesteps(&run_period, 1, 2)
                .expect("leap-day environment axis should build");
        let environment_model = TypedModel {
            run_periods: vec![run_period],
            file_shading_schedule: Some(shading_schedule(365, 2, 91)),
            ..TypedModel::default()
        };
        let environment_series =
            super::super::precompute_schedule_value_series_for_environment_time_axis(
                &environment_model,
                &environment_axis,
            );
        assert_eq!(environment_series[0].schedule_id, ScheduleId(91));
        assert_eq!(environment_series[0].values.len(), 3 * 24 * 2);
        assert_eq!(
            environment_series[0].values[0],
            environment_series[0].values[48]
        );
        assert_ne!(
            environment_series[0].values[48],
            environment_series[0].values[96]
        );
    }

    #[test]
    fn hourly_time_axis_averages_tph4_and_rejects_mismatch() {
        let matching_model = TypedModel {
            timestep: TimestepConfig {
                number_of_timesteps_per_hour: 4,
            },
            file_shading_schedule: Some(shading_schedule(365, 4, 52)),
            ..TypedModel::default()
        };
        let matching_axis =
            build_hourly_time_axis(&matching_model).expect("matching hourly axis should build");
        let matching_series = super::super::precompute_schedule_value_series_for_time_axis(
            &matching_model,
            &matching_axis,
        );
        assert_eq!(matching_series[0].values[0], 1.5);
        assert_eq!(matching_series[0].values[1], 5.5);

        let mismatched_model = TypedModel {
            timestep: TimestepConfig {
                number_of_timesteps_per_hour: 2,
            },
            file_shading_schedule: Some(shading_schedule(365, 4, 53)),
            ..TypedModel::default()
        };
        let mismatched_axis =
            build_hourly_time_axis(&mismatched_model).expect("mismatched hourly axis should build");
        let mismatched_series = super::super::precompute_schedule_value_series_for_time_axis(
            &mismatched_model,
            &mismatched_axis,
        );
        assert!(
            mismatched_series[0]
                .values
                .iter()
                .all(|value| value.is_nan())
        );
    }

    #[test]
    fn generated_columns_are_first_class_plan_and_output_schedules() {
        let typed = TypedModel {
            file_shading_schedule: Some(shading_schedule(365, 1, 37)),
            schedules: vec![ScheduleConstant {
                id: ScheduleId(38),
                name: NormalizedName::new("Constant"),
                schedule_type_limits: None,
                hourly_value: 0.5,
            }],
            ..TypedModel::default()
        };
        let model = SimulationModel::from_typed(typed);
        let plan = build_execution_plan(&model);
        let evaluated_schedule_ids = plan
            .stages
            .iter()
            .flat_map(|stage| stage.steps.iter())
            .filter_map(|step| match step {
                ExecutionStep::EvaluateSchedule(schedule_id) => Some(*schedule_id),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(evaluated_schedule_ids, [ScheduleId(37), ScheduleId(38)]);

        let registry = RuntimeOutputRegistry::from_model(&model);
        assert!(
            registry
                .find_output(&RuntimeOutputRequest::hourly(
                    "South Wall_shading",
                    "Schedule Value",
                ))
                .is_some()
        );
    }

    #[test]
    fn leap_source_keeps_day_60_distinct() {
        let schedule = shading_schedule(366, 1, 9);
        let day_59 = file_shading_zone_timestep_value(&schedule, &schedule.columns[0], 1, 59, 1, 1);
        let day_60 = file_shading_zone_timestep_value(&schedule, &schedule.columns[0], 1, 60, 1, 1);

        assert_eq!(day_59, Some(f64::from(58 * 24)));
        assert_eq!(day_60, Some(f64::from(59 * 24)));
        assert_ne!(day_59, day_60);
    }

    #[test]
    fn timestep_mismatch_fails_closed_to_nan() {
        let schedule = shading_schedule(365, 4, 12);
        let environment_mismatch =
            file_shading_column_series(&schedule, &schedule.columns[0], 2, [(1, 1, 1), (1, 1, 2)]);
        let hourly_axis_mismatch =
            file_shading_column_series(&schedule, &schedule.columns[0], 1, [(1, 1, 1)]);

        assert!(
            environment_mismatch
                .values
                .iter()
                .all(|value| value.is_nan())
        );
        assert!(hourly_axis_mismatch.values[0].is_nan());
    }

    #[test]
    fn hour_only_apis_identify_generated_shading_schedule_ids() {
        let schedule = shading_schedule(365, 1, 73);
        let model = TypedModel {
            file_shading_schedule: Some(schedule),
            other_equipment: vec![OtherEquipment {
                id: InternalGainId(0),
                name: NormalizedName::new("Equipment"),
                fuel_type: NormalizedName::new("None"),
                zone: ZoneId(0),
                schedule: Some(ScheduleId(73)),
                design_level_calculation_method:
                    OtherEquipmentDesignLevelCalculationMethod::EquipmentLevel,
                design_level_w: 100.0,
                power_per_floor_area_w_per_m2: 0.0,
                power_per_person_w: 0.0,
                fraction_latent: 0.0,
                fraction_radiant: 0.0,
                fraction_lost: 0.0,
                carbon_dioxide_generation_rate_m3_per_s_w: 0.0,
            }],
            ..TypedModel::default()
        };

        let series_error = super::super::precompute_schedule_value_series(&model, 1)
            .expect_err("hour-only schedule precompute must reject File:Shading");
        assert!(series_error.contains("Schedule:File:Shading"));
        assert!(series_error.contains("zone timestep"));

        let validation_error = super::super::validate_hour_only_internal_gain_schedules(&model)
            .expect_err("hour-only internal gains must reject generated File:Shading IDs");
        assert!(matches!(
            validation_error,
            RuntimeError::InvalidInternalGainSchedule {
                schedule_id: 73,
                reason,
                ..
            } if reason.contains("Schedule:File:Shading")
                && reason.contains("zone-timestep-aware")
        ));
    }
}
