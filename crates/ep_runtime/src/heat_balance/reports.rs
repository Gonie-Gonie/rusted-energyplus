//! Heat-balance reporting source-order ownership notes.

use crate::diagnostics::*;
use crate::heat_balance::ctf::{
    surface_inside_conduction_rate_w_for_report, surface_outside_conduction_rate_w_for_report,
};
use crate::heat_balance::state::SurfaceHeatBalanceState;
use crate::heat_balance::trace::{SurfaceHeatBalanceTrace, ZoneAirDebugTrace, ZoneConductionTrace};
use crate::{OutputSeries, ResultStore};
use ep_model::{OutputHandle, ZoneId};

/// Source-order stage that writes zone heat-balance output rows.
pub const ZONE_REPORT_OWNER_STAGE: &str = "ReportHeatBalance";

/// Source-order stage that writes surface heat-balance output rows.
pub const SURFACE_REPORT_OWNER_STAGE: &str = "ReportSurfaceHeatBalance";

pub(crate) type ZoneScalarSeries = (ZoneId, String, Vec<f64>);
pub(crate) type ZoneAirHeatBalanceSeries = (ZoneId, String, Vec<f64>, Vec<f64>, Vec<f64>);

/// Diagnostic inside-face radiant internal gain source term rate variable name.
pub const SURFACE_INSIDE_RADIANT_INTERNAL_GAIN_SOURCE_TERM_RATE_VARIABLE: &str =
    "Surface Inside Face Radiant Internal Gain Source Term Rate";
/// Diagnostic inside-face radiant internal gain source term rate per-area variable name.
pub const SURFACE_INSIDE_RADIANT_INTERNAL_GAIN_SOURCE_TERM_RATE_PER_AREA_VARIABLE: &str =
    "Surface Inside Face Radiant Internal Gain Source Term Rate per Area";
/// Diagnostic inside-face absorbed shortwave source term rate variable name.
pub const SURFACE_INSIDE_SHORTWAVE_ABSORBED_SOURCE_TERM_RATE_VARIABLE: &str =
    "Surface Inside Face Shortwave Absorbed Source Term Rate";
/// Diagnostic inside-face absorbed shortwave source term rate per-area variable name.
pub const SURFACE_INSIDE_SHORTWAVE_ABSORBED_SOURCE_TERM_RATE_PER_AREA_VARIABLE: &str =
    "Surface Inside Face Shortwave Absorbed Source Term Rate per Area";
/// Diagnostic inside-face additional heat source term rate variable name.
pub const SURFACE_INSIDE_ADDITIONAL_HEAT_SOURCE_TERM_RATE_VARIABLE: &str =
    "Surface Inside Face Additional Heat Source Term Rate";
/// Diagnostic inside-face additional heat source term rate per-area variable name.
pub const SURFACE_INSIDE_ADDITIONAL_HEAT_SOURCE_TERM_RATE_PER_AREA_VARIABLE: &str =
    "Surface Inside Face Additional Heat Source Term Rate per Area";
/// Diagnostic inside-face radiant HVAC source term rate variable name.
pub const SURFACE_INSIDE_RADIANT_HVAC_SOURCE_TERM_RATE_VARIABLE: &str =
    "Surface Inside Face Radiant HVAC Source Term Rate";
/// Diagnostic inside-face radiant HVAC source term rate per-area variable name.
pub const SURFACE_INSIDE_RADIANT_HVAC_SOURCE_TERM_RATE_PER_AREA_VARIABLE: &str =
    "Surface Inside Face Radiant HVAC Source Term Rate per Area";
/// Diagnostic inside-face total non-convective source term rate variable name.
pub const SURFACE_INSIDE_TOTAL_SOURCE_TERM_RATE_VARIABLE: &str =
    "Surface Inside Face Total Source Term Rate";
/// Diagnostic inside-face total non-convective source term rate per-area variable name.
pub const SURFACE_INSIDE_TOTAL_SOURCE_TERM_RATE_PER_AREA_VARIABLE: &str =
    "Surface Inside Face Total Source Term Rate per Area";

