//! Native ASTM C1340 wrapper6437 and complete correlation6458-6594.
//! Current Surface/Space temperatures and the sole physical wind/Node owners
//! are read at each ordinary InitIntConvCoeff352 invocation.
use super::*;
use crate::heat_balance::selected_heat_balance_input::surface::FinalSurface;

// Error transport only; the selected platform operation is evaluated at its
// original expression site, without a numeric fallback or provider cache.
fn math(
    operation: &'static str,
    result: Result<f64, ep_platform_math::MathProviderError>,
) -> Result<f64, Error> {
    result.map_err(|error| {
        Error::SourceInput(Box::new(crate::RuntimeError::WarmupOutput(Box::new(
            crate::WarmupOutputError::Unavailable(format!(
                "actual Source ASTM C1340 {operation}: {error}"
            )),
        ))))
    })
}

/// InitIntConvCoeff352 -> wrapper6437. Raw system ACH is evaluated even when
/// the coefficient later chooses the exterior Surface wind instead of Vair.
#[allow(clippy::too_many_arguments)]
pub(super) fn initialize_surface<H: InsideConvectionCoefficients + ?Sized>(
    i: usize,
    surface_temperature: f64,
    space_mat: f64,
    geometry: &SurfaceGeometryInputState,
    zones: &SelectedZoneInputOwners,
    equipment: &OrdinaryEquipment,
    globals: &SelectedHeatBalanceGlobals,
    clock: &WeatherGlobalState,
    properties: &mut SourceInsideConvectionPropertyOwners<'_>,
    low_limit: f64,
    coefficients: &mut H,
    backend: &mut SourceErrorBackend<'_>,
) -> Result<(), Error> {
    let surface = geometry
        .final_surfaces
        .as_ref()
        .and_then(|rows| rows.get(i))
        .ok_or(Error::Unavailable("actual ASTM C1340 Surface ordinal"))?;
    let zone_at = index(surface.row.zone)?;
    let zone = zones
        .zones
        .as_ref()
        .and_then(|rows| rows.get(zone_at))
        .ok_or(Error::Unavailable("actual ASTM C1340 owning Zone"))?;
    let volume = zone.volume;
    let vair = math("pow", ep_platform_math::pow(volume, 1.0 / 3.0))?
        * super::ceiling::system_ach(zone, equipment, globals, clock, properties, backend)?
        / 3600.0;
    let tilt = surface
        .row
        .vertex_progress
        .properties
        .ok_or(Error::Unavailable("actual ASTM C1340 produced Tilt"))?
        .tilt_deg;
    coefficients.set_coefficient(
        i,
        coefficient(
            surface,
            surface_temperature,
            space_mat,
            vair,
            tilt,
            properties,
        )?,
    );
    // Native6453 checks the same live cell once. Its caller has no second clamp.
    if *coefficients
        .borrow_coefficient(i)
        .map_err(|e| translate(e.into()))?
        < low_limit
    {
        coefficients.set_coefficient(i, low_limit);
    }
    Ok(())
}

