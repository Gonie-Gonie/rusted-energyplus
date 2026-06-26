//! Heat-balance run-period timestep sampling loop.

use crate::heat_balance::air_manager::{
    weather_proxy_zone_air_heat_capacity_j_per_k, zone_air_heat_balance_air_storage_rate_w,
};
use crate::heat_balance::convection::{
    energyplus_building_terrain, energyplus_surface_outdoor_air_temperature_c,
    energyplus_surface_outside_wind_speed_m_per_s,
};
use crate::heat_balance::ctf::{
    heat_balance_ctf_history_slot_inside_flux_term_rate_w,
    heat_balance_ctf_history_slot_inside_temperature_term_rate_w,
    surface_ctf_inside_current_inside_term_rate_w,
    surface_ctf_inside_current_outside_term_rate_w_for_report,
    surface_ctf_inside_history_term_rate_w, surface_ctf_outside_current_inside_term_rate_w,
    surface_ctf_outside_current_outside_term_rate_w_for_report,
    surface_ctf_outside_history_term_rate_w, surface_heat_storage_rate_w,
    surface_inside_conduction_rate_w_for_report, surface_outside_conduction_rate_w_for_report,
    surface_rate_per_area_w_per_m2,
};
use crate::heat_balance::inside_convection::{
    surface_inside_convection_heat_gain_rate_per_area_w_per_m2,
    surface_inside_convection_report_coefficient_w_per_m2_k,
    zone_air_heat_balance_surface_convection_rate_at_air_temperature_w,
    zone_air_heat_balance_surface_convection_rate_from_balance_w,
    zone_air_heat_balance_surface_convection_rate_from_final_inside_hconv_report_w,
    zone_air_heat_balance_surface_convection_rate_from_surface_reference_air_w,
    zone_air_heat_balance_surface_convection_rate_w,
};
use crate::heat_balance::longwave::horizontal_infrared_sky_temperature_c;
use crate::heat_balance::reports::{
    heat_gain_rate_w, heat_loss_rate_w, zone_surface_report_conduction_rates_w,
};
use crate::heat_balance::state::{
    HeatBalanceSimulationOptions, HeatBalanceState, HeatBalanceStepInput,
    HeatBalanceSurfaceFirstSampleTrace, HeatBalanceSurfaceIterationFirstSampleTrace,
    HeatBalanceSurfaceIterationSampleTrace, HeatBalanceZoneAirFirstSampleTrace,
    HeatBalanceZoneConductionReportSource,
};
use crate::heat_balance::surface_balance::{
    reported_surface_outside_face_temperature_c, surface_exterior_report_terms,
};
use crate::heat_balance::surface_manager;
use crate::heat_balance::surface_weather::{
    energyplus_exterior_wet_reference_temperature_c,
    energyplus_weather_record_is_rain_at_timestep_with_starting_values,
};
use crate::heat_balance::timestep::advance_heat_balance_state_one_timestep_internal;
use crate::heat_balance::trace::{
    HeatBalanceCtfHistorySlotFirstSampleAccumulator, HeatBalanceRunPeriodSamples,
    SurfaceHeatBalanceTraceSums, ZoneAirDebugTraceSums, push_surface_heat_balance_trace_averages,
    push_zone_air_debug_trace_averages, push_zone_air_heat_balance_trace_values,
    push_zone_conduction_trace_averages, push_zone_scalar_trace_averages,
    surface_heat_balance_traces_from_state, zone_air_debug_traces_from_state,
    zone_air_heat_balance_trace_series_from_state, zone_conduction_traces_from_state,
    zone_scalar_trace_series_from_state,
};
use crate::heat_balance::{
    HeatBalanceZoneAirAlgorithm, heat_balance_uses_balance_surface_convection_report,
    heat_balance_uses_final_inside_convection_report,
    heat_balance_uses_previous_mat_surface_convection_report,
    heat_balance_uses_surface_reference_air_convection_report,
    heat_balance_uses_surface_reference_air_surface_convection_report,
    heat_balance_uses_weather_air_storage_report,
    heat_balance_zone_air_algorithm_execution_variant,
};
use crate::weather::{
    EpwRecord, energyplus_weather_dry_bulb_at_timestep_with_starting_values,
    energyplus_weather_horizontal_infrared_for_context,
    energyplus_weather_wind_direction_for_context, energyplus_weather_wind_speed_for_context,
    heat_balance_weather_context_for_timestep,
};
use ep_model::{FirstHourInterpolationStartingValues, SimulationModel};
use std::collections::BTreeMap;

