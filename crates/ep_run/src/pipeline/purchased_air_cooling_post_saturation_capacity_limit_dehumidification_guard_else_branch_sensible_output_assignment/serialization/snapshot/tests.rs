use super::*;

const CP419_SERIALIZER: &str = include_str!(
    "../../../purchased_air_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment/serialization/snapshot.rs"
);
const CP420_SERIALIZER: &str = include_str!("../snapshot.rs");

fn entries(source: &str) -> Vec<&str> {
    source
        .lines()
        .filter(|line| line.starts_with("        \"") && line.contains("\": "))
        .collect()
}

#[test]
fn cp420_preserves_cp419_prefix_and_declares_273_lossless_keys() {
    let cp419 = entries(CP419_SERIALIZER);
    let cp420 = entries(CP420_SERIALIZER);
    assert_eq!((cp419.len(), cp420.len()), (234, 273));
    assert_eq!(
        cp419[..228]
            .iter()
            .map(|line| line.split(':').next())
            .collect::<Vec<_>>(),
        cp420[..228]
            .iter()
            .map(|line| line.split(':').next())
            .collect::<Vec<_>>(),
    );
    assert_eq!(CP420_SERIALIZER.matches("_ieee_bits\"").count(), 71);
}

#[test]
fn cp420_tail_is_predecessor_then_eight_site_local_then_terminal() {
    let cp420 = entries(CP420_SERIALIZER);
    let keys = cp420[228..]
        .iter()
        .map(|line| line.trim().split(':').next().unwrap_or_default())
        .collect::<Vec<_>>();
    assert_eq!(keys.len(), 45);
    assert_eq!(
        &keys[..6],
        &[
            "\"predecessor_cp419_resulting_supply_humidity_ratio\"",
            "\"predecessor_cp419_resulting_supply_humidity_ratio_ieee_bits\"",
            "\"predecessor_cp419_resulting_supply_enthalpy_j_per_kg\"",
            "\"predecessor_cp419_resulting_supply_enthalpy_j_per_kg_ieee_bits\"",
            "\"predecessor_cp419_resulting_supply_temperature_c\"",
            "\"predecessor_cp419_resulting_supply_temperature_c_ieee_bits\"",
        ],
    );
    assert_eq!(
        keys[6],
        "\"post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_executed\""
    );
    assert_eq!(keys[38], "\"cooling_sensible_output_w_ieee_bits\"");
    assert_eq!(
        &keys[39..],
        &[
            "\"resulting_supply_humidity_ratio\"",
            "\"resulting_supply_humidity_ratio_ieee_bits\"",
            "\"resulting_supply_enthalpy_j_per_kg\"",
            "\"resulting_supply_enthalpy_j_per_kg_ieee_bits\"",
            "\"resulting_supply_temperature_c\"",
            "\"resulting_supply_temperature_c_ieee_bits\"",
        ],
    );
}

#[test]
fn nonfinite_projection_keeps_authoritative_bits() {
    let value = f64::from_bits(0x7ff8_0000_0000_0420);
    assert!(json_number(Some(value)).is_null());
    assert_eq!(
        ieee_bits(Some(value)).as_deref(),
        Some("0x7ff8000000000420")
    );
    assert!(ieee_bits(None).is_none());
}
