use crate::ideal_loads::{
    PurchasedAirCalcCoolingZeroSupplyMassFlowSensibleOutputPositiveZeroAssignmentSnapshot,
    PurchasedAirCalcCoolingZeroSupplyMassFlowTotalOutputPositiveZeroAssignmentSnapshot,
    private_cooling_zero_supply_mass_flow_total_output_positive_zero_assignment_characterization,
};

pub(super) fn calculation_cooling_zero_supply_mass_flow_total_output_positive_zero_assignment_snapshot(
    predecessor: PurchasedAirCalcCoolingZeroSupplyMassFlowSensibleOutputPositiveZeroAssignmentSnapshot,
) -> PurchasedAirCalcCoolingZeroSupplyMassFlowTotalOutputPositiveZeroAssignmentSnapshot {
    private_cooling_zero_supply_mass_flow_total_output_positive_zero_assignment_characterization(
        predecessor,
    )
    .expect("CP429 fixture characterization")
}
