---
status: active
claim_level: planning-guard
owner: runtime
last_reviewed: 2026-07-14
---

# Time, Weather, and Schedule Source Map

Reference version: EnergyPlus 26.1.0

Reference source root:

```text
.reference/energyplus-src/26.1.0/
```

This document locks the EnergyPlus source order and state ownership required
for the Rust calendar, weather, schedule, and timestamp runtime. The first
calendar checkpoint described below is an implementation boundary, not an
EnergyPlus conformance claim.

## Primary Source Files

| Area | EnergyPlus 26.1 source | Rust target |
|---|---|---|
| environment and simulation loops | `src/EnergyPlus/SimulationManager.cc` | `ep_runtime::time_axis`; future environment driver |
| run-period input, environment construction, calendar, and weather | `src/EnergyPlus/WeatherManager.cc` | `ep_model::RunPeriod`; `ep_runtime::time_axis`; `ep_runtime::weather` |
| schedule current-value update and lookup | `src/EnergyPlus/ScheduleManager.cc`; `src/EnergyPlus/ScheduleManager.hh` | `ep_runtime::schedules` |
| ESO, MTR, and SQL timestamp serialization | `src/EnergyPlus/OutputProcessor.cc`; `src/EnergyPlus/OutputProcessor.hh` | `ep_runtime::output`; `ep_runtime::ResultStore` |

## Locked Source Order

The required ordering is:

```text
SimulationManager::ManageSimulation
  -> environment loop: Weather::GetNextEnvironment
     -> [first call] Weather::OpenWeatherFile
        -> Weather::ProcessEPWHeader
        -> Weather::ReadUserWeatherInput
           -> Weather::GetRunPeriodData
           -> Weather::GetSpecialDayPeriodData
           -> Weather::GetDSTData
           -> Weather::SetupEnvironmentTypes
     -> Weather::SetSpecialDayDates
     -> day loop, including repeated warmup days
        -> hour loop: HourOfDay = 1..24
           -> zone-timestep loop: TimeStep = 1..TimeStepsInHour
              -> Weather::ManageWeather
                 -> Weather::InitializeWeather
                    -> [BeginDayFlag] Weather::UpdateWeatherData
                 -> EMS BeginZoneTimestepBeforeSetCurrentWeather barrier
                 -> Weather::SetCurrentWeather
                    -> Sched::UpdateScheduleVals
                       -> ScheduleDetailed::getHrTsVal or
                          ScheduleConstant::getHrTsVal
                 -> Weather::ReportWeatherAndTimeInformation
              -> exterior loads and heat-balance work
              -> output aggregation and timestamp serialization
                 through OutputProcessor::WriteTimeStampFormatData
```

`SimulationManager::ManageSimulation` owns traversal. It resets the environment
counter, calls `GetNextEnvironment`, then nests the environment, day, hour, and
zone-timestep loops. It sets `HourOfDay`, `TimeStep`, and the begin/end flags;
the last zone timestep of hour 24 propagates `EndHourFlag`, `EndDayFlag`, and,
outside warmup, `EndEnvrnFlag`. `ManageWeather` is called after those flags are
resolved and before `ManageHeatBalance`. Warmup repeats the day loop and is not
equivalent to advancing the reported run-period calendar.

## Routine and State Ownership

