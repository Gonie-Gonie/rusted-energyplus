use super::completed_cp358_case;
use crate::ideal_loads::calc::cooling_humidistat_moisture_demand_assignment::{
    advance_direct_no_oa_calc_cooling_humidistat_moisture_demand_assignment,
    completed_direct_cooling_humidistat_moisture_demand_assignment_is_consistent,
    cooling_humidistat_moisture_demand_assignment_snapshot_is_exact_direct_release,
    private_humidistat_counterfactual_from_direct_release,
    private_humidistat_counterfactual_links_to_direct_release,
    purchased_air_calc_cooling_humidistat_moisture_demand_assignment_lifecycle_summary,
};

#[test]
fn public_direct_routes_are_complete_null_and_private_h_is_parametric() {
    for (demand, availability, capacity, expected) in [
        (-1_000.0, 0.0, true, (true, false, false, false)),
        (1.0, 1.0, true, (false, true, false, false)),
        (-1.0e-40, 1.0, true, (false, false, true, false)),
        (-1_000.0, 1.0, false, (false, false, false, true)),
    ] {
        let (mut runtime, system, predecessor) =
            completed_cp358_case(demand, availability, capacity).expect("completed CP358");
        let snapshot = advance_direct_no_oa_calc_cooling_humidistat_moisture_demand_assignment(
            &mut runtime,
            &system,
            predecessor,
        )
        .expect("CP359 direct release");
        assert!(
            cooling_humidistat_moisture_demand_assignment_snapshot_is_exact_direct_release(
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
        assert!(!snapshot.dehumidification_control_humidistat_moisture_demand_assignment_executed);
        assert!(
            snapshot
                .zone_dehumidifying_setpoint_moisture_demand_kg_per_s
                .is_none()
        );
        assert!(
            snapshot
                .assigned_zone_dehumidifying_setpoint_moisture_demand_kg_per_s
                .is_none()
        );
        assert!(
            snapshot
                .resulting_zone_dehumidifying_setpoint_moisture_demand_kg_per_s
                .is_none()
        );
        let summary =
            purchased_air_calc_cooling_humidistat_moisture_demand_assignment_lifecycle_summary(
                &runtime, system.id,
            )
            .expect("CP359 lifecycle");
        assert_eq!(summary.state.latest, Some(snapshot));
        assert_eq!(summary.state.source_site_execution_count, 0);
    }

    let (mut runtime, system, predecessor) =
        completed_cp358_case(-100_000.0, 1.0, true).expect("active CP358");
    let direct = advance_direct_no_oa_calc_cooling_humidistat_moisture_demand_assignment(
        &mut runtime,
        &system,
        predecessor,
    )
    .expect("direct CP359");
    let unit = runtime.units.get(&system.id).expect("selected unit");
    let sampled = -0.0;
    let private_h = private_humidistat_counterfactual_from_direct_release(
        &runtime, unit, &system, direct, sampled,
    )
    .expect("parametric private-H CP359");
    assert!(private_h.dehumidification_control_humidistat_moisture_demand_assignment_executed);
    assert_eq!(
        private_h
            .resulting_zone_dehumidifying_setpoint_moisture_demand_kg_per_s
            .expect("result")
            .to_bits(),
        sampled.to_bits()
    );
    assert!(private_humidistat_counterfactual_links_to_direct_release(
        &runtime, unit, &system, direct, private_h, sampled
    ));
    assert!(!private_humidistat_counterfactual_links_to_direct_release(
        &runtime, unit, &system, direct, private_h, 0.0
    ));
}

#[test]
fn corruption_replay_and_witness_redistribution_reject_without_mutation() {
    let (mut runtime, system, mut corrupted) =
        completed_cp358_case(-1_000.0, 1.0, false).expect("completed CP358");
    corrupted.source_order = &[];
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_humidistat_moisture_demand_assignment(
            &mut runtime,
            &system,
            corrupted
        )
        .is_err()
    );
    assert_eq!(runtime, before);

    let (mut runtime, system, predecessor) =
        completed_cp358_case(-1_000.0, 1.0, false).expect("completed CP358");
    let snapshot = advance_direct_no_oa_calc_cooling_humidistat_moisture_demand_assignment(
        &mut runtime,
        &system,
        predecessor,
    )
    .expect("CP359");
    let before_replay = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_humidistat_moisture_demand_assignment(
            &mut runtime,
            &system,
            predecessor
        )
        .is_err()
    );
    assert_eq!(runtime, before_replay);

    let witness = runtime.cooling_humidistat_moisture_demand_assignment_latest_witness(system.id);
    assert!(runtime.units.get(&system.id).is_some_and(|unit| {
        completed_direct_cooling_humidistat_moisture_demand_assignment_is_consistent(
            &runtime, unit, &system, snapshot, witness,
        )
    }));
    {
        let state = &mut runtime
            .units
            .get_mut(&system.id)
            .expect("selected unit")
            .calc_cooling_humidistat_moisture_demand_assignment;
        state.witnessed_dehumidification_control_none_case_completed_skip_count = 0;
        state
            .witnessed_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count =
            1;
    }
    let after_forge = runtime.clone();
    assert!(runtime.units.get(&system.id).is_some_and(|unit| {
        !completed_direct_cooling_humidistat_moisture_demand_assignment_is_consistent(
            &runtime, unit, &system, snapshot, witness,
        )
    }));
    assert_eq!(runtime, after_forge);
}
