[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$CaseId = "calendar_schedule_dst_hour24_tomorrow_day_type_exact_001"
$CaseRoot = Join-Path $RepoRoot "data\conformance_cases\$CaseId"
$CasePath = Join-Path $CaseRoot "case.toml"
$IdfPath = Join-Path $CaseRoot "calendar_schedule_dst_hour24_tomorrow_day_type_exact.idf"
$WeatherPath = Join-Path $CaseRoot "calendar_schedule_dst_hour24_tomorrow_day_type_exact.epw"
$IdfRef = "data/conformance_cases/$CaseId/calendar_schedule_dst_hour24_tomorrow_day_type_exact.idf"
$WeatherRef = "data/conformance_cases/$CaseId/calendar_schedule_dst_hour24_tomorrow_day_type_exact.epw"
$GateCommand = "scripts/dev.cmd compare-calendar-schedule-dst-hour24-tomorrow-day-type-exact"
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
        throw "Missing required Schedule:Compact DST rollover file: $path"
    }
}

$caseText = Get-Content -LiteralPath $CasePath -Raw -Encoding UTF8
$idfText = Get-Content -LiteralPath $IdfPath -Raw -Encoding UTF8
$weatherLines = @(Get-Content -LiteralPath $WeatherPath -Encoding UTF8)
$weatherNonblankLines = @($weatherLines | Where-Object { -not [string]::IsNullOrWhiteSpace($_) })
$weatherRows = @($weatherLines | Select-Object -Skip 8 | Where-Object { -not [string]::IsNullOrWhiteSpace($_) })

$manifestV2 = Get-TomlSectionBlock -Text $caseText -Name "manifest_v2" -Description $CaseId
$manifestInput = Get-TomlSectionBlock -Text $caseText -Name "input" -Description $CaseId
$sourceFileRef = Get-TomlStringValue -Text $manifestV2 -Name "source_file" -Description "$CaseId [manifest_v2]"
$manifestIdfRef = Get-TomlStringValue -Text $manifestInput -Name "idf" -Description "$CaseId [input]"
$manifestWeatherRef = Get-TomlStringValue -Text $manifestInput -Name "weather" -Description "$CaseId [input]"
if ($sourceFileRef -cne $IdfRef -or $manifestIdfRef -cne $IdfRef -or $manifestWeatherRef -cne $WeatherRef) {
    throw "Case manifest must bind source_file and input IDF/weather to the canonical DST rollover fixture"
}
Assert-SamePath -Actual (Resolve-RepoReference -Reference $manifestIdfRef -Description "manifest input.idf") -Expected $IdfPath -Description "manifest input.idf"
Assert-SamePath -Actual (Resolve-RepoReference -Reference $manifestWeatherRef -Description "manifest input.weather") -Expected $WeatherPath -Description "manifest input.weather"

foreach ($contract in @(
    'comparison_class = "conformance"',
    'conformance_claim = true',
    'timestamp_contract = "ordered-exact-unique"',
    'abs_tol = 0.0',
    'rmse_tol = 0.0',
    'The zero-tolerance external schedule claim is exactly 72 ordered, unique Schedule Value samples and timestamps',
    '100 repeated for 23 hours then 124',
    '200 repeated for 23 hours then 801',
    '800 repeated for 23 hours then 901',
    'Schedule-specific daylight-saving opt-out',
    'subhourly values or interpolation',
    'other schedule families',
    'year-end schedule-ordinal wrap',
    'Rust EPW record selection',
    'internal-gain/HVAC/IdealLoads calendar consumption',
    'Rust raw ESO serialization',
    'script = "scripts/dev.cmd compare-calendar-schedule-dst-hour24-tomorrow-day-type-exact"',
    'blocking = true'
)) {
    Assert-Contains -Text $caseText -Pattern $contract -Description "canonical manifest claim or boundary"
}

