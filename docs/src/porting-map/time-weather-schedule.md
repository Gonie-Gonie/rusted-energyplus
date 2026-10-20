---
status: active
claim_level: planning-guard
owner: runtime
last_reviewed: 2026-10-20
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
| project timestep intake | `SimulationManager::GetProjectData` | Reads the `Timestep` object before environment traversal. EnergyPlus 26.1 lines 805-880 validate `TimeStepsInHour`, apply correction/default branches, then derive `TimeStepZone`, `MinutesInTimeStep`, and `TimeStepZoneSec`. The CP43 and CP44 fixtures exercise only the explicit valid value 4, yielding 15-minute/900-second zone timesteps; other timestep counts plus correction and default branches remain unclaimed. |
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
| compact schedule input/default ownership | `Sched::ProcessScheduleInput` | EnergyPlus 26.1 lines 1248-1477 create the internal compact day/week schedules and dispatch their intervals. Lines 1253 and 1404-1418 make the `Interpolate` directive optional and retain the day schedule's default `No` state when the next field is `Until`. CP43 exercises only that omitted-directive default. CP44 separately locks explicit No, Average, and Linear intake only for its one-day, three-profile `Timestep,4` fixture; explicit-mode intake beyond that exact case remains unclaimed. |
| compact interval minute expansion | `Sched::ProcessIntervalFields` | EnergyPlus 26.1 lines 2733-2961 decode each `Until` endpoint, fill the 1,440-minute day, and choose flat versus Linear minute values. CP43 exercises aligned endpoints and flat fill under default Interpolate:No. CP44 separately locks explicit-No flat fill, Average flat minute values, a flat first Linear interval, the later cross-hour Linear ramp, and the explicit-No-only non-aligned warning for `Until: 00:20` and `01:15` in one exact fixture. `Until 24:MM` correction, malformed/overlap/zero/incomplete profiles, other interval shapes, and broad diagnostic parity remain unclaimed. |
| day-schedule timestep population | `Sched::DaySchedule::populateFromMinuteVals` | EnergyPlus 26.1 lines 211-239 either averages each zone-timestep window or selects the minute at each timestep end. CP43 locks default-No end-minute sampling at aligned 15-minute endpoints for one `Timestep,4` profile. CP44 separately locks explicit No and Linear endpoint sampling plus the Average 15-minute window mean for exactly three profiles in its one-day `Timestep,4` fixture; other timestep counts, mixed multi-profile modes, hourly aggregation, and broader population behavior remain unclaimed. |
| file schedule intake and 366-day expansion | `Sched::ProcessScheduleInput` | EnergyPlus 26.1 lines 1573-1863 validate `Schedule:File` metadata, resolve and parse the selected external column, build hourly day schedules, and keep a 366-day table. CP45 locks only one flat comma CSV with one skipped header, selected column 2, 8760 numeric hourly rows, explicit No interpolation, 60 minutes per item, and DST adjustment No. Lines 1858-1862 alias February 29 to February 28 when the actual selected-column row count is below 8784. |
| shading-file schedule intake and generated columns | `Sched::ProcessScheduleInput` | EnergyPlus 26.1 lines 552-713 resolve the unique `Schedule:File:Shading` sidecar, parse comma input with one header row, and require exactly 365 or 366 days times 24 hours times the model timestep count. Lines 1871-1959 skip column zero and create one `<header>_shading` detailed schedule per unique remaining header. CP50 locks only one common-year 35,040-row, `Timestep,4`, two-surface-column CSV and the first day's two generated Schedule Value vectors; it deliberately omits surfaces and Imported ShadowCalculation. |
| inactive external-interface schedule intake | `Sched::ProcessScheduleInput`; `Sched::ExternalInterfaceSetSchedule`; `ExternalInterface::GetExternalInterfaceInput`; `ExternalInterface::WarnIfExternalInterfaceObjectsAreUsed`; `SimulationManager::ManageSimulation` | EnergyPlus 26.1 `ScheduleManager.cc` lines 490-497 size intake storage; lines 2004-2062 create one detailed schedule plus internal day/week ownership and seed every zone timestep from Initial Value; lines 2198-2219 apply type-limit validation; and lines 2706-2731 fill the day cache. `ExternalInterface.cc` lines 187-192 and 2317-2331 detect the missing BCVTB activation and emit the fixed-value warning. `SimulationManager.cc` lines 493-536 show the live-exchange call before weather/schedule work; CP51 deliberately leaves that interface inactive and locks only the immutable initial value. |
| inactive FMU import-to-schedule intake | `Sched::ProcessScheduleInput`; `Sched::ExternalInterfaceSetSchedule`; `ExternalInterface::GetExternalInterfaceInput`; `ExternalInterface::WarnIfExternalInterfaceObjectsAreUsed`; `SimulationManager::ManageSimulation` | EnergyPlus 26.1 `ScheduleManager.cc` lines 499-504 size this source-ordered family after `ExternalInterface:Schedule`; lines 2065-2130 retain the FMU file/instance/variable binding strings, create detailed day/week ownership, and seed all 366 days from Initial Value; lines 2198-2219 own the later type-limit pass; and lines 2706-2731 fill the day cache. `ExternalInterface.cc` lines 201-205 and 2317-2331 emit the missing-activation warning; lines 1331-1430 and 705-708 own live binding and updates. `SimulationManager.cc` lines 493-536 provide exchange-order context. CP52 omits both activation and provider objects and locks only the immutable initial scalar. |
| inactive FMU export-to-schedule intake | `Sched::ProcessScheduleInput`; `Sched::ExternalInterfaceSetSchedule`; `ExternalInterface::GetExternalInterfaceInput`; `ExternalInterface::WarnIfExternalInterfaceObjectsAreUsed`; `SimulationManager::ManageSimulation` | EnergyPlus 26.1 `ScheduleManager.cc` lines 483-513 place this family after FMU Import; lines 2133-2196 read the actual inner `schedule_name` despite conversion's synthetic outer key, retain the FMU variable name, apply the optional Initial Value default of 0.0 when omitted, initialize detailed day/week ownership, and seed all 366 days; lines 2198-2219 own the later type-limit pass and lines 2706-2731 fill the day cache. The bounded CP53 gate explicitly supplies 0.875 and does not promote the omitted-default branch. `ExternalInterface.cc` lines 176-199 and 2317-2331 own activation counting and the missing-activation warning; lines 98-120, 370-580, and 2118-2251 own socket/configuration, binding, live exchange, and mutable updates. `SimulationManager.cc` lines 493-536 and 559-560 provide timestep/final-exchange order context. CP53 leaves the interface inactive and locks only the immutable initial scalar. |
| day/week/year schedule intake and annual pointer expansion | `Sched::ProcessScheduleInput` | EnergyPlus 26.1 lines 803-854 load 24-value `Schedule:Day:Hourly` profiles, lines 1046-1078 resolve all 12 `Schedule:Week:Daily` day-type pointers, and lines 1149-1246 expand source-ordered `Schedule:Year` ranges into a 366-day Week table. CP46 locks only two non-wrapping Year ranges whose intentionally unassigned day 60 copies day 59's Week pointer at lines 1223-1227. |
| interval day-schedule intake | `Sched::ProcessScheduleInput` | EnergyPlus 26.1 lines 858-932 parse `Schedule:Day:Interval` after all Day:Hourly objects and before later Day/Week families. CP47 locks one blank/default-No aligned profile plus Average and Linear non-aligned profiles at `Timestep,4`, reusing `ProcessIntervalFields` and `DaySchedule::populateFromMinuteVals` minute-to-zone-timestep semantics. |
| schedule current values | `Sched::UpdateScheduleVals` | Writes every schedule's `currentVal`: an EMS value wins when actuated; otherwise it calls `getHrTsVal(state, HourOfDay, TimeStep)`. It does not calculate or advance calendar state. |
| detailed schedule lookup | `Sched::ScheduleDetailed::getHrTsVal` | Reads `DayOfYear_Schedule`, weekday, holiday index, `DSTIndicator`, hour, and zone timestep. It applies schedule-specific DST use, rolls DST-shifted hour 24 into tomorrow's weekday/holiday, selects holiday or weekday day schedule, and returns the indexed timestep value. |
| constant schedule lookup | `Sched::ScheduleConstant::getHrTsVal` | Returns the stored constant timestep value independently of date and hour; EMS override remains the caller's current-value policy. |
| timestamp serialization | `OutputProcessor::WriteTimeStampFormatData` | Serializes already-resolved day-of-simulation, month, day, hour, minute bounds, DST indicator, and day type to ESO/MTR and, when requested, the SQL time index. It does not derive or advance calendar state. |