| Source-order stage | EnergyPlus routine | State owned at this boundary |
|---|---|---|
| EPW header intake | `Weather::ProcessEPWHeader` | On the first `GetNextEnvironment` call, parses the `HOLIDAYS/DAYLIGHT SAVINGS` leap-year flag and optional daylight-saving boundaries plus `DATA PERIODS` record frequency, names, start weekdays, and start/end dates before `ReadUserWeatherInput` allocates weather-file holidays and reads IDF calendar objects. This metadata constrains later calendar and source reads; it is not itself a selected weather stream. |
| run-period intake | `Weather::GetRunPeriodData` | Validates and resolves `RunPeriodInput` start/end dates and years, start weekday, Julian bounds, number of simulation years, holiday/DST/weekend/rain/snow policies, actual-weather policy, and first-hour interpolation policy. It does not advance simulation time. |
| special-day intake | `Weather::GetSpecialDayPeriodData` | Parses ordered `RunPeriodControl:SpecialDays` definitions into date rule, duration, and special day type after any weather-file entries have been allocated. Intake does not select today's day type. |
| environment materialization | `Weather::SetupEnvironmentTypes` | Copies each run period into `Environment`, derives `StartJDay`, `EndJDay`, `RawSimDays`, `TotalDays`, leap-year handling, environment kind/name, weekday map seed, and the run-period policy flags. This is descriptor construction, not current-day state. |
| environment selection | `Weather::GetNextEnvironment` | Advances `Envrn`, selects the descriptor, and seeds `KindOfSim`, `CalendarYear`, month/day/day-of-year, `NumOfDayInEnvrn`, `CurEnvirNum`, and environment name. For a weather run it resolves `CurrentYearIsLeapYear`, weekday tables, active DST ranges, special-day dates, and the effective weather policy switches. |
| special-day projection | `Weather::SetSpecialDayDates` | Resets the annual special-day table and resolves enabled definitions against the environment weekday/leap shape before weather-day reads consume the resulting day type. The fixed IDF case below is the only external evidence; broader date, duration, ordering, weekend, and EPW branches remain unit/source evidence or unclaimed. |
| nested traversal | `SimulationManager::ManageSimulation` | Owns `DayOfSim`, `HourOfDay`, `TimeStep`, warmup repetition, and the environment/day/hour/timestep begin/end flags. Calendar and weather routines consume these counters; they do not own the nested loop. |
| weather driver | `Weather::ManageWeather` | Preserves the barrier `InitializeWeather` -> pre-weather EMS call -> `SetCurrentWeather` -> weather/time reporting for every zone timestep. |
| environment/day initialization | `Weather::InitializeWeather` | On `BeginEnvrnFlag`, initializes missing-value state and reads the first weather day. On `BeginDayFlag`, calls `UpdateWeatherData` before reading or preparing the next tomorrow record, then exposes tomorrow date/day-type fields for rollover. |
| source weather-day read | `Weather::ReadWeatherForDay`; `Weather::ReadEPlusWeatherForDay` | On the first read, validates data-period coverage and searches source order for the literal environment start month/day (and year only for actual weather). It reads the accepted source day into tomorrow state, applies the leap-day policy after positioning, and controls the restricted end-of-file rewind branch. Later reads advance one accepted source day at a time. |
| daily state commit | `Weather::UpdateWeatherData` | Commits `TomorrowVariables` and `wvarsHrTsTomorrow` to today's state, then writes current year/month/day/day-of-year, weekday, `HolidayIndex`, report day type, `DSTIndicator`, and daily solar-calendar values. This is the daily calendar commit point. |
| zone-timestep weather and time | `Weather::SetCurrentWeather` | Resolves `NextHour`, leap-shaped `DayOfYear_Schedule`, schedule values, `CurrentTime`, `SimTimeSteps`, and the current zone-timestep environment fields from the already prepared `wvarsHrTsToday` sample. In EnergyPlus 26.1, it does not read tomorrow hour 1 at hour 24: non-solar previous/current-hour interpolation was prepared while reading the day, and the solar `NextHr` used there wraps hour 24 to hour 1 of that same accepted source day. Weather values and derived psychrometric state are committed here. |
| schedule current values | `Sched::UpdateScheduleVals` | Writes every schedule's `currentVal`: an EMS value wins when actuated; otherwise it calls `getHrTsVal(state, HourOfDay, TimeStep)`. It does not calculate or advance calendar state. |
| detailed schedule lookup | `Sched::ScheduleDetailed::getHrTsVal` | Reads `DayOfYear_Schedule`, weekday, holiday index, `DSTIndicator`, hour, and zone timestep. It applies schedule-specific DST use, rolls DST-shifted hour 24 into tomorrow's weekday/holiday, selects holiday or weekday day schedule, and returns the indexed timestep value. |
| constant schedule lookup | `Sched::ScheduleConstant::getHrTsVal` | Returns the stored constant timestep value independently of date and hour; EMS override remains the caller's current-value policy. |
| timestamp serialization | `OutputProcessor::WriteTimeStampFormatData` | Serializes already-resolved day-of-simulation, month, day, hour, minute bounds, DST indicator, and day type to ESO/MTR and, when requested, the SQL time index. It does not derive or advance calendar state. |

The `getHrTsVal` token above refers to the virtual schedule lookup contract and
its concrete detailed/constant implementations. The `UpdateScheduleVals` and
`WriteTimeStampFormatData` tokens are deliberately retained because schedule
and output parity must be checked at these exact ownership boundaries.

## First Rust Checkpoint: Calendar Spine and Hourly Projection

The first checkpoint introduces one canonical run-period calendar spine:

- `ResolvedRunPeriodCalendar` owns the resolved begin/end Gregorian dates,
  start weekday, total day count, and leap-year shape for one run period.
- `EnvironmentTimeAxis` owns a one-based index among the weather run periods
  materialized by this checkpoint, environment name and `WeatherRunPeriod`
  kind, resolved calendar, first-hour weather policy, zone/system timestep
  profiles, and every zone-timestep point. The index is not yet the complete
  EnergyPlus `Envrn` ordinal because design and sizing environments are not
  materialized.
- `EnvironmentTimePoint` owns the environment identity, zero-based sample
  index, one-based `day_of_sim`, Gregorian year/month/day, Gregorian leap-year
  flag, weather-effective leap shape and `LeapYearAdd`, separate Gregorian,
  weather, and leap-shaped schedule day-of-year fields, separate Gregorian and
  simulation weekdays, current day type, explicit DST/special-day state, hour
  `1..24`, zone timestep `1..N`, minute bounds, current-time hours, one-based
  simulation-timestep index, and environment/day/hour begin/end flags.
