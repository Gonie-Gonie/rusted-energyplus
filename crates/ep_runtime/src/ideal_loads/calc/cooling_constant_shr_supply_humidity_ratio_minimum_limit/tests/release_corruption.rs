use super::super::release::active_operands_from_retained_owners_for_test;
use super::{completed_cp354_case, private_active_cp354_predecessor};

#[test]
fn private_active_operand_uses_selected_typed_system_owner() {
    let completed = completed_cp354_case(-100_000.0, 1.0, false);
    assert!(completed.is_some());
    let Some((runtime, system, direct)) = completed else {
        return;
    };
    let private = private_active_cp354_predecessor(&runtime, &system, direct)
        .expect("canonical private CP354");
    let unit = runtime.units.get(&system.id).expect("selected unit");
    let operands = active_operands_from_retained_owners_for_test(&runtime, unit, &system, private)
        .expect("CP355 retained owners");
    assert_eq!(
        operands.minimum_cooling_supply_air_humidity_ratio.to_bits(),
        system.minimum_cooling_supply_air_humidity_ratio.to_bits()
    );
}

#[test]
fn private_active_nonfinite_typed_owner_is_rejected_without_mutation() {
    for invalid in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let completed = completed_cp354_case(-100_000.0, 1.0, false);
        assert!(completed.is_some());
        let Some((runtime, mut system, direct)) = completed else {
            return;
        };
        let private = private_active_cp354_predecessor(&runtime, &system, direct)
            .expect("canonical private CP354");
        system.minimum_cooling_supply_air_humidity_ratio = invalid;
        let before = runtime.clone();
        let unit = runtime.units.get(&system.id).expect("selected unit");
        assert!(
            active_operands_from_retained_owners_for_test(&runtime, unit, &system, private)
                .is_none()
        );
        assert_eq!(runtime, before);
    }
}

#[test]
fn private_active_owner_gate_is_finite_only_not_a_duplicate_range_gate() {
    for finite in [-0.25, 2.0] {
        let completed = completed_cp354_case(-100_000.0, 1.0, false);
        assert!(completed.is_some());
        let Some((runtime, mut system, direct)) = completed else {
            return;
        };
        let private = private_active_cp354_predecessor(&runtime, &system, direct)
            .expect("canonical private CP354");
        system.minimum_cooling_supply_air_humidity_ratio = finite;
        let unit = runtime.units.get(&system.id).expect("selected unit");
        let operands =
            active_operands_from_retained_owners_for_test(&runtime, unit, &system, private)
                .expect("finite typed owner");
        assert_eq!(
            operands.minimum_cooling_supply_air_humidity_ratio.to_bits(),
            finite.to_bits()
        );
    }
}

#[test]
fn private_cp354_result_forge_is_rejected() {
    let completed = completed_cp354_case(-100_000.0, 1.0, false);
    assert!(completed.is_some());
    let Some((runtime, system, direct)) = completed else {
        return;
    };
    let mut private = private_active_cp354_predecessor(&runtime, &system, direct)
        .expect("canonical private CP354");
    for value in [
        &mut private.minimum_supply_humidity_ratio,
        &mut private.assigned_supply_humidity_ratio,
        &mut private.resulting_supply_humidity_ratio,
    ] {
        *value = value.map(next_bits);
    }
    let unit = runtime.units.get(&system.id).expect("selected unit");
    assert!(
        active_operands_from_retained_owners_for_test(&runtime, unit, &system, private).is_none()
    );
}

#[test]
fn coordinated_retained_direct_and_witness_forge_is_rejected() {
    let completed = completed_cp354_case(-100_000.0, 1.0, false);
    assert!(completed.is_some());
    let Some((mut runtime, system, direct)) = completed else {
        return;
    };
    let private = private_active_cp354_predecessor(&runtime, &system, direct)
        .expect("canonical private CP354");
    let forged = {
        let unit = runtime.units.get_mut(&system.id).expect("selected unit");
        let latest = unit
            .calc_cooling_constant_shr_supply_humidity_ratio_overdrying_limit
            .latest
            .as_mut()
            .expect("CP354 direct latest");
        latest.source_order = &[];
        *latest
    };
    runtime.set_cooling_constant_shr_supply_humidity_ratio_overdrying_limit_latest_witness(
        system.id, forged,
    );
    let unit = runtime.units.get(&system.id).expect("selected unit");
    assert!(
        active_operands_from_retained_owners_for_test(&runtime, unit, &system, private).is_none()
    );
}

fn next_bits(value: f64) -> f64 {
    f64::from_bits(value.to_bits().wrapping_add(1))
}
