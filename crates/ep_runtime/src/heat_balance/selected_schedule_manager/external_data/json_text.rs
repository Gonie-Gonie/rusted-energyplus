//! Text readJSON values: comments/BOM, exact integer kinds, direct Real64 data.
//! Iterative structure parsing adds no serde_json default nesting-depth fence.
use super::{NativeExternalDataObject as Object, NativeExternalDataValue as Value};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum NativeExternalJsonErrorKind {
    Syntax,
    InvalidString,
    NumberOverflow,
}

/// Structured cause for the file host; not a fabricated nlohmann exception text.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct NativeExternalJsonError {
    /// Zero-based original-file byte offset, including comments and BOM.
    pub(crate) byte_offset: usize,
    pub(crate) kind: NativeExternalJsonErrorKind,
    pub(crate) cause: &'static str,
}

#[derive(Clone, Copy)]
enum ArrayState {
    FirstOrEnd,
    Value,
    CommaOrEnd,
}

#[derive(Clone, Copy)]
enum ObjectState {
    FirstKeyOrEnd,
    Key,
    Colon,
    Value,
    CommaOrEnd,
}

enum Frame {
    Array {
        values: Vec<Value>,
        state: ArrayState,
    },
    Object {
        values: Object,
        key: Option<Vec<u8>>,
        state: ObjectState,
    },
}

#[derive(Clone, Copy)]
enum Need {
    RootValue,
    Array(ArrayState),
    Object(ObjectState),
}

struct Parser<'a> {
    bytes: &'a [u8],
    at: usize,
}

/// Native FileSystem::readJSON's text branch (ignore_comments=true).
/// Binary JSON formats use the separate Native binary decoder.
pub(crate) fn decode_native_text_json(bytes: &[u8]) -> Result<Value, NativeExternalJsonError> {
    let mut parser = Parser { bytes, at: 0 };
    if bytes.starts_with(b"\xef\xbb\xbf") {
        parser.at = 3;
    }
    parser.parse()
}

