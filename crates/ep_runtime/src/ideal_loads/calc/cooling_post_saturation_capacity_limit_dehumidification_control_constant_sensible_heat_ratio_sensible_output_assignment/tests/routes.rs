//! CP388 exhaustive route and checked-accounting tests.

use super::*;

#[test]
fn cp388_has_twenty_seven_inactive_and_three_constant_shr_assignment_routes() {
    let first = fixtures::chain(0, 0, false, None, 1, 99.0, 50_000.0, 0.008);
    let mut state = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentRuntimeState::new(first.cp387.system);
    let mut snapshots = Vec::new();
    let mut ordinal = 1;

    for inherited in 0..3 {
        let chain = fixtures::chain(inherited, 0, false, None, ordinal, 99.0, 50_000.0, 0.008);
        snapshots.push(advance(&mut state, chain, None));
        ordinal += 1;
    }
    for inherited in 3..8 {
        for (outcome, assignment) in [(0, false), (2, false), (1, false)] {
            let maximum = if assignment { 99.0 } else { 100.0 };
            let chain = fixtures::chain(
                inherited, outcome, assignment, None, ordinal, maximum, 50_000.0, 0.008,
            );
            snapshots.push(advance(&mut state, chain, None));
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
            let chain = fixtures::chain(
                inherited,
                1,
                true,
                Some(selector),
                ordinal,
                99.0,
                50_000.0,
                0.008,
            );
            let input =
                (selector == D::ConstantSensibleHeatRatio).then(|| fixtures::input(chain, 0.7));
            snapshots.push(advance(&mut state, chain, input));
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
                99.0,
                50_000.0,
                0.012,
            );
            let input =
                (*selector == D::ConstantSensibleHeatRatio).then(|| fixtures::input(chain, 0.65));
            snapshots.push(advance(&mut state, chain, input));
            ordinal += 1;
        }
    }

    assert_eq!(snapshots.len(), 30);
    assert_eq!(state.transition_count, 30);
    assert_eq!(state.inactive_transition_count, 27);
    assert_eq!(
        state
            .dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_count,
        3,
    );
    assert_eq!(state.predecessor_route_counts, [1; 30]);
    assert_eq!(state.source_site_execution_count, 12);
    assert_eq!(state.cooling_total_output_owned_read_count, 3);
    assert_eq!(state.cooling_total_output_bit_corroboration_count, 3);
    assert_eq!(state.cooling_sensible_heat_ratio_read_count, 3);
    assert_eq!(state.cooling_sensible_output_calculation_count, 3);
    assert_eq!(state.cooling_sensible_output_assignment_write_count, 3);
    assert_eq!(
        snapshots
            .iter()
            .filter(|snapshot| {
                cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_snapshot_is_exact_direct_release(**snapshot)
            })
            .count(),
        11,
    );
    for snapshot in snapshots {
        assert!(cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_snapshot_is_exact(snapshot));
        assert_eq!(
            snapshot
                .predecessor_resulting_supply_enthalpy_j_per_kg
                .map(f64::to_bits),
            snapshot
                .resulting_supply_enthalpy_j_per_kg
                .map(f64::to_bits),
        );
        if snapshot.dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_executed {
            let total = snapshot.cooling_total_output_w.expect("total");
            let ratio = snapshot.cooling_sensible_heat_ratio.expect("ratio");
            assert_eq!(
                snapshot.cooling_sensible_output_w.map(f64::to_bits),
                Some((total * ratio).to_bits()),
            );
        } else {
            assert!(!snapshot.cooling_total_output_read);
            assert!(snapshot.cooling_total_output_w.is_none());
            assert!(!snapshot.cooling_sensible_heat_ratio_read);
            assert!(snapshot.cooling_sensible_heat_ratio.is_none());
            assert!(!snapshot.cooling_sensible_output_calculated);
            assert!(snapshot.calculated_cooling_sensible_output_w.is_none());
            assert!(!snapshot.cooling_sensible_output_assigned);
            assert!(snapshot.cooling_sensible_output_w.is_none());
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
        99.0,
        50_000.0,
        0.008,
    );
    let setters: &[fn(&mut PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentRuntimeState)] = &[
        |state| state.transition_count = usize::MAX,
        |state| state.predecessor_route_counts[18] = usize::MAX,
        |state| state.dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_count = usize::MAX,
        |state| state.source_site_execution_count = usize::MAX,
        |state| state.cooling_total_output_owned_read_count = usize::MAX,
        |state| state.cooling_total_output_bit_corroboration_count = usize::MAX,
        |state| state.cooling_sensible_heat_ratio_read_count = usize::MAX,
        |state| state.cooling_sensible_output_calculation_count = usize::MAX,
        |state| state.cooling_sensible_output_assignment_write_count = usize::MAX,
    ];
    for set_overflow in setters {
        let mut state = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentRuntimeState::new(chain.cp387.system);
        set_overflow(&mut state);
        let before = state.clone();
        assert!(advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_state(
            &mut state,
            chain.cp387,
            Some(fixtures::input(chain, 0.7)),
        ).is_none());
        assert_eq!(state, before);
    }
}

fn advance(
    state: &mut PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentRuntimeState,
    chain: fixtures::Chain,
    input: Option<PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentActiveInput>,
) -> PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentSnapshot{
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_state(
        state,
        chain.cp387,
        input,
    )
    .expect("CP388 route")
}
