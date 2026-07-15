[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$CaseId = "calendar_schedule_week_compact_day_types_exact_001"
$CaseRoot = Join-Path $RepoRoot "data\conformance_cases\$CaseId"
$CasePath = Join-Path $CaseRoot "case.toml"
$IdfPath = Join-Path $CaseRoot "calendar_schedule_week_compact_day_types_exact.idf"
$WeatherPath = Join-Path $CaseRoot "calendar_schedule_week_compact_day_types_exact.epw"
$GateCommand = "scripts/dev.cmd compare-calendar-schedule-week-compact-day-types-exact"
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

function Assert-NotContains {
    param(
        [Parameter(Mandatory = $true)][string]$Text,
        [Parameter(Mandatory = $true)][string]$Pattern,
        [Parameter(Mandatory = $true)][string]$Description
    )
    if ($Text -match [regex]::Escape($Pattern)) {
        throw "Unexpected $($Description): $Pattern"
    }
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

foreach ($requiredPath in @(
    (Join-Path $OracleRoot "energyplus.exe"),
    (Join-Path $OracleRoot "ConvertInputFormat.exe"),
    $CasePath,
    $IdfPath,
    $WeatherPath
)) {
    if (-not (Test-Path -LiteralPath $requiredPath -PathType Leaf)) {
        throw "Missing required Schedule:Week:Compact exact-case file: $requiredPath"
    }
}

$caseText = Get-Content -LiteralPath $CasePath -Raw -Encoding UTF8
$idfText = Get-Content -LiteralPath $IdfPath -Raw -Encoding UTF8
$weatherLines = @(Get-Content -LiteralPath $WeatherPath -Encoding UTF8)
$weatherRows = @($weatherLines | Select-Object -Skip 8 | Where-Object { -not [string]::IsNullOrWhiteSpace($_) })

foreach ($contract in @(
    'comparison_class = "conformance"',
    'conformance_claim = true',
    'source_file = "data/conformance_cases/calendar_schedule_week_compact_day_types_exact_001/calendar_schedule_week_compact_day_types_exact.idf"',
    'idf = "data/conformance_cases/calendar_schedule_week_compact_day_types_exact_001/calendar_schedule_week_compact_day_types_exact.idf"',
    'weather = "data/conformance_cases/calendar_schedule_week_compact_day_types_exact_001/calendar_schedule_week_compact_day_types_exact.epw"',
    'frequency = "timestep"',
    'timestamp_contract = "ordered-exact-unique"',
    'abs_tol = 0.0',
    'rmse_tol = 0.0',
    'exactly seven source-ordered pairs with the explicit For prefix',
    'Thursday 11, Holiday 33, Saturday 22, SummerDesignDay 44, WinterDesignDay 55, CustomDay1 66, and CustomDay2 77',
    'one 672-sample ordered, unique Timestep Schedule Value series',
    'exact 12-slot WeekSchedule Timestep EIO row',
    'exactly 0 Warning and 0 Severe errors',
    'AllDays',
    'AllOtherDays',
    'omitted For prefix',
    'arbitrary combined day-type lists',
    'repeated day-type assignment',
    'missing assignments',
    'unknown tokens',
    'missing day references',
    'broad input diagnostic parity',
    'Hourly aggregation',
    'weather-file special-day precedence',
    'weekend holiday shifting',
    'overlapping special days',
    'daylight-saving',
    'today/tomorrow rollover',
    'SizingPeriod:DesignDay execution',
    'actual-weather',
    'warmup',
    'multiple environments',
    'UpdateScheduleVals orchestration',
    'EMS override behavior',
    'currentVal state',
    'Rust raw ESO serialization',
    'Rust EPW record selection',
    'broad EnergyPlus warning/error parity',
    'script = "scripts/dev.cmd compare-calendar-schedule-week-compact-day-types-exact"',
    'blocking = true'
)) {
    Assert-Contains -Text $caseText -Pattern $contract -Description "canonical manifest contract"
}
if (@([regex]::Matches($caseText, '(?m)^\[\[outputs\]\]$')).Count -ne 1) {
    throw "Manifest must retain exactly one output request"
}

$weekdayDay = "Regular Weekday Hourly"
$weekendDay = "Regular Weekend Interval"
$holidayDay = "Holiday List"
$summerDay = "Summer Design Hourly"
$winterDay = "Winter Design Interval"
$customDay1 = "Custom Day 1 List"
$customDay2 = "Custom Day 2 Hourly"
$hourly11 = @((1..24) | ForEach-Object { "11" }) -join "|"
$hourly44 = @((1..24) | ForEach-Object { "44" }) -join "|"
$hourly77 = @((1..24) | ForEach-Object { "77" }) -join "|"
$list33 = @((1..72) | ForEach-Object { "33" }) -join "|"
$list66 = @((1..72) | ForEach-Object { "66" }) -join "|"
$expectedVectors = @(
    "Version|26.1",
    "Building|Schedule Week Compact Day Types Exact Fixture|0.0|Suburbs|0.04|0.4|FullExterior|25|6",
    "Timestep|4",
    "GlobalGeometryRules|UpperLeftCorner|CounterClockWise|World",
    "RunPeriod|Schedule Week Compact Day Types Run Period|1|1|2032|1|7|2032|Thursday|No|No|No|No|No|No",
    "RunPeriodControl:SpecialDays|Week Compact Holiday|1/2|1|Holiday",
    "RunPeriodControl:SpecialDays|Week Compact Summer Design Day|1/4|1|SummerDesignDay",
    "RunPeriodControl:SpecialDays|Week Compact Winter Design Day|1/5|1|WinterDesignDay",
    "RunPeriodControl:SpecialDays|Week Compact Custom Day 1|1/6|1|CustomDay1",
    "RunPeriodControl:SpecialDays|Week Compact Custom Day 2|1/7|1|CustomDay2",
    "ScheduleTypeLimits|Any Number",
    "Schedule:Day:Hourly|$weekdayDay|Any Number|$hourly11",
    "Schedule:Day:Interval|$weekendDay|Any Number|No|Until: 00:15|22|Until: 24:00|22",
    "Schedule:Day:List|$holidayDay|Any Number|No|20|$list33",
    "Schedule:Day:Hourly|$summerDay|Any Number|$hourly44",
    "Schedule:Day:Interval|$winterDay|Any Number|No|Until: 00:15|55|Until: 24:00|55",
    "Schedule:Day:List|$customDay1|Any Number|No|20|$list66",
    "Schedule:Day:Hourly|$customDay2|Any Number|$hourly77",
    "Schedule:Week:Compact|Week Compact Day Types Week|For: Weekdays|$weekdayDay|For: Weekends|$weekendDay|For: Holiday|$holidayDay|For: SummerDesignDay|$summerDay|For: WinterDesignDay|$winterDay|For: CustomDay1|$customDay1|For: CustomDay2|$customDay2",
    "Schedule:Year|Week Compact Day Types Schedule|Any Number|Week Compact Day Types Week|1|1|12|31",
    "Output:Schedules|Timestep",
    "Output:Variable|WEEK COMPACT DAY TYPES SCHEDULE|Schedule Value|Timestep"
)
$actualVectors = @(Get-CompleteIdfObjectVectors -Text $idfText -Description "canonical fixture")
if (($actualVectors -join '||') -cne ($expectedVectors -join '||')) {
    throw "Fixture must retain the exact complete IDF object order, seven Week:Compact pairs, day-profile source values, and output requests"
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
Write-Host "OK complete Schedule:Week:Compact fixture vectors, seven source pairs, mixed day-profile references, and parser self-check"

$expectedHeaders = @(
    "LOCATION,Schedule Week Compact Day Types Exact Fixture,CO,USA,Synthetic,999999,39.74,-105.18,-7.0,1829.0",
    "DESIGN CONDITIONS,0",
    "TYPICAL/EXTREME PERIODS,0",
    "GROUND TEMPERATURES,0",
    "HOLIDAYS/DAYLIGHT SAVINGS,No,0,0,0",
    "COMMENTS 1,Deterministic seven-day Schedule Week Compact regular and special day-type fixture",
    "COMMENTS 2,Weather values are constant because only Schedule Value is compared",
    "DATA PERIODS,1,1,Data,Thursday,1/1,1/7"
)
if ($weatherLines.Count -ne 176 -or $weatherRows.Count -ne 168 -or
    (($weatherLines[0..7] -join '||') -cne ($expectedHeaders -join '||'))) {
    throw "Fixture EPW must retain eight exact headers and 168 hourly rows"
}
$weatherPayload = "?9?9?9?9E0?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9*9*9?9*9*9,10.0,5.0,50,80600,0,0,250,0,0,0,0,0,0,0,180,2.0,5,5,20.0,7777,9,999999999,0,0.0000,0,0,0.000,0.0,0.0"
for ($index = 0; $index -lt 168; ++$index) {
    $day = [int][Math]::Floor($index / 24) + 1
    $hour = ($index % 24) + 1
    $expected = "2032,1,$day,$hour,60,$weatherPayload"
    if ($weatherRows[$index] -cne $expected) {
        throw "Unexpected EPW row at index $($index): $($weatherRows[$index])"
    }
}
Write-Host "OK exact 168-hour Thursday-through-Wednesday EPW"

Remove-RepoDirectory -Path $CaseOutputRoot
$cargo = Get-Command cargo -ErrorAction Stop
$output = & $cargo.Source run -p ep_cli --quiet -- conformance time-weather-schedule-report $CasePath $OracleRoot $OutputRoot 2>&1
if ($LASTEXITCODE -ne 0) {
    $joinedOutput = $output -join [Environment]::NewLine
    throw "Schedule:Week:Compact day-types report failed with exit code $LASTEXITCODE $joinedOutput"
}
$output | ForEach-Object { Write-Host $_ }

$summaryPath = Join-Path $CaseOutputRoot "compare\compare-summary.json"
$reportPath = Join-Path $CaseOutputRoot "compare\compare-report.md"
$oracleEsoPath = Join-Path $CaseOutputRoot "oracle\eplusout.eso"
$oracleEioPath = Join-Path $CaseOutputRoot "oracle\eplusout.eio"
$oracleErrPath = Join-Path $CaseOutputRoot "oracle\eplusout.err"
$oracleEndPath = Join-Path $CaseOutputRoot "oracle\eplusout.end"
$stagedIdfPath = Join-Path $CaseOutputRoot "oracle\input.idf"
$convertedEpjsonPath = Join-Path $CaseOutputRoot "oracle\input.epJSON"
foreach ($artifactPath in @(
    $summaryPath,
    $reportPath,
    $oracleEsoPath,
    $oracleEioPath,
    $oracleErrPath,
    $oracleEndPath,
    $stagedIdfPath,
    $convertedEpjsonPath
)) {
    if (-not (Test-Path -LiteralPath $artifactPath -PathType Leaf)) {
        throw "Missing Schedule:Week:Compact report artifact: $artifactPath"
    }
}

$summary = Get-Content -LiteralPath $summaryPath -Raw -Encoding UTF8 | ConvertFrom-Json
if ($summary.case_id -cne $CaseId -or $summary.oracle_version -cne "26.1.0" -or
    $summary.comparison_class -cne "conformance" -or $summary.conformance_claim -ne $true -or
    $summary.status -cne "pass" -or $summary.series_count -ne 1 -or
    $summary.conformance_series_count -ne 1 -or $summary.time_axis_samples -ne 672 -or
    $summary.timestamp_rule -cne "zone-timestep ending samples aligned by EnergyPlus ESO timestamp labels" -or
    $summary.gate.script -cne $GateCommand -or $summary.gate.blocking -ne $true) {
    throw "Unexpected Schedule:Week:Compact report summary contract"
}
if ($null -ne $summary.weather_record_selection) {
    throw "Schedule-only comparison must retain null Rust EPW record selection"
}
$seriesRows = @($summary.series | Where-Object {
    $_.key -eq "WEEK COMPACT DAY TYPES SCHEDULE" -and $_.variable -eq "Schedule Value"
})
if ($seriesRows.Count -ne 1) {
    throw "Expected exactly one Schedule:Week:Compact Schedule Value series"
}
$series = $seriesRows[0]
$firstTimestamp = "env=SCHEDULE WEEK COMPACT DAY TYPES RUN PERIOD;day=1;month=1;date=1;dst=0;hour=1;start=0.00;end=15.00;day_type=Thursday"
$lastTimestamp = "env=SCHEDULE WEEK COMPACT DAY TYPES RUN PERIOD;day=7;month=1;date=7;dst=0;hour=24;start=45.00;end=60.00;day_type=CustomDay2"
if ($series.level -cne "conformance" -or $series.class -cne "schedule" -or
    $series.frequency -cne "timestep" -or $series.source -cne "eso" -or
    $series.alignment -cne "timestamp" -or $series.expected_samples -ne 672 -or
    $series.observed_samples -ne 672 -or $series.compared_samples -ne 672 -or
    $series.timestamp_contract -cne "ordered-exact-unique" -or
    $series.timestamp_status -cne "pass" -or $series.timestamp_expected_unique -ne $true -or
    $series.timestamp_observed_unique -ne $true -or $series.timestamp_order_match -ne $true -or
    $series.expected_first_timestamp -cne $firstTimestamp -or
    $series.observed_first_timestamp -cne $firstTimestamp -or
    $series.expected_last_timestamp -cne $lastTimestamp -or
    $series.observed_last_timestamp -cne $lastTimestamp -or
    $series.max_abs_tolerance -ne 0.0 -or $series.max_rel_tolerance -ne 0.0 -or
    $series.max_rmse_tolerance -ne 0.0 -or $series.max_abs_delta -ne 0.0 -or
    $series.rmse_delta -ne 0.0 -or $series.max_rel_delta -ne 0.0 -or
    $null -ne $series.first_divergence -or $null -ne $series.first_timestamp_divergence -or
    $series.status -cne "pass") {
    throw "Schedule:Week:Compact values and exact first/last timestamps must match at zero delta"
}
Write-Host "OK JSON single 672-sample series with exact timestamps and zero delta"

$injectionFooter = @(
    "!- eplus-rs output request injection begin",
    "!- case_id: $CaseId",
    "!- source: case manifest outputs/meters",
    "!- no new output requests; staged IDF already contains manifest requests",
    "!- eplus-rs output request injection end"
) -join "`n"
$expectedStagedIdf = $idfText + "`n" + $injectionFooter + "`n"
$stagedIdfText = [System.IO.File]::ReadAllText((Resolve-Path -LiteralPath $stagedIdfPath))
if ($stagedIdfText -cne $expectedStagedIdf) {
    throw "Oracle staged IDF must equal the canonical fixture plus the locked no-op output-injection footer"
}

$converted = Get-Content -LiteralPath $convertedEpjsonPath -Raw -Encoding UTF8 | ConvertFrom-Json
$weekFamily = $converted."Schedule:Week:Compact"
$convertedWeek = $weekFamily."Week Compact Day Types Week"
$weekEntries = @($convertedWeek.data)
$expectedDayTypeLists = @(
    "For: Weekdays",
    "For: Weekends",
    "For: Holiday",
    "For: SummerDesignDay",
    "For: WinterDesignDay",
    "For: CustomDay1",
    "For: CustomDay2"
)
$expectedDayNames = @($weekdayDay, $weekendDay, $holidayDay, $summerDay, $winterDay, $customDay1, $customDay2)
if (@($weekFamily.PSObject.Properties).Count -ne 1 -or $weekEntries.Count -ne 7) {
    throw "Converted epJSON must retain exactly one Week:Compact object with seven data entries"
}
for ($index = 0; $index -lt 7; ++$index) {
    if ($weekEntries[$index].daytype_list -cne $expectedDayTypeLists[$index] -or
        $weekEntries[$index].schedule_day_name -cne $expectedDayNames[$index]) {
        throw "Converted epJSON changed Week:Compact source pair $($index)"
    }
}
$intervalFamily = $converted."Schedule:Day:Interval"
$listFamily = $converted."Schedule:Day:List"
if (@($intervalFamily.PSObject.Properties).Count -ne 2 -or
    @($listFamily.PSObject.Properties).Count -ne 2) {
    throw "Converted epJSON must retain exactly two Interval and two List profiles"
}
foreach ($intervalSpec in @(
    [pscustomobject]@{ Name = $weekendDay; Value = 22.0 },
    [pscustomobject]@{ Name = $winterDay; Value = 55.0 }
)) {
    $profile = $intervalFamily.($intervalSpec.Name)
    $segments = @($profile.data)
    if ($profile.interpolate_to_timestep -cne "No" -or $segments.Count -ne 2 -or
        $segments[0].time -cne "Until: 00:15" -or $segments[0].value_until_time -ne $intervalSpec.Value -or
        $segments[1].time -cne "Until: 24:00" -or $segments[1].value_until_time -ne $intervalSpec.Value) {
        throw "Converted epJSON changed aligned explicit-No interval profile $($intervalSpec.Name)"
    }
}
foreach ($listSpec in @(
    [pscustomobject]@{ Name = $holidayDay; Value = 33.0 },
    [pscustomobject]@{ Name = $customDay1; Value = 66.0 }
)) {
    $profile = $listFamily.($listSpec.Name)
    $values = @($profile.extensions | ForEach-Object { [double]$_.value })
    if ($profile.interpolate_to_timestep -cne "No" -or $profile.minutes_per_item -ne 20 -or
        $values.Count -ne 72 -or @($values | Where-Object { $_ -ne $listSpec.Value }).Count -ne 0) {
        throw "Converted epJSON changed explicit-No Minutes-per-Item-20 list profile $($listSpec.Name)"
    }
}
Write-Host "OK staged IDF and converted epJSON seven-pair source order and referenced day-profile shapes"

$esoLines = @(Get-Content -LiteralPath $oracleEsoPath -Encoding UTF8)
$dictionaryRows = @($esoLines | Where-Object {
    $_ -ceq "7,1,WEEK COMPACT DAY TYPES SCHEDULE,Schedule Value [] !TimeStep"
})
$rawValues = @($esoLines | Where-Object { $_ -match '^7,\s*[-+0-9.E]+\s*$' } |
    ForEach-Object { [double](($_ -split ',', 2)[1].Trim()) })
$timestamps = @($esoLines | Where-Object { $_ -match '^2,\d+,\s*\d+,\s*\d+,\s*\d+,\s*\d+,' })
if ($dictionaryRows.Count -ne 1 -or $rawValues.Count -ne 672 -or $timestamps.Count -ne 672) {
    throw "Expected one exact Timestep dictionary and 672 raw values/timestamps"
}
$expectedDayTypes = @("Thursday", "Holiday", "Saturday", "SummerDesignDay", "WinterDesignDay", "CustomDay1", "CustomDay2")
$expectedDayValues = @(11.0, 33.0, 22.0, 44.0, 55.0, 66.0, 77.0)
for ($index = 0; $index -lt 672; ++$index) {
    $dayIndex = [int][Math]::Floor($index / 96)
    $daySample = $index % 96
    if ($rawValues[$index] -ne $expectedDayValues[$dayIndex]) {
        throw "Unexpected raw Week:Compact value at sample $($index): $($rawValues[$index])"
    }
    $hour = [int][Math]::Floor($daySample / 4) + 1
    $zoneTimestep = $daySample % 4
    $startMinute = $zoneTimestep * 15
    $endMinute = ($zoneTimestep + 1) * 15
    $timestampMatch = [regex]::Match(
        $timestamps[$index],
        '^2,\s*(\d+),\s*1,\s*(\d+),\s*0,\s*(\d+),\s*([-+0-9.]+),\s*([-+0-9.]+),(Thursday|Holiday|Saturday|SummerDesignDay|WinterDesignDay|CustomDay1|CustomDay2)$'
    )
    if (-not $timestampMatch.Success -or
        [int]$timestampMatch.Groups[1].Value -ne ($dayIndex + 1) -or
        [int]$timestampMatch.Groups[2].Value -ne ($dayIndex + 1) -or
        [int]$timestampMatch.Groups[3].Value -ne $hour -or
        [double]$timestampMatch.Groups[4].Value -ne $startMinute -or
        [double]$timestampMatch.Groups[5].Value -ne $endMinute -or
        $timestampMatch.Groups[6].Value -cne $expectedDayTypes[$dayIndex]) {
        throw "Unexpected raw Week:Compact zone-timestep timestamp at sample $($index): $($timestamps[$index])"
    }
}
Write-Host "OK exact raw ESO seven day-type value blocks and 672 timestamps"

$eioLines = @(Get-Content -LiteralPath $oracleEioPath -Encoding UTF8)
$environmentRow = "Environment,SCHEDULE WEEK COMPACT DAY TYPES RUN PERIOD,WeatherFileRunPeriod,01/01/2032,01/07/2032,Thursday,7,Use RunPeriod Specified Day,No,No,No,No,No,Clark and Allen"
$daylightRow = "Environment:Daylight Saving,No,RunPeriod Object"
$weekScheduleRow = "WeekSchedule - Timestep,WEEK COMPACT DAY TYPES WEEK,REGULAR WEEKEND INTERVAL,REGULAR WEEKDAY HOURLY,REGULAR WEEKDAY HOURLY,REGULAR WEEKDAY HOURLY,REGULAR WEEKDAY HOURLY,REGULAR WEEKDAY HOURLY,REGULAR WEEKEND INTERVAL,HOLIDAY LIST,SUMMER DESIGN HOURLY,WINTER DESIGN INTERVAL,CUSTOM DAY 1 LIST,CUSTOM DAY 2 HOURLY"
$scheduleRow = "Schedule - Timestep,WEEK COMPACT DAY TYPES SCHEDULE,ANY NUMBER,Through Dec 31,WEEK COMPACT DAY TYPES WEEK"
$specialRows = @(
    "Environment:Special Days,WEEK COMPACT HOLIDAY,Holiday,InputFile,01/02,  1",
    "Environment:Special Days,WEEK COMPACT SUMMER DESIGN DAY,SummerDesignDay,InputFile,01/04,  1",
    "Environment:Special Days,WEEK COMPACT WINTER DESIGN DAY,WinterDesignDay,InputFile,01/05,  1",
    "Environment:Special Days,WEEK COMPACT CUSTOM DAY 1,CustomDay1,InputFile,01/06,  1",
    "Environment:Special Days,WEEK COMPACT CUSTOM DAY 2,CustomDay2,InputFile,01/07,  1"
)
foreach ($expectedRow in @($environmentRow, $daylightRow, $weekScheduleRow, $scheduleRow) + $specialRows) {
    if (@($eioLines | Where-Object { $_ -ceq $expectedRow }).Count -ne 1) {
        throw "Missing or duplicated exact EnergyPlus EIO row: $expectedRow"
    }
}
Write-Host "OK exact Environment, disabled DST, five special days, 12-slot WeekSchedule, and Year EIO rows"

$errText = Get-Content -LiteralPath $oracleErrPath -Raw -Encoding UTF8
$endText = Get-Content -LiteralPath $oracleEndPath -Raw -Encoding UTF8
$completion = "EnergyPlus Completed Successfully-- 0 Warning; 0 Severe Errors;"
if ([regex]::Matches($errText, [regex]::Escape($completion)).Count -ne 1 -or
    [regex]::Matches($endText, [regex]::Escape($completion)).Count -ne 1) {
    throw "EnergyPlus ERR and END must each contain the exact 0 Warning; 0 Severe Errors completion"
}
Assert-NotContains -Text $errText -Pattern "** Warning **" -Description "EnergyPlus warning"
Write-Host "OK exact EnergyPlus completion with no warnings or severe errors"

$reportText = Get-Content -LiteralPath $reportPath -Raw -Encoding UTF8
foreach ($reportContract in @(
    "status: pass",
    "series: 1",
    "conformance_series: 1",
    "time_axis_samples: 672",
    "timestamp_rule: zone-timestep ending samples aligned by EnergyPlus ESO timestamp labels",
    "weather_record_selection_applied: false",
    "| WEEK COMPACT DAY TYPES SCHEDULE | Schedule Value | conformance | schedule | timestep | eso | timestamp | 672 | 672 | 672 | 0.000000000000 | 0.000000000000 | 0.000000000000 |"
)) {
    Assert-Contains -Text $reportText -Pattern $reportContract -Description "markdown Schedule:Week:Compact contract"
}

Write-Host "Schedule:Week:Compact regular and special day-types exact gate passed."
