//! Surface boundary balance and exterior report-term helpers.

use crate::geometry::surface_tilt_deg;
use crate::heat_balance::algorithm::{
    HeatBalanceZoneAirAlgorithm, heat_balance_zone_air_algorithm_feature_base,
};
use crate::heat_balance::convection::{
    ExteriorConvectionTerms, energyplus_building_terrain, energyplus_exterior_convection_terms,
    energyplus_surface_outdoor_air_temperature_c, heat_balance_uses_doe2_outside_convection,
};
use crate::heat_balance::ctf::{
    CtfOutsideFaceBalanceInput, CtfOutsideQuickConductionBalanceCalculation,
    CtfOutsideQuickConductionBalanceInput, energyplus_ctf_outside_face_temperature_c,
    energyplus_ctf_outside_face_temperature_quick_conduction_calculation,
};
use crate::heat_balance::longwave::{ExteriorLongwaveTerms, energyplus_exterior_longwave_terms};
use crate::heat_balance::solar::surface_incident_solar_radiation_for_weather_context_w_per_m2;
use crate::heat_balance::state::{
    SurfaceBoundaryBalanceResult, SurfaceExteriorReportTerms, SurfaceHeatBalanceState,
    SurfaceOutsideBalanceDiagnostics,
};
use crate::heat_balance::surface_boundary::surface_boundary_temperature_c;
use crate::heat_balance::surface_weather::{
    energyplus_exterior_wet_context_fraction, energyplus_exterior_wet_reference_temperature_c,
};
use crate::weather::{
    EpwRecord, HeatBalanceWeatherContext, energyplus_weather_horizontal_infrared_for_context,
    energyplus_weather_wind_direction_for_context, energyplus_weather_wind_speed_for_context,
};
use ep_model::{
    OutsideBoundaryCondition, SunExposure, Surface, SurfaceType, Terrain, TypedModel, ZoneId,
};
use std::collections::BTreeMap;

/// EnergyPlus source-order owner for outside surface balance calculation.
pub const SURFACE_BALANCE_OWNER_STAGE: &str = "CalcHeatBalanceOutsideSurf";

const EXTERIOR_SOLAR_FORCING_THRESHOLD_W_PER_M2: f64 = 300.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct QuickOutsideConductionContext {
    pub(crate) reference_air_temperature_c: f64,
    pub(crate) inside_convection_coefficient_w_per_m2_k: f64,
    pub(crate) net_inside_source_w_per_m2: f64,
    pub(crate) exterior_coefficient_surface_temperature_c: Option<f64>,
    pub(crate) use_doe2_outside_convection: bool,
}

pub(crate) fn heat_balance_surface_boundary_balance(
    model: &TypedModel,
    surface: &SurfaceHeatBalanceState,
    previous_zone_temperatures: &BTreeMap<ZoneId, f64>,
    outdoor_dry_bulb_c: f64,
    owning_zone_temperature_c: f64,
    weather_context: Option<HeatBalanceWeatherContext<'_>>,
    quick_outside_conduction: Option<QuickOutsideConductionContext>,
    use_doe2_outside_convection: bool,
) -> SurfaceBoundaryBalanceResult {
    if surface.outside_boundary_condition == OutsideBoundaryCondition::Outdoors {
        return exterior_surface_boundary_balance(
            model,
            surface,
            outdoor_dry_bulb_c,
            owning_zone_temperature_c,
            weather_context,
            quick_outside_conduction,
            use_doe2_outside_convection,
        );
    }

    SurfaceBoundaryBalanceResult {
        temperature_c: surface_boundary_temperature_c(
            surface,
            previous_zone_temperatures,
            outdoor_dry_bulb_c,
            owning_zone_temperature_c,
        ),
        exterior_report_terms: SurfaceExteriorReportTerms::default(),
        outside_balance_diagnostics: SurfaceOutsideBalanceDiagnostics::default(),
    }
}

pub(crate) fn exterior_surface_boundary_temperature_c(
    model: &TypedModel,
    surface_state: &SurfaceHeatBalanceState,
    outdoor_dry_bulb_c: f64,
    owning_zone_temperature_c: f64,
    weather_context: Option<HeatBalanceWeatherContext<'_>>,
    quick_outside_conduction: Option<QuickOutsideConductionContext>,
    use_doe2_outside_convection: bool,
) -> f64 {
    exterior_surface_boundary_balance(
        model,
        surface_state,
        outdoor_dry_bulb_c,
        owning_zone_temperature_c,
        weather_context,
        quick_outside_conduction,
        use_doe2_outside_convection,
    )
    .temperature_c
}

