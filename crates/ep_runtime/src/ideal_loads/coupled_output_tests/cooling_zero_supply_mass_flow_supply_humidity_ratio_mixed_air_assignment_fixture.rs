use crate::ideal_loads::{
    PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyEnthalpyMixedAirAssignmentSnapshot,
    PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyHumidityRatioMixedAirAssignmentSnapshot,
    private_cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment_characterization,
};

pub(super) fn calculation_cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment_snapshot(
    predecessor: PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyEnthalpyMixedAirAssignmentSnapshot,
) -> PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyHumidityRatioMixedAirAssignmentSnapshot {
    let mixed_air_humidity_ratio = predecessor
        .cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_executed
        .then_some(0.008);
    private_cooling_zero_supply_mass_flow_supply_humidity_ratio_mixed_air_assignment_characterization(
        predecessor,
        mixed_air_humidity_ratio,
    )
    .expect("CP426 fixture characterization")
}
