//! No-OA IdealLoads sensible load calculation.

use crate::{
    energyplus_moist_air_specific_heat_j_per_kg_k, energyplus_psychrometric_humidity_ratio_from_rh,
    zone_equipment::ZoneSysEnergyDemand,
};
use ep_model::{DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystem};

use super::limits::{
    IdealLoadsSensibleLimitContext, capacity_limit_w, cooling_capacity_limit_is_zero,
    flow_limit_kg_per_s, heating_capacity_limit_is_zero,
};
use super::psychrometrics::{
    MINIMUM_HUMIDITY_RATIO, humidity_ratio_from_enthalpy_and_dry_bulb, moist_air_enthalpy_j_per_kg,
    nearly_equal_humidity,
};
use super::types::{IdealLoadsSensibleMode, IdealLoadsSensibleResult, IdealLoadsZoneState};

const SMALL_TEMPERATURE_DIFFERENCE_C: f64 = 0.001;
const SMALL_HUMIDITY_RATIO_DIFFERENCE: f64 = 0.00025;

/// Calculates the no-outdoor-air, no-limit, sensible-only IdealLoads branch.
///
/// The zone demand values must already come from a source-order
/// `ZoneSysEnergyDemand` equivalent. This function intentionally does not
/// synthesize zone demand from a simplified zone model.
#[must_use]
pub fn calc_no_oa_no_limit_sensible_compat(
    system: &IdealLoadsAirSystem,
    zone_state: IdealLoadsZoneState,
    demand: ZoneSysEnergyDemand,
    unit_available: bool,
) -> IdealLoadsSensibleResult {
    calc_no_oa_no_limit_sensible_with_context_compat(
        system,
        zone_state,
        demand,
        unit_available,
        IdealLoadsSensibleLimitContext::default(),
    )
}

/// Calculates the no-outdoor-air, no-limit branch with an explicit
/// psychrometric context for humidity-control diagnostics.
#[must_use]
pub fn calc_no_oa_no_limit_sensible_with_context_compat(
    system: &IdealLoadsAirSystem,
    zone_state: IdealLoadsZoneState,
    demand: ZoneSysEnergyDemand,
    unit_available: bool,
    context: IdealLoadsSensibleLimitContext,
) -> IdealLoadsSensibleResult {
    calc_no_oa_no_limit_sensible_with_recirculation_context_compat(
        system,
        zone_state,
        zone_state,
        demand,
        unit_available,
        context,
    )
}

