//! Output-only encoding shared by PSY-02 operation and final-cache observations.

use ep_runtime::psychrometrics::{
    EnergyPlusPsychrometricCacheSlot, EnergyPlusPsychrometricFinalCaches,
    EnergyPlusPsychrometricStateSnapshot,
};
use serde_json::{Value, json};

pub(crate) fn scalar(value: f64) -> Value {
    if value.is_nan() {
        json!("NaN")
    } else if value == f64::INFINITY {
        json!("+Infinity")
    } else if value == f64::NEG_INFINITY {
        json!("-Infinity")
    } else {
        json!(value)
    }
}

pub(crate) fn snapshot(state: EnergyPlusPsychrometricStateSnapshot) -> Value {
    json!({
        "iconv_tol": scalar(state.iconv_tol), "iconv_tol_bits": format!("{:016x}", state.iconv_tol.to_bits()),
        "last_patm": scalar(state.last_patm), "last_patm_bits": format!("{:016x}", state.last_patm.to_bits()),
        "last_t_boil": scalar(state.last_t_boil), "last_t_boil_bits": format!("{:016x}", state.last_t_boil.to_bits()),
        "press_save": scalar(state.press_save), "press_save_bits": format!("{:016x}", state.press_save.to_bits()),
        "t_sat_save": scalar(state.t_sat_save), "t_sat_save_bits": format!("{:016x}", state.t_sat_save.to_bits()),
        "warmup": state.warmup, "use_interpolation": state.use_interpolation,
    })
}

pub(crate) fn slot(cache: EnergyPlusPsychrometricCacheSlot) -> Value {
    use EnergyPlusPsychrometricCacheSlot as Slot;
    let (mut record, value) = match cache {
        Slot::Twb {
            index,
            i_tdb,
            i_w,
            i_pb,
            value,
        } => (
            json!({"kind": "Twb", "index": index, "i_tdb": i_tdb, "i_w": i_w, "i_pb": i_pb}),
            value,
        ),
        Slot::Psat {
            index,
            i_tdb,
            value,
        } => (
            json!({"kind": "Psat", "index": index, "i_tdb": i_tdb}),
            value,
        ),
        Slot::TsatPb {
            index,
            i_h,
            i_pb,
            value,
        } => (
            json!({"kind": "TsatPb", "index": index, "i_h": i_h, "i_pb": i_pb}),
            value,
        ),
        Slot::TsatHPb {
            index,
            i_h,
            i_pb,
            value,
        } => (
            json!({"kind": "TsatHPb", "index": index, "i_h": i_h, "i_pb": i_pb}),
            value,
        ),
    };
    record["value"] = scalar(value);
    record["value_bits"] = json!(format!("{:016x}", value.to_bits()));
    record
}

pub(crate) fn final_caches(caches: &EnergyPlusPsychrometricFinalCaches) -> Value {
    json!({
        "Twb": caches.twb.iter().copied().map(slot).collect::<Vec<_>>(),
        "Psat": caches.psat.iter().copied().map(slot).collect::<Vec<_>>(),
        "TsatPb": caches.tsat_pb.iter().copied().map(slot).collect::<Vec<_>>(),
        "TsatHPb": caches.tsat_h_pb.iter().copied().map(slot).collect::<Vec<_>>(),
    })
}
