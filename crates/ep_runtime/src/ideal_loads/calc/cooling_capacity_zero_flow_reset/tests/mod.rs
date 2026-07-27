use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId, IdealLoadsLimit};

use super::{
    PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE,
    PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE_ORDER,
    PurchasedAirCalcCoolingCapacityZeroFlowResetInput,
    PurchasedAirCalcCoolingCapacityZeroFlowResetRuntimeState,
    advance_cooling_capacity_zero_flow_reset_state,
};
use crate::ideal_loads::PurchasedAirCalcCoolingHumidificationFlowSnapshot;

const SYSTEM: IdealLoadsAirSystemId = IdealLoadsAirSystemId(5);

fn predecessor(
    cooling_demand_w: f64,
) -> (
    crate::ideal_loads::PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    PurchasedAirCalcCoolingHumidificationFlowSnapshot,
) {
    let (mut runtime, system, sensible) =
        crate::ideal_loads::calc::cooling_dehumidification_flow_release_tests::release_case(
            cooling_demand_w,
        );
    let dehumidification =
        crate::ideal_loads::advance_direct_no_oa_calc_cooling_dehumidification_flow(
            &mut runtime,
            &system,
            sensible,
        )
        .expect("CP319");
    let humidification = crate::ideal_loads::advance_direct_no_oa_calc_cooling_humidification_flow(
        &mut runtime,
        &system,
        dehumidification,
    )
    .expect("CP320");
    (runtime, system, humidification)
}

fn input(
    limit: IdealLoadsLimit,
    capacity: f64,
) -> PurchasedAirCalcCoolingCapacityZeroFlowResetInput {
    PurchasedAirCalcCoolingCapacityZeroFlowResetInput {
        cooling_limit: limit,
        maximum_total_cooling_capacity_w: capacity,
        supply_mass_flow_rate_for_cool_kg_per_s: f64::NEG_INFINITY,
        supply_mass_flow_rate_for_dehumidification_kg_per_s: -0.0,
    }
}

fn run(
    predecessor: PurchasedAirCalcCoolingHumidificationFlowSnapshot,
    input: PurchasedAirCalcCoolingCapacityZeroFlowResetInput,
) -> super::PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot {
    let mut state = PurchasedAirCalcCoolingCapacityZeroFlowResetRuntimeState::new(SYSTEM);
    advance_cooling_capacity_zero_flow_reset_state(&mut state, predecessor, input)
}

#[test]
fn source_boundary_and_exact_ten_sites_are_stable() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2147-2152"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2155"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_CAPACITY_ZERO_FLOW_RESET_SOURCE_ORDER.len(),
        10
    );
}

#[test]
fn capacity_limit_short_circuits_second_read_and_assigns_three_positive_zeros() {
    let (_, _, predecessor) = predecessor(-1_000.0);
    let snapshot = run(predecessor, input(IdealLoadsLimit::LimitCapacity, -0.0));
    assert_eq!(snapshot.cooling_limit_capacity, Some(true));
    assert!(!snapshot.second_cooling_limit_read);
    assert!(snapshot.maximum_total_cooling_capacity_read);
    assert!(snapshot.zero_cooling_capacity_body_entered);
    for value in [
        snapshot.assigned_supply_mass_flow_rate_for_cool_kg_per_s,
        snapshot.assigned_supply_mass_flow_rate_for_dehumidification_kg_per_s,
        snapshot.assigned_supply_mass_flow_rate_for_humidification_kg_per_s,
        snapshot.resulting_supply_mass_flow_rate_for_cool_kg_per_s,
        snapshot.resulting_supply_mass_flow_rate_for_dehumidification_kg_per_s,
        snapshot.resulting_supply_mass_flow_rate_for_humidification_kg_per_s,
    ] {
        assert_eq!(value.expect("positive zero").to_bits(), 0.0_f64.to_bits());
    }
}

