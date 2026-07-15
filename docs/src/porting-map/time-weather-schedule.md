---
status: active
claim_level: planning-guard
owner: runtime
last_reviewed: 2026-07-15
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
     -> Weather::SetDSTDateRanges
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
| daylight-saving object intake | `Weather::GetDSTData` | Parses at most one `RunPeriodControl:DaylightSavingTime` start/end rule pair after EPW metadata intake. When present, this input-file declaration replaces the weather-file source independently of the RunPeriod weather-file-use flag. Missing, malformed, and duplicate diagnostics remain typed-only evidence except for the exact successful precedence fixture below. |
| environment materialization | `Weather::SetupEnvironmentTypes` | Copies each run period into `Environment`, derives `StartJDay`, `EndJDay`, `RawSimDays`, `TotalDays`, leap-year handling, environment kind/name, weekday map seed, and the run-period policy flags. This is descriptor construction, not current-day state. |
| environment selection | `Weather::GetNextEnvironment` | Advances `Envrn`, selects the descriptor, and seeds `KindOfSim`, `CalendarYear`, month/day/day-of-year, `NumOfDayInEnvrn`, `CurEnvirNum`, and environment name. For a weather run it resolves `CurrentYearIsLeapYear`, weekday tables, active DST ranges, special-day dates, and the effective weather policy switches. |
| daylight-saving range projection | `Weather::SetDSTDateRanges` | Resolves the selected input-file or weather-file date rules against the environment calendar and writes the inclusive active range consumed by daily `DSTIndicator` state. The exact fixed-date input-file-over-EPW case, the same-year EPW-only fixtures, and the single cross-year start-year-projection fixture below are narrow external evidence, not full routine completion. |
| special-day projection | `Weather::SetSpecialDayDates` | Resets the annual special-day table and resolves enabled definitions against the environment weekday/leap shape before weather-day reads consume the resulting day type. External numerical evidence is limited to the fixed IDF case, the paired common-/leap-year duration-three annual-table wraps, the paired fixed-date overlap/source-order case, two exact 2032 IDF weekday-rule forms, the fixed-Sunday Yes/No/blank and fixed-Saturday Yes/No weekend-rule cases, the fixed EPW use-policy pair, and the exact two-rule EPW weekday case below. One blocking smoke/nonclaim case separately locks rejection of an explicit nonexistent 2016 fifth Sunday; all other date forms, durations, ordering, weekend, and EPW branches remain unit/source evidence or unclaimed. |
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
  reject actual-weather traversal. Non-actual cross-year traversal is state-
  backed, but external evidence is limited to the single-boundary, single-DATA-
  PERIOD fixture below.
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

Gregorian-only axes and metadata-aware axes without an active daylight-saving
source keep `dst=false`. Metadata-aware axes resolve the parsed EPW period only
when the RunPeriod enables it. A typed `RunPeriodControl:DaylightSavingTime`
object retains its `CalendarDateRule` pair and takes precedence over the EPW
source independently of that flag; `DaylightSavingAxisState` separately retains
weather-file declaration, RunPeriod use, input-file declaration, active state,
effective source, and resolved range. Typed `RunPeriodControl:SpecialDays`
input now shares the same calendar projection:
`CalendarDateRule` retains fixed, Nth-weekday, and last-weekday rules;
`SpecialDayType` retains the five EnergyPlus special schedule types; and
`SpecialDayAxisState` resolves enabled EPW holidays before typed definitions in
their compiled vector order into the effective `DayType` and optional
`special_day_type` carried by both environment and hourly points. EPW holidays
retain their source-exact Sunday day type. The simulation-effective weekday remains separately owned;
it equals the Gregorian weekday on ordinary paths but does not advance across
a February 29 skipped by EPW policy.

Source-mapped unit tests cover Nth/last resolution beyond the two explicit IDF
forms below, inclusive duration/wrap combinations beyond the paired exact
common-/leap-year cases, and directly ordered typed-vector overwrite. Those
broader branches are not external conformance evidence. The fixed duration-one
IDF Holiday, the paired duration-three December 31 wraps, the explicit 2032
`4th Sunday in February` and `Last Sunday in February` IDF cases, the
fixed-Sunday Yes/No/blank and fixed-Saturday Yes/No weekend-rule cases, the paired
fixed EPW holiday use-policy cases, the exact two-rule EPW weekday case below,
the paired fixed-date overlap/source-order cases, and the exact start-year
annual-table projection across one non-actual year boundary are the only
external numerical special-day boundaries.
The explicit nonexistent-fifth-Sunday fixture below is blocking negative
evidence only and produces no day-type samples.

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
shared IDF fixed-date special-day projection; the external cases below lock its
fixed-Sunday plus-one-day explicit-Yes and executable-observed blank branches,
the fixed-Sunday explicit-No branch, and the fixed-Saturday plus-two-day
explicit Yes/No branches. Metadata-aware axes reject actual-weather runs.
Non-actual cross-year traversal is externally locked only for the two isolated
2031-12-30 through 2032-01-02 fixtures below: one special-day projection and one
EPW daylight-saving projection. Multiple boundaries, later annual resets or
reprojection, multiple DATA PERIODS, and February 29 coupling remain unclaimed.
Rain and snow booleans are not yet active weather behavior.
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
weather-required heat-balance `ep_run` classes. For that pair, the normalized timestamp
comparison labels and schedule-series evidence remain owned by the dedicated
CLI report and gate; their general runtime consumption remains deferred.

## Schedule:Compact Through/For Day-Type Evidence Checkpoint

`calendar_schedule_compact_through_for_day_type_hourly_exact_001` adds one
calendar-aware `Schedule:Compact` boundary. Its non-actual RunPeriod is exactly
2031-12-30 through 2032-01-03, starts on Tuesday, and contains one input-file
`1st Thursday in January` Holiday that resolves to January 2. The fixture has
one wrapping DATA PERIOD, one record per hour, no EPW holidays or daylight-
saving range, and no input-file daylight-saving object. Rust EPW record
selection is deliberately null and unclaimed because this is a schedule-only
comparison.

The compact schedule retains source order across two fixed Month/Day `Through`
periods. The first ends `1/1` and assigns Thursday=105 before
AllOtherDays=199. The second ends `12/31` and assigns Tuesday=103,
Wednesday=104, and Holiday=108 before AllOtherDays=199. Each profile has one
hourly `Until: 24:00` endpoint. The compiler expands the declared `For` tokens
and the period-local source-order AllOtherDays complement into typed day
profiles; the time-axis consumer selects a period from
`schedule_day_of_year`, a profile from the resolved day type, and a value from
the hour-ending interval.

The blocking zero-tolerance gate proves exactly 120 ordered, unique
`Schedule Value` samples and timestamps. Daily values are
`103/104/105/108/199`, each repeated for exactly 24 hours. It separately locks
all 120 raw EnergyPlus schedule values, all 120 raw day-type values
`3/4/5/8/7`, their ESO timestamp fields, the exact EnergyPlus `Environment`,
disabled `Environment:Daylight Saving`, and input-file
`Environment:Special Days` EIO rows, and clean completion with 0 Warning and 0
Severe errors. The raw day-type series is oracle evidence used by this gate;
only `Schedule Value` is promoted in the case manifest.

Hour-only downstream paths do not reinterpret a calendar-varying compact
schedule. Runtime tests lock `InvalidInternalGainSchedule` for calendar-varying
or unresolved `OtherEquipment` schedule references at the public gain-trace,
heat-balance initialization, and first-zone simulation boundaries. This is a
fail-closed safety contract and unit evidence, not external conformance for
internal-gain, HVAC, or IdealLoads calendar consumption.

Rust schedule-day ordinals `365/366/1/2/3` are source-mapped summary and unit-
test diagnostics, not fields emitted by EnergyPlus ESO or EIO. This checkpoint
does not promote `calendar_time_state` beyond scaffold and does not complete
`Sched::UpdateScheduleVals`. DST-shifted schedule-clock lookup and hour-24
tomorrow rollover, subhourly lookup or interpolation, other schedule families,
additional Through periods, RunPeriods, or year boundaries, actual weather,
design-day or warmup execution, internal-gain/HVAC/IdealLoads calendar-aware
consumption, Rust raw ESO parity, and broad EnergyPlus warning/error parity
remain outside the claim.

## Schedule:Compact DST Hour-24/Tomorrow Day-Type Evidence Checkpoint

`calendar_schedule_dst_hour24_tomorrow_day_type_exact_001` isolates the
EnergyPlus 26.1 detailed-schedule clock over only the non-actual 2032-10-30
through 2032-11-01 RunPeriod. The EPW `Last Sunday in October` through `Last
Sunday in March` range is enabled, one input-file November 1 Holiday is
resolved, and the three raw daily states are DST `0/1/1` and day types
Saturday/Sunday/Holiday (`7/1/8`). The fixture has one DATA PERIOD and one
record per hour. Rust EPW record selection is deliberately null and unclaimed
because this is a schedule-only comparison.

The zero-tolerance gate proves exactly 72 ordered, unique `Schedule Value`
samples and timestamps. Their source-exact order is `100` for 23 hours then
`124`; `200` for 23 hours then `801`; and `800` for 23 hours then `901`.
October 30 is not in DST, so hour 24 remains on the current schedule clock and
returns `124`. On October 31, current DST advances reported hours 1 through 23
to detailed-schedule hours 2 through 24, and reported hour 24 advances to the
November 1 schedule ordinal and consumes the tomorrow Holiday profile at hour
1, returning `801`. On final-run November 1, the same +1-hour shift returns
`800` for the first 23 samples; hour 24 advances the schedule ordinal to
November 2 while EnergyPlus retains the final Holiday `TomorrowVariables` day
type, so the later Holiday profile's hour-1 value is `901`.

