//! CP387 exhaustive route accounting.

use super::*;

#[test]
fn cp387_has_twenty_seven_inactive_and_three_constant_shr_assignment_routes() {
    let system = predecessor(0, 0, false, None, 1).system;
    let mut state = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentRuntimeState::new(system);
    let mut snapshots = Vec::new();
    let mut ordinal = 1;

    for inherited in 0..3 {
        let cp386 = predecessor(inherited, 0, false, None, ordinal);
        snapshots.push(
            advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_state(
                &mut state,
                cp386,
                None,
            )
            .expect("inactive base route"),
        );
        ordinal += 1;
    }
    for inherited in 3..8 {
        for (outcome, assignment) in [(0, false), (2, false), (1, false)] {
            let cp386 = predecessor(inherited, outcome, assignment, None, ordinal);
            snapshots.push(
                advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_state(
                    &mut state,
                    cp386,
                    None,
                )
                .expect("inactive lineage route"),
            );
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
            let cp386 = predecessor(inherited, 1, true, Some(selector), ordinal);
            let active_input = (selector == D::ConstantSensibleHeatRatio).then_some(input(0.008));
            snapshots.push(
                advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_state(
                    &mut state,
                    cp386,
                    active_input,
                )
                .expect("unconstrained selector route"),
            );
            ordinal += 1;
        }
    }
    for (inherited, selectors) in [
        (5, &[D::Humidistat][..]),
        (6, &[D::None][..]),
        (
            7,
            &[
                D::ConstantSensibleHeatRatio,
                D::ConstantSupplyHumidityRatio,
            ][..],
        ),
    ] {
        for selector in selectors {
            let cp386 = predecessor(inherited, 1, true, Some(*selector), ordinal);
            let active_input =
                (*selector == D::ConstantSensibleHeatRatio).then_some(input(0.012));
            snapshots.push(
                advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_state(
                    &mut state,
                    cp386,
                    active_input,
                )
                .expect("constrained selector route"),
            );
            ordinal += 1;
        }
    }

    assert_eq!(snapshots.len(), 30);
    assert_eq!(state.transition_count, 30);
    assert_eq!(state.inactive_transition_count, 27);
    assert_eq!(
        state.dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_count,
        3,
    );
    assert_eq!(state.predecessor_route_counts, [1; 30]);
    assert_eq!(state.source_site_execution_count, 12);
    assert_eq!(state.dehumidification_control_constant_sensible_heat_ratio_case_entry_count, 3);
    assert_eq!(state.mixed_air_humidity_ratio_read_count, 3);
    assert_eq!(state.psychrometric_cp_air_evaluation_count, 3);
    assert_eq!(state.cp_air_assignment_write_count, 3);
    assert_eq!(
        snapshots
            .iter()
            .filter(|snapshot| {
                cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_snapshot_is_exact_direct_release(
                    **snapshot,
                )
            })
            .count(),
        11,
    );
    for snapshot in snapshots {
        assert_eq!(
            snapshot
                .predecessor_resulting_supply_enthalpy_j_per_kg
                .map(f64::to_bits),
            snapshot.resulting_supply_enthalpy_j_per_kg.map(f64::to_bits),
        );
        if snapshot.dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed {
            let humidity = snapshot.mixed_air_humidity_ratio.expect("humidity");
            let expected = energyplus_psy_cp_air_fn_w(humidity);
            assert_eq!(
                snapshot.psychrometric_cp_air_result_j_per_kg_k.map(f64::to_bits),
                Some(expected.to_bits()),
            );
            assert_eq!(
                snapshot.cp_air_j_per_kg_k.map(f64::to_bits),
                Some(expected.to_bits()),
            );
        } else {
            assert!(!snapshot.dehumidification_control_constant_sensible_heat_ratio_case_entered);
            assert!(!snapshot.mixed_air_humidity_ratio_read);
            assert!(snapshot.mixed_air_humidity_ratio.is_none());
            assert!(!snapshot.psychrometric_cp_air_evaluated);
            assert!(snapshot.psychrometric_cp_air_result_j_per_kg_k.is_none());
            assert!(!snapshot.cp_air_assigned);
            assert!(snapshot.cp_air_j_per_kg_k.is_none());
        }
    }
}
