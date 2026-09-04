use crate::ideal_loads::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowContinueWarningCallSnapshot,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowContinueWarningTimestampCallSnapshot,
    private_heating_outdoor_air_maximum_flow_continue_warning_timestamp_call_characterization,
};

pub(super) fn calculation_heating_outdoor_air_maximum_flow_continue_warning_timestamp_call_snapshot(
    predecessor: PurchasedAirCalcHeatingOutdoorAirMaximumFlowContinueWarningCallSnapshot,
) -> PurchasedAirCalcHeatingOutdoorAirMaximumFlowContinueWarningTimestampCallSnapshot {
    private_heating_outdoor_air_maximum_flow_continue_warning_timestamp_call_characterization(
        predecessor,
    )
    .expect("CP441 fixture characterization")
}
