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
Weather::GetRunPeriodData
  -> Weather::SetupEnvironmentTypes

SimulationManager::ManageSimulation
  -> environment loop: Weather::GetNextEnvironment
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
| run-period intake | `Weather::GetRunPeriodData` | Validates and resolves `RunPeriodInput` start/end dates and years, start weekday, Julian bounds, number of simulation years, holiday/DST/weekend/rain/snow policies, actual-weather policy, and first-hour interpolation policy. It does not advance simulation time. |
| environment materialization | `Weather::SetupEnvironmentTypes` | Copies each run period into `Environment`, derives `StartJDay`, `EndJDay`, `RawSimDays`, `TotalDays`, leap-year handling, environment kind/name, weekday map seed, and the run-period policy flags. This is descriptor construction, not current-day state. |
| environment selection | `Weather::GetNextEnvironment` | Advances `Envrn`, selects the descriptor, and seeds `KindOfSim`, `CalendarYear`, month/day/day-of-year, `NumOfDayInEnvrn`, `CurEnvirNum`, and environment name. For a weather run it resolves `CurrentYearIsLeapYear`, weekday tables, active DST ranges, special-day dates, and the effective weather policy switches. |
| nested traversal | `SimulationManager::ManageSimulation` | Owns `DayOfSim`, `HourOfDay`, `TimeStep`, warmup repetition, and the environment/day/hour/timestep begin/end flags. Calendar and weather routines consume these counters; they do not own the nested loop. |
| EPW header intake | `Weather::ProcessEPWHeader` | Parses `DATA PERIODS` record frequency, names, start weekdays, and start/end dates before environment weather rows are positioned. This metadata constrains later source reads; it is not itself a selected weather stream. |
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

The checkpoint intentionally fixes `dst=false`, leaves
`special_day_type=None`, and derives `DayType` from the simulation-effective
weekday only. That weekday equals the Gregorian weekday on ordinary paths but
does not advance across a February 29 skipped by EPW policy.
Its `gregorian_year_is_leap_year` field is date arithmetic state, not
EnergyPlus' weather-effective `CurrentYearIsLeapYear`; the latter also depends
on EPW leap-year support. The paired weather-effective checkpoint below now
resolves that distinction for its declared local fixtures, without widening the
claim to general EPW environment traversal.
The typed `RunPeriod` holiday, DST, weekend-observation, rain, snow, actual
weather, and first-hour policy fields are intake state. The first-hour policy
is carried by the axis. Metadata-aware axes reject all actual-weather and
cross-year run periods rather than returning a partial calendar; their full
record traversal is not ported.
The other five policy booleans are not yet active calendar or weather behavior.
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
and snow so those unported branches cannot be mistaken for evidence.

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
The new path is consumed by the dedicated CLI report and gate; migration into
the general runtime/`ep_run` execution path also remains deferred.

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
dry-bulb values. It does not independently prove Today/Tomorrow storage,
subhourly interpolation, `SetCurrentWeather` field parity, actual-weather year
matching, cross-year traversal, execution across multiple data periods,
records-per-hour greater than one, DST, holidays or special days, missing-value
repair, cyclic multi-year execution, or broad WeatherManager compatibility.
The selector remains a dedicated time/weather report boundary; general
runtime/`ep_run` migration is still deferred.

## Current Rust Boundary