The blocking gate separately locks all 72 raw EnergyPlus schedule, daylight-
saving, and day-type ESO values and timestamp fields; the exact
`Environment`, weather-file `Environment:Daylight Saving`, and input-file
`Environment:Special Days` EIO rows; and clean completion with 0 Warning and 0
Severe errors. Only `Schedule Value` is promoted by the manifest; raw DST and
day type are oracle evidence used to prove the schedule selection sequence.

Rust `detailed_schedule_lookup_state` consumes the current point's DST state.
For shifted hours above 24 it advances the leap-shaped schedule ordinal and
uses the time axis's tomorrow day type. The time axis carries the next day's
weekday/special type while a day remains and retains the final day's type at
the no-prefetch environment boundary. This source-exact checkpoint does not
promote `calendar_time_state` beyond scaffold and does not complete
`Sched::UpdateScheduleVals` or EMS/current-value policy.

Schedule-specific DST opt-out, subhourly values or interpolation, other
schedule families, additional RunPeriods or DATA PERIODS, other DST/holiday
boundaries, year-end schedule-ordinal wrap beyond unit evidence, actual-
weather, design-day or warmup execution, internal-gain/HVAC/IdealLoads
calendar-aware consumption, Rust raw ESO parity, and broad EnergyPlus
warning/error or `getHrTsVal` parity remain outside the claim.

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

## EPW Daylight-Saving Evidence Checkpoints

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
southern-hemisphere start-after-end range.

`calendar_dst_fixed_date_hourly_exact_001` and
`calendar_dst_fixed_date_disabled_hourly_exact_001` form the fixed-date
RunPeriod-policy pair. They share the exact EPW February 29 through March 1
declaration and differ only in the explicit weather-file daylight-saving use
flag as `Yes` versus `No`.

The blocking pair gate requires 72 ordered, unique, zero-tolerance hourly
`Site Daylight Saving Time Status` samples per case. Enabled is daily status
`0/1/1`, with 48 active hours; disabled is `0/0/0`, with zero active hours.
The pair gate locks exact EnergyPlus 26.1 `Environment` and
`Environment:Daylight Saving` EIO rows; the disabled daylight-saving row is
`Environment:Daylight Saving,No,RunPeriod Object`. The Rust summary still
reports `weather_file_period_declared=true`, but `active=false` and
`resolved_period=null`. Those Rust fields are summary diagnostics, not
additional EIO output.

This pair claims only that fixed EPW declaration and explicit Yes/No policy
difference. Other periods, years, date rules, and policy combinations beyond
the separate exact precedence case, schedule or civil-clock effects,
actual-weather execution, Rust raw ESO serialization, and broad
`WeatherManager` behavior remain unclaimed. The original single-case command
remains available for compatibility; promotion uses the pair gate.

`calendar_dst_epw_idf_precedence_hourly_exact_001` adds the one promoted
input-file precedence boundary. It uses the exact 2016-02-28 through
2016-03-01 RunPeriod with `Use Weather File Daylight Saving Period=No`, reuses
the EPW fixed 2/29 through 3/1 declaration, and contains exactly one
`RunPeriodControl:DaylightSavingTime` object declaring 2/28 through 2/29.
The input-file object wins independently of the RunPeriod flag.

The blocking zero-tolerance gate requires 72 ordered, unique EnergyPlus ESO
values and timestamp fields in daily status `1/1/0`, exactly 48 active hours,
and effective source `input-file`. It locks the exact EnergyPlus 26.1 rows
`Environment,DST FIXED DATE RUN PERIOD,WeatherFileRunPeriod,02/28/2016,03/01/2016,Sunday,3,Use RunPeriod Specified Day,No,No,No,No,No,Clark and Allen`
and `Environment:Daylight Saving,Yes,InputFile,02/28,02/29` plus clean
0 Warning/0 Severe completion.

Rust retains typed `RunPeriodDaylightSavingTime` state; JSON exposes
`input_file_period_declared` and `effective_source`, while Markdown exposes
`input_file_daylight_saving_period_declared` and
`daylight_saving_effective_source`. Existing weather-file declaration,
RunPeriod-use, active, resolved-period, and active-sample fields remain. These
are summary diagnostics, not additional EnergyPlus EIO output. Other fixed,
Nth/last, or wrap dates; missing, malformed, or duplicate error parity beyond
typed diagnostics; other years, multiple RunPeriods, policy combinations,
schedule-shift/hour-24/tomorrow/civil-clock effects, actual or cross-year
weather, DesignDay environments, Rust raw ESO serialization, and broad
`WeatherManager` behavior remain outside this claim.

`calendar_epw_dst_weekday_rules_hourly_exact_001` adds one literal EPW
start/end boundary pair. Its exact header is
`HOLIDAYS/DAYLIGHT SAVINGS,Yes,4th Monday in February,Last Wednesday in February,0`.
The explicit 2032-02-22 through 2032-02-26 RunPeriod starts Sunday and sets its
six calendar/weather policies to `No/Yes/No/No/No/No`, enabling only the
weather-file daylight-saving period. It contains neither
`RunPeriodControl:SpecialDays` nor
`RunPeriodControl:DaylightSavingTime`.

The start resolves to Monday 2032-02-23, day of year 54, and the end resolves
to Wednesday 2032-02-25, day of year 56, without year wrap. The blocking gate
requires 120 ordered, unique, zero-tolerance hourly samples with daily status
`0/1/1/1/0` and exactly 72 active hours. It also locks the conformance/weather/
hourly/ESO/timestamp series metadata, all raw EnergyPlus oracle ESO timestamp
fields and values, the exact EIO row
`Environment:Daylight Saving,Yes,WeatherFile,02/23,02/25`, and exactly
0 Warning and 0 Severe errors.

The weekday-rule checkpoint claims only that literal fourth-Monday start and
last-Wednesday end pair on the explicit 2032 calendar. Other Nth/last-weekday
forms, RunPeriod disabling beyond the paired fixed-date Yes/No case, malformed
headers, and policy precedence beyond the exact fixed-date input-file case
remain unit/source evidence or unclaimed.

`calendar_epw_dst_southern_wrap_hourly_exact_001` adds the literal EPW pair
`Last Sunday in October` through `Last Sunday in March`. On the 2032 calendar,
the EnergyPlus EIO row resolves those dates to October 31 and March 28. The Rust
summary separately locks day-of-year 305 and 88 plus `wraps_year=true` against
source-mapped expected values; those three diagnostics are not EIO-emitted
fields. The RunPeriod still executes only the same-year 2032-03-27 through
2032-03-29 end-side window, with all six policies set to
`No/Yes/No/No/No/No` and no input-file special-day or DST object.

The blocking zero-tolerance gate requires 72 ordered, unique hourly samples in
daily status `1/1/0`, exactly 48 active and 24 inactive hours. It locks the
promoted series metadata, every raw EnergyPlus oracle ESO timestamp field and
value, the exact EIO row
`Environment:Daylight Saving,Yes,WeatherFile,10/31,03/28`, and clean 0 Warning/
0 Severe completion. This proves neither cross-year RunPeriod or weather-record
traversal, full-year range execution, execution around the October start
boundary, generic southern-hemisphere or year-wrap behavior, nor other date-rule
pairs.

`calendar_epw_dst_southern_wrap_start_hourly_exact_001` adds the inclusive
October 31 start boundary for that same literal EPW pair. The RunPeriod executes
only the same-year 2032-10-30 through 2032-11-01 start-side window. Its blocking
zero-tolerance gate requires 72 ordered, unique EnergyPlus ESO values and
timestamp fields in daily status `0/1/1`, exactly 48 active and 24 inactive
hours, plus the exact EIO row with dates 10/31 and 03/28.

The Rust summary again locks day-of-year 305 and 88 plus `wraps_year=true`
against source-mapped expected values; those diagnostics are not emitted by
EIO. This checkpoint does not add full-year or cross-year traversal evidence,
and the March end boundary remains the separate end-side checkpoint. Broader
years, months, date rules, disabling beyond the paired fixed-date Yes/No case,
precedence beyond the exact fixed-date input-file case, DST clock effects,
actual-weather execution, and broad `WeatherManager` behavior remain unclaimed.

Across these six DST checkpoints, input-file daylight-saving evidence is
limited to that one fixed-date precedence case. Other input-file objects and
source/policy combinations, DST-shifted schedule lookup and
hour-24/tomorrow rollover, holidays or special days, Rust raw ESO
serialization, actual-weather execution, cross-year EPW traversal beyond the
single fixture below, DesignDay environments, and broad `WeatherManager`
compatibility remain unclaimed.

## Cross-Year EPW DST Start-Year Projection Evidence Checkpoint

`calendar_epw_dst_cross_year_start_year_projection_hourly_exact_001` uses only
the explicit non-actual 2031-12-30 through 2032-01-02 RunPeriod with Tuesday as
its start weekday. Its single wrapping, one-record-per-hour DATA PERIOD declares
`1st Thursday in January` through `1st Friday in January` as the EPW
daylight-saving range, enables that range in the RunPeriod, contains no
input-file daylight-saving or special-day object, and has no holidays.

EnergyPlus 26.1 resolves the Nth-weekday pair against the 2031 environment-start
annual table as January 2 through January 3. The blocking zero-tolerance gate
locks exactly 96 ordered, unique `Site Daylight Saving Time Status` values and
timestamps in daily order `0/0/0/1`, with 24 active and 72 inactive hours. It
also locks all 96 raw EnergyPlus ESO values and hourly timestamp rows, the exact
rows
`Environment,CROSS YEAR DST START YEAR RUN PERIOD,WeatherFileRunPeriod,12/30/2031,01/02/2032,Tuesday,4,Use RunPeriod Specified Day,Yes,No,No,No,No,Clark and Allen`
and
`Environment:Daylight Saving,Yes,WeatherFile,01/02,01/03`, and clean
0 Warning/0 Severe completion.

