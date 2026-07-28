use ep_model::DehumidificationControlType;

use super::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentActiveInput,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentRuntimeState,
    advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_state,
    release::active_input_from_owner_for_test,
};
use crate::ideal_loads::calc::cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry::{
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntryRuntimeState,
    advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry_state,
};
use crate::ideal_loads::calc::cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case::{
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseRuntimeState,
    advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_state,
};
use crate::ideal_loads::calc::cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment::completed_cp344_case;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntrySnapshot,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchSnapshot,
    PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry,
    advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case,
    advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch,
    advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment,
};
use crate::psychrometrics::energyplus_psy_cp_air_fn_w;

mod public_release;
mod release_corruption;

fn completed_cp346_case(
    cooling_demand_w: f64,
    overall_availability: f64,
    capacity_limit: bool,
) -> Option<(
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchSnapshot,
)> {
    let (mut runtime, system, predecessor) =
        completed_cp344_case(cooling_demand_w, overall_availability, capacity_limit);
    let cp345 =
        advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment(
            &mut runtime,
            &system,
            predecessor,
        )
        .ok()?;
    let cp346 =
        advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch(
            &mut runtime,
            &system,
            cp345,
        )
        .ok()?;
    Some((runtime, system, cp346))
}

fn completed_cp348_case(
    cooling_demand_w: f64,
    overall_availability: f64,
    capacity_limit: bool,
) -> Option<(
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntrySnapshot,
)>{
    let (mut runtime, system, cp346) =
        completed_cp346_case(cooling_demand_w, overall_availability, capacity_limit)?;
    let cp347 =
        advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case(
            &mut runtime,
            &system,
            cp346,
        )
        .ok()?;
    let cp348 =
        advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry(
            &mut runtime,
            &system,
            cp347,
        )
        .ok()?;
    Some((runtime, system, cp348))
}

fn private_cp348_case(
    selector: DehumidificationControlType,
) -> Option<(
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntrySnapshot,
)>{
    let (runtime, system, mut cp346) = completed_cp346_case(-1_000.0, 1.0, false)?;
    cp346.dehumidification_control_type = Some(selector);
    let mut cp347_state =
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseRuntimeState::new(
            system.id,
        );
    let cp347 =
        advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_state(
            &mut cp347_state,
            cp346,
            None,
        )?;
    let mut cp348_state =
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntryRuntimeState::new(
            system.id,
        );
    let cp348 =
        advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry_state(
            &mut cp348_state,
            cp347,
        )?;
    Some((runtime, system, cp348))
}

fn owner_input(
    runtime: &PurchasedAirRuntimeState,
    system: ep_model::IdealLoadsAirSystemId,
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntrySnapshot,
) -> Option<
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentActiveInput,
>{
    let owner = runtime
        .units
        .get(&system)?
        .calc_cooling_mixed_air_call
        .latest?;
    let witness = runtime.cooling_mixed_air_call_latest_witness(system)?;
    active_input_from_owner_for_test(predecessor, owner, witness)
}

#[test]
fn source_boundary_and_exact_three_sites_are_stable() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2216"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2217"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_SOURCE_ORDER,
        [
            "read-purchased-air-mixed-air-humidity-ratio-for-constant-sensible-heat-ratio-cp-air",
            "evaluate-psy-cp-air-fn-w-for-constant-sensible-heat-ratio-cp-air",
            "assign-local-cp-air-for-constant-sensible-heat-ratio-case",
        ]
    );
}