pub(crate) fn exterior_surface_boundary_balance(
    model: &TypedModel,
    surface_state: &SurfaceHeatBalanceState,
    outdoor_dry_bulb_c: f64,
    owning_zone_temperature_c: f64,
    weather_context: Option<HeatBalanceWeatherContext<'_>>,
    quick_outside_conduction: Option<QuickOutsideConductionContext>,
    use_doe2_outside_convection: bool,
) -> SurfaceBoundaryBalanceResult {
    let Some(context) = weather_context else {
        return SurfaceBoundaryBalanceResult {
            temperature_c: outdoor_dry_bulb_c,
            exterior_report_terms: SurfaceExteriorReportTerms::default(),
            outside_balance_diagnostics: SurfaceOutsideBalanceDiagnostics::default(),
        };
    };
    let Some(record) = context.records.get(context.record_index) else {
        return SurfaceBoundaryBalanceResult {
            temperature_c: outdoor_dry_bulb_c,
            exterior_report_terms: SurfaceExteriorReportTerms::default(),
            outside_balance_diagnostics: SurfaceOutsideBalanceDiagnostics::default(),
        };
    };
    let Some(typed_surface) = model
        .surfaces
        .iter()
        .find(|surface| surface.id == surface_state.surface_id)
    else {
        return SurfaceBoundaryBalanceResult {
            temperature_c: outdoor_dry_bulb_c,
            exterior_report_terms: SurfaceExteriorReportTerms::default(),
            outside_balance_diagnostics: SurfaceOutsideBalanceDiagnostics::default(),
        };
    };
    if !matches!(
        typed_surface.surface_type,
        SurfaceType::Roof | SurfaceType::Wall
    ) {
        return SurfaceBoundaryBalanceResult {
            temperature_c: outdoor_dry_bulb_c,
            exterior_report_terms: SurfaceExteriorReportTerms::default(),
            outside_balance_diagnostics: SurfaceOutsideBalanceDiagnostics::default(),
        };
    }
    let wet_timestep_fraction = energyplus_exterior_wet_context_fraction(context, typed_surface);
    let weather_file_wind_speed_m_per_s =
        energyplus_weather_wind_speed_for_context(context, record.wind_speed_m_per_s);
    let wind_direction_deg =
        energyplus_weather_wind_direction_for_context(context, record.wind_direction_deg);
    let horizontal_infrared_radiation_w_per_m2 = energyplus_weather_horizontal_infrared_for_context(
        context,
        record.horizontal_infrared_radiation_wh_per_m2,
    );
    let surface_outdoor_dry_bulb_c =
        energyplus_surface_outdoor_air_temperature_c(typed_surface, outdoor_dry_bulb_c);
    let wet_reference_temperature_c = energyplus_surface_outdoor_air_temperature_c(
        typed_surface,
        energyplus_exterior_wet_reference_temperature_c(context, outdoor_dry_bulb_c),
    );

    let incident_solar_w_per_m2 = if typed_surface.sun_exposure == SunExposure::SunExposed {
        let Some(site) = model.site.as_ref() else {
            return exterior_surface_energy_balance(
                surface_state,
                typed_surface,
                record,
                surface_outdoor_dry_bulb_c,
                owning_zone_temperature_c,
                0.0,
                energyplus_building_terrain(model),
                weather_file_wind_speed_m_per_s,
                wind_direction_deg,
                horizontal_infrared_radiation_w_per_m2,
                quick_outside_conduction,
                use_doe2_outside_convection,
                wet_reference_temperature_c,
                wet_timestep_fraction,
                quick_outside_conduction
                    .and_then(|context| context.exterior_coefficient_surface_temperature_c),
            );
        };
        surface_incident_solar_radiation_for_weather_context_w_per_m2(
            typed_surface,
            site,
            context.records,
            context.record_index,
            context.zone_steps_per_hour,
            context.zone_timestep,
            context.first_hour_interpolation_starting_values,
        )
    } else {
        0.0
    };
    exterior_surface_energy_balance(
        surface_state,
        typed_surface,
        record,
        surface_outdoor_dry_bulb_c,
        owning_zone_temperature_c,
        incident_solar_w_per_m2,
        energyplus_building_terrain(model),
        weather_file_wind_speed_m_per_s,
        wind_direction_deg,
        horizontal_infrared_radiation_w_per_m2,
        quick_outside_conduction,
        use_doe2_outside_convection,
        wet_reference_temperature_c,
        wet_timestep_fraction,
        quick_outside_conduction
            .and_then(|context| context.exterior_coefficient_surface_temperature_c),
    )
}

