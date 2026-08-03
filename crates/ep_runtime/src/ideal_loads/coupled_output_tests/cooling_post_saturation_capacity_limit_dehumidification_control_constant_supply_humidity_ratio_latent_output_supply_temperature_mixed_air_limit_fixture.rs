// This fixture is included only by its parent's `cfg(test)` module declaration.
#[cfg(test)]
const _: () = ();

use crate::ideal_loads::{
    PurchasedAirCalcCoolingMixedAirCallSnapshot as MixedAirOwner,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureAssignmentSnapshot as Predecessor,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureMixedAirLimitSnapshot as Snapshot,
    private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit_characterization,
};

pub(super) fn calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit_snapshot(
    predecessor: Predecessor,
    mixed_air_owner: MixedAirOwner,
) -> Snapshot {
    let active = predecessor
        .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_executed;
    private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit_characterization(
        predecessor,
        active.then_some(mixed_air_owner),
    )
    .expect("CP408 coupled-output fixture must satisfy exact CP407/CP329 lineage")
}
