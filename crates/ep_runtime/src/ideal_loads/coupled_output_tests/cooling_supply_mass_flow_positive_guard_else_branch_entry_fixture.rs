use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputSupplyTemperatureAssignmentSnapshot as Predecessor,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardElseBranchEntrySnapshot as Snapshot,
    private_cooling_supply_mass_flow_positive_guard_else_branch_entry_characterization,
};

pub(super) fn calculation_cooling_supply_mass_flow_positive_guard_else_branch_entry_snapshot(
    predecessor: Predecessor,
) -> Snapshot {
    private_cooling_supply_mass_flow_positive_guard_else_branch_entry_characterization(predecessor)
        .expect("valid CP424 coupled-output fixture")
}
