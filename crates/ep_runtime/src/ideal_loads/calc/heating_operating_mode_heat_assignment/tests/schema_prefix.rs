//! Exact CP431 prefix and CP432 schema/order tests.

use super::*;

#[test]
fn cp432_schema_is_exact_351_125_4_3_with_cp431_first_339_and_locked_tail() {
    let cp431 = public_fields(include_str!("../../heating_mode_guard.rs"));
    let cp432 = public_fields(include_str!("../../heating_operating_mode_heat_assignment.rs"));
    assert_eq!(cp431.len(), 342);
    assert_eq!(cp432.len(), 351);
    assert_eq!(&cp432[..339], &cp431[..339]);
    assert_eq!(
        &cp432[339..],
        &[
            "predecessor_cp431_resulting_supply_humidity_ratio",
            "predecessor_cp431_resulting_supply_enthalpy_j_per_kg",
            "predecessor_cp431_resulting_supply_temperature_c",
            "heating_operating_mode_heat_assignment_executed",
            "cp431_retained_supply_humidity_ratio_state_owned",
            "cp431_retained_supply_enthalpy_state_owned",
            "cp431_retained_supply_temperature_state_owned",
            "heating_operating_mode_heat_assignment_performed",
            "assigned_heating_operating_mode",
            "resulting_supply_humidity_ratio",
            "resulting_supply_enthalpy_j_per_kg",
            "resulting_supply_temperature_c",
        ],
    );
    let source = snapshot_block(include_str!("../../heating_operating_mode_heat_assignment.rs"));
    assert_eq!(source.matches("Option<f64>").count(), 125);
    assert_eq!(source.matches("Option<bool>").count(), 4);
    assert_eq!(source.matches("Option<DehumidificationControlType>").count(), 1);
    assert_eq!(
        source
            .matches("Option<PurchasedAirTemperatureControlType>")
            .count(),
        1,
    );
    assert_eq!(source.matches("Option<IdealLoadsSensibleMode>").count(), 1);
}

#[test]
fn predecessor_reconstruction_and_cold_validated_paths_are_bit_exact_for_all_61_routes() {
    for predecessor in cp431_all_snapshots_for_successor_tests() {
        let snapshot = advance(&mut State::new(predecessor.system), predecessor).expect("CP432");
        let reconstructed =
            super::super::heating_operating_mode_heat_assignment_predecessor_cp431_snapshot(
                snapshot,
            );
        assert!(crate::ideal_loads::heating_mode_guard_snapshots_match_bit_exact(
            reconstructed,
            predecessor,
        ));
        assert_bits(
            snapshot.predecessor_cp431_resulting_supply_humidity_ratio,
            predecessor.resulting_supply_humidity_ratio,
        );
        assert_bits(
            snapshot.predecessor_cp431_resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        );
        assert_bits(
            snapshot.predecessor_cp431_resulting_supply_temperature_c,
            predecessor.resulting_supply_temperature_c,
        );
        assert!(super::super::heating_operating_mode_heat_assignment_snapshots_match_bit_exact(
            snapshot,
            snapshot,
        ));
    }
}

fn public_fields(source: &'static str) -> Vec<&'static str> {
    snapshot_block(source)
        .lines()
        .filter_map(|line| line.trim().strip_prefix("pub "))
        .filter_map(|line| line.split_once(':').map(|(field, _)| field))
        .collect()
}

fn snapshot_block(source: &'static str) -> &'static str {
    let start = source
        .find("pub struct PurchasedAirCalc")
        .expect("snapshot start");
    let tail = &source[start..];
    let end = tail.find("\n}\n").expect("snapshot end") + 3;
    &tail[..end]
}
