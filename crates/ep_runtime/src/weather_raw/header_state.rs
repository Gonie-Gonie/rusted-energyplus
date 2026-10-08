//! Defined EPW header owners from WeatherManager.hh and EPVector.hh.

/// Source classification for a parsed date, preserving its integer values.
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DateType {
    /// No valid date was established.
    #[default]
    Invalid = -1,
    /// A literal month and day.
    MonthDay = 1,
    /// A numbered weekday in a month.
    NthDayInMonth = 2,
    /// A last weekday, or the source's single ordinal-number classification.
    LastDayInMonth = 3,
}

/// EPVector's allocation state and ordered, value-constructed elements.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct AllocatedValues<T> {
    /// Allocation remains true after `allocate(0)`.
    pub allocated: bool,
    /// Elements in the original one-based storage order.
    pub values: Vec<T>,
}
impl<T: Default + Clone> AllocatedValues<T> {
    /// Resize and reset every retained and new slot to its actual constructor.
    pub fn allocate(&mut self, size: usize) {
        self.allocated = true;
        self.values.resize(size, T::default());
        self.values.fill(T::default());
    }
}

/// A distinct DSTPeriod owner; EPWDST and DST are separate instances.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct DstPeriod {
    /// Start date classification.
    pub start_date_type: DateType,
    /// Start weekday number.
    pub start_weekday: i32,
    /// Start month number.
    pub start_month: i32,
    /// Start day or weekday occurrence number.
    pub start_day: i32,
    /// End date classification.
    pub end_date_type: DateType,
    /// End month number.
    pub end_month: i32,
    /// End day or weekday occurrence number.
    pub end_day: i32,
    /// End weekday number.
    pub end_weekday: i32,
}

/// A literal SpecialDayData value, including fields this parser does not write.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct SpecialDay {
    /// Uppercased source name.
    pub name: String,
    /// Date classification.
    pub date_type: DateType,
    /// Source month.
    pub month: i32,
    /// Source day or occurrence.
    pub day: i32,
    /// Source weekday, zero for a month/day date.
    pub weekday: i32,
    /// Month times 32 plus day for month/day dates.
    pub compressed_date: i32,
    /// Whether this value came from the weather file.
    pub weather_file: bool,
    /// Duration written by this parser.
    pub duration: i32,
    /// Day type written by this parser.
    pub day_type: i32,
    /// Later lifecycle's actual start month; not written here.
    pub actual_start_month: i32,
    /// Later lifecycle's actual start day; not written here.
    pub actual_start_day: i32,
    /// Later lifecycle's usage flag; not written here.
    pub used: bool,
}

/// DataPeriodData with its source constructor and all selected stores.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DataPeriod {
    /// Uppercased period description.
    pub name: String,
    /// Uppercased weekday token.
    pub day_of_week: String,
    /// Source constructor defaults this to one; the header leaves it alone.
    pub number_years_data: i32,
    /// Source day-type lookup result, including invalid values.
    pub weekday: i32,
    /// Start month.
    pub start_month: i32,
    /// Start day.
    pub start_day: i32,
    /// Optional start year, zero when the token parser finds no year.
    pub start_year: i32,
    /// End month.
    pub end_month: i32,
    /// End day.
    pub end_day: i32,
    /// Optional end year or a copied start year.
    pub end_year: i32,
    /// Source period length, including its literal no-year wrap formula.
    pub number_days: i32,
    /// Weekdays of month starts, reset before conditional setup.
    pub month_weekdays: [i32; 12],
    /// Ordinal or actual-year Julian start date.
    pub start_julian_day: i32,
    /// Ordinal or actual-year Julian end date.
    pub end_julian_day: i32,
    /// Set when a nonzero start year was stored.
    pub has_year_data: bool,
}
impl Default for DataPeriod {
    fn default() -> Self {
        Self {
            name: String::new(),
            day_of_week: String::new(),
            number_years_data: 1,
            weekday: 0,
            start_month: 0,
            start_day: 0,
            start_year: 0,
            end_month: 0,
            end_day: 0,
            end_year: 0,
            number_days: 0,
            month_weekdays: [0; 12],
            start_julian_day: 0,
            end_julian_day: 0,
            has_year_data: false,
        }
    }
}

