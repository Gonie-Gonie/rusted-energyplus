//! Pure copies: existing R/C/steady CTF context is not normalized layer storage.
use ep_model::{MaterialDefinition, TypedModel};
use ep_runtime::heat_balance::{HeatBalanceState, SurfaceCtfState};
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
    json!({"value":if value.is_finite() { Some(value) } else { None },
        "value_bits":format!("{:016x}",value.to_bits()),"value_class":class})
}
fn values(source: &[f64]) -> Vec<Value> {
    source.iter().copied().map(scalar).collect()
}
fn missing(reason: &str) -> Value {
    json!({"owner_available":false,"value":null,"reason":reason,"counted_as_PASS":false})
}
pub(super) fn unavailable() -> Value {
    json!({"PostLoad":missing("existing Rust has no normalized active-prefix/checkpoint owner"),
        "PostMerge":missing("existing Rust has no normalized active-prefix/checkpoint owner"),
        "PostConversion":missing("existing Rust has no converted active-prefix/rs/cnd/dyn owner"),
        "IsUsedCTF":missing("existing Rust has surface construction references but no actual IsUsedCTF owner"),
        "ErrorsFound":missing("existing Rust has no shared CTF preprocessing ErrorsFound owner"),
        "DoCTFErrorReport":missing("existing Rust has no shared CTF preprocessing DoCTFErrorReport owner")})
}
pub(super) fn typed(model: &TypedModel) -> Value {
    let materials = model.materials.iter().enumerate().map(|(index,material)| {
        let payload = match material.definition {
            MaterialDefinition::Regular(source) => json!({"kind":"Regular","fields":{
                "Thickness":scalar(source.thickness_m),"Conductivity":scalar(source.conductivity_w_per_m_k),
                "Density":scalar(source.density_kg_per_m3),"SpecHeat":scalar(source.specific_heat_j_per_kg_k)},
                "stored_Resistance":missing("current RegularMaterial has no stored Resistance field; derived d/k is not read")}),
            MaterialDefinition::NoMass(source) => json!({"kind":"NoMass",
                "stored_Resistance":{"owner_available":true,"value":scalar(source.thermal_resistance_m2_k_per_w)},
                "regular_member_fields":null,"regular_member_fields_fabricated":false}),
            _ => json!({"kind":format!("{:?}",material.family()),"selected_member_payload":null,
                "unavailable_reason":"material family outside selected Regular/NoMass observer payload"}),
        };
        json!({"model_vector_index":index,"id":material.id.0,"name":material.name.0,"payload":payload})
    }).collect::<Vec<_>>();
    json!({"materials":materials,"constructions":model.constructions.iter().enumerate().map(|(index,source)|json!({
        "model_vector_index":index,"id":source.id.0,"name":source.name.0,"kind":format!("{:?}",source.kind),
        "declared_layer_ids":source.layers.iter().map(|id|id.0).collect::<Vec<_>>(),
        "effective_layer_ids":source.effective_layers().iter().map(|id|id.0).collect::<Vec<_>>(),
        "outside_layer_id":source.outside_layer.map(|id|id.0),
        "internal_heat_source_declaration":format!("{:?}",source.internal_heat_source),
        "selected_CTF01_owners":unavailable()
    })).collect::<Vec<_>>(),"surfaces":model.surfaces.iter().enumerate().map(|(index,source)|json!({
        "model_vector_index":index,"id":source.id.0,"name":source.name.0,"construction_id":source.construction.0
    })).collect::<Vec<_>>()})
}
fn ctf(source: &SurfaceCtfState) -> Value {
    json!({"context_only_unpaired":true,"outside_0":scalar(source.outside_0_w_per_m2_k),
        "cross_0":scalar(source.cross_0_w_per_m2_k),"inside_0":scalar(source.inside_0_w_per_m2_k),
        "flux_0":source.flux_0.map(scalar),"const_in_part":scalar(source.const_in_part_w_per_m2),
        "const_out_part":scalar(source.const_out_part_w_per_m2),
        "outside_history":values(&source.outside_history_w_per_m2_k),"cross_history":values(&source.cross_history_w_per_m2_k),
        "inside_history":values(&source.inside_history_w_per_m2_k),"flux_history":values(&source.flux_history),
        "outside_temperature_history":values(&source.outside_temperature_history_c),
        "inside_temperature_history":values(&source.inside_temperature_history_c),
        "outside_flux_history":values(&source.outside_flux_history_w_per_m2),
        "inside_flux_history":values(&source.inside_flux_history_w_per_m2)})
}
pub(super) fn state(source: &HeatBalanceState) -> Value {
    json!({"context_only_unpaired":true,"timestep_index":source.timestep_index,
        "cache_summary":{"hash":source.construction_cache_hash,"entry_count":source.construction_cache_entry_count,
            "no_mass_count":source.construction_cache_no_mass_count,"massive_count":source.construction_cache_massive_ctf_count,
            "eio_seeded_count":source.construction_cache_eio_seeded_count,
            "rust_generated_count":source.construction_cache_rust_generated_count},
        "private_cache_rows_directly_observed":false,"surfaces":source.surfaces.iter().enumerate().map(|(index,s)|json!({
            "state_vector_index":index,"surface_id":s.surface_id.0,"surface_name":s.surface_name,
            "construction_id":s.construction_id.0,"construction_name":s.construction_name,
            "actual_cache_index_copy":s.construction_thermal_data_index,
            "outside_layer_material_id":s.outside_layer_material_id.0,
            "outside_layer_material_name":s.outside_layer_material_name,
            "SI_resistance_context":scalar(s.thermal_resistance_m2_k_per_w),
            "SI_heat_capacity_context":s.heat_capacity_j_per_m2_k.map(scalar),"ctf_context":ctf(&s.ctf)
        })).collect::<Vec<_>>()})
}