$manifestOutputs = [regex]::Matches($caseText, '(?ms)^\[\[outputs\]\]\s*(?<body>.*?)(?=^\[|\z)')
if ($manifestOutputs.Count -ne 1) {
    throw "Canonical manifest must promote exactly one output, found $($manifestOutputs.Count)"
}
$manifestOutputBody = $manifestOutputs[0].Groups["body"].Value
foreach ($field in @(
    'key = "DST FINAL ROLLOVER SCHEDULE"',
    'variable = "Schedule Value"',
    'frequency = "hourly"',
    'class = "schedule"',
    'source = "eso"',
    'domain = "schedule"',
    'level = "conformance"',
    'abs_tol = 0.0',
    'rmse_tol = 0.0',
    'timestamp_contract = "ordered-exact-unique"'
)) {
    Assert-Contains -Text $manifestOutputBody -Pattern $field -Description "single promoted schedule output"
}
if ($manifestOutputBody -match 'Site Daylight Saving Time Status|Site Day Type Index') {
    throw "Auxiliary DST and day-type raw ESO outputs must not be promoted as conformance series"
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
    for ($chunkIndex = 0; $chunkIndex -lt $chunks.Count; ++$chunkIndex) {
        $objectMatch = [regex]::Match(
            $chunks[$chunkIndex],
            '(?s)^(?<type>[A-Za-z0-9:]+)\s*,(?<body>.*)$'
        )
        if (-not $objectMatch.Success) {
            throw "$Description contains a nonblank semicolon-delimited chunk that is not an IDF object at index ${chunkIndex}: $($chunks[$chunkIndex])"
        }
        $type = $objectMatch.Groups["type"].Value
        $fields = @([regex]::Split($objectMatch.Groups["body"].Value, ',') | ForEach-Object { $_.Trim() })
        $vectors += "$type|$($fields -join '|')"
    }
    return $vectors
}

$actualIdfObjectVectors = @(Get-CompleteIdfObjectVectors -Text $idfText -Description "canonical fixture")
$expectedIdfObjectVectors = @(
    "Version|26.1",
    "Building|Calendar Schedule DST Final Rollover Exact Fixture|0.0|Suburbs|0.04|0.4|FullExterior|25|6",
    "Timestep|4",
    "GlobalGeometryRules|UpperLeftCorner|CounterClockWise|World",
    "RunPeriod|DST Schedule Final Rollover Run Period|10|30|2032|11|1|2032|Saturday|No|Yes|No|No|No|No",
    "RunPeriodControl:SpecialDays|Final Rollover Holiday|11/1|1|Holiday",
    "ScheduleTypeLimits|Any Number",
    "Schedule:Compact|DST Final Rollover Schedule|Any Number|Through: 10/30|For: AllDays|Until: 23:00|100|Until: 24:00|124|Through: 10/31|For: AllDays|Until: 1:00|201|Until: 24:00|200|Through: 11/1|For: Holiday|Until: 1:00|801|Until: 24:00|800|For: AllOtherDays|Until: 1:00|301|Until: 24:00|300|Through: 12/31|For: Holiday|Until: 1:00|901|Until: 24:00|900|For: AllOtherDays|Until: 1:00|401|Until: 24:00|400",
    "Output:Variable|DST FINAL ROLLOVER SCHEDULE|Schedule Value|Hourly",
    "Output:Variable|Environment|Site Daylight Saving Time Status|Hourly",
    "Output:Variable|Environment|Site Day Type Index|Hourly"
)
if (($actualIdfObjectVectors -join '||') -cne ($expectedIdfObjectVectors -join '||')) {
    throw "Fixture must retain the exact complete IDF object order and field vectors"
}
Write-Host "OK complete fixture IDF object order and field vectors"

