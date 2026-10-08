//! Public two-record existing wind pipeline; no private-helper shim or formula.
use super::{
    Result,
    inputs::{array, integer, real, require, text},
    raw_dto,
};
use ep_model::FirstHourInterpolationStartingValues;
use ep_runtime::weather::{EpwRecord, WeatherTimestepSeries};
use serde_json::{Value, json};

fn record(padding: &Value, direction: f64) -> Result<EpwRecord> {
    Ok(EpwRecord {
        year: integer(&padding["year"])?,
        month: integer(&padding["month"])?,
        day: integer(&padding["day"])?,
        hour: integer(&padding["hour"])?,
        minute: integer(&padding["minute"])?,
        dry_bulb_c: real(&padding["dry_bulb_c"])?,
        dew_point_c: real(&padding["dew_point_c"])?,
        relative_humidity_percent: real(&padding["relative_humidity_percent"])?,
        atmospheric_pressure_pa: real(&padding["atmospheric_pressure_pa"])?,
        horizontal_infrared_radiation_wh_per_m2: real(
            &padding["horizontal_infrared_radiation_wh_per_m2"],
        )?,
        global_horizontal_radiation_wh_per_m2: real(
            &padding["global_horizontal_radiation_wh_per_m2"],
        )?,
        direct_normal_radiation_wh_per_m2: real(&padding["direct_normal_radiation_wh_per_m2"])?,
        diffuse_horizontal_radiation_wh_per_m2: real(
            &padding["diffuse_horizontal_radiation_wh_per_m2"],
        )?,
        wind_direction_deg: direction,
        wind_speed_m_per_s: real(&padding["wind_speed_m_per_s"])?,
        liquid_precipitation_depth_mm: real(&padding["liquid_precipitation_depth_mm"])?,
    })
}
pub(super) fn observe(input: &Value, adapter: &Value) -> Result<Value> {
    require(
        integer(&adapter["zone_steps_per_hour"])? == 4
            && integer(&adapter["selected_record_index"])? == 1
            && text(&adapter["first_hour_interpolation_starting_values"])? == "Hour24",
        "declared public wind adapter selection differs",
    )?;
    let previous = real(&input["previous"])?;
    let current = real(&input["current"])?;
    require(
        (0.0..=360.0).contains(&previous) && (0.0..=360.0).contains(&current),
        "bounded input wind required",
    )?;
    let weights = array(&adapter["weight_to_timestep"])?;
    require(
        weights.len() == 4,
        "four exact declared weight-to-step rows required",
    )?;
    for (index, bits) in [
        "3fd0000000000000",
        "3fe0000000000000",
        "3fe8000000000000",
        "3ff0000000000000",
    ]
    .iter()
    .enumerate()
    {
        require(
            text(&weights[index]["current_weight"]["bits"])? == *bits
                && integer(&weights[index]["time_step"])? == u32::try_from(index + 1)?,
            "literal quarter-step adapter mapping differs",
        )?;
    }
    let bits = text(&input["current_weight"]["bits"])?;
    let selected = weights
        .iter()
        .filter(|row| row["current_weight"]["bits"] == bits)
        .collect::<Vec<_>>();
    require(
        selected.len() == 1,
        "wind weight must select exactly one declared zone step",
    )?;
    let step = integer(&selected[0]["time_step"])?;
    let padding = array(&adapter["record_padding"])?;
    require(
        padding.len() == 2,
        "two complete declared record paddings required",
    )?;
    let records = [
        record(&padding[0], previous)?,
        record(&padding[1], current)?,
    ];
    let series = WeatherTimestepSeries::from_records(
        &records,
        4,
        FirstHourInterpolationStartingValues::Hour24,
    );
    let sample = series
        .sample_for(1, step)
        .ok_or("actual public wind sample unavailable")?;
    Ok(
        json!({"id":input["id"],"inputs":input,"actual_return":raw_dto::scalar(sample.wind_direction_deg),
        "actual_rust_invoked":true,"actual_public_Rust_API":"WeatherTimestepSeries::from_records/sample_for",
        "actual_selected_record_index":sample.record_index,"actual_selected_time_step":sample.timestep,
        "direct_private_helper_invoked":false,"native_pure_function_invocation_claimed":false,
        "additional_pipeline_physics_paired":false,"adapter_padding_is_literal_caller_input":true,
        "call_outcome":{"status":"source_returned","source_fatal":false}}),
    )
}
