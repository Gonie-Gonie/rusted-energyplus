//! CP380 public release, lineage-corruption, and commit-atomicity tests.

use ep_model::IdealLoadsLimit;

use super::super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardError as Error,
    advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_guard,
    completed_direct_cooling_post_saturation_capacity_limit_guard_is_consistent,
    cooling_post_saturation_capacity_limit_guard_latest_metadata_is_consistent,
    cooling_post_saturation_capacity_limit_guard_snapshot_is_exact_direct_release,
};
use super::completed_cp379_case;
use crate::ideal_loads::calc::cooling_supply_enthalpy_post_saturation_assignment::cooling_supply_enthalpy_post_saturation_assignment_snapshots_match_bit_exact;

#[test]
fn cp380_public_direct_uses_only_configured_selector_and_retained_lineages() {
    let (mut runtime, system, cp379) = completed_cp379_case();
    let retained_before = runtime
        .units
        .get(&system.id)
        .and_then(|unit| {
            unit.calc_cooling_supply_enthalpy_post_saturation_assignment
                .latest
        })
        .expect("retained CP379");
    let snapshot = advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_guard(
        &mut runtime,
        &system,
        cp379,
    )
    .expect("CP380 direct release");

    assert!(
        cooling_post_saturation_capacity_limit_guard_snapshot_is_exact_direct_release(snapshot)
    );
    assert!(snapshot.capacity_limit_guard_evaluated);
    assert!(snapshot.configured_cooling_limit_owned_read);
    assert!(snapshot.cp337_same_call_selector_lineage_corroborated);
    assert_eq!(snapshot.first_cooling_limit, Some(system.cooling_limit));
    assert_eq!(
        snapshot.capacity_limit_body_entered,
        matches!(
            system.cooling_limit,
            IdealLoadsLimit::LimitCapacity | IdealLoadsLimit::LimitFlowRateAndCapacity
        ),
    );
    let unit = runtime.units.get(&system.id).expect("known unit");
    assert!(
        unit.calc_cooling_supply_enthalpy_post_saturation_assignment
            .latest
            .is_some_and(|retained| {
                cooling_supply_enthalpy_post_saturation_assignment_snapshots_match_bit_exact(
                    retained,
                    retained_before,
                )
            })
    );
    let witness = runtime.cooling_post_saturation_capacity_limit_guard_latest_witness(system.id);
    assert!(
        completed_direct_cooling_post_saturation_capacity_limit_guard_is_consistent(
            &runtime, unit, &system, snapshot, witness,
        )
    );
    assert!(cooling_post_saturation_capacity_limit_guard_latest_metadata_is_consistent(unit, 1));
}

#[test]
fn cp380_rejects_cp379_bit_drift_transactionally() {
    let (mut runtime, system, cp379) = completed_cp379_case();
    let mut forged = runtime
        .cooling_supply_enthalpy_post_saturation_assignment_latest_witness(system.id)
        .expect("CP379 witness");
    forged.resulting_supply_enthalpy_j_per_kg = forged
        .resulting_supply_enthalpy_j_per_kg
        .map(|value| f64::from_bits(value.to_bits() ^ 1));
    runtime
        .set_cooling_supply_enthalpy_post_saturation_assignment_latest_witness(system.id, forged);
    let before = runtime.clone();
    assert!(matches!(
        advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_guard(
            &mut runtime,
            &system,
            cp379,
        ),
        Err(Error::CoolingSupplyEnthalpyPostSaturationAssignmentSnapshotMismatch { .. })
    ));
    assert_eq!(runtime, before);
}

#[test]
fn cp380_rejects_cp337_same_call_and_selector_lineage_drift_transactionally() {
    for ordinal_drift in [false, true] {
        let (mut runtime, system, cp379) = completed_cp379_case();
        let mut forged = runtime
            .cooling_positive_supply_capacity_limit_guard_latest_witness(system.id)
            .expect("CP337 witness");
        if ordinal_drift {
            forged.parent_call_ordinal += 1;
        } else {
            forged.first_cooling_limit = Some(alternate_limit(system.cooling_limit));
        }
        runtime.set_cooling_positive_supply_capacity_limit_guard_latest_witness(system.id, forged);
        let before = runtime.clone();
        assert!(matches!(
            advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_guard(
                &mut runtime,
                &system,
                cp379,
            ),
            Err(Error::CoolingLimitSelectorLineageMismatch { .. })
        ));
        assert_eq!(runtime, before);
    }
}

#[test]
fn cp380_rejects_post_prefix_system_selector_mutation_transactionally() {
    let (mut runtime, mut system, cp379) = completed_cp379_case();
    system.cooling_limit = alternate_limit(system.cooling_limit);
    let before = runtime.clone();
    assert!(matches!(
        advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_guard(
            &mut runtime,
            &system,
            cp379,
        ),
        Err(Error::CoolingLimitSelectorLineageMismatch { .. })
            | Err(Error::SystemOutsideDirectSubset { .. })
    ));
    assert_eq!(runtime, before);
}

#[test]
fn cp380_replay_and_release_witness_corruption_preserve_runtime() {
    let (mut runtime, system, cp379) = completed_cp379_case();
    let snapshot = advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_guard(
        &mut runtime,
        &system,
        cp379,
    )
    .expect("first CP380 release");
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_guard(
            &mut runtime,
            &system,
            cp379,
        )
        .is_err()
    );
    assert_eq!(runtime, before);

    let mut forged = snapshot;
    forged.capacity_limit_body_entered = !forged.capacity_limit_body_entered;
    runtime.set_cooling_post_saturation_capacity_limit_guard_latest_witness(system.id, forged);
    let before = runtime.clone();
    assert!(matches!(
        advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_guard(
            &mut runtime,
            &system,
            cp379,
        ),
        Err(Error::RuntimeStateInvariantViolation { .. }) | Err(Error::PredecessorCallOrder { .. })
    ));
    assert_eq!(runtime, before);
}

#[test]
fn cp380_counter_overflow_preserves_runtime_state_and_witness() {
    let (mut runtime, system, cp379) = completed_cp379_case();
    runtime
        .units
        .get_mut(&system.id)
        .expect("known unit")
        .calc_cooling_post_saturation_capacity_limit_guard
        .source_site_execution_count = usize::MAX;
    let before = runtime.clone();
    assert!(matches!(
        advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_guard(
            &mut runtime,
            &system,
            cp379,
        ),
        Err(Error::RuntimeStateInvariantViolation { .. })
    ));
    assert_eq!(runtime, before);
}

fn alternate_limit(limit: IdealLoadsLimit) -> IdealLoadsLimit {
    if limit == IdealLoadsLimit::LimitCapacity {
        IdealLoadsLimit::NoLimit
    } else {
        IdealLoadsLimit::LimitCapacity
    }
}
