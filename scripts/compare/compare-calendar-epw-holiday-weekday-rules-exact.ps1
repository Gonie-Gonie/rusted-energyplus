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
$CaseId = "calendar_epw_holiday_weekday_rules_hourly_exact_001"
$CaseRoot = Join-Path $RepoRoot "data\conformance_cases\$CaseId"
$CasePath = Join-Path $CaseRoot "case.toml"
$IdfName = "calendar_epw_holiday_weekday_rules_hourly_exact.idf"
$WeatherName = "calendar_epw_holiday_weekday_rules_hourly_exact.epw"
$IdfPath = Join-Path $CaseRoot $IdfName
$WeatherPath = Join-Path $CaseRoot $WeatherName
$CaseOutputRoot = Join-Path $OutputRoot $CaseId
$IdfRef = "data/conformance_cases/$CaseId/$IdfName"
$WeatherRef = "data/conformance_cases/$CaseId/$WeatherName"
$ExpectedHeader = "HOLIDAYS/DAYLIGHT SAVINGS,Yes,0,0,2,Fourth Monday EPW Holiday,4th Monday in February,Last Wednesday EPW Holiday,Last Wednesday in February"
$ExpectedFirstTimestamp = "env=EPW HOLIDAY WEEKDAY RULES RUN PERIOD;day=1;month=2;date=23;dst=0;hour=1;start=0.00;end=60.00;day_type=Sunday"
$ExpectedLastTimestamp = "env=EPW HOLIDAY WEEKDAY RULES RUN PERIOD;day=3;month=2;date=25;dst=0;hour=24;start=0.00;end=60.00;day_type=Sunday"
$ExpectedResolved = @(
    [pscustomobject]@{ Name = "FOURTH MONDAY EPW HOLIDAY"; Day = 23; DayOfYear = 54 },
    [pscustomobject]@{ Name = "LAST WEDNESDAY EPW HOLIDAY"; Day = 25; DayOfYear = 56 }
)

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
        throw "Missing required EPW weekday holiday conformance file: $path"
    }
}

$caseText = Get-Content -LiteralPath $CasePath -Raw -Encoding UTF8
$idfText = Get-Content -LiteralPath $IdfPath -Raw -Encoding UTF8
$weatherLines = Get-Content -LiteralPath $WeatherPath -Encoding UTF8
$weatherText = $weatherLines -join "`n"

