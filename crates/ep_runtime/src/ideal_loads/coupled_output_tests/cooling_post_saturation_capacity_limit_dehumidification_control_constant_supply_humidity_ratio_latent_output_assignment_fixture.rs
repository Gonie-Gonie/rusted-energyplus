// This fixture is included only by its parent's `cfg(test)` module declaration.
#[cfg(test)]
const _: () = ();

use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputAssignmentSnapshot as Snapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioSensibleOutputAssignmentSnapshot as Predecessor,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputMaximumCapacityAssignmentSnapshot as CoolingTotalOutputOwner,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentSnapshot as CoolingTotalOutputCorroborator,
    private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment_characterization,
};

pub(super) fn calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment_snapshot(
    predecessor: Predecessor,
    cooling_total_output_owner: CoolingTotalOutputOwner,
    cooling_total_output_corroborator: CoolingTotalOutputCorroborator,
) -> Snapshot {
    let active = predecessor
        .dehumidification_control_none_or_constant_supply_humidity_ratio_sensible_output_assignment_executed;
    private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment_characterization(
        predecessor,
        active.then_some(cooling_total_output_owner),
        active.then_some(cooling_total_output_corroborator),
    )
    .expect("CP401 coupled-output fixture must satisfy exact owner lineage")
}
