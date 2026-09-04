use crate::ideal_loads::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowContinueWarningCallSnapshot,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCallSnapshot,
    private_heating_outdoor_air_maximum_flow_continue_warning_call_characterization,
};

pub(super) fn calculation_heating_outdoor_air_maximum_flow_continue_warning_call_snapshot(
    predecessor: PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCallSnapshot,
) -> PurchasedAirCalcHeatingOutdoorAirMaximumFlowContinueWarningCallSnapshot {
    private_heating_outdoor_air_maximum_flow_continue_warning_call_characterization(predecessor)
        .expect("CP440 fixture characterization")
}