Assert-Contains -Text $caseText -Pattern "timestamp_contract = `"ordered-exact-unique`"" -Description "ordered timestamp contract"
Assert-Contains -Text $caseText -Pattern "abs_tol = 0.0" -Description "zero absolute tolerance"
Assert-Contains -Text $caseText -Pattern "rmse_tol = 0.0" -Description "zero RMSE tolerance"
Assert-Contains -Text $caseText -Pattern "idf = `"$IdfRef`"" -Description "manifest input.idf attribution"
Assert-Contains -Text $caseText -Pattern "weather = `"$WeatherRef`"" -Description "manifest input.weather attribution"
Assert-Contains -Text $caseText -Pattern 'script = "scripts/dev.cmd compare-calendar-epw-holiday-weekday-rules-exact"' -Description "manifest gate attribution"
Assert-Contains -Text $caseText -Pattern "blocking = true" -Description "manifest blocking flag"

$calendarHeaders = @($weatherLines | Where-Object { $_ -match '^\s*HOLIDAYS/DAYLIGHT SAVING' })
if ($calendarHeaders.Count -ne 1 -or $calendarHeaders[0] -cne $ExpectedHeader) {
    throw "EPW must contain exactly the expected source-ordered two-rule holiday header"
}
Assert-Contains -Text $weatherText -Pattern "DATA PERIODS,1,1,Data,Monday,2/23,2/25" -Description "three-day Monday-start data period"
$weatherRows = @($weatherLines | Select-Object -Skip 8 | Where-Object { -not [string]::IsNullOrWhiteSpace($_) })
if ($weatherRows.Count -ne 72) {
    throw "EPW weekday holiday fixture must contain 72 hourly rows, found $($weatherRows.Count)"
}
$weatherPayloads = @()
foreach ($date in @("2032,2,23", "2032,2,24", "2032,2,25")) {
    $dateRows = @($weatherRows | Where-Object { $_ -match ('^' + [regex]::Escape($date) + ',') })
    if ($dateRows.Count -ne 24) {
        throw "EPW weekday holiday fixture must contain 24 rows for $date, found $($dateRows.Count)"
    }
    $hours = @($dateRows | ForEach-Object { [int](($_ -split ',')[3]) })
    if (($hours -join ',') -cne ((1..24) -join ',')) {
        throw "EPW weekday holiday fixture must contain ordered hours 1..24 for $date"
    }
    foreach ($row in $dateRows) {
        $fields = $row -split ','
        if ($fields.Count -ne 35 -or $fields[4] -ne "60") {
            throw "EPW weekday holiday rows must contain 35 fields and minute 60"
        }
        $weatherPayloads += ($fields[5..34] -join ',')
    }
}
$weatherPayloads = @($weatherPayloads | Select-Object -Unique)
if ($weatherPayloads.Count -ne 1) {
    throw "EPW weekday holiday weather payload must remain constant across all 72 rows"
}

foreach ($policy in @(
    "Yes, !- Use Weather File Holidays and Special Days",
    "No,  !- Use Weather File Daylight Saving Period",
    "No,  !- Apply Weekend Holiday Rule",
    "No,  !- Use Weather File Rain Indicators",
    "No,  !- Use Weather File Snow Indicators",
    "No;  !- Treat Weather as Actual"
)) {
    Assert-Contains -Text $idfText -Pattern $policy -Description "explicit RunPeriod policy"
}
Assert-Contains -Text $idfText -Pattern "EPW Holiday Weekday Rules Run Period" -Description "RunPeriod identity"
$runPeriodObjects = [regex]::Matches($idfText, '(?ims)^\s*RunPeriod\s*,(?<body>.*?);')
if ($runPeriodObjects.Count -ne 1 -or
    [regex]::Matches($idfText, '(?im)^\s*Output:Variable\s*,').Count -ne 1) {
    throw "EPW weekday holiday fixture must contain exactly one RunPeriod and one Output:Variable"
}
$runPeriodBody = [regex]::Replace($runPeriodObjects[0].Groups["body"].Value, '(?m)!-.*$', '')
$runPeriodFields = @($runPeriodBody -split ',' | ForEach-Object { $_.Trim() } | Where-Object { $_ -ne "" })
$expectedRunPeriodFields = @(
    "EPW Holiday Weekday Rules Run Period",
    "2", "23", "2032", "2", "25", "2032", "Monday",
    "Yes", "No", "No", "No", "No", "No"
)
if (($runPeriodFields -join '|') -cne ($expectedRunPeriodFields -join '|')) {
    throw "EPW weekday holiday fixture must retain the exact 2032-02-23 through 2032-02-25 Monday RunPeriod and explicit policies"
}
if ([regex]::Matches($idfText, '(?im)^\s*RunPeriodControl:SpecialDays\s*,').Count -ne 0) {
    throw "EPW weekday holiday fixture must not contain RunPeriodControl:SpecialDays"
}
if ([regex]::Matches($idfText, '(?im)^\s*RunPeriodControl:DaylightSavingTime\s*,').Count -ne 0) {
    throw "EPW weekday holiday fixture must not contain RunPeriodControl:DaylightSavingTime"
}

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Remove-RepoDirectory -Path $CaseOutputRoot
Write-Host "Running exact EPW fourth-Monday and last-Wednesday holiday rule gate."
$output = & $cargo.Source run -p ep_cli --quiet -- conformance time-weather-schedule-report $CasePath $OracleRoot $OutputRoot 2>&1
if ($LASTEXITCODE -ne 0) {
    $output | ForEach-Object { Write-Host $_ }
    throw "EPW weekday holiday rule case failed: $CaseId"
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
        throw "Missing EPW weekday holiday comparison artifact: $path"
    }
}

$summary = Get-Content -LiteralPath $summaryPath -Raw -Encoding UTF8 | ConvertFrom-Json
if ($summary.status -ne "pass" -or $summary.conformance_claim -ne $true -or $summary.time_axis_samples -ne 72) {
    throw "EPW weekday holiday case must be passing 72-sample conformance evidence"
}
$specialDays = $summary.special_days
if ($specialDays.weather_file_declared -ne 2 -or
    $specialDays.run_period_uses_weather_file -ne $true -or
    $specialDays.weather_file_resolved -ne 2 -or
    $specialDays.input_file_declared -ne 0 -or
    $specialDays.apply_weekend_rule -ne $false -or
    $specialDays.resolved_count -ne 2 -or
    $specialDays.hourly_samples -ne 48) {
    throw "Unexpected EPW weekday holiday JSON state"
}
$resolved = @($specialDays.resolved)
if ($resolved.Count -ne 2) {
    throw "Expected exactly two resolved EPW weekday holidays"
}
for ($index = 0; $index -lt 2; ++$index) {
    $actual = $resolved[$index]
    $expected = $ExpectedResolved[$index]
    if ($actual.name -cne $expected.Name -or
        $actual.source -ne "weather-file" -or
        $actual.start_month -ne 2 -or $actual.start_day -ne $expected.Day -or
        $actual.start_day_of_year -ne $expected.DayOfYear -or
        $actual.duration_days -ne 1 -or $actual.day_type -ne "Sunday" -or
        $actual.day_type_index -ne 1 -or $actual.weekend_shift_days -ne 0) {
        throw "Unexpected resolved EPW weekday holiday at index $index"
    }
}

$seriesRows = @($summary.series | Where-Object { $_.key -eq "ENVIRONMENT" -and $_.variable -eq "Site Day Type Index" })
if ($seriesRows.Count -ne 1) {
    throw "Missing unique Site Day Type Index series"
}
$series = $seriesRows[0]
if ($series.level -cne "conformance" -or $series.class -cne "weather" -or
    $series.frequency -cne "hourly" -or $series.source -cne "eso" -or
    $series.alignment -cne "timestamp" -or
    $series.expected_samples -ne 72 -or $series.observed_samples -ne 72 -or $series.compared_samples -ne 72 -or
    $series.timestamp_contract -ne "ordered-exact-unique" -or $series.timestamp_status -ne "pass" -or
    $series.timestamp_expected_unique -ne $true -or $series.timestamp_observed_unique -ne $true -or
    $series.timestamp_order_match -ne $true -or
    $series.expected_first_timestamp -cne $ExpectedFirstTimestamp -or $series.observed_first_timestamp -cne $ExpectedFirstTimestamp -or
    $series.expected_last_timestamp -cne $ExpectedLastTimestamp -or $series.observed_last_timestamp -cne $ExpectedLastTimestamp -or
    $series.max_abs_tolerance -ne 0.0 -or $series.max_rel_tolerance -ne 0.0 -or $series.max_rmse_tolerance -ne 0.0 -or
    $series.max_abs_delta -ne 0.0 -or $series.rmse_delta -ne 0.0 -or $series.max_rel_delta -ne 0.0 -or
    $series.status -ne "pass") {
    throw "Ordered exact Site Day Type Index contract failed"
}

$oracleEsoLines = Get-Content -LiteralPath $oracleEsoPath -Encoding UTF8
$dictionaryRows = @($oracleEsoLines | Where-Object { $_ -match '^\d+,\d+,Environment,Site Day Type Index' })
if ($dictionaryRows.Count -ne 1) {
    throw "Expected one Site Day Type Index ESO dictionary entry"
}
$dictionaryMatch = [regex]::Match([string]$dictionaryRows[0], '^(\d+),')
if (-not $dictionaryMatch.Success) {
    throw "Missing Site Day Type Index ESO report id"
}
$reportId = $dictionaryMatch.Groups[1].Value
$valueRows = @($oracleEsoLines | Where-Object { $_ -match ('^' + $reportId + ',\s*[-+0-9.E]+\s*$') })
$values = @($valueRows | ForEach-Object { [double](($_ -split ',', 2)[1].Trim()) })
$timestampRows = @($oracleEsoLines | Where-Object { $_ -match '^2,\d+,\s*\d+,\s*\d+,\s*\d+,\s*\d+,' })
if ($values.Count -ne 72 -or $timestampRows.Count -ne 72) {
    throw "Expected 72 oracle day-type values and timestamps"
}
for ($index = 0; $index -lt 72; ++$index) {
    $dayOffset = [int][math]::Floor($index / 24)
    $expectedValue = @(1.0, 3.0, 1.0)[$dayOffset]
    $expectedLabel = @("Sunday", "Tuesday", "Sunday")[$dayOffset]
    $expectedDay = @(23, 24, 25)[$dayOffset]
    $expectedHour = ($index % 24) + 1
    if ($values[$index] -ne $expectedValue) {
        throw "Unexpected oracle Site Day Type Index at sample $index`: $($values[$index])"
    }
    $timestampMatch = [regex]::Match(
        $timestampRows[$index],
        '^2,\s*(\d+),\s*(\d+),\s*(\d+),\s*(\d+),\s*(\d+),\s*([-+0-9.]+),\s*([-+0-9.]+),([^,]+)$'
    )
    if (-not $timestampMatch.Success -or
        [int]$timestampMatch.Groups[1].Value -ne ($dayOffset + 1) -or
        [int]$timestampMatch.Groups[2].Value -ne 2 -or
        [int]$timestampMatch.Groups[3].Value -ne $expectedDay -or
        [int]$timestampMatch.Groups[4].Value -ne 0 -or
        [int]$timestampMatch.Groups[5].Value -ne $expectedHour -or
        $timestampMatch.Groups[6].Value -cne "0.00" -or
        $timestampMatch.Groups[7].Value -cne "60.00" -or
        $timestampMatch.Groups[8].Value.Trim() -cne $expectedLabel) {
        throw "Unexpected oracle day-type timestamp at sample $index`: $($timestampRows[$index])"
    }
}

