//! IdealLoads facility-meter bindings.

use super::report::{
    ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_COOLING_FUEL_ENERGY,
    ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_HEATING_FUEL_ENERGY,
};
use ep_model::IdealLoadsFuelType;

/// Runtime binding from an IdealLoads fuel type to a facility meter source row.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IdealLoadsFacilityMeterBinding {
    /// Registered EnergyPlus-style facility meter name.
    pub meter_name: &'static str,
    /// Detailed IdealLoads fuel-energy output that feeds this diagnostic meter.
    pub fuel_energy_variable: &'static str,
}

/// Source label for IdealLoads facility-meter aggregation in reports.
pub const IDEAL_LOADS_METER_AGGREGATION_SOURCE: &str = "ep_runtime::RuntimeMeterRegistry";
/// Source label for IdealLoads meter-to-fuel-energy output binding in reports.
pub const IDEAL_LOADS_METER_FUEL_ENERGY_BINDING_SOURCE: &str =
    "ep_runtime::ideal_loads_facility_meter_binding";

/// Returns the diagnostic facility meter binding for a supported IdealLoads fuel.
#[must_use]
pub fn ideal_loads_facility_meter_binding(
    fuel_type: IdealLoadsFuelType,
) -> Option<IdealLoadsFacilityMeterBinding> {
    match fuel_type {
        IdealLoadsFuelType::DistrictHeatingWater => Some(IdealLoadsFacilityMeterBinding {
            meter_name: "DistrictHeatingWater:Facility",
            fuel_energy_variable: ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_HEATING_FUEL_ENERGY,
        }),
        IdealLoadsFuelType::DistrictCooling => Some(IdealLoadsFacilityMeterBinding {
            meter_name: "DistrictCooling:Facility",
            fuel_energy_variable: ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_COOLING_FUEL_ENERGY,
        }),
        _ => None,
    }
}
