//! No-OA sensible mass-flow helper calculations.

use ep_model::IdealLoadsAirSystem;

use super::limits::{IdealLoadsSensibleLimitContext, capacity_limit_w, flow_limit_kg_per_s};
use super::types::IdealLoadsZoneState;

pub(super) const SMALL_TEMPERATURE_DIFFERENCE_C: f64 = 0.001;

pub(super) fn limited_heating_mass_flow_rate_kg_per_s(
    system: &IdealLoadsAirSystem,
    zone_state: IdealLoadsZoneState,
    heating_load_w: f64,
    cp_air_j_per_kg_k: f64,
    limit_context: IdealLoadsSensibleLimitContext,
) -> f64 {
    if matches!(
        capacity_limit_w(
            system.heating_limit,
            system.maximum_sensible_heating_capacity_w,
        ),
        Some(capacity_limit_w) if capacity_limit_w <= 0.0
    ) {
        return 0.0;
    }

    let heating_delta_t =
        system.maximum_heating_supply_air_temperature_c - zone_state.air_temperature_c;
    let mut mass_flow_rate_kg_per_s =
        if heating_load_w > 0.0 && heating_delta_t > SMALL_TEMPERATURE_DIFFERENCE_C {
            heating_load_w / (cp_air_j_per_kg_k * heating_delta_t)
        } else {
            0.0
        };

    if let Some(maximum_mass_flow_rate_kg_per_s) = flow_limit_kg_per_s(
        system.heating_limit,
        system.maximum_heating_air_flow_rate_m3_per_s,
        limit_context,
    ) && maximum_mass_flow_rate_kg_per_s > 0.0
    {
        mass_flow_rate_kg_per_s = mass_flow_rate_kg_per_s.min(maximum_mass_flow_rate_kg_per_s);
    }

    mass_flow_rate_kg_per_s
}

pub(super) fn limited_cooling_mass_flow_rate_kg_per_s(
    system: &IdealLoadsAirSystem,
    zone_state: IdealLoadsZoneState,
    cooling_load_w: f64,
    cp_air_j_per_kg_k: f64,
    limit_context: IdealLoadsSensibleLimitContext,
) -> f64 {
    if matches!(
        capacity_limit_w(system.cooling_limit, system.maximum_total_cooling_capacity_w),
        Some(capacity_limit_w) if capacity_limit_w <= 0.0
    ) {
        return 0.0;
    }

    let cooling_delta_t =
        zone_state.air_temperature_c - system.minimum_cooling_supply_air_temperature_c;
    let mut mass_flow_rate_kg_per_s =
        if cooling_load_w > 0.0 && cooling_delta_t > SMALL_TEMPERATURE_DIFFERENCE_C {
            cooling_load_w / (cp_air_j_per_kg_k * cooling_delta_t)
        } else {
            0.0
        };

    if let Some(maximum_mass_flow_rate_kg_per_s) = flow_limit_kg_per_s(
        system.cooling_limit,
        system.maximum_cooling_air_flow_rate_m3_per_s,
        limit_context,
    ) && maximum_mass_flow_rate_kg_per_s > 0.0
    {
        mass_flow_rate_kg_per_s = mass_flow_rate_kg_per_s.min(maximum_mass_flow_rate_kg_per_s);
    }

    mass_flow_rate_kg_per_s
}
