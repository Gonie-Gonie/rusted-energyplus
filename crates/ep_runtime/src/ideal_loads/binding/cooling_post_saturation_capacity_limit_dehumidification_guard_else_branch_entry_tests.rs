use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_ENTRY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_ENTRY_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_ENTRY_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchEntryRuntimeState as Cp418State,
    advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry as advance_cp418,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_snapshot_has_exact_cp417_prefix_and_marker,
    cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_snapshot_is_exact_direct_release,
};
use ep_model::{
    AutosizeOrNumber, DehumidificationControlType, HumidificationControlType, IdealLoadsLimit,
};

#[test]
fn cp418_binding_contract_is_source_ordered_after_cp417() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_ENTRY_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2327",
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_ENTRY_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2330",
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_ENTRY_SOURCE_ORDER,
        ["enter-post-saturation-capacity-limit-dehumidification-guard-else-branch-after-guard-false-fallthrough"],
    );
}

#[test]
fn public_release_enters_only_the_outer_dehumidification_guard_else_branch() {
    let (mut runtime, output) = else_fixture();
    let predecessor = output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment;
    assert!(predecessor.predecessor_dehumidification_guard_false_fallthrough);
    assert!(
        !predecessor
            .post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_executed
    );
    reset_cp418(&mut runtime, predecessor.system);

    let snapshot = advance_cp418(&mut runtime, &matching_system(), predecessor)
        .expect("finite CP418 else-entry release");
    assert!(
        cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_snapshot_has_exact_cp417_prefix_and_marker(
            snapshot,
            predecessor,
        )
    );
    assert!(snapshot.post_saturation_capacity_limit_dehumidification_guard_else_branch_entered);
    assert_eq!(
        snapshot.resulting_supply_humidity_ratio.map(f64::to_bits),
        predecessor
            .resulting_supply_humidity_ratio
            .map(f64::to_bits),
    );
    assert_eq!(
        snapshot
            .resulting_supply_enthalpy_j_per_kg
            .map(f64::to_bits),
        predecessor
            .resulting_supply_enthalpy_j_per_kg
            .map(f64::to_bits),
    );
    assert_eq!(
        snapshot.resulting_supply_temperature_c.map(f64::to_bits),
        predecessor.resulting_supply_temperature_c.map(f64::to_bits),
    );
}

#[test]
fn cold_recursive_exact_validator_accepts_scheduled_cp418_release_snapshot() {
    let (_, output) = else_fixture();
    assert!(
        cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_snapshot_is_exact_direct_release(
            output.calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry,
        )
    );
}

#[test]
fn public_release_rejects_forged_cp417_hidden_witness_transactionally() {
    let (mut runtime, output) = else_fixture();
    let predecessor = output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment;
    reset_cp418(&mut runtime, predecessor.system);
    let clean_runtime = runtime.clone();
    let mut forged = predecessor;
    forged.source_order = &["forged-hidden-cp417-witness"];
    runtime.set_cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_latest_witness(
        predecessor.system,
        forged,
    );
    let before = runtime
        .units
        .get(&predecessor.system)
        .expect("CP418 unit")
        .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry
        .clone();

    assert!(advance_cp418(&mut runtime, &matching_system(), predecessor).is_err());
    assert_eq!(
        runtime
            .units
            .get(&predecessor.system)
            .expect("CP418 unit")
            .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry,
        before,
    );

    let mut runtime = clean_runtime;
    runtime
        .units
        .get_mut(&predecessor.system)
        .expect("CP417 unit")
        .calc_cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment
        .source_site_execution_count += 1;
    let before = runtime
        .units
        .get(&predecessor.system)
        .expect("CP418 unit")
        .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry
        .clone();
    assert!(advance_cp418(&mut runtime, &matching_system(), predecessor).is_err());
    assert_eq!(
        runtime
            .units
            .get(&predecessor.system)
            .expect("CP418 unit")
            .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry,
        before,
    );
}

fn else_fixture() -> (
    crate::ideal_loads::PurchasedAirRuntimeState,
    crate::ideal_loads::DirectZonePurchasedAirScheduledCouplingOutput,
) {
    super::cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_tests::run_case(
        IdealLoadsLimit::LimitCapacity,
        0.008,
        1.0,
        5_000.0,
    )
}

fn matching_system() -> ep_model::IdealLoadsAirSystem {
    let mut system = super::ideal_loads_system();
    system.cooling_limit = IdealLoadsLimit::LimitCapacity;
    system.maximum_cooling_air_flow_rate_m3_per_s = None;
    system.maximum_total_cooling_capacity_w = Some(AutosizeOrNumber::Value(5_000.0));
    system.dehumidification_control_type = DehumidificationControlType::None;
    system.humidification_control_type = HumidificationControlType::None;
    system.minimum_cooling_supply_air_humidity_ratio = f64::NAN;
    system
}

fn reset_cp418(
    runtime: &mut crate::ideal_loads::PurchasedAirRuntimeState,
    system: ep_model::IdealLoadsAirSystemId,
) {
    runtime
        .units
        .get_mut(&system)
        .expect("CP418 unit")
        .calc_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry =
        Cp418State::new(system);
    runtime.clear_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_latest_witness_for_test(system);
}