The `getHrTsVal` token above refers to the virtual schedule lookup contract and
its concrete detailed/constant implementations. The `UpdateScheduleVals` and
`WriteTimeStampFormatData` tokens are deliberately retained because schedule
and output parity must be checked at these exact ownership boundaries.

## AirBoundary Schedule-Identity Consumer Boundary

CP88 adds one declaration-only consumer of the completed top-level schedule
namespace. `Construction:AirBoundary` remains ordered after ordinary and F/C
constructions, but Rust materializes only that family after `Schedule:Constant`,
`Schedule:Compact`, `Schedule:File`, Day/Week/Year, ExternalInterface, and both
FMU schedule families so an active `SimpleMixing` name can resolve to the
existing `ScheduleId`. This local pass-order deviation is recorded in the
heat-balance source map and does not claim complete `GetConstructData` order
parity.

A missing or blank active schedule uses an explicit
`AirBoundaryMixingSchedule::AlwaysOn` selector; it does not synthesize a new
`Schedule:Constant`, allocate another schedule ID, or change schedule object
coverage. `None` does not resolve its inactive schedule name. Both selectors
remain compile-time identities only: `Sched::UpdateScheduleVals`, `currentVal`,
`getHrTsVal`, generated `ZoneCrossMixing`, AirflowNetwork interaction, mixing
flow calculations, and every runtime/output consumer remain unsupported and
run-blocked. CP88 adds no schedule conformance case or proof variable.

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

## Evidence navigation

This map records reusable source ownership and the scope of the legacy
calendar/report fixtures. Fixture inputs, expected vectors, warnings, and
comparison commands belong in the [case index](../generated/conformance-case-index.md)
and its manifests and scripts. The
[algorithm ledger](../../../specs/algorithm_ledger.toml) owns promotion status.
The later [finite A/B port](../../../energyplus_porting_plan/README.md) has its
own completed card contracts and accepted evidence; the historical fixture
limits below do not redefine those cards. Earlier experiment narratives are
available in Git history.

## Ordered Exact Hourly and Weather-Effective Leap Evidence Checkpoint

`calendar_schedule_hourly_exact_001` and
`calendar_schedule_weather_leap_policy_no_001` isolate EPW leap policy over
2016-02-28 through 2016-03-01: Yes retains 72 hours; No discards the raw leap
day and retains 48. Gregorian, weather-effective, and leap-shaped schedule
ordinals remain separate. In the No case March 1 is respectively 61, 60, and
61; the skipped day does not advance the simulation weekday. These internal
ordinal fields are source/unit evidence, not emitted oracle fields.

For a no-leap February 29 start, literal raw-record search occurs before leap
day discard. A present February 29 can therefore supply the search point for
a March 1 effective day; a missing literal start cannot. Effective duration is
not simply Gregorian duration minus discarded dates.

The `ordered-exact-unique` contract requires timestamps on every sample,
unique ordered labels, equal lengths, and exact label equality before numeric
comparison. These are normalized runtime/parser labels. Raw ESO text,
subhourly weather, and general environment traversal are separate contracts.

## Schedule:Compact Through/For Day-Type Evidence Checkpoint

`calendar_schedule_compact_through_for_day_type_hourly_exact_001` isolates
two source-ordered Through periods and For-day selection over one non-actual
year boundary. `AllOtherDays` is the complement of earlier For selections
within its own period. Lookup uses the leap-shaped schedule ordinal, resolved
day type, and hour-ending interval. Its 120 hourly values are the declared
`103/104/105/108/199` daily profile.

Calendar-varying or unresolved `OtherEquipment` references fail closed in the
legacy hour-only gain consumers. This fixture does not complete
`Sched::UpdateScheduleVals` or establish downstream gain/HVAC consumption.

## Schedule:Compact DST Hour-24/Tomorrow Day-Type Evidence Checkpoint

`calendar_schedule_dst_hour24_tomorrow_day_type_exact_001` isolates the
three-day 2032-10-30 through 2032-11-01 schedule clock. Current DST advances
lookup by one hour. A shifted hour above 24 advances the leap-shaped ordinal
and uses the next day's type; at the final no-prefetch boundary it retains the
final day's type. Thus the fixture distinguishes current hour 24, tomorrow
Holiday, and final stale-Holiday selection.

This fixture does not
promote `calendar_time_state` beyond scaffold and does not complete
`Sched::UpdateScheduleVals`, EMS priority, or general `getHrTsVal` parity.
Schedule-specific DST opt-out and combinations with subhourly interpolation
remain outside this fixture.

## Schedule:Compact Default-No Zone-Timestep Evidence Checkpoint

`calendar_schedule_compact_zone_timestep_exact_001` isolates omitted
`Interpolate` at `Timestep,4` with aligned endpoints. The default No path fills
minute values flat; `DaySchedule::populateFromMinuteVals` samples the ending
minute, and `getHrTsVal` uses the one-based timestep slot. It locks one
immutable 96-sample day, beginning `11/12/13/14` and then `90`.
For this omitted-directive fixture, this is not evidence for accepting an explicit
`Interpolate:No` field; the explicit-mode fixture owns that separate claim.

