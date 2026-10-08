//! Separate source lexical routes for list reads and `ProcessNumber`.

fn list_start(input: &str, cursor: usize) -> Option<usize> {
    if cursor >= input.len() {
        return None;
    }
    let bytes = input.as_bytes();
    let mut position = cursor;
    while bytes.get(position) == Some(&b',') {
        position += 1;
    }
    while bytes.get(position) == Some(&b' ') {
        position += 1;
    }
    (position < bytes.len()).then_some(position)
}

/// Failed list reads leave both the reference argument and its cursor unchanged.
pub(super) fn read_int(input: &str, cursor: &mut usize, target: &mut i32) -> bool {
    let Some(start) = list_start(input, *cursor) else {
        return false;
    };
    let bytes = input.as_bytes();
    let mut end = start + usize::from(bytes[start] == b'-');
    let digits = end;
    while bytes.get(end).is_some_and(u8::is_ascii_digit) {
        end += 1;
    }
    if end == digits {
        return false;
    }
    let Ok(value) = input[start..end].parse::<i32>() else {
        return false;
    };
    *target = value;
    *cursor = end;
    true
}

/// The admitted decimal grammar consumes a prefix like `fast_float::from_chars`.
/// Rust's decimal conversion supplies the value; bit equality is checked against
/// the executed original separately. Non-finite and underflow-to-zero prefixes
/// are excluded because their original list-read writes are outside this scope.
fn finite_prefix(input: &str) -> Result<Option<(f64, usize)>, &'static str> {
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

pub(super) fn read_real(
    input: &str,
    cursor: &mut usize,
    target: &mut f64,
) -> Result<bool, &'static str> {
    let Some(start) = list_start(input, *cursor) else {
        return Ok(false);
    };
    let Some((value, consumed)) = finite_prefix(&input[start..])? else {
        return Ok(false);
    };
    *target = value;
    *cursor = start + consumed;
    Ok(true)
}

/// `ProcessNumber` differs from list reads: ASCII-space-only input returns +0,
/// FORTRAN D normalization reparses the token, and invalid input returns an error.
/// Header callers map errors to the source result +0 and set their error flag.
pub(super) fn process_number(input: &str) -> Result<f64, ()> {
    let input = input.trim_matches(' ');
    if input.is_empty() {
        return Ok(0.0);
    }
    let Some((value, mut end)) = finite_prefix(input).map_err(|_| ())? else {
        return Err(());
    };
    if end == input.len() {
        return Ok(value);
    }
    let bytes = input.as_bytes();
    if matches!(bytes.get(end), Some(b'+' | b'-')) {
        end += 1;
        if end == bytes.len() {
            return Err(());
        }
    }
    if matches!(bytes.get(end), Some(b'd' | b'D')) {
        return process_number(&input.replace(['d', 'D'], "e"));
    }
    if matches!(bytes.get(end), Some(b'e' | b'E')) {
        end += 1;
        return bytes[end..]
            .iter()
            .all(u8::is_ascii_digit)
            .then_some(value)
            .ok_or(());
    }
    Err(())
}
