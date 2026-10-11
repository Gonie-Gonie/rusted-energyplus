//! Genuine six direct-state Init calls; no production geometry or Native answers.
#[path = "ctf10_init_postlude_direct_support/compile_report_inputs.rs"]
mod compile_report_inputs;
#[path = "../../ep_runtime/examples/clk02_probe_support/digest.rs"]
mod digest;
#[path = "ctf10_init_postlude_direct_support/fields.rs"]
mod fields;
use ep_model::{ConstructionId, MaterialId, MaterialSurfaceRoughness};
use ep_runtime::heat_balance::ctf_direct_initialization::{
    CtfDirectInitializationState, CtfDirectRegularConstructionInput,
};
use ep_runtime::heat_balance::surface_manager::ctf_layer_preprocessing::CtfLayerInput;
use serde_json::{Value, json};
use std::path::{Path, PathBuf};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn require(ok: bool, message: &str) -> Result<()> {
    if ok {
        Ok(())
    } else {
        Err(message.to_owned().into())
    }
}
fn text(value: &Value) -> Result<&str> {
    value.as_str().ok_or_else(|| "typed string required".into())
}
fn rows(value: &Value) -> Result<&[Value]> {
    value
        .as_array()
        .map(Vec::as_slice)
        .ok_or_else(|| "typed array required".into())
}
fn keys(value: &Value, expected: &[&str]) -> Result<()> {
    let object = value.as_object().ok_or("typed object required")?;
    require(
        object.len() == expected.len() && expected.iter().all(|key| object.contains_key(*key)),
        "exact input-only fields required",
    )
}
fn bound(root: &Path, binding: &Value) -> Result<(PathBuf, Vec<u8>)> {
    let relative = Path::new(text(&binding["path"])?);
    require(
        relative.is_relative(),
        "relative input/source path required",
    )?;
    let path = root.join(relative).canonicalize()?;
    require(
        path.starts_with(root) && path != root && path.is_file(),
        "contained regular input/source required",
    )?;
    require(
        std::fs::metadata(&path)?.len() <= 64 * 1024 * 1024,
        "bounded input/source file required",
    )?;
    let bytes = std::fs::read(&path)?;
    require(
        digest::sha256(&bytes) == text(&binding["sha256"])?,
        "bound input/source SHA differs",
    )?;
    for key in ["bytes", "size_bytes"] {
        if let Some(size) = binding.get(key) {
            require(
                size.as_u64() == Some(u64::try_from(bytes.len())?),
                "bound byte size differs",
            )?;
        }
    }
    Ok((path, bytes))
}
fn source_pins(root: &Path, supplied: &Value, expected: &[&str]) -> Result<()> {
    let supplied = rows(supplied)?;
    require(
        supplied.len() == expected.len(),
        "source inventory length differs",
    )?;
    for (binding, path) in supplied.iter().zip(expected) {
        require(
            text(&binding["path"])? == *path,
            "candidate source path/order differs",
        )?;
        bound(root, binding)?;
    }
    Ok(())
}