EnergyPlus 26.1 source separately maps the January 2 result to
`ReadEPlusWeatherForDay` prefetch before the later January 1 annual-table
reset; that internal buffer order is not an ESO/EIO-emitted field. This
checkpoint does not claim January 3 or later annual reset/reprojection,
multiple year boundaries or DATA PERIODS, actual weather,
DST-shifted schedule-clock lookup, hour-24/tomorrow semantics, February 29
coupling, warmup lifecycle parity, records per hour above one, or broad
`WeatherManager`/schedule compatibility.

## Fixed-Date IDF Special-Day Evidence Checkpoint

Typed `RunPeriodControl:SpecialDays` objects now retain their identity, a
`CalendarDateRule`, positive duration, and `SpecialDayType`.
`SpecialDayAxisState` resolves those definitions against the shared
weather-effective calendar, projects an optional special type and effective
EnergyPlus day-type index to both environment and hourly points, and leaves the
underlying simulation weekday intact. Source-mapped unit tests cover calendar-
rule combinations beyond the promoted fixtures, duration/wrap combinations
beyond the paired exact common-/leap-year cases, and overwrite in a directly
ordered typed vector. External promotion is limited to the fixture boundaries
enumerated above and in the checkpoints below: fixed-date and duration-wrap
projection, the exact IDF and EPW weekday forms, the narrow weekend-policy
cases, fixed EPW policy, the single cross-year start-year projection, and the
declared source-order/precedence cases. The nonexistent fifth-Sunday fixture
remains blocking negative nonclaim evidence.
IDF-backed runs now retain a validated declaration-
order overlay for `RunPeriodControl:SpecialDays` while keeping the converted
epJSON object map name-keyed. The compiler consumes that overlay, so sequential
annual-table projection preserves IDF order and a later overlapping definition
overwrites an earlier one. This recovery is deliberately limited to
`RunPeriodControl:SpecialDays`; native epJSON remains in canonical name order,
and declaration order for every other object type remains unclaimed. For the
single promoted non-actual cross-year path, the annual special-day table is
resolved once from the environment start year and retained across the boundary,
matching EnergyPlus 26.1. Later annual resets or per-year reprojection are not
claimed.

`calendar_special_day_fixed_date_hourly_exact_001` is the baseline external
input-file special-day claim. It runs the explicit 2016-02-28 through 2016-03-01 calendar
with one fixed IDF February 29 duration-one Holiday. Weather-file holidays,
weekend observation, and DST are explicitly No. The blocking zero-tolerance
gate requires exactly 72 ordered, unique normalized hourly timestamps and
`Site Day Type Index` samples: 24 Sunday values of 1, followed by 24 Holiday
values of 8, followed by 24 Tuesday values of 3.

This case by itself does not claim Nth/last-weekday resolution, duration or
annual-table wrap beyond the declared duration-one date, directly ordered
typed-vector overwrite, weekend shifting, EPW holidays, the RunPeriod
weather-file-holiday use policy, EPW-versus-IDF precedence, schedule day-type
lookup, tomorrow special-day state, raw ESO timestamp serialization, or broad
`WeatherManager`/schedule compatibility.

## Single-Boundary Cross-Year Start-Year Projection Evidence Checkpoint

`calendar_special_day_cross_year_start_year_projection_hourly_exact_001`
uses only the explicit non-actual 2031-12-30 through 2032-01-02 RunPeriod,
declares Tuesday as its start weekday, and contains one input-file
`1st Thursday in January` duration-one Holiday. Its EPW contains one wrapping
`DATA PERIODS,1,1,Data,Tuesday,12/30,1/2` header, one record per hour, exactly
96 hourly rows over four source days, Leap Year Observed=Yes, and no EPW holiday
or daylight-saving declaration. Because Treat Weather as Actual is No, the EPW
row years are not promoted as actual-weather year-matching evidence.

EnergyPlus 26.1 resolves that Nth-weekday rule once against the 2031
environment-start annual table. January 2 is the first Thursday in that table,
and the resolved ordinal is retained across this one boundary instead of being
reprojected to Gregorian 2032 January 1. The blocking zero-tolerance gate locks
exactly 96 ordered, unique `Site Day Type Index` values and timestamps in daily
order Tuesday=3, Wednesday=4, Thursday=5, Holiday=8, including 24 January 2
Holiday samples. It also locks all 96 raw EnergyPlus ESO values and hourly
timestamp rows, the exact EnergyPlus 26.1 EIO rows
`Environment,CROSS YEAR SPECIAL DAY RUN PERIOD,WeatherFileRunPeriod,12/30/2031,01/02/2032,Tuesday,4,Use RunPeriod Specified Day,No,No,No,No,No,Clark and Allen`
and
`Environment:Special Days,CROSS YEAR NEW YEAR HOLIDAY,Holiday,InputFile,01/02,  1`,
and clean 0 Warning/0 Severe completion.

Rust summary fields separately lock calendar years 2031 through 2032, four
Gregorian and weather-effective days, one selected DATA PERIOD, 96 selected
records, four day-buffer transitions, and one resolved January 2/day-of-year 2
Holiday with 24 active samples. Those diagnostics are not additional fields
emitted by either EnergyPlus EIO row.

This checkpoint proves only that single non-actual December-to-January
transition and environment-start-year annual-table retention. Actual weather,
multiple year boundaries or DATA PERIODS, later annual resets or per-year
reprojection, DST cross-year semantics, February 29 coupling, EPW holidays,
weekend shifting, duration wrap, overlaps, warmup lifecycle parity, records per
hour greater than one, broad `WeatherManager` behavior, and broad schedule
behavior remain unclaimed.

## Same-Year Special-Day Duration-Wrap Evidence Checkpoint

`calendar_special_day_duration_wrap_common_year_hourly_exact_001` and
`calendar_special_day_duration_wrap_leap_year_hourly_exact_001` each use an
explicit January 1 through January 3 same-year RunPeriod and one fixed December
31 duration-three input-file Holiday. These fixtures exercise the cyclic annual
special-day table; neither RunPeriod crosses a year boundary. Weather-file
holidays, daylight saving, weekend observation, rain, snow, and actual-weather
handling are explicitly No. Their independent 72-row EPWs declare no holidays
or daylight-saving rules and set the matching common- or leap-year shape.

EnergyPlus `Weather::SetSpecialDayDates` increments from the resolved start
ordinal through the inclusive duration loop. The common-year source branch
maps `JDay1 == 366 && LeapYearAdd == 0` back to ordinal 1, while the leap-year
source branch maps `JDay1 == 367` back to ordinal 1. Rust's shared
`wrap_ordinal` projection applies the equivalent cyclic annual-table rule. The
blocking `compare-calendar-special-day-duration-wrap-exact` gate requires exact
resolved metadata and 72 ordered, unique, zero-tolerance normalized timestamps
and `Site Day Type Index` samples per case:

- common-year 2017 resolves December 31 to day of year 365 and produces daily
  indices `8/8/3`: 48 Holiday=8 samples followed by 24 Tuesday=3 samples;
- leap-year 2016 resolves December 31 to day of year 366 and produces daily
  indices `8/8/1`: 48 Holiday=8 samples followed by 24 Sunday=1 samples.

This checkpoint proves only one fixed December 31 duration-three Holiday on the
common-year and leap-year source branches of the same-year cyclic annual table.
It does not claim actual-weather execution, non-actual cross-year traversal
beyond the exact start-year fixture, or per-year reprojection;
overlap, precedence, declaration order, or warning parity; any other duration,
date, special-day type, policy value, or EPW calendar rule; schedule day-type
lookup; tomorrow special-day state; raw ESO timestamp serialization; or broad
`WeatherManager`/schedule compatibility.

## Overlapping IDF Special-Day Declaration-Order Evidence Checkpoint

`calendar_special_day_overlap_zulu_then_alpha_hourly_exact_001` and
`calendar_special_day_overlap_alpha_then_zulu_hourly_exact_001` share the
explicit 2016-02-28 through 2016-03-01 RunPeriod, the same leap-observed 72-row
EPW without holidays or daylight saving, and two duration-one February 29
`RunPeriodControl:SpecialDays` definitions: `Zulu Holiday Definition` has
Holiday index 8 and `Alpha Custom Day Definition` has CustomDay1 index 11.
Their IDFs differ only in those two declarations' source order; all RunPeriod
weather/calendar policy values are explicitly No.

The blocking `compare-calendar-special-day-overlap-order-exact` gate requires
72 ordered, unique, zero-tolerance normalized timestamps and `Site Day Type
Index` samples per case. It also locks the compiler's resolved definition order
before checking the later-definition overwrite:

- Zulu then Alpha resolves in that order, so later Alpha wins on February 29
  and the daily indices are `1/11/3`;
- Alpha then Zulu resolves in that order, so later Zulu wins on February 29 and
  the daily indices are `1/8/3`.

This checkpoint proves only original IDF declaration-order recovery for
`RunPeriodControl:SpecialDays` and later-definition overwrite when exactly two
duration-one definitions target the same fixed date. It does not claim native
epJSON declaration order, declaration order for another object type, more than
two definitions, partially overlapping durations, other dates or special-day
types, weekend interaction, EPW-versus-IDF precedence, actual-weather
execution, non-actual cross-year execution beyond the exact start-year fixture,
or EnergyPlus warning text and repetition parity.

## Nth/Last-Weekday IDF Special-Day Evidence Checkpoint

`calendar_special_day_nth_weekday_hourly_exact_001` and
`calendar_special_day_last_weekday_hourly_exact_001` share the explicit
2032-02-22 through 2032-03-01 calendar, a leap-observed EPW without holidays or
daylight saving, one duration-one input-file Holiday, and an explicit
`Apply Weekend Holiday Rule=Yes`. Both rules resolve on Sunday, so the weekend
shift is exactly zero.

