use crate::ideal_loads::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCounterIncrementSnapshot,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardSnapshot,
    private_heating_outdoor_air_maximum_flow_first_warning_counter_increment_characterization,
};

pub(super) fn calculation_heating_outdoor_air_maximum_flow_first_warning_counter_increment_snapshot(
    predecessor: PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardSnapshot,
) -> PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCounterIncrementSnapshot {
    private_heating_outdoor_air_maximum_flow_first_warning_counter_increment_characterization(
        predecessor,
    )
    .expect("CP438 fixture characterization")
}
