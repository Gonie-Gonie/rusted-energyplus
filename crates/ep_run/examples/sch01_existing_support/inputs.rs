//! Input identity and existing-loader admission; no result or expected operands.
use super::{Result, digest};
use serde_json::{Value, json};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

pub(super) fn require(ok: bool, message: &str) -> Result<()> {
    if ok {
        Ok(())
    } else {
        Err(message.to_owned().into())
    }
}
pub(super) fn text(value: &Value) -> Result<&str> {
    value.as_str().ok_or_else(|| "typed string required".into())
}
pub(super) fn flag(value: &Value) -> Result<bool> {
    value.as_bool().ok_or_else(|| "typed bool required".into())
}
pub(super) fn array(value: &Value) -> Result<&[Value]> {
    value
        .as_array()
        .map(Vec::as_slice)
        .ok_or_else(|| "typed array required".into())
}
fn integer(value: &Value) -> Result<u64> {
    value
        .as_u64()
        .ok_or_else(|| "typed unsigned integer required".into())
}
fn keys(value: &Value, expected: &[&str]) -> Result<()> {
    let actual = value.as_object().ok_or("typed object required")?;
    require(
        actual.len() == expected.len() && expected.iter().all(|name| actual.contains_key(*name)),
        "exact input-only object fields required",
    )
}
pub(super) fn relative(root: &Path, path: &Path) -> Result<String> {
    Ok(path
        .strip_prefix(root)?
        .to_str()
        .ok_or("UTF-8 path required")?
        .replace('\\', "/"))
}
pub(super) fn bound_file(root: &Path, binding: &Value) -> Result<(PathBuf, Vec<u8>)> {
    let path = root.join(text(&binding["path"])?).canonicalize()?;
    require(
        path.starts_with(root) && path != root,
        "bound path escapes repository",
    )?;
    let size = std::fs::metadata(&path)?.len();
    require(
        size <= 64 * 1024 * 1024,
        "bounded input/source file required",
    )?;
    let bytes = std::fs::read(&path)?;
    require(
        digest::sha256(&bytes) == text(&binding["sha256"])?,
        "bound file bytes differ",
    )?;
    for name in ["bytes", "size_bytes"] {
        if let Some(expected) = binding.get(name) {
            require(
                integer(expected)? == u64::try_from(bytes.len())?,
                "bound file size differs",
            )?;
        }
    }
    Ok((path, bytes))
}
pub(super) fn check_all(root: &Path, bindings: &[Value]) -> Result<()> {
    for binding in bindings {
        bound_file(root, binding)?;
    }
    Ok(())
}

