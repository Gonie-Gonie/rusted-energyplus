[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$CaseId = "calendar_epw_dst_weekday_rules_hourly_exact_001"
$CaseRoot = Join-Path $RepoRoot "data\conformance_cases\$CaseId"
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$OutputRoot = Join-Path $RepoRoot ".runtime\time-weather-schedule-conformance\26.1.0"
$CasePath = Join-Path $CaseRoot "case.toml"
$IdfName = "calendar_epw_dst_weekday_rules_hourly_exact.idf"
$WeatherName = "calendar_epw_dst_weekday_rules_hourly_exact.epw"
$IdfPath = Join-Path $CaseRoot $IdfName
$WeatherPath = Join-Path $CaseRoot $WeatherName
$CaseOutputRoot = Join-Path $OutputRoot $CaseId
$IdfRef = "data/conformance_cases/$CaseId/$IdfName"
$WeatherRef = "data/conformance_cases/$CaseId/$WeatherName"
$ExpectedHeader = "HOLIDAYS/DAYLIGHT SAVINGS,Yes,4th Monday in February,Last Wednesday in February,0"
$ExpectedDataPeriod = "DATA PERIODS,1,1,Data,Sunday,2/22,2/26"
$ExpectedFirstTimestamp = "env=CALENDAR EPW DST WEEKDAY RULES RUN PERIOD;day=1;month=2;date=22;dst=0;hour=1;start=0.00;end=60.00;day_type=Sunday"
$ExpectedLastTimestamp = "env=CALENDAR EPW DST WEEKDAY RULES RUN PERIOD;day=5;month=2;date=26;dst=0;hour=24;start=0.00;end=60.00;day_type=Thursday"
$ExpectedDates = @(22, 23, 24, 25, 26)
$ExpectedDst = @(0, 1, 1, 1, 0)
$ExpectedDayTypes = @("Sunday", "Monday", "Tuesday", "Wednesday", "Thursday")

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
        throw "Missing required EPW weekday daylight-saving conformance file: $path"
    }
}

$caseText = Get-Content -LiteralPath $CasePath -Raw -Encoding UTF8
$idfText = Get-Content -LiteralPath $IdfPath -Raw -Encoding UTF8
$weatherLines = Get-Content -LiteralPath $WeatherPath -Encoding UTF8
$weatherText = $weatherLines -join "`n"

if ($weatherLines.Count -ne 128 -or
    @($weatherLines | Where-Object { [string]::IsNullOrWhiteSpace($_) }).Count -ne 0) {
    throw "EPW weekday daylight-saving fixture must contain exactly eight headers and 120 nonblank data rows"
}

Assert-Contains -Text $caseText -Pattern 'timestamp_contract = "ordered-exact-unique"' -Description "ordered timestamp contract"
Assert-Contains -Text $caseText -Pattern 'frequency = "hourly"' -Description "hourly manifest frequency"
Assert-Contains -Text $caseText -Pattern 'class = "weather"' -Description "weather manifest class"
Assert-Contains -Text $caseText -Pattern 'source = "eso"' -Description "ESO manifest source"
Assert-Contains -Text $caseText -Pattern 'level = "conformance"' -Description "conformance manifest level"
Assert-Contains -Text $caseText -Pattern "abs_tol = 0.0" -Description "zero absolute tolerance"
Assert-Contains -Text $caseText -Pattern "rmse_tol = 0.0" -Description "zero RMSE tolerance"
Assert-Contains -Text $caseText -Pattern "idf = `"$IdfRef`"" -Description "manifest input.idf attribution"
Assert-Contains -Text $caseText -Pattern "weather = `"$WeatherRef`"" -Description "manifest input.weather attribution"
Assert-Contains -Text $caseText -Pattern 'script = "scripts/dev.cmd compare-calendar-epw-dst-weekday-rules-exact"' -Description "manifest gate attribution"
Assert-Contains -Text $caseText -Pattern "blocking = true" -Description "manifest blocking flag"

