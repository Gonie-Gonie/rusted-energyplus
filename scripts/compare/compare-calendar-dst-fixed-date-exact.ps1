[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$CaseId = "calendar_dst_fixed_date_hourly_exact_001"
$CaseRoot = Join-Path $RepoRoot "data\conformance_cases\$CaseId"
$CasePath = Join-Path $CaseRoot "case.toml"
$IdfPath = Join-Path $CaseRoot "calendar_dst_fixed_date_hourly_exact.idf"
$WeatherPath = Join-Path $CaseRoot "calendar_dst_fixed_date_hourly_exact.epw"
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$OutputRoot = Join-Path $RepoRoot ".runtime\time-weather-schedule-conformance\26.1.0"
$CaseOutputRoot = Join-Path $OutputRoot $CaseId

function Assert-RepoSubPath {
    param([Parameter(Mandatory = $true)][string]$Path)
    $full = [System.IO.Path]::GetFullPath($Path)
    $root = [System.IO.Path]::GetFullPath($RepoRoot).TrimEnd(
        [System.IO.Path]::DirectorySeparatorChar,
        [System.IO.Path]::AltDirectorySeparatorChar
    )
    $rootPrefix = $root + [System.IO.Path]::DirectorySeparatorChar
    if (-not $full.StartsWith($rootPrefix, [System.StringComparison]::OrdinalIgnoreCase)) {
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
        throw "Missing $Description`: $Pattern"
    }
    Write-Host "OK $Description`: $Pattern"
}

foreach ($path in @(
    (Join-Path $OracleRoot "energyplus.exe"),
    (Join-Path $OracleRoot "ConvertInputFormat.exe"),
    $CasePath,
    $IdfPath,
    $WeatherPath
)) {
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Missing required fixed-date DST conformance file: $path"
    }
}

$caseText = Get-Content -LiteralPath $CasePath -Raw -Encoding UTF8
$idfText = Get-Content -LiteralPath $IdfPath -Raw -Encoding UTF8
$weatherLines = Get-Content -LiteralPath $WeatherPath -Encoding UTF8
$weatherText = $weatherLines -join "`n"
$weatherRows = @($weatherLines | Select-Object -Skip 8 | Where-Object { -not [string]::IsNullOrWhiteSpace($_) })

Assert-Contains -Text $caseText -Pattern 'timestamp_contract = "ordered-exact-unique"' -Description "ordered DST timestamp contract"
Assert-Contains -Text $caseText -Pattern 'abs_tol = 0.0' -Description "zero absolute tolerance"
Assert-Contains -Text $caseText -Pattern 'rmse_tol = 0.0' -Description "zero RMSE tolerance"
Assert-Contains -Text $idfText -Pattern "Yes, !- Use Weather File Daylight Saving Period" -Description "RunPeriod weather-file DST policy"
Assert-Contains -Text $weatherText -Pattern "HOLIDAYS/DAYLIGHT SAVINGS,Yes,2/29,3/1,0" -Description "EPW fixed-date DST period"

if ($weatherRows.Count -ne 72) {
    throw "Fixed-date DST EPW must contain 72 hourly rows, found $($weatherRows.Count)"
}
foreach ($date in @("2016,2,28", "2016,2,29", "2016,3,1")) {
    $dateRows = @($weatherRows | Where-Object { $_ -match ('^' + [regex]::Escape($date) + ',') })
    if ($dateRows.Count -ne 24) {
        throw "Fixed-date DST EPW must contain 24 rows for $date, found $($dateRows.Count)"
    }
}

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Remove-RepoDirectory -Path $CaseOutputRoot
Write-Host "Running fixed-date EPW daylight-saving exact gate."
$output = & $cargo.Source run -p ep_cli --quiet -- conformance time-weather-schedule-report $CasePath $OracleRoot $OutputRoot 2>&1
if ($LASTEXITCODE -ne 0) {
    $output | ForEach-Object { Write-Host $_ }
    throw "Fixed-date EPW daylight-saving exact gate failed."
}
$text = $output -join "`n"
Assert-Contains -Text $text -Pattern "id: $CaseId" -Description "report id"
Assert-Contains -Text $text -Pattern "status: pass" -Description "report status"