$sameLineMutationNeedle = "Version,26.1;"
if ([regex]::Matches($idfText, [regex]::Escape($sameLineMutationNeedle)).Count -ne 1) {
    throw "Canonical fixture must contain exactly one Version token for the same-line parser mutation check"
}
$sameLineMutatedIdf = $idfText.Replace(
    $sameLineMutationNeedle,
    "$sameLineMutationNeedle Schedule:Constant,Same Line Parser Mutation,,1;"
)
$sameLineMutatedVectors = @(Get-CompleteIdfObjectVectors -Text $sameLineMutatedIdf -Description "same-line mutated fixture")
if ($sameLineMutatedVectors.Count -ne ($expectedIdfObjectVectors.Count + 1) -or
    $sameLineMutatedVectors[1] -cne "Schedule:Constant|Same Line Parser Mutation||1" -or
    ($sameLineMutatedVectors -join '||') -ceq ($expectedIdfObjectVectors -join '||')) {
    throw "Complete IDF parser must expose a same-line injected object and preserve its empty field"
}
Write-Host "OK same-line injected IDF object is detected with its empty field preserved"

if ([regex]::Matches($idfText, '(?im)^\s*RunPeriodControl:DaylightSavingTime\s*,').Count -ne 0) {
    throw "Fixture must source daylight saving only from the canonical EPW header"
}

$expectedWeatherHeaders = @(
    "LOCATION,Calendar Schedule DST Final Rollover Exact Fixture,CO,USA,Synthetic,999999,39.74,-105.18,-7.0,1829.0",
    "DESIGN CONDITIONS,0",
    "TYPICAL/EXTREME PERIODS,0",
    "GROUND TEMPERATURES,0",
    "HOLIDAYS/DAYLIGHT SAVINGS,Yes,Last Sunday in October,Last Sunday in March,0",
    "COMMENTS 1,Deterministic three-day southern DST schedule hour-24 rollover exact fixture",
    "COMMENTS 2,Weather values are constant because only schedule clock and calendar state are compared",
    "DATA PERIODS,1,1,Data,Saturday,10/30,11/1"
)
if ($weatherLines.Count -ne 80 -or $weatherNonblankLines.Count -ne 80 -or
    (($weatherLines[0..7] -join '||') -cne ($expectedWeatherHeaders -join '||'))) {
    throw "Fixture EPW must retain exactly 80 nonblank lines and the eight canonical headers"
}
if ($weatherRows.Count -ne 72) {
    throw "Fixture EPW must retain exactly 72 hourly rows, found $($weatherRows.Count)"
}
$expectedMonths = @(10, 10, 11)
$expectedDates = @(30, 31, 1)
$expectedWeatherPayload = "?9?9?9?9E0?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9*9*9?9*9*9,10.0,5.0,50,80600,0,0,250,0,0,0,0,0,0,0,180,2.0,5,5,20.0,7777,9,999999999,0,0.0000,0,0,0.000,0.0,0.0"
for ($index = 0; $index -lt 72; ++$index) {
    $dayIndex = [int][Math]::Floor($index / 24)
    $expectedHour = ($index % 24) + 1
    $fields = @($weatherRows[$index] -split ',')
    $expectedRow = "2032,$($expectedMonths[$dayIndex]),$($expectedDates[$dayIndex]),$expectedHour,60,$expectedWeatherPayload"
    if ($fields.Count -ne 35 -or $weatherRows[$index] -cne $expectedRow) {
        throw "Unexpected locked 35-field EPW row at source sample ${index}: $($weatherRows[$index])"
    }
}
Write-Host "OK exact 80-line EPW, 72 ordered hourly rows, and 35 fields per row"

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Remove-RepoDirectory -Path $CaseOutputRoot
Write-Host "Running exact Schedule:Compact daylight-saving hour-24 tomorrow day-type gate."
$output = & $cargo.Source run -p ep_cli --quiet -- conformance time-weather-schedule-report $CasePath $OracleRoot $OutputRoot 2>&1
if ($LASTEXITCODE -ne 0) {
    $output | ForEach-Object { Write-Host $_ }
    throw "Schedule:Compact daylight-saving hour-24 tomorrow day-type exact gate failed."
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
$expandedManifestPath = Join-Path $CaseOutputRoot "oracle\case-expanded.toml"
$stagedIdfPath = Join-Path $CaseOutputRoot "oracle\input.idf"
foreach ($path in @(
    $summaryPath,
    $reportPath,
    $oracleEsoPath,
    $oracleEioPath,
    $oracleErrPath,
    $oracleEndPath,
    $expandedManifestPath,
    $stagedIdfPath
)) {
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Missing DST rollover comparison artifact: $path"
    }
}