This fixture does not promote `calendar_time_state` beyond scaffold.
Explicit `Interpolate` directives and non-aligned warning behavior are covered
only by their separate declared fixtures. The subhourly DST/year-wrap unit
checks are internal evidence only.

## Schedule:Compact Explicit Interpolation Evidence Checkpoint

`calendar_schedule_compact_interpolation_modes_exact_001` isolates explicit
No/Average/Linear at `Timestep,4`. `ProcessIntervalFields` fills an
intermediate 1,440-minute lattice, then an immutable 96-value day cache.
No/Average fill intervals flat; Linear keeps the first interval flat and
advances later minutes from the preceding declared value. No and Linear sample
the ending minute; Average takes the 15-minute window mean. The first five
values are respectively `10/175/175/175/175`, `10/120/175/175/175`, and
`10/40/85/130/175`.

This fixture does not promote `calendar_time_state` beyond scaffold. Its
non-multiple Until warning belongs only to explicit No; Rust warning text/count parity remains
unclaimed. Other timestep counts, invalid/overlapping profiles, DST with
interpolation, and downstream consumption require their own evidence.

## Schedule:File 8760-Row Leap-Day Evidence Checkpoint

`calendar_schedule_file_8760_leap_exact_001` covers a byte-staged CSV,
one-based column 2, one skipped header, 8760 hourly items, explicit No
interpolation, and disabled schedule DST. `ProcessScheduleInput` owns file
resolution/intake and the immutable 366-day table. When the selected column
has fewer than 8784 hourly-equivalent values, schedule day 60 reuses day 59
without consuming a source day; later ordinals shift back one source day.
The paired leap-day profile is therefore repeated rather than synthesized.

8784-row behavior, other separators or item lengths, row-count mismatch and
file-search diagnostics, interpolation Yes, active DST, and arbitrary-run
sidecar staging are outside this fixture.

## Schedule:File:Shading Generated-Schedule Evidence Checkpoint

`calendar_schedule_file_shading_exact_001` covers a 365-day, four-timestep
CSV with `Date/Time` then two unique surface headers. Intake excludes column
zero, creates `<header>_shading` schedules, retains immutable column values,
and indexes them by schedule day/hour/timestep. The source row-count check is
calendar days times 24 times `TimeStepsInHour`.

The fixture contains no surfaces or Imported `ShadowCalculation`; it proves
CSV intake, generated identity, column separation, and first-day lookup.
Timestamp validation, duplicate headers, leap-year rows, surface consumption,
incident solar, and heat-balance effects are separate obligations.

## ExternalInterface:Schedule Inactive Initial-Value Evidence Checkpoint

`calendar_schedule_external_interface_initial_value_exact_001` covers the
inactive initial scalar `0.375`. `ProcessScheduleInput` resolves the optional
type limit, builds internal day/week ownership, and points all 366 days at the
initialized week. `ExternalInterfaceSetSchedule` fills every hour/timestep.
With no activation object, `WarnIfExternalInterfaceObjectsAreUsed` reports
that values will not be updated. Rust stores one immutable scalar.

Live exchange precedes weather/schedule work in `ManageSimulation`, but this
fixture does not exercise that path, BCVTB configuration, current-value
mutation, EMS precedence, warmup exchange, or downstream consumption.

## FMU Import To Schedule Inactive Initial-Value Evidence Checkpoint

`calendar_schedule_fmu_import_initial_value_exact_001` covers the inactive
`0.625` scalar with inert file/instance/variable metadata and no activation or
provider. Intake follows ordinary ExternalInterface schedules, applies the
shared case-insensitive schedule namespace, then uses the same internal
population and 366-day pointers. The missing-activation warning is part of
the case's oracle contract.

Provider lookup, FMU access/unpacking, FMI binding, live or warmup updates,
causality checks, and the activated-without-provider no-op branch are outside
this immutable initial-value claim.

## FMU Export To Schedule Inactive Initial-Value Evidence Checkpoint

`calendar_schedule_fmu_export_initial_value_exact_001` covers the inactive
`0.875` scalar. The semantic identity is the inner `schedule_name`, not the
converter's synthetic outer epJSON member key. Intake follows FMU Import,
resolves the shared schedule namespace/type limit, and initializes the same
366-day ownership. Binding metadata remains inert without activation.

Socket/configuration files, live binding/update, final exchange, warmup,
provider resolution, mutable current values, and broad diagnostic parity
remain separate contracts.

## ScheduleTypeLimits Bounded Scalar Evidence Checkpoint

`calendar_schedule_fmu_export_bounded_initial_value_exact_001` and its
`..._failure_001` pair isolate Continuous two-sided inclusive limits.
`ScheduleDetailed::setMinMaxVals` derives extrema and
`ScheduleBase::checkMinMaxVals` compares them with `FLT_EPSILON`. Equality at
upper bound `0.875` is accepted; upper bound `0.5` rejects the same scalar
before numeric output. Rust emits `ScheduleValueOutsideTypeLimits` during
typed compilation, publishes no model, and creates no runtime samples.

The Rust scalar validator also serves Constant, ordinary ExternalInterface,
and FMU Import; only the declared FMU Export pair supplies this external
claim. Rejection is semantic evidence, not equal numeric exit status or
identical diagnostics. Discrete, one-sided/default bounds, detailed-schedule
range validation, and live updates require separate evidence.

## Schedule:Day/Week/Year Leap-Table Evidence Checkpoint

`calendar_schedule_day_hourly_week_daily_year_leap_exact_001` covers immutable
24-hour Day arrays, all 12 Week:Daily pointers, and a 366-pointer Year table.
Families compile in source order before lookup. An unassigned February 29
Year slot inherits day 59's Week pointer; it still selects February 29's
own day type. Lookup then follows ordinal, Week day type, hour, and timestep.

Defaults, malformed/missing/overlapping Year ranges, DST/tomorrow behavior,
and downstream consumers are outside this fixture's 72-hour claim.

## Schedule:Day:Interval Three-Mode Evidence Checkpoint

`calendar_schedule_day_interval_modes_exact_001` uses the shared DaySchedule
namespace after Hourly and before List/Week processing. Its minute expansion
has the same flat No/Average and first-flat-then-ramp Linear semantics as
Compact; Average takes the timestep-window mean, the other modes sample the
ending minute. A Week:Daily/Year chain selects the three profiles.

The three-day `Timestep,4` fixture covers 288 samples. Other interval shapes,
No-mode unaligned warnings, hourly aggregation, type-limit diagnostics,
DST/tomorrow, and downstream consumption remain separate obligations.

## Schedule:Day:List Source-Actual Modes Evidence Checkpoint

`calendar_schedule_day_list_modes_exact_001` follows Interval in the shared
Day namespace. Minutes per Item must be a positive integer dividing 60, with
exactly `1440 / minutes` values. Each source value repeats across its allotted
minutes. Unlike Interval, List does not create a Linear ramp: source-actual
`Linear == No`, both sampling the ending minute; Average takes the window
mean. The fixture isolates 20-minute items at `Timestep,4`.