pub(super) fn admit(root: &Path, wrapper: &Value) -> Result<(Value, Vec<Value>)> {
    keys(
        wrapper,
        &[
            "schema",
            "native_request",
            "native_admission_contract",
            "native_observation_projection",
            "contracts",
            "model_cases",
            "observer_sources",
            "actual_public_API_sources",
            "frozen_before_existing_Rust_observer_numerical_execution",
            "expected_values_supplied",
            "native_results_supplied",
            "Rust_outputs_supplied_as_inputs",
        ],
    )?;
    require(
        text(&wrapper["schema"])? == "sch01-existing-observer-request.v1"
            && flag(&wrapper["frozen_before_existing_Rust_observer_numerical_execution"])?
            && !flag(&wrapper["expected_values_supplied"])?
            && !flag(&wrapper["native_results_supplied"])?
            && !flag(&wrapper["Rust_outputs_supplied_as_inputs"])?,
        "final input-only existing-observer request required",
    )?;
    let (request_path, request_bytes) = bound_file(root, &wrapper["native_request"])?;
    let native: Value = serde_json::from_slice(&request_bytes)?;
    let (_, admission_bytes) = bound_file(root, &wrapper["native_admission_contract"])?;
    let admission: Value = serde_json::from_slice(&admission_bytes)?;
    require(
        native["schema"] == "sch01-helper-cases.v1"
            && native["energyplus_commit"] == "6f2e40d10250a105b49966baa24d843711e61048"
            && admission["schema"] == "sch01-native-admission-contract.v1"
            && admission["energyplus_commit"] == native["energyplus_commit"]
            && bound_file(root, &admission["helper_request"])?.0 == request_path
            && wrapper["contracts"] == admission["contracts"]
            && wrapper["native_observation_projection"] == admission["projection"],
        "native input-only DAG differs",
    )?;
    for value in [&native, &admission] {
        require(
            flag(&value["frozen_before_numerical_execution"])?
                && !flag(&value["expected_values_supplied"])?
                && !flag(&value["expected_exits_supplied"])?
                && !flag(&value["scientific_execution_performed"])?,
            "native input policy differs",
        )?;
    }
    let mut bindings = vec![
        wrapper["native_request"].clone(),
        wrapper["native_admission_contract"].clone(),
    ];
    for name in [
        "fixed_scope",
        "actual_scope_amendment",
        "actual_scope_amendment_execution",
    ] {
        require(
            native[name] == admission[name],
            "scope/amendment binding differs",
        )?;
        bindings.push(native[name].clone());
    }
    let (_, scope_bytes) = bound_file(root, &native["fixed_scope"])?;
    let scope: Value = serde_json::from_slice(&scope_bytes)?;
    let guards = array(&scope["cases"])?
        .iter()
        .flat_map(|row| {
            [
                row["input"].clone(),
                row["weather"].clone(),
                row["metadata"].clone(),
            ]
        })
        .collect::<Vec<_>>();
    require(
        guards.len() == 45
            && native["all_45_guard_refs"] == json!(guards)
            && admission["all_45_guard_refs"] == native["all_45_guard_refs"],
        "exact ordered CON45 bindings differ",
    )?;
    bindings.extend(guards);
    keys(&admission["contracts"], &["source", "cases", "tolerances"])?;
    keys(&native["contracts"], &["source", "tolerances"])?;
    for name in ["source", "cases", "tolerances"] {
        bindings.push(admission["contracts"][name].clone());
        if name != "cases" {
            require(
                native["contracts"][name] == admission["contracts"][name],
                "contract differs",
            )?;
        }
    }
    let (_, projection_bytes) = bound_file(root, &admission["projection"])?;
    let projection: Value = serde_json::from_slice(&projection_bytes)?;
    require(
        projection["schema"] == "sch01-observation-projection.v1"
            && flag(&projection["frozen_before_numerical_execution"])?
            && projection["helper_request"] == admission["helper_request"]
            && projection["contracts"] == admission["contracts"]
            && projection["native_observation_policy"] == native["observation_policy"],
        "native projection differs",
    )?;
    bindings.push(admission["projection"].clone());
    let cases = array(&native["model_cases"])?;
    let companions = array(&wrapper["model_cases"])?;
    require(
        !cases.is_empty() && cases.len() <= 256 && cases.len() == companions.len(),
        "model case inventory differs",
    )?;
    let counts = json!({"model_cases":cases.len(),"ProcessInput":cases.len(),
        "ProcessScheduleInput":cases.len(),"total_operations":2 * cases.len()});
    require(
        native["requested_counts"] == counts && admission["requested_counts"] == counts,
        "derived requested counts differ",
    )?;
    let mut ids = BTreeSet::new();
    for (case, companion) in cases.iter().zip(companions) {
        keys(case, &["id", "lane", "input", "caller", "operations"])?;
        keys(
            companion,
            &[
                "native_case_id",
                "input",
                "input_conversion_available",
                "epjson_input",
                "input_conversion_unavailable_reason",
            ],
        )?;
        require(
            ids.insert(text(&case["id"])?.to_owned()),
            "duplicate case ID",
        )?;
        require(
            companion["native_case_id"] == case["id"] && companion["input"] == case["input"],
            "conversion case/IDF differs",
        )?;
        require(
            case["lane"] == "production-CON" || case["lane"] == "diagnostic-model",
            "unknown model lane",
        )?;
        let caller = &case["caller"];
        keys(
            caller,
            &[
                "TimeStepsInHour",
                "MinutesInTimeStep",
                "TimeStepZone",
                "isEpJSON",
                "preserveIDFOrder",
            ],
        )?;
        let steps = integer(&caller["TimeStepsInHour"])?;
        require(
            steps > 0
                && steps <= 60
                && 60 % steps == 0
                && integer(&caller["MinutesInTimeStep"])? == 60 / steps
                && !flag(&caller["isEpJSON"])?
                && flag(&caller["preserveIDFOrder"])?,
            "declared IDF caller differs",
        )?;
        keys(&caller["TimeStepZone"], &["bits"])?;
        let bits = text(&caller["TimeStepZone"]["bits"])?;
        require(
            bits.len() == 16
                && bits
                    .bytes()
                    .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)),
            "binary64 caller bits required",
        )?;
        let fraction = f64::from_bits(u64::from_str_radix(bits, 16)?);
        require(
            fraction.to_bits() == (1.0 / steps as f64).to_bits(),
            "exact source timestep fraction required",
        )?;
        let operations = array(&case["operations"])?;
        require(
            operations.len() == 2
                && operations[0]["kind"] == "ProcessInput"
                && operations[1]["kind"] == "ProcessScheduleInput",
            "two-operation parser order differs",
        )?;
        let mut operation_ids = BTreeSet::new();
        for operation in operations {
            keys(operation, &["id", "kind"])?;
            require(
                operation_ids.insert(text(&operation["id"])?.to_owned()),
                "duplicate operation ID",
            )?;
        }
        let (idf_path, _) = bound_file(root, &case["input"])?;
        require(
            idf_path.extension().and_then(|value| value.to_str()) == Some("idf"),
            "actual IDF input required",
        )?;
        if case["lane"] == "production-CON" {
            let mut found = false;
            for existing in array(&scope["cases"])? {
                if existing["input"]["sha256"] == case["input"]["sha256"]
                    && bound_file(root, &existing["input"])?.0 == idf_path
                {
                    found = true;
                }
            }
            require(found, "production input is outside unchanged CON scope")?;
        }
        bindings.push(case["input"].clone());
        if flag(&companion["input_conversion_available"])? {
            require(
                companion["input_conversion_unavailable_reason"].is_null(),
                "available conversion cannot have failure reason",
            )?;
            let (epjson_path, _) = bound_file(root, &companion["epjson_input"])?;
            require(
                epjson_path
                    .extension()
                    .and_then(|value| value.to_str())
                    .is_some_and(|extension| extension.eq_ignore_ascii_case("epjson")),
                "actual converted epJSON input required",
            )?;
            bindings.push(companion["epjson_input"].clone());
        } else {
            require(
                companion["epjson_input"].is_null()
                    && !text(&companion["input_conversion_unavailable_reason"])?.is_empty(),
                "unavailable conversion must remain explicit",
            )?;
        }
    }
    for name in ["observer_sources", "actual_public_API_sources"] {
        let sources = array(&wrapper[name])?;
        require(!sources.is_empty(), "actual source inventory required")?;
        for binding in sources {
            let path = Path::new(text(&binding["path"])?);
            require(
                path.extension()
                    .and_then(|value| value.to_str())
                    .is_some_and(|extension| extension == "rs" || extension == "toml"),
                "only source/config identities admitted here",
            )?;
            bindings.push(binding.clone());
        }
    }
    check_all(root, &bindings)?;
    Ok((native, bindings))
}
