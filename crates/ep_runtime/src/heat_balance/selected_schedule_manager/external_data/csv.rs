//! Byte-for-byte token grammar of InputProcessing/CsvParser.cc, not RFC CSV.
use super::NativeExternalDataValue as Value;
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct NativeExternalDataDiagnostic {
    pub(crate) message: Vec<u8>,
    pub(crate) continued: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct NativeCsvUndefinedBoundary {
    pub(crate) byte_offset: usize,
    pub(crate) physical_line: usize,
    pub(crate) cause: &'static str,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct NativeCsvDecode {
    pub(crate) value: Value,
    pub(crate) errors: Vec<NativeExternalDataDiagnostic>,
    pub(crate) warnings: Vec<NativeExternalDataDiagnostic>,
    /// A genuine Native uninitialized read is not a fabricated parser error.
    pub(crate) native_undefined_boundary: Option<NativeCsvUndefinedBoundary>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Token {
    End,
    Delimiter,
    LineEnd,
    Value,
}

#[derive(Clone, Copy)]
struct Cursor {
    index: usize,
    line: usize,
    line_index: usize,
    line_start: usize,
}

struct Parser<'a> {
    bytes: &'a [u8],
    delimiter: u8,
    cursor: Cursor,
    success: bool,
    errors: Vec<NativeExternalDataDiagnostic>,
    warnings: Vec<NativeExternalDataDiagnostic>,
    undefined: Option<NativeCsvUndefinedBoundary>,
}

/// One fresh Native CsvParser::decode, including partial results and diagnostics.
pub(crate) fn decode_native_csv(bytes: &[u8], delimiter: u8, rows_to_skip: i32) -> NativeCsvDecode {
    let mut parser = Parser {
        bytes,
        delimiter,
        cursor: Cursor {
            index: 0,
            line: 1,
            line_index: 0,
            line_start: 0,
        },
        success: true,
        errors: Vec::new(),
        warnings: Vec::new(),
        undefined: None,
    };
    let value = if bytes.is_empty() {
        parser.error(b"CSV File is empty".to_vec(), false);
        Value::Null
    } else {
        parser.parse_csv(rows_to_skip)
    };
    NativeCsvDecode {
        value,
        errors: parser.errors,
        warnings: parser.warnings,
        native_undefined_boundary: parser.undefined,
    }
}

impl Parser<'_> {
    fn error(&mut self, message: Vec<u8>, continued: bool) {
        self.errors
            .push(NativeExternalDataDiagnostic { message, continued });
    }

    fn warning(&mut self, message: Vec<u8>, continued: bool) {
        self.warnings
            .push(NativeExternalDataDiagnostic { message, continued });
    }

    fn advance(&mut self) {
        self.cursor.index += 1;
        self.cursor.line_index += 1;
    }

    fn eat_whitespace(&mut self) {
        while let Some(&byte) = self.bytes.get(self.cursor.index) {
            if (self.delimiter != b' ' && byte == b' ')
                || (self.delimiter != b'\t' && byte == b'\t')
                || byte == b'\r'
            {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn next_token(&mut self) -> Token {
        self.eat_whitespace();
        let Some(&byte) = self.bytes.get(self.cursor.index) else {
            return Token::End;
        };
        if byte == self.delimiter {
            self.advance();
            Token::Delimiter
        } else if byte == b'\n' {
            self.cursor.index += 1;
            self.cursor.line += 1;
            self.cursor.line_start = self.cursor.index;
            self.cursor.line_index = 0;
            Token::LineEnd
        } else {
            self.advance();
            Token::Value
        }
    }

    fn look_ahead(&mut self) -> Token {
        let saved = self.cursor;
        let token = self.next_token();
        self.cursor = saved;
        token
    }

    fn skip_rows(&mut self, rows: i32) {
        let mut skipped = 0;
        loop {
            match self.next_token() {
                Token::End => break,
                Token::LineEnd => {
                    skipped += 1;
                    if skipped == rows {
                        break;
                    }
                }
                _ => {}
            }
        }
    }

    fn find_number_columns(&mut self) -> Option<usize> {
        let saved = self.cursor;
        let mut count = 0;
        let mut previous = None;
        loop {
            let token = self.next_token();
            match token {
                Token::End => break,
                Token::Delimiter => count += 1,
                Token::LineEnd => {
                    let Some(previous) = previous else {
                        self.undefined = Some(NativeCsvUndefinedBoundary {
                            byte_offset: saved.index,
                            physical_line: saved.line,
                            cause: "CsvParser::find_number_columns reads uninitialized prev_token on a blank first data row",
                        });
                        self.cursor = saved;
                        return None;
                    };
                    if previous != Token::Delimiter {
                        count += 1;
                    }
                    break;
                }
                Token::Value => {}
            }
            previous = Some(token);
        }
        self.cursor = saved;
        Some(count)
    }

    fn parse_csv(&mut self, rows_to_skip: i32) -> Value {
        let mut header = Vec::new();
        let mut columns: Vec<Vec<Value>> = Vec::new();
        if self.bytes.len() > 3 && self.bytes.starts_with(b"\xef\xbb\xbf") {
            self.cursor.index += 3;
            self.cursor.line_index += 3;
        }
        if rows_to_skip > 1 {
            self.skip_rows(rows_to_skip);
        }
        let mut first = true;
        while self.cursor.index != self.bytes.len() {
            if first {
                if rows_to_skip == 1 {
                    header = self.parse_header();
                }
                let Some(count) = self.find_number_columns() else {
                    break;
                };
                columns = (0..count).map(|_| Vec::new()).collect();
                first = false;
                continue;
            }
            self.parse_line(&mut columns);
            if !self.success {
                break;
            }
        }
        Value::Object(BTreeMap::from([
            (b"header".to_vec(), Value::Array(header)),
            (
                b"values".to_vec(),
                Value::Array(columns.into_iter().map(Value::Array).collect()),
            ),
        ]))
    }

    fn parse_header(&mut self) -> Vec<Value> {
        let mut values = Vec::new();
        loop {
            match self.look_ahead() {
                Token::End | Token::LineEnd => {
                    self.next_token();
                    return values;
                }
                Token::Delimiter => {
                    self.next_token();
                }
                Token::Value => values.push(self.parse_value()),
            }
        }
    }

    fn physical_line(&self, start: usize) -> (&[u8], Option<usize>) {
        match self.bytes[start..]
            .iter()
            .position(|b| matches!(b, b'\r' | b'\n'))
        {
            Some(length) => (&self.bytes[start..start + length], Some(start + length)),
            // Native getCurrentLine returns an empty view without a terminator.
            None => (&[], None),
        }
    }

    fn parse_line(&mut self, columns: &mut [Vec<Value>]) {
        let mut column = 0;
        let mut parsed = 0;
        let mut extra = false;
        let this_line = self.cursor.line;
        let this_start = self.cursor.line_start;
        loop {
            let token = self.look_ahead();
            if matches!(token, Token::End | Token::LineEnd) {
                let (line, end) = self.physical_line(this_start);
                let line = line.to_vec();
                if extra {
                    self.warning(format!("CsvParser - Line {this_line} - Expected {} columns, got {parsed}. Ignored extra columns. Error in following line.", columns.len()).into_bytes(), false);
                    self.warning(line, true);
                } else if parsed != columns.len() {
                    let last = token == Token::End
                        || end.is_some_and(|at| {
                            at + 1 == self.bytes.len() || at + 2 == self.bytes.len()
                        });
                    if !line.is_empty() || !last {
                        self.success = false;
                        self.error(format!("CsvParser - Line {this_line} - Expected {} columns, got {parsed}. Error in following line.", columns.len()).into_bytes(), false);
                        self.error(line, true);
                    }
                }
                self.next_token();
                return;
            }
            if token == Token::Delimiter {
                self.next_token();
                if self.look_ahead() == Token::Delimiter {
                    let next_column = column + 1;
                    if next_column < columns.len() {
                        columns[next_column].push(Value::Null);
                        let line = self.physical_line(this_start).0.to_vec();
                        self.warning(format!("CsvParser - Line {this_line} Column {} - Blank value found, setting to null. Error in following line.", next_column + 1).into_bytes(), false);
                        self.warning(line, true);
                    } else {
                        extra = true;
                    }
                    parsed += 1;
                }
                column += 1;
            } else {
                let value = self.parse_value();
                if column < columns.len() {
                    columns[column].push(value);
                } else {
                    extra = true;
                }
                parsed += 1;
            }
        }
    }

    fn parse_value(&mut self) -> Value {
        self.eat_whitespace();
        let start = self.cursor.index;
        while let Some(&byte) = self.bytes.get(self.cursor.index) {
            if byte == self.delimiter || matches!(byte, b'\r' | b'\n') {
                break;
            }
            self.advance();
        }
        let value = &self.bytes[start..self.cursor.index];
        let plus = usize::from(value.first() == Some(&b'+'));
        if let Some((number, consumed)) = native_float_prefix(&value[plus..]) {
            let consumed = consumed + plus;
            if consumed == value.len() {
                return Value::Real(number);
            }
            if self.delimiter != b' ' && value[consumed..].iter().all(|b| *b == b' ') {
                self.cursor.index -= value.len() - consumed;
                self.cursor.line_index -= value.len() - consumed;
                return Value::Real(number);
            }
        }
        let end = value
            .iter()
            .rposition(|b| !matches!(b, b' ' | b'\t'))
            .map_or(0, |i| i + 1);
        Value::String(value[..end].to_vec())
    }
}

/// Pinned fast_float general-format prefix grammar. Its 26.1 header returns
/// success for decimal overflow/underflow; preserve Inf/zero rather than strings.
fn native_float_prefix(bytes: &[u8]) -> Option<(f64, usize)> {
    let negative = bytes.first() == Some(&b'-');
    let mut at = usize::from(negative);
    let integer_start = at;
    while bytes.get(at).is_some_and(u8::is_ascii_digit) {
        at += 1;
    }
    let mut digits = at - integer_start;
    if bytes.get(at) == Some(&b'.') {
        at += 1;
        let start = at;
        while bytes.get(at).is_some_and(u8::is_ascii_digit) {
            at += 1;
        }
        digits += at - start;
    }
    if digits > 0 {
        if matches!(bytes.get(at), Some(b'e' | b'E')) {
            let before_exponent = at;
            at += 1;
            if matches!(bytes.get(at), Some(b'+' | b'-')) {
                at += 1;
            }
            let start = at;
            while bytes.get(at).is_some_and(u8::is_ascii_digit) {
                at += 1;
            }
            if start == at {
                at = before_exponent;
            }
        }
        let text = std::str::from_utf8(&bytes[..at]).ok()?;
        return text.parse::<f64>().ok().map(|value| (value, at));
    }
    let base = usize::from(negative);
    let remaining = bytes.get(base..)?;
    if remaining
        .get(..3)
        .is_some_and(|s| s.eq_ignore_ascii_case(b"nan"))
    {
        let mut end = base + 3;
        if bytes.get(end) == Some(&b'(') {
            let mut payload = end + 1;
            while let Some(&byte) = bytes.get(payload) {
                if byte == b')' {
                    end = payload + 1;
                    break;
                }
                if !byte.is_ascii_alphanumeric() && byte != b'_' {
                    break;
                }
                payload += 1;
            }
        }
        return Some((if negative { -f64::NAN } else { f64::NAN }, end));
    }
    if remaining
        .get(..3)
        .is_some_and(|s| s.eq_ignore_ascii_case(b"inf"))
    {
        let length = if remaining
            .get(..8)
            .is_some_and(|s| s.eq_ignore_ascii_case(b"infinity"))
        {
            8
        } else {
            3
        };
        return Some((
            if negative {
                f64::NEG_INFINITY
            } else {
                f64::INFINITY
            },
            base + length,
        ));
    }
    None
}