impl Parser<'_> {
    fn error(
        &self,
        kind: NativeExternalJsonErrorKind,
        cause: &'static str,
    ) -> NativeExternalJsonError {
        NativeExternalJsonError {
            byte_offset: self.at,
            kind,
            cause,
        }
    }

    fn syntax(&self, cause: &'static str) -> NativeExternalJsonError {
        self.error(NativeExternalJsonErrorKind::Syntax, cause)
    }

    fn whitespace_and_comments(&mut self) -> Result<(), NativeExternalJsonError> {
        loop {
            while self
                .bytes
                .get(self.at)
                .is_some_and(|b| matches!(b, b' ' | b'\t' | b'\r' | b'\n'))
            {
                self.at += 1;
            }
            match self.bytes.get(self.at..self.at.saturating_add(2)) {
                Some(b"//") => {
                    self.at += 2;
                    while self
                        .bytes
                        .get(self.at)
                        .is_some_and(|b| !matches!(b, b'\r' | b'\n' | 0))
                    {
                        self.at += 1;
                    }
                    // Native scan_comment consumes its newline/NUL terminator,
                    // then scan resumes from the following byte.
                    if self.at < self.bytes.len() {
                        self.at += 1;
                    }
                }
                Some(b"/*") => {
                    self.at += 2;
                    loop {
                        if self.bytes.get(self.at..self.at.saturating_add(2)) == Some(b"*/") {
                            self.at += 2;
                            break;
                        }
                        if self.at == self.bytes.len() || self.bytes[self.at] == 0 {
                            return Err(self.syntax("unterminated JSON block comment"));
                        }
                        self.at += 1;
                    }
                }
                _ => return Ok(()),
            }
        }
    }

    fn parse(&mut self) -> Result<Value, NativeExternalJsonError> {
        let mut stack: Vec<Frame> = Vec::new();
        let mut pending = None;
        loop {
            if let Some(value) = pending.take() {
                match stack.last_mut() {
                    None => {
                        self.whitespace_and_comments()?;
                        // Native lexer scan8618 treats a raw NUL token as EOF.
                        return if self.at == self.bytes.len() || self.bytes.get(self.at) == Some(&0)
                        {
                            Ok(value)
                        } else {
                            Err(self.syntax("unexpected data after JSON value"))
                        };
                    }
                    Some(Frame::Array { values, state }) => {
                        values.push(value);
                        *state = ArrayState::CommaOrEnd;
                    }
                    Some(Frame::Object { values, key, state }) => {
                        let name = key
                            .take()
                            .ok_or_else(|| self.syntax("missing JSON object key"))?;
                        // Native's object map retains the last duplicate key.
                        values.insert(name, value);
                        *state = ObjectState::CommaOrEnd;
                    }
                }
            }
            self.whitespace_and_comments()?;
            let need = match stack.last() {
                None => Need::RootValue,
                Some(Frame::Array { state, .. }) => Need::Array(*state),
                Some(Frame::Object { state, .. }) => Need::Object(*state),
            };
            match need {
                Need::Array(ArrayState::CommaOrEnd) => match self.bytes.get(self.at) {
                    Some(b',') => {
                        self.at += 1;
                        if let Some(Frame::Array { state, .. }) = stack.last_mut() {
                            *state = ArrayState::Value;
                        }
                        continue;
                    }
                    Some(b']') => {
                        self.at += 1;
                        if let Some(Frame::Array { values, .. }) = stack.pop() {
                            pending = Some(Value::Array(values));
                        }
                        continue;
                    }
                    _ => return Err(self.syntax("expected comma or end of JSON array")),
                },
                Need::Array(ArrayState::FirstOrEnd) if self.bytes.get(self.at) == Some(&b']') => {
                    self.at += 1;
                    if let Some(Frame::Array { values, .. }) = stack.pop() {
                        pending = Some(Value::Array(values));
                    }
                    continue;
                }
                Need::Object(ObjectState::CommaOrEnd) => match self.bytes.get(self.at) {
                    Some(b',') => {
                        self.at += 1;
                        if let Some(Frame::Object { state, .. }) = stack.last_mut() {
                            *state = ObjectState::Key;
                        }
                        continue;
                    }
                    Some(b'}') => {
                        self.at += 1;
                        if let Some(Frame::Object { values, .. }) = stack.pop() {
                            pending = Some(Value::Object(values));
                        }
                        continue;
                    }
                    _ => return Err(self.syntax("expected comma or end of JSON object")),
                },
                Need::Object(ObjectState::FirstKeyOrEnd)
                    if self.bytes.get(self.at) == Some(&b'}') =>
                {
                    self.at += 1;
                    if let Some(Frame::Object { values, .. }) = stack.pop() {
                        pending = Some(Value::Object(values));
                    }
                    continue;
                }
                Need::Object(ObjectState::FirstKeyOrEnd | ObjectState::Key) => {
                    if self.bytes.get(self.at) != Some(&b'"') {
                        return Err(self.syntax("expected JSON object string key"));
                    }
                    let name = self.string()?;
                    if let Some(Frame::Object { key, state, .. }) = stack.last_mut() {
                        *key = Some(name);
                        *state = ObjectState::Colon;
                    }
                    continue;
                }
                Need::Object(ObjectState::Colon) => {
                    if self.bytes.get(self.at) != Some(&b':') {
                        return Err(self.syntax("expected colon after JSON object key"));
                    }
                    self.at += 1;
                    if let Some(Frame::Object { state, .. }) = stack.last_mut() {
                        *state = ObjectState::Value;
                    }
                    continue;
                }
                Need::RootValue | Need::Array(_) | Need::Object(ObjectState::Value) => {}
            }
            match self.bytes.get(self.at) {
                Some(b'[') => {
                    self.at += 1;
                    stack.push(Frame::Array {
                        values: Vec::new(),
                        state: ArrayState::FirstOrEnd,
                    });
                }
                Some(b'{') => {
                    self.at += 1;
                    stack.push(Frame::Object {
                        values: Object::new(),
                        key: None,
                        state: ObjectState::FirstKeyOrEnd,
                    });
                }
                Some(b'"') => pending = Some(Value::String(self.string()?)),
                Some(b'n') => {
                    self.literal(b"null")?;
                    pending = Some(Value::Null);
                }
                Some(b't') => {
                    self.literal(b"true")?;
                    pending = Some(Value::Boolean(true));
                }
                Some(b'f') => {
                    self.literal(b"false")?;
                    pending = Some(Value::Boolean(false));
                }
                Some(b'-' | b'0'..=b'9') => pending = Some(self.number()?),
                _ => return Err(self.syntax("expected JSON value")),
            }
        }
    }

    fn literal(&mut self, text: &[u8]) -> Result<(), NativeExternalJsonError> {
        if self.bytes.get(self.at..self.at.saturating_add(text.len())) != Some(text) {
            return Err(self.syntax("invalid JSON literal"));
        }
        self.at += text.len();
        Ok(())
    }

    fn string(&mut self) -> Result<Vec<u8>, NativeExternalJsonError> {
        let start = self.at;
        self.at += 1;
        let mut escaped = false;
        loop {
            let Some(&byte) = self.bytes.get(self.at) else {
                return Err(self.error(
                    NativeExternalJsonErrorKind::InvalidString,
                    "unterminated JSON string",
                ));
            };
            if byte < 0x20 {
                return Err(self.error(
                    NativeExternalJsonErrorKind::InvalidString,
                    "unescaped control byte in JSON string",
                ));
            }
            self.at += 1;
            if !escaped && byte == b'"' {
                return serde_json::from_slice::<String>(&self.bytes[start..self.at])
                    .map(String::into_bytes)
                    .map_err(|error| NativeExternalJsonError {
                        byte_offset: start + error.column().saturating_sub(1),
                        kind: NativeExternalJsonErrorKind::InvalidString,
                        cause: "invalid JSON string escape, Unicode surrogate or UTF-8",
                    });
            }
            if escaped {
                escaped = false;
            } else if byte == b'\\' {
                escaped = true;
            }
        }
    }

    fn number(&mut self) -> Result<Value, NativeExternalJsonError> {
        let start = self.at;
        let negative = self.bytes.get(self.at) == Some(&b'-');
        if negative {
            self.at += 1;
        }
        match self.bytes.get(self.at) {
            Some(b'0') => self.at += 1,
            Some(b'1'..=b'9') => {
                self.at += 1;
                while self.bytes.get(self.at).is_some_and(u8::is_ascii_digit) {
                    self.at += 1;
                }
            }
            _ => return Err(self.syntax("expected digit in JSON number")),
        }
        let mut real = false;
        if self.bytes.get(self.at) == Some(&b'.') {
            real = true;
            self.at += 1;
            let first = self.at;
            while self.bytes.get(self.at).is_some_and(u8::is_ascii_digit) {
                self.at += 1;
            }
            if self.at == first {
                return Err(self.syntax("expected digit after JSON decimal point"));
            }
        }
        if matches!(self.bytes.get(self.at), Some(b'e' | b'E')) {
            real = true;
            self.at += 1;
            if matches!(self.bytes.get(self.at), Some(b'+' | b'-')) {
                self.at += 1;
            }
            let first = self.at;
            while self.bytes.get(self.at).is_some_and(u8::is_ascii_digit) {
                self.at += 1;
            }
            if self.at == first {
                return Err(self.syntax("expected digit in JSON exponent"));
            }
        }
        let token = &self.bytes[start..self.at];
        // The scanned grammar is ASCII; an invalid source byte is not coerced.
        let text =
            std::str::from_utf8(token).map_err(|_| self.syntax("invalid JSON number byte"))?;
        if !real {
            if negative {
                if let Ok(value) = text.parse::<i64>() {
                    return Ok(Value::Signed(value));
                }
            } else if let Ok(value) = text.parse::<u64>() {
                return Ok(Value::Unsigned(value));
            }
        }
        // Nlohmann falls from overflowing integer storage to a binary64 float.
        // Existing float_roundtrip is required; no InputProcessor wrap path.
        let number = serde_json::from_slice::<serde_json::Number>(token).map_err(|_| {
            NativeExternalJsonError {
                byte_offset: start,
                kind: NativeExternalJsonErrorKind::NumberOverflow,
                cause: "JSON number cannot be represented as finite Real64",
            }
        })?;
        let value = number.as_f64().ok_or_else(|| {
            self.error(
                NativeExternalJsonErrorKind::NumberOverflow,
                "JSON number cannot be represented as Real64",
            )
        })?;
        if !value.is_finite() {
            return Err(NativeExternalJsonError {
                byte_offset: start,
                kind: NativeExternalJsonErrorKind::NumberOverflow,
                cause: "JSON number overflow",
            });
        }
        Ok(Value::Real(value))
    }
}
