use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardElseBranchEntrySnapshot,
    PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyEnthalpyMixedAirAssignmentSnapshot,
    private_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_characterization,
};

pub(super) fn calculation_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_snapshot(
    predecessor: PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardElseBranchEntrySnapshot,
) -> PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyEnthalpyMixedAirAssignmentSnapshot {
    let mixed_air_enthalpy = predecessor
        .cooling_supply_mass_flow_positive_guard_else_branch_entered
        .then_some(42_000.0);
    private_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_characterization(
        predecessor,
        mixed_air_enthalpy,
    )
    .expect("CP425 fixture characterization")
}
