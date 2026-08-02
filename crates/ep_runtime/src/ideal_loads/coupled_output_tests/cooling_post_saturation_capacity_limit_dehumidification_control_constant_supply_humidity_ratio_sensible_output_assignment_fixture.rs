use crate::ideal_loads::{
    PurchasedAirCalcCoolingMixedAirCallSnapshot as MixedAir,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCpAirAssignmentSnapshot as Predecessor,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioSensibleOutputAssignmentSnapshot as Snapshot,
    PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardSnapshot as SupplyFlow,
    private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_characterization,
};

pub(super) fn calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_snapshot(
    predecessor: Predecessor,
    mixed_air: MixedAir,
    supply_flow: SupplyFlow,
) -> Snapshot {
    let active = predecessor
        .dehumidification_control_none_or_constant_supply_humidity_ratio_cp_air_assignment_executed;
    private_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment_characterization(
        predecessor,
        active.then_some(mixed_air),
        active.then_some(supply_flow),
    )
    .expect("CP400 coupled-output fixture must satisfy exact owner lineage")
}