$calendarHeaders = @($weatherLines | Where-Object { $_ -match '^\s*HOLIDAYS/DAYLIGHT SAVING' })
if ($calendarHeaders.Count -ne 1 -or $calendarHeaders[0] -cne $ExpectedHeader) {
    throw "EPW must contain exactly the expected fourth-Monday through last-Wednesday daylight-saving header"
}
$dataPeriodHeaders = @($weatherLines | Where-Object { $_ -match '^\s*DATA PERIODS,' })
if ($dataPeriodHeaders.Count -ne 1 -or $dataPeriodHeaders[0] -cne $ExpectedDataPeriod) {
    throw "EPW must contain exactly the expected five-day Sunday-start DATA PERIODS header"
}
$weatherRows = @($weatherLines | Select-Object -Skip 8 | Where-Object { -not [string]::IsNullOrWhiteSpace($_) })
if ($weatherRows.Count -ne 120) {
    throw "EPW weekday daylight-saving fixture must contain 120 hourly rows, found $($weatherRows.Count)"
}
$orderedPayloads = @()
for ($rowIndex = 0; $rowIndex -lt 120; ++$rowIndex) {
    $dayIndex = [int][math]::Floor($rowIndex / 24)
    $expectedHour = ($rowIndex % 24) + 1
    $fields = $weatherRows[$rowIndex] -split ','
    if ($fields.Count -ne 35 -or $fields[0] -ne "2032" -or $fields[1] -ne "2" -or
        [int]$fields[2] -ne $ExpectedDates[$dayIndex] -or [int]$fields[3] -ne $expectedHour -or
        $fields[4] -ne "60") {
        throw "EPW row $rowIndex must retain the exact 2032-02-22 through 2032-02-26 date/hour order"
    }
    $orderedPayloads += ($fields[5..34] -join ',')
}
if (@($orderedPayloads | Select-Object -Unique).Count -ne 1) {
    throw "EPW weekday daylight-saving weather payload must remain constant across all 120 ordered rows"
}
$weatherPayloads = @()
for ($dayIndex = 0; $dayIndex -lt $ExpectedDates.Count; ++$dayIndex) {
    $day = $ExpectedDates[$dayIndex]
    $date = "2032,2,$day"
    $dateRows = @($weatherRows | Where-Object { $_ -match ('^' + [regex]::Escape($date) + ',') })
    if ($dateRows.Count -ne 24) {
        throw "EPW weekday daylight-saving fixture must contain 24 rows for $date, found $($dateRows.Count)"
    }
    $hours = @($dateRows | ForEach-Object { [int](($_ -split ',')[3]) })
    if (($hours -join ',') -cne ((1..24) -join ',')) {
        throw "EPW weekday daylight-saving fixture must contain ordered hours 1..24 for $date"
    }
    foreach ($row in $dateRows) {
        $fields = $row -split ','
        if ($fields.Count -ne 35 -or $fields[0] -ne "2032" -or $fields[1] -ne "2" -or
            [int]$fields[2] -ne $day -or $fields[4] -ne "60") {
            throw "EPW weekday daylight-saving rows must contain the expected date, 35 fields, and minute 60"
        }
        $weatherPayloads += ($fields[5..34] -join ',')
    }
}
$weatherPayloads = @($weatherPayloads | Select-Object -Unique)
if ($weatherPayloads.Count -ne 1) {
    throw "EPW weekday daylight-saving weather payload must remain constant across all 120 rows"
}

