//! Input-only typed admission, partial caller assignment and calendar metadata.
use super::{Result, digest};
use ep_model::{
    DayOfWeek, FirstHourInterpolationStartingValues, NormalizedName, RunPeriod, RunPeriodId,
    SiteLocation,
};
use ep_runtime::time_axis::build_hourly_time_axis_for_run_period_with_weather_metadata_and_zone_timesteps;
use ep_runtime::weather::day::{WeatherEnvironmentConfiguration, WeatherGlobalState};
use ep_runtime::weather::raw::load_parsed_epw_weather_file;
use serde_json::Value;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

pub(super) fn require(ok: bool, message: &str) -> Result<()> {
    if ok {
        Ok(())
    } else {
        Err(message.to_owned().into())
    }
}
pub(super) fn text(v: &Value) -> Result<&str> {
    v.as_str().ok_or_else(|| "typed string required".into())
}
pub(super) fn flag(v: &Value) -> Result<bool> {
    v.as_bool().ok_or_else(|| "typed bool required".into())
}
pub(super) fn integer(v: &Value) -> Result<u32> {
    Ok(u32::try_from(
        v.as_u64().ok_or("unsigned integer required")?,
    )?)
}
pub(super) fn array(v: &Value) -> Result<&[Value]> {
    v.as_array()
        .map(Vec::as_slice)
        .ok_or_else(|| "typed array required".into())
}
pub(super) fn real(v: &Value) -> Result<f64> {
    let object = v
        .as_object()
        .ok_or("exact binary64 input object required")?;
    let bits = text(&v["bits"])?;
    require(
        object.len() == 1
            && bits.len() == 16
            && bits
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)),
        "lowercase binary64 bits required",
    )?;
    let value = f64::from_bits(u64::from_str_radix(bits, 16)?);
    require(value.is_finite(), "finite declared inputs required")?;
    Ok(value)
}
pub(super) fn relative(root: &Path, path: &Path) -> Result<String> {
    Ok(path
        .strip_prefix(root)?
        .to_str()
        .ok_or("UTF-8 path required")?
        .replace('\\', "/"))
}
pub(super) fn bound_file(root: &Path, binding: &Value) -> Result<(PathBuf, Vec<u8>)> {
    let path = root.join(text(&binding["path"])?).canonicalize()?;
    require(path.starts_with(root), "input path escapes repository")?;
    let bytes = std::fs::read(&path)?;
    require(
        digest::sha256(&bytes) == text(&binding["sha256"])?,
        "bound input bytes differ",
    )?;
    Ok((path, bytes))
}
pub(super) fn bindings(native: &Value) -> Result<Vec<Value>> {
    let mut result = vec![native["fixed_scope"].clone()];
    result.extend_from_slice(array(&native["all_45_guard_refs"])?);
    for name in ["source", "tolerances"] {
        result.push(native["contracts"][name].clone());
    }
    for seq in array(&native["weather_sequences"])? {
        for name in ["input", "weather"] {
            result.push(seq[name].clone());
        }
        if !seq["external_byte_map"].is_null() {
            result.push(seq["external_byte_map"].clone());
        }
    }
    Ok(result)
}
pub(super) fn check_all(root: &Path, bindings: &[Value]) -> Result<()> {
    for binding in bindings {
        bound_file(root, binding)?;
    }
    Ok(())
}
pub(super) fn admit(native: &Value) -> Result<()> {
    require(
        text(&native["schema"])? == "clk04-helper-cases.v1"
            && native["energyplus_commit"] == "6f2e40d10250a105b49966baa24d843711e61048"
            && native["expected_values_supplied"] == false
            && native["expected_exits_supplied"] == false
            && native["original_results_supplied_to_Rust"] == false
            && native["frozen_before_numerical_execution"] == true,
        "final input-only native request required",
    )?;
    require(
        array(&native["all_45_guard_refs"])?.len() == 45
            && array(&native["setup_cases"])?.len() == 2
            && array(&native["wind_cases"])?.len() == 56
            && array(&native["weather_sequences"])?.len() == 13,
        "closed input inventory differs",
    )?;
    let mut names = BTreeSet::new();
    let mut counts = [0u32; 3];
    let mut lane_counts = [0u32; 2];
    let mut selected = [0u32; 2];
    for seq in array(&native["weather_sequences"])? {
        require(names.insert(text(&seq["id"])?), "duplicate sequence ID")?;
        let lane = match text(&seq["lane"])? {
            "diagnostic" => 0,
            "production-CON" => 1,
            _ => return Err("closed declared operating lane required".into()),
        };
        lane_counts[lane] += 1;
        let mut ids = BTreeSet::new();
        for op in array(&seq["operations"])? {
            require(ids.insert(text(&op["id"])?), "duplicate operation ID")?;
            let index = match text(&op["kind"])? {
                "GetNextEnvironment" => 0,
                "InitializeWeather" => 1,
                "SetCurrentWeather" => 2,
                _ => return Err("unsupported declared operation".into()),
            };
            counts[index] += 1;
            if index > 0 {
                let observed = flag(&op["selected_observation"])?;
                if index == 2 && observed {
                    selected[lane] += 1;
                }
            }
        }
    }
    require(
        counts == [13, 1584, 1584]
            && native["requested_counts"]["weather_operations"] == 3181
            && native["requested_counts"]["total_operations"] == 3239
            && lane_counts == [10, 3]
            && selected == [368, 480],
        "whole callback history/lane/mask cardinality differs",
    )?;
    Ok(())
}

