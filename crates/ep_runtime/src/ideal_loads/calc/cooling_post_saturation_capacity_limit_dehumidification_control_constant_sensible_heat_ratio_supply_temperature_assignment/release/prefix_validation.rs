//! Exact retained CP379 and direct CP388 prefix validation for CP389.

use super::super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentRetainedInput as RetainedInput,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentSnapshot as Snapshot,
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_state,
};
use super::snapshot_validation::snapshots_match_bit_exact;
use crate::ideal_loads::calc::{
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_committed_latest_snapshot_is_consistent,
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_snapshots_match_bit_exact,
    cooling_supply_enthalpy_post_saturation_assignment_committed_latest_snapshot_is_consistent,
    cooling_supply_enthalpy_post_saturation_assignment_snapshots_match_bit_exact,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentSnapshot as Predecessor,
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentSnapshot as TemperatureOwner,
    PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState,
};
use ep_model::IdealLoadsAirSystem;

pub(super) fn direct_prefix_is_retained_and_complete(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
    owner: TemperatureOwner,
) -> bool {
    let Some(retained_predecessor) = unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment
        .latest
    else {
        return false;
    };
    let Some(predecessor_witness) = runtime
        .cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_latest_witness(system.id)
    else {
        return false;
    };
    let Some(retained_owner) = unit
        .calc_cooling_supply_enthalpy_post_saturation_assignment
        .latest
    else {
        return false;
    };
    let Some(owner_witness) =
        runtime.cooling_supply_enthalpy_post_saturation_assignment_latest_witness(system.id)
    else {
        return false;
    };
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_snapshots_match_bit_exact(
        retained_predecessor,
        predecessor,
    ) && cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_snapshots_match_bit_exact(
        predecessor_witness,
        predecessor,
    ) && cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_committed_latest_snapshot_is_consistent(
        unit,
        system,
        predecessor_witness,
    ) && cooling_supply_enthalpy_post_saturation_assignment_snapshots_match_bit_exact(
        retained_owner,
        owner,
    ) && cooling_supply_enthalpy_post_saturation_assignment_snapshots_match_bit_exact(
        owner_witness,
        owner,
    ) && cooling_supply_enthalpy_post_saturation_assignment_committed_latest_snapshot_is_consistent(
        unit,
        owner_witness,
    )
}

pub(super) fn assignment_links_to_prefix(
    snapshot: Snapshot,
    predecessor: Predecessor,
    owner: TemperatureOwner,
) -> bool {
    let mut state = State::new(predecessor.system);
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_state(
        &mut state,
        predecessor,
        RetainedInput {
            cp379_temperature_owner: owner,
            active_owners: None,
        },
    )
    .is_some_and(|expected| snapshots_match_bit_exact(expected, snapshot))
}