#[test]
fn combined_limit_repeats_read_before_capacity_comparison() {
    let (_, _, predecessor) = predecessor(-1_000.0);
    let snapshot = run(
        predecessor,
        input(IdealLoadsLimit::LimitFlowRateAndCapacity, 1.0),
    );
    assert_eq!(snapshot.cooling_limit_capacity, Some(false));
    assert!(snapshot.second_cooling_limit_read);
    assert_eq!(
        snapshot.second_cooling_limit,
        Some(IdealLoadsLimit::LimitFlowRateAndCapacity)
    );
    assert_eq!(snapshot.cooling_limit_flow_rate_and_capacity, Some(true));
    assert!(!snapshot.zero_cooling_capacity_body_entered);
}

#[test]
fn rejected_limit_short_circuits_poisoned_capacity_and_preserves_candidate_bits() {
    let (_, _, predecessor) = predecessor(-1_000.0);
    let value = input(IdealLoadsLimit::NoLimit, f64::NAN);
    let snapshot = run(predecessor, value);
    assert!(!snapshot.maximum_total_cooling_capacity_read);
    assert!(snapshot.maximum_total_cooling_capacity_w.is_none());
    assert_eq!(
        snapshot
            .resulting_supply_mass_flow_rate_for_cool_kg_per_s
            .expect("prior")
            .to_bits(),
        value.supply_mass_flow_rate_for_cool_kg_per_s.to_bits()
    );
    assert_eq!(
        snapshot
            .resulting_supply_mass_flow_rate_for_dehumidification_kg_per_s
            .expect("prior")
            .to_bits(),
        value
            .supply_mass_flow_rate_for_dehumidification_kg_per_s
            .to_bits()
    );
}

#[test]
fn nan_capacity_falls_through_and_preserves_all_candidate_bits() {
    let (_, _, predecessor) = predecessor(-1_000.0);
    let snapshot = run(predecessor, input(IdealLoadsLimit::LimitCapacity, f64::NAN));
    assert_eq!(
        snapshot.maximum_total_cooling_capacity_equal_to_zero,
        Some(false)
    );
    assert!(!snapshot.zero_cooling_capacity_body_entered);
    assert_eq!(
        snapshot
            .predecessor_supply_mass_flow_rate_for_humidification_kg_per_s
            .expect("prior")
            .to_bits(),
        snapshot
            .resulting_supply_mass_flow_rate_for_humidification_kg_per_s
            .expect("result")
            .to_bits()
    );
}

#[test]
fn every_nonzero_or_nonfinite_capacity_preserves_all_three_candidate_bits() {
    let (_, _, predecessor) = predecessor(-1_000.0);
    for capacity in [1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let snapshot = run(predecessor, input(IdealLoadsLimit::LimitCapacity, capacity));
        assert!(!snapshot.zero_cooling_capacity_body_entered);
        for (prior, result) in [
            (
                snapshot.predecessor_supply_mass_flow_rate_for_cool_kg_per_s,
                snapshot.resulting_supply_mass_flow_rate_for_cool_kg_per_s,
            ),
            (
                snapshot.predecessor_supply_mass_flow_rate_for_dehumidification_kg_per_s,
                snapshot.resulting_supply_mass_flow_rate_for_dehumidification_kg_per_s,
            ),
            (
                snapshot.predecessor_supply_mass_flow_rate_for_humidification_kg_per_s,
                snapshot.resulting_supply_mass_flow_rate_for_humidification_kg_per_s,
            ),
        ] {
            assert_eq!(
                prior.expect("prior candidate").to_bits(),
                result.expect("preserved candidate").to_bits()
            );
        }
    }
}

