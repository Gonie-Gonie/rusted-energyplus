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
$CaseId = "calendar_dst_epw_idf_precedence_hourly_exact_001"
$CaseRoot = Join-Path $RepoRoot "data\conformance_cases\$CaseId"
$CasePath = Join-Path $CaseRoot "case.toml"
$IdfPath = Join-Path $CaseRoot "calendar_dst_epw_idf_precedence_hourly_exact.idf"
$IdfRef = "data/conformance_cases/calendar_dst_epw_idf_precedence_hourly_exact_001/calendar_dst_epw_idf_precedence_hourly_exact.idf"
$BaseIdfPath = Join-Path $RepoRoot "data\conformance_cases\calendar_dst_fixed_date_disabled_hourly_exact_001\calendar_dst_fixed_date_disabled_hourly_exact.idf"
$WeatherRef = "data/conformance_cases/calendar_dst_fixed_date_hourly_exact_001/calendar_dst_fixed_date_hourly_exact.epw"
$WeatherPath = Join-Path $RepoRoot ($WeatherRef -replace '/', '\')
$CaseOutputRoot = Join-Path $OutputRoot $CaseId
$ExpectedHeader = "HOLIDAYS/DAYLIGHT SAVINGS,Yes,2/29,3/1,0"
$ExpectedDataPeriod = "DATA PERIODS,1,1,Data,Sunday,2/28,3/1"
$ExpectedMonths = @(2, 2, 3)
$ExpectedDates = @(28, 29, 1)
$ExpectedDayTypes = @("Sunday", "Monday", "Tuesday")
$ExpectedDailyDst = @(1, 1, 0)
$ExpectedFirstTimestamp = "env=DST FIXED DATE RUN PERIOD;day=1;month=2;date=28;dst=1;hour=1;start=0.00;end=60.00;day_type=Sunday"
$ExpectedLastTimestamp = "env=DST FIXED DATE RUN PERIOD;day=3;month=3;date=1;dst=0;hour=24;start=0.00;end=60.00;day_type=Tuesday"
$ExpectedEnvironmentEio = "Environment,DST FIXED DATE RUN PERIOD,WeatherFileRunPeriod,02/28/2016,03/01/2016,Sunday,3,Use RunPeriod Specified Day,No,No,No,No,No,Clark and Allen"
$ExpectedDaylightSavingEio = "Environment:Daylight Saving,Yes,InputFile,02/28,02/29"

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
    $BaseIdfPath,
    $WeatherPath
)) {
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Missing required fixed-date EPW-versus-IDF daylight-saving precedence file: $path"
    }
}

