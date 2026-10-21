//! FileSystem::readJSON -> nlohmann::json::from_msgpack default semantics.
//! Pinned json.hpp 10981-11601; extension payloads retain bytes and subtype.
use super::{BinaryCursor, NativeBinaryDecodeError, Object, Value};

enum Frame {
    Array {
        values: Vec<Value>,
        remaining: u32,
    },
    Object {
        values: Object,
        remaining: u32,
        key: Option<Vec<u8>>,
    },
}

enum Item {
    Value(Value),
    Array(u32),
    Object(u32),
}

/// Strict, whole-document decode without a callback or an invented depth limit.
pub(crate) fn decode_native_msgpack(bytes: &[u8]) -> Result<Value, NativeBinaryDecodeError> {
    let mut cursor = BinaryCursor::new(bytes);
    let mut stack = Vec::<Frame>::new();
    let mut completed = None;
    loop {
        if let Some(value) = completed.take() {
            match stack.last_mut() {
                Some(Frame::Array { values, remaining }) => {
                    values.push(value);
                    *remaining -= 1;
                }
                Some(Frame::Object {
                    values,
                    remaining,
                    key,
                }) => {
                    let key = key.take().ok_or_else(|| {
                        NativeBinaryDecodeError::new(
                            cursor.position(),
                            "MessagePack object key state",
                        )
                    })?;
                    values.insert(key, value);
                    *remaining -= 1;
                }
                None => {
                    cursor.finish(false)?;
                    return Ok(value);
                }
            }
            continue;
        }

        if matches!(
            stack.last(),
            Some(Frame::Array { remaining: 0, .. })
                | Some(Frame::Object {
                    remaining: 0,
                    key: None,
                    ..
                })
        ) {
            completed = Some(
                match stack.pop().ok_or_else(|| {
                    NativeBinaryDecodeError::new(
                        cursor.position(),
                        "MessagePack container completion state",
                    )
                })? {
                    Frame::Array { values, .. } => Value::Array(values),
                    Frame::Object { values, .. } => Value::Object(values),
                },
            );
            continue;
        }

        if matches!(stack.last(), Some(Frame::Object { key: None, .. })) {
            let marker = cursor.read("unexpected end of MessagePack string")?;
            let parsed_key = string(&mut cursor, marker)?;
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
    let marker = cursor.read("unexpected end of MessagePack value")?;
    let value = match marker {
        0x00..=0x7f => Value::Unsigned(u64::from(marker)),
        0x80..=0x8f => return Ok(Item::Object(u32::from(marker & 0x0f))),
        0x90..=0x9f => return Ok(Item::Array(u32::from(marker & 0x0f))),
        0xa0..=0xbf | 0xd9..=0xdb => Value::String(string(cursor, marker)?),
        0xc0 => Value::Null,
        0xc2 => Value::Boolean(false),
        0xc3 => Value::Boolean(true),
        0xc4..=0xc9 | 0xd4..=0xd8 => binary(cursor, marker)?,
        0xca => {
            let bits = u32::from_be_bytes(cursor.fixed("unexpected end of MessagePack number")?);
            Value::Real(f64::from(f32::from_bits(bits)))
        }
        0xcb => {
            let bits = u64::from_be_bytes(cursor.fixed("unexpected end of MessagePack number")?);
            Value::Real(f64::from_bits(bits))
        }
        0xcc => Value::Unsigned(u64::from(
            cursor.read("unexpected end of MessagePack number")?,
        )),
        0xcd => Value::Unsigned(u64::from(u16::from_be_bytes(
            cursor.fixed("unexpected end of MessagePack number")?,
        ))),
        0xce => Value::Unsigned(u64::from(u32::from_be_bytes(
            cursor.fixed("unexpected end of MessagePack number")?,
        ))),
        0xcf => Value::Unsigned(u64::from_be_bytes(
            cursor.fixed("unexpected end of MessagePack number")?,
        )),
        0xd0 => Value::Signed(i64::from(
            cursor.read("unexpected end of MessagePack number")? as i8,
        )),
        0xd1 => Value::Signed(i64::from(i16::from_be_bytes(
            cursor.fixed("unexpected end of MessagePack number")?,
        ))),
        0xd2 => Value::Signed(i64::from(i32::from_be_bytes(
            cursor.fixed("unexpected end of MessagePack number")?,
        ))),
        0xd3 => Value::Signed(i64::from_be_bytes(
            cursor.fixed("unexpected end of MessagePack number")?,
        )),
        0xdc => {
            return Ok(Item::Array(u32::from(u16::from_be_bytes(
                cursor.fixed("unexpected end of MessagePack number")?,
            ))));
        }
        0xdd => {
            return Ok(Item::Array(u32::from_be_bytes(
                cursor.fixed("unexpected end of MessagePack number")?,
            )));
        }
        0xde => {
            return Ok(Item::Object(u32::from(u16::from_be_bytes(
                cursor.fixed("unexpected end of MessagePack number")?,
            ))));
        }
        0xdf => {
            return Ok(Item::Object(u32::from_be_bytes(
                cursor.fixed("unexpected end of MessagePack number")?,
            )));
        }
        0xe0..=0xff => Value::Signed(i64::from(marker as i8)),
        _ => {
            return Err(NativeBinaryDecodeError::new(
                cursor.position() - 1,
                "invalid MessagePack value byte",
            ));
        }
    };
    Ok(Item::Value(value))
}

fn string(cursor: &mut BinaryCursor<'_>, marker: u8) -> Result<Vec<u8>, NativeBinaryDecodeError> {
    let length = match marker {
        0xa0..=0xbf => u32::from(marker & 0x1f),
        0xd9 => u32::from(cursor.read("unexpected end of MessagePack number")?),
        0xda => u32::from(u16::from_be_bytes(
            cursor.fixed("unexpected end of MessagePack number")?,
        )),
        0xdb => u32::from_be_bytes(cursor.fixed("unexpected end of MessagePack number")?),
        _ => {
            return Err(NativeBinaryDecodeError::new(
                cursor.position() - 1,
                "invalid MessagePack string length byte",
            ));
        }
    };
    Ok(cursor
        .take(length as usize, "unexpected end of MessagePack string")?
        .to_vec())
}

fn binary(cursor: &mut BinaryCursor<'_>, marker: u8) -> Result<Value, NativeBinaryDecodeError> {
    let length = match marker {
        0xc4 | 0xc7 => u32::from(cursor.read("unexpected end of MessagePack number")?),
        0xc5 | 0xc8 => u32::from(u16::from_be_bytes(
            cursor.fixed("unexpected end of MessagePack number")?,
        )),
        0xc6 | 0xc9 => u32::from_be_bytes(cursor.fixed("unexpected end of MessagePack number")?),
        0xd4 => 1,
        0xd5 => 2,
        0xd6 => 4,
        0xd7 => 8,
        0xd8 => 16,
        _ => unreachable!("binary only receives MessagePack binary markers"),
    };
    let subtype = if matches!(marker, 0xc7..=0xc9 | 0xd4..=0xd8) {
        // Native reads int8 then static_cast<uint8_t>; keep -1 as subtype255,
        // not a timestamp or a signed semantic extension.
        Some(u64::from(
            cursor.read("unexpected end of MessagePack number")?,
        ))
    } else {
        None
    };
    let bytes = cursor
        .take(length as usize, "unexpected end of MessagePack binary")?
        .to_vec();
    Ok(Value::Binary { bytes, subtype })
}
