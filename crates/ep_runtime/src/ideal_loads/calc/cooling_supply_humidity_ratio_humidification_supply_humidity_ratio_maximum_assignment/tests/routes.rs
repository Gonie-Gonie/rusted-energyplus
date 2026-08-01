//! CP375 active and skipped route tests.

use super::*;

#[test]
fn cp375_all_eight_cp374_routes_form_an_exact_partition() {
    let predecessors = [
        cp374_from_cp369(
            inherited_skip_cp369(0),
            DehumidificationControlType::None,
            HumidificationControlType::None,
            None,
            None,
            None,
        ),
        cp374_from_cp369(
            inherited_skip_cp369(1),
            DehumidificationControlType::None,
            HumidificationControlType::None,
            None,
            None,
            None,
        ),
        cp374_from_cp369(
            inherited_skip_cp369(2),
            DehumidificationControlType::None,
            HumidificationControlType::None,
            None,
            None,
            None,
        ),
        cp374_from_cp369(
            heating_guard_false_cp369(),
            DehumidificationControlType::None,
            HumidificationControlType::Humidistat,
            None,
            None,
            None,
        ),
        skipped_cp374(),
        active_cp374(DehumidificationControlType::Humidistat, 0.008, 0.007),
        active_cp374(DehumidificationControlType::None, 0.008, 0.007),
        cp374(
            DehumidificationControlType::ConstantSensibleHeatRatio,
            HumidificationControlType::Humidistat,
            None,
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
                purchased_air_supply_humidity_ratio: 0.006,
            }),
        )
        .expect("valid CP375 route");
        let counts = [
            state.unit_off_skip_count,
            state.non_cooling_skip_count,
            state.positive_guard_false_fallthrough_skip_count,
            state.heating_availability_guard_false_fallthrough_count,
            state.humidification_control_guard_false_fallthrough_count,
            state.dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
            state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
            state.dehumidification_control_guard_false_fallthrough_count,
        ];
        assert_eq!(counts.iter().sum::<usize>(), 1);
        assert_eq!(counts[expected_route], 1);
        assert_eq!(state.source_site_execution_count, usize::from(active) * 4);
        assert_eq!(
            snapshot.purchased_air_supply_humidity_ratio_for_humidification_supply_maximum_read,
            active
        );
        assert_eq!(
            snapshot.supply_humidity_ratio_for_humidification_for_supply_maximum_read,
            active
        );
        assert_eq!(snapshot.source_shaped_two_argument_maximum_evaluated, active);
        assert_eq!(
            snapshot.purchased_air_supply_humidity_ratio_assignment_performed,
            active
        );
    }
}

#[test]
fn cp375_humidistat_and_none_routes_remain_distinct() {
    for selector in [
        DehumidificationControlType::Humidistat,
        DehumidificationControlType::None,
    ] {
        let predecessor = active_cp374(selector, 0.008, 0.007);
        let mut state = State::new(predecessor.system);
        let snapshot = advance(
            &mut state,
            predecessor,
            Some(ActiveOperands {
                purchased_air_supply_humidity_ratio: 0.006,
            }),
        )
        .expect("active CP375 route");
        assert_eq!(
            snapshot.dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_executed,
            selector == DehumidificationControlType::Humidistat
        );
        assert_eq!(
            snapshot.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_executed,
            selector == DehumidificationControlType::None
        );
    }
}

#[test]
fn cp375_operand_shape_mismatches_are_transactional_and_inactive_is_no_read() {
    let inactive = skipped_cp374();
    let mut state = State::new(inactive.system);
    let before = state.clone();
    assert!(
        advance(
            &mut state,
            inactive,
            Some(ActiveOperands {
                purchased_air_supply_humidity_ratio: f64::from_bits(0x7ff8_0000_0000_0375),
            }),
        )
        .is_none()
    );
    assert_eq!(state, before);

    let active = active_cp374(DehumidificationControlType::None, 0.008, 0.007);
    let mut state = State::new(active.system);
    let before = state.clone();
    assert!(advance(&mut state, active, None).is_none());
    assert_eq!(state, before);
}
