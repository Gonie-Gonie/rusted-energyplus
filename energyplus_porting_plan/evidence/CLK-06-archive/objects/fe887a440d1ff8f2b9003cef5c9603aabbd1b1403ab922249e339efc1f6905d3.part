//! Heat-balance trace accumulator value types.

use super::state::{
    HeatBalanceCtfHistorySlotFirstSample, HeatBalanceCtfHistorySlotHourlySample,
    HeatBalanceCtfHistorySlotSample, HeatBalanceState, HeatBalanceSurfaceFirstSampleTrace,
    HeatBalanceSurfaceIterationFirstSampleTrace, HeatBalanceSurfaceIterationSampleTrace,
    HeatBalanceZoneAirFirstSampleTrace, HeatBalanceZoneAirReportSampling,
    HeatBalanceZoneAirStateSample, ZoneHeatBalanceState,
};
use ep_model::ZoneId;
use std::collections::BTreeMap;

pub(crate) struct HeatBalanceRunPeriodSamples {
    pub(crate) zone_temperatures: Vec<(ep_model::ZoneId, String, Vec<f64>)>,
    pub(crate) zone_humidity_ratios: Vec<(ep_model::ZoneId, String, Vec<f64>)>,
    pub(crate) zone_conduction_rates: Vec<ZoneConductionTrace>,
    pub(crate) inside_surface_iteration_counts: Vec<f64>,
    pub(crate) zone_air_heat_balance_rates:
        Vec<(ep_model::ZoneId, String, Vec<f64>, Vec<f64>, Vec<f64>)>,
    pub(crate) zone_air_debug_traces: Vec<ZoneAirDebugTrace>,
    pub(crate) surface_temperatures: Vec<SurfaceHeatBalanceTrace>,
    pub(crate) outdoor_temperatures: Vec<f64>,
    pub(crate) outdoor_wet_bulb_temperatures: Vec<f64>,
    pub(crate) sky_temperatures: Vec<f64>,
    pub(crate) horizontal_infrared_radiation_rates: Vec<f64>,
    pub(crate) rain_statuses: Vec<f64>,
    pub(crate) first_sample_ctf_history_slot_accumulators:
        BTreeMap<(String, usize), HeatBalanceCtfHistorySlotFirstSampleAccumulator>,
    pub(crate) hourly_ctf_history_slots: Vec<HeatBalanceCtfHistorySlotHourlySample>,
    pub(crate) hourly_ctf_history_slots_after_advance: Vec<HeatBalanceCtfHistorySlotHourlySample>,
    pub(crate) surface_first_sample_trace: Vec<HeatBalanceSurfaceFirstSampleTrace>,
    pub(crate) zone_air_first_sample_trace: Vec<HeatBalanceZoneAirFirstSampleTrace>,
    pub(crate) surface_iteration_first_sample_trace:
        Vec<HeatBalanceSurfaceIterationFirstSampleTrace>,
    pub(crate) surface_iteration_sample_trace: Vec<HeatBalanceSurfaceIterationSampleTrace>,
}

