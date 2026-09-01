use crate::ideal_loads::{
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentSnapshot,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardSnapshot,
    private_heating_outdoor_air_maximum_flow_body_volume_flow_assignment_characterization,
};

pub(super) fn calculation_heating_outdoor_air_maximum_flow_body_volume_flow_assignment_snapshot(
    predecessor: PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardSnapshot,
    standard_air_density_kg_per_m3: f64,
) -> PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentSnapshot {
    private_heating_outdoor_air_maximum_flow_body_volume_flow_assignment_characterization(
        predecessor,
        standard_air_density_kg_per_m3,
    )
    .expect("CP436 fixture characterization")
}