The documentation-intended Linear ramp, other item/timestep counts, malformed
counts or defaults, aggregation, DST/tomorrow, and downstream consumers are
outside that claim.

## Schedule:Week:Compact Day-Type Evidence Checkpoint

`calendar_schedule_week_compact_day_types_exact_001` resolves each ordered
Day reference before `ProcessForDayTypes`. Weekdays expands Monday-Friday,
Weekends Sunday/Saturday, and the five special tokens fill the remaining
slots. Duplicate selection is an error; every one of the 12 slots must be
assigned. Week:Compact follows Week:Daily in their shared ID space, then Year
consumes the completed pointer table.

The seven-day fixture covers its declared day-type selectors and 672
samples. AllDays/AllOtherDays, arbitrary combined tokens, missing/duplicate
reference diagnostics, other pair orders, DST/tomorrow, and downstream
consumption are separate obligations.

## Source-Order EPW Record Selection Checkpoint

`EpwDataPeriods` retains declared period count, records per hour, start
weekday, and endpoint dates, including logically continued header lines.
For the supported non-actual branch, `select_epw_environment_weather`
requires one declared period covering both environment endpoints, searches
literal month/day with `MatchYear=false`, and validates each accepted
hour-1-through-hour-24 day. It returns environment order rather than assuming
source row zero.

The first accepted day starts in Tomorrow; BeginDay commits Today and
prefetches only while another simulation day remains. Final Tomorrow retains
Today. Leap-day search precedes discard. Day-local solar `NextHr` wraps hour
24 to hour 1 of the same source day; one timestep/hour uses weights `(0,1,0)`.
These buffer/solar details are source/unit contracts, distinct from the
external `weather_record_start_offset_nonactual_001` fixture's start index
24 and 48 dry-bulb samples after decoy rows.

The selector also feeds `ep_run::prepare_runtime_inputs`. That wiring does
not by itself prove actual-weather year matching, multiple periods,
subhourly processing, missing-value repair, or full `SetCurrentWeather`.

## EPW Daylight-Saving Evidence Checkpoints

EPW fixed/Nth/last-weekday boundaries are typed calendar rules; both zeros
mean no period, a one-sided zero is invalid, and the Julian-number form is
unsupported. Weather-file DST is gated by the RunPeriod flag; input-file DST
wins independently of that flag. Ranges include both endpoint ordinals.
Nth/last selection preserves RunPeriod `MonWeekDay` before environment
`LeapYearAdd`; final ordinals use the weather-effective year.

The fixed-date enabled/disabled pair, one input-file-precedence case, one
literal fourth-Monday/last-Wednesday pair, and separate southern-wrap start
and end windows are the external boundaries. Their manifests retain the
exact dates, daily vectors, EIO rows, and zero tolerances. Summary ordinals
and `wraps_year` are internal diagnostics. These windows do not imply
full-year traversal, other rules/years, schedule clock parity, or raw ESO
serialization.

`calendar_dst_epw_idf_precedence_hourly_exact_001` owns the input-file case.
JSON `input_file_period_declared` and `effective_source`, and Markdown
`input_file_daylight_saving_period_declared`, are summary diagnostics rather
than additional EnergyPlus EIO output.

## Cross-Year EPW DST Start-Year Projection Evidence Checkpoint

`calendar_epw_dst_cross_year_start_year_projection_hourly_exact_001` isolates
2031-12-30 through 2032-01-02 with one wrapping DATA PERIOD. The Nth-weekday
range is resolved against the 2031 start-year annual table; the 96-value
boundary retains daily DST `0/0/0/1`. Source order places January 2 prefetch
before the later January 1 table reset. EnergyPlus 26.1 source separately maps the January 2 result
to that prefetch; that internal buffer order is not an ESO/EIO-emitted field.
January 3/later reset, multiple boundaries/periods, actual
weather, February 29 coupling, and schedule-clock effects are not covered.

## Fixed-Date IDF Special-Day Evidence Checkpoint

`RunPeriodControl:SpecialDays` retains identity, calendar rule, positive
duration, and special type. Projection changes effective day type while
preserving the simulation weekday. Enabled EPW definitions resolve first,
then IDF definitions overwrite in validated declaration order. The IDF order
overlay applies only to this family; native epJSON remains name-ordered.
For the declared cross-year branch the start-year annual table is retained.
`calendar_special_day_fixed_date_hourly_exact_001` isolates one duration-one
February 29 Holiday; other cases below own the additional boundaries.

## Single-Boundary Cross-Year Start-Year Projection Evidence Checkpoint

`calendar_special_day_cross_year_start_year_projection_hourly_exact_001`
retains the 2031 start-year resolution of `1st Thursday in January` as
January 2 across the single 2031-12-30 to 2032-01-02 boundary. It does not
reproject to Gregorian 2032 January 1. Later annual resets, multiple year
boundaries/DATA PERIODS, actual weather, and February 29 coupling are outside
this case.

## Same-Year Special-Day Duration-Wrap Evidence Checkpoint

The common-/leap-year `calendar_special_day_duration_wrap_*` cases isolate
one December 31 duration-three Holiday in a same-year January RunPeriod.
`SetSpecialDayDates` wraps 366 to 1 in a common year and 367 to 1 in a leap
year. This is cyclic annual-table projection, not cross-year execution or
per-year reprojection.

## Overlapping IDF Special-Day Declaration-Order Evidence Checkpoint

The `calendar_special_day_overlap_*` pair reverses two duration-one
February 29 definitions. The later definition wins after IDF order recovery;
there is no claim about native epJSON declaration order, other object families,
more than two definitions, or partially overlapping durations.

## Nth/Last-Weekday IDF Special-Day Evidence Checkpoint

The explicit 2032 fourth-Sunday and last-Sunday February cases cover only those
two rules. Their source Sunday resolution has zero weekend shift, distinct
from fixed-date weekend observation. Other ordinals/years and EPW forms use
their own evidence.

## Nonexistent Fifth-Weekday Expected-Failure Checkpoint

`calendar_special_day_nonexistent_fifth_weekday_failure_001` is a blocking
negative case for February 2016's nonexistent fifth Sunday. Date resolution
fails before weekend observation can act. Original exits 1 through Severe
then Fatal; Rust exits 6 with runtime `RuntimeConvergenceFailure`. Both have
zero hourly data. These are engine-specific interface contracts, not equal
exit codes, numerical conformance, or general diagnostic parity.

## Fixed-Sunday IDF Weekend-Observation Policy Evidence Checkpoint

The `calendar_special_day_weekend_rule_*` three-case fixture isolates fixed
Sunday February 28, 2016: explicit Yes and genuinely blank IDF fields observe
Monday; No leaves Sunday. The executable-observed blank behavior differs from
the published IDD/schema default No. It does not establish omitted-field or
native-epJSON defaults. EIO Special Days date text is not used as the effective
date oracle.

