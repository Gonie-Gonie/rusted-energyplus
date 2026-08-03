//! CP404 boundary, 36-route, state-shape, and psychrometric tests.

use super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_FIRST_EXCLUDED_SOURCE as EXCLUDED,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE as SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE_ORDER as ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyHumidityRatioAssignmentRuntimeState as State,
};
use super::transition::{
    RetainedRoute, logical_route_index, predecessor_index_is_active,
    predecessor_index_is_public, source_assignment,
};
use crate::psychrometrics::energyplus_psy_w_fn_tdb_h;

#[test]
fn cp404_boundary_and_four_source_sites_are_exact() {
    assert_eq!(SOURCE, "EnergyPlus 26.1 PurchasedAirManager.cc:2299");
    assert_eq!(EXCLUDED, "EnergyPlus 26.1 PurchasedAirManager.cc:2300");
    assert_eq!(
        ORDER,
        &[
            "read-purchased-air-supply-temperature-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-latent-output-capacity-adjustment-humidity-ratio-inversion",
            "read-local-supply-enthalpy-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-latent-output-capacity-adjustment-humidity-ratio-inversion",
            "evaluate-psy-w-fn-tdb-h-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-latent-output-capacity-adjustment",
            "assign-purchased-air-supply-humidity-ratio-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-latent-output-capacity-adjustment",
        ],
    );
}

#[test]
fn thirty_six_routes_preserve_cp403_indices_and_only_six_assign() {
    let mut routes = Vec::new();
    for predecessor_index in 0..30 {
        if predecessor_index_is_active(predecessor_index) {
            routes.push(RetainedRoute {
                predecessor_index,
                guard_evaluated: true,
                assignment_executed: false,
            });
            routes.push(RetainedRoute {
                predecessor_index,
                guard_evaluated: true,
                assignment_executed: true,
            });
        } else {
            routes.push(RetainedRoute {
                predecessor_index,
                guard_evaluated: false,
                assignment_executed: false,
            });
        }
    }
    assert_eq!(routes.len(), 36);
    assert_eq!(
        routes
            .iter()
            .copied()
            .map(logical_route_index)
            .collect::<Vec<_>>(),
        (0..36).collect::<Vec<_>>(),
    );
    assert_eq!(
        routes
            .iter()
            .filter(|route| route.assignment_executed)
            .map(|route| route.predecessor_index)
            .collect::<Vec<_>>(),
        [20, 21, 24, 25, 27, 29],
    );
    assert_eq!(
        routes
            .iter()
            .copied()
            .filter(|route| route.assignment_executed)
            .map(logical_route_index)
            .collect::<Vec<_>>(),
        [21, 23, 27, 29, 32, 35],
    );
    let public = routes
        .iter()
        .filter(|route| predecessor_index_is_public(route.predecessor_index))
        .count();
    assert_eq!(public, 13);
    assert_eq!(routes.len() - public, 23);
}

#[test]
fn source_assignment_is_the_canonical_energyplus_psychrometric_helper() {
    for (temperature, enthalpy) in [
        (12.5, 31_000.0),
        (-0.0, 0.0),
        (f64::INFINITY, 1.0),
        (f64::from_bits(0x7ff8_0000_0000_0404), 42.0),
    ] {
        assert_eq!(
            source_assignment(temperature, enthalpy).to_bits(),
            energyplus_psy_w_fn_tdb_h(temperature, enthalpy).to_bits(),
        );
    }
}

#[test]
fn new_state_is_exactly_zeroed() {
    let state = State::new(ep_model::IdealLoadsAirSystemId(404));
    assert_eq!(state.transition_count, 0);
    assert_eq!(state.predecessor_route_counts, [0; 30]);
    assert_eq!(state.predecessor_guard_false_fallthrough_route_counts, [0; 30]);
    assert_eq!(state.supply_humidity_ratio_assignment_route_counts, [0; 30]);
    assert!(state.latest.is_none());
}
