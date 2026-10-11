//! Selected no-width IOFiles.hh R formatting; prospective Native text proof required.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SourceRoundFormatUnavailable {
    ExponentFormat,
    UndefinedLog10Conversion,
}

// Literal nextafter direction from IOFiles.hh249-259, retaining both zeros.
pub(super) fn source_next_float(value: f64) -> f64 {
    if value.is_nan() || value == 0.0 {
        return value;
    }
    if value.is_sign_negative() {
        if value == f64::MIN {
            value
        } else if value == f64::NEG_INFINITY {
            f64::MIN
        } else {
            f64::from_bits(value.to_bits() + 1)
        }
    } else if value == f64::MAX {
        value
    } else if value == f64::INFINITY {
        f64::MAX
    } else {
        f64::from_bits(value.to_bits() + 1)
    }
}

fn fixed(value: f64, precision: usize) -> String {
    format!("{value:.precision$}")
}

fn exponential(value: f64, precision: usize) -> Result<String, SourceRoundFormatUnavailable> {
    if value.is_nan() {
        // fmt8 write_nonfinite reads signbit even for NaN. Preserve the later
        // literal zero_pad_exponent operation on this text too.
        return Ok(if value.is_sign_negative() {
            "-NAN"
        } else {
            "NAN"
        }
        .into());
    }
    let text = format!("{value:.precision$E}");
    let (mantissa, exponent) = text
        .split_once('E')
        .ok_or(SourceRoundFormatUnavailable::ExponentFormat)?;
    let exponent: i32 = exponent
        .parse()
        .map_err(|_| SourceRoundFormatUnavailable::ExponentFormat)?;
    // Reproduce fmt8's signed exponent with a minimum of two digits first.
    Ok(format!("{mantissa}E{exponent:+03}"))
}

fn zero_pad_exponent(text: &mut String) {
    // IOFiles.hh130-139 tests the actual string position, not an E suffix.
    if text.len() > 3 && !text.as_bytes()[text.len() - 3].is_ascii_digit() {
        text.insert(text.len() - 2, '0');
    }
}

/// Exact selected R branch structure for the no-width .0/.2/.3/.4/.10R callers.
/// Values are formatted after the original nudges, with no decimal rescaling.
pub(crate) fn round(value: f64, precision: usize) -> Result<String, SourceRoundFormatUnavailable> {
    let threshold = f64::from_bits(0x3fb9_9999_9999_9999);
    let fixed_output = value >= threshold || value <= -threshold || value == 0.0 || value == -0.0;
    if fixed_output {
        // Preserve the source's positive-only branch and original log10 order.
        if value > 100000.0 {
            if !value.is_finite() {
                return Err(SourceRoundFormatUnavailable::UndefinedLog10Conversion);
            }
            let digits10 = value.log10() as i32;
            let precision_i32 = i32::try_from(precision)
                .map_err(|_| SourceRoundFormatUnavailable::ExponentFormat)?;
            let total = digits10
                .checked_add(precision_i32)
                .ok_or(SourceRoundFormatUnavailable::ExponentFormat)?;
            if total >= 17 {
                return Ok(format!("{}.", fixed(value, 0)));
            }
            return Ok(fixed(value, precision));
        }
        if value == 0.0 || value == -0.0 {
            return Ok(fixed(0.0, precision));
        }
        let nudged = source_next_float(source_next_float(source_next_float(value)));
        return Ok(fixed(nudged, precision));
    }
    let mut text = exponential(source_next_float(value), precision)?;
    zero_pad_exponent(&mut text);
    Ok(text)
}

#[cfg(test)]
#[path = "source_round_format_tests.rs"]
mod tests;
