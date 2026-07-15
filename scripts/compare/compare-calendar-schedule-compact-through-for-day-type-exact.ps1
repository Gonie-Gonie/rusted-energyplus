[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$CaseId = "calendar_schedule_compact_through_for_day_type_hourly_exact_001"
$CaseRoot = Join-Path $RepoRoot "data\conformance_cases\$CaseId"
$CasePath = Join-Path $CaseRoot "case.toml"
$IdfPath = Join-Path $CaseRoot "calendar_schedule_compact_through_for_day_type_hourly_exact.idf"
$WeatherPath = Join-Path $CaseRoot "calendar_schedule_compact_through_for_day_type_hourly_exact.epw"
$IdfRef = "data/conformance_cases/$CaseId/calendar_schedule_compact_through_for_day_type_hourly_exact.idf"
$WeatherRef = "data/conformance_cases/$CaseId/calendar_schedule_compact_through_for_day_type_hourly_exact.epw"
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

function Get-TomlSectionBlock {
    param(
        [Parameter(Mandatory = $true)][string]$Text,
        [Parameter(Mandatory = $true)][string]$Name,
        [Parameter(Mandatory = $true)][string]$Description
    )
    $pattern = '(?ms)^\[' + [regex]::Escape($Name) + '\]\s*(?<body>.*?)(?=^\[|\z)'
    $matches = [regex]::Matches($Text, $pattern)
    if ($matches.Count -ne 1) {
        throw "$Description must contain exactly one TOML section [$Name], found $($matches.Count)"
    }
    return $matches[0].Groups["body"].Value
}

function Get-TomlStringValue {
    param(
        [Parameter(Mandatory = $true)][string]$Text,
        [Parameter(Mandatory = $true)][string]$Name,
        [Parameter(Mandatory = $true)][string]$Description
    )
    $pattern = '(?m)^\s*' + [regex]::Escape($Name) + '\s*=\s*"(?<value>[^"]+)"\s*$'
    $matches = [regex]::Matches($Text, $pattern)
    if ($matches.Count -ne 1) {
        throw "$Description must contain exactly one TOML string key $Name, found $($matches.Count)"
    }
    return $matches[0].Groups["value"].Value
}

function Resolve-RepoReference {
    param(
        [Parameter(Mandatory = $true)][string]$Reference,
        [Parameter(Mandatory = $true)][string]$Description
    )
    $candidate = if ([System.IO.Path]::IsPathRooted($Reference)) {
        [System.IO.Path]::GetFullPath($Reference)
    }
    else {
        [System.IO.Path]::GetFullPath((Join-Path $RepoRoot $Reference))
    }
    if (-not (Test-Path -LiteralPath $candidate -PathType Leaf)) {
        throw "$Description does not resolve to a file: $Reference -> $candidate"
    }
    return (Resolve-Path -LiteralPath $candidate).Path
}

function Assert-SamePath {
    param(
        [Parameter(Mandatory = $true)][string]$Actual,
        [Parameter(Mandatory = $true)][string]$Expected,
        [Parameter(Mandatory = $true)][string]$Description
    )
    $actualFull = [System.IO.Path]::GetFullPath($Actual)
    $expectedFull = [System.IO.Path]::GetFullPath($Expected)
    if (-not $actualFull.Equals($expectedFull, [System.StringComparison]::OrdinalIgnoreCase)) {
        throw "$Description path mismatch: expected $expectedFull, found $actualFull"
    }
    Write-Host "OK ${Description}: $actualFull"
}

foreach ($path in @(
    (Join-Path $OracleRoot "energyplus.exe"),
    (Join-Path $OracleRoot "ConvertInputFormat.exe"),
    $CasePath,
    $IdfPath,
    $WeatherPath
)) {
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Missing required Through/For day-type schedule file: $path"
    }
}

$caseText = Get-Content -LiteralPath $CasePath -Raw -Encoding UTF8
$idfText = Get-Content -LiteralPath $IdfPath -Raw -Encoding UTF8
$weatherLines = @(Get-Content -LiteralPath $WeatherPath -Encoding UTF8)
$weatherText = $weatherLines -join "`n"
$weatherRows = @($weatherLines | Select-Object -Skip 8 | Where-Object { -not [string]::IsNullOrWhiteSpace($_) })