pub(crate) fn reported_surface_outside_face_temperature_c(
    model: &TypedModel,
    surface_state: &SurfaceHeatBalanceState,
    outdoor_dry_bulb_c: f64,
    owning_zone_temperature_c: f64,
    weather_context: Option<HeatBalanceWeatherContext<'_>>,
    zone_air_algorithm: HeatBalanceZoneAirAlgorithm,
) -> f64 {
    let zone_air_algorithm = heat_balance_zone_air_algorithm_feature_base(zone_air_algorithm);
    if surface_state.outside_boundary_condition != OutsideBoundaryCondition::Outdoors {
        return surface_state.outside_face_temperature_c;
    }
    if matches!(
        zone_air_algorithm,
        HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvCurrentLongwaveProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvCurrentAdiabaticProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStoragePreviousMatSurfaceConvectionProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceFrozenOutsideProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceAdiabaticHistoryCommitProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionCurrentAdiabaticHistoryProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedScriptFInteriorLongwaveProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2Probe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInteriorLongwaveProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2InteriorLongwaveProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideScriptFInteriorLongwaveProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2ScriptFInteriorLongwaveProbe
    ) {
        return surface_state.outside_face_temperature_c;
    }

    exterior_surface_boundary_temperature_c(
        model,
        surface_state,
        outdoor_dry_bulb_c,
        owning_zone_temperature_c,
        weather_context,
        None,
        heat_balance_uses_doe2_outside_convection(model, zone_air_algorithm),
    )
}

pub(crate) fn surface_exterior_report_terms(
    model: &TypedModel,
    surface_state: &SurfaceHeatBalanceState,
    outdoor_dry_bulb_c: f64,
    reported_outside_face_temperature_c: f64,
    weather_context: Option<HeatBalanceWeatherContext<'_>>,
    zone_air_algorithm: HeatBalanceZoneAirAlgorithm,
) -> SurfaceExteriorReportTerms {
    if surface_state.outside_boundary_condition != OutsideBoundaryCondition::Outdoors
        || surface_state.area_m2 <= 0.0
    {
        return SurfaceExteriorReportTerms::default();
    }
    if heat_balance_uses_cached_exterior_report_terms(zone_air_algorithm) {
        return surface_state.outside_report_terms;
    }
    let Some(context) = weather_context else {
        return SurfaceExteriorReportTerms::default();
    };
    let Some(record) = context.records.get(context.record_index) else {
        return SurfaceExteriorReportTerms::default();
    };
    let Some(typed_surface) = model
        .surfaces
        .iter()
        .find(|surface| surface.id == surface_state.surface_id)
    else {
        return SurfaceExteriorReportTerms::default();
    };

    let incident_solar_w_per_m2 = if typed_surface.sun_exposure == SunExposure::SunExposed {
        model
            .site
            .as_ref()
            .map(|site| {
                surface_incident_solar_radiation_for_weather_context_w_per_m2(
                    typed_surface,
                    site,
                    context.records,
                    context.record_index,
                    context.zone_steps_per_hour,
                    context.zone_timestep,
                    context.first_hour_interpolation_starting_values,
                )
            })
            .unwrap_or(0.0)
    } else {
        0.0
    };
    let solar_gain_per_area_w_per_m2 =
        surface_state.solar_absorptance.clamp(0.0, 1.0) * incident_solar_w_per_m2.max(0.0);

    let tilt_rad =
        surface_tilt_deg(typed_surface.surface_type, &typed_surface.vertices).to_radians();
    let use_doe2_outside_convection =
        heat_balance_uses_doe2_outside_convection(model, zone_air_algorithm);
    let wet_timestep_fraction = energyplus_exterior_wet_context_fraction(context, typed_surface);
    let weather_file_wind_speed_m_per_s =
        energyplus_weather_wind_speed_for_context(context, record.wind_speed_m_per_s);
    let wind_direction_deg =
        energyplus_weather_wind_direction_for_context(context, record.wind_direction_deg);
    let horizontal_infrared_radiation_w_per_m2 = energyplus_weather_horizontal_infrared_for_context(
        context,
        record.horizontal_infrared_radiation_wh_per_m2,
    );
    let surface_outdoor_dry_bulb_c =
        energyplus_surface_outdoor_air_temperature_c(typed_surface, outdoor_dry_bulb_c);
    let wet_reference_temperature_c = energyplus_surface_outdoor_air_temperature_c(
        typed_surface,
        energyplus_exterior_wet_reference_temperature_c(context, outdoor_dry_bulb_c),
    );
    let convection_terms = energyplus_exterior_convection_terms(
        surface_state,
        typed_surface,
        reported_outside_face_temperature_c,
        surface_outdoor_dry_bulb_c,
        tilt_rad,
        energyplus_building_terrain(model),
        weather_file_wind_speed_m_per_s,
        wind_direction_deg,
        use_doe2_outside_convection,
        wet_reference_temperature_c,
        wet_timestep_fraction,
    );
    let longwave_terms = energyplus_exterior_longwave_terms(
        surface_state,
        typed_surface,
        horizontal_infrared_radiation_w_per_m2,
        reported_outside_face_temperature_c,
        convection_terms.reference_temperature_c,
        surface_outdoor_dry_bulb_c,
        tilt_rad,
    );

    surface_exterior_report_terms_from_balance(
        surface_state,
        reported_outside_face_temperature_c,
        solar_gain_per_area_w_per_m2,
        ExteriorConvectionTerms {
            coefficient_w_per_m2_k: convection_terms.coefficient_w_per_m2_k,
            reference_temperature_c: convection_terms.reference_temperature_c,
        },
        longwave_terms,
    )
}

