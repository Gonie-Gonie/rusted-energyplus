//! Live header, raw-parser and byte-cursor owner for source-order day reads.

use super::day_read::{
    RawDayFailure, RawDayReadSelection, RawDayReadState, RawDaySourceFatal, RawReadProvenance,
    RawWeatherSlot,
};
use super::header_state::{strip_ascii_spaces, to_source_int};
use super::input::erase_field;
use super::lexical::{process_number, read_int};
use super::{
    HeaderCallerPreparation, HeaderContextConsumption, RawEpwHeaderState, RawEpwInput,
    RawEpwLineRead, RawEpwOutputs, RawEpwParserState, RawEpwStreamState,
    interpret_weather_data_line, open_epw_header,
};

/// A live stream. Selected day values are produced only by actual reads.
#[derive(Clone, Debug)]
pub struct RawEpwCursorOwner {
    pub(super) input: RawEpwInput,
    pub(super) header: RawEpwHeaderState,
    parser: RawEpwParserState,
    /// Explicit Rust reference-argument storage, not a native constructor claim.
    pub(super) arguments: RawEpwOutputs,
    header_errors: bool,
    header_context: HeaderContextConsumption,
    stream_after_header: RawEpwStreamState,
    interpret_count: usize,
}

impl RawEpwCursorOwner {
    /// Open the actual header from supplied bytes, preserving caller preparation.
    /// Nonfatal header ErrorsFound remains observable rather than being recast.
    pub fn open_from_bytes(
        bytes: Vec<u8>,
        mut header: RawEpwHeaderState,
        preparation: HeaderCallerPreparation,
    ) -> Result<Self, RawDayFailure> {
        let mut input = RawEpwInput::new_unopened(bytes);
        let mut header_errors = false;
        let mut context = HeaderContextConsumption::default();
        open_epw_header(
            &mut input,
            &mut header,
            &mut header_errors,
            preparation,
            &mut context,
        )?;
        Ok(Self::from_open_input(input, header, header_errors, context))
    }

    /// Attach an already opened, actual header/parser stream without seeking.
    pub fn from_open_input(
        input: RawEpwInput,
        header: RawEpwHeaderState,
        header_errors: bool,
        context: HeaderContextConsumption,
    ) -> Self {
        let parser = RawEpwParserState {
            end_day_of_month: header.month_ends,
            weather_code_missed_count: header.weather_code_missed_count,
        };
        let stream_after_header = input.snapshot();
        Self {
            input,
            header,
            parser,
            arguments: RawEpwOutputs::default(),
            header_errors,
            header_context: context,
            stream_after_header,
            interpret_count: 0,
        }
    }

    /// Actual header state, including the shared parser miss counter.
    pub fn header(&self) -> &RawEpwHeaderState {
        &self.header
    }
    /// Nonfatal ErrorsFound returned by the actual opening header operation.
    pub fn header_errors(&self) -> bool {
        self.header_errors
    }
    /// Actual header token consumption and explicitly unowned computed fields.
    pub fn header_context(&self) -> &HeaderContextConsumption {
        &self.header_context
    }
    /// The authoritative parser state used for every actual Interpret call.
    pub fn parser_state(&self) -> &RawEpwParserState {
        &self.parser
    }
    /// Current stream flags and position availability.
    pub fn stream_state(&self) -> RawEpwStreamState {
        self.input.snapshot()
    }
    /// Stream observation saved immediately after the original header operation.
    pub fn stream_after_header(&self) -> RawEpwStreamState {
        self.stream_after_header
    }
    /// Actual supplied-byte cursor, also retained when flags hide tellg position.
    pub fn byte_cursor(&self) -> usize {
        self.input.byte_cursor
    }
    /// Actual cumulative getline attempts, not a row number inferred from bytes.
    pub fn line_reads(&self) -> usize {
        self.input.line_reads
    }
    /// Actual Rust raw-parser invocation count, not a native record-index field.
    pub fn interpret_count(&self) -> usize {
        self.interpret_count
    }
    /// Last actual reference argument writes, including writes before a failure.
    pub fn last_arguments(&self) -> &RawEpwOutputs {
        &self.arguments
    }

