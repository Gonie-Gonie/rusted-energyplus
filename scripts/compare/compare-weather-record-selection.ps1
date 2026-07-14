[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
Set-Location $RepoRoot
$CaseId = "weather_record_start_offset_nonactual_001"
$CasePath = Join-Path $RepoRoot "data\conformance_cases\$CaseId\case.toml"
$IdfPath = Join-Path $RepoRoot "data\conformance_cases\$CaseId\weather_record_start_offset_nonactual.idf"
$WeatherPath = Join-Path $RepoRoot "data\conformance_cases\$CaseId\weather_record_start_offset_nonactual.epw"
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$OutputRoot = Join-Path $RepoRoot ".runtime\time-weather-schedule-conformance\26.1.0"
$CaseOutputRoot = Join-Path $OutputRoot $CaseId
$ExpectedFirstTimestamp = "env=WEATHER RECORD OFFSET RUN PERIOD;day=1;month=7;date=1;dst=0;hour=1;start=0.00;end=60.00;day_type=Friday"
$ExpectedLastTimestamp = "env=WEATHER RECORD OFFSET RUN PERIOD;day=2;month=7;date=2;dst=0;hour=24;start=0.00;end=60.00;day_type=Saturday"

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

function Assert-RepoSubPath {
    param([Parameter(Mandatory = $true)][string]$Path)
    $full = [System.IO.Path]::GetFullPath($Path)
    $root = [System.IO.Path]::GetFullPath($RepoRoot).TrimEnd([System.IO.Path]::DirectorySeparatorChar, [System.IO.Path]::AltDirectorySeparatorChar)
    $rootPrefix = $root + [System.IO.Path]::DirectorySeparatorChar
    if (-not $full.StartsWith($rootPrefix, [System.StringComparison]::OrdinalIgnoreCase)) {
        throw "Refusing to operate outside repository: $full"
    }
}

foreach ($path in @(
    (Join-Path $OracleRoot "energyplus.exe"),
    (Join-Path $OracleRoot "ConvertInputFormat.exe"),
    $CasePath,
    $IdfPath,
    $WeatherPath
)) {
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Missing required weather-record-selection file: $path"
    }
}

$manifestText = Get-Content -LiteralPath $CasePath -Raw -Encoding UTF8
Assert-Contains -Text $manifestText -Pattern 'idf = "data/conformance_cases/weather_record_start_offset_nonactual_001/weather_record_start_offset_nonactual.idf"' -Description "manifest IDF attribution"
Assert-Contains -Text $manifestText -Pattern 'weather = "data/conformance_cases/weather_record_start_offset_nonactual_001/weather_record_start_offset_nonactual.epw"' -Description "manifest weather attribution"
Assert-Contains -Text $manifestText -Pattern 'timestamp_contract = "ordered-exact-unique"' -Description "ordered timestamp contract"

$weatherLines = Get-Content -LiteralPath $WeatherPath
$dataRows = @($weatherLines | Select-Object -Skip 8 | Where-Object { -not [string]::IsNullOrWhiteSpace($_) })
$decoyRows = @($dataRows | Where-Object { $_ -match '^1999,6,30,' })
$selectedDayOneRows = @($dataRows | Where-Object { $_ -match '^2004,7,1,' })
$selectedDayTwoRows = @($dataRows | Where-Object { $_ -match '^2007,7,2,' })
if ($dataRows.Count -ne 72 -or $decoyRows.Count -ne 24 -or $selectedDayOneRows.Count -ne 24 -or $selectedDayTwoRows.Count -ne 24) {
    throw "Expected three complete 24-hour source days, found total=$($dataRows.Count) decoy=$($decoyRows.Count) day1=$($selectedDayOneRows.Count) day2=$($selectedDayTwoRows.Count)"
}
Assert-Contains -Text ($weatherLines -join "`n") -Pattern "DATA PERIODS,1,1,Data,Thursday,6/30,7/2" -Description "single source data period"
Assert-Contains -Text ($weatherLines -join "`n") -Pattern "HOLIDAYS/DAYLIGHT SAVINGS,No,0,0,0" -Description "non-leap weather policy"

