use crate::ideal_loads::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentSnapshot,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardSnapshot,
    private_heating_outdoor_air_maximum_flow_first_warning_guard_characterization,
};

pub(super) fn calculation_heating_outdoor_air_maximum_flow_first_warning_guard_snapshot(
    predecessor: PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentSnapshot,
) -> PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardSnapshot {
    private_heating_outdoor_air_maximum_flow_first_warning_guard_characterization(predecessor, 0)
        .expect("CP437 fixture characterization")
}
