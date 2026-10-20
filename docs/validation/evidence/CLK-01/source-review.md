# CLK-01 source, unit and consumed clock evidence

The selected source is EnergyPlus 26.1.0 commit
`6f2e40d10250a105b49966baa24d843711e61048`. The three CLK-01 contracts pin full
file bytes, selected source ranges, input identities and tolerances before any
Rust/reference comparison. The final selected helper/calendar unit comparison
passed 71,358 checks with zero mismatches. The separate final production report
and independent review bind all 11 actual cases and close the selected consumed
clock scope; unit rows alone do not establish physical invocation.

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
option/math/warning targets. The actual selected test-reference configuration is
RelWithDebInfo with explicit -O0 -g -DNDEBUG and the original target's later
-UNDEBUG, preserving assertions and -Werror. It also preserves the original
source FP options, disables contraction with -ffp-contract=off, and has neither
fastmath nor _GLIBCXX_DEBUG. The driver inherits those actual flags rather than
assuming Debug. This does not claim that the default optimized Windows GNU build
passes: the original optimized Boost/Kiva warning failure, Debug fegetexcept
platform failure, PE section-limit failure and long-path failure remain in the
native build provenance. No original scientific source or extra warning waiver
was introduced.
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
The final unit comparison now covers all 15 calendars and all 1,845 daily rows,
180 monthly-array entries and every annual month start exactly. Its 177,120
prepared zone points are still preparation evidence.

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
stateful reference runs and selected calendar comparisons have now completed.
Production comparison subsequently completed for all 11 selected actual cases.

The pure fallback compiled and dispatched the original pure bodies successfully.
The initial preparation comparison then matched 1,912 existing Rust helper calls
exactly: 384 isLeapYear, 764 calculateDayOfYear and 764 General::OrdinalDay.
The checker made 38,496 identity/type/order/value checks with zero mismatches;
maximum absolute error and RMSE are zero for each eligible routine. These are
pre-checkpoint diagnostic executions, not committed-source closure evidence.
The remaining 1,168 Rust helper rows are explicitly unpaired; the pure fallback
executed 776 source-only diagnostics and declared 392 genuine-state calls
unavailable. That historical preparation result remains unchanged. The later
matching native core supplied the missing mutable-state diagnostics. No numerical
tolerance or CON case changed.

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

## Original-first native and committed unit execution

The compact final unit receipt is [source-reference.json](source-reference.json).

The subsequent normal CLI matrix completed all 15 prepared cases and 11 actual
cases: A24H/A72H/Aannual plus each B limit variant24H/72H. The actual runs observed
36,960 ordered physical zone invocations with no missing clock event or calendar
mismatch. All 22 Full/Summary CLI executions completed without oracle/fixture
answers; selected-output CSV, meter CSV and complete result series match between
Full and Summary. The final [comparison-report.json](comparison-report.json)
binds the source/unit/native/build/input/command/artifact hash chains and retains
the distinct preparation and physics scopes. B annual physics, original weather
producer/global history, warmup/handoff/adaptive system order and full reporting
remain subsequent card obligations.
The original core receipt is
`.runtime/porting/reference-energyplus-26.1.0/native-core-build.json`; the matching
driver's actual configure, compile/link, flag inheritance, archived source/binary
and constructor/state-write smoke are bound by
`.runtime/porting/CLK-01/native-driver/native-driver-build.json`. Reconfiguration
added only the agreed CMAKE_PROJECT_INCLUDE. Every existing core compile command
and both core/API library hashes remained unchanged.

An isolated syntax-only preflight first failed because InputProcessor.hh declares
a global EnergyPlusData in addition to the actual EnergyPlus::EnergyPlusData.
Only the wrapper's three native state type names were qualified; that correction
is committed in e8b7d578 and the matching preflight passed. Both receipts/logs are
preserved in `.runtime/porting/CLK-01/driver-preflight` and
`driver-preflight-qualified`. The exact earlier f6f0c5b3 wrapper bytes are archived
under `.runtime/porting/CLK-01/preparation/source`; the historical pure build and
failed preflight are not relabeled as having executed the corrected source.

