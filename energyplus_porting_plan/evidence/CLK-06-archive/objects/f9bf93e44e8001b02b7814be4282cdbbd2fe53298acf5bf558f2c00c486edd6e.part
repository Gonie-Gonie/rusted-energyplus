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
    require(path.starts_with(root) && path != root, "input path escapes repository")?;
    let bytes = std::fs::read(&path)?;
    require(
        digest::sha256(&bytes) == text(&binding["sha256"])?,
        "bound input bytes differ",
    )?;
    Ok((path, bytes))
}
pub(super) fn bindings(native: &Value) -> Result<Vec<Value>> {
    let mut result = vec![native["fixed_scope"].clone(), native["production_control_inputs"].clone(),
        native["production_input_only_caller_history"].clone(), native["root_preoutput_decision"].clone()];
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
fn argument_object(value: &Value, names: &[&str]) -> Result<()> {
    let object = value.as_object().ok_or("literal finite scalar arguments required")?;
    require(object.len() == names.len(), "exact scalar argument names required")?;
    for name in names { real(&value[*name])?; }
    Ok(())
}
pub(super) fn admit(native: &Value) -> Result<()> {
    require(
        text(&native["schema"])? == "clk06-helper-cases.v1"
            && native["energyplus_commit"] == "6f2e40d10250a105b49966baa24d843711e61048"
            && native["expected_values_supplied"] == false
            && native["expected_exits_supplied"] == false
            && native["original_results_supplied_to_Rust"] == false
            && native["scientific_execution_performed"] == false
            && native["root_preoutput_decision"] == serde_json::json!({
                "path":".runtime/porting/CLK-06/root-preoutput-decisions-01/decision.json",
                "sha256":"b12d5d633cf50d669054ce4c37dc231f8ecf3192971f1b7e12ae61d40c2b0a54"})
            && native["frozen_before_numerical_execution"] == true,
        "final input-only native request required",
    )?;
    require(
        array(&native["all_45_guard_refs"])?.len() == 45
            && array(&native["setup_cases"])?.len() == 3
            && array(&native["weather_sequences"])?.len() <= 100_000
            && array(&native["emissivity_cases"])?.len() <= 100_000,
        "closed input inventory differs",
    )?;
    let mut names = BTreeSet::new();
    let mut counts = [0u32; 4];
    let mut lane_counts = [0u32; 2];
    let mut setup_steps = BTreeSet::new();
    for setup in array(&native["setup_cases"])? {
        require(setup_steps.insert(integer(&setup["time_steps_per_hour"])?), "duplicate setup steps")?;
    }
    require(setup_steps == BTreeSet::from([1, 3, 4]), "closed setup steps differ")?;
    for input in array(&native["emissivity_cases"])? {
        require(names.insert(text(&input["id"])?), "duplicate direct ID")?;
        require(input["kind"] == "CalcSkyEmissivity" && integer(&input["SkyTempModel"])? == 0,
            "only declared ClarkAllen input selector admitted")?;
        argument_object(&input["arguments"], &["OpaqueSkyCover","DryBulb","DewPoint","RelHum"])?;
    }
    for seq in array(&native["weather_sequences"])? {
        require(names.insert(text(&seq["id"])?), "duplicate sequence ID")?;
        let lane = match text(&seq["lane"])? {
            "diagnostic" => 0,
            "production-CON" => 1,
            _ => return Err("closed declared operating lane required".into()),
        };
        lane_counts[lane] += 1;
        require([1, 3, 4].contains(&integer(&seq["time_steps_per_hour"])?), "closed sequence steps differ")?;
        let controls = seq["solar_controls"].as_object().ok_or("four typed controls required")?;
        require(controls.len() == 4, "exactly four declared controls required")?;
        for key in ["DisplayWeatherMissingDataWarnings", "IgnoreSolarRadiation", "IgnoreBeamRadiation", "IgnoreDiffuseRadiation"] {
            flag(&seq["solar_controls"][key])?;
        }
        let mut ids = BTreeSet::new();
        let operations = array(&seq["operations"])?;
        require(operations.len() <= 100_000, "bounded declared operation array required")?;
        for op in operations {
            require(ids.insert(text(&op["id"])?), "duplicate operation ID")?;
            let index = match text(&op["kind"])? {
                "GetNextEnvironment" => 0,
                "InitializeWeather" => 1,
                "SetCurrentWeather" => 2,
                "calcSky" => {
                    argument_object(&op["arguments"], &["OpaqueSkyCover","DryBulb","DewPoint","RelHum","IRHoriz"])?;
                    argument_object(&op["output_initial_values"], &["HorizIRSky","SkyTemp"])?;
                    3
                },
                _ => return Err("unsupported declared operation".into()),
            };
            counts[index] = counts[index].checked_add(1).ok_or("operation count overflow")?;
            flag(&op["selected_observation"])?;
            // Validate the complete literal caller without borrowing live state.
            seed_caller(&mut WeatherGlobalState::default(), &op["caller"])?;
        }
    }
    let total_weather = counts.iter().try_fold(0u32, |sum, value| sum.checked_add(*value))
        .ok_or("weather count overflow")?;
    let direct = u32::try_from(array(&native["emissivity_cases"])?.len())?;
    let total = total_weather.checked_add(direct).and_then(|v| v.checked_add(3)).ok_or("total count overflow")?;
    require(
        native["requested_counts"] == serde_json::json!({"setup_cases":3,"emissivity_cases":direct,
            "weather_sequences":lane_counts.iter().sum::<u32>(),"CalcSkyEmissivity":direct,
            "GetNextEnvironment":counts[0],"InitializeWeather":counts[1],"SetCurrentWeather":counts[2],
            "calcSky":counts[3],"weather_operations":total_weather,"total_operations":total})
            && lane_counts[1] == 3,
        "literal sky boundary history/counts differ",
    )?;
    Ok(())
}

pub(super) fn seed_caller(owner: &mut WeatherGlobalState, caller: &Value) -> Result<()> {
    if let Some(value) = caller.get("DayOfSimChr") {
        let day = integer(&caller["DayOfSim"])?;
        require(day > 0 && text(value)? == day.to_string(), "literal DayOfSim/DayOfSimChr differ")?;
    }
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
        [1, 3, 4].contains(&steps)
            && one("Timestep")?
                .get(1)
                .ok_or("IDF timestep missing")?
                .parse::<u32>()?
                == steps,
        "declared/IDF one, three or four timestep required",
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
            == &[
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
        row.len() == 16 && row[1] == "CLK06 Unit",
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