pub(crate) fn sample_heat_balance_run_period(
    model: &SimulationModel,
    state: &mut HeatBalanceState,
    weather_dry_bulb_c: &[f64],
    weather_records: Option<&[EpwRecord]>,
    options: HeatBalanceSimulationOptions,
    zone_steps_per_hour: u32,
    seconds_per_timestep: f64,
    first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues,
) -> HeatBalanceRunPeriodSamples {
    let mut zone_temperatures = zone_scalar_trace_series_from_state(state, options.sample_count);
    let mut zone_humidity_ratios = zone_scalar_trace_series_from_state(state, options.sample_count);
    let mut zone_conduction_rates = zone_conduction_traces_from_state(state, options.sample_count);
    let mut inside_surface_iteration_counts = Vec::with_capacity(options.sample_count);
    let mut zone_air_heat_balance_rates =
        zone_air_heat_balance_trace_series_from_state(state, options.sample_count);
    let mut zone_air_debug_traces = zone_air_debug_traces_from_state(state, options.sample_count);
    let mut surface_temperatures =
        surface_heat_balance_traces_from_state(state, options.sample_count);
    let mut outdoor_temperatures = Vec::with_capacity(options.sample_count);
    let mut outdoor_wet_bulb_temperatures = Vec::with_capacity(options.sample_count);
    let mut sky_temperatures = Vec::with_capacity(options.sample_count);
    let mut horizontal_infrared_radiation_rates = Vec::with_capacity(options.sample_count);
    let mut rain_statuses = Vec::with_capacity(options.sample_count);
    let mut first_sample_ctf_history_slot_accumulators =
        BTreeMap::<(String, usize), HeatBalanceCtfHistorySlotFirstSampleAccumulator>::new();
    let mut hourly_ctf_history_slots = Vec::new();
    let mut hourly_ctf_history_slots_after_advance = Vec::new();
    let mut surface_first_sample_trace = Vec::new();
    let mut zone_air_first_sample_trace = Vec::new();
    let mut surface_iteration_first_sample_trace = Vec::new();
    let mut surface_iteration_sample_trace = Vec::new();
    let report_zone_air_algorithm =
        heat_balance_zone_air_algorithm_execution_variant(options.zone_air_algorithm);
    let use_surface_reference_air_zone_convection_report =
        heat_balance_uses_surface_reference_air_convection_report(report_zone_air_algorithm);
    let use_surface_reference_air_surface_convection_report =
        heat_balance_uses_surface_reference_air_surface_convection_report(
            report_zone_air_algorithm,
        );
    let use_final_inside_convection_report =
        heat_balance_uses_final_inside_convection_report(report_zone_air_algorithm);
    let use_inside_ctf_outside_temperature_for_conduction_report = matches!(
        report_zone_air_algorithm,
        HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatInsideCtfReportProbe
    );
    let use_surface_report_zone_conduction_rates = matches!(
        options.zone_conduction_report_source,
        HeatBalanceZoneConductionReportSource::SurfaceReport
    );

    for (hour_index, outdoor_dry_bulb_c) in weather_dry_bulb_c
        .iter()
        .copied()
        .take(options.sample_count)
        .enumerate()
    {
        let hour_ending = u32::try_from(hour_index % 24 + 1).unwrap_or(24);
        let steps = zone_steps_per_hour.max(1);
        let mut zone_temperature_sums = vec![0.0; zone_temperatures.len()];
        let mut zone_humidity_ratio_sums = vec![0.0; zone_humidity_ratios.len()];
        let mut zone_conduction_sums =
            vec![(0.0, 0.0, 0.0, 0.0, 0.0, 0.0); zone_conduction_rates.len()];
        let mut inside_surface_iteration_count_sum = 0.0;
        let mut zone_air_heat_balance_sums =
            vec![(0.0, 0.0, 0.0); zone_air_heat_balance_rates.len()];
        let mut zone_air_heat_balance_last =
            vec![(0.0, 0.0, 0.0); zone_air_heat_balance_rates.len()];
        let mut zone_air_debug_sums =
            vec![ZoneAirDebugTraceSums::default(); zone_air_debug_traces.len()];
        let mut surface_sums =
            vec![SurfaceHeatBalanceTraceSums::default(); surface_temperatures.len()];
        let mut outdoor_temperature_sum = 0.0;
        let mut outdoor_wet_bulb_temperature_sum = 0.0;
        let mut sky_temperature_sum = 0.0;
        let mut horizontal_infrared_radiation_sum = 0.0;
        let mut rain_status_sum = 0.0;
        let mut hourly_ctf_history_slot_accumulators =
            BTreeMap::<(String, usize), HeatBalanceCtfHistorySlotFirstSampleAccumulator>::new();
        let mut hourly_ctf_history_slot_after_advance_accumulators =
            BTreeMap::<(String, usize), HeatBalanceCtfHistorySlotFirstSampleAccumulator>::new();

        for substep in 1..=steps {
            let timestep_outdoor_dry_bulb_c =
                energyplus_weather_dry_bulb_at_timestep_with_starting_values(
                    weather_records,
                    hour_index,
                    outdoor_dry_bulb_c,
                    steps,
                    substep,
                    first_hour_interpolation_starting_values,
                );
            let weather_context = heat_balance_weather_context_for_timestep(
                weather_records,
                hour_index,
                steps,
                substep,
                first_hour_interpolation_starting_values,
            );
            let timestep_outdoor_wet_bulb_c = weather_context
                .map(|context| {
                    energyplus_exterior_wet_reference_temperature_c(
                        context,
                        timestep_outdoor_dry_bulb_c,
                    )
                })
                .unwrap_or(timestep_outdoor_dry_bulb_c);
            let timestep_horizontal_infrared_radiation_w_per_m2 = weather_context
                .and_then(|context| {
                    context.records.get(context.record_index).map(|record| {
                        energyplus_weather_horizontal_infrared_for_context(
                            context,
                            record.horizontal_infrared_radiation_wh_per_m2,
                        )
                    })
                })
                .unwrap_or(0.0);
            let timestep_sky_temperature_c = horizontal_infrared_sky_temperature_c(
                timestep_horizontal_infrared_radiation_w_per_m2,
                timestep_outdoor_dry_bulb_c,
            );
            let timestep_rain_status = weather_context
                .map(|context| {
                    if energyplus_weather_record_is_rain_at_timestep_with_starting_values(
                        context.records,
                        context.record_index,
                        substep,
                        steps,
                        context.first_hour_interpolation_starting_values,
                    ) {
                        1.0
                    } else {
                        0.0
                    }
                })
                .unwrap_or(0.0);
            advance_heat_balance_state_one_timestep_internal(
                &model.typed,
                &mut *state,
                HeatBalanceStepInput {
                    outdoor_dry_bulb_c: timestep_outdoor_dry_bulb_c,
                    hour_ending,
                    timestep_seconds: seconds_per_timestep,
                },
                weather_context,
                options.zone_air_algorithm,
                options.surface_iteration_count,
                options.inside_hconv_reevaluation_interval,
                options.surface_loop_zone_air_correction,
            );

            for sample in &state.last_ctf_history_slot_terms {
                hourly_ctf_history_slot_accumulators
                    .entry((sample.surface_name.clone(), sample.slot_index))
                    .or_insert_with(|| {
                        HeatBalanceCtfHistorySlotFirstSampleAccumulator::from_sample(sample)
                    })
                    .push(sample);
            }
            for sample in &state.last_ctf_history_slot_terms_after_advance {
                hourly_ctf_history_slot_after_advance_accumulators
                    .entry((sample.surface_name.clone(), sample.slot_index))
                    .or_insert_with(|| {
                        HeatBalanceCtfHistorySlotFirstSampleAccumulator::from_sample(sample)
                    })
                    .push(sample);
            }

            if hour_index == 0 {
                for sample in &state.last_ctf_history_slot_terms {
                    first_sample_ctf_history_slot_accumulators
                        .entry((sample.surface_name.clone(), sample.slot_index))
                        .or_insert_with(|| {
                            HeatBalanceCtfHistorySlotFirstSampleAccumulator::from_sample(sample)
                        })
                        .push(sample);
                }
            }

            outdoor_temperature_sum += timestep_outdoor_dry_bulb_c;
            outdoor_wet_bulb_temperature_sum += timestep_outdoor_wet_bulb_c;
            sky_temperature_sum += timestep_sky_temperature_c;
            horizontal_infrared_radiation_sum += timestep_horizontal_infrared_radiation_w_per_m2;
            rain_status_sum += timestep_rain_status;
            for (index, (zone_id, _zone_name, _values)) in zone_temperatures.iter().enumerate() {
                if let Some(zone_state) = state.zones.iter().find(|zone| zone.zone_id == *zone_id) {
                    let reported_zone_temperature_c = if matches!(
                        options.zone_air_algorithm,
                        HeatBalanceZoneAirAlgorithm::EnergyPlusHeatBalanceCompatCandidate
                            | HeatBalanceZoneAirAlgorithm::EnergyPlusSourceOrder1ZoneOpaqueCompatibility
                    ) {
                        zone_state.zone_timestep_average_air_temperature_c
                    } else {
                        zone_state.mean_air_temperature_c
                    };
                    zone_temperature_sums[index] += reported_zone_temperature_c;
                }
            }
            for (index, (zone_id, _zone_name, _values)) in zone_humidity_ratios.iter().enumerate() {
                if let Some(zone_state) = state.zones.iter().find(|zone| zone.zone_id == *zone_id) {
                    let reported_zone_humidity_ratio = if matches!(
                        options.zone_air_algorithm,
                        HeatBalanceZoneAirAlgorithm::EnergyPlusHeatBalanceCompatCandidate
                            | HeatBalanceZoneAirAlgorithm::EnergyPlusSourceOrder1ZoneOpaqueCompatibility
                    ) {
                        zone_state.zone_timestep_average_air_humidity_ratio
                    } else {
                        zone_state.air_humidity_ratio
                    };
                    zone_humidity_ratio_sums[index] += reported_zone_humidity_ratio;
                }
            }
            for (index, trace) in zone_air_debug_traces.iter().enumerate() {
                if let Some(zone_state) = state
                    .zones
                    .iter()
                    .find(|zone| zone.zone_id == trace.zone_id)
                {
                    let zone_timestep_air_power_cap_w_per_k = if seconds_per_timestep > 0.0 {
                        zone_state.air_heat_capacity_j_per_k / seconds_per_timestep
                    } else {
                        0.0
                    };
                    if hour_index == 0 {
                        let coefficients = zone_state.zone_air_temperature_coefficients;
                        let third_order_solution_temperature_c =
                            if coefficients.third_order_temp_dependent_load_w_per_k.abs()
                                <= f64::EPSILON
                            {
                                zone_state.mean_air_temperature_c
                            } else {
                                coefficients.third_order_temp_independent_load_w
                                    / coefficients.third_order_temp_dependent_load_w_per_k
                            };
                        zone_air_first_sample_trace.push(HeatBalanceZoneAirFirstSampleTrace {
                            zone_id: zone_state.zone_id,
                            zone_name: zone_state.zone_name.clone(),
                            timestep_index: substep,
                            outdoor_dry_bulb_c: timestep_outdoor_dry_bulb_c,
                            timestep_seconds: seconds_per_timestep,
                            mean_air_temperature_c: zone_state.mean_air_temperature_c,
                            zone_timestep_average_air_temperature_c: zone_state
                                .zone_timestep_average_air_temperature_c,
                            previous_mean_air_temperatures_c: zone_state
                                .previous_mean_air_temperatures_c,
                            previous_system_mean_air_temperatures_c: zone_state
                                .previous_system_mean_air_temperatures_c,
                            previous_system_timestep_count: zone_state
                                .previous_system_timestep_count,
                            air_humidity_ratio: zone_state.air_humidity_ratio,
                            zone_timestep_average_air_humidity_ratio: zone_state
                                .zone_timestep_average_air_humidity_ratio,
                            air_heat_capacity_j_per_k: zone_state.air_heat_capacity_j_per_k,
                            zone_timestep_air_power_cap_w_per_k,
                            zone_air_temperature_coefficients: coefficients,
                            third_order_solution_numerator_w: coefficients
                                .third_order_temp_independent_load_w,
                            third_order_solution_denominator_w_per_k: coefficients
                                .third_order_temp_dependent_load_w_per_k,
                            third_order_solution_temperature_c,
                        });
                    }
                    zone_air_debug_sums[index].current_temperature_c +=
                        zone_state.mean_air_temperature_c;
                    zone_air_debug_sums[index].zone_timestep_average_temperature_c +=
                        zone_state.zone_timestep_average_air_temperature_c;
                    zone_air_debug_sums[index].previous_temperature_1_c +=
                        zone_state.previous_mean_air_temperatures_c[0];
                    zone_air_debug_sums[index].previous_temperature_2_c +=
                        zone_state.previous_mean_air_temperatures_c[1];
                    zone_air_debug_sums[index].previous_temperature_3_c +=
                        zone_state.previous_mean_air_temperatures_c[2];
                    zone_air_debug_sums[index].previous_system_temperature_1_c +=
                        zone_state.previous_system_mean_air_temperatures_c[0];
                    zone_air_debug_sums[index].system_timestep_count +=
                        f64::from(zone_state.previous_system_timestep_count);
                    zone_air_debug_sums[index].humidity_ratio += zone_state.air_humidity_ratio;
                    zone_air_debug_sums[index].zone_timestep_average_humidity_ratio +=
                        zone_state.zone_timestep_average_air_humidity_ratio;
                    zone_air_debug_sums[index].air_heat_capacity_j_per_k +=
                        zone_state.air_heat_capacity_j_per_k;
                    zone_air_debug_sums[index].zone_timestep_air_power_cap_w_per_k +=
                        zone_timestep_air_power_cap_w_per_k;
                    zone_air_debug_sums[index].last_correction_air_power_cap_w_per_k += zone_state
                        .zone_air_temperature_coefficients
                        .air_power_cap_w_per_k;
                }
            }
            for (index, trace) in zone_conduction_rates.iter().enumerate() {
                if use_surface_report_zone_conduction_rates {
                    let (inside_rate, outside_rate) = zone_surface_report_conduction_rates_w(
                        &state.surfaces,
                        trace.zone_id,
                        use_inside_ctf_outside_temperature_for_conduction_report,
                    );
                    zone_conduction_sums[index].0 += inside_rate;
                    zone_conduction_sums[index].1 += heat_gain_rate_w(inside_rate);
                    zone_conduction_sums[index].2 += heat_loss_rate_w(inside_rate);
                    zone_conduction_sums[index].3 += outside_rate;
                    zone_conduction_sums[index].4 += heat_gain_rate_w(outside_rate);
                    zone_conduction_sums[index].5 += heat_loss_rate_w(outside_rate);
                } else if let Some(zone_state) = state
                    .zones
                    .iter()
                    .find(|zone| zone.zone_id == trace.zone_id)
                {
                    let inside_rate = zone_state.opaque_surface_heat_gain_w;
                    let outside_rate = zone_state.opaque_surface_outside_conduction_w;
                    zone_conduction_sums[index].0 += inside_rate;
                    zone_conduction_sums[index].1 += heat_gain_rate_w(inside_rate);
                    zone_conduction_sums[index].2 += heat_loss_rate_w(inside_rate);
                    zone_conduction_sums[index].3 += outside_rate;
                    zone_conduction_sums[index].4 += heat_gain_rate_w(outside_rate);
                    zone_conduction_sums[index].5 += heat_loss_rate_w(outside_rate);
                }
            }
            inside_surface_iteration_count_sum +=
                f64::from(state.last_inside_surface_iteration_count);
            surface_iteration_sample_trace.push(HeatBalanceSurfaceIterationSampleTrace {
                sample_index: hour_index,
                timestep_index: substep,
                inside_surface_iteration_count: state.last_inside_surface_iteration_count,
                max_inside_surface_delta_c: state.last_inside_surface_iteration_max_delta_c,
                max_delta_surface_name: state
                    .last_inside_surface_iteration_max_delta_surface_name
                    .clone(),
            });
            if hour_index == 0 {
                surface_iteration_first_sample_trace.push(
                    HeatBalanceSurfaceIterationFirstSampleTrace {
                        timestep_index: substep,
                        inside_surface_iteration_count: state.last_inside_surface_iteration_count,
                        max_inside_surface_delta_c: state.last_inside_surface_iteration_max_delta_c,
                        max_delta_surface_name: state
                            .last_inside_surface_iteration_max_delta_surface_name
                            .clone(),
                    },
                );
            }
            for (index, (zone_id, _zone_name, _internal, _surface, _storage)) in
                zone_air_heat_balance_rates.iter().enumerate()
            {
                if let Some(zone_state) = state.zones.iter().find(|zone| zone.zone_id == *zone_id) {
                    let third_order_report_air_heat_capacity_j_per_k =
                        if heat_balance_uses_weather_air_storage_report(options.zone_air_algorithm)
                        {
                            weather_proxy_zone_air_heat_capacity_j_per_k(
                                zone_state,
                                weather_context,
                                timestep_outdoor_dry_bulb_c,
                            )
                        } else {
                            None
                        };
                    let air_storage_rate_w = zone_air_heat_balance_air_storage_rate_w(
                        zone_state,
                        seconds_per_timestep,
                        options.zone_air_algorithm,
                        third_order_report_air_heat_capacity_j_per_k,
                    );
                    let air_storage_rate_w = zone_state
                        .system_timestep_average_air_storage_report_w
                        .unwrap_or(air_storage_rate_w);
                    let surface_convection_rate_w = if use_final_inside_convection_report {
                        zone_air_heat_balance_surface_convection_rate_from_final_inside_hconv_report_w(
                                &state.surfaces,
                                &state.zones,
                                *zone_id,
                                use_surface_reference_air_zone_convection_report,
                            )
                    } else if use_surface_reference_air_zone_convection_report {
                        zone_air_heat_balance_surface_convection_rate_from_surface_reference_air_w(
                            &state.surfaces,
                            *zone_id,
                        )
                    } else if heat_balance_uses_balance_surface_convection_report(
                        options.zone_air_algorithm,
                    ) {
                        zone_air_heat_balance_surface_convection_rate_from_balance_w(
                            zone_state,
                            air_storage_rate_w,
                        )
                    } else if heat_balance_uses_previous_mat_surface_convection_report(
                        options.zone_air_algorithm,
                    ) {
                        zone_air_heat_balance_surface_convection_rate_at_air_temperature_w(
                            zone_state,
                            zone_state.previous_mean_air_temperatures_c[0],
                        )
                    } else {
                        zone_air_heat_balance_surface_convection_rate_w(zone_state)
                    };
                    let surface_convection_rate_w = zone_state
                        .system_timestep_average_surface_convection_report_w
                        .unwrap_or(surface_convection_rate_w);
                    let values = (
                        zone_state.convective_internal_gain_w,
                        surface_convection_rate_w,
                        air_storage_rate_w,
                    );
                    zone_air_heat_balance_sums[index].0 += values.0;
                    zone_air_heat_balance_sums[index].1 += values.1;
                    zone_air_heat_balance_sums[index].2 += values.2;
                    zone_air_heat_balance_last[index] = values;
                }
            }
            surface_manager::report_surface_heat_balance_source_order_path(|| {
                for (index, trace) in surface_temperatures.iter().enumerate() {
                    if let Some(surface_state) = state
                        .surfaces
                        .iter()
                        .find(|surface| surface.surface_id == trace.surface_id)
                    {
                        let inside_convection_heat_gain_rate_per_area =
                            surface_inside_convection_heat_gain_rate_per_area_w_per_m2(
                                surface_state,
                                &state.zones,
                                use_surface_reference_air_surface_convection_report,
                                use_final_inside_convection_report,
                            );
                        let inside_convection_heat_gain_rate =
                            surface_state.area_m2 * inside_convection_heat_gain_rate_per_area;
                        let inside_convection_coefficient_w_per_m2_k =
                            surface_inside_convection_report_coefficient_w_per_m2_k(
                                surface_state,
                                &state.zones,
                                use_surface_reference_air_surface_convection_report,
                                use_final_inside_convection_report,
                            );
                        let inside_net_surface_thermal_radiation_heat_gain_rate =
                            surface_state.area_m2 * surface_state.inside_net_longwave_w_per_m2;
                        let inside_rate = surface_inside_conduction_rate_w_for_report(
                            surface_state,
                            use_inside_ctf_outside_temperature_for_conduction_report,
                        );
                        let outside_rate = surface_outside_conduction_rate_w_for_report(
                            surface_state,
                            use_inside_ctf_outside_temperature_for_conduction_report,
                        );
                        let storage_rate = surface_heat_storage_rate_w(inside_rate, outside_rate);
                        let storage_rate_per_area =
                            surface_rate_per_area_w_per_m2(storage_rate, surface_state.area_m2);
                        let outside_face_temperature_c =
                            reported_surface_outside_face_temperature_c(
                                &model.typed,
                                surface_state,
                                timestep_outdoor_dry_bulb_c,
                                surface_state.inside_face_temperature_c,
                                weather_context,
                                options.zone_air_algorithm,
                            );
                        let exterior_terms = surface_exterior_report_terms(
                            &model.typed,
                            surface_state,
                            timestep_outdoor_dry_bulb_c,
                            outside_face_temperature_c,
                            weather_context,
                            options.zone_air_algorithm,
                        );
                        let typed_surface = model
                            .typed
                            .surfaces
                            .iter()
                            .find(|surface| surface.id == surface_state.surface_id);
                        let surface_outdoor_air_dry_bulb_temperature_c = typed_surface
                            .map(|surface| {
                                energyplus_surface_outdoor_air_temperature_c(
                                    surface,
                                    timestep_outdoor_dry_bulb_c,
                                )
                            })
                            .unwrap_or(timestep_outdoor_dry_bulb_c);
                        let surface_outdoor_air_wet_bulb_temperature_c = typed_surface
                            .map(|surface| {
                                energyplus_surface_outdoor_air_temperature_c(
                                    surface,
                                    timestep_outdoor_wet_bulb_c,
                                )
                            })
                            .unwrap_or(timestep_outdoor_wet_bulb_c);
                        let (
                            weather_file_wind_speed_m_per_s,
                            surface_outdoor_air_wind_direction_deg,
                        ) = weather_context
                            .and_then(|context| {
                                context.records.get(context.record_index).map(|record| {
                                    (
                                        energyplus_weather_wind_speed_for_context(
                                            context,
                                            record.wind_speed_m_per_s,
                                        ),
                                        energyplus_weather_wind_direction_for_context(
                                            context,
                                            record.wind_direction_deg,
                                        ),
                                    )
                                })
                            })
                            .unwrap_or((0.0, 0.0));
                        let surface_outdoor_air_wind_speed_m_per_s = typed_surface
                            .map(|surface| {
                                energyplus_surface_outside_wind_speed_m_per_s(
                                    surface,
                                    energyplus_building_terrain(&model.typed),
                                    weather_file_wind_speed_m_per_s,
                                )
                            })
                            .unwrap_or(weather_file_wind_speed_m_per_s);
                        if hour_index == 0 {
                            let zone_mean_air_temperature_c = state
                                .zones
                                .iter()
                                .find(|zone| zone.zone_id == surface_state.zone_id)
                                .map(|zone| zone.mean_air_temperature_c)
                                .unwrap_or(f64::NAN);
                            surface_first_sample_trace.push(HeatBalanceSurfaceFirstSampleTrace {
                                surface_name: surface_state.surface_name.clone(),
                                construction_name: surface_state.construction_name.clone(),
                                timestep_index: substep,
                                outdoor_dry_bulb_c: timestep_outdoor_dry_bulb_c,
                                zone_mean_air_temperature_c,
                                inside_face_temperature_c: surface_state.inside_face_temperature_c,
                                inside_convection_input_inside_face_temperature_c: surface_state
                                    .inside_convection_input_inside_face_temperature_c,
                                inside_convection_input_reference_air_temperature_c: surface_state
                                    .inside_convection_input_reference_air_temperature_c,
                                outside_face_temperature_c,
                                inside_convection_heat_gain_rate_w:
                                    inside_convection_heat_gain_rate,
                                inside_net_surface_thermal_radiation_heat_gain_rate_w:
                                    inside_net_surface_thermal_radiation_heat_gain_rate,
                                inside_conduction_rate_w: inside_rate,
                                outside_conduction_rate_w: outside_rate,
                                heat_storage_rate_w: storage_rate,
                                outside_convection_heat_gain_rate_w: exterior_terms
                                    .convection_heat_gain_rate_w,
                                outside_net_thermal_radiation_heat_gain_rate_w: exterior_terms
                                    .net_thermal_radiation_heat_gain_rate_w,
                                outside_solar_radiation_heat_gain_rate_w: exterior_terms
                                    .solar_radiation_heat_gain_rate_w,
                            });
                        }
                        let sums = &mut surface_sums[index];
                        sums.inside_face_temperature_c += surface_state.inside_face_temperature_c;
                        sums.inside_adjacent_air_temperature_c +=
                            surface_state.inside_reference_air_temperature_c;
                        sums.outside_face_temperature_c += outside_face_temperature_c;
                        sums.outside_outdoor_air_dry_bulb_temperature_c +=
                            surface_outdoor_air_dry_bulb_temperature_c;
                        sums.outside_outdoor_air_wet_bulb_temperature_c +=
                            surface_outdoor_air_wet_bulb_temperature_c;
                        sums.outside_outdoor_air_wind_speed_m_per_s +=
                            surface_outdoor_air_wind_speed_m_per_s;
                        sums.outside_outdoor_air_wind_direction_deg +=
                            surface_outdoor_air_wind_direction_deg;
                        sums.inside_convection_heat_gain_rate_w += inside_convection_heat_gain_rate;
                        sums.inside_convection_heat_gain_rate_per_area_w_per_m2 +=
                            inside_convection_heat_gain_rate_per_area;
                        sums.inside_convection_coefficient_w_per_m2_k +=
                            inside_convection_coefficient_w_per_m2_k;
                        sums.inside_net_surface_thermal_radiation_heat_gain_rate_w +=
                            inside_net_surface_thermal_radiation_heat_gain_rate;
                        sums.inside_net_surface_thermal_radiation_heat_gain_rate_per_area_w_per_m2 +=
                        surface_state.inside_net_longwave_w_per_m2;
                        sums.outside_convection_heat_gain_rate_w +=
                            exterior_terms.convection_heat_gain_rate_w;
                        sums.outside_convection_heat_gain_rate_per_area_w_per_m2 +=
                            exterior_terms.convection_heat_gain_rate_per_area_w_per_m2;
                        sums.outside_convection_coefficient_w_per_m2_k +=
                            exterior_terms.convection_coefficient_w_per_m2_k;
                        sums.outside_net_thermal_radiation_heat_gain_rate_w +=
                            exterior_terms.net_thermal_radiation_heat_gain_rate_w;
                        sums.outside_net_thermal_radiation_heat_gain_rate_per_area_w_per_m2 +=
                            exterior_terms.net_thermal_radiation_heat_gain_rate_per_area_w_per_m2;
                        sums.outside_thermal_radiation_to_air_coefficient_w_per_m2_k +=
                            exterior_terms.thermal_radiation_to_air_coefficient_w_per_m2_k;
                        sums.outside_thermal_radiation_to_sky_coefficient_w_per_m2_k +=
                            exterior_terms.thermal_radiation_to_sky_coefficient_w_per_m2_k;
                        sums.outside_thermal_radiation_to_ground_coefficient_w_per_m2_k +=
                            exterior_terms.thermal_radiation_to_ground_coefficient_w_per_m2_k;
                        sums.outside_solar_radiation_heat_gain_rate_w +=
                            exterior_terms.solar_radiation_heat_gain_rate_w;
                        sums.outside_solar_radiation_heat_gain_rate_per_area_w_per_m2 +=
                            exterior_terms.solar_radiation_heat_gain_rate_per_area_w_per_m2;
                        let outside_balance = surface_state.outside_balance_diagnostics;
                        sums.outside_balance_report_temperature_c +=
                            outside_balance.report_temperature_c;
                        sums.outside_balance_coefficient_temperature_c +=
                            outside_balance.coefficient_surface_temperature_c;
                        sums.outside_balance_convection_reference_temperature_c +=
                            outside_balance.convection_reference_temperature_c;
                        sums.outside_balance_equivalent_radiant_temperature_c +=
                            outside_balance.equivalent_radiant_temperature_c;
                        sums.outside_balance_radiation_coefficient_w_per_m2_k +=
                            outside_balance.outside_radiation_coefficient_w_per_m2_k;
                        sums.outside_quick_balance_inside_source_term_w_per_m2 +=
                            outside_balance.quick_net_inside_source_w_per_m2;
                        sums.outside_quick_balance_inside_balance_term_w_per_m2 +=
                            outside_balance.quick_inside_balance_term_w_per_m2;
                        sums.outside_quick_balance_numerator_w_per_m2 +=
                            outside_balance.quick_numerator_w_per_m2;
                        sums.outside_quick_balance_denominator_w_per_m2_k +=
                            outside_balance.quick_denominator_w_per_m2_k;
                        sums.outside_quick_balance_coupling_factor +=
                            outside_balance.quick_coupling_factor;
                        sums.inside_conduction_rate_w += inside_rate;
                        sums.inside_conduction_gain_rate_w += heat_gain_rate_w(inside_rate);
                        sums.inside_conduction_loss_rate_w += heat_loss_rate_w(inside_rate);
                        sums.inside_conduction_rate_per_area_w_per_m2 +=
                            surface_rate_per_area_w_per_m2(inside_rate, surface_state.area_m2);
                        sums.ctf_inside_current_outside_term_rate_w +=
                            surface_ctf_inside_current_outside_term_rate_w_for_report(
                                surface_state,
                                use_inside_ctf_outside_temperature_for_conduction_report,
                            );
                        sums.ctf_inside_current_inside_term_rate_w +=
                            surface_ctf_inside_current_inside_term_rate_w(surface_state);
                        sums.ctf_inside_history_term_rate_w +=
                            surface_ctf_inside_history_term_rate_w(surface_state);
                        sums.ctf_inside_history_temperature_term_rate_w +=
                            heat_balance_ctf_history_slot_inside_temperature_term_rate_w(
                                &state.last_ctf_history_slot_terms,
                                &surface_state.surface_name,
                            );
                        sums.ctf_inside_history_flux_term_rate_w +=
                            heat_balance_ctf_history_slot_inside_flux_term_rate_w(
                                &state.last_ctf_history_slot_terms,
                                &surface_state.surface_name,
                            );
                        sums.outside_conduction_rate_w += outside_rate;
                        sums.outside_conduction_gain_rate_w += heat_gain_rate_w(outside_rate);
                        sums.outside_conduction_loss_rate_w += heat_loss_rate_w(outside_rate);
                        sums.outside_conduction_rate_per_area_w_per_m2 +=
                            surface_rate_per_area_w_per_m2(outside_rate, surface_state.area_m2);
                        sums.ctf_outside_current_outside_term_rate_w +=
                            surface_ctf_outside_current_outside_term_rate_w_for_report(
                                surface_state,
                                use_inside_ctf_outside_temperature_for_conduction_report,
                            );
                        sums.ctf_outside_current_inside_term_rate_w +=
                            surface_ctf_outside_current_inside_term_rate_w(surface_state);
                        sums.ctf_outside_history_term_rate_w +=
                            surface_ctf_outside_history_term_rate_w(surface_state);
                        sums.heat_storage_rate_w += storage_rate;
                        sums.heat_storage_rate_per_area_w_per_m2 += storage_rate_per_area;
                    }
                }
            });
        }

        hourly_ctf_history_slots.extend(
            hourly_ctf_history_slot_accumulators
                .into_values()
                .map(|accumulator| accumulator.finalize_hourly(hour_index)),
        );
        hourly_ctf_history_slots_after_advance.extend(
            hourly_ctf_history_slot_after_advance_accumulators
                .into_values()
                .map(|accumulator| accumulator.finalize_hourly(hour_index)),
        );

        let divisor = f64::from(steps);
        push_zone_scalar_trace_averages(&mut zone_temperatures, &zone_temperature_sums, divisor);
        push_zone_scalar_trace_averages(
            &mut zone_humidity_ratios,
            &zone_humidity_ratio_sums,
            divisor,
        );
        push_zone_conduction_trace_averages(
            &mut zone_conduction_rates,
            &zone_conduction_sums,
            divisor,
        );
        inside_surface_iteration_counts.push(inside_surface_iteration_count_sum);
        push_zone_air_heat_balance_trace_values(
            &mut zone_air_heat_balance_rates,
            &zone_air_heat_balance_sums,
            &zone_air_heat_balance_last,
            options.zone_air_report_sampling,
            divisor,
        );
        push_zone_air_debug_trace_averages(
            &mut zone_air_debug_traces,
            &zone_air_debug_sums,
            divisor,
        );
        push_surface_heat_balance_trace_averages(&mut surface_temperatures, &surface_sums, divisor);
        outdoor_temperatures.push(outdoor_temperature_sum / divisor);
        outdoor_wet_bulb_temperatures.push(outdoor_wet_bulb_temperature_sum / divisor);
        sky_temperatures.push(sky_temperature_sum / divisor);
        horizontal_infrared_radiation_rates.push(horizontal_infrared_radiation_sum / divisor);
        rain_statuses.push(rain_status_sum / divisor);
    }

    HeatBalanceRunPeriodSamples {
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
        first_sample_ctf_history_slot_accumulators,
        hourly_ctf_history_slots,
        hourly_ctf_history_slots_after_advance,
        surface_first_sample_trace,
        zone_air_first_sample_trace,
        surface_iteration_first_sample_trace,
        surface_iteration_sample_trace,
    }
}
