//! Selected EnergyPlus raw EPW owners, before physical weather projection.

mod cursor;
mod dates;
mod day_read;
mod source_day_read;
pub(crate) use source_day_read::{
    SourceRawDayReadCallbacks, SourceRawDayReadClock, SourceRawDayReadFailure,
};
mod header;
mod header_state;
mod input;
mod lexical;
mod loader;
mod record;
mod record_types;
mod source_header;
mod source_open;
mod source_typical;
mod source_user_input;
pub(crate) use source_header::source_julian_date;
pub(crate) use source_typical::source_ordinal_day;

#[cfg(test)]
mod cursor_tests;
#[cfg(test)]
mod input_tests;

pub mod production_trace;

pub use cursor::RawEpwCursorOwner;
pub use day_read::{
    RawDayFailure, RawDayReadCallbacks, RawDayReadMode, RawDayReadSelection, RawDayReadState,
    RawDaySourceFatal, RawReadProvenance, RawWeatherDay, RawWeatherSlot,
};

pub use header::{HeaderKind, open_epw_header, process_epw_header};
pub use header_state::{
    AllocatedValues, DataPeriod, DateType, DstPeriod, HeaderCallerPreparation,
    HeaderContextConsumption, RawEpwHeaderError, RawEpwHeaderState, SourceTypicalExtremeData,
    SpecialDay,
};
pub use input::{RawEpwInput, RawEpwLineRead, RawEpwStreamState};
pub(crate) use loader::project_record;
pub use loader::{ParsedEpwWeatherFile, load_parsed_epw_weather_file};
pub use record::interpret_weather_data_line;
pub use record_types::{
    DATE_KEYS, MANDATORY_KEYS, OPTIONAL_KEYS, RawEpwOutputs, RawEpwParserState, RawRecordFailure,
};
