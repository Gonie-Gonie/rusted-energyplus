use crate::ideal_loads::{
    PurchasedAirCalcHeatingModeGuardElseBranchEntrySnapshot,
    PurchasedAirCalcHeatingOperatingModeHeatAssignmentSnapshot,
    private_heating_mode_guard_else_branch_entry_characterization,
};

pub(super) fn calculation_heating_mode_guard_else_branch_entry_snapshot(
    predecessor: PurchasedAirCalcHeatingOperatingModeHeatAssignmentSnapshot,
) -> PurchasedAirCalcHeatingModeGuardElseBranchEntrySnapshot {
    private_heating_mode_guard_else_branch_entry_characterization(predecessor)
        .expect("CP433 fixture characterization")
}
