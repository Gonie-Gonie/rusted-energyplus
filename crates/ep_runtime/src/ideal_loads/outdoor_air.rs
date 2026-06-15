//! IdealLoads outdoor-air design-flow and narrow sensible-load helpers.

use crate::{
    energyplus_moist_air_specific_heat_j_per_kg_k,
    ideal_loads::{IdealLoadsSensibleMode, moist_air_enthalpy_j_per_kg},
    zone_equipment::ZoneSysEnergyDemand,
};
use ep_model::{
    DesignSpecificationOutdoorAir, DesignSpecificationOutdoorAirMethod, IdealLoadsAirSystem,
};

const SMALL_TEMPERATURE_DIFFERENCE_C: f64 = 0.001;
const ENERGYPLUS_DRY_AIR_ENTHALPY_COEFFICIENT_KJ_PER_KG_K: f64 = 1.004_84;
const ENERGYPLUS_WATER_VAPOR_ENTHALPY_OFFSET_KJ_PER_KG: f64 = 2500.94;
const ENERGYPLUS_WATER_VAPOR_ENTHALPY_COEFFICIENT_KJ_PER_KG_K: f64 = 1.858_95;

/// Zone context needed by `DesignSpecification:OutdoorAir`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IdealLoadsOutdoorAirContext {
    /// Zone design occupant count used by Flow/Person.
    pub design_people_count: f64,
    /// Zone floor area in m2 used by Flow/Area.
    pub zone_floor_area_m2: f64,
    /// Zone volume in m3 used by AirChanges/Hour.
    pub zone_volume_m3: f64,
}

/// Zone or outdoor-air node conditions used by the OA sensible-load subset.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IdealLoadsOutdoorAirNodeState {
    /// Air temperature in C.
    pub air_temperature_c: f64,
    /// Air humidity ratio in kgWater/kgDryAir.
    pub air_humidity_ratio: f64,
}

/// Diagnostic report values for the narrow IdealLoads OA sensible branch.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IdealLoadsOutdoorAirSensibleResult {
    /// Operating mode inferred from EnergyPlus minimum-OA sensible gate.
    pub mode: IdealLoadsSensibleMode,
    /// EnergyPlus minimum-OA sensible output used for mode selection.
    pub minimum_outdoor_air_sensible_output_w: f64,
    /// Final outdoor-air sensible output relative to zone conditions.
    pub outdoor_air_sensible_output_w: f64,
    /// Reported OA sensible heating rate.
    pub outdoor_air_sensible_heating_rate_w: f64,
    /// Reported OA sensible cooling rate.
    pub outdoor_air_sensible_cooling_rate_w: f64,
    /// Final outdoor-air latent output relative to zone conditions.
    pub outdoor_air_latent_output_w: f64,
    /// Reported OA latent heating rate.
    pub outdoor_air_latent_heating_rate_w: f64,
    /// Reported OA latent cooling rate.
    pub outdoor_air_latent_cooling_rate_w: f64,
    /// Reported OA total heating rate.
    pub outdoor_air_total_heating_rate_w: f64,
    /// Reported OA total cooling rate.
    pub outdoor_air_total_cooling_rate_w: f64,
    /// Final supply mass flow used by the no-limit OA branch.
    pub supply_mass_flow_rate_kg_per_s: f64,
    /// Final supply air temperature for the no-limit OA branch.
    pub supply_air_temperature_c: f64,
    /// Final supply air humidity ratio for the no-humidity-control OA branch.
    pub supply_air_humidity_ratio: f64,
    /// Mixed-air temperature after OA/recirculation mixing.
    pub mixed_air_temperature_c: f64,
    /// Mixed-air humidity ratio after OA/recirculation mixing.
    pub mixed_air_humidity_ratio: f64,
}