- `build_environment_time_axes` builds the ordered axes for typed run periods;
  `build_environment_time_axes_with_weather_metadata` applies parsed EPW
  calendar policy to the same projection; the one-run-period builders reject
  an invalid date or reversed range, while metadata-aware builders additionally
  reject actual-weather and cross-year traversal until those broader EPW
  traversal branches are ported.
- Existing `TimeAxis` and `TimePoint` values remain the canonical axis's
  one-sample-per-hour, hour-ending projection for existing weather, schedule,
  output, and report consumers. They are not a second calendar authority.

At this checkpoint, EnergyPlus-style missing-year resolution is state-backed.
A yearless non-February-29 start uses the weekday-matching lookup year when a
start weekday is supplied and otherwise uses 2017; a yearless February 29 start
uses a matching leap year or the 2012 default. An explicit start year owns the
Gregorian weekday even when the input weekday differs. A missing end year first
uses the start year, then advances to the next valid year when the end date
would precede the start; a yearless February 29 end advances to the required
leap year. Explicit reversed ranges remain errors. Gregorian date traversal,
cross-year ranges, leap days, weekday progression, zone-timestep minute bounds,
counters, and begin/end flags are also state-backed. `schedule_day_of_year`
retains a February 29 slot even in a non-leap year, matching the indexing shape
consumed by EnergyPlus schedules.

Gregorian-only axes and metadata-aware axes without an active weather-file
period keep `dst=false`. Metadata-aware axes now resolve the parsed EPW
daylight-saving period when the RunPeriod enables that period. Typed
`RunPeriodControl:SpecialDays` input now shares the same calendar projection:
`CalendarDateRule` retains fixed, Nth-weekday, and last-weekday rules;
`SpecialDayType` retains the five EnergyPlus special schedule types; and
`SpecialDayAxisState` resolves enabled EPW holidays before typed definitions in
their compiled vector order into the effective `DayType` and optional
`special_day_type` carried by both environment and hourly points. EPW holidays
retain their source-exact Sunday day type. The simulation-effective weekday remains separately owned;
it equals the Gregorian weekday on ordinary paths but does not advance across
a February 29 skipped by EPW policy.

Source-mapped unit tests cover Nth/last resolution, inclusive duration with a
same-year annual-table wrap, directly ordered typed-vector overwrite, and the
fixed single-day weekend shift.
Those branches are not external conformance evidence. The fixed duration-one
IDF Holiday and paired fixed EPW holiday use-policy cases below are the only
external special-day boundaries.
Its `gregorian_year_is_leap_year` field is date arithmetic state, not
EnergyPlus' weather-effective `CurrentYearIsLeapYear`; the latter also depends
on EPW leap-year support. The paired weather-effective checkpoint below now
resolves that distinction for its declared local fixtures, without widening the
claim to general EPW environment traversal.
The typed `RunPeriod` holiday, DST, weekend-observation, rain, snow, actual
weather, and first-hour policy fields are intake state. The first-hour policy
is carried by the axis, the weather-file DST use flag gates the parsed EPW
period, and the weather-file holiday use flag gates parsed EPW holiday
definitions without filtering input-file special days. The latter independence
property is unit evidence only because the paired external fixtures contain no
input-file special-day object. The weekend-observation flag participates in the
unit-tested IDF fixed-date special-day projection. Metadata-aware axes reject all
actual-weather and cross-year run periods rather than returning a partial
calendar; their full record traversal is not ported. Rain and snow booleans are
not yet active weather behavior.
The checkpoint also excludes warmup points, design days,
`RunPeriod:CustomRange`, EnergyPlus environment filtering, and EnergyPlus'
invalid/default `Timestep` normalization rules. Those exclusions prevent the
hourly projection from being described as a complete port of
`ManageSimulation` or `GetNextEnvironment`.

## Ordered Exact Hourly and Weather-Effective Leap Evidence Checkpoint

`calendar_schedule_hourly_exact_001` exercises the hourly projection across
the explicit Gregorian range 2016-02-28 through 2016-03-01. It requests an
all-days `Schedule:Compact` profile whose 24 hourly values are 1 through 24,
so the oracle and Rust sides each produce 72 samples spanning the leap day when
its EPW declares `Leap Year Observed=Yes`.

