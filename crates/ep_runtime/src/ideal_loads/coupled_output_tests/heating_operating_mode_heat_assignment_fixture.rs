use crate::ideal_loads::{
    PurchasedAirCalcHeatingModeGuardSnapshot,
    PurchasedAirCalcHeatingOperatingModeHeatAssignmentSnapshot,
    private_heating_operating_mode_heat_assignment_characterization,
};

pub(super) fn calculation_heating_operating_mode_heat_assignment_snapshot(
    predecessor: PurchasedAirCalcHeatingModeGuardSnapshot,
) -> PurchasedAirCalcHeatingOperatingModeHeatAssignmentSnapshot {
    private_heating_operating_mode_heat_assignment_characterization(predecessor)
        .expect("CP432 fixture characterization")
}
