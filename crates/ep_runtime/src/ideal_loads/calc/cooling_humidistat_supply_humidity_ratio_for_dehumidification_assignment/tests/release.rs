use super::completed_cp359_case;
use crate::ideal_loads::calc::cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment::{
    advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment,
    completed_direct_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_is_consistent,
    cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_snapshot_is_exact_direct_release,
    private_humidistat_counterfactual_from_direct_release,
    private_humidistat_counterfactual_links_to_direct_release,
    purchased_air_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_lifecycle_summary,
};

#[test]
fn public_direct_routes_are_complete_null() {
    for (demand, availability, capacity, expected) in [
        (-1_000.0, 0.0, true, (true, false, false, false)),
        (1.0, 1.0, true, (false, true, false, false)),
        (-1.0e-40, 1.0, true, (false, false, true, false)),
        (-1_000.0, 1.0, false, (false, false, false, true)),
    ] {
        let (mut runtime, system, predecessor) =
            completed_cp359_case(demand, availability, capacity).expect("completed CP359");
        let snapshot =
            advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment(
                &mut runtime,
                &system,
                predecessor,
            )
            .expect("CP360 direct release");
        assert!(
            cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_snapshot_is_exact_direct_release(
                snapshot
            )
        );
        assert_eq!(
            (
                snapshot.unit_off_skipped,
                snapshot.non_cooling_skipped,
                snapshot.positive_guard_false_fallthrough_skipped,
                snapshot.dehumidification_control_none_case_completed_skip,
            ),
            expected
        );
        for flag in [
            snapshot.zone_dehumidifying_setpoint_moisture_demand_read,
            snapshot.supply_mass_flow_rate_read,
            snapshot.moisture_demand_derived_supply_humidity_ratio_calculated,
            snapshot.zone_node_humidity_ratio_read,
            snapshot.supply_humidity_ratio_for_dehumidification_calculated,
            snapshot.supply_humidity_ratio_for_dehumidification_assigned,
        ] {
            assert!(!flag);
        }
        for value in [
            snapshot.predecessor_resulting_zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
            snapshot.zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
            snapshot.supply_mass_flow_rate_kg_per_s,
            snapshot.moisture_demand_derived_supply_humidity_ratio,
            snapshot.zone_node_humidity_ratio,
            snapshot.calculated_supply_humidity_ratio_for_dehumidification,
            snapshot.assigned_supply_humidity_ratio_for_dehumidification,
            snapshot.resulting_supply_humidity_ratio_for_dehumidification,
        ] {
            assert!(value.is_none());
        }
        let summary =
            purchased_air_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_lifecycle_summary(
                &runtime,
                system.id,
            )
            .expect("CP360 lifecycle");
        assert_eq!(summary.state.latest, Some(snapshot));
        assert_eq!(summary.state.source_site_execution_count, 0);
    }
}

#[test]
fn private_h_is_parametric_except_for_cp330_owned_flow() {
    let (mut runtime, system, predecessor) =
        completed_cp359_case(-100_000.0, 1.0, true).expect("active CP359");
    let direct =
        advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment(
            &mut runtime,
            &system,
            predecessor,
        )
        .expect("direct CP360");
    let unit = runtime.units.get(&system.id).expect("selected unit");
    let demand = -0.0;
    let zone_humidity = f64::from_bits(0x7ff8_0000_0000_0042);
    let private_h = private_humidistat_counterfactual_from_direct_release(
        &runtime,
        unit,
        &system,
        direct,
        demand,
        zone_humidity,
    )
    .expect("parametric private-H CP360");
    assert!(
        private_h
            .dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_assignment_executed
    );
    assert_eq!(
        private_h
            .zone_dehumidifying_setpoint_moisture_demand_kg_per_s
            .expect("demand")
            .to_bits(),
        demand.to_bits()
    );
    assert_eq!(
        private_h
            .zone_node_humidity_ratio
            .expect("Zone-node humidity")
            .to_bits(),
        zone_humidity.to_bits()
    );
    assert!(private_humidistat_counterfactual_links_to_direct_release(
        &runtime,
        unit,
        &system,
        direct,
        private_h,
        demand,
        zone_humidity,
    ));
    assert!(!private_humidistat_counterfactual_links_to_direct_release(
        &runtime,
        unit,
        &system,
        direct,
        private_h,
        0.0,
        zone_humidity,
    ));
    assert!(!private_humidistat_counterfactual_links_to_direct_release(
        &runtime,
        unit,
        &system,
        direct,
        private_h,
        demand,
        f64::from_bits(0x7ff8_0000_0000_0043),
    ));
    assert!(runtime.units.get(&system.id).is_some_and(|unit| {
        completed_direct_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_is_consistent(
            &runtime,
            unit,
            &system,
            direct,
            runtime.cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_latest_witness(system.id),
        )
    }));
}
