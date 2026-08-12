use super::*;

const CP417_SNAPSHOT_SERIALIZER_SOURCE: &str = include_str!(
    "../../../purchased_air_cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment/serialization/snapshot.rs"
);
const SNAPSHOT_SERIALIZER_SOURCE: &str = include_str!("../snapshot.rs");

#[test]
fn cp418_snapshot_serializer_retains_cp417_prefix_and_appends_one_final_key() {
    let cp417_entries: Vec<_> = CP417_SNAPSHOT_SERIALIZER_SOURCE
        .lines()
        .filter(|line| line.starts_with("        \"") && line.contains("\": "))
        .collect();
    let cp418_entries: Vec<_> = SNAPSHOT_SERIALIZER_SOURCE
        .lines()
        .filter(|line| line.starts_with("        \"") && line.contains("\": "))
        .collect();
    assert_eq!((cp417_entries.len(), cp418_entries.len()), (216, 217));
    assert_eq!(
        cp417_entries
            .iter()
            .map(|line| line.split(':').next())
            .collect::<Vec<_>>(),
        cp418_entries[..216]
            .iter()
            .map(|line| line.split(':').next())
            .collect::<Vec<_>>(),
    );
    assert!(
        cp418_entries[216]
            .contains("post_saturation_capacity_limit_dehumidification_guard_else_branch_entered")
    );
    assert_eq!(
        SNAPSHOT_SERIALIZER_SOURCE.matches("_ieee_bits\"").count(),
        54
    );
}

#[test]
fn nonfinite_json_projection_retains_authoritative_bits() {
    let value = f64::from_bits(0x7ff8_0000_0000_0418);
    assert!(json_number(Some(value)).is_null());
    assert_eq!(
        ieee_bits(Some(value)).as_deref(),
        Some("0x7ff8000000000418")
    );
    assert!(ieee_bits(None).is_none());
}