$weatherLines = Get-Content -LiteralPath $WeatherPath -Encoding UTF8
$weatherNonblankLines = @($weatherLines | Where-Object { -not [string]::IsNullOrWhiteSpace($_) })
if ($weatherLines.Count -ne 81 -or $weatherNonblankLines.Count -ne 80 -or
    -not [string]::IsNullOrWhiteSpace($weatherLines[-1])) {
    throw "Fixed-date DST precedence EPW must contain exactly 80 nonblank lines followed by one trailing blank line"
}
$calendarHeaders = @($weatherNonblankLines | Where-Object { $_ -match '^\s*HOLIDAYS/DAYLIGHT SAVING' })
if ($calendarHeaders.Count -ne 1 -or $calendarHeaders[0] -cne $ExpectedHeader) {
    throw "Fixed-date DST precedence EPW must contain exactly the expected 2/29 through 3/1 header"
}
$dataPeriodHeaders = @($weatherNonblankLines | Where-Object { $_ -match '^\s*DATA PERIODS,' })
if ($dataPeriodHeaders.Count -ne 1 -or $dataPeriodHeaders[0] -cne $ExpectedDataPeriod) {
    throw "Fixed-date DST precedence EPW must contain exactly the expected Sunday-start DATA PERIODS header"
}
$weatherRows = @($weatherNonblankLines | Select-Object -Skip 8)
if ($weatherRows.Count -ne 72) {
    throw "Fixed-date DST precedence EPW must contain 72 hourly rows, found $($weatherRows.Count)"
}
$orderedPayloads = @()
for ($rowIndex = 0; $rowIndex -lt 72; ++$rowIndex) {
    $dayIndex = [int][math]::Floor($rowIndex / 24)
    $expectedHour = ($rowIndex % 24) + 1
    $fields = $weatherRows[$rowIndex] -split ','
    if ($fields.Count -ne 35 -or $fields[0] -ne "2016" -or
        [int]$fields[1] -ne $ExpectedMonths[$dayIndex] -or
        [int]$fields[2] -ne $ExpectedDates[$dayIndex] -or
        [int]$fields[3] -ne $expectedHour -or $fields[4] -ne "60") {
        throw "Fixed-date DST precedence EPW row $rowIndex must retain exact date/hour order, 35 fields, and minute 60"
    }
    $orderedPayloads += ($fields[5..34] -join ',')
}
if (@($orderedPayloads | Select-Object -Unique).Count -ne 1) {
    throw "Fixed-date DST precedence EPW weather payload must remain constant across all 72 rows"
}
for ($dayIndex = 0; $dayIndex -lt 3; ++$dayIndex) {
    $date = "2016,$($ExpectedMonths[$dayIndex]),$($ExpectedDates[$dayIndex])"
    $dateRows = @($weatherRows | Where-Object { $_ -match ('^' + [regex]::Escape($date) + ',') })
    $hours = @($dateRows | ForEach-Object { [int](($_ -split ',')[3]) })
    if ($dateRows.Count -ne 24 -or ($hours -join ',') -cne ((1..24) -join ',')) {
        throw "Fixed-date DST precedence EPW must contain ordered hours 1..24 for $date"
    }
}

