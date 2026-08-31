use crate::ideal_loads::{
    PurchasedAirCalcHeatingModeGuardElseBranchEntrySnapshot,
    PurchasedAirCalcHeatingOperatingModeDeadbandAssignmentSnapshot,
    private_heating_operating_mode_deadband_assignment_characterization,
};

pub(super) fn calculation_heating_operating_mode_deadband_assignment_snapshot(
    predecessor: PurchasedAirCalcHeatingModeGuardElseBranchEntrySnapshot,
) -> PurchasedAirCalcHeatingOperatingModeDeadbandAssignmentSnapshot {
    private_heating_operating_mode_deadband_assignment_characterization(predecessor)
        .expect("CP434 fixture characterization")
}