pub(crate) struct SurfaceHeatBalanceTrace {
    pub(crate) surface_id: ep_model::SurfaceId,
    pub(crate) surface_name: String,
    pub(crate) inside_face_temperature_c: Vec<f64>,
    pub(crate) inside_adjacent_air_temperature_c: Vec<f64>,
    pub(crate) outside_face_temperature_c: Vec<f64>,
    pub(crate) outside_outdoor_air_dry_bulb_temperature_c: Vec<f64>,
    pub(crate) outside_outdoor_air_wet_bulb_temperature_c: Vec<f64>,
    pub(crate) outside_outdoor_air_wind_speed_m_per_s: Vec<f64>,
    pub(crate) outside_outdoor_air_wind_direction_deg: Vec<f64>,
    pub(crate) inside_convection_heat_gain_rate_w: Vec<f64>,
    pub(crate) inside_convection_heat_gain_rate_per_area_w_per_m2: Vec<f64>,
    pub(crate) inside_convection_coefficient_w_per_m2_k: Vec<f64>,
    pub(crate) inside_net_surface_thermal_radiation_heat_gain_rate_w: Vec<f64>,
    pub(crate) inside_net_surface_thermal_radiation_heat_gain_rate_per_area_w_per_m2: Vec<f64>,
    pub(crate) inside_radiant_internal_gain_source_term_rate_w: Vec<f64>,
    pub(crate) inside_radiant_internal_gain_source_term_rate_per_area_w_per_m2: Vec<f64>,
    pub(crate) inside_shortwave_absorbed_source_term_rate_w: Vec<f64>,
    pub(crate) inside_shortwave_absorbed_source_term_rate_per_area_w_per_m2: Vec<f64>,
    pub(crate) inside_additional_heat_source_term_rate_w: Vec<f64>,
    pub(crate) inside_additional_heat_source_term_rate_per_area_w_per_m2: Vec<f64>,
    pub(crate) inside_radiant_hvac_source_term_rate_w: Vec<f64>,
    pub(crate) inside_radiant_hvac_source_term_rate_per_area_w_per_m2: Vec<f64>,
    pub(crate) inside_total_source_term_rate_w: Vec<f64>,
    pub(crate) inside_total_source_term_rate_per_area_w_per_m2: Vec<f64>,
    pub(crate) outside_convection_heat_gain_rate_w: Vec<f64>,
    pub(crate) outside_convection_heat_gain_rate_per_area_w_per_m2: Vec<f64>,
    pub(crate) outside_convection_coefficient_w_per_m2_k: Vec<f64>,
    pub(crate) outside_net_thermal_radiation_heat_gain_rate_w: Vec<f64>,
    pub(crate) outside_net_thermal_radiation_heat_gain_rate_per_area_w_per_m2: Vec<f64>,
    pub(crate) outside_thermal_radiation_to_air_coefficient_w_per_m2_k: Vec<f64>,
    pub(crate) outside_thermal_radiation_to_sky_coefficient_w_per_m2_k: Vec<f64>,
    pub(crate) outside_thermal_radiation_to_ground_coefficient_w_per_m2_k: Vec<f64>,
    pub(crate) outside_solar_radiation_heat_gain_rate_w: Vec<f64>,
    pub(crate) outside_solar_radiation_heat_gain_rate_per_area_w_per_m2: Vec<f64>,
    pub(crate) outside_balance_report_temperature_c: Vec<f64>,
    pub(crate) outside_balance_coefficient_temperature_c: Vec<f64>,
    pub(crate) outside_balance_convection_reference_temperature_c: Vec<f64>,
    pub(crate) outside_balance_equivalent_radiant_temperature_c: Vec<f64>,
    pub(crate) outside_balance_radiation_coefficient_w_per_m2_k: Vec<f64>,
    pub(crate) outside_quick_balance_inside_source_term_w_per_m2: Vec<f64>,
    pub(crate) outside_quick_balance_inside_balance_term_w_per_m2: Vec<f64>,
    pub(crate) outside_quick_balance_numerator_w_per_m2: Vec<f64>,
    pub(crate) outside_quick_balance_denominator_w_per_m2_k: Vec<f64>,
    pub(crate) outside_quick_balance_coupling_factor: Vec<f64>,
    pub(crate) inside_conduction_rate_w: Vec<f64>,
    pub(crate) inside_conduction_gain_rate_w: Vec<f64>,
    pub(crate) inside_conduction_loss_rate_w: Vec<f64>,
    pub(crate) inside_conduction_rate_per_area_w_per_m2: Vec<f64>,
    pub(crate) ctf_inside_current_outside_term_rate_w: Vec<f64>,
    pub(crate) ctf_inside_current_inside_term_rate_w: Vec<f64>,
    pub(crate) ctf_inside_history_term_rate_w: Vec<f64>,
    pub(crate) ctf_inside_history_temperature_term_rate_w: Vec<f64>,
    pub(crate) ctf_inside_history_flux_term_rate_w: Vec<f64>,
    pub(crate) outside_conduction_rate_w: Vec<f64>,
    pub(crate) outside_conduction_gain_rate_w: Vec<f64>,
    pub(crate) outside_conduction_loss_rate_w: Vec<f64>,
    pub(crate) outside_conduction_rate_per_area_w_per_m2: Vec<f64>,
    pub(crate) ctf_outside_current_outside_term_rate_w: Vec<f64>,
    pub(crate) ctf_outside_current_inside_term_rate_w: Vec<f64>,
    pub(crate) ctf_outside_history_term_rate_w: Vec<f64>,
    pub(crate) heat_storage_rate_w: Vec<f64>,
    pub(crate) heat_storage_rate_per_area_w_per_m2: Vec<f64>,
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct SurfaceHeatBalanceTraceSums {
    pub(crate) inside_face_temperature_c: f64,
    pub(crate) inside_adjacent_air_temperature_c: f64,
    pub(crate) outside_face_temperature_c: f64,
    pub(crate) outside_outdoor_air_dry_bulb_temperature_c: f64,
    pub(crate) outside_outdoor_air_wet_bulb_temperature_c: f64,
    pub(crate) outside_outdoor_air_wind_speed_m_per_s: f64,
    pub(crate) outside_outdoor_air_wind_direction_deg: f64,
    pub(crate) inside_convection_heat_gain_rate_w: f64,
    pub(crate) inside_convection_heat_gain_rate_per_area_w_per_m2: f64,
    pub(crate) inside_convection_coefficient_w_per_m2_k: f64,
    pub(crate) inside_net_surface_thermal_radiation_heat_gain_rate_w: f64,
    pub(crate) inside_net_surface_thermal_radiation_heat_gain_rate_per_area_w_per_m2: f64,
    pub(crate) inside_radiant_internal_gain_source_term_rate_w: f64,
    pub(crate) inside_radiant_internal_gain_source_term_rate_per_area_w_per_m2: f64,
    pub(crate) inside_shortwave_absorbed_source_term_rate_w: f64,
    pub(crate) inside_shortwave_absorbed_source_term_rate_per_area_w_per_m2: f64,
    pub(crate) inside_additional_heat_source_term_rate_w: f64,
    pub(crate) inside_additional_heat_source_term_rate_per_area_w_per_m2: f64,
    pub(crate) inside_radiant_hvac_source_term_rate_w: f64,
    pub(crate) inside_radiant_hvac_source_term_rate_per_area_w_per_m2: f64,
    pub(crate) inside_total_source_term_rate_w: f64,
    pub(crate) inside_total_source_term_rate_per_area_w_per_m2: f64,
    pub(crate) outside_convection_heat_gain_rate_w: f64,
    pub(crate) outside_convection_heat_gain_rate_per_area_w_per_m2: f64,
    pub(crate) outside_convection_coefficient_w_per_m2_k: f64,
    pub(crate) outside_net_thermal_radiation_heat_gain_rate_w: f64,
    pub(crate) outside_net_thermal_radiation_heat_gain_rate_per_area_w_per_m2: f64,
    pub(crate) outside_thermal_radiation_to_air_coefficient_w_per_m2_k: f64,
    pub(crate) outside_thermal_radiation_to_sky_coefficient_w_per_m2_k: f64,
    pub(crate) outside_thermal_radiation_to_ground_coefficient_w_per_m2_k: f64,
    pub(crate) outside_solar_radiation_heat_gain_rate_w: f64,
    pub(crate) outside_solar_radiation_heat_gain_rate_per_area_w_per_m2: f64,
    pub(crate) outside_balance_report_temperature_c: f64,
    pub(crate) outside_balance_coefficient_temperature_c: f64,
    pub(crate) outside_balance_convection_reference_temperature_c: f64,
    pub(crate) outside_balance_equivalent_radiant_temperature_c: f64,
    pub(crate) outside_balance_radiation_coefficient_w_per_m2_k: f64,
    pub(crate) outside_quick_balance_inside_source_term_w_per_m2: f64,
    pub(crate) outside_quick_balance_inside_balance_term_w_per_m2: f64,
    pub(crate) outside_quick_balance_numerator_w_per_m2: f64,
    pub(crate) outside_quick_balance_denominator_w_per_m2_k: f64,
    pub(crate) outside_quick_balance_coupling_factor: f64,
    pub(crate) inside_conduction_rate_w: f64,
    pub(crate) inside_conduction_gain_rate_w: f64,
    pub(crate) inside_conduction_loss_rate_w: f64,
    pub(crate) inside_conduction_rate_per_area_w_per_m2: f64,
    pub(crate) ctf_inside_current_outside_term_rate_w: f64,
    pub(crate) ctf_inside_current_inside_term_rate_w: f64,
    pub(crate) ctf_inside_history_term_rate_w: f64,
    pub(crate) ctf_inside_history_temperature_term_rate_w: f64,
    pub(crate) ctf_inside_history_flux_term_rate_w: f64,
    pub(crate) outside_conduction_rate_w: f64,
    pub(crate) outside_conduction_gain_rate_w: f64,
    pub(crate) outside_conduction_loss_rate_w: f64,
    pub(crate) outside_conduction_rate_per_area_w_per_m2: f64,
    pub(crate) ctf_outside_current_outside_term_rate_w: f64,
    pub(crate) ctf_outside_current_inside_term_rate_w: f64,
    pub(crate) ctf_outside_history_term_rate_w: f64,
    pub(crate) heat_storage_rate_w: f64,
    pub(crate) heat_storage_rate_per_area_w_per_m2: f64,
}

#[derive(Clone, Debug)]
pub(crate) struct HeatBalanceCtfHistorySlotFirstSampleAccumulator {
    surface_name: String,
    construction_name: String,
    slot_index: usize,
    area_m2: f64,
    timestep_count: usize,
    outside_history_coefficient_w_per_m2_k: f64,
    cross_history_coefficient_w_per_m2_k: f64,
    inside_history_coefficient_w_per_m2_k: f64,
    flux_history_coefficient: f64,
    outside_temperature_history_c: f64,
    inside_temperature_history_c: f64,
    outside_flux_history_w_per_m2: f64,
    inside_flux_history_w_per_m2: f64,
    inside_temperature_term_w: f64,
    inside_flux_term_w: f64,
    inside_total_term_w: f64,
    outside_temperature_term_w: f64,
    outside_flux_term_w: f64,
    outside_total_term_w: f64,
}

impl HeatBalanceCtfHistorySlotFirstSampleAccumulator {
    pub(crate) fn from_sample(sample: &HeatBalanceCtfHistorySlotSample) -> Self {
        Self {
            surface_name: sample.surface_name.clone(),
            construction_name: sample.construction_name.clone(),
            slot_index: sample.slot_index,
            area_m2: sample.area_m2,
            timestep_count: 0,
            outside_history_coefficient_w_per_m2_k: sample.outside_history_coefficient_w_per_m2_k,
            cross_history_coefficient_w_per_m2_k: sample.cross_history_coefficient_w_per_m2_k,
            inside_history_coefficient_w_per_m2_k: sample.inside_history_coefficient_w_per_m2_k,
            flux_history_coefficient: sample.flux_history_coefficient,
            outside_temperature_history_c: 0.0,
            inside_temperature_history_c: 0.0,
            outside_flux_history_w_per_m2: 0.0,
            inside_flux_history_w_per_m2: 0.0,
            inside_temperature_term_w: 0.0,
            inside_flux_term_w: 0.0,
            inside_total_term_w: 0.0,
            outside_temperature_term_w: 0.0,
            outside_flux_term_w: 0.0,
            outside_total_term_w: 0.0,
        }
    }

