use ep_model::{DayOfWeek, SpecialDayType};

/// Schedule day type selected for a simulation day.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DayType {
    /// Sunday schedule day.
    Sunday,
    /// Monday schedule day.
    Monday,
    /// Tuesday schedule day.
    Tuesday,
    /// Wednesday schedule day.
    Wednesday,
    /// Thursday schedule day.
    Thursday,
    /// Friday schedule day.
    Friday,
    /// Saturday schedule day.
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

impl From<DayOfWeek> for DayType {
    fn from(value: DayOfWeek) -> Self {
        match value {
            DayOfWeek::Monday => Self::Monday,
            DayOfWeek::Tuesday => Self::Tuesday,
            DayOfWeek::Wednesday => Self::Wednesday,
            DayOfWeek::Thursday => Self::Thursday,
            DayOfWeek::Friday => Self::Friday,
            DayOfWeek::Saturday => Self::Saturday,
            DayOfWeek::Sunday => Self::Sunday,
        }
    }
}

impl From<SpecialDayType> for DayType {
    fn from(value: SpecialDayType) -> Self {
        match value {
            SpecialDayType::Holiday => Self::Holiday,
            SpecialDayType::SummerDesignDay => Self::SummerDesignDay,
            SpecialDayType::WinterDesignDay => Self::WinterDesignDay,
            SpecialDayType::CustomDay1 => Self::CustomDay1,
            SpecialDayType::CustomDay2 => Self::CustomDay2,
        }
    }
}

impl DayType {
    /// Returns the EnergyPlus timestamp label for this schedule day type.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Sunday => "Sunday",
            Self::Monday => "Monday",
            Self::Tuesday => "Tuesday",
            Self::Wednesday => "Wednesday",
            Self::Thursday => "Thursday",
            Self::Friday => "Friday",
            Self::Saturday => "Saturday",
            Self::Holiday => "Holiday",
            Self::SummerDesignDay => "SummerDesignDay",
            Self::WinterDesignDay => "WinterDesignDay",
            Self::CustomDay1 => "CustomDay1",
            Self::CustomDay2 => "CustomDay2",
        }
    }

    /// Returns the EnergyPlus `Site Day Type Index` report value.
    #[must_use]
    pub const fn energyplus_index(self) -> u32 {
        match self {
            Self::Sunday => 1,
            Self::Monday => 2,
            Self::Tuesday => 3,
            Self::Wednesday => 4,
            Self::Thursday => 5,
            Self::Friday => 6,
            Self::Saturday => 7,
            Self::Holiday => 8,
            Self::SummerDesignDay => 9,
            Self::WinterDesignDay => 10,
            Self::CustomDay1 => 11,
            Self::CustomDay2 => 12,
        }
    }
}
