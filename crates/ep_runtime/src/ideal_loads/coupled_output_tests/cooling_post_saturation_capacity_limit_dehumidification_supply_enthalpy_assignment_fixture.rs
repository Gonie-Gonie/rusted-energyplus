use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyEnthalpyAssignmentSnapshot as Snapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioAssignmentSnapshot as Predecessor,
    private_cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_characterization,
};

pub(super) fn calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_snapshot(
    predecessor: Predecessor,
) -> Snapshot {
    private_cooling_post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_characterization(
        predecessor,
    )
    .expect("valid CP417 coupled-output fixture")
}
