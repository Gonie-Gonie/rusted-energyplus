//! IdealLoads outdoor-air design-flow and narrow sensible-load helpers.

mod dcv;
mod design_flow;
mod economizer;
mod minimum_flow;
mod mixed_air;
mod psychrometrics;
mod supply;

pub use dcv::*;
pub use design_flow::*;
pub use minimum_flow::*;

use crate::{
    energyplus_moist_air_specific_heat_j_per_kg_k,
    ideal_loads::{
        IdealLoadsInitFlags, IdealLoadsSensibleLimitContext, IdealLoadsSensibleMode,
        moist_air_enthalpy_j_per_kg,
    },
    node::IdealLoadsSupplyNodeUpdate,
    zone_equipment::ZoneSysEnergyDemand,
};
use ep_model::{IdealLoadsAirSystem, IdealLoadsAirSystemId, NodeId};

use economizer::calc_economizer_adjusted_outdoor_air_mass_flow_rate_kg_per_s;
use mixed_air::mixed_air_state;
use supply::{outdoor_air_supply_mass_flow_rate_kg_per_s, supply_air_state};

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

/// Inputs consumed by the outdoor-air source-order PurchasedAir wrapper.
#[derive(Clone, Copy, Debug)]
pub struct SimPurchasedAirOutdoorAirCompatInput<'a> {
    /// Prebound typed IdealLoads system.
    pub system: &'a IdealLoadsAirSystem,
    /// Resolved supply node to update.
    pub supply_node: NodeId,
    /// Zone state visible to `CalcPurchAirLoads`.
    pub zone_state: IdealLoadsOutdoorAirNodeState,
    /// Recirculation/mixed-air inlet state.
    pub recirculation_state: IdealLoadsOutdoorAirNodeState,
    /// Outdoor-air inlet state.
    pub outdoor_air_state: IdealLoadsOutdoorAirNodeState,
    /// Source-order zone demand snapshot.
    pub demand: ZoneSysEnergyDemand,
    /// Raw source-order inputs used to resolve minimum outdoor-air flow.
    pub minimum_outdoor_air: IdealLoadsMinimumOutdoorAirCompatInput<'a>,
    /// System timestep used by active-time report variables.
    pub system_timestep_hours: f64,
    /// Standard density and timestep barometric pressure.
    pub limit_context: IdealLoadsSensibleLimitContext,
    /// Availability-schedule result for this timestep.
    pub unit_available: bool,
}

/// Outdoor-air PurchasedAir wrapper result in source order.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SimPurchasedAirOutdoorAirCompatOutput {
    /// Typed IdealLoads system ID used instead of a runtime string lookup.
    pub system_id: IdealLoadsAirSystemId,
    /// Rust-visible branch selected inside `CalcPurchAirLoads`.
    pub selected_branch: &'static str,
    /// `InitPurchasedAir` equivalent flags.
    pub init_flags: IdealLoadsInitFlags,
    /// `CalcPurchAirMinOAMassFlow` equivalent result, absent when the unit is off.
    pub minimum_outdoor_air: Option<IdealLoadsMinimumOutdoorAirCompatResult>,
    /// `CalcPurchAirLoads` equivalent outdoor-air result.
    pub calculation: IdealLoadsOutdoorAirSensibleResult,
    /// `UpdatePurchasedAir` equivalent node write.
    pub supply_node_update: IdealLoadsSupplyNodeUpdate,
    /// Optional diagnostic trace payload for source-order auditing.
    pub trace: IdealLoadsOutdoorAirPurchasedAirTrace,
}