`calendar_schedule_weather_leap_policy_no_001` is the paired negative-policy
case. It uses the same IDF and retains the same 72 raw weather rows; only the EPW
calendar policy changes to `Leap Year Observed=No`. The parsed
`EpwWeatherFile` keeps header metadata and records together, and the hourly
report path applies that metadata through `ResolvedWeatherEnvironmentCalendar`.
The Yes case therefore retains 72 samples and ends on Tuesday March 1, while the
No case skips all 24 February 29 simulation samples and retains 48 samples,
ending on simulation Monday March 1 even though that Gregorian date is Tuesday.
On that no-leap March 1 point the Gregorian day of year is 61, the
weather-effective day of year is 60, and the leap-shaped schedule day of year
remains 61. Runtime unit tests lock those three meanings as separate internal
state against the mapped EnergyPlus assignments. The paired external oracle
gate does not independently prove any of those three day-of-year fields: its current
AllDays/Until schedule depends only on hour, while its normalized timestamp
contains month/day and simulation weekday but not ordinal day.
For a same-year no-leap-policy RunPeriod whose input boundary itself is February
29, EnergyPlus computes duration with non-leap ordinals before weather reading:
February 29 aliases ordinal 60, then the raw February 29 day is discarded and
March 1 supplies that simulation day. Rust mirrors this source rule, so a
February-29-only period has one Gregorian input day, one skipped raw leap date,
and one effective March 1 simulation day; `effective_days` is therefore not
defined as `gregorian_days - leap_days_skipped`. Unit tests also cover February
28 through February 29 becoming February 28 plus March 1. These endpoint tests
are source-mapped internal evidence, not additional external case claims.
This calendar layer feeds the later literal EPW start-record selector described
below. EnergyPlus succeeds for a February 29 start under a no-leap header only
when a raw February 29 row is present to find and discard; without that literal
row it rewinds and terminates. Rust unit tests now lock both outcomes, while the
paired calendar fixtures remain header-policy evidence rather than independent
record-selection evidence.
Both fixtures disable weather-file holidays, DST, weekend observation, rain,
and snow so those unexercised branches cannot be mistaken for evidence from
this pair.

Both cases opt into an `ordered-exact-unique` timestamp contract. Unlike the
existing label-alignment comparator, this contract treats each input slice as
the ordering authority and requires every sample to have a timestamp, every
timestamp on each side to be unique, equal lengths, and exact timestamp-string
equality at every index before the numeric tolerance result can pass. The
blocking gate also locks the first and last labels and zero schedule-value
delta. Existing cases retain their prior order-independent timestamp alignment
unless they explicitly request this contract.
The manifest schema currently permits the option only for hourly schedule or
weather ESO series, the two report boundaries that consume and gate it; other
output families are rejected instead of silently ignoring the declaration.

The exact strings are normalized comparison labels assembled from runtime-owned
calendar fields and the EnergyPlus ESO parser's timestamp fields. This proves
the ordered hourly projection and header-driven leap-policy difference for the
declared pair; it does not prove the raw text emitted by
`OutputProcessor::WriteTimeStampFormatData`, subhourly records, general EPW
record selection or data-period matching, DST, holidays, actual-weather
execution, cross-year traversal, any of the three internal day-of-year fields, warmup, or
the full schedule lookup family.
The weather-aware axis and record selector are also consumed during setup for
weather-required heat-balance `ep_run` classes. The normalized timestamp
comparison labels and schedule-series evidence remain owned by the dedicated
CLI report and gate; their general runtime consumption remains deferred.

## Source-Order EPW Record Selection Checkpoint

`EpwWeatherFile` now retains typed `EpwDataPeriods` metadata with its weather
rows. The parser validates the declared period count, records per hour, start
weekday, and period endpoint dates, including logical header fields continued
onto following physical lines before the first weather row.
`select_epw_environment_weather` consumes a
metadata-aware `TimeAxis`, requires one declared data period to cover both
environment endpoints, and positions the source stream by literal month/day for
the supported non-actual branch. Source years are deliberately ignored on that
branch, matching EnergyPlus' `MatchYear=false` search rule. The selector then
validates one complete hour-1-through-hour-24 source day at a time and returns a
dense environment-order stream rather than assuming that the RunPeriod starts
at EPW row zero.

The selector also locks the narrow source lifecycle needed around that stream.
The accepted first day is initially the logical Tomorrow buffer; each BeginDay
transition commits it to Today and prefetches the next accepted source day only
when another simulation day remains. On the final simulation day, Tomorrow
therefore remains the same source day as Today. A no-leap environment searches
for its literal start first and only then discards a raw February 29 day. Unit
tests cover the present-versus-missing February 29 start, malformed or partial
source days, the last-day no-prefetch rule, restricted full-cycle rewind
eligibility, Today/Tomorrow source-index transitions, and first-hour versus
prior-day-hour-24 interpolation seeds. These are source-mapped internal tests;
they are not additional oracle claims for EnergyPlus' internal buffers or
subhourly values.

Separate unit tests lock a narrow solar-sampling boundary on the dense accepted
hourly stream. `next_solar_weather_record_within_day` wraps each source day's
hour 24 to hour 1 of that same 24-record day, so record indexes 23 and 47 use
indexes 0 and 24 as solar `NextHr` rather than crossing into the following
accepted day. With one zone timestep per hour, the solar weights are
`(previous, current, next) = (0, 1, 0)`, so the current hourly record is used
without interpolation. This is source-mapped internal evidence only: it adds no
external oracle claim and does not prove subhourly solar interpolation, the
other `SetCurrentWeather` fields, complete solar processing, or broad
`WeatherManager` compatibility.

