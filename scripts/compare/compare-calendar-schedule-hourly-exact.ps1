[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$OutputRoot = Join-Path $RepoRoot ".runtime\time-weather-schedule-conformance\26.1.0"
$CaseId = "calendar_schedule_hourly_exact_001"
$CasePath = Join-Path $RepoRoot "data\conformance_cases\$CaseId\case.toml"
$WeatherPath = Join-Path $RepoRoot "data\conformance_cases\$CaseId\calendar_schedule_hourly_exact.epw"
$CaseOutputRoot = Join-Path $OutputRoot $CaseId
$CompareRoot = Join-Path $CaseOutputRoot "compare"
$ExpectedFirstTimestamp = "env=LEAP DAY EXACT RUN PERIOD;day=1;month=2;date=28;dst=0;hour=1;start=0.00;end=60.00;day_type=Sunday"
$ExpectedLastTimestamp = "env=LEAP DAY EXACT RUN PERIOD;day=3;month=3;date=1;dst=0;hour=24;start=0.00;end=60.00;day_type=Tuesday"

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

function Assert-Contains {
    param(
        [Parameter(Mandatory = $true)][string]$Text,
        [Parameter(Mandatory = $true)][string]$Pattern,
        [Parameter(Mandatory = $true)][string]$Description
    )
    if ($Text -notmatch [regex]::Escape($Pattern)) {
        Write-Host $Text
        throw "Missing $Description`: $Pattern"
    }
    Write-Host "OK $Description`: $Pattern"
}

function Assert-FileExists {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Description
    )
    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        throw "Missing $Description`: $Path"
    }
    Write-Host "OK $Description`: $Path"
}

foreach ($path in @(
    (Join-Path $OracleRoot "energyplus.exe"),
    (Join-Path $OracleRoot "ConvertInputFormat.exe"),
    $WeatherPath,
    $CasePath
)) {
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Missing required calendar schedule exact file: $path"
    }
}

Remove-RepoDirectory -Path $CaseOutputRoot

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Write-Host "Running ordered-exact-unique hourly calendar and Schedule:Compact gate."
$output = & $cargo.Source run -p ep_cli --quiet -- conformance time-weather-schedule-report $CasePath $OracleRoot $OutputRoot 2>&1
if ($LASTEXITCODE -ne 0) {
    $output | ForEach-Object { Write-Host $_ }
    throw "Calendar schedule hourly exact gate failed."
}

$text = ($output -join "`n")
Assert-Contains -Text $text -Pattern "Time, Weather, and Schedule Conformance Report" -Description "report header"
Assert-Contains -Text $text -Pattern "id: $CaseId" -Description "case id"
Assert-Contains -Text $text -Pattern "comparison_class: conformance" -Description "comparison class"
Assert-Contains -Text $text -Pattern "conformance_claim: true" -Description "conformance claim"
Assert-Contains -Text $text -Pattern "conformance_series: 1" -Description "conformance series count"
Assert-Contains -Text $text -Pattern "status: pass" -Description "gate status"

$summaryPath = Join-Path $CompareRoot "compare-summary.json"
$reportPath = Join-Path $CompareRoot "compare-report.md"
Assert-FileExists -Path $summaryPath -Description "calendar schedule exact summary"
Assert-FileExists -Path $reportPath -Description "calendar schedule exact report"

$summary = Get-Content -LiteralPath $summaryPath -Raw | ConvertFrom-Json
if ($summary.case_id -ne $CaseId) {
    throw "Unexpected case_id: $($summary.case_id)"
}
if ($summary.comparison_class -ne "conformance") {
    throw "Unexpected comparison_class: $($summary.comparison_class)"
}
if ($summary.conformance_claim -ne $true) {
    throw "Calendar schedule exact summary must claim conformance for this gated case"
}
if ($summary.status -ne "pass") {
    throw "Unexpected calendar schedule exact status: $($summary.status)"
}
if ($summary.time_axis_samples -ne 72) {
    throw "Unexpected calendar time-axis sample count: $($summary.time_axis_samples)"
}
if ($summary.series_count -ne 1) {
    throw "Unexpected series_count: $($summary.series_count)"
}
if ($summary.conformance_series_count -ne 1) {
    throw "Unexpected conformance_series_count: $($summary.conformance_series_count)"
}

