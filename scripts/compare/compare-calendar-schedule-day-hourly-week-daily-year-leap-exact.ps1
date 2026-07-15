[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$CaseId = "calendar_schedule_day_hourly_week_daily_year_leap_exact_001"
$CaseRoot = Join-Path $RepoRoot "data\conformance_cases\$CaseId"
$CasePath = Join-Path $CaseRoot "case.toml"
$IdfPath = Join-Path $CaseRoot "calendar_schedule_day_hourly_week_daily_year_leap_exact.idf"
$WeatherPath = Join-Path $CaseRoot "calendar_schedule_day_hourly_week_daily_year_leap_exact.epw"
$GateCommand = "scripts/dev.cmd compare-calendar-schedule-day-hourly-week-daily-year-leap-exact"
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
        throw "Missing $($Description): $Pattern"
    }
    Write-Host "OK $($Description): $Pattern"
}

function Get-CompleteIdfObjectVectors {
    param(
        [Parameter(Mandatory = $true)][string]$Text,
        [Parameter(Mandatory = $true)][string]$Description
    )
    $withoutComments = [regex]::Replace($Text, '(?m)!.*$', '')
    $chunks = @($withoutComments -split ';' | ForEach-Object { $_.Trim() } | Where-Object { $_.Length -gt 0 })
    if ($chunks.Count -eq 0) {
        throw "$Description must contain at least one IDF object"
    }
    $vectors = @()
    foreach ($chunk in $chunks) {
        $object = [regex]::Match($chunk, '(?s)^(?<type>[A-Za-z0-9:]+)\s*,(?<body>.*)$')
        if (-not $object.Success) {
            throw "$Description contains a non-object semicolon-delimited chunk: $chunk"
        }
        $fields = @([regex]::Split($object.Groups["body"].Value, ',') | ForEach-Object { $_.Trim() })
        $vectors += "$($object.Groups["type"].Value)|$($fields -join '|')"
    }
    return $vectors
}

foreach ($path in @(
    (Join-Path $OracleRoot "energyplus.exe"),
    (Join-Path $OracleRoot "ConvertInputFormat.exe"),
    $CasePath,
    $IdfPath,
    $WeatherPath
)) {
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Missing required Day:Hourly/Week:Daily/Year exact-case file: $path"
    }
}

$caseText = Get-Content -LiteralPath $CasePath -Raw -Encoding UTF8
$idfText = Get-Content -LiteralPath $IdfPath -Raw -Encoding UTF8
$weatherLines = @(Get-Content -LiteralPath $WeatherPath -Encoding UTF8)
$weatherRows = @($weatherLines | Select-Object -Skip 8 | Where-Object { -not [string]::IsNullOrWhiteSpace($_) })

foreach ($contract in @(
    'comparison_class = "conformance"',
    'conformance_claim = true',
    'source_file = "data/conformance_cases/calendar_schedule_day_hourly_week_daily_year_leap_exact_001/calendar_schedule_day_hourly_week_daily_year_leap_exact.idf"',
    'idf = "data/conformance_cases/calendar_schedule_day_hourly_week_daily_year_leap_exact_001/calendar_schedule_day_hourly_week_daily_year_leap_exact.idf"',
    'weather = "data/conformance_cases/calendar_schedule_day_hourly_week_daily_year_leap_exact_001/calendar_schedule_day_hourly_week_daily_year_leap_exact.epw"',
    'frequency = "hourly"',
    'timestamp_contract = "ordered-exact-unique"',
    'abs_tol = 0.0',
    'rmse_tol = 0.0',
    'February 28 is 101 through 124, February 29 is 201 through 224, and March 1 is 301 through 324',
    'Schedule:Day:Interval',
    'Schedule:Day:List',
    'Schedule:Week:Compact',
    'diagnostic parity',
    'daylight-saving',
    'holiday',
    'today/tomorrow day-type combinations',
    'EMS override behavior',
    'currentVal state',
    'downstream schedule consumption',
    'design-day',
    'warmup',
    'actual-weather',
    'Rust raw ESO serialization',
    'broad EnergyPlus warning/error parity',
    'script = "scripts/dev.cmd compare-calendar-schedule-day-hourly-week-daily-year-leap-exact"',
    'blocking = true'
)) {
    Assert-Contains -Text $caseText -Pattern $contract -Description "canonical manifest contract"
}
if (@([regex]::Matches($caseText, '(?m)^\[\[outputs\]\]$')).Count -ne 1) {
    throw "Manifest must retain exactly one output request"
}

