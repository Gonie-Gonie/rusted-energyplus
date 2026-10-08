//! One opened header/record stream with canaries reset before every raw call.

use super::{Result, digest, dto, headers, inputs, records};
use ep_runtime::weather::raw::{
    HeaderCallerPreparation, HeaderContextConsumption, RawEpwHeaderState, RawEpwInput,
    open_epw_header,
};
use serde_json::{Value, json};
use std::path::Path;

pub(super) fn run(item: &Value, root: &Path) -> Result<Value> {
    inputs::require(
        inputs::flag(&item["ProcessHeader"])?
            && inputs::flag(&item["same_owner_for_all_raw_records"])?
            && inputs::flag(&item["reset_declared_output_canaries_before_each_call"])?
            && item["physics_executed"] == false,
        "fixed EPW lifecycle policy differs",
    )?;
    let mut state = RawEpwHeaderState::default();
    let constructor = dto::header(&state);
    let prepared = dto::header(&state);
    let mut errors = inputs::flag(&item["initial_ErrorsFound"])?;
    let errors_before = errors;
    let (path, bytes) = inputs::file(root, &item["file"])?;
    let file_path = path.strip_prefix(root)?;
    let actual_file =
        json!({"path":file_path,"sha256":digest::sha256(&bytes),"size_bytes":bytes.len()});
    let mut input = RawEpwInput::new_unopened(bytes);
    let stream_before = dto::stream(&input, Path::new(""));
    let mut context = HeaderContextConsumption::default();
    let open_call_outcome = headers::outcome(open_epw_header(
        &mut input,
        &mut state,
        &mut errors,
        HeaderCallerPreparation::default(),
        &mut context,
    ))?;
    let header_after = dto::header(&state);
    let stream_after_header = dto::stream(&input, file_path);
    let mut records = Vec::new();
    let mut read_attempts = 0usize;
    let mut all_returned = open_call_outcome["source_fatal"] == false;
    let mut terminal_read = Value::Null;
    if all_returned {
        loop {
            let before_read = dto::stream(&input, file_path);
            let read = input
                .read_line()
                .map_err(|error| format!("fixed EPW read failed: {error:?}"))?;
            read_attempts += 1;
            let after_read = dto::stream(&input, file_path);
            if !read.read_good {
                terminal_read = json!({"read_ordinal":read_attempts,"data":read.data,"eof":read.eof,
                    "good":read.read_good,"stream_before":before_read,"stream_after":after_read});
                break;
            }
            let mut row = records::call(&mut state, &item["initial_outputs"], &read.data)?;
            row["record_index"] = json!(records.len());
            row["read_ordinal"] = json!(read_attempts);
            row["input_line_utf8"] = json!(read.data);
            row["input_line_sha256"] = json!(digest::sha256(read.data.as_bytes()));
            row["read_eof"] = json!(read.eof);
            row["read_good"] = json!(read.read_good);
            row["stream_before_read"] = before_read;
            row["stream_after_read"] = after_read;
            if row["call_outcome"]["source_fatal"] == true {
                all_returned = false;
            }
            records.push(row);
        }
    }
    inputs::require(
        u64::try_from(records.len())?
            == item["record_count"]
                .as_u64()
                .ok_or("declared fixed record_count must be integer")?,
        "actual fixed raw record count differs",
    )?;
    Ok(
        json!({"case_id":inputs::text(&item["case_id"])?,"route":item["route"],
            "input_declared_binding":item["file"],"actual_input_file":actual_file,
            "constructor":constructor,"prepared":prepared,"header_after":header_after,
            "ErrorsFound_before":errors_before,"ErrorsFound_after":errors,"open_call_outcome":open_call_outcome,
            "stream_before_open":stream_before,"stream_after_header":stream_after_header,
            "record_count":records.len(),"records":records,"terminal_read":terminal_read,
            "actual_wrapper_readLine_invocations_after_open":read_attempts,
            "actual_wrapper_raw_parser_invocations":records.len(),"actual_wrapper_open_invocations":1,
            "all_records_returned_normally":all_returned,"final_state":dto::header(&state),
            "final_stream":dto::stream(&input,file_path),"physics_executed":false,
            "civil_calendar_supplied_to_raw_parser":false,"whole_simulation_weather_admission_claimed":false,
        }),
    )
}
