//! Consumed Zone outdoor setters followed by the existing Surface setters.
//! The same retained weather session supplies both Station/Site and Current.
use super::*;
use crate::heat_balance::selected_heat_balance_input::zone::SelectedZoneData;

pub(super) fn invoke(
    model: &SimulationModel,
    state: &mut HeatBalanceState,
    weather: Option<HeatBalanceWeatherContext<'_>>,
    driver: Option<HeatBalanceWeatherDriver<'_>>,
    output: Option<&mut CtfInitializationContext<'_>>,
) -> Result<(), RuntimeError> {
    let actual = weather
        .and_then(|value| value.owned)
        .ok_or_else(|| unavailable("Zone outdoor requires actual current production weather"))?;
    let clock = actual
        .ctf_history_clock
        .ok_or_else(|| unavailable("Zone outdoor requires actual caller WarmupFlag"))?;
    let Some(HeatBalanceWeatherDriver::Production(series)) = driver else {
        return Err(unavailable(
            "Source outdoor requires the retained production weather session",
        ));
    };
    let output =
        output.ok_or_else(|| unavailable("Zone outdoor requires the actual error backend"))?;
    validate_surfaces(model, state)?;
    let globals = state
        .selected_heat_balance_globals
        .as_ref()
        .ok_or_else(|| unavailable("missing Source outdoor globals"))?;
    if globals.any_energy_management_system_in_model || state.ordinary_equipment.is_none() {
        return Err(unavailable(
            "Source outdoor requires admitted non-EMS geometry equipment input",
        ));
    }
    let doing_sizing = globals.doing_sizing;
    let zones = state
        .selected_zone_input
        .as_mut()
        .and_then(|owner| owner.zones.as_mut())
        .ok_or_else(|| unavailable("Source outdoor before actual Zone allocation"))?;
    let ids = zones
        .iter()
        .filter_map(|zone| zone.typed_zone_id)
        .collect::<std::collections::BTreeSet<_>>();
    if zones.len() != model.typed.zones.len()
        || ids.len() != zones.len()
        || state.zones.len() != zones.len()
        || state
            .zones
            .iter()
            .map(|zone| zone.zone_id)
            .collect::<std::collections::BTreeSet<_>>()
            != ids
        || zones.iter().any(|zone| {
            zone.centroid_m.is_none()
                || zone.linked_outdoor_air_node != 0
                || !model.typed.zones.iter().any(|typed| {
                    Some(typed.id) == zone.typed_zone_id
                        && typed.name == ep_model::NormalizedName::new(&zone.name)
                })
        })
    {
        return Err(unavailable(
            "Source outdoor Zone identity/centroid/linked-node owner changed",
        ));
    }
    series.with_source_environment_mut(|environment, _minutes| {
        if !environment.source_weather_station_initialized
            || !environment.source_site_atmosphere_initialized
        {
            return Err(unavailable(
                "Source outdoor before real Station/Site returns",
            ));
        }
        let current = environment.current_weather;
        if [
            current.out_dry_bulb_temp,
            current.out_wet_bulb_temp,
            current.wind_speed,
            current.wind_dir,
        ]
        .map(f64::to_bits)
            != [
                actual.current_weather.out_dry_bulb_temp,
                actual.current_weather.out_wet_bulb_temp,
                actual.current_weather.wind_speed,
                actual.current_weather.wind_dir,
            ]
            .map(f64::to_bits)
        {
            return Err(unavailable(
                "Source outdoor current context belongs to another weather slot",
            ));
        }
        let parameters = environment.outdoor_environment_parameters;
        let weather = SurfaceOutdoorWeather {
            dry_bulb_c: current.out_dry_bulb_temp,
            wet_bulb_c: current.out_wet_bulb_temp,
            wind_speed_m_per_s: current.wind_speed,
            wind_direction_deg: current.wind_dir,
        };
        output.backend.state.warmup_flag = matches!(clock.phase, WeatherDayPhase::Warmup { .. });
        output.backend.state.doing_sizing = doing_sizing;
        update_zones(zones, parameters, weather, &mut output.backend)?;
        apply_surfaces(
            &mut state.surfaces,
            parameters,
            weather,
            &mut output.backend,
        )
    })
}

// Source DataHeatBalance SetZoneOutBulbTempAt then CheckZoneOutBulbTempAt.
// Return the existing transient height projection for the later wind setter;
// this is not another retained Zone or weather owner. No wind field is changed.
pub(crate) fn update_zone_out_bulb_temperatures(
    zones: &mut [SelectedZoneData],
    parameters: SurfaceOutdoorEnvironmentParameters,
    dry_bulb_c: f64,
    wet_bulb_c: f64,
    backend: &mut SourceErrorBackend<'_>,
) -> Result<Vec<f64>, SurfaceOutdoorEnvironmentError> {
    if zones.iter().any(|zone| zone.linked_outdoor_air_node != 0) {
        return Err(SurfaceOutdoorEnvironmentError::Unavailable(
            "Zone outdoor centroid/linked-node association",
        ));
    }
    // Validate every centroid before writes. This transient scalar projection
    // neither replaces geometry nor retains a second Zone/weather owner.
    let heights = zones
        .iter()
        .map(|zone| {
            zone.centroid_m.map(|centroid| centroid[2]).ok_or(
                SurfaceOutdoorEnvironmentError::Unavailable("Zone outdoor centroid unavailable"),
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    for (zone, &height) in zones.iter_mut().zip(&heights) {
        zone.dry_bulb = parameters.temperature_at_height(dry_bulb_c, height);
        zone.wet_bulb = parameters.temperature_at_height(wet_bulb_c, height);
    }
    check_out_bulb_temperatures(
        "Zone",
        zones
            .iter()
            .zip(&heights)
            .map(|(zone, &height)| (zone.dry_bulb, zone.wet_bulb, height, zone.name.as_str())),
        backend,
    )?;
    Ok(heights)
}

// Source DataHeatBalance SetZoneOutBulbTempAt/Check/SetWindSpeedAt/SetWindDirAt.
// The selected raw admission excludes local-environment and EMS families;
// linked-node and actual EMS ownership are also checked before any writes.
pub(super) fn update_zones(
    zones: &mut [SelectedZoneData],
    parameters: SurfaceOutdoorEnvironmentParameters,
    weather: SurfaceOutdoorWeather,
    backend: &mut SourceErrorBackend<'_>,
) -> Result<(), SurfaceOutdoorEnvironmentError> {
    let heights = update_zone_out_bulb_temperatures(
        zones,
        parameters,
        weather.dry_bulb_c,
        weather.wet_bulb_c,
        backend,
    )?;
    let factor = weather.wind_speed_m_per_s
        * parameters.weather_file_wind_modifier
        * parameters
            .site_wind_boundary_layer_height_m
            .powf(-parameters.site_wind_exponent);
    for (zone, &height) in zones.iter_mut().zip(&heights) {
        zone.wind_speed = if parameters.site_wind_exponent == 0.0 {
            weather.wind_speed_m_per_s
        } else if height <= 0.0 {
            0.0
        } else {
            factor * height.powf(parameters.site_wind_exponent)
        };
    }
    for zone in zones {
        zone.wind_direction = weather.wind_direction_deg;
    }
    Ok(())
}
