//! Literal nlohmann BSON subset, including its raw-byte and document-size rules.
use super::{BinaryCursor, NativeBinaryDecodeError, Object, Value};

enum Frame {
    Object {
        values: Object,
        key: Option<Vec<u8>>,
    },
    Array {
        values: Vec<Value>,
    },
}

enum Parsed {
    Value(Value),
    Container(Frame),
}

pub(crate) fn decode_native_bson(bytes: &[u8]) -> Result<Value, NativeBinaryDecodeError> {
    let mut cursor = BinaryCursor::new(bytes);
    let mut stack = vec![document(&mut cursor, false)?];
    let mut pending = None;
    loop {
        if let Some(value) = pending.take() {
            if let Some(frame) = stack.last_mut() {
                match frame {
                    Frame::Array { values } => values.push(value),
                    Frame::Object { values, key } => {
                        let key = key.take().ok_or_else(|| {
                            NativeBinaryDecodeError::new(cursor.position(), "BSON object key state")
                        })?;
                        values.insert(key, value);
                    }
                }
            } else {
                cursor.finish(false)?;
                return Ok(value);
            }
        }
        let kind = cursor.read("BSON element type or document end")?;
        if kind == 0 {
            pending = Some(match stack.pop() {
                Some(Frame::Object { values, .. }) => Value::Object(values),
                Some(Frame::Array { values }) => Value::Array(values),
                None => {
                    return Err(NativeBinaryDecodeError::new(
                        cursor.position(),
                        "BSON document end without frame",
                    ));
                }
            });
            continue;
        }
        let key = c_string(&mut cursor)?;
        if let Some(Frame::Object { key: target, .. }) = stack.last_mut() {
            *target = Some(key);
        }
        // Array names are consumed but never validated, sorted or interpreted.
        match element(&mut cursor, kind)? {
            Parsed::Value(value) => pending = Some(value),
            Parsed::Container(frame) => stack.push(frame),
        }
    }
}

fn document(cursor: &mut BinaryCursor<'_>, array: bool) -> Result<Frame, NativeBinaryDecodeError> {
    // Native reads int32 document_size but never uses or checks the value.
    let _document_size = i32::from_le_bytes(cursor.fixed("BSON document size")?);
    Ok(if array {
        Frame::Array { values: Vec::new() }
    } else {
        Frame::Object {
            values: Object::new(),
            key: None,
        }
    })
}

fn c_string(cursor: &mut BinaryCursor<'_>) -> Result<Vec<u8>, NativeBinaryDecodeError> {
    let mut bytes = Vec::new();
    loop {
        let byte = cursor.read("BSON cstring")?;
        if byte == 0 {
            return Ok(bytes);
        }
        bytes.push(byte);
    }
}

fn element(cursor: &mut BinaryCursor<'_>, kind: u8) -> Result<Parsed, NativeBinaryDecodeError> {
    let value = match kind {
        0x01 => Value::Real(f64::from_le_bytes(cursor.fixed("BSON double")?)),
        0x02 => {
            let length = i32::from_le_bytes(cursor.fixed("BSON string length")?);
            if length < 1 {
                return Err(NativeBinaryDecodeError::new(
                    cursor.position(),
                    "BSON string length must be at least one",
                ));
            }
            let bytes = cursor
                .take((length - 1) as usize, "BSON string bytes")?
                .to_vec();
            // get_bson_string consumes a final byte; it does not require zero.
            cursor.read("BSON string final byte")?;
            Value::String(bytes)
        }
        0x03 => return Ok(Parsed::Container(document(cursor, false)?)),
        0x04 => return Ok(Parsed::Container(document(cursor, true)?)),
        0x05 => {
            let length = i32::from_le_bytes(cursor.fixed("BSON binary length")?);
            if length < 0 {
                return Err(NativeBinaryDecodeError::new(
                    cursor.position(),
                    "BSON binary length cannot be negative",
                ));
            }
            let subtype = cursor.read("BSON binary subtype")?;
            let bytes = cursor.take(length as usize, "BSON binary bytes")?.to_vec();
            // Subtype2 has no old-binary inner-length special case in Native.
            Value::Binary {
                bytes,
                subtype: Some(u64::from(subtype)),
            }
        }
        0x08 => Value::Boolean(cursor.read("BSON boolean")? != 0),
        0x0a => Value::Null,
        0x10 => Value::Signed(i64::from(i32::from_le_bytes(cursor.fixed("BSON int32")?))),
        0x12 => Value::Signed(i64::from_le_bytes(cursor.fixed("BSON int64")?)),
        0x11 => Value::Unsigned(u64::from_le_bytes(cursor.fixed("BSON uint64")?)),
        _ => {
            return Err(NativeBinaryDecodeError::new(
                cursor.position(),
                "unsupported Native BSON record type",
            ));
        }
    };
    Ok(Parsed::Value(value))
}