## Fixed-Saturday IDF Weekend-Observation Policy Evidence Checkpoint

The `calendar_special_day_weekend_saturday_*` explicit Yes/No pair isolates
Saturday February 27, 2016. Yes shifts by two days to Monday; No retains
Saturday. Blank behavior and other calendars/rules are outside this pair.

## Fixed-Date EPW Holiday Use-Policy Evidence Checkpoint

The `calendar_epw_holiday_fixed_date_*` enabled/disabled pair covers the
RunPeriod use flag. Enabled EPW holidays have source-exact Sunday/index 1,
rather than IDF Holiday/index 8; disabling the flag filters EPW definitions.
IDF-definition independence is source/unit evidence in these cases because
they contain no IDF special day.

## EPW Nth/Last-Weekday Holiday Evidence Checkpoint

`calendar_epw_holiday_weekday_rules_hourly_exact_001` covers only fourth
Monday and last Wednesday in February 2032, in that header order. Both project
Sunday/index 1. The EIO definition order/source is checked; its date fields
are not promoted as effective-date evidence. Other rules, overlaps, weekend
shifts, and diagnostic parity remain outside this fixture.

## EPW-versus-IDF Special-Day Precedence Evidence Checkpoint

`calendar_special_day_epw_idf_precedence_hourly_exact_001` covers one mixed
fixed-date collision. The enabled EPW Holiday resolves first, then IDF
CustomDay1 overwrites it, producing daily indices `1/11/3`. Additional
collisions/types/orders and broader warning, traversal, or schedule behavior
require separate evidence.

## Immutable Schedule Cache Infrastructure Checkpoint

This checkpoint changes Rust representation and consumer wiring only. It adds
no EnergyPlus oracle case, algorithm promotion, object promotion, or variable
promotion. The existing schedule evidence and tolerances remain the numerical
authority.

The immutable cache is owned by these Rust targets:

| Rust target | Infrastructure responsibility |
|---|---|
| `ep_runtime::schedules::ScheduleSeriesCache` | owns one source-ordered set of schedule entries for one time axis and provides typed-ID lookup |
| `ep_runtime::schedules::ScheduleSampleStorage` | represents an immutable repeated scalar without a dense sample array, or a varying series as a boxed dense array |
| `ep_runtime::schedules::ScheduleCacheProfile` | reports deterministic scalar/dense series counts, logical and dense-allocated sample counts, index kind, and distinct ambiguous-ID count |
| `ep_runtime::schedules::HeatBalanceInternalGainScheduleOperationProfile` | records simulation-owned, deterministic build, initialization, warmup, and run-period operation counts for the referenced-only `OtherEquipment` cache without mutating cache state |
| `precompute_schedule_cache_for_time_axis` and `precompute_schedule_cache_for_environment_time_axis` | compile the existing supported schedule families against the shared hourly or environment time axis |
| the legacy `precompute_schedule_value_series*` functions | materialize `Vec` traces from the cache so existing callers retain their public return types and numerical behavior |
| `ep_cli` schedule-report consumers | own the cache across report generation and resolve each requested schedule once by typed ID rather than scanning legacy traces |
| `ep_run::PreparedRuntimeInputs` | owns and prepares the cache, then reports its structural profile without claiming numerical heat-balance or IdealLoads consumption |
| `precompute_hour_only_internal_gain_schedule_cache` | validates only `OtherEquipment` references, then builds one referenced-only 24-sample cache for the existing hour-only Constant, inactive ExternalInterface/FMU, and calendar-invariant Compact subset |
| Rust heat-balance `OtherEquipment` consumers | reuse the same referenced-only cache through initialization, warmup, and run-period timesteps; the public one-step API intentionally retains live-model fallback lookup |

The full-axis hourly and environment cache builders preserve the established
family and source order:

```text
Schedule:File:Shading generated columns
  -> Schedule:Constant
  -> Schedule:Compact
  -> Schedule:File
  -> Schedule:Year
  -> ExternalInterface:Schedule
  -> ExternalInterface:FunctionalMockupUnitImport:To:Schedule
  -> ExternalInterface:FunctionalMockupUnitExport:To:Schedule
```

Within those full-axis builders, sparse, out-of-order, high, and duplicate
public `ScheduleId` values do not size an array by the maximum ID.
Identity-ordered IDs use direct slot lookup; all other shapes use a bounded
sorted-pair index. Duplicate lookup remains compatible with the former
source-order scan: the first source-order entry wins, while every entry remains
present during source-order iteration and the profile records the number of
distinct ambiguous IDs. The specialized internal-gain builder instead
deduplicates IDs by first `OtherEquipment` declaration reference, then resolves
each selected ID in Constant, ExternalInterface, FMU import, FMU export, and
Compact family order with first-in-family lookup.

`Schedule:Constant`, inactive `ExternalInterface:Schedule`, inactive FMU import
To Schedule, and inactive FMU export To Schedule each store one scalar plus the
logical sample count. The structural tests require four such families to report
zero dense allocated samples. Varying Compact, File, File:Shading, and Year
series remain dense.

The compatibility invariant is the legacy numerical result, not a new
EnergyPlus behavior claim. Cache tests compare legacy and cache entry identity,
name, kind, length, source order, and every value; finite values retain their
exact `f64` bits and existing fail-closed NaN positions remain NaN. The legacy
adapters deliberately rematerialize scalar arrays, so their allocation behavior
is not performance evidence.

Rust unit and structure tests now prove that the validated hour-only
`OtherEquipment` heat-balance path consumes one immutable referenced-only cache
through initialization, warmup, and run-period timesteps. Cached lookup preserves
fallback bits, family priority, first-wins duplicates, fail-closed NaN, hour
clamping, substep reuse, and daily reuse; repeated gain lookup no longer scans
schedule families or reparses Compact segments. A separate public one-step test
keeps the live-model fallback boundary explicit.

The full-axis `ScheduleCacheProfile` remains deterministic representation and
allocation evidence only. Separately, the simulation-owned
`HeatBalanceInternalGainScheduleOperationProfile` measures the referenced-only
hour-only `OtherEquipment` cache and excludes `ep_cli` full-axis precomputation
and the public one-step fallback. It records one successful cache construction,
the cache entry and logical-sample counts, and Compact value evaluations during
construction, then separates indexed cache reads for initialization, warmup,
and the reported run period. A referenced calendar-invariant Compact profile is
evaluated 24 times while its 24-sample cache entry is built. During those three
simulation phases the cached path records zero live fallback lookups, zero
entries into the legacy schedule-object-family chain, zero Compact profile
resolutions, and zero Compact value evaluations. The counters belong to the
simulation result; the cache remains immutable.