$oracleEioLines = Get-Content -LiteralPath $oracleEioPath -Encoding UTF8
$specialDayEioRows = @($oracleEioLines | Where-Object { $_ -match '^Environment:Special Days,' })
if ($specialDayEioRows.Count -ne 2) {
    throw "Expected exactly two EnergyPlus EIO Special Days rows"
}
for ($index = 0; $index -lt 2; ++$index) {
    $fields = $specialDayEioRows[$index] -split ','
    if ($fields.Count -lt 4 -or $fields[1] -cne $ExpectedResolved[$index].Name -or
        $fields[2] -cne "Sunday" -or $fields[3] -cne "WeatherFile") {
        throw "Unexpected EnergyPlus EIO Special Days name/type/source order at index $index"
    }
}

$oracleErrText = Get-Content -LiteralPath $oracleErrPath -Raw -Encoding UTF8
Assert-Contains -Text $oracleErrText -Pattern "EnergyPlus Completed Successfully-- 0 Warning; 0 Severe Errors;" -Description "clean EnergyPlus completion"
if ([regex]::Matches($oracleErrText, '(?m)^\s*\*\* Warning \*\*').Count -ne 0 -or
    [regex]::Matches($oracleErrText, '(?m)^\s*\*\* Severe\s+\*\*').Count -ne 0) {
    throw "EPW weekday holiday oracle must complete without warning or severe markers"
}
$oracleEndText = Get-Content -LiteralPath $oracleEndPath -Raw -Encoding UTF8
Assert-Contains -Text $oracleEndText -Pattern "EnergyPlus Completed Successfully-- 0 Warning; 0 Severe Errors;" -Description "clean EnergyPlus end record"

