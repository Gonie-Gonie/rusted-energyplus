//! No-OA IdealLoads sensible load calculation.

use crate::{
    energyplus_moist_air_specific_heat_j_per_kg_k,
    zone_equipment::{ZoneSensibleDemandInputKind, ZoneSysEnergyDemand},
};
use ep_model::{HumidificationControlType, IdealLoadsAirSystem};

use super::humidity::{
    cooling_supply_humidity_ratio, heating_supply_humidity_ratio,
    humidistat_dehumidification_mass_flow_rate_kg_per_s,
    humidistat_humidification_mass_flow_rate_kg_per_s,
};
use super::limits::{
    IdealLoadsSensibleLimitContext, capacity_limit_w, cooling_capacity_limit_is_zero,
    flow_limit_kg_per_s, heating_capacity_limit_is_zero,
};
use super::mass_flow::{
    SMALL_TEMPERATURE_DIFFERENCE_C, limited_cooling_mass_flow_rate_kg_per_s,
    limited_heating_mass_flow_rate_kg_per_s,
};
use super::psychrometrics::{moist_air_enthalpy_j_per_kg, nearly_equal_humidity};
use super::types::{IdealLoadsSensibleMode, IdealLoadsSensibleResult, IdealLoadsZoneState};

// The source threshold lane preserves the inclusive no-OA `0 >= QCoolSP`
// priority. Oracle/default active-split fixtures retain zero as inactive until
// their upstream injection is removed.
fn source_selects_no_oa_cooling(demand: ZoneSysEnergyDemand) -> bool {
    match demand.sensible_input_kind {
        ZoneSensibleDemandInputKind::ActiveLoadSplitCompatibility => {
            0.0 > demand.remaining_output_req_to_cool_sp_w
        }
        ZoneSensibleDemandInputKind::SourceSetpointThresholds => {
            0.0 >= demand.remaining_output_req_to_cool_sp_w
        }
    }
}

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
    let cooling_threshold_selected = source_selects_no_oa_cooling(demand);
    let cooling_load_w = (-demand.remaining_output_req_to_cool_sp_w).max(0.0);

    let heating_delta_t =
        system.maximum_heating_supply_air_temperature_c - zone_state.air_temperature_c;
    let heating_mass_flow_rate_kg_per_s =
        if heating_load_w > 0.0 && heating_delta_t > SMALL_TEMPERATURE_DIFFERENCE_C {
            heating_load_w / (cp_air_j_per_kg_k * heating_delta_t)
        } else {
            0.0
        };
    let heating_mass_flow_rate_kg_per_s = heating_mass_flow_rate_kg_per_s
        .max(humidistat_humidification_mass_flow_rate_kg_per_s(
            system, zone_state, demand, context,
        ))
        .max(heating_section_dehumidification_mass_flow_rate_kg_per_s(
            system, zone_state, demand, context,
        ));

    let cooling_delta_t =
        zone_state.air_temperature_c - system.minimum_cooling_supply_air_temperature_c;
    let cooling_sensible_mass_flow_rate_kg_per_s =
        if cooling_load_w > 0.0 && cooling_delta_t > SMALL_TEMPERATURE_DIFFERENCE_C {
            cooling_load_w / (cp_air_j_per_kg_k * cooling_delta_t)
        } else {
            0.0
        };
    let cooling_mass_flow_rate_kg_per_s = cooling_sensible_mass_flow_rate_kg_per_s.max(
        humidistat_dehumidification_mass_flow_rate_kg_per_s(system, zone_state, demand, context),
    );

    if cooling_threshold_selected {
        if cooling_mass_flow_rate_kg_per_s > 0.0 {
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
                IdealLoadsSensibleMode::Cooling,
                cp_air_j_per_kg_k,
                zone_state.air_temperature_c,
                supply_humidity_ratio,
            )
        }
    } else if heating_load_w > 0.0 && heating_mass_flow_rate_kg_per_s > 0.0 {
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
    } else if heating_mass_flow_rate_kg_per_s > 0.0 {
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
    let cooling_threshold_selected = source_selects_no_oa_cooling(demand);
    let cooling_load_w = (-demand.remaining_output_req_to_cool_sp_w).max(0.0);

    let heating_mass_flow_rate_kg_per_s = limited_heating_mass_flow_rate_kg_per_s(
        system,
        zone_state,
        heating_load_w,
        cp_air_j_per_kg_k,
        limit_context,
    );
    let sized_limits = limit_context.sized_limits_or_system(system);
    let mut heating_mass_flow_rate_kg_per_s = heating_mass_flow_rate_kg_per_s
        .max(humidistat_humidification_mass_flow_rate_kg_per_s(
            system,
            zone_state,
            demand,
            limit_context,
        ))
        .max(heating_section_dehumidification_mass_flow_rate_kg_per_s(
            system,
            zone_state,
            demand,
            limit_context,
        ));
    if let Some(maximum_mass_flow_rate_kg_per_s) = flow_limit_kg_per_s(
        system.heating_limit,
        sized_limits.maximum_heating_air_flow_rate_m3_per_s,
        limit_context.initialized_heating_air_mass_flow_limit_kg_per_s,
        limit_context,
    ) && maximum_mass_flow_rate_kg_per_s > 0.0
    {
        heating_mass_flow_rate_kg_per_s =
            heating_mass_flow_rate_kg_per_s.min(maximum_mass_flow_rate_kg_per_s);
    }
    if heating_capacity_limit_is_zero(system, limit_context) {
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
        humidistat_dehumidification_mass_flow_rate_kg_per_s(
            system,
            zone_state,
            demand,
            limit_context,
        ),
    );
    if let Some(maximum_mass_flow_rate_kg_per_s) = flow_limit_kg_per_s(
        system.cooling_limit,
        sized_limits.maximum_cooling_air_flow_rate_m3_per_s,
        limit_context.initialized_cooling_air_mass_flow_limit_kg_per_s,
        limit_context,
    ) && maximum_mass_flow_rate_kg_per_s > 0.0
    {
        cooling_mass_flow_rate_kg_per_s =
            cooling_mass_flow_rate_kg_per_s.min(maximum_mass_flow_rate_kg_per_s);
    }
    if cooling_capacity_limit_is_zero(system, limit_context) {
        cooling_mass_flow_rate_kg_per_s = 0.0;
    }

    if cooling_threshold_selected {
        if cooling_mass_flow_rate_kg_per_s > 0.0 {
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
                IdealLoadsSensibleMode::Cooling,
                cp_air_j_per_kg_k,
                zone_state.air_temperature_c,
                supply_humidity_ratio,
            )
        }
    } else if heating_load_w > 0.0 && heating_mass_flow_rate_kg_per_s > 0.0 {
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
    } else if heating_mass_flow_rate_kg_per_s > 0.0 {
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
    } else {
        zero_result(
            IdealLoadsSensibleMode::Deadband,
            cp_air_j_per_kg_k,
            zone_state.air_temperature_c,
            supply_humidity_ratio,
        )
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
    let sized_limits = context.sized_limits_or_system(system);
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
        sized_limits.maximum_sensible_heating_capacity_w,
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
    let (sensible_coil_load_w, latent_coil_load_w) = coil_loads_from_states(
        heating_mass_flow_rate_kg_per_s,
        supply_temperature_c,
        supply_humidity_ratio,
        recirculation_state,
        mixed_air_enthalpy_j_per_kg,
        supply_enthalpy_j_per_kg,
    );
    let supply_air_sensible_heating_rate_w = sensible_coil_load_w.max(0.0);
    let supply_air_sensible_cooling_rate_w = sensible_coil_load_w.min(0.0).abs();
    let supply_air_latent_heating_rate_w = latent_coil_load_w.max(0.0);
    let supply_air_latent_cooling_rate_w = latent_coil_load_w.min(0.0).abs();

    let sensible_output_to_zone_w = heating_mass_flow_rate_kg_per_s
        * cp_air_j_per_kg_k
        * (supply_temperature_c - zone_state.air_temperature_c);
    let zone_sensible_heating_rate_w = sensible_output_to_zone_w.max(0.0);
    let zone_sensible_cooling_rate_w = sensible_output_to_zone_w.min(0.0).abs();
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

fn heating_section_dehumidification_mass_flow_rate_kg_per_s(
    system: &IdealLoadsAirSystem,
    zone_state: IdealLoadsZoneState,
    demand: ZoneSysEnergyDemand,
    context: IdealLoadsSensibleLimitContext,
) -> f64 {
    if matches!(
        system.humidification_control_type,
        HumidificationControlType::Humidistat | HumidificationControlType::None
    ) {
        humidistat_dehumidification_mass_flow_rate_kg_per_s(system, zone_state, demand, context)
    } else {
        0.0
    }
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
    let sized_limits = context.sized_limits_or_system(system);
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
        sized_limits.maximum_total_cooling_capacity_w,
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
    let (sensible_coil_load_w, latent_coil_load_w) = coil_loads_from_states(
        cooling_mass_flow_rate_kg_per_s,
        supply_temperature_c,
        supply_humidity_ratio,
        mixed_air_state,
        mixed_air_enthalpy_j_per_kg,
        supply_enthalpy_j_per_kg,
    );
    let supply_air_sensible_heating_rate_w = sensible_coil_load_w.max(0.0);
    let supply_air_sensible_cooling_rate_w = sensible_coil_load_w.min(0.0).abs();
    let supply_air_latent_heating_rate_w = latent_coil_load_w.max(0.0);
    let supply_air_latent_cooling_rate_w = latent_coil_load_w.min(0.0).abs();

    let sensible_output_to_zone_w = cooling_mass_flow_rate_kg_per_s
        * cp_air_j_per_kg_k
        * (supply_temperature_c - zone_state.air_temperature_c);
    let zone_sensible_cooling_rate_w = sensible_output_to_zone_w.min(0.0).abs();
    let zone_sensible_heating_rate_w = sensible_output_to_zone_w.max(0.0);
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

fn coil_loads_from_states(
    supply_mass_flow_rate_kg_per_s: f64,
    supply_temperature_c: f64,
    supply_humidity_ratio: f64,
    mixed_air_state: IdealLoadsZoneState,
    mixed_air_enthalpy_j_per_kg: f64,
    supply_enthalpy_j_per_kg: f64,
) -> (f64, f64) {
    let humidity_unchanged =
        nearly_equal_humidity(supply_humidity_ratio, mixed_air_state.air_humidity_ratio);
    let temperature_unchanged =
        (supply_temperature_c - mixed_air_state.air_temperature_c).abs() <= f64::EPSILON;
    if humidity_unchanged && temperature_unchanged {
        (0.0, 0.0)
    } else if humidity_unchanged {
        (
            supply_mass_flow_rate_kg_per_s
                * (supply_enthalpy_j_per_kg - mixed_air_enthalpy_j_per_kg),
            0.0,
        )
    } else {
        let sensible_coil_load_w = supply_mass_flow_rate_kg_per_s
            * energyplus_moist_air_specific_heat_j_per_kg_k(mixed_air_state.air_humidity_ratio)
            * (supply_temperature_c - mixed_air_state.air_temperature_c);
        let latent_coil_load_w = supply_mass_flow_rate_kg_per_s
            * (supply_enthalpy_j_per_kg - mixed_air_enthalpy_j_per_kg)
            - sensible_coil_load_w;
        (sensible_coil_load_w, latent_coil_load_w)
    }
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
