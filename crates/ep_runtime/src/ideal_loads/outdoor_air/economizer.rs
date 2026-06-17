//! Economizer outdoor-air flow reset helpers.

use crate::{
    ideal_loads::{IdealLoadsSensibleMode, moist_air_enthalpy_j_per_kg},
    zone_equipment::ZoneSysEnergyDemand,
};
use ep_model::{IdealLoadsAirSystem, OutdoorAirEconomizerType};

use super::{IdealLoadsOutdoorAirNodeState, SMALL_TEMPERATURE_DIFFERENCE_C};

pub(super) fn calc_economizer_adjusted_outdoor_air_mass_flow_rate_kg_per_s(
    system: &IdealLoadsAirSystem,
    zone_state: IdealLoadsOutdoorAirNodeState,
    recirculation_state: IdealLoadsOutdoorAirNodeState,
    outdoor_air_state: IdealLoadsOutdoorAirNodeState,
    demand: ZoneSysEnergyDemand,
    mode: IdealLoadsSensibleMode,
    cp_air_j_per_kg_k: f64,
    system_timestep_hours: f64,
    outdoor_air_mass_flow_rate_kg_per_s: &mut f64,
) -> f64 {
    if mode != IdealLoadsSensibleMode::Cooling
        || !economizer_allows_outdoor_air_flow_reset(
            system.outdoor_air_economizer_type,
            recirculation_state,
            outdoor_air_state,
        )
    {
        return 0.0;
    }

    let delta_t = outdoor_air_state.air_temperature_c - zone_state.air_temperature_c;
    if delta_t >= -SMALL_TEMPERATURE_DIFFERENCE_C
        || demand.remaining_output_req_to_cool_sp_w >= 0.0
        || !cp_air_j_per_kg_k.is_finite()
        || cp_air_j_per_kg_k <= 0.0
    {
        return 0.0;
    }

    let economizer_supply_mass_flow_rate_kg_per_s =
        demand.remaining_output_req_to_cool_sp_w / (cp_air_j_per_kg_k * delta_t);
    if !economizer_supply_mass_flow_rate_kg_per_s.is_finite()
        || economizer_supply_mass_flow_rate_kg_per_s <= *outdoor_air_mass_flow_rate_kg_per_s
    {
        return 0.0;
    }

    *outdoor_air_mass_flow_rate_kg_per_s = economizer_supply_mass_flow_rate_kg_per_s.max(0.0);
    system_timestep_hours.max(0.0)
}

fn economizer_allows_outdoor_air_flow_reset(
    economizer_type: OutdoorAirEconomizerType,
    recirculation_state: IdealLoadsOutdoorAirNodeState,
    outdoor_air_state: IdealLoadsOutdoorAirNodeState,
) -> bool {
    match economizer_type {
        OutdoorAirEconomizerType::NoEconomizer => false,
        OutdoorAirEconomizerType::DifferentialDryBulb => {
            outdoor_air_state.air_temperature_c < recirculation_state.air_temperature_c
        }
        OutdoorAirEconomizerType::DifferentialEnthalpy => {
            let outdoor_air_enthalpy_j_per_kg = moist_air_enthalpy_j_per_kg(
                outdoor_air_state.air_temperature_c,
                outdoor_air_state.air_humidity_ratio,
            );
            let recirculation_enthalpy_j_per_kg = moist_air_enthalpy_j_per_kg(
                recirculation_state.air_temperature_c,
                recirculation_state.air_humidity_ratio,
            );
            outdoor_air_enthalpy_j_per_kg < recirculation_enthalpy_j_per_kg
        }
    }
}
