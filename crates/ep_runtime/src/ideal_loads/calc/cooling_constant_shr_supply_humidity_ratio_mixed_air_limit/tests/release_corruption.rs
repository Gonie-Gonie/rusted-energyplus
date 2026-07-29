use super::super::release::active_operands_from_retained_owners_for_test;
use super::{completed_cp355_case, private_active_cp355_predecessor};

#[test]
fn private_active_operand_uses_only_same_call_retained_witnessed_cp329_owner() {
    let completed = completed_cp355_case(-100_000.0, 1.0, false);
    assert!(completed.is_some());
    let Some((runtime, system, direct)) = completed else {
        return;
    };
    let private = private_active_cp355_predecessor(&runtime, &system, direct)
        .expect("canonical private CP355");
    let unit = runtime.units.get(&system.id).expect("selected unit");
    let mixed_air = unit
        .calc_cooling_mixed_air_call
        .latest
        .expect("same-call CP329");
    let operands = active_operands_from_retained_owners_for_test(&runtime, unit, &system, private)
        .expect("CP356 retained owners");
    assert_eq!(
        operands.mixed_air_humidity_ratio.to_bits(),
        mixed_air
            .mixed_air_humidity_ratio
            .expect("CP329 humidity")
            .to_bits()
    );
}

#[test]
fn private_cp355_result_forge_is_rejected() {
    let completed = completed_cp355_case(-100_000.0, 1.0, false);
    assert!(completed.is_some());
    let Some((runtime, system, direct)) = completed else {
        return;
    };
    let mut private = private_active_cp355_predecessor(&runtime, &system, direct)
        .expect("canonical private CP355");
    for value in [
        &mut private.maximum_supply_humidity_ratio,
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
fn cp329_latest_only_owner_forge_is_rejected_without_mutation() {
    let completed = completed_cp355_case(-100_000.0, 1.0, false);
    assert!(completed.is_some());
    let Some((mut runtime, system, direct)) = completed else {
        return;
    };
    let private = private_active_cp355_predecessor(&runtime, &system, direct)
        .expect("canonical private CP355");
    {
        let unit = runtime.units.get_mut(&system.id).expect("selected unit");
        let mixed_air = unit
            .calc_cooling_mixed_air_call
            .latest
            .as_mut()
            .expect("CP329 latest");
        mixed_air.mixed_air_humidity_ratio = mixed_air.mixed_air_humidity_ratio.map(next_bits);
    }
    let before = runtime.clone();
    let unit = runtime.units.get(&system.id).expect("selected unit");
    assert!(
        active_operands_from_retained_owners_for_test(&runtime, unit, &system, private).is_none()
    );
    assert_eq!(runtime, before);
}

#[test]
fn coordinated_cp329_latest_and_witness_owner_forge_is_rejected() {
    let completed = completed_cp355_case(-100_000.0, 1.0, false);
    assert!(completed.is_some());
    let Some((mut runtime, system, direct)) = completed else {
        return;
    };
    let private = private_active_cp355_predecessor(&runtime, &system, direct)
        .expect("canonical private CP355");
    let forged = {
        let unit = runtime.units.get_mut(&system.id).expect("selected unit");
        let latest = unit
            .calc_cooling_mixed_air_call
            .latest
            .as_mut()
            .expect("CP329 latest");
        latest.mixed_air_humidity_ratio = latest.mixed_air_humidity_ratio.map(next_bits);
        *latest
    };
    runtime.set_cooling_mixed_air_call_latest_witness(system.id, forged);
    let unit = runtime.units.get(&system.id).expect("selected unit");
    assert!(
        active_operands_from_retained_owners_for_test(&runtime, unit, &system, private).is_none()
    );
}

#[test]
fn coordinated_cp355_direct_and_witness_forge_breaks_recursive_bridge() {
    let completed = completed_cp355_case(-100_000.0, 1.0, false);
    assert!(completed.is_some());
    let Some((mut runtime, system, direct)) = completed else {
        return;
    };
    let private = private_active_cp355_predecessor(&runtime, &system, direct)
        .expect("canonical private CP355");
    let forged = {
        let unit = runtime.units.get_mut(&system.id).expect("selected unit");
        let latest = unit
            .calc_cooling_constant_shr_supply_humidity_ratio_minimum_limit
            .latest
            .as_mut()
            .expect("CP355 direct latest");
        latest.source_order = &[];
        *latest
    };
    runtime.set_cooling_constant_shr_supply_humidity_ratio_minimum_limit_latest_witness(
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