/// Calculates the design outdoor-air volume flow in m3/s for supported methods.
#[must_use]
pub fn calc_design_outdoor_air_volume_flow_m3_per_s(
    specification: &DesignSpecificationOutdoorAir,
    context: IdealLoadsOutdoorAirContext,
) -> Option<f64> {
    let per_person = nonnegative_product(
        specification.outdoor_air_flow_per_person_m3_per_s_person,
        context.design_people_count,
    );
    let per_area = nonnegative_product(
        specification.outdoor_air_flow_per_zone_floor_area_m3_per_s_m2,
        context.zone_floor_area_m2,
    );
    let per_zone = nonnegative(specification.outdoor_air_flow_per_zone_m3_per_s);
    let air_changes = nonnegative_product(
        specification.outdoor_air_flow_air_changes_per_hour,
        context.zone_volume_m3,
    ) / 3600.0;

    match specification.method {
        DesignSpecificationOutdoorAirMethod::FlowPerPerson => Some(per_person),
        DesignSpecificationOutdoorAirMethod::FlowPerArea => Some(per_area),
        DesignSpecificationOutdoorAirMethod::FlowPerZone => Some(per_zone),
        DesignSpecificationOutdoorAirMethod::AirChangesPerHour => Some(air_changes),
        DesignSpecificationOutdoorAirMethod::Sum => {
            Some(per_person + per_area + per_zone + air_changes)
        }
        DesignSpecificationOutdoorAirMethod::Maximum => {
            Some(per_person.max(per_area).max(per_zone).max(air_changes))
        }
        DesignSpecificationOutdoorAirMethod::IndoorAirQualityProcedure
        | DesignSpecificationOutdoorAirMethod::ProportionalControlBasedOnDesignOccupancy
        | DesignSpecificationOutdoorAirMethod::ProportionalControlBasedOnOccupancySchedule => None,
    }
}

/// Applies the current OA schedule and standard density to the design volume flow.
#[must_use]
pub fn calc_scheduled_outdoor_air_mass_flow_rate_kg_per_s(
    specification: &DesignSpecificationOutdoorAir,
    context: IdealLoadsOutdoorAirContext,
    schedule_value: Option<f64>,
    standard_air_density_kg_per_m3: f64,
) -> Option<f64> {
    if !standard_air_density_kg_per_m3.is_finite() || standard_air_density_kg_per_m3 < 0.0 {
        return None;
    }
    let design_volume_flow_m3_per_s =
        calc_design_outdoor_air_volume_flow_m3_per_s(specification, context)?;
    Some(
        design_volume_flow_m3_per_s
            * schedule_multiplier(schedule_value)
            * standard_air_density_kg_per_m3,
    )
}

