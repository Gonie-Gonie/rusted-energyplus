//! CP374 active and skipped route tests.

use super::*;

#[test]
fn cp374_all_eight_cp373_routes_form_an_exact_partition() {
    let predecessors = [
        cp373_from_cp369(
            inherited_skip_cp369(0),
            DehumidificationControlType::None,
            HumidificationControlType::None,
            None,
            None,
        ),
        cp373_from_cp369(
            inherited_skip_cp369(1),
            DehumidificationControlType::None,
            HumidificationControlType::None,
            None,
            None,
        ),
        cp373_from_cp369(
            inherited_skip_cp369(2),
            DehumidificationControlType::None,
            HumidificationControlType::None,
            None,
            None,
        ),
        cp373_from_cp369(
            heating_guard_false_cp369(),
            DehumidificationControlType::None,
            HumidificationControlType::Humidistat,
            None,
            None,
        ),
        skipped_cp373(),
        active_cp373(DehumidificationControlType::Humidistat, 0.001, 0.5, 0.004),
        active_cp373(DehumidificationControlType::None, 0.001, 0.5, 0.004),
        cp373(
            DehumidificationControlType::ConstantSensibleHeatRatio,
            HumidificationControlType::Humidistat,
            None,
            None,
        ),
    ];

    for (expected_route, predecessor) in predecessors.into_iter().enumerate() {
        let active = matches!(expected_route, 5 | 6);
        let mut state = State::new(predecessor.system);
        let snapshot = advance(
            &mut state,
            predecessor,
            active.then_some(ActiveOperands {
                maximum_heating_supply_air_humidity_ratio: 0.005,
            }),
        )
        .expect("valid CP374 route");
        let counts = [
            state.unit_off_skip_count,
            state.non_cooling_skip_count,
            state.positive_guard_false_fallthrough_skip_count,
            state.heating_availability_guard_false_fallthrough_count,
            state.humidification_control_guard_false_fallthrough_count,
            state.dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_maximum_limit_count,
            state.dehumidification_control_none_supply_humidity_ratio_for_humidification_maximum_limit_count,
            state.dehumidification_control_guard_false_fallthrough_count,
        ];
        assert_eq!(counts.iter().sum::<usize>(), 1);
        assert_eq!(counts[expected_route], 1);
        assert_eq!(state.source_site_execution_count, usize::from(active) * 4);
        assert_eq!(
            snapshot.supply_humidity_ratio_for_humidification_for_maximum_limit_minimum_read,
            active
        );
        assert_eq!(
            snapshot.maximum_heating_supply_air_humidity_ratio_for_minimum_read,
            active
        );
        assert_eq!(
            snapshot.source_shaped_two_argument_minimum_evaluated,
            active
        );
        assert_eq!(
            snapshot.supply_humidity_ratio_for_humidification_assignment_performed,
            active
        );
    }
}

#[test]
fn cp374_humidistat_and_none_routes_remain_distinct() {
    for selector in [
        DehumidificationControlType::Humidistat,
        DehumidificationControlType::None,
    ] {
        let predecessor = active_cp373(selector, 0.001, 0.5, 0.004);
        let mut state = State::new(predecessor.system);
        let snapshot = advance(
            &mut state,
            predecessor,
            Some(ActiveOperands {
                maximum_heating_supply_air_humidity_ratio: 0.005,
            }),
        )
        .expect("active CP374 route");
        assert_eq!(
            snapshot.dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_maximum_limit_executed,
            selector == DehumidificationControlType::Humidistat
        );
        assert_eq!(
            snapshot.dehumidification_control_none_supply_humidity_ratio_for_humidification_maximum_limit_executed,
            selector == DehumidificationControlType::None
        );
    }
}

#[test]
fn cp374_operand_shape_mismatches_are_transactional() {
    let inactive = skipped_cp373();
    let mut state = State::new(inactive.system);
    let before = state.clone();
    assert!(
        advance(
            &mut state,
            inactive,
            Some(ActiveOperands {
                maximum_heating_supply_air_humidity_ratio: 0.01,
            }),
        )
        .is_none()
    );
    assert_eq!(state, before);

    let active = active_cp373(DehumidificationControlType::None, 0.001, 0.5, 0.004);
    let mut state = State::new(active.system);
    let before = state.clone();
    assert!(advance(&mut state, active, None).is_none());
    assert_eq!(state, before);
}
