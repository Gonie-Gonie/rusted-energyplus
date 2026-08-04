use super::*;

const SNAPSHOT_SERIALIZER_SOURCE: &str = include_str!("../snapshot.rs");

#[test]
fn cp415_snapshot_serializer_declares_168_unique_json_entries_and_40_sidecars() {
    let entries = SNAPSHOT_SERIALIZER_SOURCE
        .lines()
        .filter(|line| line.starts_with("        \"") && line.contains("\": "))
        .count();
    let sidecars = SNAPSHOT_SERIALIZER_SOURCE.matches("_ieee_bits\"").count();
    assert_eq!((entries, sidecars), (168, 40));
}

#[test]
fn cp415_terminal_predecessor_and_result_keys_keep_canonical_order() {
    let predecessor = SNAPSHOT_SERIALIZER_SOURCE
        .find("\"predecessor_cp414_resulting_supply_temperature_c\"")
        .expect("CP414 predecessor temperature key");
    let limit = SNAPSHOT_SERIALIZER_SOURCE
        .find("\"post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_executed\"")
        .expect("CP415 mixed-air-limit marker");
    let result = SNAPSHOT_SERIALIZER_SOURCE
        .rfind("\"resulting_supply_temperature_c\"")
        .expect("CP415 resulting temperature key");
    assert!(predecessor < limit && limit < result);
}

#[test]
fn nonfinite_json_projection_retains_authoritative_bits() {
    let value = f64::from_bits(0x7ff8_0000_0000_0415);
    assert!(json_number(Some(value)).is_null());
    assert_eq!(
        ieee_bits(Some(value)).as_deref(),
        Some("0x7ff8000000000415")
    );
    assert!(ieee_bits(None).is_none());
}
