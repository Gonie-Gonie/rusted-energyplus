//! Real Weather3 functions on one retained owner, not an independent solver.
use super::WeatherDayState;
use serde_json::Value;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn array(v: &Value) -> Result<&Vec<Value>> {
    v.as_array()
        .ok_or_else(|| "Native weather array absent".into())
}
fn real(v: &Value) -> Result<f64> {
    let s = v["value_bits"].as_str().ok_or("raw weather input absent")?;
    if s.len() != 16
        || !s
            .bytes()
            .all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c))
    {
        return Err("raw weather encoding invalid".into());
    }
    let value = f64::from_bits(u64::from_str_radix(s, 16)?);
    if !value.is_finite() {
        return Err("nonfinite named weather input".into());
    }
    Ok(value)
}
fn bits(v: f64, e: &Value, label: &str) -> Result<()> {
    if e["value_bits"].as_str() != Some(format!("{:016x}", v.to_bits()).as_str()) {
        return Err(format!(
            "{label}: actual Source/Native bits differ: Source={:016x} Native={}",
            v.to_bits(),
            e["value_bits"].as_str().unwrap_or("<missing>")
        )
        .into());
    }
    Ok(())
}
fn contextual<T>(result: Result<T>, case: &Value, time: Option<&Value>, phase: &str) -> Result<T> {
    result.map_err(|error| {
        format!(
            "Weather3 case={} phase={phase} time_bits={}: {error}",
            case["name"].as_str().unwrap_or("<missing>"),
            time.and_then(|v| v["value_bits"].as_str())
                .unwrap_or("<none>")
        )
        .into()
    })
}
fn values(v: &[f64], e: &Value, label: &str) -> Result<()> {
    let e = array(e)?;
    if e.len() != v.len() {
        return Err(format!("{label}: shape differs").into());
    }
    for (a, b) in v.iter().zip(e) {
        bits(*a, b, label)?;
    }
    Ok(())
}
fn native_identity(e: &Value) -> Result<u64> {
    let address = e["state_address"]
        .as_u64()
        .ok_or("Native Weather owner address must be unsigned")?;
    if address == 0 {
        return Err("Native Weather owner address must be nonnull".into());
    }
    Ok(address)
}
fn state(v: &WeatherDayState, e: &Value, native_owner: u64, source_owner: usize) -> Result<()> {
    if native_identity(e)? != native_owner {
        return Err("Native Weather owner changed within lifetime".into());
    }
    if v as *const WeatherDayState as usize != source_owner {
        return Err("actual Source WeatherDayState owner changed within lifetime".into());
    }

    let c = &v.environment.current_weather;
    if e["SunIsUp"].as_bool() != Some(c.sun_is_up)
        || e["SunIsUpPrevTS"].as_bool() != Some(c.sun_is_up_previous_timestep)
    {
        return Err("retained sun flags differ".into());
    }
    values(&c.solar_direction_cosines, &e["SOLCOS"], "SOLCOS")?;
    for (value, key) in [
        (v.weather.hour_angle, "HrAngle"),
        (v.weather.solar_altitude_angle, "SolarAltitudeAngle"),
        (v.weather.solar_azimuth_angle, "SolarAzimuthAngle"),
        (v.today_variables.sin_solar_declin_angle, "TodaySinDeclin"),
        (v.today_variables.cos_solar_declin_angle, "TodayCosDeclin"),
        (v.today_variables.equation_of_time, "TodayEquationOfTime"),
    ] {
        bits(value, &e[key], key)?;
    }
    Ok(())
}
impl WeatherDayState {
    pub(crate) fn run_native_gap_solar_clock(input: &Value, native: &Value) -> Result<usize> {
        let cases = array(&input["solar_clock"])?;
        let observed = array(&native["solar_clock"])?;
        if cases.len() != observed.len() {
            return Err("Weather3 case count differs".into());
        }
        let mut count = 0;
        for (case, oracle) in cases.iter().zip(observed) {
            if case != &oracle["input"] || case["name"] != oracle["name"] {
                return Err("Weather3 primary identity differs".into());
            }
            let mut owner = Self::default();
            let source_owner = &owner as *const WeatherDayState as usize;
            let native_owner = native_identity(&oracle["initial"])?;
            owner.environment.latitude = real(&case["latitude"])?;
            owner.environment.sin_latitude = real(&case["sin_latitude"])?;
            owner.environment.cos_latitude = real(&case["cos_latitude"])?;
            owner.environment.longitude = real(&case["longitude"])?;
            owner.environment.time_zone_meridian = real(&case["time_zone_meridian"])?;
            owner.environment.one_step_solar_time_offset = real(&case["TS1TimeOffset"])?;
            owner.global.time_steps_in_hour = i32::try_from(
                case["steps_per_hour"]
                    .as_i64()
                    .ok_or("weather timestep count absent")?,
            )?;
            let coefficients = owner.native_gap_daily_solar_coefficients(i32::try_from(
                case["day_of_year"].as_i64().ok_or("Julian day absent")?,
            )?);
            for (value, key) in coefficients.into_iter().zip([
                "A",
                "B",
                "C",
                "AnnVarSolConstant",
                "EquationOfTime",
                "SineSolarDeclination",
                "CosineSolarDeclination",
            ]) {
                contextual(
                    bits(value, &oracle["daily"][key], key),
                    case,
                    None,
                    "daily-return",
                )?;
            }
            contextual(
                state(&owner, &oracle["initial"], native_owner, source_owner),
                case,
                None,
                "initial",
            )?;
            count += 1;
            let times = array(&case["times"])?;
            let calls = array(&oracle["calls"])?;
            if times.len() != calls.len() {
                return Err("Weather3 retained call count differs".into());
            }
            for (time, expected) in times.iter().zip(calls) {
                if time != &expected["primary_time"] {
                    return Err("Weather3 current time input differs".into());
                }
                owner.global.current_time = real(time)?;
                let direction = owner.native_gap_sun_direction_cosines(owner.global.current_time);
                contextual(
                    values(
                        &direction,
                        &expected["direction"],
                        "CalculateSunDirectionCosines",
                    ),
                    case,
                    Some(time),
                    "CalculateSunDirectionCosines-return",
                )?;
                contextual(
                    state(&owner, &expected["before"], native_owner, source_owner),
                    case,
                    Some(time),
                    "before-DetermineSunUpDown",
                )?;
                super::current::determine_sun_up_down(&mut owner)?;
                contextual(
                    state(&owner, &expected["after"], native_owner, source_owner),
                    case,
                    Some(time),
                    "after-DetermineSunUpDown",
                )?;
                count += 2;
            }
        }
        Ok(count)
    }
}
