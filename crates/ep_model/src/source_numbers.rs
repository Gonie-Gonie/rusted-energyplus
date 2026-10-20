//! Pure source numeric conversion shared by raw weather readers and compiler projections.
//!
//! One prefix scanner retains the Native consumed position. ProcessNumber then
//! applies its own suffix/error writes; mandatory list reads keep their narrower
//! finite, non-underflow domain and their original failure strings.

/// An input failed the admitted finite source `ProcessNumber` conversion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceNumberError;

/// Actual `ProcessNumber` result, before a caller's integer conversion.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SourceNumberOutcome {
    /// Successful finite value, including a genuine signed underflow zero.
    Finite(f64),
    /// The Native routine writes positive zero and sets its error flag.
    ErrorZero,
    /// A suffix branch retains a non-finite value without setting the error flag.
    /// A later real-to-int conversion is undefined and must remain unavailable.
    NonFinite,
}

enum NumberPrefix {
    Decimal {
        value: f64,
        consumed: usize,
        nonzero_digit: bool,
    },
    NonFinite {
        consumed: usize,
    },
}

/// Prefix grammar of the pinned fast_float, including its special prefixes.
fn source_number_prefix(input: &str) -> Result<Option<NumberPrefix>, &'static str> {
    let bytes = input.as_bytes();
    let mut end = usize::from(bytes.first() == Some(&b'-'));
    let before = end;
    let mut nonzero_digit = false;
    while bytes.get(end).is_some_and(u8::is_ascii_digit) {
        nonzero_digit |= bytes[end] != b'0';
        end += 1;
    }
    let mut digits = end - before;
    if bytes.get(end) == Some(&b'.') {
        end += 1;
        let after = end;
        while bytes.get(end).is_some_and(u8::is_ascii_digit) {
            nonzero_digit |= bytes[end] != b'0';
            end += 1;
        }
        digits += end - after;
    }
    if digits == 0 {
        let token = &input[before..];
        if token
            .get(..3)
            .is_some_and(|prefix| prefix.eq_ignore_ascii_case("nan"))
        {
            end = before + 3;
            if bytes.get(end) == Some(&b'(') {
                // Invalid or unclosed payload leaves the consumed position at
                // the end of nan, exactly parse_infnan2351-2360.
                let mut cursor = end + 1;
                while let Some(&byte) = bytes.get(cursor) {
                    if byte == b')' {
                        end = cursor + 1;
                        break;
                    }
                    if !byte.is_ascii_alphanumeric() && byte != b'_' {
                        break;
                    }
                    cursor += 1;
                }
            }
            return Ok(Some(NumberPrefix::NonFinite { consumed: end }));
        }
        if token
            .get(..3)
            .is_some_and(|prefix| prefix.eq_ignore_ascii_case("inf"))
        {
            end = before
                + if token
                    .get(..8)
                    .is_some_and(|prefix| prefix.eq_ignore_ascii_case("infinity"))
                {
                    8
                } else {
                    3
                };
            return Ok(Some(NumberPrefix::NonFinite { consumed: end }));
        }
        return Ok(None);
    }
    if matches!(bytes.get(end), Some(b'e' | b'E')) {
        let exponent_start = end;
        end += 1;
        if matches!(bytes.get(end), Some(b'+' | b'-')) {
            end += 1;
        }
        let exponent_digits = end;
        while bytes.get(end).is_some_and(u8::is_ascii_digit) {
            end += 1;
        }
        if end == exponent_digits {
            end = exponent_start;
        }
    }
    let value = input[..end]
        .parse::<f64>()
        .map_err(|_| "decimal conversion outside the bounded grammar")?;
    Ok(Some(NumberPrefix::Decimal {
        value,
        consumed: end,
        nonzero_digit,
    }))
}

/// Mandatory list-read prefix. Preserve the existing finite, non-underflow
/// admission, consumed position and failure strings; ProcessNumber is separate.
pub fn source_finite_number_prefix(input: &str) -> Result<Option<(f64, usize)>, &'static str> {
    match source_number_prefix(input)? {
        None => Ok(None),
        Some(NumberPrefix::NonFinite { .. }) => Err("non-finite mandatory number"),
        Some(NumberPrefix::Decimal {
            value,
            consumed,
            nonzero_digit,
        }) => {
            if !value.is_finite() || (value == 0.0 && nonzero_digit) {
                return Err("decimal magnitude outside the finite list-read domain");
            }
            Ok(Some((value, consumed)))
        }
    }
}

/// Exact Utility `ProcessNumber` prefix/suffix/error classification. This is
/// distinct from list reads: full non-finite input is an error-zero, while a
/// successful E/D suffix can retain non-finite output before an unsafe cast.
#[must_use]
pub fn process_source_number_outcome(input: &str) -> SourceNumberOutcome {
    use SourceNumberOutcome::{ErrorZero, Finite, NonFinite};
    let input = input.trim_matches(' ');
    if input.is_empty() {
        return Finite(0.0);
    }
    let (value, mut end) = match source_number_prefix(input) {
        Ok(Some(NumberPrefix::Decimal {
            value, consumed, ..
        })) => (value.is_finite().then_some(value), consumed),
        Ok(Some(NumberPrefix::NonFinite { consumed })) => (None, consumed),
        Ok(None) | Err(_) => return ErrorZero,
    };
    if end == input.len() {
        return value.map_or(ErrorZero, Finite);
    }
    let bytes = input.as_bytes();
    if matches!(bytes.get(end), Some(b'+' | b'-')) {
        end += 1;
        if end == bytes.len() {
            return ErrorZero;
        }
    }
    if matches!(bytes.get(end), Some(b'd' | b'D')) {
        return process_source_number_outcome(&input.replace(['d', 'D'], "e"));
    }
    if matches!(bytes.get(end), Some(b'e' | b'E')) {
        end += 1;
        return if bytes[end..].iter().all(u8::is_ascii_digit) {
            value.map_or(NonFinite, Finite)
        } else {
            ErrorZero
        };
    }
    ErrorZero
}

/// Finite-value adapter for scalar/Until callers. Native error-zero remains an
/// error here; only actual successful finite values (including +/-0) are values.
pub fn process_source_number(input: &str) -> Result<f64, SourceNumberError> {
    match process_source_number_outcome(input) {
        SourceNumberOutcome::Finite(value) => Ok(value),
        SourceNumberOutcome::ErrorZero | SourceNumberOutcome::NonFinite => Err(SourceNumberError),
    }
}