pub(crate) fn surface_exterior_report_terms_from_balance(
    surface_state: &SurfaceHeatBalanceState,
    outside_face_temperature_c: f64,
    solar_gain_per_area_w_per_m2: f64,
    convection_terms: ExteriorConvectionTerms,
    longwave_terms: ExteriorLongwaveTerms,
) -> SurfaceExteriorReportTerms {
    let convection_gain_per_area_w_per_m2 = -convection_terms.coefficient_w_per_m2_k
        * (outside_face_temperature_c - convection_terms.reference_temperature_c);
    let net_radiation_gain_per_area_w_per_m2 =
        longwave_terms.net_heat_gain_per_area_w_per_m2(outside_face_temperature_c);

    SurfaceExteriorReportTerms {
        convection_heat_gain_rate_w: convection_gain_per_area_w_per_m2 * surface_state.area_m2,
        convection_heat_gain_rate_per_area_w_per_m2: convection_gain_per_area_w_per_m2,
        convection_coefficient_w_per_m2_k: convection_terms.coefficient_w_per_m2_k,
        net_thermal_radiation_heat_gain_rate_w: net_radiation_gain_per_area_w_per_m2
            * surface_state.area_m2,
        net_thermal_radiation_heat_gain_rate_per_area_w_per_m2:
            net_radiation_gain_per_area_w_per_m2,
        thermal_radiation_to_air_coefficient_w_per_m2_k: longwave_terms.air_coefficient_w_per_m2_k,
        thermal_radiation_to_sky_coefficient_w_per_m2_k: longwave_terms.sky_coefficient_w_per_m2_k,
        thermal_radiation_to_ground_coefficient_w_per_m2_k: longwave_terms
            .ground_coefficient_w_per_m2_k,
        solar_radiation_heat_gain_rate_w: solar_gain_per_area_w_per_m2 * surface_state.area_m2,
        solar_radiation_heat_gain_rate_per_area_w_per_m2: solar_gain_per_area_w_per_m2,
    }
}

pub(crate) fn heat_balance_uses_cached_exterior_report_terms(
    zone_air_algorithm: HeatBalanceZoneAirAlgorithm,
) -> bool {
    let zone_air_algorithm = heat_balance_zone_air_algorithm_feature_base(zone_air_algorithm);
    matches!(
        zone_air_algorithm,
        HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvCurrentLongwaveProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvCurrentAdiabaticProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStoragePreviousMatSurfaceConvectionProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceFrozenOutsideProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceAdiabaticHistoryCommitProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionCurrentAdiabaticHistoryProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2Probe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInteriorLongwaveProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2InteriorLongwaveProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideScriptFInteriorLongwaveProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2ScriptFInteriorLongwaveProbe
    )
}

