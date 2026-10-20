//! Actual whole-Zone CeilingDiffuser prepass and its real Node/property children.
use super::*;
use crate::heat_balance::error_backend::SourceDiagnosticFlags;
use crate::heat_balance::selected_heat_balance_input::zone::SelectedZoneData;
use crate::psychrometrics::SourcePsychrometricOwners;
use ep_model::InsideSurfaceConvectionAlgorithm as Inside;

/// InitIntConvCoeff182-207. GetUser has completed before this call. The flag
/// changes before any diagnostic; the live Zone fallback changes only after both.
pub(super) fn check_zone_nodes(
    state: &mut InsideConvectionState,
    zones: &mut SelectedZoneInputOwners,
    readiness: super::super::InsideConvectionReadiness,
    globals: &SelectedHeatBalanceGlobals,
    clock: &WeatherGlobalState,
    developer: &bool,
    backend: &mut SourceErrorBackend<'_>,
) -> Result<(), Error> {
    if state.node_check
        && !readiness.system_sizing
        && !readiness.zone_sizing
        && readiness.equipment_inputs_filled
        && readiness.nodes_allocated
    {
        state.node_check = false;
        for number in 1..=globals.num_of_zones {
            let at = index(number)?;
            let zone = zones
                .zones
                .as_mut()
                .and_then(|rows| rows.get_mut(at))
                .ok_or(Error::Unavailable("actual CeilingDiffuser NodeCheck Zone"))?;
            if zone.inside_algorithm != Inside::CeilingDiffuser || zone.system_zone_node_number != 0
            {
                continue;
            }
            let flags = SourceDiagnosticFlags {
                kickoff: &globals.kickoff_simulation,
                developer,
                warmup: &clock.warmup_flag,
                doing_sizing: &globals.doing_sizing,
            };
            backend.show_source_severe_error(&format!(
                "InitInteriorConvectionCoeffs: Inside Convection=CeilingDiffuser, but no system inlet node defined, Zone={}", zone.name), &flags)?;
            backend.show_continue_error(&format!(
                "Defaulting inside convection to TARP. Check ZoneHVAC:EquipmentConnections for Zone={}", zone.name))?;
            zone.inside_algorithm = Inside::Tarp;
        }
    }
    Ok(())
}

fn multiplier(zone: &SelectedZoneData) -> Result<i32, Error> {
    // Both Original sites multiply the two integer owners before Real64 conversion.
    zone.multiplier
        .checked_mul(zone.list_multiplier)
        .ok_or(Error::Unavailable(
            "actual CeilingDiffuser integer Zone multiplier product",
        ))
}

/// Objexx min(a,b) is a<b?a:b, distinct from std::min for unordered operands.
fn limit_ach(mut ach: f64, mass_flow: f64) -> f64 {
    if mass_flow < 0.01 {
        ach = 0.0;
    } else {
        ach = if ach < 100.0 { ach } else { 100.0 };
        ach = if ach < 0.0 { 0.0 } else { ach };
    }
    ach
}

/// CalcZoneSystemACH2050 -> CalcZoneSystemVolFlowRate2108 -> MinFlow2123.
/// Property diagnostics happen even when the eventual mass flow gate is zero.
pub(super) fn ach(
    zone: &SelectedZoneData,
    equipment: &OrdinaryEquipment,
    globals: &SelectedHeatBalanceGlobals,
    clock: &WeatherGlobalState,
    properties: &mut SourceInsideConvectionPropertyOwners<'_>,
    backend: &mut SourceErrorBackend<'_>,
) -> Result<f64, Error> {
    let ach = if equipment
        .source_convection_readiness(
            clock.begin_envrn_flag,
            globals.system_sizing_calculation,
            globals.zone_sizing_calculation,
        )
        .nodes_allocated
    {
        let volume = zone.volume;
        let volume_flow = if clock.begin_envrn_flag || zone.system_zone_node_number <= 0 {
            0.0
        } else {
            let node = equipment.source_zone_air_node(zone.system_zone_node_number)?;
            let zone_mult = multiplier(zone)?;
            let pressure = properties.environment.current_weather.out_baro_press;
            let mut owners = SourcePsychrometricOwners {
                clock,
                globals,
                environment: properties.environment,
                hvac: properties.hvac,
                recurring: &mut *properties.recurring,
                backend,
            };
            let humidity = properties.psychrometrics.source_dew_point_humidity_ratio(
                *node.temperature,
                pressure,
                "",
                &mut owners,
            )?;
            let density = properties.psychrometrics.source_air_density(
                pressure,
                *node.temperature,
                humidity,
                "",
                properties.developer,
                &mut owners,
            )?;
            *node.mass_flow_rate / (density * f64::from(zone_mult))
        };
        volume_flow / volume * 3600.0
    } else {
        0.0
    };
    let zone_mult = f64::from(multiplier(zone)?);
    let mass_flow = if !clock.begin_envrn_flag && zone.system_zone_node_number > 0 {
        *equipment
            .source_zone_air_node(zone.system_zone_node_number)?
            .mass_flow_rate
            / zone_mult
    } else {
        0.0
    };
    Ok(limit_ach(ach, mass_flow))
}