Zero schedule-family-chain scans does not mean zero remaining work:
`OtherEquipment`, zone, surface, and people iteration remains. These
deterministic operation counts are not wall-clock timing, a cache-specific
speedup attribution, or a new numerical compatibility claim. This checkpoint
adds no EnergyPlus numerical heat-balance or Schedule Value promotion and does
not claim Schedule:File, Schedule:File:Shading, Schedule:Year, or calendar-aware
downstream consumption; IdealLoads, HVAC availability, or thermostat
consumption; a weather-cache redesign; cache sharing across environments or
repeated runs; live EMS/current-value mutation; live ExternalInterface/FMU
exchange; warning parity; broad ScheduleManager completion; or elimination of
lookups in other runtime hot paths.

## Current Rust Boundary

| Boundary | Current Rust status | Missing source behavior |
|---|---|---|
| run-period input | typed dates, optional years, and start weekday feed EnergyPlus-style year and weekday resolution; the first-hour policy is carried on the axis, the weather-file DST use flag gates a parsed EPW period, while typed IDF `RunPeriodControl:DaylightSavingTime` overrides that source independently of the flag. External DST evidence covers the paired fixed-date Yes/No policy cases, the exact fixed-date IDF-over-disabled-EPW precedence case, one literal fourth-Monday-through-last-Wednesday boundary pair, and the paired exact last-Sunday-October-through-last-Sunday-March end-side and start-side wrap cases, and the weather-file holiday use flag gates parsed fixed and weekday-rule EPW holiday definitions. The exact EPW weekday case locks enabled policy only. The fixed-Sunday gate locks explicit Yes/No plus the EnergyPlus 26.1 executable's blank-A5-as-Yes branch, the fixed-Saturday gate locks explicit Yes/No, typed IDF `RunPeriodControl:SpecialDays` definitions feed the shared day-type projection, IDF-backed loads preserve the validated SpecialDays declaration order used by the exact overlapping pair so the later definition wins, and the exact mixed-source case locks weather-file-then-input-file resolution when one later IDF CustomDay1 overrides an enabled EPW Holiday on the same fixed date. Metadata-aware actual-weather inputs fail explicitly; non-actual cross-year input and typed special-day projection are externally locked only for the single start-year-retention fixture | custom ranges, design-day environments, environment filtering, declaration-order recovery for object types beyond SpecialDays, overlap shapes beyond the exact pair, omitted-A5/native-epJSON default behavior, blank/default behavior for other fields, EPW holiday policy combinations beyond the fixed pair and one enabled weekday case, EPW-versus-IDF precedence beyond the one exact mixed collision, other EPW DST boundary forms, broader years/months/rules, generic southern/wrap behavior, actual-weather execution or non-actual cross-year execution beyond the exact single-boundary fixture, other DST policy/source combinations and IDF objects beyond the exact fixed-date precedence case, actual-weather traversal, cross-year weather traversal beyond that fixture, later annual resets or per-year special-day reprojection, and full EnergyPlus warning-text parity |
| canonical calendar | `ResolvedRunPeriodCalendar` retains Gregorian interpretation, while non-actual `ResolvedWeatherEnvironmentCalendar` applies the EPW leap-year header (including the February 29 endpoint ordinal alias). The metadata-aware axis resolves an enabled EPW DST rule into inclusive daily `dst` state, enabled EPW holidays into source-exact Sunday day type, and typed IDF special-day rules into effective `DayType`/`special_day_type`; `EnvironmentTimePoint` separately owns Gregorian, weather-effective, and schedule day-of-year plus simulation weekday. External numerical DST evidence is limited to the paired fixed-date Yes/No policy cases, the exact fixed-date input-file-over-EPW precedence case with 72 samples in daily status 1/1/0 and 48 active hours, the literal 2032 `4th Monday in February` through `Last Wednesday in February` pair, and the paired southern-wrap cases' 72 EnergyPlus ESO values/timestamp fields over the same-year March end-side 1/1/0 and October start-side 0/1/1 windows plus shared EIO dates 10/31 and 03/28; Rust day-of-year 305/88 and `wraps_year=true` are separate source-mapped diagnostics not emitted by EIO. The exact cross-year DST fixture separately locks one non-actual 2031-12-30 through 2032-01-02 traversal: 96 exact samples in daily status 0/0/0/1, 24 active and 72 inactive hours, exact EIO dates 01/02 through 01/03, and source-mapped January 2 prefetch retention whose internal buffer order is not EIO/ESO output. External numerical special-day evidence is limited to the fixed duration-one IDF Holiday, the paired common-/leap-year December 31 duration-three annual-table wraps, the exact 2031-to-2032 start-year annual-table-retention case, the exact 2032 `4th Sunday in February` and `Last Sunday in February` IDF forms, the fixed-Sunday explicit-Yes/explicit-No/blank and fixed-Saturday explicit-Yes/explicit-No weekend-policy cases, the paired fixed EPW holiday use-policy cases, the exact EPW `4th Monday in February` then `Last Wednesday in February` header, the paired fixed-date SpecialDays declaration-order/overwrite cases, and the one fixed-date EPW-Holiday-then-IDF-CustomDay1 precedence case. The CP42 detailed-schedule case separately consumes current DST and one tomorrow Holiday/final-stale day-type path; it does not promote the remaining calendar family. The blocking smoke/nonclaim fifth-Sunday case separately locks only rejection of the explicit nonexistent 2016 rule and zero produced samples | warmup lifecycle, other DST years/months/date-rule pairs, generic southern/wrap behavior, actual-weather execution or cross-year execution beyond the exact single-boundary fixtures, other RunPeriod policy/source combinations and input-file DST objects beyond the exact fixed-date precedence case, declaration order beyond SpecialDays and the exact EPW two-rule header, overlap behavior beyond the exact pair, warning-text/repetition parity, weekend shifting beyond the fixed-Sunday plus-one-day Yes/blank and No cases and fixed-Saturday plus-two-day explicit Yes/No pair, other blank/default or omitted policy inputs, other Nth ordinals/weekdays/months/years/strings beyond the exact IDF and EPW successes and one IDF failure, other nonexistent-occurrence combinations or multi-error behavior, other special-day types, durations/dates beyond the paired exact annual-table wraps, multiple or overlapping holidays beyond the exact two nonoverlapping EPW rules, EPW-versus-IDF precedence beyond the one exact mixed collision, schedule/timestamp DST or special-day consumers beyond the exact CP42 detailed-schedule path, tomorrow special-day state beyond its one tomorrow-Holiday/final-stale boundary, actual-weather behavior, cross-year weather traversal beyond those fixtures, later annual resets including January 3 DST state, or per-year special-day reprojection, EnergyPlus `Timestep` default/invalid-value normalization, and environment kinds beyond weather run periods |
| hourly consumers | `TimeAxis` is an hour-ending projection of the resolved environment calendar; the paired calendar cases lock 72 leap-observed labels ending Tuesday versus 48 no-leap-policy labels ending simulation Monday from the same IDF and 72 raw EPW rows. The fixed-date DST policy pair locks 72 ordered state/timestamp samples per case: enabled daily status 0/1/1 with 48 active hours versus disabled 0/0/0 with zero active hours. The fixed-date IDF-over-EPW precedence case locks 72 ordered samples in daily status 1/1/0 with 48 active hours and effective source input-file. The literal 2032 EPW weekday-boundary case locks 120 ordered samples as daily status 0/1/1/1/0 with 72 active hours. The southern-wrap end-side case locks only the same-year March window: 72 ordered samples as daily status 1/1/0 with 48 active and 24 inactive hours. The paired start-side case locks the same-year October 30 through November 1 window as 72 ordered samples in daily status 0/1/1 with the same 48 active and 24 inactive hours. The cross-year start-year DST case locks 96 ordered samples in daily status 0/0/0/1 with 24 active and 72 inactive hours; January 2 prefetch retention is source-mapped internal ordering, not an ESO/EIO-emitted field. The fixed IDF special-day case locks 72 ordered day-type/timestamp samples as 24 each of 1/8/3. The cross-year start-year case locks 96 ordered samples in daily indices 3/4/5/8 with January 2 Holiday. The duration-wrap pair locks 72 ordered samples per case and 48 Holiday samples per case: common-year 2017 starts at day of year 365 and uses daily indices 8/8/3, while leap-year 2016 starts at day of year 366 and uses 8/8/1. The two exact 2032 IDF weekday-rule cases each lock 216 ordered samples: fourth Sunday uses daily indices 8/2/3/4/5/6/7/1/2, and last Sunday uses 1/2/3/4/5/6/7/8/2. The nonexistent fifth-Sunday blocking smoke case separately locks engine-specific rejection and zero produced samples without adding numerical day-type evidence. The fixed-Sunday weekend-policy cases lock 72 samples each as explicit Yes and blank 24 each of 1/8/3, with identical values and timestamps, and explicit No 24 each of 8/2/3. The fixed-Saturday weekend-policy pair locks 72 samples each as enabled 24 each of 7/1/8 and disabled 24 each of 8/1/2. The paired fixed EPW holiday cases lock 72 samples each as enabled 24 each of 1/1/3 and disabled 24 each of 1/2/3; the exact EPW weekday-rule case locks 72 enabled samples as 1/3/1. The overlapping SpecialDays pair locks 72 samples each: Zulu then Alpha makes later CustomDay1 win as 1/11/3, while Alpha then Zulu makes later Holiday win as 1/8/3. The mixed EPW/IDF precedence case locks 72 samples as 1/11/3 when the later IDF CustomDay1 overrides the enabled EPW Holiday. The exact Through/For case separately locks one five-day calendar-aware Schedule:Compact series as daily 103/104/105/108/199 with 120 ordered zero-tolerance values and timestamps. The CP42 detailed-schedule case separately locks 72 values as 100x23+124, 200x23+801, and 800x23+901, including current DST +1-hour lookup, next-ordinal/tomorrow-Holiday selection, and the final stale Holiday type. The CP43 default-No case separately locks one aligned `Timestep,4` Schedule:Compact series as exactly 96 values and normalized 15-minute endpoint labels: 11/12/13/14 followed by 90 for 92 samples. The CP44 explicit-mode case separately locks three one-day `Timestep,4` series with 96 values each: No 10/175/175/175/175, Average 10/120/175/175/175, and Linear 10/40/85/130/175 followed by 175. Weather-required heat-balance `ep_run` setup now builds the same metadata-aware axis before runtime execution | runtime consumers outside those weather-required heat-balance classes, DST behavior beyond the fixed-date Yes/No pair, the one exact fixed-date IDF-over-EPW precedence case, one literal 2032 weekday boundary pair, the paired exact southern-wrap end-side and start-side cases, and the exact cross-year start-year case through January 2, special-day behavior beyond the declared fixed-date, exact cross-year start-year projection, paired duration wrap, exact IDF and EPW weekday-rule forms, one exact expected-failure form, the exact two-definition overlap/source-order pair, the one exact mixed-source precedence collision, and policy cases, schedule day-type lookup beyond the exact Through/For and CP42 DST-rollover cases, zone-timestep/subhourly schedule lookup beyond the exact no-DST CP43 default-No and CP44 explicit-mode cases, tomorrow state beyond the one CP42 tomorrow-Holiday/final-stale path, downstream internal-gain/HVAC/IdealLoads consumption of the precomputed Schedule Value series, and all remaining calendar-dependent output semantics |
| EPW weather | `EpwWeatherFile` keeps parsed leap policy, typed optional DST and holiday rules, `DATA PERIODS` metadata, and `EpwRecord` rows. The dedicated hourly report applies the leap policy and enabled DST/holiday policies and selects a complete non-actual, one-record-per-hour stream by source date; external cross-year evidence is limited to the one wrapping DATA PERIOD fixture. The fixed-date DST pair externally locks only the explicit RunPeriod Yes/No policy difference against one shared 2/29-through-3/1 declaration: enabled is 0/1/1 with 48 active hours, while disabled is 0/0/0 with zero active hours; the pair locks exact EnergyPlus 26.1 `Environment` and `Environment:Daylight Saving` EIO rows, including disabled row `Environment:Daylight Saving,No,RunPeriod Object`; the fixed-date precedence case locks the shared EPW declaration being superseded by one input-file 2/28-through-2/29 object despite RunPeriod use No, daily status 1/1/0 with 48 active hours, and exact row `Environment:Daylight Saving,Yes,InputFile,02/28,02/29`; the literal 2032 EPW fourth-Monday-through-last-Wednesday pair additionally locks 120 state/timestamp samples, 72 active hours, raw EnergyPlus oracle ESO timestamp fields and values, and one exact WeatherFile EIO row; the southern-wrap end-side and start-side cases externally lock their respective same-year March 1/1/0 and October 0/1/1 72-sample windows with 48 active hours each, all EnergyPlus ESO timestamp fields and values, and the exact WeatherFile EIO dates 10/31 and 03/28, while Rust day-of-year 305/88 and wraps-year flag remain source-mapped summary diagnostics not emitted by EIO; the exact cross-year DST case locks one 96-row wrapping DATA PERIOD in daily status 0/0/0/1 with 24 active hours, exact 01/02 through 01/03 EIO dates, and source-mapped January 2 prefetch retention; the paired EPW holiday cases lock one fixed holiday enabled as source-exact Sunday index 1 versus disabled as the underlying Monday index 2; the exact EPW weekday-rule case locks `4th Monday in February` then `Last Wednesday in February` resolving to days of year 54 and 56, both as source-exact Sunday/index 1; the mixed-source case locks that enabled EPW Holiday resolving before and then yielding to one later IDF CustomDay1 on the same date; the offset case locks 24 decoy rows skipped and 48 dry-bulb rows in exact timestamp/value order; the cross-year start-year case locks one wrapping 96-row DATA PERIOD in ordered 3/4/5/8 day-type order. Unit tests separately lock Today/Tomorrow source-index transitions, interpolation seeds, the day-local hour-24 solar `NextHr`, and the one-timestep-per-hour current-only solar branch | other EPW DST years/months/rules, January 3 and later reset/reprojection, generic southern/wrap behavior, actual weather or cross-year execution beyond the exact fixture, other RunPeriod policy/source combinations and IDF DST behavior beyond the exact fixed-date precedence case, EPW holiday weekend/weekday-rule/header-order/multiple-policy/overlap/precedence behavior beyond the exact promoted cases, actual-weather year matching, cross-year traversal beyond the exact single-boundary fixtures, multiple-data-period execution, records-per-hour greater than one, complete Today/Tomorrow value-state parity, missing/range repair, cyclic multi-year execution, weather consumers outside the stated `ep_run` setup, subhourly solar interpolation, and complete `ReadEPlusWeatherForDay`/`SetCurrentWeather`/solar/`WeatherManager` parity |
| schedules | `Schedule:Constant` and compiled `Schedule:Compact` periods/day-type profiles/`Until` endpoints produce hourly series. The paired calendar cases lock the same all-days 1-through-24 profile for 72 versus 48 weather-effective hours; the Through/For case locks 120 calendar-aware values as 103/104/105/108/199; the exact DST rollover case locks 72 values as 100x23+124, 200x23+801, and 800x23+901, including current DST +1-hour lookup, hour-24 next-ordinal/tomorrow-Holiday selection, and the final stale Holiday tomorrow type; the CP43 exception locks exactly one aligned, no-DST, omitted-directive default-No `Timestep,4` compact series as 96 values and normalized timestamp labels, 11/12/13/14 then 90x92; and the CP44 exception locks exactly three explicit No/Average/Linear `Timestep,4` series with 96 samples each, first-five vectors 10/175/175/175/175, 10/120/175/175/175, and 10/40/85/130/175, then 175 | schedule-specific DST opt-out, zone-timestep/subhourly lookup beyond the exact CP43 and CP44 exceptions, explicit `Interpolate` directives, Average/Linear, or non-aligned sampling behavior beyond the exact CP44 fixture, public/input object families beyond `Schedule:Constant`/`Schedule:Compact`, additional calendar and rollover boundaries, downstream internal-gain/HVAC/IdealLoads calendar consumption, EMS current-value semantics, `UpdateScheduleVals`, and broad `getHrTsVal` parity |
| output time | hourly consumers use an output-owned normalized comparison label projected from the shared axis; the paired leap-policy schedule cases, record-selection weather case, paired fixed-date DST policy cases, the fixed-date DST EPW/IDF precedence case, literal 2032 EPW weekday-boundary DST case, exact southern-wrap DST end-side and start-side cases, exact cross-year start-year DST case, fixed IDF special-day case, exact cross-year start-year projection case, paired common-/leap-year duration-wrap cases, paired fixed-date SpecialDays declaration-order cases, the fixed-date EPW/IDF precedence case, exact 2032 fourth-/last-Sunday IDF cases, fixed-Sunday Yes/No/blank and fixed-Saturday Yes/No weekend-policy cases, paired fixed EPW holiday policy cases, and the exact EPW weekday-rule case enforce only their declared ordered, unique, exact normalized hourly labels and variables. CP43 separately locks exactly 96 normalized Timestep labels at aligned 15-minute endpoints for its one-day `Timestep,4` schedule case. CP44 separately locks the same 96 normalized Timestep labels for three explicit-mode series in its one-day `Timestep,4` case | Rust raw ESO serialization; EnergyPlus timestep records beyond the exact CP43 and CP44 TimeStep oracles, exact day/month/run-period EnergyPlus ESO records, and MTR/SQL records from `WriteTimeStampFormatData`; broader DST/day-type serialization and tomorrow-state formatting remain unclaimed |

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

