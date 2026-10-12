//! Selected GetDesignDayData projection for the shared ordinary input domain.
//! No second family grammar and no physical weather derived from EPW samples.
use ep_raw_model::{FieldName, RawModel, RawObject, RawValue};
use ep_runtime::weather::day::{
    DesignDayHumidity, DesignDayInput, DesignDayParameters, SetupSimulationControls,
    WeatherEnvironmentConfiguration,
};

fn field<'a>(object: &'a RawObject, name: &str) -> Option<&'a RawValue> {
    object.fields.get(&FieldName(name.into()))
}

fn blank(value: &RawValue) -> bool {
    matches!(value, RawValue::Null)
        || matches!(value, RawValue::String(text) if text.trim().is_empty())
}

fn numeric(object: &RawObject, name: &str, default: Option<f64>) -> Result<f64, String> {
    match field(object, name).filter(|value| !blank(value)) {
        Some(RawValue::Number(value)) => value
            .parse::<f64>()
            .ok()
            .filter(|v| v.is_finite())
            .ok_or_else(|| format!("DesignDay {name} requires a finite numeric input")),
        None => {
            default.ok_or_else(|| format!("DesignDay {name} is required on the selected branch"))
        }
        _ => Err(format!(
            "DesignDay {name} is outside the numeric input domain"
        )),
    }
}

fn integer(object: &RawObject, name: &str, default: Option<i32>) -> Result<i32, String> {
    let value = numeric(object, name, default.map(f64::from))?;
    if value.fract() != 0.0 || value < f64::from(i32::MIN) || value > f64::from(i32::MAX) {
        return Err(format!(
            "DesignDay {name} is outside the integer input domain"
        ));
    }
    Ok(value as i32)
}

fn text<'a>(object: &'a RawObject, name: &str, default: &'a str) -> Result<&'a str, String> {
    match field(object, name).filter(|value| !blank(value)) {
        Some(RawValue::String(value)) => Ok(value),
        None => Ok(default),
        _ => Err(format!(
            "DesignDay {name} is outside the string input domain"
        )),
    }
}

fn yes_no(object: &RawObject, name: &str, default: bool) -> Result<bool, String> {
    let value = text(object, name, if default { "Yes" } else { "No" })?;
    if value.eq_ignore_ascii_case("Yes") {
        Ok(true)
    } else if value.eq_ignore_ascii_case("No") {
        Ok(false)
    } else {
        Err(format!("{name} requires the actual Yes/No input domain"))
    }
}

fn selected(object: &RawObject, name: &str, expected: &str) -> Result<(), String> {
    if text(object, name, expected)?.eq_ignore_ascii_case(expected) {
        Ok(())
    } else {
        Err(format!(
            "DesignDay {name} is outside the selected {expected} branch"
        ))
    }
}

/// Capture the actual system input once, preserving untrimmed first-character semantics.
pub(crate) fn capture_reverse_design_days() -> Result<bool, String> {
    match std::env::var("REVERSEDD") {
        Ok(value) => Ok(matches!(
            value.as_bytes().first(),
            Some(b'Y' | b'y' | b'T' | b't')
        )),
        Err(std::env::VarError::NotPresent) => Ok(false),
        Err(std::env::VarError::NotUnicode(_)) => {
            Err("REVERSEDD is outside the UTF-8 input domain".into())
        }
    }
}

