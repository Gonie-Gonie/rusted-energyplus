//! Default ClarkAllen sky emissivity and weather-file infrared processing.
//!
//! WeatherManager.cc:3200-3226,8873-8907; alternative models and selected
//! Environment/Current globals are outside this default-model API.

const KELVIN: f64 = 273.15;
const STEFAN_BOLTZMANN: f64 = 5.6697E-8;

/// The two actual `calcSky` outputs, in native formal-output order.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WeatherSky {
    /// Processed horizontal infrared sky radiation in W/m2.
    pub horiz_ir_sky: f64,
    /// Processed sky temperature in degrees C.
    pub sky_temp: f64,
}

// ObjexxFCL Fmath.hh:90-96. Equal values select the second operand, including
// opposite signed zeros; f64::min has a different operand-selection contract.
fn source_min(first: f64, second: f64) -> f64 {
    if first < second { first } else { second }
}

fn pow_2(value: f64) -> f64 {
    value * value
}

fn pow_3(value: f64) -> f64 {
    value * value * value
}

fn pow_4(value: f64) -> f64 {
    let squared = value * value;
    squared * squared
}

fn root_4(value: f64) -> f64 {
    value.sqrt().sqrt()
}

/// Returns default ClarkAllen sky emissivity, including opaque-cloud adjustment.
///
/// Cloud cover is the source tenths value; dry bulb and dew point are degrees C.
/// Relative humidity retains the native formal input (caller fraction), which
/// ClarkAllen does not use. Other sky models are not selected by this function.
#[must_use]
pub fn default_clark_allen_sky_emissivity(
    opaque_sky_cover: f64,
    dry_bulb: f64,
    dew_point: f64,
    relative_humidity: f64,
) -> f64 {
    let _ = relative_humidity;
    let clear_sky = 0.787 + 0.764 * ((source_min(dry_bulb, dew_point) + KELVIN) / KELVIN).ln();
    clear_sky
        * (1.0 + 0.0224 * opaque_sky_cover - 0.0035 * pow_2(opaque_sky_cover)
            + 0.00028 * pow_3(opaque_sky_cover))
}

/// Returns source sky/IR outputs for ClarkAllen with weather-file IR enabled.
///
/// Inputs follow native `calcSky` order after its state and output references:
/// opaque cover, dry bulb C, dew point C, relative-humidity fraction, IR W/m2.
/// IR <= 0 becomes the 9999 missing sentinel; positive IR below 9999 is retained.
#[must_use]
pub fn default_weather_file_sky(
    opaque_sky_cover: f64,
    dry_bulb: f64,
    dew_point: f64,
    relative_humidity: f64,
    infrared: f64,
) -> WeatherSky {
    let mut infrared = infrared;
    if infrared <= 0.0 {
        infrared = 9999.0;
    }
    if infrared >= 9999.0 {
        let emissivity = default_clark_allen_sky_emissivity(
            opaque_sky_cover,
            dry_bulb,
            dew_point,
            relative_humidity,
        );
        WeatherSky {
            horiz_ir_sky: emissivity * STEFAN_BOLTZMANN * pow_4(dry_bulb + KELVIN),
            sky_temp: (dry_bulb + KELVIN) * root_4(emissivity) - KELVIN,
        }
    } else {
        WeatherSky {
            horiz_ir_sky: infrared,
            sky_temp: root_4(infrared / STEFAN_BOLTZMANN) - KELVIN,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_min_selects_second_signed_zero_on_ties() {
        assert_eq!(source_min(0.0, -0.0).to_bits(), (-0.0_f64).to_bits());
        assert_eq!(source_min(-0.0, 0.0).to_bits(), 0.0_f64.to_bits());
    }

    #[test]
    fn default_clear_sky_retains_native_unused_humidity_formal() {
        assert_eq!(
            default_clark_allen_sky_emissivity(0.0, 0.0, 0.0, 0.5),
            0.787
        );
        assert_eq!(
            default_clark_allen_sky_emissivity(5.0, 20.0, 10.0, 0.5).to_bits(),
            default_clark_allen_sky_emissivity(5.0, 20.0, 10.0, 50.0).to_bits()
        );
    }

    #[test]
    fn measured_ir_is_retained_below_the_missing_threshold() {
        for infrared in [0.1, 300.0, f64::from_bits(9999.0_f64.to_bits() - 1)] {
            assert_eq!(
                default_weather_file_sky(5.0, 20.0, 10.0, 0.5, infrared)
                    .horiz_ir_sky
                    .to_bits(),
                infrared.to_bits()
            );
        }
    }

    #[test]
    fn nonpositive_ir_and_missing_sentinel_share_the_default_branch() {
        let missing = default_weather_file_sky(5.0, 20.0, 10.0, 0.5, 9999.0);
        for infrared in [-1.0, -0.0, 0.0, 9999.0, 10000.0] {
            assert_eq!(
                default_weather_file_sky(5.0, 20.0, 10.0, 0.5, infrared),
                missing
            );
        }
    }
}
