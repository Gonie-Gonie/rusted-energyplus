use super::*;

const SNAPSHOT_SERIALIZER_SOURCE: &str = include_str!("../snapshot.rs");

#[test]
fn cp417_snapshot_serializer_declares_216_unique_json_entries_and_54_sidecars() {
    let entries = SNAPSHOT_SERIALIZER_SOURCE
        .lines()
        .filter(|line| line.starts_with("        \"") && line.contains("\": "))
        .count();
    let sidecars = SNAPSHOT_SERIALIZER_SOURCE.matches("_ieee_bits\"").count();
    assert_eq!((entries, sidecars), (216, 54));
}

#[test]
fn cp417_terminal_predecessor_and_result_keys_keep_canonical_order() {
    let predecessor = SNAPSHOT_SERIALIZER_SOURCE
        .find("\"predecessor_cp416_resulting_supply_temperature_c\"")
        .expect("CP416 predecessor temperature key");
    let assignment = SNAPSHOT_SERIALIZER_SOURCE
        .find("\"post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_executed\"")
        .expect("CP417 supply-enthalpy assignment marker");
    let result = SNAPSHOT_SERIALIZER_SOURCE
        .rfind("\"resulting_supply_temperature_c\"")
        .expect("CP417 resulting temperature key");
    assert!(predecessor < assignment && assignment < result);
}

#[test]
fn nonfinite_json_projection_retains_authoritative_bits() {
    let value = f64::from_bits(0x7ff8_0000_0000_0417);
    assert!(json_number(Some(value)).is_null());
    assert_eq!(
        ieee_bits(Some(value)).as_deref(),
        Some("0x7ff8000000000417")
    );
    assert!(ieee_bits(None).is_none());
}