`weather_record_start_offset_nonactual_001` is the external record-selection
gate. Its EPW has 72 rows in source order: 24 June 30 decoy rows followed by the
requested July 1 and July 2 rows. The three source days use 1999, 2004, and 2007
while the RunPeriod uses 2016, so the case distinguishes non-actual month/day
positioning from leading-row selection without claiming year matching. The
gate requires source start index 24, exactly 48 selected hourly rows, exact and
unique ordered normalized timestamps from Friday July 1 hour 1 through Saturday
July 2 hour 24, and zero-tolerance/zero-delta `Site Outdoor Air Drybulb
Temperature` values.

That external case proves only same-year, non-actual, single-data-period,
one-record-per-hour record-date selection, order, timestamp labels, and declared
dry-bulb values. It does not independently prove Today/Tomorrow storage, the
internal day-local hour-24 solar `NextHr` or one-timestep-per-hour
no-interpolation behavior, subhourly weather values, `SetCurrentWeather` field
parity, actual-weather year matching, cross-year traversal, execution across
multiple data periods, records-per-hour greater than one, DST, holidays or
special days, missing-value repair, cyclic multi-year execution, or broad
WeatherManager compatibility.
The selector feeds both the dedicated time/weather report boundary and
`ep_run::prepare_runtime_inputs` for weather-required heat-balance classes.
That arbitrary-run wiring is runtime plumbing, not independent oracle evidence
or a completion claim for any mapped EnergyPlus routine.

## Fixed-Date EPW Daylight-Saving Evidence Checkpoint

`EpwCalendarMetadata` now retains an optional typed
`EpwDaylightSavingPeriod`. Its two `EpwCalendarDateRule` boundaries support the
EnergyPlus EPW forms used by `ProcessDateString`: fixed month/day, an Nth
weekday in a month, and the last weekday in a month. A zero start and zero end
mean that the weather file declares no period; a one-sided zero or an invalid
date rule is rejected rather than silently activating a partial range.
`ProcessDateString` also accepts a nonzero single-number Julian-date form; that
form is not parsed or claimed at this checkpoint.

For a supported same-year metadata-aware axis, the RunPeriod's
`use_weather_file_daylight_saving_period` flag controls whether the EPW period
is active. Month/day validity and final ordinals use the weather-effective year,
while Nth/last-weekday selection preserves EnergyPlus' RunPeriod `MonWeekDay`
projection seeded before the environment-specific `LeapYearAdd`. The resolved
start/end ordinal range is inclusive. The resulting daily state is projected
to both `EnvironmentTimePoint.dst` and hourly `TimePoint.dst`. Unit tests cover
fixed dates, Nth-weekday and last-weekday rules, the leap-policy/`MonWeekDay`
split, an inactive RunPeriod flag, invalid occurrences, and the
southern-hemisphere start-after-end range. The Nth/last-weekday and southern
year-wrap paths are source-mapped unit evidence only.

`calendar_dst_fixed_date_hourly_exact_001` is the sole external DST claim at
this checkpoint. Its EPW declares a fixed February 29 through March 1 period,
and its 2016 RunPeriod explicitly enables the weather-file period. The blocking
zero-tolerance gate requires exactly 72 ordered, unique hourly `Site Daylight
Saving Time Status` samples and normalized timestamps: 24 inactive samples on
February 28 followed by 48 active samples on February 29 and March 1. It also
checks that the shared metadata-aware `TimeAxis` reports the same resolved
period and active-sample count.

This case does not claim the IDF
`RunPeriodControl:DaylightSavingTime` object, DST-shifted schedule lookup,
schedule hour-24/tomorrow rollover, holidays or special days, raw ESO timestamp
serialization, actual-weather execution, cross-year EPW traversal, or broad
`WeatherManager` compatibility.

## Fixed-Date IDF Special-Day Evidence Checkpoint

Typed `RunPeriodControl:SpecialDays` objects now retain their identity, a
`CalendarDateRule`, positive duration, and `SpecialDayType`.
`SpecialDayAxisState` resolves those definitions against the shared
weather-effective calendar, projects an optional special type and effective
EnergyPlus day-type index to both environment and hourly points, and leaves the
underlying simulation weekday intact. Source-mapped unit tests cover
Nth/last-weekday rules, inclusive duration with a same-year annual-table wrap,
overwrite in a directly ordered typed vector, and the fixed single-day weekend
shift; none of those broader paths is promoted by this checkpoint. The current
name-keyed raw-model boundary does not preserve overlapping IDF source order,
so compiled overlap precedence remains explicitly unclaimed. Gregorian
cross-year axes with typed special days fail explicitly until each year's
weekday and leap shape can be reprojected instead of reusing the start year.

