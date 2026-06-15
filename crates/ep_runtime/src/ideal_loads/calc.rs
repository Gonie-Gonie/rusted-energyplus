//! No-OA IdealLoads sensible load calculation.

use crate::{
    energyplus_moist_air_density_kg_per_m3, energyplus_moist_air_specific_heat_j_per_kg_k,
    zone_equipment::ZoneSysEnergyDemand,
};
use ep_model::{AutosizeOrNumber, IdealLoadsAirSystem, IdealLoadsLimit};

const SMALL_TEMPERATURE_DIFFERENCE_C: f64 = 0.001;
const DEFAULT_STANDARD_AIR_DENSITY_KG_PER_M3: f64 = 1.2;
const STANDARD_PRESSURE_SEA_LEVEL_PA: f64 = 101_325.0;
const ENERGYPLUS_STANDARD_DRY_BULB_C: f64 = 20.0;
const ENERGYPLUS_STANDARD_HUMIDITY_RATIO: f64 = 0.0;

/// Operating mode selected by the first IdealLoads sensible subset.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IdealLoadsSensibleMode {
    /// Unit is unavailable.
    Off,
    /// No sensible load is active.
    Deadband,
    /// Cooling branch is active.
    Cooling,
    /// Heating branch is active.
    Heating,
}

/// Zone-side inputs to `CalcPurchAirLoads`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IdealLoadsZoneState {
    /// Zone air temperature in C.
    pub air_temperature_c: f64,
    /// Zone air humidity ratio in kgWater/kgDryAir.
    pub air_humidity_ratio: f64,
}

/// Runtime context needed for numeric IdealLoads flow limits.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IdealLoadsSensibleLimitContext {
    /// Standard air density in kg/m3 used to convert volumetric limits to mass limits.
    pub standard_air_density_kg_per_m3: f64,
}

impl Default for IdealLoadsSensibleLimitContext {
    fn default() -> Self {
        Self {
            standard_air_density_kg_per_m3: DEFAULT_STANDARD_AIR_DENSITY_KG_PER_M3,
        }
    }
}

impl IdealLoadsSensibleLimitContext {
    /// Builds the limit context from EnergyPlus `StdRhoAir` source-order inputs.
    #[must_use]
    pub fn from_site_elevation_m(elevation_m: f64) -> Option<Self> {
        energyplus_standard_air_density_kg_per_m3(elevation_m).map(
            |standard_air_density_kg_per_m3| Self {
                standard_air_density_kg_per_m3,
            },
        )
    }
}