$manifestV2 = Get-TomlSectionBlock -Text $caseText -Name "manifest_v2" -Description $CaseId
$manifestInput = Get-TomlSectionBlock -Text $caseText -Name "input" -Description $CaseId
$sourceFileRef = Get-TomlStringValue -Text $manifestV2 -Name "source_file" -Description "$CaseId [manifest_v2]"
$manifestIdfRef = Get-TomlStringValue -Text $manifestInput -Name "idf" -Description "$CaseId [input]"
$manifestWeatherRef = Get-TomlStringValue -Text $manifestInput -Name "weather" -Description "$CaseId [input]"
if ($sourceFileRef -cne $IdfRef -or $manifestIdfRef -cne $IdfRef -or
    $manifestWeatherRef -cne $WeatherRef) {
    throw "Case manifest must bind source_file and input IDF/weather to the canonical Through/For fixture"
}
Assert-SamePath -Actual (Resolve-RepoReference -Reference $manifestIdfRef -Description "manifest input.idf") -Expected $IdfPath -Description "manifest input.idf"
Assert-SamePath -Actual (Resolve-RepoReference -Reference $manifestWeatherRef -Description "manifest input.weather") -Expected $WeatherPath -Description "manifest input.weather"

Assert-Contains -Text $caseText -Pattern 'timestamp_contract = "ordered-exact-unique"' -Description "ordered exact timestamp contract"
Assert-Contains -Text $caseText -Pattern 'abs_tol = 0.0' -Description "zero absolute tolerance"
Assert-Contains -Text $caseText -Pattern 'rmse_tol = 0.0' -Description "zero RMSE tolerance"
Assert-Contains -Text $caseText -Pattern 'daily order 103, 104, 105, 108, 199' -Description "exact daily schedule claim"
Assert-Contains -Text $caseText -Pattern 'source-order AllOtherDays complement' -Description "source-order complement boundary"
Assert-Contains -Text $caseText -Pattern 'DST-shifted schedule-clock lookup' -Description "DST clock nonclaim"
Assert-Contains -Text $caseText -Pattern 'Rust EPW record selection' -Description "Rust weather-record selection nonclaim"
Assert-Contains -Text $caseText -Pattern 'internal-gain/HVAC/IdealLoads calendar consumption' -Description "downstream calendar-consumer nonclaim"
Assert-Contains -Text $caseText -Pattern 'script = "scripts/dev.cmd compare-calendar-schedule-compact-through-for-day-type-exact"' -Description "blocking gate attribution"

$runPeriodObjects = [regex]::Matches($idfText, '(?ims)^\s*RunPeriod\s*,(?<body>.*?);')
$specialDayObjects = [regex]::Matches($idfText, '(?ims)^\s*RunPeriodControl:SpecialDays\s*,(?<body>.*?);')
$inputDstObjects = [regex]::Matches($idfText, '(?ims)^\s*RunPeriodControl:DaylightSavingTime\s*,(?<body>.*?);')
$scheduleLimitObjects = [regex]::Matches($idfText, '(?ims)^\s*ScheduleTypeLimits\s*,(?<body>.*?);')
$scheduleCompactObjects = [regex]::Matches($idfText, '(?ims)^\s*Schedule:Compact\s*,(?<body>.*?);')
$outputObjects = [regex]::Matches($idfText, '(?ims)^\s*Output:Variable\s*,(?<body>.*?);')
$idfObjects = [regex]::Matches($idfText, '(?ims)^\s*(?<type>[A-Za-z0-9:]+)\s*,(?<body>.*?);')
if ($runPeriodObjects.Count -ne 1 -or $specialDayObjects.Count -ne 1 -or
    $inputDstObjects.Count -ne 0 -or $scheduleLimitObjects.Count -ne 1 -or
    $scheduleCompactObjects.Count -ne 1 -or $outputObjects.Count -ne 2) {
    throw "Fixture must contain one RunPeriod, SpecialDays, ScheduleTypeLimits, and Schedule:Compact, two outputs, and no input DST object"
}

