use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationGuardSnapshot as Predecessor,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationAssignmentSnapshot as Snapshot,
    private_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_characterization,
};

pub(super) fn calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_snapshot(
    predecessor: Predecessor,
    barometric_pressure_pa: f64,
) -> Snapshot {
    private_cooling_post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_characterization(
        predecessor,
        barometric_pressure_pa,
    )
    .expect("valid CP414 coupled-output fixture")
}
