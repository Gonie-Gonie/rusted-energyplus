//! Input-only bounded request and calendar configuration; no reference results.
use super::{Result, digest};
use ep_model::{
    DayOfWeek, FirstHourInterpolationStartingValues, NormalizedName, RunPeriod, RunPeriodId,
    SiteLocation,
};
use ep_runtime::time_axis::build_hourly_time_axis_for_run_period_with_weather_metadata_and_zone_timesteps;
use ep_runtime::weather::day::WeatherEnvironmentConfiguration;
use ep_runtime::weather::raw::load_parsed_epw_weather_file;
use serde_json::Value;
use std::path::{Path, PathBuf};
pub(super) fn require(condition: bool, message: &str) -> Result<()> {
    if condition {
        Ok(())
    } else {
        Err(message.to_owned().into())
    }
}

pub(super) fn text(value: &Value) -> Result<&str> {
    value
        .as_str()
        .ok_or_else(|| "typed string input required".into())
}

pub(super) fn integer(value: &Value) -> Result<u32> {
    Ok(u32::try_from(
        value.as_u64().ok_or("nonnegative integer input required")?,
    )?)
}

pub(super) fn flag(value: &Value) -> Result<bool> {
    value
        .as_bool()
        .ok_or_else(|| "typed boolean input required".into())
}

pub(super) fn array(value: &Value) -> Result<&[Value]> {
    value
        .as_array()
        .map(Vec::as_slice)
        .ok_or_else(|| "typed array input required".into())
}

pub(super) fn relative(root: &Path, path: &Path) -> Result<String> {
    Ok(path
        .strip_prefix(root)?
        .to_str()
        .ok_or("UTF-8 repository path required")?
        .replace('\\', "/"))
}

pub(super) fn bound_file(root: &Path, binding: &Value) -> Result<(PathBuf, Vec<u8>)> {
    let path = root.join(text(&binding["path"])?).canonicalize()?;
    require(path.starts_with(root), "input reference escapes repository")?;
    let bytes = std::fs::read(&path)?;
    require(
        digest::sha256(&bytes) == text(&binding["sha256"])?,
        "actual input byte hash differs",
    )?;
    Ok((path, bytes))
}

pub(super) fn weekday(value: u32) -> Result<DayOfWeek> {
    match value {
        1 => Ok(DayOfWeek::Sunday),
        2 => Ok(DayOfWeek::Monday),
        3 => Ok(DayOfWeek::Tuesday),
        4 => Ok(DayOfWeek::Wednesday),
        5 => Ok(DayOfWeek::Thursday),
        6 => Ok(DayOfWeek::Friday),
        7 => Ok(DayOfWeek::Saturday),
        _ => Err("native weekday input outside 1..7".into()),
    }
}

pub(super) fn period(input: &Value) -> Result<RunPeriod> {
    let policy = match text(&input["first_hour_interpolation_starting_values"])? {
        "Hour1" => FirstHourInterpolationStartingValues::Hour1,
        "Hour24" => FirstHourInterpolationStartingValues::Hour24,
        _ => return Err("declared interpolation policy invalid".into()),
    };
    let year = |value: &Value| -> Result<Option<u32>> {
        if value.is_null() {
            Ok(None)
        } else {
            Ok(Some(integer(value)?))
        }
    };
    Ok(RunPeriod {
        id: RunPeriodId(0),
        name: NormalizedName::new(text(&input["name"])?),
        begin_month: integer(&input["begin_month"])?,
        begin_day_of_month: integer(&input["begin_day_of_month"])?,
        begin_year: year(&input["begin_year"])?,
        end_month: integer(&input["end_month"])?,
        end_day_of_month: integer(&input["end_day_of_month"])?,
        end_year: year(&input["end_year"])?,
        day_of_week_for_start_day: Some(weekday(integer(&input["start_weekday"])?)?),
        first_hour_interpolation_starting_values: policy,
        use_weather_file_holidays_and_special_days: flag(&input["use_weather_holidays"])?,
        use_weather_file_daylight_saving_period: flag(&input["use_weather_dst"])?,
        apply_weekend_holiday_rule: flag(&input["apply_weekend_holiday_rule"])?,
        // validated_period checks actual IDF Yes inputs before these are used.
        use_weather_file_rain_indicators: true,
        use_weather_file_snow_indicators: true,
        treat_weather_as_actual: flag(&input["treat_weather_as_actual"])?,
    })
}