$dayASundayValues = @(101..124)
$dayAMondayValues = @(201..224)
$dayBTuesdayValues = @(301..324)
$weekASunday = "Week A Sunday Hourly"
$weekAMonday = "Week A Monday Hourly"
$weekBTuesday = "Week B Tuesday Hourly"
$expectedVectors = @(
    "Version|26.1",
    "Building|Schedule Day Hourly Week Daily Year Leap Exact Fixture|0.0|Suburbs|0.04|0.4|FullExterior|25|6",
    "Timestep|4",
    "GlobalGeometryRules|UpperLeftCorner|CounterClockWise|World",
    "RunPeriod|Schedule Day Hourly Week Daily Year Leap Run Period|2|28|2016|3|1|2016|Sunday|No|No|No|No|No|No",
    "ScheduleTypeLimits|Any Number",
    "Schedule:Day:Hourly|$weekASunday|Any Number|$($dayASundayValues -join '|')",
    "Schedule:Day:Hourly|$weekAMonday|Any Number|$($dayAMondayValues -join '|')",
    "Schedule:Day:Hourly|$weekBTuesday|Any Number|$($dayBTuesdayValues -join '|')",
    "Schedule:Week:Daily|Week A Daily|$weekASunday|$weekAMonday|$weekASunday|$weekASunday|$weekASunday|$weekASunday|$weekASunday|$weekASunday|$weekASunday|$weekASunday|$weekASunday|$weekASunday",
    "Schedule:Week:Daily|Week B Daily|$weekBTuesday|$weekBTuesday|$weekBTuesday|$weekBTuesday|$weekBTuesday|$weekBTuesday|$weekBTuesday|$weekBTuesday|$weekBTuesday|$weekBTuesday|$weekBTuesday|$weekBTuesday",
    "Schedule:Year|Year Leap Schedule|Any Number|Week A Daily|1|1|2|28|Week B Daily|3|1|12|31",
    "Output:Variable|YEAR LEAP SCHEDULE|Schedule Value|Hourly"
)
$actualVectors = @(Get-CompleteIdfObjectVectors -Text $idfText -Description "canonical fixture")
if (($actualVectors -join '||') -cne ($expectedVectors -join '||')) {
    throw "Fixture must retain the exact complete IDF object order and fields"
}
$mutated = $idfText.Replace(
    "Version,26.1;",
    "Version,26.1; Schedule:Constant,Same Line Parser Mutation,,1;"
)
$mutatedVectors = @(Get-CompleteIdfObjectVectors -Text $mutated -Description "same-line mutation")
if ($mutatedVectors.Count -ne ($expectedVectors.Count + 1) -or
    $mutatedVectors[1] -cne "Schedule:Constant|Same Line Parser Mutation||1") {
    throw "Complete IDF parser self-check did not expose the same-line injected object"
}
Write-Host "OK complete fixture IDF vectors and parser self-check"

$expectedHeaders = @(
    "LOCATION,Schedule Day Hourly Week Daily Year Leap Exact Fixture,CO,USA,Synthetic,999999,39.74,-105.18,-7.0,1829.0",
    "DESIGN CONDITIONS,0",
    "TYPICAL/EXTREME PERIODS,0",
    "GROUND TEMPERATURES,0",
    "HOLIDAYS/DAYLIGHT SAVINGS,Yes,0,0,0",
    "COMMENTS 1,Deterministic 72-hour leap-day weather fixture for Day Hourly Week Daily Year schedules",
    "COMMENTS 2,Weather values are constant because only Schedule Value is compared",
    "DATA PERIODS,1,1,Data,Sunday,2/28,3/1"
)
if ($weatherLines.Count -ne 80 -or $weatherRows.Count -ne 72 -or
    (($weatherLines[0..7] -join '||') -cne ($expectedHeaders -join '||'))) {
    throw "Fixture EPW must retain eight exact headers and 72 hourly rows"
}
$weatherPayload = "?9?9?9?9E0?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9*9*9?9*9*9,10.0,5.0,50,80600,0,0,250,0,0,0,0,0,0,0,180,2.0,5,5,20.0,7777,9,999999999,0,0.0000,0,0,0.000,0.0,0.0"
$weatherDates = @(
    [pscustomobject]@{ Month = 2; Day = 28 },
    [pscustomobject]@{ Month = 2; Day = 29 },
    [pscustomobject]@{ Month = 3; Day = 1 }
)
for ($index = 0; $index -lt 72; ++$index) {
    $date = $weatherDates[[int][Math]::Floor($index / 24)]
    $hour = ($index % 24) + 1
    $expected = "2016,$($date.Month),$($date.Day),$hour,60,$weatherPayload"
    if ($weatherRows[$index] -cne $expected) {
        throw "Unexpected EPW row at index $($index): $($weatherRows[$index])"
    }
}
Write-Host "OK exact 72-hour leap-observed EPW"