$reportText = Get-Content -LiteralPath $reportPath -Raw -Encoding UTF8
foreach ($entry in @(
    "weather_file_holidays_declared: 2",
    "run_period_uses_weather_file_holidays: true",
    "weather_file_holidays_resolved: 2",
    "input_file_special_days_declared: 0",
    "special_day_weekend_rule: false",
    "special_days_resolved: 2",
    "special_day_hourly_samples: 48"
)) {
    Assert-Contains -Text $reportText -Pattern $entry -Description "markdown special-day state"
}
$resolvedReportRows = @($reportText -split "`r?`n" | Where-Object { $_ -match '^special_day_resolved:' })
$expectedReportRows = @(
    "special_day_resolved: FOURTH MONDAY EPW HOLIDAY 2/23 duration=1 day_type=Sunday weekend_shift_days=0 source=weather-file",
    "special_day_resolved: LAST WEDNESDAY EPW HOLIDAY 2/25 duration=1 day_type=Sunday weekend_shift_days=0 source=weather-file"
)
if ($resolvedReportRows.Count -ne 2 -or
    $resolvedReportRows[0] -cne $expectedReportRows[0] -or
    $resolvedReportRows[1] -cne $expectedReportRows[1]) {
    throw "Markdown resolved EPW weekday holidays must retain exact header order"
}

Write-Host "Exact EPW fourth-Monday and last-Wednesday holiday rule gate passed."
