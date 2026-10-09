//! InputFile binary getline semantics; native iostate integer bits remain unpaired.
use super::dates::{number_with_error, process_date_string};
use super::header_state::{
    DateType, HeaderContextConsumption, RawEpwHeaderError, source_count, strip_ascii_spaces,
    to_source_int,
};

/// A supplied-byte EPW stream retaining its actual cursor across header and records.
#[derive(Clone, Debug)]
pub struct RawEpwInput {
    bytes: Vec<u8>,
    /// Byte cursor, including consumed LF bytes.
    pub byte_cursor: usize,
    /// Whether this supplied-byte stream is open.
    pub opened: bool,
    /// End-of-file flag, separate from a read's successful data extraction.
    pub eof: bool,
    /// Failed-extraction flag.
    pub failed: bool,
    /// Bad-stream flag; a closed InputFile reports badbit.
    pub bad: bool,
    /// Number of attempted line reads for consumption diagnostics.
    pub line_reads: usize,
}

/// Semantic stream state; no synthetic native rdstate bits or error wording.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RawEpwStreamState {
    /// Open-state observation.
    pub is_open: bool,
    /// All stream flags are clear, as InputFile::good actually implements.
    pub good: bool,
    /// EOF-state observation.
    pub eof: bool,
    /// Failed-extraction-state observation.
    pub fail: bool,
    /// Bad-stream-state observation.
    pub bad: bool,
    /// Actual byte position only for an open stream with all flags clear.
    pub position_byte: Option<i64>,
}

/// One getline result; `read_good` can be true while the stream has eofbit.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RawEpwLineRead {
    /// Extracted UTF-8 bytes, with one terminal CR removed on successful reads.
    pub data: String,
    /// EOF at this extraction.
    pub eof: bool,
    /// Successful extraction, even if no final LF was present.
    pub read_good: bool,
    /// Actual byte cursor before the call.
    pub start_byte: usize,
    /// Actual byte cursor after the call.
    pub end_byte: usize,
}

impl RawEpwLineRead {
    /// Source ReadResult::update retains the old data on failed extraction.
    /// Offsets describe the actual latest attempted read, including failures.
    pub fn update(&mut self, next: Self) {
        self.eof = next.eof;
        self.read_good = next.read_good;
        self.start_byte = next.start_byte;
        self.end_byte = next.end_byte;
        if next.read_good {
            self.data = next.data;
        }
    }
}

impl RawEpwInput {
    /// Construct the closed supplied-byte owner used before explicit opening.
    pub fn new_unopened(bytes: Vec<u8>) -> Self {
        Self {
            bytes,
            byte_cursor: 0,
            opened: false,
            eof: false,
            failed: false,
            bad: true,
            line_reads: 0,
        }
    }

    /// Open from byte zero and clear stream flags, retaining the supplied bytes.
    pub fn reopen(&mut self) {
        self.opened = true;
        self.byte_cursor = 0;
        self.eof = false;
        self.failed = false;
        self.bad = false;
    }

    /// InputFile::close releases the stream; closed rdstate reports badbit.
    /// Supplied bytes and Rust read-attempt bookkeeping remain available for a
    /// later open, but no native closed-stream position is claimed.
    pub fn close(&mut self) {
        self.opened = false;
        self.eof = false;
        self.failed = false;
        self.bad = true;
    }

    /// InputFile::rewind clears flags and seeks to zero on an open owner only.
    /// It retains the actual line-attempt count and does not reopen the stream.
    pub fn rewind(&mut self) {
        if self.opened {
            self.eof = false;
            self.failed = false;
            self.bad = false;
            self.byte_cursor = 0;
        }
    }

    /// Literal InputFile::backspace scan, including its cursor-one edge case.
    /// This is a byte operation, rather than a saved previous-line position.
    pub fn backspace(&mut self) -> Result<(), RawEpwHeaderError> {
        if !self.opened {
            return Ok(());
        }
        self.eof = false;
        self.failed = false;
        self.bad = false;
        if self.byte_cursor > self.bytes.len() {
            return Err(RawEpwHeaderError::OutsideBoundedDomain(
                "backspace cursor outside supplied stream",
            ));
        }
        let mut scan = self.byte_cursor.saturating_sub(1);
        while scan > 0 {
            scan -= 1;
            self.byte_cursor = scan;
            if self.bytes[scan] == b'\n' {
                self.byte_cursor = scan + 1;
                break;
            }
        }
        Ok(())
    }

    /// Observe cursor availability without altering flags through a failed tellg.
    pub fn snapshot(&self) -> RawEpwStreamState {
        let good = self.opened && !self.eof && !self.failed && !self.bad;
        RawEpwStreamState {
            is_open: self.opened,
            good,
            eof: self.eof,
            fail: self.failed,
            bad: self.bad,
            position_byte: if good {
                i64::try_from(self.byte_cursor).ok()
            } else {
                None
            },
        }
    }

