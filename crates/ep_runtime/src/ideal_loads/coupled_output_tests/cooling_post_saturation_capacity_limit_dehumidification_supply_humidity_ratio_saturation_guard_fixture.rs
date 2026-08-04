use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationAssignmentSnapshot as Predecessor,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationGuardSnapshot as Snapshot,
    private_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_characterization,
};

pub(super) fn calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_snapshot(
    predecessor: Predecessor,
) -> Snapshot {
    private_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_characterization(
        predecessor,
    )
    .expect("valid CP413 coupled-output fixture")
}