$actualIdfObjectVectors = @($idfObjects | ForEach-Object {
    $type = $_.Groups["type"].Value
    $body = [regex]::Replace($_.Groups["body"].Value, '(?m)!-.*$', '')
    $fields = @($body -split ',' | ForEach-Object { $_.Trim() })
    "$type|$($fields -join '|')"
})
$expectedIdfObjectVectors = @(
    "Version|26.1",
    "Building|Calendar Schedule Compact Through For Day Type Exact Fixture|0.0|Suburbs|0.04|0.4|FullExterior|25|6",
    "Timestep|4",
    "GlobalGeometryRules|UpperLeftCorner|CounterClockWise|World",
    "RunPeriod|Through For Day Type Run Period|12|30|2031|1|3|2032|Tuesday|No|No|No|No|No|No",
    "RunPeriodControl:SpecialDays|Cross Year New Year Holiday|1st Thursday in January|1|Holiday",
    "ScheduleTypeLimits|Any Number",
    "Schedule:Compact|Through For Day Type Schedule|Any Number|Through: 1/1|For: Thursday|Until: 24:00|105|For: AllOtherDays|Until: 24:00|199|Through: 12/31|For: Tuesday|Until: 24:00|103|For: Wednesday|Until: 24:00|104|For: Holiday|Until: 24:00|108|For: AllOtherDays|Until: 24:00|199",
    "Output:Variable|Through For Day Type Schedule|Schedule Value|Hourly",
    "Output:Variable|Environment|Site Day Type Index|Hourly"
)
if (($actualIdfObjectVectors -join '||') -cne ($expectedIdfObjectVectors -join '||')) {
    throw "Fixture must retain the exact complete IDF object order and field vectors"
}
Write-Host "OK complete fixture IDF object order and field vectors"

$runPeriodBody = [regex]::Replace($runPeriodObjects[0].Groups["body"].Value, '(?m)!-.*$', '')
$runPeriodFields = @($runPeriodBody -split ',' | ForEach-Object { $_.Trim() })
$expectedRunPeriodFields = @(
    "Through For Day Type Run Period",
    "12", "30", "2031", "1", "3", "2032", "Tuesday",
    "No", "No", "No", "No", "No", "No"
)
if (($runPeriodFields -join '|') -cne ($expectedRunPeriodFields -join '|')) {
    throw "Fixture must retain the exact five-day non-actual Tuesday-start RunPeriod"
}
$specialDayBody = [regex]::Replace($specialDayObjects[0].Groups["body"].Value, '(?m)!-.*$', '')
$specialDayFields = @($specialDayBody -split ',' | ForEach-Object { $_.Trim() })
if (($specialDayFields -join '|') -cne "Cross Year New Year Holiday|1st Thursday in January|1|Holiday") {
    throw "Fixture must retain exactly one input-file 1st Thursday in January Holiday"
}
$scheduleLimitFields = @($scheduleLimitObjects[0].Groups["body"].Value -split ',' | ForEach-Object { $_.Trim() })
if (($scheduleLimitFields -join '|') -cne "Any Number") {
    throw "Fixture must retain exactly one Any Number ScheduleTypeLimits object"
}
$compactBody = [regex]::Replace($scheduleCompactObjects[0].Groups["body"].Value, '(?m)!-.*$', '')
$compactFields = @($compactBody -split ',' | ForEach-Object { $_.Trim() })
$expectedCompactFields = @(
    "Through For Day Type Schedule", "Any Number",
    "Through: 1/1", "For: Thursday", "Until: 24:00", "105",
    "For: AllOtherDays", "Until: 24:00", "199",
    "Through: 12/31", "For: Tuesday", "Until: 24:00", "103",
    "For: Wednesday", "Until: 24:00", "104",
    "For: Holiday", "Until: 24:00", "108",
    "For: AllOtherDays", "Until: 24:00", "199"
)
if (($compactFields -join '|') -cne ($expectedCompactFields -join '|')) {
    throw "Schedule:Compact must retain the exact source-ordered Through/For/Until field vector"
}
$outputVectors = @($outputObjects | ForEach-Object {
    $body = [regex]::Replace($_.Groups["body"].Value, '(?m)!-.*$', '')
    (@($body -split ',' | ForEach-Object { $_.Trim() }) -join '|')
})
if (($outputVectors -join '||') -cne "Through For Day Type Schedule|Schedule Value|Hourly||Environment|Site Day Type Index|Hourly") {
    throw "Fixture must request exactly hourly Schedule Value followed by Site Day Type Index"
}

