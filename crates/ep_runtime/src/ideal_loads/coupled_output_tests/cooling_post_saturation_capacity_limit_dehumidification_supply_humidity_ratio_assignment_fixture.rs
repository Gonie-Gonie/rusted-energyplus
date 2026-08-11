use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioAssignmentSnapshot as Snapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationMixedAirLimitSnapshot as Predecessor,
    private_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment_characterization,
};

pub(super) fn calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment_snapshot(
    predecessor: Predecessor,
) -> Snapshot {
    private_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment_characterization(
        predecessor,
    )
    .expect("valid CP416 coupled-output fixture")
}