/// Literal Native6458-6594. No minflow, ACH ceiling, forced-flow bypass,
/// recomputed geometry, epsilon, high limit, or post-hoc finite repair occurs.
fn coefficient(
    surface: &FinalSurface,
    tsurf: f64,
    tair: f64,
    vair: f64,
    tilt: f64,
    properties: &SourceInsideConvectionPropertyOwners<'_>,
) -> Result<f64, Error> {
    const G: f64 = 9.807; // Constant::Gravity, DataGlobalConstants.hh589.
    const DEG_TO_RAD: f64 = std::f64::consts::PI / 180.0; // Constant::DegToRad.
    const ONE_THIRD: f64 = 1.0 / 3.0;
    // Native6539 intentionally uses this approximate literal in only one arm.
    #[allow(clippy::approx_constant)]
    const NATIVE_APPROX_PI: f64 = 3.14159;

    let length = if tilt == 0.0 || tilt == 180.0 {
        math("sqrt", ep_platform_math::sqrt(surface.row.area))?
    } else {
        surface.row.vertex_progress.height
    };
    let velocity = if surface.row.exterior_boundary == 0 {
        let id = surface
            .typed_id
            .ok_or(Error::Unavailable("actual ASTM C1340 Surface identity"))?;
        properties
            .physical
            .iter()
            .find(|row| row.surface_id == id)
            .ok_or(Error::Unavailable(
                "actual ASTM C1340 physical SurfOutWindSpeed",
            ))?
            .outdoor_environment
            .row
            .wind_speed_m_per_s
    } else {
        vair
    };
    let pr =
        0.7880 - (2.631 * math("pow", ep_platform_math::pow(10.0_f64, -4.0))? * (tair + 273.15));
    let beta_si = 1.0 / (tair + 273.15);
    let rho_si = (22.0493 / (tair + 273.15)) * 16.0;
    let cp_si = 0.068559
        * (3.4763
            + (1.066 * math("pow", ep_platform_math::pow(10.0_f64, -4.0))? * (tair + 273.15)))
        * 4186.8;
    let dv = (241.9 * math("pow", ep_platform_math::pow(10.0_f64, -7.0))?)
        * (145.8 * (tair + 273.15) * math("pow", ep_platform_math::pow(tair + 273.15, 0.5))?)
        / ((tair + 273.15) + 110.4);
    let visc = dv * (0.45359237 / (0.3048 * 3600.0)) / rho_si;
    let k_si_n = 0.6325
        * math("pow", ep_platform_math::pow(10.0_f64, -5.0))?
        * math("pow", ep_platform_math::pow(tair + 273.15, 0.5))?
        * 241.77;
    // The exponent is -12/(Tair+273.15), not a constant -12.
    let k_si_d = 1.0
        + (245.4
            * math(
                "pow",
                ep_platform_math::pow(10.0_f64, -12.0 / (tair + 273.15)),
            )?)
            / (tair + 273.15);
    let k_si = 1.730735 * (k_si_n / k_si_d);
    let delta_temp = tsurf - tair;
    let ra = (G * beta_si * rho_si * cp_si * delta_temp * (length * length * length)).abs()
        / (visc * k_si);
    let re = (velocity * length) / visc;

    let nun = if tilt == 0.0 {
        if delta_temp > 0.0 {
            0.58 * math("pow", ep_platform_math::pow(ra, 0.2))?
        } else if ra < 8000000.0 {
            0.54 * math("pow", ep_platform_math::pow(ra, 0.25))?
        } else {
            0.15 * math("pow", ep_platform_math::pow(ra, 1.0 / 3.0))?
        }
    } else if tilt > 0.0 && tilt < 90.0 {
        if delta_temp > 0.0 {
            if tilt < 2.0 {
                0.58 * math("pow", ep_platform_math::pow(ra, 0.2))?
            } else {
                0.56 * math(
                    "pow",
                    ep_platform_math::pow(
                        ra * math("sin", ep_platform_math::sin(tilt * DEG_TO_RAD))?,
                        0.25,
                    ),
                )?
            }
        } else {
            let grc = if tilt < 15.0 {
                1000000.0
            } else if tilt <= 75.0 {
                math(
                    "pow",
                    ep_platform_math::pow(10.0_f64, tilt / (1.1870 + (0.0870 * tilt))),
                )?
            } else {
                5000000000.0
            };
            if (ra / pr) <= grc {
                0.56 * math(
                    "pow",
                    ep_platform_math::pow(
                        ra * math(
                            "sin",
                            ep_platform_math::sin(tilt * NATIVE_APPROX_PI / 180.0),
                        )?,
                        0.25,
                    ),
                )?
            } else {
                0.14 * (math("pow", ep_platform_math::pow(ra, ONE_THIRD))?
                    - math("pow", ep_platform_math::pow(grc * pr, ONE_THIRD))?)
                    + 0.56
                        * math(
                            "pow",
                            ep_platform_math::pow(
                                grc * pr * math("sin", ep_platform_math::sin(tilt * DEG_TO_RAD))?,
                                0.25,
                            ),
                        )?
            }
        }
    } else if tilt == 180.0 {
        if delta_temp <= 0.0 {
            0.58 * math("pow", ep_platform_math::pow(ra, 0.2))?
        } else if ra < 8000000.0 {
            0.54 * math("pow", ep_platform_math::pow(ra, 0.25))?
        } else {
            0.15 * math("pow", ep_platform_math::pow(ra, 1.0 / 3.0))?
        }
    } else if tilt > 90.0 && tilt < 180.0 {
        if delta_temp <= 0.0 {
            if tilt > 178.0 {
                0.58 * math("pow", ep_platform_math::pow(ra, 0.2))?
            } else {
                0.56 * math(
                    "pow",
                    ep_platform_math::pow(
                        ra * math("sin", ep_platform_math::sin(tilt * DEG_TO_RAD))?,
                        0.25,
                    ),
                )?
            }
        } else {
            let grc = if tilt > 165.0 {
                1000000.0
            } else if tilt <= 105.0 {
                math(
                    "pow",
                    ep_platform_math::pow(10.0_f64, tilt / (1.1870 + (0.0870 * tilt))),
                )?
            } else {
                5000000000.0
            };
            if (ra / pr) <= grc {
                0.56 * math(
                    "pow",
                    ep_platform_math::pow(
                        ra * math("sin", ep_platform_math::sin(tilt * DEG_TO_RAD))?,
                        0.25,
                    ),
                )?
            } else {
                0.14 * (math("pow", ep_platform_math::pow(ra, ONE_THIRD))?
                    - math("pow", ep_platform_math::pow(grc * pr, ONE_THIRD))?)
                    + 0.56
                        * math(
                            "pow",
                            ep_platform_math::pow(
                                grc * pr * math("sin", ep_platform_math::sin(tilt * DEG_TO_RAD))?,
                                0.25,
                            ),
                        )?
            }
        }
    } else if ra < 1000000000.0 {
        0.59 * math("pow", ep_platform_math::pow(ra, 0.25))?
    } else {
        0.10 * math("pow", ep_platform_math::pow(ra, 1.0 / 3.0))?
    };

    let nuf = if re < 500000.0 {
        0.664
            * math("pow", ep_platform_math::pow(pr, 1.0 / 3.0))?
            * math("pow", ep_platform_math::pow(re, 0.5))?
    } else {
        math("pow", ep_platform_math::pow(pr, 1.0 / 3.0))?
            * ((0.037 * math("pow", ep_platform_math::pow(re, 0.8))?) - 850.0)
    };
    let hf = nuf * k_si / length;
    let hn = nun * k_si / length;
    math(
        "pow",
        ep_platform_math::pow(
            math("pow", ep_platform_math::pow(hf, 3.0))?
                + math("pow", ep_platform_math::pow(hn, 3.0))?,
            1.0 / 3.0,
        ),
    )
}
