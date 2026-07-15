use crate::{DayScheduleId, NormalizedName, ScheduleId, ScheduleTypeLimitId, WeekScheduleId};

/// Schedule numeric type.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NumericType {
    /// Continuous values.
    Continuous,
    /// Discrete values.
    Discrete,
}

/// Schedule type limits.
#[derive(Clone, Debug, PartialEq)]
pub struct ScheduleTypeLimits {
    /// Typed ID.
    pub id: ScheduleTypeLimitId,
    /// Object name.
    pub name: NormalizedName,
    /// Optional lower limit.
    pub lower_limit: Option<f64>,
    /// Optional upper limit.
    pub upper_limit: Option<f64>,
    /// Numeric type.
    pub numeric_type: Option<NumericType>,
}

/// Constant schedule.
#[derive(Clone, Debug, PartialEq)]
pub struct ScheduleConstant {
    /// Typed ID.
    pub id: ScheduleId,
    /// Schedule name.
    pub name: NormalizedName,
    /// Optional type limits.
    pub schedule_type_limits: Option<ScheduleTypeLimitId>,
    /// Constant hourly value.
    pub hourly_value: f64,
}

/// Immutable hourly profile referenced by week schedules.
#[derive(Clone, Debug, PartialEq)]
pub struct ScheduleDayHourly {
    /// Typed day-schedule ID.
    pub id: DayScheduleId,
    /// Day-schedule name.
    pub name: NormalizedName,
    /// Optional type limits.
    pub schedule_type_limits: Option<ScheduleTypeLimitId>,
    /// Hour-ending values for hours 1 through 24.
    pub hourly_values: [f64; 24],
}

/// Immutable interval profile referenced by week schedules.
#[derive(Clone, Debug, PartialEq)]
pub struct ScheduleDayInterval {
    /// Typed day-schedule ID shared with the other day-schedule families.
    pub id: DayScheduleId,
    /// Day-schedule name.
    pub name: NormalizedName,
    /// Optional type limits.
    pub schedule_type_limits: Option<ScheduleTypeLimitId>,
    /// Interpolation mode used to populate zone-timestep values.
    pub interpolation: ScheduleInterpolation,
    /// Source-ordered daily `Until` value segments.
    pub segments: Vec<ScheduleCompactSegment>,
}

/// Immutable source-value list referenced by week schedules.
#[derive(Clone, Debug, PartialEq)]
pub struct ScheduleDayList {
    /// Typed day-schedule ID shared with the other day-schedule families.
    pub id: DayScheduleId,
    /// Day-schedule name.
    pub name: NormalizedName,
    /// Optional type limits.
    pub schedule_type_limits: Option<ScheduleTypeLimitId>,
    /// Interpolation mode used to populate zone-timestep values.
    pub interpolation: ScheduleInterpolation,
    /// Source minutes represented by each list item.
    pub minutes_per_item: u32,
    /// Source-ordered values covering exactly one 24-hour day.
    pub values: Vec<f64>,
}

/// Immutable mapping from all EnergyPlus day types to typed day schedules.
#[derive(Clone, Debug, PartialEq)]
pub struct ScheduleWeekDaily {
    /// Typed week-schedule ID.
    pub id: WeekScheduleId,
    /// Week-schedule name.
    pub name: NormalizedName,
    /// Day-schedule IDs ordered Sunday through CustomDay2.
    pub day_schedules: [DayScheduleId; 12],
}

/// Immutable compact mapping from EnergyPlus day-type selectors to typed day schedules.
#[derive(Clone, Debug, PartialEq)]
pub struct ScheduleWeekCompact {
    /// Typed week-schedule ID shared with `Schedule:Week:Daily`.
    pub id: WeekScheduleId,
    /// Week-schedule name.
    pub name: NormalizedName,
    /// Materialized day-schedule IDs ordered Sunday through CustomDay2.
    pub day_schedules: [DayScheduleId; 12],
}

/// Immutable annual schedule assembled from daily week-schedule pointers.
#[derive(Clone, Debug, PartialEq)]
pub struct ScheduleYear {
    /// Typed ID shared with top-level schedule families.
    pub id: ScheduleId,
    /// Annual schedule name.
    pub name: NormalizedName,
    /// Optional type limits.
    pub schedule_type_limits: Option<ScheduleTypeLimitId>,
    /// Week-schedule pointer for every leap-shaped ordinal day, 1 through 366.
    pub week_schedules: [WeekScheduleId; 366],
}

/// Schedule initialized locally for an inactive EnergyPlus external interface.
#[derive(Clone, Debug, PartialEq)]
pub struct ExternalInterfaceSchedule {
    /// Typed ID shared with all other top-level schedule families.
    pub id: ScheduleId,
    /// Schedule name.
    pub name: NormalizedName,
    /// Optional type limits.
    pub schedule_type_limits: Option<ScheduleTypeLimitId>,
    /// Value held while live external-interface exchange is inactive.
    pub initial_value: f64,
}

/// FMU-import schedule initialized locally while its external interface is inactive.
#[derive(Clone, Debug, PartialEq)]
pub struct ExternalInterfaceFmuImportSchedule {
    /// Typed ID shared with all other top-level schedule families.
    pub id: ScheduleId,
    /// Schedule name.
    pub name: NormalizedName,
    /// Optional type limits.
    pub schedule_type_limits: Option<ScheduleTypeLimitId>,
    /// Retain-case FMU archive filename.
    pub fmu_file_name: String,
    /// Retain-case FMU instance name.
    pub fmu_instance_name: String,
    /// Retain-case FMU output-variable name.
    pub fmu_variable_name: String,
    /// Value held while live FMU-import exchange is inactive.
    pub initial_value: f64,
}

