use crate::ideal_loads::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCallSnapshot,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCounterIncrementSnapshot,
    private_heating_outdoor_air_maximum_flow_first_warning_call_characterization,
};

pub(super) fn calculation_heating_outdoor_air_maximum_flow_first_warning_call_snapshot(
    predecessor: PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCounterIncrementSnapshot,
) -> PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCallSnapshot {
    private_heating_outdoor_air_maximum_flow_first_warning_call_characterization(predecessor)
        .expect("CP439 fixture characterization")
}