$expandedText = Get-Content -LiteralPath $expandedManifestPath -Raw -Encoding UTF8
$expandedInput = Get-TomlSectionBlock -Text $expandedText -Name "input" -Description "expanded manifest"
$expandedInjection = Get-TomlSectionBlock -Text $expandedText -Name "output_injection" -Description "expanded manifest"
$expandedIdfRef = Get-TomlStringValue -Text $expandedInput -Name "source_idf" -Description "expanded manifest [input]"
$expandedWeatherRef = Get-TomlStringValue -Text $expandedInput -Name "source_weather" -Description "expanded manifest [input]"
$expandedStagedIdf = Get-TomlStringValue -Text $expandedInput -Name "staged_idf" -Description "expanded manifest [input]"
$expandedConvertedEpjson = Get-TomlStringValue -Text $expandedInput -Name "converted_epjson" -Description "expanded manifest [input]"
Assert-SamePath -Actual (Resolve-RepoReference -Reference $expandedIdfRef -Description "expanded source_idf") -Expected $IdfPath -Description "expanded source_idf"
Assert-SamePath -Actual (Resolve-RepoReference -Reference $expandedWeatherRef -Description "expanded source_weather") -Expected $WeatherPath -Description "expanded source_weather"
if ($expandedStagedIdf -cne "input.idf" -or $expandedConvertedEpjson -cne "input.epJSON") {
    throw "Expanded manifest must retain the exact staged IDF and converted epJSON provenance names"
}
foreach ($injectionField in @(
    'schema = "rusted-energyplus.output-injection.v1"',
    'staged_idf_contains_manifest_requests = true',
    'outputs = 0',
    'meters = 0',
    'surface_details = false'
)) {
    Assert-Contains -Text $expandedInjection -Pattern $injectionField -Description "expanded no-op output injection"
}
Assert-Contains -Text $expandedText -Pattern "script = `"$GateCommand`"" -Description "expanded blocking gate attribution"

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
    throw "Oracle staged IDF must equal the canonical DST rollover fixture plus the locked no-op output-injection footer"
}
Write-Host "OK oracle staged IDF canonical text and output-injection footer"

$summary = Get-Content -LiteralPath $summaryPath -Raw -Encoding UTF8 | ConvertFrom-Json
if ($summary.schema_version -ne 1 -or $summary.case_id -cne $CaseId -or
    $summary.oracle_version -cne "26.1.0" -or $summary.comparison_class -cne "conformance" -or
    $summary.conformance_claim -ne $true -or $summary.status -cne "pass" -or
    $summary.time_axis_samples -ne 72 -or $summary.series_count -ne 1 -or
    $summary.conformance_series_count -ne 1 -or $summary.gate.script -cne $GateCommand -or
    $summary.gate.blocking -ne $true) {
    throw "DST rollover case must retain its exact passing single-series conformance and blocking-gate contract"
}
if ($summary.report_contract.format -cne "markdown" -or
    $summary.report_contract.path -cne ".runtime/time-weather-schedule-conformance/26.1.0/$CaseId/compare/compare-report.md" -or
    $summary.artifacts.compare_report_md -cne "compare-report.md" -or
    $summary.artifacts.compare_summary_json -cne "compare-summary.json") {
    throw "Unexpected DST rollover report and summary artifact contract"
}

$calendar = $summary.weather_calendar
if ($calendar.policy_applied -ne $true -or $calendar.weather_file_allows_leap_years -ne $true -or
    $calendar.start_year -ne 2032 -or $calendar.end_year -ne 2032 -or
    $calendar.gregorian_calendar_days -ne 3 -or $calendar.weather_effective_calendar_days -ne 3 -or
    $calendar.leap_days_skipped -ne 0 -or
    $calendar.start_year_gregorian_leap -ne $true -or
    $calendar.start_year_weather_effective_leap -ne $true -or
    $calendar.end_year_gregorian_leap -ne $true -or
    $calendar.end_year_weather_effective_leap -ne $true) {
    throw "Unexpected DST rollover three-day leap-year weather calendar summary"
}
$daylightSaving = $calendar.daylight_saving
if ($daylightSaving.weather_file_period_declared -ne $true -or
    $daylightSaving.run_period_uses_weather_file_period -ne $true -or
    $daylightSaving.input_file_period_declared -ne $false -or
    $daylightSaving.active -ne $true -or $daylightSaving.effective_source -cne "weather-file" -or
    $calendar.daylight_saving_hourly_samples -ne 48) {
    throw "Unexpected DST rollover daylight-saving source or hourly state"
}
$resolvedDst = $daylightSaving.resolved_period
if ($null -eq $resolvedDst -or $resolvedDst.start_month -ne 10 -or $resolvedDst.start_day -ne 31 -or
    $resolvedDst.start_day_of_year -ne 305 -or $resolvedDst.end_month -ne 3 -or
    $resolvedDst.end_day -ne 28 -or $resolvedDst.end_day_of_year -ne 88 -or
    $resolvedDst.wraps_year -ne $true) {
    throw "Unexpected resolved Last Sunday in October through Last Sunday in March DST period"
}

$specialDays = $summary.special_days
if ($specialDays.weather_file_declared -ne 0 -or
    $specialDays.run_period_uses_weather_file -ne $false -or
    $specialDays.weather_file_resolved -ne 0 -or
    $specialDays.input_file_declared -ne 1 -or
    $specialDays.apply_weekend_rule -ne $false -or
    $specialDays.resolved_count -ne 1 -or $specialDays.hourly_samples -ne 24) {
    throw "Unexpected DST rollover special-day summary counts"
}
$resolvedSpecialDays = @($specialDays.resolved)
if ($resolvedSpecialDays.Count -ne 1) {
    throw "Expected exactly one resolved input-file Holiday"
}
$resolvedSpecialDay = $resolvedSpecialDays[0]
if ($resolvedSpecialDay.name -cne "FINAL ROLLOVER HOLIDAY" -or
    $resolvedSpecialDay.source -cne "input-file" -or
    $resolvedSpecialDay.start_month -ne 11 -or $resolvedSpecialDay.start_day -ne 1 -or
    $resolvedSpecialDay.start_day_of_year -ne 306 -or $resolvedSpecialDay.duration_days -ne 1 -or
    $resolvedSpecialDay.day_type -cne "Holiday" -or $resolvedSpecialDay.day_type_index -ne 8 -or
    $resolvedSpecialDay.weekend_shift_days -ne 0) {
    throw "Unexpected resolved November 1 input-file Holiday"
}
if ($null -ne $summary.weather_record_selection) {
    throw "Schedule-only DST rollover comparison must not claim Rust EPW record selection"
}

$allSeries = @($summary.series)
if ($allSeries.Count -ne 1) {
    throw "Summary must contain exactly one promoted series object"
}
$seriesRows = @($allSeries | Where-Object {
    $_.key -eq "DST FINAL ROLLOVER SCHEDULE" -and $_.variable -eq "Schedule Value"
})
if ($seriesRows.Count -ne 1) {
    throw "Expected exactly one DST Final Rollover Schedule Value series"
}
$series = $seriesRows[0]
if ($series.level -cne "conformance" -or $series.class -cne "schedule" -or
    $series.frequency -cne "hourly" -or $series.source -cne "eso" -or
    $series.alignment -cne "timestamp" -or $series.expected_samples -ne 72 -or
    $series.observed_samples -ne 72 -or $series.compared_samples -ne 72) {
    throw "Unexpected DST rollover Schedule Value series metadata or counts"
}
if ($series.timestamp_contract -cne "ordered-exact-unique" -or $series.timestamp_status -cne "pass" -or
    $series.timestamp_expected_unique -ne $true -or $series.timestamp_observed_unique -ne $true -or
    $series.timestamp_order_match -ne $true) {
    throw "DST rollover timestamp uniqueness or order failed"
}
$expectedFirstTimestamp = "env=DST SCHEDULE FINAL ROLLOVER RUN PERIOD;day=1;month=10;date=30;dst=0;hour=1;start=0.00;end=60.00;day_type=Saturday"
$expectedLastTimestamp = "env=DST SCHEDULE FINAL ROLLOVER RUN PERIOD;day=3;month=11;date=1;dst=1;hour=24;start=0.00;end=60.00;day_type=Holiday"
if ($series.expected_first_timestamp -cne $expectedFirstTimestamp -or
    $series.observed_first_timestamp -cne $expectedFirstTimestamp -or
    $series.expected_last_timestamp -cne $expectedLastTimestamp -or
    $series.observed_last_timestamp -cne $expectedLastTimestamp) {
    throw "Unexpected DST rollover timestamp endpoints"
}
if ($series.max_abs_tolerance -ne 0.0 -or $series.max_rel_tolerance -ne 0.0 -or
    $series.max_rmse_tolerance -ne 0.0 -or $series.max_abs_delta -ne 0.0 -or
    $series.rmse_delta -ne 0.0 -or $series.max_rel_delta -ne 0.0 -or
    $null -ne $series.first_divergence -or $null -ne $series.first_timestamp_divergence -or
    $series.status -cne "pass") {
    throw "DST rollover schedule values must match exactly at zero tolerance"
}

$oracleEsoLines = @(Get-Content -LiteralPath $oracleEsoPath -Encoding UTF8)
$expectedDictionaryRows = @(
    "7,1,Environment,Site Daylight Saving Time Status [] !Hourly",
    "8,1,Environment,Site Day Type Index [] !Hourly",
    "9,1,DST FINAL ROLLOVER SCHEDULE,Schedule Value [] !Hourly"
)
$actualDictionaryRows = @($oracleEsoLines | Where-Object {
    $_ -match '^\d+,1,(?:Environment|DST FINAL ROLLOVER SCHEDULE),(?:Site Daylight Saving Time Status|Site Day Type Index|Schedule Value) \[\] !Hourly$'
})
if (($actualDictionaryRows -join '||') -cne ($expectedDictionaryRows -join '||')) {
    throw "Expected the exact ordered raw ESO DST, day-type, and schedule dictionaries"
}
$dstValues = @($oracleEsoLines | Where-Object { $_ -match '^7,\s*[-+0-9.E]+\s*$' } | ForEach-Object { [double](($_ -split ',', 2)[1].Trim()) })
$dayTypeValues = @($oracleEsoLines | Where-Object { $_ -match '^8,\s*[-+0-9.E]+\s*$' } | ForEach-Object { [double](($_ -split ',', 2)[1].Trim()) })
$scheduleValues = @($oracleEsoLines | Where-Object { $_ -match '^9,\s*[-+0-9.E]+\s*$' } | ForEach-Object { [double](($_ -split ',', 2)[1].Trim()) })
$timestampRows = @($oracleEsoLines | Where-Object { $_ -match '^2,\d+,\s*\d+,\s*\d+,\s*\d+,\s*\d+,' })
if ($dstValues.Count -ne 72 -or $dayTypeValues.Count -ne 72 -or $scheduleValues.Count -ne 72 -or $timestampRows.Count -ne 72) {
    throw "Expected 72 raw oracle DST values, day types, schedule values, and hourly timestamps"
}

$expectedDailyDst = @(0.0, 1.0, 1.0)
$expectedDailyTypeValues = @(7.0, 1.0, 8.0)
$expectedDailyTypeNames = @("Saturday", "Sunday", "Holiday")
for ($index = 0; $index -lt 72; ++$index) {
    $dayIndex = [int][Math]::Floor($index / 24)
    $expectedHour = ($index % 24) + 1
    $expectedScheduleValue = if ($dayIndex -eq 0) {
        if ($expectedHour -eq 24) { 124.0 } else { 100.0 }
    }
    elseif ($dayIndex -eq 1) {
        if ($expectedHour -eq 24) { 801.0 } else { 200.0 }
    }
    else {
        if ($expectedHour -eq 24) { 901.0 } else { 800.0 }
    }
    if ($dstValues[$index] -ne $expectedDailyDst[$dayIndex] -or
        $dayTypeValues[$index] -ne $expectedDailyTypeValues[$dayIndex] -or
        $scheduleValues[$index] -ne $expectedScheduleValue) {
        throw "Unexpected raw DST/day-type/schedule value at sample ${index}: $($dstValues[$index]) / $($dayTypeValues[$index]) / $($scheduleValues[$index])"
    }
    $timestampMatch = [regex]::Match(
        $timestampRows[$index],
        '^2,\s*(\d+),\s*(\d+),\s*(\d+),\s*(\d+),\s*(\d+),\s*([-+0-9.]+),\s*([-+0-9.]+),([^,]+)$'
    )
    if (-not $timestampMatch.Success -or
        [int]$timestampMatch.Groups[1].Value -ne ($dayIndex + 1) -or
        [int]$timestampMatch.Groups[2].Value -ne $expectedMonths[$dayIndex] -or
        [int]$timestampMatch.Groups[3].Value -ne $expectedDates[$dayIndex] -or
        [int]$timestampMatch.Groups[4].Value -ne [int]$expectedDailyDst[$dayIndex] -or
        [int]$timestampMatch.Groups[5].Value -ne $expectedHour -or
        $timestampMatch.Groups[6].Value -cne "0.00" -or
        $timestampMatch.Groups[7].Value -cne "60.00" -or
        $timestampMatch.Groups[8].Value.Trim() -cne $expectedDailyTypeNames[$dayIndex]) {
        throw "Unexpected raw DST rollover timestamp at sample ${index}: $($timestampRows[$index])"
    }
}

$expectedScheduleCounts = @{
    "100" = 23
    "124" = 1
    "200" = 23
    "801" = 1
    "800" = 23
    "901" = 1
}
foreach ($entry in $expectedScheduleCounts.GetEnumerator()) {
    $value = [double]$entry.Key
    $count = @($scheduleValues | Where-Object { $_ -eq $value }).Count
    if ($count -ne $entry.Value) {
        throw "Expected exactly $($entry.Value) raw schedule samples with value $value, found $count"
    }
}
if (@($dstValues | Where-Object { $_ -eq 0.0 }).Count -ne 24 -or @($dstValues | Where-Object { $_ -eq 1.0 }).Count -ne 48) {
    throw "Raw DST vector must remain 0 for 24 samples followed by 1 for 48 samples"
}
foreach ($expectedDayType in $expectedDailyTypeValues) {
    if (@($dayTypeValues | Where-Object { $_ -eq $expectedDayType }).Count -ne 24) {
        throw "Expected exactly 24 raw day-type samples with index $expectedDayType"
    }
}
Write-Host "OK exact raw ESO schedule, DST, day-type, and timestamp vectors"

$oracleEioLines = @(Get-Content -LiteralPath $oracleEioPath -Encoding UTF8)
$expectedEnvironmentRow = "Environment,DST SCHEDULE FINAL ROLLOVER RUN PERIOD,WeatherFileRunPeriod,10/30/2032,11/01/2032,Saturday,3,Use RunPeriod Specified Day,Yes,No,No,No,No,Clark and Allen"
$expectedDstRow = "Environment:Daylight Saving,Yes,WeatherFile,10/31,03/28"
$expectedSpecialDayRow = "Environment:Special Days,FINAL ROLLOVER HOLIDAY,Holiday,InputFile,11/01,  1"
$environmentRows = @($oracleEioLines | Where-Object { $_ -like "Environment,DST SCHEDULE FINAL ROLLOVER RUN PERIOD,*" })
$dstRows = @($oracleEioLines | Where-Object { $_ -like "Environment:Daylight Saving,*" })
$specialDayRows = @($oracleEioLines | Where-Object { $_ -like "Environment:Special Days,*" })
if ($environmentRows.Count -ne 1 -or $environmentRows[0] -cne $expectedEnvironmentRow -or
    @($environmentRows[0] -split ',').Count -ne 14 -or
    $dstRows.Count -ne 1 -or $dstRows[0] -cne $expectedDstRow -or @($dstRows[0] -split ',').Count -ne 5 -or
    $specialDayRows.Count -ne 1 -or $specialDayRows[0] -cne $expectedSpecialDayRow -or @($specialDayRows[0] -split ',').Count -ne 6) {
    throw "Unexpected exact DST rollover Environment, Daylight Saving, or Special Days EIO rows"
}
Write-Host "OK exact Environment, weather-file DST, and November 1 Holiday EIO rows"

$oracleErrText = Get-Content -LiteralPath $oracleErrPath -Raw -Encoding UTF8
$oracleEndText = Get-Content -LiteralPath $oracleEndPath -Raw -Encoding UTF8
$completion = "EnergyPlus Completed Successfully-- 0 Warning; 0 Severe Errors;"
Assert-Contains -Text $oracleErrText -Pattern $completion -Description "clean EnergyPlus error-file completion"
Assert-Contains -Text $oracleEndText -Pattern $completion -Description "clean EnergyPlus end record"
if ([regex]::Matches($oracleErrText, '(?m)^\s*\*\* Warning \*\*').Count -ne 0 -or
    [regex]::Matches($oracleErrText, '(?m)^\s*\*\* Severe\s+\*\*').Count -ne 0) {
    throw "DST rollover oracle must complete without warning or severe markers"
}

$reportText = Get-Content -LiteralPath $reportPath -Raw -Encoding UTF8
foreach ($reportContract in @(
    "status: pass",
    "series: 1",
    "conformance_series: 1",
    "time_axis_samples: 72",
    "gregorian_calendar_years: 2032..2032",
    "weather_file_daylight_saving_period_declared: true",
    "run_period_uses_weather_file_daylight_saving_period: true",
    "input_file_daylight_saving_period_declared: false",
    "daylight_saving_active: true",
    "daylight_saving_effective_source: weather-file",
    "daylight_saving_resolved_period: 10/31 through 3/28 (wraps_year=true)",
    "daylight_saving_hourly_samples: 48",
    "input_file_special_days_declared: 1",
    "special_days_resolved: 1",
    "special_day_hourly_samples: 24",
    "special_day_resolved: FINAL ROLLOVER HOLIDAY 11/1 duration=1 day_type=Holiday weekend_shift_days=0 source=input-file",
    "weather_record_selection_applied: false",
    "| DST FINAL ROLLOVER SCHEDULE | Schedule Value | conformance | schedule | hourly | eso | timestamp | 72 | 72 | 72 | 0.000000000000 | 0.000000000000 | 0.000000000000 |"
)) {
    Assert-Contains -Text $reportText -Pattern $reportContract -Description "markdown DST rollover contract"
}

Write-Host "Schedule:Compact daylight-saving hour-24 tomorrow day-type exact gate passed."
