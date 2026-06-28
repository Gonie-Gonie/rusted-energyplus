[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$CaseId = "official_1zone_uncontrolled_dynamic_diagnostic_001"
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$OutputRoot = Join-Path $RepoRoot ".runtime\official-dynamic-warmup-lanes\26.1.0"
$CasePath = Join-Path $RepoRoot "data\conformance_cases\$CaseId\case.toml"
$SummaryPath = Join-Path $OutputRoot "warmup-lane-summary.json"
$ReportPath = Join-Path $OutputRoot "warmup-lane-summary.md"

function Assert-RepoSubPath {
    param([Parameter(Mandatory = $true)][string]$Path)
    $full = [System.IO.Path]::GetFullPath($Path)
    $root = [System.IO.Path]::GetFullPath($RepoRoot)
    if (-not $full.StartsWith($root, [System.StringComparison]::OrdinalIgnoreCase)) {
        throw "Refusing to operate outside repository: $full"
    }
}

function Remove-RepoDirectory {
    param([Parameter(Mandatory = $true)][string]$Path)
    if (Test-Path -LiteralPath $Path) {
        Assert-RepoSubPath -Path $Path
        Remove-Item -LiteralPath $Path -Recurse -Force
    }
}

function Get-SeriesRow {
    param(
        [Parameter(Mandatory = $true)]$Summary,
        [Parameter(Mandatory = $true)][string]$Key,
        [Parameter(Mandatory = $true)][string]$Variable
    )
    @($Summary.series | Where-Object { $_.output.key -eq $Key -and $_.output.variable -eq $Variable -and $_.status -eq "extracted" }) | Select-Object -First 1
}

function Get-FirstDelta {
    param($SeriesRow)
    if (-not $SeriesRow) {
        return [double]::NaN
    }
    $rows = @($SeriesRow.sample_rows)
    if ($rows.Count -eq 0) {
        return [double]::NaN
    }
    [double]$rows[0].abs_delta_c
}

function Get-ScalarDelta {
    param($Row)
    if ($null -eq $Row) {
        return [double]::NaN
    }
    [double]$Row.delta
}

function Assert-Finite {
    param(
        [Parameter(Mandatory = $true)][double]$Value,
        [Parameter(Mandatory = $true)][string]$Description
    )
    if ([double]::IsNaN($Value) -or [double]::IsInfinity($Value)) {
        throw "$Description is not finite"
    }
}

foreach ($path in @(
    (Join-Path $OracleRoot "energyplus.exe"),
    (Join-Path $OracleRoot "ConvertInputFormat.exe"),
    (Join-Path $OracleRoot "WeatherData\USA_CO_Golden-NREL.724666_TMY3.epw"),
    $CasePath
)) {
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Missing required official dynamic warmup-lane file: $path"
    }
}

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Remove-RepoDirectory -Path $OutputRoot
New-Item -ItemType Directory -Force -Path $OutputRoot | Out-Null

$envNames = @(
    "RUSTED_ENERGYPLUS_HEAT_BALANCE_CTF_SEED_POLICY",
    "RUSTED_ENERGYPLUS_HEAT_BALANCE_CTF_INITIAL_HISTORY_POLICY",
    "RUSTED_ENERGYPLUS_HEAT_BALANCE_ZONE_AIR_ALGORITHM",
    "RUSTED_ENERGYPLUS_HEAT_BALANCE_WARMUP_MINIMUM_DAYS",
    "RUSTED_ENERGYPLUS_HEAT_BALANCE_SURFACE_ITERATIONS",
    "RUSTED_ENERGYPLUS_HEAT_BALANCE_SURFACE_LOOP_ZONE_AIR_CORRECTION"
)
$previousEnv = @{}
foreach ($name in $envNames) {
    $previousEnv[$name] = [Environment]::GetEnvironmentVariable($name, "Process")
}

$lanes = @(
    [pscustomobject]@{
        id = "no-warmup"
        label = "No warmup diagnostic lane"
        warmup_env = "disabled"
    },
    [pscustomobject]@{
        id = "building-convergence"
        label = "EnergyPlus-style Building convergence lane"
        warmup_env = $null
    },
    [pscustomobject]@{
        id = "fixed-20"
        label = "Fixed 20-day warmup lane"
        warmup_env = "20"
    }
)