- The exact IDF form `4th Sunday in February` resolves to 2032-02-22, day of
  year 53. Its 216 ordered, exact, unique, zero-tolerance hourly samples have
  daily indices `8/2/3/4/5/6/7/1/2`.
- The exact IDF form `Last Sunday in February` resolves to leap day 2032-02-29,
  day of year 60. Its 216 corresponding samples have daily indices
  `1/2/3/4/5/6/7/8/2`.

This checkpoint claims only those two literal IDF forms on that explicit 2032
calendar. Other IDF Nth ordinals, weekdays, months, years, or date strings
remain outside its numerical claim. The one explicit nonexistent fifth-Sunday
rejection is covered only by the blocking smoke/nonclaim checkpoint below. The
separate EPW checkpoint covers only its literal fourth-Monday and last-Wednesday
header rules. Durations or wraps beyond the paired exact common-/leap-year
cases, overlap, precedence, declaration order beyond that one EPW header,
non-actual cross-year execution beyond the exact start-year fixture or per-year
reprojection, schedule lookup, tomorrow state, and raw ESO timestamp
serialization remain outside the claim.

## Nonexistent Fifth-Weekday Expected-Failure Checkpoint

`calendar_special_day_nonexistent_fifth_weekday_failure_001` is blocking
smoke/nonclaim negative evidence for only one duration-one input-file Holiday
using the exact `5th Sunday in February` form over 2016-02-28 through
2016-03-01. February 2016 has only four Sundays. `Apply Weekend Holiday Rule`
is explicitly Yes, but weekend observation applies only after a date resolves
and therefore does not rescue this nonexistent Nth rule.

The blocking `compare-calendar-special-day-nonexistent-nth-error` gate locks
two engine-specific failure contracts before hourly data is produced:

- EnergyPlus exits 1 through the `SetSpecialDayDates` not-enough-Nths Severe
  followed by Fatal path. Its summary has 0 counted warnings and 1 severe
  error, and its data-row count is zero. The locked messages identify
  `Special Day Date, Nth Day of Month, not enough Nths` for
  `MISSING FIFTH SUNDAY HOLIDAY`, followed by the `SetSpecialDayDates`
  program-termination condition.
- The Rust arbitrary-run CLI exits 6 with diagnostic phase `runtime`, code
  `RuntimeConvergenceFailure`, and the exact diagnostic `failed to build
  weather-aware time axis: run period MISSING FIFTH SUNDAY RUN PERIOD special
  day MISSING FIFTH SUNDAY HOLIDAY has no occurrence 5 of Sunday in month 2`.
  It produces zero hourly samples.

The different numeric exit statuses are each interface contracts, not values
that are expected to equal one another. This smoke case makes no numerical
`Site Day Type Index` conformance claim. Other ordinals, weekdays, months,
years, or strings; EPW rules; multiple errors or definitions; partial-artifact
parity; weekend behavior beyond this explicit non-rescue; duration greater than
one; overlap, precedence, declaration order, year wrap, cross-year
reprojection, schedule lookup, tomorrow state, and successful-run raw ESO
serialization remain outside the claim.

## Fixed-Sunday IDF Weekend-Observation Policy Evidence Checkpoint

`calendar_special_day_weekend_rule_enabled_hourly_exact_001`,
`calendar_special_day_weekend_rule_disabled_hourly_exact_001`, and
`calendar_special_day_weekend_rule_blank_hourly_exact_001` use the same
explicit 2016-02-28 through 2016-03-01 calendar, the same fixed MonthDay 2/28
duration-one input-file Holiday, and the same leap-observed EPW without
weather-file holidays. Their IDFs differ only in the `Apply Weekend Holiday
Rule` A5 field: explicit Yes, explicit No, or genuinely blank. Weather-file
holidays, DST, rain, snow, and actual-weather policies are explicit No in all
three fixtures. The blocking zero-tolerance gate requires ordered, exact, unique
normalized timestamps and 72 `Site Day Type Index` samples in each case:

- Yes observes the Sunday Holiday on Monday 2/29: 24 Sunday=1, 24 Holiday=8,
  and 24 Tuesday=3;
- No leaves the Holiday on Sunday 2/28: 24 Holiday=8, 24 Monday=2, and 24
  Tuesday=3.
- blank follows the EnergyPlus 26.1.0 executable's enabled branch and exactly
  matches explicit Yes: 24 Sunday=1, 24 Holiday=8, and 24 Tuesday=3.

EnergyPlus 26.1.0's `Energy+.idd` and epJSON schema both declare A5's default as
`No`, but the executable comparison resolves the blank IDF field as
`apply_weekend_rule=true`, shifts the duration-one Sunday Holiday to Monday
2/29 (day of year 60, shift 1), and produces values and timestamps identical to
explicit Yes. This checkpoint records that executable-observed mismatch; it
does not reinterpret the published IDD/schema default for other input paths.
All three EnergyPlus oracle runs finish with exactly 0 Warning and 0 Severe
errors. The gate does not use the upstream EIO `Special Days` date as resolved-
date evidence because that row is printed one day earlier than the executable's
effective day-type projection.

These three cases prove only this fixed Sunday 2/28, duration-one Holiday on the
explicit 2016 three-day calendar. The fixed-Saturday plus-two-day branch is the
separate pair below. They do not claim omitted A5 or native epJSON default
behavior, any other blank/default field, EnergyPlus warning text or repetition
parity beyond the exact clean 0/0 completion counts, EIO special-day date
semantics, EPW holidays, other special-day
types, duration greater than one, weekday-rule forms beyond the separate two
exact 2032 IDF cases, multiple or overlapping definitions, source-order
precedence, leap-policy behavior beyond
the declared 2016 dates, year-end or cross-year projection, schedule day-type lookup, tomorrow
special-day state, raw ESO timestamp serialization, or broad
`WeatherManager`/schedule compatibility.

## Fixed-Saturday IDF Weekend-Observation Policy Evidence Checkpoint

`calendar_special_day_weekend_saturday_enabled_hourly_exact_001` and
`calendar_special_day_weekend_saturday_disabled_hourly_exact_001` use the same
explicit 2016-02-27 through 2016-02-29 calendar, the same fixed MonthDay 2/27
duration-one input-file Holiday, and the same leap-observed EPW without
weather-file holidays. Their IDFs differ only in the explicit
`Apply Weekend Holiday Rule` value; weather-file holidays and DST are No in
both fixtures. The blocking zero-tolerance gate requires ordered, exact, unique
normalized timestamps and 72 `Site Day Type Index` samples in each case:

- Yes observes the Saturday Holiday on Monday 2/29, a plus-two-day shift: 24
  Saturday=7, 24 Sunday=1, and 24 Holiday=8;
- No leaves the Holiday on Saturday 2/27: 24 Holiday=8, 24 Sunday=1, and 24
  Monday=2.

This pair proves only the fixed-Saturday plus-two-day observation branch under
explicit Yes and No policy values. The fixed-Sunday plus-one-day branch remains
the three-case checkpoint above. It does not claim blank A5 behavior for this
Saturday rule, omitted/native-epJSON defaults, other blank/default fields, EPW
holidays, other special-day types, duration greater than one, weekday-rule
forms beyond the separate two exact 2032 IDF cases, multiple or overlapping
definitions, source-order precedence, leap-policy behavior beyond the declared
2016 dates, year-end or
cross-year projection, schedule day-type lookup, tomorrow special-day state,
raw ESO timestamp serialization, or broad `WeatherManager`/schedule
compatibility.

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
filtering. The separate checkpoint below covers exactly two EPW weekday rules;
this pair does not claim them. Neither checkpoint claims weekend shifting,
multiple or overlapping holidays beyond the exact two nonoverlapping weekday
definitions, EPW-versus-IDF precedence, schedule day-type lookup, tomorrow
special-day state, raw ESO timestamp serialization, actual-weather execution,
cross-year reprojection or traversal, or broad `WeatherManager`/schedule
compatibility. Input-file-special-day independence from the disabled EPW policy
remains unit evidence because these fixtures have no input-file special day.

## EPW Nth/Last-Weekday Holiday Evidence Checkpoint

`calendar_epw_holiday_weekday_rules_hourly_exact_001` uses one explicit
2032-02-23 through 2032-02-25 non-actual RunPeriod and a 72-row leap-observed
EPW. The EPW `HOLIDAYS/DAYLIGHT SAVINGS` header declares, in this order,
`4th Monday in February` and `Last Wednesday in February`. The RunPeriod
explicitly enables weather-file holidays and disables weather-file daylight
saving, weekend observation, rain, snow, and actual-weather handling. The IDF
contains no `RunPeriodControl:SpecialDays` or daylight-saving object.

The blocking `compare-calendar-epw-holiday-weekday-rules-exact` gate requires
the two resolved weather-file definitions to remain in that header order:

- `4th Monday in February` resolves to 2032-02-23, day of year 54;
- `Last Wednesday in February` resolves to 2032-02-25, day of year 56.

It separately requires exactly two EnergyPlus EIO `Special Days` rows whose
names appear as Fourth Monday then Last Wednesday and whose types are Sunday;
the gate deliberately does not inspect those rows' date fields.

EnergyPlus assigns both enabled weather-file holidays the source-exact Sunday
day type and index 1. The 72 ordered, exact, unique, zero-tolerance normalized
timestamps and `Site Day Type Index` samples therefore have daily indices
`1/3/1`. The EnergyPlus oracle finishes with exactly 0 Warning and 0 Severe
errors.

