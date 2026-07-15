[CmdletBinding()]
param([switch]$SelfTest)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")

$RepoRoot = Get-RepoRoot
Set-Location $RepoRoot

function Read-RepoText {
    param([Parameter(Mandatory = $true)][string]$Path)
    return Get-Content -Encoding UTF8 -Raw -LiteralPath $Path
}

function Assert-FileExists {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Description
    )

    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        throw "$Description missing: $Path"
    }
}

function Assert-Contains {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Pattern,
        [Parameter(Mandatory = $true)][string]$Description
    )

    $text = Read-RepoText -Path $Path
    if ($text -notmatch $Pattern) {
        throw "$Description missing in $Path"
    }
}

function Assert-NotContains {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Pattern,
        [Parameter(Mandatory = $true)][string]$Description
    )

    $text = Read-RepoText -Path $Path
    if ($text -match $Pattern) {
        throw "$Description unexpectedly present in $Path"
    }
}

function Assert-LineLimit {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][int]$Limit,
        [Parameter(Mandatory = $true)][string]$Description
    )

    $lineCount = (Get-Content -Encoding UTF8 -LiteralPath $Path | Measure-Object -Line).Lines
    if ($lineCount -gt $Limit) {
        throw "$Description exceeds $Limit LOC: $Path has $lineCount LOC"
    }
}

function Remove-RustTestModuleText {
    param([Parameter(Mandatory = $true)][string]$Text)

    $testModule = [regex]::Match(
        $Text,
        '(?m)^\s*#\s*\[\s*cfg\s*\(\s*test\s*\)\s*\]\s*\r?\n\s*mod\s+tests\b'
    )
    if ($testModule.Success) {
        return $Text.Substring(0, $testModule.Index)
    }
    return $Text
}

function Read-RustProductionText {
    param([Parameter(Mandatory = $true)][string]$Path)

    return Remove-RustTestModuleText -Text (Read-RepoText -Path $Path)
}

function Assert-RustProductionTreeNotContains {
    param(
        [Parameter(Mandatory = $true)][string[]]$Roots,
        [Parameter(Mandatory = $true)][string[]]$AllowedPrefixes,
        [Parameter(Mandatory = $true)][string]$Pattern,
        [Parameter(Mandatory = $true)][string]$Description
    )

    $violations = [System.Collections.Generic.List[string]]::new()
    foreach ($root in $Roots) {
        foreach ($file in Get-ChildItem -LiteralPath $root -Filter "*.rs" -File -Recurse) {
            $relativePath = $file.FullName.Substring($RepoRoot.Length).TrimStart('\', '/') -replace '\\', '/'
            if ($relativePath -match '(^|/)tests(/|\.rs$)') {
                continue
            }
            if (@($AllowedPrefixes | Where-Object { $relativePath.StartsWith($_, [System.StringComparison]::Ordinal) }).Count -gt 0) {
                continue
            }

            $text = Read-RustProductionText -Path $file.FullName
            $match = [regex]::Match($text, $Pattern)
            if ($match.Success) {
                $line = ($text.Substring(0, $match.Index) -split "`n").Count
                $violations.Add("${relativePath}:${line}")
            }
        }
    }

    if ($violations.Count -gt 0) {
        throw "$Description unexpectedly present outside its diagnostic/test boundary: $($violations -join ', ')"
    }
}

function Assert-RustProductionMatchCount {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Pattern,
        [Parameter(Mandatory = $true)][int]$ExpectedCount,
        [Parameter(Mandatory = $true)][string]$Description
    )

    $text = Read-RustProductionText -Path $Path
    $actualCount = [regex]::Matches($text, $Pattern).Count
    if ($actualCount -ne $ExpectedCount) {
        throw "$Description expected $ExpectedCount production matches in $Path, got $actualCount"
    }
}

function Invoke-DiagnosticProbeBoundarySelfTest {
    $diagnosticTypePattern = '\bDiagnosticHeatBalanceProbe\b'
    $longVariantPattern = '\bEnergyPlus[A-Za-z0-9_]*Probe\b'
    $injectedProduction = @'
fn injected_compatibility_consumer() {
    let _probe: Option<DiagnosticHeatBalanceProbe> = None;
    let _variant = EnergyPlusInjectedLongProbe;
}
'@
    $production = Remove-RustTestModuleText -Text $injectedProduction
    if ($production -notmatch $diagnosticTypePattern -or $production -notmatch $longVariantPattern) {
        throw "diagnostic probe boundary self-test failed to detect an injected production violation"
    }

    $injectedTestOnly = @'
#[cfg(test)]
mod tests {
    fn diagnostic_variant_is_allowed_in_tests() {
        let _variant = EnergyPlusInjectedLongProbe;
    }
}
'@
    $testOnlyProduction = Remove-RustTestModuleText -Text $injectedTestOnly
    if ($testOnlyProduction -match $longVariantPattern) {
        throw "diagnostic probe boundary self-test failed to exclude a test-only variant"
    }

    Write-Host "Heat-balance diagnostic probe boundary self-test complete."
}

$heatBalanceMod = "crates\ep_runtime\src\heat_balance\mod.rs"
$algorithm = "crates\ep_runtime\src\heat_balance\algorithm.rs"
$manager = "crates\ep_runtime\src\heat_balance\manager.rs"
$surfaceManager = "crates\ep_runtime\src\heat_balance\surface_manager.rs"
$surfaceBalance = "crates\ep_runtime\src\heat_balance\surface_balance.rs"
$surfaceBoundary = "crates\ep_runtime\src\heat_balance\surface_boundary.rs"
$surfaceLoop = "crates\ep_runtime\src\heat_balance\surface_loop.rs"
$warmup = "crates\ep_runtime\src\heat_balance\warmup.rs"
$airManager = "crates\ep_runtime\src\heat_balance\air_manager.rs"
$zonePredictorCorrector = "crates\ep_runtime\src\heat_balance\zone_predictor_corrector.rs"
$zoneAirCorrection = "crates\ep_runtime\src\heat_balance\zone_air_correction.rs"
$ctf = "crates\ep_runtime\src\heat_balance\ctf.rs"
$insideConvection = "crates\ep_runtime\src\heat_balance\inside_convection.rs"
$initialization = "crates\ep_runtime\src\heat_balance\initialization.rs"
$initializationScheduleCache = "crates\ep_runtime\src\heat_balance\initialization\schedule_cache.rs"
$convection = "crates\ep_runtime\src\heat_balance\convection.rs"
$longwave = "crates\ep_runtime\src\heat_balance\longwave.rs"
$radiation = "crates\ep_runtime\src\heat_balance\radiation.rs"
$solar = "crates\ep_runtime\src\heat_balance\solar.rs"
$reports = "crates\ep_runtime\src\heat_balance\reports.rs"
$state = "crates\ep_runtime\src\heat_balance\state.rs"
$runPeriod = "crates\ep_runtime\src\heat_balance\run_period.rs"
$trace = "crates\ep_runtime\src\heat_balance\trace.rs"
$summary = "crates\ep_runtime\src\heat_balance\summary.rs"
$surfaceWeather = "crates\ep_runtime\src\heat_balance\surface_weather.rs"
$timestep = "crates\ep_runtime\src\heat_balance\timestep.rs"
$typedIds = "crates\ep_model\src\ids.rs"
$calendarObjects = "crates\ep_model\src\objects\calendar.rs"
$scheduleObjects = "crates\ep_model\src\objects\schedules.rs"
$rawModel = "crates\ep_raw_model\src\lib.rs"
$idfOrder = "crates\ep_raw_model\src\idf_order.rs"
$compiler = "crates\ep_compiler\src\compiler.rs"
$diagnosticProbe = "crates\ep_runtime\src\diagnostic_probes\heat_balance.rs"
$executionPlan = "crates\ep_runtime\src\execution_plan.rs"
$precompute = "crates\ep_runtime\src\precompute.rs"
$pipeline = "crates\ep_run\src\pipeline.rs"
$runConfig = "crates\ep_run\src\config.rs"
$runSupport = "crates\ep_run\src\support.rs"
$cli = "crates\ep_cli\src\main.rs"
$runtime = "crates\ep_runtime\src\runtime.rs"
$runtimeError = "crates\ep_runtime\src\error.rs"
$weather = "crates\ep_runtime\src\weather.rs"
$weatherCalendar = "crates\ep_runtime\src\weather_calendar.rs"
$weatherTests = "crates\ep_runtime\src\weather_tests.rs"
$timeAxis = "crates\ep_runtime\src\time_axis.rs"
$timeAxisWeatherCalendar = "crates\ep_runtime\src\time_axis\weather_calendar.rs"
$weatherEnvironment = "crates\ep_runtime\src\weather_environment.rs"
$calendarRules = "crates\ep_runtime\src\time_axis\calendar_rules.rs"
$dayType = "crates\ep_runtime\src\time_axis\day_type.rs"
$daylightSaving = "crates\ep_runtime\src\time_axis\daylight_saving.rs"
$specialDays = "crates\ep_runtime\src\time_axis\special_days.rs"
$scheduleModel = "crates\ep_model\src\objects\schedules.rs"
$schedules = "crates\ep_runtime\src\schedules.rs"
$scheduleCache = "crates\ep_runtime\src\schedules\cache.rs"
$internalGainScheduleCache = "crates\ep_runtime\src\schedules\internal_gain_cache.rs"
$internalGainScheduleProfile = "crates\ep_runtime\src\schedules\internal_gain_profile.rs"
$scheduleConstant = "crates\ep_runtime\src\schedules\constant.rs"
$timeWeatherSchedule = "crates\ep_cli\src\time_weather_schedule.rs"
$timeWeatherScheduleSpecialDays = "crates\ep_cli\src\time_weather_schedule_special_days.rs"
$algorithmLedger = "specs\algorithm_ledger.toml"
$durationWrapGate = "scripts\compare\compare-calendar-special-day-duration-wrap-exact.ps1"
$weekendHolidayGate = "scripts\compare\compare-calendar-weekend-holiday-policy-exact.ps1"
$overlapOrderGate = "scripts\compare\compare-calendar-special-day-overlap-order-exact.ps1"
$epwIdfPrecedenceGate = "scripts\compare\compare-calendar-special-day-epw-idf-precedence-exact.ps1"
$epwWeekdayHolidayGate = "scripts\compare\compare-calendar-epw-holiday-weekday-rules-exact.ps1"
$fixedDstPolicyGate = "scripts\compare\compare-calendar-dst-fixed-date-policy-exact.ps1"
$dstEpwIdfPrecedenceGate = "scripts\compare\compare-calendar-dst-epw-idf-precedence-exact.ps1"
$epwWeekdayDstGate = "scripts\compare\compare-calendar-epw-dst-weekday-rules-exact.ps1"
$epwSouthernDstGate = "scripts\compare\compare-calendar-epw-dst-southern-wrap-exact.ps1"
$epwSouthernDstStartGate = "scripts\compare\compare-calendar-epw-dst-southern-wrap-start-exact.ps1"
$crossYearDstGate = "scripts\compare\compare-calendar-epw-dst-cross-year-start-year-projection-exact.ps1"
$crossYearSpecialDayGate = "scripts\compare\compare-calendar-special-day-cross-year-start-year-projection-exact.ps1"
$compactThroughForGate = "scripts\compare\compare-calendar-schedule-compact-through-for-day-type-exact.ps1"
$scheduleDstRolloverGate = "scripts\compare\compare-calendar-schedule-dst-hour24-tomorrow-day-type-exact.ps1"
$compactZoneTimestepGate = "scripts\compare\compare-calendar-schedule-compact-zone-timestep-exact.ps1"
$compactInterpolationGate = "scripts\compare\compare-calendar-schedule-compact-interpolation-modes-exact.ps1"
$probeSummaryReport = "tools\reporting\dynamic_heat_balance_probe_summary.py"
$dynamicDiagnosticScript = "scripts\compare\official-dynamic-heat-balance-diagnostic.ps1"
$dynamicCompatScript = "scripts\compare\official-dynamic-heat-balance-compat-candidate.ps1"
$runtimeTestSourceOrder = "crates\ep_runtime\src\runtime\tests\part01.rs"
$runtimeTestDynamic = "crates\ep_runtime\src\runtime\tests\part03.rs"
$runtimeTestResults = "crates\ep_runtime\src\runtime\tests\part05.rs"
$runtimeTestRadiation = "crates\ep_runtime\src\runtime\tests\part04.rs"
$runtimeTestCalendar = "crates\ep_runtime\src\runtime\tests\part10.rs"
$runtimeTestSpecialDays = "crates\ep_runtime\src\runtime\tests\part11.rs"
$runtimeTestEpwHolidays = "crates\ep_runtime\src\runtime\tests\part12.rs"
$runtimeTestScheduleCache = "crates\ep_runtime\src\runtime\tests\part13.rs"

foreach ($entry in @(
        @($heatBalanceMod, "heat-balance module facade"),
        @($algorithm, "heat-balance algorithm selector module"),
        @($manager, "HeatBalanceManager source-order module"),
        @($surfaceManager, "HeatBalanceSurfaceManager source-order module"),
        @($surfaceBalance, "surface balance ownership module"),
        @($surfaceBoundary, "surface boundary ownership module"),
        @($surfaceLoop, "surface loop ownership module"),
        @($warmup, "warmup ownership module"),
        @($airManager, "HeatBalanceAirManager source-order module"),
        @($zonePredictorCorrector, "ZoneTempPredictorCorrector source-order module"),
        @($zoneAirCorrection, "zone-air correction ownership module"),
        @($ctf, "CTF ownership module"),
        @($insideConvection, "inside convection ownership module"),
        @($initialization, "heat-balance initialization ownership module"),
        @($initializationScheduleCache, "heat-balance initialization schedule-cache adapter"),
        @($convection, "convection ownership module"),
        @($longwave, "exterior longwave ownership module"),
        @($radiation, "radiation ownership module"),
        @($solar, "solar radiation ownership module"),
        @($reports, "report ownership module"),
        @($state, "heat-balance state ownership module"),
        @($runPeriod, "run-period sampling ownership module"),
        @($trace, "heat-balance trace ownership module"),
        @($summary, "heat-balance summary ownership module"),
        @($surfaceWeather, "surface weather ownership module"),
        @($timestep, "heat-balance timestep ownership module"),
        @($typedIds, "typed compact ID module"),
        @($calendarObjects, "typed calendar control module"),
        @($scheduleObjects, "typed compact schedule module"),
        @($rawModel, "raw-model declaration-order API"),
        @($idfOrder, "targeted IDF declaration-order recovery"),
        @($compiler, "typed model compiler"),
        @($diagnosticProbe, "diagnostic probe selector module"),
        @($executionPlan, "execution plan module"),
        @($precompute, "runtime precompute module"),
        @($pipeline, "arbitrary-run pipeline"),
        @($runConfig, "arbitrary-run configuration"),
        @($runSupport, "arbitrary-run support assessment"),
        @($cli, "CLI conformance gate"),
        @($runtime, "runtime orchestration root"),
        @($runtimeError, "runtime fail-closed error contract"),
        @($weather, "runtime weather module"),
        @($weatherCalendar, "EPW calendar metadata parser"),
        @($weatherTests, "EPW weather parser tests"),
        @($timeAxis, "runtime time-axis module"),
        @($timeAxisWeatherCalendar, "runtime weather-calendar axis module"),
        @($weatherEnvironment, "runtime weather-environment selector"),
        @($calendarRules, "shared calendar-rule resolver"),
        @($dayType, "EnergyPlus day-type module"),
        @($daylightSaving, "runtime daylight-saving resolver"),
        @($specialDays, "runtime special-day resolver"),
        @($schedules, "runtime schedules module"),
        @($internalGainScheduleCache, "referenced-only internal-gain schedule cache module"),
        @($internalGainScheduleProfile, "internal-gain deterministic operation profile module"),
        @($timeWeatherSchedule, "time/weather/schedule report module"),
        @($timeWeatherScheduleSpecialDays, "time/weather/schedule special-day report module"),
        @($algorithmLedger, "algorithm source-order ledger"),
        @($durationWrapGate, "common-year and leap-year special-day duration-wrap gate"),
        @($weekendHolidayGate, "fixed-Sunday Yes/No/blank weekend holiday gate"),
        @($overlapOrderGate, "paired special-day overlap declaration-order gate"),
        @($crossYearSpecialDayGate, "cross-year start-year special-day gate"),
        @($compactThroughForGate, "Schedule:Compact Through/For day-type gate"),
        @($scheduleDstRolloverGate, "Schedule:Compact DST hour-24 tomorrow-day-type gate"),
        @($compactZoneTimestepGate, "Schedule:Compact default-No zone-timestep gate"),
        @($crossYearDstGate, "cross-year start-year daylight-saving gate"),
        @($probeSummaryReport, "dynamic heat-balance probe summary reporter"),
        @($dynamicDiagnosticScript, "dynamic heat-balance diagnostic comparison script"),
        @($dynamicCompatScript, "dynamic heat-balance compatibility comparison script"),
        @($runtimeTestSourceOrder, "runtime source-order tests"),
        @($runtimeTestDynamic, "runtime dynamic heat-balance tests"),
        @($runtimeTestResults, "runtime heat-balance result tests"),
        @($runtimeTestRadiation, "runtime heat-balance radiation tests"),
        @($runtimeTestCalendar, "runtime calendar and DST tests"),
        @($runtimeTestSpecialDays, "runtime special-day tests"),
        @($runtimeTestEpwHolidays, "runtime EPW holiday tests"),
        @($runtimeTestScheduleCache, "runtime schedule-cache consumer tests")
    )) {
    Assert-FileExists -Path $entry[0] -Description $entry[1]
}

Assert-LineLimit -Path $algorithm -Limit 200 -Description "probe-agnostic heat-balance runtime-config module"
Assert-LineLimit -Path $diagnosticProbe -Limit 800 -Description "heat-balance diagnostic selector module"
Assert-LineLimit -Path $manager -Limit 180 -Description "HeatBalanceManager source-order module"
Assert-LineLimit -Path $surfaceManager -Limit 400 -Description "HeatBalanceSurfaceManager source-order orchestration module"
Assert-LineLimit -Path $surfaceBalance -Limit 720 -Description "surface balance ownership module"
Assert-LineLimit -Path $surfaceBoundary -Limit 280 -Description "surface boundary ownership module"
Assert-LineLimit -Path $surfaceLoop -Limit 430 -Description "surface loop ownership module"
Assert-LineLimit -Path $airManager -Limit 260 -Description "HeatBalanceAirManager source-order module"
Assert-LineLimit -Path $zonePredictorCorrector -Limit 270 -Description "ZoneTempPredictorCorrector source-order module"
Assert-LineLimit -Path $zoneAirCorrection -Limit 520 -Description "zone-air correction ownership module"
Assert-LineLimit -Path $ctf -Limit 800 -Description "CTF ownership module"
Assert-LineLimit -Path $insideConvection -Limit 360 -Description "inside convection ownership module"
Assert-LineLimit -Path $initialization -Limit 220 -Description "heat-balance initialization ownership module"
Assert-LineLimit -Path $convection -Limit 420 -Description "convection ownership module"
Assert-LineLimit -Path $longwave -Limit 180 -Description "exterior longwave ownership module"
Assert-LineLimit -Path $radiation -Limit 800 -Description "radiation ownership module"
Assert-LineLimit -Path $solar -Limit 760 -Description "solar radiation ownership module"
Assert-LineLimit -Path $reports -Limit 900 -Description "report ownership module"
Assert-LineLimit -Path $state -Limit 980 -Description "heat-balance state ownership module"
Assert-LineLimit -Path $runPeriod -Limit 920 -Description "run-period sampling ownership module"
Assert-LineLimit -Path $trace -Limit 800 -Description "heat-balance trace ownership module"
Assert-LineLimit -Path $summary -Limit 160 -Description "heat-balance summary ownership module"
Assert-LineLimit -Path $warmup -Limit 220 -Description "warmup ownership module"
Assert-LineLimit -Path $surfaceWeather -Limit 180 -Description "surface weather ownership module"
Assert-LineLimit -Path $timestep -Limit 800 -Description "heat-balance timestep ownership module"

Assert-Contains -Path $heatBalanceMod -Pattern 'pub mod manager;' -Description "HeatBalanceManager module declaration"
Assert-Contains -Path $heatBalanceMod -Pattern 'pub mod surface_manager;' -Description "HeatBalanceSurfaceManager module declaration"
Assert-Contains -Path $heatBalanceMod -Pattern 'pub mod surface_balance;' -Description "surface balance module declaration"
Assert-Contains -Path $heatBalanceMod -Pattern 'pub mod surface_boundary;' -Description "surface boundary module declaration"
Assert-Contains -Path $heatBalanceMod -Pattern 'pub\(crate\) mod surface_loop;' -Description "surface loop module declaration"
Assert-Contains -Path $heatBalanceMod -Pattern 'pub\(crate\) mod warmup;' -Description "warmup module declaration"
Assert-Contains -Path $heatBalanceMod -Pattern 'pub mod air_manager;' -Description "HeatBalanceAirManager module declaration"
Assert-Contains -Path $heatBalanceMod -Pattern 'pub mod zone_predictor_corrector;' -Description "ZoneTempPredictorCorrector module declaration"
Assert-Contains -Path $heatBalanceMod -Pattern 'pub mod zone_air_correction;' -Description "zone-air correction module declaration"
Assert-Contains -Path $heatBalanceMod -Pattern 'pub mod ctf;' -Description "CTF module declaration"
Assert-Contains -Path $heatBalanceMod -Pattern 'pub mod inside_convection;' -Description "inside convection module declaration"
Assert-Contains -Path $heatBalanceMod -Pattern 'pub mod initialization;' -Description "heat-balance initialization module declaration"
Assert-Contains -Path $heatBalanceMod -Pattern 'pub mod convection;' -Description "convection module declaration"
Assert-Contains -Path $heatBalanceMod -Pattern 'pub mod longwave;' -Description "longwave module declaration"
Assert-Contains -Path $heatBalanceMod -Pattern 'pub mod radiation;' -Description "radiation module declaration"
Assert-Contains -Path $heatBalanceMod -Pattern 'pub mod solar;' -Description "solar radiation module declaration"
Assert-Contains -Path $heatBalanceMod -Pattern 'pub use solar::\*;' -Description "solar radiation module facade export"
Assert-Contains -Path $heatBalanceMod -Pattern 'pub mod reports;' -Description "reports module declaration"
Assert-Contains -Path $heatBalanceMod -Pattern 'pub\(crate\) mod run_period;' -Description "run-period sampling module declaration"
Assert-Contains -Path $heatBalanceMod -Pattern 'pub mod surface_weather;' -Description "surface weather module declaration"
Assert-Contains -Path $heatBalanceMod -Pattern 'pub mod timestep;' -Description "heat-balance timestep module declaration"

foreach ($compactId in @(
        "SurfaceId",
        "ZoneId",
        "NodeId",
        "MaterialId",
        "ConstructionId",
        "ScheduleId"
    )) {
    Assert-Contains -Path $typedIds -Pattern "typed_id!\($compactId\);" -Description "compact typed ID $compactId"
}

Assert-Contains -Path $manager -Pattern 'pub fn manage_heat_balance_source_order_stages\s*\(' -Description "HeatBalanceManager source-order list"
foreach ($routine in @(
        "ManageHeatBalance",
        "GetHeatBalanceInput",
        "InitHeatBalance",
        "RecKeepHeatBalance",
        "ReportHeatBalance",
        "CheckWarmupConvergence"
    )) {
    Assert-Contains -Path $manager -Pattern $routine -Description "HeatBalanceManager routine $routine"
}
foreach ($target in @(
        "manage_heat_balance_source_order_path",
        "get_heat_balance_input_stage",
        "init_heat_balance_stage",
        "init_heat_balance_source_order_path",
        "rec_keep_heat_balance_stage",
        "report_heat_balance_stage",
        "check_warmup_convergence_stage"
    )) {
    Assert-Contains -Path $manager -Pattern "$target(?:<[^>]+>)?\s*\(" -Description "HeatBalanceManager ledger target $target"
}