/// Calculates the no-outdoor-air, no-limit branch with explicit mixed-air
/// recirculation state for humidity-control diagnostics.
#[must_use]
pub fn calc_no_oa_no_limit_sensible_with_recirculation_context_compat(
    system: &IdealLoadsAirSystem,
    zone_state: IdealLoadsZoneState,
    recirculation_state: IdealLoadsZoneState,
    demand: ZoneSysEnergyDemand,
    unit_available: bool,
    context: IdealLoadsSensibleLimitContext,
) -> IdealLoadsSensibleResult {
    let cp_air_j_per_kg_k =
        energyplus_moist_air_specific_heat_j_per_kg_k(zone_state.air_humidity_ratio);
    let supply_humidity_ratio = recirculation_state.air_humidity_ratio;

    if !unit_available {
        return zero_result(
            IdealLoadsSensibleMode::Off,
            cp_air_j_per_kg_k,
            zone_state.air_temperature_c,
            supply_humidity_ratio,
        );
    }

    let heating_load_w = demand.remaining_output_req_to_heat_sp_w.max(0.0);
    let cooling_load_w = demand.remaining_output_req_to_cool_sp_w.abs().max(0.0);

    let heating_delta_t =
        system.maximum_heating_supply_air_temperature_c - zone_state.air_temperature_c;
    let heating_mass_flow_rate_kg_per_s =
        if heating_load_w > 0.0 && heating_delta_t > SMALL_TEMPERATURE_DIFFERENCE_C {
            heating_load_w / (cp_air_j_per_kg_k * heating_delta_t)
        } else {
            0.0
        };
    let heating_mass_flow_rate_kg_per_s = heating_mass_flow_rate_kg_per_s.max(
        humidistat_humidification_mass_flow_rate_kg_per_s(system, zone_state, demand),
    );

    let cooling_delta_t =
        zone_state.air_temperature_c - system.minimum_cooling_supply_air_temperature_c;
    let cooling_sensible_mass_flow_rate_kg_per_s =
        if cooling_load_w > 0.0 && cooling_delta_t > SMALL_TEMPERATURE_DIFFERENCE_C {
            cooling_load_w / (cp_air_j_per_kg_k * cooling_delta_t)
        } else {
            0.0
        };
    let cooling_mass_flow_rate_kg_per_s = cooling_sensible_mass_flow_rate_kg_per_s.max(
        humidistat_dehumidification_mass_flow_rate_kg_per_s(system, zone_state, demand),
    );

    if heating_mass_flow_rate_kg_per_s > 0.0
        && heating_mass_flow_rate_kg_per_s >= cooling_mass_flow_rate_kg_per_s
    {
        heating_result_with_limits(
            system,
            zone_state,
            recirculation_state,
            cp_air_j_per_kg_k,
            supply_humidity_ratio,
            heating_load_w,
            heating_mass_flow_rate_kg_per_s,
            demand,
            context,
        )
    } else if cooling_mass_flow_rate_kg_per_s > 0.0 {
        cooling_result_with_limits(
            system,
            zone_state,
            recirculation_state,
            cp_air_j_per_kg_k,
            supply_humidity_ratio,
            cooling_load_w,
            cooling_mass_flow_rate_kg_per_s,
            demand,
            context,
        )
    } else {
        zero_result(
            IdealLoadsSensibleMode::Deadband,
            cp_air_j_per_kg_k,
            zone_state.air_temperature_c,
            supply_humidity_ratio,
        )
    }
}

/// Calculates the no-outdoor-air sensible-only IdealLoads branch with numeric
/// flow and capacity limits.
///
/// `Autosize` limits are treated as unresolved and are therefore not applied by
/// this helper. Runtime sizing support must provide numeric values before a
/// finite-limit case can be promoted to conformance.
#[must_use]
pub fn calc_no_oa_sensible_with_limits_compat(
    system: &IdealLoadsAirSystem,
    zone_state: IdealLoadsZoneState,
    demand: ZoneSysEnergyDemand,
    unit_available: bool,
    limit_context: IdealLoadsSensibleLimitContext,
) -> IdealLoadsSensibleResult {
    calc_no_oa_sensible_with_limits_and_recirculation_compat(
        system,
        zone_state,
        zone_state,
        demand,
        unit_available,
        limit_context,
    )
}

