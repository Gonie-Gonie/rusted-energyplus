//! Immutable initial-value handling for supported external-interface schedule families.

use super::{CachedScheduleSeries, ScheduleSampleStorage, ScheduleSeriesKind};
use ep_model::{
    ExternalInterfaceFmuExportSchedule, ExternalInterfaceFmuImportSchedule,
    ExternalInterfaceSchedule, ScheduleId, TypedModel,
};

pub(super) fn external_interface_schedule_value(
    model: &TypedModel,
    schedule_id: ScheduleId,
) -> Option<f64> {
    model
        .external_interface_schedules
        .iter()
        .find(|schedule| schedule.id == schedule_id)
        .map(|schedule| schedule.initial_value)
        .or_else(|| {
            model
                .external_interface_fmu_import_schedules
                .iter()
                .find(|schedule| schedule.id == schedule_id)
                .map(|schedule| schedule.initial_value)
        })
        .or_else(|| {
            model
                .external_interface_fmu_export_schedules
                .iter()
                .find(|schedule| schedule.id == schedule_id)
                .map(|schedule| schedule.initial_value)
        })
}

fn external_interface_cached_schedule_series(
    schedule: &ExternalInterfaceSchedule,
    sample_count: usize,
) -> CachedScheduleSeries {
    initial_value_cached_schedule_series(
        schedule.id,
        &schedule.name.0,
        schedule.initial_value,
        sample_count,
    )
}

fn external_interface_fmu_import_cached_schedule_series(
    schedule: &ExternalInterfaceFmuImportSchedule,
    sample_count: usize,
) -> CachedScheduleSeries {
    initial_value_cached_schedule_series(
        schedule.id,
        &schedule.name.0,
        schedule.initial_value,
        sample_count,
    )
}

fn external_interface_fmu_export_cached_schedule_series(
    schedule: &ExternalInterfaceFmuExportSchedule,
    sample_count: usize,
) -> CachedScheduleSeries {
    initial_value_cached_schedule_series(
        schedule.id,
        &schedule.name.0,
        schedule.initial_value,
        sample_count,
    )
}

fn initial_value_cached_schedule_series(
    schedule_id: ScheduleId,
    schedule_name: &str,
    initial_value: f64,
    sample_count: usize,
) -> CachedScheduleSeries {
    CachedScheduleSeries {
        schedule_id,
        schedule_name: schedule_name.to_string(),
        kind: ScheduleSeriesKind::ExternalInterfaceInitialValue {
            value: initial_value,
        },
        samples: ScheduleSampleStorage::Scalar {
            value: initial_value,
            len: sample_count,
        },
    }
}

pub(super) fn external_interface_cached_schedule_series_iter(
    model: &TypedModel,
    sample_count: usize,
) -> impl Iterator<Item = CachedScheduleSeries> + '_ {
    model
        .external_interface_schedules
        .iter()
        .map(move |schedule| external_interface_cached_schedule_series(schedule, sample_count))
        .chain(
            model
                .external_interface_fmu_import_schedules
                .iter()
                .map(move |schedule| {
                    external_interface_fmu_import_cached_schedule_series(schedule, sample_count)
                }),
        )
        .chain(
            model
                .external_interface_fmu_export_schedules
                .iter()
                .map(move |schedule| {
                    external_interface_fmu_export_cached_schedule_series(schedule, sample_count)
                }),
        )
}

#[cfg(test)]
mod tests {
    use super::super::{
        internal_gain_w, precompute_schedule_value_series,
        precompute_schedule_value_series_for_environment_time_axis,
        precompute_schedule_value_series_for_time_axis, schedule_value,
        validate_hour_only_internal_gain_schedules,
    };
    use super::*;
    use crate::time_axis::{
        build_environment_time_axis_for_run_period_with_zone_timesteps, build_hourly_time_axis,
    };
    use crate::{
        ExecutionStageKind, ExecutionStep, RuntimeOutputRegistry, RuntimeOutputRequest,
        build_execution_plan,
    };
    use ep_model::{
        DayOfWeek, ExternalInterfaceFmuExportSchedule, ExternalInterfaceFmuImportSchedule,
        FirstHourInterpolationStartingValues, InternalGainId, NormalizedName, OtherEquipment,
        OtherEquipmentDesignLevelCalculationMethod, RunPeriod, RunPeriodId, ScheduleConstant,
        ScheduleId, ScheduleYear, SimulationModel, TimestepConfig, TypedModel, WeekScheduleId,
        ZoneId,
    };

    fn external_schedule(id: u32, value: f64) -> ExternalInterfaceSchedule {
        ExternalInterfaceSchedule {
            id: ScheduleId(id),
            name: NormalizedName::new("External Fraction"),
            schedule_type_limits: None,
            initial_value: value,
        }
    }

    fn fmu_import_schedule(id: u32, value: f64) -> ExternalInterfaceFmuImportSchedule {
        ExternalInterfaceFmuImportSchedule {
            id: ScheduleId(id),
            name: NormalizedName::new("FMU Import Fraction"),
            schedule_type_limits: None,
            fmu_file_name: "UnusedModel.fmu".to_string(),
            fmu_instance_name: "UnusedInstance".to_string(),
            fmu_variable_name: "UnusedOutput".to_string(),
            initial_value: value,
        }
    }

