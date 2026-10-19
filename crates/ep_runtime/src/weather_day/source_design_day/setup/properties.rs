//! Source compositions on the SAME accepted property cache owner.
//! SetUp's real WarmupFlag=true makes the pinned warning helpers have no effects.
use super::{Error, SourcePsychrometricAccess};
use crate::psychrometrics::EnergyPlusPsychrometricFunction as Function;
fn evaluate(
    psy: &mut SourcePsychrometricAccess,
    function: Function,
    inputs: &[f64],
) -> Result<f64, Error> {
    Ok(psy
        .evaluate(function, inputs)
        .map_err(Error::Unavailable)?
        .result)
}
pub(super) fn source_min(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}
pub(super) fn source_max(a: f64, b: f64) -> f64 {
    if a < b { b } else { a }
}
pub(super) fn from_relative_humidity(
    psy: &mut SourcePsychrometricAccess,
    temperature: f64,
    relative: f64,
    pressure: f64,
) -> Result<f64, Error> {
    evaluate(psy, Function::WFromRh, &[temperature, relative, pressure])
}
pub(super) fn from_wet_bulb(
    psy: &mut SourcePsychrometricAccess,
    temperature: f64,
    wet_bulb: f64,
    pressure: f64,
) -> Result<f64, Error> {
    evaluate(psy, Function::WFromTwb, &[temperature, wet_bulb, pressure])
}
pub(super) fn from_enthalpy(
    psy: &mut SourcePsychrometricAccess,
    temperature: f64,
    enthalpy: f64,
) -> Result<f64, Error> {
    evaluate(psy, Function::WFromH, &[temperature, enthalpy])
}
pub(super) fn wet_bulb(
    psy: &mut SourcePsychrometricAccess,
    temperature: f64,
    humidity: f64,
    pressure: f64,
) -> Result<f64, Error> {
    evaluate(psy, Function::Twb, &[temperature, humidity, pressure])
}
pub(super) fn from_dew_point(
    psy: &mut SourcePsychrometricAccess,
    temperature: f64,
    pressure: f64,
) -> Result<f64, Error> {
    let dew_pressure = evaluate(psy, Function::Psat, &[temperature])?;
    let humidity = dew_pressure * 0.62198 / (pressure - dew_pressure);
    if humidity < 0.0 {
        let mut delta_t = 0.0;
        let mut adjusted_pressure = dew_pressure;
        if pressure <= 0.0 {
            return Err(Error::Unavailable(
                "actual PsyWFnTdpPb negative/zero-pressure nonterminating supplied domain",
            ));
        }
        while adjusted_pressure >= pressure {
            delta_t += 1.0;
            adjusted_pressure = evaluate(psy, Function::Psat, &[temperature - delta_t])?;
        }
        let adjusted_humidity = adjusted_pressure * 0.62198 / (pressure - adjusted_pressure);
        // Native error(W<=-.0001) is called here; its first !WarmupFlag test
        // is false from the genuine enclosing SetUp write, so no further effect.
        return Ok(adjusted_humidity);
    }
    Ok(humidity)
}
pub(super) fn dew_point(
    psy: &mut SourcePsychrometricAccess,
    humidity: f64,
    pressure: f64,
) -> Result<f64, Error> {
    let limited = source_max(humidity, 1.0e-5);
    let dew_pressure = pressure * limited / (0.62198 + limited);
    evaluate(psy, Function::TsatPb, &[dew_pressure])
}
// Preserve Original ordered bounds; range negation can change floating-point NaN behavior.
#[allow(clippy::manual_range_contains)]
pub(super) fn relative_humidity(
    psy: &mut SourcePsychrometricAccess,
    temperature: f64,
    humidity: f64,
    pressure: f64,
) -> Result<f64, Error> {
    let saturation = evaluate(psy, Function::Psat, &[temperature])?;
    let limited = source_max(humidity, 1.0e-5);
    let degree = limited / (0.62198 * saturation / (pressure - saturation));
    let relative = degree / (1.0 - (1.0 - degree) * (saturation / pressure));
    if relative < 0.0 || relative > 1.0 {
        // Same actual WarmupFlag=true guards every reached error-helper effect.
        return Ok(source_min(source_max(relative, 0.01), 1.0));
    }
    Ok(relative)
}