`calendar_special_day_fixed_date_hourly_exact_001` is the sole external
input-file special-day claim. It runs the explicit 2016-02-28 through 2016-03-01 calendar
with one fixed IDF February 29 duration-one Holiday. Weather-file holidays,
weekend observation, and DST are explicitly No. The blocking zero-tolerance
gate requires exactly 72 ordered, unique normalized hourly timestamps and
`Site Day Type Index` samples: 24 Sunday values of 1, followed by 24 Holiday
values of 8, followed by 24 Tuesday values of 3.

This case does not claim Nth/last-weekday resolution, duration or annual-table
wrap beyond the declared duration-one date, directly ordered typed-vector
overwrite, weekend shifting, EPW holidays, the RunPeriod weather-file-holiday
use policy, EPW-versus-IDF precedence, schedule day-type lookup, tomorrow
special-day state, raw ESO timestamp serialization, or broad
`WeatherManager`/schedule compatibility.

## Fixed-Date EPW Holiday Use-Policy Evidence Checkpoint

`EpwCalendarMetadata` now retains the ordered holiday names and calendar rules
declared by the EPW `HOLIDAYS/DAYLIGHT SAVINGS` header. When the active
RunPeriod enables weather-file holidays, `SpecialDayAxisState` projects those
definitions before input-file special days. EnergyPlus assigns weather-file
holidays the source-exact Sunday day type and index 1, not the input-file
Holiday index 8. Disabling the RunPeriod policy filters the EPW definitions.

`calendar_epw_holiday_fixed_date_enabled_hourly_exact_001` and
`calendar_epw_holiday_fixed_date_disabled_hourly_exact_001` share one
leap-observed 72-row EPW with one fixed February 29 holiday. Their IDFs differ
only in `Use Weather File Holidays and Special Days`; neither contains a
`RunPeriodControl:SpecialDays` object, and both explicitly disable weekend
observation and DST. The blocking zero-tolerance gate requires exactly 72
ordered, unique normalized timestamps and `Site Day Type Index` samples in
each case:

- enabled: 24 Sunday=1, 24 source-exact EPW-holiday Sunday=1, and 24 Tuesday=3;
- disabled: 24 Sunday=1, 24 Monday=2, and 24 Tuesday=3.

This pair proves only fixed-date EPW holiday intake and RunPeriod enable/disable
filtering. It does not claim weekend shifting, Nth- or last-weekday rules,
multiple or overlapping holidays, EPW-versus-IDF precedence, schedule day-type
lookup, tomorrow special-day state, raw ESO timestamp serialization,
actual-weather execution, cross-year reprojection or traversal, or broad
`WeatherManager`/schedule compatibility. Input-file-special-day independence
from the disabled EPW policy remains unit evidence because these fixtures have
no input-file special day.

## Current Rust Boundary