foreach ($routine in @(
        "ManageSurfaceHeatBalance",
        "InitSurfaceHeatBalance",
        "CalcHeatBalanceOutsideSurf",
        "CalcHeatBalanceInsideSurf",
        "UpdateFinalSurfaceHeatBalance",
        "UpdateThermalHistories",
        "ReportSurfaceHeatBalance"
    )) {
    Assert-Contains -Path $surfaceManager -Pattern $routine -Description "HeatBalanceSurfaceManager routine $routine"
}
foreach ($target in @(
        "manage_surface_heat_balance_stage",
        "manage_surface_heat_balance_source_order_path",
        "init_surface_heat_balance_stage",
        "init_surface_heat_balance_source_order_path",
        "calc_heat_balance_outside_surf_stage",
        "calc_heat_balance_outside_surf_source_order_path",
        "calc_heat_balance_inside_surf_stage",
        "calc_heat_balance_inside_surf_source_order_path",
        "update_final_surface_heat_balance_stage",
        "update_final_surface_heat_balance_source_order_path",
        "update_thermal_histories_stage",
        "update_thermal_histories_source_order_path",
        "report_surface_heat_balance_stage",
        "report_surface_heat_balance_source_order_path"
    )) {
    Assert-Contains -Path $surfaceManager -Pattern "$target(?:<[^>]+>)?\s*\(" -Description "HeatBalanceSurfaceManager ledger target $target"
}