/// EnergyPlus engineering-reference inside-face heat-balance terms.
///
/// All fields are per-area terms in W/m2. The q_* names mirror the
/// engineering-reference notation while preserving the Rust report slots that
/// distinguish longwave, shortwave, conduction, and convection terms.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct InsideFaceBalanceEquationTerms {
    /// Internal longwave exchange, EnergyPlus `SurfQdotRadNetLWInPerArea`.
    pub(crate) q_lwx_w_per_m2: f64,
    /// Shortwave radiation from lights/internal sources.
    pub(crate) q_sw_w_per_m2: f64,
    /// Longwave radiation from internal sources/equipment/people.
    pub(crate) q_lws_w_per_m2: f64,
    /// Conduction flux to the inside face.
    pub(crate) q_ki_w_per_m2: f64,
    /// Solar/shortwave radiation absorbed at the inside face.
    pub(crate) q_sol_w_per_m2: f64,
    /// Convection exchange with zone air.
    pub(crate) q_conv_w_per_m2: f64,
    /// Additional inside heat source term.
    pub(crate) q_additional_inside_heat_source_w_per_m2: f64,
    /// Radiant HVAC source term.
    pub(crate) q_radiant_hvac_w_per_m2: f64,
}

impl InsideFaceBalanceEquationTerms {
    pub(crate) fn non_convective_source_w_per_m2(self) -> f64 {
        self.q_lwx_w_per_m2
            + self.q_sw_w_per_m2
            + self.q_lws_w_per_m2
            + self.q_sol_w_per_m2
            + self.q_additional_inside_heat_source_w_per_m2
            + self.q_radiant_hvac_w_per_m2
    }

    pub(crate) fn signed_balance_terms_w_per_m2(self) -> f64 {
        self.non_convective_source_w_per_m2() + self.q_ki_w_per_m2 + self.q_conv_w_per_m2
    }
}

pub(crate) fn surface_inside_face_balance_equation_terms_w_per_m2(
    surface: &SurfaceHeatBalanceState,
    q_ki_w_per_m2: f64,
    q_conv_w_per_m2: f64,
) -> InsideFaceBalanceEquationTerms {
    InsideFaceBalanceEquationTerms {
        q_lwx_w_per_m2: surface.inside_net_longwave_w_per_m2,
        q_sw_w_per_m2: 0.0,
        q_lws_w_per_m2: surface.inside_radiant_internal_gain_w_per_m2,
        q_ki_w_per_m2,
        q_sol_w_per_m2: surface.inside_shortwave_absorbed_w_per_m2,
        q_conv_w_per_m2,
        q_additional_inside_heat_source_w_per_m2: surface.inside_additional_heat_source_w_per_m2,
        q_radiant_hvac_w_per_m2: surface.inside_radiant_hvac_w_per_m2,
    }
}

pub(crate) fn surface_inside_ctf_source_terms_w_per_m2(surface: &SurfaceHeatBalanceState) -> f64 {
    surface_inside_face_balance_equation_terms_w_per_m2(surface, 0.0, 0.0)
        .non_convective_source_w_per_m2()
}

