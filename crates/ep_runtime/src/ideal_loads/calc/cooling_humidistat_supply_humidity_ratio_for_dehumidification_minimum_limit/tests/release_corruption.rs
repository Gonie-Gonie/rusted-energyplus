use super::completed_cp360_case;
use crate::ideal_loads::calc::cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit::{
    active_operands_from_retained_owners_for_test,
    advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit,
    completed_direct_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_is_consistent,
    private_humidistat_counterfactual_from_direct_release,
    private_humidistat_counterfactual_links_to_direct_release,
};

#[test]
fn corruption_replay_and_witness_redistribution_reject_transactionally() {
    let (mut runtime, system, mut corrupted) =
        completed_cp360_case(-1_000.0, 1.0, false).expect("completed CP360");
    corrupted.source_order = &[];
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit(
            &mut runtime,
            &system,
            corrupted,
        )
        .is_err()
    );
    assert_eq!(runtime, before);

    let (mut runtime, system, predecessor) =
        completed_cp360_case(-1_000.0, 1.0, false).expect("completed CP360");
    let snapshot =
        advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit(
            &mut runtime,
            &system,
            predecessor,
        )
        .expect("CP361");
    let before_replay = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before_replay);

    let witness = runtime
        .cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_latest_witness(
            system.id,
        );
    assert!(runtime.units.get(&system.id).is_some_and(|unit| {
        completed_direct_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_is_consistent(
            &runtime, unit, &system, snapshot, witness,
        )
    }));
    {
        let state = &mut runtime
            .units
            .get_mut(&system.id)
            .expect("selected unit")
            .calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit;
        state.witnessed_dehumidification_control_none_case_completed_skip_count = 0;
        state
            .witnessed_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count =
            1;
    }
    assert!(runtime.units.get(&system.id).is_some_and(|unit| {
        !completed_direct_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_is_consistent(
            &runtime, unit, &system, snapshot, witness,
        )
    }));
}

#[test]
fn private_typed_owner_gate_is_finite_only() {
    for invalid in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let (mut runtime, mut system, predecessor) =
            completed_cp360_case(-100_000.0, 1.0, true).expect("completed CP360");
        let direct =
            advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit(
                &mut runtime,
                &system,
                predecessor,
            )
            .expect("direct CP361");
        system.minimum_cooling_supply_air_humidity_ratio = invalid;
        let unit = runtime.units.get(&system.id).expect("selected unit");
        assert!(
            private_humidistat_counterfactual_from_direct_release(
                &runtime, unit, &system, direct, -0.002, 0.008,
            )
            .is_none()
        );
    }

    for finite in [-0.25, 2.0] {
        let (mut runtime, mut system, predecessor) =
            completed_cp360_case(-100_000.0, 1.0, true).expect("completed CP360");
        let direct =
            advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit(
                &mut runtime,
                &system,
                predecessor,
            )
            .expect("direct CP361");
        system.minimum_cooling_supply_air_humidity_ratio = finite;
        let unit = runtime.units.get(&system.id).expect("selected unit");
        let private = private_humidistat_counterfactual_from_direct_release(
            &runtime, unit, &system, direct, -0.002, 0.008,
        )
        .expect("finite-only owner");
        assert_eq!(
            private
                .minimum_cooling_supply_air_humidity_ratio
                .expect("minimum")
                .to_bits(),
            finite.to_bits()
        );
        let private_cp360 = unit
            .calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment
            .latest
            .expect("direct CP360");
        assert!(
            active_operands_from_retained_owners_for_test(
                &runtime,
                unit,
                &system,
                crate::ideal_loads::calc::cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment::private_humidistat_counterfactual_from_direct_release(
                    &runtime,
                    unit,
                    &system,
                    private_cp360,
                    -0.002,
                    0.008,
                )
                .expect("private CP360"),
            )
            .is_some()
        );
    }
}

#[test]
fn private_numeric_corruption_and_release_overflow_reject() {
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
    let private_h = private_humidistat_counterfactual_from_direct_release(
        &runtime, unit, &system, direct, -0.0, -0.0,
    )
    .expect("private CP361");
    let mut corrupted = private_h;
    corrupted.assigned_supply_humidity_ratio_for_dehumidification = Some(0.0);
    assert!(!super::super::release::snapshots_match_bit_exact_for_test(
        private_h, corrupted,
    ));
    assert!(!private_humidistat_counterfactual_links_to_direct_release(
        &runtime, unit, &system, direct, corrupted, -0.0, -0.0,
    ));

    let (mut overflow_runtime, overflow_system, predecessor) =
        completed_cp360_case(-1_000.0, 1.0, false).expect("completed CP360");
    overflow_runtime
        .units
        .get_mut(&overflow_system.id)
        .expect("selected unit")
        .calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit
        .transition_count = usize::MAX;
    let before = overflow_runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit(
            &mut overflow_runtime,
            &overflow_system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(overflow_runtime, before);
}
