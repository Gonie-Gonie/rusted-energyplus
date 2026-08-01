//! CP373 active/skip route and operand-shape tests.

use super::*;

#[test]
fn cp373_all_eight_cp372_routes_form_an_exact_partition() {
    let predecessors = [
        cp372_from_cp369(
            inherited_skip_cp369(0),
            DehumidificationControlType::None,
            HumidificationControlType::None,
            None,
        ),
        cp372_from_cp369(
            inherited_skip_cp369(1),
            DehumidificationControlType::None,
            HumidificationControlType::None,
            None,
        ),
        cp372_from_cp369(
            inherited_skip_cp369(2),
            DehumidificationControlType::None,
            HumidificationControlType::None,
            None,
        ),
        cp372_from_cp369(
            heating_guard_false_cp369(),
            DehumidificationControlType::None,
            HumidificationControlType::Humidistat,
            None,
        ),
        skipped_cp372(),
        active_cp372(DehumidificationControlType::Humidistat, 0.001),
        active_cp372(DehumidificationControlType::None, 0.001),
        cp372(
            DehumidificationControlType::ConstantSensibleHeatRatio,
            HumidificationControlType::Humidistat,
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
                supply_mass_flow_rate_kg_per_s: 0.5,
                zone_node_humidity_ratio: 0.004,
            }),
        )
        .expect("valid CP373 route");
        let counts = [
            state.unit_off_skip_count,
            state.non_cooling_skip_count,
            state.positive_guard_false_fallthrough_skip_count,
            state.heating_availability_guard_false_fallthrough_count,
            state.humidification_control_guard_false_fallthrough_count,
            state.dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_assignment_count,
            state.dehumidification_control_none_supply_humidity_ratio_for_humidification_assignment_count,
            state.dehumidification_control_guard_false_fallthrough_count,
        ];
        assert_eq!(counts.iter().sum::<usize>(), 1);
        assert_eq!(counts[expected_route], 1);
        assert_eq!(state.source_site_execution_count, usize::from(active) * 6);
        assert_eq!(snapshot.supply_mass_flow_rate_read, active);
    }
}

#[test]
fn cp373_humidistat_and_none_routes_remain_distinct() {
    for selector in [
        DehumidificationControlType::Humidistat,
        DehumidificationControlType::None,
    ] {
        let predecessor = active_cp372(selector, 0.001);
        let mut state = State::new(predecessor.system);
        let snapshot = advance(
            &mut state,
            predecessor,
            Some(ActiveOperands {
                supply_mass_flow_rate_kg_per_s: 0.5,
                zone_node_humidity_ratio: 0.004,
            }),
        )
        .expect("active route");
        assert_eq!(
            snapshot
                .dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_assignment_executed,
            selector == DehumidificationControlType::Humidistat
        );
        assert_eq!(
            snapshot
                .dehumidification_control_none_supply_humidity_ratio_for_humidification_assignment_executed,
            selector == DehumidificationControlType::None
        );
    }
}

#[test]
fn cp373_inactive_route_executes_no_sites_and_accepts_no_operands() {
    let predecessor = skipped_cp372();
    let mut state = State::new(predecessor.system);
    let snapshot = advance(&mut state, predecessor, None).expect("CP373 skip");

    assert_eq!(state.humidification_control_guard_false_fallthrough_count, 1);
    assert_eq!(state.source_site_execution_count, 0);
    assert!(!snapshot.zone_humidifying_setpoint_moisture_demand_read);
    assert!(!snapshot.supply_mass_flow_rate_read);
    assert!(!snapshot.zone_node_humidity_ratio_read);
    assert_eq!(
        snapshot.resulting_supply_humidity_ratio_for_humidification,
        None
    );

    let mut mismatch_state = State::new(predecessor.system);
    let before = mismatch_state.clone();
    assert!(
        advance(
            &mut mismatch_state,
            predecessor,
            Some(ActiveOperands {
                supply_mass_flow_rate_kg_per_s: 1.0,
                zone_node_humidity_ratio: 0.0,
            }),
        )
        .is_none()
    );
    assert_eq!(mismatch_state, before);
}

#[test]
fn cp373_active_operand_shape_mismatch_is_transactional() {
    let predecessor = active_cp372(DehumidificationControlType::None, 0.001);
    let mut state = State::new(predecessor.system);
    let before = state.clone();
    assert!(advance(&mut state, predecessor, None).is_none());
    assert_eq!(state, before);
}