if (Test-Path -LiteralPath $CaseOutputRoot) {
    Assert-RepoSubPath -Path $CaseOutputRoot
    Remove-Item -LiteralPath $CaseOutputRoot -Recurse -Force
}

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Write-Host "Running source-order EPW record-selection gate."
$output = & $cargo.Source run -p ep_cli --quiet -- conformance time-weather-schedule-report $CasePath $OracleRoot $OutputRoot 2>&1
if ($LASTEXITCODE -ne 0) {
    $output | ForEach-Object { Write-Host $_ }
    throw "Weather record-selection case failed."
}
$outputText = $output -join "`n"
Assert-Contains -Text $outputText -Pattern "id: $CaseId" -Description "report case id"
Assert-Contains -Text $outputText -Pattern "status: pass" -Description "report status"

$summaryPath = Join-Path $CaseOutputRoot "compare\compare-summary.json"
$reportPath = Join-Path $CaseOutputRoot "compare\compare-report.md"
foreach ($path in @($summaryPath, $reportPath)) {
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Missing weather record-selection artifact: $path"
    }
}

$summary = Get-Content -LiteralPath $summaryPath -Raw | ConvertFrom-Json
if ($summary.status -ne "pass" -or $summary.conformance_claim -ne $true -or $summary.time_axis_samples -ne 48) {
    throw "Weather record-selection summary must be a passing 48-sample conformance claim."
}
$selection = $summary.weather_record_selection
if ($selection.applied -ne $true -or $selection.data_period_index -ne 1) {
    throw "EPW DATA PERIOD selection was not applied."
}
if ($selection.source_start_record_index -ne 24 -or $selection.initial_tomorrow_source_record_index -ne 24) {
    throw "The literal July 1 start must skip exactly the 24 June 30 decoy rows."
}
if ($selection.selected_hourly_records -ne 48 -or $selection.skipped_raw_february_29_days -ne 0 -or $selection.day_buffer_transitions -ne 2) {
    throw "Unexpected selected-record or day-buffer counts."
}

$series = $summary.series | Where-Object {
    $_.key -eq "Environment" -and $_.variable -eq "Site Outdoor Air Drybulb Temperature"
}
if ($null -eq $series) {
    throw "Missing dry-bulb comparison series."
}
if ($series.expected_samples -ne 48 -or $series.observed_samples -ne 48 -or $series.compared_samples -ne 48) {
    throw "Dry-bulb comparison must contain 48 samples."
}
if ($series.timestamp_contract -ne "ordered-exact-unique" -or $series.timestamp_status -ne "pass" -or $series.timestamp_order_match -ne $true) {
    throw "Ordered timestamp contract failed."
}
if ($series.timestamp_expected_unique -ne $true -or $series.timestamp_observed_unique -ne $true -or $null -ne $series.first_timestamp_divergence) {
    throw "Timestamp uniqueness or exact sequence failed."
}
if ($series.expected_first_timestamp -ne $ExpectedFirstTimestamp -or $series.observed_first_timestamp -ne $ExpectedFirstTimestamp) {
    throw "Unexpected first timestamp."
}
if ($series.expected_last_timestamp -ne $ExpectedLastTimestamp -or $series.observed_last_timestamp -ne $ExpectedLastTimestamp) {
    throw "Unexpected last timestamp."
}
if ($series.max_abs_tolerance -ne 0.0 -or $series.max_rmse_tolerance -ne 0.0 -or $series.max_abs_delta -ne 0.0 -or $series.rmse_delta -ne 0.0 -or $series.status -ne "pass") {
    throw "Dry-bulb values must match with zero tolerance and zero delta."
}

$reportText = Get-Content -LiteralPath $reportPath -Raw
Assert-Contains -Text $reportText -Pattern "weather_record_selection_applied: true" -Description "markdown record selection"
Assert-Contains -Text $reportText -Pattern "weather_source_start_record_index: 24" -Description "markdown literal start offset"
Assert-Contains -Text $reportText -Pattern "weather_selected_hourly_records: 48" -Description "markdown selected records"

Write-Host "Weather record-selection gate passed."
