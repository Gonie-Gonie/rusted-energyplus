//! Mixed-air and heat-recovery helpers for IdealLoads outdoor-air compatibility.

use crate::{
    energyplus_moist_air_specific_heat_j_per_kg_k,
    ideal_loads::{IdealLoadsSensibleMode, moist_air_enthalpy_j_per_kg},
};
use ep_model::{HeatRecoveryType, IdealLoadsAirSystem};

use super::{
    IdealLoadsOutdoorAirNodeState,
    psychrometrics::{
        dry_bulb_from_enthalpy_and_humidity_ratio, heat_recovery_saturation_adjusted_state,
    },
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct IdealLoadsMixedAirResult {
    pub(super) mixed_air_temperature_c: f64,
    pub(super) mixed_air_humidity_ratio: f64,
    pub(super) heat_recovery_sensible_output_w: f64,
    pub(super) heat_recovery_latent_output_w: f64,
    pub(super) heat_recovery_sensible_heating_rate_w: f64,
    pub(super) heat_recovery_sensible_cooling_rate_w: f64,
    pub(super) heat_recovery_latent_heating_rate_w: f64,
    pub(super) heat_recovery_latent_cooling_rate_w: f64,
    pub(super) heat_recovery_total_heating_rate_w: f64,
    pub(super) heat_recovery_total_cooling_rate_w: f64,
    pub(super) heat_recovery_active_time_hr: f64,
}

pub(super) fn mixed_air_state(
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