// Filled only from current source bytes while preparing the immutable proposal.
pub(super) const API_SOURCES: &[(&str, &str)] = &[
    (
        "crates/ep_run/Cargo.toml",
        "7739bf48b53716089466f6e9d734d1f97038d9cb919787de75804eab474aa2bd",
    ),
    (
        "crates/ep_raw_model/src/lib.rs",
        "08f4a3bcf27c2f4e2577b534ad505176d0877b46709bb512b71685ebef6fa9a8",
    ),
    (
        "crates/ep_raw_model/src/idf_order.rs",
        "5f2a1984ebc76a51938232ab7002c9c683666cf88d765af8e02dbf126bad7772",
    ),
    (
        "crates/ep_compiler/src/lib.rs",
        "1931ed21f1fae2fe5d7e90bb954f1f91812fec1d1eeb7daf102a6ba0007424fe",
    ),
    (
        "crates/ep_compiler/src/compiler.rs",
        "bac1a9c8de1228a67cf5ae9c8c5ac7d4524d6e61882938969c43238217ddbdbc",
    ),
    (
        "crates/ep_model/src/lib.rs",
        "2a756532553034fb2b92c892cb2e6f3c147aa129f6dce64b63763f364cb3cee4",
    ),
    (
        "crates/ep_model/src/model.rs",
        "297b2f7200d76592f0b59276cbb0f4038f2a5e4a1477f2f1beaa923444eadf99",
    ),
    (
        "crates/ep_model/src/objects/materials.rs",
        "ea0769227adb6f000d1e777b74a61829bd458a6bdf52d8d5221e71a1c380bacc",
    ),
    (
        "crates/ep_model/src/objects/construction.rs",
        "f1886c8b2f9aede1e661210520b668d52f74d884d9a207de9f0c046e4170f191",
    ),
    (
        "crates/ep_runtime/src/lib.rs",
        "f3c569d10f96a1c4922c682ae88ff523b48e01d1bf267bfa107f2afca08bddb7",
    ),
    (
        "crates/ep_runtime/src/heat_balance/mod.rs",
        "45e6f8b9dfc0d7f85bd854a849cc0092c3ee7739fd4895c543aa73a1ea8bce46",
    ),
    (
        "crates/ep_runtime/src/heat_balance/initialization.rs",
        "ba68a33bd1c42a6b63b3fb317e68017f5c196d6a95eb15a3ea305d265ba4db34",
    ),
    (
        "crates/ep_runtime/src/heat_balance/initialization/schedule_cache.rs",
        "2b3343ff4f3cec29be8cc77bcd4d14a8bbcc78464812ad04b88d22469f7f0dfb",
    ),
    (
        "crates/ep_runtime/src/heat_balance/initialization/state_shell.rs",
        "cfbe0e4edb4dd1447430f96fc860042a2fd7f8d4eb6c6486c50f485333295ead",
    ),
    (
        "crates/ep_runtime/src/heat_balance/surface_manager.rs",
        "80234dc5130ac9fd42b26909aea0775f34f0566a3ea5d3e6da1d32c61186cde4",
    ),
    (
        "crates/ep_runtime/src/heat_balance/surface_manager/construction_cache.rs",
        "dac632149885383131df32f283a1d4fa5b2fd09ebf3ad6fdcd022825a115ea50",
    ),
    (
        "crates/ep_runtime/src/heat_balance/state.rs",
        "ce0fc15586ec35ef4709c24fba8133c2655bbfb8cd323bfbb41bff476c3497a1",
    ),
    (
        "crates/ep_runtime/src/heat_balance/ctf.rs",
        "b79c21fd3126a460c17cd54ec5f66d37e723e0348ac6b6af9e9fedd9e78ea3ec",
    ),
    (
        "crates/ep_runtime/examples/clk02_probe_support/digest.rs",
        "23af1d1177b51ba0b7b321b8cadffeb2a937868ad04ebf3851971ecbf5defc74",
    ),
];
pub(super) const OBSERVER_PATHS: &[&str] = &[
    "crates/ep_run/examples/ctf01_existing_observer.rs",
    "crates/ep_run/examples/ctf01_existing_support/fields.rs",
];