| Boundary | Current Rust status | Missing source behavior |
|---|---|---|
| run-period input | typed dates, optional years, and start weekday feed EnergyPlus-style year and weekday resolution; the first-hour policy is carried on the axis, the weather-file DST use flag gates a parsed EPW period, the weather-file holiday use flag gates parsed EPW holiday definitions, and typed IDF `RunPeriodControl:SpecialDays` definitions feed the shared day-type projection. Metadata-aware actual-weather/cross-year inputs and Gregorian cross-year inputs with typed special days fail explicitly | custom ranges, design-day environments, environment filtering, overlapping IDF source-order preservation, weekend and multi-holiday behavior beyond unit/source evidence, EPW-versus-IDF precedence, the IDF DST object, actual-weather traversal, cross-year weather traversal, per-year special-day reprojection, and full EnergyPlus warning-text parity |
| canonical calendar | `ResolvedRunPeriodCalendar` retains Gregorian interpretation, while same-year non-actual `ResolvedWeatherEnvironmentCalendar` applies the EPW leap-year header (including the February 29 endpoint ordinal alias). The metadata-aware axis resolves an enabled EPW DST rule into inclusive daily `dst` state, enabled EPW holidays into source-exact Sunday day type, and typed IDF special-day rules into effective `DayType`/`special_day_type`; `EnvironmentTimePoint` separately owns Gregorian, weather-effective, and schedule day-of-year plus simulation weekday. External special-day evidence is limited to the fixed duration-one IDF Holiday and paired fixed EPW holiday use-policy cases | warmup lifecycle, the IDF DST object, overlapping IDF source-order preservation, weekend shifting, Nth/last, multiple or overlapping holidays, EPW-versus-IDF precedence, schedule/timestamp DST or special-day consumers, tomorrow special-day state, actual-weather behavior, cross-year weather traversal or per-year special-day reprojection, EnergyPlus `Timestep` default/invalid-value normalization, and environment kinds beyond weather run periods |
| hourly consumers | `TimeAxis` is an hour-ending projection of the resolved environment calendar; the paired calendar cases lock 72 leap-observed labels ending Tuesday versus 48 no-leap-policy labels ending simulation Monday from the same IDF and 72 raw EPW rows. The fixed-date DST case separately locks 72 ordered state/timestamp samples, 24 inactive then 48 active. The fixed IDF special-day case locks 72 ordered day-type/timestamp samples as 24 each of 1/8/3. The paired EPW holiday cases lock 72 samples each as enabled 24 each of 1/1/3 and disabled 24 each of 1/2/3. Weather-required heat-balance `ep_run` setup now builds the same metadata-aware axis before runtime execution | runtime consumers outside those weather-required heat-balance classes, DST behavior beyond the fixed-date case, special-day behavior beyond the declared fixed-date cases, schedule day-type lookup, tomorrow state, runtime consumption of the precomputed Schedule Value series, and all remaining calendar-dependent output semantics |
| EPW weather | `EpwWeatherFile` keeps parsed leap policy, typed optional DST and holiday rules, `DATA PERIODS` metadata, and `EpwRecord` rows. The dedicated hourly report applies the leap policy and enabled DST/holiday policies and selects a complete same-year non-actual, one-record-per-hour stream by source date. The fixed-date DST case externally locks only its DST state/timestamps; the paired EPW holiday cases lock one fixed holiday enabled as source-exact Sunday index 1 versus disabled as the underlying Monday index 2; the offset case locks 24 decoy rows skipped and 48 dry-bulb rows in exact timestamp/value order. Unit tests separately lock Today/Tomorrow source-index transitions, interpolation seeds, the day-local hour-24 solar `NextHr`, and the one-timestep-per-hour current-only solar branch | IDF DST input, EPW holiday weekend/Nth/last/multiple/overlap/precedence behavior, actual-weather year matching, cross-year traversal, multiple-data-period execution, records-per-hour greater than one, complete Today/Tomorrow value-state parity, missing/range repair, cyclic multi-year execution, weather consumers outside the stated `ep_run` setup, subhourly solar interpolation, and complete `ReadEPlusWeatherForDay`/`SetCurrentWeather`/solar/`WeatherManager` parity |
| schedules | `Schedule:Constant` and an all-days `Schedule:Compact` `Until` subset can produce hourly series; the paired exact cases lock the same 1-through-24 daily profile for 72 versus 48 weather-effective hours | `Through`/`For` day-type expansion, zone-timestep lookup, holiday/DST rollover, full day schedules, EMS current-value semantics, and exact `getHrTsVal` parity |
| output time | hourly consumers use an output-owned normalized comparison label projected from the shared axis; the paired leap-policy schedule cases, record-selection weather case, fixed-date DST case, fixed IDF special-day case, and paired EPW holiday policy cases enforce only their declared ordered, unique, exact normalized labels and variables | raw and exact timestep/hour/day/month/run-period ESO, MTR, and SQL records from `WriteTimeStampFormatData`; DST/day-type serialization and tomorrow-state formatting remain unclaimed |

Existing dry-bulb, dew-point, relative-humidity, pressure, wind, radiation, and
precipitation diagnostics remain useful evidence for individual weather
fields. Only the declared offset case promotes record-date selection and ordered
hourly dry-bulb values; other record-order smoke comparisons do not prove
calendar selection, internal weather-day handoff, or timestamp conformance.

## Promotion Gates After the First Checkpoint

Gates are sequential because each later lookup consumes state owned by the
earlier gate.

1. **Remaining DST gate.** The fixed-date EPW period, RunPeriod use flag, and
   daily/hourly state now have one external exact gate. Promote Nth/last-weekday
   and southern year-wrap rules beyond unit evidence, add the IDF DST object,
   and port the DST-shifted schedule lookup plus hour-24/tomorrow rollover
   consumed by `ScheduleDetailed::getHrTsVal`.
2. **Remaining special-day gate.** The fixed duration-one IDF Holiday and the
   paired fixed EPW holiday use-policy branch now have external exact gates.
   Promote Nth/last-weekday, same-year annual-table duration/wrap, directly
   ordered typed-vector overwrite, weekend rules, and multiple/overlapping
   definitions beyond unit/source evidence; add per-year cross-year
   reprojection and EPW-versus-IDF precedence; then port tomorrow's holiday/day
   type and the weekday-versus-special `DayType` selection used by schedules
   and timestamps.
3. **Remaining EPW environment gate.** Extend the same-year non-actual hourly
   selector into actual-weather year matching, cross-year and multiple-data-period
   execution, records-per-hour greater than one, full Today/Tomorrow value-state
   handoff, multi-year wrap, missing/range rules, and all active
   `InitializeWeather`, `UpdateWeatherData`, and `SetCurrentWeather` fields at
   every zone timestep.
4. **Schedule gate.** Compile full supported schedule objects into the
   EnergyPlus 366-day/day-type/timestep layout, update one `currentVal` per
   zone timestep through `UpdateScheduleVals`, and match `getHrTsVal` across
   weekday, weekend, holiday, DST transition, and hour-24 rollover cases.
