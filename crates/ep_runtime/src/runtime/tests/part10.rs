    fn test_run_period(
        name: &str,
        begin_month: u32,
        begin_day_of_month: u32,
        end_month: u32,
        end_day_of_month: u32,
    ) -> RunPeriod {
        RunPeriod {
            id: RunPeriodId(0),
            name: NormalizedName::new(name),
            begin_month,
            begin_day_of_month,
            begin_year: None,
            end_month,
            end_day_of_month,
            end_year: None,
            day_of_week_for_start_day: None,
            first_hour_interpolation_starting_values:
                FirstHourInterpolationStartingValues::Hour24,
            use_weather_file_holidays_and_special_days: true,
            use_weather_file_daylight_saving_period: true,
            apply_weekend_holiday_rule: true,
            use_weather_file_rain_indicators: true,
            use_weather_file_snow_indicators: true,
            treat_weather_as_actual: false,
        }
    }

    #[test]
    fn hourly_timestamp_label_uses_projected_calendar_state()
    -> Result<(), Box<dyn std::error::Error>> {
        let axis = build_hourly_time_axis_for_run_period(&test_run_period(
            "Timestamp Environment",
            1,
            1,
            1,
            1,
        ))?;

        assert_eq!(
            normalized_hourly_timestamp_label(&axis, &axis.points[0]),
            "env=TIMESTAMP ENVIRONMENT;day=1;month=1;date=1;dst=0;hour=1;start=0.00;end=60.00;day_type=Sunday"
        );

        Ok(())
    }

    #[test]
    fn run_period_calendar_uses_energyplus_yearless_weekday_lookup_and_correction()
    -> Result<(), Box<dyn std::error::Error>> {
        let mut yearless = test_run_period("Yearless Monday", 1, 1, 1, 1);
        yearless.day_of_week_for_start_day = Some(DayOfWeek::Monday);

        let resolved = resolve_run_period_calendar(&yearless)?;

        assert_eq!(resolved.start_year, 2007);
        assert_eq!(resolved.end_year, 2007);
        assert_eq!(resolved.start_day_of_week, DayOfWeek::Monday);

        yearless.begin_year = Some(2020);
        yearless.end_year = Some(2020);
        let corrected = resolve_run_period_calendar(&yearless)?;

        assert_eq!(corrected.start_year, 2020);
        assert_eq!(corrected.start_day_of_week, DayOfWeek::Wednesday);

        let default_leap = resolve_run_period_calendar(&test_run_period(
            "Yearless Leap Default",
            2,
            29,
            2,
            29,
        ))?;
        assert_eq!(default_leap.start_year, 2012);
        assert_eq!(default_leap.start_day_of_week, DayOfWeek::Wednesday);

        let mut weekday_leap =
            test_run_period("Yearless Leap Monday", 2, 29, 2, 29);
        weekday_leap.day_of_week_for_start_day = Some(DayOfWeek::Monday);
        let resolved_leap = resolve_run_period_calendar(&weekday_leap)?;
        assert_eq!(resolved_leap.start_year, 2016);
        assert_eq!(resolved_leap.start_day_of_week, DayOfWeek::Monday);

        Ok(())
    }

    #[test]
    fn run_period_calendar_rolls_omitted_end_year_and_finds_next_leap_day()
    -> Result<(), Box<dyn std::error::Error>> {
        let cross_year = resolve_run_period_calendar(&test_run_period(
            "Cross Year",
            12,
            31,
            1,
            1,
        ))?;

        assert_eq!(cross_year.start_year, 2017);
        assert_eq!(cross_year.end_year, 2018);
        assert_eq!(cross_year.total_days, 2);

        let next_leap =
            resolve_run_period_calendar(&test_run_period("Next Leap", 3, 1, 2, 29))?;

        assert_eq!(next_leap.start_year, 2017);
        assert_eq!(next_leap.end_year, 2020);
        assert!(next_leap.end_year_is_leap_year);

        let mut leap_transition =
            test_run_period("Leap Year Transition", 12, 31, 1, 1);
        leap_transition.begin_year = Some(2019);
        leap_transition.end_year = Some(2020);
        let transition_axis = build_hourly_time_axis_for_run_period(&leap_transition)?;
        assert!(!transition_axis.points[0].gregorian_year_is_leap_year);
        assert!(transition_axis.points[24].gregorian_year_is_leap_year);

        Ok(())
    }

    #[test]
    fn run_period_calendar_rejects_pre_gregorian_start_and_end_year_without_start() {
        let mut pre_gregorian = test_run_period("Too Early", 1, 1, 1, 1);
        pre_gregorian.begin_year = Some(1582);
        assert!(matches!(
            resolve_run_period_calendar(&pre_gregorian),
            Err(TimeAxisError::StartYearBeforeGregorianCalendar { year: 1582, .. })
        ));

        let mut missing_start_year = test_run_period("Missing Start Year", 1, 1, 1, 1);
        missing_start_year.end_year = Some(2020);
        assert!(matches!(
            resolve_run_period_calendar(&missing_start_year),
            Err(TimeAxisError::EndYearWithoutStartYear { .. })
        ));
    }

    #[test]
    fn environment_time_axis_uses_zone_timestep_source_order_and_flags()
    -> Result<(), Box<dyn std::error::Error>> {
        let mut model = TypedModel {
            timestep: TimestepConfig {
                number_of_timesteps_per_hour: 4,
            },
            ..TypedModel::default()
        };
        let mut run_period = test_run_period("Quarter Hour", 1, 1, 1, 1);
        run_period.begin_year = Some(2021);
        run_period.end_year = Some(2021);
        model.run_periods.push(run_period);

        let axes = build_environment_time_axes(&model)?;
        let axis = &axes[0];

        assert_eq!(axis.sample_count(), 96);
        assert_eq!(axis.zone_timestep.timesteps_per_hour, 4);
        assert_eq!(axis.points[0].zone_timestep, 1);
        assert_eq!(axis.points[0].start_minute, 0.0);
        assert_eq!(axis.points[0].end_minute, 15.0);
        assert_eq!(axis.points[0].current_time_hours, 0.25);
        assert_eq!(axis.points[0].simulation_timestep, 1);
        assert!(axis.points[0].begin_environment);
        assert!(axis.points[0].begin_day);
        assert!(axis.points[0].begin_hour);
        assert!(!axis.points[0].end_hour);
        assert_eq!(axis.points[3].current_time_hours, 1.0);
        assert!(axis.points[3].end_hour);
        assert!(!axis.points[3].end_day);
        assert!(!axis.points[4].begin_day);
        assert!(axis.points[4].begin_hour);
        let final_point = axis.points.last().expect("one-day axis has a final point");
        assert_eq!(final_point.current_time_hours, 24.0);
        assert_eq!(final_point.simulation_timestep, 96);
        assert!(final_point.end_hour);
        assert!(final_point.end_day);
        assert!(final_point.end_environment);
        assert!(!final_point.dst);
        assert_eq!(final_point.special_day_type, None);
        assert!(!final_point.gregorian_year_is_leap_year);

        Ok(())
    }

    #[test]
    fn environment_time_axes_include_every_run_period_and_fallback()
    -> Result<(), Box<dyn std::error::Error>> {
        let fallback = build_environment_time_axes(&TypedModel::default())?;
        assert_eq!(fallback.len(), 1);
        assert_eq!(fallback[0].environment_index, 1);
        assert_eq!(fallback[0].sample_count(), 24 * 6);

        let mut model = TypedModel::default();
        model
            .run_periods
            .push(test_run_period("First Environment", 1, 1, 1, 1));
        model
            .run_periods
            .push(test_run_period("Second Environment", 2, 1, 2, 1));

        let axes = build_environment_time_axes(&model)?;

        assert_eq!(axes.len(), 2);
        assert_eq!(axes[0].environment_index, 1);
        assert_eq!(axes[1].environment_index, 2);
        assert_eq!(axes[0].environment_name, "FIRST ENVIRONMENT");
        assert_eq!(axes[1].environment_name, "SECOND ENVIRONMENT");
        assert_eq!(axes[1].points[0].environment_index, 2);

        Ok(())
    }
