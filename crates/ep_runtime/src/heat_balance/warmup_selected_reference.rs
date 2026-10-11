//! Ignored crate-owned observer: real private selected methods, no Native answers.
use super::*;
use serde_json::{Value, json};
use std::fs::{self, OpenOptions};
use std::io::Write;

fn required<'a>(v: &'a Value, name: &str) -> Result<&'a Value, String> {
    v.get(name)
        .ok_or_else(|| format!("missing literal input field: {name}"))
}

fn number(v: &Value) -> Result<f64, String> {
    let fields = v.as_object().ok_or("literal scalar object required")?;
    let token = required(v, "bits")?
        .as_str()
        .ok_or("literal bit string required")?;
    if fields.len() != 1
        || token.len() != 16
        || !token
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        return Err("canonical literal binary64 bits required".into());
    }
    let value = f64::from_bits(u64::from_str_radix(token, 16).map_err(|e| e.to_string())?);
    if !value.is_finite() {
        return Err("finite direct input required".into());
    }
    Ok(value)
}

fn positive_u32(v: &Value) -> Result<u32, String> {
    let value = u32::try_from(v.as_u64().ok_or("nonnegative integer required")?)
        .map_err(|e| e.to_string())?;
    if value == 0 {
        return Err("positive direct clock/day value required".into());
    }
    Ok(value)
}

fn scalar(v: f64) -> Value {
    let class = if v.is_nan() {
        "nan"
    } else if v == f64::INFINITY {
        "positive_infinity"
    } else if v == f64::NEG_INFINITY {
        "negative_infinity"
    } else if v == 0.0 {
        if v.is_sign_negative() {
            "negative_zero"
        } else {
            "positive_zero"
        }
    } else {
        "finite"
    };
    json!({"value": if v.is_finite() { Some(v) } else { None }, "value_bits": format!("{:016x}", v.to_bits()), "value_class": class})
}

fn snapshot(owner: &HeatBalanceWarmupConvergenceState) -> Value {
    let zones: Vec<Value> = owner.zones.iter().map(|z| json!({
        "actual_Rust_direct_zone_id": z.zone_id.0,
        "selected_owners": {
            "MaxHeatLoadZone": scalar(z.current[0]), "MaxCoolLoadZone": scalar(z.current[1]),
            "MaxTempZone": scalar(z.current[2]), "MinTempZone": scalar(z.current[3]),
            "MaxHeatLoadPrevDay": scalar(z.previous[0]), "MaxCoolLoadPrevDay": scalar(z.previous[1]),
            "MaxTempPrevDay": scalar(z.previous[2]), "MinTempPrevDay": scalar(z.previous[3]),
            "PassFlag": z.pass_flags, "TestMaxTempValue": scalar(z.test_values[0]),
            "TestMinTempValue": scalar(z.test_values[1]), "TestMaxHeatLoadValue": scalar(z.test_values[2]),
            "TestMaxCoolLoadValue": scalar(z.test_values[3])
        },
        "unpaired_actual_Rust_fields": {
            "air_system_sensible_rates_w": z.air_system_sensible_rates_w.map(|(h,c)| json!([scalar(h),scalar(c)])),
            "sensible_report_timestep": z.sensible_report_timestep
        }
    })).collect();
    json!({"zones": zones, "WarmupFlag": owner.warmup_flag,
        "unpaired_actual_Rust_fields": {"checked_day": owner.checked_day,
            "convergence_checks_failed": owner.convergence_checks_failed,
            "maximum_day_forced_exit": owner.maximum_day_forced_exit,
            "unavailable_reason": owner.unavailable_reason},
        "unavailable_Native_owner_families": ["rolling6", "WarmupTempDiff", "WarmupLoadDiff",
            "TempZoneRpt", "LoadZoneRpt", "MaxLoadZoneRpt", "TempZoneRptStdDev", "LoadZoneRptStdDev",
            "CountWarmupDayPoints", "warning_and_report_header_flags", "sensible_report_producer_and_energies",
            "error_backend_and_callbacks", "error_console_and_EIO_prefixes", "ordinary_weather_handoff"],
        "unavailable_counted_as_PASS": false})
}