$series = $summary.series | Where-Object {
    $_.key -eq "CALENDAR HOURLY 1 TO 24" -and $_.variable -eq "Schedule Value"
}
if ($null -eq $series) {
    throw "Missing Calendar Hourly 1 To 24 Schedule Value series"
}
if ($series.level -ne "conformance") {
    throw "Unexpected Schedule Value level: $($series.level)"
}
if ($series.alignment -ne "timestamp") {
    throw "Schedule Value must use timestamp alignment"
}
if ($series.timestamp_contract -ne "ordered-exact-unique") {
    throw "Unexpected timestamp_contract: $($series.timestamp_contract)"
}
if ($series.expected_samples -ne 72 -or $series.observed_samples -ne 72 -or $series.compared_samples -ne 72) {
    throw "Timestamp/value sample counts must all equal 72: expected=$($series.expected_samples), observed=$($series.observed_samples), compared=$($series.compared_samples)"
}
if ($series.timestamp_expected_unique -ne $true) {
    throw "EnergyPlus timestamps must be unique"
}
if ($series.timestamp_observed_unique -ne $true) {
    throw "Rust timestamps must be unique"
}
if ($series.timestamp_order_match -ne $true) {
    throw "EnergyPlus and Rust timestamps must match exactly in file order"
}
if ($series.timestamp_status -ne "pass") {
    throw "Unexpected timestamp contract status: $($series.timestamp_status)"
}
if ($null -ne $series.first_timestamp_divergence) {
    throw "Timestamp sequence diverged: $($series.first_timestamp_divergence | ConvertTo-Json -Compress)"
}
if ($series.expected_first_timestamp -ne $ExpectedFirstTimestamp) {
    throw "Unexpected EnergyPlus first timestamp: $($series.expected_first_timestamp)"
}
if ($series.observed_first_timestamp -ne $ExpectedFirstTimestamp) {
    throw "Unexpected Rust first timestamp: $($series.observed_first_timestamp)"
}
if ($series.expected_last_timestamp -ne $ExpectedLastTimestamp) {
    throw "Unexpected EnergyPlus last timestamp: $($series.expected_last_timestamp)"
}
if ($series.observed_last_timestamp -ne $ExpectedLastTimestamp) {
    throw "Unexpected Rust last timestamp: $($series.observed_last_timestamp)"
}
if ($series.max_abs_tolerance -ne 0.0 -or $series.max_rel_tolerance -ne 0.0 -or $series.max_rmse_tolerance -ne 0.0) {
    throw "Calendar schedule exact case must use zero absolute, relative, and RMSE tolerances"
}
if ($series.max_abs_delta -ne 0.0) {
    throw "Schedule max_abs_delta must be exactly zero: $($series.max_abs_delta)"
}
if ($series.rmse_delta -ne 0.0) {
    throw "Schedule rmse_delta must be exactly zero: $($series.rmse_delta)"
}
if ($series.max_rel_delta -ne 0.0) {
    throw "Schedule max_rel_delta must be exactly zero: $($series.max_rel_delta)"
}
if ($series.status -ne "pass") {
    throw "Unexpected Schedule Value status: $($series.status)"
}

$reportText = Get-Content -LiteralPath $reportPath -Raw
Assert-Contains -Text $reportText -Pattern "Time, Weather, and Schedule Conformance Report" -Description "markdown report header"
Assert-Contains -Text $reportText -Pattern "gate_blocking: true" -Description "markdown blocking gate"
Assert-Contains -Text $reportText -Pattern "ordered-exact-unique" -Description "markdown timestamp contract"
Assert-Contains -Text $reportText -Pattern "| CALENDAR HOURLY 1 TO 24 | Schedule Value | conformance" -Description "markdown conformance row"

Write-Host "Calendar schedule hourly exact gate passed."
