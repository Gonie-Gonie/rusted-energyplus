//! Literal nlohmann UBJSON reader; BJData-only markers are not UBJSON.
use super::{BinaryCursor, NativeBinaryDecodeError, Object, Value};

enum Frame {
    Array {
        values: Vec<Value>,
        remaining: Option<usize>,
        marker: Option<u8>,
    },
    Object {
        values: Object,
        key: Option<Vec<u8>>,
        remaining: Option<usize>,
        marker: Option<u8>,
    },
}

enum Parsed {
    Value(Value),
    Container(Frame),
}

enum Next {
    Value(Option<u8>),
    Complete,
}

pub(crate) fn decode_native_ubjson(bytes: &[u8]) -> Result<Value, NativeBinaryDecodeError> {
    let mut cursor = BinaryCursor::new(bytes);
    let mut stack = Vec::new();
    let mut pending = None;
    match parse_value(&mut cursor, None)? {
        Parsed::Value(value) => pending = Some(value),
        Parsed::Container(frame) => stack.push(frame),
    }
    loop {
        if let Some(value) = pending.take() {
            if let Some(frame) = stack.last_mut() {
                attach(frame, value, cursor.position())?;
            } else {
                cursor.finish(true)?;
                return Ok(value);
            }
        }
        let next = match stack.last_mut() {
            Some(frame) => next(frame, &mut cursor)?,
            None => {
                return Err(NativeBinaryDecodeError::new(
                    cursor.position(),
                    "UBJSON container state has no value",
                ));
            }
        };
        match next {
            Next::Value(marker) => match parse_value(&mut cursor, marker)? {
                Parsed::Value(value) => pending = Some(value),
                Parsed::Container(frame) => stack.push(frame),
            },
            Next::Complete => {
                pending = Some(match stack.pop() {
                    Some(Frame::Array { values, .. }) => Value::Array(values),
                    Some(Frame::Object { values, .. }) => Value::Object(values),
                    None => {
                        return Err(NativeBinaryDecodeError::new(
                            cursor.position(),
                            "UBJSON container completion has no frame",
                        ));
                    }
                });
            }
        }
    }
}

fn skip_noops(cursor: &mut BinaryCursor<'_>) -> Result<(), NativeBinaryDecodeError> {
    while cursor.peek() == Some(b'N') {
        cursor.read("UBJSON no-op")?;
    }
    Ok(())
}

fn integer_length(
    cursor: &mut BinaryCursor<'_>,
    ignore_noops: bool,
) -> Result<i64, NativeBinaryDecodeError> {
    if ignore_noops {
        skip_noops(cursor)?;
    }
    let marker = cursor.read("UBJSON length marker")?;
    match marker {
        b'U' => Ok(i64::from(cursor.read("UBJSON uint8 length")?)),
        b'i' => Ok(i64::from(i8::from_be_bytes(
            cursor.fixed("UBJSON int8 length")?,
        ))),
        b'I' => Ok(i64::from(i16::from_be_bytes(
            cursor.fixed("UBJSON int16 length")?,
        ))),
        b'l' => Ok(i64::from(i32::from_be_bytes(
            cursor.fixed("UBJSON int32 length")?,
        ))),
        b'L' => Ok(i64::from_be_bytes(cursor.fixed("UBJSON int64 length")?)),
        _ => Err(NativeBinaryDecodeError::new(
            cursor.position(),
            "UBJSON requires U/i/I/l/L length marker",
        )),
    }
}

fn count(cursor: &mut BinaryCursor<'_>) -> Result<usize, NativeBinaryDecodeError> {
    let number = integer_length(cursor, true)?;
    usize::try_from(number).map_err(|_| {
        NativeBinaryDecodeError::new(
            cursor.position(),
            "UBJSON count is negative or overflows size_t",
        )
    })
}

fn string(cursor: &mut BinaryCursor<'_>) -> Result<Vec<u8>, NativeBinaryDecodeError> {
    let length = integer_length(cursor, false)?;
    // get_string<signed>(len) loops zero times for a negative length.
    if length < 0 {
        return Ok(Vec::new());
    }
    let length = usize::try_from(length).map_err(|_| {
        NativeBinaryDecodeError::new(cursor.position(), "UBJSON string length exceeds size_t")
    })?;
    Ok(cursor.take(length, "UBJSON string bytes")?.to_vec())
}

