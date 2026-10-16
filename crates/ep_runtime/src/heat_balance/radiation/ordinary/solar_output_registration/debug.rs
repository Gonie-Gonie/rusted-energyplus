//! SkyDifSolarShading10547-10589 real Source list and debug dictionary calls.
use super::*;
pub(crate) fn register_source_sky_diffuse_debug_outputs<
    H: SourceOutputFileHost,
    D: SourceOutputDictionaryHost,
>(
    owners: &SourceSolarOutputOwners<'_>,
    detailed_shading: bool,
    context: &mut SourceSolarOutputRegistrationContext<'_, '_, H, D>,
) -> Result<(), Error> {
    for &number in &owners.geometry.surface_lists.all_exterior_solar {
        let key = &owners
            .geometry
            .final_surfaces
            .as_deref()
            .and_then(|rows| rows.get(index(number).ok()?))
            .ok_or(Error::Unavailable(
                "same actual AllExtSolarSurfaceList Name",
            ))?
            .row
            .name;
        if detailed_shading {
            context.declare(
                owners,
                number,
                key,
                Declaration {
                    name: "Debug Surface Solar Shading Model DifShdgRatioIsoSky",
                    units: Units::Dimensionless,
                    store: Store::Average,
                    target: Target::Real(Field::SurfCurDifShdgRatioIsoSky),
                },
            )?;
        } else {
            context.declare(
                owners,
                number,
                key,
                Declaration {
                    name: "Debug Surface Solar Shading Model DifShdgRatioIsoSky",
                    units: Units::Dimensionless,
                    store: Store::Average,
                    target: Target::Real(Field::SurfDifShdgRatioIsoSky),
                },
            )?;
        }
        context.declare(
            owners,
            number,
            key,
            Declaration {
                name: "Debug Surface Solar Shading Model DifShdgRatioHoriz",
                units: Units::Dimensionless,
                store: Store::Average,
                target: Target::Real(Field::SurfDifShdgRatioHoriz),
            },
        )?;
        context.declare(
            owners,
            number,
            key,
            Declaration {
                name: "Debug Surface Solar Shading Model WithShdgIsoSky",
                units: Units::Dimensionless,
                store: Store::Average,
                target: Target::Real(Field::SurfWithShdgIsoSky),
            },
        )?;
        context.declare(
            owners,
            number,
            key,
            Declaration {
                name: "Debug Surface Solar Shading Model WoShdgIsoSky",
                units: Units::Dimensionless,
                store: Store::Average,
                target: Target::Real(Field::SurfWoShdgIsoSky),
            },
        )?;
    }
    Ok(())
}
