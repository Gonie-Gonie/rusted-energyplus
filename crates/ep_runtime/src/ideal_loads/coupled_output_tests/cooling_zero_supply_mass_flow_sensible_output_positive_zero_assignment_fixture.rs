use crate::ideal_loads::{
    PurchasedAirCalcCoolingZeroSupplyMassFlowSensibleOutputPositiveZeroAssignmentSnapshot,
    PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyTemperatureMixedAirAssignmentSnapshot,
    private_cooling_zero_supply_mass_flow_sensible_output_positive_zero_assignment_characterization,
};

pub(super) fn calculation_cooling_zero_supply_mass_flow_sensible_output_positive_zero_assignment_snapshot(
    predecessor: PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyTemperatureMixedAirAssignmentSnapshot,
) -> PurchasedAirCalcCoolingZeroSupplyMassFlowSensibleOutputPositiveZeroAssignmentSnapshot {
    private_cooling_zero_supply_mass_flow_sensible_output_positive_zero_assignment_characterization(
        predecessor,
    )
    .expect("CP428 fixture characterization")
}
