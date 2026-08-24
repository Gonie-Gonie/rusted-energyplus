use crate::ideal_loads::{
    PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyHumidityRatioMixedAirAssignmentSnapshot,
    PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyTemperatureMixedAirAssignmentSnapshot,
    private_cooling_zero_supply_mass_flow_supply_temperature_mixed_air_assignment_characterization,
};

pub(super) fn calculation_cooling_zero_supply_mass_flow_supply_temperature_mixed_air_assignment_snapshot(
    predecessor: PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyHumidityRatioMixedAirAssignmentSnapshot,
) -> PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyTemperatureMixedAirAssignmentSnapshot {
    let mixed_air_temperature_c = predecessor
        .cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment_executed
        .then_some(22.0);
    private_cooling_zero_supply_mass_flow_supply_temperature_mixed_air_assignment_characterization(
        predecessor,
        mixed_air_temperature_c,
    )
    .expect("CP427 fixture characterization")
}
