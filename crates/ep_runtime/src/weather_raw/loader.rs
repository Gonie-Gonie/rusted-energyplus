//! One raw parse followed by an explicit projection into the existing consumer.

use super::{
    HeaderCallerPreparation, HeaderContextConsumption, RawEpwHeaderState, RawEpwInput,
    RawEpwOutputs, RawEpwParserState, RawEpwStreamState, interpret_weather_data_line,
    open_epw_header,
};
use crate::weather::{
    EpwError, EpwRecord, EpwWeatherFile, parse_epw_calendar_metadata, parse_epw_data_periods,
};
use std::path::Path;

/// Raw source owners retained beside their explicit existing physical projection.
#[derive(Clone, Debug)]
pub struct ParsedEpwWeatherFile {
    /// Selected raw LOCATION, DST, holiday and data-period owner.
    pub raw_header: RawEpwHeaderState,
    /// Raw records in source order, before missing-value or physical processing.
    pub raw_records: Vec<RawEpwOutputs>,
    /// Existing calendar admission and eleven-field physical consumer interface.
    pub physical_projection: EpwWeatherFile,
    /// Actual size of the supplied file bytes.
    pub input_byte_length: usize,
    /// Actual stream position after the original header operation.
    pub stream_after_header: RawEpwStreamState,
    /// Actual stream after record extraction terminates.
    pub final_stream: RawEpwStreamState,
}

fn error(line: usize, reason: impl std::fmt::Display) -> EpwError {
    EpwError::InvalidValue {
        line,
        field: "raw EPW parser",
        value: reason.to_string(),
    }
}

/// Loads raw/header owners once and passes their values to the current runtime.
///
/// Civil-calendar admission remains in the existing calendar/data-period
/// adapters. This projection's rainfall policy is the existing compatibility
/// policy, distinct from the raw source value and later weather physics cards.
pub fn load_parsed_epw_weather_file(
    path: impl AsRef<Path>,
) -> Result<ParsedEpwWeatherFile, EpwError> {
    let bytes = std::fs::read(path)?;
    let input_byte_length = bytes.len();
    let text = std::str::from_utf8(&bytes)
        .map_err(|reason| error(1, reason))?
        .to_owned();
    let mut input = RawEpwInput::new_unopened(bytes);
    let mut raw_header = RawEpwHeaderState::default();
    let mut errors = false;
    open_epw_header(
        &mut input,
        &mut raw_header,
        &mut errors,
        HeaderCallerPreparation::default(),
        &mut HeaderContextConsumption::default(),
    )
    .map_err(|reason| error(input.line_reads, format!("{reason:?}")))?;
    if errors {
        return Err(error(input.line_reads, "header ErrorsFound"));
    }
    let (data_periods, _) = parse_epw_data_periods(&text)?;
    let calendar_metadata = parse_epw_calendar_metadata(&text)?;
    let stream_after_header = input.snapshot();
    let mut parser = RawEpwParserState {
        end_day_of_month: raw_header.month_ends,
        weather_code_missed_count: raw_header.weather_code_missed_count,
    };
    let mut raw_records = Vec::new();
    let mut records = Vec::new();
    let mut blank_seen = false;
    loop {
        let read = input
            .read_line()
            .map_err(|reason| error(input.line_reads, format!("{reason:?}")))?;
        if !read.read_good {
            break;
        }
        if read.data.trim().is_empty() {
            blank_seen = true;
            continue;
        }
        if blank_seen {
            return Err(error(input.line_reads, "interior blank weather row"));
        }
        let mut output = RawEpwOutputs::default();
        let result = interpret_weather_data_line(&mut parser, &read.data, &mut output);
        // Store the actual same-owner counter even if the raw call fails.
        raw_header.weather_code_missed_count = parser.weather_code_missed_count;
        result.map_err(|reason| error(input.line_reads, reason))?;
        records.push(project_record(&output, input.line_reads)?);
        raw_records.push(output);
    }
    Ok(ParsedEpwWeatherFile {
        raw_header,
        raw_records,
        physical_projection: EpwWeatherFile {
            calendar_metadata,
            data_periods,
            records,
        },
        input_byte_length,
        stream_after_header,
        final_stream: input.snapshot(),
    })
}

fn project_record(raw: &RawEpwOutputs, line: usize) -> Result<EpwRecord, EpwError> {
    let mut dates = [0_u32; 5];
    for (target, value) in dates.iter_mut().zip(raw.dates) {
        *target = u32::try_from(value).map_err(|reason| error(line, reason))?;
    }
    let real = raw.mandatory_reals;
    let liquid = raw.optional_reals[5];
    Ok(EpwRecord {
        year: dates[0],
        month: dates[1],
        day: dates[2],
        hour: dates[3],
        minute: dates[4],
        dry_bulb_c: real[0],
        dew_point_c: real[1],
        relative_humidity_percent: real[2],
        atmospheric_pressure_pa: real[3],
        horizontal_infrared_radiation_wh_per_m2: real[6],
        global_horizontal_radiation_wh_per_m2: real[7],
        direct_normal_radiation_wh_per_m2: real[8],
        diffuse_horizontal_radiation_wh_per_m2: real[9],
        wind_direction_deg: real[14],
        wind_speed_m_per_s: real[15],
        liquid_precipitation_depth_mm: if liquid >= 99.0 { 0.0 } else { liquid.max(0.0) },
    })
}
