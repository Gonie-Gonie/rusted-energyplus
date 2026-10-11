//! Ignored input-only observer of the real selected setter APIs.
use super::*;
use serde_json::{Value, json};
use std::collections::BTreeSet;
use std::fs::{self, OpenOptions};
use std::io::Write;

fn required<'a>(v: &'a Value, name: &str) -> Result<&'a Value, String> {
    v.get(name)
        .ok_or_else(|| format!("missing literal input field: {name}"))
}
fn keys(v: &Value, names: &[&str]) -> Result<(), String> {
    let fields = v.as_object().ok_or("literal object required")?;
    if fields.len() != names.len() || names.iter().any(|name| !fields.contains_key(*name)) {
        return Err("exact literal input object shape required".into());
    }
    Ok(())
}
fn number(v: &Value, retained_nan: bool) -> Result<f64, String> {
    keys(v, &["bits"])?;
    let token = required(v, "bits")?
        .as_str()
        .ok_or("literal bit string required")?;
    if token.len() != 16
        || !token
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        return Err("canonical16-hex literal input required".into());
    }
    let value = f64::from_bits(u64::from_str_radix(token, 16).map_err(|e| e.to_string())?);
    if !(value.is_finite() || retained_nan && value.is_nan()) {
        return Err("finite active or declared skipped NaN owner required".into());
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
    json!({"value": if v.is_finite() { Some(v) } else { None }, "value_bits": format!("{:016x}",v.to_bits()), "value_class":class})
}
fn parameters(p: SurfaceOutdoorEnvironmentParameters) -> Value {
    json!({"SiteTempGradient":scalar(p.site_temperature_gradient_k_per_m),
        "WeatherFileTempModCoeff":scalar(p.weather_file_temperature_modifier_c),
        "SiteWindExp":scalar(p.site_wind_exponent),"SiteWindBLHeight":scalar(p.site_wind_boundary_layer_height_m),
        "WeatherFileWindModCoeff":scalar(p.weather_file_wind_modifier)})
}
fn snapshot(
    p: SurfaceOutdoorEnvironmentParameters,
    weather: Option<SurfaceOutdoorWeather>,
    rows: &[SurfaceOutdoorEnvironmentRow],
    capacity: usize,
    ids: &[(u64, String)],
) -> Value {
    let values: Vec<Value> = rows.iter().enumerate().map(|(i,r)| json!({
        "Rust_row_index":i,"actual_input_surface_index":ids[i].0,"Name":ids[i].1,
        "Centroid_z":scalar(r.centroid_height_m),"ExtWind":r.wind_exposed,
        "SurfOutDryBulbTemp":scalar(r.dry_bulb_c),"SurfOutWetBulbTemp":scalar(r.wet_bulb_c),
        "SurfOutWindSpeed":scalar(r.wind_speed_m_per_s),"SurfOutWindDir":scalar(r.wind_direction_deg)})).collect();
    json!({"owner":{"parameters":parameters(p),"rows":values,"actual_Rust_Vec_len":rows.len(),"actual_Rust_Vec_capacity":capacity,
        "weather":weather.map(|w|json!({"OutDryBulbTemp":scalar(w.dry_bulb_c),"OutWetBulbTemp":scalar(w.wet_bulb_c),
            "WindSpeed":scalar(w.wind_speed_m_per_s),"WindDir":scalar(w.wind_direction_deg)}))},
        "unavailable_Native_owner_families":["cold_check_and_backend","constructor_weather_before_literal_write","EPVector_and_Objexx_allocation_implementation"],
        "unavailable_counted_as_PASS":false})
}
fn evaluate(case: &Value) -> Result<Value, String> {
    keys(
        case,
        &[
            "case_id",
            "purpose",
            "admission_lane",
            "initial",
            "operations",
        ],
    )?;
    let initial = required(case, "initial")?;
    keys(initial, &["site_parameters", "backend_flags", "surfaces"])?;
    let lane = required(case, "admission_lane")?
        .as_str()
        .ok_or("explicit direct lane required")?;
    if !["fixed-default-state-direct", "diagnostic-direct-operands"].contains(&lane) {
        return Err("unadmitted direct lane".into());
    }
    let input = required(initial, "site_parameters")?;
    keys(
        input,
        &[
            "Terrain_source_label",
            "SiteWindExp",
            "SiteWindBLHeight",
            "SiteTempGradient_override",
            "WeatherFileTempModCoeff_override",
            "WeatherFileWindModCoeff_override",
        ],
    )?;
    let terrain = match required(input, "Terrain_source_label")?
        .as_str()
        .ok_or("terrain Source label required")?
    {
        "Country" => Terrain::Country,
        "Suburbs" => Terrain::Suburbs,
        "City" => Terrain::City,
        "Ocean" => Terrain::Ocean,
        "Urban" => Terrain::Urban,
        _ => return Err("unknown actual terrain input".into()),
    };
    // Actual proposed constructor/default producer, never expected/output values.
    let mut p = SurfaceOutdoorEnvironmentParameters::default_weather_station(terrain);
    let default_parameters = parameters(p);
    p.site_wind_exponent = number(required(input, "SiteWindExp")?, false)?;
    p.site_wind_boundary_layer_height_m = number(required(input, "SiteWindBLHeight")?, false)?;
    if p.site_wind_exponent < 0.0 || p.site_wind_boundary_layer_height_m <= 0.0 {
        return Err("invalid direct site input".into());
    }
    for (name, destination) in [
        (
            "SiteTempGradient_override",
            &mut p.site_temperature_gradient_k_per_m,
        ),
        (
            "WeatherFileTempModCoeff_override",
            &mut p.weather_file_temperature_modifier_c,
        ),
        (
            "WeatherFileWindModCoeff_override",
            &mut p.weather_file_wind_modifier,
        ),
    ] {
        let value = required(input, name)?;
        if !value.is_null() {
            *destination = number(value, false)?;
        }
    }
    let flags = required(initial, "backend_flags")?;
    keys(
        flags,
        &[
            "WarmupFlag",
            "DoingSizing",
            "KickOffSimulation",
            "DoingInputProcessing",
            "printConsoleOutput",
        ],
    )?;
    if flags
        .as_object()
        .ok_or("backend input object required")?
        .values()
        .any(|v| !v.is_boolean())
    {
        return Err("literal backend flags required".into());
    }
    let initial_rows = required(initial, "surfaces")?
        .as_array()
        .ok_or("full ordered surface input required")?;
    let mut rows = Vec::with_capacity(initial_rows.len());
    let mut ids = Vec::with_capacity(initial_rows.len());
    let mut names = BTreeSet::new();
    for (i, r) in initial_rows.iter().enumerate() {
        keys(
            r,
            &[
                "actual_surface_index",
                "Name",
                "Centroid_z",
                "ExtWind",
                "SurfOutDryBulbTemp",
                "SurfOutWetBulbTemp",
                "SurfOutWindSpeed",
                "SurfOutWindDir",
            ],
        )?;
        let index = required(r, "actual_surface_index")?
            .as_u64()
            .ok_or("actual one-based input index required")?;
        let name = required(r, "Name")?
            .as_str()
            .ok_or("literal surface name required")?;
        if index != u64::try_from(i + 1).map_err(|e| e.to_string())?
            || name.is_empty()
            || !names.insert(name)
        {
            return Err("surface identity/order differs".into());
        }
        let exposed = required(r, "ExtWind")?
            .as_bool()
            .ok_or("literal exposure flag required")?;
        rows.push(SurfaceOutdoorEnvironmentRow {
            centroid_height_m: number(required(r, "Centroid_z")?, false)?,
            wind_exposed: exposed,
            dry_bulb_c: number(required(r, "SurfOutDryBulbTemp")?, false)?,
            wet_bulb_c: number(required(r, "SurfOutWetBulbTemp")?, false)?,
            wind_speed_m_per_s: number(
                required(r, "SurfOutWindSpeed")?,
                !exposed && p.site_wind_exponent != 0.0,
            )?,
            wind_direction_deg: number(required(r, "SurfOutWindDir")?, false)?,
        });
        ids.push((index, name.to_owned()));
    }
    let prepared = snapshot(p, None, &rows, rows.capacity(), &ids);
    let mut weather = None;
    let mut operations = Vec::new();
    let mut call_count = 0_usize;
    let mut point_call_count = 0_usize;
    let mut opids = BTreeSet::new();
    for op in required(case, "operations")?
        .as_array()
        .ok_or("full operation array required")?
    {
        keys(op, &["operation_id", "weather", "sequence"])?;
        let id = required(op, "operation_id")?
            .as_str()
            .ok_or("literal operation identity required")?;
        if !opids.insert(id) {
            return Err("duplicate operation identity".into());
        }
        if required(op, "sequence")? != &json!(["bulbs", "check", "wind_speed", "wind_direction"]) {
            return Err("declared manager sequence differs".into());
        }
        let before = snapshot(p, weather, &rows, rows.capacity(), &ids);
        let w = required(op, "weather")?;
        keys(
            w,
            &["OutDryBulbTemp", "OutWetBulbTemp", "WindSpeed", "WindDir"],
        )?;
        let current = SurfaceOutdoorWeather {
            dry_bulb_c: number(required(w, "OutDryBulbTemp")?, false)?,
            wet_bulb_c: number(required(w, "OutWetBulbTemp")?, false)?,
            wind_speed_m_per_s: number(required(w, "WindSpeed")?, false)?,
            wind_direction_deg: number(required(w, "WindDir")?, false)?,
        };
        weather = Some(current);
        let after_writes = snapshot(p, weather, &rows, rows.capacity(), &ids);
        let mut calls = Vec::new();
        for stage in ["bulbs", "check", "wind_speed", "wind_direction"] {
            let call_before = snapshot(p, weather, &rows, rows.capacity(), &ids);
            let invoked = match stage {
                "bulbs" => {
                    set_surface_out_bulb_temperatures(p, current, &mut rows);
                    true
                }
                "wind_speed" => {
                    set_surface_wind_speeds(p, current.wind_speed_m_per_s, &mut rows);
                    true
                }
                "wind_direction" => {
                    set_surface_wind_directions(current.wind_direction_deg, &mut rows);
                    true
                }
                "check" => false,
                _ => return Err("unknown selected stage".into()),
            };
            if invoked {
                call_count += 1;
            }
            // Independent actual point-method probes; no setter or expected arithmetic.
            let point_helpers: Vec<Value> = rows.iter().enumerate().filter_map(|(i,row)| match stage {
                "bulbs"=>{point_call_count+=2;Some(json!({"Rust_row_index":i,
                    "temperature_at_height_dry":scalar(p.temperature_at_height(current.dry_bulb_c,row.centroid_height_m)),
                    "temperature_at_height_wet":scalar(p.temperature_at_height(current.wet_bulb_c,row.centroid_height_m))}))},
                "wind_speed"=>{point_call_count+=1;Some(json!({"Rust_row_index":i,
                    "wind_speed_at_height":scalar(p.wind_speed_at_height(current.wind_speed_m_per_s,row.centroid_height_m))}))},
                _=>None,
            }).collect();
            calls.push(json!({"stage":stage,"actual_point_helper_returns":point_helpers,"actual_selected_method_invoked":invoked,"actual_selected_method_return_observed":invoked,
                "actual_selected_call_ordinal":if invoked {Some(call_count)} else {None},
                "status":if invoked {"actual-selected-setter-returned"} else {"unavailable-no-Rust-cold-check-backend"},
                "before":call_before,"after":snapshot(p,weather,&rows,rows.capacity(),&ids),"counted_as_PASS":false}));
        }
        operations.push(json!({"operation_id":id,"literal_operation":op,"before_declared_weather_writes":before,
            "after_declared_weather_writes":after_writes,"calls":calls,"independent_setter_probes_not_ordinary_manager":true}));
    }
    Ok(
        json!({"case_id":required(case,"case_id")?,"literal_input":case,"actual_default_station_constructor_invoked":true,
        "actual_default_station_parameters":default_parameters,"prepared_state":prepared,"operations":operations,
        "final":snapshot(p,weather,&rows,rows.capacity(),&ids),"actual_selected_method_call_count":call_count,
        "actual_point_helper_call_count":point_call_count,"cold_check_or_backend_invoked":false,"ordinary_production_handoff_verified":false,"numerical_PASS_claimed":false}),
    )
}
fn observe() -> Result<(), String> {
    let input_path = std::path::PathBuf::from(
        std::env::var_os("SRC05_REFERENCE_INPUT").ok_or("Root-recorded input operand required")?,
    );
    let output_path = std::path::PathBuf::from(
        std::env::var_os("SRC05_REFERENCE_OUTPUT")
            .ok_or("Root-recorded fresh output operand required")?,
    );
    let bytes = fs::read(&input_path).map_err(|e| e.to_string())?;
    let request: Value = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
    keys(
        &request,
        &[
            "schema",
            "expected_values_supplied",
            "ordinary_production_prestate_supplied",
            "cases",
        ],
    )?;
    if required(&request, "schema")? != "src05-surface-environment-direct-request.v1"
        || required(&request, "expected_values_supplied")? != false
        || required(&request, "ordinary_production_prestate_supplied")? != false
    {
        return Err("literal-only Native request required".into());
    }
    let mut ids = BTreeSet::new();
    let mut cases = Vec::new();
    for case in required(&request, "cases")?
        .as_array()
        .ok_or("all declared cases required")?
    {
        let id = required(case, "case_id")?
            .as_str()
            .ok_or("case identity required")?;
        if !ids.insert(id) {
            return Err("duplicate case identity".into());
        }
        cases.push(evaluate(case)?);
    }
    let result = json!({"schema":"src05-surface-environment-selected-results.v1","cases":cases,"actual_input_path":input_path,"actual_input_size_bytes":bytes.len(),
        "full_input_SHA_verified_internally":false,"external_full_input_SHA_and_Source_capture_before_after_required":true,
        "all_declared_operations_retained":true,"Native_results_or_expected_values_read":false,
        "cold_check_backend_or_ordinary_manager_PASS_claimed":false,"scientific_PASS_claimed":false,"gates_updated":false});
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
        .join(".runtime/porting/SRC-05")
        .canonicalize()
        .map_err(|e| e.to_string())?;
    if !parent.starts_with(runtime_root) {
        return Err("output must remain in ignored SRC05 runtime".into());
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
#[ignore = "Root-only literal input observation with full Source/environment custody"]
fn observe_literal_surface_environment_setters() -> Result<(), String> {
    observe()
}
