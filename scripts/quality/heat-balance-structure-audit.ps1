[CmdletBinding()]
param()

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
$diagnosticProbe = "crates\ep_runtime\src\diagnostic_probes\heat_balance.rs"
$executionPlan = "crates\ep_runtime\src\execution_plan.rs"
$precompute = "crates\ep_runtime\src\precompute.rs"
$pipeline = "crates\ep_run\src\pipeline.rs"
$runSupport = "crates\ep_run\src\support.rs"
$cli = "crates\ep_cli\src\main.rs"
$runtime = "crates\ep_runtime\src\runtime.rs"
$weather = "crates\ep_runtime\src\weather.rs"
$schedules = "crates\ep_runtime\src\schedules.rs"
$runtimeTestSourceOrder = "crates\ep_runtime\src\runtime\tests\part01.rs"
$runtimeTestDynamic = "crates\ep_runtime\src\runtime\tests\part02.rs"

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
        @($diagnosticProbe, "diagnostic probe selector module"),
        @($executionPlan, "execution plan module"),
        @($precompute, "runtime precompute module"),
        @($pipeline, "arbitrary-run pipeline"),
        @($runSupport, "arbitrary-run support assessment"),
        @($cli, "CLI conformance gate"),
        @($runtime, "runtime orchestration root"),
        @($weather, "runtime weather module"),
        @($schedules, "runtime schedules module"),
        @($runtimeTestSourceOrder, "runtime source-order tests"),
        @($runtimeTestDynamic, "runtime dynamic heat-balance tests")
    )) {
    Assert-FileExists -Path $entry[0] -Description $entry[1]
}