$expectedHeaders = @(
    "LOCATION,Calendar Schedule Compact Through For Day Type Fixture,CO,USA,Synthetic,999999,39.74,-105.18,-7.0,1829.0",
    "DESIGN CONDITIONS,0",
    "TYPICAL/EXTREME PERIODS,0",
    "GROUND TEMPERATURES,0",
    "HOLIDAYS/DAYLIGHT SAVINGS,Yes,0,0,0",
    "COMMENTS 1,Deterministic five-day cross-year weather fixture for Schedule Compact Through and For day-type consumption",
    "COMMENTS 2,Weather row years are source-only because Treat Weather as Actual is No",
    "DATA PERIODS,1,1,Data,Tuesday,12/30,1/3"
)
$nonblankWeatherLines = @($weatherLines | Where-Object { -not [string]::IsNullOrWhiteSpace($_) })
if ($weatherLines.Count -ne 128 -or $nonblankWeatherLines.Count -ne 128 -or
    (($weatherLines[0..7] -join [Environment]::NewLine) -cne ($expectedHeaders -join [Environment]::NewLine)) -or
    $weatherRows.Count -ne 120) {
    throw "Through/For EPW must retain exactly eight locked headers and 120 nonblank hourly rows"
}
$expectedYears = @(2031, 2031, 2032, 2032, 2032)
$expectedMonths = @(12, 12, 1, 1, 1)
$expectedDays = @(30, 31, 1, 2, 3)
$expectedPayload = "?9?9?9?9E0?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9*9*9?9*9*9,10.0,5.0,50,80600,0,0,250,0,0,0,0,0,0,0,180,2.0,5,5,20.0,7777,9,999999999,0,0.0000,0,0,0.000,0.0,0.0"
for ($index = 0; $index -lt 120; ++$index) {
    $dayOffset = [Math]::Floor($index / 24)
    $expectedHour = ($index % 24) + 1
    $fields = @($weatherRows[$index] -split ',')
    if ($fields.Count -ne 35 -or [int]$fields[0] -ne $expectedYears[$dayOffset] -or
        [int]$fields[1] -ne $expectedMonths[$dayOffset] -or
        [int]$fields[2] -ne $expectedDays[$dayOffset] -or
        [int]$fields[3] -ne $expectedHour -or [int]$fields[4] -ne 60 -or
        (($fields[5..34] -join ',') -cne $expectedPayload)) {
        throw "Unexpected locked EPW row at source sample ${index}: $($weatherRows[$index])"
    }
}

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Remove-RepoDirectory -Path $CaseOutputRoot
Write-Host "Running exact Schedule:Compact Through and For day-type gate."
$output = & $cargo.Source run -p ep_cli --quiet -- conformance time-weather-schedule-report $CasePath $OracleRoot $OutputRoot 2>&1
if ($LASTEXITCODE -ne 0) {
    $output | ForEach-Object { Write-Host $_ }
    throw "Schedule:Compact Through and For day-type exact gate failed."
}
$text = $output -join "`n"
Assert-Contains -Text $text -Pattern "id: $CaseId" -Description "report id"
Assert-Contains -Text $text -Pattern "status: pass" -Description "report status"

$summaryPath = Join-Path $CaseOutputRoot "compare\compare-summary.json"
$reportPath = Join-Path $CaseOutputRoot "compare\compare-report.md"
$oracleEsoPath = Join-Path $CaseOutputRoot "oracle\eplusout.eso"
$oracleEioPath = Join-Path $CaseOutputRoot "oracle\eplusout.eio"
$oracleErrPath = Join-Path $CaseOutputRoot "oracle\eplusout.err"
$expandedManifestPath = Join-Path $CaseOutputRoot "oracle\case-expanded.toml"
$stagedIdfPath = Join-Path $CaseOutputRoot "oracle\input.idf"
foreach ($path in @(
    $summaryPath,
    $reportPath,
    $oracleEsoPath,
    $oracleEioPath,
    $oracleErrPath,
    $expandedManifestPath,
    $stagedIdfPath
)) {
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Missing Through/For day-type comparison artifact: $path"
    }
}