pub(super) fn objects(bytes: &[u8]) -> Result<Vec<Vec<String>>> {
    let text = std::str::from_utf8(bytes)?;
    let stripped = text
        .lines()
        .map(|line| line.split('!').next().unwrap_or(""))
        .collect::<Vec<_>>()
        .join("\n");
    Ok(stripped
        .split(';')
        .filter(|part| !part.trim().is_empty())
        .map(|part| {
            part.split(',')
                .map(|token| token.trim().to_owned())
                .collect()
        })
        .collect())
}

fn token(row: &[String], index: usize) -> &str {
    row.get(index).map(String::as_str).unwrap_or("")
}

fn yes_no(value: &str, default: bool) -> Result<bool> {
    if value.is_empty() {
        Ok(default)
    } else if value.eq_ignore_ascii_case("Yes") {
        Ok(true)
    } else if value.eq_ignore_ascii_case("No") {
        Ok(false)
    } else {
        Err("actual IDF Yes/No input invalid".into())
    }
}

fn validated_period(row: &[String], declaration: &Value) -> Result<RunPeriod> {
    // Pinned IDD RunPeriod 1398-1488; native GetRunPeriodData 5300-5372.
    // This bounded lane requires literal dates and weekday, with only the two
    // admitted omitted trailing fields using their actual source defaults.
    require(
        (14..=16).contains(&row.len()),
        "bounded RunPeriod field count differs",
    )?;
    let mut actual = period(declaration)?;
    require(
        token(row, 1).eq_ignore_ascii_case(text(&declaration["name"])?),
        "IDF and declared RunPeriod name differ",
    )?;
    for (index, key) in [
        (2, "begin_month"),
        (3, "begin_day_of_month"),
        (5, "end_month"),
        (6, "end_day_of_month"),
    ] {
        require(
            token(row, index).parse::<u32>()? == integer(&declaration[key])?,
            "IDF and declared RunPeriod date differ",
        )?;
    }
    for (index, year) in [(4, actual.begin_year), (7, actual.end_year)] {
        let parsed = if token(row, index).is_empty() {
            None
        } else {
            Some(token(row, index).parse::<u32>()?)
        };
        require(parsed == year, "IDF and declared RunPeriod year differ")?;
    }
    let names = [
        "Sunday",
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
    ];
    let weekday_number = names
        .iter()
        .position(|name| token(row, 8).eq_ignore_ascii_case(name))
        .ok_or("literal IDF weekday required")?
        + 1;
    require(
        u32::try_from(weekday_number)? == integer(&declaration["start_weekday"])?,
        "IDF and declared weekday differ",
    )?;
    for (index, declared, default) in [
        (9, actual.use_weather_file_holidays_and_special_days, true),
        (10, actual.use_weather_file_daylight_saving_period, true),
        (11, actual.apply_weekend_holiday_rule, true),
        (14, actual.treat_weather_as_actual, false),
    ] {
        require(
            yes_no(token(row, index), default)? == declared,
            "IDF and declared RunPeriod flag differ",
        )?;
    }
    // Rain/snow flags are not separately declared in this input packet: admit
    // their actual literal Yes inputs, rather than silently inserting defaults.
    actual.use_weather_file_rain_indicators = yes_no(token(row, 12), true)?;
    actual.use_weather_file_snow_indicators = yes_no(token(row, 13), true)?;
    require(
        !token(row, 12).is_empty()
            && !token(row, 13).is_empty()
            && actual.use_weather_file_rain_indicators
            && actual.use_weather_file_snow_indicators,
        "bounded actual IDF rain/snow Yes inputs required",
    )?;
    let policy = token(row, 15);
    let actual_policy = if policy.is_empty() || policy.eq_ignore_ascii_case("Hour24") {
        FirstHourInterpolationStartingValues::Hour24
    } else if policy.eq_ignore_ascii_case("Hour1") {
        FirstHourInterpolationStartingValues::Hour1
    } else {
        return Err("actual IDF interpolation policy invalid".into());
    };
    require(
        actual_policy == actual.first_hour_interpolation_starting_values,
        "IDF and declared interpolation policy differ",
    )?;
    Ok(actual)
}