    /// Apply the genuine caller's count reset to the same parser/header owner.
    pub fn set_weather_code_missed_count(&mut self, value: i32) {
        self.parser.weather_code_missed_count = value;
        self.header.weather_code_missed_count = value;
    }

    /// Apply a caller calendar write to the parser's shared month-end table.
    pub fn set_month_ends(&mut self, value: [i32; 12]) {
        self.parser.end_day_of_month = value;
        self.header.month_ends = value;
    }

    /// Close the actual input stream while preserving all weather owners.
    pub fn close(&mut self) {
        self.input.close();
    }

    /// OpenEPlusWeatherFile(ProcessHeader=true) on the same supplied bytes.
    /// Existing header fields remain until the real destructive parser writes
    /// them. No reference arguments or weather-code counter are reset.
    pub fn reopen_header(
        &mut self,
        preparation: HeaderCallerPreparation,
    ) -> Result<(), RawDayFailure> {
        self.input.close();
        let result = open_epw_header(
            &mut self.input,
            &mut self.header,
            &mut self.header_errors,
            preparation,
            &mut self.header_context,
        );
        self.parser.end_day_of_month = self.header.month_ends;
        self.parser.weather_code_missed_count = self.header.weather_code_missed_count;
        self.stream_after_header = self.input.snapshot();
        result.map_err(RawDayFailure::Header)
    }

    /// OpenEPlusWeatherFile(ProcessHeader=false), as GetNextEnvironment calls.
    /// It reopens and skips header text without parsing or allocating it again.
    pub fn reopen_without_processing_header(&mut self) -> Result<(), RawDayFailure> {
        self.input.close();
        self.input.reopen();
        let result = self.skip_header();
        self.stream_after_header = self.input.snapshot();
        result
    }

    /// The actual source one-record byte scan; never a cached-day rewind.
    pub fn backspace_record(&mut self) -> Result<(), RawDayFailure> {
        self.input.backspace()?;
        Ok(())
    }

    /// Rewind then execute SkipEPlusWFHeader, without processing headers again.
    pub fn rewind_and_skip_header(&mut self) -> Result<(), RawDayFailure> {
        self.input.rewind();
        self.skip_header()
    }

    /// Literal DATA PERIODS substring search and dummy token consumption.
    /// The prefix remains present for the ignored-error ProcessNumber call.
    pub fn skip_header(&mut self) -> Result<(), RawDayFailure> {
        let mut read;
        loop {
            read = self.input.read_line()?;
            if read.eof {
                return Err(super::RawEpwHeaderError::SourceFatal("unexpected-header-EOF").into());
            }
            if !read.read_good {
                return Err(RawDayFailure::OutsideBoundedDomain(
                    "non-progressing header skip stream",
                ));
            }
            read.data.make_ascii_uppercase();
            if read.data.contains("DATA PERIODS") {
                break;
            }
        }
        let mut number_arguments = 2_i32;
        let mut argument = 1_i32;
        while argument <= number_arguments {
            strip_ascii_spaces(&mut read.data);
            let end = self.input.field_end(&mut read.data)?;
            if argument == 1 {
                let number_periods =
                    to_source_int(process_number(&read.data[..end]).unwrap_or(0.0))?;
                number_arguments = number_periods
                    .checked_mul(4)
                    .and_then(|count| count.checked_add(2))
                    .ok_or(RawDayFailure::OutsideBoundedDomain(
                        "header skip argument count overflow",
                    ))?;
            }
            erase_field(&mut read.data, end);
            argument = argument
                .checked_add(1)
                .ok_or(RawDayFailure::OutsideBoundedDomain(
                    "header skip argument index overflow",
                ))?;
        }
        Ok(())
    }