Assert-Contains -Path $surfaceBalance -Pattern 'CalcHeatBalanceOutsideSurf' -Description "surface balance source owner"
Assert-Contains -Path $surfaceBalance -Pattern 'pub\(crate\) struct QuickOutsideConductionContext' -Description "quick outside conduction context owner"
Assert-Contains -Path $surfaceBalance -Pattern 'heat_balance_surface_boundary_balance' -Description "surface boundary balance owner"
Assert-Contains -Path $surfaceBalance -Pattern 'exterior_surface_boundary_balance' -Description "exterior surface boundary balance owner"
Assert-Contains -Path $surfaceBalance -Pattern 'reported_surface_outside_face_temperature_c' -Description "reported outside face temperature owner"
Assert-Contains -Path $surfaceBalance -Pattern 'surface_exterior_report_terms' -Description "surface exterior report terms owner"
Assert-Contains -Path $surfaceBalance -Pattern 'surface_inside_ctf_source_terms_w_per_m2' -Description "inside CTF source term owner"
Assert-Contains -Path $surfaceBalance -Pattern 'exterior_surface_energy_balance' -Description "exterior surface energy balance owner"
Assert-NotContains -Path $runtime -Pattern 'struct QuickOutsideConductionContext' -Description "runtime-owned quick outside conduction context"
Assert-NotContains -Path $runtime -Pattern 'fn heat_balance_surface_boundary_balance\s*\(' -Description "runtime-owned surface boundary balance"
Assert-NotContains -Path $runtime -Pattern 'fn exterior_surface_boundary_balance\s*\(' -Description "runtime-owned exterior surface boundary balance"
Assert-NotContains -Path $runtime -Pattern 'fn reported_surface_outside_face_temperature_c\s*\(' -Description "runtime-owned reported outside face temperature"
Assert-NotContains -Path $runtime -Pattern 'fn surface_exterior_report_terms\s*\(' -Description "runtime-owned surface exterior report terms"
Assert-NotContains -Path $runtime -Pattern 'fn surface_inside_ctf_source_terms_w_per_m2\s*\(' -Description "runtime-owned inside CTF source term"
Assert-NotContains -Path $runtime -Pattern 'fn exterior_surface_energy_balance\s*\(' -Description "runtime-owned exterior surface energy balance"
Assert-Contains -Path $surfaceLoop -Pattern 'CalcHeatBalanceInsideSurf' -Description "surface loop source owner"
Assert-Contains -Path $surfaceLoop -Pattern 'InterleavedSurfaceZoneBalanceResult' -Description "interleaved surface-zone loop result owner"
Assert-Contains -Path $surfaceLoop -Pattern 'run_interleaved_surface_zone_balance' -Description "interleaved surface-zone loop owner"
Assert-Contains -Path $surfaceLoop -Pattern 'run_surface_balance_passes' -Description "surface balance pass loop owner"
Assert-Contains -Path $surfaceLoop -Pattern 'ENERGYPLUS_MAX_ALLOWED_INSIDE_SURFACE_DELTA_C' -Description "surface convergence tolerance owner"
Assert-NotContains -Path $runtime -Pattern 'struct InterleavedSurfaceZoneBalanceResult' -Description "runtime-owned interleaved surface-zone loop result"
Assert-NotContains -Path $runtime -Pattern 'fn run_interleaved_surface_zone_balance\s*\(' -Description "runtime-owned interleaved surface-zone loop"
Assert-NotContains -Path $runtime -Pattern 'fn run_surface_balance_passes\s*\(' -Description "runtime-owned surface balance pass loop"
Assert-NotContains -Path $runtime -Pattern 'ENERGYPLUS_MAX_ALLOWED_INSIDE_SURFACE_DELTA_C' -Description "runtime-owned surface convergence tolerance"
Assert-Contains -Path $surfaceBoundary -Pattern 'pub\(crate\) struct SurfaceBoundaryTarget' -Description "surface boundary target owner"
Assert-Contains -Path $surfaceBoundary -Pattern 'resolve_surface_boundary_target' -Description "surface boundary target resolver owner"
Assert-Contains -Path $surfaceBoundary -Pattern 'seed_initial_surface_ctf_boundary_histories' -Description "initial CTF boundary seeding owner"
Assert-Contains -Path $surfaceBoundary -Pattern 'seed_energyplus_initial_surface_ctf_histories' -Description "EnergyPlus initial CTF boundary seeding owner"
Assert-Contains -Path $surfaceBoundary -Pattern 'surface_boundary_temperature_c' -Description "surface boundary temperature owner"
Assert-Contains -Path $surfaceBoundary -Pattern 'surface_steady_u_value_w_per_m2_k' -Description "surface steady U-value owner"
Assert-Contains -Path $surfaceBoundary -Pattern 'sync_adiabatic_outside_faces_to_inside_faces' -Description "adiabatic outside-face sync owner"
Assert-Contains -Path $surfaceBoundary -Pattern 'inside_ctf_outside_temperature_history_commit_override_c' -Description "inside CTF outside history commit override owner"
Assert-NotContains -Path $runtime -Pattern 'fn sync_adiabatic_outside_faces_to_inside_faces\s*\(' -Description "runtime-owned adiabatic outside-face sync"
Assert-NotContains -Path $runtime -Pattern 'fn inside_ctf_outside_temperature_history_commit_override_c\s*\(' -Description "runtime-owned inside CTF outside history commit override"
Assert-NotContains -Path $runtime -Pattern 'struct SurfaceBoundaryTarget' -Description "runtime-owned surface boundary target"
Assert-NotContains -Path $runtime -Pattern 'fn resolve_surface_boundary_target\s*\(' -Description "runtime-owned surface boundary target resolver"
Assert-NotContains -Path $runtime -Pattern 'fn seed_initial_surface_ctf_boundary_histories\s*\(' -Description "runtime-owned initial CTF boundary seeding"
Assert-NotContains -Path $runtime -Pattern 'fn seed_energyplus_initial_surface_ctf_histories\s*\(' -Description "runtime-owned EnergyPlus initial CTF boundary seeding"
Assert-NotContains -Path $runtime -Pattern 'fn surface_boundary_temperature_c\s*\(' -Description "runtime-owned surface boundary temperature"
Assert-NotContains -Path $runtime -Pattern 'fn surface_steady_u_value_w_per_m2_k\s*\(' -Description "runtime-owned surface steady U-value"
Assert-Contains -Path $airManager -Pattern 'ManageAirHeatBalance' -Description "HeatBalanceAirManager routine"
Assert-Contains -Path $airManager -Pattern 'manage_air_heat_balance_stage\s*\(' -Description "HeatBalanceAirManager ledger target manage_air_heat_balance_stage"
Assert-Contains -Path $airManager -Pattern 'manage_air_heat_balance_source_order_path(?:<[^>]+>)?\s*\(' -Description "HeatBalanceAirManager source-order wrapper"
Assert-Contains -Path $airManager -Pattern 'manage_air_heat_balance_compat(?:<[^>]+>)?\s*\(' -Description "HeatBalanceAirManager compatibility alias"
Assert-Contains -Path $airManager -Pattern 'weather_context_zone_air_heat_capacity_j_per_k' -Description "weather-driven zone-air heat capacity owner"
Assert-Contains -Path $airManager -Pattern 'update_zone_air_heat_capacities_from_weather_context' -Description "zone-air weather capacity updater owner"
Assert-Contains -Path $airManager -Pattern 'seed_zone_air_humidity_ratios_from_weather_series' -Description "zone-air weather-series humidity seeding owner"
Assert-Contains -Path $airManager -Pattern 'zone_air_heat_balance_air_storage_rate_w' -Description "zone-air storage report owner"
Assert-NotContains -Path $runtime -Pattern 'fn weather_context_zone_air_heat_capacity_j_per_k\s*\(' -Description "runtime-owned zone-air weather capacity implementation"
Assert-NotContains -Path $runtime -Pattern 'fn update_zone_air_heat_capacities_from_weather_context\s*\(' -Description "runtime-owned zone-air weather capacity updater"
Assert-NotContains -Path $runtime -Pattern 'fn seed_zone_air_humidity_ratios_from_weather_series\s*\(' -Description "runtime-owned weather-series humidity seeding"
Assert-NotContains -Path $runtime -Pattern 'fn zone_air_heat_balance_air_storage_rate_w\s*\(' -Description "runtime-owned zone-air storage report implementation"
Assert-Contains -Path $zonePredictorCorrector -Pattern 'ManageZoneAirUpdates' -Description "ZoneTempPredictorCorrector routine"
Assert-Contains -Path $zonePredictorCorrector -Pattern 'manage_zone_air_updates_stage\s*\(' -Description "ZoneTempPredictorCorrector ledger target manage_zone_air_updates_stage"
Assert-Contains -Path $zonePredictorCorrector -Pattern 'manage_zone_air_updates_source_order_path(?:<[^>]+>)?\s*\(' -Description "ZoneTempPredictorCorrector ManageZoneAirUpdates source-order wrapper"
Assert-Contains -Path $zonePredictorCorrector -Pattern 'manage_zone_air_updates_compat(?:<[^>]+>)?\s*\(' -Description "ZoneTempPredictorCorrector compatibility wrapper"
Assert-Contains -Path $zonePredictorCorrector -Pattern 'predict_system_loads_compat(?:<[^>]+>)?\s*\(' -Description "ZoneTempPredictorCorrector predictor compatibility wrapper"
Assert-Contains -Path $zonePredictorCorrector -Pattern 'ZONE_AIR_PREDICT_STEP_PATH' -Description "ZoneTempPredictorCorrector PredictStep source-order path"
Assert-Contains -Path $zonePredictorCorrector -Pattern 'PredictStep' -Description "ZoneTempPredictorCorrector PredictStep source routine"
Assert-Contains -Path $zonePredictorCorrector -Pattern 'predict_step_source_order_path(?:<[^>]+>)?\s*\(' -Description "ZoneTempPredictorCorrector PredictStep source-order wrapper"
Assert-Contains -Path $zonePredictorCorrector -Pattern 'ZONE_AIR_CORRECT_STEP_PATH' -Description "ZoneTempPredictorCorrector CorrectStep source-order path"
Assert-Contains -Path $zonePredictorCorrector -Pattern 'CorrectStep' -Description "ZoneTempPredictorCorrector CorrectStep source routine"
Assert-Contains -Path $zonePredictorCorrector -Pattern 'correct_step_source_order_path(?:<[^>]+>)?\s*\(' -Description "ZoneTempPredictorCorrector CorrectStep source-order wrapper"
Assert-Contains -Path $zonePredictorCorrector -Pattern 'ZONE_AIR_HISTORY_PUSH_REVERT_PATH' -Description "ZoneTempPredictorCorrector history push/revert path"
Assert-Contains -Path $zonePredictorCorrector -Pattern 'revert_zone_timestep_histories_source_order_path(?:<[^>]+>)?\s*\(' -Description "ZoneTempPredictorCorrector RevertZoneTimestepHistories source-order wrapper"
Assert-Contains -Path $zonePredictorCorrector -Pattern 'push_zone_timestep_histories_source_order_path(?:<[^>]+>)?\s*\(' -Description "ZoneTempPredictorCorrector PushZoneTimestepHistories source-order wrapper"
Assert-Contains -Path $zonePredictorCorrector -Pattern 'push_system_timestep_histories_source_order_path(?:<[^>]+>)?\s*\(' -Description "ZoneTempPredictorCorrector PushSystemTimestepHistories source-order wrapper"
Assert-Contains -Path $zonePredictorCorrector -Pattern 'revert_zone_timestep_histories_compat(?:<[^>]+>)?\s*\(' -Description "ZoneTempPredictorCorrector RevertZoneTimestepHistories compatibility wrapper"
Assert-Contains -Path $zonePredictorCorrector -Pattern 'push_zone_timestep_histories_compat(?:<[^>]+>)?\s*\(' -Description "ZoneTempPredictorCorrector PushZoneTimestepHistories compatibility wrapper"
Assert-Contains -Path $zonePredictorCorrector -Pattern 'push_system_timestep_histories_compat(?:<[^>]+>)?\s*\(' -Description "ZoneTempPredictorCorrector PushSystemTimestepHistories compatibility wrapper"
Assert-Contains -Path $zonePredictorCorrector -Pattern 'energyplus_zone_air_temperature_coefficients' -Description "ZoneTempPredictorCorrector coefficient owner"
Assert-Contains -Path $zonePredictorCorrector -Pattern 'energyplus_third_order_zone_air_temperature_c' -Description "ZoneTempPredictorCorrector third-order solver owner"
Assert-Contains -Path $zonePredictorCorrector -Pattern 'energyplus_analytical_zone_air_temperature_c' -Description "ZoneTempPredictorCorrector analytical solver owner"
Assert-NotContains -Path $runtime -Pattern 'fn energyplus_zone_air_temperature_coefficients\s*\(' -Description "runtime-owned zone-air coefficient implementation"
Assert-NotContains -Path $runtime -Pattern 'fn energyplus_third_order_zone_air_temperature_c\s*\(' -Description "runtime-owned third-order zone-air implementation"
Assert-NotContains -Path $runtime -Pattern 'fn energyplus_analytical_zone_air_temperature_c\s*\(' -Description "runtime-owned analytical zone-air implementation"
Assert-Contains -Path $zoneAirCorrection -Pattern 'ManageZoneAirUpdates' -Description "zone-air correction source owner"
Assert-Contains -Path $zoneAirCorrection -Pattern 'correct_zone_air_temperatures_from_current_surfaces' -Description "zone-air temperature correction owner"
Assert-Contains -Path $zoneAirCorrection -Pattern 'correct_zone_air_humidity_ratios_from_current_state' -Description "zone-air humidity correction owner"
Assert-Contains -Path $zoneAirCorrection -Pattern 'apply_energyplus_adaptive_system_timestep_zone_air_correction' -Description "adaptive system timestep correction owner"
Assert-Contains -Path $zoneAirCorrection -Pattern 'energyplus_down_interpolate_three_history_values' -Description "system timestep history interpolation owner"
Assert-Contains -Path $zoneAirCorrection -Pattern 'zone_air_system_timestep_storage_report_rate_w' -Description "system timestep air storage report owner"
Assert-NotContains -Path $runtime -Pattern 'fn heat_balance_zone_temperature_map\s*\(' -Description "runtime-owned zone temperature map"
Assert-NotContains -Path $runtime -Pattern 'fn correct_zone_air_temperatures_from_current_surfaces\s*\(' -Description "runtime-owned zone-air temperature correction"
Assert-NotContains -Path $runtime -Pattern 'fn correct_zone_air_humidity_ratios_from_current_state\s*\(' -Description "runtime-owned zone-air humidity correction"
Assert-NotContains -Path $runtime -Pattern 'fn apply_energyplus_adaptive_system_timestep_zone_air_correction\s*\(' -Description "runtime-owned adaptive system timestep correction"
Assert-NotContains -Path $runtime -Pattern 'fn zone_air_system_timestep_storage_report_rate_w\s*\(' -Description "runtime-owned system timestep air storage report"
Assert-NotContains -Path $runtime -Pattern 'fn correct_single_zone_air_temperature_from_current_surfaces\s*\(' -Description "runtime-owned single-zone temperature correction"
Assert-NotContains -Path $runtime -Pattern 'fn correct_single_zone_air_humidity_ratio_from_history\s*\(' -Description "runtime-owned single-zone humidity correction"
Assert-NotContains -Path $runtime -Pattern 'fn energyplus_down_interpolate_three_history_values\s*\(' -Description "runtime-owned down interpolation"
Assert-Contains -Path $insideConvection -Pattern 'CalcHeatBalanceInsideSurf' -Description "inside convection source owner"
Assert-Contains -Path $insideConvection -Pattern 'heat_balance_inside_convection_coefficients' -Description "inside convection coefficient owner"
Assert-Contains -Path $insideConvection -Pattern 'heat_balance_inside_convection_coefficient_inputs' -Description "inside convection coefficient input owner"
Assert-Contains -Path $insideConvection -Pattern 'zone_surface_convection_sums' -Description "zone surface convection sum owner"
Assert-Contains -Path $insideConvection -Pattern 'surface_inside_convection_report_coefficient_w_per_m2_k' -Description "inside convection report coefficient owner"
Assert-Contains -Path $insideConvection -Pattern 'surface_inside_convection_heat_gain_rate_per_area_w_per_m2' -Description "inside convection heat gain report owner"
Assert-Contains -Path $insideConvection -Pattern 'zone_air_heat_balance_surface_convection_rate_w' -Description "zone-air surface convection report owner"
Assert-NotContains -Path $runtime -Pattern 'fn heat_balance_inside_convection_coefficients\s*\(' -Description "runtime-owned inside convection coefficients"
Assert-NotContains -Path $runtime -Pattern 'fn heat_balance_inside_convection_coefficient_inputs\s*\(' -Description "runtime-owned inside convection coefficient inputs"
Assert-NotContains -Path $runtime -Pattern 'fn zone_surface_convection_sums\s*\(' -Description "runtime-owned zone surface convection sums"
Assert-NotContains -Path $runtime -Pattern 'fn surface_inside_convection_reference_air_temperature_c\s*\(' -Description "runtime-owned inside convection reference air report"
Assert-NotContains -Path $runtime -Pattern 'fn surface_inside_convection_report_coefficient_w_per_m2_k\s*\(' -Description "runtime-owned inside convection report coefficient"
Assert-NotContains -Path $runtime -Pattern 'fn surface_inside_convection_heat_gain_rate_per_area_w_per_m2\s*\(' -Description "runtime-owned inside convection heat gain report"
Assert-NotContains -Path $runtime -Pattern 'fn zone_air_heat_balance_surface_convection_rate_from_surface_reference_air_for_indices_w\s*\(' -Description "runtime-owned surface reference air convection report"
Assert-NotContains -Path $runtime -Pattern 'fn zone_air_heat_balance_surface_convection_rate_from_final_inside_hconv_report_w\s*\(' -Description "runtime-owned final hconv convection report"
Assert-NotContains -Path $runtime -Pattern 'fn zone_air_heat_balance_surface_convection_rate_w\s*\(' -Description "runtime-owned zone-air surface convection report"
Assert-NotContains -Path $runtime -Pattern 'fn zone_air_heat_balance_surface_convection_rate_at_air_temperature_w\s*\(' -Description "runtime-owned zone-air convection at air temperature report"
Assert-NotContains -Path $runtime -Pattern 'fn zone_air_heat_balance_surface_convection_rate_from_balance_w\s*\(' -Description "runtime-owned balance surface convection report"
Assert-Contains -Path $ctf -Pattern 'UpdateThermalHistories' -Description "CTF history owner stage"
Assert-Contains -Path $ctf -Pattern 'surface_inside_conduction_rate_w_for_report' -Description "CTF inside conduction report owner"
Assert-Contains -Path $ctf -Pattern 'surface_outside_conduction_rate_w_for_report' -Description "CTF outside conduction report owner"
Assert-Contains -Path $ctf -Pattern 'surface_ctf_history_slot_samples' -Description "CTF history slot sampling owner"
Assert-Contains -Path $ctf -Pattern 'advance_surface_ctf_histories' -Description "CTF history advancement owner"
Assert-NotContains -Path $runtime -Pattern 'fn surface_inside_conduction_rate_w' -Description "runtime-owned inside conduction implementation"
Assert-NotContains -Path $runtime -Pattern 'fn surface_outside_conduction_rate_w' -Description "runtime-owned outside conduction implementation"
Assert-NotContains -Path $runtime -Pattern 'fn update_surface_ctf_history_constants' -Description "runtime-owned CTF history constants implementation"
Assert-NotContains -Path $runtime -Pattern 'fn advance_surface_ctf_histories' -Description "runtime-owned CTF history advancement implementation"
Assert-Contains -Path $convection -Pattern 'CalcHeatBalanceInsideSurf' -Description "inside convection source owner"
Assert-Contains -Path $convection -Pattern 'CalcHeatBalanceOutsideSurf' -Description "outside convection source owner"
Assert-Contains -Path $convection -Pattern 'energyplus_tarp_inside_convection_coefficient_w_per_m2_k' -Description "inside TARP convection owner"
Assert-Contains -Path $convection -Pattern 'energyplus_doe2_outside_convection_coefficient_w_per_m2_k' -Description "DOE-2 outside convection owner"
Assert-Contains -Path $convection -Pattern 'exterior_convection_coefficient_w_per_m2_k' -Description "fallback exterior convection owner"
Assert-Contains -Path $convection -Pattern 'pub\(crate\) struct ExteriorConvectionTerms' -Description "exterior convection terms owner"
Assert-Contains -Path $convection -Pattern 'energyplus_exterior_convection_terms' -Description "exterior convection term calculation owner"
Assert-Contains -Path $convection -Pattern 'energyplus_surface_outside_wind_speed_m_per_s' -Description "exterior wind speed profile owner"
Assert-Contains -Path $convection -Pattern 'energyplus_surface_outdoor_air_temperature_c' -Description "exterior air temperature profile owner"
Assert-Contains -Path $convection -Pattern 'heat_balance_uses_doe2_outside_convection' -Description "DOE-2 outside convection selector owner"
Assert-NotContains -Path $runtime -Pattern 'fn energyplus_tarp_inside_convection_coefficient_w_per_m2_k\s*\(' -Description "runtime-owned TARP inside convection implementation"
Assert-NotContains -Path $runtime -Pattern 'fn energyplus_ashrae_tarp_natural_convection_w_per_m2_k\s*\(' -Description "runtime-owned ASHRAE TARP helper implementation"
Assert-NotContains -Path $runtime -Pattern 'fn energyplus_doe2_outside_convection_coefficient_w_per_m2_k\s*\(' -Description "runtime-owned DOE-2 outside convection implementation"
Assert-NotContains -Path $runtime -Pattern 'fn exterior_convection_coefficient_w_per_m2_k\s*\(' -Description "runtime-owned fallback exterior convection implementation"
Assert-NotContains -Path $runtime -Pattern 'struct ExteriorConvectionTerms' -Description "runtime-owned exterior convection terms"
Assert-NotContains -Path $runtime -Pattern 'fn energyplus_exterior_convection_terms\s*\(' -Description "runtime-owned exterior convection term calculation"
Assert-NotContains -Path $runtime -Pattern 'fn energyplus_surface_outside_wind_speed_m_per_s\s*\(' -Description "runtime-owned exterior wind speed profile"
Assert-NotContains -Path $runtime -Pattern 'fn energyplus_surface_outdoor_air_temperature_c\s*\(' -Description "runtime-owned exterior air temperature profile"
Assert-NotContains -Path $runtime -Pattern 'fn heat_balance_uses_doe2_outside_convection\s*\(' -Description "runtime-owned DOE-2 outside convection selector"
Assert-Contains -Path $longwave -Pattern 'CalcHeatBalanceOutsideSurf' -Description "exterior longwave source owner"
Assert-Contains -Path $longwave -Pattern 'pub\(crate\) struct ExteriorLongwaveTerms' -Description "exterior longwave terms owner"
Assert-Contains -Path $longwave -Pattern 'energyplus_exterior_longwave_terms' -Description "exterior longwave term calculation owner"
Assert-Contains -Path $longwave -Pattern 'energyplus_linearized_radiation_coefficient_w_per_m2_k' -Description "linearized exterior radiation coefficient owner"
Assert-Contains -Path $longwave -Pattern 'horizontal_infrared_sky_temperature_c' -Description "horizontal infrared sky temperature owner"
Assert-NotContains -Path $runtime -Pattern 'struct ExteriorLongwaveTerms' -Description "runtime-owned exterior longwave terms"
Assert-NotContains -Path $runtime -Pattern 'fn energyplus_exterior_longwave_terms\s*\(' -Description "runtime-owned exterior longwave implementation"
Assert-NotContains -Path $runtime -Pattern 'fn energyplus_linearized_radiation_coefficient_w_per_m2_k\s*\(' -Description "runtime-owned linearized exterior radiation coefficient"
Assert-NotContains -Path $runtime -Pattern 'fn horizontal_infrared_sky_temperature_c\s*\(' -Description "runtime-owned horizontal infrared sky temperature"
Assert-Contains -Path $radiation -Pattern 'CalcHeatBalanceOutsideSurf' -Description "exterior radiation source owner"
Assert-Contains -Path $radiation -Pattern 'CalcHeatBalanceInsideSurf' -Description "interior radiation source owner"
Assert-Contains -Path $solar -Pattern 'append_surface_incident_solar_radiation_series' -Description "surface incident solar diagnostic owner"
Assert-Contains -Path $solar -Pattern 'surface_incident_solar_components_hourly_average_w_per_m2' -Description "surface incident solar component owner"
Assert-Contains -Path $radiation -Pattern 'pub\(crate\) enum InteriorLongwaveExchangeProbe' -Description "interior longwave probe selector owner"
Assert-Contains -Path $radiation -Pattern 'update_surface_inside_longwave_exchange_probe' -Description "grey interior longwave probe owner"
Assert-Contains -Path $radiation -Pattern 'update_surface_inside_scriptf_longwave_exchange_probe' -Description "ScriptF interior longwave probe owner"
Assert-Contains -Path $radiation -Pattern 'energyplus_scriptf_from_view_factors' -Description "ScriptF matrix owner"
Assert-Contains -Path $radiation -Pattern 'energyplus_approximate_view_factors' -Description "interior approximate view-factor owner"
Assert-NotContains -Path $runtime -Pattern 'fn append_surface_incident_solar_radiation_series' -Description "runtime-owned surface incident solar diagnostic"
Assert-NotContains -Path $runtime -Pattern 'fn surface_incident_solar_components_hourly_average_w_per_m2' -Description "runtime-owned surface incident solar components"
Assert-NotContains -Path $runtime -Pattern 'fn update_surface_inside_longwave_exchange_probe\s*\(' -Description "runtime-owned grey interior longwave implementation"
Assert-NotContains -Path $runtime -Pattern 'fn update_surface_inside_scriptf_longwave_exchange_probe\s*\(' -Description "runtime-owned ScriptF interior longwave implementation"
Assert-NotContains -Path $runtime -Pattern 'fn energyplus_scriptf_from_view_factors\s*\(' -Description "runtime-owned ScriptF matrix implementation"
Assert-NotContains -Path $runtime -Pattern 'fn energyplus_approximate_view_factors\s*\(' -Description "runtime-owned approximate view-factor implementation"
Assert-Contains -Path $reports -Pattern 'ReportHeatBalance' -Description "zone report owner"
Assert-Contains -Path $reports -Pattern 'HeatBalanceResultSeriesTraces' -Description "heat-balance result trace bundle owner"
Assert-Contains -Path $reports -Pattern 'heat_balance_result_store_from_traces' -Description "heat-balance ResultStore materialization owner"
Assert-Contains -Path $reports -Pattern 'ResultStore::new' -Description "heat-balance ResultStore writer owner"
Assert-Contains -Path $trace -Pattern 'HeatBalanceRunPeriodSamples' -Description "run-period sample bundle owner"
Assert-Contains -Path $trace -Pattern 'zone_scalar_trace_series_from_state' -Description "zone scalar trace factory owner"
Assert-Contains -Path $trace -Pattern 'push_zone_scalar_trace_averages' -Description "zone scalar trace average owner"
Assert-Contains -Path $trace -Pattern 'push_zone_air_heat_balance_trace_values' -Description "zone-air heat-balance trace push owner"
Assert-Contains -Path $trace -Pattern 'push_surface_heat_balance_trace_averages' -Description "surface trace average push owner"
Assert-Contains -Path $heatBalanceMod -Pattern 'pub mod summary;' -Description "summary module declaration"
Assert-Contains -Path $heatBalanceMod -Pattern 'pub use summary::\*;' -Description "summary module facade export"
Assert-Contains -Path $state -Pattern 'HeatBalanceSimulationOptions' -Description "heat-balance simulation options owner"
Assert-Contains -Path $state -Pattern 'pub\(crate\) struct HeatBalanceSurfaceIndexes' -Description "precomputed heat-balance surface index owner"
foreach ($surfaceIndex in @(
        "surfaces_by_zone",
        "surfaces_by_construction",
        "opaque_surfaces",
        "fenestration_surfaces",
        "ctf_surfaces",
        "no_mass_surfaces"
    )) {
    Assert-Contains -Path $state -Pattern $surfaceIndex -Description "precomputed heat-balance surface index $surfaceIndex"
}
foreach ($surfaceCacheField in @(
        "area_m2",
        "azimuth_deg",
        "tilt_deg",
        "thermal_resistance_m2_k_per_w",
        "heat_capacity_j_per_m2_k",
        "outside_boundary_condition",
        "outside_boundary_target_surface_id",
        "outside_boundary_target_zone_id",
        "ctf"
    )) {
    Assert-Contains -Path $state -Pattern $surfaceCacheField -Description "precomputed surface cache field $surfaceCacheField"
}
Assert-Contains -Path $surfaceManager -Pattern 'pub\(crate\) struct ConstructionThermalData' -Description "construction thermal data cache entry"
Assert-Contains -Path $initialization -Pattern 'construction_thermal_data' -Description "construction thermal data cached during heat-balance initialization"
Assert-Contains -Path $initialization -Pattern 'construction_ctf_coefficients_by_name' -Description "CTF coefficient cache initialized by construction"
Assert-Contains -Path $weather -Pattern 'pub struct WeatherTimestepSeries' -Description "precomputed weather timestep series"
Assert-Contains -Path $weather -Pattern 'pub fn precompute_weather_timestep_series' -Description "weather timestep precompute entry"
Assert-Contains -Path $weather -Pattern 'pub\(crate\) fn next_solar_weather_record_within_day\s*\(' -Description "day-local solar NextHr weather selector"
Assert-Contains -Path $weatherCalendar -Pattern 'pub use ep_model::CalendarDateRule as EpwCalendarDateRule' -Description "shared typed EPW calendar date rule"
Assert-Contains -Path $weatherCalendar -Pattern 'pub daylight_saving_period: Option<EpwDaylightSavingPeriod>' -Description "EPW daylight-saving metadata"
Assert-Contains -Path $weatherCalendar -Pattern 'fn parse_calendar_date_rule\s*\(' -Description "EPW daylight-saving date-rule parser"
Assert-Contains -Path $weatherCalendar -Pattern 'pub struct EpwHoliday' -Description "typed EPW holiday metadata"
Assert-Contains -Path $weatherCalendar -Pattern 'pub holidays: Vec<EpwHoliday>' -Description "source-order EPW holiday collection"
Assert-Contains -Path $weatherCalendar -Pattern 'let holiday_count = parse_holiday_count' -Description "EPW holiday-count parser"
Assert-Contains -Path $weatherCalendar -Pattern 'holidays\.push\(EpwHoliday \{ name, date \}\)' -Description "EPW holiday name/date intake"
Assert-Contains -Path $weatherTests -Pattern 'fn parses_epw_holidays_in_header_order\s*\(' -Description "EPW holiday parser order test"
Assert-Contains -Path $weatherTests -Pattern 'fn rejects_invalid_or_incomplete_epw_holiday_fields\s*\(' -Description "EPW holiday parser validation test"
Assert-Contains -Path $daylightSaving -Pattern 'pub struct DaylightSavingAxisState' -Description "shared time-axis daylight-saving state"
Assert-Contains -Path $daylightSaving -Pattern 'fn resolve_daylight_saving_axis_state\s*\(' -Description "time-axis daylight-saving resolver"
Assert-Contains -Path $timeAxis -Pattern 'dst: daylight_saving_is_active\(daylight_saving, weather_day_of_year\)' -Description "daily daylight-saving projection into time points"
Assert-Contains -Path $runtimeTestCalendar -Pattern 'fn weather_file_fixed_date_daylight_saving_is_inclusive_on_both_time_axes\s*\(' -Description "fixed-date inclusive DST axis test"
Assert-Contains -Path $runtimeTestCalendar -Pattern 'fn weather_file_nth_weekday_daylight_saving_rules_resolve_like_energyplus\s*\(' -Description "nth-weekday DST rule test"
Assert-Contains -Path $runtimeTestCalendar -Pattern 'fn weather_file_nth_weekday_daylight_saving_preserves_run_period_month_weekdays\s*\(' -Description "leap-policy RunPeriod month-weekday DST test"
Assert-Contains -Path $runtimeTestCalendar -Pattern 'fn weather_file_last_weekday_daylight_saving_rules_resolve_like_energyplus\s*\(' -Description "last-weekday DST rule test"
Assert-Contains -Path $runtimeTestCalendar -Pattern 'fn weather_file_daylight_saving_range_wraps_across_the_weather_year\s*\(' -Description "year-wrapping DST range test"
Assert-Contains -Path $timeWeatherSchedule -Pattern 'Site Daylight Saving Time Status' -Description "DST status report mapping"
Assert-Contains -Path $timeWeatherSchedule -Pattern 'daylight_saving_hourly_samples' -Description "DST report diagnostic sample count"
Assert-Contains -Path $calendarObjects -Pattern 'pub struct RunPeriodSpecialDay' -Description "typed input-file special-day object"
Assert-Contains -Path $calendarObjects -Pattern 'pub fn parse_calendar_date_rule' -Description "shared calendar date-rule parser"
Assert-Contains -Path $calendarObjects -Pattern 'parse_ordinal_number\(tokens\[0\]\)' -Description "Nth-weekday ordinal parser path"
Assert-Contains -Path $calendarObjects -Pattern 'NthWeekdayInMonth' -Description "typed Nth-weekday calendar-rule variant"
Assert-Contains -Path $calendarObjects -Pattern 'LastWeekdayInMonth' -Description "typed last-weekday calendar-rule variant"
Assert-Contains -Path $rawModel -Pattern 'pub fn ordered_instances\s*\(' -Description "raw-model ordered-instance gateway"
Assert-Contains -Path $rawModel -Pattern 'pub fn load_epjson_file_with_idf_order\s*\(' -Description "IDF-backed epJSON order-overlay loader"
Assert-Contains -Path $idfOrder -Pattern 'object_type: "RunPeriodControl:SpecialDays"' -Description "SpecialDays-only declaration-order target"
Assert-Contains -Path $idfOrder -Pattern 'IDF declaration-order recovery count mismatch' -Description "IDF/epJSON order-overlay count mismatch rejection"
Assert-Contains -Path $idfOrder -Pattern 'converted epJSON contains multiple case-insensitive name matches' -Description "ambiguous IDF order-overlay name rejection"
Assert-Contains -Path $compiler -Pattern 'fn parse_run_period_special_days\s*\(' -Description "typed RunPeriodControl SpecialDays compiler"
Assert-Contains -Path $compiler -Pattern 'raw_model\.ordered_instances\(object_type\)' -Description "compiler consumes raw-model ordered-instance gateway"
Assert-Contains -Path $compiler -Pattern 'fn parses_typed_run_period_special_day_rules_types_and_coverage\s*\(' -Description "typed Nth/last special-day compiler test"
Assert-Contains -Path $calendarRules -Pattern 'fn resolve_calendar_date_rule\s*\(' -Description "shared EnergyPlus calendar date-rule resolver"
Assert-Contains -Path $calendarRules -Pattern 'CalendarDateRule::NthWeekdayInMonth' -Description "Nth-weekday calendar resolver branch"
Assert-Contains -Path $calendarRules -Pattern 'CalendarDateRule::LastWeekdayInMonth' -Description "last-weekday calendar resolver branch"
Assert-Contains -Path $calendarRules -Pattern 'CalendarRuleResolutionError::NthWeekdayDoesNotExist' -Description "nonexistent Nth-weekday resolver error"
Assert-Contains -Path $calendarRules -Pattern 'day_of_month > days_in_month\(weather_shape_year, month\)' -Description "nonexistent Nth-weekday month-bound rejection"
Assert-Contains -Path $dayType -Pattern 'pub const fn energyplus_index\s*\(' -Description "EnergyPlus Site Day Type Index mapping"
Assert-Contains -Path $specialDays -Pattern 'pub struct SpecialDayAxisState' -Description "shared time-axis special-day state"
Assert-Contains -Path $specialDays -Pattern 'fn resolve_special_day_axis_state\s*\(' -Description "time-axis special-day resolver"
Assert-Contains -Path $specialDays -Pattern 'weather_calendar[\s\S]*start_year_is_weather_effective_leap_year[\s\S]*unwrap_or\(calendar\.start_year_is_leap_year\)' -Description "special-day annual table uses the environment start-year weather shape"
Assert-Contains -Path $specialDays -Pattern 'SpecialDayDateRuleDoesNotExist' -Description "special-day nonexistent Nth-weekday error mapping"
Assert-Contains -Path $specialDays -Pattern 'CalendarRuleResolutionError::NthWeekdayDoesNotExist[\s\S]*TimeAxisError::SpecialDayDateRuleDoesNotExist' -Description "Nth-weekday resolver-to-special-day error mapping"
Assert-Contains -Path $specialDays -Pattern 'run_period.use_weather_file_holidays_and_special_days' -Description "RunPeriod EPW holiday use-policy branch"
Assert-Contains -Path $specialDays -Pattern 'SpecialDaySource::WeatherFile' -Description "weather-file special-day source attribution"
Assert-Contains -Path $specialDays -Pattern 'DayType::Sunday' -Description "source-exact EPW holiday Sunday day type"
Assert-Contains -Path $specialDays -Pattern 'for offset in 0\.\.duration_days' -Description "special-day inclusive duration loop"
Assert-Contains -Path $specialDays -Pattern 'wrap_ordinal\(start_day_of_year \+ offset, days_in_year\)' -Description "special-day common-year and leap-year annual-table wrap application"
Assert-Contains -Path $specialDays -Pattern 'fn wrap_ordinal\s*\([\s\S]*% days_in_year' -Description "special-day cyclic annual-table wrap owner"
Assert-Contains -Path $specialDays -Pattern 'day_types_by_ordinal\[ordinal as usize\] = Some\(day_type\)' -Description "later special-day definitions overwrite ordinal state"
Assert-Contains -Path $timeAxis -Pattern 'special_day_type: day\.special_day_type' -Description "special day projected into both time-point axes"
Assert-Contains -Path $runtimeTestSpecialDays -Pattern 'fn model_special_day_overrides_both_axes_for_every_hour_of_leap_day\s*\(' -Description "fixed-date special day both-axis test"
Assert-Contains -Path $runtimeTestSpecialDays -Pattern 'fn special_day_duration_is_inclusive_and_wraps_the_same_year_annual_table\s*\(' -Description "special-day same-year annual-table duration and wrap test"
Assert-Contains -Path $runtimeTestSpecialDays -Pattern 'start\.day_of_year, 366' -Description "special-day leap-year day-366 wrap test boundary"
Assert-Contains -Path $durationWrapGate -Pattern 'StartDayOfYear = 365' -Description "EnergyPlus common-year duration wrap at annual day 365"
Assert-Contains -Path $durationWrapGate -Pattern 'StartDayOfYear = 366' -Description "EnergyPlus leap-year duration wrap at annual day 366"
Assert-Contains -Path $durationWrapGate -Pattern 'if \(\$index -lt 48\) \{ 8\.0 \} else \{ \$case\.FinalDayTypeIndex \}' -Description "EnergyPlus duration-three wrap produces two in-range Holiday days then weekday"
Assert-Contains -Path $durationWrapGate -Pattern 'EnergyPlus Completed Successfully-- 0 Warning; 0 Severe Errors;' -Description "EnergyPlus duration-wrap clean source branch"
Assert-Contains -Path $weekendHolidayGate -Pattern 'Id = "calendar_special_day_weekend_rule_blank_hourly_exact_001"[\s\S]*?WeekendPolicy = \$true[\s\S]*?BlankPolicy = \$true[\s\S]*?StartDay = 29[\s\S]*?StartDayOfYear = 60[\s\S]*?ShiftDays = 1' -Description "blank A5 follows EnergyPlus executable enabled Sunday-shift branch"
Assert-Contains -Path $weekendHolidayGate -Pattern '\(\?m\)\^\\s\*,\\s\*!-\\s\*Apply Weekend Holiday Rule\\s\*\$' -Description "blank A5 is genuinely empty"
Assert-Contains -Path $weekendHolidayGate -Pattern '\$casePolicyFieldMatches\.Count -ne 1' -Description "every fixed-Sunday fixture has exactly one annotated A5 field"
Assert-Contains -Path $weekendHolidayGate -Pattern '\$idfText\.Remove\(\$policyFieldMatch\.Index, \$policyFieldMatch\.Length\)\.Insert\(' -Description "three-case normalization replaces only the matched A5 field span"
Assert-Contains -Path $weekendHolidayGate -Pattern 'Weekend holiday IDFs differ outside the explicit Yes/No/blank policy field' -Description "three fixed-Sunday fixtures isolate only A5"
Assert-Contains -Path $weekendHolidayGate -Pattern '\$specialDays\.apply_weekend_rule -ne \$case\.WeekendPolicy' -Description "reported blank A5 policy follows executable state"
Assert-Contains -Path $weekendHolidayGate -Pattern '\$resolved\[0\]\.start_day_of_year -ne \$case\.StartDayOfYear' -Description "fixed-Sunday resolved day-of-year contract"
Assert-Contains -Path $weekendHolidayGate -Pattern '\$resolved\[0\]\.weekend_shift_days -ne \$case\.ShiftDays' -Description "fixed-Sunday resolved shift contract"
Assert-Contains -Path $weekendHolidayGate -Pattern 'EnergyPlus Completed Successfully-- 0 Warning; 0 Severe Errors;' -Description "fixed-Sunday three-case clean EnergyPlus completion"
Assert-Contains -Path $weekendHolidayGate -Pattern 'Blank and explicit Yes oracle values diverge at sample \$index' -Description "blank and explicit Yes exact value equality"
Assert-Contains -Path $weekendHolidayGate -Pattern 'Blank and explicit Yes oracle timestamps diverge at sample \$index' -Description "blank and explicit Yes exact timestamp equality"
Assert-Contains -Path $weekendHolidayGate -Pattern 'Fixed-Sunday explicit Yes/No and blank weekend holiday policy exact gate passed\.' -Description "fixed-Sunday three-case blocking gate"
Assert-Contains -Path $overlapOrderGate -Pattern 'ExpectedNames = @\("ZULU HOLIDAY DEFINITION", "ALPHA CUSTOM DAY DEFINITION"\)' -Description "Zulu-then-Alpha SpecialDays source order"
Assert-Contains -Path $overlapOrderGate -Pattern 'ExpectedNames = @\("ALPHA CUSTOM DAY DEFINITION", "ZULU HOLIDAY DEFINITION"\)' -Description "Alpha-then-Zulu SpecialDays source order"
Assert-Contains -Path $overlapOrderGate -Pattern 'identical SpecialDays definitions in exact reverse order' -Description "paired overlap fixtures isolate reversed declaration order"
Assert-Contains -Path $overlapOrderGate -Pattern '\$specialDays\.input_file_declared -ne 2' -Description "paired overlap gate resolves exactly two input-file definitions"
Assert-Contains -Path $overlapOrderGate -Pattern '\$entry\.name -cne \$case\.ExpectedNames\[\$definitionIndex\]' -Description "resolved SpecialDays retain IDF declaration order"
Assert-Contains -Path $overlapOrderGate -Pattern '\$expectedValue = if \(\$dayOffset -eq 0\) \{ 1\.0 \} elseif \(\$dayOffset -eq 1\) \{ \$case\.MiddleDayTypeIndex \} else \{ 3\.0 \}' -Description "later overlapping SpecialDays definition wins exact middle-day value"
Assert-Contains -Path $overlapOrderGate -Pattern 'EnergyPlus-versus-Rust warning text, count, repetition, and diagnostics parity' -Description "overlap numerical claim excludes warning parity"
Assert-Contains -Path $epwIdfPrecedenceGate -Pattern 'weather-file-then-input-file' -Description "EPW/IDF overlap gate locks source precedence"
Assert-Contains -Path $epwIdfPrecedenceGate -Pattern 'CustomDay1' -Description "EPW/IDF overlap gate locks input-file winning day type"
Assert-Contains -Path $epwIdfPrecedenceGate -Pattern '\$BaseIdfPath = Join-Path \$RepoRoot "data\\conformance_cases\\calendar_epw_holiday_fixed_date_enabled_hourly_exact_001\\calendar_epw_holiday_fixed_date_enabled_hourly_exact\.idf"' -Description "EPW/IDF overlap reuses enabled base IDF"
Assert-Contains -Path $epwIdfPrecedenceGate -Pattern '\$strippedIdf = \[regex\]::Replace\(\$idfText, \$specialDayPattern, ''''\)' -Description "EPW/IDF overlap strips the one added SpecialDays object"
Assert-Contains -Path $epwIdfPrecedenceGate -Pattern 'Precedence fixture must differ from the existing EPW-holiday-enabled fixture only by its one SpecialDays object' -Description "EPW/IDF overlap base-plus-one-object isolation"
Assert-Contains -Path $epwIdfPrecedenceGate -Pattern 'warning text, count, repetition, and diagnostics parity' -Description "EPW/IDF numerical claim excludes warning parity"
Assert-Contains -Path $epwWeekdayHolidayGate -Pattern '\$ExpectedHeader = "HOLIDAYS/DAYLIGHT SAVINGS,Yes,0,0,2,Fourth Monday EPW Holiday,4th Monday in February,Last Wednesday EPW Holiday,Last Wednesday in February"' -Description "exact source-ordered EPW weekday holiday header"
Assert-Contains -Path $epwWeekdayHolidayGate -Pattern '\$fields\.Count -ne 35 -or \$fields\[4\] -ne "60"[\s\S]*\$weatherPayloads\.Count -ne 1[\s\S]*exactly one RunPeriod and one Output:Variable' -Description "EPW weekday holiday complete isolated source rows"
Assert-Contains -Path $epwWeekdayHolidayGate -Pattern '\$expectedRunPeriodFields = @\([\s\S]*"2", "23", "2032", "2", "25", "2032", "Monday"[\s\S]*"Yes", "No", "No", "No", "No", "No"' -Description "EPW weekday holiday exact explicit 2032 RunPeriod"
Assert-Contains -Path $epwWeekdayHolidayGate -Pattern 'DayOfYear = 54[\s\S]*DayOfYear = 56' -Description "EPW weekday holiday exact resolved day-of-year order"
Assert-Contains -Path $epwWeekdayHolidayGate -Pattern '\$actual\.source -ne "weather-file"[\s\S]*\$actual\.day_type -ne "Sunday"[\s\S]*\$actual\.day_type_index -ne 1' -Description "EPW weekday holiday source-exact Sunday type"
Assert-Contains -Path $epwWeekdayHolidayGate -Pattern '\$series\.level -cne "conformance"[\s\S]*\$series\.class -cne "weather"[\s\S]*\$series\.frequency -cne "hourly"[\s\S]*\$series\.source -cne "eso"[\s\S]*\$series\.alignment -cne "timestamp"' -Description "EPW weekday holiday promoted series metadata gate"
Assert-Contains -Path $epwWeekdayHolidayGate -Pattern '\$expectedValue = @\(1\.0, 3\.0, 1\.0\)\[\$dayOffset\]' -Description "EPW weekday holiday exact 72-sample daily values"
Assert-Contains -Path $epwWeekdayHolidayGate -Pattern '\$timestampMatch = \[regex\]::Match\([\s\S]*\$timestampMatch\.Groups\[1\]\.Value -ne \(\$dayOffset \+ 1\)[\s\S]*\$timestampMatch\.Groups\[5\]\.Value -ne \$expectedHour[\s\S]*\$timestampMatch\.Groups\[8\]\.Value\.Trim\(\) -cne \$expectedLabel' -Description "EPW weekday holiday exact raw timestamp fields"
Assert-Contains -Path $epwWeekdayHolidayGate -Pattern '\$specialDayEioRows\.Count -ne 2[\s\S]*\$fields\[1\] -cne \$ExpectedResolved\[\$index\]\.Name[\s\S]*\$fields\[2\] -cne "Sunday" -or \$fields\[3\] -cne "WeatherFile"' -Description "EPW weekday holiday EIO ordered name/type/source gate"
Assert-Contains -Path $epwWeekdayHolidayGate -Pattern 'EnergyPlus Completed Successfully-- 0 Warning; 0 Severe Errors;[\s\S]*Exact EPW fourth-Monday and last-Wednesday holiday rule gate passed\.' -Description "EPW weekday holiday exact clean completion gate"
Assert-Contains -Path $epwWeekdayHolidayGate -Pattern '\$oracleEndText[\s\S]*clean EnergyPlus end record' -Description "EPW weekday holiday exact clean eplusout.end gate"
Assert-Contains -Path $fixedDstPolicyGate -Pattern '\$ExpectedHeader = "HOLIDAYS/DAYLIGHT SAVINGS,Yes,2/29,3/1,0"[\s\S]*\$ExpectedDataPeriod = "DATA PERIODS,1,1,Data,Sunday,2/28,3/1"[\s\S]*\$weatherLines\.Count -ne 81[\s\S]*\$weatherNonblankLines\.Count -ne 80[\s\S]*IsNullOrWhiteSpace\(\$weatherLines\[-1\]\)[\s\S]*\$weatherRows\.Count -ne 72' -Description "fixed-date DST policy pair exact 80-nonblank-plus-trailing-blank, 72-row source shape"
Assert-Contains -Path $fixedDstPolicyGate -Pattern 'for \(\$rowIndex = 0; \$rowIndex -lt 72; \+\+\$rowIndex\)[\s\S]*\$fields\[1\] -ne \$ExpectedMonths\[\$dayIndex\][\s\S]*\$fields\[2\] -ne \$ExpectedDates\[\$dayIndex\][\s\S]*\$fields\[3\] -ne \$expectedHour' -Description "fixed-date DST policy pair full 72-row source order"
Assert-Contains -Path $fixedDstPolicyGate -Pattern 'Id = "calendar_dst_fixed_date_hourly_exact_001"[\s\S]*UsePolicy = \$true[\s\S]*ExpectedDst = @\(0, 1, 1\)[\s\S]*ActiveSamples = 48[\s\S]*HasResolvedPeriod = \$true[\s\S]*Id = "calendar_dst_fixed_date_disabled_hourly_exact_001"[\s\S]*UsePolicy = \$false[\s\S]*ExpectedDst = @\(0, 0, 0\)[\s\S]*ActiveSamples = 0[\s\S]*HasResolvedPeriod = \$false' -Description "fixed-date DST policy pair exact case states"
Assert-Contains -Path $fixedDstPolicyGate -Pattern 'ExpectedEnvironmentEio = "Environment,DST FIXED DATE RUN PERIOD,WeatherFileRunPeriod,02/28/2016,03/01/2016,Sunday,3,Use RunPeriod Specified Day,Yes,No,No,No,No,Clark and Allen"[\s\S]*ExpectedDaylightSavingEio = "Environment:Daylight Saving,Yes,WeatherFile,02/29,03/01"[\s\S]*ExpectedEnvironmentEio = "Environment,DST FIXED DATE RUN PERIOD,WeatherFileRunPeriod,02/28/2016,03/01/2016,Sunday,3,Use RunPeriod Specified Day,No,No,No,No,No,Clark and Allen"[\s\S]*ExpectedDaylightSavingEio = "Environment:Daylight Saving,No,RunPeriod Object"' -Description "fixed-date DST policy pair exact EnergyPlus 26.1 EIO rows"
Assert-Contains -Path $fixedDstPolicyGate -Pattern 'script = "scripts/dev\.cmd compare-calendar-dst-fixed-date-policy-exact"[\s\S]*\$policyRows\.Count -ne 1[\s\S]*\$expectedRunPeriodFields = @\([\s\S]*"DST Fixed Date Run Period", "2", "28", "2016", "3", "1", "2016", "Sunday"[\s\S]*"No", \$expectedUseField, "No", "No", "No", "No"[\s\S]*RunPeriodControl:SpecialDays[\s\S]*RunPeriodControl:DaylightSavingTime' -Description "fixed-date DST policy pair exact manifest and isolated RunPeriod contracts"
Assert-Contains -Path $fixedDstPolicyGate -Pattern '\$enabledPolicyMatches\.Count -ne 1 -or \$disabledPolicyMatches\.Count -ne 1[\s\S]*ToBase64String\(\$enabledNormalizedBytes\) -cne \[Convert\]::ToBase64String\(\$disabledNormalizedBytes\)' -Description "fixed-date DST policy pair byte-equivalent single-token isolation"
Assert-Contains -Path $fixedDstPolicyGate -Pattern '\$summary\.case_id -cne \$case\.Id[\s\S]*\$summary\.comparison_class -cne "conformance"[\s\S]*\$summary\.time_axis_samples -ne 72[\s\S]*\$summary\.series_count -ne 1[\s\S]*\$summary\.conformance_series_count -ne 1[\s\S]*\$summary\.gate\.script -cne "scripts/dev\.cmd compare-calendar-dst-fixed-date-policy-exact"[\s\S]*\$summary\.gate\.blocking -ne \$true' -Description "fixed-date DST policy pair exact summary identity and gate metadata"
Assert-Contains -Path $fixedDstPolicyGate -Pattern 'weather_file_period_declared -ne \$true[\s\S]*run_period_uses_weather_file_period -ne \$case\.UsePolicy[\s\S]*\$daylightSaving\.active -ne \$case\.UsePolicy[\s\S]*start_day_of_year -ne 60[\s\S]*end_day_of_year -ne 61[\s\S]*wraps_year -ne \$false[\s\S]*\$null -ne \$daylightSaving\.resolved_period[\s\S]*daylight_saving_hourly_samples -ne \$case\.ActiveSamples' -Description "fixed-date DST policy pair enabled resolution and disabled null state"
Assert-Contains -Path $fixedDstPolicyGate -Pattern '\$series\.level -cne "conformance"[\s\S]*\$series\.class -cne "weather"[\s\S]*\$series\.frequency -cne "hourly"[\s\S]*\$series\.source -cne "eso"[\s\S]*\$series\.alignment -cne "timestamp"[\s\S]*expected_samples -ne 72[\s\S]*timestamp_contract -cne "ordered-exact-unique"[\s\S]*max_abs_delta -ne 0\.0' -Description "fixed-date DST policy pair promoted exact series metadata"
Assert-Contains -Path $fixedDstPolicyGate -Pattern '\$null -ne \$series\.first_divergence -or \$null -ne \$series\.first_timestamp_divergence' -Description "fixed-date DST policy pair unique series has no divergence"
Assert-Contains -Path $fixedDstPolicyGate -Pattern '\$values\.Count -ne 72 -or \$timestampRows\.Count -ne 72[\s\S]*\$expectedDst = \[int\]\$case\.ExpectedDst\[\$dayOffset\][\s\S]*\$values\[\$index\] -ne \[double\]\$expectedDst[\s\S]*Groups\[4\]\.Value -ne \$expectedDst[\s\S]*Groups\[8\]\.Value\.Trim\(\) -cne \$ExpectedDayTypes' -Description "fixed-date DST policy pair raw EnergyPlus ESO values and timestamp DST fields"
Assert-Contains -Path $fixedDstPolicyGate -Pattern '\^\\d\+,1,Environment,Site Daylight Saving Time Status \\\[\\\] !Hourly\$' -Description "fixed-date DST policy pair exact ESO dictionary type and frequency"
Assert-Contains -Path $fixedDstPolicyGate -Pattern '\$environmentEioRows\.Count -ne 1[\s\S]*ExpectedEnvironmentEio[\s\S]*Count -ne 14[\s\S]*\$daylightSavingEioRows\.Count -ne 1[\s\S]*ExpectedDaylightSavingEio[\s\S]*ExpectedDaylightSavingEioFieldCount[\s\S]*enabledEnvironmentFields\[8\] -cne "Yes"[\s\S]*disabledEnvironmentFields\[8\] -cne "No"[\s\S]*differ outside the Use Daylight Saving field' -Description "fixed-date DST policy pair exact EIO field hardening"
Assert-Contains -Path $fixedDstPolicyGate -Pattern 'EnergyPlus Completed Successfully-- 0 Warning; 0 Severe Errors;[\s\S]*\$oracleEndText[\s\S]*Paired fixed-date EPW daylight-saving RunPeriod use-policy exact gate passed\.' -Description "fixed-date DST policy pair exact clean completion gate"
Assert-Contains -Path $dstEpwIdfPrecedenceGate -Pattern '\$ExpectedHeader = "HOLIDAYS/DAYLIGHT SAVINGS,Yes,2/29,3/1,0"[\s\S]*\$ExpectedDataPeriod = "DATA PERIODS,1,1,Data,Sunday,2/28,3/1"[\s\S]*\$ExpectedDailyDst = @\(1, 1, 0\)[\s\S]*\$weatherLines\.Count -ne 81[\s\S]*\$weatherNonblankLines\.Count -ne 80[\s\S]*\$weatherRows\.Count -ne 72' -Description "fixed-date EPW-versus-IDF DST precedence exact source shape and daily state"
Assert-Contains -Path $dstEpwIdfPrecedenceGate -Pattern 'for \(\$rowIndex = 0; \$rowIndex -lt 72; \+\+\$rowIndex\)[\s\S]*\$fields\.Count -ne 35[\s\S]*\$fields\[1\] -ne \$ExpectedMonths\[\$dayIndex\][\s\S]*\$fields\[2\] -ne \$ExpectedDates\[\$dayIndex\][\s\S]*\$fields\[3\] -ne \$expectedHour' -Description "fixed-date EPW-versus-IDF DST precedence full 72-row source order"
Assert-Contains -Path $dstEpwIdfPrecedenceGate -Pattern 'comparison_class = "conformance"[\s\S]*timestamp_contract = "ordered-exact-unique"[\s\S]*blocking = true[\s\S]*the only IDF addition is one RunPeriodControl:DaylightSavingTime object declaring 2/28 through 2/29[\s\S]*The input-file object takes precedence independently of that flag[\s\S]*script = "scripts/dev\.cmd compare-calendar-dst-epw-idf-precedence-exact"' -Description "fixed-date EPW-versus-IDF DST precedence manifest and bounded claim contracts"
Assert-Contains -Path $dstEpwIdfPrecedenceGate -Pattern '\$runPeriodObjects\.Count -ne 1 -or \$daylightSavingObjects\.Count -ne 1 -or \$outputObjects\.Count -ne 1[\s\S]*RunPeriodControl:SpecialDays[\s\S]*\$expectedRunPeriodFields = @\([\s\S]*"No", "No", "No", "No", "No", "No"[\s\S]*\$daylightSavingFields -join ''\|''\) -cne "2/28\|2/29"' -Description "fixed-date EPW-versus-IDF DST precedence isolated exact IDF objects"
Assert-Contains -Path $dstEpwIdfPrecedenceGate -Pattern '\$baseRawText\.Replace\(\$outputAnchor, \$insertedObject \+ \$outputAnchor\)[\s\S]*ToBase64String\(\$expectedPrecedenceBytes\) -cne \[Convert\]::ToBase64String\(\$observedPrecedenceBytes\)' -Description "fixed-date EPW-versus-IDF DST precedence byte-exact single-object insertion"
Assert-Contains -Path $dstEpwIdfPrecedenceGate -Pattern '\$summary\.case_id -cne \$CaseId[\s\S]*\$summary\.time_axis_samples -ne 72[\s\S]*\$summary\.series_count -ne 1[\s\S]*\$summary\.gate\.script -cne "scripts/dev\.cmd compare-calendar-dst-epw-idf-precedence-exact"[\s\S]*weather_file_period_declared -ne \$true[\s\S]*run_period_uses_weather_file_period -ne \$false[\s\S]*input_file_period_declared -ne \$true[\s\S]*effective_source -cne "input-file"' -Description "fixed-date EPW-versus-IDF DST precedence exact summary identity and selected source"
Assert-Contains -Path $dstEpwIdfPrecedenceGate -Pattern 'start_day_of_year -ne 59[\s\S]*end_day_of_year -ne 60[\s\S]*wraps_year -ne \$false[\s\S]*daylight_saving_hourly_samples -ne 48' -Description "fixed-date EPW-versus-IDF DST precedence resolved period and active hours"
Assert-Contains -Path $dstEpwIdfPrecedenceGate -Pattern '\$series\.level -cne "conformance"[\s\S]*timestamp_contract -cne "ordered-exact-unique"[\s\S]*expected_first_timestamp -cne \$ExpectedFirstTimestamp[\s\S]*max_abs_delta -ne 0\.0[\s\S]*\$null -ne \$series\.first_timestamp_divergence' -Description "fixed-date EPW-versus-IDF DST precedence ordered exact unique zero-delta series"
Assert-Contains -Path $dstEpwIdfPrecedenceGate -Pattern '\^\\d\+,1,Environment,Site Daylight Saving Time Status \\\[\\\] !Hourly\$[\s\S]*\$values\.Count -ne 72 -or \$timestampRows\.Count -ne 72[\s\S]*\$expectedDstValue = \[int\]\$ExpectedDailyDst\[\$dayOffset\][\s\S]*Groups\[4\]\.Value -ne \$expectedDstValue[\s\S]*Groups\[8\]\.Value\.Trim\(\) -cne \$ExpectedDayTypes' -Description "fixed-date EPW-versus-IDF DST precedence raw EnergyPlus ESO values and timestamps"
Assert-Contains -Path $dstEpwIdfPrecedenceGate -Pattern 'ExpectedEnvironmentEio = "Environment,DST FIXED DATE RUN PERIOD,WeatherFileRunPeriod,02/28/2016,03/01/2016,Sunday,3,Use RunPeriod Specified Day,No,No,No,No,No,Clark and Allen"[\s\S]*ExpectedDaylightSavingEio = "Environment:Daylight Saving,Yes,InputFile,02/28,02/29"[\s\S]*\$environmentEioRows\.Count -ne 1[\s\S]*Count -ne 14[\s\S]*\$daylightSavingEioRows\.Count -ne 1[\s\S]*Count -ne 5' -Description "fixed-date EPW-versus-IDF DST precedence exact EnergyPlus 26.1 EIO rows"
Assert-Contains -Path $dstEpwIdfPrecedenceGate -Pattern 'input_file_daylight_saving_period_declared: true[\s\S]*daylight_saving_effective_source: input-file[\s\S]*daylight_saving_hourly_samples: 48' -Description "fixed-date EPW-versus-IDF DST precedence Markdown diagnostics"
Assert-Contains -Path $dstEpwIdfPrecedenceGate -Pattern 'EnergyPlus Completed Successfully-- 0 Warning; 0 Severe Errors;[\s\S]*\$oracleEndText[\s\S]*Fixed-date EPW-versus-IDF daylight-saving precedence exact gate passed\.' -Description "fixed-date EPW-versus-IDF DST precedence clean completion gate"
Assert-Contains -Path $calendarObjects -Pattern 'pub struct RunPeriodDaylightSavingTime[\s\S]*pub start_date: CalendarDateRule[\s\S]*pub end_date: CalendarDateRule' -Description "typed input-file daylight-saving object"
Assert-Contains -Path $compiler -Pattern 'fn parse_run_period_daylight_saving_time\s*\([\s\S]*single_object\(OBJECT_TYPE\)[\s\S]*RunPeriodDaylightSavingTime' -Description "typed unique input-file daylight-saving compiler"
Assert-Contains -Path $daylightSaving -Pattern 'pub enum DaylightSavingPeriodSource[\s\S]*InputFile[\s\S]*input_file_period_declared[\s\S]*if let Some\(period\) = input_file_period[\s\S]*DaylightSavingPeriodSource::InputFile' -Description "input-file daylight-saving precedence resolver"
Assert-Contains -Path $timeWeatherSchedule -Pattern 'input_file_daylight_saving_period_declared:[\s\S]*daylight_saving_effective_source:[\s\S]*input_file_period_declared[\s\S]*effective_source' -Description "input-file daylight-saving Markdown and JSON diagnostics"
Assert-Contains -Path $algorithmLedger -Pattern 'source_routine = "GetDSTData"[\s\S]*completion_status = "source_mapped"[\s\S]*source_routine = "SetDSTDateRanges"[\s\S]*completion_status = "source_mapped"' -Description "daylight-saving intake and projection source-map routines"
Assert-Contains -Path $crossYearSpecialDayGate -Pattern '\$CaseId = "calendar_special_day_cross_year_start_year_projection_hourly_exact_001"[\s\S]*timestamp_contract = "ordered-exact-unique"[\s\S]*Tuesday=3, Wednesday=4, Thursday=5, Holiday=8[\s\S]*script = "scripts/dev\.cmd compare-calendar-special-day-cross-year-start-year-projection-exact"' -Description "cross-year start-year special-day exact manifest contract"
Assert-Contains -Path $crossYearSpecialDayGate -Pattern '\$sourceFileRef -cne \$IdfRef -or \$manifestIdfRef -cne \$IdfRef[\s\S]*\$manifestWeatherRef -cne \$WeatherRef[\s\S]*Assert-SamePath[^\r\n]*\$manifestIdfRef[\s\S]*Assert-SamePath[^\r\n]*\$manifestWeatherRef[\s\S]*\$expandedIdfRef[\s\S]*\$expandedWeatherRef[\s\S]*Assert-SamePath[^\r\n]*\$expandedIdfRef[\s\S]*Assert-SamePath[^\r\n]*\$expandedWeatherRef[\s\S]*\$expectedStagedIdf = \$idfText[\s\S]*\$stagedIdfText -cne \$expectedStagedIdf' -Description "cross-year special-day canonical manifest, expanded input, and staged-IDF provenance"
Assert-Contains -Path $crossYearSpecialDayGate -Pattern '\$ExpectedDataPeriod = "DATA PERIODS,1,1,Data,Tuesday,12/30,1/2"|DATA PERIODS,1,1,Data,Tuesday,12/30,1/2' -Description "cross-year start-year single wrapping data period"
Assert-Contains -Path $crossYearSpecialDayGate -Pattern '\$runPeriodObjects\.Count -ne 1 -or \$specialDayObjects\.Count -ne 1 -or \$outputObjects\.Count -ne 1[\s\S]*"12", "30", "2031", "1", "2", "2032", "Tuesday"[\s\S]*"No", "No", "No", "No", "No", "No"[\s\S]*"Cross Year New Year Holiday", "1st Thursday in January", "1", "Holiday"[\s\S]*RunPeriodControl:DaylightSavingTime' -Description "cross-year start-year isolated RunPeriod and one special-day object"
Assert-Contains -Path $crossYearSpecialDayGate -Pattern '\$weatherLines\.Count -ne 104[\s\S]*\$weatherRows\.Count -ne 96[\s\S]*\$expectedYears = @\(2031, 2031, 2032, 2032\)[\s\S]*\$expectedMonths = @\(12, 12, 1, 1\)[\s\S]*\$expectedDays = @\(30, 31, 1, 2\)[\s\S]*for \(\$index = 0; \$index -lt 96; \+\+\$index\)' -Description "cross-year start-year exact 96-row source order"
Assert-Contains -Path $crossYearSpecialDayGate -Pattern '\$summary\.time_axis_samples -ne 96[\s\S]*\$calendar\.start_year -ne 2031 -or \$calendar\.end_year -ne 2032[\s\S]*gregorian_calendar_days -ne 4[\s\S]*weather_effective_calendar_days -ne 4[\s\S]*\$selection\.data_period_index -ne 1[\s\S]*selected_hourly_records -ne 96[\s\S]*day_buffer_transitions -ne 4' -Description "cross-year start-year calendar and weather-selection summary"
Assert-Contains -Path $crossYearSpecialDayGate -Pattern '\$specialDays\.input_file_declared -ne 1[\s\S]*resolved_count -ne 1[\s\S]*hourly_samples -ne 24[\s\S]*start_month -ne 1[\s\S]*start_day -ne 2[\s\S]*start_day_of_year -ne 2[\s\S]*day_type -ne "Holiday"[\s\S]*day_type_index -ne 8[\s\S]*weekend_shift_days -ne 0' -Description "cross-year special day retains the start-year January 2 projection"
Assert-Contains -Path $crossYearSpecialDayGate -Pattern '\$resolved\[0\]\.source -cne "input-file"[\s\S]*special_day_resolved: CROSS YEAR NEW YEAR HOLIDAY 1/2 duration=1 day_type=Holiday weekend_shift_days=0 source=input-file' -Description "cross-year special-day input-file source diagnostics"
Assert-Contains -Path $crossYearSpecialDayGate -Pattern '\$series\.expected_samples -ne 96[\s\S]*timestamp_contract -ne "ordered-exact-unique"[\s\S]*\$expectedFirst = "env=CROSS YEAR SPECIAL DAY RUN PERIOD;day=1;month=12;date=30[\s\S]*\$expectedLast = "env=CROSS YEAR SPECIAL DAY RUN PERIOD;day=4;month=1;date=2[\s\S]*max_abs_delta -ne 0\.0[\s\S]*first_timestamp_divergence' -Description "cross-year special-day ordered exact unique zero-delta series"
Assert-Contains -Path $crossYearSpecialDayGate -Pattern 'Site Day Type Index \\\[\\\] !Hourly\$[\s\S]*\$values\.Count -ne 96 -or \$timestampRows\.Count -ne 96[\s\S]*\$expectedDailyValues = @\(3\.0, 4\.0, 5\.0, 8\.0\)[\s\S]*\$expectedDailyTypes = @\("Tuesday", "Wednesday", "Thursday", "Holiday"\)[\s\S]*for \(\$index = 0; \$index -lt 96; \+\+\$index\)' -Description "cross-year special-day raw EnergyPlus values and timestamp day types"
Assert-Contains -Path $crossYearSpecialDayGate -Pattern 'Environment,CROSS YEAR SPECIAL DAY RUN PERIOD,WeatherFileRunPeriod,12/30/2031,01/02/2032,Tuesday,4,Use RunPeriod Specified Day,No,No,No,No,No,Clark and Allen[\s\S]*Environment:Special Days,CROSS YEAR NEW YEAR HOLIDAY,Holiday,InputFile,01/02,  1[\s\S]*EnergyPlus Completed Successfully-- 0 Warning; 0 Severe Errors;[\s\S]*special_day_hourly_samples: 24[\s\S]*Cross-year start-year special-day projection exact gate passed\.' -Description "cross-year start-year exact EIO, clean completion, and blocking success"
Assert-Contains -Path $timeAxisWeatherCalendar -Pattern 'run_period\.treat_weather_as_actual[\s\S]*ActualWeatherUnsupported[\s\S]*gregorian\.start_year != gregorian\.end_year[\s\S]*checked_sub\(leap_days_skipped\)' -Description "non-actual cross-year weather-calendar branch with actual-weather rejection"
Assert-Contains -Path $weatherEnvironment -Pattern 'for day_index in 0\.\.weather_calendar\.total_days[\s\S]*first_record\.month != expected\.month[\s\S]*advance_source_day' -Description "weather environment traverses cross-year source days against the resolved axis"
Assert-Contains -Path $algorithmLedger -Pattern 'calendar_special_day_cross_year_start_year_projection_hourly_exact_001[\s\S]*resolved against the 2031 environment-start annual table[\s\S]*96 ordered-exact-unique zero-tolerance Site Day Type Index samples' -Description "cross-year start-year special-day bounded algorithm evidence"
Assert-Contains -Path $epwWeekdayDstGate -Pattern '\$ExpectedHeader = "HOLIDAYS/DAYLIGHT SAVINGS,Yes,4th Monday in February,Last Wednesday in February,0"[\s\S]*\$ExpectedDataPeriod = "DATA PERIODS,1,1,Data,Sunday,2/22,2/26"[\s\S]*\$weatherLines\.Count -ne 128[\s\S]*\$weatherRows\.Count -ne 120' -Description "EPW weekday DST exact eight-header, 120-row source shape"
Assert-Contains -Path $epwWeekdayDstGate -Pattern '\$ExpectedDataPeriod = "DATA PERIODS,1,1,Data,Sunday,2/22,2/26"[\s\S]*\$dataPeriodHeaders\.Count -ne 1 -or \$dataPeriodHeaders\[0\] -cne \$ExpectedDataPeriod' -Description "EPW weekday DST unique case-sensitive data-period header"
Assert-Contains -Path $epwWeekdayDstGate -Pattern 'for \(\$rowIndex = 0; \$rowIndex -lt 120; \+\+\$rowIndex\)[\s\S]*\$dayIndex = \[int\]\[math\]::Floor\(\$rowIndex / 24\)[\s\S]*\$expectedHour = \(\$rowIndex % 24\) \+ 1[\s\S]*\$fields\[2\] -ne \$ExpectedDates\[\$dayIndex\][\s\S]*\$fields\[3\] -ne \$expectedHour' -Description "EPW weekday DST full 120-row source order"
Assert-Contains -Path $epwWeekdayDstGate -Pattern '\$expectedRunPeriodFields = @\([\s\S]*"2", "22", "2032", "2", "26", "2032", "Sunday"[\s\S]*"No", "Yes", "No", "No", "No", "No"[\s\S]*RunPeriodControl:SpecialDays[\s\S]*RunPeriodControl:DaylightSavingTime' -Description "EPW weekday DST exact isolated RunPeriod policies"
Assert-Contains -Path $epwWeekdayDstGate -Pattern '\$resolvedPeriod\.start_month -ne 2[\s\S]*start_day -ne 23[\s\S]*start_day_of_year -ne 54[\s\S]*end_day -ne 25[\s\S]*end_day_of_year -ne 56[\s\S]*wraps_year -ne \$false' -Description "EPW weekday DST exact resolved nonwrapping boundaries"
Assert-Contains -Path $epwWeekdayDstGate -Pattern '\$summary\.time_axis_samples -ne 120[\s\S]*weather_file_period_declared -ne \$true[\s\S]*run_period_uses_weather_file_period -ne \$true[\s\S]*\$calendar\.daylight_saving_hourly_samples -ne 72' -Description "EPW weekday DST exact active policy and sample counts"
Assert-Contains -Path $epwWeekdayDstGate -Pattern '\$summary\.case_id -cne \$CaseId[\s\S]*\$summary\.comparison_class -cne "conformance"[\s\S]*\$summary\.series_count -ne 1[\s\S]*\$summary\.conformance_series_count -ne 1[\s\S]*\$summary\.gate\.script -cne "scripts/dev\.cmd compare-calendar-epw-dst-weekday-rules-exact"[\s\S]*\$summary\.gate\.blocking -ne \$true' -Description "EPW weekday DST exact summary identity and gate metadata"
Assert-Contains -Path $epwWeekdayDstGate -Pattern '\$ExpectedDates = @\(22, 23, 24, 25, 26\)[\s\S]*\$ExpectedDst = @\(0, 1, 1, 1, 0\)[\s\S]*\$ExpectedDayTypes = @\("Sunday", "Monday", "Tuesday", "Wednesday", "Thursday"\)' -Description "EPW weekday DST five-day exact daily state"
Assert-Contains -Path $epwWeekdayDstGate -Pattern '\$series\.level -cne "conformance"[\s\S]*\$series\.class -cne "weather"[\s\S]*\$series\.frequency -cne "hourly"[\s\S]*\$series\.source -cne "eso"[\s\S]*\$series\.alignment -cne "timestamp"[\s\S]*expected_samples -ne 120[\s\S]*timestamp_contract -ne "ordered-exact-unique"[\s\S]*max_abs_delta -ne 0\.0' -Description "EPW weekday DST promoted exact series metadata"
Assert-Contains -Path $epwWeekdayDstGate -Pattern '\$null -ne \$series\.first_divergence -or \$null -ne \$series\.first_timestamp_divergence' -Description "EPW weekday DST unique series has no first divergence"
Assert-Contains -Path $epwWeekdayDstGate -Pattern '\$values\.Count -ne 120 -or \$timestampRows\.Count -ne 120[\s\S]*\$values\[\$index\] -ne \[double\]\$ExpectedDst\[\$dayOffset\][\s\S]*\$timestampMatch\.Groups\[1\]\.Value -ne \(\$dayOffset \+ 1\)[\s\S]*Groups\[8\]\.Value\.Trim\(\) -cne \$ExpectedDayTypes' -Description "EPW weekday DST raw EnergyPlus ESO values and timestamp fields"
Assert-Contains -Path $epwWeekdayDstGate -Pattern '\^\\d\+,1,Environment,Site Daylight Saving Time Status \\\[\\\] !Hourly\$' -Description "EPW weekday DST exact ESO dictionary type and frequency"
Assert-Contains -Path $epwWeekdayDstGate -Pattern '\$daylightSavingEioRows\.Count -ne 1[\s\S]*Environment:Daylight Saving,Yes,WeatherFile,02/23,02/25' -Description "EPW weekday DST exact EIO source and dates"
Assert-Contains -Path $epwWeekdayDstGate -Pattern 'EnergyPlus Completed Successfully-- 0 Warning; 0 Severe Errors;[\s\S]*\$oracleEndText[\s\S]*Exact EPW fourth-Monday through last-Wednesday daylight-saving gate passed\.' -Description "EPW weekday DST exact clean completion gate"
Assert-Contains -Path $epwSouthernDstGate -Pattern '\$ExpectedHeader = "HOLIDAYS/DAYLIGHT SAVINGS,Yes,Last Sunday in October,Last Sunday in March,0"[\s\S]*\$ExpectedDataPeriod = "DATA PERIODS,1,1,Data,Saturday,3/27,3/29"[\s\S]*\$weatherLines\.Count -ne 80[\s\S]*\$weatherRows\.Count -ne 72' -Description "EPW southern-wrap DST exact eight-header, 72-row source shape"
Assert-Contains -Path $epwSouthernDstGate -Pattern '\$ExpectedDataPeriod = "DATA PERIODS,1,1,Data,Saturday,3/27,3/29"[\s\S]*\$dataPeriodHeaders\.Count -ne 1 -or \$dataPeriodHeaders\[0\] -cne \$ExpectedDataPeriod' -Description "EPW southern-wrap DST unique case-sensitive data-period header"
Assert-Contains -Path $epwSouthernDstGate -Pattern 'for \(\$rowIndex = 0; \$rowIndex -lt 72; \+\+\$rowIndex\)[\s\S]*\$dayIndex = \[int\]\[math\]::Floor\(\$rowIndex / 24\)[\s\S]*\$expectedHour = \(\$rowIndex % 24\) \+ 1[\s\S]*\$fields\[2\] -ne \$ExpectedDates\[\$dayIndex\][\s\S]*\$fields\[3\] -ne \$expectedHour' -Description "EPW southern-wrap DST full 72-row source order"
Assert-Contains -Path $epwSouthernDstGate -Pattern '\$expectedRunPeriodFields = @\([\s\S]*"3", "27", "2032", "3", "29", "2032", "Saturday"[\s\S]*"No", "Yes", "No", "No", "No", "No"[\s\S]*RunPeriodControl:SpecialDays[\s\S]*RunPeriodControl:DaylightSavingTime' -Description "EPW southern-wrap DST exact isolated RunPeriod policies"
Assert-Contains -Path $epwSouthernDstGate -Pattern '\$resolvedPeriod\.start_month -ne 10[\s\S]*start_day -ne 31[\s\S]*start_day_of_year -ne 305[\s\S]*end_day -ne 28[\s\S]*end_day_of_year -ne 88[\s\S]*wraps_year -ne \$true' -Description "EPW southern-wrap DST exact resolved wrapping boundaries"
Assert-Contains -Path $epwSouthernDstGate -Pattern '\$summary\.status -cne "pass" -or \$summary\.time_axis_samples -ne 72[\s\S]*weather_file_period_declared -ne \$true[\s\S]*run_period_uses_weather_file_period -ne \$true[\s\S]*\$calendar\.daylight_saving_hourly_samples -ne 48' -Description "EPW southern-wrap DST exact active policy and sample counts"
Assert-Contains -Path $epwSouthernDstGate -Pattern '\$summary\.case_id -cne \$CaseId[\s\S]*\$summary\.comparison_class -cne "conformance"[\s\S]*\$summary\.series_count -ne 1[\s\S]*\$summary\.conformance_series_count -ne 1[\s\S]*\$summary\.gate\.script -cne "scripts/dev\.cmd compare-calendar-epw-dst-southern-wrap-exact"[\s\S]*\$summary\.gate\.blocking -ne \$true' -Description "EPW southern-wrap DST exact summary identity and gate metadata"
Assert-Contains -Path $epwSouthernDstGate -Pattern '\$ExpectedDates = @\(27, 28, 29\)[\s\S]*\$ExpectedDst = @\(1, 1, 0\)[\s\S]*\$ExpectedDayTypes = @\("Saturday", "Sunday", "Monday"\)' -Description "EPW southern-wrap DST three-day exact daily state"
Assert-Contains -Path $epwSouthernDstGate -Pattern '\$series\.level -cne "conformance"[\s\S]*\$series\.class -cne "weather"[\s\S]*\$series\.frequency -cne "hourly"[\s\S]*\$series\.source -cne "eso"[\s\S]*\$series\.alignment -cne "timestamp"[\s\S]*expected_samples -ne 72[\s\S]*timestamp_contract -ne "ordered-exact-unique"[\s\S]*max_abs_delta -ne 0\.0' -Description "EPW southern-wrap DST promoted exact series metadata"
Assert-Contains -Path $epwSouthernDstGate -Pattern '\$null -ne \$series\.first_divergence -or \$null -ne \$series\.first_timestamp_divergence' -Description "EPW southern-wrap DST unique series has no first divergence"
Assert-Contains -Path $epwSouthernDstGate -Pattern '\$values\.Count -ne 72 -or \$timestampRows\.Count -ne 72[\s\S]*\$values\[\$index\] -ne \[double\]\$ExpectedDst\[\$dayOffset\][\s\S]*\$timestampMatch\.Groups\[1\]\.Value -ne \(\$dayOffset \+ 1\)[\s\S]*Groups\[8\]\.Value\.Trim\(\) -cne \$ExpectedDayTypes' -Description "EPW southern-wrap DST raw EnergyPlus ESO values and timestamp fields"
Assert-Contains -Path $epwSouthernDstGate -Pattern '\^\\d\+,1,Environment,Site Daylight Saving Time Status \\\[\\\] !Hourly\$' -Description "EPW southern-wrap DST exact ESO dictionary type and frequency"
Assert-Contains -Path $epwSouthernDstGate -Pattern '\$daylightSavingEioRows\.Count -ne 1[\s\S]*Environment:Daylight Saving,Yes,WeatherFile,10/31,03/28' -Description "EPW southern-wrap DST exact EIO source and dates"
Assert-Contains -Path $epwSouthernDstGate -Pattern 'EnergyPlus Completed Successfully-- 0 Warning; 0 Severe Errors;[\s\S]*\$oracleEndText[\s\S]*Exact EPW last-Sunday October through last-Sunday March daylight-saving gate passed\.' -Description "EPW southern-wrap DST exact clean completion gate"
Assert-Contains -Path $epwSouthernDstStartGate -Pattern '\$ExpectedHeader = "HOLIDAYS/DAYLIGHT SAVINGS,Yes,Last Sunday in October,Last Sunday in March,0"[\s\S]*\$ExpectedDataPeriod = "DATA PERIODS,1,1,Data,Saturday,10/30,11/1"[\s\S]*\$weatherLines\.Count -ne 80[\s\S]*\$weatherRows\.Count -ne 72' -Description "EPW southern-wrap start DST exact eight-header, 72-row source shape"
Assert-Contains -Path $epwSouthernDstStartGate -Pattern '\$ExpectedDataPeriod = "DATA PERIODS,1,1,Data,Saturday,10/30,11/1"[\s\S]*\$dataPeriodHeaders\.Count -ne 1 -or \$dataPeriodHeaders\[0\] -cne \$ExpectedDataPeriod' -Description "EPW southern-wrap start DST unique case-sensitive data-period header"
Assert-Contains -Path $epwSouthernDstStartGate -Pattern 'for \(\$rowIndex = 0; \$rowIndex -lt 72; \+\+\$rowIndex\)[\s\S]*\$dayIndex = \[int\]\[math\]::Floor\(\$rowIndex / 24\)[\s\S]*\$expectedHour = \(\$rowIndex % 24\) \+ 1[\s\S]*\$fields\[1\] -ne \$ExpectedMonths\[\$dayIndex\][\s\S]*\$fields\[2\] -ne \$ExpectedDates\[\$dayIndex\][\s\S]*\$fields\[3\] -ne \$expectedHour' -Description "EPW southern-wrap start DST full 72-row source order"
Assert-Contains -Path $epwSouthernDstStartGate -Pattern '\$expectedRunPeriodFields = @\([\s\S]*"10", "30", "2032", "11", "1", "2032", "Saturday"[\s\S]*"No", "Yes", "No", "No", "No", "No"[\s\S]*RunPeriodControl:SpecialDays[\s\S]*RunPeriodControl:DaylightSavingTime' -Description "EPW southern-wrap start DST exact isolated RunPeriod policies"
Assert-Contains -Path $epwSouthernDstStartGate -Pattern '\$resolvedPeriod\.start_month -ne 10[\s\S]*start_day -ne 31[\s\S]*start_day_of_year -ne 305[\s\S]*end_day -ne 28[\s\S]*end_day_of_year -ne 88[\s\S]*wraps_year -ne \$true' -Description "EPW southern-wrap start DST exact source-mapped boundaries"
Assert-Contains -Path $epwSouthernDstStartGate -Pattern '\$summary\.status -cne "pass" -or \$summary\.time_axis_samples -ne 72[\s\S]*weather_file_period_declared -ne \$true[\s\S]*run_period_uses_weather_file_period -ne \$true[\s\S]*\$calendar\.daylight_saving_hourly_samples -ne 48' -Description "EPW southern-wrap start DST exact active policy and sample counts"
Assert-Contains -Path $epwSouthernDstStartGate -Pattern '\$summary\.case_id -cne \$CaseId[\s\S]*\$summary\.comparison_class -cne "conformance"[\s\S]*\$summary\.series_count -ne 1[\s\S]*\$summary\.conformance_series_count -ne 1[\s\S]*\$summary\.gate\.script -cne "scripts/dev\.cmd compare-calendar-epw-dst-southern-wrap-start-exact"[\s\S]*\$summary\.gate\.blocking -ne \$true' -Description "EPW southern-wrap start DST exact summary identity and gate metadata"
Assert-Contains -Path $epwSouthernDstStartGate -Pattern '\$ExpectedMonths = @\(10, 10, 11\)[\s\S]*\$ExpectedDates = @\(30, 31, 1\)[\s\S]*\$ExpectedDst = @\(0, 1, 1\)[\s\S]*\$ExpectedDayTypes = @\("Saturday", "Sunday", "Monday"\)' -Description "EPW southern-wrap start DST three-day exact daily state"
Assert-Contains -Path $epwSouthernDstStartGate -Pattern '\$series\.level -cne "conformance"[\s\S]*\$series\.class -cne "weather"[\s\S]*\$series\.frequency -cne "hourly"[\s\S]*\$series\.source -cne "eso"[\s\S]*\$series\.alignment -cne "timestamp"[\s\S]*expected_samples -ne 72[\s\S]*timestamp_contract -ne "ordered-exact-unique"[\s\S]*max_abs_delta -ne 0\.0' -Description "EPW southern-wrap start DST promoted exact series metadata"
Assert-Contains -Path $epwSouthernDstStartGate -Pattern '\$null -ne \$series\.first_divergence -or \$null -ne \$series\.first_timestamp_divergence' -Description "EPW southern-wrap start DST unique series has no first divergence"
Assert-Contains -Path $epwSouthernDstStartGate -Pattern '\$values\.Count -ne 72 -or \$timestampRows\.Count -ne 72[\s\S]*\$values\[\$index\] -ne \[double\]\$ExpectedDst\[\$dayOffset\][\s\S]*Groups\[2\]\.Value -ne \$ExpectedMonths\[\$dayOffset\][\s\S]*Groups\[8\]\.Value\.Trim\(\) -cne \$ExpectedDayTypes' -Description "EPW southern-wrap start DST raw EnergyPlus ESO values and timestamp fields"
Assert-Contains -Path $epwSouthernDstStartGate -Pattern '\^\\d\+,1,Environment,Site Daylight Saving Time Status \\\[\\\] !Hourly\$' -Description "EPW southern-wrap start DST exact ESO dictionary type and frequency"
Assert-Contains -Path $epwSouthernDstStartGate -Pattern '\$daylightSavingEioRows\.Count -ne 1[\s\S]*Environment:Daylight Saving,Yes,WeatherFile,10/31,03/28' -Description "EPW southern-wrap start DST exact EIO source and dates"
Assert-Contains -Path $epwSouthernDstStartGate -Pattern 'EnergyPlus Completed Successfully-- 0 Warning; 0 Severe Errors;[\s\S]*\$oracleEndText[\s\S]*Exact EPW last-Sunday October through last-Sunday March start-side daylight-saving gate passed\.' -Description "EPW southern-wrap start DST exact clean completion gate"
Assert-Contains -Path $crossYearDstGate -Pattern '\$CaseId = "calendar_epw_dst_cross_year_start_year_projection_hourly_exact_001"[\s\S]*daily order 0, 0, 0, 1[\s\S]*script = "scripts/dev\.cmd compare-calendar-epw-dst-cross-year-start-year-projection-exact"' -Description "cross-year start-year DST exact manifest contract"
Assert-Contains -Path $crossYearDstGate -Pattern '\$sourceFileRef -cne \$IdfRef -or \$manifestIdfRef -cne \$IdfRef[\s\S]*\$manifestWeatherRef -cne \$WeatherRef[\s\S]*Assert-SamePath[^\r\n]*\$expandedIdfRef[\s\S]*Assert-SamePath[^\r\n]*\$expandedWeatherRef[\s\S]*\$expectedStagedIdf = \$idfText[\s\S]*\$stagedIdfText -cne \$expectedStagedIdf' -Description "cross-year start-year DST canonical manifest, expanded input, and staged-IDF provenance"
Assert-Contains -Path $crossYearDstGate -Pattern '\$runPeriodObjects\.Count -ne 1 -or \$specialDayObjects\.Count -ne 0[\s\S]*\$inputDstObjects\.Count -ne 0 -or \$outputObjects\.Count -ne 1[\s\S]*"12", "30", "2031", "1", "2", "2032", "Tuesday"[\s\S]*"No", "Yes", "No", "No", "No", "No"' -Description "cross-year start-year DST isolated RunPeriod and calendar-control exclusions"
Assert-Contains -Path $crossYearDstGate -Pattern 'HOLIDAYS/DAYLIGHT SAVINGS,Yes,1st Thursday in January,1st Friday in January,0[\s\S]*DATA PERIODS,1,1,Data,Tuesday,12/30,1/2[\s\S]*\$weatherLines\.Count -ne 104[\s\S]*\$weatherRows\.Count -ne 96[\s\S]*\$expectedYears = @\(2031, 2031, 2032, 2032\)[\s\S]*for \(\$index = 0; \$index -lt 96; \+\+\$index\)' -Description "cross-year start-year DST exact header and 96-row source order"
Assert-Contains -Path $crossYearDstGate -Pattern '\$summary\.time_axis_samples -ne 96[\s\S]*\$calendar\.start_year -ne 2031 -or \$calendar\.end_year -ne 2032[\s\S]*gregorian_calendar_days -ne 4[\s\S]*weather_effective_calendar_days -ne 4[\s\S]*selected_hourly_records -ne 96[\s\S]*day_buffer_transitions -ne 4' -Description "cross-year start-year DST calendar and weather-selection summary"
Assert-Contains -Path $crossYearDstGate -Pattern 'weather_file_period_declared -ne \$true[\s\S]*run_period_uses_weather_file_period -ne \$true[\s\S]*input_file_period_declared -ne \$false[\s\S]*effective_source -cne "weather-file"[\s\S]*daylight_saving_hourly_samples -ne 24[\s\S]*start_day_of_year -ne 2[\s\S]*end_day_of_year -ne 3[\s\S]*wraps_year -ne \$false' -Description "cross-year start-year DST 2031 annual-table resolution and active hours"
Assert-Contains -Path $crossYearDstGate -Pattern '\$specialDays\.weather_file_declared -ne 0[\s\S]*\$specialDays\.input_file_declared -ne 0[\s\S]*resolved_count -ne 0[\s\S]*hourly_samples -ne 0' -Description "cross-year start-year DST special-day isolation"
Assert-Contains -Path $crossYearDstGate -Pattern '\$series\.expected_samples -ne 96[\s\S]*timestamp_contract -ne "ordered-exact-unique"[\s\S]*\$expectedFirst = "env=CROSS YEAR DST START YEAR RUN PERIOD;day=1;month=12;date=30[\s\S]*\$expectedLast = "env=CROSS YEAR DST START YEAR RUN PERIOD;day=4;month=1;date=2;dst=1[\s\S]*max_abs_delta -ne 0\.0[\s\S]*first_timestamp_divergence' -Description "cross-year start-year DST ordered exact unique zero-delta series"
Assert-Contains -Path $crossYearDstGate -Pattern 'Site Daylight Saving Time Status \\\[\\\] !Hourly\$[\s\S]*\$values\.Count -ne 96 -or \$timestampRows\.Count -ne 96[\s\S]*\$expectedDailyValues = @\(0\.0, 0\.0, 0\.0, 1\.0\)[\s\S]*\$expectedDailyTypes = @\("Tuesday", "Wednesday", "Thursday", "Friday"\)' -Description "cross-year start-year DST raw EnergyPlus values and timestamp fields"
Assert-Contains -Path $crossYearDstGate -Pattern 'Environment,CROSS YEAR DST START YEAR RUN PERIOD,WeatherFileRunPeriod,12/30/2031,01/02/2032,Tuesday,4,Use RunPeriod Specified Day,Yes,No,No,No,No,Clark and Allen[\s\S]*Environment:Daylight Saving,Yes,WeatherFile,01/02,01/03[\s\S]*EnergyPlus Completed Successfully-- 0 Warning; 0 Severe Errors;[\s\S]*Cross-year start-year daylight-saving projection exact gate passed\.' -Description "cross-year start-year DST exact EIO, clean completion, and blocking success"
Assert-Contains -Path $algorithmLedger -Pattern 'calendar_epw_dst_cross_year_start_year_projection_hourly_exact_001[\s\S]*2031 environment-start annual table[\s\S]*The zero-tolerance gate proves 96 ordered-exact-unique Site Daylight Saving Time Status samples' -Description "cross-year start-year DST bounded algorithm evidence"
Assert-Contains -Path $scheduleObjects -Pattern 'pub enum ScheduleDayType[\s\S]*Sunday,[\s\S]*Saturday,[\s\S]*Holiday,[\s\S]*SummerDesignDay,[\s\S]*WinterDesignDay,[\s\S]*CustomDay1,[\s\S]*CustomDay2' -Description "typed twelve-way Schedule:Compact day types"
Assert-Contains -Path $scheduleObjects -Pattern 'pub struct ScheduleCompactDayProfile[\s\S]*day_types: Vec<ScheduleDayType>[\s\S]*segments: Vec<ScheduleCompactSegment>[\s\S]*pub struct ScheduleCompactPeriod[\s\S]*through_schedule_day_of_year: u16[\s\S]*day_profiles: Vec<ScheduleCompactDayProfile>' -Description "typed source-ordered compact schedule periods and profiles"
Assert-Contains -Path $compiler -Pattern 'fn compact_schedule_periods\s*\([\s\S]*fn compact_schedule_day_types\s*\([\s\S]*AllOtherDays[\s\S]*fn finish_compact_schedule_profile\s*\([\s\S]*fn finish_compact_schedule_period\s*\(' -Description "compiler Through/For/AllOtherDays source-order pipeline"
Assert-Contains -Path $compiler -Pattern 'fn parses_schedule_compact_periods_and_source_ordered_all_other_days\s*\([\s\S]*Through: 1/1[\s\S]*For: Thursday[\s\S]*For: AllOtherDays[\s\S]*Through: 12/31[\s\S]*For: Holiday' -Description "compiler exact Through/For source-order test"
Assert-Contains -Path $compiler -Pattern 'fn schedule_compact_all_other_days_is_applied_after_same_field_selectors\s*\([\s\S]*For: AllOtherDays Monday AllOtherDays[\s\S]*assert_eq!\(day_types\[0\], ScheduleDayType::Monday\)[\s\S]*for day_type in ALL_SCHEDULE_DAY_TYPES' -Description "compiler same-For explicit selector precedes two-pass AllOtherDays complement"
Assert-Contains -Path $compiler -Pattern 'fn rejects_schedule_compact_duplicate_group_and_all_other_assignments\s*\([\s\S]*fn expands_schedule_compact_weekday_weekend_and_special_day_tokens\s*\([\s\S]*fn rejects_schedule_compact_through_order_and_missing_final_date\s*\([\s\S]*fn rejects_schedule_compact_until_order_and_incomplete_profiles\s*\([\s\S]*fn rejects_schedule_compact_unknown_day_type\s*\(' -Description "compiler compact-schedule boundary diagnostics"
Assert-Contains -Path $scheduleCache -Pattern 'pub fn precompute_schedule_cache_for_time_axis\s*\([\s\S]*compact_schedule_series_for_time_axis' -Description "immutable hourly cache compiles compact schedules from the shared TimeAxis"
Assert-Contains -Path $schedules -Pattern 'fn compact_schedule_series_for_time_axis\s*\([\s\S]*detailed_schedule_lookup_state\(point\)[\s\S]*fn detailed_schedule_lookup_state\s*\([\s\S]*point\.schedule_day_of_year[\s\S]*tomorrow_special_day_type[\s\S]*unwrap_or_else\(\|\| point\.tomorrow_day_of_week\.into\(\)\)[\s\S]*fn detailed_schedule_lookup_state_from_input\s*\([\s\S]*input\.hour\.clamp\(1, 24\) \+ u32::from\(input\.dst\)' -Description "runtime compact cache values consume calendar ordinal, current DST, tomorrow special-day precedence, and tomorrow weekday"
Assert-Contains -Path $timeAxis -Pattern 'pub tomorrow_day_of_week: DayOfWeek[\s\S]*pub tomorrow_day_type: DayType[\s\S]*pub tomorrow_special_day_type: Option<DayType>[\s\S]*let tomorrow = days\.get\(day_index \+ 1\)\.unwrap_or\(day\)[\s\S]*tomorrow_day_type: tomorrow\.day_type' -Description "time axes expose next-day state and retain the final no-prefetch day"
Assert-Contains -Path $schedules -Pattern '\.find\(\|period\| schedule_day_of_year <= u32::from\(period\.through_schedule_day_of_year\)\)[\s\S]*\.find\(\|profile\| profile\.day_types\.contains\(&day_type\)\)[\s\S]*compact_interval_value' -Description "runtime Through then For then Until lookup order"
Assert-Contains -Path $runtimeTestSourceOrder -Pattern 'fn compact_schedule_time_axis_consumes_cross_year_period_day_type_and_hour\s*\([\s\S]*fn compact_schedule_time_axis_selects_until_segment_by_hour\s*\([\s\S]*fn hour_only_schedule_consumers_reject_calendar_variation_and_missing_ids\s*\(' -Description "runtime calendar-aware and hour-only compact schedule tests"
Assert-Contains -Path $runtimeError -Pattern 'InvalidInternalGainSchedule \{[\s\S]*equipment_name: String[\s\S]*schedule_id: u32[\s\S]*reason: String' -Description "typed fail-closed invalid internal-gain schedule error"
Assert-Contains -Path $schedules -Pattern 'pub\(crate\) fn validate_hour_only_internal_gain_schedules\s*\([\s\S]*hour_only_single_period_compact_schedule_segments\(schedule\)[\s\S]*schedule ID \{\} is unresolved[\s\S]*RuntimeError::InvalidInternalGainSchedule' -Description "hour-only internal gains reject calendar-varying and unresolved schedules"
Assert-Contains -Path $runtimeTestSourceOrder -Pattern 'calendar-varying internal-gain schedule must be rejected[\s\S]*initialize_heat_balance_state[\s\S]*Err\(RuntimeError::InvalidInternalGainSchedule[\s\S]*simulate_first_zone_uncontrolled[\s\S]*Err\(RuntimeError::InvalidInternalGainSchedule[\s\S]*missing convective schedule must be rejected[\s\S]*missing radiant schedule must be rejected' -Description "gain trace, heat-balance initialization, and first-zone paths fail closed"
Assert-Contains -Path $compactThroughForGate -Pattern '\$CaseId = "calendar_schedule_compact_through_for_day_type_hourly_exact_001"[\s\S]*\$sourceFileRef -cne \$IdfRef[\s\S]*Assert-SamePath[\s\S]*\$expectedStagedIdf = \$idfText' -Description "Through/For canonical manifest and staged-input provenance"
Assert-Contains -Path $compactThroughForGate -Pattern '\$expandedStagedIdf -cne "input\.idf" -or \$expandedConvertedEpjson -cne "input\.epJSON"[\s\S]*\$expectedStagedIdf = \$idfText \+ "`n" \+ \$injectionFooter \+ "`n"' -Description "Through/For exact expanded staged-IDF and converted-epJSON provenance"
Assert-Contains -Path $compactThroughForGate -Pattern '\$actualIdfObjectVectors = @\([\s\S]*\$expectedIdfObjectVectors = @\([\s\S]*"Version\|26\.1"[\s\S]*"RunPeriod\|Through For Day Type Run Period[\s\S]*"Schedule:Compact\|Through For Day Type Schedule[\s\S]*"Output:Variable\|Environment\|Site Day Type Index\|Hourly"[\s\S]*\$actualIdfObjectVectors -join' -Description "Through/For complete ten-object order and field-vector lock"
Assert-Contains -Path $compactThroughForGate -Pattern '\$expectedCompactFields = @\([\s\S]*"Through: 1/1"[\s\S]*"For: Thursday"[\s\S]*"For: AllOtherDays"[\s\S]*"Through: 12/31"[\s\S]*"For: Tuesday"[\s\S]*"For: Wednesday"[\s\S]*"For: Holiday"[\s\S]*\$weatherLines\.Count -ne 128[\s\S]*\$weatherRows\.Count -ne 120' -Description "Through/For exact IDF field vector and EPW shape"
Assert-Contains -Path $compactThroughForGate -Pattern '\$summary\.time_axis_samples -ne 120[\s\S]*\$summary\.series_count -ne 1[\s\S]*\$null -ne \$summary\.weather_record_selection[\s\S]*\$series\.expected_samples -ne 120[\s\S]*timestamp_contract -ne "ordered-exact-unique"[\s\S]*max_abs_delta -ne 0\.0' -Description "Through/For single exact series and explicit record-selection nonclaim"
Assert-Contains -Path $compactThroughForGate -Pattern '\$scheduleValues\.Count -ne 120 -or \$dayTypeValues\.Count -ne 120 -or \$timestampRows\.Count -ne 120[\s\S]*\$expectedDailyScheduleValues = @\(103\.0, 104\.0, 105\.0, 108\.0, 199\.0\)[\s\S]*\$expectedDailyTypeValues = @\(3\.0, 4\.0, 5\.0, 8\.0, 7\.0\)' -Description "Through/For raw EnergyPlus schedule, day-type, and timestamp rows"
Assert-Contains -Path $compactThroughForGate -Pattern 'Environment,THROUGH FOR DAY TYPE RUN PERIOD,WeatherFileRunPeriod,12/30/2031,01/03/2032,Tuesday,5,Use RunPeriod Specified Day,No,No,No,No,No,Clark and Allen[\s\S]*Environment:Daylight Saving,No,RunPeriod Object[\s\S]*Environment:Special Days,CROSS YEAR NEW YEAR HOLIDAY,Holiday,InputFile,01/02,  1[\s\S]*EnergyPlus Completed Successfully-- 0 Warning; 0 Severe Errors;[\s\S]*Schedule:Compact Through and For day-type exact gate passed\.' -Description "Through/For exact EIO, clean completion, and blocking success"
Assert-Contains -Path $algorithmLedger -Pattern 'calendar_schedule_compact_through_for_day_type_hourly_exact_001[\s\S]*120 ordered-exact-unique zero-tolerance Schedule Value samples[\s\S]*Rust EPW record selection is null and unclaimed' -Description "Through/For bounded algorithm evidence"
Assert-Contains -Path $algorithmLedger -Pattern '(?s)\[\[algorithm\]\]\s*id = "calendar_time_state"(?:(?!\[\[algorithm\]\]).)*status = "scaffold"(?:(?!\[\[algorithm\]\]).)*claim_level = "none"' -Description "calendar algorithm remains scaffold with no family claim"
Assert-Contains -Path $algorithmLedger -Pattern 'routine\.update_schedule_vals\.completion_status = "source_mapped"' -Description "UpdateScheduleVals remains source-mapped"
Assert-Contains -Path $runtimeTestSourceOrder -Pattern 'fn detailed_schedule_dst_shift_uses_tomorrow_type_and_final_stale_type\s*\([\s\S]*vec!\[100\.0; 23\][\s\S]*expected\.push\(124\.0\)[\s\S]*vec!\[200\.0; 23\][\s\S]*expected\.push\(801\.0\)[\s\S]*vec!\[800\.0; 23\][\s\S]*expected\.push\(901\.0\)[\s\S]*tomorrow_day_type[\s\S]*day_type' -Description "runtime detailed schedule exact DST, tomorrow Holiday, and final stale-type sequence"
Assert-Contains -Path $runtimeTestSourceOrder -Pattern 'fn detailed_schedule_dst_hour_24_wraps_schedule_ordinal_367_to_one\s*\([\s\S]*schedule_day_of_year, 366[\s\S]*expected\.push\(11\.0\)' -Description "runtime detailed schedule leap-shaped ordinal wrap unit boundary"
Assert-Contains -Path $scheduleDstRolloverGate -Pattern '\$CaseId = "calendar_schedule_dst_hour24_tomorrow_day_type_exact_001"[\s\S]*\$sourceFileRef -cne \$IdfRef[\s\S]*Assert-SamePath[\s\S]*\$expandedStagedIdf -cne "input\.idf"[\s\S]*\$expectedStagedIdf = \$idfText' -Description "DST schedule rollover canonical manifest, expanded inputs, and staged-IDF provenance"
Assert-Contains -Path $scheduleDstRolloverGate -Pattern '\$expectedIdfObjectVectors = @\([\s\S]*Schedule:Compact\|DST Final Rollover Schedule[\s\S]*HOLIDAYS/DAYLIGHT SAVINGS,Yes,Last Sunday in October,Last Sunday in March,0[\s\S]*DATA PERIODS,1,1,Data,Saturday,10/30,11/1[\s\S]*\$weatherRows\.Count -ne 72' -Description "DST schedule rollover complete IDF order and exact EPW shape"
Assert-Contains -Path $scheduleDstRolloverGate -Pattern '\$summary\.time_axis_samples -ne 72[\s\S]*\$calendar\.daylight_saving_hourly_samples -ne 48[\s\S]*\$resolvedSpecialDay\.day_type -cne "Holiday"[\s\S]*\$null -ne \$summary\.weather_record_selection[\s\S]*\$series\.expected_samples -ne 72[\s\S]*max_abs_delta -ne 0\.0' -Description "DST schedule rollover single exact series, calendar state, and record-selection nonclaim"
Assert-Contains -Path $scheduleDstRolloverGate -Pattern '\$dstValues\.Count -ne 72[\s\S]*\$expectedDailyDst = @\(0\.0, 1\.0, 1\.0\)[\s\S]*\$expectedDailyTypeValues = @\(7\.0, 1\.0, 8\.0\)[\s\S]*\$expectedScheduleCounts = @\{[\s\S]*"100" = 23[\s\S]*"124" = 1[\s\S]*"200" = 23[\s\S]*"801" = 1[\s\S]*"800" = 23[\s\S]*"901" = 1' -Description "DST schedule rollover raw schedule, DST, day-type, and timestamp vectors"
Assert-Contains -Path $scheduleDstRolloverGate -Pattern 'Environment,DST SCHEDULE FINAL ROLLOVER RUN PERIOD,WeatherFileRunPeriod,10/30/2032,11/01/2032,Saturday,3,Use RunPeriod Specified Day,Yes,No,No,No,No,Clark and Allen[\s\S]*Environment:Daylight Saving,Yes,WeatherFile,10/31,03/28[\s\S]*Environment:Special Days,FINAL ROLLOVER HOLIDAY,Holiday,InputFile,11/01,  1[\s\S]*EnergyPlus Completed Successfully-- 0 Warning; 0 Severe Errors;[\s\S]*Schedule:Compact daylight-saving hour-24 tomorrow day-type exact gate passed\.' -Description "DST schedule rollover exact EIO, clean completion, and blocking success"
Assert-Contains -Path $algorithmLedger -Pattern 'calendar_schedule_dst_hour24_tomorrow_day_type_exact_001[\s\S]*72 ordered-exact-unique zero-tolerance Schedule Value samples[\s\S]*final-run hour retaining the stale final Holiday TomorrowVariables day type' -Description "DST schedule rollover bounded algorithm evidence"
Assert-Contains -Path $algorithmLedger -Pattern 'src/EnergyPlus/ScheduleManager\.cc\|getHrTsVal\|getHrTsVal\|Environment[\s\S]*crates/ep_runtime/src/schedules\.rs::detailed_schedule_lookup_state[\s\S]*routine\.schedule_detailed_get_hr_ts_val\.source_routine = "getHrTsVal"[\s\S]*routine\.schedule_detailed_get_hr_ts_val\.completion_status = "source_mapped"' -Description "ScheduleDetailed getHrTsVal source and Rust lookup remain source-mapped"
Assert-Contains -Path "docs\src\porting-map\time-weather-schedule.md" -Pattern 'Schedule:Compact DST Hour-24/Tomorrow Day-Type Evidence Checkpoint[\s\S]*does not\s+promote `calendar_time_state` beyond scaffold[\s\S]*does not complete[\s\S]*`Sched::UpdateScheduleVals`' -Description "DST schedule rollover bounded porting-map checkpoint"
Assert-Contains -Path "tools\reporting\conformance_evidence_report.py" -Pattern '"calendar_schedule_dst_hour24_tomorrow_day_type_exact_001": "DST schedule rollover"[\s\S]*command="compare-calendar-schedule-dst-hour24-tomorrow-day-type-exact"' -Description "DST schedule rollover evidence reporter registration"
Assert-Contains -Path $compiler -Pattern 'fn parse_timestep\s*\([\s\S]*model\.timestep = TimestepConfig[\s\S]*number_of_timesteps_per_hour' -Description "compiler owns explicit Timestep intake"
Assert-Contains -Path $compiler -Pattern 'fn compact_schedule_periods\s*\([\s\S]*compact_directive\(text, "Interpolate"\)[\s\S]*parse_schedule_interpolation\(text\)[\s\S]*ScheduleCompactUntilNotAlignedToTimestep[\s\S]*fn parse_schedule_interpolation\s*\([\s\S]*ScheduleInterpolation::No[\s\S]*ScheduleInterpolation::Average[\s\S]*ScheduleInterpolation::Linear' -Description "compiler owns typed compact interpolation and No-only alignment warning"
Assert-Contains -Path $scheduleCache -Pattern 'pub fn precompute_schedule_cache_for_environment_time_axis\s*\([\s\S]*compact_schedule_series_for_environment_time_axis' -Description "immutable environment cache compiles compact schedules from the shared EnvironmentTimeAxis"
Assert-Contains -Path $schedules -Pattern 'fn compact_schedule_series_for_environment_time_axis\s*\([\s\S]*detailed_schedule_environment_lookup_state\(point\)[\s\S]*fn detailed_schedule_environment_lookup_state\s*\([\s\S]*timestep_end_minute: point\.end_minute\.round\(\)\.clamp\(1\.0, 60\.0\)' -Description "environment cache values consume each zone-timestep end minute"
Assert-Contains -Path $schedules -Pattern 'pub fn precompile_compact_schedule_intervals\s*\([\s\S]*end_minute_of_day[\s\S]*fn compact_interval_value\s*\([\s\S]*minute >= interval\.start_minute_of_day' -Description "compact interval precompile and inclusive minute lookup"
Assert-Contains -Path $runtimeTestSourceOrder -Pattern 'fn compact_schedule_environment_axis_selects_each_zone_timestep_end_minute\s*\([\s\S]*number_of_timesteps_per_hour: 4[\s\S]*sample_count\(\), 96[\s\S]*end_minute, 15\.0[\s\S]*end_minute, 60\.0[\s\S]*vec!\[11\.0, 12\.0, 13\.0, 14\.0\][\s\S]*normalized_environment_timestep_timestamp_label' -Description "runtime exact four-zone-timestep endpoint selection test"
Assert-Contains -Path $compactZoneTimestepGate -Pattern '\$CaseId = "calendar_schedule_compact_zone_timestep_exact_001"[\s\S]*frequency = "timestep"[\s\S]*timestamp_contract = "ordered-exact-unique"[\s\S]*exactly 96 ordered, unique Timestep Schedule Value samples and timestamps[\s\S]*An explicit Interpolate:No directive' -Description "zone-timestep canonical manifest and omitted-directive nonclaim contracts"
Assert-Contains -Path $compactZoneTimestepGate -Pattern '\$expectedVectors = @\([\s\S]*"Timestep\|4"[\s\S]*Schedule:Compact\|Zone Timestep Schedule[\s\S]*Until: 00:15\|11[\s\S]*Until: 00:30\|12[\s\S]*Until: 00:45\|13[\s\S]*Until: 01:00\|14[\s\S]*Until: 24:00\|90[\s\S]*Output:Variable\|ZONE TIMESTEP SCHEDULE\|Schedule Value\|Timestep' -Description "zone-timestep complete IDF vectors and aligned default-No profile"
Assert-Contains -Path $compactZoneTimestepGate -Pattern 'HOLIDAYS/DAYLIGHT SAVINGS,No,0,0,0[\s\S]*DATA PERIODS,1,1,Data,Thursday,1/1,1/1[\s\S]*\$weatherLines\.Count -ne 32[\s\S]*\$weatherRows\.Count -ne 24' -Description "zone-timestep exact no-DST one-day EPW shape"
Assert-Contains -Path $compactZoneTimestepGate -Pattern '\$summary\.series_count -ne 1[\s\S]*time_axis_samples -ne 96[\s\S]*zone-timestep ending samples aligned by EnergyPlus ESO timestamp labels[\s\S]*\$null -ne \$summary\.weather_record_selection' -Description "zone-timestep summary count, rule, and record-selection nonclaim"
Assert-Contains -Path $compactZoneTimestepGate -Pattern '\$series\.frequency -cne "timestep"[\s\S]*expected_samples -ne 96[\s\S]*ordered-exact-unique[\s\S]*start=0\.00;end=15\.00[\s\S]*hour=24;start=45\.00;end=60\.00[\s\S]*max_abs_delta -ne 0\.0' -Description "zone-timestep exact series metadata, endpoints, and zero delta"
Assert-Contains -Path $compactZoneTimestepGate -Pattern 'Schedule Value \[\] !TimeStep[\s\S]*\$values\.Count -ne 96[\s\S]*\$timestamps\.Count -ne 96[\s\S]*for \(\$index = 0; \$index -lt 96; \+\+\$index\)[\s\S]*11\.0 \+ \$zoneTimestep[\s\S]*Where-Object \{ \$_ -eq 90\.0 \}\)\.Count -ne 92' -Description "zone-timestep raw TimeStep dictionary, 96 values/timestamps, and exact values"
Assert-Contains -Path $compactZoneTimestepGate -Pattern 'Environment,SCHEDULE COMPACT ZONE TIMESTEP RUN PERIOD,WeatherFileRunPeriod,01/01/2032,01/01/2032,Thursday,1,Use RunPeriod Specified Day,No,No,No,No,No,Clark and Allen[\s\S]*Environment:Daylight Saving,No,RunPeriod Object[\s\S]*EnergyPlus Completed Successfully-- 0 Warning; 0 Severe Errors;[\s\S]*Schedule:Compact zone-timestep exact gate passed\.' -Description "zone-timestep exact EIO, clean completion, and blocking success"
Assert-Contains -Path $algorithmLedger -Pattern 'src/EnergyPlus/SimulationManager\.cc\|GetProjectData\|GetProjectData\|Environment[\s\S]*src/EnergyPlus/ScheduleManager\.cc\|ProcessScheduleInput\|ProcessScheduleInput\|Environment[\s\S]*ProcessIntervalFields\|ProcessIntervalFields[\s\S]*populateFromMinuteVals\|populateFromMinuteVals[\s\S]*getHrTsVal\|getHrTsVal' -Description "zone-timestep EnergyPlus source-order routine mapping"
Assert-Contains -Path $algorithmLedger -Pattern 'Compiler::parse_timestep[\s\S]*Compiler::parse_compact_schedules[\s\S]*Compiler::compact_schedule_periods[\s\S]*precompute_schedule_cache_for_environment_time_axis[\s\S]*detailed_schedule_environment_lookup_state[\s\S]*precompile_compact_schedule_intervals[\s\S]*normalized_environment_timestep_timestamp_label' -Description "zone-timestep Rust target mapping"
Assert-Contains -Path $algorithmLedger -Pattern 'calendar_schedule_compact_zone_timestep_exact_001[\s\S]*default-No end-minute sampling[\s\S]*Rust EPW record selection is null and unclaimed[\s\S]*Explicit Interpolate directives' -Description "zone-timestep bounded algorithm evidence and nonclaims"
Assert-Contains -Path $algorithmLedger -Pattern 'routine\.process_schedule_input\.source_routine = "ProcessScheduleInput"[\s\S]*routine\.process_interval_fields\.source_routine = "ProcessIntervalFields"[\s\S]*routine\.day_schedule_populate_from_minute_vals\.source_routine = "populateFromMinuteVals"[\s\S]*routine\.schedule_detailed_get_hr_ts_val\.source_routine = "getHrTsVal"' -Description "zone-timestep required routines remain source-mapped"
Assert-Contains -Path "docs\src\porting-map\time-weather-schedule.md" -Pattern 'Schedule:Compact Default-No Zone-Timestep Evidence Checkpoint[\s\S]*does not promote `calendar_time_state` beyond scaffold[\s\S]*Explicit `Interpolate` directives[\s\S]*internal evidence only' -Description "zone-timestep bounded porting-map checkpoint"
Assert-Contains -Path "tools\reporting\conformance_evidence_report.py" -Pattern '"calendar_schedule_compact_zone_timestep_exact_001": "Compact zone timestep"[\s\S]*command="compare-calendar-schedule-compact-zone-timestep-exact"' -Description "zone-timestep evidence reporter registration"
Assert-Contains -Path $scheduleModel -Pattern 'pub enum ScheduleInterpolation\s*\{[\s\S]*No,[\s\S]*Average,[\s\S]*Linear,[\s\S]*pub struct ScheduleCompactDayProfile\s*\{[\s\S]*pub interpolation: ScheduleInterpolation' -Description "typed compact-schedule interpolation model"
Assert-Contains -Path $compiler -Pattern 'fn parses_schedule_compact_interpolation_modes\s*\([\s\S]*Interpolate: No[\s\S]*Interpolate: Average[\s\S]*Interpolate: Linear[\s\S]*fn rejects_invalid_duplicate_and_misplaced_schedule_compact_interpolation\s*\([\s\S]*InvalidScheduleCompactInterpolation[\s\S]*DuplicateScheduleCompactInterpolation[\s\S]*InvalidScheduleCompactInterpolationOrder[\s\S]*fn warns_only_for_no_interpolation_until_not_aligned_to_valid_timestep\s*\([\s\S]*ScheduleCompactUntilNotAlignedToTimestep[\s\S]*assert_eq!\(warnings\.len\(\), 2\)' -Description "compiler interpolation modes, fail-closed diagnostics, and No-only warning tests"
Assert-Contains -Path $schedules -Pattern 'pub struct CompiledScheduleDayProfile\s*\{[\s\S]*Immutable daily values at zone-timestep resolution[\s\S]*zone_timestep_values: Vec<f64>[\s\S]*pub fn precompile_compact_schedule_periods\s*\([\s\S]*fn precompile_compact_schedule_day_profile\s*\([\s\S]*expand_compact_schedule_minute_values\(profile\)[\s\S]*chunks_exact\(minutes_per_timestep as usize\)[\s\S]*ScheduleInterpolation::Average[\s\S]*window\.iter\(\)\.sum::<f64>\(\)[\s\S]*ScheduleInterpolation::No \| ScheduleInterpolation::Linear[\s\S]*window\.last\(\)[\s\S]*fn expand_compact_schedule_minute_values\s*\([\s\S]*current_value \+= increment' -Description "intermediate minute expansion and immutable zone-timestep cache semantics"
Assert-Contains -Path $runtimeTestSourceOrder -Pattern 'fn compact_schedule_environment_axis_applies_no_average_and_linear_interpolation\s*\([\s\S]*ScheduleInterpolation::No[\s\S]*ScheduleInterpolation::Average[\s\S]*ScheduleInterpolation::Linear[\s\S]*fn compact_schedule_linear_interpolation_keeps_first_interval_flat_across_hour_boundary\s*\([\s\S]*\[10\.0, 175\.0, 175\.0, 175\.0, 175\.0\][\s\S]*\[10\.0, 120\.0, 175\.0, 175\.0, 175\.0\][\s\S]*\[10\.0, 40\.0, 85\.0, 130\.0, 175\.0\]' -Description "runtime interpolation and source-order cross-hour exact vectors"
Assert-Contains -Path $compactInterpolationGate -Pattern '\$CaseId = "calendar_schedule_compact_interpolation_modes_exact_001"[\s\S]*frequency = "timestep"[\s\S]*timestamp_contract = "ordered-exact-unique"[\s\S]*other timestep counts[\s\S]*Until 24:MM correction[\s\S]*multi-profile mixed interpolation modes[\s\S]*Rust raw ESO serialization' -Description "interpolation canonical manifest and bounded nonclaims"
Assert-Contains -Path $compactInterpolationGate -Pattern '\$expectedVectors = @\([\s\S]*Interpolate: No\|Until: 00:20\|10\|Until: 01:15\|175[\s\S]*Interpolate: Average\|Until: 00:20\|10\|Until: 01:15\|175[\s\S]*Interpolate: Linear\|Until: 00:20\|10\|Until: 01:15\|175[\s\S]*\$weatherLines\.Count -ne 32[\s\S]*\$weatherRows\.Count -ne 24' -Description "interpolation complete IDF vectors and one-day EPW"
Assert-Contains -Path $compactInterpolationGate -Pattern '\$summary\.series_count -ne 3[\s\S]*time_axis_samples -ne 96[\s\S]*\$null -ne \$summary\.weather_record_selection[\s\S]*FirstValues = @\(10\.0, 175\.0, 175\.0, 175\.0, 175\.0\)[\s\S]*FirstValues = @\(10\.0, 120\.0, 175\.0, 175\.0, 175\.0\)[\s\S]*FirstValues = @\(10\.0, 40\.0, 85\.0, 130\.0, 175\.0\)[\s\S]*max_abs_delta -ne 0\.0' -Description "interpolation three exact series and zero delta"
Assert-Contains -Path $compactInterpolationGate -Pattern 'Schedule Value \[\] !TimeStep[\s\S]*\$timestamps\.Count -ne 96[\s\S]*\$values\.Count -ne 96[\s\S]*Environment,SCHEDULE COMPACT INTERPOLATION MODES RUN PERIOD[\s\S]*EnergyPlus Completed Successfully-- 1 Warning; 0 Severe Errors;[\s\S]*Invalid "until" field value is not a multiple[\s\S]*INTERPOLATION NO SCHEDULE_dy_1[\s\S]*Assert-NotContains[\s\S]*INTERPOLATION AVERAGE SCHEDULE[\s\S]*Assert-NotContains[\s\S]*INTERPOLATION LINEAR SCHEDULE[\s\S]*Schedule:Compact interpolation-modes exact gate passed\.' -Description "interpolation raw ESO, exact EIO/completion, and explicit-No-only oracle warning"
Assert-Contains -Path $algorithmLedger -Pattern 'id = "schedule_compact_interpolation_modes"[\s\S]*status = "conformance"[\s\S]*calendar_schedule_compact_interpolation_modes_exact_001[\s\S]*three 96-sample ordered-exact-unique zero-tolerance[\s\S]*Rust warning text/count parity remains unclaimed[\s\S]*calendar_time_state algorithm[\s\S]*remain scaffold/source_mapped' -Description "bounded interpolation algorithm evidence and routine nonpromotion"
Assert-Contains -Path "docs\src\porting-map\time-weather-schedule.md" -Pattern 'Schedule:Compact Explicit Interpolation Evidence Checkpoint[\s\S]*intermediate 1,440-minute lattice[\s\S]*immutable 96-value[\s\S]*does not promote .*calendar_time_state.* beyond scaffold[\s\S]*Rust warning text/count parity remains\s+unclaimed' -Description "bounded interpolation porting-map checkpoint"
Assert-Contains -Path "docs\src\porting-map\time-weather-schedule.md" -Pattern 'compact schedule input/default ownership[\s\S]*CP44 separately locks explicit No, Average, and Linear intake only for its one-day[\s\S]*compact interval minute expansion[\s\S]*CP44 separately locks explicit-No flat fill, Average flat minute values, a flat first Linear interval[\s\S]*day-schedule timestep population[\s\S]*CP44 separately locks explicit No and Linear endpoint sampling plus the Average 15-minute window mean' -Description "source map tables retain the exact CP44 exception"
Assert-Contains -Path "docs\src\porting-map\time-weather-schedule.md" -Pattern '\| hourly consumers \|[\s\S]*CP44 explicit-mode case separately locks three one-day[\s\S]*\| schedules \|[\s\S]*CP44 exception locks exactly three explicit No/Average/Linear[\s\S]*beyond the exact CP44 fixture[\s\S]*\| output time \|[\s\S]*CP44 separately locks the same 96 normalized Timestep labels' -Description "consumer, schedule, and output-time tables retain CP44 boundaries"
Assert-Contains -Path "tools\reporting\conformance_evidence_report.py" -Pattern '"calendar_schedule_compact_interpolation_modes_exact_001": "Compact interpolation"[\s\S]*command="compare-calendar-schedule-compact-interpolation-modes-exact"[\s\S]*calendar_schedule_compact_interpolation_modes_exact_001\\compare\\compare-summary\.json' -Description "interpolation evidence reporter registration"
Assert-Contains -Path $algorithmLedger -Pattern 'routine\.set_special_day_dates\.completion_status = "source_mapped"' -Description "SetSpecialDayDates remains source-mapped"
Assert-Contains -Path $runtimeTestSpecialDays -Pattern 'fn later_typed_special_day_definition_overwrites_an_earlier_definition\s*\(' -Description "special-day later-wins unit test"
Assert-Contains -Path $runtimeTestSpecialDays -Pattern 'fn weekend_rule_shifts_only_fixed_single_day_special_days_to_monday\s*\(' -Description "special-day weekend-rule unit test"
Assert-Contains -Path $runtimeTestSpecialDays -Pattern 'fn nth_and_last_special_day_rules_resolve_and_nonexistent_nth_is_rejected\s*\(' -Description "special-day Nth/last resolution and invalid-fifth test"
Assert-Contains -Path $runtimeTestSpecialDays -Pattern 'build_hourly_time_axis\(&nonexistent_model\)\.expect_err' -Description "nonexistent Nth-weekday time-axis rejection test"
Assert-Contains -Path $pipeline -Pattern 'prepare_runtime_inputs\([\s\S]*diagnostics\.error\("RuntimeConvergenceFailure", "runtime", error\)' -Description "ep_run runtime-input error diagnostic projection"
Assert-Contains -Path $pipeline -Pattern 'load_epjson_file_with_idf_order' -Description "IDF arbitrary-run preserves configured declaration order"
Assert-Contains -Path $pipeline -Pattern 'RunExitCode::Runtime' -Description "ep_run runtime failure exit mapping"
Assert-Contains -Path $runtimeTestSpecialDays -Pattern 'fn cross_year_special_days_reuse_the_source_start_year_annual_table\s*\([\s\S]*\(1, 2, 2\)[\s\S]*4 \* 24[\s\S]*DayType::Tuesday[\s\S]*DayType::Wednesday[\s\S]*DayType::Thursday[\s\S]*DayType::Holiday' -Description "cross-year special-day start-year annual-table retention test"
Assert-Contains -Path $runtimeTestEpwHolidays -Pattern 'fn run_period_flag_enables_epw_sunday_type_holiday_on_both_time_axes\s*\(' -Description "EPW holiday policy and Sunday-type both-axis test"
Assert-Contains -Path $runtimeTestEpwHolidays -Pattern 'fn disabling_epw_holidays_does_not_disable_input_file_special_days\s*\(' -Description "EPW-only holiday filtering test"
Assert-Contains -Path $runtimeTestEpwHolidays -Pattern 'fn input_file_custom_day_overwrites_weather_file_holiday_on_the_same_date\s*\(' -Description "EPW then input-file same-date precedence unit test"
Assert-Contains -Path $runtimeTestEpwHolidays -Pattern 'crate::SpecialDaySource::WeatherFile,[\s\S]*crate::SpecialDaySource::InputFile' -Description "resolved special-day order is weather-file then input-file"
Assert-Contains -Path $runtimeTestEpwHolidays -Pattern 'crate::DayType::CustomDay1,\s*Some\(crate::DayType::CustomDay1\),\s*11' -Description "later input-file CustomDay1 is effective day type"
Assert-Contains -Path $timeWeatherSchedule -Pattern 'Site Day Type Index' -Description "special day type report mapping"
Assert-Contains -Path $timeWeatherScheduleSpecialDays -Pattern 'special_day_hourly_samples' -Description "special-day report diagnostic sample count"
Assert-Contains -Path $timeWeatherScheduleSpecialDays -Pattern 'weather_file_holidays_declared' -Description "EPW holiday declaration diagnostic"
Assert-Contains -Path $timeWeatherScheduleSpecialDays -Pattern 'run_period_uses_weather_file_holidays' -Description "EPW holiday use-policy diagnostic"
Assert-Contains -Path $timeWeatherScheduleSpecialDays -Pattern 'weather_file_holidays_resolved' -Description "EPW holiday resolution diagnostic"
Assert-Contains -Path $timeWeatherScheduleSpecialDays -Pattern 'special_day.source.label\(\)' -Description "special-day source diagnostic attribution"
Assert-Contains -Path $timeWeatherScheduleSpecialDays -Pattern 'special_day_resolved:' -Description "resolved special-day markdown diagnostic"
Assert-Contains -Path $timeWeatherScheduleSpecialDays -Pattern 'special_day.start.day_of_year' -Description "resolved special-day ordinal JSON diagnostic"
Assert-Contains -Path $timeWeatherScheduleSpecialDays -Pattern 'special_day.weekend_shift_days' -Description "resolved special-day weekend-shift diagnostic"
Assert-Contains -Path $solar -Pattern 'next_solar_weather_record_within_day\s*\(' -Description "solar interpolation consumes day-local NextHr weather selector"
Assert-Contains -Path $runtimeTestSourceOrder -Pattern 'fn solar_next_hour_record_wraps_within_each_accepted_day\s*\(' -Description "accepted-day solar Hour24 NextHr wrap test"
Assert-Contains -Path $runtimeTestSourceOrder -Pattern 'solar_weather_interpolation_weights\(1,\s*1\),\s*\(0\.0,\s*1\.0,\s*0\.0\)' -Description "single-timestep current-only solar weather weights"
Assert-Contains -Path $runtime -Pattern 'precompute_weather_timestep_series' -Description "heat-balance runtime uses weather timestep precompute"
Assert-Contains -Path $scheduleCache -Pattern 'pub enum ScheduleSampleStorage\s*\{[\s\S]*Scalar\s*\{[\s\S]*Dense\(Box<\[f64\]>\)' -Description "immutable scalar-or-dense schedule sample storage"
Assert-Contains -Path $scheduleCache -Pattern 'pub struct ScheduleSeriesCache\s*\{[\s\S]*series: Box<\[CachedScheduleSeries\]>[\s\S]*profile: ScheduleCacheProfile' -Description "immutable indexed schedule cache and structural profile"
Assert-Contains -Path $scheduleCache -Pattern 'pub fn get\(&self, schedule_id: ScheduleId\)[\s\S]*pub fn profile\(&self\) -> ScheduleCacheProfile' -Description "typed-ID cache lookup and deterministic profile API"
Assert-Contains -Path $internalGainScheduleCache -Pattern 'pub\(crate\) fn precompute_hour_only_internal_gain_schedule_cache\s*\([\s\S]*validate_hour_only_internal_gain_schedules\(model\)[\s\S]*for equipment in &model\.other_equipment[\s\S]*referenced_schedule_ids\.insert\(schedule_id\)' -Description "referenced-only OtherEquipment schedule cache validates before source-order collection"
Assert-Contains -Path $internalGainScheduleCache -Pattern 'constant_cached_schedule_series[\s\S]*external_interface_cached_schedule_series_iter[\s\S]*compact_schedule_value\(segments, hour_ending \* 60\)\.unwrap_or\(f64::NAN\)' -Description "hour-only cache preserves Constant, External/FMU, Compact priority and raw fallback values"
Assert-Contains -Path $internalGainScheduleCache -Pattern 'hour_ending\.clamp\(1, 24\) - 1[\s\S]*schedule_cache\s*\.value\(schedule_id, sample_index\)[\s\S]*unwrap_or\(f64::NAN\)' -Description "cached hour lookup clamps and fails closed"
Assert-Contains -Path $internalGainScheduleProfile -Pattern 'pub struct InternalGainSchedulePhaseOperations\s*\{[\s\S]*cached_value_lookup_count[\s\S]*live_schedule_family_chain_scan_count[\s\S]*compact_profile_resolution_count[\s\S]*compact_value_evaluation_count' -Description "phase-local cached/live schedule operation counters"
Assert-Contains -Path $internalGainScheduleProfile -Pattern 'pub struct HeatBalanceInternalGainScheduleOperationProfile\s*\{[\s\S]*referenced_only_cache_build_count[\s\S]*cache_build_compact_value_evaluation_count[\s\S]*initialization[\s\S]*warmup[\s\S]*run_period' -Description "simulation-owned phase-separated internal-gain schedule profile"
Assert-Contains -Path $schedules -Pattern 'hour_only_schedule_multiplier_live_profiled[\s\S]*schedule_value_with_operations\(model, schedule_id, hour_ending, Some\(operations\)\)[\s\S]*pub\(crate\) fn schedule_value[\s\S]*schedule_value_with_operations\(model, schedule_id, hour_ending, None\)[\s\S]*fn schedule_value_with_operations' -Description "profiled live control and legacy lookup share one source-order resolver"
Assert-Contains -Path $internalGainScheduleCache -Pattern 'precompute_hour_only_internal_gain_schedule_cache_with_build_operations[\s\S]*&mut compact_value_evaluation_count[\s\S]*precompute_hour_only_internal_gain_schedule_cache_profiled[\s\S]*for_single_build\([\s\S]*cache\.len\(\)[\s\S]*cache\.profile\(\)\.logical_sample_count[\s\S]*compact_value_evaluation_count[\s\S]*for hour_ending in 1_u32\.\.=24[\s\S]*compact_value_evaluation_count\.saturating_add\(1\)[\s\S]*compact_schedule_value' -Description "specialized cache directly counts Compact materialization evaluations and reports entries and logical samples"
Assert-Contains -Path $initializationScheduleCache -Pattern 'initialize_heat_balance_state_with_ctf_coefficients_and_schedule_cache_profiled[\s\S]*profile\.initialization[\s\S]*convective_internal_gain_w_from_cache_profiled[\s\S]*update_surface_radiant_internal_gain_source_terms_from_cache_profiled' -Description "initialization records profiled cache reads"
Assert-Contains -Path $runtime -Pattern 'mut internal_gain_schedule_cache_profile[\s\S]*precompute_hour_only_internal_gain_schedule_cache_profiled[\s\S]*internal_gain_schedule_cache_profile\.warmup[\s\S]*internal_gain_schedule_cache_profile\.run_period[\s\S]*internal_gain_schedule_cache_profile,' -Description "one simulation-owned profile spans cache build, initialization, warmup, and run period"
Assert-Contains -Path $timestep -Pattern 'advance_heat_balance_state_one_timestep_internal_with_schedule_cache_profiled[\s\S]*convective_internal_gain_w_from_cache_profiled[\s\S]*convective_internal_gain_w_live_profiled[\s\S]*update_surface_radiant_internal_gain_source_terms_from_cache_profiled[\s\S]*update_surface_radiant_internal_gain_source_terms_live_profiled' -Description "profiled cached path and explicit live control remain separate"
Assert-Contains -Path $runPeriod -Pattern 'sample_heat_balance_run_period\s*\([\s\S]*schedule_operations: &mut InternalGainSchedulePhaseOperations[\s\S]*advance_heat_balance_state_one_timestep_internal_with_schedule_cache_profiled' -Description "run-period sampler records actual cached reads"
Assert-Contains -Path $runtimeTestScheduleCache -Pattern 'heat_balance_cache_reuses_hour_samples_across_substeps_warmup_and_two_days[\s\S]*profile\.initialization\.cached_value_lookup_count, 2[\s\S]*profile\.warmup\.cached_value_lookup_count, 288[\s\S]*profile\.run_period\.cached_value_lookup_count, 576[\s\S]*cached_and_live_internal_gain_timestep_paths_are_bit_equal_with_nonvacuous_operation_counts[\s\S]*live_operations\.compact_value_evaluation_count, 48[\s\S]*heat_balance_cache_ignores_unreferenced_invalid_compact_schedule' -Description "exact lifecycle counts, timestep bitwise A/B, nonvacuous live control, and referenced-only boundary tests"
Assert-Contains -Path $cli -Pattern 'internal_gain_schedule_cache_operations[\s\S]*deterministic operation counts; not wall-clock timing or cache-specific speedup attribution[\s\S]*total_live_schedule_family_chain_scan_count[\s\S]*total_compact_value_evaluation_count' -Description "CLI exports bounded deterministic operation evidence"
Assert-Contains -Path $precompute -Pattern 'pub struct RuntimePrecomputedData' -Description "runtime precomputed data bundle"
Assert-Contains -Path $precompute -Pattern 'output_registry: RuntimeOutputRegistry' -Description "run cached output registry"
Assert-Contains -Path $precompute -Pattern 'build_execution_plan_with_output_registry' -Description "execution plan uses cached output registry"
Assert-Contains -Path $executionPlan -Pattern 'pub fn build_execution_plan_with_output_registry' -Description "execution plan cache-aware builder"
Assert-Contains -Path $pipeline -Pattern 'precompute_runtime_data' -Description "pipeline caches runtime precomputed data"
Assert-Contains -Path $pipeline -Pattern '"output_registry_count": precomputed\.output_registry\.len\(\)' -Description "execution-plan artifact records cached output registry"
Assert-Contains -Path $pipeline -Pattern 'rust_runtime_setup' -Description "runtime setup phase separated from runtime loop"
Assert-Contains -Path $pipeline -Pattern 'prepare_runtime_inputs' -Description "runtime inputs prepared before execution loop"
Assert-Contains -Path $pipeline -Pattern 'PreparedRuntimeInputs' -Description "prepared runtime input bundle"
Assert-Contains -Path $pipeline -Pattern 'weather runtimes load rich EPW metadata, build the metadata-aware time axis, select source-order records, and precompute weather timesteps' -Description "weather runtime setup timing scope"
$prepareRuntimeInputsScope = '(?s)fn prepare_runtime_inputs\s*\((?:(?!\r?\nfn execute_rust_runtime\s*\().)*'
$executeRustRuntimeScope = '(?s)fn execute_rust_runtime\s*\((?:(?!\r?\nfn runtime_class_requires_weather\s*\().)*'
Assert-Contains -Path $pipeline -Pattern ($prepareRuntimeInputsScope + '\bload_epw_weather_file\s*\(') -Description "rich EPW loading inside runtime input preparation"
Assert-Contains -Path $pipeline -Pattern ($prepareRuntimeInputsScope + '\bbuild_hourly_time_axis_with_weather_metadata\s*\(') -Description "metadata-aware time-axis construction inside runtime input preparation"
Assert-Contains -Path $pipeline -Pattern ($prepareRuntimeInputsScope + '\bselect_epw_environment_weather\s*\(') -Description "source-order EPW selection inside runtime input preparation"
Assert-Contains -Path $pipeline -Pattern ($prepareRuntimeInputsScope + '\bprecompute_weather_timestep_series\s*\(') -Description "weather timestep precompute inside runtime input preparation"
Assert-Contains -Path $pipeline -Pattern ($prepareRuntimeInputsScope + '\bprecompute_schedule_cache_for_time_axis\s*\(') -Description "immutable schedule cache construction inside runtime input preparation"
Assert-Contains -Path $pipeline -Pattern 'struct PreparedRuntimeInputs\s*\{[\s\S]*schedule_cache: ScheduleSeriesCache' -Description "ep_run prepared inputs own one immutable schedule cache"
Assert-Contains -Path $pipeline -Pattern ($executeRustRuntimeScope + 'runtime_inputs\.schedule_cache\.sample_count\(\)[\s\S]*runtime_inputs\.schedule_cache\.profile\(\)[\s\S]*runtime_inputs\.schedule_cache\.len\(\)') -Description "ep_run reads cache size and structural profile without schedule values"
Assert-NotContains -Path $pipeline -Pattern ($executeRustRuntimeScope + 'schedule_cache\.(?:get|value)\s*\(') -Description "ep_run numerical schedule-cache lookup inside runtime execution"
Assert-Contains -Path $pipeline -Pattern 'fn schedule_cache_json\s*\([\s\S]*scalar_series_count[\s\S]*allocated_dense_sample_count[\s\S]*index_kind[\s\S]*"schedule_cache": schedule_cache_json' -Description "ep_run run summary exports deterministic cache profile metadata"
Assert-NotContains -Path $pipeline -Pattern 'schedule_series:\s*Vec<ScheduleValueSeries>|precompute_schedule_value_series_for_time_axis' -Description "legacy materialized schedule series in ep_run prepared inputs"
Assert-Contains -Path $cli -Pattern 'ep_runtime consumes a separately prepared referenced-only 24-hour cache for validated hour-only OtherEquipment gains, while this full-axis schedule cache remains profile-only' -Description "runtime phase distinguishes specialized heat-balance consumption from full-axis profile-only cache"
Assert-Contains -Path $timeWeatherSchedule -Pattern 'schedule_cache: Option<&ScheduleSeriesCache>[\s\S]*schedule_cache\s*\.\s*get\(schedule_id\)[\s\S]*trace\.values\(\)' -Description "schedule report holds the cache and resolves requested values by typed ID"
Assert-Contains -Path $timeWeatherSchedule -Pattern 'precompute_schedule_cache_for_environment_time_axis[\s\S]*precompute_schedule_cache_for_time_axis' -Description "schedule report selects the cache builder for its time axis"
Assert-NotContains -Path $timeWeatherSchedule -Pattern 'ScheduleValueSeries|precompute_schedule_value_series' -Description "legacy materialized schedule series in schedule report consumers"
Assert-NotContains -Path $pipeline -Pattern ($executeRustRuntimeScope + '\bload_epjson_file\s*\(') -Description "epJSON parsing inside runtime execution"
Assert-NotContains -Path $pipeline -Pattern ($executeRustRuntimeScope + '\bload_epw_(?:records|weather_file)\s*\(') -Description "EPW loading inside runtime execution"
Assert-NotContains -Path $pipeline -Pattern ($executeRustRuntimeScope + '\bselect_epw_environment_weather\s*\(') -Description "EPW source-order selection inside runtime execution"
Assert-Contains -Path $initialization -Pattern 'HeatBalanceSurfaceIndexes::from_model_surfaces' -Description "heat-balance surface indexes initialized once"
Assert-Contains -Path $zoneAirCorrection -Pattern 'HeatBalanceSurfaceIndexes' -Description "zone-air correction consumes precomputed surface indexes"
Assert-Contains -Path $timestep -Pattern 'state\.surface_indexes\.surfaces_for_zone' -Description "timestep hot path uses precomputed zone surface indexes"
Assert-Contains -Path $reports -Pattern 'zone_surface_report_conduction_rates_for_indices_w' -Description "zone conduction report uses precomputed surface indexes"
Assert-Contains -Path $radiation -Pattern 'HeatBalanceSurfaceIndexes' -Description "radiation loop consumes precomputed surface indexes"
Assert-NotContains -Path $radiation -Pattern 'BTreeMap::<ZoneId, Vec<usize>>::new' -Description "radiation-owned per-call zone surface grouping"
Assert-Contains -Path $state -Pattern 'pub zones: Vec<ZoneHeatBalanceState>' -Description "heat-balance runtime loops have compact zone Vec state"
Assert-Contains -Path $state -Pattern 'pub surfaces: Vec<SurfaceHeatBalanceState>' -Description "heat-balance runtime loops have compact surface Vec state"
Assert-Contains -Path $algorithm -Pattern 'pub\(crate\) struct HeatBalanceRuntimeConfig' -Description "heat-balance branch conditions are compiled into a probe-agnostic runtime config"
Assert-Contains -Path $cli -Pattern 'let mut trace_level = TraceLevel::Normal' -Description "diagnostic trace defaults to normal level"
Assert-Contains -Path $runConfig -Pattern 'pub struct TraceSelection' -Description "selected trace target configuration"
Assert-Contains -Path $runConfig -Pattern 'pub surface_names: Vec<String>' -Description "selected surface trace targets"
Assert-Contains -Path $runConfig -Pattern 'pub node_names: Vec<String>' -Description "selected node trace targets"
Assert-Contains -Path $cli -Pattern '--trace-surface' -Description "CLI selected surface trace option"
Assert-Contains -Path $cli -Pattern '--trace-node' -Description "CLI selected node trace option"
Assert-Contains -Path $pipeline -Pattern 'selected_trace_enabled' -Description "selected trace requires explicit target names"
Assert-Contains -Path $pipeline -Pattern 'zone/surface/ctf payloads are emitted only for explicitly requested names' -Description "selected trace policy artifact"
Assert-Contains -Path $pipeline -Pattern 'write_runtime_artifacts' -Description "output export after runtime"
Assert-Contains -Path $pipeline -Pattern 'rust_output_export' -Description "output export phase timing"
Assert-Contains -Path $pipeline -Pattern 'render_run_report' -Description "report generation after runtime completion path"
Assert-Contains -Path $runtime -Pattern 'ScheduleSeriesCache[\s\S]*precompute_constant_schedule_cache[\s\S]*precompute_schedule_cache_for_time_axis' -Description "runtime public surface exports immutable schedule-cache APIs"
Assert-Contains -Path $scheduleConstant -Pattern '(?s)pub fn simulate_constant_schedules.*precompute_constant_schedule_cache\(model, sample_count\)\.into_traces\(\)' -Description "legacy constant simulation delegates to the immutable constant cache"
Assert-Contains -Path $schedules -Pattern 'precompute_schedule_cache_for_time_axis\(model, time_axis\)\.into_traces\(\)' -Description "legacy hourly adapter preserves numerical traces from the immutable cache"
Assert-Contains -Path $summary -Pattern 'HeatBalanceWarmupSummary' -Description "warmup summary owner"
Assert-Contains -Path $summary -Pattern 'HeatBalanceSimulationSummary' -Description "heat-balance simulation summary owner"
Assert-Contains -Path $summary -Pattern 'HeatBalanceSimulation' -Description "heat-balance simulation result owner"
Assert-NotContains -Path $state -Pattern 'pub struct HeatBalanceSimulationSummary' -Description "state-owned simulation summary"
Assert-Contains -Path $runPeriod -Pattern 'sample_heat_balance_run_period' -Description "run-period sampler owner"
Assert-Contains -Path $runtime -Pattern 'sample_heat_balance_run_period' -Description "runtime delegates run-period sampling"
Assert-Contains -Path $runtime -Pattern 'init_heat_balance_source_order_path' -Description "runtime enters InitHeatBalance source-order wrapper"
Assert-Contains -Path $surfaceWeather -Pattern 'CalcHeatBalanceOutsideSurf' -Description "surface weather source owner"
Assert-Contains -Path $surfaceWeather -Pattern 'energyplus_exterior_wet_timestep_fraction' -Description "exterior wet timestep fraction owner"
Assert-Contains -Path $surfaceWeather -Pattern 'energyplus_exterior_wet_context_fraction' -Description "exterior wet context fraction owner"
Assert-Contains -Path $surfaceWeather -Pattern 'energyplus_weather_record_is_rain_at_timestep' -Description "rain interpolation owner"
Assert-Contains -Path $surfaceWeather -Pattern 'energyplus_exterior_wet_reference_temperature_c' -Description "wet exterior reference temperature owner"
Assert-NotContains -Path $runtime -Pattern 'fn energyplus_exterior_wet_timestep_fraction\s*\(' -Description "runtime-owned exterior wet timestep fraction"
Assert-NotContains -Path $runtime -Pattern 'fn energyplus_exterior_wet_context_fraction\s*\(' -Description "runtime-owned exterior wet context fraction"
Assert-NotContains -Path $runtime -Pattern 'fn energyplus_weather_record_is_rain_at_timestep\s*\(' -Description "runtime-owned rain interpolation"
Assert-NotContains -Path $runtime -Pattern 'fn energyplus_exterior_wet_reference_temperature_c\s*\(' -Description "runtime-owned wet exterior reference temperature"
Assert-Contains -Path $timestep -Pattern 'advance_heat_balance_state_one_timestep' -Description "heat-balance timestep advance owner"
Assert-Contains -Path $timestep -Pattern 'advance_heat_balance_state_one_timestep_internal' -Description "heat-balance internal timestep advance owner"
Assert-Contains -Path $timestep -Pattern 'manager::manage_heat_balance_source_order_path' -Description "timestep enters ManageHeatBalance source-order wrapper"
Assert-Contains -Path $timestep -Pattern 'surface_manager::manage_surface_heat_balance_source_order_path' -Description "timestep enters ManageSurfaceHeatBalance source-order wrapper"
Assert-Contains -Path $timestep -Pattern 'surface_manager::init_surface_heat_balance_source_order_path' -Description "timestep enters InitSurfaceHeatBalance source-order wrapper"
Assert-Contains -Path $timestep -Pattern 'surface_manager::calc_heat_balance_outside_surf_source_order_path' -Description "timestep enters CalcHeatBalanceOutsideSurf source-order wrapper"
Assert-Contains -Path $timestep -Pattern 'surface_manager::calc_heat_balance_inside_surf_source_order_path' -Description "timestep enters CalcHeatBalanceInsideSurf source-order wrapper"
Assert-Contains -Path $timestep -Pattern 'air_manager::manage_air_heat_balance_compat' -Description "timestep enters ManageAirHeatBalance compatibility wrapper"
Assert-Contains -Path $timestep -Pattern 'zone_predictor_corrector::manage_zone_air_updates_compat' -Description "timestep enters ManageZoneAirUpdates compatibility wrapper"
Assert-Contains -Path $timestep -Pattern 'zone_predictor_corrector::predict_system_loads_compat' -Description "timestep enters PredictStep compatibility wrapper"
Assert-Contains -Path $timestep -Pattern 'zone_predictor_corrector::correct_step_source_order_path' -Description "timestep enters CorrectStep source-order wrapper"
Assert-Contains -Path $timestep -Pattern 'surface_manager::update_final_surface_heat_balance_source_order_path' -Description "timestep enters UpdateFinalSurfaceHeatBalance source-order wrapper"
Assert-Contains -Path $timestep -Pattern 'surface_manager::update_thermal_histories_source_order_path' -Description "timestep enters UpdateThermalHistories source-order wrapper"
Assert-Contains -Path $runPeriod -Pattern 'surface_manager::report_surface_heat_balance_source_order_path' -Description "run-period enters ReportSurfaceHeatBalance source-order wrapper"
Assert-Contains -Path $zoneAirCorrection -Pattern 'revert_zone_timestep_histories_compat' -Description "adaptive zone-air correction enters RevertZoneTimestepHistories compatibility wrapper"
Assert-Contains -Path $zoneAirCorrection -Pattern 'push_system_timestep_histories_compat' -Description "adaptive zone-air correction enters PushSystemTimestepHistories compatibility wrapper"
Assert-NotContains -Path $runtime -Pattern 'fn advance_heat_balance_state_one_timestep\s*\(' -Description "runtime-owned heat-balance timestep advance"
Assert-NotContains -Path $runtime -Pattern 'fn advance_heat_balance_state_one_timestep_internal\s*\(' -Description "runtime-owned internal heat-balance timestep advance"
Assert-Contains -Path $warmup -Pattern 'CheckWarmupConvergence' -Description "warmup source owner"
Assert-Contains -Path $warmup -Pattern 'run_heat_balance_run_period_warmup' -Description "run-period warmup loop owner"
Assert-Contains -Path $warmup -Pattern 'heat_balance_zone_temperature_snapshot' -Description "warmup zone-temperature snapshot owner"
Assert-Contains -Path $warmup -Pattern 'max_abs_pair_delta' -Description "warmup convergence delta owner"
Assert-NotContains -Path $runtime -Pattern 'fn run_heat_balance_run_period_warmup\s*\(' -Description "runtime-owned run-period warmup loop"
Assert-NotContains -Path $runtime -Pattern 'fn heat_balance_zone_temperature_snapshot\s*\(' -Description "runtime-owned warmup zone-temperature snapshot"
Assert-NotContains -Path $runtime -Pattern 'fn max_abs_pair_delta\s*\(' -Description "runtime-owned warmup convergence delta"
Assert-Contains -Path $reports -Pattern 'zone_surface_report_conduction_rates_for_indices_w' -Description "zone surface conduction report owner"
Assert-Contains -Path $reports -Pattern 'heat_gain_rate_w' -Description "positive heat-gain report helper owner"
Assert-Contains -Path $reports -Pattern 'heat_loss_rate_w' -Description "positive heat-loss report helper owner"
Assert-NotContains -Path $runtime -Pattern 'fn zone_surface_report_conduction_rates_for_indices_w\s*\(' -Description "runtime-owned zone surface conduction report"
Assert-NotContains -Path $runtime -Pattern 'fn heat_gain_rate_w\s*\(' -Description "runtime-owned heat gain report helper"
Assert-NotContains -Path $runtime -Pattern 'fn heat_loss_rate_w\s*\(' -Description "runtime-owned heat loss report helper"
Assert-Contains -Path $reports -Pattern 'ReportSurfaceHeatBalance' -Description "surface report owner"

Assert-Contains -Path $executionPlan -Pattern 'ManageZoneAirUpdates' -Description "ManageZoneAirUpdates execution stage kind"
Assert-Contains -Path $executionPlan -Pattern 'pub const fn is_source_order_barrier' -Description "source-order capable execution stage kind"
Assert-Contains -Path $executionPlan -Pattern 'manage_heat_balance_source_order_stages' -Description "execution plan consumes heat-balance source-order module"
Assert-Contains -Path $executionPlan -Pattern 'ExecutionStageKind::ManageZoneAirUpdates' -Description "zone-air steps bind to ManageZoneAirUpdates"
Assert-Contains -Path $executionPlan -Pattern 'SimPurchasedAir,' -Description "SimPurchasedAir stage kind"
Assert-Contains -Path $executionPlan -Pattern 'GetPurchasedAir,' -Description "GetPurchasedAir stage kind"
Assert-Contains -Path $executionPlan -Pattern 'InitPurchasedAir,' -Description "InitPurchasedAir stage kind"
Assert-Contains -Path $executionPlan -Pattern 'CalcPurchAirLoads,' -Description "CalcPurchAirLoads stage kind"
Assert-Contains -Path $executionPlan -Pattern 'UpdatePurchasedAir,' -Description "UpdatePurchasedAir stage kind"
Assert-Contains -Path $executionPlan -Pattern 'ReportPurchasedAir,' -Description "ReportPurchasedAir stage kind"
Assert-Contains -Path $runtimeTestSourceOrder -Pattern 'ExecutionStageKind::ManageZoneAirUpdates' -Description "runtime tests assert ManageZoneAirUpdates barrier"
Assert-Contains -Path $pipeline -Pattern 'ExecutionPlanSourceOrderMismatch' -Description "stage order mismatch blocks compatibility runtime"
Assert-Contains -Path $pipeline -Pattern '"source_order_gate": source_order_gate' -Description "execution-plan.json stores source-order gate"
Assert-Contains -Path $pipeline -Pattern '"expected_source_order_stages": expected_source_order_stages' -Description "execution-plan.json stores expected source-order stages"
Assert-Contains -Path $pipeline -Pattern '"actual_executed_source_order_stages": actual_executed_source_order_stages' -Description "execution-plan.json stores actual source-order stages"
Assert-Contains -Path $pipeline -Pattern '"compatibility_stages": plan\.compatibility_stages' -Description "execution-plan.json stores expected EnergyPlus stage list"
Assert-Contains -Path $pipeline -Pattern '"stages": plan\.stages' -Description "execution-plan.json stores executable stage list"
Assert-Contains -Path $pipeline -Pattern 'trace_level_enables_stage_snapshots' -Description "trace-level stage snapshot switch"
Assert-Contains -Path $pipeline -Pattern 'stage_snapshot_policy' -Description "stage snapshot non-mutating policy"
Assert-Contains -Path $pipeline -Pattern 'metadata-only source-order snapshots' -Description "stage snapshots exclude simulation values"
Assert-Contains -Path $pipeline -Pattern 'source_order_stage_state_snapshots' -Description "stage state snapshot trace builder"
Assert-Contains -Path $pipeline -Pattern 'source-order-stage-state-snapshots\.json' -Description "stage state snapshot diagnostic artifact"
Assert-Contains -Path $pipeline -Pattern 'rusted-energyplus\.source-order-stage-state-snapshot\.v1' -Description "versioned stage state snapshot schema"
Assert-Contains -Path $pipeline -Pattern 'trace_artifact_only' -Description "stage state snapshots are trace-only"
foreach ($stageSnapshotTarget in @(
        'init-heat-balance',
        'calc-heat-balance-outside-surf',
        'calc-heat-balance-inside-surf',
        'manage-air-heat-balance',
        'update-thermal-histories',
        'report-surface-heat-balance',
        'ZoneTempPredictorCorrector::PredictStep',
        'ZoneTempPredictorCorrector::CorrectStep',
        'sim-purchased-air',
        'calc-purch-air-loads',
        'update-purchased-air',
        'report-purchased-air'
    )) {
    Assert-Contains -Path $pipeline -Pattern ([regex]::Escape($stageSnapshotTarget)) -Description "stage state snapshot target $stageSnapshotTarget"
}

Assert-Contains -Path $algorithm -Pattern 'pub\(crate\) struct HeatBalanceRuntimeConfig' -Description "probe-agnostic runtime config owner"
Assert-Contains -Path $algorithm -Pattern 'pub\(crate\) struct HeatBalanceTimestepAlgorithmFlags' -Description "probe-agnostic timestep flag bundle owner"
Assert-Contains -Path $algorithm -Pattern 'pub\(crate\) const fn runtime_config\s*\(' -Description "compatibility runtime config selector"
foreach ($runtimeConfigField in @(
        'use_third_order_zone_air_correction',
        'preserve_surface_inside_temperature_for_first_longwave',
        'use_weather_air_storage_report',
        'use_balance_surface_convection_report',
        'use_surface_reference_air_surface_convection_report',
        'use_final_inside_convection_report'
    )) {
    Assert-Contains -Path $algorithm -Pattern "pub\(crate\) $runtimeConfigField`: bool" -Description "runtime config field $runtimeConfigField"
}
Assert-Contains -Path $algorithm -Pattern '(?s)Self::SourceOrder1ZoneOpaqueCompat => HeatBalanceRuntimeConfig \{.*use_third_order_zone_air_correction: true.*use_weather_air_storage_report: true.*use_surface_reference_air_surface_convection_report: true' -Description "compatibility selector declares runtime config directly"
Assert-Contains -Path $algorithm -Pattern '(?s)timestep: HeatBalanceTimestepAlgorithmFlags \{.*correct_zone_air_after_surface_pass: true.*interleave_zone_air_surface_passes: true.*use_quick_outside_conduction: true' -Description "compatibility selector declares timestep flags directly"
Assert-Contains -Path $algorithm -Pattern 'pub enum CompatibilityHeatBalanceAlgorithm' -Description "compatibility algorithm enum"
Assert-Contains -Path $algorithm -Pattern 'SourceOrder1ZoneOpaqueCompat' -Description "explicit source-order compatibility selector"
Assert-NotContains -Path $algorithm -Pattern '\bHeatBalanceZoneAirAlgorithm\b' -Description "legacy diagnostic selector in compatibility algorithm module"
Assert-NotContains -Path $algorithm -Pattern 'pub enum DiagnosticHeatBalanceProbe' -Description "diagnostic probe enum in compatibility algorithm module"
Assert-Contains -Path $diagnosticProbe -Pattern 'pub enum DiagnosticHeatBalanceProbe' -Description "diagnostic probe enum"
Assert-Contains -Path $diagnosticProbe -Pattern 'pub enum HeatBalanceZoneAirAlgorithm' -Description "legacy diagnostic selector owner"
Assert-Contains -Path $diagnosticProbe -Pattern 'pub fn from_cli_name\s*\(' -Description "diagnostic selector CLI parser owner"
Assert-Contains -Path $diagnosticProbe -Pattern 'pub const fn cli_name\s*\(' -Description "diagnostic selector CLI name owner"
Assert-Contains -Path $diagnosticProbe -Pattern 'pub\(crate\) const fn runtime_config\s*\(' -Description "diagnostic selector runtime config conversion owner"
Assert-NotContains -Path $diagnosticProbe -Pattern 'pub enum CompatibilityHeatBalanceAlgorithm' -Description "compatibility algorithm enum in diagnostic probe module"
Assert-RustProductionTreeNotContains `
    -Roots @("crates\ep_runtime\src", "crates\ep_run\src", "crates\ep_cli\src") `
    -AllowedPrefixes @("crates/ep_runtime/src/diagnostic_probes/") `
    -Pattern '\bDiagnosticHeatBalanceProbe\b' `
    -Description "diagnostic heat-balance probe type"
Assert-RustProductionTreeNotContains `
    -Roots @("crates\ep_runtime\src", "crates\ep_run\src", "crates\ep_cli\src") `
    -AllowedPrefixes @("crates/ep_runtime/src/diagnostic_probes/") `
    -Pattern '\bEnergyPlus[A-Za-z0-9_]*Probe\b' `
    -Description "diagnostic heat-balance probe variant"
Assert-RustProductionTreeNotContains `
    -Roots @("crates\ep_runtime\src\heat_balance") `
    -AllowedPrefixes @(
        "crates/ep_runtime/src/heat_balance/state.rs",
        "crates/ep_runtime/src/heat_balance/timestep.rs"
    ) `
    -Pattern '\bHeatBalanceZoneAirAlgorithm\b' `
    -Description "legacy heat-balance selector in an algorithm consumer"
$runtimeConfigCallPattern = [regex]::Escape(([char]46).ToString() + 'runtime_config()')
$optionsRuntimeConfigPattern = 'options\.zone_air_algorithm' + $runtimeConfigCallPattern
Assert-Contains -Path $runtime -Pattern ('let heat_balance_runtime_config = ' + $optionsRuntimeConfigPattern) -Description "runtime selection boundary converts once to runtime config"
Assert-Contains -Path $timestep -Pattern ('HeatBalanceZoneAirAlgorithm::SimplifiedAnalytical' + $runtimeConfigCallPattern) -Description "public diagnostic timestep wrapper converts its default selector"
Assert-RustProductionMatchCount -Path $runtime -Pattern $optionsRuntimeConfigPattern -ExpectedCount 1 -Description "runtime selector-to-config conversion"
Assert-RustProductionMatchCount -Path $timestep -Pattern '\bHeatBalanceZoneAirAlgorithm\b' -ExpectedCount 2 -Description "timestep legacy selector allowlist"
Assert-Contains -Path $runPeriod -Pattern 'runtime_config: HeatBalanceRuntimeConfig' -Description "run-period consumer accepts runtime config"
Assert-NotContains -Path $runPeriod -Pattern ('zone_air_algorithm' + $runtimeConfigCallPattern) -Description "run-period consumer converts a legacy selector"
Assert-Contains -Path $cli -Pattern 'HeatBalanceZoneAirAlgorithm::from_cli_name\(value\)' -Description "CLI delegates diagnostic selector parsing"
Assert-Contains -Path $cli -Pattern 'zone_air_algorithm\.cli_name\(\)' -Description "CLI delegates diagnostic selector display names"
Assert-Contains -Path $diagnosticProbe -Pattern 'Diagnostic-only heat-balance selectors and non-claim baselines' -Description "diagnostic probe non-claim boundary"
Assert-Contains -Path $diagnosticProbe -Pattern 'pub struct DiagnosticProbeMetadata' -Description "diagnostic probe metadata"
Assert-Contains -Path $diagnosticProbe -Pattern 'why_it_exists' -Description "diagnostic probe metadata why-it-exists field"
Assert-Contains -Path $diagnosticProbe -Pattern 'mismatch_investigated' -Description "diagnostic probe metadata mismatch field"
Assert-Contains -Path $diagnosticProbe -Pattern 'EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatProbe' -Description "diagnostic probe name includes purpose"
Assert-Contains -Path $algorithm -Pattern 'allows_conformance_promotion' -Description "diagnostic probes cannot promote conformance"
Assert-Contains -Path $runSupport -Pattern 'pub struct SelectedAlgorithmLane' -Description "run support selected algorithm lane metadata"
Assert-Contains -Path $runSupport -Pattern 'diagnostic_probe_used' -Description "run support diagnostic-probe lane flag"
Assert-Contains -Path $runSupport -Pattern 'conformance_promotion_allowed' -Description "run support conformance promotion lane flag"
Assert-Contains -Path $pipeline -Pattern '"selected_algorithm_lane": assessment\.selected_algorithm_lane\.clone\(\)' -Description "run summary selected algorithm lane"
Assert-Contains -Path $pipeline -Pattern 'SelectedAlgorithmLane::none' -Description "early run summary selected algorithm lane"
Assert-Contains -Path $cli -Pattern 'context\.conformance_claim && diagnostic\.diagnostic_probe_used' -Description "conformance gate rejects diagnostic probe lane"
Assert-Contains -Path $runtimeTestSourceOrder -Pattern 'assert!\(!probe\.allows_conformance_promotion\(\)\)' -Description "probe alias not accepted as compatibility algorithm"
Assert-Contains -Path "data\conformance_cases\official_1zone_uncontrolled_dynamic_diagnostic_001\case.toml" -Pattern 'comparison_class = "diagnostic-only"' -Description "diagnostic probe manifest class"
Assert-Contains -Path "data\conformance_cases\official_1zone_uncontrolled_dynamic_diagnostic_001\case.toml" -Pattern 'conformance_claim = false' -Description "diagnostic probe manifest claim boundary"
Assert-Contains -Path $dynamicDiagnosticScript -Pattern 'comparison_class: diagnostic-only' -Description "diagnostic probe report class"
Assert-Contains -Path $dynamicDiagnosticScript -Pattern 'conformance_claim: false' -Description "diagnostic probe report claim boundary"
Assert-Contains -Path "crates\ep_conformance\src\tests.rs" -Pattern 'rejects_diagnostic_case_with_true_conformance_claim' -Description "diagnostic probe outputs cannot become conformance claims"
Assert-Contains -Path "data\conformance_cases\official_1zone_uncontrolled_dynamic_conformance_candidate_001\case.toml" -Pattern 'conformance_claim = true' -Description "probe removal keeps conformance evidence separate"
Assert-Contains -Path $runtimeTestResults -Pattern 'fn heat_balance_trace_writes_zone_air_temperature_results\s*\(' -Description "runtime CTF component identity test"
Assert-NotContains -Path $dynamicDiagnosticScript -Pattern '\$insideComponentSum|\$outsideComponentSum|\$storageFromConduction' -Description "script-owned CTF component identity reconstruction"
Assert-NotContains -Path $dynamicDiagnosticScript -Pattern '\$referenceAirSignedSplitSum|\$referenceAirAbsSplitSum|\$referenceAirCancellation|\$rustInsideHistorySplitSum' -Description "script-owned max-sample source reconstruction"
Assert-NotContains -Path $dynamicDiagnosticScript -Pattern '\$maxSampleInsideSlotSum|\$insideSlotSum|\$outsideSlotSum' -Description "script-owned CTF slot aggregation"
Assert-Contains -Path $runtimeTestResults -Pattern 'inside_conduction_series\.values\[0\]\s*-\s*inside_current_outside_term\.values\[0\]' -Description "runtime inside CTF component identity assertion"
Assert-Contains -Path $runtimeTestResults -Pattern 'outside_conduction_series\.values\[0\]\s*-\s*outside_current_outside_term\.values\[0\]' -Description "runtime outside CTF component identity assertion"
Assert-Contains -Path $runtimeTestResults -Pattern 'storage_series\.values\[0\]\s*\+\s*inside_conduction_series\.values\[0\]\s*\+\s*outside_conduction_series\.values\[0\]' -Description "runtime CTF storage identity assertion"
Assert-Contains -Path $runtimeTestDynamic -Pattern 'let inside_slot_sum = slot_samples' -Description "runtime multi-slot inside CTF aggregation test"
Assert-Contains -Path $runtimeTestDynamic -Pattern 'let outside_slot_sum = slot_samples' -Description "runtime multi-slot outside CTF aggregation test"
Assert-Contains -Path $runtimeTestRadiation -Pattern 'fn approximate_view_factors_match_energyplus_1zone_eio\s*\(' -Description "official 1Zone view-factor runtime test"
Assert-NotContains -Path $dynamicCompatScript -Pattern '\$largestInsideLongwaveArea|\$totalInsideLongwaveArea' -Description "script-owned large-surface view-factor predicate"

Assert-Contains -Path $probeSummaryReport -Pattern 'rusted-energyplus\.dynamic-heat-balance-probe-summary\.v18' -Description "precomputed diagnostic report schema"
Assert-Contains -Path $probeSummaryReport -Pattern 'zone_air_surface_convection_closure_deltas' -Description "report consumes Rust surface-convection closure diagnostics"
Assert-Contains -Path $probeSummaryReport -Pattern 'missing-precomputed-rust-diagnostics' -Description "stale report artifact marker"
Assert-NotContains -Path $probeSummaryReport -Pattern 'def\s+slot_inside_history_temperature_equivalent_delta_c\s*\(' -Description "Python CTF temperature-equivalent physics fallback"
Assert-NotContains -Path $probeSummaryReport -Pattern 'def\s+signed_delta\s*\(' -Description "Python signed CTF source reconstruction helper"
Assert-NotContains -Path $probeSummaryReport -Pattern 'def\s+residual_stats\s*\(' -Description "Python surface-convection closure reconstruction helper"
Assert-NotContains -Path $probeSummaryReport -Pattern 'oracle_reference_air_source_w\s*-\s*rust_reference_air_source_w' -Description "Python reference-air source reconstruction"
Assert-NotContains -Path $probeSummaryReport -Pattern 'oracle_residuals|rust_residuals|delta_residuals' -Description "Python zone surface-convection residual reconstruction"

if ($SelfTest) {
    Invoke-DiagnosticProbeBoundarySelfTest
}

Write-Host "Heat-balance structure audit complete."
