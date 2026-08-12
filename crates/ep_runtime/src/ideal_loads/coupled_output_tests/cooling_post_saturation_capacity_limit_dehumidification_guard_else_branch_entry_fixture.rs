use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchEntrySnapshot as Snapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyEnthalpyAssignmentSnapshot as Predecessor,
    private_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_characterization,
};

pub(super) fn calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_snapshot(
    predecessor: Predecessor,
) -> Snapshot {
    private_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_entry_characterization(
        predecessor,
    )
    .expect("valid CP418 coupled-output fixture")
}