pub(crate) struct HeatBalanceResultSeriesTraces {
    pub(crate) zone_temperatures: Vec<ZoneScalarSeries>,
    pub(crate) zone_humidity_ratios: Vec<ZoneScalarSeries>,
    pub(crate) zone_conduction_rates: Vec<ZoneConductionTrace>,
    pub(crate) inside_surface_iteration_counts: Vec<f64>,
    pub(crate) zone_air_heat_balance_rates: Vec<ZoneAirHeatBalanceSeries>,
    pub(crate) zone_air_debug_traces: Vec<ZoneAirDebugTrace>,
    pub(crate) surface_temperatures: Vec<SurfaceHeatBalanceTrace>,
    pub(crate) outdoor_temperatures: Vec<f64>,
    pub(crate) outdoor_wet_bulb_temperatures: Vec<f64>,
    pub(crate) sky_temperatures: Vec<f64>,
    pub(crate) horizontal_infrared_radiation_rates: Vec<f64>,
    pub(crate) rain_statuses: Vec<f64>,
}

pub(crate) fn zone_surface_report_conduction_rates_for_indices_w(
    surfaces: &[SurfaceHeatBalanceState],
    surface_indices: &[usize],
    use_inside_ctf_outside_temperature_for_conduction_report: bool,
) -> (f64, f64) {
    surface_indices
        .iter()
        .filter_map(|surface_index| surfaces.get(*surface_index))
        .map(|surface| {
            (
                surface_inside_conduction_rate_w_for_report(
                    surface,
                    use_inside_ctf_outside_temperature_for_conduction_report,
                ),
                surface_outside_conduction_rate_w_for_report(
                    surface,
                    use_inside_ctf_outside_temperature_for_conduction_report,
                ),
            )
        })
        .fold(
            (0.0, 0.0),
            |(inside_sum, outside_sum), (inside, outside)| {
                (inside_sum + inside, outside_sum + outside)
            },
        )
}

pub(crate) fn heat_gain_rate_w(rate_w: f64) -> f64 {
    rate_w.max(0.0)
}

pub(crate) fn heat_loss_rate_w(rate_w: f64) -> f64 {
    (-rate_w).max(0.0)
}

