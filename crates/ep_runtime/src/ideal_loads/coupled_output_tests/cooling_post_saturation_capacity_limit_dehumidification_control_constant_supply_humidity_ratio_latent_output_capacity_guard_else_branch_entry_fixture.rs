// This fixture is included only by its parent's `cfg(test)` module declaration.
#[cfg(test)]
const _: () = ();

use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputCapacityGuardElseBranchEntrySnapshot as Snapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputMaximumCapacityAssignmentSnapshot as Predecessor,
    private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_characterization,
};

pub(super) fn calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_snapshot(
    predecessor: Predecessor,
) -> Snapshot {
    private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_characterization(
        predecessor,
    )
    .expect("CP406 coupled-output fixture must satisfy exact predecessor lineage")
}