/// Calculates diagnostic-only IdealLoads outdoor-air report rates and mixed-air state.
///
/// This mirrors the no-economizer/no-heat-recovery/no-humidity/no-limit subset:
/// EnergyPlus first uses minimum OA sensible output to choose heat/cool/deadband,
/// then recomputes final `OASenOutput` using zone humidity for report sorting.
#[must_use]
pub fn calc_outdoor_air_sensible_report_rates_compat(
    system: &IdealLoadsAirSystem,
    zone_state: IdealLoadsOutdoorAirNodeState,
    recirculation_state: IdealLoadsOutdoorAirNodeState,
    outdoor_air_state: IdealLoadsOutdoorAirNodeState,
    demand: ZoneSysEnergyDemand,
    outdoor_air_mass_flow_rate_kg_per_s: f64,
    unit_available: bool,
) -> IdealLoadsOutdoorAirSensibleResult {
    if !unit_available || outdoor_air_mass_flow_rate_kg_per_s <= 0.0 {
        return IdealLoadsOutdoorAirSensibleResult {
            mode: if unit_available {
                IdealLoadsSensibleMode::Deadband
            } else {
                IdealLoadsSensibleMode::Off
            },
            minimum_outdoor_air_sensible_output_w: 0.0,
            outdoor_air_sensible_output_w: 0.0,
            outdoor_air_sensible_heating_rate_w: 0.0,
            outdoor_air_sensible_cooling_rate_w: 0.0,
            outdoor_air_latent_output_w: 0.0,
            outdoor_air_latent_heating_rate_w: 0.0,
            outdoor_air_latent_cooling_rate_w: 0.0,
            outdoor_air_total_heating_rate_w: 0.0,
            outdoor_air_total_cooling_rate_w: 0.0,
            supply_mass_flow_rate_kg_per_s: 0.0,
            supply_air_temperature_c: recirculation_state.air_temperature_c,
            supply_air_humidity_ratio: recirculation_state.air_humidity_ratio,
            mixed_air_temperature_c: recirculation_state.air_temperature_c,
            mixed_air_humidity_ratio: recirculation_state.air_humidity_ratio,
        };
    }

    let delta_t = outdoor_air_state.air_temperature_c - zone_state.air_temperature_c;
    let minimum_cp_air_j_per_kg_k =
        energyplus_moist_air_specific_heat_j_per_kg_k(outdoor_air_state.air_humidity_ratio);
    let minimum_outdoor_air_sensible_output_w =
        outdoor_air_mass_flow_rate_kg_per_s * minimum_cp_air_j_per_kg_k * delta_t;

    let mode = if minimum_outdoor_air_sensible_output_w >= demand.remaining_output_req_to_cool_sp_w
    {
        IdealLoadsSensibleMode::Cooling
    } else if minimum_outdoor_air_sensible_output_w < demand.remaining_output_req_to_heat_sp_w {
        IdealLoadsSensibleMode::Heating
    } else {
        IdealLoadsSensibleMode::Deadband
    };

    let final_cp_air_j_per_kg_k =
        energyplus_moist_air_specific_heat_j_per_kg_k(zone_state.air_humidity_ratio);
    let outdoor_air_sensible_output_w =
        outdoor_air_mass_flow_rate_kg_per_s * final_cp_air_j_per_kg_k * delta_t;
    let outdoor_air_sensible_heating_rate_w = if mode == IdealLoadsSensibleMode::Heating {
        (-outdoor_air_sensible_output_w).max(0.0)
    } else {
        0.0
    };
    let outdoor_air_sensible_cooling_rate_w = if mode == IdealLoadsSensibleMode::Cooling {
        outdoor_air_sensible_output_w.max(0.0)
    } else {
        0.0
    };
    let outdoor_air_latent_output_w = outdoor_air_mass_flow_rate_kg_per_s
        * (moist_air_enthalpy_j_per_kg(
            outdoor_air_state.air_temperature_c,
            outdoor_air_state.air_humidity_ratio,
        ) - moist_air_enthalpy_j_per_kg(
            zone_state.air_temperature_c,
            zone_state.air_humidity_ratio,
        ))
        - outdoor_air_sensible_output_w;
    let outdoor_air_latent_heating_rate_w = 0.0;
    let outdoor_air_latent_cooling_rate_w = 0.0;
    let outdoor_air_total_heating_rate_w =
        outdoor_air_sensible_heating_rate_w + outdoor_air_latent_heating_rate_w;
    let outdoor_air_total_cooling_rate_w =
        outdoor_air_sensible_cooling_rate_w + outdoor_air_latent_cooling_rate_w;
    let supply_mass_flow_rate_kg_per_s = outdoor_air_supply_mass_flow_rate_kg_per_s(
        system,
        zone_state,
        demand,
        mode,
        final_cp_air_j_per_kg_k,
        outdoor_air_mass_flow_rate_kg_per_s,
    );
    let (mixed_air_temperature_c, mixed_air_humidity_ratio) = mixed_air_state(
        recirculation_state,
        outdoor_air_state,
        outdoor_air_mass_flow_rate_kg_per_s,
        supply_mass_flow_rate_kg_per_s,
    );
    let (supply_air_temperature_c, supply_air_humidity_ratio) = supply_air_state(
        system,
        zone_state,
        demand,
        mode,
        final_cp_air_j_per_kg_k,
        supply_mass_flow_rate_kg_per_s,
        mixed_air_temperature_c,
        mixed_air_humidity_ratio,
    );

    IdealLoadsOutdoorAirSensibleResult {
        mode,
        minimum_outdoor_air_sensible_output_w,
        outdoor_air_sensible_output_w,
        outdoor_air_sensible_heating_rate_w,
        outdoor_air_sensible_cooling_rate_w,
        outdoor_air_latent_output_w,
        outdoor_air_latent_heating_rate_w,
        outdoor_air_latent_cooling_rate_w,
        outdoor_air_total_heating_rate_w,
        outdoor_air_total_cooling_rate_w,
        supply_mass_flow_rate_kg_per_s,
        supply_air_temperature_c,
        supply_air_humidity_ratio,
        mixed_air_temperature_c,
        mixed_air_humidity_ratio,
    }
}