fn validated_site(
    objects: &[Vec<String>],
    header: &ep_runtime::weather::raw::RawEpwHeaderState,
) -> Result<SiteLocation> {
    let locations = objects
        .iter()
        .filter(|row| row[0].eq_ignore_ascii_case("Site:Location"))
        .collect::<Vec<_>>();
    require(
        locations.len() <= 1,
        "at most one actual Site:Location required",
    )?;
    if let Some(row) = locations.first() {
        require(
            (6..=7).contains(&row.len()) && !token(row, 1).is_empty(),
            "bounded actual Site:Location fields required",
        )?;
        require(
            !yes_no(token(row, 6), false)?,
            "Keep Site Location Information is outside the frozen lane",
        )?;
        for (index, expected) in [
            (2, header.latitude),
            (3, header.longitude),
            (4, header.time_zone),
            (5, header.elevation),
        ] {
            let value = token(row, index).parse::<f64>()?;
            require(
                value.is_finite() && value.to_bits() == expected.to_bits(),
                "actual IDF and EPW site coordinates differ",
            )?;
        }
    }
    // Genuine ResolveLocationInformation cc4433-4467 overrides location from
    // the existing EPW for RunPeriodWeather unless Keep Site is enabled. The
    // frozen diagnostic IDFs omit Site:Location; the CON IDFs agree numerically.
    Ok(SiteLocation {
        name: NormalizedName::new(&header.weather_location_title),
        latitude_deg: header.latitude,
        longitude_deg: header.longitude,
        time_zone_hours: header.time_zone,
        elevation_m: header.elevation,
    })
}

pub(super) fn configuration(
    input: &Value,
    idf_bytes: &[u8],
    weather: &Path,
) -> Result<WeatherEnvironmentConfiguration> {
    let objects = objects(idf_bytes)?;
    let count = |name: &str| {
        objects
            .iter()
            .filter(|row| row[0].eq_ignore_ascii_case(name))
            .count()
    };
    require(
        count("RunPeriod") == 1,
        "sole declared RunPeriod input required",
    )?;
    require(count("Timestep") == 1, "sole actual IDF Timestep required")?;
    let timestep = objects
        .iter()
        .find(|row| row[0].eq_ignore_ascii_case("Timestep"))
        .ok_or("IDF Timestep missing")?;
    require(
        timestep
            .get(1)
            .ok_or("IDF Timestep value missing")?
            .parse::<u32>()?
            == integer(&input["time_steps_per_hour"])?,
        "IDF and declared timestep differ",
    )?;
    let parsed = load_parsed_epw_weather_file(weather)?;
    let row = objects
        .iter()
        .find(|row| row[0].eq_ignore_ascii_case("RunPeriod"))
        .ok_or("IDF RunPeriod missing")?;
    let rp = validated_period(row, &input["run_period"])?;
    let axis = build_hourly_time_axis_for_run_period_with_weather_metadata_and_zone_timesteps(
        &rp,
        &parsed.physical_projection.calendar_metadata,
        integer(&input["time_steps_per_hour"])?,
    )?;
    let header = parsed.raw_header;
    let site = validated_site(&objects, &header)?;
    Ok(WeatherEnvironmentConfiguration::new(
        &axis,
        &rp,
        &site,
        i32::try_from(count("SizingPeriod:DesignDay"))?,
        i32::try_from(count("RunPeriodControl:SpecialDays"))?,
    )?)
}
