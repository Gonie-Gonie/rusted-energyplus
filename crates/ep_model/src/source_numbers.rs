//! Pure source numeric conversion shared by raw weather readers and compiler projections.
//!
//! Keep the admitted prefix grammar and `ProcessNumber` suffix/error behavior in
//! one implementation. Callers retain their own reference writes and error flags.

/// An input failed the admitted source `ProcessNumber` conversion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceNumberError;

/// The admitted decimal grammar consumes a prefix like `fast_float::from_chars`.
/// Rust's decimal conversion supplies the value; bit equality is checked against
/// the executed original separately. Non-finite and underflow-to-zero prefixes
/// are excluded because their original list-read writes are outside this scope.
pub fn source_finite_number_prefix(input: &str) -> Result<Option<(f64, usize)>, &'static str> {
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
        if token.get(..3).is_some_and(|prefix| {
            prefix.eq_ignore_ascii_case("nan") || prefix.eq_ignore_ascii_case("inf")
        }) {
            return Err("non-finite mandatory number");
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
    if !value.is_finite() || (value == 0.0 && nonzero_digit) {
        return Err("decimal magnitude outside the finite list-read domain");
    }
    Ok(Some((value, end)))
}

/// `ProcessNumber` differs from list reads: ASCII-space-only input returns +0,
/// FORTRAN D normalization reparses the token, and invalid input returns an error.
/// Header callers map errors to the source result +0 and set their error flag.
pub fn process_source_number(input: &str) -> Result<f64, SourceNumberError> {
    let input = input.trim_matches(' ');
    if input.is_empty() {
        return Ok(0.0);
    }
    let Some((value, mut end)) =
        source_finite_number_prefix(input).map_err(|_| SourceNumberError)?
    else {
        return Err(SourceNumberError);
    };
    if end == input.len() {
        return Ok(value);
    }
    let bytes = input.as_bytes();
    if matches!(bytes.get(end), Some(b'+' | b'-')) {
        end += 1;
        if end == bytes.len() {
            return Err(SourceNumberError);
        }
    }
    if matches!(bytes.get(end), Some(b'd' | b'D')) {
        return process_source_number(&input.replace(['d', 'D'], "e"));
    }
    if matches!(bytes.get(end), Some(b'e' | b'E')) {
        end += 1;
        return bytes[end..]
            .iter()
            .all(u8::is_ascii_digit)
            .then_some(value)
            .ok_or(SourceNumberError);
    }
    Err(SourceNumberError)
}