    /// Perform the selected InputFile::readLine operation on supplied UTF-8 bytes.
    pub fn read_line(&mut self) -> Result<RawEpwLineRead, RawEpwHeaderError> {
        self.line_reads += 1;
        let start = self.byte_cursor;
        let empty = |eof| RawEpwLineRead {
            data: String::new(),
            eof,
            read_good: false,
            start_byte: start,
            end_byte: start,
        };
        if !self.opened {
            return Ok(empty(true));
        }
        if start > self.bytes.len() {
            return Err(RawEpwHeaderError::OutsideBoundedDomain(
                "byte cursor outside supplied stream",
            ));
        }
        if self.bad || self.failed || self.eof || start == self.bytes.len() {
            self.failed = true;
            self.eof = self.eof || start == self.bytes.len();
            return Ok(empty(self.eof));
        }
        let delimiter = self.bytes[start..].iter().position(|byte| *byte == b'\n');
        let end = delimiter.map_or(self.bytes.len(), |offset| start + offset);
        self.byte_cursor = delimiter.map_or(end, |_| end + 1);
        self.eof = delimiter.is_none();
        let mut raw = self.bytes[start..end].to_vec();
        if raw.last() == Some(&b'\r') {
            raw.pop();
        }
        let data = String::from_utf8(raw)
            .map_err(|_| RawEpwHeaderError::OutsideBoundedDomain("non-UTF8 byte line"))?;
        Ok(RawEpwLineRead {
            data,
            eof: self.eof,
            read_good: true,
            start_byte: start,
            end_byte: self.byte_cursor,
        })
    }

    pub(super) fn field_end(&mut self, line: &mut String) -> Result<usize, RawEpwHeaderError> {
        strip_ascii_spaces(line);
        if let Some(at) = line.find(',') {
            return Ok(at);
        }
        if !line.is_empty() {
            return Ok(line.len());
        }
        loop {
            let next = self.read_line()?;
            *line = next.data;
            strip_ascii_spaces(line);
            line.make_ascii_uppercase();
            if let Some(at) = line.find(',') {
                return Ok(at);
            }
            if next.eof {
                return Err(RawEpwHeaderError::OutsideBoundedDomain(
                    "source continuation loop has no comma before EOF",
                ));
            }
        }
    }
}

pub(super) fn erase_field(line: &mut String, end: usize) {
    line.drain(..(end + 1).min(line.len()));
}

impl HeaderContextConsumption {
    pub(super) fn consume_typical(
        &mut self,
        line: &mut String,
        input: &mut RawEpwInput,
        errors: &mut bool,
    ) -> Result<(), RawEpwHeaderError> {
        strip_ascii_spaces(line);
        let mut end = line.find(',');
        if end.is_none() && line.is_empty() {
            while end.is_none() && line.is_empty() {
                let read = input.read_line()?;
                *line = read.data;
                strip_ascii_spaces(line);
                end = line.find(',');
                if read.eof && line.is_empty() {
                    return Err(RawEpwHeaderError::OutsideBoundedDomain(
                        "typical continuation EOF loop",
                    ));
                }
            }
        } else if end.is_none() {
            end = Some(line.len());
        }
        let at = end.map_or(line.len(), |position| position);
        let count = to_source_int(number_with_error(&line[..at]).0)?;
        self.typical_declared_count = Some(count);
        // In the no-comma continuation case, npos+1 wraps to zero in C++.
        if let Some(at) = end {
            erase_field(line, at);
        }
        'periods: for index in 1..=source_count(count)? {
            strip_ascii_spaces(line);
            for part in 0..4 {
                if let Some(at) = line.find(',') {
                    let token = line[..at].to_owned();
                    if part >= 2 {
                        let date = process_date_string(&token.to_ascii_uppercase(), false, errors)?;
                        if date.date_type == DateType::Invalid {
                            *errors = true;
                        }
                    }
                    self.typical_consumed_fields.push(token);
                    erase_field(line, at);
                } else if part <= 1 {
                    self.typical_declared_count = Some(index as i32 - 1);
                    break 'periods;
                } else if part == 3 {
                    let date = process_date_string(&line.to_ascii_uppercase(), false, errors)?;
                    if date.date_type == DateType::Invalid {
                        *errors = true;
                    }
                    self.typical_consumed_fields.push(line.clone());
                }
            }
        }
        self.unimplemented_computed_state
            .push("TypicalExtremePeriod title/short/match/date totals");
        Ok(())
    }

    pub(super) fn consume_ground(&mut self, line: &mut String) -> Result<(), RawEpwHeaderError> {
        if let Some(at) = line.find(',') {
            let (number, error) = number_with_error(&line[..at]);
            let count = to_source_int(number)?;
            if !error && count >= 1 {
                erase_field(line, at);
                for _ in 1..=4 {
                    if let Some(at) = line.find(',') {
                        self.ground_consumed_fields.push(line[..at].to_owned());
                        erase_field(line, at);
                    } else {
                        line.clear();
                        break;
                    }
                }
                for _ in 1..=12 {
                    if let Some(at) = line.find(',') {
                        number_with_error(&line[..at]);
                        self.ground_consumed_fields.push(line[..at].to_owned());
                        erase_field(line, at);
                    } else {
                        if !line.is_empty() {
                            number_with_error(line);
                            self.ground_consumed_fields.push(line.clone());
                        }
                        break;
                    }
                }
            }
        }
        self.unimplemented_computed_state
            .push("GroundTempsFCFromEPWHeader and wthFCGroundTemps");
        Ok(())
    }
}