$summaryPath = Join-Path $CaseOutputRoot "compare\compare-summary.json"
$reportPath = Join-Path $CaseOutputRoot "compare\compare-report.md"
foreach ($path in @($summaryPath, $reportPath)) {
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Missing fixed-date DST comparison artifact: $path"
    }
}

$summary = Get-Content -LiteralPath $summaryPath -Raw -Encoding UTF8 | ConvertFrom-Json
if ($summary.status -ne "pass" -or $summary.conformance_claim -ne $true) {
    throw "Fixed-date DST case must be passing conformance evidence"
}
if ($summary.time_axis_samples -ne 72) {
    throw "Expected 72 hourly TimeAxis samples, found $($summary.time_axis_samples)"
}
if ($summary.weather_calendar.daylight_saving.weather_file_period_declared -ne $true -or
    $summary.weather_calendar.daylight_saving.run_period_uses_weather_file_period -ne $true -or
    $summary.weather_calendar.daylight_saving.active -ne $true) {
    throw "EPW fixed-date daylight-saving declaration and RunPeriod policy must be active"
}
$resolvedPeriod = $summary.weather_calendar.daylight_saving.resolved_period
if ($resolvedPeriod.start_month -ne 2 -or $resolvedPeriod.start_day -ne 29 -or
    $resolvedPeriod.end_month -ne 3 -or $resolvedPeriod.end_day -ne 1 -or
    $resolvedPeriod.wraps_year -ne $false) {
    throw "Unexpected resolved fixed-date daylight-saving period"
}
if ($summary.weather_calendar.daylight_saving_hourly_samples -ne 48) {
    throw "Expected 48 DST-active hourly samples, found $($summary.weather_calendar.daylight_saving_hourly_samples)"
}

$series = $summary.series | Where-Object {
    $_.key -eq "ENVIRONMENT" -and $_.variable -eq "Site Daylight Saving Time Status"
}
if ($null -eq $series) {
    throw "Missing Site Daylight Saving Time Status series"
}
if ($series.expected_samples -ne 72 -or $series.observed_samples -ne 72 -or $series.compared_samples -ne 72) {
    throw "Unexpected DST series sample counts"
}
if ($series.timestamp_contract -ne "ordered-exact-unique" -or $series.timestamp_status -ne "pass") {
    throw "Ordered DST timestamp contract failed"
}
if ($series.timestamp_expected_unique -ne $true -or $series.timestamp_observed_unique -ne $true -or $series.timestamp_order_match -ne $true) {
    throw "DST timestamp uniqueness/order failed"
}
$expectedFirst = "env=DST FIXED DATE RUN PERIOD;day=1;month=2;date=28;dst=0;hour=1;start=0.00;end=60.00;day_type=Sunday"
$expectedLast = "env=DST FIXED DATE RUN PERIOD;day=3;month=3;date=1;dst=1;hour=24;start=0.00;end=60.00;day_type=Tuesday"
if ($series.expected_first_timestamp -ne $expectedFirst -or $series.observed_first_timestamp -ne $expectedFirst) {
    throw "Unexpected first fixed-date DST timestamp"
}
if ($series.expected_last_timestamp -ne $expectedLast -or $series.observed_last_timestamp -ne $expectedLast) {
    throw "Unexpected last fixed-date DST timestamp"
}
if ($series.max_abs_tolerance -ne 0.0 -or $series.max_rel_tolerance -ne 0.0 -or $series.max_rmse_tolerance -ne 0.0) {
    throw "Fixed-date DST case must use zero tolerances"
}
if ($series.max_abs_delta -ne 0.0 -or $series.rmse_delta -ne 0.0 -or $series.max_rel_delta -ne 0.0 -or $series.status -ne "pass") {
    throw "DST state values must match exactly"
}

$reportText = Get-Content -LiteralPath $reportPath -Raw -Encoding UTF8
Assert-Contains -Text $reportText -Pattern "daylight_saving_resolved_period: 2/29 through 3/1 (wraps_year=false)" -Description "markdown resolved DST period"
Assert-Contains -Text $reportText -Pattern "daylight_saving_hourly_samples: 48" -Description "markdown DST-active sample count"
Assert-Contains -Text $reportText -Pattern "Site Daylight Saving Time Status" -Description "markdown DST output row"
Write-Host "Fixed-date EPW daylight-saving exact gate passed."
