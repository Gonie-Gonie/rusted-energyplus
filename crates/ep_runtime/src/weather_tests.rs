use super::*;

fn epw_text(leap_year_observed: &str) -> String {
    format!(
        "LOCATION,Example\n\
DESIGN CONDITIONS\n\
TYPICAL/EXTREME PERIODS\n\
GROUND TEMPERATURES\n\
HOLIDAYS/DAYLIGHT SAVINGS,{leap_year_observed},0,0,0\n\
COMMENTS 1\n\
COMMENTS 2\n\
DATA PERIODS,1,1,Data,Sunday,2/29,2/29\n\
2016,2,29,1,60,Source,1.0,0.0,50,101325,0,0,300,10,20,30,0,0,0,0,180,2.5\n"
    )
}

#[test]
fn parses_leap_year_observed_yes_and_no() -> Result<(), Box<dyn std::error::Error>> {
    let leap_weather = parse_epw_weather_file(&epw_text("Yes"))?;
    let non_leap_weather = parse_epw_weather_file(&epw_text("no"))?;

    assert!(leap_weather.calendar_metadata.leap_year_observed);
    assert!(!non_leap_weather.calendar_metadata.leap_year_observed);
    assert_eq!(leap_weather.calendar_metadata.daylight_saving_period, None);
    assert_eq!(leap_weather.data_periods.records_per_hour, 1);
    assert_eq!(leap_weather.data_periods.periods.len(), 1);
    assert_eq!(
        leap_weather.data_periods.periods[0].start_day_of_week,
        ep_model::DayOfWeek::Sunday
    );
    assert_eq!(
        leap_weather.data_periods.periods[0].start_date,
        EpwDataPeriodDate {
            year: None,
            month: 2,
            day: 29
        }
    );
    assert_eq!(leap_weather.records.len(), 1);
    assert_eq!(leap_weather.records[0].day, 29);

    Ok(())
}

#[test]
fn parses_fixed_month_day_daylight_saving_period() -> Result<(), Box<dyn std::error::Error>> {
    let contents = epw_text("No").replace(
        "HOLIDAYS/DAYLIGHT SAVINGS,No,0,0,0",
        "HOLIDAYS/DAYLIGHT SAVINGS,No,3/10,November 3,0",
    );

    let metadata = parse_epw_weather_file(&contents)?.calendar_metadata;

    assert_eq!(
        metadata.daylight_saving_period,
        Some(EpwDaylightSavingPeriod {
            start: EpwCalendarDateRule::MonthDay {
                month: 3,
                day_of_month: 10,
            },
            end: EpwCalendarDateRule::MonthDay {
                month: 11,
                day_of_month: 3,
            },
        })
    );
    Ok(())
}

#[test]
fn parses_nth_and_last_weekday_daylight_saving_rules() -> Result<(), Box<dyn std::error::Error>> {
    let contents = epw_text("Yes").replace(
        "HOLIDAYS/DAYLIGHT SAVINGS,Yes,0,0,0",
        "HOLIDAYS/DAYLIGHT SAVINGS,Yes,2nd Sunday in March,Last Sunday in November,0",
    );

    let metadata = parse_epw_weather_file(&contents)?.calendar_metadata;

    assert_eq!(
        metadata.daylight_saving_period,
        Some(EpwDaylightSavingPeriod {
            start: EpwCalendarDateRule::NthWeekdayInMonth {
                nth: 2,
                weekday: ep_model::DayOfWeek::Sunday,
                month: 3,
            },
            end: EpwCalendarDateRule::LastWeekdayInMonth {
                weekday: ep_model::DayOfWeek::Sunday,
                month: 11,
            },
        })
    );
    Ok(())
}

#[test]
fn accepts_energyplus_month_weekday_order_and_case() -> Result<(), Box<dyn std::error::Error>> {
    let contents = epw_text("Yes").replace(
        "HOLIDAYS/DAYLIGHT SAVINGS,Yes,0,0,0",
        "HOLIDAYS/DAYLIGHT SAVINGS,Yes,1st may monday,LAST friday OF october,0",
    );

    let metadata = parse_epw_weather_file(&contents)?.calendar_metadata;

    assert_eq!(
        metadata.daylight_saving_period,
        Some(EpwDaylightSavingPeriod {
            start: EpwCalendarDateRule::NthWeekdayInMonth {
                nth: 1,
                weekday: ep_model::DayOfWeek::Monday,
                month: 5,
            },
            end: EpwCalendarDateRule::LastWeekdayInMonth {
                weekday: ep_model::DayOfWeek::Friday,
                month: 10,
            },
        })
    );
    Ok(())
}

#[test]
fn zero_dst_endpoints_disable_the_epw_period() -> Result<(), Box<dyn std::error::Error>> {
    let contents = epw_text("No").replace(
        "HOLIDAYS/DAYLIGHT SAVINGS,No,0,0,0",
        "HOLIDAYS/DAYLIGHT SAVINGS,No,0.0,0,0",
    );

    assert_eq!(
        parse_epw_weather_file(&contents)?
            .calendar_metadata
            .daylight_saving_period,
        None
    );
    Ok(())
}