Each legacy fixture claims only its manifest's declared input, calendar,
series, normalized timestamp contract, and comparison policy. Raw text,
internal ordinals/buffers, warning semantics, downstream consumers, or broad
EnergyPlus routine completion require separate evidence. Unit-derived state
contracts remain distinct from independent Native observations. Negative cases
retain engine-specific failures and do not become numeric PASS results.

Use the [case index](../generated/conformance-case-index.md),
[algorithm ledger](../../../specs/algorithm_ledger.toml), and
[finite porting plan](../../../energyplus_porting_plan/README.md) for the
applicable status and evidence. A passing fixture or representation change
cannot widen another claim.

## CLK-02: bounded raw EPW parsing and actual production handoff

The CLK-02 evidence packet at
[`energyplus_porting_plan/evidence/CLK-02.json`](../../../energyplus_porting_plan/evidence/CLK-02.json)
binds the EnergyPlus 26.1.0 source contract, actual original helper output,
canonical Rust probe, production CLI runs, and their recorded comparisons.
Its scope is raw record interpretation and the selected header state; it does
not promote this document's other weather or calendar boundaries.

`ep_runtime::weather::raw` owns the raw date/time integers, twenty mandatory
and six optional floating-point outputs, weather observation/code fields,
the persistent missing-code counter, and selected EPW header state. Separate
EPWDST and DST objects, allocation flags, constructor/prepared state, partial
writes on actual source-fatal outcomes, ordered continuations, and available
stream positions are retained. Finite scalar bits, signed zero, typed state,
array cardinality, and input identity are compared exactly. Native error text,
rdstate integers, private locals, and unused Typical/Extreme/Ground computed
state remain outside the paired fields.

