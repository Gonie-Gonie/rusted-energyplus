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
| weather driver | `Weather::ManageWeather` | Preserves the barrier `InitializeWeather` -> pre-weather EMS call -> `SetCurrentWeather` -> weather/time reporting for every zone timestep. |
| environment/day initialization | `Weather::InitializeWeather` | On `BeginEnvrnFlag`, initializes missing-value state and reads the first weather day. On `BeginDayFlag`, calls `UpdateWeatherData` before reading or preparing the next tomorrow record, then exposes tomorrow date/day-type fields for rollover. |
| daily state commit | `Weather::UpdateWeatherData` | Commits `TomorrowVariables` and `wvarsHrTsTomorrow` to today's state, then writes current year/month/day/day-of-year, weekday, `HolidayIndex`, report day type, `DSTIndicator`, and daily solar-calendar values. This is the daily calendar commit point. |
| zone-timestep weather and time | `Weather::SetCurrentWeather` | Resolves `NextHour`, leap-shaped `DayOfYear_Schedule`, schedule values, interpolation weights, `CurrentTime`, `SimTimeSteps`, and the current zone-timestep environment fields. Hour 24 can read hour 1 of tomorrow; weather values and derived psychrometric state are committed here. |
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
  reject actual-weather and cross-year traversal until their EPW data-period
  and record-selection branches are ported.
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
This calendar layer models `SetupEnvironmentTypes`; it does not yet perform the
later literal EPW start-record search. EnergyPlus succeeds for a February 29
start under a no-leap header only when a raw February 29 row is present to find
and discard, as it is in the declared fixture; without that row the oracle
rewinds and terminates. A future record-aware gate must cover both outcomes.
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
The manifest schema currently permits the option only for hourly schedule ESO
series, the sole report boundary that consumes and gates it; other output
families are rejected instead of silently ignoring the declaration.

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

## Current Rust Boundary

| Boundary | Current Rust status | Missing source behavior |
|---|---|---|
| run-period input | typed dates, optional years, and start weekday feed EnergyPlus-style year and weekday resolution; the first-hour policy is carried on the axis, metadata-aware actual-weather and cross-year inputs fail explicitly, and the other five RunPeriod weather-policy booleans remain typed intake only | custom ranges, design-day environments, environment filtering, active RunPeriod holiday/DST/weather behavior, actual-weather and cross-year traversal, and full EnergyPlus warning-text parity |
| canonical calendar | `ResolvedRunPeriodCalendar` retains Gregorian interpretation, while same-year non-actual `ResolvedWeatherEnvironmentCalendar` applies the EPW leap-year header (including the February 29 endpoint ordinal alias) and `EnvironmentTimePoint` separately owns Gregorian, weather-effective, and schedule day-of-year plus simulation weekday | warmup lifecycle, DST ranges, special-day overrides, actual-weather and cross-year behavior, EnergyPlus `Timestep` default/invalid-value normalization, environment kinds beyond weather run periods, and source-order environment selection |
| legacy hourly consumers | `TimeAxis` is an hour-ending projection of the resolved environment calendar; the paired calendar cases lock 72 leap-observed labels ending Tuesday versus 48 no-leap-policy labels ending simulation Monday from the same IDF and 72 raw EPW rows | migration into general runtime/`ep_run` consumers, calendar projections beyond the declared pair, and all remaining calendar-dependent output semantics |
| EPW weather | `EpwWeatherFile` keeps parsed calendar metadata and `EpwRecord` rows together; the dedicated hourly report applies `Leap Year Observed` before projection, while `WeatherTimestepSeries` precomputes the current interpolation subset | general environment-date and record selection, today/tomorrow lifecycle, EPW data-period rules, actual-weather traversal, cross-year behavior, missing/range handling, and complete `SetCurrentWeather` parity |
| schedules | `Schedule:Constant` and an all-days `Schedule:Compact` `Until` subset can produce hourly series; the paired exact cases lock the same 1-through-24 daily profile for 72 versus 48 weather-effective hours | `Through`/`For` day-type expansion, zone-timestep lookup, holiday/DST rollover, full day schedules, EMS current-value semantics, and exact `getHrTsVal` parity |
| output time | hourly consumers use an output-owned normalized comparison label projected from the shared axis; the paired leap-policy cases enforce ordered, unique, exact normalized labels | raw and exact timestep/hour/day/month/run-period ESO, MTR, and SQL records from `WriteTimeStampFormatData`, including DST and day type |

Existing dry-bulb, dew-point, relative-humidity, pressure, wind, radiation, and
precipitation diagnostics remain useful evidence for individual weather
fields. Record-order smoke comparisons do not prove calendar selection,
weather-day handoff, or timestamp conformance.

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
3. **EPW environment gate.** Extend the paired header-level leap-policy state
   into record selection by resolved environment and data period rather than
   vector position; port first-day/today/tomorrow handoff, typical versus actual
   weather, multi-year wrap, cross-year policy, missing/range rules, and all active `InitializeWeather`,
   `UpdateWeatherData`, and `SetCurrentWeather` fields at every zone timestep.
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
normalized hourly Schedule Value conformance boundary stated above. General EPW
record selection, runtime/`ep_run` migration, DST, holidays, raw ESO output,
actual-weather execution, and cross-year traversal remain explicitly deferred.