fn header(
    cursor: &mut BinaryCursor<'_>,
) -> Result<(Option<usize>, Option<u8>), NativeBinaryDecodeError> {
    skip_noops(cursor)?;
    let remaining = if cursor.peek() == Some(b'$') {
        cursor.read("UBJSON optimized type prefix")?;
        let marker = cursor.read("UBJSON optimized type")?;
        skip_noops(cursor)?;
        if cursor.read("UBJSON optimized count prefix")? != b'#' {
            return Err(NativeBinaryDecodeError::new(
                cursor.position(),
                "UBJSON optimized type requires # count",
            ));
        }
        // Native uses zero as its absent-type sentinel, including literal $\0.
        return Ok((Some(count(cursor)?), (marker != 0).then_some(marker)));
    } else if cursor.peek() == Some(b'#') {
        cursor.read("UBJSON count prefix")?;
        Some(count(cursor)?)
    } else {
        None
    };
    Ok((remaining, None))
}

fn parse_value(
    cursor: &mut BinaryCursor<'_>,
    supplied: Option<u8>,
) -> Result<Parsed, NativeBinaryDecodeError> {
    let marker = if let Some(marker) = supplied {
        marker
    } else {
        skip_noops(cursor)?;
        cursor.read("UBJSON value marker")?
    };
    let value = match marker {
        b'T' => Value::Boolean(true),
        b'F' => Value::Boolean(false),
        b'Z' => Value::Null,
        b'U' => Value::Unsigned(u64::from(cursor.read("UBJSON uint8")?)),
        b'i' => Value::Signed(i64::from(i8::from_be_bytes(cursor.fixed("UBJSON int8")?))),
        b'I' => Value::Signed(i64::from(i16::from_be_bytes(cursor.fixed("UBJSON int16")?))),
        b'l' => Value::Signed(i64::from(i32::from_be_bytes(cursor.fixed("UBJSON int32")?))),
        b'L' => Value::Signed(i64::from_be_bytes(cursor.fixed("UBJSON int64")?)),
        b'd' => Value::Real(f64::from(f32::from_be_bytes(
            cursor.fixed("UBJSON float32")?,
        ))),
        b'D' => Value::Real(f64::from_be_bytes(cursor.fixed("UBJSON float64")?)),
        b'H' => {
            let length = count(cursor)?;
            let start = cursor.position();
            high_precision(cursor.take(length, "UBJSON high-precision bytes")?, start)?
        }
        b'C' => {
            let byte = cursor.read("UBJSON character")?;
            if byte > 127 {
                return Err(NativeBinaryDecodeError::new(
                    cursor.position(),
                    "UBJSON character must be 0x00..0x7f",
                ));
            }
            Value::String(vec![byte])
        }
        b'S' => Value::String(string(cursor)?),
        b'[' | b'{' => {
            let is_object = marker == b'{';
            let (remaining, marker) = header(cursor)?;
            return Ok(Parsed::Container(if is_object {
                Frame::Object {
                    values: Object::new(),
                    key: None,
                    remaining,
                    marker,
                }
            } else {
                // The container's value marker is independent of its optimized
                // element marker returned by header().
                Frame::Array {
                    values: Vec::new(),
                    remaining,
                    marker,
                }
            }));
        }
        _ => {
            return Err(NativeBinaryDecodeError::new(
                cursor.position(),
                "invalid UBJSON value marker",
            ));
        }
    };
    Ok(Parsed::Value(value))
}

