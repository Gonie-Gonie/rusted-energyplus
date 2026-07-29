use super::completed_cp359_case;
use crate::ideal_loads::calc::cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment::{
    advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment,
    completed_direct_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_is_consistent,
    cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_snapshot_is_exact_direct_release,
    private_humidistat_counterfactual_from_direct_release,
    private_humidistat_counterfactual_links_to_direct_release,
};

#[test]
fn corruption_replay_and_witness_redistribution_reject_transactionally() {
    let (mut runtime, system, mut corrupted) =
        completed_cp359_case(-1_000.0, 1.0, false).expect("completed CP359");
    corrupted.source_order = &[];
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment(
            &mut runtime,
            &system,
            corrupted,
        )
        .is_err()
    );
    assert_eq!(runtime, before);

    let (mut runtime, system, predecessor) =
        completed_cp359_case(-1_000.0, 1.0, false).expect("completed CP359");
    let snapshot =
        advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment(
            &mut runtime,
            &system,
            predecessor,
        )
        .expect("CP360");
    let before_replay = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before_replay);

    let witness = runtime
        .cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_latest_witness(
            system.id,
        );
    assert!(runtime.units.get(&system.id).is_some_and(|unit| {
        completed_direct_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_is_consistent(
            &runtime, unit, &system, snapshot, witness,
        )
    }));
    {
        let state = &mut runtime
            .units
            .get_mut(&system.id)
            .expect("selected unit")
            .calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment;
        state.witnessed_dehumidification_control_none_case_completed_skip_count = 0;
        state
            .witnessed_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count =
            1;
    }
    assert!(runtime.units.get(&system.id).is_some_and(|unit| {
        !completed_direct_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_is_consistent(
            &runtime, unit, &system, snapshot, witness,
        )
    }));
}

#[test]
fn cp330_latest_witness_and_coordinated_flow_corruption_reject_private_h() {
    let (mut runtime, system, predecessor) =
        completed_cp359_case(-100_000.0, 1.0, true).expect("active CP359");
    let direct =
        advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment(
            &mut runtime,
            &system,
            predecessor,
        )
        .expect("direct CP360");
    assert!(runtime.units.get(&system.id).is_some_and(|unit| {
        private_humidistat_counterfactual_from_direct_release(
            &runtime, unit, &system, direct, -0.002, 0.008,
        )
        .is_some()
    }));

    let original = runtime.clone();
    runtime
        .units
        .get_mut(&system.id)
        .expect("selected unit")
        .calc_cooling_supply_mass_flow_positive_guard
        .latest
        .as_mut()
        .expect("CP330 owner")
        .supply_mass_flow_rate_kg_per_s = Some(7.0);
    assert!(runtime.units.get(&system.id).is_some_and(|unit| {
        private_humidistat_counterfactual_from_direct_release(
            &runtime, unit, &system, direct, -0.002, 0.008,
        )
        .is_none()
    }));

    runtime = original;
    let mut forged = runtime
        .units
        .get(&system.id)
        .and_then(|unit| unit.calc_cooling_supply_mass_flow_positive_guard.latest)
        .expect("CP330 owner");
    forged.supply_mass_flow_rate_kg_per_s = Some(7.0);
    runtime
        .units
        .get_mut(&system.id)
        .expect("selected unit")
        .calc_cooling_supply_mass_flow_positive_guard
        .latest = Some(forged);
    runtime.set_cooling_supply_mass_flow_positive_guard_latest_witness(system.id, forged);
    assert!(runtime.units.get(&system.id).is_some_and(|unit| {
        private_humidistat_counterfactual_from_direct_release(
            &runtime, unit, &system, direct, -0.002, 0.008,
        )
        .is_none()
    }));
}

#[test]
fn private_numeric_corruption_and_release_overflow_reject() {
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
    let private_h = private_humidistat_counterfactual_from_direct_release(
        &runtime, unit, &system, direct, -0.0, -0.0,
    )
    .expect("private CP360");
    let mut corrupted = private_h;
    corrupted.assigned_supply_humidity_ratio_for_dehumidification = Some(0.0);
    assert!(
        !cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_snapshot_is_exact_direct_release(
            corrupted
        )
    );
    assert!(!super::super::release::snapshots_match_bit_exact_for_test(
        private_h, corrupted,
    ));
    assert!(!private_humidistat_counterfactual_links_to_direct_release(
        &runtime, unit, &system, direct, corrupted, -0.0, -0.0,
    ));

    let (mut overflow_runtime, overflow_system, predecessor) =
        completed_cp359_case(-1_000.0, 1.0, false).expect("completed CP359");
    overflow_runtime
        .units
        .get_mut(&overflow_system.id)
        .expect("selected unit")
        .calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment
        .transition_count = usize::MAX;
    let before = overflow_runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment(
            &mut overflow_runtime,
            &overflow_system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(overflow_runtime, before);
}
