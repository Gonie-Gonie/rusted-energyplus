//! Genuine OpenEPlusWeatherFile and SkipEPlusWFHeader on one physical input.
use super::dates::number_with_error;
use super::header::HEADERS;
use super::header_state::to_source_int;
use super::input::erase_field;
use super::{RawDayFailure, RawEpwCursorOwner, RawEpwHeaderError};
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::heat_balance::selected_input_processor::SelectedInputProcessor;
use crate::heat_balance::source_output_registration::SourceOutputReportStreams;
use crate::weather::day::SourceWeatherInitializationError as Failure;
use std::io::{self, Write};
use std::path::Path;

struct EsoOutput<'a>(&'a mut dyn SourceOutputReportStreams);
impl Write for EsoOutput<'_> {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.0.write_eso(bytes)?;
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
fn raw(error: RawEpwHeaderError) -> Failure {
    RawDayFailure::Header(error).into()
}
fn fatal_to_eso(
    backend: &mut SourceErrorBackend<'_>,
    files: &mut dyn SourceOutputReportStreams,
    text: &str,
) -> Failure {
    match backend.show_fatal_error_to_output(text, &mut EsoOutput(files)) {
        Ok(never) => match never {},
        Err(error) => Error::Fatal(error).into(),
    }
}
fn fatal(backend: &mut SourceErrorBackend<'_>, text: &str) -> Failure {
    match backend.show_fatal_error(text) {
        Ok(never) => match never {},
        Err(error) => Error::Fatal(error).into(),
    }
}

impl RawEpwCursorOwner {
    /// Actual4371–4413 with retained header/parser/arguments and the real input path.
    pub(crate) fn open_source_epw_file(
        &mut self,
        path: &Path,
        process_header: bool,
        errors: &mut bool,
        processor: &mut SelectedInputProcessor,
        files: &mut dyn SourceOutputReportStreams,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Failure> {
        self.input.close();
        self.input.source_file_path = path.to_path_buf();
        self.input.open_source_stream().map_err(Error::Io)?;
        if !self.input.snapshot().good {
            return Err(fatal_to_eso(
                backend,
                files,
                "OpenWeatherFile: Could not OPEN EPW Weather File",
            ));
        }
        let result = if process_header {
            self.process_source_headers(errors, processor, files, backend)
        } else {
            self.skip_source_header(files, backend)
        };
        // This is the same retained Raw parser association and diagnostic stream
        // observation as the legacy caller, including preceding writes on failure.
        self.parser.end_day_of_month = self.header.month_ends;
        self.parser.weather_code_missed_count = self.header.weather_code_missed_count;
        self.stream_after_header = self.input.snapshot();
        result
    }

    fn process_source_headers(
        &mut self,
        errors: &mut bool,
        processor: &mut SelectedInputProcessor,
        files: &mut dyn SourceOutputReportStreams,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Failure> {
        for (kind, expected) in HEADERS {
            let read = self.input.read_line().map_err(raw)?;
            if read.eof {
                return Err(fatal_to_eso(
                    backend,
                    files,
                    &format!(
                        "OpenWeatherFile: Unexpected End-of-File on EPW Weather file, while reading header information, looking for header={expected}"
                    ),
                ));
            }
            if read.data.as_bytes().last() == Some(&0) {
                backend.show_severe_error_to_output(
                    "OpenWeatherFile: EPW Weather File appears to be a Unicode or binary file.",
                    &mut EsoOutput(files),
                )?;
                backend.show_continue_error("...This file cannot be read by this program. Please save as PC or Unix file and try again")?;
                return Err(fatal(
                    backend,
                    "Program terminates due to previous condition.",
                ));
            }
            let first = read.data.as_bytes().iter().position(|byte| *byte != b' ');
            if first != read.data.find(expected) {
                continue;
            }
            let mut line = read.data;
            super::source_header::process_source_epw_header(
                kind,
                &mut line,
                errors,
                &mut self.header,
                &mut self.input,
                processor,
                &mut self.header_context,
                backend,
            )?;
        }
        Ok(())
    }

    fn skip_source_header(
        &mut self,
        files: &mut dyn SourceOutputReportStreams,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Failure> {
        let mut read;
        loop {
            read = self.input.read_line().map_err(raw)?;
            if read.eof {
                return Err(fatal_to_eso(
                    backend,
                    files,
                    "Unexpected End-of-File on EPW Weather file, while reading header information, looking for header=DATA PERIODS",
                ));
            }
            if !read.read_good {
                return Err(
                    Error::Unavailable("Native non-progressing SkipEPlusWFHeader stream").into(),
                );
            }
            read.data.make_ascii_uppercase();
            if read.data.contains("DATA PERIODS") {
                break;
            }
        }
        let mut arguments = 2_i32;
        let mut current = 0_i32;
        let mut field = 1_i32;
        while field <= arguments {
            let end = self.input.field_end(&mut read.data).map_err(raw)?;
            if field == 1 {
                let periods = to_source_int(number_with_error(&read.data[..end]).0).map_err(raw)?;
                arguments = periods
                    .checked_mul(4)
                    .and_then(|v| v.checked_add(arguments))
                    .ok_or(Error::Unavailable(
                        "Native SkipEPlusWFHeader NumHdArgs arithmetic8247",
                    ))?;
            } else if field >= 3 && (field - 3) % 4 == 0 {
                current = current.checked_add(1).ok_or(Error::Unavailable(
                    "Native SkipEPlusWFHeader CurCount increment8250",
                ))?;
            }
            erase_field(&mut read.data, end);
            field = field
                .checked_add(1)
                .ok_or(Error::Unavailable("Native SkipEPlusWFHeader i increment"))?;
        }
        let _ = current; // Native's actual local writes have no subsequent consumer.
        Ok(())
    }
}
