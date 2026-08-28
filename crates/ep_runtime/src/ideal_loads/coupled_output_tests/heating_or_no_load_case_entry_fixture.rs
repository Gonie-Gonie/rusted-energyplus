use crate::ideal_loads::{
    PurchasedAirCalcCoolingZeroSupplyMassFlowTotalOutputPositiveZeroAssignmentSnapshot,
    PurchasedAirCalcHeatingOrNoLoadCaseEntrySnapshot,
    private_heating_or_no_load_case_entry_characterization,
};

pub(super) fn calculation_heating_or_no_load_case_entry_snapshot(
    predecessor: PurchasedAirCalcCoolingZeroSupplyMassFlowTotalOutputPositiveZeroAssignmentSnapshot,
) -> PurchasedAirCalcHeatingOrNoLoadCaseEntrySnapshot {
    private_heating_or_no_load_case_entry_characterization(predecessor)
        .expect("CP430 fixture characterization")
}
