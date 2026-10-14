//! Opt-in actual root/cache numerical diagnostic inside the existing lifecycle body.
//! Inputs are prepared and authenticated before the recorded test command.
use crate::psychrometrics::{
    self, EnergyPlusPsychrometricCacheSlot as Slot,
    EnergyPlusPsychrometricStateSnapshot as Snapshot,
    observed_replay::{self, CacheEvent, CacheRequest, CacheStage, WindowBefore},
};
use serde_json::{Value, json};
use std::io::Read;
type TestResult = Result<(), Box<dyn std::error::Error>>;

fn bits(value: &Value) -> Result<u64, Box<dyn std::error::Error>> {
    Ok(u64::from_str_radix(
        value.as_str().ok_or("missing actual bit string")?,
        16,
    )?)
}
fn real(value: &Value) -> Result<f64, Box<dyn std::error::Error>> {
    Ok(f64::from_bits(bits(value)?))
}
fn state(value: &Value) -> Result<Snapshot, Box<dyn std::error::Error>> {
    let b = &value["bits"];
    Ok(Snapshot {
        iconv_tol: real(&b[0])?,
        last_patm: real(&b[1])?,
        last_t_boil: real(&b[2])?,
        press_save: real(&b[3])?,
        t_sat_save: real(&b[4])?,
        warmup: value["warmup"].as_bool().ok_or("missing actual warmup")?,
        use_interpolation: value["interpolation"]
            .as_bool()
            .ok_or("missing actual interpolation")?,
    })
}
fn slot(value: &Value) -> Result<Slot, Box<dyn std::error::Error>> {
    let index = usize::try_from(
        value["index"]
            .as_u64()
            .ok_or("missing actual cache index")?,
    )?;
    let tags = &value["tags"];
    let scalar = real(&value["bits"])?;
    Ok(
        match value["kind"].as_str().ok_or("missing actual cache kind")? {
            "Twb" => Slot::Twb {
                index,
                i_tdb: tags[0].as_u64().ok_or("actual Twb tag")?,
                i_w: tags[1].as_u64().ok_or("actual Twb tag")?,
                i_pb: tags[2].as_u64().ok_or("actual Twb tag")?,
                value: scalar,
            },
            "Psat" => Slot::Psat {
                index,
                i_tdb: tags[0].as_i64().ok_or("actual Psat tag")?,
                value: scalar,
            },
            "Tsat" => Slot::TsatPb {
                index,
                i_h: tags[0].as_i64().ok_or("actual Tsat tag")?,
                i_pb: tags[1].as_i64().ok_or("actual Tsat tag")?,
                value: scalar,
            },
            _ => return Err("unsupported actual cache family".into()),
        },
    )
}
fn state_json(s: Snapshot) -> Value {
    json!({"bits":[format!("{:016x}",s.iconv_tol.to_bits()),format!("{:016x}",s.last_patm.to_bits()),
        format!("{:016x}",s.last_t_boil.to_bits()),format!("{:016x}",s.press_save.to_bits()),format!("{:016x}",s.t_sat_save.to_bits())],
        "warmup":s.warmup,"interpolation":s.use_interpolation})
}
fn slot_json(s: Slot) -> Value {
    match s {
        Slot::Twb {
            index,
            i_tdb,
            i_w,
            i_pb,
            value,
        } => {
            json!({"kind":"Twb","index":index,"tags":[i_tdb,i_w,i_pb],"bits":format!("{:016x}",value.to_bits())})
        }
        Slot::Psat {
            index,
            i_tdb,
            value,
        } => {
            json!({"kind":"Psat","index":index,"tags":[i_tdb],"bits":format!("{:016x}",value.to_bits())})
        }
        Slot::TsatPb {
            index,
            i_h,
            i_pb,
            value,
        } => {
            json!({"kind":"Tsat","index":index,"tags":[i_h,i_pb],"bits":format!("{:016x}",value.to_bits())})
        }
        Slot::TsatHPb {
            index,
            i_h,
            i_pb,
            value,
        } => {
            json!({"kind":"UNOBSERVED-TsatHPb","index":index,"tags":[i_h,i_pb],"bits":format!("{:016x}",value.to_bits())})
        }
    }
}
fn event_json(event: &CacheEvent) -> Value {
    let request = event.request.map(|r| match r {
        CacheRequest::Twb(t, w, p) => json!({"kind":"Twb","tags":[t,w,p]}),
        CacheRequest::Psat(t) => json!({"kind":"Psat","tags":[t]}),
        CacheRequest::TsatPb(p) => json!({"kind":"Tsat","tags":[p]}),
    });
    let stage = match event.stage {
        CacheStage::Before => "Before",
        CacheStage::Miss => "Miss",
        CacheStage::TagWrite => "TagWrite",
        CacheStage::ValueWrite => "ValueWrite",
        CacheStage::After => "After",
    };
    json!({"stage":stage,"request":request,"slot":slot_json(event.slot),"scalars":state_json(event.scalars)})
}
fn compare_events(
    actual: &[CacheEvent],
    expected: &[Value],
    require_complete: bool,
) -> (usize, Vec<Value>) {
    let mut different = 0;
    let mut details = Vec::new();
    for (index, (a, e)) in actual.iter().zip(expected).enumerate() {
        let a = event_json(a);
        if a != *e {
            different += 1;
            eprintln!("DD PSY strict cache event {index}: Rust {a}; Native {e}");
            if details.len() < 64 {
                details.push(json!({"index":index,"actual":a,"native":e}));
            }
        }
    }
    if (require_complete && actual.len() != expected.len()) || actual.len() > expected.len() {
        different += 1;
        if details.len() < 64 {
            details.push(json!({"cache_event_count_actual":actual.len(),"cache_event_count_native":expected.len()}));
        }
    }
    (different, details)
}