    pub(super) fn admit_day_read(
        &self,
        selection: RawDayReadSelection,
    ) -> Result<(), RawDayFailure> {
        if self.header.intervals_per_hour != 1
            || self.header.number_data_periods != 1
            || self.header.data_periods.values.len() != 1
        {
            return Err(RawDayFailure::OutsideBoundedDomain(
                "hourly single-period EPW day reader",
            ));
        }
        if selection.match_year || selection.days_in_year <= 0 {
            return Err(RawDayFailure::OutsideBoundedDomain(
                "non-actual RunPeriod with positive NumDaysInYear",
            ));
        }
        Ok(())
    }

    pub(super) fn slot(&self, read: &RawEpwLineRead) -> RawWeatherSlot {
        RawWeatherSlot {
            raw: self.arguments,
            provenance: RawReadProvenance {
                start_byte: read.start_byte,
                end_byte: read.end_byte,
                line_read_attempt: self.input.line_reads,
            },
        }
    }

    pub(super) fn interpret(&mut self, read: &RawEpwLineRead) -> Result<(), RawDayFailure> {
        self.interpret_count =
            self.interpret_count
                .checked_add(1)
                .ok_or(RawDayFailure::OutsideBoundedDomain(
                    "raw parser invocation count overflow",
                ))?;
        let result = interpret_weather_data_line(&mut self.parser, &read.data, &mut self.arguments);
        // Same-owner observation, including actual writes before source failure.
        self.header.weather_code_missed_count = self.parser.weather_code_missed_count;
        self.header.month_ends = self.parser.end_day_of_month;
        result.map_err(RawDayFailure::Record)
    }

    pub(super) fn search_first_day(
        &mut self,
        selection: RawDayReadSelection,
        state: &mut RawDayReadState,
    ) -> Result<RawWeatherSlot, RawDayFailure> {
        state.read_time = 1.0 / f64::from(self.header.intervals_per_hour);
        state.current_day_of_week = self.header.data_periods.values[0]
            .weekday
            .checked_sub(1)
            .ok_or(RawDayFailure::OutsideBoundedDomain(
                "initial weekday subtraction overflow",
            ))?;
        state.last_hour_set = false;
        self.arguments.dates = [0; 5];
        let mut read = RawEpwLineRead {
            data: String::new(),
            eof: true,
            read_good: false,
            start_byte: self.input.byte_cursor,
            end_byte: self.input.byte_cursor,
        };
        let mut rewinds = 0;
        loop {
            read.update(self.input.read_line()?);
            if read.read_good {
                self.interpret(&read)?;
            } else if read.eof && rewinds == 0 {
                self.rewind_and_skip_header()?;
                rewinds += 1;
                read.update(self.input.read_line()?);
                // The source interprets even a failed replacement extraction.
                self.interpret(&read)?;
            }
            if !read.read_good {
                return Err(RawDayFailure::SourceFatal(RawDaySourceFatal::SearchRead));
            }
            if state.current_day_of_week <= 7 {
                state.current_day_of_week = state.current_day_of_week % 7 + 1;
            }
            let date = self.arguments.dates;
            if date[1] == selection.start_month && date[2] == selection.start_day {
                let selected = self.slot(&read);
                self.backspace_record()?;
                if state.current_day_of_week <= 7 {
                    state.current_day_of_week -= 1;
                }
                // Initial physical range diagnostics are owned by the producer;
                // their source severe errors do not terminate this search.
                return Ok(selected);
            }
            for _ in 0..23 {
                read.update(self.input.read_line()?);
                if !read.read_good {
                    // Source readList uses the retained final skipped line and
                    // writes only date references, without Interpret/counters.
                    let mut cursor = 0;
                    for date in &mut self.arguments.dates {
                        let _ = read_int(&read.data, &mut cursor, date);
                    }
                    return Err(RawDayFailure::SourceFatal(RawDaySourceFatal::SearchRead));
                }
            }
        }
    }
}
