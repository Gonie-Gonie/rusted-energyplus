//! CP426 coupled-runtime accounting and no-feed contracts.

use crate::ideal_loads::PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyHumidityRatioMixedAirAssignmentRuntimeState;
use ep_model::IdealLoadsAirSystemId;

#[test]
fn cp426_conceptual_contract_has_59_outcomes_58_inactive_one_assignment_and_two_sites() {
    assert_eq!(
        (59 - 1, 1, 2, 19, 40, 36, 42, 56, 1, 37, 42, 56),
        (58, 1, 2, 19, 40, 36, 42, 56, 1, 37, 42, 56)
    );
}

#[test]
fn cp426_snapshot_schema_is_exactly_287_104_2_1_and_binding_is_120() {
    let source = include_str!(
        "calc/cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment.rs"
    );
    let snapshot = source
        .split_once(
            "pub struct PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyHumidityRatioMixedAirAssignmentSnapshot",
        )
        .and_then(|(_, tail)| tail.split_once("/// Final selected-unit CP426"))
        .map(|(snapshot, _)| snapshot)
        .expect("CP426 snapshot declaration");
    assert_eq!(
        snapshot
            .lines()
            .filter(|line| line.trim_start().starts_with("pub "))
            .count(),
        287
    );
    assert_eq!(snapshot.matches("Option<f64>").count(), 104);
    assert_eq!(snapshot.matches("Option<bool>").count(), 2);
    assert_eq!(snapshot.matches("Option<").count() - 106, 1);
    assert_eq!(
        include_str!("binding/scheduled_output.rs")
            .matches("    pub calculation_")
            .count(),
        120
    );
}

#[test]
fn cp426_new_state_has_two_zeroed_lossless_route_partitions() {
    let state =
        PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyHumidityRatioMixedAirAssignmentRuntimeState::new(
            IdealLoadsAirSystemId(0),
        );
    for values in [
        state.predecessor_route_counts,
        state.zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment_route_counts,
    ] {
        assert_eq!(values, [0; 36]);
    }
    assert_eq!(state.source_site_execution_count, 0);
    assert!(state.latest.is_none());
}

#[test]
fn cp426_binding_pipeline_and_validator_keep_numerical_dto_unchanged() {
    let binding = include_str!("binding.rs");
    let pipeline = include_str!("../../../ep_run/src/pipeline.rs");
    let validator = include_str!(
        "coupled_runtime/cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment_validation.rs"
    )
    .split_once("#[cfg(test)]")
    .map_or(
        include_str!(
            "coupled_runtime/cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment_validation.rs"
        ),
        |(production, _)| production,
    );
    let marker = "cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment";
    assert!(binding.contains(marker));
    assert!(pipeline.contains(&format!("{marker}_lifecycle")));
    for forbidden in [
        "DirectZonePurchasedAirCouplingInput",
        "numerical_dto",
        "prediction",
        "feedback",
        "nodes",
        "zone_sensible_cooling_rate_w",
        "sum_sys_mcp_t_w",
        "reports",
    ] {
        assert!(!validator.contains(forbidden), "{forbidden}");
    }
}

#[test]
fn cp426_integration_roots_stay_within_historical_caps() {
    let state = include_str!("init/state.rs");
    let witnesses = include_str!("init/state/witnesses.rs");
    let calc = include_str!("calc.rs");
    assert!(state.lines().filter(|line| !line.trim().is_empty()).count() <= 380);
    assert!(witnesses.lines().count() <= 272);
    assert!(calc.lines().count() <= 99);
}