| Boundary | Current Rust status | Missing source behavior |
|---|---|---|
| run-period input | typed dates, optional years, and start weekday feed EnergyPlus-style year and weekday resolution; the first-hour policy is carried on the axis, metadata-aware actual-weather and cross-year inputs fail explicitly, and the other five RunPeriod weather-policy booleans remain typed intake only | custom ranges, design-day environments, environment filtering, active RunPeriod holiday/DST/weather behavior, actual-weather and cross-year traversal, and full EnergyPlus warning-text parity |
| canonical calendar | `ResolvedRunPeriodCalendar` retains Gregorian interpretation, while same-year non-actual `ResolvedWeatherEnvironmentCalendar` applies the EPW leap-year header (including the February 29 endpoint ordinal alias) and `EnvironmentTimePoint` separately owns Gregorian, weather-effective, and schedule day-of-year plus simulation weekday | warmup lifecycle, DST ranges, special-day overrides, actual-weather and cross-year behavior, EnergyPlus `Timestep` default/invalid-value normalization, and environment kinds beyond weather run periods |
| legacy hourly consumers | `TimeAxis` is an hour-ending projection of the resolved environment calendar; the paired calendar cases lock 72 leap-observed labels ending Tuesday versus 48 no-leap-policy labels ending simulation Monday from the same IDF and 72 raw EPW rows | migration into general runtime/`ep_run` consumers, calendar projections beyond the declared pair, and all remaining calendar-dependent output semantics |
| EPW weather | `EpwWeatherFile` keeps parsed calendar and `DATA PERIODS` metadata with `EpwRecord` rows; the dedicated hourly report applies `Leap Year Observed` and selects a complete same-year non-actual, one-record-per-hour stream by source date. The external offset case locks 24 decoy rows skipped and 48 dry-bulb rows in exact timestamp/value order; unit tests separately lock Today/Tomorrow source-index transitions and interpolation seeds | actual-weather year matching, cross-year traversal, multiple-data-period execution, records-per-hour greater than one, complete Today/Tomorrow value-state parity, missing/range repair, cyclic multi-year execution, general runtime/`ep_run` migration, and complete `ReadEPlusWeatherForDay`/`SetCurrentWeather` parity |
| schedules | `Schedule:Constant` and an all-days `Schedule:Compact` `Until` subset can produce hourly series; the paired exact cases lock the same 1-through-24 daily profile for 72 versus 48 weather-effective hours | `Through`/`For` day-type expansion, zone-timestep lookup, holiday/DST rollover, full day schedules, EMS current-value semantics, and exact `getHrTsVal` parity |
| output time | hourly consumers use an output-owned normalized comparison label projected from the shared axis; the paired leap-policy schedule cases and the record-selection weather case enforce ordered, unique, exact normalized labels | raw and exact timestep/hour/day/month/run-period ESO, MTR, and SQL records from `WriteTimeStampFormatData`, including DST and day type |

Existing dry-bulb, dew-point, relative-humidity, pressure, wind, radiation, and
precipitation diagnostics remain useful evidence for individual weather
fields. Only the declared offset case promotes record-date selection and ordered
hourly dry-bulb values; other record-order smoke comparisons do not prove
calendar selection, internal weather-day handoff, or timestamp conformance.

## Promotion Gates After the First Checkpoint

Gates are sequential because each later lookup consumes state owned by the
earlier gate.

1. **DST gate.** Port effective `UseDST` policy, EPW/IDF DST range selection,
   northern- and southern-hemisphere year-boundary behavior, daily
   `DSTIndicator`, and the hour-24 rollover fields consumed by
   `ScheduleDetailed::getHrTsVal`.
2. **Special-day gate.** Port EPW and `RunPeriodControl:SpecialDays` dates,
   `UseHolidays`, `ApplyWeekendRule`, special-day precedence, `HolidayIndex`,
   tomorrow's holiday/day type, and the weekday-versus-special `DayType`
   selection used by schedules and timestamps.
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
Today/Tomorrow or interpolation unit evidence into an external state claim, and
it does not complete `ProcessEPWHeader`, `ReadWeatherForDay`,
`ReadEPlusWeatherForDay`, `UpdateWeatherData`, or `SetCurrentWeather`.

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
boundary. Record selection beyond that case, runtime/`ep_run` migration, DST,
holidays, raw ESO output, actual-weather execution, cross-year traversal,
multiple-data-period execution, and records-per-hour greater than one remain
explicitly deferred.
