//! Actual DataSurfaces594-667 children on the sole physical outdoor cells.
//! Geometry stays live and immutable; no resync, snapshots or copied-back rows.
use super::{SurfaceOutdoorEnvironmentRow, SurfaceOutdoorSetterRow, SurfaceOutdoorWeather};
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::selected_heat_balance_input::{
    HeatBalanceInputError as Error, surface::SurfaceGeometryInputState,
};
use crate::weather::day::WeatherEnvironmentState;
use std::collections::BTreeMap;

/// Sole four DataSurfaces outdoor owners, independent of legacy CTF/temperature state.
/// Caller supplies the actual GetSurfaceData allocation and surface identity.
/// This carrier has no constructor, thermal seed, coefficient cache or copy adapter.
#[derive(Debug, PartialEq)]
pub(crate) struct SourceSurfaceOutdoorState {
    pub(crate) surface_id: ep_model::SurfaceId,
    pub(crate) outdoor_environment: super::SurfaceOutdoorEnvironmentState,
}

/// Temporary references only. No second geometry/weather/outdoor state exists.
struct SourceSurfaceOutdoorRow<'a> {
    centroid_height_m: &'a f64,
    wind_exposed: &'a bool,
    name: &'a str,
    fields: &'a mut SurfaceOutdoorEnvironmentRow,
}
impl SurfaceOutdoorSetterRow for SourceSurfaceOutdoorRow<'_> {
    fn centroid_height_m(&self) -> f64 {
        *self.centroid_height_m
    }
    fn wind_exposed(&self) -> bool {
        *self.wind_exposed
    }
    fn fields(&mut self) -> &mut SurfaceOutdoorEnvironmentRow {
        self.fields
    }
}

/// Source ordinal traversal with actual typed binding to the physical carrier.
/// Validate the complete admitted bijection before any mathematical write.
fn source_rows<'a>(
    geometry: &'a SurfaceGeometryInputState,
    physical_surfaces: &'a mut [SourceSurfaceOutdoorState],
) -> Result<Vec<SourceSurfaceOutdoorRow<'a>>, Error> {
    let surfaces = geometry.final_surfaces.as_ref().ok_or(Error::Unavailable(
        "actual final Surface allocation for outdoor setters",
    ))?;
    let total = usize::try_from(geometry.total_surfaces)
        .map_err(|_| Error::Unavailable("actual Source TotSurfaces"))?;
    if surfaces.len() != total || physical_surfaces.len() != total {
        return Err(Error::Unavailable(
            "actual Source outdoor total/physical allocation association",
        ));
    }
    let mut physical = BTreeMap::new();
    for surface in physical_surfaces {
        if physical
            .insert(surface.surface_id, &mut surface.outdoor_environment.row)
            .is_some()
        {
            return Err(Error::Unavailable(
                "duplicate physical Surface outdoor owner",
            ));
        }
    }
    let mut ordered = Vec::with_capacity(total);
    for surface in surfaces {
        let id = surface.typed_id.ok_or(Error::Unavailable(
            "actual final Surface typed binding for outdoor setters",
        ))?;
        let fields = physical.remove(&id).ok_or(Error::Unavailable(
            "actual Source/physical Surface outdoor bijection",
        ))?;
        ordered.push(SourceSurfaceOutdoorRow {
            centroid_height_m: &surface.centroid[2],
            wind_exposed: &surface.row.exterior_wind,
            name: &surface.row.name,
            fields,
        });
    }
    if !physical.is_empty() {
        return Err(Error::Unavailable(
            "unpaired physical Surface outdoor owner",
        ));
    }
    Ok(ordered)
}

fn require_station_site(environment: &WeatherEnvironmentState) -> Result<(), Error> {
    if !environment.source_weather_station_initialized
        || !environment.source_site_atmosphere_initialized
    {
        return Err(Error::Unavailable(
            "outdoor profile before actual Station/Site producer returns",
        ));
    }
    Ok(())
}

/// InitSurface311, all TotSurfaces; zero gradient does not read centroid.
pub(crate) fn set_source_surface_out_bulb_temperatures(
    geometry: &SurfaceGeometryInputState,
    physical_surfaces: &mut [SourceSurfaceOutdoorState],
    environment: &WeatherEnvironmentState,
) -> Result<(), Error> {
    require_station_site(environment)?;
    let weather = SurfaceOutdoorWeather {
        dry_bulb_c: environment.current_weather.out_dry_bulb_temp,
        wet_bulb_c: environment.current_weather.out_wet_bulb_temp,
        wind_speed_m_per_s: environment.current_weather.wind_speed,
        wind_direction_deg: environment.current_weather.wind_dir,
    };
    super::set_surface_out_bulb_temperature_rows(
        environment.outdoor_environment_parameters,
        weather,
        source_rows(geometry, physical_surfaces)?.into_iter(),
    );
    Ok(())
}

/// InitSurface312, cumulative min0 and actual reached final Surface name/height.
/// Backend failure and fatal effects retain their exact original owned causes.
pub(crate) fn check_source_surface_out_bulb_temperatures(
    geometry: &SurfaceGeometryInputState,
    physical_surfaces: &mut [SourceSurfaceOutdoorState],
    backend: &mut SourceErrorBackend<'_>,
) -> Result<(), Error> {
    super::ordinary::check_out_bulb_temperatures(
        "Surface",
        source_rows(geometry, physical_surfaces)?
            .into_iter()
            .map(|row| {
                (
                    row.fields.dry_bulb_c,
                    row.fields.wet_bulb_c,
                    *row.centroid_height_m,
                    row.name,
                )
            }),
        backend,
    )
    .map_err(|error| match error {
        super::SurfaceOutdoorEnvironmentError::Unavailable(reason) => Error::Unavailable(reason),
        super::SurfaceOutdoorEnvironmentError::Format(reason) => Error::Format(reason),
        super::SurfaceOutdoorEnvironmentError::Backend(error) => Error::Backend(error),
        super::SurfaceOutdoorEnvironmentError::Fatal(error) => Error::Fatal(error),
    })
}

/// InitSurface314, actual factor order; nonzero exponent leaves NoWind untouched.
pub(crate) fn set_source_surface_wind_speeds(
    geometry: &SurfaceGeometryInputState,
    physical_surfaces: &mut [SourceSurfaceOutdoorState],
    environment: &WeatherEnvironmentState,
) -> Result<(), Error> {
    require_station_site(environment)?;
    super::set_surface_wind_speed_rows(
        environment.outdoor_environment_parameters,
        environment.current_weather.wind_speed,
        source_rows(geometry, physical_surfaces)?.into_iter(),
    );
    Ok(())
}

/// InitSurface315, every allocated row receives the actual unnormalized WindDir.
pub(crate) fn set_source_surface_wind_directions(
    geometry: &SurfaceGeometryInputState,
    physical_surfaces: &mut [SourceSurfaceOutdoorState],
    environment: &WeatherEnvironmentState,
) -> Result<(), Error> {
    super::set_surface_wind_direction_rows(
        environment.current_weather.wind_dir,
        source_rows(geometry, physical_surfaces)?.into_iter(),
    );
    Ok(())
}
