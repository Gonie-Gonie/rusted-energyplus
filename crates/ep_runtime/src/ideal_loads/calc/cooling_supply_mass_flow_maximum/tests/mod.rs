use ep_model::IdealLoadsAirSystemId;

use super::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyMassFlowMaximumInput,
    PurchasedAirCalcCoolingSupplyMassFlowMaximumOperand as Operand,
    PurchasedAirCalcCoolingSupplyMassFlowMaximumRuntimeState,
    PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot,
    advance_cooling_supply_mass_flow_maximum_state,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot, PurchasedAirRuntimeState,
};

mod ieee;
mod release_corruption;

pub(super) fn release_case(
    cooling_demand_w: f64,
) -> (
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot,
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
    let reset = crate::ideal_loads::advance_direct_no_oa_calc_cooling_capacity_zero_flow_reset(
        &mut runtime,
        &system,
        humidification,
    )
    .expect("CP321");
    (runtime, system, reset)
}

pub(super) fn run(
    predecessor: PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot,
    outdoor_air: f64,
) -> PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot {
    let mut state =
        PurchasedAirCalcCoolingSupplyMassFlowMaximumRuntimeState::new(predecessor.system);
    advance_cooling_supply_mass_flow_maximum_state(
        &mut state,
        predecessor,
        PurchasedAirCalcCoolingSupplyMassFlowMaximumInput {
            outdoor_air_mass_flow_rate_kg_per_s: outdoor_air,
        },
    )
}

#[test]
fn source_boundary_and_exact_six_sites_are_stable() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2155"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2157"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_MAXIMUM_SOURCE_ORDER.len(),
        6
    );
}

#[test]
fn source_tree_selects_greatest_operand_and_records_every_site() {
    let (_, _, mut predecessor) = release_case(-1_000.0);
    predecessor.resulting_supply_mass_flow_rate_for_cool_kg_per_s = Some(1.0);
    predecessor.resulting_supply_mass_flow_rate_for_dehumidification_kg_per_s = Some(3.0);
    predecessor.resulting_supply_mass_flow_rate_for_humidification_kg_per_s = Some(2.0);
    let snapshot = run(predecessor, 4.0);

    assert_eq!(
        snapshot.positive_zero_outdoor_air_winner,
        Some(Operand::OutdoorAir)
    );
    assert_eq!(
        snapshot.cooling_dehumidification_winner,
        Some(Operand::Dehumidification)
    );
    assert_eq!(
        snapshot.leading_candidate_pair_winner,
        Some(Operand::OutdoorAir)
    );
    assert_eq!(snapshot.final_winner, Some(Operand::OutdoorAir));
    assert_eq!(snapshot.resulting_supply_mass_flow_rate_kg_per_s, Some(4.0));
    assert!(snapshot.supply_mass_flow_rate_assigned);
}

#[test]
fn unit_off_and_non_cooling_skip_all_six_sites() {
    let (_, _, non_cooling) = release_case(1.0);
    let mut unit_off = non_cooling;
    unit_off.unit_body_entered = false;
    unit_off.unit_off_skipped = true;
    unit_off.non_cooling_skipped = false;
    for snapshot in [run(non_cooling, f64::NAN), run(unit_off, f64::NAN)] {
        assert!(!snapshot.outdoor_air_mass_flow_rate_read);
        assert!(!snapshot.positive_zero_vs_outdoor_air_comparison_evaluated);
        assert!(!snapshot.supply_mass_flow_rate_assigned);
        assert!(snapshot.resulting_supply_mass_flow_rate_kg_per_s.is_none());
    }
}

#[test]
fn state_counters_partition_cooling_and_skip_routes() {
    let (_, _, cooling) = release_case(-1_000.0);
    let (_, _, non_cooling) = release_case(1.0);
    let system = IdealLoadsAirSystemId(cooling.system.0);
    let mut state = PurchasedAirCalcCoolingSupplyMassFlowMaximumRuntimeState::new(system);
    advance_cooling_supply_mass_flow_maximum_state(
        &mut state,
        cooling,
        PurchasedAirCalcCoolingSupplyMassFlowMaximumInput {
            outdoor_air_mass_flow_rate_kg_per_s: 0.0,
        },
    );
    advance_cooling_supply_mass_flow_maximum_state(
        &mut state,
        non_cooling,
        PurchasedAirCalcCoolingSupplyMassFlowMaximumInput {
            outdoor_air_mass_flow_rate_kg_per_s: f64::NAN,
        },
    );
    assert_eq!(state.transition_count, 2);
    assert_eq!(state.cooling_body_entry_count, 1);
    assert_eq!(state.non_cooling_skip_count, 1);
    assert_eq!(state.maximum_evaluation_count, 1);
    assert_eq!(state.supply_mass_flow_rate_assignment_count, 1);
}