Assert-LineLimit -Path $algorithm -Limit 650 -Description "heat-balance algorithm selector module"
Assert-LineLimit -Path $manager -Limit 180 -Description "HeatBalanceManager source-order module"
Assert-LineLimit -Path $surfaceManager -Limit 240 -Description "HeatBalanceSurfaceManager source-order module"
Assert-LineLimit -Path $surfaceBalance -Limit 720 -Description "surface balance ownership module"
Assert-LineLimit -Path $surfaceBoundary -Limit 280 -Description "surface boundary ownership module"
Assert-LineLimit -Path $surfaceLoop -Limit 420 -Description "surface loop ownership module"
Assert-LineLimit -Path $airManager -Limit 260 -Description "HeatBalanceAirManager source-order module"
Assert-LineLimit -Path $zonePredictorCorrector -Limit 240 -Description "ZoneTempPredictorCorrector source-order module"
Assert-LineLimit -Path $zoneAirCorrection -Limit 520 -Description "zone-air correction ownership module"
Assert-LineLimit -Path $ctf -Limit 800 -Description "CTF ownership module"
Assert-LineLimit -Path $insideConvection -Limit 360 -Description "inside convection ownership module"
Assert-LineLimit -Path $initialization -Limit 220 -Description "heat-balance initialization ownership module"
Assert-LineLimit -Path $convection -Limit 420 -Description "convection ownership module"
Assert-LineLimit -Path $longwave -Limit 180 -Description "exterior longwave ownership module"
Assert-LineLimit -Path $radiation -Limit 800 -Description "radiation ownership module"
Assert-LineLimit -Path $solar -Limit 760 -Description "solar radiation ownership module"
Assert-LineLimit -Path $reports -Limit 820 -Description "report ownership module"
Assert-LineLimit -Path $state -Limit 800 -Description "heat-balance state ownership module"
Assert-LineLimit -Path $runPeriod -Limit 800 -Description "run-period sampling ownership module"
Assert-LineLimit -Path $trace -Limit 780 -Description "heat-balance trace ownership module"
Assert-LineLimit -Path $summary -Limit 140 -Description "heat-balance summary ownership module"
Assert-LineLimit -Path $warmup -Limit 180 -Description "warmup ownership module"
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
Assert-Contains -Path $airManager -Pattern 'weather_context_zone_air_heat_capacity_j_per_k' -Description "weather-driven zone-air heat capacity owner"
Assert-Contains -Path $airManager -Pattern 'update_zone_air_heat_capacities_from_weather_context' -Description "zone-air weather capacity updater owner"
Assert-Contains -Path $airManager -Pattern 'seed_zone_air_humidity_ratios_from_weather_records' -Description "zone-air weather humidity seeding owner"
Assert-Contains -Path $airManager -Pattern 'zone_air_heat_balance_air_storage_rate_w' -Description "zone-air storage report owner"
Assert-NotContains -Path $runtime -Pattern 'fn weather_context_zone_air_heat_capacity_j_per_k\s*\(' -Description "runtime-owned zone-air weather capacity implementation"
Assert-NotContains -Path $runtime -Pattern 'fn update_zone_air_heat_capacities_from_weather_context\s*\(' -Description "runtime-owned zone-air weather capacity updater"
Assert-NotContains -Path $runtime -Pattern 'fn seed_zone_air_humidity_ratios_from_weather_records\s*\(' -Description "runtime-owned weather humidity seeding"
Assert-NotContains -Path $runtime -Pattern 'fn zone_air_heat_balance_air_storage_rate_w\s*\(' -Description "runtime-owned zone-air storage report implementation"
Assert-Contains -Path $zonePredictorCorrector -Pattern 'ManageZoneAirUpdates' -Description "ZoneTempPredictorCorrector routine"
Assert-Contains -Path $zonePredictorCorrector -Pattern 'manage_zone_air_updates_stage\s*\(' -Description "ZoneTempPredictorCorrector ledger target manage_zone_air_updates_stage"
Assert-Contains -Path $zonePredictorCorrector -Pattern 'manage_zone_air_updates_source_order_path(?:<[^>]+>)?\s*\(' -Description "ZoneTempPredictorCorrector ManageZoneAirUpdates source-order wrapper"
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
Assert-Contains -Path $surfaceManager -Pattern 'pub\(crate\) type ConstructionThermalData' -Description "construction thermal data cache type"
Assert-Contains -Path $initialization -Pattern 'construction_thermal_data' -Description "construction thermal data cached during heat-balance initialization"
Assert-Contains -Path $initialization -Pattern 'construction_ctf_coefficients_by_name' -Description "CTF coefficient cache initialized by construction"
Assert-Contains -Path $weather -Pattern 'pub struct WeatherTimestepSeries' -Description "precomputed weather timestep series"
Assert-Contains -Path $weather -Pattern 'pub fn precompute_weather_timestep_series' -Description "weather timestep precompute entry"
Assert-Contains -Path $runtime -Pattern 'precompute_weather_timestep_series' -Description "heat-balance runtime uses weather timestep precompute"
Assert-Contains -Path $schedules -Pattern 'pub type ScheduleValueSeries' -Description "precomputed schedule value series type"
Assert-Contains -Path $schedules -Pattern 'pub fn precompute_schedule_value_series' -Description "schedule value precompute entry"
Assert-Contains -Path $precompute -Pattern 'pub struct RuntimePrecomputedData' -Description "runtime precomputed data bundle"
Assert-Contains -Path $precompute -Pattern 'output_registry: RuntimeOutputRegistry' -Description "run cached output registry"
Assert-Contains -Path $precompute -Pattern 'build_execution_plan_with_output_registry' -Description "execution plan uses cached output registry"
Assert-Contains -Path $executionPlan -Pattern 'pub fn build_execution_plan_with_output_registry' -Description "execution plan cache-aware builder"
Assert-Contains -Path $pipeline -Pattern 'precompute_runtime_data' -Description "pipeline caches runtime precomputed data"
Assert-Contains -Path $pipeline -Pattern '"output_registry_count": precomputed\.output_registry\.len\(\)' -Description "execution-plan artifact records cached output registry"
Assert-Contains -Path $initialization -Pattern 'HeatBalanceSurfaceIndexes::from_model_surfaces' -Description "heat-balance surface indexes initialized once"
Assert-Contains -Path $zoneAirCorrection -Pattern 'HeatBalanceSurfaceIndexes' -Description "zone-air correction consumes precomputed surface indexes"
Assert-Contains -Path $timestep -Pattern 'state\.surface_indexes\.surfaces_for_zone' -Description "timestep hot path uses precomputed zone surface indexes"
Assert-Contains -Path $reports -Pattern 'zone_surface_report_conduction_rates_for_indices_w' -Description "zone conduction report uses precomputed surface indexes"
Assert-Contains -Path $radiation -Pattern 'HeatBalanceSurfaceIndexes' -Description "radiation loop consumes precomputed surface indexes"
Assert-NotContains -Path $radiation -Pattern 'BTreeMap::<ZoneId, Vec<usize>>::new' -Description "radiation-owned per-call zone surface grouping"
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
Assert-Contains -Path $timestep -Pattern 'air_manager::manage_air_heat_balance_source_order_path' -Description "timestep enters ManageAirHeatBalance source-order wrapper"
Assert-Contains -Path $timestep -Pattern 'zone_predictor_corrector::manage_zone_air_updates_source_order_path' -Description "timestep enters ManageZoneAirUpdates source-order wrapper"
Assert-Contains -Path $timestep -Pattern 'zone_predictor_corrector::predict_step_source_order_path' -Description "timestep enters PredictStep source-order wrapper"
Assert-Contains -Path $timestep -Pattern 'zone_predictor_corrector::correct_step_source_order_path' -Description "timestep enters CorrectStep source-order wrapper"
Assert-Contains -Path $timestep -Pattern 'surface_manager::update_final_surface_heat_balance_source_order_path' -Description "timestep enters UpdateFinalSurfaceHeatBalance source-order wrapper"
Assert-Contains -Path $timestep -Pattern 'surface_manager::update_thermal_histories_source_order_path' -Description "timestep enters UpdateThermalHistories source-order wrapper"
Assert-Contains -Path $runPeriod -Pattern 'surface_manager::report_surface_heat_balance_source_order_path' -Description "run-period enters ReportSurfaceHeatBalance source-order wrapper"
Assert-Contains -Path $zoneAirCorrection -Pattern 'revert_zone_timestep_histories_source_order_path' -Description "adaptive zone-air correction enters RevertZoneTimestepHistories source-order wrapper"
Assert-Contains -Path $zoneAirCorrection -Pattern 'push_system_timestep_histories_source_order_path' -Description "adaptive zone-air correction enters PushSystemTimestepHistories source-order wrapper"
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

