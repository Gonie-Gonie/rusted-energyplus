//! FileSystem::readJSON -> nlohmann::json::from_cbor default semantics.
//! Pinned json.hpp 10205-10971; no tag handler override or SAX callback.
use super::{BinaryCursor, NativeBinaryDecodeError, Object, Value};

enum Frame {
    Array {
        values: Vec<Value>,
        remaining: Option<u64>,
    },
    Object {
        values: Object,
        remaining: Option<u64>,
        key: Option<Vec<u8>>,
    },
}

enum Item {
    Value(Value),
    Array(Option<u64>),
    Object(Option<u64>),
}

/// Native's default from_cbor is strict and rejects every tag, even unused data.
/// Structure frames and chunk depth retain Native nesting without a depth cap.
pub(crate) fn decode_native_cbor(bytes: &[u8]) -> Result<Value, NativeBinaryDecodeError> {
    let mut cursor = BinaryCursor::new(bytes);
    let mut stack = Vec::<Frame>::new();
    let mut completed = None;
    loop {
        if let Some(value) = completed.take() {
            match stack.last_mut() {
                Some(Frame::Array { values, remaining }) => {
                    values.push(value);
                    if let Some(remaining) = remaining {
                        *remaining -= 1;
                    }
                }
                Some(Frame::Object {
                    values,
                    remaining,
                    key,
                }) => {
                    // A key is read before requesting its value. Native DOM
                    // operator[] assignment makes repeated byte keys last-win.
                    let key = key.take().ok_or_else(|| {
                        NativeBinaryDecodeError::new(cursor.position(), "CBOR object key state")
                    })?;
                    values.insert(key, value);
                    if let Some(remaining) = remaining {
                        *remaining -= 1;
                    }
                }
                None => {
                    cursor.finish(false)?;
                    return Ok(value);
                }
            }
            continue;
        }

        let definite_end = matches!(
            stack.last(),
            Some(Frame::Array {
                remaining: Some(0),
                ..
            }) | Some(Frame::Object {
                remaining: Some(0),
                key: None,
                ..
            })
        );
        let indefinite_end = cursor.peek() == Some(0xff)
            && matches!(
                stack.last(),
                Some(Frame::Array {
                    remaining: None,
                    ..
                }) | Some(Frame::Object {
                    remaining: None,
                    key: None,
                    ..
                })
            );
        if definite_end || indefinite_end {
            if indefinite_end {
                cursor.read("unexpected end of CBOR container")?;
            }
            completed = Some(
                match stack.pop().ok_or_else(|| {
                    NativeBinaryDecodeError::new(
                        cursor.position(),
                        "CBOR container completion state",
                    )
                })? {
                    Frame::Array { values, .. } => Value::Array(values),
                    Frame::Object { values, .. } => Value::Object(values),
                },
            );
            continue;
        }

        if matches!(stack.last(), Some(Frame::Object { key: None, .. })) {
            // get_cbor_object calls get_cbor_string directly, including its
            // string-context EOF and its indefinite-chunk handling.
            let marker = cursor.read("unexpected end of CBOR string")?;
            let parsed_key = byte_chunks(&mut cursor, marker, false)?;
            if let Some(Frame::Object { key, .. }) = stack.last_mut() {
                *key = Some(parsed_key);
            }
        }

        match item(&mut cursor)? {
            Item::Value(value) => completed = Some(value),
            Item::Array(remaining) => stack.push(Frame::Array {
                values: Vec::new(),
                remaining,
            }),
            Item::Object(remaining) => stack.push(Frame::Object {
                values: Object::new(),
                remaining,
                key: None,
            }),
        }
    }
}