#[test]
fn pure_transition_partitions_all_seven_routes_and_only_k_executes_three_sites() {
    let completed = completed_cp348_case(-1_000.0, 1.0, false);
    assert!(completed.is_some());
    let Some((_, _, none)) = completed else {
        return;
    };
    let mut state =
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentRuntimeState::new(
            none.system,
        );
    for (demand, availability) in [(-1_000.0, 0.0), (1.0, 1.0), (-1.0e-40, 1.0)] {
        let completed = completed_cp348_case(demand, availability, true);
        assert!(completed.is_some());
        let Some((_, _, predecessor)) = completed else {
            return;
        };
        assert!(
            advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_state(
                &mut state,
                predecessor,
                None,
            )
            .is_some()
        );
    }
    assert!(
        advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_state(
            &mut state,
            none,
            None,
        )
        .is_some()
    );
    for selector in [
        DehumidificationControlType::ConstantSensibleHeatRatio,
        DehumidificationControlType::Humidistat,
        DehumidificationControlType::ConstantSupplyHumidityRatio,
    ] {
        let private = private_cp348_case(selector);
        assert!(private.is_some());
        let Some((runtime, system, predecessor)) = private else {
            return;
        };
        let input = if selector == DehumidificationControlType::ConstantSensibleHeatRatio {
            owner_input(&runtime, system.id, predecessor)
        } else {
            None
        };
        let snapshot =
            advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_state(
                &mut state,
                predecessor,
                input,
            );
        assert!(snapshot.is_some());
        let Some(snapshot) = snapshot else {
            return;
        };
        assert_eq!(
            snapshot
                .dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed,
            selector == DehumidificationControlType::ConstantSensibleHeatRatio
        );
        if let Some(input) = input {
            assert_eq!(
                snapshot.mixed_air_humidity_ratio.map(f64::to_bits),
                Some(input.mixed_air_humidity_ratio.to_bits())
            );
            assert_eq!(
                snapshot.cp_air_j_per_kg_k.map(f64::to_bits),
                Some(energyplus_psy_cp_air_fn_w(input.mixed_air_humidity_ratio).to_bits())
            );
        }
    }
    assert_eq!(state.transition_count, 7);
    assert_eq!(state.unit_off_skip_count, 1);
    assert_eq!(state.non_cooling_skip_count, 1);
    assert_eq!(state.positive_guard_false_fallthrough_skip_count, 1);
    assert_eq!(
        state.dehumidification_control_none_case_completed_skip_count,
        1
    );
    assert_eq!(
        state.dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_count,
        1
    );
    assert_eq!(
        state.dehumidification_control_humidistat_case_selected_skip_count,
        1
    );
    assert_eq!(
        state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
        1
    );
    assert_eq!(state.source_site_execution_count, 3);
    assert_eq!(state.mixed_air_humidity_ratio_read_count, 1);
    assert_eq!(state.psychrometric_cp_air_evaluation_count, 1);
    assert_eq!(state.cp_air_assignment_write_count, 1);
}

#[test]
fn pure_transition_rejects_route_input_and_identity_mismatch_before_mutation() {
    let private = private_cp348_case(DehumidificationControlType::ConstantSensibleHeatRatio);
    assert!(private.is_some());
    let Some((runtime, system, predecessor)) = private else {
        return;
    };
    let mut state =
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentRuntimeState::new(
            system.id,
        );
    let before = state.clone();
    assert!(
        advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_state(
            &mut state,
            predecessor,
            None,
        )
        .is_none()
    );
    assert_eq!(state, before);
    for value in [-1.0, f64::NAN, f64::INFINITY, f64::MAX] {
        assert!(
            advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_state(
                &mut state,
                predecessor,
                Some(
                    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentActiveInput {
                        mixed_air_humidity_ratio: value,
                    },
                ),
            )
            .is_none()
        );
        assert_eq!(state, before);
    }

    let completed = completed_cp348_case(-1_000.0, 1.0, false);
    assert!(completed.is_some());
    let Some((_, _, skipped)) = completed else {
        return;
    };
    assert!(
        advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_state(
            &mut state,
            skipped,
            owner_input(&runtime, system.id, predecessor),
        )
        .is_none()
    );
    assert_eq!(state, before);

    let mut wrong_state =
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentRuntimeState::new(
            ep_model::IdealLoadsAirSystemId(system.id.0 + 1),
        );
    let before = wrong_state.clone();
    assert!(
        advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_state(
            &mut wrong_state,
            predecessor,
            owner_input(&runtime, system.id, predecessor),
        )
        .is_none()
    );
    assert_eq!(wrong_state, before);
}

#[test]
fn private_owner_input_requires_exact_same_call_cp329_bits() {
    let private = private_cp348_case(DehumidificationControlType::ConstantSensibleHeatRatio);
    assert!(private.is_some());
    let Some((runtime, system, predecessor)) = private else {
        return;
    };
    let owner = runtime
        .units
        .get(&system.id)
        .and_then(|unit| unit.calc_cooling_mixed_air_call.latest);
    let witness = runtime.cooling_mixed_air_call_latest_witness(system.id);
    assert!(owner.is_some());
    assert!(witness.is_some());
    let (Some(owner), Some(witness)) = (owner, witness) else {
        return;
    };
    assert!(active_input_from_owner_for_test(predecessor, owner, witness).is_some());

    let mut wrong_call = owner;
    wrong_call.parent_call_ordinal += 1;
    assert!(active_input_from_owner_for_test(predecessor, wrong_call, witness).is_none());

    let mut wrong_bits = witness;
    wrong_bits.mixed_air_humidity_ratio = wrong_bits
        .mixed_air_humidity_ratio
        .map(|value| f64::from_bits(value.to_bits() ^ 1));
    assert!(active_input_from_owner_for_test(predecessor, owner, wrong_bits).is_none());
}