This checkpoint claims only those two literal EPW rules, their stated 2032
resolutions and header order, the exact 72 values/timestamps, weather-file
source attribution, Sunday/index-1 projection, and clean 0/0 completion counts.
It does not claim the corresponding IDF weekday forms, fixed-date EPW rules,
other ordinals, weekdays, months, years, or EPW header orderings; overlaps,
multiple policy combinations, weekend shifting, malformed headers, or warning
text/count/repetition parity beyond the exact clean counts; EIO special-day date
semantics, schedule lookup, tomorrow state, actual-weather execution, cross-year
traversal/reprojection, or raw ESO serialization.

## EPW-versus-IDF Special-Day Precedence Evidence Checkpoint

`calendar_special_day_epw_idf_precedence_hourly_exact_001` reuses the enabled
EPW-holiday case's IDF content and its explicit 2016-02-28 through 2016-03-01
calendar plus leap-observed 72-row EPW. The precedence IDF differs from that
base only by adding one duration-one input-file `CustomDay1` definition on
February 29. The shared projection resolves the EPW Holiday first as the
source-exact Sunday type and then the input-file definition, so the later IDF
`CustomDay1` state wins.

The blocking `compare-calendar-special-day-epw-idf-precedence-exact` gate
requires that resolved weather-file-then-input-file order and exactly 72
ordered, unique, zero-tolerance normalized timestamps and `Site Day Type Index`
samples in daily order `1/11/3`: 24 Sunday=1, 24 CustomDay1=11, and 24
Tuesday=3.

This checkpoint proves only that one mixed fixed-date collision: the EPW
Holiday is resolved before one later duration-one IDF CustomDay1 definition.
Reversed or multiple input definitions, other EPW or IDF rules, special-day
types, durations, and overlap shapes; EnergyPlus-versus-Rust warning text,
count, repetition, or diagnostics parity; schedule lookup, tomorrow state, raw
ESO serialization, actual-weather execution, and cross-year traversal or
reprojection remain outside the claim.

## Current Rust Boundary

| Boundary | Current Rust status | Missing source behavior |
|---|---|---|
| run-period input | typed dates, optional years, and start weekday feed EnergyPlus-style year and weekday resolution; the first-hour policy is carried on the axis, the weather-file DST use flag gates a parsed EPW period, while typed IDF `RunPeriodControl:DaylightSavingTime` overrides that source independently of the flag. External DST evidence covers the paired fixed-date Yes/No policy cases, the exact fixed-date IDF-over-disabled-EPW precedence case, one literal fourth-Monday-through-last-Wednesday boundary pair, and the paired exact last-Sunday-October-through-last-Sunday-March end-side and start-side wrap cases, and the weather-file holiday use flag gates parsed fixed and weekday-rule EPW holiday definitions. The exact EPW weekday case locks enabled policy only. The fixed-Sunday gate locks explicit Yes/No plus the EnergyPlus 26.1 executable's blank-A5-as-Yes branch, the fixed-Saturday gate locks explicit Yes/No, typed IDF `RunPeriodControl:SpecialDays` definitions feed the shared day-type projection, IDF-backed loads preserve the validated SpecialDays declaration order used by the exact overlapping pair so the later definition wins, and the exact mixed-source case locks weather-file-then-input-file resolution when one later IDF CustomDay1 overrides an enabled EPW Holiday on the same fixed date. Metadata-aware actual-weather inputs fail explicitly; non-actual cross-year input and typed special-day projection are externally locked only for the single start-year-retention fixture | custom ranges, design-day environments, environment filtering, declaration-order recovery for object types beyond SpecialDays, overlap shapes beyond the exact pair, omitted-A5/native-epJSON default behavior, blank/default behavior for other fields, EPW holiday policy combinations beyond the fixed pair and one enabled weekday case, EPW-versus-IDF precedence beyond the one exact mixed collision, other EPW DST boundary forms, broader years/months/rules, generic southern/wrap behavior, actual-weather execution or non-actual cross-year execution beyond the exact single-boundary fixture, other DST policy/source combinations and IDF objects beyond the exact fixed-date precedence case, actual-weather traversal, cross-year weather traversal beyond that fixture, later annual resets or per-year special-day reprojection, and full EnergyPlus warning-text parity |
| canonical calendar | `ResolvedRunPeriodCalendar` retains Gregorian interpretation, while non-actual `ResolvedWeatherEnvironmentCalendar` applies the EPW leap-year header (including the February 29 endpoint ordinal alias). The metadata-aware axis resolves an enabled EPW DST rule into inclusive daily `dst` state, enabled EPW holidays into source-exact Sunday day type, and typed IDF special-day rules into effective `DayType`/`special_day_type`; `EnvironmentTimePoint` separately owns Gregorian, weather-effective, and schedule day-of-year plus simulation weekday. External numerical DST evidence is limited to the paired fixed-date Yes/No policy cases, the exact fixed-date input-file-over-EPW precedence case with 72 samples in daily status 1/1/0 and 48 active hours, the literal 2032 `4th Monday in February` through `Last Wednesday in February` pair, and the paired southern-wrap cases' 72 EnergyPlus ESO values/timestamp fields over the same-year March end-side 1/1/0 and October start-side 0/1/1 windows plus shared EIO dates 10/31 and 03/28; Rust day-of-year 305/88 and `wraps_year=true` are separate source-mapped diagnostics not emitted by EIO. The exact cross-year DST fixture separately locks one non-actual 2031-12-30 through 2032-01-02 traversal: 96 exact samples in daily status 0/0/0/1, 24 active and 72 inactive hours, exact EIO dates 01/02 through 01/03, and source-mapped January 2 prefetch retention whose internal buffer order is not EIO/ESO output. External numerical special-day evidence is limited to the fixed duration-one IDF Holiday, the paired common-/leap-year December 31 duration-three annual-table wraps, the exact 2031-to-2032 start-year annual-table-retention case, the exact 2032 `4th Sunday in February` and `Last Sunday in February` IDF forms, the fixed-Sunday explicit-Yes/explicit-No/blank and fixed-Saturday explicit-Yes/explicit-No weekend-policy cases, the paired fixed EPW holiday use-policy cases, the exact EPW `4th Monday in February` then `Last Wednesday in February` header, the paired fixed-date SpecialDays declaration-order/overwrite cases, and the one fixed-date EPW-Holiday-then-IDF-CustomDay1 precedence case. The CP42 detailed-schedule case separately consumes current DST and one tomorrow Holiday/final-stale day-type path; it does not promote the remaining calendar family. The blocking smoke/nonclaim fifth-Sunday case separately locks only rejection of the explicit nonexistent 2016 rule and zero produced samples | warmup lifecycle, other DST years/months/date-rule pairs, generic southern/wrap behavior, actual-weather execution or cross-year execution beyond the exact single-boundary fixtures, other RunPeriod policy/source combinations and input-file DST objects beyond the exact fixed-date precedence case, declaration order beyond SpecialDays and the exact EPW two-rule header, overlap behavior beyond the exact pair, warning-text/repetition parity, weekend shifting beyond the fixed-Sunday plus-one-day Yes/blank and No cases and fixed-Saturday plus-two-day explicit Yes/No pair, other blank/default or omitted policy inputs, other Nth ordinals/weekdays/months/years/strings beyond the exact IDF and EPW successes and one IDF failure, other nonexistent-occurrence combinations or multi-error behavior, other special-day types, durations/dates beyond the paired exact annual-table wraps, multiple or overlapping holidays beyond the exact two nonoverlapping EPW rules, EPW-versus-IDF precedence beyond the one exact mixed collision, schedule/timestamp DST or special-day consumers beyond the exact CP42 detailed-schedule path, tomorrow special-day state beyond its one tomorrow-Holiday/final-stale boundary, actual-weather behavior, cross-year weather traversal beyond those fixtures, later annual resets including January 3 DST state, or per-year special-day reprojection, EnergyPlus `Timestep` default/invalid-value normalization, and environment kinds beyond weather run periods |
| hourly consumers | `TimeAxis` is an hour-ending projection of the resolved environment calendar; the paired calendar cases lock 72 leap-observed labels ending Tuesday versus 48 no-leap-policy labels ending simulation Monday from the same IDF and 72 raw EPW rows. The fixed-date DST policy pair locks 72 ordered state/timestamp samples per case: enabled daily status 0/1/1 with 48 active hours versus disabled 0/0/0 with zero active hours. The fixed-date IDF-over-EPW precedence case locks 72 ordered samples in daily status 1/1/0 with 48 active hours and effective source input-file. The literal 2032 EPW weekday-boundary case locks 120 ordered samples as daily status 0/1/1/1/0 with 72 active hours. The southern-wrap end-side case locks only the same-year March window: 72 ordered samples as daily status 1/1/0 with 48 active and 24 inactive hours. The paired start-side case locks the same-year October 30 through November 1 window as 72 ordered samples in daily status 0/1/1 with the same 48 active and 24 inactive hours. The cross-year start-year DST case locks 96 ordered samples in daily status 0/0/0/1 with 24 active and 72 inactive hours; January 2 prefetch retention is source-mapped internal ordering, not an ESO/EIO-emitted field. The fixed IDF special-day case locks 72 ordered day-type/timestamp samples as 24 each of 1/8/3. The cross-year start-year case locks 96 ordered samples in daily indices 3/4/5/8 with January 2 Holiday. The duration-wrap pair locks 72 ordered samples per case and 48 Holiday samples per case: common-year 2017 starts at day of year 365 and uses daily indices 8/8/3, while leap-year 2016 starts at day of year 366 and uses 8/8/1. The two exact 2032 IDF weekday-rule cases each lock 216 ordered samples: fourth Sunday uses daily indices 8/2/3/4/5/6/7/1/2, and last Sunday uses 1/2/3/4/5/6/7/8/2. The nonexistent fifth-Sunday blocking smoke case separately locks engine-specific rejection and zero produced samples without adding numerical day-type evidence. The fixed-Sunday weekend-policy cases lock 72 samples each as explicit Yes and blank 24 each of 1/8/3, with identical values and timestamps, and explicit No 24 each of 8/2/3. The fixed-Saturday weekend-policy pair locks 72 samples each as enabled 24 each of 7/1/8 and disabled 24 each of 8/1/2. The paired fixed EPW holiday cases lock 72 samples each as enabled 24 each of 1/1/3 and disabled 24 each of 1/2/3; the exact EPW weekday-rule case locks 72 enabled samples as 1/3/1. The overlapping SpecialDays pair locks 72 samples each: Zulu then Alpha makes later CustomDay1 win as 1/11/3, while Alpha then Zulu makes later Holiday win as 1/8/3. The mixed EPW/IDF precedence case locks 72 samples as 1/11/3 when the later IDF CustomDay1 overrides the enabled EPW Holiday. The exact Through/For case separately locks one five-day calendar-aware Schedule:Compact series as daily 103/104/105/108/199 with 120 ordered zero-tolerance values and timestamps. The CP42 detailed-schedule case separately locks 72 values as 100x23+124, 200x23+801, and 800x23+901, including current DST +1-hour lookup, next-ordinal/tomorrow-Holiday selection, and the final stale Holiday type. Weather-required heat-balance `ep_run` setup now builds the same metadata-aware axis before runtime execution | runtime consumers outside those weather-required heat-balance classes, DST behavior beyond the fixed-date Yes/No pair, the one exact fixed-date IDF-over-EPW precedence case, one literal 2032 weekday boundary pair, the paired exact southern-wrap end-side and start-side cases, and the exact cross-year start-year case through January 2, special-day behavior beyond the declared fixed-date, exact cross-year start-year projection, paired duration wrap, exact IDF and EPW weekday-rule forms, one exact expected-failure form, the exact two-definition overlap/source-order pair, the one exact mixed-source precedence collision, and policy cases, schedule day-type lookup beyond the exact Through/For and CP42 DST-rollover cases, tomorrow state beyond the one CP42 tomorrow-Holiday/final-stale path, downstream internal-gain/HVAC/IdealLoads consumption of the precomputed Schedule Value series, and all remaining calendar-dependent output semantics |
| EPW weather | `EpwWeatherFile` keeps parsed leap policy, typed optional DST and holiday rules, `DATA PERIODS` metadata, and `EpwRecord` rows. The dedicated hourly report applies the leap policy and enabled DST/holiday policies and selects a complete non-actual, one-record-per-hour stream by source date; external cross-year evidence is limited to the one wrapping DATA PERIOD fixture. The fixed-date DST pair externally locks only the explicit RunPeriod Yes/No policy difference against one shared 2/29-through-3/1 declaration: enabled is 0/1/1 with 48 active hours, while disabled is 0/0/0 with zero active hours; the pair locks exact EnergyPlus 26.1 `Environment` and `Environment:Daylight Saving` EIO rows, including disabled row `Environment:Daylight Saving,No,RunPeriod Object`; the fixed-date precedence case locks the shared EPW declaration being superseded by one input-file 2/28-through-2/29 object despite RunPeriod use No, daily status 1/1/0 with 48 active hours, and exact row `Environment:Daylight Saving,Yes,InputFile,02/28,02/29`; the literal 2032 EPW fourth-Monday-through-last-Wednesday pair additionally locks 120 state/timestamp samples, 72 active hours, raw EnergyPlus oracle ESO timestamp fields and values, and one exact WeatherFile EIO row; the southern-wrap end-side and start-side cases externally lock their respective same-year March 1/1/0 and October 0/1/1 72-sample windows with 48 active hours each, all EnergyPlus ESO timestamp fields and values, and the exact WeatherFile EIO dates 10/31 and 03/28, while Rust day-of-year 305/88 and wraps-year flag remain source-mapped summary diagnostics not emitted by EIO; the exact cross-year DST case locks one 96-row wrapping DATA PERIOD in daily status 0/0/0/1 with 24 active hours, exact 01/02 through 01/03 EIO dates, and source-mapped January 2 prefetch retention; the paired EPW holiday cases lock one fixed holiday enabled as source-exact Sunday index 1 versus disabled as the underlying Monday index 2; the exact EPW weekday-rule case locks `4th Monday in February` then `Last Wednesday in February` resolving to days of year 54 and 56, both as source-exact Sunday/index 1; the mixed-source case locks that enabled EPW Holiday resolving before and then yielding to one later IDF CustomDay1 on the same date; the offset case locks 24 decoy rows skipped and 48 dry-bulb rows in exact timestamp/value order; the cross-year start-year case locks one wrapping 96-row DATA PERIOD in ordered 3/4/5/8 day-type order. Unit tests separately lock Today/Tomorrow source-index transitions, interpolation seeds, the day-local hour-24 solar `NextHr`, and the one-timestep-per-hour current-only solar branch | other EPW DST years/months/rules, January 3 and later reset/reprojection, generic southern/wrap behavior, actual weather or cross-year execution beyond the exact fixture, other RunPeriod policy/source combinations and IDF DST behavior beyond the exact fixed-date precedence case, EPW holiday weekend/weekday-rule/header-order/multiple-policy/overlap/precedence behavior beyond the exact promoted cases, actual-weather year matching, cross-year traversal beyond the exact single-boundary fixtures, multiple-data-period execution, records-per-hour greater than one, complete Today/Tomorrow value-state parity, missing/range repair, cyclic multi-year execution, weather consumers outside the stated `ep_run` setup, subhourly solar interpolation, and complete `ReadEPlusWeatherForDay`/`SetCurrentWeather`/solar/`WeatherManager` parity |
| schedules | `Schedule:Constant` and compiled `Schedule:Compact` periods/day-type profiles/`Until` endpoints produce hourly series. The paired calendar cases lock the same all-days 1-through-24 profile for 72 versus 48 weather-effective hours; the Through/For case locks 120 calendar-aware values as 103/104/105/108/199; and the exact DST rollover case locks 72 values as 100x23+124, 200x23+801, and 800x23+901, including current DST +1-hour lookup, hour-24 next-ordinal/tomorrow-Holiday selection, and the final stale Holiday tomorrow type | schedule-specific DST opt-out, zone-timestep/subhourly lookup or interpolation, schedule families beyond Constant/Compact, additional calendar and rollover boundaries, downstream internal-gain/HVAC/IdealLoads calendar consumption, EMS current-value semantics, `UpdateScheduleVals`, and broad `getHrTsVal` parity |
| output time | hourly consumers use an output-owned normalized comparison label projected from the shared axis; the paired leap-policy schedule cases, record-selection weather case, paired fixed-date DST policy cases, the fixed-date DST EPW/IDF precedence case, literal 2032 EPW weekday-boundary DST case, exact southern-wrap DST end-side and start-side cases, exact cross-year start-year DST case, fixed IDF special-day case, exact cross-year start-year projection case, paired common-/leap-year duration-wrap cases, paired fixed-date SpecialDays declaration-order cases, the fixed-date EPW/IDF precedence case, exact 2032 fourth-/last-Sunday IDF cases, fixed-Sunday Yes/No/blank and fixed-Saturday Yes/No weekend-policy cases, paired fixed EPW holiday policy cases, and the exact EPW weekday-rule case enforce only their declared ordered, unique, exact normalized labels and variables | Rust raw ESO serialization and raw/exact timestep/day/month/run-period EnergyPlus ESO, MTR, and SQL records from `WriteTimeStampFormatData`; broader DST/day-type serialization and tomorrow-state formatting remain unclaimed |