fn item(cursor: &mut BinaryCursor<'_>) -> Result<Item, NativeBinaryDecodeError> {
    let marker = cursor.read("unexpected end of CBOR value")?;
    let value = match marker {
        0x00..=0x17 => Value::Unsigned(u64::from(marker)),
        0x18..=0x1b => Value::Unsigned(argument(cursor, marker)?),
        0x20..=0x37 => Value::Signed(-1 - i64::from(marker - 0x20)),
        0x38..=0x3b => {
            // Native 10310-10314 casts the u64 operand to its signed type
            // before -1-n. This is the pinned 64-bit two's-complement result,
            // including encodings beyond the mathematical signed range.
            Value::Signed((!argument(cursor, marker)?) as i64)
        }
        0x40..=0x5b | 0x5f => Value::Binary {
            bytes: byte_chunks(cursor, marker, true)?,
            subtype: None,
        },
        0x60..=0x7b | 0x7f => Value::String(byte_chunks(cursor, marker, false)?),
        0x80..=0x9b => return Ok(Item::Array(container_length(cursor, marker)?)),
        0x9f => return Ok(Item::Array(None)),
        0xa0..=0xbb => return Ok(Item::Object(container_length(cursor, marker)?)),
        0xbf => return Ok(Item::Object(None)),
        0xf4 => Value::Boolean(false),
        0xf5 => Value::Boolean(true),
        0xf6 => Value::Null,
        0xf9 => {
            let half = u16::from_be_bytes(cursor.fixed("unexpected end of CBOR number")?);
            Value::Real(half_real64(half))
        }
        0xfa => {
            let bits = u32::from_be_bytes(cursor.fixed("unexpected end of CBOR number")?);
            Value::Real(f64::from(f32::from_bits(bits)))
        }
        0xfb => {
            let bits = u64::from_be_bytes(cursor.fixed("unexpected end of CBOR number")?);
            Value::Real(f64::from_bits(bits))
        }
        _ => {
            // Includes all tag markers, F7 undefined, invalid simple values,
            // reserved length markers and an unpaired break. Do not read any
            // tag payload: Native default tag_handler is error.
            return Err(NativeBinaryDecodeError::new(
                cursor.position() - 1,
                "invalid CBOR value byte",
            ));
        }
    };
    Ok(Item::Value(value))
}

/// get_number is network-endian; noncanonical wider integer encodings are valid.
fn argument(cursor: &mut BinaryCursor<'_>, marker: u8) -> Result<u64, NativeBinaryDecodeError> {
    match marker & 0x1f {
        value @ 0..=23 => Ok(u64::from(value)),
        24 => Ok(u64::from(cursor.read("unexpected end of CBOR number")?)),
        25 => Ok(u64::from(u16::from_be_bytes(
            cursor.fixed("unexpected end of CBOR number")?,
        ))),
        26 => Ok(u64::from(u32::from_be_bytes(
            cursor.fixed("unexpected end of CBOR number")?,
        ))),
        27 => Ok(u64::from_be_bytes(
            cursor.fixed("unexpected end of CBOR number")?,
        )),
        _ => Err(NativeBinaryDecodeError::new(
            cursor.position() - 1,
            "invalid CBOR length byte",
        )),
    }
}

fn container_length(
    cursor: &mut BinaryCursor<'_>,
    marker: u8,
) -> Result<Option<u64>, NativeBinaryDecodeError> {
    let length = argument(cursor, marker)?;
    // Native detail::unknown_size() is size_t(-1) on the pinned 64-bit host.
    // A definite 64-bit all-ones length aliases that same indefinite sentinel.
    Ok((length != u64::MAX).then_some(length))
}

/// Native permits recursively nested indefinite chunks of the same string kind.
/// Copy original bytes; neither value strings nor map keys are UTF-8 validated.
fn byte_chunks(
    cursor: &mut BinaryCursor<'_>,
    mut marker: u8,
    binary: bool,
) -> Result<Vec<u8>, NativeBinaryDecodeError> {
    let base = if binary { 0x40 } else { 0x60 };
    let indefinite = base + 0x1f;
    let eof = if binary {
        "unexpected end of CBOR binary"
    } else {
        "unexpected end of CBOR string"
    };
    let invalid = if binary {
        "invalid CBOR binary length byte"
    } else {
        "invalid CBOR string length byte"
    };
    let mut bytes = Vec::new();
    let mut depth = 0usize;
    loop {
        if marker == indefinite {
            depth += 1;
        } else if (base..=base + 0x1b).contains(&marker) {
            let length = argument(cursor, marker)?;
            // uint64_t and size_t have the same width in the pinned Native.
            bytes.extend_from_slice(cursor.take(length as usize, eof)?);
        } else {
            return Err(NativeBinaryDecodeError::new(cursor.position() - 1, invalid));
        }

        if depth == 0 {
            return Ok(bytes);
        }
        marker = cursor.read(eof)?;
        while marker == 0xff {
            depth -= 1;
            if depth == 0 {
                return Ok(bytes);
            }
            marker = cursor.read(eof)?;
        }
    }
}

/// Exact real64 values of Native's RFC7049 ldexp half decoder (10616-10661).
fn half_real64(half: u16) -> f64 {
    let exponent = (half >> 10) & 0x1f;
    let mantissa = half & 0x03ff;
    let value = match exponent {
        0 => f64::from(mantissa) * (1.0 / 16_777_216.0),
        31 if mantissa == 0 => f64::INFINITY,
        31 => f64::NAN,
        _ => {
            // Every finite half is exactly representable as real64. This is
            // Native ldexp(mantissa+1024, exponent-25) without libm rounding.
            f64::from_bits((u64::from(exponent) + 1008) << 52 | u64::from(mantissa) << 42)
        }
    };
    if half & 0x8000 != 0 { -value } else { value }
}
