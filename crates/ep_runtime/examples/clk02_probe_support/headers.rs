//! Execute direct enum and whole-open routes as distinct actual owner calls.

use super::{Result, digest, dto, inputs};
use ep_runtime::weather::raw::{
    HeaderCallerPreparation, HeaderContextConsumption, HeaderKind, RawEpwHeaderError,
    RawEpwHeaderState, RawEpwInput, open_epw_header, process_epw_header,
};
use serde_json::{Value, json};
use std::path::Path;

fn kind(name: &str) -> Result<HeaderKind> {
    match name {
        "Location" => Ok(HeaderKind::Location),
        "HolidaysDST" => Ok(HeaderKind::HolidaysDst),
        "DataPeriods" => Ok(HeaderKind::DataPeriods),
        "Comments1" => Ok(HeaderKind::Comments1),
        "DesignConditions" => Ok(HeaderKind::DesignConditions),
        _ => Err(format!("unadmitted direct header enum: {name}").into()),
    }
}

fn prepare(state: &mut RawEpwHeaderState, input: &Value) -> Result<HeaderCallerPreparation> {
    state.latitude = inputs::number(&input["WeatherFileLatitude"])?;
    state.longitude = inputs::number(&input["WeatherFileLongitude"])?;
    state.time_zone = inputs::number(&input["WeatherFileTimeZone"])?;
    state.elevation = inputs::number(&input["WeatherFileElevation"])?;
    state.header_title = inputs::text(&input["EPWHeaderTitle"])?.to_owned();
    state.leap_year_add = inputs::integer(&input["LeapYearAdd"])?;
    inputs::require(
        inputs::integer(&input["NumEPWTypExtSets"])? == 0,
        "direct header source-only typical count prerequisite differs",
    )?;
    let count = inputs::integer(&input["InputProcessorSpecialDaysObjectCount"])?;
    inputs::require(count == 0, "empty InputProcessor prerequisite differs")?;
    Ok(HeaderCallerPreparation {
        run_period_control_special_days: count,
    })
}

pub(super) fn outcome(result: std::result::Result<(), RawEpwHeaderError>) -> Result<Value> {
    match result {
        Ok(()) => Ok(dto::returned()),
        Err(RawEpwHeaderError::SourceFatal(message)) => Ok(dto::fatal(message)),
        Err(RawEpwHeaderError::OutsideBoundedDomain(reason)) => {
            Err(format!("header probe input outside bounded source domain: {reason}").into())
        }
    }
}

pub(super) fn case(item: &Value, root: &Path) -> Result<Value> {
    let mut state = RawEpwHeaderState::default();
    let constructor = dto::header(&state);
    let mut errors = inputs::flag(&item["initial_ErrorsFound"])?;
    let errors_before = errors;
    let route = inputs::text(&item["route"])?;
    let direct = route == "direct-original-ProcessEPWHeader";
    inputs::require(
        direct || route == "whole-original-OpenEPlusWeatherFile",
        "header route differs",
    )?;
    inputs::require(
        item["expected_values_supplied"] == false && item["expected_exit_supplied"] == false,
        "header diagnostics must supply no answers",
    )?;
    let binding = &item[if direct { "stream" } else { "file" }];
    let (path, bytes) = inputs::file(root, binding)?;
    let actual_file = json!({"path":path.strip_prefix(root)?,"sha256":digest::sha256(&bytes),"size_bytes":bytes.len()});
    let mut input = RawEpwInput::new_unopened(bytes);
    let mut line = String::new();
    let mut caller = HeaderCallerPreparation::default();
    if direct {
        caller = prepare(&mut state, &item["prepared_state"])?;
        line = inputs::line(item, "initial_Line", "initial_Line_sha256")?;
        input.reopen();
    } else {
        state.leap_year_add = inputs::integer(&item["prepared_LeapYearAdd"])?;
        inputs::require(
            inputs::flag(&item["ProcessHeader"])?,
            "whole header processing must be enabled",
        )?;
    }
    let prepared = dto::header(&state);
    let stream_before = dto::stream(
        &input,
        if direct {
            path.strip_prefix(root)?
        } else {
            Path::new("")
        },
    );
    let line_before = line.clone();
    let mut context = HeaderContextConsumption::default();
    let result = if direct {
        process_epw_header(
            kind(inputs::text(&item["header_type"])?)?,
            &mut line,
            &mut errors,
            &mut state,
            &mut input,
            caller,
            &mut context,
        )
    } else {
        open_epw_header(&mut input, &mut state, &mut errors, caller, &mut context)
    };
    let call_outcome = outcome(result)?;
    Ok(
        json!({"case_id":inputs::text(&item["case_id"])?,"route":route,
            "input_declared_binding":binding,"actual_input_file":actual_file,
            "constructor":constructor,"prepared":prepared,"before":prepared,"after":dto::header(&state),
            "ErrorsFound_before":errors_before,"ErrorsFound_after":errors,"Line_observed":direct,
            "Line_before":if direct{Some(line_before.as_str())}else{None},
            "Line_after":if direct{Some(line.as_str())}else{None},
            "Line_before_sha256":if direct{Some(digest::sha256(line_before.as_bytes()))}else{None},
            "Line_after_sha256":if direct{Some(digest::sha256(line.as_bytes()))}else{None},
            "stream_before":stream_before,"stream_after":dto::stream(&input,path.strip_prefix(root)?),
            "call_outcome":call_outcome,"actual_wrapper_root_invocations":1,
            "internal_dispatch_or_read_call_counts_observed":false,"physics_executed":false,
            "source_only_context_observation":{"typical_consumed_fields":context.typical_consumed_fields,
                "ground_consumed_fields":context.ground_consumed_fields,
                "unimplemented_computed_state":context.unimplemented_computed_state},
        }),
    )
}
