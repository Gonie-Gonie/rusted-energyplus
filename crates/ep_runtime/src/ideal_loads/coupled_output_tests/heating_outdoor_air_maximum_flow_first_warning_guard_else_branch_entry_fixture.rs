use crate::ideal_loads::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowContinueWarningTimestampCallSnapshot,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardElseBranchEntrySnapshot,
    private_heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_characterization,
};

pub(super) fn calculation_heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_snapshot(
    predecessor: PurchasedAirCalcHeatingOutdoorAirMaximumFlowContinueWarningTimestampCallSnapshot,
) -> PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardElseBranchEntrySnapshot {
    private_heating_outdoor_air_maximum_flow_first_warning_guard_else_branch_entry_characterization(
        predecessor,
    )
    .expect("CP442 fixture characterization")
}
