//! Actual existing public calls, with honest unavailable and error outcomes.
use super::{Result, consumer, fields, inputs};
use serde_json::{Value, json};
use std::path::Path;

fn operation(
    input: &Value,
    invoked: bool,
    api: Option<&str>,
    status: &str,
    error: Option<String>,
    before: Value,
    after: Value,
) -> Value {
    json!({"id":input["id"],"kind":input["kind"],"actual_rust_invoked":invoked,
        "actual_public_API":api,"call_outcome":{"status":status,"error_message":error},
        "before":before,"after":after})
}

pub(super) fn model_case(root: &Path, case: &Value, companion: &Value) -> Result<Value> {
    let operations = inputs::array(&case["operations"])?;
    let (idf_path, idf_bytes) = inputs::bound_file(root, &case["input"])?;
    let mut result = json!({"id":case["id"],"lane":case["lane"],"input":case["input"],
        "native_declared_caller":case["caller"],"input_conversion_available":companion["input_conversion_available"],
        "epjson_input":companion["epjson_input"],"input_conversion_unavailable_reason":companion["input_conversion_unavailable_reason"],
        "operations":[],"final_state":{"raw_model_available":false,"typed_model_available":false,
            "raw_model":null,"compile_report":null,"typed_model":null},
        "selected_consumer_handoff":null,"Native_partial_graph_reconstructed":false});
    if !inputs::flag(&companion["input_conversion_available"])? {
        result["operations"] = json!([
            operation(
                &operations[0],
                false,
                None,
                "unavailable-converted-input",
                None,
                Value::Null,
                Value::Null
            ),
            operation(
                &operations[1],
                false,
                None,
                "source_not_invoked_without_raw_model",
                None,
                Value::Null,
                Value::Null
            )
        ]);
        return Ok(result);
    }
    let (epjson_path, epjson_bytes) = inputs::bound_file(root, &companion["epjson_input"])?;
    let raw = match ep_raw_model::load_epjson_file_with_idf_order(&epjson_path, &idf_path) {
        Ok(value) => value,
        Err(error) => {
            result["operations"] = json!([
                operation(
                    &operations[0],
                    true,
                    Some("ep_raw_model::load_epjson_file_with_idf_order"),
                    "source_error_returned",
                    Some(error.to_string()),
                    Value::Null,
                    Value::Null
                ),
                operation(
                    &operations[1],
                    false,
                    None,
                    "source_not_invoked_without_raw_model",
                    None,
                    Value::Null,
                    Value::Null
                )
            ]);
            inputs::require(
                std::fs::read(&idf_path)? == idf_bytes
                    && std::fs::read(&epjson_path)? == epjson_bytes,
                "loader inputs changed",
            )?;
            return Ok(result);
        }
    };
    let raw_snapshot = fields::raw(&raw)?;
    let compiled = ep_compiler::compile_raw_model(&raw);
    let report = fields::report(&compiled.report);
    let (typed_model, handoff) = match compiled.model.as_ref() {
        Some(model) => (fields::model(model), consumer::handoff(model)?),
        None => (Value::Null, Value::Null),
    };
    let final_state = json!({"raw_model_available":true,
        "typed_model_available":compiled.model.is_some(),"compilation_has_errors":compiled.has_errors(),
        "raw_model":raw_snapshot,"compile_report":report,"typed_model":typed_model,
        "compiler_model_timestep_matches_native_declared_caller":compiled.model.as_ref().map(|model|
            json!(model.timestep.number_of_timesteps_per_hour) == case["caller"]["TimeStepsInHour"]),
        "partial_compiler_model_fabricated":false});
    result["operations"] = json!([
        operation(
            &operations[0],
            true,
            Some("ep_raw_model::load_epjson_file_with_idf_order"),
            "source_returned",
            None,
            Value::Null,
            json!({"raw_model_available":true,"raw_model":raw_snapshot})
        ),
        operation(
            &operations[1],
            true,
            Some("ep_compiler::compile_raw_model"),
            "source_returned",
            None,
            json!({"raw_model_available":true,"raw_model":raw_snapshot}),
            final_state.clone()
        )
    ]);
    result["final_state"] = final_state;
    result["selected_consumer_handoff"] = handoff;
    inputs::require(
        std::fs::read(&idf_path)? == idf_bytes && std::fs::read(&epjson_path)? == epjson_bytes,
        "public-call inputs changed",
    )?;
    Ok(result)
}