foreach ($policy in @(
    "No,  !- Use Weather File Holidays and Special Days",
    "Yes, !- Use Weather File Daylight Saving Period",
    "No,  !- Apply Weekend Holiday Rule",
    "No,  !- Use Weather File Rain Indicators",
    "No,  !- Use Weather File Snow Indicators",
    "No;  !- Treat Weather as Actual"
)) {
    Assert-Contains -Text $idfText -Pattern $policy -Description "explicit RunPeriod policy"
}
Assert-Contains -Text $idfText -Pattern "Calendar EPW DST Weekday Rules Run Period" -Description "RunPeriod identity"
$runPeriodObjects = [regex]::Matches($idfText, '(?ims)^\s*RunPeriod\s*,(?<body>.*?);')
if ($runPeriodObjects.Count -ne 1 -or
    [regex]::Matches($idfText, '(?im)^\s*Output:Variable\s*,').Count -ne 1) {
    throw "EPW weekday daylight-saving fixture must contain exactly one RunPeriod and one Output:Variable"
}
$runPeriodBody = [regex]::Replace($runPeriodObjects[0].Groups["body"].Value, '(?m)!-.*$', '')
$runPeriodFields = @($runPeriodBody -split ',' | ForEach-Object { $_.Trim() } | Where-Object { $_ -ne "" })
$expectedRunPeriodFields = @(
    "Calendar EPW DST Weekday Rules Run Period",
    "2", "22", "2032", "2", "26", "2032", "Sunday",
    "No", "Yes", "No", "No", "No", "No"
)
if (($runPeriodFields -join '|') -cne ($expectedRunPeriodFields -join '|')) {
    throw "EPW weekday daylight-saving fixture must retain the exact 2032-02-22 through 2032-02-26 Sunday RunPeriod and explicit policies"
}
$outputObject = [regex]::Match($idfText, '(?ims)^\s*Output:Variable\s*,(?<body>.*?);')
$outputFields = @($outputObject.Groups["body"].Value -split ',' | ForEach-Object { $_.Trim() } | Where-Object { $_ -ne "" })
if (($outputFields -join '|') -cne "Environment|Site Daylight Saving Time Status|Hourly") {
    throw "EPW weekday daylight-saving fixture must request exactly the hourly Site Daylight Saving Time Status"
}
if ([regex]::Matches($idfText, '(?im)^\s*RunPeriodControl:SpecialDays\s*,').Count -ne 0) {
    throw "EPW weekday daylight-saving fixture must not contain RunPeriodControl:SpecialDays"
}
if ([regex]::Matches($idfText, '(?im)^\s*RunPeriodControl:DaylightSavingTime\s*,').Count -ne 0) {
    throw "EPW weekday daylight-saving fixture must not contain RunPeriodControl:DaylightSavingTime"
}

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Remove-RepoDirectory -Path $CaseOutputRoot
Write-Host "Running exact EPW fourth-Monday through last-Wednesday daylight-saving gate."
$output = & $cargo.Source run -p ep_cli --quiet -- conformance time-weather-schedule-report $CasePath $OracleRoot $OutputRoot 2>&1
if ($LASTEXITCODE -ne 0) {
    $output | ForEach-Object { Write-Host $_ }
    throw "EPW weekday daylight-saving rule case failed: $CaseId"
}
$outputText = $output -join "`n"
Assert-Contains -Text $outputText -Pattern "id: $CaseId" -Description "report id"
Assert-Contains -Text $outputText -Pattern "status: pass" -Description "report status"

$summaryPath = Join-Path $CaseOutputRoot "compare\compare-summary.json"
$reportPath = Join-Path $CaseOutputRoot "compare\compare-report.md"
$oracleEsoPath = Join-Path $CaseOutputRoot "oracle\eplusout.eso"
$oracleEioPath = Join-Path $CaseOutputRoot "oracle\eplusout.eio"
$oracleErrPath = Join-Path $CaseOutputRoot "oracle\eplusout.err"
$oracleEndPath = Join-Path $CaseOutputRoot "oracle\eplusout.end"
foreach ($path in @($summaryPath, $reportPath, $oracleEsoPath, $oracleEioPath, $oracleErrPath, $oracleEndPath)) {
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Missing EPW weekday daylight-saving comparison artifact: $path"
    }
}