/// Selected raw header state, before physical weather and calendar projection.
#[derive(Clone, Debug, PartialEq)]
pub struct RawEpwHeaderState {
    /// WeatherManager's destructively assembled EPWHeaderTitle.
    pub header_title: String,
    /// Separate DataEnvironment title copied after LOCATION completes.
    pub weather_location_title: String,
    /// Raw weather-file latitude.
    pub latitude: f64,
    /// Raw weather-file longitude.
    pub longitude: f64,
    /// Raw weather-file time zone.
    pub time_zone: f64,
    /// Raw weather-file elevation.
    pub elevation: f64,
    /// Header leap-year permission, distinct from the prepared leap addend.
    pub allows_leap_years: bool,
    /// Whether an active EPW DST period survived parsing.
    pub epw_daylight_saving: bool,
    /// Weather-file DST object.
    pub epw_dst: DstPeriod,
    /// Distinct copied active DST object.
    pub dst: DstPeriod,
    /// Weather-file holidays plus explicit InputProcessor special-day count.
    pub number_special_days: i32,
    /// Allocated special-day storage and ordered values.
    pub special_days: AllocatedValues<SpecialDay>,
    /// Declared data-period count.
    pub number_data_periods: i32,
    /// Declared record intervals per hour; no positive-value admission here.
    pub intervals_per_hour: i32,
    /// Allocated data-period storage and ordered values.
    pub data_periods: AllocatedValues<DataPeriod>,
    /// Caller-prepared LeapYearAdd, not inferred from WFAllowsLeapYears.
    pub leap_year_add: i32,
    /// Caller-prepared EndDayOfMonth values.
    pub month_ends: [i32; 12],
    /// Raw parser's weather-code miss counter on this same owner.
    pub weather_code_missed_count: i32,
}
impl Default for RawEpwHeaderState {
    fn default() -> Self {
        Self {
            header_title: String::new(),
            weather_location_title: String::new(),
            latitude: 0.0,
            longitude: 0.0,
            time_zone: 0.0,
            elevation: 0.0,
            allows_leap_years: false,
            epw_daylight_saving: false,
            epw_dst: DstPeriod::default(),
            dst: DstPeriod::default(),
            number_special_days: 0,
            special_days: AllocatedValues::default(),
            number_data_periods: 0,
            intervals_per_hour: 1,
            data_periods: AllocatedValues::default(),
            leap_year_add: 0,
            month_ends: [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31],
            weather_code_missed_count: 0,
        }
    }
}

/// Explicit external InputProcessor prerequisite for this bounded header call.
#[derive(Clone, Copy, Debug, Default)]
pub struct HeaderCallerPreparation {
    /// Number of actual RunPeriodControl:SpecialDays objects.
    pub run_period_control_special_days: i32,
}

/// Consumption of source-only Typical/Ground branches, without derived science.
#[derive(Clone, Debug, Default)]
pub struct HeaderContextConsumption {
    /// Source Typical/Extreme count, including its early truncation store.
    pub typical_declared_count: Option<i32>,
    /// Consumed Typical/Extreme tokens in actual source order.
    pub typical_consumed_fields: Vec<String>,
    /// Consumed Ground tokens in actual source order.
    pub ground_consumed_fields: Vec<String>,
    /// Explicitly unowned computed source-only fields.
    pub unimplemented_computed_state: Vec<&'static str>,
}

/// Separate source termination from an input outside the defined bounded port.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RawEpwHeaderError {
    /// The original selected header operation terminates with FatalError.
    SourceFatal(&'static str),
    /// Original unsafe/unwritten storage or an unsupported byte-input domain.
    OutsideBoundedDomain(&'static str),
}

pub(super) fn to_source_int(value: f64) -> Result<i32, RawEpwHeaderError> {
    let truncated = value.trunc();
    if !truncated.is_finite() || truncated < f64::from(i32::MIN) || truncated > f64::from(i32::MAX)
    {
        return Err(RawEpwHeaderError::OutsideBoundedDomain(
            "undefined original real-to-int conversion",
        ));
    }
    Ok(truncated as i32)
}

pub(super) fn source_count(value: i32) -> Result<usize, RawEpwHeaderError> {
    usize::try_from(value)
        .map_err(|_| RawEpwHeaderError::OutsideBoundedDomain("negative original array allocation"))
}

pub(super) fn defined(value: Option<i32>, reason: &'static str) -> Result<i32, RawEpwHeaderError> {
    value.ok_or(RawEpwHeaderError::OutsideBoundedDomain(reason))
}

pub(super) fn strip_ascii_spaces(text: &mut String) {
    *text = text.trim_matches(' ').to_owned();
}