fn next(frame: &mut Frame, cursor: &mut BinaryCursor<'_>) -> Result<Next, NativeBinaryDecodeError> {
    match frame {
        Frame::Array {
            remaining, marker, ..
        } => {
            if *remaining == Some(0) || (remaining.is_some() && *marker == Some(b'N')) {
                return Ok(Next::Complete);
            }
            if remaining.is_none() {
                skip_noops(cursor)?;
                if cursor.peek() == Some(b']') {
                    cursor.read("UBJSON array end")?;
                    return Ok(Next::Complete);
                }
            }
            Ok(Next::Value(*marker))
        }
        Frame::Object {
            key,
            remaining,
            marker,
            ..
        } => {
            if *remaining == Some(0) {
                return Ok(Next::Complete);
            }
            if remaining.is_none() {
                skip_noops(cursor)?;
                if cursor.peek() == Some(b'}') {
                    cursor.read("UBJSON object end")?;
                    return Ok(Next::Complete);
                }
            }
            *key = Some(string(cursor)?);
            Ok(Next::Value(*marker))
        }
    }
}

fn attach(frame: &mut Frame, value: Value, at: usize) -> Result<(), NativeBinaryDecodeError> {
    let remaining = match frame {
        Frame::Array {
            values, remaining, ..
        } => {
            values.push(value);
            remaining
        }
        Frame::Object {
            values,
            key,
            remaining,
            ..
        } => {
            let key = key
                .take()
                .ok_or_else(|| NativeBinaryDecodeError::new(at, "UBJSON object key state"))?;
            values.insert(key, value);
            remaining
        }
    };
    if let Some(remaining) = remaining {
        *remaining -= 1;
    }
    Ok(())
}

fn high_precision(bytes: &[u8], start: usize) -> Result<Value, NativeBinaryDecodeError> {
    let bad = |at, cause| NativeBinaryDecodeError::new(start + at, cause);
    let mut at = 0;
    if bytes.first() == Some(&0xef) {
        if !bytes.starts_with(b"\xef\xbb\xbf") {
            return Err(bad(0, "invalid UBJSON high-precision number BOM"));
        }
        at = 3;
    }
    while bytes
        .get(at)
        .is_some_and(|byte| matches!(byte, b' ' | b'\t' | b'\r' | b'\n'))
    {
        at += 1;
    }
    let number_start = at;
    let negative = bytes.get(at) == Some(&b'-');
    if negative {
        at += 1;
    }
    match bytes.get(at) {
        Some(b'0') => at += 1,
        Some(b'1'..=b'9') => {
            at += 1;
            while bytes.get(at).is_some_and(u8::is_ascii_digit) {
                at += 1;
            }
        }
        _ => return Err(bad(at, "invalid UBJSON high-precision number integer part")),
    }
    let mut floating = false;
    if bytes.get(at) == Some(&b'.') {
        floating = true;
        at += 1;
        let first = at;
        while bytes.get(at).is_some_and(u8::is_ascii_digit) {
            at += 1;
        }
        if at == first {
            return Err(bad(at, "UBJSON high-precision decimal requires a digit"));
        }
    }
    if bytes
        .get(at)
        .is_some_and(|byte| matches!(byte, b'e' | b'E'))
    {
        floating = true;
        at += 1;
        if bytes
            .get(at)
            .is_some_and(|byte| matches!(byte, b'+' | b'-'))
        {
            at += 1;
        }
        let first = at;
        while bytes.get(at).is_some_and(u8::is_ascii_digit) {
            at += 1;
        }
        if at == first {
            return Err(bad(at, "UBJSON high-precision exponent requires a digit"));
        }
    }
    let number_end = at;
    while bytes
        .get(at)
        .is_some_and(|byte| matches!(byte, b' ' | b'\t' | b'\r' | b'\n'))
    {
        at += 1;
    }
    // The second lexer scan returns end_of_input for NUL without checking
    // later bytes in the already consumed, length-delimited H payload.
    if bytes.get(at).is_some_and(|byte| *byte != 0) {
        return Err(bad(at, "UBJSON high-precision number has a second token"));
    }
    let token = std::str::from_utf8(&bytes[number_start..number_end])
        .map_err(|_| bad(number_start, "UBJSON number token must be ASCII"))?;
    if !floating {
        if negative {
            if let Ok(value) = token.parse::<i64>() {
                return Ok(Value::Signed(value));
            }
        } else if let Ok(value) = token.parse::<u64>() {
            return Ok(Value::Unsigned(value));
        }
    }
    token.parse::<f64>().map(Value::Real).map_err(|_| {
        bad(
            number_start,
            "UBJSON high-precision floating conversion failed",
        )
    })
}