#[test]
fn rejects_a_single_zero_daylight_saving_endpoint() {
    for (start, end, expected_field) in [
        ("0", "11/3", "daylight saving start date"),
        ("3/10", "0.0", "daylight saving end date"),
    ] {
        let contents = epw_text("No").replace(
            "HOLIDAYS/DAYLIGHT SAVINGS,No,0,0,0",
            &format!("HOLIDAYS/DAYLIGHT SAVINGS,No,{start},{end},0"),
        );

        assert!(matches!(
            parse_epw_weather_file(&contents),
            Err(EpwError::InvalidValue { line: 5, field, value })
                if field == expected_field && (value == start || value == end)
        ));
    }
}

#[test]
fn rejects_invalid_daylight_saving_date_rules() {
    for (field_index, value) in [
        (2, "2/30"),
        (2, "6th Sunday in March"),
        (2, "2nd Funday in March"),
        (2, "May 1st Monday"),
        (3, "Last Sunday in Smarch"),
        (3, "366"),
    ] {
        let mut fields = ["HOLIDAYS/DAYLIGHT SAVINGS", "Yes", "3/10", "11/3", "0"];
        fields[field_index] = value;
        let contents =
            epw_text("Yes").replace("HOLIDAYS/DAYLIGHT SAVINGS,Yes,0,0,0", &fields.join(","));
        let expected_field = if field_index == 2 {
            "daylight saving start date"
        } else {
            "daylight saving end date"
        };

        assert!(matches!(
            parse_epw_weather_file(&contents),
            Err(EpwError::InvalidValue {
                line: 5,
                field,
                value: actual_value,
            }) if field == expected_field && actual_value == value
        ));
    }
}

#[test]
fn rejects_missing_daylight_saving_endpoint_fields() {
    for (header, expected_field) in [
        (
            "HOLIDAYS/DAYLIGHT SAVINGS,Yes",
            "daylight saving start date",
        ),
        (
            "HOLIDAYS/DAYLIGHT SAVINGS,Yes,3/10",
            "daylight saving end date",
        ),
    ] {
        let contents = epw_text("Yes").replace("HOLIDAYS/DAYLIGHT SAVINGS,Yes,0,0,0", header);

        assert!(matches!(
            parse_epw_weather_file(&contents),
            Err(EpwError::MissingField { line: 5, field }) if field == expected_field
        ));
    }
}

#[test]
fn matches_energyplus_calendar_header_prefix_case_and_whitespace()
-> Result<(), Box<dyn std::error::Error>> {
    for header in [
        "HOLIDAYS/DAYLIGHT SAVING",
        "HOLIDAYS/DAYLIGHT SAVINGS",
        "  holidays/daylight saving  ",
        "\tHoLiDaYs/DaYlIgHt SaViNgS\t",
    ] {
        let contents = epw_text("Yes").replace("HOLIDAYS/DAYLIGHT SAVINGS", header);
        assert!(
            parse_epw_weather_file(&contents)?
                .calendar_metadata
                .leap_year_observed,
            "header should match: {header:?}"
        );
    }

    Ok(())
}

#[test]
fn uses_first_non_whitespace_character_for_leap_year_policy()
-> Result<(), Box<dyn std::error::Error>> {
    for value in ["Y", "yes", " Yesterday ", "y-anything"] {
        assert!(
            parse_epw_weather_file(&epw_text(value))?
                .calendar_metadata
                .leap_year_observed,
            "value should allow leap years: {value:?}"
        );
    }
    for value in ["N", "no", "Sometimes", "0"] {
        assert!(
            !parse_epw_weather_file(&epw_text(value))?
                .calendar_metadata
                .leap_year_observed,
            "value should disallow leap years: {value:?}"
        );
    }

    Ok(())
}

#[test]
fn rejects_missing_calendar_header() {
    let contents = epw_text("Yes").replace(
        "HOLIDAYS/DAYLIGHT SAVINGS,Yes,0,0,0",
        "UNRELATED HEADER,Yes,0,0,0",
    );

    assert!(matches!(
        parse_epw_weather_file(&contents),
        Err(EpwError::MissingHeader { header })
            if header == "HOLIDAYS/DAYLIGHT SAVINGS"
    ));
}

#[test]
fn rejects_empty_leap_year_observed_value() {
    assert!(matches!(
        parse_epw_weather_file(&epw_text("  ")),
        Err(EpwError::InvalidValue {
            line: 5,
            field: "leap year observed",
            value
        }) if value == "  "
    ));
}

#[test]
fn rejects_missing_leap_year_observed_field() {
    let contents = epw_text("Yes").replace(
        "HOLIDAYS/DAYLIGHT SAVINGS,Yes,0,0,0",
        "HOLIDAYS/DAYLIGHT SAVINGS",
    );

    assert!(matches!(
        parse_epw_weather_file(&contents),
        Err(EpwError::MissingField {
            line: 5,
            field: "leap year observed"
        })
    ));
}