    pub(crate) fn push(&mut self, sample: &HeatBalanceCtfHistorySlotSample) {
        self.timestep_count += 1;
        self.outside_temperature_history_c += sample.outside_temperature_history_c;
        self.inside_temperature_history_c += sample.inside_temperature_history_c;
        self.outside_flux_history_w_per_m2 += sample.outside_flux_history_w_per_m2;
        self.inside_flux_history_w_per_m2 += sample.inside_flux_history_w_per_m2;
        self.inside_temperature_term_w += sample.inside_temperature_term_w;
        self.inside_flux_term_w += sample.inside_flux_term_w;
        self.inside_total_term_w += sample.inside_total_term_w;
        self.outside_temperature_term_w += sample.outside_temperature_term_w;
        self.outside_flux_term_w += sample.outside_flux_term_w;
        self.outside_total_term_w += sample.outside_total_term_w;
    }

    pub(crate) fn finalize(self) -> HeatBalanceCtfHistorySlotFirstSample {
        let divisor = self.timestep_count.max(1) as f64;
        HeatBalanceCtfHistorySlotFirstSample {
            surface_name: self.surface_name,
            construction_name: self.construction_name,
            slot_index: self.slot_index,
            area_m2: self.area_m2,
            timestep_count: self.timestep_count,
            outside_history_coefficient_w_per_m2_k: self.outside_history_coefficient_w_per_m2_k,
            cross_history_coefficient_w_per_m2_k: self.cross_history_coefficient_w_per_m2_k,
            inside_history_coefficient_w_per_m2_k: self.inside_history_coefficient_w_per_m2_k,
            flux_history_coefficient: self.flux_history_coefficient,
            outside_temperature_history_c: self.outside_temperature_history_c / divisor,
            inside_temperature_history_c: self.inside_temperature_history_c / divisor,
            outside_flux_history_w_per_m2: self.outside_flux_history_w_per_m2 / divisor,
            inside_flux_history_w_per_m2: self.inside_flux_history_w_per_m2 / divisor,
            inside_temperature_term_w: self.inside_temperature_term_w / divisor,
            inside_flux_term_w: self.inside_flux_term_w / divisor,
            inside_total_term_w: self.inside_total_term_w / divisor,
            outside_temperature_term_w: self.outside_temperature_term_w / divisor,
            outside_flux_term_w: self.outside_flux_term_w / divisor,
            outside_total_term_w: self.outside_total_term_w / divisor,
        }
    }