The genuine native executable ran all 3,080 helper inputs and all 15 original
parsed IDFs first, with exit zero. Only afterward did the immutable committed Rust
example run the same input requests, also with exit zero. That example is from
fe57a357, SHA 2e8b5130d4c0e1bc8663a80e76f74ec86e02d6c78c08d95539577e0ee9d5b7d1;
e8b7d578 leaves its crates tree and binary unchanged. Actual commands are:

    python tools/porting/clk01_reference.py --run-helpers --reference-exe .runtime/porting/CLK-01/native-driver/clk01_reference.exe --native-core-build .runtime/porting/reference-energyplus-26.1.0/native-core-build.json --native-driver-build .runtime/porting/CLK-01/native-driver/native-driver-build.json --runtime-bin .runtime/reference-tools/winlibs-gcc13.2.0-ucrt-r3/mingw64/bin --output-dir .runtime/porting/CLK-01/committed-unit/original-helpers
    python tools/porting/clk01_reference.py --run-calendar --reference-exe .runtime/porting/CLK-01/native-driver/clk01_reference.exe --native-core-build .runtime/porting/reference-energyplus-26.1.0/native-core-build.json --native-driver-build .runtime/porting/CLK-01/native-driver/native-driver-build.json --runtime-bin .runtime/reference-tools/winlibs-gcc13.2.0-ucrt-r3/mingw64/bin --output-dir .runtime/porting/CLK-01/committed-unit/original-calendar
    .runtime/porting/CLK-01/committed-build/fe57a357/clk01_calendar.exe .runtime/porting/CLK-01/committed-unit/original-helpers/helper-tuples.json
    .runtime/porting/CLK-01/committed-build/fe57a357/clk01_calendar.exe .runtime/porting/CLK-01/committed-unit/original-calendar/calendar-cases.json
    python tools/porting/check_clk01_units.py --cpp-helpers .runtime/porting/CLK-01/committed-unit/original-helpers/helpers-results.json --rust-helpers .runtime/porting/CLK-01/committed-unit/rust-helpers/helpers-results.json --cpp-calendar .runtime/porting/CLK-01/committed-unit/original-calendar/calendar-results.json --rust-calendar .runtime/porting/CLK-01/committed-unit/rust-calendar/calendar-results.json --cpp-build-receipt .runtime/porting/CLK-01/native-driver/native-driver-build.json --rust-build-receipt energyplus_porting_plan/evidence/CLK-01/build-receipt.json --helper-request .runtime/porting/CLK-01/committed-unit/original-helpers/helper-tuples.json --calendar-request .runtime/porting/CLK-01/committed-unit/original-calendar/calendar-cases.json --output-dir .runtime/porting/CLK-01/committed-unit/comparison

The exact checker exited zero: 71,358 checks, zero mismatches, and zero maximum
absolute error/RMSE for all 1,912 eligible scalar helpers. All 15 resolved calendars
and 1,845 daily rows match. The 180 original monthly-array entries match the actual
annual consumed month-start projection, and every annual calendar includes all
12 month starts. The original-only 1,168 helper rows now also contain actual
genuine-state writes for 384 calculateDayOfWeek calls and before/after arrays for
eight SetupWeekDaysByMonth calls. These direct mutable-state diagnostics remain
unpaired; they are not counted as Rust helper passes or a full raw 366-array clone.

Raw commands, input identities, stdout/stderr and execution receipts are under
`.runtime/porting/CLK-01/committed-unit/{original-helpers,original-calendar,rust-helpers,rust-calendar}`.
The report is `committed-unit/comparison/unit-comparison.json` with SHA
9ca1197b98c06ad572fc571c2d0118fc3f2113b251da62ffef38f6f8213f55fc.
The unit runner executes no zone physics: prepared zone rows, including B annual,
do not imply actual invocation or physical equivalence. The subsequent separate
production/native pre-report/ESO evidence is bound by comparison-report.json;
unit completion alone does not update a gate.