#[test]
fn record_parser_remains_compatible_with_minimal_synthetic_headers()
-> Result<(), Box<dyn std::error::Error>> {
    let contents = epw_text("Yes").replace(
        "HOLIDAYS/DAYLIGHT SAVINGS,Yes,0,0,0",
        "HOLIDAYS/DAYLIGHT SAVINGS",
    );

    let records = parse_epw_records(&contents)?;

    assert_eq!(records.len(), 1);
    assert_eq!(records[0].month, 2);
    assert_eq!(records[0].day, 29);

    Ok(())
}

#[test]
fn parses_multiple_data_periods_and_inherits_missing_end_year()
-> Result<(), Box<dyn std::error::Error>> {
    let contents = epw_text("Yes").replace(
        "DATA PERIODS,1,1,Data,Sunday,2/29,2/29",
        "DATA PERIODS,2,1,First,Sunday,2016/ 1/ 1,6/30,Second,Friday,1 July,Dec-31",
    );

    let weather_file = parse_epw_weather_file(&contents)?;

    assert_eq!(weather_file.data_periods.periods.len(), 2);
    assert_eq!(
        weather_file.data_periods.periods[0].start_date.year,
        Some(2016)
    );
    assert_eq!(
        weather_file.data_periods.periods[0].end_date.year,
        Some(2016)
    );
    assert_eq!(
        weather_file.data_periods.periods[1].start_day_of_week,
        ep_model::DayOfWeek::Friday
    );

    Ok(())
}

#[test]
fn parses_data_period_continuation_lines_before_weather_rows()
-> Result<(), Box<dyn std::error::Error>> {
    let contents = epw_text("Yes").replace(
        "DATA PERIODS,1,1,Data,Sunday,2/29,2/29",
        "DATA PERIODS,2,1,First,Sunday,2016/1/1,6/30,\nSecond,Friday,1 July,Dec-31",
    );

    let weather_file = parse_epw_weather_file(&contents)?;

    assert_eq!(weather_file.data_periods.periods.len(), 2);
    assert_eq!(weather_file.data_periods.periods[1].name, "Second");
    assert_eq!(weather_file.records.len(), 1);
    assert_eq!(weather_file.records[0].year, 2016);

    Ok(())
}

#[test]
fn rejects_invalid_data_period_header_values() {
    let invalid_weekday = epw_text("Yes").replace(
        "DATA PERIODS,1,1,Data,Sunday,2/29,2/29",
        "DATA PERIODS,1,1,Data,Funday,2/29,2/29",
    );
    assert!(matches!(
        parse_epw_weather_file(&invalid_weekday),
        Err(EpwError::InvalidValue {
            line: 8,
            field: "data period start day of week",
            ..
        })
    ));

    let invalid_date = epw_text("Yes").replace(
        "DATA PERIODS,1,1,Data,Sunday,2/29,2/29",
        "DATA PERIODS,1,1,Data,Sunday,2/30,2/29",
    );
    assert!(matches!(
        parse_epw_weather_file(&invalid_date),
        Err(EpwError::InvalidValue {
            line: 8,
            field: "data period start date",
            ..
        })
    ));

    let zero_records_per_hour = epw_text("Yes").replace(
        "DATA PERIODS,1,1,Data,Sunday,2/29,2/29",
        "DATA PERIODS,1,0,Data,Sunday,2/29,2/29",
    );
    assert!(matches!(
        parse_epw_weather_file(&zero_records_per_hour),
        Err(EpwError::InvalidValue {
            line: 8,
            field: "records per hour",
            ..
        })
    ));
}

#[test]
fn record_loader_preserves_record_only_api_results() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::temp_dir().join(format!(
        "rusted-energyplus-weather-{}-{}.epw",
        std::process::id(),
        line!()
    ));
    std::fs::write(&path, epw_text("Yes"))?;

    let weather_file = load_epw_weather_file(&path)?;
    let records = load_epw_records(&path)?;
    std::fs::remove_file(path)?;

    assert!(weather_file.calendar_metadata.leap_year_observed);
    assert_eq!(records, weather_file.records);

    Ok(())
}

#[test]
fn rich_parser_rejects_interior_blank_data_rows_while_legacy_record_parser_skips_them()
-> Result<(), Box<dyn std::error::Error>> {
    let contents = epw_text("Yes").replace(
        "DATA PERIODS,1,1,Data,Sunday,2/29,2/29\n2016",
        "DATA PERIODS,1,1,Data,Sunday,2/29,2/29\n\n2016",
    );

    assert!(matches!(
        parse_epw_weather_file(&contents),
        Err(EpwError::InvalidValue {
            line: 9,
            field: "weather data row",
            value,
        }) if value.is_empty()
    ));
    assert_eq!(parse_epw_records(&contents)?.len(), 1);

    Ok(())
}