$caseText = Get-Content -LiteralPath $CasePath -Raw -Encoding UTF8
$idfText = Get-Content -LiteralPath $IdfPath -Raw -Encoding UTF8
$baseIdfText = Get-Content -LiteralPath $BaseIdfPath -Raw -Encoding UTF8
foreach ($contract in @(
    'comparison_class = "conformance"',
    'conformance_claim = true',
    'timestamp_contract = "ordered-exact-unique"',
    'frequency = "hourly"',
    'class = "weather"',
    'source = "eso"',
    'level = "conformance"',
    'abs_tol = 0.0',
    'rmse_tol = 0.0',
    'blocking = true'
)) {
    Assert-Contains -Text $caseText -Pattern $contract -Description "$CaseId manifest contract"
}
foreach ($scopeStatement in @(
    'the only IDF addition is one RunPeriodControl:DaylightSavingTime object declaring 2/28 through 2/29',
    'The input-file object takes precedence independently of that flag, proving daily states 1, 1, and 0 with exactly 48 active hours',
    'External evidence is limited to 72 ordered, unique raw EnergyPlus ESO values and timestamp fields, the exact EnergyPlus 26.1 Environment and Environment:Daylight Saving rows, and clean 0 Warning and 0 Severe completion',
    'The Rust weather-file declaration, RunPeriod-use, input-file declaration, active-state, effective-source, resolved-period, and active-sample fields are locked summary diagnostics; they are not additional fields emitted by the EnergyPlus EIO rows',
    'Blank, omitted, defaulted, duplicate, or invalid input-file objects',
    'weather-file enablement with an input-file object; no-EPW execution; and multiple RunPeriods remain outside this claim',
    'Rust raw ESO serialization; broad WeatherManager compatibility; and other output variables remain outside this claim'
)) {
    Assert-Contains -Text $caseText -Pattern $scopeStatement -Description "$CaseId bounded claim or nonclaim"
}
Assert-Contains -Text $caseText -Pattern "source_file = `"$IdfRef`"" -Description "$CaseId manifest source-file attribution"
Assert-Contains -Text $caseText -Pattern "idf = `"$IdfRef`"" -Description "$CaseId manifest input.idf attribution"
Assert-Contains -Text $caseText -Pattern "weather = `"$WeatherRef`"" -Description "$CaseId shared manifest weather attribution"
Assert-Contains -Text $caseText -Pattern 'script = "scripts/dev.cmd compare-calendar-dst-epw-idf-precedence-exact"' -Description "$CaseId blocking gate attribution"

$runPeriodObjects = [regex]::Matches($idfText, '(?ims)^\s*RunPeriod\s*,(?<body>.*?);')
$daylightSavingObjects = [regex]::Matches($idfText, '(?ims)^\s*RunPeriodControl:DaylightSavingTime\s*,(?<body>.*?);')
$outputObjects = [regex]::Matches($idfText, '(?ims)^\s*Output:Variable\s*,(?<body>.*?);')
if ($runPeriodObjects.Count -ne 1 -or $daylightSavingObjects.Count -ne 1 -or $outputObjects.Count -ne 1) {
    throw "$CaseId must contain exactly one RunPeriod, one RunPeriodControl:DaylightSavingTime, and one Output:Variable"
}
if ([regex]::Matches($baseIdfText, '(?im)^\s*RunPeriodControl:DaylightSavingTime\s*,').Count -ne 0 -or
    [regex]::Matches($idfText, '(?im)^\s*RunPeriodControl:SpecialDays\s*,').Count -ne 0) {
    throw "The disabled base must have no input-file DST object and the precedence fixture must have no special-day object"
}
$runPeriodBody = [regex]::Replace($runPeriodObjects[0].Groups["body"].Value, '(?m)!-.*$', '')
$runPeriodFields = @($runPeriodBody -split ',' | ForEach-Object { $_.Trim() } | Where-Object { $_ -ne "" })
$expectedRunPeriodFields = @(
    "DST Fixed Date Run Period", "2", "28", "2016", "3", "1", "2016", "Sunday",
    "No", "No", "No", "No", "No", "No"
)
if (($runPeriodFields -join '|') -cne ($expectedRunPeriodFields -join '|')) {
    throw "$CaseId must retain the disabled base's exact fixed-date RunPeriod and explicit policies"
}
$daylightSavingFields = @($daylightSavingObjects[0].Groups["body"].Value -split ',' | ForEach-Object { $_.Trim() } | Where-Object { $_ -ne "" })
if (($daylightSavingFields -join '|') -cne "2/28|2/29") {
    throw "$CaseId must declare exactly the input-file daylight-saving period 2/28 through 2/29"
}
$outputFields = @($outputObjects[0].Groups["body"].Value -split ',' | ForEach-Object { $_.Trim() } | Where-Object { $_ -ne "" })
if (($outputFields -join '|') -cne "Environment|Site Daylight Saving Time Status|Hourly") {
    throw "$CaseId must request exactly the hourly Site Daylight Saving Time Status"
}

$strictUtf8 = New-Object System.Text.UTF8Encoding($false, $true)
$baseRawText = $strictUtf8.GetString([System.IO.File]::ReadAllBytes($BaseIdfPath))
$precedenceRawText = $strictUtf8.GetString([System.IO.File]::ReadAllBytes($IdfPath))
$outputAnchor = "Output:Variable,`n  Environment,`n  Site Daylight Saving Time Status,`n  Hourly;`n"
$insertedObject = "RunPeriodControl:DaylightSavingTime,`n  2/28,`n  2/29;`n`n"
if ([regex]::Matches($baseRawText, [regex]::Escape($outputAnchor)).Count -ne 1 -or
    [regex]::Matches($precedenceRawText, [regex]::Escape($insertedObject)).Count -ne 1) {
    throw "The disabled base anchor and exact input-file daylight-saving insertion must each occur once"
}
$expectedPrecedenceBytes = $strictUtf8.GetBytes($baseRawText.Replace($outputAnchor, $insertedObject + $outputAnchor))
$observedPrecedenceBytes = $strictUtf8.GetBytes($precedenceRawText)
if ([Convert]::ToBase64String($expectedPrecedenceBytes) -cne [Convert]::ToBase64String($observedPrecedenceBytes)) {
    throw "$CaseId must be byte-equivalent to the disabled base after adding exactly the one input-file daylight-saving object"
}

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Write-Host "Running fixed-date EPW-versus-IDF daylight-saving precedence exact gate."
Remove-RepoDirectory -Path $CaseOutputRoot
$output = & $cargo.Source run -p ep_cli --quiet -- conformance time-weather-schedule-report $CasePath $OracleRoot $OutputRoot 2>&1
if ($LASTEXITCODE -ne 0) {
    $output | ForEach-Object { Write-Host $_ }
    throw "Fixed-date EPW-versus-IDF daylight-saving precedence case failed: $CaseId"
}
$outputText = $output -join "`n"
Assert-Contains -Text $outputText -Pattern "id: $CaseId" -Description "$CaseId report id"
Assert-Contains -Text $outputText -Pattern "status: pass" -Description "$CaseId report status"

