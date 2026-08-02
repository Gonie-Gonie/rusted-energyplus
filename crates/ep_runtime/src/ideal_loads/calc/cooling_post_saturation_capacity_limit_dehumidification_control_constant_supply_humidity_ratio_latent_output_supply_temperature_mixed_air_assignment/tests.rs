//! CP403 boundary, 36-route, state-shape, and raw-copy tests.

use super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE as EXCLUDED,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_ASSIGNMENT_SOURCE as SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_MIXED_AIR_ASSIGNMENT_SOURCE_ORDER as ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureMixedAirAssignmentRuntimeState as State,
};
use super::transition::{
    RetainedRoute, logical_route_index, predecessor_index_is_active,
    predecessor_index_is_public, source_assignment,
};

#[test]
fn cp403_boundary_and_two_source_sites_are_exact() {
    assert_eq!(SOURCE, "EnergyPlus 26.1 PurchasedAirManager.cc:2298");
    assert_eq!(EXCLUDED, "EnergyPlus 26.1 PurchasedAirManager.cc:2299");
    assert_eq!(
        ORDER,
        &[
            "read-retained-mixed-air-temperature-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-latent-output-capacity-adjustment-supply-temperature-assignment",
            "assign-purchased-air-supply-temperature-from-mixed-air-temperature-for-post-saturation-capacity-limit-none-or-constant-supply-humidity-ratio-latent-output-capacity-adjustment",
        ],
    );
}

#[test]
fn thirty_six_predecessor_routes_are_one_to_one_and_only_six_assign() {
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
    assert_eq!(
        routes
            .iter()
            .filter(|route| predecessor_index_is_public(route.predecessor_index))
            .count(),
        13,
    );
    assert_eq!(
        routes
            .iter()
            .copied()
            .filter(|route| {
                route.assignment_executed
                    && predecessor_index_is_public(route.predecessor_index)
            })
            .map(logical_route_index)
            .collect::<Vec<_>>(),
        [21, 27],
    );
}

#[test]
fn assignment_copies_arbitrary_binary64_payloads_without_a_finite_gate() {
    for bits in [
        0,
        1u64 << 63,
        1,
        f64::INFINITY.to_bits(),
        f64::NEG_INFINITY.to_bits(),
        0x7ff8_0000_0000_0403,
    ] {
        let input = f64::from_bits(bits);
        assert_eq!(source_assignment(input).to_bits(), bits);
    }
}

#[test]
fn new_state_is_exactly_zeroed() {
    let state = State::new(ep_model::IdealLoadsAirSystemId(403));
    assert_eq!(state.transition_count, 0);
    assert_eq!(state.predecessor_route_counts, [0; 30]);
    assert_eq!(state.predecessor_guard_false_fallthrough_route_counts, [0; 30]);
    assert_eq!(state.supply_temperature_mixed_air_assignment_route_counts, [0; 30]);
    assert!(state.latest.is_none());
}
