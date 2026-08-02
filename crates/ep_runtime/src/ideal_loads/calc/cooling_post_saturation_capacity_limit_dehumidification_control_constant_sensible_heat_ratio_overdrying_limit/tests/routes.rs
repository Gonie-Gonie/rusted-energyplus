//! CP391 exhaustive route and checked-accounting tests.

use super::*;
use crate::ideal_loads::calc::cooling_positive_supply_temperature_minimum_limit::source_shaped_two_argument_maximum;
use crate::psychrometrics::energyplus_psy_h_fn_tdb_w;

#[test]
fn cp391_preserves_thirty_routes_and_limits_exactly_three() {
    let chains = fixtures::all_chains();
    let mut state =
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitRuntimeState::new(
            chains[0].cp390.system,
        );
    let snapshots: Vec<_> = chains
        .iter()
        .map(|chain| {
            advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_state(
                &mut state,
                chain.cp390,
            )
            .expect("CP391 route")
        })
        .collect();

    assert_eq!(snapshots.len(), 30);
    assert_eq!(state.transition_count, 30);
    assert_eq!(state.inactive_transition_count, 27);
    assert_eq!(
        state.dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_count,
        3
    );
    assert_eq!(state.predecessor_route_counts, [1; 30]);
    assert_eq!(state.source_site_execution_count, 15);
    assert_eq!(state.cp390_supply_enthalpy_state_owner_count, 17);
    assert_eq!(state.unchanged_supply_enthalpy_preservation_count, 14);
    assert_eq!(
        snapshots
            .iter()
            .filter(|snapshot| snapshot.preexisting_supply_enthalpy_j_per_kg.is_some())
            .count(),
        17,
    );
    assert_eq!(
        snapshots
            .iter()
            .filter(|snapshot| snapshot.preexisting_supply_enthalpy_j_per_kg.is_none())
            .count(),
        13,
    );
    assert_eq!(
        snapshots
            .iter()
            .filter(|snapshot| {
                !snapshot
                    .dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_executed
                    && snapshot.preexisting_supply_enthalpy_j_per_kg.is_some()
                    && snapshot
                        .resulting_supply_enthalpy_j_per_kg
                        .map(f64::to_bits)
                        == snapshot
                            .preexisting_supply_enthalpy_j_per_kg
                            .map(f64::to_bits)
            })
            .count(),
        14,
    );
    for count in [
        state.supply_enthalpy_owned_read_count,
        state.supply_enthalpy_for_overdrying_limit_maximum_read_count,
        state.supply_temperature_owned_read_count,
        state.supply_temperature_for_minimum_humidity_ratio_enthalpy_read_count,
        state.psychrometric_minimum_supply_enthalpy_evaluation_count,
        state.source_shaped_two_argument_maximum_evaluation_count,
        state.supply_enthalpy_assignment_write_count,
    ] {
        assert_eq!(count, 3);
    }
    assert_eq!(
        snapshots
            .iter()
            .filter(|snapshot| {
                cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_snapshot_is_exact_direct_release(
                    **snapshot,
                )
            })
            .count(),
        11,
    );
    assert_eq!(
        snapshots
            .iter()
            .filter(|snapshot| {
                !cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_snapshot_is_exact_direct_release(
                    **snapshot,
                )
            })
            .count(),
        19,
    );

    for (index, (chain, snapshot)) in chains.into_iter().zip(snapshots).enumerate() {
        assert!(
            cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_snapshot_is_exact(
                snapshot,
            )
        );
        assert_eq!(
            cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_snapshot_is_exact_direct_release(
                snapshot,
            ),
            matches!(index, 0..=8 | 20 | 24),
        );
        let has_enthalpy = matches!(index, 5 | 8 | 11 | 14 | 17..=29);
        assert_eq!(
            snapshot.preexisting_supply_enthalpy_j_per_kg.is_some(),
            has_enthalpy
        );
        assert_eq!(
            snapshot
                .preexisting_supply_enthalpy_j_per_kg
                .map(f64::to_bits),
            chain
                .cp390
                .resulting_supply_enthalpy_j_per_kg
                .map(f64::to_bits),
        );
        assert_eq!(
            snapshot.resulting_supply_temperature_c.map(f64::to_bits),
            chain.cp390.resulting_supply_temperature_c.map(f64::to_bits),
        );
        if matches!(index, 18 | 22 | 28) {
            let left = chain
                .cp390
                .resulting_supply_enthalpy_j_per_kg
                .expect("active enthalpy");
            let temperature = chain
                .cp390
                .resulting_supply_temperature_c
                .expect("active temperature");
            let psychrometric = energyplus_psy_h_fn_tdb_w(temperature, 1.0e-5);
            let expected = source_shaped_two_argument_maximum(left, psychrometric);
            assert_eq!(
                snapshot
                    .resulting_supply_enthalpy_j_per_kg
                    .map(f64::to_bits),
                Some(expected.to_bits()),
            );
        } else {
            assert_eq!(
                snapshot
                    .resulting_supply_enthalpy_j_per_kg
                    .map(f64::to_bits),
                snapshot
                    .preexisting_supply_enthalpy_j_per_kg
                    .map(f64::to_bits),
            );
        }
    }
}

#[test]
fn every_active_counter_overflow_rejects_before_mutation() {
    let chain = fixtures::chain(
        3,
        1,
        true,
        Some(D::ConstantSensibleHeatRatio),
        1,
        0.7,
        18.0,
        1.0,
    );
    type State = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitRuntimeState;
    let setters: &[fn(&mut State)] = &[
        |state| state.transition_count = usize::MAX,
        |state| state.predecessor_route_counts[18] = usize::MAX,
        |state| state.cp390_supply_enthalpy_state_owner_count = usize::MAX,
        |state| {
            state.dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_count =
                usize::MAX
        },
        |state| state.source_site_execution_count = usize::MAX,
        |state| state.supply_enthalpy_owned_read_count = usize::MAX,
        |state| state.supply_enthalpy_for_overdrying_limit_maximum_read_count = usize::MAX,
        |state| state.supply_temperature_owned_read_count = usize::MAX,
        |state| {
            state.supply_temperature_for_minimum_humidity_ratio_enthalpy_read_count = usize::MAX
        },
        |state| state.psychrometric_minimum_supply_enthalpy_evaluation_count = usize::MAX,
        |state| state.source_shaped_two_argument_maximum_evaluation_count = usize::MAX,
        |state| state.supply_enthalpy_assignment_write_count = usize::MAX,
    ];
    for set_overflow in setters {
        let mut state = State::new(chain.cp390.system);
        set_overflow(&mut state);
        let before = state.clone();
        assert!(
            advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_state(
                &mut state,
                chain.cp390,
            )
            .is_none()
        );
        assert_eq!(state, before);
    }
}

#[test]
fn inactive_owner_and_preservation_counter_overflow_reject_before_mutation() {
    type State = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitRuntimeState;
    let chain = fixtures::chain(3, 1, false, None, 1, 0.7, 18.0, 1.0);
    for set_overflow in [
        (|state: &mut State| state.inactive_transition_count = usize::MAX) as fn(&mut State),
        |state| state.cp390_supply_enthalpy_state_owner_count = usize::MAX,
        |state| state.unchanged_supply_enthalpy_preservation_count = usize::MAX,
    ] {
        let mut state = State::new(chain.cp390.system);
        set_overflow(&mut state);
        let before = state.clone();
        assert!(
            advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_state(
                &mut state,
                chain.cp390,
            )
            .is_none()
        );
        assert_eq!(state, before);
    }
}