fn report_window(
    window: &Value,
    completed: &[Value],
    events: &[CacheEvent],
    complete: bool,
    interruption: Option<String>,
    final_scalars: Option<Snapshot>,
    final_slots: Option<&[Slot]>,
) -> Result<usize, Box<dyn std::error::Error>> {
    let expected = window["events"].as_array().ok_or("actual cache events")?;
    let (cache_differences, details) = compare_events(events, expected, complete);
    let root_differences = completed
        .iter()
        .filter(|row| row["equal"].as_bool() == Some(false))
        .count();
    let mut final_differences = 0;
    if complete {
        if final_scalars.map(state_json) != Some(window["final_scalars"].clone()) {
            final_differences += 1;
        }
        let actual = final_slots
            .ok_or("complete window missing final cache observations")?
            .iter()
            .copied()
            .map(slot_json)
            .collect::<Vec<_>>();
        let expected = window["final_touched_slots"]
            .as_array()
            .ok_or("actual final touched slots")?;
        if actual.len() != expected.len() || actual.iter().any(|row| !expected.contains(row)) {
            final_differences += 1;
        }
    }
    let different = root_differences + cache_differences + final_differences;
    let outcome = if different > 0 {
        "FAIL"
    } else if complete {
        "OBSERVED_ROOT_CACHE_STRICT_PASS"
    } else {
        "UNKNOWN"
    };
    eprintln!(
        "{}",
        json!({"schema":"src06-passive-DD-PSY-root-cache-diagnostic.v1",
        "window_ordinal":window["window_ordinal"],"outcome":outcome,"observed_window_complete":complete,
        "interruption":interruption,"completed_root_comparisons":completed,
        "expected_roots":window["roots"].as_array().ok_or("actual roots")?.len(),
        "observed_cache_events":events.len(),"expected_cache_events":expected.len(),
        "root_difference_count":root_differences,"cache_difference_count":cache_differences,
        "final_state_difference_count":final_differences,"cache_differences_first64":details,
        "unmatched_suffix_status":if complete {"NONE"} else {"UNKNOWN"},
        "full_DP_RH_assignment_raw_Iterate_closure":"UNKNOWN",
        "whole_DD_9_FAIL_8_UNPAIRED_unchanged":true})
    );
    Ok(different)
}

