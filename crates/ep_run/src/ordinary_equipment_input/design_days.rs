//! SourceA inactive DesignDay selectors; genuine getters own all effects.
use super::selected;
use ep_raw_model::RawObject;

pub(super) fn admitted_object(object: &RawObject) -> bool {
    selected(
        object,
        "dry_bulb_temperature_range_modifier_type",
        &[
            "DefaultMultipliers",
            "MultiplierSchedule",
            "DifferenceSchedule",
            "TemperatureProfileSchedule",
        ],
    ) && selected(
        object,
        "humidity_condition_type",
        &[
            "WetBulb",
            "DewPoint",
            "HumidityRatio",
            "Enthalpy",
            "WetBulbProfileDefaultMultipliers",
            "RelativeHumiditySchedule",
            "WetBulbProfileDifferenceSchedule",
            "WetBulbProfileMultiplierSchedule",
        ],
    ) && selected(
        object,
        "solar_model_indicator",
        &[
            "ASHRAEClearSky",
            "ZhangHuang",
            "Schedule",
            "ASHRAETau",
            "ASHRAETau2017",
        ],
    ) && selected(
        object,
        "begin_environment_reset_mode",
        &[
            "FullResetAtBeginEnvironment",
            "SuppressAllBeginEnvironmentResets",
        ],
    )
    // Original named-day strings are preserved. Genuine Source getters resolve
    // only the selected dry-bulb, humidity and solar schedule roles.
}

// The official instance schema has no additionalProperties=false. Unknown
// members are opaque original-IP metadata, never alternate humidity operands.
pub(super) fn known_field(name: &str) -> bool {
    matches!(
        name,
        "month"
            | "day_of_month"
            | "day_type"
            | "maximum_dry_bulb_temperature"
            | "daily_dry_bulb_temperature_range"
            | "dry_bulb_temperature_range_modifier_type"
            | "dry_bulb_temperature_range_modifier_day_schedule_name"
            | "humidity_condition_type"
            | "wetbulb_or_dewpoint_at_maximum_dry_bulb"
            | "humidity_condition_day_schedule_name"
            | "humidity_ratio_at_maximum_dry_bulb"
            | "enthalpy_at_maximum_dry_bulb"
            | "daily_wet_bulb_temperature_range"
            | "barometric_pressure"
            | "wind_speed"
            | "wind_direction"
            | "rain_indicator"
            | "snow_indicator"
            | "daylight_saving_time_indicator"
            | "solar_model_indicator"
            | "beam_solar_day_schedule_name"
            | "diffuse_solar_day_schedule_name"
            | "ashrae_clear_sky_optical_depth_for_beam_irradiance_taub_"
            | "ashrae_clear_sky_optical_depth_for_diffuse_irradiance_taud_"
            | "sky_clearness"
            | "maximum_number_warmup_days"
            | "begin_environment_reset_mode"
    )
}
