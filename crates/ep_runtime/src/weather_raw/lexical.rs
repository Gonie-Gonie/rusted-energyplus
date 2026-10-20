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

pub(super) fn read_real(
    input: &str,
    cursor: &mut usize,
    target: &mut f64,
) -> Result<bool, &'static str> {
    let Some(start) = list_start(input, *cursor) else {
        return Ok(false);
    };
    let Some((value, consumed)) = ep_model::source_finite_number_prefix(&input[start..])? else {
        return Ok(false);
    };
    *target = value;
    *cursor = start + consumed;
    Ok(true)
}

/// Preserve the existing weather caller's unit-error/flag adapter.
pub(super) fn process_number(input: &str) -> Result<f64, ()> {
    ep_model::process_source_number(input).map_err(|_| ())
}
