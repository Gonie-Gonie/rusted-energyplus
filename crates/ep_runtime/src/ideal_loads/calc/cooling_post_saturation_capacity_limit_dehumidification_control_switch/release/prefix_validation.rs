//! CP385-to-CP386 retained-lineage validation.

use ep_model::IdealLoadsAirSystem;

use super::super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchActiveInput as ActiveInput,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchSnapshot as Snapshot,
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_switch_state,
};
use super::snapshot_validation::snapshots_match_bit_exact;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentSnapshot as Predecessor,
    PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState,
};

pub(super) fn direct_predecessor_is_retained_and_complete(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
) -> bool {
    let witness = runtime
        .cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_latest_witness(system.id);
    unit.calc_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment
        .latest
        .is_some_and(|retained| {
            crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_snapshots_match_bit_exact(
                retained,
                predecessor,
            )
        })
        && crate::ideal_loads::calc::completed_direct_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_is_consistent(
            runtime,
            unit,
            system,
            predecessor,
            witness,
        )
}

pub(super) fn switch_links_to_predecessor(
    snapshot: Snapshot,
    predecessor: Predecessor,
    active_input: Option<ActiveInput>,
) -> bool {
    let mut state = State::new(predecessor.system);
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_switch_state(
        &mut state,
        predecessor,
        active_input,
    )
    .is_some_and(|expected| snapshots_match_bit_exact(expected, snapshot))
}