#[test]
fn unit_off_and_non_cooling_skip_every_site() {
    let (_, _, non_cooling) = predecessor(1.0);
    let mut unit_off = non_cooling;
    unit_off.unit_body_entered = false;
    unit_off.unit_off_skipped = true;
    unit_off.non_cooling_skipped = false;
    for snapshot in [
        run(non_cooling, input(IdealLoadsLimit::LimitCapacity, 0.0)),
        run(unit_off, input(IdealLoadsLimit::LimitCapacity, 0.0)),
    ] {
        assert!(!snapshot.first_cooling_limit_read);
        assert!(!snapshot.maximum_total_cooling_capacity_read);
        assert!(
            snapshot
                .resulting_supply_mass_flow_rate_for_cool_kg_per_s
                .is_none()
        );
    }
}

#[test]
fn public_no_limit_release_skips_capacity_read_and_is_transactional_on_replay() {
    let (mut runtime, system, predecessor) = predecessor(-1_000.0);
    let snapshot = crate::ideal_loads::advance_direct_no_oa_calc_cooling_capacity_zero_flow_reset(
        &mut runtime,
        &system,
        predecessor,
    )
    .expect("CP321");
    assert!(!snapshot.maximum_total_cooling_capacity_read);
    let before = runtime.clone();
    assert!(
        crate::ideal_loads::advance_direct_no_oa_calc_cooling_capacity_zero_flow_reset(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}

#[test]
fn public_release_rejects_post_init_sized_limit_forgery_without_mutation() {
    let (mut runtime, mut system, predecessor) = predecessor(-1_000.0);
    let before = runtime.clone();
    system.cooling_limit = IdealLoadsLimit::LimitCapacity;
    system.maximum_total_cooling_capacity_w = Some(ep_model::AutosizeOrNumber::Value(0.0));
    assert!(
        crate::ideal_loads::advance_direct_no_oa_calc_cooling_capacity_zero_flow_reset(
            &mut runtime,
            &system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}

#[test]
fn public_release_rejects_post_init_cooling_limit_selector_mutation_transactionally() {
    let (mut runtime, mut system, predecessor) = predecessor(-1_000.0);
    system.cooling_limit = IdealLoadsLimit::LimitFlowRate;
    system.maximum_cooling_air_flow_rate_m3_per_s = Some(ep_model::AutosizeOrNumber::Value(0.25));
    let before = runtime.clone();

    assert_eq!(
        crate::ideal_loads::advance_direct_no_oa_calc_cooling_capacity_zero_flow_reset(
            &mut runtime,
            &system,
            predecessor,
        ),
        Err(
            super::PurchasedAirCalcCoolingCapacityZeroFlowResetError::SizedLimitsMismatch {
                system: predecessor.system,
            }
        )
    );
    assert_eq!(runtime, before);
}

#[test]
fn public_release_rejects_coordinated_system_and_sized_overlay_mutation_transactionally() {
    let (mut runtime, mut system, predecessor) = predecessor(-1_000.0);
    system.cooling_limit = IdealLoadsLimit::LimitCapacity;
    system.maximum_total_cooling_capacity_w = Some(ep_model::AutosizeOrNumber::Value(0.0));
    runtime
        .units
        .get_mut(&predecessor.system)
        .expect("selected unit")
        .sized_limits
        .as_mut()
        .expect("retained sized limits")
        .maximum_total_cooling_capacity_w = system.maximum_total_cooling_capacity_w;
    let before = runtime.clone();

    assert_eq!(
        crate::ideal_loads::advance_direct_no_oa_calc_cooling_capacity_zero_flow_reset(
            &mut runtime,
            &system,
            predecessor,
        ),
        Err(
            super::PurchasedAirCalcCoolingCapacityZeroFlowResetError::SizedLimitsMismatch {
                system: predecessor.system,
            }
        )
    );
    assert_eq!(runtime, before);
}

#[test]
fn dehumidification_enum_is_not_consulted_by_the_cp321_transition() {
    let (_, system, predecessor) = predecessor(-1_000.0);
    assert_eq!(
        system.dehumidification_control_type,
        DehumidificationControlType::None
    );
    let snapshot = run(predecessor, input(IdealLoadsLimit::LimitCapacity, 0.0));
    assert!(snapshot.zero_cooling_capacity_body_entered);
}
