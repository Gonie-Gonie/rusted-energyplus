use ep_model::IdealLoadsLimit;

use crate::ideal_loads::{
    PurchasedAirCalcHeatingOperatingModeDeadbandAssignmentSnapshot,
    PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardSnapshot,
    private_heating_outdoor_air_maximum_flow_guard_characterization,
};

pub(super) fn calculation_heating_outdoor_air_maximum_flow_guard_snapshot(
    predecessor: PurchasedAirCalcHeatingOperatingModeDeadbandAssignmentSnapshot,
    heating_limit: IdealLoadsLimit,
    outdoor_air_mass_flow_rate_kg_per_s: f64,
    maximum_heating_air_mass_flow_rate_kg_per_s: f64,
) -> PurchasedAirCalcHeatingOutdoorAirMaximumFlowGuardSnapshot {
    private_heating_outdoor_air_maximum_flow_guard_characterization(
        predecessor,
        heating_limit,
        outdoor_air_mass_flow_rate_kg_per_s,
        maximum_heating_air_mass_flow_rate_kg_per_s,
    )
    .expect("CP435 fixture characterization")
}
