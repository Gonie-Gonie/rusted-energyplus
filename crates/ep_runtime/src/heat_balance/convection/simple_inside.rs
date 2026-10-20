//! Exact non-Kiva CalcASHRAESimpleIntConvCoeff scalar, Original1829-1879.
//! Caller ownership, the raw SurfHConvInt write and both lower-limit reads stay
//! in InitIntConvCoeff. The stored outward cosine is passed without negation.

/// Evaluate the actual Simple correlation with its strict Source thresholds.
#[must_use]
pub fn coefficient_w_per_m2_k(
    surface_temperature_c: f64,
    air_temperature_c: f64,
    outward_cos_tilt: f64,
) -> f64 {
    if outward_cos_tilt.abs() < 0.3827 {
        return 3.076;
    }
    let delta_temperature_cos_tilt = (air_temperature_c - surface_temperature_c) * outward_cos_tilt;
    if outward_cos_tilt.abs() >= 0.9239 {
        if delta_temperature_cos_tilt > 0.0 {
            return 4.040;
        }
        if delta_temperature_cos_tilt < 0.0 {
            return 0.948;
        }
        return 3.076;
    }
    if delta_temperature_cos_tilt > 0.0 {
        return 3.870;
    }
    if delta_temperature_cos_tilt < 0.0 {
        return 2.281;
    }
    3.076
}

#[cfg(test)]
mod tests {
    use super::coefficient_w_per_m2_k as coefficient;

    #[test]
    fn simple_exact_threshold_neighbors_and_both_outward_cosine_signs() {
        let vertical = 0.3827_f64;
        let horizontal = 0.9239_f64;
        for (magnitude, enhanced, reduced) in [
            (f64::from_bits(vertical.to_bits() - 1), 3.076, 3.076),
            (vertical, 3.870, 2.281),
            (f64::from_bits(vertical.to_bits() + 1), 3.870, 2.281),
            (f64::from_bits(horizontal.to_bits() - 1), 3.870, 2.281),
            (horizontal, 4.040, 0.948),
            (f64::from_bits(horizontal.to_bits() + 1), 4.040, 0.948),
            (1.0, 4.040, 0.948),
        ] {
            for sign in [-1.0, 1.0] {
                let cosine = sign * magnitude;
                assert_eq!(coefficient(20.0, 20.0 + sign, cosine), enhanced);
                assert_eq!(coefficient(20.0, 20.0 - sign, cosine), reduced);
                assert_eq!(coefficient(20.0, 20.0, cosine), 3.076);
            }
        }
        for cosine in [0.0, -0.0] {
            assert_eq!(coefficient(20.0, 19.0, cosine), 3.076);
            assert_eq!(coefficient(20.0, 21.0, cosine), 3.076);
        }
    }
}