/// Diagnostic trace payload for the outdoor-air PurchasedAir wrapper.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IdealLoadsOutdoorAirPurchasedAirTrace {
    /// Zone state consumed by the calc stage.
    pub zone_state: IdealLoadsOutdoorAirNodeState,
    /// Recirculation state consumed by the calc stage.
    pub recirculation_state: IdealLoadsOutdoorAirNodeState,
    /// Outdoor-air state consumed by the calc stage.
    pub outdoor_air_state: IdealLoadsOutdoorAirNodeState,
    /// Zone demand consumed by the calc stage.
    pub demand: ZoneSysEnergyDemand,
    /// Evaluated OA schedule value supplied at the timestep boundary.
    pub outdoor_air_schedule_value: Option<f64>,
    /// Current occupants supplied for OccupancySchedule DCV.
    pub current_people_count: Option<f64>,
    /// CO2 setpoint demand supplied for CO2Setpoint DCV in kg/s.
    pub co2_setpoint_required_mass_flow_rate_kg_per_s: Option<f64>,
    /// Minimum outdoor-air mass flow consumed by the calc stage.
    pub minimum_outdoor_air_mass_flow_rate_kg_per_s: f64,
}

/// Executes the outdoor-air source-order `SimPurchasedAir` equivalent.
pub fn sim_purchased_air_outdoor_air_compat(
    input: SimPurchasedAirOutdoorAirCompatInput<'_>,
) -> Result<SimPurchasedAirOutdoorAirCompatOutput, SimPurchasedAirOutdoorAirCompatError> {
    let init_flags = IdealLoadsInitFlags::diagnostic_adapter_assumed_ready();
    let minimum_outdoor_air = if input.unit_available {
        Some(resolve_minimum_outdoor_air_compat(
            input.system,
            input.minimum_outdoor_air,
            input.limit_context,
        )?)
    } else {
        None
    };
    let minimum_outdoor_air_mass_flow_rate_kg_per_s = minimum_outdoor_air
        .map(|result| result.final_minimum_mass_flow_rate_kg_per_s)
        .unwrap_or(0.0);
    let calculation = calc_outdoor_air_sensible_report_rates_compat(
        input.system,
        input.zone_state,
        input.recirculation_state,
        input.outdoor_air_state,
        input.demand,
        minimum_outdoor_air_mass_flow_rate_kg_per_s,
        input.system_timestep_hours,
        input.limit_context.barometric_pressure_pa,
        input.unit_available,
    );
    let supply_node_update = IdealLoadsSupplyNodeUpdate {
        node: input.supply_node,
        temperature_c: calculation.supply_air_temperature_c,
        humidity_ratio: calculation.supply_air_humidity_ratio,
        mass_flow_rate_kg_per_s: calculation.supply_mass_flow_rate_kg_per_s,
        enthalpy_j_per_kg: moist_air_enthalpy_j_per_kg(
            calculation.supply_air_temperature_c,
            calculation.supply_air_humidity_ratio,
        ),
    };
    let trace = IdealLoadsOutdoorAirPurchasedAirTrace {
        zone_state: input.zone_state,
        recirculation_state: input.recirculation_state,
        outdoor_air_state: input.outdoor_air_state,
        demand: input.demand,
        outdoor_air_schedule_value: input.minimum_outdoor_air.outdoor_air_schedule_value,
        current_people_count: input.minimum_outdoor_air.current_people_count,
        co2_setpoint_required_mass_flow_rate_kg_per_s: input
            .minimum_outdoor_air
            .co2_setpoint_required_mass_flow_rate_kg_per_s,
        minimum_outdoor_air_mass_flow_rate_kg_per_s,
    };

    Ok(SimPurchasedAirOutdoorAirCompatOutput {
        system_id: input.system.id,
        selected_branch: "outdoor_air",
        init_flags,
        minimum_outdoor_air,
        calculation,
        supply_node_update,
        trace,
    })
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

#[cfg(test)]
#[path = "outdoor_air_dcv_tests.rs"]
mod outdoor_air_dcv_tests;

#[cfg(test)]
#[path = "outdoor_air_tests.rs"]
mod outdoor_air_tests;

#[cfg(test)]
#[path = "outdoor_air_wrapper_tests.rs"]
mod outdoor_air_wrapper_tests;
