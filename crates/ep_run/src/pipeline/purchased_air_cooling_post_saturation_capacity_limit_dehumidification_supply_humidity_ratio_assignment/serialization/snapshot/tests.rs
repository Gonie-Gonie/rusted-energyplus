use super::*;

const SNAPSHOT_SERIALIZER_SOURCE: &str = include_str!("../snapshot.rs");

#[test]
fn cp416_snapshot_serializer_declares_192_unique_json_entries_and_47_sidecars() {
    let entries = SNAPSHOT_SERIALIZER_SOURCE
        .lines()
        .filter(|line| line.starts_with("        \"") && line.contains("\": "))
        .count();
    let sidecars = SNAPSHOT_SERIALIZER_SOURCE.matches("_ieee_bits\"").count();
    assert_eq!((entries, sidecars), (192, 47));
}

#[test]
fn cp416_terminal_predecessor_and_result_keys_keep_canonical_order() {
    let predecessor = SNAPSHOT_SERIALIZER_SOURCE
        .find("\"predecessor_cp415_resulting_supply_temperature_c\"")
        .expect("CP415 predecessor temperature key");
    let assignment = SNAPSHOT_SERIALIZER_SOURCE
        .find("\"post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment_executed\"")
        .expect("CP416 humidity-ratio assignment marker");
    let result = SNAPSHOT_SERIALIZER_SOURCE
        .rfind("\"resulting_supply_temperature_c\"")
        .expect("CP416 resulting temperature key");
    assert!(predecessor < assignment && assignment < result);
}

#[test]
fn nonfinite_json_projection_retains_authoritative_bits() {
    let value = f64::from_bits(0x7ff8_0000_0000_0416);
    assert!(json_number(Some(value)).is_null());
    assert_eq!(
        ieee_bits(Some(value)).as_deref(),
        Some("0x7ff8000000000416")
    );
    assert!(ieee_bits(None).is_none());
}