Remove-RepoDirectory -Path $CaseOutputRoot
$cargo = Get-Command cargo -ErrorAction Stop
$output = & $cargo.Source run -p ep_cli --quiet -- conformance time-weather-schedule-report $CasePath $OracleRoot $OutputRoot 2>&1
if ($LASTEXITCODE -ne 0) {
    $joinedOutput = $output -join [Environment]::NewLine
    throw "Day:Hourly/Week:Daily/Year report failed with exit code $LASTEXITCODE $joinedOutput"
}
$output | ForEach-Object { Write-Host $_ }

$summaryPath = Join-Path $CaseOutputRoot "compare\compare-summary.json"
$reportPath = Join-Path $CaseOutputRoot "compare\compare-report.md"
$oracleEsoPath = Join-Path $CaseOutputRoot "oracle\eplusout.eso"
$oracleEioPath = Join-Path $CaseOutputRoot "oracle\eplusout.eio"
$oracleErrPath = Join-Path $CaseOutputRoot "oracle\eplusout.err"
$oracleEndPath = Join-Path $CaseOutputRoot "oracle\eplusout.end"
foreach ($path in @(
    $summaryPath,
    $reportPath,
    $oracleEsoPath,
    $oracleEioPath,
    $oracleErrPath,
    $oracleEndPath
)) {
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Missing Day:Hourly/Week:Daily/Year report artifact: $path"
    }
}

$summary = Get-Content -LiteralPath $summaryPath -Raw -Encoding UTF8 | ConvertFrom-Json
if ($summary.case_id -cne $CaseId -or $summary.oracle_version -cne "26.1.0" -or
    $summary.comparison_class -cne "conformance" -or $summary.conformance_claim -ne $true -or
    $summary.status -cne "pass" -or $summary.series_count -ne 1 -or
    $summary.conformance_series_count -ne 1 -or $summary.time_axis_samples -ne 72 -or
    $summary.timestamp_rule -cne "hour-ending hourly samples aligned by EnergyPlus ESO timestamp labels" -or
    $summary.gate.script -cne $GateCommand -or $summary.gate.blocking -ne $true) {
    throw "Unexpected Day:Hourly/Week:Daily/Year report summary contract"
}
if ($null -ne $summary.weather_record_selection) {
    throw "Schedule-only comparison must not claim Rust EPW record selection"
}
$seriesRows = @($summary.series | Where-Object {
    $_.key -eq "YEAR LEAP SCHEDULE" -and $_.variable -eq "Schedule Value"
})
if ($seriesRows.Count -ne 1) {
    throw "Expected exactly one Schedule:Year Schedule Value series"
}
$series = $seriesRows[0]
if ($series.level -cne "conformance" -or $series.class -cne "schedule" -or
    $series.frequency -cne "hourly" -or $series.source -cne "eso" -or
    $series.alignment -cne "timestamp" -or $series.expected_samples -ne 72 -or
    $series.observed_samples -ne 72 -or $series.compared_samples -ne 72 -or
    $series.timestamp_contract -cne "ordered-exact-unique" -or
    $series.timestamp_status -cne "pass" -or $series.timestamp_expected_unique -ne $true -or
    $series.timestamp_observed_unique -ne $true -or $series.timestamp_order_match -ne $true) {
    throw "Unexpected Schedule:Year series metadata, count, or timestamp contract"
}
$firstTimestamp = "env=SCHEDULE DAY HOURLY WEEK DAILY YEAR LEAP RUN PERIOD;day=1;month=2;date=28;dst=0;hour=1;start=0.00;end=60.00;day_type=Sunday"
$lastTimestamp = "env=SCHEDULE DAY HOURLY WEEK DAILY YEAR LEAP RUN PERIOD;day=3;month=3;date=1;dst=0;hour=24;start=0.00;end=60.00;day_type=Tuesday"
if ($series.expected_first_timestamp -cne $firstTimestamp -or
    $series.observed_first_timestamp -cne $firstTimestamp -or
    $series.expected_last_timestamp -cne $lastTimestamp -or
    $series.observed_last_timestamp -cne $lastTimestamp -or
    $series.max_abs_tolerance -ne 0.0 -or $series.max_rel_tolerance -ne 0.0 -or
    $series.max_rmse_tolerance -ne 0.0 -or $series.max_abs_delta -ne 0.0 -or
    $series.rmse_delta -ne 0.0 -or $series.max_rel_delta -ne 0.0 -or
    $null -ne $series.first_divergence -or $null -ne $series.first_timestamp_divergence -or
    $series.status -cne "pass") {
    throw "Schedule:Year values and timestamps must match exactly at zero tolerance"
}

$esoLines = @(Get-Content -LiteralPath $oracleEsoPath -Encoding UTF8)
$dictionaryRows = @($esoLines | Where-Object {
    $_ -ceq "7,1,YEAR LEAP SCHEDULE,Schedule Value [] !Hourly"
})
$values = @($esoLines | Where-Object { $_ -match '^7,\s*[-+0-9.E]+\s*$' } |
    ForEach-Object { [double](($_ -split ',', 2)[1].Trim()) })