fn evaluate(case: &Value) -> Result<Value, String> {
    let initial = required(case, "initial")?;
    let names = required(initial, "zone_names")?
        .as_array()
        .ok_or("complete zone-name array required")?;
    let options = HeatBalanceWarmupOptions {
        enabled: true,
        minimum_days: positive_u32(required(initial, "MinNumberOfWarmupDays")?)?,
        maximum_days: positive_u32(required(initial, "MaxNumberOfWarmupDays")?)?,
        temperature_convergence_tolerance_delta_c: number(required(initial, "TempConvergTol")?)?,
        loads_convergence_tolerance_w: number(required(initial, "LoadsConvergTol")?)?,
    };
    if options.minimum_days > options.maximum_days
        || options.temperature_convergence_tolerance_delta_c < 0.0
        || options.loads_convergence_tolerance_w < 0.0
    {
        return Err("unadmitted min/max/tolerance inputs".into());
    }
    let mut zones = Vec::with_capacity(names.len());
    for index in 0..names.len() {
        zones.push(HeatBalanceWarmupZoneConvergence::new(ep_model::ZoneId(
            u32::try_from(index).map_err(|e| e.to_string())?,
        )));
    }
    // Same selected constructor used by the real warmup loop; no observed state is seeded.
    let mut owner = HeatBalanceWarmupConvergenceState {
        zones,
        checked_day: 0,
        convergence_checks_failed: false,
        warmup_flag: true,
        maximum_day_forced_exit: false,
        unavailable_reason: None,
    };
    let constructed = snapshot(&owner);
    let mut operations = Vec::new();
    for op in required(case, "operations")?
        .as_array()
        .ok_or("ordered operations required")?
    {
        let kind = required(op, "operation")?
            .as_str()
            .ok_or("operation name required")?;
        let observations = required(op, "observations")?
            .as_array()
            .ok_or("complete ordered observations required")?;
        let day = positive_u32(required(required(op, "clock")?, "DayOfSim")?)?;
        let before = snapshot(&owner);
        let (status, invoked, calls) = match kind {
            "record" => {
                if observations.len() != owner.zones.len() {
                    return Err("incomplete direct record operands".into());
                }
                for (index, (zone, observed)) in
                    owner.zones.iter_mut().zip(observations).enumerate()
                {
                    let native_index = required(observed, "native_zone_index")?
                        .as_u64()
                        .ok_or("Native unit index required")?;
                    let rust_id = required(observed, "Rust_direct_zone_id")?
                        .as_u64()
                        .ok_or("Rust direct unit ID required")?;
                    if native_index != u64::try_from(index + 1).map_err(|e| e.to_string())?
                        || rust_id != u64::from(zone.zone_id.0)
                    {
                        return Err("explicit direct unit association/order differs".into());
                    }
                    let ztav = number(required(observed, "ZTAV")?)?;
                    let heat = number(required(observed, "heat_rate")?)?;
                    let cool = number(required(observed, "cool_rate")?)?;
                    if heat < 0.0 || cool < 0.0 {
                        return Err("nonnegative sensible input rates required".into());
                    }
                    zone.record(ztav, heat, cool);
                }
                ("selected-private-method-returned", true, owner.zones.len())
            }
            "check" => {
                if !observations.is_empty() {
                    return Err("Check does not take record operands".into());
                }
                owner.check_day(day, options);
                ("selected-private-method-returned", true, 1)
            }
            "report" => {
                if !observations.is_empty() {
                    return Err("Report does not take record operands".into());
                }
                ("unavailable-no-Rust-rolling-report-backend-owner", false, 0)
            }
            _ => return Err("unknown operation; no silent skip".into()),
        };
        operations.push(
            json!({"operation_id": required(op,"operation_id")?, "operation": kind,
            "literal_input": op, "status": status, "actual_selected_method_invoked": invoked,
            "actual_selected_method_call_count": calls, "before": before, "after": snapshot(&owner),
            "Native_sensible_report_or_ordinary_record_state_invoked": false,
            "Native_endpoint_or_branch_result_read": false, "numerical_PASS_claimed": false}),
        );
    }
    Ok(
        json!({"case_id": required(case,"case_id")?, "literal_initial_input": initial,
        "constructed_selected_owners": constructed, "operations": operations, "final": snapshot(&owner),
        "actual_private_zone_constructor_calls": names.len(),
        "direct_unit_IDs_claimed_as_production_IDs": false,
        "producer_or_whole_SYS02_SYS03_PASS_claimed": false}),
    )
}

