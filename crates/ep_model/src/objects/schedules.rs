use crate::{NormalizedName, ScheduleId, ScheduleTypeLimitId};

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

/// One value segment in a compact schedule day profile.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScheduleCompactSegment {
    /// Minute of day at which this segment ends, 1 through 1440.
    pub until_minute_of_day: u32,
    /// Segment value.
    pub value: f64,
}

/// Interpolation mode applied within one compact-schedule day profile.
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