$summary = Get-Content -LiteralPath $summaryPath -Raw -Encoding UTF8 | ConvertFrom-Json
if ($summary.case_id -cne $CaseId -or $summary.oracle_version -cne "26.1.0" -or
    $summary.comparison_class -cne "conformance" -or $summary.conformance_claim -ne $true -or
    $summary.status -cne "pass" -or $summary.time_axis_samples -ne 120 -or
    $summary.series_count -ne 1 -or $summary.conformance_series_count -ne 1 -or
    $summary.gate.script -cne "scripts/dev.cmd compare-calendar-epw-dst-weekday-rules-exact" -or
    $summary.gate.blocking -ne $true) {
    throw "EPW weekday daylight-saving case must retain its exact passing single-series conformance and blocking-gate contract"
}
$calendar = $summary.weather_calendar
if ($calendar.policy_applied -ne $true -or $calendar.weather_file_allows_leap_years -ne $true -or
    $calendar.gregorian_calendar_days -ne 5 -or $calendar.weather_effective_calendar_days -ne 5 -or
    $calendar.leap_days_skipped -ne 0) {
    throw "Unexpected five-day leap-year weather calendar state"
}
$daylightSaving = $calendar.daylight_saving
if ($daylightSaving.weather_file_period_declared -ne $true -or
    $daylightSaving.run_period_uses_weather_file_period -ne $true -or
    $daylightSaving.active -ne $true) {
    throw "EPW weekday daylight-saving declaration and RunPeriod policy must be active"
}
$resolvedPeriod = $daylightSaving.resolved_period
if ($resolvedPeriod.start_month -ne 2 -or $resolvedPeriod.start_day -ne 23 -or
    $resolvedPeriod.start_day_of_year -ne 54 -or
    $resolvedPeriod.end_month -ne 2 -or $resolvedPeriod.end_day -ne 25 -or
    $resolvedPeriod.end_day_of_year -ne 56 -or $resolvedPeriod.wraps_year -ne $false) {
    throw "Unexpected resolved fourth-Monday through last-Wednesday daylight-saving period"
}
if ($calendar.daylight_saving_hourly_samples -ne 72) {
    throw "Expected 72 DST-active hourly samples, found $($calendar.daylight_saving_hourly_samples)"
}
$specialDays = $summary.special_days
if ($specialDays.weather_file_declared -ne 0 -or $specialDays.run_period_uses_weather_file -ne $false -or
    $specialDays.weather_file_resolved -ne 0 -or $specialDays.input_file_declared -ne 0 -or
    $specialDays.apply_weekend_rule -ne $false -or $specialDays.resolved_count -ne 0 -or
    $specialDays.hourly_samples -ne 0) {
    throw "Weekday daylight-saving fixture must not activate special days"
}
$selection = $summary.weather_record_selection
if ($selection.applied -ne $true -or $selection.data_period_index -ne 1 -or
    $selection.source_start_record_index -ne 0 -or $selection.initial_tomorrow_source_record_index -ne 0 -or
    $selection.selected_hourly_records -ne 120 -or $selection.skipped_raw_february_29_days -ne 0 -or
    $selection.day_buffer_transitions -ne 5) {
    throw "Unexpected weather record selection state"
}

$seriesRows = @($summary.series | Where-Object {
    $_.key -eq "ENVIRONMENT" -and $_.variable -eq "Site Daylight Saving Time Status"
})
if ($seriesRows.Count -ne 1) {
    throw "Missing unique Site Daylight Saving Time Status series"
}
$series = $seriesRows[0]
if ($series.level -cne "conformance" -or $series.class -cne "weather" -or
    $series.frequency -cne "hourly" -or $series.source -cne "eso" -or
    $series.alignment -cne "timestamp" -or
    $series.expected_samples -ne 120 -or $series.observed_samples -ne 120 -or $series.compared_samples -ne 120 -or
    $series.timestamp_contract -ne "ordered-exact-unique" -or $series.timestamp_status -ne "pass" -or
    $series.timestamp_expected_unique -ne $true -or $series.timestamp_observed_unique -ne $true -or
    $series.timestamp_order_match -ne $true -or
    $series.expected_first_timestamp -cne $ExpectedFirstTimestamp -or $series.observed_first_timestamp -cne $ExpectedFirstTimestamp -or
    $series.expected_last_timestamp -cne $ExpectedLastTimestamp -or $series.observed_last_timestamp -cne $ExpectedLastTimestamp -or
    $series.max_abs_tolerance -ne 0.0 -or $series.max_rel_tolerance -ne 0.0 -or $series.max_rmse_tolerance -ne 0.0 -or
    $series.max_abs_delta -ne 0.0 -or $series.rmse_delta -ne 0.0 -or $series.max_rel_delta -ne 0.0 -or
    $null -ne $series.first_divergence -or $null -ne $series.first_timestamp_divergence -or
    $series.status -ne "pass") {
    throw "Ordered exact Site Daylight Saving Time Status contract failed"
}

