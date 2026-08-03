use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioPreSaturationOriginalAssignmentSnapshot as Predecessor,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationAssignmentSnapshot as Snapshot,
    private_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_characterization,
};

pub(super) fn calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_snapshot(
    predecessor: Predecessor,
    pressure_pa: f64,
) -> Snapshot {
    let active = predecessor
        .post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment_executed;
    private_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_characterization(
        predecessor,
        active.then_some(pressure_pa),
    )
    .expect("valid CP412 coupled-output fixture")
}