$expandedText = Get-Content -LiteralPath $expandedManifestPath -Raw -Encoding UTF8
$expandedInput = Get-TomlSectionBlock -Text $expandedText -Name "input" -Description "expanded manifest"
$expandedIdfRef = Get-TomlStringValue -Text $expandedInput -Name "source_idf" -Description "expanded manifest [input]"
$expandedWeatherRef = Get-TomlStringValue -Text $expandedInput -Name "source_weather" -Description "expanded manifest [input]"
$expandedStagedIdf = Get-TomlStringValue -Text $expandedInput -Name "staged_idf" -Description "expanded manifest [input]"
$expandedConvertedEpjson = Get-TomlStringValue -Text $expandedInput -Name "converted_epjson" -Description "expanded manifest [input]"
Assert-SamePath -Actual (Resolve-RepoReference -Reference $expandedIdfRef -Description "expanded source_idf") -Expected $IdfPath -Description "expanded source_idf"
Assert-SamePath -Actual (Resolve-RepoReference -Reference $expandedWeatherRef -Description "expanded source_weather") -Expected $WeatherPath -Description "expanded source_weather"
if ($expandedStagedIdf -cne "input.idf" -or $expandedConvertedEpjson -cne "input.epJSON") {
    throw "Expanded manifest must retain the exact staged IDF and converted epJSON provenance names"
}
Write-Host "OK expanded staged_idf and converted_epjson provenance names"

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
    throw "Oracle staged IDF must equal the canonical Through/For fixture plus the locked no-op output-injection footer"
}
Write-Host "OK oracle staged IDF canonical text and output-injection footer"

$summary = Get-Content -LiteralPath $summaryPath -Raw -Encoding UTF8 | ConvertFrom-Json
if ($summary.case_id -cne $CaseId -or $summary.oracle_version -cne "26.1.0" -or
    $summary.comparison_class -cne "conformance" -or $summary.conformance_claim -ne $true -or
    $summary.status -cne "pass" -or $summary.time_axis_samples -ne 120 -or
    $summary.series_count -ne 1 -or $summary.conformance_series_count -ne 1 -or
    $summary.gate.script -cne "scripts/dev.cmd compare-calendar-schedule-compact-through-for-day-type-exact" -or
    $summary.gate.blocking -ne $true) {
    throw "Through/For case must retain its exact passing single-series conformance and blocking-gate contract"
}
$calendar = $summary.weather_calendar
if ($calendar.policy_applied -ne $true -or $calendar.weather_file_allows_leap_years -ne $true -or
    $calendar.start_year -ne 2031 -or $calendar.end_year -ne 2032 -or
    $calendar.gregorian_calendar_days -ne 5 -or $calendar.weather_effective_calendar_days -ne 5 -or
    $calendar.leap_days_skipped -ne 0 -or
    $calendar.start_year_gregorian_leap -ne $false -or
    $calendar.start_year_weather_effective_leap -ne $false -or
    $calendar.end_year_gregorian_leap -ne $true -or
    $calendar.end_year_weather_effective_leap -ne $true) {
    throw "Unexpected Through/For cross-year weather calendar summary"
}
$dst = $calendar.daylight_saving
if ($dst.weather_file_period_declared -ne $false -or
    $dst.run_period_uses_weather_file_period -ne $false -or
    $dst.input_file_period_declared -ne $false -or $dst.active -ne $false -or
    $dst.effective_source -cne "none" -or $null -ne $dst.resolved_period -or
    $calendar.daylight_saving_hourly_samples -ne 0) {
    throw "Through/For fixture must keep daylight saving fully disabled"
}
if ($null -ne $summary.weather_record_selection) {
    throw "Schedule-only Through/For comparison must not claim Rust EPW record selection"
}
$specialDays = $summary.special_days
if ($specialDays.weather_file_declared -ne 0 -or
    $specialDays.run_period_uses_weather_file -ne $false -or
    $specialDays.weather_file_resolved -ne 0 -or
    $specialDays.input_file_declared -ne 1 -or $specialDays.apply_weekend_rule -ne $false -or
    $specialDays.resolved_count -ne 1 -or $specialDays.hourly_samples -ne 24) {
    throw "Unexpected Through/For special-day summary counts"
}
$resolvedSpecialDays = @($specialDays.resolved)
if ($resolvedSpecialDays.Count -ne 1) {
    throw "Expected exactly one resolved input-file Holiday"
}
$resolvedSpecialDay = $resolvedSpecialDays[0]
if ($resolvedSpecialDay.name -cne "CROSS YEAR NEW YEAR HOLIDAY" -or
    $resolvedSpecialDay.source -cne "input-file" -or
    $resolvedSpecialDay.start_month -ne 1 -or $resolvedSpecialDay.start_day -ne 2 -or
    $resolvedSpecialDay.start_day_of_year -ne 2 -or $resolvedSpecialDay.duration_days -ne 1 -or
    $resolvedSpecialDay.day_type -cne "Holiday" -or $resolvedSpecialDay.day_type_index -ne 8 -or
    $resolvedSpecialDay.weekend_shift_days -ne 0) {
    throw "Unexpected resolved January 2 input-file Holiday"
}

