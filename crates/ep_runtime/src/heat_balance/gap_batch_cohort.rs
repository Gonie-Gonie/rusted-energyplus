//! Existing process-main cohort, exactly one selected scientific group.
//! Root authenticates the external finite fixture before/after this process.
mod surface;
use serde_json::Value;
use std::{io,path::Path};
type Result<T=()> = std::result::Result<T,Box<dyn std::error::Error>>;
fn invalid(message:&str)->io::Error {io::Error::new(io::ErrorKind::InvalidData,message)}
pub(crate) fn run(group:&str,fixture_path:&Path)->Result<usize> {
    if !matches!(group,"purchased_air"|"surface"|"exterior"|"output_processor"|"air_history") {
        return Err(invalid("selected implemented whole scientific group").into());
    }
    // Runtime loading is test-feature-only. No production input or expected
    // value enters a live owner from this document except declared INPUTS.
    let fixture:Value=serde_json::from_slice(&std::fs::read(fixture_path)?)?;
    if fixture.get("schema").and_then(Value::as_str)!=Some("src06-Native-gap-batch-fixture.v1") {
        return Err(invalid("authenticated original-core fixture schema").into());
    }
    let named=fixture.get("named_inputs").ok_or_else(||invalid("complete named primary inputs"))?;
    let native=fixture.get("native_output").ok_or_else(||invalid("complete actual Native selected-group output"))?;
    if named.get("schema").and_then(Value::as_str)!=Some("src06-original-core-gap-batch-input.v1")
        || native.get("schema").and_then(Value::as_str)!=Some("src06-original-core-gap-batch-output.v1")
        || native.get("normal_completion")!=Some(&Value::Bool(true))
        || native.get("group_order")!=Some(&serde_json::json!([group]))
        || native.get("groups").and_then(Value::as_object).is_none_or(|groups|groups.len()!=1||!groups.contains_key(group)) {
        return Err(invalid("one genuine normal completed Native group required").into());
    }
    match group {
        "purchased_air"=>super::ordinary_equipment::OrdinaryEquipment::run_native_gap_purchased_air_cohort(&fixture),
        "surface"=>surface::run(&fixture),
        "air_history"=>super::ordinary_equipment::OrdinaryEquipment::run_native_gap_air_history_cohort(&fixture),
        "output_processor"=>super::ordinary_equipment::OrdinaryEquipment::run_native_gap_output_processor_cohort(&fixture),
        "exterior"=>Err(invalid("Source Exterior adapter literal join pending").into()),
        _=>Err(invalid("unreachable selected group").into()),
    }
}