Production runtime preparation calls `load_parsed_epw_weather_file` once. The returned
`ParsedEpwWeatherFile` retains the raw owner outputs and a distinct
`physical_projection` for the existing environment selector and weather
precomputation. Ten shared weather values copy their literal raw public slots.
The existing liquid-precipitation adapter maps its missing sentinel and
negative values to zero; that adapter value is checked at the actual consumer
handoff without certifying EnergyPlus' later precipitation policy. Raw record
year remains distinct from the civil simulation year.

Full observations connect the actual selected source record indices to these
same raw outputs, selected hourly records, and the already-constructed weather
consumer context. The observer copies received values; it neither reparses
records nor recomputes weather. Summary uses the same canonical loading path
with collection disabled and is checked for unchanged ordinary results. Trace
omissions and the collecting-thread boundary are explicit. Observed selection
indices are Rust provenance, rather than native record-selection parity.

The existing public `load_epw_weather_file` and parsing APIs remain supported
for their previous callers and the preserved legacy baseline. Their existence
does not certify broader admission or later physical calculations. CLK-03
through CLK-06 retain Today/Tomorrow lifecycle, native record selection,
sentinel replacement, rain/snow policy, interpolation, sky/IR calculation,
calendar/warmup correspondence, and later weather physics. Parsing all 8,760
fixed-file rows does not certify annual thermal or HVAC performance.