$allSeries = @($summary.series)
if ($allSeries.Count -ne 1) {
    throw "Summary must contain exactly one promoted series object"
}
$seriesRows = @($allSeries | Where-Object {
    $_.key -eq "THROUGH FOR DAY TYPE SCHEDULE" -and $_.variable -eq "Schedule Value"
})
if ($seriesRows.Count -ne 1) {
    throw "Expected exactly one Through For Day Type Schedule Value series"
}
$series = $seriesRows[0]
if ($series.level -cne "conformance" -or $series.class -cne "schedule" -or
    $series.frequency -cne "hourly" -or $series.source -cne "eso" -or
    $series.alignment -cne "timestamp" -or $series.expected_samples -ne 120 -or
    $series.observed_samples -ne 120 -or $series.compared_samples -ne 120) {
    throw "Unexpected Through/For Schedule Value series metadata or counts"
}
if ($series.timestamp_contract -ne "ordered-exact-unique" -or $series.timestamp_status -ne "pass" -or
    $series.timestamp_expected_unique -ne $true -or $series.timestamp_observed_unique -ne $true -or
    $series.timestamp_order_match -ne $true) {
    throw "Through/For timestamp uniqueness or order failed"
}
$expectedFirst = "env=THROUGH FOR DAY TYPE RUN PERIOD;day=1;month=12;date=30;dst=0;hour=1;start=0.00;end=60.00;day_type=Tuesday"
$expectedLast = "env=THROUGH FOR DAY TYPE RUN PERIOD;day=5;month=1;date=3;dst=0;hour=24;start=0.00;end=60.00;day_type=Saturday"
if ($series.expected_first_timestamp -ne $expectedFirst -or $series.observed_first_timestamp -ne $expectedFirst -or
    $series.expected_last_timestamp -ne $expectedLast -or $series.observed_last_timestamp -ne $expectedLast) {
    throw "Unexpected Through/For timestamp endpoints"
}
if ($series.max_abs_tolerance -ne 0.0 -or $series.max_rel_tolerance -ne 0.0 -or
    $series.max_rmse_tolerance -ne 0.0 -or $series.max_abs_delta -ne 0.0 -or
    $series.rmse_delta -ne 0.0 -or $series.max_rel_delta -ne 0.0 -or
    $null -ne $series.first_divergence -or $null -ne $series.first_timestamp_divergence -or
    $series.status -cne "pass") {
    throw "Through/For schedule values must match exactly at zero tolerance"
}

