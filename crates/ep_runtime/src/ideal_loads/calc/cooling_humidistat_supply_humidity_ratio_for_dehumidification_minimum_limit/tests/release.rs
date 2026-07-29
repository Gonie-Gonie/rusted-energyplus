use super::completed_cp360_case;
use crate::ideal_loads::calc::cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit::{
    advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit,
    completed_direct_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_is_consistent,
    cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_snapshot_is_exact_direct_release,
    private_humidistat_counterfactual_from_direct_release,
    private_humidistat_counterfactual_links_to_direct_release,
    purchased_air_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_lifecycle_summary,
};

#[test]
fn public_direct_routes_are_complete_null_and_do_not_validate_rhs() {
    for (demand, availability, capacity, expected) in [
        (-1_000.0, 0.0, true, (true, false, false, false)),
        (1.0, 1.0, true, (false, true, false, false)),
        (-1.0e-40, 1.0, true, (false, false, true, false)),
        (-1_000.0, 1.0, false, (false, false, false, true)),
    ] {
        let (mut runtime, mut system, predecessor) =
            completed_cp360_case(demand, availability, capacity).expect("completed CP360");
        system.minimum_cooling_supply_air_humidity_ratio = f64::from_bits(0x7ff8_0000_0000_00a5);
        let snapshot =
            advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit(
                &mut runtime,
                &system,
                predecessor,
            )
            .expect("CP361 direct release");
        assert!(
            cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_snapshot_is_exact_direct_release(
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
            snapshot.supply_humidity_ratio_for_dehumidification_for_minimum_limit_maximum_read,
            snapshot.minimum_cooling_supply_air_humidity_ratio_for_maximum_read,
            snapshot.source_shaped_two_argument_maximum_evaluated,
            snapshot.supply_humidity_ratio_for_dehumidification_assignment_performed,
        ] {
            assert!(!flag);
        }
        for value in [
            snapshot.predecessor_resulting_supply_humidity_ratio_for_dehumidification,
            snapshot.supply_humidity_ratio_for_dehumidification_before_minimum_limit,
            snapshot.minimum_cooling_supply_air_humidity_ratio,
            snapshot.maximum_supply_humidity_ratio_for_dehumidification,
            snapshot.assigned_supply_humidity_ratio_for_dehumidification,
            snapshot.resulting_supply_humidity_ratio_for_dehumidification,
        ] {
            assert!(value.is_none());
        }
        let summary =
            purchased_air_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_lifecycle_summary(
                &runtime,
                system.id,
            )
            .expect("CP361 lifecycle");
        assert_eq!(summary.state.latest, Some(snapshot));
        assert_eq!(summary.state.source_site_execution_count, 0);
    }
}

#[test]
fn private_h_reuses_cp360_bridge_and_selected_typed_minimum() {
    let (mut runtime, system, predecessor) =
        completed_cp360_case(-100_000.0, 1.0, true).expect("completed CP360");
    let direct =
        advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit(
            &mut runtime,
            &system,
            predecessor,
        )
        .expect("direct CP361");
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
    .expect("private-H CP361");
    assert!(
        private_h
            .dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_executed
    );
    assert_eq!(
        private_h
            .minimum_cooling_supply_air_humidity_ratio
            .expect("typed minimum")
            .to_bits(),
        system.minimum_cooling_supply_air_humidity_ratio.to_bits()
    );
    assert_eq!(
        private_h
            .resulting_supply_humidity_ratio_for_dehumidification
            .expect("left-biased NaN")
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
        demand,
        f64::from_bits(0x7ff8_0000_0000_0043),
    ));
    assert!(runtime.units.get(&system.id).is_some_and(|unit| {
        completed_direct_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_is_consistent(
            &runtime,
            unit,
            &system,
            direct,
            runtime.cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_latest_witness(system.id),
        )
    }));
}
