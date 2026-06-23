//! Heat-balance trace accumulator value types.

use super::state::{
    HeatBalanceCtfHistorySlotFirstSample, HeatBalanceCtfHistorySlotHourlySample,
    HeatBalanceCtfHistorySlotSample, HeatBalanceZoneAirStateSample, ZoneHeatBalanceState,
};
use ep_model::ZoneId;

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
        air_heat_capacity_j_per_k: zone.air_heat_capacity_j_per_k,
        zone_air_temperature_coefficients: zone.zone_air_temperature_coefficients,
    }
}