$oracleEsoLines = Get-Content -LiteralPath $oracleEsoPath -Encoding UTF8
$scheduleDictionaryRows = @($oracleEsoLines | Where-Object {
    $_ -match '^\d+,1,THROUGH FOR DAY TYPE SCHEDULE,Schedule Value \[\] !Hourly$'
})
$dayTypeDictionaryRows = @($oracleEsoLines | Where-Object {
    $_ -match '^\d+,1,Environment,Site Day Type Index \[\] !Hourly$'
})
if ($scheduleDictionaryRows.Count -ne 1 -or $dayTypeDictionaryRows.Count -ne 1) {
    throw "Expected one exact hourly Schedule Value and Site Day Type Index ESO dictionary entry"
}
$scheduleIdMatch = [regex]::Match([string]$scheduleDictionaryRows[0], '^(\d+),')
$dayTypeIdMatch = [regex]::Match([string]$dayTypeDictionaryRows[0], '^(\d+),')
if (-not $scheduleIdMatch.Success -or -not $dayTypeIdMatch.Success) {
    throw "Malformed Through/For ESO dictionary entries"
}
$scheduleReportId = $scheduleIdMatch.Groups[1].Value
$dayTypeReportId = $dayTypeIdMatch.Groups[1].Value
$scheduleValues = @($oracleEsoLines | Where-Object { $_ -match ('^' + $scheduleReportId + ',\s*[-+0-9.E]+\s*$') } | ForEach-Object {
    [double](($_ -split ',', 2)[1].Trim())
})
$dayTypeValues = @($oracleEsoLines | Where-Object { $_ -match ('^' + $dayTypeReportId + ',\s*[-+0-9.E]+\s*$') } | ForEach-Object {
    [double](($_ -split ',', 2)[1].Trim())
})
$timestampRows = @($oracleEsoLines | Where-Object { $_ -match '^2,\d+,\s*\d+,\s*\d+,\s*\d+,\s*\d+,' })
if ($scheduleValues.Count -ne 120 -or $dayTypeValues.Count -ne 120 -or $timestampRows.Count -ne 120) {
    throw "Expected 120 raw oracle schedule values, day types, and hourly timestamps"
}
$expectedDailyScheduleValues = @(103.0, 104.0, 105.0, 108.0, 199.0)
$expectedDailyTypeValues = @(3.0, 4.0, 5.0, 8.0, 7.0)
$expectedDailyTypeNames = @("Tuesday", "Wednesday", "Thursday", "Holiday", "Saturday")
for ($index = 0; $index -lt 120; ++$index) {
    $dayIndex = [Math]::Floor($index / 24)
    $expectedHour = ($index % 24) + 1
    if ($scheduleValues[$index] -ne $expectedDailyScheduleValues[$dayIndex] -or
        $dayTypeValues[$index] -ne $expectedDailyTypeValues[$dayIndex]) {
        throw "Unexpected raw schedule/day-type value at sample ${index}: $($scheduleValues[$index]) / $($dayTypeValues[$index])"
    }
    $timestampMatch = [regex]::Match(
        $timestampRows[$index],
        '^2,\s*(\d+),\s*(\d+),\s*(\d+),\s*(\d+),\s*(\d+),\s*([-+0-9.]+),\s*([-+0-9.]+),([^,]+)$'
    )
    if (-not $timestampMatch.Success -or
        [int]$timestampMatch.Groups[1].Value -ne ($dayIndex + 1) -or
        [int]$timestampMatch.Groups[2].Value -ne $expectedMonths[$dayIndex] -or
        [int]$timestampMatch.Groups[3].Value -ne $expectedDays[$dayIndex] -or
        [int]$timestampMatch.Groups[4].Value -ne 0 -or
        [int]$timestampMatch.Groups[5].Value -ne $expectedHour -or
        $timestampMatch.Groups[6].Value -cne "0.00" -or
        $timestampMatch.Groups[7].Value -cne "60.00" -or
        $timestampMatch.Groups[8].Value.Trim() -cne $expectedDailyTypeNames[$dayIndex]) {
        throw "Unexpected raw Through/For timestamp at sample ${index}: $($timestampRows[$index])"
    }
}

foreach ($expectedValue in $expectedDailyScheduleValues) {
    $count = @($scheduleValues | Where-Object { $_ -eq $expectedValue }).Count
    if ($count -ne 24) {
        throw "Expected exactly 24 raw schedule samples with value $expectedValue, found $count"
    }
}
foreach ($expectedValue in $expectedDailyTypeValues) {
    $count = @($dayTypeValues | Where-Object { $_ -eq $expectedValue }).Count
    if ($count -ne 24) {
        throw "Expected exactly 24 raw day-type samples with value $expectedValue, found $count"
    }
}

