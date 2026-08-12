use super::*;

const CP418_SNAPSHOT_SERIALIZER_SOURCE: &str = include_str!(
    "../../../purchased_air_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry/serialization/snapshot.rs"
);
const SNAPSHOT_SERIALIZER_SOURCE: &str = include_str!("../snapshot.rs");

#[test]
fn cp419_snapshot_serializer_retains_cp418_prefix_through_field_159_and_declares_234_lossless_keys()
{
    let cp418_entries: Vec<_> = CP418_SNAPSHOT_SERIALIZER_SOURCE
        .lines()
        .filter(|line| line.starts_with("        \"") && line.contains("\": "))
        .collect();
    let cp419_entries: Vec<_> = SNAPSHOT_SERIALIZER_SOURCE
        .lines()
        .filter(|line| line.starts_with("        \"") && line.contains("\": "))
        .collect();
    assert_eq!((cp418_entries.len(), cp419_entries.len()), (217, 234));
    assert_eq!(
        cp418_entries[..210]
            .iter()
            .map(|line| line.split(':').next())
            .collect::<Vec<_>>(),
        cp419_entries[..210]
            .iter()
            .map(|line| line.split(':').next())
            .collect::<Vec<_>>(),
    );
    assert_eq!(
        cp419_entries[210..]
            .iter()
            .map(|line| line.trim().split(':').next().unwrap_or_default())
            .collect::<Vec<_>>(),
        [
            "\"predecessor_cp418_resulting_supply_humidity_ratio\"",
            "\"predecessor_cp418_resulting_supply_humidity_ratio_ieee_bits\"",
            "\"predecessor_cp418_resulting_supply_enthalpy_j_per_kg\"",
            "\"predecessor_cp418_resulting_supply_enthalpy_j_per_kg_ieee_bits\"",
            "\"predecessor_cp418_resulting_supply_temperature_c\"",
            "\"predecessor_cp418_resulting_supply_temperature_c_ieee_bits\"",
            "\"predecessor_post_saturation_capacity_limit_dehumidification_guard_else_branch_entered\"",
            "\"post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_executed\"",
            "\"cp329_retained_mixed_air_humidity_ratio_owned_read\"",
            "\"mixed_air_humidity_ratio_for_cp_air_read\"",
            "\"mixed_air_humidity_ratio_for_cp_air\"",
            "\"mixed_air_humidity_ratio_for_cp_air_ieee_bits\"",
            "\"psychrometric_cp_air_evaluated\"",
            "\"psychrometric_cp_air_result_j_per_kg_k\"",
            "\"psychrometric_cp_air_result_j_per_kg_k_ieee_bits\"",
            "\"cp_air_assigned\"",
            "\"cp_air_j_per_kg_k\"",
            "\"cp_air_j_per_kg_k_ieee_bits\"",
            "\"resulting_supply_humidity_ratio\"",
            "\"resulting_supply_humidity_ratio_ieee_bits\"",
            "\"resulting_supply_enthalpy_j_per_kg\"",
            "\"resulting_supply_enthalpy_j_per_kg_ieee_bits\"",
            "\"resulting_supply_temperature_c\"",
            "\"resulting_supply_temperature_c_ieee_bits\"",
        ],
    );
    assert_eq!(
        SNAPSHOT_SERIALIZER_SOURCE.matches("_ieee_bits\"").count(),
        60
    );
}

#[test]
fn nonfinite_json_projection_retains_authoritative_bits() {
    let value = f64::from_bits(0x7ff8_0000_0000_0419);
    assert!(json_number(Some(value)).is_null());
    assert_eq!(
        ieee_bits(Some(value)).as_deref(),
        Some("0x7ff8000000000419")
    );
    assert!(ieee_bits(None).is_none());
}
