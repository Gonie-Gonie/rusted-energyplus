# CLK-01 source preparation — pending validation

The selected source is EnergyPlus 26.1.0 commit
`6f2e40d10250a105b49966baa24d843711e61048`. The three CLK-01 contracts pin full
file bytes, selected source ranges, input identities and tolerances before any
Rust/reference comparison. No CLK-01 gate is closed by this preparation.

The production input set remains the 15 CON cases: ordinary weather RunPeriod,
explicit 2013, one zone, one selected weather environment, no sizing execution,
and no actual DST or special-day override. Weather holidays and DST policy remain
enabled in the input; the pinned EPW header is `No,0,0,0`. Actual override absence
must be checked after weather resolution. A helper leap-year probe does not admit
a leap-year production RunPeriod.

The named functions need the active date helpers and selected environment setup
as their direct source closure. GetRunPeriodData computes the Gregorian weekday
through calculateDayOfWeek, which writes `dataEnvrn.DayOfWeek`; it writes the
RunPeriod records and month weekday array. SetupWeekDaysByMonth reads the actual
WeatherData EndDayOfMonth and LeapYearAdd, together with the initial 12-element
array. It has forward, backward and retained-entry behavior. Its backward
February branch adds LeapYearAdd after subtracting February's length. Preserve
that original branch in source-only state diagnostics; no corrected substitute
is used. CalendarFrame production remains restricted to the admitted 2013 path.

SetupEnvironmentTypes combines Gregorian leap status with the EPW leap policy
for the ordinary same-year environment. The native unit driver calls this
original body on genuine EnergyPlusData. It declares the function's prerequisite
state explicitly: definition counts obtained from the original InputProcessor,
the selected EPW leap policy, and the allocated environment array. Inactive
design-day definitions are counted but their simulation payloads are not
fabricated. This is a selected parsed-state unit invocation, not execution of the
complete GetWeatherInput/GetNextEnvironment factory or sizing simulation.

The C++ reference is linked to the unchanged original EnergyPlus static core and
uses the same GNU compiler and actual selected configuration's ABI and project
option/math/warning targets. Source-supported RelWithDebInfo has no
_GLIBCXX_DEBUG; the driver inherits those actual flags rather than assuming Debug.
Its original InputProcessor::processInput path parses and validates the pinned
IDFs before GetRunPeriodData. It does not insert common unit-test objects or
expected answers. A separate fallback compiles byte-exact original pure bodies
with original GregorianDate/type declarations and ObjexxFCL mod. That fallback
returns explicit unsupported stateful calls; it cannot prove GetRunPeriodData,
calculateDayOfWeek mutation or SetupWeekDaysByMonth state. The installed official
DLL is used only through its opaque public API by the separate native trace
harness; its private MSVC state is never passed to the GNU driver.

The Rust direct helper facade may forward only existing is_leap_year/day_of_year
implementations. The three eligible pairings are isLeapYear, calculateDayOfYear
and General::OrdinalDay on valid scalar domains. Julian/Gregorian inverse,
validMonthDay, calculateDayOfWeek global mutation and mutable monthly-array
diagnostics have no equivalent existing Rust stateful helper API and remain
explicitly unpaired. validMonthDay only checks upper day bounds in the original
body; its zero/negative-day diagnostics do not weaken production input admission.

The date projection calls original native date, weekday and ordinal helpers for
each declared 2013 input day. Gregorian/weather ordinal and the always-leap
schedule ordinal remain separate. All 15 prepared calendars, including every
annual day, must be compared; those rows are preparation evidence, not actual
zone invocation evidence. Actual consumer coverage requires representative A/B
24H and 72H invocations, with A annual clock coverage when feasible. B annual
whole-physics execution remains a later SYS obligation because its current
validation cost is large. No prepared calendar is reported as a physical run.

The observation phase is
`callback_begin_zone_timestep_before_init_heat_balance`. SimulationManager calls
Weather::ManageWeather before ManageHeatBalance; HeatBalanceManager invokes this
callback before InitHeatBalance and before reporting. Native public exports
return weather-record year and civil CalendarYear separately. The current
consumed civil date, weekday, three ordinals, day type, hour and zone interval are
the comparison contract. Immutable raw pre- and after-report API clock/year
observations remain separate: even the pre-report callback can carry a shortened
SYS-adjusted API currentTime/minutes, and
OutputProcessor increments CalendarYear after December's final reporting block.
Neither that reporting mutation nor complete OutputProcessor serialization is a
CLK-01 completion claim. Native ESO date columns provide an independent date
consistency observation. Normal CLI selected-output CSV currently consumes sample
indices; no artificial timestamp-normalizer call is added for coverage.

Today/Tomorrow weather buffers, last-day retention, prefetch, reset flags and
their warmup/repeat timing are recorded as source context only and belong to
CLK-03. Schedule scalar lookup/value updates belong to SCH-03. Active holiday
and DST branches, design/sizing, multi-year and actual-weather production inputs,
adaptive SYS sequencing and whole-building/HVAC numerical equivalence remain
outside this card's selected proof.

Preparation commands:

```powershell
python tools/porting/clk01_reference.py --prepare
python tools/porting/clk01_reference.py --check
```

The matching original-core build can add the driver without editing original
sources using `-DCMAKE_PROJECT_INCLUDE=<absolute-path>/clk01_reference.cmake` and
then build target `clk01_reference`. This must be coordinated with the native
build owner; linking only energypluslib would omit its private ABI/math options.
Raw helper/calendar requests, original-body extraction and preparation hashes
are written to `.runtime/porting/CLK-01/preparation`. Native compilation,
stateful reference runs and calendar/production comparisons are pending.

The pure fallback compiled and dispatched the original pure bodies successfully.
The initial preparation comparison then matched 1,912 existing Rust helper calls
exactly: 384 isLeapYear, 764 calculateDayOfYear and 764 General::OrdinalDay.
The checker made 38,496 identity/type/order/value checks with zero mismatches;
maximum absolute error and RMSE are zero for each eligible routine. These are
pre-checkpoint diagnostic executions, not committed-source closure evidence.
The remaining 1,168 Rust helper rows are explicitly unpaired; the pure fallback
executed 776 source-only diagnostics and declared 392 genuine-state calls
unavailable. Source mutable-array/state proof still requires the matching native
core. No numerical tolerance or CON case changed.

The first pure executable launch failed with Windows loader exit 3221225781
because the compiler runtime directory was absent from PATH. Its original log
files were overwritten by the successful retry before the launcher gained an
existing-output guard. The observed exit remains recorded, but that initial
failure has no preserved raw logs and provides no numerical evidence. Subsequent
executions require a fresh output directory, and explicit runtime path bindings
are recorded.

Persistent comparison usage:

    python tools/porting/check_clk01_units.py --cpp-helpers <original-helper-results.json> --rust-helpers <actual-Rust-helper-results.json> --output-dir <fresh-unit-comparison-dir>
    python tools/porting/check_clk01_units.py --cpp-calendar <original-native-calendar-results.json> --rust-calendar <actual-Rust-calendar-results.json> --output-dir <fresh-calendar-comparison-dir>

The calendar comparison requires all 15 original parsed cases and all annual
month starts. It compares each original GetRunPeriodData monthly array to the
actual consumed annual month-start projection, in addition to every declared
day's date/weekday/ordinals. It never clones the original global arrays in Rust.