$laneResults = @()
try {
    foreach ($lane in $lanes) {
        Write-Host "Running warmup lane $($lane.id)."
        [Environment]::SetEnvironmentVariable("RUSTED_ENERGYPLUS_HEAT_BALANCE_CTF_SEED_POLICY", "all-eio", "Process")
        [Environment]::SetEnvironmentVariable("RUSTED_ENERGYPLUS_HEAT_BALANCE_CTF_INITIAL_HISTORY_POLICY", "energyplus-surf-initial", "Process")
        [Environment]::SetEnvironmentVariable("RUSTED_ENERGYPLUS_HEAT_BALANCE_ZONE_AIR_ALGORITHM", "energyplus-heat-balance-compat-candidate", "Process")
        [Environment]::SetEnvironmentVariable("RUSTED_ENERGYPLUS_HEAT_BALANCE_SURFACE_ITERATIONS", "20", "Process")
        [Environment]::SetEnvironmentVariable("RUSTED_ENERGYPLUS_HEAT_BALANCE_SURFACE_LOOP_ZONE_AIR_CORRECTION", "after-surface-loop", "Process")
        [Environment]::SetEnvironmentVariable("RUSTED_ENERGYPLUS_HEAT_BALANCE_WARMUP_MINIMUM_DAYS", $lane.warmup_env, "Process")

        $laneOutputRoot = Join-Path $OutputRoot $lane.id
        $output = & $cargo.Source run -p ep_cli --quiet -- conformance heat-balance-diagnostic-report $CasePath $OracleRoot $laneOutputRoot 2>&1
        if ($LASTEXITCODE -ne 0) {
            $output | ForEach-Object { Write-Host $_ }
            throw "Warmup lane $($lane.id) failed."
        }

        $compareRoot = Join-Path (Join-Path $laneOutputRoot $CaseId) "compare"
        $compareSummaryPath = Join-Path $compareRoot "compare-summary.json"
        $compareReportPath = Join-Path $compareRoot "compare-report.md"
        if (-not (Test-Path -LiteralPath $compareSummaryPath -PathType Leaf)) {
            throw "Warmup lane $($lane.id) did not produce compare-summary.json"
        }
        $summary = Get-Content -LiteralPath $compareSummaryPath -Raw | ConvertFrom-Json

        $mat = Get-SeriesRow -Summary $summary -Key "ZONE ONE" -Variable "Zone Mean Air Temperature"
        $floorInsideTemp = Get-SeriesRow -Summary $summary -Key "ZN001:FLR001" -Variable "Surface Inside Face Temperature"
        $floorStorage = Get-SeriesRow -Summary $summary -Key "ZN001:FLR001" -Variable "Surface Heat Storage Rate"
        $floorStorageFlux = Get-SeriesRow -Summary $summary -Key "ZN001:FLR001" -Variable "Surface Heat Storage Rate per Area"
        $floorInsideCurrent = @($summary.floor_inside_current_diagnostics | Where-Object { $_.key -eq "ZN001:FLR001" }) | Select-Object -First 1

        $laneResults += [pscustomobject]@{
            id = $lane.id
            label = $lane.label
            warmup_env = $lane.warmup_env
            compare_summary_json = $compareSummaryPath
            compare_report_md = $compareReportPath
            warmup_enabled = [bool]$summary.heat_balance_warmup.enabled
            warmup_minimum_days = [int]$summary.heat_balance_warmup.minimum_days
            warmup_maximum_days = [int]$summary.heat_balance_warmup.maximum_days
            warmup_day_count = [int]$summary.heat_balance_warmup.day_count
            warmup_converged = [bool]$summary.heat_balance_warmup.converged
            warmup_final_mat_delta_c = [double]$summary.heat_balance_warmup.final_max_zone_temperature_delta_c
            oracle_run_period_warmup_days = $summary.heat_balance_warmup.oracle_run_period_day_count
            warmup_day_count_delta = $summary.heat_balance_warmup.day_count_delta
            first_run_period_mat_delta_c = Get-FirstDelta -SeriesRow $mat
            first_run_period_floor_inside_temp_delta_c = Get-FirstDelta -SeriesRow $floorInsideTemp
            first_run_period_floor_storage_delta_w = Get-FirstDelta -SeriesRow $floorStorage
            floor_storage_rmse_delta_w = if ($floorStorage) { [double]$floorStorage.rmse_delta_c } else { [double]::NaN }
            floor_storage_max_delta_w = if ($floorStorage) { [double]$floorStorage.max_abs_delta_c } else { [double]::NaN }
            floor_storage_flux_rmse_delta_w_per_m2 = if ($floorStorageFlux) { [double]$floorStorageFlux.rmse_delta_c } else { [double]::NaN }
            warmup_end_surface_temperature_delta_c = Get-ScalarDelta -Row $summary.warmup_end_state_deltas.surface_temperature
            warmup_end_ctf_history_delta_w = Get-ScalarDelta -Row $summary.warmup_end_state_deltas.ctf_history
            warmup_end_zone_history_delta_w = Get-ScalarDelta -Row $summary.warmup_end_state_deltas.zone_history
            floor_inside_current_classification = if ($floorInsideCurrent) { $floorInsideCurrent.current_inside_mismatch_classification } else { "missing" }
            floor_inside_current_delta_w = if ($floorInsideCurrent) { [double]$floorInsideCurrent.inside_current_inside_term_delta_w } else { [double]::NaN }
        }
    }
}
finally {
    foreach ($name in $envNames) {
        [Environment]::SetEnvironmentVariable($name, $previousEnv[$name], "Process")
    }
}