Assert-Contains -Path $algorithm -Pattern 'heat_balance_uses_third_order_zone_air_correction' -Description "third-order zone-air flag owner"
Assert-Contains -Path $algorithm -Pattern 'heat_balance_preserves_surface_inside_temperature_for_first_longwave' -Description "first-longwave inside-temperature preservation flag owner"
Assert-Contains -Path $algorithm -Pattern 'HeatBalanceTimestepAlgorithmFlags' -Description "timestep algorithm flag bundle owner"
Assert-Contains -Path $algorithm -Pattern 'heat_balance_timestep_algorithm_flags' -Description "timestep algorithm flag selector owner"
Assert-Contains -Path $algorithm -Pattern 'heat_balance_uses_weather_air_storage_report' -Description "weather air-storage report flag owner"
Assert-Contains -Path $algorithm -Pattern 'heat_balance_uses_balance_surface_convection_report' -Description "balance surface convection report flag owner"
Assert-Contains -Path $algorithm -Pattern 'heat_balance_uses_surface_reference_air_surface_convection_report' -Description "surface reference-air convection report flag owner"
Assert-Contains -Path $algorithm -Pattern 'heat_balance_uses_final_inside_convection_report' -Description "final inside convection report flag owner"
Assert-NotContains -Path $runtime -Pattern 'fn heat_balance_uses_third_order_zone_air_correction\s*\(' -Description "runtime-owned third-order zone-air flag"
Assert-NotContains -Path $runtime -Pattern 'fn heat_balance_preserves_surface_inside_temperature_for_first_longwave\s*\(' -Description "runtime-owned first-longwave preservation flag"
Assert-NotContains -Path $runtime -Pattern 'fn heat_balance_uses_weather_air_storage_report\s*\(' -Description "runtime-owned weather air-storage report flag"
Assert-NotContains -Path $runtime -Pattern 'fn heat_balance_uses_previous_mat_surface_convection_report\s*\(' -Description "runtime-owned previous-MAT convection report flag"
Assert-NotContains -Path $runtime -Pattern 'fn heat_balance_uses_balance_surface_convection_report\s*\(' -Description "runtime-owned balance convection report flag"
Assert-NotContains -Path $runtime -Pattern 'fn heat_balance_uses_surface_reference_air_convection_report\s*\(' -Description "runtime-owned surface reference-air report flag"
Assert-NotContains -Path $runtime -Pattern 'fn heat_balance_uses_surface_reference_air_surface_convection_report\s*\(' -Description "runtime-owned surface reference-air convection report flag"
Assert-NotContains -Path $runtime -Pattern 'fn heat_balance_uses_final_inside_convection_report\s*\(' -Description "runtime-owned final inside convection report flag"
Assert-Contains -Path $algorithm -Pattern 'pub enum CompatibilityHeatBalanceAlgorithm' -Description "compatibility algorithm enum"
Assert-Contains -Path $algorithm -Pattern 'pub enum HeatBalanceZoneAirSelection' -Description "typed heat-balance selection enum"
Assert-Contains -Path $algorithm -Pattern 'EnergyPlusSourceOrder1ZoneOpaqueCompatibility' -Description "explicit source-order selector"
Assert-NotContains -Path $algorithm -Pattern 'CompatibilityHeatBalanceAlgorithm::SourceOrder1ZoneOpaqueCompat =>\s*\{\s*HeatBalanceZoneAirAlgorithm::EnergyPlusHeatBalanceCompatCandidate' -Description "compatibility selector mapped to legacy candidate alias"
Assert-NotContains -Path $algorithm -Pattern 'pub enum DiagnosticHeatBalanceProbe' -Description "diagnostic probe enum in compatibility algorithm module"
Assert-Contains -Path $diagnosticProbe -Pattern 'pub enum DiagnosticHeatBalanceProbe' -Description "diagnostic probe enum"
Assert-NotContains -Path $diagnosticProbe -Pattern 'pub enum CompatibilityHeatBalanceAlgorithm' -Description "compatibility algorithm enum in diagnostic probe module"
Assert-Contains -Path $diagnosticProbe -Pattern 'HeatBalanceZoneAirSelection::Diagnostic' -Description "diagnostic selectors remain diagnostic"
Assert-Contains -Path $diagnosticProbe -Pattern 'Diagnostic-only heat-balance probes and non-claim baselines' -Description "diagnostic probe non-claim boundary"
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
Assert-Contains -Path "scripts\compare\official-dynamic-heat-balance-diagnostic.ps1" -Pattern 'comparison_class: diagnostic-only' -Description "diagnostic probe report class"
Assert-Contains -Path "scripts\compare\official-dynamic-heat-balance-diagnostic.ps1" -Pattern 'conformance_claim: false' -Description "diagnostic probe report claim boundary"
Assert-Contains -Path "crates\ep_conformance\src\tests.rs" -Pattern 'rejects_diagnostic_case_with_true_conformance_claim' -Description "diagnostic probe outputs cannot become conformance claims"
Assert-Contains -Path "data\conformance_cases\official_1zone_uncontrolled_dynamic_conformance_candidate_001\case.toml" -Pattern 'conformance_claim = true' -Description "probe removal keeps conformance evidence separate"

Write-Host "Heat-balance structure audit complete."
