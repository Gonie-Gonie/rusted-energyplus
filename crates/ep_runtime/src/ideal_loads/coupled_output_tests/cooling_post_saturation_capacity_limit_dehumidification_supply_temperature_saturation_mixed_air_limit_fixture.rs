use crate::ideal_loads::{
    PurchasedAirCalcCoolingMixedAirCallSnapshot as MixedAirOwner,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationAssignmentSnapshot as Predecessor,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationMixedAirLimitSnapshot as Snapshot,
    private_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_characterization,
};

pub(super) fn calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_snapshot(
    predecessor: Predecessor,
    mixed_air_owner: MixedAirOwner,
) -> Snapshot {
    let owner = predecessor
        .post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_executed
        .then_some(mixed_air_owner);
    private_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_characterization(
        predecessor,
        owner,
    )
    .expect("valid CP415 coupled-output fixture")
}
