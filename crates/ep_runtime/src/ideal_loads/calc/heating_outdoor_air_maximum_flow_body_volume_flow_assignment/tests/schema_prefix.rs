//! CP436 flat-schema, lossless-prefix, enum, and cold/validated parity locks.

use super::*;

#[test]
fn cp436_schema_is_exact_402_140_8_6_with_cp435_first_382_and_locked_tail() {
    let cp435 = public_fields(include_str!(
        "../../heating_outdoor_air_maximum_flow_guard.rs"
    ));
    let cp436 = public_fields(include_str!(
        "../../heating_outdoor_air_maximum_flow_body_volume_flow_assignment.rs"
    ));
    assert_eq!(cp435.len(), 385);
    assert_eq!(cp436.len(), 402);
    assert_eq!(&cp436[..382], &cp435[..382]);
    assert_eq!(
        &cp436[382..],
        &[
            "predecessor_cp435_resulting_supply_humidity_ratio",
            "predecessor_cp435_resulting_supply_enthalpy_j_per_kg",
            "predecessor_cp435_resulting_supply_temperature_c",
            "heating_outdoor_air_maximum_flow_body_volume_flow_assignment_executed",
            "cp435_retained_supply_humidity_ratio_state_owned",
            "cp435_retained_supply_enthalpy_state_owned",
            "cp435_retained_supply_temperature_state_owned",
            "cp435_retained_outdoor_air_mass_flow_rate_owned_read",
            "outdoor_air_mass_flow_rate_for_outdoor_air_volume_flow_division_read",
            "outdoor_air_mass_flow_rate_for_outdoor_air_volume_flow_division_kg_per_s",
            "begin_environment_standard_air_density_owned_read",
            "standard_air_density_for_outdoor_air_volume_flow_division_read",
            "standard_air_density_for_outdoor_air_volume_flow_division_kg_per_m3",
            "outdoor_air_mass_flow_rate_standard_air_density_division_evaluated",
            "calculated_outdoor_air_volume_flow_rate_m3_per_s",
            "local_outdoor_air_volume_flow_rate_assignment_performed",
            "assigned_outdoor_air_volume_flow_rate_m3_per_s",
            "resulting_supply_humidity_ratio",
            "resulting_supply_enthalpy_j_per_kg",
            "resulting_supply_temperature_c",
        ],
    );
    let mut unique = cp436.clone();
    unique.sort_unstable();
    unique.dedup();
    assert_eq!(unique.len(), 402);

    let block = snapshot_block(include_str!(
        "../../heating_outdoor_air_maximum_flow_body_volume_flow_assignment.rs"
    ));
    assert_eq!(block.matches("Option<f64>").count(), 140);
    assert_eq!(block.matches("Option<bool>").count(), 8);
    assert_eq!(block.matches("Option<").count() - 140 - 8, 6);
}

#[test]
fn predecessor_reconstruction_and_cold_validated_paths_are_bit_exact_for_all_64_routes() {
    for predecessor in cp435_all_snapshots_for_successor_tests() {
        let predecessor_route = predecessor_route_for(predecessor);
        let route = route_for(predecessor);
        let cold = advance(&mut State::new(predecessor.system), predecessor, 2.0)
            .expect("cold CP436");
        let validated = advance_validated(
            &mut State::new(predecessor.system),
            predecessor,
            predecessor_route,
            2.0,
            route,
        )
        .expect("validated CP436");
        let reconstructed = super::super::heating_outdoor_air_maximum_flow_body_volume_flow_assignment_predecessor_cp435_snapshot(cold);
        assert!(
            crate::ideal_loads::heating_outdoor_air_maximum_flow_guard_snapshots_match_bit_exact(
                reconstructed,
                predecessor,
            )
        );
        assert!(super::super::heating_outdoor_air_maximum_flow_body_volume_flow_assignment_snapshots_match_bit_exact(
            cold,
            validated,
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
    let source = &source[start..];
    let end = source.find("\n}\n\n/// Final").expect("snapshot end");
    &source[..end]
}