/// Result of the no-OA sensible calculation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IdealLoadsSensibleResult {
    /// Selected operating mode.
    pub mode: IdealLoadsSensibleMode,
    /// EnergyPlus `PsyCpAirFnW` value in J/kg-K.
    pub cp_air_j_per_kg_k: f64,
    /// Final supply temperature in C.
    pub supply_temperature_c: f64,
    /// Final supply humidity ratio in kgWater/kgDryAir.
    pub supply_humidity_ratio: f64,
    /// Final supply enthalpy in J/kg.
    pub supply_enthalpy_j_per_kg: f64,
    /// Final supply mass flow in kg/s.
    pub supply_mass_flow_rate_kg_per_s: f64,
    /// Sensible heating flow contribution in kg/s.
    pub heating_mass_flow_rate_kg_per_s: f64,
    /// Sensible cooling flow contribution in kg/s.
    pub cooling_mass_flow_rate_kg_per_s: f64,
    /// Zone total heating rate in W.
    pub zone_total_heating_rate_w: f64,
    /// Zone total cooling rate in W.
    pub zone_total_cooling_rate_w: f64,
    /// Zone sensible heating rate in W.
    pub zone_sensible_heating_rate_w: f64,
    /// Zone sensible cooling rate in W.
    pub zone_sensible_cooling_rate_w: f64,
    /// Supply air total heating rate in W.
    pub supply_air_total_heating_rate_w: f64,
    /// Supply air total cooling rate in W.
    pub supply_air_total_cooling_rate_w: f64,
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
    let cp_air_j_per_kg_k =
        energyplus_moist_air_specific_heat_j_per_kg_k(zone_state.air_humidity_ratio);
    let supply_humidity_ratio = zone_state.air_humidity_ratio;

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

    let cooling_delta_t =
        zone_state.air_temperature_c - system.minimum_cooling_supply_air_temperature_c;
    let cooling_mass_flow_rate_kg_per_s =
        if cooling_load_w > 0.0 && cooling_delta_t > SMALL_TEMPERATURE_DIFFERENCE_C {
            cooling_load_w / (cp_air_j_per_kg_k * cooling_delta_t)
        } else {
            0.0
        };

    if heating_mass_flow_rate_kg_per_s > 0.0
        && heating_mass_flow_rate_kg_per_s >= cooling_mass_flow_rate_kg_per_s
    {
        let supply_temperature_c = system.maximum_heating_supply_air_temperature_c;
        let supply_enthalpy_j_per_kg =
            moist_air_enthalpy_j_per_kg(supply_temperature_c, supply_humidity_ratio);
        IdealLoadsSensibleResult {
            mode: IdealLoadsSensibleMode::Heating,
            cp_air_j_per_kg_k,
            supply_temperature_c,
            supply_humidity_ratio,
            supply_enthalpy_j_per_kg,
            supply_mass_flow_rate_kg_per_s: heating_mass_flow_rate_kg_per_s,
            heating_mass_flow_rate_kg_per_s,
            cooling_mass_flow_rate_kg_per_s: 0.0,
            zone_total_heating_rate_w: heating_load_w,
            zone_total_cooling_rate_w: 0.0,
            zone_sensible_heating_rate_w: heating_load_w,
            zone_sensible_cooling_rate_w: 0.0,
            supply_air_total_heating_rate_w: heating_load_w,
            supply_air_total_cooling_rate_w: 0.0,
        }
    } else if cooling_mass_flow_rate_kg_per_s > 0.0 {
        let supply_temperature_c = system.minimum_cooling_supply_air_temperature_c;
        let supply_enthalpy_j_per_kg =
            moist_air_enthalpy_j_per_kg(supply_temperature_c, supply_humidity_ratio);
        IdealLoadsSensibleResult {
            mode: IdealLoadsSensibleMode::Cooling,
            cp_air_j_per_kg_k,
            supply_temperature_c,
            supply_humidity_ratio,
            supply_enthalpy_j_per_kg,
            supply_mass_flow_rate_kg_per_s: cooling_mass_flow_rate_kg_per_s,
            heating_mass_flow_rate_kg_per_s: 0.0,
            cooling_mass_flow_rate_kg_per_s,
            zone_total_heating_rate_w: 0.0,
            zone_total_cooling_rate_w: cooling_load_w,
            zone_sensible_heating_rate_w: 0.0,
            zone_sensible_cooling_rate_w: cooling_load_w,
            supply_air_total_heating_rate_w: 0.0,
            supply_air_total_cooling_rate_w: cooling_load_w,
        }
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
    let cooling_mass_flow_rate_kg_per_s = limited_cooling_mass_flow_rate_kg_per_s(
        system,
        zone_state,
        cooling_load_w,
        cp_air_j_per_kg_k,
        limit_context,
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

/// EnergyPlus `PsyHFnTdbW`-style moist-air enthalpy in J/kg.
#[must_use]
pub fn moist_air_enthalpy_j_per_kg(dry_bulb_c: f64, humidity_ratio: f64) -> f64 {
    1000.0 * (1.006 * dry_bulb_c + humidity_ratio * (2501.0 + 1.86 * dry_bulb_c))
}

/// Returns EnergyPlus `StdRhoAir` from site elevation.
#[must_use]
pub fn energyplus_standard_air_density_kg_per_m3(elevation_m: f64) -> Option<f64> {
    if !elevation_m.is_finite() {
        return None;
    }
    let base = 1.0 - 2.255_77e-05 * elevation_m;
    if base <= 0.0 {
        return None;
    }
    let standard_barometric_pressure_pa = STANDARD_PRESSURE_SEA_LEVEL_PA * base.powf(5.2559);
    energyplus_moist_air_density_kg_per_m3(
        standard_barometric_pressure_pa,
        ENERGYPLUS_STANDARD_DRY_BULB_C,
        ENERGYPLUS_STANDARD_HUMIDITY_RATIO,
    )
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

fn heating_result_with_limits(
    system: &IdealLoadsAirSystem,
    zone_state: IdealLoadsZoneState,
    recirculation_state: IdealLoadsZoneState,
    cp_air_j_per_kg_k: f64,
    supply_humidity_ratio: f64,
    heating_load_w: f64,
    heating_mass_flow_rate_kg_per_s: f64,
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

    let heating_output_to_zone_w = heating_mass_flow_rate_kg_per_s
        * cp_air_j_per_kg_k
        * (supply_temperature_c - zone_state.air_temperature_c).max(0.0);

    let supply_enthalpy_j_per_kg =
        moist_air_enthalpy_j_per_kg(supply_temperature_c, supply_humidity_ratio);

    IdealLoadsSensibleResult {
        mode: IdealLoadsSensibleMode::Heating,
        cp_air_j_per_kg_k,
        supply_temperature_c,
        supply_humidity_ratio,
        supply_enthalpy_j_per_kg,
        supply_mass_flow_rate_kg_per_s: heating_mass_flow_rate_kg_per_s,
        heating_mass_flow_rate_kg_per_s,
        cooling_mass_flow_rate_kg_per_s: 0.0,
        zone_total_heating_rate_w: heating_output_to_zone_w,
        zone_total_cooling_rate_w: 0.0,
        zone_sensible_heating_rate_w: heating_output_to_zone_w,
        zone_sensible_cooling_rate_w: 0.0,
        supply_air_total_heating_rate_w: heating_output_to_zone_w,
        supply_air_total_cooling_rate_w: 0.0,
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

    let cooling_output_to_zone_w = cooling_mass_flow_rate_kg_per_s
        * cp_air_j_per_kg_k
        * (zone_state.air_temperature_c - supply_temperature_c).max(0.0);
    let supply_enthalpy_j_per_kg =
        moist_air_enthalpy_j_per_kg(supply_temperature_c, supply_humidity_ratio);

    IdealLoadsSensibleResult {
        mode: IdealLoadsSensibleMode::Cooling,
        cp_air_j_per_kg_k,
        supply_temperature_c,
        supply_humidity_ratio,
        supply_enthalpy_j_per_kg,
        supply_mass_flow_rate_kg_per_s: cooling_mass_flow_rate_kg_per_s,
        heating_mass_flow_rate_kg_per_s: 0.0,
        cooling_mass_flow_rate_kg_per_s,
        zone_total_heating_rate_w: 0.0,
        zone_total_cooling_rate_w: cooling_output_to_zone_w,
        zone_sensible_heating_rate_w: 0.0,
        zone_sensible_cooling_rate_w: cooling_output_to_zone_w,
        supply_air_total_heating_rate_w: 0.0,
        supply_air_total_cooling_rate_w: cooling_output_to_zone_w,
    }
}

fn flow_limit_kg_per_s(
    limit: IdealLoadsLimit,
    flow_limit_m3_per_s: Option<AutosizeOrNumber>,
    limit_context: IdealLoadsSensibleLimitContext,
) -> Option<f64> {
    if !limit_includes_flow_rate(limit) {
        return None;
    }

    numeric_autosize_value(flow_limit_m3_per_s).map(|flow_limit_m3_per_s| {
        flow_limit_m3_per_s * limit_context.standard_air_density_kg_per_m3
    })
}

fn capacity_limit_w(
    limit: IdealLoadsLimit,
    capacity_limit_w: Option<AutosizeOrNumber>,
) -> Option<f64> {
    if !limit_includes_capacity(limit) {
        return None;
    }

    numeric_autosize_value(capacity_limit_w)
}

fn numeric_autosize_value(value: Option<AutosizeOrNumber>) -> Option<f64> {
    match value {
        Some(AutosizeOrNumber::Value(value)) => Some(value),
        Some(AutosizeOrNumber::Autosize) | None => None,
    }
}

fn limit_includes_flow_rate(limit: IdealLoadsLimit) -> bool {
    matches!(
        limit,
        IdealLoadsLimit::LimitFlowRate | IdealLoadsLimit::LimitFlowRateAndCapacity
    )
}

fn limit_includes_capacity(limit: IdealLoadsLimit) -> bool {
    matches!(
        limit,
        IdealLoadsLimit::LimitCapacity | IdealLoadsLimit::LimitFlowRateAndCapacity
    )
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
        supply_air_total_heating_rate_w: 0.0,
        supply_air_total_cooling_rate_w: 0.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zone_equipment::ZoneSysEnergyDemand;
    use ep_model::{
        AutosizeOrNumber, DehumidificationControlType, DemandControlledVentilationType,
        HeatRecoveryType, HumidificationControlType, IdealLoadsAirSystem, IdealLoadsAirSystemId,
        IdealLoadsFuelType, IdealLoadsLimit, NormalizedName, OutdoorAirEconomizerType, ZoneId,
    };

    #[test]
    fn no_oa_sensible_heating_uses_supply_delta_t_and_moist_air_cp() {
        let system = test_system();
        let zone_state = IdealLoadsZoneState {
            air_temperature_c: 20.0,
            air_humidity_ratio: 0.008,
        };
        let result = calc_no_oa_no_limit_sensible_compat(
            &system,
            zone_state,
            ZoneSysEnergyDemand::sensible_only(ZoneId(0), 3000.0, 0.0),
            true,
        );

        let cp = energyplus_moist_air_specific_heat_j_per_kg_k(0.008);
        assert_eq!(result.mode, IdealLoadsSensibleMode::Heating);
        assert!((result.cp_air_j_per_kg_k - cp).abs() < 1.0e-12);
        assert!((result.supply_temperature_c - 50.0).abs() < 1.0e-12);
        assert!((result.supply_mass_flow_rate_kg_per_s - 3000.0 / (cp * 30.0)).abs() < 1.0e-12);
        assert!((result.zone_total_heating_rate_w - 3000.0).abs() < 1.0e-12);
        assert_eq!(result.zone_total_cooling_rate_w, 0.0);
    }

    #[test]
    fn no_oa_sensible_cooling_uses_absolute_cooling_demand() {
        let system = test_system();
        let zone_state = IdealLoadsZoneState {
            air_temperature_c: 25.0,
            air_humidity_ratio: 0.008,
        };
        let result = calc_no_oa_no_limit_sensible_compat(
            &system,
            zone_state,
            ZoneSysEnergyDemand::sensible_only(ZoneId(0), 0.0, -2400.0),
            true,
        );

        let cp = energyplus_moist_air_specific_heat_j_per_kg_k(0.008);
        assert_eq!(result.mode, IdealLoadsSensibleMode::Cooling);
        assert!((result.supply_temperature_c - 13.0).abs() < 1.0e-12);
        assert!((result.supply_mass_flow_rate_kg_per_s - 2400.0 / (cp * 12.0)).abs() < 1.0e-12);
        assert!((result.zone_total_cooling_rate_w - 2400.0).abs() < 1.0e-12);
        assert_eq!(result.zone_total_heating_rate_w, 0.0);
    }

    #[test]
    fn unavailable_unit_writes_dead_flow_and_zone_condition() {
        let system = test_system();
        let zone_state = IdealLoadsZoneState {
            air_temperature_c: 22.5,
            air_humidity_ratio: 0.007,
        };
        let result = calc_no_oa_no_limit_sensible_compat(
            &system,
            zone_state,
            ZoneSysEnergyDemand::sensible_only(ZoneId(0), 3000.0, -3000.0),
            false,
        );

        assert_eq!(result.mode, IdealLoadsSensibleMode::Off);
        assert_eq!(result.supply_mass_flow_rate_kg_per_s, 0.0);
        assert!((result.supply_temperature_c - 22.5).abs() < 1.0e-12);
    }

    #[test]
    fn standard_air_density_uses_energyplus_elevation_formula() {
        let density = energyplus_standard_air_density_kg_per_m3(1829.0)
            .expect("valid Golden CO elevation standard density");
        assert_close(density, 0.965_081_520_139_901_8, 1.0e-12);

        let context = IdealLoadsSensibleLimitContext::from_site_elevation_m(1829.0)
            .expect("valid Golden CO IdealLoads limit context");
        assert_close(context.standard_air_density_kg_per_m3, density, 1.0e-12);
    }

    #[test]
    fn limit_aware_helper_matches_no_limit_heating_result() {
        let system = test_system();
        let zone_state = IdealLoadsZoneState {
            air_temperature_c: 20.0,
            air_humidity_ratio: 0.008,
        };
        let demand = ZoneSysEnergyDemand::sensible_only(ZoneId(0), 3000.0, 0.0);

        let expected = calc_no_oa_no_limit_sensible_compat(&system, zone_state, demand, true);
        let actual = calc_no_oa_sensible_with_limits_compat(
            &system,
            zone_state,
            demand,
            true,
            IdealLoadsSensibleLimitContext::default(),
        );

        assert_eq!(actual.mode, expected.mode);
        assert_close(
            actual.supply_temperature_c,
            expected.supply_temperature_c,
            1.0e-12,
        );
        assert_close(
            actual.supply_mass_flow_rate_kg_per_s,
            expected.supply_mass_flow_rate_kg_per_s,
            1.0e-12,
        );
        assert_close(
            actual.zone_total_heating_rate_w,
            expected.zone_total_heating_rate_w,
            1.0e-9,
        );
    }

    #[test]
    fn heating_flow_limit_clamps_mass_flow_and_actual_output() {
        let mut system = test_system();
        system.heating_limit = IdealLoadsLimit::LimitFlowRate;
        system.maximum_heating_air_flow_rate_m3_per_s = Some(AutosizeOrNumber::Value(0.05));
        let zone_state = IdealLoadsZoneState {
            air_temperature_c: 20.0,
            air_humidity_ratio: 0.008,
        };

        let result = calc_no_oa_sensible_with_limits_compat(
            &system,
            zone_state,
            ZoneSysEnergyDemand::sensible_only(ZoneId(0), 3000.0, 0.0),
            true,
            IdealLoadsSensibleLimitContext::default(),
        );

        let cp = energyplus_moist_air_specific_heat_j_per_kg_k(0.008);
        let maximum_mass_flow_rate_kg_per_s = 0.05 * DEFAULT_STANDARD_AIR_DENSITY_KG_PER_M3;
        let expected_output_w = maximum_mass_flow_rate_kg_per_s * cp * 30.0;
        assert_eq!(result.mode, IdealLoadsSensibleMode::Heating);
        assert_close(
            result.supply_mass_flow_rate_kg_per_s,
            maximum_mass_flow_rate_kg_per_s,
            1.0e-12,
        );
        assert_close(result.supply_temperature_c, 50.0, 1.0e-12);
        assert_close(result.zone_total_heating_rate_w, expected_output_w, 1.0e-9);
        assert!(result.zone_total_heating_rate_w < 3000.0);
    }

    #[test]
    fn heating_capacity_limit_caps_output_and_adjusts_supply_temperature() {
        let mut system = test_system();
        system.heating_limit = IdealLoadsLimit::LimitCapacity;
        system.maximum_sensible_heating_capacity_w = Some(AutosizeOrNumber::Value(1000.0));
        let zone_state = IdealLoadsZoneState {
            air_temperature_c: 20.0,
            air_humidity_ratio: 0.008,
        };

        let result = calc_no_oa_sensible_with_limits_compat(
            &system,
            zone_state,
            ZoneSysEnergyDemand::sensible_only(ZoneId(0), 3000.0, 0.0),
            true,
            IdealLoadsSensibleLimitContext::default(),
        );

        let cp = energyplus_moist_air_specific_heat_j_per_kg_k(0.008);
        let unlimited_mass_flow_rate_kg_per_s = 3000.0 / (cp * 30.0);
        let expected_supply_temperature_c =
            20.0 + 1000.0 / (cp * unlimited_mass_flow_rate_kg_per_s);
        assert_eq!(result.mode, IdealLoadsSensibleMode::Heating);
        assert_close(result.zone_total_heating_rate_w, 1000.0, 1.0e-12);
        assert_close(
            result.supply_temperature_c,
            expected_supply_temperature_c,
            1.0e-12,
        );
    }

    #[test]
    fn cooling_flow_limit_clamps_mass_flow_and_actual_output() {
        let mut system = test_system();
        system.cooling_limit = IdealLoadsLimit::LimitFlowRate;
        system.maximum_cooling_air_flow_rate_m3_per_s = Some(AutosizeOrNumber::Value(0.05));
        let zone_state = IdealLoadsZoneState {
            air_temperature_c: 25.0,
            air_humidity_ratio: 0.008,
        };

        let result = calc_no_oa_sensible_with_limits_compat(
            &system,
            zone_state,
            ZoneSysEnergyDemand::sensible_only(ZoneId(0), 0.0, -2400.0),
            true,
            IdealLoadsSensibleLimitContext::default(),
        );

        let cp = energyplus_moist_air_specific_heat_j_per_kg_k(0.008);
        let maximum_mass_flow_rate_kg_per_s = 0.05 * DEFAULT_STANDARD_AIR_DENSITY_KG_PER_M3;
        let expected_output_w = maximum_mass_flow_rate_kg_per_s * cp * 12.0;
        assert_eq!(result.mode, IdealLoadsSensibleMode::Cooling);
        assert_close(
            result.supply_mass_flow_rate_kg_per_s,
            maximum_mass_flow_rate_kg_per_s,
            1.0e-12,
        );
        assert_close(result.supply_temperature_c, 13.0, 1.0e-12);
        assert_close(result.zone_total_cooling_rate_w, expected_output_w, 1.0e-9);
        assert!(result.zone_total_cooling_rate_w < 2400.0);
    }

    #[test]
    fn cooling_capacity_limit_caps_output_and_adjusts_supply_temperature() {
        let mut system = test_system();
        system.cooling_limit = IdealLoadsLimit::LimitCapacity;
        system.maximum_total_cooling_capacity_w = Some(AutosizeOrNumber::Value(1000.0));
        let zone_state = IdealLoadsZoneState {
            air_temperature_c: 25.0,
            air_humidity_ratio: 0.008,
        };

        let result = calc_no_oa_sensible_with_limits_compat(
            &system,
            zone_state,
            ZoneSysEnergyDemand::sensible_only(ZoneId(0), 0.0, -2400.0),
            true,
            IdealLoadsSensibleLimitContext::default(),
        );

        let cp = energyplus_moist_air_specific_heat_j_per_kg_k(0.008);
        let unlimited_mass_flow_rate_kg_per_s = 2400.0 / (cp * 12.0);
        let expected_supply_temperature_c =
            25.0 - 1000.0 / (cp * unlimited_mass_flow_rate_kg_per_s);
        assert_eq!(result.mode, IdealLoadsSensibleMode::Cooling);
        assert_close(result.zone_total_cooling_rate_w, 1000.0, 1.0e-12);
        assert_close(
            result.supply_temperature_c,
            expected_supply_temperature_c,
            1.0e-12,
        );
    }

    #[test]
    fn zero_capacity_limit_disables_sensible_branch_flow() {
        let mut system = test_system();
        system.heating_limit = IdealLoadsLimit::LimitCapacity;
        system.maximum_sensible_heating_capacity_w = Some(AutosizeOrNumber::Value(0.0));
        let zone_state = IdealLoadsZoneState {
            air_temperature_c: 20.0,
            air_humidity_ratio: 0.008,
        };

        let result = calc_no_oa_sensible_with_limits_compat(
            &system,
            zone_state,
            ZoneSysEnergyDemand::sensible_only(ZoneId(0), 3000.0, 0.0),
            true,
            IdealLoadsSensibleLimitContext::default(),
        );

        assert_eq!(result.mode, IdealLoadsSensibleMode::Deadband);
        assert_eq!(result.supply_mass_flow_rate_kg_per_s, 0.0);
        assert_eq!(result.zone_total_heating_rate_w, 0.0);
    }

    fn assert_close(actual: f64, expected: f64, tolerance: f64) {
        assert!(
            (actual - expected).abs() <= tolerance,
            "{actual} was not within {tolerance} of {expected}"
        );
    }

    fn test_system() -> IdealLoadsAirSystem {
        IdealLoadsAirSystem {
            id: IdealLoadsAirSystemId(0),
            name: NormalizedName::new("ZONE ONE IDEAL LOADS"),
            availability_schedule: None,
            zone_supply_air_node_name: NormalizedName::new("ZONE ONE INLETS"),
            zone_exhaust_air_node_name: None,
            system_inlet_air_node_name: None,
            maximum_heating_supply_air_temperature_c: 50.0,
            minimum_cooling_supply_air_temperature_c: 13.0,
            maximum_heating_supply_air_humidity_ratio: 0.0156,
            minimum_cooling_supply_air_humidity_ratio: 0.0077,
            heating_limit: IdealLoadsLimit::NoLimit,
            maximum_heating_air_flow_rate_m3_per_s: None,
            maximum_sensible_heating_capacity_w: None,
            cooling_limit: IdealLoadsLimit::NoLimit,
            maximum_cooling_air_flow_rate_m3_per_s: None,
            maximum_total_cooling_capacity_w: None,
            heating_availability_schedule: None,
            cooling_availability_schedule: None,
            dehumidification_control_type: DehumidificationControlType::ConstantSensibleHeatRatio,
            cooling_sensible_heat_ratio: 0.7,
            humidification_control_type: HumidificationControlType::None,
            design_specification_outdoor_air_object_name: None,
            outdoor_air_inlet_node_name: None,
            demand_controlled_ventilation_type: DemandControlledVentilationType::None,
            outdoor_air_economizer_type: OutdoorAirEconomizerType::NoEconomizer,
            heat_recovery_type: HeatRecoveryType::None,
            sensible_heat_recovery_effectiveness: 0.7,
            latent_heat_recovery_effectiveness: 0.65,
            design_specification_zonehvac_sizing_object_name: None,
            heating_fuel_efficiency_schedule: None,
            heating_fuel_type: IdealLoadsFuelType::DistrictHeatingWater,
            cooling_fuel_efficiency_schedule: None,
            cooling_fuel_type: IdealLoadsFuelType::DistrictCooling,
        }
    }
}