$oracleEsoLines = Get-Content -LiteralPath $oracleEsoPath -Encoding UTF8
$dictionaryRows = @($oracleEsoLines | Where-Object {
    $_ -match '^\d+,1,Environment,Site Daylight Saving Time Status \[\] !Hourly$'
})
if ($dictionaryRows.Count -ne 1) {
    throw "Expected one Site Daylight Saving Time Status ESO dictionary entry"
}
$dictionaryMatch = [regex]::Match([string]$dictionaryRows[0], '^(\d+),')
if (-not $dictionaryMatch.Success) {
    throw "Missing Site Daylight Saving Time Status ESO report id"
}
$reportId = $dictionaryMatch.Groups[1].Value
$valueRows = @($oracleEsoLines | Where-Object { $_ -match ('^' + $reportId + ',\s*[-+0-9.E]+\s*$') })
$values = @($valueRows | ForEach-Object { [double](($_ -split ',', 2)[1].Trim()) })
$timestampRows = @($oracleEsoLines | Where-Object { $_ -match '^2,\d+,\s*\d+,\s*\d+,\s*\d+,\s*\d+,' })
if ($values.Count -ne 120 -or $timestampRows.Count -ne 120) {
    throw "Expected 120 oracle daylight-saving values and timestamps"
}
for ($index = 0; $index -lt 120; ++$index) {
    $dayOffset = [int][math]::Floor($index / 24)
    $expectedHour = ($index % 24) + 1
    if ($values[$index] -ne [double]$ExpectedDst[$dayOffset]) {
        throw "Unexpected oracle Site Daylight Saving Time Status at sample $index`: $($values[$index])"
    }
    $timestampMatch = [regex]::Match(
        $timestampRows[$index],
        '^2,\s*(\d+),\s*(\d+),\s*(\d+),\s*(\d+),\s*(\d+),\s*([-+0-9.]+),\s*([-+0-9.]+),([^,]+)$'
    )
    if (-not $timestampMatch.Success -or
        [int]$timestampMatch.Groups[1].Value -ne ($dayOffset + 1) -or
        [int]$timestampMatch.Groups[2].Value -ne 2 -or
        [int]$timestampMatch.Groups[3].Value -ne $ExpectedDates[$dayOffset] -or
        [int]$timestampMatch.Groups[4].Value -ne $ExpectedDst[$dayOffset] -or
        [int]$timestampMatch.Groups[5].Value -ne $expectedHour -or
        $timestampMatch.Groups[6].Value -cne "0.00" -or
        $timestampMatch.Groups[7].Value -cne "60.00" -or
        $timestampMatch.Groups[8].Value.Trim() -cne $ExpectedDayTypes[$dayOffset]) {
        throw "Unexpected oracle daylight-saving timestamp at sample $index`: $($timestampRows[$index])"
    }
}

$oracleEioLines = Get-Content -LiteralPath $oracleEioPath -Encoding UTF8
$daylightSavingEioRows = @($oracleEioLines | Where-Object { $_ -match '^Environment:Daylight Saving,' })
if ($daylightSavingEioRows.Count -ne 1 -or
    [string]$daylightSavingEioRows[0] -cne "Environment:Daylight Saving,Yes,WeatherFile,02/23,02/25") {
    throw "Expected exact EnergyPlus EIO weather-file daylight-saving row"
}

$oracleErrText = Get-Content -LiteralPath $oracleErrPath -Raw -Encoding UTF8
Assert-Contains -Text $oracleErrText -Pattern "EnergyPlus Completed Successfully-- 0 Warning; 0 Severe Errors;" -Description "clean EnergyPlus completion"
if ([regex]::Matches($oracleErrText, '(?m)^\s*\*\* Warning \*\*').Count -ne 0 -or
    [regex]::Matches($oracleErrText, '(?m)^\s*\*\* Severe\s+\*\*').Count -ne 0) {
    throw "EPW weekday daylight-saving oracle must complete without warning or severe markers"
}
$oracleEndText = Get-Content -LiteralPath $oracleEndPath -Raw -Encoding UTF8
Assert-Contains -Text $oracleEndText -Pattern "EnergyPlus Completed Successfully-- 0 Warning; 0 Severe Errors;" -Description "clean EnergyPlus end record"

$reportText = Get-Content -LiteralPath $reportPath -Raw -Encoding UTF8
foreach ($entry in @(
    "weather_file_daylight_saving_period_declared: true",
    "run_period_uses_weather_file_daylight_saving_period: true",
    "daylight_saving_active: true",
    "daylight_saving_resolved_period: 2/23 through 2/25 (wraps_year=false)",
    "daylight_saving_hourly_samples: 72",
    "weather_file_holidays_declared: 0",
    "special_days_resolved: 0",
    "weather_selected_hourly_records: 120",
    "weather_day_buffer_transitions: 5"
)) {
    Assert-Contains -Text $reportText -Pattern $entry -Description "markdown weekday daylight-saving state"
}

Write-Host "Exact EPW fourth-Monday through last-Wednesday daylight-saving gate passed."
