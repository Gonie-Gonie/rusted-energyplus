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
DATA PERIODS\n\
2016,2,29,1,60,Source,1.0,0.0,50,101325,0,0,300,10,20,30,0,0,0,0,180,2.5\n"
    )
}

#[test]
fn parses_leap_year_observed_yes_and_no() -> Result<(), Box<dyn std::error::Error>> {
    let leap_weather = parse_epw_weather_file(&epw_text("Yes"))?;
    let non_leap_weather = parse_epw_weather_file(&epw_text("no"))?;

    assert!(leap_weather.calendar_metadata.leap_year_observed);
    assert!(!non_leap_weather.calendar_metadata.leap_year_observed);
    assert_eq!(leap_weather.records.len(), 1);
    assert_eq!(leap_weather.records[0].day, 29);

    Ok(())
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
