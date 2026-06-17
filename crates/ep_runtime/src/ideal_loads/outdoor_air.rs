//! IdealLoads outdoor-air design-flow and narrow sensible-load helpers.

mod dcv;
mod design_flow;
mod economizer;
mod psychrometrics;

pub use dcv::*;
pub use design_flow::*;

use crate::{
    energyplus_moist_air_specific_heat_j_per_kg_k,
    ideal_loads::{IdealLoadsSensibleMode, moist_air_enthalpy_j_per_kg},
    zone_equipment::ZoneSysEnergyDemand,
};
use ep_model::{HeatRecoveryType, IdealLoadsAirSystem};

use economizer::calc_economizer_adjusted_outdoor_air_mass_flow_rate_kg_per_s;
use psychrometrics::{
    dry_bulb_from_enthalpy_and_humidity_ratio, heat_recovery_saturation_adjusted_state,
};

pub(super) const SMALL_TEMPERATURE_DIFFERENCE_C: f64 = 0.001;

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
    /// Final outdoor-air mass flow after any active economizer adjustment.
    pub outdoor_air_mass_flow_rate_kg_per_s: f64,
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
    /// Reported economizer active time for this system timestep.
    pub economizer_active_time_hr: f64,
    /// Final heat-recovery sensible output relative to outdoor-air inlet conditions.
    pub heat_recovery_sensible_output_w: f64,
    /// Final heat-recovery latent output relative to outdoor-air inlet conditions.
    pub heat_recovery_latent_output_w: f64,
    /// Reported heat-recovery sensible heating rate.
    pub heat_recovery_sensible_heating_rate_w: f64,
    /// Reported heat-recovery sensible cooling rate.
    pub heat_recovery_sensible_cooling_rate_w: f64,
    /// Reported heat-recovery latent heating rate.
    pub heat_recovery_latent_heating_rate_w: f64,
    /// Reported heat-recovery latent cooling rate.
    pub heat_recovery_latent_cooling_rate_w: f64,
    /// Reported heat-recovery total heating rate.
    pub heat_recovery_total_heating_rate_w: f64,
    /// Reported heat-recovery total cooling rate.
    pub heat_recovery_total_cooling_rate_w: f64,
    /// Reported heat-recovery active time for this system timestep.
    pub heat_recovery_active_time_hr: f64,
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
    minimum_outdoor_air_mass_flow_rate_kg_per_s: f64,
    system_timestep_hours: f64,
    barometric_pressure_pa: f64,
    unit_available: bool,
) -> IdealLoadsOutdoorAirSensibleResult {
    let mut outdoor_air_mass_flow_rate_kg_per_s =
        if minimum_outdoor_air_mass_flow_rate_kg_per_s.is_finite() {
            minimum_outdoor_air_mass_flow_rate_kg_per_s.max(0.0)
        } else {
            0.0
        };
    if !unit_available {
        return IdealLoadsOutdoorAirSensibleResult {
            mode: IdealLoadsSensibleMode::Off,
            minimum_outdoor_air_sensible_output_w: 0.0,
            outdoor_air_mass_flow_rate_kg_per_s: 0.0,
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
            economizer_active_time_hr: 0.0,
            heat_recovery_sensible_output_w: 0.0,
            heat_recovery_latent_output_w: 0.0,
            heat_recovery_sensible_heating_rate_w: 0.0,
            heat_recovery_sensible_cooling_rate_w: 0.0,
            heat_recovery_latent_heating_rate_w: 0.0,
            heat_recovery_latent_cooling_rate_w: 0.0,
            heat_recovery_total_heating_rate_w: 0.0,
            heat_recovery_total_cooling_rate_w: 0.0,
            heat_recovery_active_time_hr: 0.0,
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
    let economizer_active_time_hr = calc_economizer_adjusted_outdoor_air_mass_flow_rate_kg_per_s(
        system,
        zone_state,
        recirculation_state,
        outdoor_air_state,
        demand,
        mode,
        final_cp_air_j_per_kg_k,
        system_timestep_hours,
        &mut outdoor_air_mass_flow_rate_kg_per_s,
    );
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
    let mixed_air_result = mixed_air_state(
        system,
        recirculation_state,
        outdoor_air_state,
        mode,
        system_timestep_hours,
        barometric_pressure_pa,
        outdoor_air_mass_flow_rate_kg_per_s,
        supply_mass_flow_rate_kg_per_s,
    );
    let mixed_air_temperature_c = mixed_air_result.mixed_air_temperature_c;
    let mixed_air_humidity_ratio = mixed_air_result.mixed_air_humidity_ratio;
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
        outdoor_air_mass_flow_rate_kg_per_s,
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
        economizer_active_time_hr,
        heat_recovery_sensible_output_w: mixed_air_result.heat_recovery_sensible_output_w,
        heat_recovery_latent_output_w: mixed_air_result.heat_recovery_latent_output_w,
        heat_recovery_sensible_heating_rate_w: mixed_air_result
            .heat_recovery_sensible_heating_rate_w,
        heat_recovery_sensible_cooling_rate_w: mixed_air_result
            .heat_recovery_sensible_cooling_rate_w,
        heat_recovery_latent_heating_rate_w: mixed_air_result.heat_recovery_latent_heating_rate_w,
        heat_recovery_latent_cooling_rate_w: mixed_air_result.heat_recovery_latent_cooling_rate_w,
        heat_recovery_total_heating_rate_w: mixed_air_result.heat_recovery_total_heating_rate_w,
        heat_recovery_total_cooling_rate_w: mixed_air_result.heat_recovery_total_cooling_rate_w,
        heat_recovery_active_time_hr: mixed_air_result.heat_recovery_active_time_hr,
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

#[derive(Clone, Copy, Debug, PartialEq)]
struct IdealLoadsMixedAirResult {
    mixed_air_temperature_c: f64,
    mixed_air_humidity_ratio: f64,
    heat_recovery_sensible_output_w: f64,
    heat_recovery_latent_output_w: f64,
    heat_recovery_sensible_heating_rate_w: f64,
    heat_recovery_sensible_cooling_rate_w: f64,
    heat_recovery_latent_heating_rate_w: f64,
    heat_recovery_latent_cooling_rate_w: f64,
    heat_recovery_total_heating_rate_w: f64,
    heat_recovery_total_cooling_rate_w: f64,
    heat_recovery_active_time_hr: f64,
}

fn mixed_air_state(
    system: &IdealLoadsAirSystem,
    recirculation_state: IdealLoadsOutdoorAirNodeState,
    outdoor_air_state: IdealLoadsOutdoorAirNodeState,
    mode: IdealLoadsSensibleMode,
    system_timestep_hours: f64,
    barometric_pressure_pa: f64,
    outdoor_air_mass_flow_rate_kg_per_s: f64,
    supply_mass_flow_rate_kg_per_s: f64,
) -> IdealLoadsMixedAirResult {
    if outdoor_air_mass_flow_rate_kg_per_s <= 0.0 || supply_mass_flow_rate_kg_per_s <= 0.0 {
        return IdealLoadsMixedAirResult {
            mixed_air_temperature_c: recirculation_state.air_temperature_c,
            mixed_air_humidity_ratio: recirculation_state.air_humidity_ratio,
            heat_recovery_sensible_output_w: 0.0,
            heat_recovery_latent_output_w: 0.0,
            heat_recovery_sensible_heating_rate_w: 0.0,
            heat_recovery_sensible_cooling_rate_w: 0.0,
            heat_recovery_latent_heating_rate_w: 0.0,
            heat_recovery_latent_cooling_rate_w: 0.0,
            heat_recovery_total_heating_rate_w: 0.0,
            heat_recovery_total_cooling_rate_w: 0.0,
            heat_recovery_active_time_hr: 0.0,
        };
    }

    let recirculation_enthalpy_j_per_kg = moist_air_enthalpy_j_per_kg(
        recirculation_state.air_temperature_c,
        recirculation_state.air_humidity_ratio,
    );
    let outdoor_air_inlet_enthalpy_j_per_kg = moist_air_enthalpy_j_per_kg(
        outdoor_air_state.air_temperature_c,
        outdoor_air_state.air_humidity_ratio,
    );
    let heat_recovery_active = heat_recovery_allows_outdoor_air_tempering(
        system.heat_recovery_type,
        recirculation_state,
        outdoor_air_state,
        recirculation_enthalpy_j_per_kg,
        outdoor_air_inlet_enthalpy_j_per_kg,
        mode,
    );
    let heat_recovery_active_time_hr = if heat_recovery_active {
        system_timestep_hours.max(0.0)
    } else {
        0.0
    };

    let mut outdoor_air_after_heat_recovery_temperature_c = if heat_recovery_active {
        outdoor_air_state.air_temperature_c
            + system.sensible_heat_recovery_effectiveness
                * (recirculation_state.air_temperature_c - outdoor_air_state.air_temperature_c)
    } else {
        outdoor_air_state.air_temperature_c
    };
    let mut outdoor_air_after_heat_recovery_humidity_ratio = if heat_recovery_active
        && system.heat_recovery_type == HeatRecoveryType::Enthalpy
    {
        outdoor_air_state.air_humidity_ratio
            + system.latent_heat_recovery_effectiveness
                * (recirculation_state.air_humidity_ratio - outdoor_air_state.air_humidity_ratio)
    } else {
        outdoor_air_state.air_humidity_ratio
    };
    let mut outdoor_air_after_heat_recovery_enthalpy_j_per_kg = moist_air_enthalpy_j_per_kg(
        outdoor_air_after_heat_recovery_temperature_c,
        outdoor_air_after_heat_recovery_humidity_ratio,
    );
    if heat_recovery_active {
        (
            outdoor_air_after_heat_recovery_temperature_c,
            outdoor_air_after_heat_recovery_humidity_ratio,
        ) = heat_recovery_saturation_adjusted_state(
            outdoor_air_after_heat_recovery_temperature_c,
            outdoor_air_after_heat_recovery_humidity_ratio,
            outdoor_air_after_heat_recovery_enthalpy_j_per_kg,
            barometric_pressure_pa,
        );
        outdoor_air_after_heat_recovery_enthalpy_j_per_kg = moist_air_enthalpy_j_per_kg(
            outdoor_air_after_heat_recovery_temperature_c,
            outdoor_air_after_heat_recovery_humidity_ratio,
        );
    }
    let (mixed_air_temperature_c, mixed_air_humidity_ratio) = if supply_mass_flow_rate_kg_per_s
        <= outdoor_air_mass_flow_rate_kg_per_s
    {
        (
            outdoor_air_after_heat_recovery_temperature_c,
            outdoor_air_after_heat_recovery_humidity_ratio,
        )
    } else {
        let recirculation_mass_flow_rate_kg_per_s =
            supply_mass_flow_rate_kg_per_s - outdoor_air_mass_flow_rate_kg_per_s;
        let mixed_air_enthalpy_j_per_kg = (recirculation_mass_flow_rate_kg_per_s
            * recirculation_enthalpy_j_per_kg
            + outdoor_air_mass_flow_rate_kg_per_s
                * outdoor_air_after_heat_recovery_enthalpy_j_per_kg)
            / supply_mass_flow_rate_kg_per_s;
        let mixed_air_humidity_ratio = (recirculation_mass_flow_rate_kg_per_s
            * recirculation_state.air_humidity_ratio
            + outdoor_air_mass_flow_rate_kg_per_s * outdoor_air_after_heat_recovery_humidity_ratio)
            / supply_mass_flow_rate_kg_per_s;
        (
            dry_bulb_from_enthalpy_and_humidity_ratio(
                mixed_air_enthalpy_j_per_kg,
                mixed_air_humidity_ratio,
            ),
            mixed_air_humidity_ratio,
        )
    };
    let cp_air_j_per_kg_k =
        energyplus_moist_air_specific_heat_j_per_kg_k(outdoor_air_state.air_humidity_ratio);
    let heat_recovery_sensible_output_w = outdoor_air_mass_flow_rate_kg_per_s
        * cp_air_j_per_kg_k
        * (outdoor_air_after_heat_recovery_temperature_c - outdoor_air_state.air_temperature_c);
    let heat_recovery_latent_output_w = outdoor_air_mass_flow_rate_kg_per_s
        * (outdoor_air_after_heat_recovery_enthalpy_j_per_kg - outdoor_air_inlet_enthalpy_j_per_kg)
        - heat_recovery_sensible_output_w;
    let heat_recovery_sensible_heating_rate_w = heat_recovery_sensible_output_w.max(0.0);
    let heat_recovery_sensible_cooling_rate_w = heat_recovery_sensible_output_w.min(0.0).abs();
    let heat_recovery_latent_heating_rate_w = heat_recovery_latent_output_w.max(0.0);
    let heat_recovery_latent_cooling_rate_w = heat_recovery_latent_output_w.min(0.0).abs();
    IdealLoadsMixedAirResult {
        mixed_air_temperature_c,
        mixed_air_humidity_ratio,
        heat_recovery_sensible_output_w,
        heat_recovery_latent_output_w,
        heat_recovery_sensible_heating_rate_w,
        heat_recovery_sensible_cooling_rate_w,
        heat_recovery_latent_heating_rate_w,
        heat_recovery_latent_cooling_rate_w,
        heat_recovery_total_heating_rate_w: heat_recovery_sensible_heating_rate_w
            + heat_recovery_latent_heating_rate_w,
        heat_recovery_total_cooling_rate_w: heat_recovery_sensible_cooling_rate_w
            + heat_recovery_latent_cooling_rate_w,
        heat_recovery_active_time_hr,
    }
}

fn heat_recovery_allows_outdoor_air_tempering(
    heat_recovery_type: HeatRecoveryType,
    recirculation_state: IdealLoadsOutdoorAirNodeState,
    outdoor_air_state: IdealLoadsOutdoorAirNodeState,
    recirculation_enthalpy_j_per_kg: f64,
    outdoor_air_inlet_enthalpy_j_per_kg: f64,
    mode: IdealLoadsSensibleMode,
) -> bool {
    match (heat_recovery_type, mode) {
        (HeatRecoveryType::Sensible, IdealLoadsSensibleMode::Heating) => {
            recirculation_state.air_temperature_c > outdoor_air_state.air_temperature_c
        }
        (HeatRecoveryType::Sensible, IdealLoadsSensibleMode::Cooling) => {
            recirculation_state.air_temperature_c < outdoor_air_state.air_temperature_c
        }
        (HeatRecoveryType::Enthalpy, IdealLoadsSensibleMode::Heating) => {
            recirculation_enthalpy_j_per_kg > outdoor_air_inlet_enthalpy_j_per_kg
        }
        (HeatRecoveryType::Enthalpy, IdealLoadsSensibleMode::Cooling) => {
            recirculation_enthalpy_j_per_kg < outdoor_air_inlet_enthalpy_j_per_kg
        }
        _ => false,
    }
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

#[cfg(test)]
#[path = "outdoor_air_dcv_tests.rs"]
mod outdoor_air_dcv_tests;

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
            0.25,
            101_325.0,
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
            0.25,
            101_325.0,
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
    fn differential_dry_bulb_economizer_raises_outdoor_air_flow_when_cooling() {
        let mut system = test_system();
        system.outdoor_air_economizer_type = OutdoorAirEconomizerType::DifferentialDryBulb;
        let result = calc_outdoor_air_sensible_report_rates_compat(
            &system,
            IdealLoadsOutdoorAirNodeState {
                air_temperature_c: 24.0,
                air_humidity_ratio: 0.006,
            },
            IdealLoadsOutdoorAirNodeState {
                air_temperature_c: 24.0,
                air_humidity_ratio: 0.006,
            },
            IdealLoadsOutdoorAirNodeState {
                air_temperature_c: 10.0,
                air_humidity_ratio: 0.004,
            },
            ZoneSysEnergyDemand::sensible_only(ep_model::ZoneId(0), 0.0, -500.0),
            0.01,
            0.25,
            101_325.0,
            true,
        );

        assert_eq!(result.mode, IdealLoadsSensibleMode::Cooling);
        assert!(result.outdoor_air_mass_flow_rate_kg_per_s > 0.01);
        assert_eq!(result.economizer_active_time_hr, 0.25);
        assert!(
            result.supply_mass_flow_rate_kg_per_s >= result.outdoor_air_mass_flow_rate_kg_per_s
        );
    }

    #[test]
    fn differential_enthalpy_economizer_raises_outdoor_air_flow_when_cooling() {
        let mut system = test_system();
        system.outdoor_air_economizer_type = OutdoorAirEconomizerType::DifferentialEnthalpy;
        let result = calc_outdoor_air_sensible_report_rates_compat(
            &system,
            IdealLoadsOutdoorAirNodeState {
                air_temperature_c: 24.0,
                air_humidity_ratio: 0.010,
            },
            IdealLoadsOutdoorAirNodeState {
                air_temperature_c: 24.0,
                air_humidity_ratio: 0.010,
            },
            IdealLoadsOutdoorAirNodeState {
                air_temperature_c: 12.0,
                air_humidity_ratio: 0.002,
            },
            ZoneSysEnergyDemand::sensible_only(ep_model::ZoneId(0), 0.0, -500.0),
            0.01,
            0.25,
            101_325.0,
            true,
        );

        assert_eq!(result.mode, IdealLoadsSensibleMode::Cooling);
        assert!(result.outdoor_air_mass_flow_rate_kg_per_s > 0.01);
        assert_eq!(result.economizer_active_time_hr, 0.25);
        assert!(
            result.supply_mass_flow_rate_kg_per_s >= result.outdoor_air_mass_flow_rate_kg_per_s
        );
    }

    #[test]
    fn sensible_heat_recovery_reports_active_heating_output() {
        let mut system = test_system();
        system.heat_recovery_type = HeatRecoveryType::Sensible;
        let result = calc_outdoor_air_sensible_report_rates_compat(
            &system,
            IdealLoadsOutdoorAirNodeState {
                air_temperature_c: 21.0,
                air_humidity_ratio: 0.006,
            },
            IdealLoadsOutdoorAirNodeState {
                air_temperature_c: 21.0,
                air_humidity_ratio: 0.006,
            },
            IdealLoadsOutdoorAirNodeState {
                air_temperature_c: 0.0,
                air_humidity_ratio: 0.004,
            },
            ZoneSysEnergyDemand::sensible_only(ep_model::ZoneId(0), 500.0, 0.0),
            0.01,
            0.25,
            101_325.0,
            true,
        );

        assert_eq!(result.mode, IdealLoadsSensibleMode::Heating);
        assert_eq!(result.heat_recovery_active_time_hr, 0.25);
        assert!(result.heat_recovery_sensible_heating_rate_w > 0.0);
        assert_eq!(result.heat_recovery_sensible_cooling_rate_w, 0.0);
        assert_close(result.heat_recovery_latent_output_w, 0.0, 1.0e-9);
        assert_eq!(
            result.heat_recovery_total_heating_rate_w,
            result.heat_recovery_sensible_heating_rate_w
        );
    }

    #[test]
    fn enthalpy_heat_recovery_reports_active_latent_heating_output() {
        let mut system = test_system();
        system.heat_recovery_type = HeatRecoveryType::Enthalpy;
        let result = calc_outdoor_air_sensible_report_rates_compat(
            &system,
            IdealLoadsOutdoorAirNodeState {
                air_temperature_c: 21.0,
                air_humidity_ratio: 0.010,
            },
            IdealLoadsOutdoorAirNodeState {
                air_temperature_c: 21.0,
                air_humidity_ratio: 0.010,
            },
            IdealLoadsOutdoorAirNodeState {
                air_temperature_c: 0.0,
                air_humidity_ratio: 0.002,
            },
            ZoneSysEnergyDemand::sensible_only(ep_model::ZoneId(0), 500.0, 0.0),
            0.01,
            0.25,
            101_325.0,
            true,
        );

        assert_eq!(result.mode, IdealLoadsSensibleMode::Heating);
        assert_eq!(result.heat_recovery_active_time_hr, 0.25);
        assert!(result.heat_recovery_sensible_heating_rate_w > 0.0);
        assert!(result.heat_recovery_latent_heating_rate_w > 0.0);
        assert_eq!(result.heat_recovery_latent_cooling_rate_w, 0.0);
        assert!(
            result.heat_recovery_total_heating_rate_w
                > result.heat_recovery_sensible_heating_rate_w
        );
        assert!(result.mixed_air_humidity_ratio > 0.002);
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
