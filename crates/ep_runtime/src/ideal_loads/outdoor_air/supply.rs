//! Supply mass-flow and supply-air helpers for IdealLoads outdoor-air compatibility.

use crate::{ideal_loads::IdealLoadsSensibleMode, zone_equipment::ZoneSysEnergyDemand};
use ep_model::IdealLoadsAirSystem;

use super::{IdealLoadsOutdoorAirNodeState, SMALL_TEMPERATURE_DIFFERENCE_C};

pub(super) fn outdoor_air_supply_mass_flow_rate_kg_per_s(
    system: &IdealLoadsAirSystem,
    zone_state: IdealLoadsOutdoorAirNodeState,
    demand: ZoneSysEnergyDemand,
    mode: IdealLoadsSensibleMode,
    cp_air_j_per_kg_k: f64,
    outdoor_air_mass_flow_rate_kg_per_s: f64,
) -> f64 {
    let sensible_flow_rate_kg_per_s = match mode {
        IdealLoadsSensibleMode::Heating => {
            let delta_t =
                system.maximum_heating_supply_air_temperature_c - zone_state.air_temperature_c;
            if delta_t > SMALL_TEMPERATURE_DIFFERENCE_C
                && demand.remaining_output_req_to_heat_sp_w > 0.0
            {
                demand.remaining_output_req_to_heat_sp_w / (cp_air_j_per_kg_k * delta_t)
            } else {
                0.0
            }
        }
        IdealLoadsSensibleMode::Cooling => {
            let delta_t =
                system.minimum_cooling_supply_air_temperature_c - zone_state.air_temperature_c;
            if delta_t < -SMALL_TEMPERATURE_DIFFERENCE_C
                && demand.remaining_output_req_to_cool_sp_w < 0.0
            {
                demand.remaining_output_req_to_cool_sp_w / (cp_air_j_per_kg_k * delta_t)
            } else {
                0.0
            }
        }
        IdealLoadsSensibleMode::Deadband | IdealLoadsSensibleMode::Off => 0.0,
    };
    outdoor_air_mass_flow_rate_kg_per_s
        .max(sensible_flow_rate_kg_per_s)
        .max(0.0)
}

pub(super) fn supply_air_state(
    system: &IdealLoadsAirSystem,
    zone_state: IdealLoadsOutdoorAirNodeState,
    demand: ZoneSysEnergyDemand,
    mode: IdealLoadsSensibleMode,
    cp_air_j_per_kg_k: f64,
    supply_mass_flow_rate_kg_per_s: f64,
    mixed_air_temperature_c: f64,
    mixed_air_humidity_ratio: f64,
) -> (f64, f64) {
    if supply_mass_flow_rate_kg_per_s <= 0.0 {
        return (mixed_air_temperature_c, mixed_air_humidity_ratio);
    }

    let supply_air_temperature_c = match mode {
        IdealLoadsSensibleMode::Cooling => (demand.remaining_output_req_to_cool_sp_w
            / (cp_air_j_per_kg_k * supply_mass_flow_rate_kg_per_s)
            + zone_state.air_temperature_c)
            .max(system.minimum_cooling_supply_air_temperature_c)
            .min(mixed_air_temperature_c),
        IdealLoadsSensibleMode::Heating => (demand.remaining_output_req_to_heat_sp_w
            / (cp_air_j_per_kg_k * supply_mass_flow_rate_kg_per_s)
            + zone_state.air_temperature_c)
            .min(system.maximum_heating_supply_air_temperature_c)
            .max(mixed_air_temperature_c),
        IdealLoadsSensibleMode::Deadband | IdealLoadsSensibleMode::Off => mixed_air_temperature_c,
    };
    (supply_air_temperature_c, mixed_air_humidity_ratio)
}