/// Delimiter used by one flat `Schedule:File` input.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ScheduleFileColumnSeparator {
    /// Comma-separated values.
    #[default]
    Comma,
    /// Tab-separated values.
    Tab,
    /// Space-separated values.
    Space,
    /// Semicolon-separated values.
    Semicolon,
}

impl ScheduleFileColumnSeparator {
    /// Returns the delimiter character consumed by the flat-file parser.
    #[must_use]
    pub const fn delimiter(self) -> char {
        match self {
            Self::Comma => ',',
            Self::Tab => '\t',
            Self::Space => ' ',
            Self::Semicolon => ';',
        }
    }
}

/// Hourly values loaded from one supported `Schedule:File` sidecar.
#[derive(Clone, Debug, PartialEq)]
pub struct ScheduleFile {
    /// Typed ID shared with all other schedule families.
    pub id: ScheduleId,
    /// Schedule name.
    pub name: NormalizedName,
    /// Optional type limits.
    pub schedule_type_limits: Option<ScheduleTypeLimitId>,
    /// Source filename retained for diagnostics and provenance.
    pub file_name: String,
    /// One-based selected column.
    pub column_number: u32,
    /// Header rows skipped before numeric data.
    pub rows_to_skip_at_top: u32,
    /// Declared source-hour count.
    pub number_of_hours_of_data: u32,
    /// Source column delimiter.
    pub column_separator: ScheduleFileColumnSeparator,
    /// Whether EnergyPlus timestep interpolation was requested.
    pub interpolate_to_timestep: bool,
    /// Source minutes represented by each item.
    pub minutes_per_item: u32,
    /// Whether the file lookup follows daylight-saving shifts.
    pub adjust_schedule_for_daylight_savings: bool,
    /// Immutable source values loaded during compilation.
    pub values: Vec<f64>,
}

/// All surface sunlit-fraction schedules loaded from one `Schedule:File:Shading` sidecar.
#[derive(Clone, Debug, PartialEq)]
pub struct ScheduleFileShading {
    /// Source filename retained for diagnostics and provenance.
    pub file_name: String,
    /// Zone timesteps represented by each source hour.
    pub timesteps_per_hour: u32,
    /// Number of source calendar days, either 365 or 366.
    pub source_day_count: u32,
    /// Generated surface schedules in deterministic header order.
    pub columns: Vec<ScheduleFileShadingColumn>,
}

/// One generated surface schedule from a `Schedule:File:Shading` column.
#[derive(Clone, Debug, PartialEq)]
pub struct ScheduleFileShadingColumn {
    /// Typed ID shared with all other top-level schedule families.
    pub id: ScheduleId,
    /// Surface-column header retained for diagnostics and provenance.
    pub surface_header: String,
    /// Normalized generated schedule name, `{surface_header}_shading`.
    pub schedule_name: NormalizedName,
    /// Immutable source values ordered by day, hour, and zone timestep.
    pub values: Vec<f64>,
}

/// One value segment in an interval-based schedule day profile.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScheduleCompactSegment {
    /// Minute of day at which this segment ends, 1 through 1440.
    pub until_minute_of_day: u32,
    /// Segment value.
    pub value: f64,
}

/// Interpolation mode applied within one interval-based day profile.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ScheduleInterpolation {
    /// Hold each segment value constant until the next `Until` boundary.
    #[default]
    No,
    /// Average minute-level values over each zone timestep.
    Average,
    /// Linearly interpolate between consecutive segment values.
    Linear,
}

/// EnergyPlus schedule day type consumed by `Schedule:Compact` `For` rules.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScheduleDayType {
    /// Sunday.
    Sunday,
    /// Monday.
    Monday,
    /// Tuesday.
    Tuesday,
    /// Wednesday.
    Wednesday,
    /// Thursday.
    Thursday,
    /// Friday.
    Friday,
    /// Saturday.
    Saturday,
    /// Holiday schedule day.
    Holiday,
    /// Summer design-day schedule day.
    SummerDesignDay,
    /// Winter design-day schedule day.
    WinterDesignDay,
    /// First custom schedule day.
    CustomDay1,
    /// Second custom schedule day.
    CustomDay2,
}

/// One source-ordered day profile within a compact-schedule period.
#[derive(Clone, Debug, PartialEq)]
pub struct ScheduleCompactDayProfile {
    /// Day types assigned by this `For` rule, after group expansion.
    pub day_types: Vec<ScheduleDayType>,
    /// Interpolation mode declared for this `For` rule.
    pub interpolation: ScheduleInterpolation,
    /// Source-ordered daily `Until` value segments.
    pub segments: Vec<ScheduleCompactSegment>,
}

/// One source-ordered `Through` period in a compact schedule.
#[derive(Clone, Debug, PartialEq)]
pub struct ScheduleCompactPeriod {
    /// Leap-shaped schedule ordinal selected by the `Through` date, 1 through 366.
    pub through_schedule_day_of_year: u16,
    /// Source-ordered `For` day profiles in this period.
    pub day_profiles: Vec<ScheduleCompactDayProfile>,
}

/// Compact schedule using source-ordered `Through`, `For`, `Interpolate`, and `Until` rules.
#[derive(Clone, Debug, PartialEq)]
pub struct ScheduleCompact {
    /// Typed ID.
    pub id: ScheduleId,
    /// Schedule name.
    pub name: NormalizedName,
    /// Optional type limits.
    pub schedule_type_limits: Option<ScheduleTypeLimitId>,
    /// Source-ordered annual `Through` periods.
    pub periods: Vec<ScheduleCompactPeriod>,
}
