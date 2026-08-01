//! CP389 exhaustive route, overwrite/preservation, and checked-accounting tests.

use super::*;

#[test]
fn cp389_has_twenty_seven_retained_temperatures_and_three_exact_overwrites() {
    let first = fixtures::chain(0, 0, false, None, 1, 0.7, 18.0, 1.0);
    let mut state = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentRuntimeState::new(first.cp388.system);
    let mut snapshots = Vec::new();
    let mut ordinal = 1;

    for inherited in 0..3 {
        let chain = fixtures::chain(inherited, 0, false, None, ordinal, 0.7, 18.0, 1.0);
        snapshots.push(advance(&mut state, chain));
        ordinal += 1;
    }
    for inherited in 3..8 {
        for outcome in [0, 2, 1] {
            let chain = fixtures::chain(inherited, outcome, false, None, ordinal, 0.7, 18.0, 1.0);
            snapshots.push(advance(&mut state, chain));
            ordinal += 1;
        }
    }
    let selectors = [
        D::ConstantSensibleHeatRatio,
        D::Humidistat,
        D::None,
        D::ConstantSupplyHumidityRatio,
    ];
    for inherited in [3, 4] {
        for selector in selectors {
            let chain =
                fixtures::chain(inherited, 1, true, Some(selector), ordinal, 0.7, 18.0, 1.0);
            snapshots.push(advance(&mut state, chain));
            ordinal += 1;
        }
    }
    for (inherited, selectors) in [
        (5, &[D::Humidistat][..]),
        (6, &[D::None][..]),
        (
            7,
            &[D::ConstantSensibleHeatRatio, D::ConstantSupplyHumidityRatio][..],
        ),
    ] {
        for selector in selectors {
            let chain = fixtures::chain(
                inherited,
                1,
                true,
                Some(*selector),
                ordinal,
                0.65,
                19.0,
                1.0,
            );
            snapshots.push(advance(&mut state, chain));
            ordinal += 1;
        }
    }

    assert_eq!(snapshots.len(), 30);
    assert_eq!(state.transition_count, 30);
    assert_eq!(state.inactive_transition_count, 27);
    assert_eq!(state.dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_count, 3);
    assert_eq!(state.predecessor_route_counts, [1; 30]);
    assert_eq!(state.source_site_execution_count, 24);
    assert_eq!(state.cp379_supply_temperature_state_owner_count, 27);
    assert_eq!(state.unchanged_supply_temperature_preservation_count, 24);
    for count in [
        state.mixed_air_temperature_owned_read_count,
        state.cooling_sensible_output_owned_read_count,
        state.cp_air_owned_read_count,
        state.supply_mass_flow_rate_owned_read_count,
        state.supply_mass_flow_rate_bit_corroboration_count,
        state.air_capacity_rate_calculation_count,
        state.sensible_temperature_drop_calculation_count,
        state.supply_temperature_calculation_count,
        state.supply_temperature_assignment_write_count,
    ] {
        assert_eq!(count, 3);
    }
    assert_eq!(
        snapshots
            .iter()
            .filter(|snapshot| cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_snapshot_is_exact_direct_release(**snapshot))
            .count(),
        11,
    );

    for (index, snapshot) in snapshots.into_iter().enumerate() {
        assert!(cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_snapshot_is_exact(snapshot));
        assert_eq!(
            snapshot.preexisting_supply_temperature_c.is_some(),
            index >= 3
        );
        if matches!(index, 18 | 22 | 28) {
            let cp_air = snapshot.cp_air_j_per_kg_k.expect("CpAir");
            let flow = snapshot.supply_mass_flow_rate_kg_per_s.expect("flow");
            let sensible = snapshot.cooling_sensible_output_w.expect("sensible");
            let mixed = snapshot.mixed_air_temperature_c.expect("mixed");
            let denominator = cp_air * flow;
            let drop = sensible / denominator;
            assert_eq!(
                snapshot
                    .cp_air_times_supply_mass_flow_rate_w_per_k
                    .map(f64::to_bits),
                Some(denominator.to_bits())
            );
            assert_eq!(
                snapshot
                    .cooling_sensible_output_over_air_capacity_rate_k
                    .map(f64::to_bits),
                Some(drop.to_bits())
            );
            assert_eq!(
                snapshot.resulting_supply_temperature_c.map(f64::to_bits),
                Some((mixed - drop).to_bits())
            );
        } else {
            assert_eq!(
                snapshot.resulting_supply_temperature_c.map(f64::to_bits),
                snapshot.preexisting_supply_temperature_c.map(f64::to_bits)
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
    type State = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentRuntimeState;
    let setters: &[fn(&mut State)] = &[
        |state| state.transition_count = usize::MAX,
        |state| state.predecessor_route_counts[18] = usize::MAX,
        |state| {
            state.dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_count = usize::MAX
        },
        |state| state.source_site_execution_count = usize::MAX,
        |state| state.cp379_supply_temperature_state_owner_count = usize::MAX,
        |state| state.mixed_air_temperature_owned_read_count = usize::MAX,
        |state| state.cooling_sensible_output_owned_read_count = usize::MAX,
        |state| state.cp_air_owned_read_count = usize::MAX,
        |state| state.supply_mass_flow_rate_owned_read_count = usize::MAX,
        |state| state.supply_mass_flow_rate_bit_corroboration_count = usize::MAX,
        |state| state.air_capacity_rate_calculation_count = usize::MAX,
        |state| state.sensible_temperature_drop_calculation_count = usize::MAX,
        |state| state.supply_temperature_calculation_count = usize::MAX,
        |state| state.supply_temperature_assignment_write_count = usize::MAX,
    ];
    for set_overflow in setters {
        let mut state = State::new(chain.cp388.system);
        set_overflow(&mut state);
        let before = state.clone();
        assert!(advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_state(
            &mut state,
            chain.cp388,
            chain.retained_input(),
        ).is_none());
        assert_eq!(state, before);
    }
}

fn advance(
    state: &mut PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentRuntimeState,
    chain: fixtures::Chain,
) -> PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentSnapshot{
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_state(
        state,
        chain.cp388,
        chain.retained_input(),
    )
    .expect("CP389 route")
}