Existing dry-bulb, dew-point, relative-humidity, pressure, wind, radiation, and
precipitation diagnostics remain useful evidence for individual weather
fields. Only the declared offset case promotes record-date selection and ordered
hourly dry-bulb values; other record-order smoke comparisons do not prove
calendar selection, internal weather-day handoff, or timestamp conformance.

## Promotion Gates After the First Checkpoint

Gates are sequential because each later lookup consumes state owned by the
earlier gate.

1. **Remaining DST gate.** The paired fixed-date EPW Yes/No policy cases, the
   exact fixed-date input-file-over-disabled-EPW precedence case, one literal
   EPW fourth-Monday-through-last-Wednesday boundary pair, and the exact
   last-Sunday-October-through-last-Sunday-March end-side and start-side windows
   now have external exact gates. The single 2031-12-30 through 2032-01-02
   start-year projection also externally locks 96 daily `0/0/0/1` samples,
   24 active hours, and January 2 prefetch retention as source-mapped internal
   ordering. Promote other years, months, and date-rule pairs, generic
   southern/wrap behavior, actual weather, cross-year execution beyond that
   exact fixture, and January 3 or later reset/reprojection; add other RunPeriod
   policy/source branches and input-file objects beyond the exact precedence
   case. The separate three-day detailed-schedule fixture now locks one current-
   DST +1-hour and hour-24/tomorrow-Holiday path, including the final stale
   Holiday tomorrow type; promote other DST schedule boundaries, schedule-
   specific opt-out, and subhourly behavior before claiming broad
   `ScheduleDetailed::getHrTsVal` parity.
