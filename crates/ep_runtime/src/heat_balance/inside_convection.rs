//! Inside surface convection coefficient and report helpers.

use crate::heat_balance::convection::energyplus_tarp_inside_convection_coefficient_w_per_m2_k;
use crate::heat_balance::state::{
    InsideConvectionCoefficientInputState, SurfaceHeatBalanceState, ZoneHeatBalanceState,
};
use ep_model::{SurfaceId, ZoneId};
use std::collections::BTreeMap;

/// EnergyPlus source-order owner for inside surface convection reporting.
pub const INSIDE_CONVECTION_OWNER_STAGE: &str = "CalcHeatBalanceInsideSurf";

pub(crate) fn heat_balance_inside_convection_coefficients(
    surfaces: &[SurfaceHeatBalanceState],
    zone_temperatures: &BTreeMap<ZoneId, f64>,
    inside_surface_temperature_overrides: Option<&BTreeMap<SurfaceId, f64>>,
) -> BTreeMap<SurfaceId, f64> {
    surfaces
        .iter()
        .map(|surface| {
            let inside_face_temperature_c = inside_surface_temperature_overrides
                .and_then(|temperatures| temperatures.get(&surface.surface_id).copied())
                .unwrap_or(surface.inside_face_temperature_c);
            let zone_temperature_c = zone_temperatures
                .get(&surface.zone_id)
                .copied()
                .unwrap_or(surface.inside_face_temperature_c);
            (
                surface.surface_id,
                energyplus_tarp_inside_convection_coefficient_w_per_m2_k(
                    surface,
                    inside_face_temperature_c,
                    zone_temperature_c,
                ),
            )
        })
        .collect()
}

pub(crate) fn heat_balance_inside_convection_coefficient_inputs(
    surfaces: &[SurfaceHeatBalanceState],
    zone_temperatures: &BTreeMap<ZoneId, f64>,
    inside_surface_temperature_overrides: Option<&BTreeMap<SurfaceId, f64>>,
) -> BTreeMap<SurfaceId, InsideConvectionCoefficientInputState> {
    surfaces
        .iter()
        .map(|surface| {
            let inside_face_temperature_c = inside_surface_temperature_overrides
                .and_then(|temperatures| temperatures.get(&surface.surface_id).copied())
                .unwrap_or(surface.inside_face_temperature_c);
            let reference_air_temperature_c = zone_temperatures
                .get(&surface.zone_id)
                .copied()
                .unwrap_or(surface.inside_face_temperature_c);
            (
                surface.surface_id,
                InsideConvectionCoefficientInputState {
                    inside_face_temperature_c,
                    reference_air_temperature_c,
                },
            )
        })
        .collect()
}

pub(crate) fn zone_surface_convection_sums(
    surfaces: &[SurfaceHeatBalanceState],
    zone_id: ZoneId,
) -> (f64, f64, f64) {
    let (sum_ha_w_per_k, sum_hat_surf_w) = surfaces
        .iter()
        .filter(|surface| surface.zone_id == zone_id)
        .map(|surface| {
            let surface_ha_w_per_k =
                surface.inside_convection_coefficient_w_per_m2_k * surface.area_m2;
            (
                surface_ha_w_per_k,
                surface_ha_w_per_k * surface.inside_face_temperature_c,
            )
        })
        .fold((0.0, 0.0), |(sum_ha, sum_hat), (ha, hat)| {
            (sum_ha + ha, sum_hat + hat)
        });

    (sum_ha_w_per_k, sum_hat_surf_w, 0.0)
}

pub(crate) fn zone_air_heat_balance_surface_convection_rate_from_surface_reference_air_w(
    surfaces: &[SurfaceHeatBalanceState],
    zone_id: ZoneId,
) -> f64 {
    surfaces
        .iter()
        .filter(|surface| surface.zone_id == zone_id)
        .map(|surface| {
            surface.inside_convection_coefficient_w_per_m2_k
                * surface.area_m2
                * (surface.inside_face_temperature_c - surface.inside_reference_air_temperature_c)
        })
        .sum()
}