$summaryPath = Join-Path $CaseOutputRoot "compare\compare-summary.json"
$reportPath = Join-Path $CaseOutputRoot "compare\compare-report.md"
$oracleEsoPath = Join-Path $CaseOutputRoot "oracle\eplusout.eso"
$oracleEioPath = Join-Path $CaseOutputRoot "oracle\eplusout.eio"
$oracleErrPath = Join-Path $CaseOutputRoot "oracle\eplusout.err"
$oracleEndPath = Join-Path $CaseOutputRoot "oracle\eplusout.end"
foreach ($path in @($summaryPath, $reportPath, $oracleEsoPath, $oracleEioPath, $oracleErrPath, $oracleEndPath)) {
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Missing $CaseId comparison artifact: $path"
    }
}

$summary = Get-Content -LiteralPath $summaryPath -Raw -Encoding UTF8 | ConvertFrom-Json
if ($summary.case_id -cne $CaseId -or $summary.oracle_version -cne "26.1.0" -or
    $summary.comparison_class -cne "conformance" -or $summary.conformance_claim -ne $true -or
    $summary.status -cne "pass" -or $summary.time_axis_samples -ne 72 -or
    $summary.series_count -ne 1 -or $summary.conformance_series_count -ne 1 -or
    $summary.gate.script -cne "scripts/dev.cmd compare-calendar-dst-epw-idf-precedence-exact" -or
    $summary.gate.blocking -ne $true) {
    throw "$CaseId must retain its exact passing single-series conformance and blocking-gate contract"
}
$calendar = $summary.weather_calendar
if ($calendar.policy_applied -ne $true -or $calendar.weather_file_allows_leap_years -ne $true -or
    $calendar.gregorian_calendar_days -ne 3 -or $calendar.weather_effective_calendar_days -ne 3 -or
    $calendar.leap_days_skipped -ne 0 -or $calendar.start_year_gregorian_leap -ne $true -or
    $calendar.start_year_weather_effective_leap -ne $true) {
    throw "Unexpected $CaseId three-day leap-year weather calendar state"
}
$daylightSaving = $calendar.daylight_saving
if ($daylightSaving.weather_file_period_declared -ne $true -or
    $daylightSaving.run_period_uses_weather_file_period -ne $false -or
    $daylightSaving.input_file_period_declared -ne $true -or
    $daylightSaving.active -ne $true -or
    $daylightSaving.effective_source -cne "input-file") {
    throw "Unexpected $CaseId weather-file/use/input-file/active/effective-source daylight-saving state"
}
$resolvedPeriod = $daylightSaving.resolved_period
if ($null -eq $resolvedPeriod -or
    $resolvedPeriod.start_month -ne 2 -or $resolvedPeriod.start_day -ne 28 -or
    $resolvedPeriod.start_day_of_year -ne 59 -or
    $resolvedPeriod.end_month -ne 2 -or $resolvedPeriod.end_day -ne 29 -or
    $resolvedPeriod.end_day_of_year -ne 60 -or $resolvedPeriod.wraps_year -ne $false -or
    $calendar.daylight_saving_hourly_samples -ne 48) {
    throw "Unexpected $CaseId input-file daylight-saving resolved period or active-sample count"
}

