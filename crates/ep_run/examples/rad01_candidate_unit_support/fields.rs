//! Copies of actual RAD-01 public owners; no view-factor or warning arithmetic.
use ep_model::SurfaceType;
use ep_runtime::heat_balance::radiation::{
    ApproximateViewFactorContext, ApproximateViewFactorScopeError, ApproximateViewFactorSurface,
    ApproximateViewFactors,
};
use serde_json::{Value, json};

pub(super) fn scalar(value: f64) -> Value {
    let class = if value.is_nan() {
        "nan"
    } else if value.is_infinite() {
        if value.is_sign_negative() {
            "negative_infinity"
        } else {
            "positive_infinity"
        }
    } else if value == 0.0 {
        if value.is_sign_negative() {
            "negative_zero"
        } else {
            "positive_zero"
        }
    } else {
        "finite"
    };
    json!({"value": if value.is_finite() { Some(value) } else { None },
        "value_bits": format!("{:016x}", value.to_bits()), "value_class": class})
}
pub(super) fn context(value: ApproximateViewFactorContext) -> Value {
    json!({"enclosure_index": value.enclosure_index, "zone_id": value.zone_id.0})
}
pub(super) fn surface(value: &ApproximateViewFactorSurface) -> Value {
    let class = match value.surface_type {
        SurfaceType::Wall => "Wall",
        SurfaceType::Floor => "Floor",
        SurfaceType::Roof => "Roof",
        SurfaceType::Ceiling => "Ceiling",
    };
    json!({"surface_id": value.surface_id.0, "surface_name": value.surface_name,
        "zone_id": value.zone_id.0, "zone_name": value.zone_name, "surface_type": class,
        "area_m2": scalar(value.area_m2), "azimuth_deg": scalar(value.azimuth_deg),
        "tilt_deg": scalar(value.tilt_deg)})
}
pub(super) fn result(
    value: &Result<ApproximateViewFactors, ApproximateViewFactorScopeError>,
) -> Value {
    match value {
        Err(error) => json!({"status": "scope-error-returned", "owner_available": false,
            "scope_error": format!("{error:?}"), "snapshot": null, "counted_as_PASS": false,
            "Native_outcome_inferred": false}),
        Ok(actual) => json!({"status": "source-returned", "owner_available": true,
            "scope_error": null, "snapshot": {
                "context": context(actual.context),
                "ordered_surface_ids": actual.ordered_surface_ids.iter().map(|v| v.0).collect::<Vec<_>>(),
                "ZoneArea": actual.zone_area_seen_m2.iter().map(|v| scalar(*v)).collect::<Vec<_>>(),
                "F": actual.view_factors.iter().map(|v| scalar(*v)).collect::<Vec<_>>(),
                "F_shape": [actual.ordered_surface_ids.len(), actual.ordered_surface_ids.len()],
                "F_layout": "actual Rust row-first F(j,i), flat offset j*N+i",
                "warnings": actual.warnings.iter().map(|v| json!({
                    "surface_index": v.surface_index, "surface_id": v.surface_id.0,
                    "surface_name": v.surface_name, "zone_id": v.zone_id.0,
                    "zone_name": v.zone_name})).collect::<Vec<_>>()},
            "counted_as_PASS": false, "Native_outcome_inferred": false}),
    }
}
pub(super) const OBSERVER_PATHS: &[&str] = &[
    "crates/ep_run/examples/rad01_candidate_unit_observer.rs",
    "crates/ep_run/examples/rad01_candidate_unit_support/fields.rs",
];
pub(super) const API_PATHS: &[&str] = &[
    "crates/ep_run/Cargo.toml",
    "crates/ep_model/src/lib.rs",
    "crates/ep_model/src/ids.rs",
    "crates/ep_model/src/objects/mod.rs",
    "crates/ep_model/src/objects/surfaces.rs",
    "crates/ep_runtime/Cargo.toml",
    "crates/ep_runtime/src/lib.rs",
    "crates/ep_runtime/src/heat_balance/mod.rs",
    "crates/ep_runtime/src/heat_balance/radiation.rs",
    "crates/ep_runtime/examples/clk02_probe_support/digest.rs",
];