fn replay_window(window: &Value) -> Result<(usize, usize, usize), Box<dyn std::error::Error>> {
    let seed = WindowBefore {
        scalars: state(&window["before"])?,
        first_touch: window["first_touch"]
            .as_array()
            .ok_or("actual first touches")?
            .iter()
            .map(slot)
            .collect::<Result<Vec<_>, _>>()?,
        tsat_precision_bits: u32::try_from(
            window["tsat_precision_bits"]
                .as_u64()
                .ok_or("actual Tsat precision")?,
        )?,
    };
    let roots = window["roots"].as_array().ok_or("actual roots")?;
    if roots.len() > 65_536 {
        return Err("bounded observed root ledger exceeded".into());
    }
    let mut completed = Vec::new();
    let observed = match observed_replay::with_observed_window(seed, || -> TestResult {
        for (index, row) in roots.iter().enumerate() {
            let expected = bits(&row["result"])?;
            let actual = root(row)?;
            completed.push(json!({"index":index,"routine":row["routine"],
                "actual_bits":format!("{actual:016x}"),"native_bits":format!("{expected:016x}"),"equal":actual==expected}));
        }
        Ok(())
    }) {
        Ok(value) => value,
        Err(error) => {
            report_window(
                window,
                &completed,
                &error.1,
                false,
                Some(error.to_string()),
                None,
                None,
            )?;
            return Err(error.into());
        }
    };
    if let Err(error) = observed.value {
        report_window(
            window,
            &completed,
            &observed.events,
            false,
            Some(error.to_string()),
            None,
            None,
        )?;
        return Err(error);
    }
    let differences = report_window(
        window,
        &completed,
        &observed.events,
        true,
        None,
        Some(observed.final_scalars),
        Some(&observed.final_touched_slots),
    )?;
    Ok((completed.len(), observed.events.len(), differences))
}

fn root(row: &Value) -> Result<u64, Box<dyn std::error::Error>> {
    let x = row["inputs"]
        .as_array()
        .ok_or("actual root formals")?
        .iter()
        .map(real)
        .collect::<Result<Vec<_>, _>>()?;
    let routine = row["routine"].as_str().ok_or("actual root routine")?;
    let count = if matches!(routine, "PsyWFnTdpPb" | "PsyTdpFnWPb") {
        2
    } else {
        3
    };
    if x.len() != count {
        return Err("actual root argument count differs".into());
    }
    let value = match routine {
        "PsyWFnTdbTwbPb" => psychrometrics::energyplus_psy_w_fn_tdb_twb_pb(x[0], x[1], x[2]),
        "PsyTwbFnTdbWPb" => psychrometrics::energyplus_psy_twb_fn_tdb_w_pb(x[0], x[1], x[2]),
        "PsyWFnTdpPb" => {
            super::super::super::design_day::replay_humidity_from_dew_point(x[0], x[1])?
        }
        "PsyTdpFnWPb" => super::super::super::design_day::replay_dew_point(x[0], x[1]),
        "PsyRhFnTdbWPb" => {
            super::super::super::design_day::replay_relative_humidity(x[0], x[1], x[2])?
        }
        _ => return Err("unavailable actual outer root".into()),
    };
    Ok(value.to_bits())
}

pub(super) fn verify_if_selected() -> TestResult {
    let Some(path) = std::env::var_os("EP_DD_PSY_ACTUAL_REPLAY_INPUT") else {
        return Ok(());
    };
    let mut data = Vec::new();
    std::fs::File::open(path)?
        .take(16 * 1024 * 1024 + 1)
        .read_to_end(&mut data)?;
    if data.len() > 16 * 1024 * 1024 {
        return Err("bounded actual replay input exceeded".into());
    }
    let fixture: Value = serde_json::from_slice(&data)?;
    assert_eq!(
        fixture["schema"],
        "src06-passive-DD-PSY-actual-root-cache-inputs.v1"
    );
    assert_eq!(
        fixture["facts"]["sha256"],
        "139dba2ce0204b4b55655b2b3c02a6fc5a8ee32ab6465e6a773aa7c726f8a562"
    );
    let mut roots_seen = 0;
    let mut events_seen = 0;
    let mut differences = 0;
    for window in fixture["windows"]
        .as_array()
        .ok_or("actual replay windows")?
    {
        let (roots, events, changed) = replay_window(window)?;
        roots_seen += roots;
        events_seen += events;
        differences += changed;
    }
    eprintln!(
        "DD PSY actual roots={roots_seen}, cache events={events_seen}, strict differences={differences}; full DP/RH assignment/raw-branch closure remains UNKNOWN"
    );
    assert_eq!(
        differences, 0,
        "strict observed root/cache numerical diagnostic FAIL; no tolerance relaxation"
    );
    Ok(())
}