fn boolean(value: &Value) -> Result<bool> {
    value
        .as_bool()
        .ok_or_else(|| "actual boolean input required".into())
}
fn real(value: &Value) -> Result<f64> {
    let value = value
        .as_f64()
        .ok_or("actual finite numeric input required")?;
    require(value.is_finite(), "finite literal direct input required")?;
    Ok(value)
}
fn direct_case(root: &Path, row: &Value, idf: &Value, epjson: &Value) -> Result<Value> {
    let mut out = json!({"id":row["id"],"lane":"explicit-direct-state-Rust-Init-unit","actual_input_row":row,
        "actual_raw_input":idf,"actual_report_epjson":epjson,"loader":null,"compiler":null,
        "typed_report_inputs":null,"preparation":null,"initial_registry":null,"setup_calls":[],
        "direct_initializer_invoked":false,"direct_initializer_invocation_count":0,
        "caller_before_initializer":null,"caller_after_return_or_error":null,
        "direct_state_before_initializer":null,"direct_state_after_return_or_error":null,
        "initialization":null,"Native_preparation_calls_replayed":false,"Native_outcome_inferred":false,"counted_as_PASS":false});
    let (idf_path, _) = bound(root, idf)?;
    let (epjson_path, _) = bound(root, epjson)?;
    let raw = match ep_raw_model::load_epjson_file_with_idf_order(&epjson_path, &idf_path) {
        Ok(raw) => {
            out["loader"] = json!({"invoked":true,"status":"source-returned"});
            raw
        }
        Err(error) => {
            out["loader"] = json!({"invoked":true,"status":"source-error-returned","message":error.to_string()});
            return Ok(out);
        }
    };
    let (prepared, report) = compile_report_inputs::compile_direct_report_inputs(&raw);
    out["compiler"] = json!({"invoked":true,"model_available":prepared.is_some(),"raw_object_count":report.raw_object_count,
        "typed_object_count":report.typed_object_count,"completed_stages":format!("{:?}",report.completed_stages),
        "ordered_diagnostics":format!("{:?}",report.diagnostics),"defaults_applied":format!("{:?}",report.defaults_applied),
        "only_ReportInputs_moved_out":false,"typed_simulation_geometry_or_timestep_used":true,"typed_simulation_geometry_used":false,"actual_compiled_Timestep_retained_for_GetProjectData":true});
    let Some(prepared) = prepared else {
        out["initialization"] =
            json!({"invoked":false,"status":"unavailable-genuine-report-input-compilation"});
        return Ok(out);
    };
    compile_report_inputs::admit_project_defaults(&raw, &prepared)?;
    let inputs = &prepared.reports;
    out["typed_report_inputs"] = fields::typed_report_inputs(inputs);
    let mut caller = fields::Caller::new();
    let preparation_before = caller.snapshot();
    match caller.prepare(&prepared) {
        Ok(assigned) => {
            out["preparation"] = json!({"invoked":true,"status":"source-returned",
            "source_order":["SimulationManager::OpenOutputFiles","SimulationManager::GetProjectData","EMSManager::CheckIfAnyEMS -> ScanForReports(EnergyManagementSystem)"],
            "actual_TimeStepsInHour":prepared.time_steps_in_hour,"actual_TimeStepZone":fields::scalar(prepared.time_step_zone()),
            "assigned_EMS_report":assigned,"caller_before":preparation_before,"caller_after":caller.snapshot()})
        }
        Err(error) => {
            out["preparation"] = json!({"invoked":true,"status":"source-error-returned","error":fields::setup_error(&error),
                "caller_before":preparation_before,"caller_after":caller.snapshot()});
            return Ok(out);
        }
    }
    let registry_before = caller.snapshot();
    match caller.setup("SetPredefinedTables", inputs) {
        Ok(assigned) => {
            out["initial_registry"] = json!({"invoked":true,"status":"source-returned","assigned_report":assigned,
            "caller_before":registry_before,"caller_after":caller.snapshot()})
        }
        Err(error) => {
            out["initial_registry"] = json!({"invoked":true,"status":"source-error-returned","error":fields::setup_error(&error),
            "caller_before":registry_before,"caller_after":caller.snapshot()});
            return Ok(out);
        }
    }
    let m = &row["material"];
    let c = &row["construction"];
    require(
        c["OutsideRoughness"].as_i64() == Some(2),
        "Source literal ordinal2 MediumRough required",
    )?;
    let input = CtfDirectRegularConstructionInput {
        construction_id: ConstructionId(1),
        construction_name: text(&c["Name"])?,
        material_name: text(&m["Name"])?,
        layer: CtfLayerInput {
            material_id: MaterialId(1),
            thickness: real(&m["Thickness"])?,
            conductivity: real(&m["Conductivity"])?,
            density: real(&m["Density"])?,
            specific_heat: real(&m["SpecHeat"])?,
            resistance: real(&m["Resistance"])?,
            resistance_only: boolean(&m["ROnly"])?,
        },
        is_used_ctf: boolean(&c["IsUsedCTF"])?,
        source_sink_present: false,
        solution_dimensions: 1,
        absorptance: [0.0; 4],
        outside_roughness: MaterialSurfaceRoughness::MediumRough,
        time_step_zone: prepared.time_step_zone(),
        any_internal_heat_source_in_input: false,
    };
    let mut state = CtfDirectInitializationState::new(true, 0);
    out["actual_direct_input_context"] = json!({"construction_id":1,"material_id":1,"group":"Regular","TotLayers":1,"LayerPoint":[1],
        "SourceSinkPresent":input.source_sink_present,"SolutionDimensions":input.solution_dimensions,
        "absorptance":input.absorptance.map(fields::scalar),"OutsideRoughness":"MediumRough",
        "actual_global_TimeStepZone":fields::scalar(input.time_step_zone),"AnyInternalHeatSourceInInput":input.any_internal_heat_source_in_input,
        "constructor_defaults_not_compiled_Timestep_used":false,"actual_GetProjectData_Timestep_used":true,"production_Rust_ID_equivalence_claimed":false});
    let mut setup = Vec::new();
    for action in rows(&row["setup_calls"])? {
        let action = text(action)?;
        let before = caller.snapshot();
        match caller.setup(action,inputs) {
            Ok(assigned)=>setup.push(json!({"action":action,"actual_source_invoked":true,"status":"source-returned","assigned_report":assigned,
                "caller_before":before,"caller_after":caller.snapshot()})),
            Err(error)=>{
                setup.push(json!({"action":action,"actual_source_invoked":true,"status":"source-error-returned","error":fields::setup_error(&error),
                    "caller_before":before,"caller_after":caller.snapshot()}));out["setup_calls"]=json!(setup);
                out["direct_state_after_return_or_error"]=fields::direct_state(&state);out["caller_after_return_or_error"]=caller.snapshot();return Ok(out);
            },
        }
    }
    out["setup_calls"] = json!(setup);
    out["caller_before_initializer"] = caller.snapshot();
    out["direct_state_before_initializer"] = fields::direct_state(&state);
    out["direct_initializer_invoked"] = json!(true);
    out["direct_initializer_invocation_count"] = json!(1);
    let actual = caller.initialize(&input, inputs, &mut state);
    out["initialization"] = match actual {
        Ok(()) => json!({"invoked":true,"status":"source-returned","error":null}),
        Err(error) => {
            json!({"invoked":true,"status":"source-error-returned","error":fields::error(&error)})
        }
    };
    out["caller_after_return_or_error"] = caller.snapshot();
    out["direct_state_after_return_or_error"] = fields::direct_state(&state);
    Ok(out)
}
fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(2);
    }
}
fn run() -> Result<()> {
    let mut args = std::env::args_os().skip(1);
    let request_path = PathBuf::from(
        args.next()
            .ok_or("input-only direct observer request required")?,
    )
    .canonicalize()?;
    let request_sha = args
        .next()
        .ok_or("request SHA required")?
        .into_string()
        .map_err(|_| "UTF-8 request SHA required")?;
    require(args.next().is_none(), "exact request path and SHA required")?;
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .ok_or("repository root unavailable")?
        .canonicalize()?;
    require(
        request_path.starts_with(&root)
            && request_path.is_file()
            && std::fs::metadata(&request_path)?.len() <= 1024 * 1024,
        "bounded contained request required",
    )?;
    let original = std::fs::read(&request_path)?;
    require(
        digest::sha256(&original) == request_sha,
        "request SHA differs",
    )?;
    let request: Value = serde_json::from_slice(&original)?;
    keys(
        &request,
        &[
            "schema",
            "direct_unit_request",
            "native_admission_contract",
            "native_frozen_packet",
            "report_input_epjson",
            "report_conversion_proof_bindings",
            "observer_sources",
            "actual_public_API_sources",
            "actual_Cargo_source_capture_must_match",
            "frozen_before_execution",
            "expected_values_supplied",
            "Native_results_supplied",
        ],
    )?;
    require(
        request["schema"] == "ctf10-Init-postlude-candidate-direct-observer-request.v1"
            && request["actual_Cargo_source_capture_must_match"] == true
            && request["frozen_before_execution"] == true
            && request["expected_values_supplied"] == false
            && request["Native_results_supplied"] == false,
        "input-only direct request required",
    )?;
    source_pins(
        &root,
        &request["observer_sources"],
        &fields::observer_paths(),
    )?;
    source_pins(
        &root,
        &request["actual_public_API_sources"],
        &fields::api_paths(),
    )?;
    let (_, freeze_bytes) = bound(&root, &request["native_frozen_packet"])?;
    let freeze: Value = serde_json::from_slice(&freeze_bytes)?;
    let (_, direct_bytes) = bound(&root, &request["direct_unit_request"])?;
    let direct: Value = serde_json::from_slice(&direct_bytes)?;
    let (_, admission_bytes) = bound(&root, &request["native_admission_contract"])?;
    let admission: Value = serde_json::from_slice(&admission_bytes)?;
    require(
        freeze["schema"] == "ctf10-native-Init-postlude-frozen-inputs.v1"
            && freeze["frozen_before_this_execution"] == true
            && freeze["expected_values_supplied"] == false
            && freeze["expected_exits_supplied"] == false
            && freeze["scientific_execution_performed"] == false
            && freeze["numerical_PASS_claimed"] == false
            && freeze["gates_updated"] == false
            && freeze["declared_direct_cases"] == 6
            && freeze["direct_unit_request"] == request["direct_unit_request"]
            && freeze["native_admission_contract"] == request["native_admission_contract"],
        "Native freeze/direct admission custody differs",
    )?;
    require(
        direct["schema"] == "ctf10-Init-postlude-direct-unit-request.v1"
            && direct["frozen_before_this_helper_execution"] == true
            && direct["expected_values_supplied"] == false
            && direct["expected_exits_supplied"] == false
            && direct["scientific_execution_performed"] == false
            && admission["schema"] == "ctf10-production-native-admission-contract.v1"
            && admission["direct_unit_request"] == request["direct_unit_request"]
            && admission["actual_scope_amendment_execution_role"]
                == "real-CTF-10-Init-postlude-check-only",
        "frozen exact direct source request required",
    )?;
    require(
        freeze["native_source_manifest"]["sha256"]
            == "9a664b1a4ac99b8b6b00813aaa43a59bb61bab24b3752ebaf06d22324d29160b",
        "reviewed Native06 Source required",
    )?;
    let (_, native_bytes) = bound(&root, &freeze["native_source_manifest"])?;
    let native: Value = serde_json::from_slice(&native_bytes)?;
    require(
        admission["native_source_files"] == native["native_source_files"]
            && rows(&admission["native_source_files"])?.len() == 12,
        "exact Native twelve Source refs required",
    )?;
    require(
        direct["literal_case_source"]["sha256"]
            == "020c6bf53a8fd6bf49caafd1bd74eb3dae45b5272cdd6d27dade57392daf8b08",
        "reviewed six literal inputs required",
    )?;
    let (_, literal_bytes) = bound(&root, &direct["literal_case_source"])?;
    let literal: Value = serde_json::from_slice(&literal_bytes)?;
    require(
        literal["schema"] == "ctf10-Init-postlude-direct-unit-cases-proposed.v1"
            && literal["cases"] == direct["cases"]
            && literal["expected_values_supplied"] == false
            && literal["expected_exits_supplied"] == false
            && rows(&direct["cases"])?.len() == 6
            && direct["input"]["sha256"] == literal["literal_IDF_source"]["sha256"]
            && direct["input"]["size_bytes"] == literal["literal_IDF_source"]["size_bytes"],
        "literal case/input identity differs",
    )?;
    let mut bindings = vec![
        request["native_frozen_packet"].clone(),
        request["direct_unit_request"].clone(),
        request["native_admission_contract"].clone(),
        freeze["native_source_manifest"].clone(),
        direct["literal_case_source"].clone(),
        direct["input"].clone(),
        request["report_input_epjson"].clone(),
        freeze["helper_request"].clone(),
        freeze["projection"].clone(),
        freeze["observation_policy"].clone(),
    ];
    require(
        !rows(&request["report_conversion_proof_bindings"])?.is_empty(),
        "actual input-only conversion proof bindings required",
    )?;
    let mut actual_conversion_count = 0;
    for binding in rows(&request["report_conversion_proof_bindings"])? {
        let (_, bytes) = bound(&root, binding)?;
        if let Ok(proof) = serde_json::from_slice::<Value>(&bytes)
            && proof["schema"] == "sch01-input-conversion-command.v1"
        {
            require(
                proof["case_id"] == "CTF10-DIRECT-REPORT"
                    && proof["exit_code"] == 0
                    && proof["launch_error"].is_null()
                    && proof["epjson_available"] == true
                    && proof["unavailable_reason"].is_null()
                    && proof["available_epjson_input"] == request["report_input_epjson"]
                    && proof["source_input"]["sha256"] == direct["input"]["sha256"]
                    && proof["staged_input"]["sha256"] == direct["input"]["sha256"]
                    && proof["source_input"]["size_bytes"] == direct["input"]["size_bytes"]
                    && proof["staged_input"]["size_bytes"] == direct["input"]["size_bytes"]
                    && proof["original_and_staged_IDF_preserved"] == true
                    && proof["numerical_engine_executed"] == false
                    && proof["Native_scientific_outputs_used_as_inputs"] == false
                    && proof["expected_answers_supplied"] == false
                    && proof["converter"]["path"]
                        == ".runtime/energyplus/26.1.0/ConvertInputFormat.exe"
                    && rows(&proof["command"])?.len() == 2
                    && proof["command"][1] == "input.idf",
                "genuine input-only conversion proof differs",
            )?;
            bindings.push(proof["source_input"].clone());
            bindings.push(proof["staged_input"].clone());
            actual_conversion_count += 1;
        }
    }
    require(
        actual_conversion_count == 1,
        "one actual successful direct report conversion proof required",
    )?;
    bindings.extend(
        rows(&request["report_conversion_proof_bindings"])?
            .iter()
            .cloned(),
    );
    bindings.extend(rows(&admission["native_source_files"])?.iter().cloned());
    for key in ["source", "cases", "tolerances"] {
        require(
            admission["contracts"][key] == freeze["contracts"][key],
            "frozen contract identity differs",
        )?;
        bindings.push(freeze["contracts"][key].clone());
    }
    for binding in &bindings {
        bound(&root, binding)?;
    }
    let cases = rows(&direct["cases"])?
        .iter()
        .map(|row| {
            direct_case(
                &root,
                row,
                &direct["input"],
                &request["report_input_epjson"],
            )
        })
        .collect::<Result<Vec<_>>>()?;
    let invoked = cases
        .iter()
        .filter(|v| v["direct_initializer_invoked"] == true)
        .count();
    let returned = cases
        .iter()
        .filter(|v| v["initialization"]["status"] == "source-returned")
        .count();
    let fatal = cases
        .iter()
        .filter(|v| v["initialization"]["error"]["actual_SourceFatalError_returned"] == true)
        .count();
    for binding in &bindings {
        bound(&root, binding)?;
    }
    source_pins(
        &root,
        &request["observer_sources"],
        &fields::observer_paths(),
    )?;
    source_pins(
        &root,
        &request["actual_public_API_sources"],
        &fields::api_paths(),
    )?;
    require(
        std::fs::read(&request_path)? == original,
        "request changed during direct calls",
    )?;
    println!(
        "{}",
        serde_json::to_string(
            &json!({"schema":"ctf10-Init-postlude-candidate-direct-observer-results.v1",
        "actual_request":{"path":request_path,"sha256":digest::sha256(&original)},"direct_unit_request":request["direct_unit_request"],
        "native_admission_contract":request["native_admission_contract"],"native_frozen_packet":request["native_frozen_packet"],
        "report_input_epjson":request["report_input_epjson"],"report_conversion_proof_bindings":request["report_conversion_proof_bindings"],
        "observer_sources":request["observer_sources"],"actual_public_API_sources":request["actual_public_API_sources"],"direct_unit_cases":cases,
        "actual_direct_initializer_invocations":invoked,"actual_direct_source_returns":returned,"actual_typed_Source_fatal_returns":fatal,
        "all_bound_inputs_unchanged":true,"all_literal_cases_consumed":true,"actual_Cargo_source_capture_must_be_validated_independently":true,
        "ordinary_fixed_CON_or_whole_building_admission_claimed":false,"Native_expected_values_read":false,"Native_outcomes_inferred":false,
        "scientific_comparison_performed":false,"numerical_PASS_claimed":false,"whole_Init_parity_claimed":false,"gates_updated":false})
        )?
    );
    Ok(())
}