fn outdoor_air_supply_mass_flow_rate_kg_per_s(
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

fn mixed_air_state(
    recirculation_state: IdealLoadsOutdoorAirNodeState,
    outdoor_air_state: IdealLoadsOutdoorAirNodeState,
    outdoor_air_mass_flow_rate_kg_per_s: f64,
    supply_mass_flow_rate_kg_per_s: f64,
) -> (f64, f64) {
    if outdoor_air_mass_flow_rate_kg_per_s <= 0.0 || supply_mass_flow_rate_kg_per_s <= 0.0 {
        return (
            recirculation_state.air_temperature_c,
            recirculation_state.air_humidity_ratio,
        );
    }
    if supply_mass_flow_rate_kg_per_s <= outdoor_air_mass_flow_rate_kg_per_s {
        return (
            outdoor_air_state.air_temperature_c,
            outdoor_air_state.air_humidity_ratio,
        );
    }

    let recirculation_mass_flow_rate_kg_per_s =
        supply_mass_flow_rate_kg_per_s - outdoor_air_mass_flow_rate_kg_per_s;
    let recirculation_enthalpy_j_per_kg = moist_air_enthalpy_j_per_kg(
        recirculation_state.air_temperature_c,
        recirculation_state.air_humidity_ratio,
    );
    let outdoor_air_enthalpy_j_per_kg = moist_air_enthalpy_j_per_kg(
        outdoor_air_state.air_temperature_c,
        outdoor_air_state.air_humidity_ratio,
    );
    let mixed_air_enthalpy_j_per_kg = (recirculation_mass_flow_rate_kg_per_s
        * recirculation_enthalpy_j_per_kg
        + outdoor_air_mass_flow_rate_kg_per_s * outdoor_air_enthalpy_j_per_kg)
        / supply_mass_flow_rate_kg_per_s;
    let mixed_air_humidity_ratio = (recirculation_mass_flow_rate_kg_per_s
        * recirculation_state.air_humidity_ratio
        + outdoor_air_mass_flow_rate_kg_per_s * outdoor_air_state.air_humidity_ratio)
        / supply_mass_flow_rate_kg_per_s;
    (
        dry_bulb_from_enthalpy_and_humidity_ratio(
            mixed_air_enthalpy_j_per_kg,
            mixed_air_humidity_ratio,
        ),
        mixed_air_humidity_ratio,
    )
}

fn supply_air_state(
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

fn dry_bulb_from_enthalpy_and_humidity_ratio(enthalpy_j_per_kg: f64, humidity_ratio: f64) -> f64 {
    (enthalpy_j_per_kg / 1000.0 - ENERGYPLUS_WATER_VAPOR_ENTHALPY_OFFSET_KJ_PER_KG * humidity_ratio)
        / (ENERGYPLUS_DRY_AIR_ENTHALPY_COEFFICIENT_KJ_PER_KG_K
            + ENERGYPLUS_WATER_VAPOR_ENTHALPY_COEFFICIENT_KJ_PER_KG_K * humidity_ratio)
}

fn schedule_multiplier(value: Option<f64>) -> f64 {
    let Some(value) = value else {
        return 1.0;
    };
    if !value.is_finite() {
        return 0.0;
    }
    value.clamp(0.0, 1.0)
}

fn nonnegative(value: f64) -> f64 {
    if value.is_finite() {
        value.max(0.0)
    } else {
        0.0
    }
}

fn nonnegative_product(left: f64, right: f64) -> f64 {
    nonnegative(left) * nonnegative(right)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ep_model::{
        DehumidificationControlType, DemandControlledVentilationType,
        DesignSpecificationOutdoorAir, DesignSpecificationOutdoorAirId,
        DesignSpecificationOutdoorAirMethod, HeatRecoveryType, HumidificationControlType,
        IdealLoadsAirSystem, IdealLoadsAirSystemId, IdealLoadsFuelType, IdealLoadsLimit,
        NormalizedName, OutdoorAirEconomizerType,
    };

    #[test]
    fn flow_zone_uses_declared_zone_volume_flow() {
        let mut specification = test_specification();
        specification.method = DesignSpecificationOutdoorAirMethod::FlowPerZone;
        specification.outdoor_air_flow_per_zone_m3_per_s = 0.05;

        let result = calc_design_outdoor_air_volume_flow_m3_per_s(
            &specification,
            IdealLoadsOutdoorAirContext {
                design_people_count: 3.0,
                zone_floor_area_m2: 20.0,
                zone_volume_m3: 60.0,
            },
        );

        assert_eq!(result, Some(0.05));
    }

    #[test]
    fn sum_combines_supported_terms() {
        let mut specification = test_specification();
        specification.method = DesignSpecificationOutdoorAirMethod::Sum;
        specification.outdoor_air_flow_per_person_m3_per_s_person = 0.004;
        specification.outdoor_air_flow_per_zone_floor_area_m3_per_s_m2 = 0.0003;
        specification.outdoor_air_flow_per_zone_m3_per_s = 0.02;
        specification.outdoor_air_flow_air_changes_per_hour = 0.6;

        let result = calc_design_outdoor_air_volume_flow_m3_per_s(
            &specification,
            IdealLoadsOutdoorAirContext {
                design_people_count: 5.0,
                zone_floor_area_m2: 40.0,
                zone_volume_m3: 90.0,
            },
        )
        .expect("sum method is supported");

        let expected = 0.004 * 5.0 + 0.0003 * 40.0 + 0.02 + 0.6 * 90.0 / 3600.0;
        assert_close(result, expected, 1.0e-12);
    }

    #[test]
    fn maximum_selects_largest_supported_term() {
        let mut specification = test_specification();
        specification.method = DesignSpecificationOutdoorAirMethod::Maximum;
        specification.outdoor_air_flow_per_person_m3_per_s_person = 0.004;
        specification.outdoor_air_flow_per_zone_floor_area_m3_per_s_m2 = 0.002;
        specification.outdoor_air_flow_per_zone_m3_per_s = 0.03;
        specification.outdoor_air_flow_air_changes_per_hour = 1.0;

        let result = calc_design_outdoor_air_volume_flow_m3_per_s(
            &specification,
            IdealLoadsOutdoorAirContext {
                design_people_count: 4.0,
                zone_floor_area_m2: 40.0,
                zone_volume_m3: 120.0,
            },
        )
        .expect("maximum method is supported");

        assert_close(result, 0.002 * 40.0, 1.0e-12);
    }

    #[test]
    fn mass_flow_applies_clamped_schedule_and_standard_density() {
        let mut specification = test_specification();
        specification.method = DesignSpecificationOutdoorAirMethod::FlowPerZone;
        specification.outdoor_air_flow_per_zone_m3_per_s = 0.1;

        let result = calc_scheduled_outdoor_air_mass_flow_rate_kg_per_s(
            &specification,
            IdealLoadsOutdoorAirContext {
                design_people_count: 0.0,
                zone_floor_area_m2: 0.0,
                zone_volume_m3: 0.0,
            },
            Some(2.0),
            1.2,
        );

        assert_eq!(result, Some(0.12));
    }

    #[test]
    fn sensible_report_sorts_cold_oa_as_heating_load_when_heat_mode_is_active() {
        let result = calc_outdoor_air_sensible_report_rates_compat(
            &test_system(),
            IdealLoadsOutdoorAirNodeState {
                air_temperature_c: 21.0,
                air_humidity_ratio: 0.006,
            },
            IdealLoadsOutdoorAirNodeState {
                air_temperature_c: 21.0,
                air_humidity_ratio: 0.006,
            },
            IdealLoadsOutdoorAirNodeState {
                air_temperature_c: 5.0,
                air_humidity_ratio: 0.004,
            },
            ZoneSysEnergyDemand::sensible_only(ep_model::ZoneId(0), 100.0, 0.0),
            0.05,
            true,
        );

        assert_eq!(result.mode, IdealLoadsSensibleMode::Heating);
        assert!(result.supply_mass_flow_rate_kg_per_s >= 0.05);
        assert!(result.mixed_air_temperature_c < 21.0);
        assert!(result.supply_air_temperature_c >= result.mixed_air_temperature_c);
        assert_eq!(
            result.supply_air_humidity_ratio,
            result.mixed_air_humidity_ratio
        );
        assert!(result.outdoor_air_sensible_output_w < 0.0);
        assert!(result.outdoor_air_latent_output_w.is_finite());
        assert_eq!(
            result.outdoor_air_total_heating_rate_w,
            result.outdoor_air_sensible_heating_rate_w
        );
        assert_eq!(result.outdoor_air_latent_heating_rate_w, 0.0);
        assert_close(
            result.outdoor_air_sensible_heating_rate_w,
            -result.outdoor_air_sensible_output_w,
            1.0e-12,
        );
        assert_eq!(result.outdoor_air_sensible_cooling_rate_w, 0.0);
    }

    #[test]
    fn sensible_report_sorts_warm_oa_as_cooling_load_when_cool_mode_is_active() {
        let result = calc_outdoor_air_sensible_report_rates_compat(
            &test_system(),
            IdealLoadsOutdoorAirNodeState {
                air_temperature_c: 24.0,
                air_humidity_ratio: 0.006,
            },
            IdealLoadsOutdoorAirNodeState {
                air_temperature_c: 24.0,
                air_humidity_ratio: 0.006,
            },
            IdealLoadsOutdoorAirNodeState {
                air_temperature_c: 32.0,
                air_humidity_ratio: 0.008,
            },
            ZoneSysEnergyDemand::sensible_only(ep_model::ZoneId(0), 0.0, -50.0),
            0.05,
            true,
        );

        assert_eq!(result.mode, IdealLoadsSensibleMode::Cooling);
        assert!(result.supply_mass_flow_rate_kg_per_s >= 0.05);
        assert!(result.mixed_air_temperature_c > 24.0);
        assert!(result.supply_air_temperature_c <= result.mixed_air_temperature_c);
        assert_eq!(
            result.supply_air_humidity_ratio,
            result.mixed_air_humidity_ratio
        );
        assert!(result.outdoor_air_sensible_output_w > 0.0);
        assert!(result.outdoor_air_latent_output_w.is_finite());
        assert_eq!(
            result.outdoor_air_total_cooling_rate_w,
            result.outdoor_air_sensible_cooling_rate_w
        );
        assert_eq!(result.outdoor_air_latent_cooling_rate_w, 0.0);
        assert_close(
            result.outdoor_air_sensible_cooling_rate_w,
            result.outdoor_air_sensible_output_w,
            1.0e-12,
        );
        assert_eq!(result.outdoor_air_sensible_heating_rate_w, 0.0);
    }

    #[test]
    fn unsupported_methods_remain_unresolved() {
        let mut specification = test_specification();
        specification.method = DesignSpecificationOutdoorAirMethod::IndoorAirQualityProcedure;

        assert_eq!(
            calc_design_outdoor_air_volume_flow_m3_per_s(
                &specification,
                IdealLoadsOutdoorAirContext {
                    design_people_count: 1.0,
                    zone_floor_area_m2: 1.0,
                    zone_volume_m3: 1.0,
                },
            ),
            None
        );
    }

    fn test_specification() -> DesignSpecificationOutdoorAir {
        DesignSpecificationOutdoorAir {
            id: DesignSpecificationOutdoorAirId(0),
            name: NormalizedName::new("OUTDOOR AIR SPEC"),
            method: DesignSpecificationOutdoorAirMethod::FlowPerPerson,
            outdoor_air_flow_per_person_m3_per_s_person: 0.00944,
            outdoor_air_flow_per_zone_floor_area_m3_per_s_m2: 0.0,
            outdoor_air_flow_per_zone_m3_per_s: 0.0,
            outdoor_air_flow_air_changes_per_hour: 0.0,
            outdoor_air_schedule: None,
            proportional_control_minimum_outdoor_air_flow_rate_schedule: None,
        }
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
            design_specification_outdoor_air_object_name: Some(NormalizedName::new(
                "OUTDOOR AIR SPEC",
            )),
            outdoor_air_inlet_node_name: Some(NormalizedName::new("OUTDOOR AIR NODE")),
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

    fn assert_close(actual: f64, expected: f64, tolerance: f64) {
        assert!(
            (actual - expected).abs() <= tolerance,
            "{actual} was not within {tolerance} of {expected}"
        );
    }
}
