//! Actual calcSky8873–8907, writing its two formal outputs in Source order.
use super::{Error, Failure, ordinal};
use crate::psychrometrics::{EnergyPlusPsychrometricFunction, SourcePsychrometricAccess};
use crate::weather::day::SourceEnvironmentData;

pub(super) fn calc_source_sky(
    environments: &Option<Vec<SourceEnvironmentData>>,
    current_environment: &i32,
    horizontal_infrared: &mut f64,
    sky_temperature: &mut f64,
    opaque_sky: f64,
    dry: f64,
    dew: f64,
    relative_humidity: f64,
    mut infrared: f64,
    psy: &mut SourcePsychrometricAccess,
) -> Result<(), Failure> {
    const K: f64 = 273.15;
    const SIGMA: f64 = 5.6697E-8;
    if infrared <= 0.0 {
        infrared = 9999.0;
    }
    let row = environments
        .as_ref()
        .and_then(|v| v.get(ordinal(*current_environment).ok()?))
        .ok_or(Error::Unavailable("actual calcSky current Environment8885"))?;
    if !row.use_weather_file_horizontal_infrared || infrared >= 9999.0 {
        // Actual CalcSkyEmissivity3200–3226. The default child is shared with
        // the retained wrapper; alternative models use the same PROPERTY_STATE.
        let emissivity = match row.sky_temperature_model {
            4 | 5 => {
                let saturation = psy
                    .evaluate(EnergyPlusPsychrometricFunction::Psat, &[dry])
                    .map_err(Error::Unavailable)?
                    .result;
                let partial_pressure = relative_humidity * saturation * 0.01;
                let clear = if row.sky_temperature_model == 4 {
                    0.618 + 0.056 * partial_pressure.powf(0.5)
                } else {
                    0.685 + 0.000032 * partial_pressure * (1699.0 / (dry + K)).exp()
                };
                clear
                    * (1.0 + 0.0224 * opaque_sky - 0.0035 * (opaque_sky * opaque_sky)
                        + 0.00028 * (opaque_sky * opaque_sky * opaque_sky))
            }
            6 => {
                let dew = if dry < dew { dry } else { dew };
                let hundredth = dew / 100.0;
                let clear = 0.758 + 0.521 * hundredth + 0.625 * (hundredth * hundredth);
                clear
                    * (1.0 + 0.0224 * opaque_sky - 0.0035 * (opaque_sky * opaque_sky)
                        + 0.00028 * (opaque_sky * opaque_sky * opaque_sky))
            }
            _ => super::super::sky::default_clark_allen_sky_emissivity(
                opaque_sky,
                dry,
                dew,
                relative_humidity,
            ),
        };
        let squared = (dry + K) * (dry + K);
        *horizontal_infrared = emissivity * SIGMA * (squared * squared);
        *sky_temperature = if matches!(row.sky_temperature_model, 4 | 5 | 6 | 0) {
            (dry + K) * emissivity.sqrt().sqrt() - K
        } else {
            0.0
        };
    } else {
        *horizontal_infrared = infrared;
        *sky_temperature = if matches!(row.sky_temperature_model, 4 | 5 | 6 | 0) {
            (infrared / SIGMA).sqrt().sqrt() - K
        } else {
            0.0
        };
    }
    Ok(())
}
