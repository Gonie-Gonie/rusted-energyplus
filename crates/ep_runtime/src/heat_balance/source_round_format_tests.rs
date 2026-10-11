use super::*;

#[test]
fn zeros_and_exact_half_values_use_the_original_nudge_route() {
    for precision in [0, 2, 3, 4, 10] {
        assert_eq!(round(-0.0, precision), round(0.0, precision));
    }
    assert_eq!(round(2.5, 0), Ok("3".into()));
    assert_eq!(round(-2.5, 0), Ok("-3".into()));
    assert_eq!(round(1.2345, 3), Ok("1.235".into()));
    assert_eq!(round(-1.2345, 3), Ok("-1.235".into()));
}

#[test]
fn fixed_threshold_and_large_positive_asymmetry_remain_literal() {
    let threshold = f64::from_bits(0x3fb9_9999_9999_9999);
    assert_eq!(round(threshold, 3), Ok("0.100".into()));
    assert_eq!(
        round(f64::from_bits(threshold.to_bits() - 1), 3),
        Ok("1.000E-001".into())
    );
    assert_eq!(round(10_000_000.0, 10), Ok("10000000.".into()));
    assert_eq!(round(-10_000_000.0, 10), Ok("-10000000.0000000056".into()));
}

#[test]
fn exponent_padding_is_the_source_string_operation_even_for_signed_nan() {
    let mut ordinary = "1.000E-04".to_owned();
    zero_pad_exponent(&mut ordinary);
    assert_eq!(ordinary, "1.000E-004");
    let mut already_three = "1.000E-324".to_owned();
    zero_pad_exponent(&mut already_three);
    assert_eq!(already_three, "1.000E-324");
    assert_eq!(
        round(f64::from_bits(0x7ff8_0000_0000_1234), 3),
        Ok("NAN".into())
    );
    assert_eq!(
        round(f64::from_bits(0xfff8_0000_0000_1234), 3),
        Ok("-N0AN".into())
    );
}

#[test]
fn nextafter_endpoints_and_undefined_positive_infinity_are_explicit() {
    assert_eq!(source_next_float(-0.0).to_bits(), (-0.0_f64).to_bits());
    assert_eq!(source_next_float(f64::MIN).to_bits(), f64::MIN.to_bits());
    assert_eq!(source_next_float(f64::MAX).to_bits(), f64::MAX.to_bits());
    assert_eq!(
        source_next_float(f64::NEG_INFINITY).to_bits(),
        f64::MIN.to_bits()
    );
    assert_eq!(
        round(f64::INFINITY, 3),
        Err(SourceRoundFormatUnavailable::UndefinedLog10Conversion)
    );
    assert_eq!(round(f64::NEG_INFINITY, 3), Ok(format!("{:.3}", f64::MIN)));
}
