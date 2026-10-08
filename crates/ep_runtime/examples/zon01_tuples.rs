//! Input-only selected constructor/reconstruction/member/guard ownership.
//!
//! Calls the same actual public kernel as production. External Begin/OutW are
//! explicit local input context; no native weather/manager sibling state is made.

use ep_runtime::heat_balance::{ZoneAirEnvironmentGuard, ZoneAirInitializationState};
use serde_json::{Value, json};
use std::{collections::BTreeSet, error::Error, io::Read};

const SCALARS: [&str; 18] = [
    "MAT",
    "ZT",
    "ZTAV",
    "XMPT",
    "TMX",
    "TM2",
    "airHumRat",
    "airHumRatAvg",
    "WTimeMinusP",
    "W1",
    "WMX",
    "WM2",
    "airHumRatTemp",
    "tempIndLoad",
    "tempDepLoad",
    "airRelHum",
    "AirPowerCap",
    "T1",
];
const ARRAYS: [&str; 6] = [
    "XMAT",
    "DSXMAT",
    "ZTM",
    "WPrevZoneTS",
    "DSWPrevZoneTS",
    "WPrevZoneTSTemp",
];

fn bits(value: f64) -> String {
    format!("{:016x}", value.to_bits())
}
fn scalar(value: f64) -> Value {
    let class = if value.is_nan() {
        "nan"
    } else if value == f64::INFINITY {
        "positive_infinity"
    } else if value == f64::NEG_INFINITY {
        "negative_infinity"
    } else if value == 0.0 {
        if value.is_sign_negative() {
            "negative_zero"
        } else {
            "positive_zero"
        }
    } else {
        "finite"
    };
    json!({"value":if value.is_finite(){Some(value)}else{None},"value_bits":bits(value),"value_class":class})
}
fn text<'a>(value: &'a Value, key: &str) -> Result<&'a str, String> {
    value[key]
        .as_str()
        .ok_or_else(|| format!("expected string {key}"))
}
fn own_number(value: &Value, token: &Value) -> Result<f64, String> {
    let number = value.as_f64().ok_or("expected numerical input")?;
    if !number.is_finite() || token.as_str() != Some(bits(number).as_str()) {
        return Err(
            "independently parsed finite binary64 input differs from authoritative bits".into(),
        );
    }
    Ok(number)
}
fn validate_operation(operation: &Value) -> Result<(), String> {
    text(operation, "operation_id")?;
    let name = text(operation, "operation")?;
    if !matches!(
        name,
        "snapshot"
            | "seed_zone_state"
            | "bulk_reconstruct_and_current_w_seed"
            | "bare_begin_environment"
            | "guarded_init_zone_air_setpoints"
    ) {
        return Err("unknown frozen operation".into());
    }
    if let Some(number) = operation.get("out_hum_rat") {
        own_number(number, &operation["out_hum_rat_bits"])?;
    } else if operation.get("out_hum_rat_bits").is_some() {
        return Err("unbound external-W bits".into());
    }
    if operation
        .get("begin_environment")
        .is_some_and(|value| !value.is_boolean())
    {
        return Err("expected boolean environment input".into());
    }
    if name == "seed_zone_state" {
        let fields = operation["fields"]
            .as_object()
            .ok_or("expected seed fields")?;
        let tokens = operation["input_field_bits"]
            .as_object()
            .ok_or("expected seed bit fields")?;
        if fields.len() != tokens.len() || fields.keys().any(|key| !tokens.contains_key(key)) {
            return Err("seed field/bit keys differ".into());
        }
        for (key, value) in fields {
            if SCALARS.contains(&key.as_str()) {
                own_number(value, &tokens[key])?;
            } else if ARRAYS.contains(&key.as_str()) {
                let values = value.as_array().ok_or("expected four-slot input")?;
                let tokens = tokens[key].as_array().ok_or("expected four bit tokens")?;
                if values.len() != 4 || tokens.len() != 4 {
                    return Err("source input slot count differs".into());
                }
                for (number, token) in values.iter().zip(tokens) {
                    own_number(number, token)?;
                }
            } else {
                return Err(format!("unknown source input field {key}"));
            }
        }
    }
    Ok(())
}
fn fields(owner: &ZoneAirInitializationState) -> Value {
    json!({
        "MAT":scalar(owner.mat),"ZT":scalar(owner.zt),"ZTAV":scalar(owner.ztav),
        "XMPT":scalar(owner.xmpt),"XMAT":owner.xmat.map(scalar),"DSXMAT":owner.dsxmat.map(scalar),
        "TMX":scalar(owner.tmx),"TM2":scalar(owner.tm2),
        "airHumRat":scalar(owner.air_hum_rat),"airHumRatAvg":scalar(owner.air_hum_rat_avg),
        "ZTM":owner.ztm.map(scalar),"WPrevZoneTS":owner.w_prev_zone_ts.map(scalar),
        "DSWPrevZoneTS":owner.dsw_prev_zone_ts.map(scalar),"WPrevZoneTSTemp":owner.w_prev_zone_ts_temp.map(scalar),
        "WTimeMinusP":scalar(owner.w_time_minus_p),"W1":scalar(owner.w1),"WMX":scalar(owner.wmx),"WM2":scalar(owner.wm2),
        "airHumRatTemp":scalar(owner.air_hum_rat_temp),"tempIndLoad":scalar(owner.temp_ind_load),
        "tempDepLoad":scalar(owner.temp_dep_load),"airRelHum":scalar(owner.air_rel_hum),
        "AirPowerCap":scalar(owner.air_power_cap),"T1":scalar(owner.t1),
    })
}
fn snapshot(
    phase: &str,
    owner: Option<&ZoneAirInitializationState>,
    named: bool,
    guard: ZoneAirEnvironmentGuard,
    begin: bool,
    out_w: f64,
) -> Value {
    let zones = owner.map(|owner|json!({"id":0,"name":if named {Some("PREPARED-ONE-ZONE")}else{None},"fields":fields(owner)}));
    json!({"phase":phase,"zones":zones.into_iter().collect::<Vec<_>>(),
        "zone_owner_allocated":owner.is_some(),"zone_owner_count":usize::from(owner.is_some()),
        "flags":{"MyEnvrnFlag":guard.my_environment_flag,"BeginEnvrnFlag":begin},
        "OutHumRat":scalar(out_w),"context":null,
        "unpaired_native_context_and_manager_sibling_flags":true})
}
fn seed(owner: &mut ZoneAirInitializationState, operation: &Value) -> Result<(), String> {
    let fields = operation["fields"]
        .as_object()
        .ok_or("expected seed fields")?;
    for (key, value) in fields {
        let target = match key.as_str() {
            "MAT" => Some(&mut owner.mat),
            "ZT" => Some(&mut owner.zt),
            "ZTAV" => Some(&mut owner.ztav),
            "XMPT" => Some(&mut owner.xmpt),
            "TMX" => Some(&mut owner.tmx),
            "TM2" => Some(&mut owner.tm2),
            "airHumRat" => Some(&mut owner.air_hum_rat),
            "airHumRatAvg" => Some(&mut owner.air_hum_rat_avg),
            "WTimeMinusP" => Some(&mut owner.w_time_minus_p),
            "W1" => Some(&mut owner.w1),
            "WMX" => Some(&mut owner.wmx),
            "WM2" => Some(&mut owner.wm2),
            "airHumRatTemp" => Some(&mut owner.air_hum_rat_temp),
            "tempIndLoad" => Some(&mut owner.temp_ind_load),
            "tempDepLoad" => Some(&mut owner.temp_dep_load),
            "airRelHum" => Some(&mut owner.air_rel_hum),
            "AirPowerCap" => Some(&mut owner.air_power_cap),
            "T1" => Some(&mut owner.t1),
            _ => None,
        };
        if let Some(target) = target {
            *target = own_number(value, &operation["input_field_bits"][key])?;
            continue;
        }
        let target = match key.as_str() {
            "XMAT" => &mut owner.xmat,
            "DSXMAT" => &mut owner.dsxmat,
            "ZTM" => &mut owner.ztm,
            "WPrevZoneTS" => &mut owner.w_prev_zone_ts,
            "DSWPrevZoneTS" => &mut owner.dsw_prev_zone_ts,
            "WPrevZoneTSTemp" => &mut owner.w_prev_zone_ts_temp,
            _ => return Err("unknown validated input field".into()),
        };
        for (index, element) in target.iter_mut().enumerate() {
            *element = own_number(&value[index], &operation["input_field_bits"][key][index])?;
        }
    }
    Ok(())
}
fn execute(request: &Value) -> Result<Value, String> {
    if request["schema"].as_str() != Some("zon01-helper-cases.v1")
        || request["expected_values_supplied"].as_bool() != Some(false)
    {
        return Err("expected frozen input-only ZON-01 request".into());
    }
    let sequences = request["sequences"]
        .as_array()
        .ok_or("expected sequences")?;
    let mut ids = BTreeSet::new();
    let mut results = Vec::new();
    for sequence in sequences {
        let id = text(sequence, "sequence_id")?;
        if !ids.insert(id) {
            return Err("duplicate sequence_id".into());
        }
        let mut guard = ZoneAirEnvironmentGuard::default();
        // Explicit external input context starts at the source declaration defaults.
        // No EnergyPlus manager/context object or unrelated flags are fabricated.
        let mut begin = false;
        let mut out_w = 0.0;
        let constructor = snapshot(
            "selected-Rust-container-before-zone-allocation",
            None,
            false,
            guard,
            begin,
            out_w,
        );
        let mut owner = ZoneAirInitializationState::default();
        let allocated = snapshot(
            "selected-Rust-zone-constructor",
            Some(&owner),
            false,
            guard,
            begin,
            out_w,
        );
        let prepared = snapshot(
            "explicit-one-zone-selected-Rust-guard-context",
            Some(&owner),
            true,
            guard,
            begin,
            out_w,
        );
        let mut operations = Vec::new();
        let mut operation_ids = BTreeSet::new();
        let mut bare_calls = 0_usize;
        let mut guard_calls = 0_usize;
        let mut bulk_calls = 0_usize;
        for operation in sequence["operations"]
            .as_array()
            .ok_or("expected operations")?
        {
            validate_operation(operation)?;
            let operation_id = text(operation, "operation_id")?;
            if !operation_ids.insert(operation_id) {
                return Err("duplicate operation_id".into());
            }
            let name = text(operation, "operation")?;
            let before_inputs = snapshot(
                "before-operation-input-writes",
                Some(&owner),
                true,
                guard,
                begin,
                out_w,
            );
            if let Some(value) = operation.get("out_hum_rat") {
                out_w = own_number(value, &operation["out_hum_rat_bits"])?;
            }
            if let Some(value) = operation.get("begin_environment") {
                begin = value.as_bool().ok_or("expected boolean")?;
            }
            let before = snapshot(
                "after-declared-operation-inputs-before-selected-Rust-call",
                Some(&owner),
                true,
                guard,
                begin,
                out_w,
            );
            let eligible = guard.my_environment_flag && begin;
            let mut invocation = None;
            match name {
                "snapshot" => {}
                "seed_zone_state" => seed(&mut owner, operation)?,
                "bulk_reconstruct_and_current_w_seed" => {
                    owner.bulk_reconstruct_and_current_w_seed(out_w);
                    bulk_calls += 1;
                }
                "bare_begin_environment" => {
                    owner.begin_environment_init(out_w);
                    bare_calls += 1;
                }
                "guarded_init_zone_air_setpoints" => {
                    invocation = Some(guard.apply(begin, out_w, std::slice::from_mut(&mut owner)));
                    guard_calls += 1;
                }
                _ => return Err("unknown validated operation".into()),
            }
            operations.push(json!({"operation_id":operation_id,"operation":name,"input":operation,
                "before_inputs":before_inputs,"before":before,
                "after":snapshot("after-selected-Rust-wrapper-operation",Some(&owner),true,guard,begin,out_w),
                "source_guard_eligibility_before_call":eligible,"member_call_count_observed":false,
                "numeric_input_identity_checked":true,
                "Rust_guard_invocation":invocation.map(|value|json!({
                    "begin_environment":value.begin_environment,"my_environment_before":value.my_environment_before,
                    "eligible_before":value.eligible_before,"my_environment_after":value.my_environment_after,
                    "initializer_invocations":value.initializer_invocations,"initializer_invocations_are_Rust_only":true,
                })),
                "wrapper_invocations":{"bare_begin_environment":bare_calls,"guarded_init_zone_air_setpoints":guard_calls,
                    "bulk_reconstruct_and_current_w_seed":bulk_calls}}));
        }
        results.push(json!({"sequence_id":id,"kind":text(sequence,"kind")?,"input":sequence,
            "status":"source_complete","route":"selected-public-Rust-zone-air-initialization-owner",
            "constructor":constructor,"allocated_zone_constructor":allocated,"prepared_manager_state":prepared,
            "operations":operations,"wrapper_invocations":{"bare_begin_environment":bare_calls,
                "guarded_init_zone_air_setpoints":guard_calls,"bulk_reconstruct_and_current_w_seed":bulk_calls},
            "original_parser_admission_claimed":false,"physics_executed":false,
            "whole_original_manager_siblings_implemented":false}));
    }
    Ok(
        json!({"schema":"zon01-helper-results.v1","implementation_stage":"canonical_owned_state",
        "sequences":results,"reference_outputs_supplied_to_Rust":false,"expected_values_supplied":false,
        "physics_executed":false,"gates_updated":false,
        "unpaired_source_context":["native calendar/weather/warmup","manager sibling flags/thermostat/day/demand/hybrid/Space resets","native member invocation counts"]}),
    )
}
fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args_os().skip(1);
    let input = if let Some(path) = args.next() {
        std::fs::read_to_string(path)?
    } else {
        let mut data = String::new();
        std::io::stdin().read_to_string(&mut data)?;
        data
    };
    if args.next().is_some() {
        return Err("usage: zon01_tuples [INPUT.json]".into());
    }
    println!(
        "{}",
        serde_json::to_string(&execute(&serde_json::from_str(&input)?)?)?
    );
    Ok(())
}