$specialDays = $summary.special_days
if ($specialDays.weather_file_declared -ne 0 -or $specialDays.run_period_uses_weather_file -ne $false -or
    $specialDays.weather_file_resolved -ne 0 -or $specialDays.input_file_declared -ne 0 -or
    $specialDays.apply_weekend_rule -ne $false -or $specialDays.resolved_count -ne 0 -or
    $specialDays.hourly_samples -ne 0) {
    throw "$CaseId must not activate holidays or input-file special days"
}
$selection = $summary.weather_record_selection
if ($selection.applied -ne $true -or $selection.data_period_index -ne 1 -or
    $selection.source_start_record_index -ne 0 -or $selection.initial_tomorrow_source_record_index -ne 0 -or
    $selection.selected_hourly_records -ne 72 -or $selection.skipped_raw_february_29_days -ne 0 -or
    $selection.day_buffer_transitions -ne 3) {
    throw "Unexpected $CaseId weather record selection state"
}

$seriesRows = @($summary.series | Where-Object {
    $_.key -eq "ENVIRONMENT" -and $_.variable -eq "Site Daylight Saving Time Status"
})
if ($seriesRows.Count -ne 1) {
    throw "Missing unique Site Daylight Saving Time Status series for $CaseId"
}
$series = $seriesRows[0]
if ($series.level -cne "conformance" -or $series.class -cne "weather" -or
    $series.frequency -cne "hourly" -or $series.source -cne "eso" -or
    $series.alignment -cne "timestamp" -or
    $series.expected_samples -ne 72 -or $series.observed_samples -ne 72 -or
    $series.compared_samples -ne 72 -or
    $series.timestamp_contract -cne "ordered-exact-unique" -or
    $series.timestamp_status -cne "pass" -or
    $series.timestamp_expected_unique -ne $true -or $series.timestamp_observed_unique -ne $true -or
    $series.timestamp_order_match -ne $true) {
    throw "Exact Site Daylight Saving Time Status series metadata failed for $CaseId"
}
if ($series.expected_first_timestamp -cne $ExpectedFirstTimestamp -or
    $series.observed_first_timestamp -cne $ExpectedFirstTimestamp -or
    $series.expected_last_timestamp -cne $ExpectedLastTimestamp -or
    $series.observed_last_timestamp -cne $ExpectedLastTimestamp -or
    $series.max_abs_tolerance -ne 0.0 -or $series.max_rel_tolerance -ne 0.0 -or
    $series.max_rmse_tolerance -ne 0.0 -or $series.max_abs_delta -ne 0.0 -or
    $series.rmse_delta -ne 0.0 -or $series.max_rel_delta -ne 0.0 -or
    $null -ne $series.first_divergence -or $null -ne $series.first_timestamp_divergence -or
    $series.status -cne "pass") {
    throw "Ordered exact Site Daylight Saving Time Status values or timestamps failed for $CaseId"
}

$oracleEsoLines = Get-Content -LiteralPath $oracleEsoPath -Encoding UTF8
$dictionaryRows = @($oracleEsoLines | Where-Object {
    $_ -match '^\d+,1,Environment,Site Daylight Saving Time Status \[\] !Hourly$'
})
if ($dictionaryRows.Count -ne 1) {
    throw "Expected one exact Site Daylight Saving Time Status ESO dictionary row for $CaseId"
}
$dictionaryMatch = [regex]::Match([string]$dictionaryRows[0], '^(\d+),')
if (-not $dictionaryMatch.Success) {
    throw "Missing Site Daylight Saving Time Status ESO report id for $CaseId"
}
$reportId = $dictionaryMatch.Groups[1].Value
$valueRows = @($oracleEsoLines | Where-Object { $_ -match ('^' + $reportId + ',\s*[-+0-9.E]+\s*$') })
$values = @($valueRows | ForEach-Object { [double](($_ -split ',', 2)[1].Trim()) })
$timestampRows = @($oracleEsoLines | Where-Object { $_ -match '^2,\d+,\s*\d+,\s*\d+,\s*\d+,\s*\d+,' })
if ($values.Count -ne 72 -or $timestampRows.Count -ne 72) {
    throw "Expected 72 raw oracle daylight-saving values and timestamps for $CaseId"
}
for ($index = 0; $index -lt 72; ++$index) {
    $dayOffset = [int][math]::Floor($index / 24)
    $expectedHour = ($index % 24) + 1
    $expectedDstValue = [int]$ExpectedDailyDst[$dayOffset]
    if ($values[$index] -ne [double]$expectedDstValue) {
        throw "Unexpected $CaseId oracle Site Daylight Saving Time Status at sample $index`: $($values[$index])"
    }
    $timestampMatch = [regex]::Match(
        $timestampRows[$index],
        '^2,\s*(\d+),\s*(\d+),\s*(\d+),\s*(\d+),\s*(\d+),\s*([-+0-9.]+),\s*([-+0-9.]+),([^,]+)$'
    )
    if (-not $timestampMatch.Success -or
        [int]$timestampMatch.Groups[1].Value -ne ($dayOffset + 1) -or
        [int]$timestampMatch.Groups[2].Value -ne $ExpectedMonths[$dayOffset] -or
        [int]$timestampMatch.Groups[3].Value -ne $ExpectedDates[$dayOffset] -or
        [int]$timestampMatch.Groups[4].Value -ne $expectedDstValue -or
        [int]$timestampMatch.Groups[5].Value -ne $expectedHour -or
        $timestampMatch.Groups[6].Value -cne "0.00" -or
        $timestampMatch.Groups[7].Value -cne "60.00" -or
        $timestampMatch.Groups[8].Value.Trim() -cne $ExpectedDayTypes[$dayOffset]) {
        throw "Unexpected $CaseId oracle daylight-saving timestamp identity at sample $index`: $($timestampRows[$index])"
    }
}

