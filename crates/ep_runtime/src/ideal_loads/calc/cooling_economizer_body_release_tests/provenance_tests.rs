use super::*;

#[test]
fn public_body_rejects_alternate_history_condition_and_body_splice_transactionally() {
    let (mut target, system, first_target_condition) =
        body_release_fixture_with_cooling_demand(1.0);
    advance_direct_no_oa_calc_cooling_economizer_body(&mut target, &system, first_target_condition)
        .expect("target first CP317");
    let target_predecessor = advance_subsequent_body_predecessor(&mut target, &system, -1.0);

    let (mut donor, donor_system, first_donor_condition) =
        body_release_fixture_with_cooling_demand(-1.0);
    advance_direct_no_oa_calc_cooling_economizer_body(
        &mut donor,
        &donor_system,
        first_donor_condition,
    )
    .expect("donor first CP317");
    let donor_predecessor = advance_subsequent_body_predecessor(&mut donor, &donor_system, -1.0);

    let donor_unit = donor.units.get(&donor_system.id).expect("donor unit");
    let donor_condition = donor_unit.calc_cooling_economizer_condition.clone();
    let donor_body = donor_unit.calc_cooling_economizer_body.clone();
    let target_unit = target.units.get_mut(&system.id).expect("target unit");
    target_unit.calc_cooling_economizer_condition = donor_condition;
    target_unit.calc_cooling_economizer_body = donor_body;

    assert_eq!(
        target_predecessor, donor_predecessor,
        "the call-local predecessor matches while retained histories differ"
    );
    assert_rejected_without_mutation(target, &system, donor_predecessor);
}

#[test]
fn public_body_rejects_alternate_history_whole_unit_transplant_transactionally() {
    let (mut target, system, first_target_condition) =
        body_release_fixture_with_cooling_demand(1.0);
    advance_direct_no_oa_calc_cooling_economizer_body(&mut target, &system, first_target_condition)
        .expect("target first CP317");
    let _target_predecessor = advance_subsequent_body_predecessor(&mut target, &system, -1.0);

    let (mut donor, donor_system, first_donor_condition) =
        body_release_fixture_with_cooling_demand(-1.0);
    advance_direct_no_oa_calc_cooling_economizer_body(
        &mut donor,
        &donor_system,
        first_donor_condition,
    )
    .expect("donor first CP317");
    let donor_predecessor = advance_subsequent_body_predecessor(&mut donor, &donor_system, -1.0);
    let donor_unit = donor
        .units
        .get(&donor_system.id)
        .expect("donor unit")
        .clone();
    target.units.insert(system.id, donor_unit);

    assert_rejected_without_mutation(target, &system, donor_predecessor);
}