/// Calculates the no-outdoor-air sensible-only IdealLoads branch with numeric
/// flow/capacity limits and explicit no-OA recirculation node state.
///
/// EnergyPlus capacity-limit adjustments use the mixed-air state, which is the
/// recirculation node state when outdoor air is inactive. The zone load and
/// final output-to-zone calculation still use the controlled zone node state.
#[must_use]
pub fn calc_no_oa_sensible_with_limits_and_recirculation_compat(
    system: &IdealLoadsAirSystem,
    zone_state: IdealLoadsZoneState,
    recirculation_state: IdealLoadsZoneState,
    demand: ZoneSysEnergyDemand,
    unit_available: bool,
    limit_context: IdealLoadsSensibleLimitContext,
) -> IdealLoadsSensibleResult {
    let cp_air_j_per_kg_k =
        energyplus_moist_air_specific_heat_j_per_kg_k(zone_state.air_humidity_ratio);
    let supply_humidity_ratio = recirculation_state.air_humidity_ratio;

    if !unit_available {
        return zero_result(
            IdealLoadsSensibleMode::Off,
            cp_air_j_per_kg_k,
            zone_state.air_temperature_c,
            supply_humidity_ratio,
        );
    }

    let heating_load_w = demand.remaining_output_req_to_heat_sp_w.max(0.0);
    let cooling_load_w = demand.remaining_output_req_to_cool_sp_w.abs().max(0.0);

    let heating_mass_flow_rate_kg_per_s = limited_heating_mass_flow_rate_kg_per_s(
        system,
        zone_state,
        heating_load_w,
        cp_air_j_per_kg_k,
        limit_context,
    );
    let mut heating_mass_flow_rate_kg_per_s = heating_mass_flow_rate_kg_per_s.max(
        humidistat_humidification_mass_flow_rate_kg_per_s(system, zone_state, demand),
    );
    if let Some(maximum_mass_flow_rate_kg_per_s) = flow_limit_kg_per_s(
        system.heating_limit,
        system.maximum_heating_air_flow_rate_m3_per_s,
        limit_context,
    ) && maximum_mass_flow_rate_kg_per_s > 0.0
    {
        heating_mass_flow_rate_kg_per_s =
            heating_mass_flow_rate_kg_per_s.min(maximum_mass_flow_rate_kg_per_s);
    }
    if heating_capacity_limit_is_zero(system) {
        heating_mass_flow_rate_kg_per_s = 0.0;
    }
    let cooling_sensible_mass_flow_rate_kg_per_s = limited_cooling_mass_flow_rate_kg_per_s(
        system,
        zone_state,
        cooling_load_w,
        cp_air_j_per_kg_k,
        limit_context,
    );
    let mut cooling_mass_flow_rate_kg_per_s = cooling_sensible_mass_flow_rate_kg_per_s.max(
        humidistat_dehumidification_mass_flow_rate_kg_per_s(system, zone_state, demand),
    );
    if let Some(maximum_mass_flow_rate_kg_per_s) = flow_limit_kg_per_s(
        system.cooling_limit,
        system.maximum_cooling_air_flow_rate_m3_per_s,
        limit_context,
    ) && maximum_mass_flow_rate_kg_per_s > 0.0
    {
        cooling_mass_flow_rate_kg_per_s =
            cooling_mass_flow_rate_kg_per_s.min(maximum_mass_flow_rate_kg_per_s);
    }
    if cooling_capacity_limit_is_zero(system) {
        cooling_mass_flow_rate_kg_per_s = 0.0;
    }

    if heating_mass_flow_rate_kg_per_s > 0.0
        && heating_mass_flow_rate_kg_per_s >= cooling_mass_flow_rate_kg_per_s
    {
        heating_result_with_limits(
            system,
            zone_state,
            recirculation_state,
            cp_air_j_per_kg_k,
            supply_humidity_ratio,
            heating_load_w,
            heating_mass_flow_rate_kg_per_s,
            demand,
            limit_context,
        )
    } else if cooling_mass_flow_rate_kg_per_s > 0.0 {
        cooling_result_with_limits(
            system,
            zone_state,
            recirculation_state,
            cp_air_j_per_kg_k,
            supply_humidity_ratio,
            cooling_load_w,
            cooling_mass_flow_rate_kg_per_s,
            demand,
            limit_context,
        )
    } else {
        zero_result(
            IdealLoadsSensibleMode::Deadband,
            cp_air_j_per_kg_k,
            zone_state.air_temperature_c,
            supply_humidity_ratio,
        )
    }
}

