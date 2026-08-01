//! CP379 public release, transitive-lineage, and commit-atomicity tests.

use super::super::{
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentError as Error,
    advance_direct_no_oa_calc_cooling_supply_enthalpy_post_saturation_assignment,
    completed_direct_cooling_supply_enthalpy_post_saturation_assignment_is_consistent,
    cooling_supply_enthalpy_post_saturation_assignment_latest_metadata_is_consistent,
    cooling_supply_enthalpy_post_saturation_assignment_snapshot_is_exact_direct_release,
};
use super::completed_cp378_case;
use crate::psychrometrics::energyplus_psy_h_fn_tdb_w;

#[test]
fn cp379_public_direct_uses_only_retained_cp377_and_cp378_operands() {
    let (mut runtime, system, cp378) = completed_cp378_case();
    let cp377 = runtime
        .units
        .get(&system.id)
        .and_then(|unit| {
            unit.calc_cooling_supply_humidity_ratio_saturation_assignment
                .latest
        })
        .expect("retained CP377");
    let temperature = cp377
        .supply_temperature_for_saturation_humidity_ratio_c
        .expect("CP377 temperature");
    let humidity_ratio = cp378
        .resulting_supply_humidity_ratio
        .expect("CP378 humidity ratio");
    let expected = energyplus_psy_h_fn_tdb_w(temperature, humidity_ratio);
    let snapshot = advance_direct_no_oa_calc_cooling_supply_enthalpy_post_saturation_assignment(
        &mut runtime,
        &system,
        cp378,
    )
    .expect("CP379 direct release");

    assert!(
        cooling_supply_enthalpy_post_saturation_assignment_snapshot_is_exact_direct_release(
            snapshot,
        )
    );
    assert_eq!(
        snapshot
            .resulting_supply_enthalpy_j_per_kg
            .map(f64::to_bits),
        Some(expected.to_bits()),
    );
    let unit = runtime.units.get(&system.id).expect("known unit");
    let witness =
        runtime.cooling_supply_enthalpy_post_saturation_assignment_latest_witness(system.id);
    assert!(
        completed_direct_cooling_supply_enthalpy_post_saturation_assignment_is_consistent(
            &runtime, unit, &system, snapshot, witness,
        )
    );
    assert!(
        cooling_supply_enthalpy_post_saturation_assignment_latest_metadata_is_consistent(unit, 1)
    );
}

#[test]
fn cp379_rejects_cp378_humidity_bit_drift_transactionally() {
    let (mut runtime, system, cp378) = completed_cp378_case();
    let mut forged = runtime
        .cooling_supply_humidity_ratio_saturation_limit_assignment_latest_witness(system.id)
        .expect("CP378 witness");
    forged.resulting_supply_humidity_ratio = forged
        .resulting_supply_humidity_ratio
        .map(|value| f64::from_bits(value.to_bits() ^ 1));
    runtime.set_cooling_supply_humidity_ratio_saturation_limit_assignment_latest_witness(
        system.id, forged,
    );
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_enthalpy_post_saturation_assignment(
            &mut runtime,
            &system,
            cp378,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}

#[test]
fn cp379_rejects_cp377_temperature_and_owner_drift_transactionally() {
    for owner_drift in [false, true] {
        let (mut runtime, system, cp378) = completed_cp378_case();
        let mut forged = runtime
            .cooling_supply_humidity_ratio_saturation_assignment_latest_witness(system.id)
            .expect("CP377 witness");
        if owner_drift {
            forged.cp334_supply_temperature_mixed_air_limit_owned_read = true;
            forged.cp344_capacity_limit_supply_temperature_mixed_air_limit_owned_read = true;
        } else {
            forged.supply_temperature_for_saturation_humidity_ratio_c = forged
                .supply_temperature_for_saturation_humidity_ratio_c
                .map(|value| f64::from_bits(value.to_bits() ^ 1));
        }
        runtime.set_cooling_supply_humidity_ratio_saturation_assignment_latest_witness(
            system.id, forged,
        );
        let before = runtime.clone();
        assert!(
            advance_direct_no_oa_calc_cooling_supply_enthalpy_post_saturation_assignment(
                &mut runtime,
                &system,
                cp378,
            )
            .is_err()
        );
        assert_eq!(runtime, before);
    }
}

#[test]
fn cp379_replay_and_every_release_corruption_preserve_state_and_witness() {
    let (mut runtime, system, cp378) = completed_cp378_case();
    let snapshot = advance_direct_no_oa_calc_cooling_supply_enthalpy_post_saturation_assignment(
        &mut runtime,
        &system,
        cp378,
    )
    .expect("first CP379 release");
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_enthalpy_post_saturation_assignment(
            &mut runtime,
            &system,
            cp378,
        )
        .is_err()
    );
    assert_eq!(runtime, before);

    let mut forged = snapshot;
    forged.resulting_supply_enthalpy_j_per_kg = forged
        .resulting_supply_enthalpy_j_per_kg
        .map(|value| f64::from_bits(value.to_bits() ^ 1));
    runtime
        .set_cooling_supply_enthalpy_post_saturation_assignment_latest_witness(system.id, forged);
    let before = runtime.clone();
    assert!(matches!(
        advance_direct_no_oa_calc_cooling_supply_enthalpy_post_saturation_assignment(
            &mut runtime,
            &system,
            cp378,
        ),
        Err(Error::RuntimeStateInvariantViolation { .. }) | Err(Error::PredecessorCallOrder { .. })
    ));
    assert_eq!(runtime, before);
}

#[test]
fn cp379_counter_overflow_preserves_runtime_state_and_witness() {
    let (mut runtime, system, cp378) = completed_cp378_case();
    runtime
        .units
        .get_mut(&system.id)
        .expect("known unit")
        .calc_cooling_supply_enthalpy_post_saturation_assignment
        .local_supply_enthalpy_after_saturation_limit_assignment_count = usize::MAX;
    let before = runtime.clone();
    assert!(matches!(
        advance_direct_no_oa_calc_cooling_supply_enthalpy_post_saturation_assignment(
            &mut runtime,
            &system,
            cp378,
        ),
        Err(Error::RuntimeStateInvariantViolation { .. })
    ));
    assert_eq!(runtime, before);
}