2. **Remaining special-day gate.** The fixed duration-one IDF Holiday, the exact
   common-year and leap-year December 31 duration-three annual-table wraps, the exact
   2032 `4th Sunday in February` and `Last Sunday in February` IDF forms,
   the single 2031-to-2032 start-year annual-table-retention case,
   fixed-Sunday plus-one-day explicit-Yes and executable-observed blank branches,
   fixed-Sunday explicit No, fixed-Saturday plus-two-day explicit Yes/No,
   paired fixed EPW holiday use-policy branch, and the exact EPW fourth-Monday
   plus last-Wednesday rule case
   now have external exact gates. The paired fixed-date two-definition overlap
   also has an exact gate for SpecialDays-only IDF declaration order and later-
   definition overwrite. The one fixed-date mixed-source case separately locks
   EPW-Holiday-then-IDF-CustomDay1 resolution and the later input-file
   overwrite as daily indices 1/11/3. The explicit 2016 nonexistent fifth-Sunday
   form has a blocking smoke/nonclaim rejection gate. Promote other Nth
   ordinals, weekdays, months, years, and date strings; add negative gates for
   other nonexistent occurrences and multiple-error interactions; promote other
   EPW Nth/Last forms, header orderings, and policy combinations, plus other
   duration/date/type combinations,
   directly ordered typed-vector overwrite, omitted-A5/native-epJSON defaults
   and blank/default behavior for other fields, and
   overlap shapes beyond the exact two-definition fixed-date pair; add later
   annual resets, per-year cross-year reprojection, multiple-boundary and
   February 29 combinations, and EPW-versus-IDF precedence beyond the one exact
   mixed collision. The detailed-schedule DST fixture now locks tomorrow
   Holiday selection on one boundary and the final stale Holiday type; expand
   tomorrow weekday/special-type behavior to other dates, policies, schedules,
   and timestamps.
3. **Remaining EPW environment gate.** Extend the non-actual hourly selector
   beyond the exact one-boundary, one-DATA-PERIOD case into actual-weather year
   matching, additional or multiple year boundaries and DATA PERIODS,
   records-per-hour greater than one, full Today/Tomorrow value-state handoff,
   multi-year wrap, missing/range rules, and all active
   `InitializeWeather`, `UpdateWeatherData`, and `SetCurrentWeather` fields at
   every zone timestep.
4. **Schedule gate.** Extend the exact Compact Through/For and three-day DST
   hour-24/tomorrow evidence to the full supported schedule-object set and the
   EnergyPlus 366-day/day-type/timestep layout, update one `currentVal` per zone
   timestep through `UpdateScheduleVals`, and match `getHrTsVal` across broader
   weekday, weekend, holiday, DST transition, opt-out, subhourly, and rollover
   cases.
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

The fixed-date DST pair adds only its shared EPW February 29 through March 1
period and explicit RunPeriod Yes/No policy difference. Enabled locks 72
ordered normalized samples in daily order 0/1/1 with 48 active hours; disabled
locks 0/0/0 with zero active hours. The pair gate locks exact EnergyPlus 26.1
`Environment` and `Environment:Daylight Saving` EIO rows; the disabled
daylight-saving row is `Environment:Daylight Saving,No,RunPeriod Object`, while Rust retains the
weather-file declaration with `active=false` and `resolved_period=null`;
those Rust fields are summary diagnostics rather than additional EIO output.
Other periods, years, rules, policy combinations beyond the separate exact
precedence case, clock effects, actual weather, raw Rust ESO, and broad
`WeatherManager` remain unclaimed.

The fixed-date input-file precedence fixture adds only the exact
2016-02-28 through 2016-03-01 RunPeriod, its weather-file DST use flag `No`,
the shared EPW 2/29 through 3/1 declaration, and one input-file 2/28 through
2/29 daylight-saving object. The input-file source wins, locking 72 ordered,
unique, zero-delta EnergyPlus ESO values and timestamp fields in daily status
`1/1/0` with 48 active hours. The exact EnergyPlus 26.1 EIO rows are
`Environment,DST FIXED DATE RUN PERIOD,WeatherFileRunPeriod,02/28/2016,03/01/2016,Sunday,3,Use RunPeriod Specified Day,No,No,No,No,No,Clark and Allen`
and `Environment:Daylight Saving,Yes,InputFile,02/28,02/29`, with clean
0 Warning/0 Severe completion. Rust's typed object, JSON
`input_file_period_declared`/`effective_source`, and Markdown
`input_file_daylight_saving_period_declared`/`daylight_saving_effective_source`
are summary diagnostics rather than additional EIO output. Other fixed,
Nth/last, or wrap rules; missing, malformed, or duplicate error parity beyond
typed diagnostics; other years, multiple RunPeriods, and policy combinations;
schedule/civil-clock effects; actual, cross-year, or DesignDay execution; raw
Rust ESO; and broad `WeatherManager` remain unclaimed.

The weekday-rule fixture adds only the literal EPW start/end pair
`4th Monday in February` through `Last Wednesday in February` on the explicit
2032-02-22 through 2032-02-26 calendar. It resolves to days of year 54 through
56 without wrap and locks 120 ordered normalized samples in daily order
0/1/1/1/0, exactly 72 active hours, the raw EnergyPlus oracle ESO timestamp
fields and values, the exact WeatherFile EIO row, and clean 0/0 completion.
The southern-wrap fixture adds only the literal EPW pair `Last Sunday in
October` through `Last Sunday in March`, its EnergyPlus EIO dates 10/31 and
03/28, and the same-year March 27 through March 29 end-side execution window. It
locks 72 ordered normalized samples in daily order 1/1/0, 48 active and 24
inactive hours, every raw EnergyPlus oracle ESO timestamp field and value, the
exact EnergyPlus EIO row, and clean 0/0 completion. Rust day-of-year 305 and 88
plus `wraps_year=true` are separate source-mapped summary diagnostics, not
EIO-emitted fields. Cross-year RunPeriod or weather-record traversal, full-year
range execution, execution around the October start boundary, generic southern-
hemisphere or year-wrap behavior, and other date-rule pairs remain unclaimed.
The southern-wrap start-side fixture adds only the inclusive October 31 boundary
over the same-year October 30 through November 1 window. It locks 72 ordered
EnergyPlus ESO values and timestamp fields in daily order 0/1/1, with 48 active
and 24 inactive hours, plus the exact EnergyPlus EIO dates 10/31 and 03/28.
Rust day-of-year 305/88 and `wraps_year=true` remain source-mapped summary
diagnostics rather than EIO-emitted fields. This case does not prove full-year
or cross-year traversal, and the March end boundary remains the separate
end-side case. Broader years/months/rules, disabling beyond the paired
fixed-date Yes/No case, precedence beyond the exact fixed-date input-file case,
DST clock effects, actual weather, and broad `WeatherManager` remain unclaimed.
Input-file DST objects and other RunPeriod policy/source branches beyond that
precedence case, schedule DST shift and hour-24 rollover, special days, Rust
raw ESO serialization, actual-weather execution, and cross-year traversal
beyond the separate exact start-year fixture remain outside this claim.

The fixed IDF special-day fixture adds only one February 29 duration-one
Holiday and 72 ordered normalized hourly timestamps and `Site Day Type Index`
values: 24 Sunday=1, 24 Holiday=8, and 24 Tuesday=3. Its weather-file holidays,
weekend observation, and DST flags are explicitly No and do not prove those
policies. Other Nth/last rules beyond the two exact 2032 IDF forms below,
same-year annual-table duration/wrap beyond the paired exact common-/leap-year
checkpoint below, and directly ordered typed-vector overwrite remain
unit/source evidence. Only the separate paired checkpoint below promotes
compiled IDF overlap precedence; cross-year behavior remains limited to the
separate exact start-year-retention fixture.

The cross-year start-year fixture adds only one non-actual 2031-12-30 through
2032-01-02 transition, one wrapping DATA PERIOD with one record per hour, and
one `1st Thursday in January` input-file Holiday. EnergyPlus resolves the rule
against the 2031 environment-start annual table and retains January 2 across
that boundary. Its 96 ordered, unique, zero-tolerance `Site Day Type Index`
values and timestamps follow daily order 3/4/5/8, with exactly 24 January 2
Holiday samples. The gate also locks the 96 raw ESO values and hourly timestamp
rows, exact EnergyPlus 26.1 Environment and Environment:Special Days EIO rows,
and clean 0/0 completion. A Gregorian 2032 January 1 reprojection, actual
weather, later annual resets/reprojection, multiple boundaries or DATA PERIODS,
DST cross-year semantics, February 29 coupling, EPW holidays, weekend shifting,
duration wrap, overlaps, warmup lifecycle parity, records per hour above one,
and broad weather/schedule behavior remain unclaimed.

The paired duration-wrap fixtures add only one December 31 duration-three
input-file Holiday to explicit January 1 through January 3 same-year annual
tables. The common-year 2017 case resolves day of year 365 and locks 72 ordered,
unique, zero-tolerance `Site Day Type Index` samples in daily order 8/8/3; the
leap-year 2016 case resolves day of year 366 and locks 72 samples in order
8/8/1. Each contains exactly 48 Holiday=8 samples, and every RunPeriod policy is
explicitly No. This proves only the common-year and leap-year source branches
for cyclic annual-table wrap. Actual-weather execution, non-actual cross-year
execution beyond the exact start-year fixture, or per-year reprojection,
overlap/precedence/declaration order/warnings, and other durations, dates,
types, policies, or EPW rules remain outside this claim.

The paired overlapping IDF fixtures add exactly two duration-one February 29
definitions to the explicit 2016-02-28 through 2016-03-01 calendar and differ
only in declaration order. Zulu Holiday then Alpha CustomDay1 locks later Alpha
as daily indices 1/11/3; Alpha then Zulu locks later Zulu as 1/8/3. Both cases
have 72 ordered, unique, zero-tolerance samples. This promotes only original
IDF declaration-order recovery for `RunPeriodControl:SpecialDays` and the
resulting later-definition overwrite. Native epJSON ordering, declaration order
for other object types, other overlap shapes/counts/durations/types,
EPW-versus-IDF precedence, and EnergyPlus warning text/repetition parity remain
outside the claim.