fn limited_heating_mass_flow_rate_kg_per_s(
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

fn limited_cooling_mass_flow_rate_kg_per_s(
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

fn humidistat_dehumidification_mass_flow_rate_kg_per_s(
    system: &IdealLoadsAirSystem,
    zone_state: IdealLoadsZoneState,
    demand: ZoneSysEnergyDemand,
) -> f64 {
    if system.dehumidification_control_type != DehumidificationControlType::Humidistat
        || cooling_capacity_limit_is_zero(system)
    {
        return 0.0;
    }

    let moisture_demand_kg_per_s = demand.remaining_output_req_to_dehumid_sp_kg_per_s;
    let delta_humidity_ratio =
        system.minimum_cooling_supply_air_humidity_ratio - zone_state.air_humidity_ratio;
    if delta_humidity_ratio < -SMALL_HUMIDITY_RATIO_DIFFERENCE && moisture_demand_kg_per_s < 0.0 {
        (moisture_demand_kg_per_s / delta_humidity_ratio).max(0.0)
    } else {
        0.0
    }
}

fn humidistat_humidification_mass_flow_rate_kg_per_s(
    system: &IdealLoadsAirSystem,
    zone_state: IdealLoadsZoneState,
    demand: ZoneSysEnergyDemand,
) -> f64 {
    if system.humidification_control_type != HumidificationControlType::Humidistat
        || heating_capacity_limit_is_zero(system)
    {
        return 0.0;
    }

    let moisture_demand_kg_per_s = demand.remaining_output_req_to_humid_sp_kg_per_s;
    let delta_humidity_ratio =
        system.maximum_heating_supply_air_humidity_ratio - zone_state.air_humidity_ratio;
    if delta_humidity_ratio > SMALL_HUMIDITY_RATIO_DIFFERENCE && moisture_demand_kg_per_s > 0.0 {
        (moisture_demand_kg_per_s / delta_humidity_ratio).max(0.0)
    } else {
        0.0
    }
}

fn heating_result_with_limits(
    system: &IdealLoadsAirSystem,
    zone_state: IdealLoadsZoneState,
    recirculation_state: IdealLoadsZoneState,
    cp_air_j_per_kg_k: f64,
    mixed_supply_humidity_ratio: f64,
    heating_load_w: f64,
    heating_mass_flow_rate_kg_per_s: f64,
    demand: ZoneSysEnergyDemand,
    context: IdealLoadsSensibleLimitContext,
) -> IdealLoadsSensibleResult {
    let mut supply_temperature_c = zone_state.air_temperature_c
        + heating_load_w / (cp_air_j_per_kg_k * heating_mass_flow_rate_kg_per_s);
    supply_temperature_c = supply_temperature_c
        .min(system.maximum_heating_supply_air_temperature_c)
        .max(recirculation_state.air_temperature_c);

    let cp_mixed_air_j_per_kg_k =
        energyplus_moist_air_specific_heat_j_per_kg_k(recirculation_state.air_humidity_ratio);
    let mut heating_coil_output_w = heating_mass_flow_rate_kg_per_s
        * cp_mixed_air_j_per_kg_k
        * (supply_temperature_c - recirculation_state.air_temperature_c).max(0.0);

    if let Some(maximum_heating_capacity_w) = capacity_limit_w(
        system.heating_limit,
        system.maximum_sensible_heating_capacity_w,
    ) && heating_coil_output_w > maximum_heating_capacity_w
    {
        heating_coil_output_w = maximum_heating_capacity_w;
        supply_temperature_c = recirculation_state.air_temperature_c
            + heating_coil_output_w / (cp_mixed_air_j_per_kg_k * heating_mass_flow_rate_kg_per_s);
    }

    let supply_humidity_ratio = heating_supply_humidity_ratio(
        system,
        zone_state,
        supply_temperature_c,
        mixed_supply_humidity_ratio,
        heating_mass_flow_rate_kg_per_s,
        demand,
        context,
    );
    let mixed_air_enthalpy_j_per_kg = moist_air_enthalpy_j_per_kg(
        recirculation_state.air_temperature_c,
        recirculation_state.air_humidity_ratio,
    );
    let supply_enthalpy_j_per_kg =
        moist_air_enthalpy_j_per_kg(supply_temperature_c, supply_humidity_ratio);
    let supply_air_sensible_heating_rate_w = heating_mass_flow_rate_kg_per_s
        * cp_mixed_air_j_per_kg_k
        * (supply_temperature_c - recirculation_state.air_temperature_c).max(0.0);
    let sensible_coil_load_w = supply_air_sensible_heating_rate_w;
    let latent_coil_load_w = if nearly_equal_humidity(
        supply_humidity_ratio,
        recirculation_state.air_humidity_ratio,
    ) {
        0.0
    } else {
        heating_mass_flow_rate_kg_per_s * (supply_enthalpy_j_per_kg - mixed_air_enthalpy_j_per_kg)
            - sensible_coil_load_w
    };
    let supply_air_latent_heating_rate_w = latent_coil_load_w.max(0.0);
    let supply_air_latent_cooling_rate_w = latent_coil_load_w.min(0.0).abs();

    let sensible_output_to_zone_w = heating_mass_flow_rate_kg_per_s
        * cp_air_j_per_kg_k
        * (supply_temperature_c - zone_state.air_temperature_c);
    let zone_sensible_heating_rate_w = sensible_output_to_zone_w.max(0.0);
    let zone_air_enthalpy_j_per_kg =
        moist_air_enthalpy_j_per_kg(zone_state.air_temperature_c, zone_state.air_humidity_ratio);
    let latent_output_to_zone_w =
        if nearly_equal_humidity(supply_humidity_ratio, zone_state.air_humidity_ratio) {
            0.0
        } else {
            heating_mass_flow_rate_kg_per_s
                * (supply_enthalpy_j_per_kg - zone_air_enthalpy_j_per_kg)
                - sensible_output_to_zone_w
        };
    let zone_latent_heating_rate_w = latent_output_to_zone_w.max(0.0);
    let zone_latent_cooling_rate_w = latent_output_to_zone_w.min(0.0).abs();

    IdealLoadsSensibleResult {
        mode: IdealLoadsSensibleMode::Heating,
        cp_air_j_per_kg_k,
        supply_temperature_c,
        supply_humidity_ratio,
        supply_enthalpy_j_per_kg,
        supply_mass_flow_rate_kg_per_s: heating_mass_flow_rate_kg_per_s,
        heating_mass_flow_rate_kg_per_s,
        cooling_mass_flow_rate_kg_per_s: 0.0,
        zone_total_heating_rate_w: zone_sensible_heating_rate_w + zone_latent_heating_rate_w,
        zone_total_cooling_rate_w: zone_latent_cooling_rate_w,
        zone_sensible_heating_rate_w,
        zone_sensible_cooling_rate_w: 0.0,
        zone_latent_heating_rate_w,
        zone_latent_cooling_rate_w,
        supply_air_sensible_heating_rate_w,
        supply_air_sensible_cooling_rate_w: 0.0,
        supply_air_latent_heating_rate_w,
        supply_air_latent_cooling_rate_w,
        supply_air_total_heating_rate_w: supply_air_sensible_heating_rate_w
            + supply_air_latent_heating_rate_w,
        supply_air_total_cooling_rate_w: supply_air_latent_cooling_rate_w,
    }
}

fn heating_supply_humidity_ratio(
    system: &IdealLoadsAirSystem,
    zone_state: IdealLoadsZoneState,
    supply_temperature_c: f64,
    mixed_supply_humidity_ratio: f64,
    supply_mass_flow_rate_kg_per_s: f64,
    demand: ZoneSysEnergyDemand,
    context: IdealLoadsSensibleLimitContext,
) -> f64 {
    let supply_humidity_ratio = match system.humidification_control_type {
        HumidificationControlType::Humidistat if supply_mass_flow_rate_kg_per_s > 0.0 => {
            let supply_humidity_ratio_for_humidification =
                (demand.remaining_output_req_to_humid_sp_kg_per_s / supply_mass_flow_rate_kg_per_s
                    + zone_state.air_humidity_ratio)
                    .min(system.maximum_heating_supply_air_humidity_ratio);
            mixed_supply_humidity_ratio.max(supply_humidity_ratio_for_humidification)
        }
        HumidificationControlType::ConstantSupplyHumidityRatio
            if supply_mass_flow_rate_kg_per_s > 0.0 =>
        {
            system.maximum_heating_supply_air_humidity_ratio
        }
        _ => mixed_supply_humidity_ratio,
    };
    let saturation_humidity_ratio = energyplus_psychrometric_humidity_ratio_from_rh(
        supply_temperature_c,
        1.0,
        context.barometric_pressure_pa,
    )
    .unwrap_or(f64::INFINITY);
    supply_humidity_ratio.min(saturation_humidity_ratio)
}

fn cooling_result_with_limits(
    system: &IdealLoadsAirSystem,
    zone_state: IdealLoadsZoneState,
    recirculation_state: IdealLoadsZoneState,
    cp_air_j_per_kg_k: f64,
    supply_humidity_ratio: f64,
    cooling_load_w: f64,
    cooling_mass_flow_rate_kg_per_s: f64,
    demand: ZoneSysEnergyDemand,
    context: IdealLoadsSensibleLimitContext,
) -> IdealLoadsSensibleResult {
    let mut supply_temperature_c = zone_state.air_temperature_c
        - cooling_load_w / (cp_air_j_per_kg_k * cooling_mass_flow_rate_kg_per_s);
    supply_temperature_c = supply_temperature_c
        .max(system.minimum_cooling_supply_air_temperature_c)
        .min(recirculation_state.air_temperature_c);

    let cp_mixed_air_j_per_kg_k =
        energyplus_moist_air_specific_heat_j_per_kg_k(recirculation_state.air_humidity_ratio);
    let mut cooling_coil_output_w = cooling_mass_flow_rate_kg_per_s
        * cp_mixed_air_j_per_kg_k
        * (recirculation_state.air_temperature_c - supply_temperature_c).max(0.0);

    if let Some(maximum_cooling_capacity_w) = capacity_limit_w(
        system.cooling_limit,
        system.maximum_total_cooling_capacity_w,
    ) && cooling_coil_output_w >= maximum_cooling_capacity_w
    {
        cooling_coil_output_w = maximum_cooling_capacity_w;
        supply_temperature_c = recirculation_state.air_temperature_c
            - cooling_coil_output_w / (cooling_mass_flow_rate_kg_per_s * cp_mixed_air_j_per_kg_k);
    }

    cooling_result_from_states(
        system,
        zone_state,
        recirculation_state,
        cp_air_j_per_kg_k,
        supply_temperature_c,
        supply_humidity_ratio,
        cooling_mass_flow_rate_kg_per_s,
        demand,
        context,
    )
}

fn cooling_result_from_states(
    system: &IdealLoadsAirSystem,
    zone_state: IdealLoadsZoneState,
    mixed_air_state: IdealLoadsZoneState,
    cp_air_j_per_kg_k: f64,
    supply_temperature_c: f64,
    mixed_supply_humidity_ratio: f64,
    cooling_mass_flow_rate_kg_per_s: f64,
    demand: ZoneSysEnergyDemand,
    context: IdealLoadsSensibleLimitContext,
) -> IdealLoadsSensibleResult {
    let mixed_air_enthalpy_j_per_kg = moist_air_enthalpy_j_per_kg(
        mixed_air_state.air_temperature_c,
        mixed_air_state.air_humidity_ratio,
    );
    let supply_air_sensible_cooling_rate_w = cooling_mass_flow_rate_kg_per_s
        * energyplus_moist_air_specific_heat_j_per_kg_k(mixed_air_state.air_humidity_ratio)
        * (mixed_air_state.air_temperature_c - supply_temperature_c).max(0.0);
    let supply_humidity_ratio = cooling_supply_humidity_ratio(
        system,
        zone_state,
        mixed_air_state,
        supply_temperature_c,
        mixed_supply_humidity_ratio,
        cooling_mass_flow_rate_kg_per_s,
        demand,
        supply_air_sensible_cooling_rate_w,
        mixed_air_enthalpy_j_per_kg,
        context,
    );
    let supply_enthalpy_j_per_kg =
        moist_air_enthalpy_j_per_kg(supply_temperature_c, supply_humidity_ratio);
    let sensible_coil_load_w = -supply_air_sensible_cooling_rate_w;
    let latent_coil_load_w =
        if nearly_equal_humidity(supply_humidity_ratio, mixed_air_state.air_humidity_ratio) {
            0.0
        } else {
            cooling_mass_flow_rate_kg_per_s
                * (supply_enthalpy_j_per_kg - mixed_air_enthalpy_j_per_kg)
                - sensible_coil_load_w
        };
    let supply_air_latent_heating_rate_w = latent_coil_load_w.max(0.0);
    let supply_air_latent_cooling_rate_w = latent_coil_load_w.min(0.0).abs();

    let sensible_output_to_zone_w = cooling_mass_flow_rate_kg_per_s
        * cp_air_j_per_kg_k
        * (supply_temperature_c - zone_state.air_temperature_c);
    let zone_sensible_cooling_rate_w = sensible_output_to_zone_w.min(0.0).abs();
    let zone_air_enthalpy_j_per_kg =
        moist_air_enthalpy_j_per_kg(zone_state.air_temperature_c, zone_state.air_humidity_ratio);
    let latent_output_to_zone_w =
        if nearly_equal_humidity(supply_humidity_ratio, zone_state.air_humidity_ratio) {
            0.0
        } else {
            cooling_mass_flow_rate_kg_per_s
                * (supply_enthalpy_j_per_kg - zone_air_enthalpy_j_per_kg)
                - sensible_output_to_zone_w
        };
    let zone_latent_heating_rate_w = latent_output_to_zone_w.max(0.0);
    let zone_latent_cooling_rate_w = latent_output_to_zone_w.min(0.0).abs();

    let zone_sensible_heating_rate_w = 0.0;
    let supply_air_sensible_heating_rate_w = 0.0;
    IdealLoadsSensibleResult {
        mode: IdealLoadsSensibleMode::Cooling,
        cp_air_j_per_kg_k,
        supply_temperature_c,
        supply_humidity_ratio,
        supply_enthalpy_j_per_kg,
        supply_mass_flow_rate_kg_per_s: cooling_mass_flow_rate_kg_per_s,
        heating_mass_flow_rate_kg_per_s: 0.0,
        cooling_mass_flow_rate_kg_per_s,
        zone_total_heating_rate_w: zone_sensible_heating_rate_w + zone_latent_heating_rate_w,
        zone_total_cooling_rate_w: zone_sensible_cooling_rate_w + zone_latent_cooling_rate_w,
        zone_sensible_heating_rate_w,
        zone_sensible_cooling_rate_w,
        zone_latent_heating_rate_w,
        zone_latent_cooling_rate_w,
        supply_air_sensible_heating_rate_w,
        supply_air_sensible_cooling_rate_w,
        supply_air_latent_heating_rate_w,
        supply_air_latent_cooling_rate_w,
        supply_air_total_heating_rate_w: supply_air_sensible_heating_rate_w
            + supply_air_latent_heating_rate_w,
        supply_air_total_cooling_rate_w: supply_air_sensible_cooling_rate_w
            + supply_air_latent_cooling_rate_w,
    }
}

fn cooling_supply_humidity_ratio(
    system: &IdealLoadsAirSystem,
    zone_state: IdealLoadsZoneState,
    mixed_air_state: IdealLoadsZoneState,
    supply_temperature_c: f64,
    mixed_supply_humidity_ratio: f64,
    supply_mass_flow_rate_kg_per_s: f64,
    demand: ZoneSysEnergyDemand,
    supply_air_sensible_cooling_rate_w: f64,
    mixed_air_enthalpy_j_per_kg: f64,
    context: IdealLoadsSensibleLimitContext,
) -> f64 {
    let supply_humidity_ratio = match system.dehumidification_control_type {
        DehumidificationControlType::ConstantSensibleHeatRatio
            if supply_mass_flow_rate_kg_per_s > 0.0 && system.cooling_sensible_heat_ratio > 0.0 =>
        {
            let cooling_total_output_w =
                supply_air_sensible_cooling_rate_w / system.cooling_sensible_heat_ratio;
            let supply_enthalpy_j_per_kg = (mixed_air_enthalpy_j_per_kg
                - cooling_total_output_w / supply_mass_flow_rate_kg_per_s)
                .max(moist_air_enthalpy_j_per_kg(
                    supply_temperature_c,
                    MINIMUM_HUMIDITY_RATIO,
                ));
            let humidity_from_enthalpy = humidity_ratio_from_enthalpy_and_dry_bulb(
                supply_enthalpy_j_per_kg,
                supply_temperature_c,
            )
            .max(MINIMUM_HUMIDITY_RATIO);
            mixed_supply_humidity_ratio
                .min(humidity_from_enthalpy)
                .max(system.minimum_cooling_supply_air_humidity_ratio)
                .min(mixed_air_state.air_humidity_ratio)
        }
        DehumidificationControlType::ConstantSupplyHumidityRatio
            if supply_mass_flow_rate_kg_per_s > 0.0 =>
        {
            system
                .minimum_cooling_supply_air_humidity_ratio
                .max(MINIMUM_HUMIDITY_RATIO)
        }
        DehumidificationControlType::Humidistat if supply_mass_flow_rate_kg_per_s > 0.0 => {
            let supply_humidity_ratio_for_dehumidification = (demand
                .remaining_output_req_to_dehumid_sp_kg_per_s
                / supply_mass_flow_rate_kg_per_s
                + zone_state.air_humidity_ratio)
                .max(system.minimum_cooling_supply_air_humidity_ratio);
            mixed_supply_humidity_ratio.min(supply_humidity_ratio_for_dehumidification)
        }
        _ => mixed_supply_humidity_ratio,
    };
    let saturation_humidity_ratio = energyplus_psychrometric_humidity_ratio_from_rh(
        supply_temperature_c,
        1.0,
        context.barometric_pressure_pa,
    )
    .unwrap_or(f64::INFINITY);
    supply_humidity_ratio.min(saturation_humidity_ratio)
}

fn zero_result(
    mode: IdealLoadsSensibleMode,
    cp_air_j_per_kg_k: f64,
    supply_temperature_c: f64,
    supply_humidity_ratio: f64,
) -> IdealLoadsSensibleResult {
    IdealLoadsSensibleResult {
        mode,
        cp_air_j_per_kg_k,
        supply_temperature_c,
        supply_humidity_ratio,
        supply_enthalpy_j_per_kg: moist_air_enthalpy_j_per_kg(
            supply_temperature_c,
            supply_humidity_ratio,
        ),
        supply_mass_flow_rate_kg_per_s: 0.0,
        heating_mass_flow_rate_kg_per_s: 0.0,
        cooling_mass_flow_rate_kg_per_s: 0.0,
        zone_total_heating_rate_w: 0.0,
        zone_total_cooling_rate_w: 0.0,
        zone_sensible_heating_rate_w: 0.0,
        zone_sensible_cooling_rate_w: 0.0,
        zone_latent_heating_rate_w: 0.0,
        zone_latent_cooling_rate_w: 0.0,
        supply_air_sensible_heating_rate_w: 0.0,
        supply_air_sensible_cooling_rate_w: 0.0,
        supply_air_latent_heating_rate_w: 0.0,
        supply_air_latent_cooling_rate_w: 0.0,
        supply_air_total_heating_rate_w: 0.0,
        supply_air_total_cooling_rate_w: 0.0,
    }
}