/// Original2152/4945-5041. Window ISO15099 remains a reached explicit boundary.
fn coefficient(
    ach: f64,
    surface_temperature: f64,
    air_temperature: f64,
    cos_tilt: f64,
    _humidity: f64,
    _height: f64,
    is_window: bool,
) -> Result<f64, Error> {
    let cos45 = 2.0_f64.sqrt() / 2.0;
    let (offset, scale, exponent, forced) = if cos_tilt < -cos45 {
        (3.873, 0.082, 0.98, 4.11365377688938)
    } else if cos_tilt > cos45 {
        (2.234, 4.099, 0.503, 9.35711423763866)
    } else {
        (1.208, 1.012, 0.604, 3.17299636062606)
    };
    if ach >= 3.0 {
        return Ok(offset + scale * ach.powf(exponent));
    }
    if is_window {
        return Err(Error::Unavailable(
            "actual CeilingDiffuser ISO15099 window child",
        ));
    }
    let natural = super::super::natural_coefficient_w_per_m2_k(
        surface_temperature,
        air_temperature,
        -cos_tilt,
    );
    if ach <= 0.5 {
        return Ok(natural);
    }
    Ok(natural + ((forced - natural) * ((ach - 0.5) / 2.5)))
}

/// Literal NumOfZones prepass275 and each real Zone.spaceIndexes/HT range2196.
/// No Space MAT, typed fallback enum, shadow coefficient, or high limit is used.
#[allow(clippy::too_many_arguments)]
pub(super) fn initialize<
    H: InsideConvectionCoefficients + ?Sized,
    T: InsideSurfaceTemperatureValues + ?Sized,
>(
    geometry: &SurfaceGeometryInputState,
    zones: &SelectedZoneInputOwners,
    air: &SourceAirHeatBalance,
    surfaces: &[InsideConvectionSurface],
    temperatures: &T,
    coefficients: &mut H,
    equipment: &OrdinaryEquipment,
    globals: &SelectedHeatBalanceGlobals,
    clock: &WeatherGlobalState,
    properties: &mut SourceInsideConvectionPropertyOwners<'_>,
    low_limit: f64,
    backend: &mut SourceErrorBackend<'_>,
) -> Result<(), Error> {
    for number in 1..=globals.num_of_zones {
        let at = index(number)?;
        let zone = zones
            .zones
            .as_ref()
            .and_then(|rows| rows.get(at))
            .ok_or(Error::Unavailable("actual CeilingDiffuser Zone prepass"))?;
        if zone.inside_algorithm != Inside::CeilingDiffuser {
            continue;
        }
        let ach = ach(zone, equipment, globals, clock, properties, backend)?;
        let zone_air = &air
            .zones
            .as_ref()
            .and_then(|rows| rows.get(at))
            .ok_or(Error::Unavailable(
                "actual CeilingDiffuser zoneHeatBalance allocation",
            ))?
            .air;
        let humidity = zone_air.air_hum_rat_avg;
        for &space_number in &zone.space_indexes {
            let space_at = index(space_number)?;
            let space = zones
                .spaces
                .as_ref()
                .and_then(|rows| rows.get(space_at))
                .ok_or(Error::Unavailable(
                    "actual CeilingDiffuser Zone.spaceIndexes",
                ))?;
            for surface_number in space.ht_surface_first..=space.ht_surface_last {
                let i = index(surface_number)?;
                let surface = geometry
                    .final_surfaces
                    .as_ref()
                    .and_then(|rows| rows.get(i))
                    .ok_or(Error::Unavailable("actual CeilingDiffuser HTSurface range"))?;
                if surface.row.exterior_boundary == -6 {
                    return Err(Error::Unavailable(
                        "actual CeilingDiffuser Kiva callback child",
                    ));
                }
                let view = surfaces
                    .get(i)
                    .ok_or(Error::Unavailable("actual CeilingDiffuser static surface"))?;
                coefficients.set_coefficient(
                    i,
                    coefficient(
                        ach,
                        *temperatures
                            .borrow_temperature(i)
                            .map_err(|e| translate(e.into()))?,
                        zone_air.mat,
                        view.outward_cos_tilt,
                        humidity,
                        surface.row.vertex_progress.height,
                        view.construction_is_window,
                    )?,
                );
                if *coefficients
                    .borrow_coefficient(i)
                    .map_err(|e| translate(e.into()))?
                    < low_limit
                {
                    coefficients.set_coefficient(i, low_limit);
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;