pub(crate) fn heat_balance_result_store_from_traces(
    traces: HeatBalanceResultSeriesTraces,
) -> ResultStore {
    let HeatBalanceResultSeriesTraces {
        zone_temperatures,
        zone_humidity_ratios,
        zone_conduction_rates,
        inside_surface_iteration_counts,
        zone_air_heat_balance_rates,
        zone_air_debug_traces,
        surface_temperatures,
        outdoor_temperatures,
        outdoor_wet_bulb_temperatures,
        sky_temperatures,
        horizontal_infrared_radiation_rates,
        rain_statuses,
    } = traces;
    let mut results = ResultStore::new();
    let mut handle_index = 0;
    for (_zone_id, zone_name, values) in zone_temperatures {
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: zone_name,
            variable_name: "Zone Mean Air Temperature".to_string(),
            units: "C".to_string(),
            values,
        });
        handle_index += 1;
    }
    for (_zone_id, zone_name, values) in zone_humidity_ratios {
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: zone_name,
            variable_name: "Zone Mean Air Humidity Ratio".to_string(),
            units: "kgWater/kgDryAir".to_string(),
            values,
        });
        handle_index += 1;
    }
    for trace in zone_conduction_rates {
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.zone_name.clone(),
            variable_name: "Zone Opaque Surface Inside Faces Conduction Rate".to_string(),
            units: "W".to_string(),
            values: trace.inside_conduction_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.zone_name.clone(),
            variable_name: "Zone Opaque Surface Inside Faces Conduction Heat Gain Rate".to_string(),
            units: "W".to_string(),
            values: trace.inside_conduction_gain_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.zone_name.clone(),
            variable_name: "Zone Opaque Surface Inside Faces Conduction Heat Loss Rate".to_string(),
            units: "W".to_string(),
            values: trace.inside_conduction_loss_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.zone_name.clone(),
            variable_name: "Zone Opaque Surface Outside Faces Conduction Rate".to_string(),
            units: "W".to_string(),
            values: trace.outside_conduction_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.zone_name.clone(),
            variable_name: "Zone Opaque Surface Outside Faces Conduction Heat Gain Rate"
                .to_string(),
            units: "W".to_string(),
            values: trace.outside_conduction_gain_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.zone_name,
            variable_name: "Zone Opaque Surface Outside Faces Conduction Heat Loss Rate"
                .to_string(),
            units: "W".to_string(),
            values: trace.outside_conduction_loss_rate_w,
        });
        handle_index += 1;
    }
    results.add_series(OutputSeries {
        handle: OutputHandle(handle_index),
        key: "Simulation".to_string(),
        variable_name: SURFACE_INSIDE_HEAT_BALANCE_ITERATION_COUNT_VARIABLE.to_string(),
        units: String::new(),
        values: inside_surface_iteration_counts,
    });
    handle_index += 1;
    for (
        _zone_id,
        zone_name,
        internal_gain_values,
        surface_convection_values,
        air_storage_values,
    ) in zone_air_heat_balance_rates
    {
        let outdoor_air_transfer_values = vec![0.0; internal_gain_values.len()];
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: zone_name.clone(),
            variable_name: "Zone Air Heat Balance Internal Convective Heat Gain Rate".to_string(),
            units: "W".to_string(),
            values: internal_gain_values,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: zone_name.clone(),
            variable_name: "Zone Air Heat Balance Surface Convection Rate".to_string(),
            units: "W".to_string(),
            values: surface_convection_values,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: zone_name.clone(),
            variable_name: "Zone Air Heat Balance Air Energy Storage Rate".to_string(),
            units: "W".to_string(),
            values: air_storage_values,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: zone_name,
            variable_name: "Zone Air Heat Balance Outdoor Air Transfer Rate".to_string(),
            units: "W".to_string(),
            values: outdoor_air_transfer_values,
        });
        handle_index += 1;
    }
    for trace in zone_air_debug_traces {
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.zone_name.clone(),
            variable_name: RUST_ZONE_AIR_CURRENT_TEMPERATURE_VARIABLE.to_string(),
            units: "C".to_string(),
            values: trace.current_temperature_c,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.zone_name.clone(),
            variable_name: RUST_ZONE_AIR_ZONE_TIMESTEP_AVERAGE_TEMPERATURE_VARIABLE.to_string(),
            units: "C".to_string(),
            values: trace.zone_timestep_average_temperature_c,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.zone_name.clone(),
            variable_name: RUST_ZONE_AIR_PREVIOUS_TEMPERATURE_1_VARIABLE.to_string(),
            units: "C".to_string(),
            values: trace.previous_temperature_1_c,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.zone_name.clone(),
            variable_name: RUST_ZONE_AIR_PREVIOUS_TEMPERATURE_2_VARIABLE.to_string(),
            units: "C".to_string(),
            values: trace.previous_temperature_2_c,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.zone_name.clone(),
            variable_name: RUST_ZONE_AIR_PREVIOUS_TEMPERATURE_3_VARIABLE.to_string(),
            units: "C".to_string(),
            values: trace.previous_temperature_3_c,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.zone_name.clone(),
            variable_name: RUST_ZONE_AIR_PREVIOUS_SYSTEM_TEMPERATURE_1_VARIABLE.to_string(),
            units: "C".to_string(),
            values: trace.previous_system_temperature_1_c,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.zone_name.clone(),
            variable_name: RUST_ZONE_AIR_SYSTEM_TIMESTEP_COUNT_VARIABLE.to_string(),
            units: String::new(),
            values: trace.system_timestep_count,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.zone_name.clone(),
            variable_name: RUST_ZONE_AIR_HUMIDITY_RATIO_VARIABLE.to_string(),
            units: "kgWater/kgDryAir".to_string(),
            values: trace.humidity_ratio,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.zone_name.clone(),
            variable_name: RUST_ZONE_AIR_ZONE_TIMESTEP_AVERAGE_HUMIDITY_RATIO_VARIABLE.to_string(),
            units: "kgWater/kgDryAir".to_string(),
            values: trace.zone_timestep_average_humidity_ratio,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.zone_name.clone(),
            variable_name: RUST_ZONE_AIR_HEAT_CAPACITY_VARIABLE.to_string(),
            units: "J/K".to_string(),
            values: trace.air_heat_capacity_j_per_k,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.zone_name.clone(),
            variable_name: RUST_ZONE_AIR_ZONE_TIMESTEP_AIR_POWER_CAP_VARIABLE.to_string(),
            units: "W/K".to_string(),
            values: trace.zone_timestep_air_power_cap_w_per_k,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.zone_name,
            variable_name: RUST_ZONE_AIR_LAST_CORRECTION_AIR_POWER_CAP_VARIABLE.to_string(),
            units: "W/K".to_string(),
            values: trace.last_correction_air_power_cap_w_per_k,
        });
        handle_index += 1;
    }
    for trace in surface_temperatures {
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Inside Face Temperature".to_string(),
            units: "C".to_string(),
            values: trace.inside_face_temperature_c,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Inside Face Adjacent Air Temperature".to_string(),
            units: "C".to_string(),
            values: trace.inside_adjacent_air_temperature_c,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Outside Face Temperature".to_string(),
            units: "C".to_string(),
            values: trace.outside_face_temperature_c,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Outside Face Outdoor Air Drybulb Temperature".to_string(),
            units: "C".to_string(),
            values: trace.outside_outdoor_air_dry_bulb_temperature_c,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Outside Face Outdoor Air Wetbulb Temperature".to_string(),
            units: "C".to_string(),
            values: trace.outside_outdoor_air_wet_bulb_temperature_c,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Outside Face Outdoor Air Wind Speed".to_string(),
            units: "m/s".to_string(),
            values: trace.outside_outdoor_air_wind_speed_m_per_s,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Outside Face Outdoor Air Wind Direction".to_string(),
            units: "deg".to_string(),
            values: trace.outside_outdoor_air_wind_direction_deg,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Inside Face Convection Heat Transfer Coefficient".to_string(),
            units: "W/m2-K".to_string(),
            values: trace.inside_convection_coefficient_w_per_m2_k,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Inside Face Convection Heat Gain Rate".to_string(),
            units: "W".to_string(),
            values: trace.inside_convection_heat_gain_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Inside Face Convection Heat Gain Rate per Area".to_string(),
            units: "W/m2".to_string(),
            values: trace.inside_convection_heat_gain_rate_per_area_w_per_m2,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Inside Face Net Surface Thermal Radiation Heat Gain Rate"
                .to_string(),
            units: "W".to_string(),
            values: trace.inside_net_surface_thermal_radiation_heat_gain_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name:
                "Surface Inside Face Net Surface Thermal Radiation Heat Gain Rate per Area"
                    .to_string(),
            units: "W/m2".to_string(),
            values: trace.inside_net_surface_thermal_radiation_heat_gain_rate_per_area_w_per_m2,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: SURFACE_INSIDE_RADIANT_INTERNAL_GAIN_SOURCE_TERM_RATE_VARIABLE
                .to_string(),
            units: "W".to_string(),
            values: trace.inside_radiant_internal_gain_source_term_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: SURFACE_INSIDE_RADIANT_INTERNAL_GAIN_SOURCE_TERM_RATE_PER_AREA_VARIABLE
                .to_string(),
            units: "W/m2".to_string(),
            values: trace.inside_radiant_internal_gain_source_term_rate_per_area_w_per_m2,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: SURFACE_INSIDE_SHORTWAVE_ABSORBED_SOURCE_TERM_RATE_VARIABLE.to_string(),
            units: "W".to_string(),
            values: trace.inside_shortwave_absorbed_source_term_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: SURFACE_INSIDE_SHORTWAVE_ABSORBED_SOURCE_TERM_RATE_PER_AREA_VARIABLE
                .to_string(),
            units: "W/m2".to_string(),
            values: trace.inside_shortwave_absorbed_source_term_rate_per_area_w_per_m2,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: SURFACE_INSIDE_ADDITIONAL_HEAT_SOURCE_TERM_RATE_VARIABLE.to_string(),
            units: "W".to_string(),
            values: trace.inside_additional_heat_source_term_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: SURFACE_INSIDE_ADDITIONAL_HEAT_SOURCE_TERM_RATE_PER_AREA_VARIABLE
                .to_string(),
            units: "W/m2".to_string(),
            values: trace.inside_additional_heat_source_term_rate_per_area_w_per_m2,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: SURFACE_INSIDE_RADIANT_HVAC_SOURCE_TERM_RATE_VARIABLE.to_string(),
            units: "W".to_string(),
            values: trace.inside_radiant_hvac_source_term_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: SURFACE_INSIDE_RADIANT_HVAC_SOURCE_TERM_RATE_PER_AREA_VARIABLE
                .to_string(),
            units: "W/m2".to_string(),
            values: trace.inside_radiant_hvac_source_term_rate_per_area_w_per_m2,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: SURFACE_INSIDE_TOTAL_SOURCE_TERM_RATE_VARIABLE.to_string(),
            units: "W".to_string(),
            values: trace.inside_total_source_term_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: SURFACE_INSIDE_TOTAL_SOURCE_TERM_RATE_PER_AREA_VARIABLE.to_string(),
            units: "W/m2".to_string(),
            values: trace.inside_total_source_term_rate_per_area_w_per_m2,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Outside Face Convection Heat Gain Rate".to_string(),
            units: "W".to_string(),
            values: trace.outside_convection_heat_gain_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Outside Face Convection Heat Gain Rate per Area".to_string(),
            units: "W/m2".to_string(),
            values: trace.outside_convection_heat_gain_rate_per_area_w_per_m2,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Outside Face Convection Heat Transfer Coefficient".to_string(),
            units: "W/m2-K".to_string(),
            values: trace.outside_convection_coefficient_w_per_m2_k,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Outside Face Net Thermal Radiation Heat Gain Rate".to_string(),
            units: "W".to_string(),
            values: trace.outside_net_thermal_radiation_heat_gain_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Outside Face Net Thermal Radiation Heat Gain Rate per Area"
                .to_string(),
            units: "W/m2".to_string(),
            values: trace.outside_net_thermal_radiation_heat_gain_rate_per_area_w_per_m2,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name:
                "Surface Outside Face Thermal Radiation to Air Heat Transfer Coefficient"
                    .to_string(),
            units: "W/m2-K".to_string(),
            values: trace.outside_thermal_radiation_to_air_coefficient_w_per_m2_k,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name:
                "Surface Outside Face Thermal Radiation to Sky Heat Transfer Coefficient"
                    .to_string(),
            units: "W/m2-K".to_string(),
            values: trace.outside_thermal_radiation_to_sky_coefficient_w_per_m2_k,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name:
                "Surface Outside Face Thermal Radiation to Ground Heat Transfer Coefficient"
                    .to_string(),
            units: "W/m2-K".to_string(),
            values: trace.outside_thermal_radiation_to_ground_coefficient_w_per_m2_k,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Outside Face Solar Radiation Heat Gain Rate".to_string(),
            units: "W".to_string(),
            values: trace.outside_solar_radiation_heat_gain_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Outside Face Solar Radiation Heat Gain Rate per Area"
                .to_string(),
            units: "W/m2".to_string(),
            values: trace.outside_solar_radiation_heat_gain_rate_per_area_w_per_m2,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: SURFACE_OUTSIDE_BALANCE_REPORT_TEMPERATURE_VARIABLE.to_string(),
            units: "C".to_string(),
            values: trace.outside_balance_report_temperature_c,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: SURFACE_OUTSIDE_BALANCE_COEFFICIENT_TEMPERATURE_VARIABLE.to_string(),
            units: "C".to_string(),
            values: trace.outside_balance_coefficient_temperature_c,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: SURFACE_OUTSIDE_BALANCE_CONVECTION_REFERENCE_TEMPERATURE_VARIABLE
                .to_string(),
            units: "C".to_string(),
            values: trace.outside_balance_convection_reference_temperature_c,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: SURFACE_OUTSIDE_BALANCE_EQUIVALENT_RADIANT_TEMPERATURE_VARIABLE
                .to_string(),
            units: "C".to_string(),
            values: trace.outside_balance_equivalent_radiant_temperature_c,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: SURFACE_OUTSIDE_BALANCE_RADIATION_COEFFICIENT_VARIABLE.to_string(),
            units: "W/m2-K".to_string(),
            values: trace.outside_balance_radiation_coefficient_w_per_m2_k,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: SURFACE_OUTSIDE_QUICK_BALANCE_INSIDE_SOURCE_TERM_VARIABLE.to_string(),
            units: "W/m2".to_string(),
            values: trace.outside_quick_balance_inside_source_term_w_per_m2,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: SURFACE_OUTSIDE_QUICK_BALANCE_INSIDE_BALANCE_TERM_VARIABLE.to_string(),
            units: "W/m2".to_string(),
            values: trace.outside_quick_balance_inside_balance_term_w_per_m2,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: SURFACE_OUTSIDE_QUICK_BALANCE_NUMERATOR_VARIABLE.to_string(),
            units: "W/m2".to_string(),
            values: trace.outside_quick_balance_numerator_w_per_m2,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: SURFACE_OUTSIDE_QUICK_BALANCE_DENOMINATOR_VARIABLE.to_string(),
            units: "W/m2-K".to_string(),
            values: trace.outside_quick_balance_denominator_w_per_m2_k,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: SURFACE_OUTSIDE_QUICK_BALANCE_COUPLING_FACTOR_VARIABLE.to_string(),
            units: String::new(),
            values: trace.outside_quick_balance_coupling_factor,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Inside Face Conduction Heat Transfer Rate".to_string(),
            units: "W".to_string(),
            values: trace.inside_conduction_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Inside Face Conduction Heat Gain Rate".to_string(),
            units: "W".to_string(),
            values: trace.inside_conduction_gain_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Inside Face Conduction Heat Loss Rate".to_string(),
            units: "W".to_string(),
            values: trace.inside_conduction_loss_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Inside Face Conduction Heat Transfer Rate per Area".to_string(),
            units: "W/m2".to_string(),
            values: trace.inside_conduction_rate_per_area_w_per_m2,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: SURFACE_CTF_INSIDE_CURRENT_OUTSIDE_TERM_RATE_VARIABLE.to_string(),
            units: "W".to_string(),
            values: trace.ctf_inside_current_outside_term_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: SURFACE_CTF_INSIDE_CURRENT_INSIDE_TERM_RATE_VARIABLE.to_string(),
            units: "W".to_string(),
            values: trace.ctf_inside_current_inside_term_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: SURFACE_CTF_INSIDE_HISTORY_TERM_RATE_VARIABLE.to_string(),
            units: "W".to_string(),
            values: trace.ctf_inside_history_term_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: SURFACE_CTF_INSIDE_HISTORY_TEMPERATURE_TERM_RATE_VARIABLE.to_string(),
            units: "W".to_string(),
            values: trace.ctf_inside_history_temperature_term_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: SURFACE_CTF_INSIDE_HISTORY_FLUX_TERM_RATE_VARIABLE.to_string(),
            units: "W".to_string(),
            values: trace.ctf_inside_history_flux_term_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Outside Face Conduction Heat Transfer Rate".to_string(),
            units: "W".to_string(),
            values: trace.outside_conduction_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Outside Face Conduction Heat Gain Rate".to_string(),
            units: "W".to_string(),
            values: trace.outside_conduction_gain_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Outside Face Conduction Heat Loss Rate".to_string(),
            units: "W".to_string(),
            values: trace.outside_conduction_loss_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Outside Face Conduction Heat Transfer Rate per Area"
                .to_string(),
            units: "W/m2".to_string(),
            values: trace.outside_conduction_rate_per_area_w_per_m2,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: SURFACE_CTF_OUTSIDE_CURRENT_OUTSIDE_TERM_RATE_VARIABLE.to_string(),
            units: "W".to_string(),
            values: trace.ctf_outside_current_outside_term_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: SURFACE_CTF_OUTSIDE_CURRENT_INSIDE_TERM_RATE_VARIABLE.to_string(),
            units: "W".to_string(),
            values: trace.ctf_outside_current_inside_term_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: SURFACE_CTF_OUTSIDE_HISTORY_TERM_RATE_VARIABLE.to_string(),
            units: "W".to_string(),
            values: trace.ctf_outside_history_term_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Heat Storage Rate".to_string(),
            units: "W".to_string(),
            values: trace.heat_storage_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name,
            variable_name: "Surface Heat Storage Rate per Area".to_string(),
            units: "W/m2".to_string(),
            values: trace.heat_storage_rate_per_area_w_per_m2,
        });
        handle_index += 1;
    }
    results.add_series(OutputSeries {
        handle: OutputHandle(handle_index),
        key: "Environment".to_string(),
        variable_name: "Site Outdoor Air Drybulb Temperature".to_string(),
        units: "C".to_string(),
        values: outdoor_temperatures,
    });
    handle_index += 1;
    results.add_series(OutputSeries {
        handle: OutputHandle(handle_index),
        key: "Environment".to_string(),
        variable_name: "Site Outdoor Air Wetbulb Temperature".to_string(),
        units: "C".to_string(),
        values: outdoor_wet_bulb_temperatures,
    });
    handle_index += 1;
    results.add_series(OutputSeries {
        handle: OutputHandle(handle_index),
        key: "Environment".to_string(),
        variable_name: "Site Sky Temperature".to_string(),
        units: "C".to_string(),
        values: sky_temperatures,
    });
    handle_index += 1;
    results.add_series(OutputSeries {
        handle: OutputHandle(handle_index),
        key: "Environment".to_string(),
        variable_name: "Site Horizontal Infrared Radiation Rate per Area".to_string(),
        units: "W/m2".to_string(),
        values: horizontal_infrared_radiation_rates,
    });
    handle_index += 1;
    results.add_series(OutputSeries {
        handle: OutputHandle(handle_index),
        key: "Environment".to_string(),
        variable_name: "Site Rain Status".to_string(),
        units: String::new(),
        values: rain_statuses,
    });

    results
}
