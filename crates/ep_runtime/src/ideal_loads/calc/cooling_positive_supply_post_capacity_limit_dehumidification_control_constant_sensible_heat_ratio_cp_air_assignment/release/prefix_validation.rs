//! CP329/CP348-to-CP349 retained-lineage validation.

#[cfg(test)]
use super::super::PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentActiveInput;
use super::super::transition::predecessor_route;
use super::super::{
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingMixedAirCallSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntrySnapshot,
    cooling_mixed_air_call_snapshots_match_bit_exact,
};
use crate::psychrometrics::energyplus_psy_cp_air_fn_w;

pub(super) fn cp_air_assignment_links_to_predecessor(
    assignment:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot,
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntrySnapshot,
) -> bool {
    let Some(route) = predecessor_route(predecessor) else {
        return false;
    };
    assignment.system == predecessor.system
        && assignment.parent_call_ordinal == predecessor.parent_call_ordinal
        && assignment.controlled_zone == predecessor.controlled_zone
        && assignment.unit_body_entered == predecessor.unit_body_entered
        && assignment.predecessor_cooling_body_entered
            == predecessor.predecessor_cooling_body_entered
        && assignment.predecessor_no_outdoor_air_fallback_entered
            == predecessor.predecessor_no_outdoor_air_fallback_entered
        && assignment.predecessor_positive_supply_mass_flow_body_entered
            == predecessor.predecessor_positive_supply_mass_flow_body_entered
        && assignment.unit_off_skipped == predecessor.unit_off_skipped
        && assignment.non_cooling_skipped == predecessor.non_cooling_skipped
        && assignment.positive_guard_false_fallthrough_skipped
            == predecessor.positive_guard_false_fallthrough_skipped
        && assignment.predecessor_dehumidification_control_type
            == predecessor.predecessor_dehumidification_control_type
        && assignment.predecessor_dehumidification_control_none_case_completed
            == predecessor.predecessor_dehumidification_control_none_case_completed
        && assignment.predecessor_dehumidification_control_none_case_completed_skip
            == predecessor.dehumidification_control_none_case_completed_skip
        && assignment.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered
            == predecessor.dehumidification_control_constant_sensible_heat_ratio_case_entered
        && assignment.predecessor_dehumidification_control_humidistat_case_selected_skip
            == predecessor.dehumidification_control_humidistat_case_selected_skip
        && assignment
            .predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
            == predecessor
                .dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
        && assignment.dehumidification_control_none_case_completed_skip
            == (route == Route::DehumidificationControlNoneCaseCompletedSkip)
        && assignment
            .dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed
            == (route == Route::DehumidificationControlConstantSensibleHeatRatioCpAirAssigned)
        && assignment.dehumidification_control_humidistat_case_selected_skip
            == (route == Route::DehumidificationControlHumidistatCaseSelectedSkip)
        && assignment.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip
            == (route == Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip)
}

pub(super) fn active_operand_links_to_retained_owner(
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntrySnapshot,
    owner: PurchasedAirCalcCoolingMixedAirCallSnapshot,
    owner_witness: PurchasedAirCalcCoolingMixedAirCallSnapshot,
    operand: Option<f64>,
) -> bool {
    let Some(operand) = operand else {
        return false;
    };
    predecessor_route(predecessor)
        == Some(Route::DehumidificationControlConstantSensibleHeatRatioCpAirAssigned)
        && owner.system == predecessor.system
        && owner.parent_call_ordinal == predecessor.parent_call_ordinal
        && owner.controlled_zone == predecessor.controlled_zone
        && owner.cooling_call_executed
        && owner.no_outdoor_air_fallback_entered
        && owner.mixed_air_humidity_ratio_assigned
        && owner
            .mixed_air_humidity_ratio
            .is_some_and(|value| value.to_bits() == operand.to_bits())
        && cooling_mixed_air_call_snapshots_match_bit_exact(owner, owner_witness)
        && operand.is_finite()
        && operand >= 0.0
        && energyplus_psy_cp_air_fn_w(operand).is_finite()
}

#[cfg(test)]
pub(in crate::ideal_loads::calc) fn active_input_from_owner(
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntrySnapshot,
    owner: PurchasedAirCalcCoolingMixedAirCallSnapshot,
    owner_witness: PurchasedAirCalcCoolingMixedAirCallSnapshot,
) -> Option<
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentActiveInput,
>{
    let operand = owner.mixed_air_humidity_ratio;
    active_operand_links_to_retained_owner(predecessor, owner, owner_witness, operand).then_some(
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentActiveInput {
            mixed_air_humidity_ratio: operand?,
        },
    )
}
