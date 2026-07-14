fn epw_holiday_test_run_period(use_weather_file_holidays: bool) -> RunPeriod {
    let mut run_period = special_day_test_run_period(
        "EPW Holiday Policy",
        (2016, 2, 28),
        (2016, 3, 1),
        false,
    );
    run_period.use_weather_file_holidays_and_special_days = use_weather_file_holidays;
    run_period
}

fn fixed_epw_holiday_metadata() -> crate::weather::EpwCalendarMetadata {
    crate::weather::EpwCalendarMetadata {
        leap_year_observed: true,
        daylight_saving_period: None,
        holidays: vec![crate::weather::EpwHoliday {
            name: "EPW LEAP HOLIDAY".to_string(),
            date: crate::weather::EpwCalendarDateRule::MonthDay {
                month: 2,
                day_of_month: 29,
            },
        }],
    }
}

#[test]
fn run_period_flag_enables_epw_sunday_type_holiday_on_both_time_axes()
-> Result<(), Box<dyn std::error::Error>> {
    let metadata = fixed_epw_holiday_metadata();

    for use_weather_file_holidays in [true, false] {
        let model = special_day_test_model(
            epw_holiday_test_run_period(use_weather_file_holidays),
            Vec::new(),
        );
        let hourly_axis =
            crate::build_hourly_time_axis_with_weather_metadata(&model, &metadata)?;
        let environment_axes = build_environment_time_axes_with_weather_metadata(&model, &metadata)?;
        let environment_axis = &environment_axes[0];

        for state in [&hourly_axis.special_days, &environment_axis.special_days] {
            assert_eq!(state.weather_file_holidays_declared, 1);
            assert_eq!(
                state.run_period_uses_weather_file_holidays,
                use_weather_file_holidays
            );
            assert_eq!(
                state.weather_file_holidays_resolved,
                usize::from(use_weather_file_holidays)
            );
            assert_eq!(
                state.resolved_days.len(),
                usize::from(use_weather_file_holidays)
            );
        }

        let middle_day = if use_weather_file_holidays {
            (
                crate::DayType::Sunday,
                Some(crate::DayType::Sunday),
                1,
            )
        } else {
            (crate::DayType::Monday, None, 2)
        };
        assert_hourly_special_day(
            &hourly_axis,
            (2016, 2, 28),
            (crate::DayType::Sunday, None, 1),
        );
        assert_hourly_special_day(&hourly_axis, (2016, 2, 29), middle_day);
        assert_hourly_special_day(
            &hourly_axis,
            (2016, 3, 1),
            (crate::DayType::Tuesday, None, 3),
        );
        assert_environment_special_day(environment_axis, (2016, 2, 29), middle_day);

        if use_weather_file_holidays {
            let resolved = &hourly_axis.special_days.resolved_days[0];
            assert_eq!(resolved.source, crate::SpecialDaySource::WeatherFile);
            assert_eq!(resolved.name, "EPW LEAP HOLIDAY");
            assert_eq!(resolved.day_type, crate::DayType::Sunday);
            assert_eq!(resolved.start.day_of_year, 60);
        }
    }

    Ok(())
}

#[test]
fn disabling_epw_holidays_does_not_disable_input_file_special_days()
-> Result<(), Box<dyn std::error::Error>> {
    let model = special_day_test_model(
        epw_holiday_test_run_period(false),
        vec![special_day_input(
            0,
            "Input File Holiday",
            SpecialDayDateRule::MonthDay {
                month: 3,
                day_of_month: 1,
            },
            1,
            SpecialDayType::Holiday,
        )],
    );

    let axis = crate::build_hourly_time_axis_with_weather_metadata(
        &model,
        &fixed_epw_holiday_metadata(),
    )?;

    assert_eq!(axis.special_days.weather_file_holidays_declared, 1);
    assert!(!axis.special_days.run_period_uses_weather_file_holidays);
    assert_eq!(axis.special_days.weather_file_holidays_resolved, 0);
    assert_eq!(axis.special_days.input_file_special_days_declared, 1);
    assert_eq!(axis.special_days.resolved_days.len(), 1);
    assert_eq!(
        axis.special_days.resolved_days[0].source,
        crate::SpecialDaySource::InputFile
    );
    assert_hourly_special_day(
        &axis,
        (2016, 2, 29),
        (crate::DayType::Monday, None, 2),
    );
    assert_hourly_special_day(
        &axis,
        (2016, 3, 1),
        (crate::DayType::Holiday, Some(crate::DayType::Holiday), 8),
    );

    Ok(())
}