    fn fmu_export_schedule(id: u32, value: f64) -> ExternalInterfaceFmuExportSchedule {
        ExternalInterfaceFmuExportSchedule {
            id: ScheduleId(id),
            name: NormalizedName::new("FMU Export Fraction"),
            schedule_type_limits: None,
            fmu_variable_name: "UnusedInput".to_string(),
            initial_value: value,
        }
    }

    fn one_day_run_period() -> RunPeriod {
        RunPeriod {
            id: RunPeriodId(0),
            name: NormalizedName::new("One Day"),
            begin_month: 1,
            begin_day_of_month: 1,
            begin_year: Some(2032),
            end_month: 1,
            end_day_of_month: 1,
            end_year: Some(2032),
            day_of_week_for_start_day: Some(DayOfWeek::Thursday),
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
    fn initial_value_is_immutable_on_hourly_and_zone_timestep_axes() {
        let model = TypedModel {
            timestep: TimestepConfig {
                number_of_timesteps_per_hour: 4,
            },
            external_interface_schedules: vec![external_schedule(12, 0.375)],
            ..TypedModel::default()
        };

        let hour_only = precompute_schedule_value_series(&model, 24)
            .expect("external schedules support hour-only precompute");
        assert_eq!(hour_only[0].values, vec![0.375; 24]);

        let hourly_axis = build_hourly_time_axis(&model).expect("hourly axis should build");
        let hourly = precompute_schedule_value_series_for_time_axis(&model, &hourly_axis);
        assert_eq!(hourly[0].values.len(), hourly_axis.points.len());
        assert!(hourly[0].values.iter().all(|value| *value == 0.375));
        assert_eq!(
            hourly[0].kind,
            ScheduleSeriesKind::ExternalInterfaceInitialValue { value: 0.375 }
        );

        let environment_axis = build_environment_time_axis_for_run_period_with_zone_timesteps(
            &one_day_run_period(),
            1,
            4,
        )
        .expect("one-day environment axis should build");
        let environment =
            precompute_schedule_value_series_for_environment_time_axis(&model, &environment_axis);
        assert_eq!(environment[0].values, vec![0.375; 96]);
    }

    #[test]
    fn fmu_import_initial_value_is_immutable_on_all_time_axis_paths() {
        let model = TypedModel {
            timestep: TimestepConfig {
                number_of_timesteps_per_hour: 4,
            },
            external_interface_fmu_import_schedules: vec![fmu_import_schedule(13, 0.625)],
            ..TypedModel::default()
        };

        let hour_only = precompute_schedule_value_series(&model, 24)
            .expect("inactive FMU-import schedules support hour-only precompute");
        assert_eq!(hour_only[0].values, vec![0.625; 24]);

        let hourly_axis = build_hourly_time_axis(&model).expect("hourly axis should build");
        let hourly = precompute_schedule_value_series_for_time_axis(&model, &hourly_axis);
        assert_eq!(hourly[0].values.len(), hourly_axis.points.len());
        assert!(hourly[0].values.iter().all(|value| *value == 0.625));
        assert_eq!(
            hourly[0].kind,
            ScheduleSeriesKind::ExternalInterfaceInitialValue { value: 0.625 }
        );

        let environment_axis = build_environment_time_axis_for_run_period_with_zone_timesteps(
            &one_day_run_period(),
            1,
            4,
        )
        .expect("one-day environment axis should build");
        let environment =
            precompute_schedule_value_series_for_environment_time_axis(&model, &environment_axis);
        assert_eq!(environment[0].values, vec![0.625; 96]);
    }

    #[test]
    fn fmu_export_initial_value_is_immutable_on_all_time_axis_paths() {
        let model = TypedModel {
            timestep: TimestepConfig {
                number_of_timesteps_per_hour: 4,
            },
            external_interface_fmu_export_schedules: vec![fmu_export_schedule(14, 0.875)],
            ..TypedModel::default()
        };

        let hour_only = precompute_schedule_value_series(&model, 24)
            .expect("inactive FMU-export schedules support hour-only precompute");
        assert_eq!(hour_only[0].values, vec![0.875; 24]);

        let hourly_axis = build_hourly_time_axis(&model).expect("hourly axis should build");
        let hourly = precompute_schedule_value_series_for_time_axis(&model, &hourly_axis);
        assert_eq!(hourly[0].values.len(), hourly_axis.points.len());
        assert!(hourly[0].values.iter().all(|value| *value == 0.875));
        assert_eq!(
            hourly[0].kind,
            ScheduleSeriesKind::ExternalInterfaceInitialValue { value: 0.875 }
        );

        let environment_axis = build_environment_time_axis_for_run_period_with_zone_timesteps(
            &one_day_run_period(),
            1,
            4,
        )
        .expect("one-day environment axis should build");
        let environment =
            precompute_schedule_value_series_for_environment_time_axis(&model, &environment_axis);
        assert_eq!(environment[0].values, vec![0.875; 96]);
    }

    #[test]
    fn fmu_export_default_zero_initial_value_supports_downstream_consumers() {
        let schedule = fmu_export_schedule(28, 0.0);
        let model = TypedModel {
            external_interface_fmu_export_schedules: vec![schedule],
            other_equipment: vec![OtherEquipment {
                id: InternalGainId(0),
                name: NormalizedName::new("Equipment"),
                fuel_type: NormalizedName::new("None"),
                zone: ZoneId(0),
                schedule: Some(ScheduleId(28)),
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

        assert_eq!(schedule_value(&model, ScheduleId(28), 19), Some(0.0));
        validate_hour_only_internal_gain_schedules(&model)
            .expect("FMU-export default-zero values are scalar schedules");
        assert_eq!(internal_gain_w(&model, ZoneId(0), 19), 0.0);
    }

    #[test]
    fn fmu_import_initial_value_supports_hour_only_downstream_consumers() {
        let schedule = fmu_import_schedule(27, 0.375);
        let model = TypedModel {
            external_interface_fmu_import_schedules: vec![schedule],
            other_equipment: vec![OtherEquipment {
                id: InternalGainId(0),
                name: NormalizedName::new("Equipment"),
                fuel_type: NormalizedName::new("None"),
                zone: ZoneId(0),
                schedule: Some(ScheduleId(27)),
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

        assert_eq!(schedule_value(&model, ScheduleId(27), 19), Some(0.375));
        validate_hour_only_internal_gain_schedules(&model)
            .expect("external initial values are hour-only scalar schedules");
        assert_eq!(internal_gain_w(&model, ZoneId(0), 19), 37.5);
    }

    #[test]
    fn external_schedule_families_follow_year_schedules_in_traces_plan_and_outputs() {
        let typed = TypedModel {
            schedules: vec![ScheduleConstant {
                id: ScheduleId(7),
                name: NormalizedName::new("Constant"),
                schedule_type_limits: None,
                hourly_value: 0.5,
            }],
            year_schedules: vec![ScheduleYear {
                id: ScheduleId(8),
                name: NormalizedName::new("Annual"),
                schedule_type_limits: None,
                week_schedules: [WeekScheduleId(0); 366],
            }],
            external_interface_schedules: vec![external_schedule(9, 0.375)],
            external_interface_fmu_import_schedules: vec![fmu_import_schedule(10, 0.625)],
            external_interface_fmu_export_schedules: vec![fmu_export_schedule(11, 0.875)],
            ..TypedModel::default()
        };
        let axis = build_hourly_time_axis(&typed).expect("hourly axis should build");
        let traces = precompute_schedule_value_series_for_time_axis(&typed, &axis);
        assert_eq!(
            traces
                .iter()
                .map(|trace| trace.schedule_id)
                .collect::<Vec<_>>(),
            [
                ScheduleId(7),
                ScheduleId(8),
                ScheduleId(9),
                ScheduleId(10),
                ScheduleId(11)
            ]
        );

        let model = SimulationModel::from_typed(typed);
        let plan = build_execution_plan(&model);
        let evaluated = plan
            .stages
            .iter()
            .flat_map(|stage| stage.steps.iter())
            .filter_map(|step| match step {
                ExecutionStep::EvaluateSchedule(id) => Some(*id),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(
            evaluated,
            [
                ScheduleId(7),
                ScheduleId(8),
                ScheduleId(9),
                ScheduleId(10),
                ScheduleId(11)
            ]
        );
        let init = plan
            .stages
            .iter()
            .find(|stage| stage.kind == ExecutionStageKind::InitHeatBalance)
            .expect("InitHeatBalance stage should exist");
        assert_eq!(
            init.prebound.schedule_ids,
            [
                ScheduleId(7),
                ScheduleId(8),
                ScheduleId(9),
                ScheduleId(10),
                ScheduleId(11)
            ]
        );

        let registry = RuntimeOutputRegistry::from_model(&model);
        let registered_schedule_names = registry
            .outputs()
            .iter()
            .filter(|output| output.variable_name == "Schedule Value")
            .map(|output| output.key.as_str())
            .collect::<Vec<_>>();
        assert_eq!(
            registered_schedule_names,
            [
                "CONSTANT",
                "ANNUAL",
                "EXTERNAL FRACTION",
                "FMU IMPORT FRACTION",
                "FMU EXPORT FRACTION"
            ]
        );
        assert!(
            registry
                .find_output(&RuntimeOutputRequest::hourly(
                    "External Fraction",
                    "Schedule Value",
                ))
                .is_some()
        );
        assert!(
            registry
                .find_output(&RuntimeOutputRequest::hourly(
                    "FMU Import Fraction",
                    "Schedule Value",
                ))
                .is_some()
        );
        assert!(
            registry
                .find_output(&RuntimeOutputRequest::hourly(
                    "FMU Export Fraction",
                    "Schedule Value",
                ))
                .is_some()
        );
    }
}