pub(super) fn seed_caller(owner: &mut WeatherGlobalState, caller: &Value) -> Result<()> {
    for (key, v) in caller.as_object().ok_or("literal caller object required")? {
        match key.as_str() {
            "DayOfSim" => owner.day_of_sim = i32::try_from(integer(v)?)?,
            "DayOfSimChr" => owner.day_of_sim_chr = text(v)?.to_owned(),
            "HourOfDay" => owner.hour_of_day = i32::try_from(integer(v)?)?,
            "TimeStep" => owner.time_step = i32::try_from(integer(v)?)?,
            "PreviousHour" => owner.previous_hour = i32::try_from(integer(v)?)?,
            "BeginSimFlag" => owner.begin_sim_flag = flag(v)?,
            "BeginEnvrnFlag" => owner.begin_envrn_flag = flag(v)?,
            "BeginDayFlag" => owner.begin_day_flag = flag(v)?,
            "BeginHourFlag" => owner.begin_hour_flag = flag(v)?,
            "BeginTimeStepFlag" => owner.begin_time_step_flag = flag(v)?,
            "EndDayFlag" => owner.end_day_flag = flag(v)?,
            "EndHourFlag" => owner.end_hour_flag = flag(v)?,
            "EndEnvrnFlag" => owner.end_envrn_flag = flag(v)?,
            "WarmupFlag" => owner.warmup_flag = flag(v)?,
            _ => return Err(format!("unsupported declared caller field {key}").into()),
        }
    }
    Ok(())
}

