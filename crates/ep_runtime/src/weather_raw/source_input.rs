//! Actual InputFile binary stream on the same raw parser/cursor owner.
use super::{RawEpwHeaderError, RawEpwInput, RawEpwLineRead, SourcePhysicalInputStream};
use std::io::{Read, Seek, SeekFrom};

impl RawEpwInput {
    /// IOFiles.cc143–149: file_size before creating the binary fstream.
    /// The caller has already performed close and actual filePath assignment.
    pub(crate) fn open_source_stream(&mut self) -> std::io::Result<()> {
        self.source_file_size = std::fs::metadata(&self.source_file_path)?.len();
        match std::fs::File::open(&self.source_file_path) {
            Ok(stream) => {
                self.source_stream = Some(SourcePhysicalInputStream {
                    handle: Some(std::io::BufReader::new(stream)),
                });
                self.byte_cursor = 0;
                self.opened = true;
                self.eof = false;
                self.failed = false;
                self.bad = false;
            }
            Err(_) => {
                // Native creates a fstream whose open failure is observed by good().
                self.source_stream = Some(SourcePhysicalInputStream { handle: None });
                // Actual is_open dynamic_casts ifstream; this fstream stays nonnull.
                self.opened = true;
                self.eof = false;
                self.failed = true;
                self.bad = false;
            }
        }
        Ok(())
    }

    /// Actual binary getline, retaining the same handle rather than a byte reload.
    pub(super) fn read_source_stream_line(
        &mut self,
        start: usize,
    ) -> Result<RawEpwLineRead, RawEpwHeaderError> {
        let empty = |eof| RawEpwLineRead {
            data: String::new(),
            eof,
            read_good: false,
            start_byte: start,
            end_byte: start,
        };
        if self.bad || self.failed || self.eof {
            self.failed = true;
            return Ok(empty(self.eof));
        }
        let stream = self
            .source_stream
            .as_mut()
            .and_then(|source| source.handle.as_mut())
            .ok_or(RawEpwHeaderError::OutsideBoundedDomain(
                "actual opened Source InputFile stream",
            ))?;
        let mut line = Vec::new();
        loop {
            let mut byte = [0_u8; 1];
            match stream.read(&mut byte) {
                Ok(0) => {
                    self.eof = true;
                    if line.is_empty() {
                        self.failed = true;
                        return Ok(empty(true));
                    }
                    break;
                }
                Ok(1) => {
                    self.byte_cursor = self.byte_cursor.checked_add(1).ok_or(
                        RawEpwHeaderError::OutsideBoundedDomain("Source byte cursor overflow"),
                    )?;
                    if byte[0] == b'\n' {
                        break;
                    }
                    line.push(byte[0]);
                }
                Ok(_) => unreachable!("one-byte Read buffer"),
                Err(_) => {
                    self.bad = true;
                    self.failed = true;
                    return Err(RawEpwHeaderError::OutsideBoundedDomain(
                        "native physical InputFile read error/exception state",
                    ));
                }
            }
        }
        if line.last() == Some(&b'\r') {
            line.pop();
        }
        let data = String::from_utf8(line).map_err(|_| {
            RawEpwHeaderError::OutsideBoundedDomain("non-UTF8 Source physical byte line")
        })?;
        Ok(RawEpwLineRead {
            data,
            eof: self.eof,
            read_good: true,
            start_byte: start,
            end_byte: self.byte_cursor,
        })
    }

    pub(super) fn rewind_source_stream(&mut self) {
        if let Some(source) = self.source_stream.as_mut() {
            self.eof = false;
            self.failed = false;
            self.bad = false;
            if let Some(stream) = source.handle.as_mut() {
                match stream.seek(SeekFrom::Start(0)) {
                    Ok(_) => self.byte_cursor = 0,
                    Err(_) => self.failed = true,
                }
            } else {
                self.failed = true;
            }
        }
    }

    /// Same byte scan as InputFile::backspace, using the actual physical cursor.
    pub(super) fn backspace_source_stream(&mut self) -> Result<(), RawEpwHeaderError> {
        self.eof = false;
        self.failed = false;
        self.bad = false;
        let stream = self
            .source_stream
            .as_mut()
            .and_then(|source| source.handle.as_mut())
            .ok_or(RawEpwHeaderError::OutsideBoundedDomain(
                "actual Source backspace stream",
            ))?;
        let mut scan = self.byte_cursor.saturating_sub(1);
        while scan > 0 {
            scan -= 1;
            let offset = u64::try_from(scan).map_err(|_| {
                RawEpwHeaderError::OutsideBoundedDomain("actual Source backspace offset")
            })?;
            stream.seek(SeekFrom::Start(offset)).map_err(|_| {
                RawEpwHeaderError::OutsideBoundedDomain("native Source backspace seek failure")
            })?;
            self.byte_cursor = scan;
            let mut byte = [0_u8; 1];
            if stream.read(&mut byte).map_err(|_| {
                RawEpwHeaderError::OutsideBoundedDomain("native Source backspace read failure")
            })? != 1
            {
                return Err(RawEpwHeaderError::OutsideBoundedDomain(
                    "native Source backspace failed extraction",
                ));
            }
            // Native get followed by seekg(-1,cur) retains the scanned position.
            stream.seek(SeekFrom::Start(offset)).map_err(|_| {
                RawEpwHeaderError::OutsideBoundedDomain(
                    "native Source backspace second seek failure",
                )
            })?;
            if byte[0] == b'\n' {
                self.byte_cursor = scan + 1;
                stream.seek(SeekFrom::Start(offset + 1)).map_err(|_| {
                    RawEpwHeaderError::OutsideBoundedDomain(
                        "native Source backspace newline seek failure",
                    )
                })?;
                break;
            }
        }
        Ok(())
    }
}