    pub(crate) fn finalize_hourly(
        self,
        sample_index: usize,
    ) -> HeatBalanceCtfHistorySlotHourlySample {
        let divisor = self.timestep_count.max(1) as f64;
        HeatBalanceCtfHistorySlotHourlySample {
            sample_index,
            surface_name: self.surface_name,
            construction_name: self.construction_name,
            slot_index: self.slot_index,
            area_m2: self.area_m2,
            timestep_count: self.timestep_count,
            outside_history_coefficient_w_per_m2_k: self.outside_history_coefficient_w_per_m2_k,
            cross_history_coefficient_w_per_m2_k: self.cross_history_coefficient_w_per_m2_k,
            inside_history_coefficient_w_per_m2_k: self.inside_history_coefficient_w_per_m2_k,
            flux_history_coefficient: self.flux_history_coefficient,
            outside_temperature_history_c: self.outside_temperature_history_c / divisor,
            inside_temperature_history_c: self.inside_temperature_history_c / divisor,
            outside_flux_history_w_per_m2: self.outside_flux_history_w_per_m2 / divisor,
            inside_flux_history_w_per_m2: self.inside_flux_history_w_per_m2 / divisor,
            inside_temperature_term_w: self.inside_temperature_term_w / divisor,
            inside_flux_term_w: self.inside_flux_term_w / divisor,
            inside_total_term_w: self.inside_total_term_w / divisor,
            outside_temperature_term_w: self.outside_temperature_term_w / divisor,
            outside_flux_term_w: self.outside_flux_term_w / divisor,
            outside_total_term_w: self.outside_total_term_w / divisor,
        }
    }
}

pub(crate) struct ZoneConductionTrace {
    pub(crate) zone_id: ZoneId,
    pub(crate) zone_name: String,
    pub(crate) inside_conduction_rate_w: Vec<f64>,
    pub(crate) inside_conduction_gain_rate_w: Vec<f64>,
    pub(crate) inside_conduction_loss_rate_w: Vec<f64>,
    pub(crate) outside_conduction_rate_w: Vec<f64>,
    pub(crate) outside_conduction_gain_rate_w: Vec<f64>,
    pub(crate) outside_conduction_loss_rate_w: Vec<f64>,
}

pub(crate) struct ZoneAirDebugTrace {
    pub(crate) zone_id: ZoneId,
    pub(crate) zone_name: String,
    pub(crate) current_temperature_c: Vec<f64>,
    pub(crate) zone_timestep_average_temperature_c: Vec<f64>,
    pub(crate) previous_temperature_1_c: Vec<f64>,
    pub(crate) previous_temperature_2_c: Vec<f64>,
    pub(crate) previous_temperature_3_c: Vec<f64>,
    pub(crate) previous_system_temperature_1_c: Vec<f64>,
    pub(crate) system_timestep_count: Vec<f64>,
    pub(crate) humidity_ratio: Vec<f64>,
    pub(crate) zone_timestep_average_humidity_ratio: Vec<f64>,
    pub(crate) air_heat_capacity_j_per_k: Vec<f64>,
    pub(crate) zone_timestep_air_power_cap_w_per_k: Vec<f64>,
    pub(crate) last_correction_air_power_cap_w_per_k: Vec<f64>,
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct ZoneAirDebugTraceSums {
    pub(crate) current_temperature_c: f64,
    pub(crate) zone_timestep_average_temperature_c: f64,
    pub(crate) previous_temperature_1_c: f64,
    pub(crate) previous_temperature_2_c: f64,
    pub(crate) previous_temperature_3_c: f64,
    pub(crate) previous_system_temperature_1_c: f64,
    pub(crate) system_timestep_count: f64,
    pub(crate) humidity_ratio: f64,
    pub(crate) zone_timestep_average_humidity_ratio: f64,
    pub(crate) air_heat_capacity_j_per_k: f64,
    pub(crate) zone_timestep_air_power_cap_w_per_k: f64,
    pub(crate) last_correction_air_power_cap_w_per_k: f64,
}

pub(crate) fn heat_balance_zone_air_state_sample(
    zone: &ZoneHeatBalanceState,
) -> HeatBalanceZoneAirStateSample {
    HeatBalanceZoneAirStateSample {
        zone_id: zone.zone_id,
        zone_name: zone.zone_name.clone(),
        mean_air_temperature_c: zone.mean_air_temperature_c,
        zone_timestep_average_air_temperature_c: zone.zone_timestep_average_air_temperature_c,
        previous_mean_air_temperatures_c: zone.previous_mean_air_temperatures_c,
        previous_system_mean_air_temperatures_c: zone.previous_system_mean_air_temperatures_c,
        previous_system_timestep_count: zone.previous_system_timestep_count,
        air_humidity_ratio: zone.air_humidity_ratio,
        zone_timestep_average_air_humidity_ratio: zone.zone_timestep_average_air_humidity_ratio,
        previous_air_humidity_ratios: zone.previous_air_humidity_ratios,
        previous_system_air_humidity_ratios: zone.previous_system_air_humidity_ratios,
        use_zone_timestep_history: zone.use_zone_timestep_history,
        shorten_timestep_sys: zone.shorten_timestep_sys,
        prior_timestep_seconds: zone.prior_timestep_seconds,
        air_heat_capacity_j_per_k: zone.air_heat_capacity_j_per_k,
        sum_mcp_w_per_k: zone.sum_mcp_w_per_k,
        sum_mcp_t_w: zone.sum_mcp_t_w,
        sum_sys_mcp_w_per_k: zone.sum_sys_mcp_w_per_k,
        sum_sys_mcp_t_w: zone.sum_sys_mcp_t_w,
        zone_air_temperature_coefficients: zone.zone_air_temperature_coefficients,
    }
}

pub(crate) fn zone_scalar_trace_series_from_state(
    state: &HeatBalanceState,
    sample_count: usize,
) -> Vec<(ZoneId, String, Vec<f64>)> {
    state
        .zones
        .iter()
        .map(|zone| {
            (
                zone.zone_id,
                zone.zone_name.clone(),
                Vec::with_capacity(sample_count),
            )
        })
        .collect()
}

pub(crate) fn zone_conduction_traces_from_state(
    state: &HeatBalanceState,
    sample_count: usize,
) -> Vec<ZoneConductionTrace> {
    state
        .zones
        .iter()
        .map(|zone| ZoneConductionTrace {
            zone_id: zone.zone_id,
            zone_name: zone.zone_name.clone(),
            inside_conduction_rate_w: Vec::with_capacity(sample_count),
            inside_conduction_gain_rate_w: Vec::with_capacity(sample_count),
            inside_conduction_loss_rate_w: Vec::with_capacity(sample_count),
            outside_conduction_rate_w: Vec::with_capacity(sample_count),
            outside_conduction_gain_rate_w: Vec::with_capacity(sample_count),
            outside_conduction_loss_rate_w: Vec::with_capacity(sample_count),
        })
        .collect()
}

pub(crate) fn zone_air_heat_balance_trace_series_from_state(
    state: &HeatBalanceState,
    sample_count: usize,
) -> Vec<(ZoneId, String, Vec<f64>, Vec<f64>, Vec<f64>)> {
    state
        .zones
        .iter()
        .map(|zone| {
            (
                zone.zone_id,
                zone.zone_name.clone(),
                Vec::with_capacity(sample_count),
                Vec::with_capacity(sample_count),
                Vec::with_capacity(sample_count),
            )
        })
        .collect()
}

pub(crate) fn zone_air_debug_traces_from_state(
    state: &HeatBalanceState,
    sample_count: usize,
) -> Vec<ZoneAirDebugTrace> {
    state
        .zones
        .iter()
        .map(|zone| ZoneAirDebugTrace {
            zone_id: zone.zone_id,
            zone_name: zone.zone_name.clone(),
            current_temperature_c: Vec::with_capacity(sample_count),
            zone_timestep_average_temperature_c: Vec::with_capacity(sample_count),
            previous_temperature_1_c: Vec::with_capacity(sample_count),
            previous_temperature_2_c: Vec::with_capacity(sample_count),
            previous_temperature_3_c: Vec::with_capacity(sample_count),
            previous_system_temperature_1_c: Vec::with_capacity(sample_count),
            system_timestep_count: Vec::with_capacity(sample_count),
            humidity_ratio: Vec::with_capacity(sample_count),
            zone_timestep_average_humidity_ratio: Vec::with_capacity(sample_count),
            air_heat_capacity_j_per_k: Vec::with_capacity(sample_count),
            zone_timestep_air_power_cap_w_per_k: Vec::with_capacity(sample_count),
            last_correction_air_power_cap_w_per_k: Vec::with_capacity(sample_count),
        })
        .collect()
}

pub(crate) fn surface_heat_balance_traces_from_state(
    state: &HeatBalanceState,
    sample_count: usize,
) -> Vec<SurfaceHeatBalanceTrace> {
    state
        .surfaces
        .iter()
        .map(|surface| SurfaceHeatBalanceTrace {
            surface_id: surface.surface_id,
            surface_name: surface.surface_name.clone(),
            inside_face_temperature_c: Vec::with_capacity(sample_count),
            inside_adjacent_air_temperature_c: Vec::with_capacity(sample_count),
            outside_face_temperature_c: Vec::with_capacity(sample_count),
            outside_outdoor_air_dry_bulb_temperature_c: Vec::with_capacity(sample_count),
            outside_outdoor_air_wet_bulb_temperature_c: Vec::with_capacity(sample_count),
            outside_outdoor_air_wind_speed_m_per_s: Vec::with_capacity(sample_count),
            outside_outdoor_air_wind_direction_deg: Vec::with_capacity(sample_count),
            inside_convection_heat_gain_rate_w: Vec::with_capacity(sample_count),
            inside_convection_heat_gain_rate_per_area_w_per_m2: Vec::with_capacity(sample_count),
            inside_convection_coefficient_w_per_m2_k: Vec::with_capacity(sample_count),
            inside_net_surface_thermal_radiation_heat_gain_rate_w: Vec::with_capacity(sample_count),
            inside_net_surface_thermal_radiation_heat_gain_rate_per_area_w_per_m2:
                Vec::with_capacity(sample_count),
            inside_radiant_internal_gain_source_term_rate_w: Vec::with_capacity(sample_count),
            inside_radiant_internal_gain_source_term_rate_per_area_w_per_m2: Vec::with_capacity(
                sample_count,
            ),
            inside_shortwave_absorbed_source_term_rate_w: Vec::with_capacity(sample_count),
            inside_shortwave_absorbed_source_term_rate_per_area_w_per_m2: Vec::with_capacity(
                sample_count,
            ),
            inside_additional_heat_source_term_rate_w: Vec::with_capacity(sample_count),
            inside_additional_heat_source_term_rate_per_area_w_per_m2: Vec::with_capacity(
                sample_count,
            ),
            inside_radiant_hvac_source_term_rate_w: Vec::with_capacity(sample_count),
            inside_radiant_hvac_source_term_rate_per_area_w_per_m2: Vec::with_capacity(
                sample_count,
            ),
            inside_total_source_term_rate_w: Vec::with_capacity(sample_count),
            inside_total_source_term_rate_per_area_w_per_m2: Vec::with_capacity(sample_count),
            outside_convection_heat_gain_rate_w: Vec::with_capacity(sample_count),
            outside_convection_heat_gain_rate_per_area_w_per_m2: Vec::with_capacity(sample_count),
            outside_convection_coefficient_w_per_m2_k: Vec::with_capacity(sample_count),
            outside_net_thermal_radiation_heat_gain_rate_w: Vec::with_capacity(sample_count),
            outside_net_thermal_radiation_heat_gain_rate_per_area_w_per_m2: Vec::with_capacity(
                sample_count,
            ),
            outside_thermal_radiation_to_air_coefficient_w_per_m2_k: Vec::with_capacity(
                sample_count,
            ),
            outside_thermal_radiation_to_sky_coefficient_w_per_m2_k: Vec::with_capacity(
                sample_count,
            ),
            outside_thermal_radiation_to_ground_coefficient_w_per_m2_k: Vec::with_capacity(
                sample_count,
            ),
            outside_solar_radiation_heat_gain_rate_w: Vec::with_capacity(sample_count),
            outside_solar_radiation_heat_gain_rate_per_area_w_per_m2: Vec::with_capacity(
                sample_count,
            ),
            outside_balance_report_temperature_c: Vec::with_capacity(sample_count),
            outside_balance_coefficient_temperature_c: Vec::with_capacity(sample_count),
            outside_balance_convection_reference_temperature_c: Vec::with_capacity(sample_count),
            outside_balance_equivalent_radiant_temperature_c: Vec::with_capacity(sample_count),
            outside_balance_radiation_coefficient_w_per_m2_k: Vec::with_capacity(sample_count),
            outside_quick_balance_inside_source_term_w_per_m2: Vec::with_capacity(sample_count),
            outside_quick_balance_inside_balance_term_w_per_m2: Vec::with_capacity(sample_count),
            outside_quick_balance_numerator_w_per_m2: Vec::with_capacity(sample_count),
            outside_quick_balance_denominator_w_per_m2_k: Vec::with_capacity(sample_count),
            outside_quick_balance_coupling_factor: Vec::with_capacity(sample_count),
            inside_conduction_rate_w: Vec::with_capacity(sample_count),
            inside_conduction_gain_rate_w: Vec::with_capacity(sample_count),
            inside_conduction_loss_rate_w: Vec::with_capacity(sample_count),
            inside_conduction_rate_per_area_w_per_m2: Vec::with_capacity(sample_count),
            ctf_inside_current_outside_term_rate_w: Vec::with_capacity(sample_count),
            ctf_inside_current_inside_term_rate_w: Vec::with_capacity(sample_count),
            ctf_inside_history_term_rate_w: Vec::with_capacity(sample_count),
            ctf_inside_history_temperature_term_rate_w: Vec::with_capacity(sample_count),
            ctf_inside_history_flux_term_rate_w: Vec::with_capacity(sample_count),
            outside_conduction_rate_w: Vec::with_capacity(sample_count),
            outside_conduction_gain_rate_w: Vec::with_capacity(sample_count),
            outside_conduction_loss_rate_w: Vec::with_capacity(sample_count),
            outside_conduction_rate_per_area_w_per_m2: Vec::with_capacity(sample_count),
            ctf_outside_current_outside_term_rate_w: Vec::with_capacity(sample_count),
            ctf_outside_current_inside_term_rate_w: Vec::with_capacity(sample_count),
            ctf_outside_history_term_rate_w: Vec::with_capacity(sample_count),
            heat_storage_rate_w: Vec::with_capacity(sample_count),
            heat_storage_rate_per_area_w_per_m2: Vec::with_capacity(sample_count),
        })
        .collect()
}

pub(crate) fn push_zone_scalar_trace_averages(
    traces: &mut [(ZoneId, String, Vec<f64>)],
    sums: &[f64],
    divisor: f64,
) {
    for ((_, _, values), sum) in traces.iter_mut().zip(sums.iter().copied()) {
        values.push(sum / divisor);
    }
}

pub(crate) fn push_zone_conduction_trace_averages(
    traces: &mut [ZoneConductionTrace],
    sums: &[(f64, f64, f64, f64, f64, f64)],
    divisor: f64,
) {
    for (trace, sums) in traces.iter_mut().zip(sums.iter().copied()) {
        trace.inside_conduction_rate_w.push(sums.0 / divisor);
        trace.inside_conduction_gain_rate_w.push(sums.1 / divisor);
        trace.inside_conduction_loss_rate_w.push(sums.2 / divisor);
        trace.outside_conduction_rate_w.push(sums.3 / divisor);
        trace.outside_conduction_gain_rate_w.push(sums.4 / divisor);
        trace.outside_conduction_loss_rate_w.push(sums.5 / divisor);
    }
}

pub(crate) fn push_zone_air_heat_balance_trace_values(
    traces: &mut [(ZoneId, String, Vec<f64>, Vec<f64>, Vec<f64>)],
    sums: &[(f64, f64, f64)],
    last_values: &[(f64, f64, f64)],
    sampling: HeatBalanceZoneAirReportSampling,
    divisor: f64,
) {
    for (index, (_, _, internal_gain_values, surface_convection_values, air_storage_values)) in
        traces.iter_mut().enumerate()
    {
        let values = match sampling {
            HeatBalanceZoneAirReportSampling::Average => {
                let sums = sums[index];
                (sums.0 / divisor, sums.1 / divisor, sums.2 / divisor)
            }
            HeatBalanceZoneAirReportSampling::LastSystemState => last_values[index],
        };
        internal_gain_values.push(values.0);
        surface_convection_values.push(values.1);
        air_storage_values.push(values.2);
    }
}
pub(crate) fn push_zone_air_debug_trace_averages(
    traces: &mut [ZoneAirDebugTrace],
    sums: &[ZoneAirDebugTraceSums],
    divisor: f64,
) {
    for (trace, sums) in traces.iter_mut().zip(sums.iter().copied()) {
        trace
            .current_temperature_c
            .push(sums.current_temperature_c / divisor);
        trace
            .zone_timestep_average_temperature_c
            .push(sums.zone_timestep_average_temperature_c / divisor);
        trace
            .previous_temperature_1_c
            .push(sums.previous_temperature_1_c / divisor);
        trace
            .previous_temperature_2_c
            .push(sums.previous_temperature_2_c / divisor);
        trace
            .previous_temperature_3_c
            .push(sums.previous_temperature_3_c / divisor);
        trace
            .previous_system_temperature_1_c
            .push(sums.previous_system_temperature_1_c / divisor);
        trace
            .system_timestep_count
            .push(sums.system_timestep_count / divisor);
        trace.humidity_ratio.push(sums.humidity_ratio / divisor);
        trace
            .zone_timestep_average_humidity_ratio
            .push(sums.zone_timestep_average_humidity_ratio / divisor);
        trace
            .air_heat_capacity_j_per_k
            .push(sums.air_heat_capacity_j_per_k / divisor);
        trace
            .zone_timestep_air_power_cap_w_per_k
            .push(sums.zone_timestep_air_power_cap_w_per_k / divisor);
        trace
            .last_correction_air_power_cap_w_per_k
            .push(sums.last_correction_air_power_cap_w_per_k / divisor);
    }
}

pub(crate) fn push_surface_heat_balance_trace_averages(
    traces: &mut [SurfaceHeatBalanceTrace],
    sums: &[SurfaceHeatBalanceTraceSums],
    divisor: f64,
) {
    for (trace, sums) in traces.iter_mut().zip(sums.iter().copied()) {
        trace
            .inside_face_temperature_c
            .push(sums.inside_face_temperature_c / divisor);
        trace
            .inside_adjacent_air_temperature_c
            .push(sums.inside_adjacent_air_temperature_c / divisor);
        trace
            .outside_face_temperature_c
            .push(sums.outside_face_temperature_c / divisor);
        trace
            .outside_outdoor_air_dry_bulb_temperature_c
            .push(sums.outside_outdoor_air_dry_bulb_temperature_c / divisor);
        trace
            .outside_outdoor_air_wet_bulb_temperature_c
            .push(sums.outside_outdoor_air_wet_bulb_temperature_c / divisor);
        trace
            .outside_outdoor_air_wind_speed_m_per_s
            .push(sums.outside_outdoor_air_wind_speed_m_per_s / divisor);
        trace
            .outside_outdoor_air_wind_direction_deg
            .push(sums.outside_outdoor_air_wind_direction_deg / divisor);
        trace
            .inside_convection_heat_gain_rate_w
            .push(sums.inside_convection_heat_gain_rate_w / divisor);
        trace
            .inside_convection_heat_gain_rate_per_area_w_per_m2
            .push(sums.inside_convection_heat_gain_rate_per_area_w_per_m2 / divisor);
        trace
            .inside_convection_coefficient_w_per_m2_k
            .push(sums.inside_convection_coefficient_w_per_m2_k / divisor);
        trace
            .inside_net_surface_thermal_radiation_heat_gain_rate_w
            .push(sums.inside_net_surface_thermal_radiation_heat_gain_rate_w / divisor);
        trace
            .inside_net_surface_thermal_radiation_heat_gain_rate_per_area_w_per_m2
            .push(
                sums.inside_net_surface_thermal_radiation_heat_gain_rate_per_area_w_per_m2
                    / divisor,
            );
        trace
            .inside_radiant_internal_gain_source_term_rate_w
            .push(sums.inside_radiant_internal_gain_source_term_rate_w / divisor);
        trace
            .inside_radiant_internal_gain_source_term_rate_per_area_w_per_m2
            .push(sums.inside_radiant_internal_gain_source_term_rate_per_area_w_per_m2 / divisor);
        trace
            .inside_shortwave_absorbed_source_term_rate_w
            .push(sums.inside_shortwave_absorbed_source_term_rate_w / divisor);
        trace
            .inside_shortwave_absorbed_source_term_rate_per_area_w_per_m2
            .push(sums.inside_shortwave_absorbed_source_term_rate_per_area_w_per_m2 / divisor);
        trace
            .inside_additional_heat_source_term_rate_w
            .push(sums.inside_additional_heat_source_term_rate_w / divisor);
        trace
            .inside_additional_heat_source_term_rate_per_area_w_per_m2
            .push(sums.inside_additional_heat_source_term_rate_per_area_w_per_m2 / divisor);
        trace
            .inside_radiant_hvac_source_term_rate_w
            .push(sums.inside_radiant_hvac_source_term_rate_w / divisor);
        trace
            .inside_radiant_hvac_source_term_rate_per_area_w_per_m2
            .push(sums.inside_radiant_hvac_source_term_rate_per_area_w_per_m2 / divisor);
        trace
            .inside_total_source_term_rate_w
            .push(sums.inside_total_source_term_rate_w / divisor);
        trace
            .inside_total_source_term_rate_per_area_w_per_m2
            .push(sums.inside_total_source_term_rate_per_area_w_per_m2 / divisor);
        trace
            .outside_convection_heat_gain_rate_w
            .push(sums.outside_convection_heat_gain_rate_w / divisor);
        trace
            .outside_convection_heat_gain_rate_per_area_w_per_m2
            .push(sums.outside_convection_heat_gain_rate_per_area_w_per_m2 / divisor);
        trace
            .outside_convection_coefficient_w_per_m2_k
            .push(sums.outside_convection_coefficient_w_per_m2_k / divisor);
        trace
            .outside_net_thermal_radiation_heat_gain_rate_w
            .push(sums.outside_net_thermal_radiation_heat_gain_rate_w / divisor);
        trace
            .outside_net_thermal_radiation_heat_gain_rate_per_area_w_per_m2
            .push(sums.outside_net_thermal_radiation_heat_gain_rate_per_area_w_per_m2 / divisor);
        trace
            .outside_thermal_radiation_to_air_coefficient_w_per_m2_k
            .push(sums.outside_thermal_radiation_to_air_coefficient_w_per_m2_k / divisor);
        trace
            .outside_thermal_radiation_to_sky_coefficient_w_per_m2_k
            .push(sums.outside_thermal_radiation_to_sky_coefficient_w_per_m2_k / divisor);
        trace
            .outside_thermal_radiation_to_ground_coefficient_w_per_m2_k
            .push(sums.outside_thermal_radiation_to_ground_coefficient_w_per_m2_k / divisor);
        trace
            .outside_solar_radiation_heat_gain_rate_w
            .push(sums.outside_solar_radiation_heat_gain_rate_w / divisor);
        trace
            .outside_solar_radiation_heat_gain_rate_per_area_w_per_m2
            .push(sums.outside_solar_radiation_heat_gain_rate_per_area_w_per_m2 / divisor);
        trace
            .outside_balance_report_temperature_c
            .push(sums.outside_balance_report_temperature_c / divisor);
        trace
            .outside_balance_coefficient_temperature_c
            .push(sums.outside_balance_coefficient_temperature_c / divisor);
        trace
            .outside_balance_convection_reference_temperature_c
            .push(sums.outside_balance_convection_reference_temperature_c / divisor);
        trace
            .outside_balance_equivalent_radiant_temperature_c
            .push(sums.outside_balance_equivalent_radiant_temperature_c / divisor);
        trace
            .outside_balance_radiation_coefficient_w_per_m2_k
            .push(sums.outside_balance_radiation_coefficient_w_per_m2_k / divisor);
        trace
            .outside_quick_balance_inside_source_term_w_per_m2
            .push(sums.outside_quick_balance_inside_source_term_w_per_m2 / divisor);
        trace
            .outside_quick_balance_inside_balance_term_w_per_m2
            .push(sums.outside_quick_balance_inside_balance_term_w_per_m2 / divisor);
        trace
            .outside_quick_balance_numerator_w_per_m2
            .push(sums.outside_quick_balance_numerator_w_per_m2 / divisor);
        trace
            .outside_quick_balance_denominator_w_per_m2_k
            .push(sums.outside_quick_balance_denominator_w_per_m2_k / divisor);
        trace
            .outside_quick_balance_coupling_factor
            .push(sums.outside_quick_balance_coupling_factor / divisor);
        trace
            .inside_conduction_rate_w
            .push(sums.inside_conduction_rate_w / divisor);
        trace
            .inside_conduction_gain_rate_w
            .push(sums.inside_conduction_gain_rate_w / divisor);
        trace
            .inside_conduction_loss_rate_w
            .push(sums.inside_conduction_loss_rate_w / divisor);
        trace
            .inside_conduction_rate_per_area_w_per_m2
            .push(sums.inside_conduction_rate_per_area_w_per_m2 / divisor);
        trace
            .ctf_inside_current_outside_term_rate_w
            .push(sums.ctf_inside_current_outside_term_rate_w / divisor);
        trace
            .ctf_inside_current_inside_term_rate_w
            .push(sums.ctf_inside_current_inside_term_rate_w / divisor);
        trace
            .ctf_inside_history_term_rate_w
            .push(sums.ctf_inside_history_term_rate_w / divisor);
        trace
            .ctf_inside_history_temperature_term_rate_w
            .push(sums.ctf_inside_history_temperature_term_rate_w / divisor);
        trace
            .ctf_inside_history_flux_term_rate_w
            .push(sums.ctf_inside_history_flux_term_rate_w / divisor);
        trace
            .outside_conduction_rate_w
            .push(sums.outside_conduction_rate_w / divisor);
        trace
            .outside_conduction_gain_rate_w
            .push(sums.outside_conduction_gain_rate_w / divisor);
        trace
            .outside_conduction_loss_rate_w
            .push(sums.outside_conduction_loss_rate_w / divisor);
        trace
            .outside_conduction_rate_per_area_w_per_m2
            .push(sums.outside_conduction_rate_per_area_w_per_m2 / divisor);
        trace
            .ctf_outside_current_outside_term_rate_w
            .push(sums.ctf_outside_current_outside_term_rate_w / divisor);
        trace
            .ctf_outside_current_inside_term_rate_w
            .push(sums.ctf_outside_current_inside_term_rate_w / divisor);
        trace
            .ctf_outside_history_term_rate_w
            .push(sums.ctf_outside_history_term_rate_w / divisor);
        trace
            .heat_storage_rate_w
            .push(sums.heat_storage_rate_w / divisor);
        trace
            .heat_storage_rate_per_area_w_per_m2
            .push(sums.heat_storage_rate_per_area_w_per_m2 / divisor);
    }
}