if ($laneResults.Count -ne 3) {
    throw "Expected three warmup lane results, got $($laneResults.Count)"
}
$noWarmup = $laneResults | Where-Object { $_.id -eq "no-warmup" } | Select-Object -First 1
$convergence = $laneResults | Where-Object { $_.id -eq "building-convergence" } | Select-Object -First 1
$fixed20 = $laneResults | Where-Object { $_.id -eq "fixed-20" } | Select-Object -First 1
if ($noWarmup.warmup_enabled) {
    throw "No-warmup lane unexpectedly enabled warmup"
}
if (-not $convergence.warmup_enabled) {
    throw "Building convergence lane did not enable warmup"
}
if (-not $fixed20.warmup_enabled -or $fixed20.warmup_day_count -lt 20) {
    throw "Fixed 20-day warmup lane did not run at least 20 days"
}
foreach ($lane in $laneResults) {
    Assert-Finite -Value ([double]$lane.first_run_period_mat_delta_c) -Description "$($lane.id) first run-period MAT delta"
    Assert-Finite -Value ([double]$lane.first_run_period_floor_inside_temp_delta_c) -Description "$($lane.id) first run-period floor inside temperature delta"
    Assert-Finite -Value ([double]$lane.first_run_period_floor_storage_delta_w) -Description "$($lane.id) first run-period floor storage delta"
    Assert-Finite -Value ([double]$lane.floor_storage_rmse_delta_w) -Description "$($lane.id) floor storage RMSE delta"
    Assert-Finite -Value ([double]$lane.warmup_end_zone_history_delta_w) -Description "$($lane.id) warmup end zone history delta"
}

