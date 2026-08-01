//! CP390 exhaustive route and checked-accounting tests.

use super::*;
use crate::ideal_loads::calc::cooling_positive_supply_temperature_mixed_air_limit::source_shaped_two_argument_minimum;

#[test]
fn cp390_preserves_thirty_routes_and_limits_exactly_three() {
    let chains = fixtures::all_chains();
    let mut state = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureMixedAirLimitRuntimeState::new(chains[0].cp389.system);
    let snapshots: Vec<_> = chains
        .iter()
        .map(|chain| {
            advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_state(
                &mut state,
                chain.cp389,
                chain.owner(),
            )
            .expect("CP390 route")
        })
        .collect();

    assert_eq!(snapshots.len(), 30);
    assert_eq!(state.transition_count, 30);
    assert_eq!(state.inactive_transition_count, 27);
    assert_eq!(state.dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_count, 3);
    assert_eq!(state.predecessor_route_counts, [1; 30]);
    assert_eq!(state.source_site_execution_count, 12);
    assert_eq!(state.cp389_supply_temperature_state_owner_count, 27);
    assert_eq!(state.unchanged_supply_temperature_preservation_count, 24);
    for count in [
        state.supply_temperature_owned_read_count,
        state.supply_temperature_for_minimum_read_count,
        state.mixed_air_temperature_owned_read_count,
        state.mixed_air_temperature_bit_corroboration_count,
        state.mixed_air_temperature_for_minimum_read_count,
        state.source_shaped_two_argument_minimum_evaluation_count,
        state.supply_temperature_assignment_write_count,
    ] {
        assert_eq!(count, 3);
    }
    assert_eq!(
        snapshots
            .iter()
            .filter(|snapshot| cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release(**snapshot))
            .count(),
        11,
    );

    for (index, (chain, snapshot)) in chains.into_iter().zip(snapshots).enumerate() {
        assert!(cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_snapshot_is_exact(snapshot));
        assert_eq!(
            snapshot.preexisting_supply_temperature_c.is_some(),
            index >= 3
        );
        assert_eq!(
            snapshot.preexisting_supply_temperature_c.map(f64::to_bits),
            chain.cp389.resulting_supply_temperature_c.map(f64::to_bits),
        );
        assert_eq!(
            snapshot
                .resulting_supply_enthalpy_j_per_kg
                .map(f64::to_bits),
            chain
                .cp389
                .resulting_supply_enthalpy_j_per_kg
                .map(f64::to_bits),
        );
        if matches!(index, 18 | 22 | 28) {
            let left = chain.cp389.resulting_supply_temperature_c.expect("left");
            let right = chain.cp389.mixed_air_temperature_c.expect("right");
            let expected = source_shaped_two_argument_minimum(left, right);
            assert_eq!(
                snapshot.resulting_supply_temperature_c.map(f64::to_bits),
                Some(expected.to_bits()),
            );
        } else {
            assert_eq!(
                snapshot.resulting_supply_temperature_c.map(f64::to_bits),
                snapshot.preexisting_supply_temperature_c.map(f64::to_bits),
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
    type State = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureMixedAirLimitRuntimeState;
    let setters: &[fn(&mut State)] = &[
        |state| state.transition_count = usize::MAX,
        |state| state.predecessor_route_counts[18] = usize::MAX,
        |state| state.cp389_supply_temperature_state_owner_count = usize::MAX,
        |state| {
            state.dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_count = usize::MAX
        },
        |state| state.source_site_execution_count = usize::MAX,
        |state| state.supply_temperature_owned_read_count = usize::MAX,
        |state| state.supply_temperature_for_minimum_read_count = usize::MAX,
        |state| state.mixed_air_temperature_owned_read_count = usize::MAX,
        |state| state.mixed_air_temperature_bit_corroboration_count = usize::MAX,
        |state| state.mixed_air_temperature_for_minimum_read_count = usize::MAX,
        |state| state.source_shaped_two_argument_minimum_evaluation_count = usize::MAX,
        |state| state.supply_temperature_assignment_write_count = usize::MAX,
    ];
    for set_overflow in setters {
        let mut state = State::new(chain.cp389.system);
        set_overflow(&mut state);
        let before = state.clone();
        assert!(advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_state(
            &mut state,
            chain.cp389,
            chain.owner(),
        ).is_none());
        assert_eq!(state, before);
    }
}

#[test]
fn inactive_counter_overflow_rejects_before_mutation() {
    let chain = fixtures::chain(3, 0, false, None, 1, 0.7, 18.0, 1.0);
    type State = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureMixedAirLimitRuntimeState;
    for set_overflow in [
        |state: &mut State| state.inactive_transition_count = usize::MAX,
        |state: &mut State| state.unchanged_supply_temperature_preservation_count = usize::MAX,
    ] {
        let mut state = State::new(chain.cp389.system);
        set_overflow(&mut state);
        let before = state.clone();
        assert!(advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_state(
            &mut state,
            chain.cp389,
            None,
        ).is_none());
        assert_eq!(state, before);
    }
}