$oracleEioLines = Get-Content -LiteralPath $oracleEioPath -Encoding UTF8
$expectedEnvironmentRow = "Environment,THROUGH FOR DAY TYPE RUN PERIOD,WeatherFileRunPeriod,12/30/2031,01/03/2032,Tuesday,5,Use RunPeriod Specified Day,No,No,No,No,No,Clark and Allen"
$expectedDstRow = "Environment:Daylight Saving,No,RunPeriod Object"
$expectedSpecialDayRow = "Environment:Special Days,CROSS YEAR NEW YEAR HOLIDAY,Holiday,InputFile,01/02,  1"
$environmentRows = @($oracleEioLines | Where-Object { $_ -like "Environment,THROUGH FOR DAY TYPE RUN PERIOD,*" })
$dstRows = @($oracleEioLines | Where-Object { $_ -like "Environment:Daylight Saving,*" })
$specialDayRows = @($oracleEioLines | Where-Object { $_ -like "Environment:Special Days,*" })
if ($environmentRows.Count -ne 1 -or $environmentRows[0] -cne $expectedEnvironmentRow -or
    @($environmentRows[0] -split ',').Count -ne 14 -or
    $dstRows.Count -ne 1 -or $dstRows[0] -cne $expectedDstRow -or
    @($dstRows[0] -split ',').Count -ne 3 -or
    $specialDayRows.Count -ne 1 -or $specialDayRows[0] -cne $expectedSpecialDayRow -or
    @($specialDayRows[0] -split ',').Count -ne 6) {
    throw "Unexpected exact Through/For Environment, Daylight Saving, or Special Days EIO rows"
}
Write-Host "OK exact Through/For Environment, disabled DST, and January 2 Holiday EIO rows"

$errText = Get-Content -LiteralPath $oracleErrPath -Raw -Encoding UTF8
Assert-Contains -Text $errText -Pattern "EnergyPlus Completed Successfully-- 0 Warning; 0 Severe Errors;" -Description "clean EnergyPlus completion"
if ([regex]::Matches($errText, '(?m)^\s*\*\* Warning \*\*').Count -ne 0 -or
    [regex]::Matches($errText, '(?m)^\s*\*\* Severe\s+\*\*').Count -ne 0) {
    throw "Through/For oracle must complete without warning or severe markers"
}

$reportText = Get-Content -LiteralPath $reportPath -Raw -Encoding UTF8
Assert-Contains -Text $reportText -Pattern "gregorian_calendar_years: 2031..2032" -Description "markdown cross-year calendar range"
Assert-Contains -Text $reportText -Pattern "start_year_gregorian_leap: false" -Description "markdown common start year"
Assert-Contains -Text $reportText -Pattern "start_year_weather_effective_leap: false" -Description "markdown common weather-effective start year"
Assert-Contains -Text $reportText -Pattern "end_year_gregorian_leap: true" -Description "markdown Gregorian leap end year"
Assert-Contains -Text $reportText -Pattern "end_year_weather_effective_leap: true" -Description "markdown weather-effective leap end year"
Assert-Contains -Text $reportText -Pattern "daylight_saving_active: false" -Description "markdown disabled daylight saving"
Assert-Contains -Text $reportText -Pattern "daylight_saving_effective_source: none" -Description "markdown absent daylight-saving source"
Assert-Contains -Text $reportText -Pattern "daylight_saving_hourly_samples: 0" -Description "markdown zero daylight-saving samples"
Assert-Contains -Text $reportText -Pattern "weather_file_holidays_declared: 0" -Description "markdown zero EPW holidays"
Assert-Contains -Text $reportText -Pattern "run_period_uses_weather_file_holidays: false" -Description "markdown disabled EPW holiday policy"
Assert-Contains -Text $reportText -Pattern "input_file_special_days_declared: 1" -Description "markdown one input special day"
Assert-Contains -Text $reportText -Pattern "special_days_resolved: 1" -Description "markdown one resolved special day"
Assert-Contains -Text $reportText -Pattern "special_day_hourly_samples: 24" -Description "markdown Holiday sample count"
Assert-Contains -Text $reportText -Pattern "special_day_resolved: CROSS YEAR NEW YEAR HOLIDAY 1/2 duration=1 day_type=Holiday weekend_shift_days=0 source=input-file" -Description "markdown exact January 2 Holiday"
Assert-Contains -Text $reportText -Pattern "| THROUGH FOR DAY TYPE SCHEDULE | Schedule Value | conformance" -Description "markdown schedule conformance row"

Write-Host "Schedule:Compact Through and For day-type exact gate passed."
