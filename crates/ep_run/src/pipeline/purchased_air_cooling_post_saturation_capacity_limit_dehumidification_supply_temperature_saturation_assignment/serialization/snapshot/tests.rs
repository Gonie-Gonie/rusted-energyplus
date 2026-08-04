use super::*;

const SNAPSHOT_SERIALIZER_SOURCE: &str = include_str!("../snapshot.rs");

#[test]
fn cp414_snapshot_serializer_declares_144_unique_json_entries_and_32_sidecars() {
    let entries = SNAPSHOT_SERIALIZER_SOURCE
        .lines()
        .filter(|line| line.starts_with("        \"") && line.contains("\": "))
        .count();
    let sidecars = SNAPSHOT_SERIALIZER_SOURCE.matches("_ieee_bits\"").count();
    assert_eq!((entries, sidecars), (144, 32));
}

#[test]
fn cp414_terminal_predecessor_and_result_keys_keep_canonical_order() {
    let predecessor = SNAPSHOT_SERIALIZER_SOURCE
        .find("\"predecessor_cp413_resulting_supply_temperature_c\"")
        .expect("CP413 predecessor temperature key");
    let assignment = SNAPSHOT_SERIALIZER_SOURCE
        .find("\"post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_executed\"")
        .expect("CP414 assignment marker");
    let result = SNAPSHOT_SERIALIZER_SOURCE
        .find("\"resulting_supply_temperature_c\"")
        .expect("CP414 resulting temperature key");
    assert!(predecessor < assignment && assignment < result);
}

#[test]
fn nonfinite_json_projection_retains_authoritative_bits() {
    let value = f64::from_bits(0x7ff8_0000_0000_0414);
    assert!(json_number(Some(value)).is_null());
    assert_eq!(
        ieee_bits(Some(value)).as_deref(),
        Some("0x7ff8000000000414")
    );
    assert!(ieee_bits(None).is_none());
}
