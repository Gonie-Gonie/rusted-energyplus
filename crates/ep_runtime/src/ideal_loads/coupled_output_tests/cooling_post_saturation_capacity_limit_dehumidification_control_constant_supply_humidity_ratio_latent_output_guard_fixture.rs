// This fixture is included only by its parent's `cfg(test)` module declaration.
#[cfg(test)]
const _: () = ();

use crate::ideal_loads::{
    PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot as CapacityOwner,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardSnapshot as CapacityCorroborator,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputAssignmentSnapshot as Predecessor,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputGuardActiveInput as ActiveInput,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputGuardSnapshot as Snapshot,
    private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard_characterization,
};

pub(super) fn calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard_snapshot(
    predecessor: Predecessor,
    capacity_owner: CapacityOwner,
    capacity_corroborator: CapacityCorroborator,
) -> Snapshot {
    let active = predecessor
        .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_assignment_executed;
    let input = active.then(|| ActiveInput {
        cooling_latent_output_w: predecessor
            .cooling_latent_output_w
            .expect("active CP402 fixture CP401 latent-output owner"),
        maximum_total_cooling_capacity_w: capacity_owner
            .maximum_total_cooling_capacity_w
            .expect("active CP402 fixture CP321 capacity owner"),
        cp401_cooling_latent_output_owned_read: true,
        cp321_maximum_total_cooling_capacity_owned_read: true,
        cp340_same_call_maximum_total_cooling_capacity_bit_corroborated: true,
    });
    if active {
        assert_eq!(
            capacity_owner
                .maximum_total_cooling_capacity_w
                .map(f64::to_bits),
            capacity_corroborator
                .maximum_total_cooling_capacity_w
                .map(f64::to_bits),
        );
    }
    private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard_characterization(
        predecessor,
        input,
    )
    .expect("CP402 coupled-output fixture must satisfy exact owner lineage")
}