pub(super) fn configuration(
    input: &Value,
    idf: &[u8],
    weather: &Path,
) -> Result<WeatherEnvironmentConfiguration> {
    if text(&input["lane"])? == "production-CON" {
        // Frozen CON RunPeriod declarations use the existing CLK03 metadata shape.
        // This existing adapter validates actual IDF controls/location and creates
        // a separate metadata axis; it supplies no preview weather to live state.
        return super::existing_inputs::configuration(input, idf, weather);
    }
    require(
        text(&input["lane"])? == "diagnostic",
        "declared operating lane required",
    )?;
    let stripped = std::str::from_utf8(idf)?
        .lines()
        .map(|l| l.split('!').next().unwrap_or(""))
        .collect::<Vec<_>>()
        .join("\n");
    let objects = stripped
        .split(';')
        .filter(|part| !part.trim().is_empty())
        .map(|part| {
            part.split(',')
                .map(|t| t.trim().to_owned())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let one = |kind: &str| -> Result<&Vec<String>> {
        let found = objects
            .iter()
            .filter(|r| r[0].eq_ignore_ascii_case(kind))
            .collect::<Vec<_>>();
        require(found.len() == 1, "sole literal IDF object required")?;
        Ok(found[0])
    };
    let steps = integer(&input["time_steps_per_hour"])?;
    require(
        [1, 4].contains(&steps)
            && one("Timestep")?
                .get(1)
                .ok_or("IDF timestep missing")?
                .parse::<u32>()?
                == steps,
        "declared/IDF one or four timestep required",
    )?;
    require(
        objects.len() == 6,
        "bounded six-object diagnostic IDF required",
    )?;
    require(
        one("Version")?.get(1).is_some_and(|v| v == "26.1"),
        "declared version required",
    )?;
    one("Building")?;
    require(
        one("GlobalGeometryRules")?
            == &vec![
                "GlobalGeometryRules",
                "UpperLeftCorner",
                "Counterclockwise",
                "World",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>(),
        "literal geometry rules differ",
    )?;
    let row = one("RunPeriod")?;
    let rp = &input["run_period"];
    require(
        row.len() == 16 && row[1] == "CLK04 Unit",
        "bounded explicit RunPeriod required",
    )?;
    for (i, key) in [
        (2, "begin_month"),
        (3, "begin_day"),
        (4, "begin_year"),
        (5, "end_month"),
        (6, "end_day"),
        (7, "end_year"),
    ] {
        require(
            row[i].parse::<u32>()? == integer(&rp[key])?,
            "IDF and literal run-period dates differ",
        )?;
    }
    require(
        row[8] == text(&rp["start_weekday"])?
            && row[8] == "Monday"
            && row[9..15] == ["No", "No", "No", "Yes", "Yes", "No"],
        "literal run-period flags/weekday differ",
    )?;
    let policy = match row[15].as_str() {
        "Hour1" => FirstHourInterpolationStartingValues::Hour1,
        "Hour24" => FirstHourInterpolationStartingValues::Hour24,
        _ => return Err("interpolation policy invalid".into()),
    };
    require(
        row[15] == text(&rp["first_hour_policy"])?,
        "declared first-hour policy differs",
    )?;
    let run_period = RunPeriod {
        id: RunPeriodId(0),
        name: NormalizedName::new(&row[1]),
        begin_month: integer(&rp["begin_month"])?,
        begin_day_of_month: integer(&rp["begin_day"])?,
        begin_year: Some(integer(&rp["begin_year"])?),
        end_month: integer(&rp["end_month"])?,
        end_day_of_month: integer(&rp["end_day"])?,
        end_year: Some(integer(&rp["end_year"])?),
        day_of_week_for_start_day: Some(DayOfWeek::Monday),
        first_hour_interpolation_starting_values: policy,
        use_weather_file_holidays_and_special_days: false,
        use_weather_file_daylight_saving_period: false,
        apply_weekend_holiday_rule: false,
        use_weather_file_rain_indicators: true,
        use_weather_file_snow_indicators: true,
        treat_weather_as_actual: false,
    };
    // Separate existing eager metadata preparation owner; no parsed weather values
    // or preview parser counters are supplied to the independently opened live session.
    let parsed = load_parsed_epw_weather_file(weather)?;
    let header = parsed.raw_header;
    let site_row = one("Site:Location")?;
    require(
        site_row.len() == 6,
        "bounded explicit Site:Location required",
    )?;
    for (i, value) in [
        (2, header.latitude),
        (3, header.longitude),
        (4, header.time_zone),
        (5, header.elevation),
    ] {
        let literal = site_row[i].parse::<f64>()?;
        require(
            literal.is_finite() && literal.to_bits() == value.to_bits(),
            "actual IDF and EPW site coordinates differ",
        )?;
    }
    let site = SiteLocation {
        name: NormalizedName::new(&header.weather_location_title),
        latitude_deg: header.latitude,
        longitude_deg: header.longitude,
        time_zone_hours: header.time_zone,
        elevation_m: header.elevation,
    };
    let axis = build_hourly_time_axis_for_run_period_with_weather_metadata_and_zone_timesteps(
        &run_period,
        &parsed.physical_projection.calendar_metadata,
        steps,
    )?;
    Ok(WeatherEnvironmentConfiguration::new(
        &axis,
        &run_period,
        &site,
        0,
        0,
    )?)
}