$timestamps = @($esoLines | Where-Object { $_ -match '^2,\d+,\s*\d+,\s*\d+,\s*\d+,\s*\d+,' })
if ($dictionaryRows.Count -ne 1 -or $values.Count -ne 72 -or $timestamps.Count -ne 72) {
    throw "Expected one exact Hourly dictionary and 72 raw values/timestamps"
}
$expectedMonths = @(2, 2, 3)
$expectedDates = @(28, 29, 1)
$expectedDayTypes = @("Sunday", "Monday", "Tuesday")
$expectedBases = @(101.0, 201.0, 301.0)
for ($index = 0; $index -lt 72; ++$index) {
    $dayIndex = [int][Math]::Floor($index / 24)
    $hourIndex = $index % 24
    $hour = $hourIndex + 1
    $expectedValue = $expectedBases[$dayIndex] + $hourIndex
    if ($values[$index] -ne $expectedValue) {
        throw "Unexpected raw Schedule:Year value at sample $($index): $($values[$index])"
    }
    $timestampMatch = [regex]::Match(
        $timestamps[$index],
        '^2,\s*(\d+),\s*(\d+),\s*(\d+),\s*0,\s*(\d+),\s*0\.00,\s*60\.00,(Sunday|Monday|Tuesday)$'
    )
    if (-not $timestampMatch.Success -or
        [int]$timestampMatch.Groups[1].Value -ne ($dayIndex + 1) -or
        [int]$timestampMatch.Groups[2].Value -ne $expectedMonths[$dayIndex] -or
        [int]$timestampMatch.Groups[3].Value -ne $expectedDates[$dayIndex] -or
        [int]$timestampMatch.Groups[4].Value -ne $hour -or
        $timestampMatch.Groups[5].Value -cne $expectedDayTypes[$dayIndex]) {
        throw "Unexpected raw Schedule:Year timestamp at sample $($index): $($timestamps[$index])"
    }
}
if ($values[0] -ne 101.0 -or $values[23] -ne 124.0 -or
    $values[24] -ne 201.0 -or $values[47] -ne 224.0 -or
    $values[48] -ne 301.0 -or $values[71] -ne 324.0) {
    throw "Each day must retain its exact 24-hour Day:Hourly profile"
}
Write-Host "OK exact raw ESO Day:Hourly values, Week:Daily day types, Year leap pointer, and timestamps"

$eioLines = @(Get-Content -LiteralPath $oracleEioPath -Encoding UTF8)
$environmentRow = "Environment,SCHEDULE DAY HOURLY WEEK DAILY YEAR LEAP RUN PERIOD,WeatherFileRunPeriod,02/28/2016,03/01/2016,Sunday,3,Use RunPeriod Specified Day,No,No,No,No,No,Clark and Allen"
$daylightRow = "Environment:Daylight Saving,No,RunPeriod Object"
if (@($eioLines | Where-Object { $_ -ceq $environmentRow }).Count -ne 1 -or
    @($eioLines | Where-Object { $_ -ceq $daylightRow }).Count -ne 1) {
    throw "Unexpected exact Environment or disabled daylight-saving EIO row"
}
$completion = "EnergyPlus Completed Successfully-- 0 Warning; 0 Severe Errors;"
$errText = Get-Content -LiteralPath $oracleErrPath -Raw -Encoding UTF8
$endText = Get-Content -LiteralPath $oracleEndPath -Raw -Encoding UTF8
if ([regex]::Matches($errText, [regex]::Escape($completion)).Count -ne 1 -or
    [regex]::Matches($endText, [regex]::Escape($completion)).Count -ne 1) {
    throw "EnergyPlus ERR and END must each contain the exact clean completion"
}
Write-Host "OK exact EIO environment and clean EnergyPlus completion"

$reportText = Get-Content -LiteralPath $reportPath -Raw -Encoding UTF8
foreach ($reportContract in @(
    "status: pass",
    "series: 1",
    "conformance_series: 1",
    "time_axis_samples: 72",
    "timestamp_rule: hour-ending hourly samples aligned by EnergyPlus ESO timestamp labels",
    "weather_record_selection_applied: false",
    "| YEAR LEAP SCHEDULE | Schedule Value | conformance | schedule | hourly | eso | timestamp | 72 | 72 | 72 | 0.000000000000 | 0.000000000000 | 0.000000000000 |"
)) {
    Assert-Contains -Text $reportText -Pattern $reportContract -Description "markdown Day:Hourly/Week:Daily/Year contract"
}

Write-Host "Schedule:Day:Hourly/Week:Daily/Year leap-day exact gate passed."