$oracleEioLines = Get-Content -LiteralPath $oracleEioPath -Encoding UTF8
$environmentEioRows = @($oracleEioLines | Where-Object { $_ -match '^Environment,' })
if ($environmentEioRows.Count -ne 1 -or
    [string]$environmentEioRows[0] -cne $ExpectedEnvironmentEio -or
    @(([string]$environmentEioRows[0]) -split ',').Count -ne 14) {
    throw "Expected exact 14-field EnergyPlus Environment EIO row for $CaseId"
}
$daylightSavingEioRows = @($oracleEioLines | Where-Object { $_ -match '^Environment:Daylight Saving,' })
if ($daylightSavingEioRows.Count -ne 1 -or
    [string]$daylightSavingEioRows[0] -cne $ExpectedDaylightSavingEio -or
    @(([string]$daylightSavingEioRows[0]) -split ',').Count -ne 5) {
    throw "Expected exact five-field EnergyPlus input-file Environment:Daylight Saving EIO row for $CaseId"
}

$oracleErrText = Get-Content -LiteralPath $oracleErrPath -Raw -Encoding UTF8
Assert-Contains -Text $oracleErrText -Pattern "EnergyPlus Completed Successfully-- 0 Warning; 0 Severe Errors;" -Description "$CaseId clean EnergyPlus completion"
if ([regex]::Matches($oracleErrText, '(?m)^\s*\*\* Warning \*\*').Count -ne 0 -or
    [regex]::Matches($oracleErrText, '(?m)^\s*\*\* Severe\s+\*\*').Count -ne 0) {
    throw "$CaseId oracle must complete without warning or severe markers"
}
$oracleEndText = Get-Content -LiteralPath $oracleEndPath -Raw -Encoding UTF8
Assert-Contains -Text $oracleEndText -Pattern "EnergyPlus Completed Successfully-- 0 Warning; 0 Severe Errors;" -Description "$CaseId clean EnergyPlus end record"

$reportText = Get-Content -LiteralPath $reportPath -Raw -Encoding UTF8
foreach ($entry in @(
    "weather_file_daylight_saving_period_declared: true",
    "run_period_uses_weather_file_daylight_saving_period: false",
    "input_file_daylight_saving_period_declared: true",
    "daylight_saving_active: true",
    "daylight_saving_effective_source: input-file",
    "daylight_saving_resolved_period: 2/28 through 2/29 (wraps_year=false)",
    "daylight_saving_hourly_samples: 48",
    "weather_file_holidays_declared: 0",
    "special_days_resolved: 0",
    "weather_selected_hourly_records: 72",
    "weather_day_buffer_transitions: 3"
)) {
    Assert-Contains -Text $reportText -Pattern $entry -Description "$CaseId markdown fixed-date precedence state"
}

Write-Host "Fixed-date EPW-versus-IDF daylight-saving precedence exact gate passed."
