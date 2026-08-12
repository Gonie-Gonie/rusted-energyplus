//! CP419 raw IEEE semantics, direct-domain admission, and inactive-owner tests.

use super::*;

#[test]
fn active_raw_ieee_inputs_use_canonical_cp_air_and_direct_predicates_reject_them() {
    let predecessor = predecessor_fixture(4, false, false);
    assert!(predecessor.post_saturation_capacity_limit_dehumidification_guard_else_branch_entered);
    let nan = f64::from_bits(0x7ff8_0000_0000_1234);
    for humidity_ratio in [-0.001, f64::NEG_INFINITY, nan, f64::INFINITY, f64::MAX] {
        let mut state = State::new(predecessor.system);
        let snapshot = advance(
            &mut state,
            predecessor,
            Some(ActiveInput {
                mixed_air_humidity_ratio: humidity_ratio,
            }),
        )
        .expect("raw CP419 accepts every f64 operand");
        assert_eq!(
            snapshot
                .mixed_air_humidity_ratio_for_cp_air
                .expect("retained owner")
                .to_bits(),
            humidity_ratio.to_bits(),
        );
        let expected = crate::psychrometrics::energyplus_psy_cp_air_fn_w(humidity_ratio);
        assert_eq!(
            snapshot
                .cp_air_j_per_kg_k
                .expect("assigned CpAir")
                .to_bits(),
            expected.to_bits(),
        );
        assert!(
            cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_snapshot_is_exact(
                snapshot,
            )
        );
        assert!(
            !cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_snapshot_is_exact_direct_release(
                snapshot,
            )
        );
        assert!(
            !cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_snapshot_has_exact_cp418_prefix_and_local_assignment(
                snapshot,
                predecessor,
                Some(humidity_ratio),
            )
        );
        assert_eq!(state.transition_count, 1);
    }

    let floor = crate::psychrometrics::energyplus_psy_cp_air_fn_w(1.0e-5);
    assert_eq!(
        crate::psychrometrics::energyplus_psy_cp_air_fn_w(-0.001).to_bits(),
        floor.to_bits(),
    );
    assert_eq!(
        crate::psychrometrics::energyplus_psy_cp_air_fn_w(f64::NEG_INFINITY).to_bits(),
        floor.to_bits(),
    );
    assert_eq!(
        crate::psychrometrics::energyplus_psy_cp_air_fn_w(nan).to_bits(),
        (1.004_84e3 + nan * 1.858_95e3).to_bits(),
    );
    assert_eq!(
        crate::psychrometrics::energyplus_psy_cp_air_fn_w(f64::INFINITY),
        f64::INFINITY,
    );
    assert_eq!(
        crate::psychrometrics::energyplus_psy_cp_air_fn_w(f64::MAX),
        f64::INFINITY,
    );
}

#[test]
fn active_public_domain_ieee_inputs_remain_direct_exact() {
    let predecessor = predecessor_fixture(4, false, false);
    for humidity_ratio in [0.0, -0.0, f64::from_bits(1), 0.008] {
        let snapshot = advance(
            &mut State::new(predecessor.system),
            predecessor,
            Some(ActiveInput {
                mixed_air_humidity_ratio: humidity_ratio,
            }),
        )
        .expect("public-domain CP419 humidity ratio");
        assert!(
            cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_cp_air_assignment_snapshot_is_exact_direct_release(
                snapshot,
            )
        );
    }
}

#[test]
fn inactive_route_requires_no_owner_and_rejects_supplied_owner_transactionally() {
    let predecessor = predecessor_fixture(20, true, true);
    assert!(!predecessor.post_saturation_capacity_limit_dehumidification_guard_else_branch_entered);
    let snapshot = advance(&mut State::new(predecessor.system), predecessor, None)
        .expect("inactive CP419 route");
    assert!(!snapshot.cp329_retained_mixed_air_humidity_ratio_owned_read);
    assert!(!snapshot.mixed_air_humidity_ratio_for_cp_air_read);
    assert!(snapshot.mixed_air_humidity_ratio_for_cp_air.is_none());
    assert!(!snapshot.psychrometric_cp_air_evaluated);
    assert!(snapshot.psychrometric_cp_air_result_j_per_kg_k.is_none());
    assert!(!snapshot.cp_air_assigned);
    assert!(snapshot.cp_air_j_per_kg_k.is_none());

    let mut state = State::new(predecessor.system);
    let before = state.clone();
    assert!(
        advance(
            &mut state,
            predecessor,
            Some(ActiveInput {
                mixed_air_humidity_ratio: 0.008,
            }),
        )
        .is_none()
    );
    assert_eq!(state, before);
}