$summaryRows = foreach ($lane in $laneResults) {
    $reduction = [double]$noWarmup.floor_storage_rmse_delta_w - [double]$lane.floor_storage_rmse_delta_w
    $warmupDominant = ([double]$lane.warmup_end_surface_temperature_delta_c -gt 0.01) -or ([double]$lane.warmup_end_ctf_history_delta_w -gt 1.0) -or ([double]$lane.warmup_end_zone_history_delta_w -gt 1.0)
    [pscustomobject]@{
        id = $lane.id
        label = $lane.label
        warmup_enabled = $lane.warmup_enabled
        warmup_minimum_days = $lane.warmup_minimum_days
        warmup_maximum_days = $lane.warmup_maximum_days
        warmup_day_count = $lane.warmup_day_count
        warmup_converged = $lane.warmup_converged
        warmup_final_mat_delta_c = $lane.warmup_final_mat_delta_c
        warmup_zone_extrema_delta_c = $lane.warmup_final_mat_delta_c
        oracle_run_period_warmup_days = $lane.oracle_run_period_warmup_days
        warmup_day_count_delta = $lane.warmup_day_count_delta
        first_run_period_mat_delta_c = $lane.first_run_period_mat_delta_c
        first_run_period_floor_inside_temp_delta_c = $lane.first_run_period_floor_inside_temp_delta_c
        first_run_period_floor_storage_delta_w = $lane.first_run_period_floor_storage_delta_w
        floor_storage_rmse_delta_w = $lane.floor_storage_rmse_delta_w
        floor_storage_rmse_reduction_vs_no_warmup_w = $reduction
        floor_storage_max_delta_w = $lane.floor_storage_max_delta_w
        floor_storage_flux_rmse_delta_w_per_m2 = $lane.floor_storage_flux_rmse_delta_w_per_m2
        warmup_end_surface_temperature_delta_c = $lane.warmup_end_surface_temperature_delta_c
        warmup_end_ctf_history_delta_w = $lane.warmup_end_ctf_history_delta_w
        warmup_end_zone_history_delta_w = $lane.warmup_end_zone_history_delta_w
        floor_inside_current_classification = $lane.floor_inside_current_classification
        floor_inside_current_delta_w = $lane.floor_inside_current_delta_w
        warmup_mismatch_dominant = $warmupDominant
        source_order_guidance = if ($warmupDominant) { "resolve-warmup-handoff-before-surface-equation" } else { "warmup-end-state-close-check-source-order" }
        compare_summary_json = $lane.compare_summary_json
        compare_report_md = $lane.compare_report_md
    }
}

$summaryObject = [pscustomobject]@{
    schema_version = 1
    case_id = $CaseId
    oracle_root = $OracleRoot
    lanes = @($summaryRows)
}
$summaryObject | ConvertTo-Json -Depth 12 | Set-Content -LiteralPath $SummaryPath -Encoding UTF8

$report = New-Object System.Text.StringBuilder
[void]$report.AppendLine("# Official Dynamic Warmup Lane Summary")
[void]$report.AppendLine("")
[void]$report.AppendLine("| lane | enabled | days | converged | first_mat_delta_c | first_floor_temp_delta_c | first_floor_storage_delta_w | floor_storage_rmse_w | reduction_vs_no_warmup_w | warmup_zone_extrema_delta_c | warmup_surface_delta_c | warmup_ctf_delta_w | warmup_zone_history_delta_w | guidance |")
[void]$report.AppendLine("|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|")
foreach ($lane in $summaryRows) {
    [void]$report.AppendLine((
        "| {0} | {1} | {2} | {3} | {4:F12} | {5:F12} | {6:F12} | {7:F12} | {8:F12} | {9:F12} | {10:F12} | {11:F12} | {12:F12} | {13} |" -f
        $lane.id,
        $lane.warmup_enabled,
        $lane.warmup_day_count,
        $lane.warmup_converged,
        [double]$lane.first_run_period_mat_delta_c,
        [double]$lane.first_run_period_floor_inside_temp_delta_c,
        [double]$lane.first_run_period_floor_storage_delta_w,
        [double]$lane.floor_storage_rmse_delta_w,
        [double]$lane.floor_storage_rmse_reduction_vs_no_warmup_w,
        [double]$lane.warmup_zone_extrema_delta_c,
        [double]$lane.warmup_end_surface_temperature_delta_c,
        [double]$lane.warmup_end_ctf_history_delta_w,
        [double]$lane.warmup_end_zone_history_delta_w,
        $lane.source_order_guidance
    ))
}
[void]$report.AppendLine("")
[void]$report.AppendLine("Artifacts:")
foreach ($lane in $summaryRows) {
    [void]$report.AppendLine("- $($lane.id): $($lane.compare_summary_json)")
}
Set-Content -LiteralPath $ReportPath -Value $report.ToString() -Encoding UTF8

Write-Host "Warmup lane summary written: $SummaryPath"
Write-Host "Warmup lane report written: $ReportPath"
