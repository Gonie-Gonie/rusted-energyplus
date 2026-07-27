use super::release_case;
use crate::ideal_loads::{
    advance_direct_no_oa_calc_cooling_supply_mass_flow_maximum,
    cooling_supply_mass_flow_maximum_snapshot_is_exact_direct_release,
};
use ep_model::{AutosizeOrNumber, IdealLoadsAirSystemId, IdealLoadsLimit};

#[test]
fn public_release_consumes_cp321_and_cp311_and_is_transactional_on_replay() {
    let (mut runtime, system, predecessor) = release_case(-1_000.0);
    let snapshot = advance_direct_no_oa_calc_cooling_supply_mass_flow_maximum(
        &mut runtime,
        &system,
        predecessor,
    )
    .expect("CP322");
    assert!(cooling_supply_mass_flow_maximum_snapshot_is_exact_direct_release(snapshot));
    assert_eq!(
        snapshot
            .outdoor_air_mass_flow_rate_kg_per_s
            .expect("no-OA input")
            .to_bits(),
        0.0_f64.to_bits()
    );
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_maximum(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}

#[test]
fn supplied_cp321_candidate_corruption_is_rejected_without_mutation() {
    let (mut runtime, system, mut predecessor) = release_case(-1_000.0);
    predecessor.resulting_supply_mass_flow_rate_for_cool_kg_per_s = Some(-0.0);
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_maximum(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}

#[test]
fn retained_cp311_outdoor_air_corruption_is_rejected_without_mutation() {
    let (mut runtime, system, predecessor) = release_case(-1_000.0);
    runtime
        .units
        .get_mut(&system.id)
        .expect("unit")
        .calc_minimum_oa_prefix
        .latest
        .as_mut()
        .expect("CP311")
        .working_outdoor_air_mass_flow_rate_kg_per_s = Some(-0.0);
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_maximum(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}

#[test]
fn forged_pending_cp322_counter_is_rejected_without_mutation() {
    let (mut runtime, system, predecessor) = release_case(-1_000.0);
    runtime
        .units
        .get_mut(&system.id)
        .expect("unit")
        .calc_cooling_supply_mass_flow_maximum
        .maximum_evaluation_count = 1;
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_maximum(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}

#[test]
fn post_init_selector_mutation_is_rejected_without_mutation() {
    let (mut runtime, mut system, predecessor) = release_case(-1_000.0);
    system.cooling_limit = IdealLoadsLimit::LimitFlowRate;
    system.maximum_cooling_air_flow_rate_m3_per_s = Some(AutosizeOrNumber::Value(0.25));
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_maximum(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}

#[test]
fn retained_sized_overlay_corruption_is_rejected_without_mutation() {
    let (mut runtime, system, predecessor) = release_case(-1_000.0);
    runtime
        .units
        .get_mut(&system.id)
        .expect("unit")
        .sized_limits
        .as_mut()
        .expect("sized limits")
        .maximum_total_cooling_capacity_w = Some(AutosizeOrNumber::Value(1.0));
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_maximum(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}

#[test]
fn typed_system_identity_mismatch_is_rejected_without_mutation() {
    let (mut runtime, mut system, predecessor) = release_case(-1_000.0);
    system.id = IdealLoadsAirSystemId(system.id.0 + 1);
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_maximum(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}

#[test]
fn retained_cp321_latest_corruption_is_rejected_without_mutation() {
    let (mut runtime, system, predecessor) = release_case(-1_000.0);
    runtime
        .units
        .get_mut(&system.id)
        .expect("unit")
        .calc_cooling_capacity_zero_flow_reset
        .latest
        .as_mut()
        .expect("CP321")
        .resulting_supply_mass_flow_rate_for_cool_kg_per_s = Some(-0.0);
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_mass_flow_maximum(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}