/// Project the real selected DD collection and SimulationControl defaults.
/// Existing ordinary_equipment_input admission remains the sole family/field grammar.
pub(crate) fn project(
    raw: &RawModel,
    configuration: &mut WeatherEnvironmentConfiguration,
    reverse: bool,
) -> Result<(), String> {
    let objects = raw
        .ordered_instances("SizingPeriod:DesignDay")
        .map_err(|e| e.to_string())?;
    if i32::try_from(objects.len()).ok() != Some(configuration.design_day_count) {
        return Err("DesignDay actual collection/count association differs".into());
    }
    if reverse && objects.len() == 1 {
        return Err("ReverseDD with one DesignDay requires the actual severe-error backend".into());
    }
    let mut days = Vec::with_capacity(objects.len());
    for (name, object) in objects {
        selected(
            object,
            "dry_bulb_temperature_range_modifier_type",
            "DefaultMultipliers",
        )?;
        selected(object, "solar_model_indicator", "ASHRAEClearSky")?;
        selected(
            object,
            "begin_environment_reset_mode",
            "FullResetAtBeginEnvironment",
        )?;
        for key in [
            "dry_bulb_temperature_range_modifier_day_schedule_name",
            "humidity_condition_day_schedule_name",
            "beam_solar_day_schedule_name",
            "diffuse_solar_day_schedule_name",
        ] {
            if field(object, key).is_some_and(|value| !blank(value)) {
                return Err(format!(
                    "DesignDay schedule {key} is outside the selected branch"
                ));
            }
        }
        let humidity_type = match text(object, "humidity_condition_type", "WetBulb")? {
            value if value.eq_ignore_ascii_case("WetBulb") => DesignDayHumidity::WetBulb,
            value if value.eq_ignore_ascii_case("DewPoint") => DesignDayHumidity::DewPoint,
            _ => return Err("DesignDay humidity type is outside selected WetBulb/DewPoint".into()),
        };
        let day_type_name = text(object, "day_type", "")?;
        let day_type = [
            "Sunday",
            "Monday",
            "Tuesday",
            "Wednesday",
            "Thursday",
            "Friday",
            "Saturday",
            "Holiday",
            "SummerDesignDay",
            "WinterDesignDay",
            "CustomDay1",
            "CustomDay2",
        ]
        .iter()
        .position(|value| value.eq_ignore_ascii_case(day_type_name))
        .map(|index| index as i32 + 1)
        .ok_or("DesignDay requires an actual day type")?;
        let barometric_pressure = field(object, "barometric_pressure")
            .filter(|value| !blank(value))
            .map(|_| numeric(object, "barometric_pressure", None))
            .transpose()?;
        let parameters = DesignDayParameters {
            name: name.0.to_ascii_uppercase(),
            month: integer(object, "month", None)?,
            day_of_month: integer(object, "day_of_month", None)?,
            day_type,
            maximum_dry_bulb: numeric(object, "maximum_dry_bulb_temperature", None)?,
            daily_dry_bulb_range: numeric(object, "daily_dry_bulb_temperature_range", Some(0.0))?,
            humidity_type,
            humidity_indicator: numeric(object, "wetbulb_or_dewpoint_at_maximum_dry_bulb", None)?,
            barometric_pressure,
            wind_speed: numeric(object, "wind_speed", None)?,
            wind_direction: numeric(object, "wind_direction", None)?,
            rain: yes_no(object, "rain_indicator", false)?,
            snow: yes_no(object, "snow_indicator", false)?,
            daylight_saving: yes_no(object, "daylight_saving_time_indicator", false)?,
            // Blank clearness follows a real warning branch, not silent default success.
            sky_clearness: numeric(object, "sky_clearness", None)?,
            maximum_warmup_days: integer(object, "maximum_number_warmup_days", Some(-1))?,
        };
        days.push(
            DesignDayInput::from_selected_parameters(parameters, configuration.steps())
                .map_err(|e| e.to_string())?,
        );
    }
    if reverse && days.len() > 1 {
        days.swap(0, 1);
    }
    let controls = raw
        .ordered_instances("SimulationControl")
        .map_err(|e| e.to_string())?;
    let controls = controls.first().map(|(_, object)| *object);
    let flag = |name: &str, default: bool| -> Result<bool, String> {
        controls.map_or(Ok(default), |object| yes_no(object, name, default))
    };
    let setup = SetupSimulationControls::from_admitted_input_flags(
        flag("do_zone_sizing_calculation", false)?,
        flag("do_system_sizing_calculation", false)?,
        flag("do_plant_sizing_calculation", false)?,
        flag("do_hvac_sizing_simulation_for_sizing_periods", false)?,
        flag("run_simulation_for_sizing_periods", true)?,
        flag("run_simulation_for_weather_file_run_periods", true)?,
    );
    configuration.design_days = days;
    configuration.reverse_design_days = reverse;
    configuration.setup_simulation_controls = Some(setup);
    Ok(())
}