The mixed-source precedence fixture reuses the enabled EPW-holiday IDF and
differs only by adding one duration-one February 29 input-file CustomDay1
definition. The enabled fixed EPW Holiday resolves first as source-exact Sunday
index 1; the later input-file definition wins, so its 72 ordered, unique,
zero-tolerance samples have daily indices 1/11/3. This promotes only that exact
weather-file-then-input-file collision. Reversed or multiple input definitions,
other rules/types/durations/overlap shapes, warning parity, schedule/tomorrow
state, raw ESO serialization, actual weather, and cross-year behavior remain
outside the claim.

The paired IDF weekday-rule fixtures add only `4th Sunday in February` and
`Last Sunday in February` over 2032-02-22 through 2032-03-01. The fourth Sunday
resolves to 2/22, day of year 53, and its 216 ordered, unique, zero-tolerance
samples have daily indices 8/2/3/4/5/6/7/1/2. The last Sunday resolves to leap
day 2/29, day of year 60, and its 216 samples have daily indices
1/2/3/4/5/6/7/8/2. `Apply Weekend Holiday Rule` is explicitly Yes in both, and
the Sunday rules shift zero days. Other IDF Nth/weekday/month/year strings and
duration/wrap behavior beyond the dedicated exact pair remain outside this
numerical claim. The separate EPW checkpoint covers only its exact fourth-
Monday and last-Wednesday rules. Other EPW rules/header orderings,
overlap/order, actual cross-year behavior, schedule lookup, tomorrow state, and
raw ESO serialization remain outside the claim.

The blocking nonexistent-fifth-Sunday fixture adds only semantic
expected-failure evidence for the exact 2016 `5th Sunday in February` rule.
WeekendRule Yes does not rescue it. EnergyPlus exits 1 through its
`SetSpecialDayDates`
not-enough-Nths Severe-to-Fatal path with 0 warnings, 1 severe error, and zero
data rows. Rust arbitrary-run exits 6 with phase `runtime`, code
`RuntimeConvergenceFailure`, the exact special-day rejection, and zero hourly
samples. Numeric exit-code equality, numerical day-type conformance, other
ordinal/weekday/month/year combinations, EPW rules, multiple errors or
definitions, partial-artifact parity, duration/overlap/precedence/order,
year-wrap/cross-year behavior, schedule/tomorrow state, and successful-run raw
ESO serialization remain outside this smoke/nonclaim boundary.

The three fixed-Sunday IDF fixtures add only the RunPeriod weekend-rule
explicit Yes/No and blank A5 forms around one MonthDay 2/28 duration-one
Holiday. Their 72 ordered,
unique, zero-tolerance normalized hourly timestamps and `Site Day Type Index`
values are exactly 24 each of Sunday/Holiday/Tuesday (1/8/3) when enabled and
Holiday/Monday/Tuesday (8/2/3) when disabled; blank is sample-for-sample equal
to explicit Yes for the gated oracle values, with identical timestamp rows, and resolves 2/29,
day of year 60, shift 1. EnergyPlus 26.1's IDD and epJSON schema say default
`No`, while this exact blank IDF executes as enabled. This proves only Sunday
plus-one-day observation and that narrow executable mismatch. Saturday
plus-two-day observation remains the separate pair below. Omitted A5, native
epJSON defaults, other blank/default fields, EIO special-day date semantics,
warning text/repetition parity beyond exact clean 0/0 completion, EPW holidays, other day
types, duration greater than one, weekday-rule forms beyond the separate two
exact 2032 IDF cases, overlap/order, leap-policy
behavior beyond the declared 2016 dates, year-end/cross-year behavior, schedule
lookup, tomorrow state, and raw ESO timestamp serialization remain outside this
claim.

The paired fixed-Saturday IDF fixtures add only the explicit RunPeriod weekend
rule toggle around one MonthDay 2/27 duration-one Holiday. Their 72 ordered,
unique, zero-tolerance normalized hourly timestamps and `Site Day Type Index`
values are exactly 24 each of Saturday/Sunday/Holiday (7/1/8) when enabled and
Holiday/Sunday/Monday (8/1/2) when disabled. This proves only Saturday
plus-two-day observation; Sunday plus-one-day observation remains the
three-case evidence above. Blank A5 behavior for this Saturday rule,
omitted/native-epJSON defaults, other blank/default fields, EPW holidays, other day types,
duration greater than one, weekday-rule forms beyond the separate two exact
2032 IDF cases, overlap/order, leap-policy behavior
beyond the declared 2016 dates, year-end/cross-year behavior, schedule lookup,
tomorrow state, and raw ESO timestamp serialization remain outside this claim.

The paired fixed EPW holiday fixtures add only one February 29 weather-file
holiday and the RunPeriod use-policy toggle. Their 72 ordered normalized hourly
timestamps and `Site Day Type Index` values are exactly 24 each of 1/1/3 when
enabled and 1/2/3 when disabled. The enabled middle day is EnergyPlus'
source-exact EPW-holiday Sunday index 1, not input-file Holiday index 8.
The separate exact EPW weekday-rule case below does not widen this pair's
fixed-date policy claim. Weekend shifting, other EPW Nth/Last forms and header
orderings, multiple or overlapping holidays beyond that exact nonoverlapping
pair, EPW-versus-IDF precedence beyond the separate exact mixed collision,
schedule day-type lookup, tomorrow special-day state, raw ESO timestamp
serialization, and cross-year behavior remain outside this claim.

The exact EPW weekday-rule fixture adds only `4th Monday in February` followed
by `Last Wednesday in February` over the explicit non-actual 2032-02-23 through
2032-02-25 calendar. The rules resolve in header order to days of year 54 and
56 as weather-file Sunday/index 1. Its 72 ordered, unique, zero-tolerance
timestamps and `Site Day Type Index` values have daily indices `1/3/1`, and
EnergyPlus completes with exactly 0 Warning and 0 Severe errors. Corresponding
IDF forms, fixed EPW dates, other ordinals/weekdays/months/years/header
orderings, overlaps, multiple policy combinations, warning parity beyond clean
counts, EIO date semantics, schedule/tomorrow state, actual weather, and cross-
year behavior remain outside this claim.

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
boundary, the fixed-date DST pair keeps only its enabled 72-sample `0/1/1`
boundary with 48 active hours and disabled `0/0/0` boundary with zero active hours, the
fixed-date input-file precedence case keeps only its ordered 72-sample
`1/1/0` boundary with 48 active hours and effective source `input-file`, the literal
EPW weekday-boundary DST case keeps only its ordered 120-sample
`0/1/1/1/0` boundary with 72 active hours, the southern-wrap DST case keeps
only its same-year end-side ordered 72-sample `1/1/0` boundary with 48 active
hours, the southern-wrap start-side DST case keeps only its same-year ordered
72-sample `0/1/1` boundary with 48 active hours. The cross-year start-year DST
case keeps only its ordered 96-sample `0/0/0/1` boundary with 24 active and 72
inactive hours, the exact 01/02 through 01/03 EIO range, and source-mapped
January 2 prefetch retention through that final simulated day. The fixed IDF special-day case
keeps only its ordered 72-sample 1/8/3 boundary. The cross-year start-year case
keeps only its ordered 96-sample 3/4/5/8 boundary, January 2 Holiday, and the
single non-actual 2031-to-2032 traversal. The
duration-wrap pair keeps only its ordered 72-sample common-year 8/8/3 and
leap-year 8/8/1 boundaries, with 48 Holiday samples in each case. The
overlap/source-order pair keeps only its ordered 72-sample Zulu-then-Alpha
1/11/3 and Alpha-then-Zulu 1/8/3 boundaries. The mixed-source precedence case
keeps only its ordered 72-sample weather-file-then-input-file 1/11/3 boundary.
The fixed-Sunday weekend cases keep only their ordered 72-sample explicit-Yes
and blank 1/8/3 plus explicit-No 8/2/3 boundaries; the blank and explicit-Yes
oracle values and timestamps are identical. The fixed-Saturday weekend pair keeps only its
ordered 72-sample enabled 7/1/8 and disabled 8/1/2 boundaries. The paired EPW
holiday cases keep only their ordered 72-sample enabled 1/1/3 and disabled
1/2/3 boundaries. The EPW weekday-rule case keeps only its exact ordered header,
day-of-year 54/56 resolutions, and ordered 72-sample 1/3/1 boundary. The paired
IDF weekday-rule cases keep only their explicit
2032 forms and ordered 216-sample daily-index boundaries: fourth Sunday
8/2/3/4/5/6/7/1/2 and last Sunday 1/2/3/4/5/6/7/8/2. The
nonexistent-fifth-Sunday case keeps only its blocking smoke/nonclaim
engine-specific rejection contracts and zero-data boundary; it adds no
numerical day-type evidence.
Their consumption by weather-required heat-balance `ep_run` setup adds no
independent conformance evidence. Record selection beyond the offset case, DST
behavior beyond the seven exact DST cases including the one input-file
precedence fixture, special-day behavior beyond the fixed IDF,
exact cross-year start-year projection,
paired common-/leap-year duration wrap,
the exact two-definition fixed-date SpecialDays overlap/source-order pair,
exact 2032 fourth-/last-Sunday successes, exact 2016 fifth-Sunday failure,
fixed-Sunday Yes/No/blank and fixed-Saturday Yes/No
weekend-policy, paired fixed EPW cases, and the exact two-rule EPW weekday case,
EPW-versus-IDF precedence beyond the one exact mixed collision, schedule
day-type lookup, tomorrow special-day state, weather
consumers outside the stated setup, Rust raw ESO serialization and raw
EnergyPlus output frequencies outside the locked hourly oracle fields,
actual-weather execution, cross-year traversal beyond the exact single-boundary
fixtures, multiple-data-period execution, records-per-hour greater than one,
later annual resets/reprojection including January 3 DST state, other DST
cross-year semantics, February 29
coupling, subhourly solar interpolation,
and complete `SetCurrentWeather`/solar/`WeatherManager` conformance remain
explicitly deferred.
