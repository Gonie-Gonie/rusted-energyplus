//! Genuine average-temperature children9995/10094 on actual constructor flags.
use super::{Error, SelectedHeatBalanceGlobals, SurfaceGeometryInputState};
pub(super) fn ground_temperature_average(
    geometry: &mut SurfaceGeometryInputState,
    globals: &SelectedHeatBalanceGlobals,
) -> Result<(), Error> {
    if !globals.any_local_environments_in_model {
        return Ok(());
    }
    for number in 1..=geometry.total_surfaces {
        let row = geometry
            .final_surfaces
            .as_ref()
            .and_then(|rows| super::index(number).ok().and_then(|i| rows.get(i)))
            .ok_or(Error::Unavailable(
                "GetGroundSurfacesTemperatureAverage actual Surface",
            ))?;
        if !row.is_ground_surface_property_defined {
            continue;
        }
        return Err(Error::Unavailable(
            "GetGroundSurfacesTemperatureAverage actual GroundSurfsProperty10009",
        ));
    }
    Ok(())
}
pub(super) fn surrounding_temperature_average(
    geometry: &mut SurfaceGeometryInputState,
    globals: &SelectedHeatBalanceGlobals,
) -> Result<(), Error> {
    if !globals.any_local_environments_in_model {
        return Ok(());
    }
    for row in geometry.final_surfaces.as_ref().ok_or(Error::Unavailable(
        "GetSurroundingSurfacesTemperatureAverage actual Surface array",
    ))? {
        if !row.has_surrounding_surface_property {
            continue;
        }
        return Err(Error::Unavailable(
            "GetSurroundingSurfacesTemperatureAverage actual SurroundingSurfsProperty10110",
        ));
    }
    Ok(())
}
