use super::*;

#[test]
fn public_body_rejects_entry_prefix_and_initialization_corruption_transactionally() {
    let (runtime, system, predecessor) = body_release_fixture();

    let mut entry_corruption = runtime.clone();
    entry_corruption
        .units
        .get_mut(&system.id)
        .expect("selected unit")
        .calc_entry
        .reset_count = usize::MAX;
    assert_rejected_without_mutation(entry_corruption, &system, predecessor);

    let mut initialization_corruption = runtime;
    initialization_corruption
        .units
        .get_mut(&system.id)
        .expect("selected unit")
        .environment_initialization_count = 0;
    assert_rejected_without_mutation(initialization_corruption, &system, predecessor);
}