5. **Timestamp gate.** Match the time-family ESO/MTR text fields and SQL time
   index produced through `WriteTimeStampFormatData` at timestep, hourly,
   daily, monthly, and run-period boundaries, including warmup suppression or
   tagging rules.

Each gate requires declared oracle fixtures, field-by-field exact or explicit
tolerance policy, and blocking comparison artifacts. A later gate cannot use
script-side timestamp reconstruction to substitute for missing runtime state.

## Claim Boundary and Stop Rule

The calendar spine by itself claims no broad EnergyPlus time, weather, schedule,
or timestamp conformance. The paired exact fixtures claim only their declared
normalized hourly labels and AllDays/Until Schedule Value series: 72 samples
ending Tuesday for `Leap Year Observed=Yes`, and 48 samples ending simulation
Monday for `No`, from the same IDF and 72 raw EPW rows. In particular, those
case-scoped counts, a parsed EPW header or row, or a matching constant schedule
are not evidence that
`ManageSimulation`, `ManageWeather`, `getHrTsVal`, or
`WriteTimeStampFormatData` has been fully ported.

The separate offset fixture adds only exact same-year non-actual source-date
positioning and 48 ordered hourly dry-bulb timestamp/value samples after 24
decoy rows. Its report-visible buffer counts do not turn the internal
Today/Tomorrow, interpolation, day-local hour-24 solar `NextHr`, or
one-timestep-per-hour current-only solar unit evidence into an external state
claim, and it does not complete `ProcessEPWHeader`, `ReadWeatherForDay`,
`ReadEPlusWeatherForDay`, `UpdateWeatherData`, or `SetCurrentWeather`.

The fixed-date DST fixture adds only its EPW February 29 through March 1 period,
the enabled RunPeriod weather-file policy, and 72 ordered normalized hourly
timestamps and `Site Daylight Saving Time Status` values: 24 inactive followed
by 48 active. Nth/last-weekday and southern year-wrap behavior remains unit
evidence. The IDF DST object, schedule DST shift and hour-24 rollover, special
days, and raw ESO timestamp serialization remain outside this claim.

The fixed IDF special-day fixture adds only one February 29 duration-one
Holiday and 72 ordered normalized hourly timestamps and `Site Day Type Index`
values: 24 Sunday=1, 24 Holiday=8, and 24 Tuesday=3. Its weather-file holidays,
weekend observation, and DST flags are explicitly No and do not prove those
policies. Nth/last rules, same-year annual-table duration/wrap beyond the
declared date, directly ordered typed-vector overwrite, and weekend shifting
remain unit/source evidence; compiled IDF overlap precedence and cross-year
reprojection remain unclaimed.

The paired fixed EPW holiday fixtures add only one February 29 weather-file
holiday and the RunPeriod use-policy toggle. Their 72 ordered normalized hourly
timestamps and `Site Day Type Index` values are exactly 24 each of 1/1/3 when
enabled and 1/2/3 when disabled. The enabled middle day is EnergyPlus'
source-exact EPW-holiday Sunday index 1, not input-file Holiday index 8.
Weekend shifting, Nth/last rules, multiple or overlapping holidays,
EPW-versus-IDF precedence, schedule day-type lookup, tomorrow special-day
state, raw ESO timestamp serialization, and cross-year behavior remain outside
this claim.

Promotion requires all of the following on the same canonical axis:

- exact environment/day/hour/zone-timestep ordering, including warmup and
  environment boundaries where claimed;
- exact leap-year, DST, holiday, special-day, tomorrow-rollover, and day-type
  state for the declared cases;
- EPW selection and current/tomorrow weather values aligned by timestamp;
- exact schedule values at every declared zone timestep; and
- exact time-family timestamps and reporting-frequency boundary rows.

Until those gates pass, broader results remain foundation, smoke, or diagnostic
evidence only; the declared 72-label/48-label pair keeps only the narrow
normalized hourly Schedule Value conformance boundary stated above, and the
offset case keeps only its narrow non-actual ordered record-date/dry-bulb
boundary, the fixed-date DST case keeps only its 72-sample boundary, and the
fixed IDF special-day case keeps only its ordered 72-sample 1/8/3 boundary. The
paired EPW holiday cases keep only their ordered 72-sample enabled 1/1/3 and
disabled 1/2/3 boundaries.
Their consumption by weather-required heat-balance `ep_run` setup adds no
independent conformance evidence. Record selection beyond the offset case, DST
behavior beyond the fixed-date case, special-day behavior beyond the fixed IDF
and paired fixed EPW cases, EPW-versus-IDF precedence, schedule day-type lookup,
tomorrow special-day state, weather consumers outside the stated setup, raw ESO
output, actual-weather execution, cross-year traversal, multiple-data-period
execution, records-per-hour greater than one, subhourly solar interpolation,
and complete `SetCurrentWeather`/solar/`WeatherManager` conformance remain
explicitly deferred.
