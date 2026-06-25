use crate::{AutoOrNumber, NormalizedName, Point3, RunPeriodId, ZoneId};

/// EnergyPlus-compatible model version.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Version {
    /// Major version.
    pub major: u16,
    /// Minor version.
    pub minor: u16,
    /// Patch version.
    pub patch: u16,
}

impl Version {
    /// Initial oracle version.
    #[must_use]
    pub const fn oracle_26_1_0() -> Self {
        Self {
            major: 26,
            minor: 1,
            patch: 0,
        }
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// Terrain enum used by Building.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Terrain {
    /// City center.
    City,
    /// Flat open country.
    Country,
    /// Large water body within 5 km.
    Ocean,
    /// Country towns and suburbs.
    Suburbs,
    /// Urban, industrial, or forest.
    Urban,
}

/// Solar distribution enum used by Building.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SolarDistribution {
    /// Minimal shadowing.
    MinimalShadowing,
    /// Full exterior.
    FullExterior,
    /// Full exterior with reflections.
    FullExteriorWithReflections,
    /// Full interior and exterior.
    FullInteriorAndExterior,
    /// Full interior and exterior with reflections.
    FullInteriorAndExteriorWithReflections,
}

/// Default inside surface convection algorithm.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InsideSurfaceConvectionAlgorithm {
    /// Constant natural convection.
    Simple,
    /// ASHRAE TARP natural convection.
    Tarp,
    /// Ceiling diffuser mixed convection.
    CeilingDiffuser,
    /// EnergyPlus adaptive inside convection model selection.
    AdaptiveConvectionAlgorithm,
    /// ASTM C1340 mixed convection correlations.
    AstmC1340,
}

/// Default outside surface convection algorithm.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OutsideSurfaceConvectionAlgorithm {
    /// Simple combined exterior coefficient.
    SimpleCombined,
    /// TARP exterior convection.
    Tarp,
    /// MoWiTT smooth-surface exterior convection.
    MoWitt,
    /// DOE-2 rough-surface exterior convection.
    Doe2,
    /// EnergyPlus adaptive outside convection model selection.
    AdaptiveConvectionAlgorithm,
}

/// Global surface convection algorithm settings.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SurfaceConvectionAlgorithms {
    /// Parsed `SurfaceConvectionAlgorithm:Inside` setting, when present.
    pub inside: Option<InsideSurfaceConvectionAlgorithm>,
    /// Parsed `SurfaceConvectionAlgorithm:Outside` setting, when present.
    pub outside: Option<OutsideSurfaceConvectionAlgorithm>,
}

/// Building-level typed settings.
#[derive(Clone, Debug, PartialEq)]
pub struct Building {
    /// Object name.
    pub name: NormalizedName,
    /// North axis in degrees.
    pub north_axis_deg: f64,
    /// Terrain classification.
    pub terrain: Terrain,
    /// Loads convergence tolerance in watts.
    pub loads_convergence_tolerance_w: f64,
    /// Temperature convergence tolerance in delta C.
    pub temperature_convergence_tolerance_delta_c: f64,
    /// Solar distribution algorithm.
    pub solar_distribution: SolarDistribution,
    /// Maximum warmup day count.
    pub maximum_number_of_warmup_days: u32,
    /// Minimum warmup day count.
    pub minimum_number_of_warmup_days: u32,
}

/// Zone timestep configuration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimestepConfig {
    /// Number of zone timesteps per hour.
    pub number_of_timesteps_per_hour: u32,
}

impl Default for TimestepConfig {
    fn default() -> Self {
        Self {
            number_of_timesteps_per_hour: 6,
        }
    }
}

/// Calendar day of week used by `RunPeriod`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DayOfWeek {
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
    /// Sunday.
    Sunday,
}

/// RunPeriod first-hour weather interpolation starting point.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum FirstHourInterpolationStartingValues {
    /// Use the first hour of the first run-period weather day.
    Hour1,
    /// Use the 24th hour of the first run-period weather day.
    #[default]
    Hour24,
}

/// Run period date range.
#[derive(Clone, Debug, PartialEq)]
pub struct RunPeriod {
    /// Typed ID.
    pub id: RunPeriodId,
    /// Object name.
    pub name: NormalizedName,
    /// Begin month.
    pub begin_month: u32,
    /// Begin day of month.
    pub begin_day_of_month: u32,
    /// Optional begin year.
    pub begin_year: Option<u32>,
    /// End month.
    pub end_month: u32,
    /// End day of month.
    pub end_day_of_month: u32,
    /// Optional end year.
    pub end_year: Option<u32>,
    /// Optional declared start day of week.
    pub day_of_week_for_start_day: Option<DayOfWeek>,
    /// First-hour weather interpolation starting point.
    pub first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues,
}

/// Site location.
#[derive(Clone, Debug, PartialEq)]
pub struct SiteLocation {
    /// Object name.
    pub name: NormalizedName,
    /// Latitude in degrees.
    pub latitude_deg: f64,
    /// Longitude in degrees.
    pub longitude_deg: f64,
    /// Time zone offset in hours.
    pub time_zone_hours: f64,
    /// Elevation in meters.
    pub elevation_m: f64,
}

/// Thermal zone.
#[derive(Clone, Debug, PartialEq)]
pub struct Zone {
    /// Typed ID.
    pub id: ZoneId,
    /// Zone name.
    pub name: NormalizedName,
    /// Direction of relative north in degrees.
    pub direction_of_relative_north_deg: f64,
    /// Zone origin.
    pub origin: Point3,
    /// EnergyPlus zone type.
    pub zone_type: u32,
    /// Zone multiplier.
    pub multiplier: u32,
    /// Ceiling height.
    pub ceiling_height: AutoOrNumber,
    /// Zone volume.
    pub volume: AutoOrNumber,
}
