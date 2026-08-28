use crate::ideal_loads::{
    PurchasedAirCalcCoolingEntryGateCommittedHeatingModeGuardNumericOperands,
    PurchasedAirCalcHeatingModeGuardActiveInput, PurchasedAirCalcHeatingModeGuardSnapshot,
    PurchasedAirCalcHeatingOrNoLoadCaseEntrySnapshot, PurchasedAirTemperatureControlType,
    private_heating_mode_guard_characterization,
};

pub(super) fn calculation_heating_mode_guard_snapshot(
    predecessor: PurchasedAirCalcHeatingOrNoLoadCaseEntrySnapshot,
    minimum_outdoor_air_sensible_output_w: f64,
    heating_setpoint_demand_w: f64,
) -> PurchasedAirCalcHeatingModeGuardSnapshot {
    let temperature_control_type = (minimum_outdoor_air_sensible_output_w
        < heating_setpoint_demand_w)
        .then_some(PurchasedAirTemperatureControlType::DualHeatCool);
    private_heating_mode_guard_characterization(
        predecessor,
        predecessor.heating_or_no_load_case_entered.then_some(
            PurchasedAirCalcHeatingModeGuardActiveInput {
                numeric: PurchasedAirCalcCoolingEntryGateCommittedHeatingModeGuardNumericOperands {
                    minimum_outdoor_air_sensible_output_w,
                    heating_setpoint_demand_w,
                },
                temperature_control_type,
            },
        ),
    )
    .expect("CP431 fixture characterization")
}