fn observe() -> Result<(), String> {
    let input_path = std::path::PathBuf::from(
        std::env::var_os("SYS02_REFERENCE_INPUT").ok_or("Root-recorded input operand required")?,
    );
    let output_path = std::path::PathBuf::from(
        std::env::var_os("SYS02_REFERENCE_OUTPUT")
            .ok_or("Root-recorded fresh output operand required")?,
    );
    let bytes = fs::read(&input_path).map_err(|e| e.to_string())?;
    let request: Value = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
    if required(&request, "schema")? != "sys02-warmup-selected-request.v1"
        || required(&request, "expected_values_supplied")? != false
        || required(&request, "Native_results_supplied")? != false
    {
        return Err("literal companion request required; no expected/result packet".into());
    }
    let mut ids = std::collections::BTreeSet::new();
    let mut cases = Vec::new();
    for case in required(&request, "cases")?
        .as_array()
        .ok_or("complete ordered cases required")?
    {
        let id = required(case, "case_id")?
            .as_str()
            .ok_or("case identity required")?;
        if !ids.insert(id) {
            return Err("duplicate case identity".into());
        }
        cases.push(evaluate(case)?);
    }
    let result = json!({"schema": "sys02-warmup-selected-results.v1", "cases": cases,
        "actual_input_path": input_path, "actual_input_size_bytes": bytes.len(),
        "full_input_SHA_verified_internally": false,
        "external_full_input_SHA_and_Source_capture_before_after_required": true,
        "all_declared_operations_retained": true,
        "Native_pre_Record_operand_and_initial_owner_join_required_before_pairing": true,
        "SNLoad_to_rate_equations_in_observer": false,
        "Native_results_or_expected_values_read": false,
        "production_call_path_or_whole_SYS02_SYS03_PASS_claimed": false,
        "scientific_PASS_claimed": false, "gates_updated": false});
    let parent = output_path
        .parent()
        .ok_or("fresh output parent required")?
        .canonicalize()
        .map_err(|e| e.to_string())?;
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .map_err(|e| e.to_string())?;
    let runtime_root = root
        .join(".runtime/porting/SYS-02")
        .canonicalize()
        .map_err(|e| e.to_string())?;
    if !parent.starts_with(runtime_root) {
        return Err("output must stay in ignored SYS02 runtime evidence".into());
    }
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&output_path)
        .map_err(|e| e.to_string())?;
    serde_json::to_writer(&mut output, &result).map_err(|e| e.to_string())?;
    output.write_all(b"\n").map_err(|e| e.to_string())?;
    output.flush().map_err(|e| e.to_string())?;
    if fs::read(&input_path).map_err(|e| e.to_string())? != bytes {
        return Err("input bytes changed during observation".into());
    }
    Ok(())
}

#[test]
#[ignore = "Root-only input observation with explicit recorded input/output and Source custody"]
fn observe_literal_selected_warmup_methods() -> Result<(), String> {
    observe()
}
