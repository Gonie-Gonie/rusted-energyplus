use super::{C0, CSH, H, N, P, Q, U, operands, predecessor};
use crate::ideal_loads::calc::cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit::{
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_MINIMUM_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_MINIMUM_LIMIT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_MINIMUM_LIMIT_SOURCE_ORDER,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitActiveOperands as Operands,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitRuntimeState as State,
    advance_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_state as advance,
};
use ep_model::IdealLoadsAirSystemId;

#[test]
fn source_boundary_four_sites_and_seven_routes_are_exact() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_MINIMUM_LIMIT_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2231"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_MINIMUM_LIMIT_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2232"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_MINIMUM_LIMIT_SOURCE_ORDER,
        &[
            "read-local-supply-humidity-ratio-for-dehumidification-for-humidistat-minimum-limit-maximum",
            "read-purchased-air-minimum-cooling-supply-air-humidity-ratio-for-humidistat-minimum-limit-maximum",
            "apply-source-shaped-two-argument-maximum-for-humidistat-minimum-limit",
            "assign-local-supply-humidity-ratio-for-dehumidification-for-humidistat-minimum-limit",
        ]
    );

    let mut state = State::new(IdealLoadsAirSystemId(7));
    for (index, route) in [U, N, P, C0, Q, H, CSH].into_iter().enumerate() {
        let predecessor = predecessor(route, index + 1, 0.006);
        let left = predecessor.resulting_supply_humidity_ratio_for_dehumidification;
        let snapshot =
            advance(&mut state, predecessor, operands(route, 0.0077)).expect("CP361 route");
        let active = route == H;
        assert_eq!(
            snapshot
                .dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_executed,
            active
        );
        for flag in [
            snapshot.supply_humidity_ratio_for_dehumidification_for_minimum_limit_maximum_read,
            snapshot.minimum_cooling_supply_air_humidity_ratio_for_maximum_read,
            snapshot.source_shaped_two_argument_maximum_evaluated,
            snapshot.supply_humidity_ratio_for_dehumidification_assignment_performed,
        ] {
            assert_eq!(flag, active);
        }
        for value in [
            snapshot.predecessor_resulting_supply_humidity_ratio_for_dehumidification,
            snapshot.supply_humidity_ratio_for_dehumidification_before_minimum_limit,
            snapshot.minimum_cooling_supply_air_humidity_ratio,
            snapshot.maximum_supply_humidity_ratio_for_dehumidification,
            snapshot.assigned_supply_humidity_ratio_for_dehumidification,
            snapshot.resulting_supply_humidity_ratio_for_dehumidification,
        ] {
            assert_eq!(value.is_some(), active);
        }
        if active {
            assert_eq!(
                snapshot
                    .supply_humidity_ratio_for_dehumidification_before_minimum_limit
                    .map(f64::to_bits),
                left.map(f64::to_bits)
            );
            assert_eq!(
                snapshot
                    .resulting_supply_humidity_ratio_for_dehumidification
                    .expect("result")
                    .to_bits(),
                0.0077_f64.to_bits()
            );
        }
    }
    assert_eq!(state.transition_count, 7);
    assert_eq!(state.source_site_execution_count, 4);
    assert_eq!(
        [
            state.supply_humidity_ratio_for_dehumidification_for_minimum_limit_maximum_read_count,
            state.minimum_cooling_supply_air_humidity_ratio_for_maximum_read_count,
            state.source_shaped_two_argument_maximum_evaluation_count,
            state.supply_humidity_ratio_for_dehumidification_assignment_count,
        ],
        [1; 4]
    );
}

#[test]
fn active_operand_contract_and_predecessor_shape_are_transactional() {
    let mut state = State::new(IdealLoadsAirSystemId(7));
    let before = state.clone();
    assert!(advance(&mut state, predecessor(H, 1, 0.006), None).is_none());
    assert_eq!(state, before);

    let supplied_on_skip = Some(Operands {
        minimum_cooling_supply_air_humidity_ratio: 0.0077,
    });
    assert!(advance(&mut state, predecessor(C0, 1, 0.006), supplied_on_skip).is_none());
    assert_eq!(state, before);

    let mut forged = predecessor(H, 1, 0.006);
    forged.assigned_supply_humidity_ratio_for_dehumidification = Some(0.007);
    assert!(advance(&mut state, forged, operands(H, 0.0077)).is_none());
    assert_eq!(state, before);
}