pub(crate) fn exterior_surface_energy_balance(
    surface_state: &SurfaceHeatBalanceState,
    typed_surface: &Surface,
    _record: &EpwRecord,
    outdoor_dry_bulb_c: f64,
    _owning_zone_temperature_c: f64,
    incident_solar_w_per_m2: f64,
    terrain: Terrain,
    weather_file_wind_speed_m_per_s: f64,
    wind_direction_deg: f64,
    horizontal_infrared_radiation_w_per_m2: f64,
    quick_outside_conduction: Option<QuickOutsideConductionContext>,
    use_doe2_outside_convection: bool,
    wet_reference_temperature_c: f64,
    wet_timestep_fraction: f64,
    exterior_coefficient_surface_temperature_c: Option<f64>,
) -> SurfaceBoundaryBalanceResult {
    if quick_outside_conduction.is_none() {
        if wet_timestep_fraction <= f64::EPSILON
            && incident_solar_w_per_m2 < EXTERIOR_SOLAR_FORCING_THRESHOLD_W_PER_M2
        {
            return SurfaceBoundaryBalanceResult {
                temperature_c: outdoor_dry_bulb_c,
                exterior_report_terms: SurfaceExteriorReportTerms::default(),
                outside_balance_diagnostics: SurfaceOutsideBalanceDiagnostics::default(),
            };
        }
    }

    let solar_absorptance = surface_state.solar_absorptance.clamp(0.0, 1.0);
    let solar_gain_per_area_w_per_m2 = solar_absorptance * incident_solar_w_per_m2.max(0.0);
    let tilt_rad =
        surface_tilt_deg(typed_surface.surface_type, &typed_surface.vertices).to_radians();
    let coefficient_surface_temperature_c = exterior_coefficient_surface_temperature_c
        .unwrap_or(surface_state.outside_face_temperature_c);
    let use_doe2_outside_convection = use_doe2_outside_convection
        || quick_outside_conduction
            .map(|context| context.use_doe2_outside_convection)
            .unwrap_or(false);
    let convection_terms = energyplus_exterior_convection_terms(
        surface_state,
        typed_surface,
        coefficient_surface_temperature_c,
        outdoor_dry_bulb_c,
        tilt_rad,
        terrain,
        weather_file_wind_speed_m_per_s,
        wind_direction_deg,
        use_doe2_outside_convection,
        wet_reference_temperature_c,
        wet_timestep_fraction,
    );
    let longwave_terms = energyplus_exterior_longwave_terms(
        surface_state,
        typed_surface,
        horizontal_infrared_radiation_w_per_m2,
        coefficient_surface_temperature_c,
        convection_terms.reference_temperature_c,
        outdoor_dry_bulb_c,
        tilt_rad,
    );

    let environmental = CtfOutsideFaceBalanceInput {
        outdoor_air_temperature_c: convection_terms.reference_temperature_c,
        radiant_temperature_c: longwave_terms
            .equivalent_radiant_temperature_c(convection_terms.reference_temperature_c),
        outside_convection_coefficient_w_per_m2_k: convection_terms.coefficient_w_per_m2_k,
        outside_radiation_coefficient_w_per_m2_k: longwave_terms
            .equivalent_coefficient_w_per_m2_k(),
        absorbed_outside_source_w_per_m2: solar_absorptance * incident_solar_w_per_m2.max(0.0),
    };
    let (temperature_c, quick_net_inside_source_w_per_m2, quick_calculation) =
        if let Some(context) = quick_outside_conduction {
            let quick_input = CtfOutsideQuickConductionBalanceInput {
                environmental,
                reference_air_temperature_c: context.reference_air_temperature_c,
                inside_convection_coefficient_w_per_m2_k: context
                    .inside_convection_coefficient_w_per_m2_k,
                net_inside_source_w_per_m2: context.net_inside_source_w_per_m2,
            };
            let calculation = energyplus_ctf_outside_face_temperature_quick_conduction_calculation(
                surface_state,
                quick_input,
            );
            (
                calculation.temperature_c,
                context.net_inside_source_w_per_m2,
                calculation,
            )
        } else {
            (
                energyplus_ctf_outside_face_temperature_c(surface_state, environmental),
                0.0,
                CtfOutsideQuickConductionBalanceCalculation::default(),
            )
        };
    let exterior_report_terms = surface_exterior_report_terms_from_balance(
        surface_state,
        temperature_c,
        solar_gain_per_area_w_per_m2,
        convection_terms,
        longwave_terms,
    );
    let outside_balance_diagnostics = SurfaceOutsideBalanceDiagnostics {
        report_temperature_c: temperature_c,
        coefficient_surface_temperature_c,
        convection_reference_temperature_c: environmental.outdoor_air_temperature_c,
        wet_timestep_fraction: wet_timestep_fraction.clamp(0.0, 1.0),
        wet_reference_temperature_c,
        equivalent_radiant_temperature_c: environmental.radiant_temperature_c,
        outside_radiation_coefficient_w_per_m2_k: environmental
            .outside_radiation_coefficient_w_per_m2_k,
        quick_net_inside_source_w_per_m2,
        quick_inside_balance_term_w_per_m2: quick_calculation.inside_balance_term_w_per_m2,
        quick_numerator_w_per_m2: quick_calculation.numerator_w_per_m2,
        quick_denominator_w_per_m2_k: quick_calculation.denominator_w_per_m2_k,
        quick_coupling_factor: quick_calculation.coupling_factor,
    };

    SurfaceBoundaryBalanceResult {
        temperature_c,
        exterior_report_terms,
        outside_balance_diagnostics,
    }
}