pub(crate) fn surface_inside_convection_reference_air_temperature_c(
    surface: &SurfaceHeatBalanceState,
    zones: &[ZoneHeatBalanceState],
    use_surface_reference_air_report: bool,
) -> f64 {
    if use_surface_reference_air_report {
        surface.inside_reference_air_temperature_c
    } else {
        zones
            .iter()
            .find(|zone| zone.zone_id == surface.zone_id)
            .map(|zone| zone.mean_air_temperature_c)
            .unwrap_or(surface.inside_face_temperature_c)
    }
}

pub(crate) fn surface_inside_convection_report_coefficient_w_per_m2_k(
    surface: &SurfaceHeatBalanceState,
    zones: &[ZoneHeatBalanceState],
    use_surface_reference_air_report: bool,
    use_final_inside_convection_report: bool,
) -> f64 {
    if use_final_inside_convection_report {
        let reference_air_temperature_c = surface_inside_convection_reference_air_temperature_c(
            surface,
            zones,
            use_surface_reference_air_report,
        );
        energyplus_tarp_inside_convection_coefficient_w_per_m2_k(
            surface,
            surface.inside_face_temperature_c,
            reference_air_temperature_c,
        )
    } else {
        surface.inside_convection_coefficient_w_per_m2_k
    }
}

pub(crate) fn surface_inside_convection_heat_gain_rate_per_area_w_per_m2(
    surface: &SurfaceHeatBalanceState,
    zones: &[ZoneHeatBalanceState],
    use_surface_reference_air_report: bool,
    use_final_inside_convection_report: bool,
) -> f64 {
    let reference_air_temperature_c = surface_inside_convection_reference_air_temperature_c(
        surface,
        zones,
        use_surface_reference_air_report,
    );
    surface_inside_convection_report_coefficient_w_per_m2_k(
        surface,
        zones,
        use_surface_reference_air_report,
        use_final_inside_convection_report,
    ) * (reference_air_temperature_c - surface.inside_face_temperature_c)
}

pub(crate) fn zone_air_heat_balance_surface_convection_rate_from_final_inside_hconv_report_w(
    surfaces: &[SurfaceHeatBalanceState],
    zones: &[ZoneHeatBalanceState],
    zone_id: ZoneId,
    use_surface_reference_air_report: bool,
) -> f64 {
    surfaces
        .iter()
        .filter(|surface| surface.zone_id == zone_id)
        .map(|surface| {
            let reference_air_temperature_c = surface_inside_convection_reference_air_temperature_c(
                surface,
                zones,
                use_surface_reference_air_report,
            );
            let coefficient_w_per_m2_k = surface_inside_convection_report_coefficient_w_per_m2_k(
                surface,
                zones,
                use_surface_reference_air_report,
                true,
            );
            coefficient_w_per_m2_k
                * surface.area_m2
                * (surface.inside_face_temperature_c - reference_air_temperature_c)
        })
        .sum()
}

pub(crate) fn zone_air_heat_balance_surface_convection_rate_w(
    zone_state: &ZoneHeatBalanceState,
) -> f64 {
    zone_air_heat_balance_surface_convection_rate_at_air_temperature_w(
        zone_state,
        zone_state.mean_air_temperature_c,
    )
}

pub(crate) fn zone_air_heat_balance_surface_convection_rate_at_air_temperature_w(
    zone_state: &ZoneHeatBalanceState,
    reference_air_temperature_c: f64,
) -> f64 {
    zone_state.sum_hat_surf_w
        - zone_state.sum_hat_ref_w
        - zone_state.sum_ha_w_per_k * reference_air_temperature_c
}

pub(crate) fn zone_air_heat_balance_surface_convection_rate_from_balance_w(
    zone_state: &ZoneHeatBalanceState,
    air_storage_rate_w: f64,
) -> f64 {
    air_storage_rate_w - zone_state.convective_internal_gain_w
}
