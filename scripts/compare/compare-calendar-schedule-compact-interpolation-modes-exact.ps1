[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$CaseId = "calendar_schedule_compact_interpolation_modes_exact_001"
$CaseRoot = Join-Path $RepoRoot "data\conformance_cases\$CaseId"
$CasePath = Join-Path $CaseRoot "case.toml"
$IdfPath = Join-Path $CaseRoot "calendar_schedule_compact_interpolation_modes_exact.idf"
$WeatherPath = Join-Path $CaseRoot "calendar_schedule_compact_interpolation_modes_exact.epw"
$GateCommand = "scripts/dev.cmd compare-calendar-schedule-compact-interpolation-modes-exact"
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

foreach ($path in @(
    (Join-Path $OracleRoot "energyplus.exe"),
    (Join-Path $OracleRoot "ConvertInputFormat.exe"),
    $CasePath,
    $IdfPath,
    $WeatherPath
)) {
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Missing required interpolation-modes schedule file: $path"
    }
}

$caseText = Get-Content -LiteralPath $CasePath -Raw -Encoding UTF8
$idfText = Get-Content -LiteralPath $IdfPath -Raw -Encoding UTF8
$weatherLines = @(Get-Content -LiteralPath $WeatherPath -Encoding UTF8)
$weatherRows = @($weatherLines | Select-Object -Skip 8 | Where-Object { -not [string]::IsNullOrWhiteSpace($_) })

foreach ($contract in @(
    'comparison_class = "conformance"',
    'conformance_claim = true',
    'source_file = "data/conformance_cases/calendar_schedule_compact_interpolation_modes_exact_001/calendar_schedule_compact_interpolation_modes_exact.idf"',
    'idf = "data/conformance_cases/calendar_schedule_compact_interpolation_modes_exact_001/calendar_schedule_compact_interpolation_modes_exact.idf"',
    'weather = "data/conformance_cases/calendar_schedule_compact_interpolation_modes_exact_001/calendar_schedule_compact_interpolation_modes_exact.epw"',
    'frequency = "timestep"',
    'timestamp_contract = "ordered-exact-unique"',
    'abs_tol = 0.0',
    'rmse_tol = 0.0',
    'No [10, 175, 175, 175, 175], Average [10, 120, 175, 175, 175], and Linear [10, 40, 85, 130, 175]',
    'samples 6 through 96 are 175 for every series',
    'cross-hour 01:15 boundary',
    'other timestep counts',
    'Until 24:MM correction',
    'overlap/zero/incomplete schedule errors',
    'daylight-saving combinations',
    'multi-profile mixed interpolation modes',
    'hourly aggregation',
    'downstream schedule consumption',
    'Other schedule families',
    'UpdateScheduleVals orchestration',
    'EMS override behavior',
    'Rust currentVal store',
    'Rust raw ESO serialization',
    'Rust EPW record selection',
    'broad EnergyPlus warning/error parity',
    'script = "scripts/dev.cmd compare-calendar-schedule-compact-interpolation-modes-exact"',
    'blocking = true'
)) {
    Assert-Contains -Text $caseText -Pattern $contract -Description "canonical manifest contract"
}
if (@([regex]::Matches($caseText, '(?m)^\[\[outputs\]\]$')).Count -ne 3 -or
    @([regex]::Matches($caseText, '(?m)^frequency = "timestep"$')).Count -ne 3 -or
    @([regex]::Matches($caseText, '(?m)^timestamp_contract = "ordered-exact-unique"$')).Count -ne 3) {
    throw "Manifest must retain exactly three timestep ordered-exact-unique outputs"
}

$actualVectors = @(Get-CompleteIdfObjectVectors -Text $idfText -Description "canonical fixture")
$expectedVectors = @(
    "Version|26.1",
    "Building|Schedule Compact Interpolation Modes Exact Fixture|0.0|Suburbs|0.04|0.4|FullExterior|25|6",
    "Timestep|4",
    "GlobalGeometryRules|UpperLeftCorner|CounterClockWise|World",
    "RunPeriod|Schedule Compact Interpolation Modes Run Period|1|1|2032|1|1|2032|Thursday|No|No|No|No|No|No",
    "ScheduleTypeLimits|Any Number",
    "Schedule:Compact|Interpolation No Schedule|Any Number|Through: 12/31|For: AllDays|Interpolate: No|Until: 00:20|10|Until: 01:15|175|Until: 24:00|175",
    "Schedule:Compact|Interpolation Average Schedule|Any Number|Through: 12/31|For: AllDays|Interpolate: Average|Until: 00:20|10|Until: 01:15|175|Until: 24:00|175",
    "Schedule:Compact|Interpolation Linear Schedule|Any Number|Through: 12/31|For: AllDays|Interpolate: Linear|Until: 00:20|10|Until: 01:15|175|Until: 24:00|175",
    "Output:Variable|INTERPOLATION NO SCHEDULE|Schedule Value|Timestep",
    "Output:Variable|INTERPOLATION AVERAGE SCHEDULE|Schedule Value|Timestep",
    "Output:Variable|INTERPOLATION LINEAR SCHEDULE|Schedule Value|Timestep"
)
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
    "LOCATION,Schedule Compact Interpolation Modes Exact Fixture,CO,USA,Synthetic,999999,39.74,-105.18,-7.0,1829.0",
    "DESIGN CONDITIONS,0",
    "TYPICAL/EXTREME PERIODS,0",
    "GROUND TEMPERATURES,0",
    "HOLIDAYS/DAYLIGHT SAVINGS,No,0,0,0",
    "COMMENTS 1,Deterministic one-day Schedule Compact interpolation modes fixture",
    "COMMENTS 2,Weather values are constant because only schedule values are compared",
    "DATA PERIODS,1,1,Data,Thursday,1/1,1/1"
)
if ($weatherLines.Count -ne 32 -or $weatherRows.Count -ne 24 -or
    (($weatherLines[0..7] -join '||') -cne ($expectedHeaders -join '||'))) {
    throw "Fixture EPW must retain eight exact headers and 24 hourly rows"
}
$weatherPayload = "?9?9?9?9E0?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9*9*9?9*9*9,10.0,5.0,50,80600,0,0,250,0,0,0,0,0,0,0,180,2.0,5,5,20.0,7777,9,999999999,0,0.0000,0,0,0.000,0.0,0.0"
for ($index = 0; $index -lt 24; ++$index) {
    $expected = "2032,1,1,$($index + 1),60,$weatherPayload"
    if ($weatherRows[$index] -cne $expected) {
        throw "Unexpected EPW row at index $($index): $($weatherRows[$index])"
    }
}
Write-Host "OK exact one-day EPW"

Remove-RepoDirectory -Path $CaseOutputRoot
$cargo = Get-Command cargo -ErrorAction Stop
$output = & $cargo.Source run -p ep_cli --quiet -- conformance time-weather-schedule-report $CasePath $OracleRoot $OutputRoot 2>&1
if ($LASTEXITCODE -ne 0) {
    $joinedOutput = $output -join [Environment]::NewLine
    throw "Interpolation-modes schedule report failed with exit code $LASTEXITCODE $joinedOutput"
}
$output | ForEach-Object { Write-Host $_ }

$summaryPath = Join-Path $CaseOutputRoot "compare\compare-summary.json"
$reportPath = Join-Path $CaseOutputRoot "compare\compare-report.md"
$oracleEsoPath = Join-Path $CaseOutputRoot "oracle\eplusout.eso"
$oracleEioPath = Join-Path $CaseOutputRoot "oracle\eplusout.eio"
$oracleErrPath = Join-Path $CaseOutputRoot "oracle\eplusout.err"
$oracleEndPath = Join-Path $CaseOutputRoot "oracle\eplusout.end"
foreach ($path in @($summaryPath, $reportPath, $oracleEsoPath, $oracleEioPath, $oracleErrPath, $oracleEndPath)) {
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Missing report artifact: $path"
    }
}

$summary = Get-Content -LiteralPath $summaryPath -Raw -Encoding UTF8 | ConvertFrom-Json
if ($summary.case_id -cne $CaseId -or $summary.oracle_version -cne "26.1.0" -or
    $summary.comparison_class -cne "conformance" -or $summary.conformance_claim -ne $true -or
    $summary.status -cne "pass" -or $summary.series_count -ne 3 -or
    $summary.conformance_series_count -ne 3 -or $summary.time_axis_samples -ne 96 -or
    $summary.timestamp_rule -cne "zone-timestep ending samples aligned by EnergyPlus ESO timestamp labels" -or
    $summary.gate.script -cne $GateCommand -or $summary.gate.blocking -ne $true) {
    throw "Unexpected interpolation-modes report summary contract"
}
if ($null -ne $summary.weather_record_selection) {
    throw "Schedule-only comparison must retain null Rust EPW record selection"
}

$expectedFirstTimestamp = "env=SCHEDULE COMPACT INTERPOLATION MODES RUN PERIOD;day=1;month=1;date=1;dst=0;hour=1;start=0.00;end=15.00;day_type=Thursday"
$expectedLastTimestamp = "env=SCHEDULE COMPACT INTERPOLATION MODES RUN PERIOD;day=1;month=1;date=1;dst=0;hour=24;start=45.00;end=60.00;day_type=Thursday"
$probes = @(
    [pscustomobject]@{
        Id = 7
        Key = "INTERPOLATION NO SCHEDULE"
        Dictionary = "7,1,INTERPOLATION NO SCHEDULE,Schedule Value [] !TimeStep"
        FirstValues = @(10.0, 175.0, 175.0, 175.0, 175.0)
    },
    [pscustomobject]@{
        Id = 8
        Key = "INTERPOLATION AVERAGE SCHEDULE"
        Dictionary = "8,1,INTERPOLATION AVERAGE SCHEDULE,Schedule Value [] !TimeStep"
        FirstValues = @(10.0, 120.0, 175.0, 175.0, 175.0)
    },
    [pscustomobject]@{
        Id = 9
        Key = "INTERPOLATION LINEAR SCHEDULE"
        Dictionary = "9,1,INTERPOLATION LINEAR SCHEDULE,Schedule Value [] !TimeStep"
        FirstValues = @(10.0, 40.0, 85.0, 130.0, 175.0)
    }
)

$summarySeries = @($summary.series)
if ($summarySeries.Count -ne 3) {
    throw "Expected exactly three JSON series rows"
}
for ($probeIndex = 0; $probeIndex -lt $probes.Count; ++$probeIndex) {
    $probe = $probes[$probeIndex]
    $series = $summarySeries[$probeIndex]
    if ($series.key -cne $probe.Key -or $series.variable -cne "Schedule Value" -or
        $series.level -cne "conformance" -or $series.class -cne "schedule" -or
        $series.frequency -cne "timestep" -or $series.source -cne "eso" -or
        $series.alignment -cne "timestamp" -or $series.expected_samples -ne 96 -or
        $series.observed_samples -ne 96 -or $series.compared_samples -ne 96 -or
        $series.timestamp_contract -cne "ordered-exact-unique" -or
        $series.timestamp_status -cne "pass" -or $series.timestamp_expected_unique -ne $true -or
        $series.timestamp_observed_unique -ne $true -or $series.timestamp_order_match -ne $true) {
        throw "Unexpected JSON metadata, sample count, source order, or timestamp contract for $($probe.Key)"
    }
    if ($series.expected_first_timestamp -cne $expectedFirstTimestamp -or
        $series.observed_first_timestamp -cne $expectedFirstTimestamp -or
        $series.expected_last_timestamp -cne $expectedLastTimestamp -or
        $series.observed_last_timestamp -cne $expectedLastTimestamp -or
        $series.max_abs_tolerance -ne 0.0 -or $series.max_rel_tolerance -ne 0.0 -or
        $series.max_rmse_tolerance -ne 0.0 -or $series.max_abs_delta -ne 0.0 -or
        $series.rmse_delta -ne 0.0 -or $series.max_rel_delta -ne 0.0 -or
        $null -ne $series.first_divergence -or $null -ne $series.first_timestamp_divergence -or
        $series.status -cne "pass") {
        throw "JSON values and exact first/last timestamps must match at zero delta for $($probe.Key)"
    }
}
Write-Host "OK JSON 3/3 series with exact timestamps and zero delta"

$esoLines = @(Get-Content -LiteralPath $oracleEsoPath -Encoding UTF8)
$dictionaryRows = @($esoLines | Where-Object { $_ -match '^[789],1,.*Schedule Value \[\] !TimeStep$' })
$expectedDictionaryRows = @($probes | ForEach-Object { $_.Dictionary })
if ($dictionaryRows.Count -ne 3 -or
    (($dictionaryRows -join '||') -cne ($expectedDictionaryRows -join '||'))) {
    throw "Expected exact source-ordered ESO dictionary probe rows 7, 8, and 9"
}

$timestamps = @($esoLines | Where-Object { $_ -match '^2,\d+,\s*\d+,\s*\d+,\s*\d+,\s*\d+,' })
if ($timestamps.Count -ne 96) {
    throw "Expected exactly 96 raw ESO zone-timestep timestamps"
}
for ($index = 0; $index -lt 96; ++$index) {
    $hour = [int][Math]::Floor($index / 4) + 1
    $zoneTimestep = $index % 4
    $startMinute = $zoneTimestep * 15
    $endMinute = ($zoneTimestep + 1) * 15
    $timestampMatch = [regex]::Match(
        $timestamps[$index],
        '^2,\s*1,\s*1,\s*1,\s*0,\s*(\d+),\s*([-+0-9.]+),\s*([-+0-9.]+),Thursday$'
    )
    if (-not $timestampMatch.Success -or
        [int]$timestampMatch.Groups[1].Value -ne $hour -or
        [double]$timestampMatch.Groups[2].Value -ne $startMinute -or
        [double]$timestampMatch.Groups[3].Value -ne $endMinute) {
        throw "Unexpected raw zone-timestep timestamp at sample $($index): $($timestamps[$index])"
    }
}

foreach ($probe in $probes) {
    $valuePattern = '^' + [regex]::Escape([string]$probe.Id) + ',\s*[-+0-9.E]+\s*$'
    $values = @($esoLines | Where-Object { $_ -match $valuePattern } |
        ForEach-Object { [double](($_ -split ',', 2)[1].Trim()) })
    if ($values.Count -ne 96) {
        throw "Expected exactly 96 raw ESO values for $($probe.Key), got $($values.Count)"
    }
    for ($index = 0; $index -lt 5; ++$index) {
        if ($values[$index] -ne $probe.FirstValues[$index]) {
            throw "Unexpected leading interpolation value for $($probe.Key) at sample $($index): $($values[$index])"
        }
    }
    $trailingValues = @($values | Select-Object -Skip 5)
    if ($trailingValues.Count -ne 91 -or
        @($trailingValues | Where-Object { $_ -ne 175.0 }).Count -ne 0) {
        throw "Expected samples 6 through 96 for $($probe.Key) to equal 175"
    }
}
Write-Host "OK exact raw ESO source order, 96 timestamps, and 96 values per series"

$eioLines = @(Get-Content -LiteralPath $oracleEioPath -Encoding UTF8)
$environmentRow = "Environment,SCHEDULE COMPACT INTERPOLATION MODES RUN PERIOD,WeatherFileRunPeriod,01/01/2032,01/01/2032,Thursday,1,Use RunPeriod Specified Day,No,No,No,No,No,Clark and Allen"
$daylightRow = "Environment:Daylight Saving,No,RunPeriod Object"
if (@($eioLines | Where-Object { $_ -ceq $environmentRow }).Count -ne 1 -or
    @($eioLines | Where-Object { $_ -ceq $daylightRow }).Count -ne 1) {
    throw "Unexpected exact Environment or disabled daylight-saving EIO row"
}

$errText = Get-Content -LiteralPath $oracleErrPath -Raw -Encoding UTF8
$endText = Get-Content -LiteralPath $oracleEndPath -Raw -Encoding UTF8
$completion = "EnergyPlus Completed Successfully-- 1 Warning; 0 Severe Errors;"
if ([regex]::Matches($errText, [regex]::Escape($completion)).Count -ne 1 -or
    [regex]::Matches($endText, [regex]::Escape($completion)).Count -ne 1) {
    throw "EnergyPlus ERR and END must each contain the exact 1 Warning; 0 Severe Errors completion"
}
$untilWarning = 'ProcessScheduleInput: DecodeHHMMField, Invalid "until" field value is not a multiple of the minutes for each timestep: UNTIL: 00:20'
$noWarningContext = "Other errors may result. Occurred in Day Schedule=INTERPOLATION NO SCHEDULE_dy_1"
if ([regex]::Matches($errText, [regex]::Escape($untilWarning)).Count -ne 1 -or
    [regex]::Matches($errText, [regex]::Escape($noWarningContext)).Count -ne 1) {
    throw "Expected exactly one non-multiple UNTIL: 00:20 warning for explicit Interpolate: No"
}
Assert-NotContains -Text $errText -Pattern "Occurred in Day Schedule=INTERPOLATION AVERAGE SCHEDULE" -Description "Average interpolation non-multiple warning"
Assert-NotContains -Text $errText -Pattern "Occurred in Day Schedule=INTERPOLATION LINEAR SCHEDULE" -Description "Linear interpolation non-multiple warning"
Write-Host "OK exact EnergyPlus completion and explicit-No-only warning"

$reportText = Get-Content -LiteralPath $reportPath -Raw -Encoding UTF8
foreach ($reportContract in @(
    "status: pass",
    "series: 3",
    "conformance_series: 3",
    "time_axis_samples: 96",
    "timestamp_rule: zone-timestep ending samples aligned by EnergyPlus ESO timestamp labels",
    "weather_record_selection_applied: false",
    "| INTERPOLATION NO SCHEDULE | Schedule Value | conformance | schedule | timestep | eso | timestamp | 96 | 96 | 96 | 0.000000000000 | 0.000000000000 | 0.000000000000 |",
    "| INTERPOLATION AVERAGE SCHEDULE | Schedule Value | conformance | schedule | timestep | eso | timestamp | 96 | 96 | 96 | 0.000000000000 | 0.000000000000 | 0.000000000000 |",
    "| INTERPOLATION LINEAR SCHEDULE | Schedule Value | conformance | schedule | timestep | eso | timestamp | 96 | 96 | 96 | 0.000000000000 | 0.000000000000 | 0.000000000000 |"
)) {
    Assert-Contains -Text $reportText -Pattern $reportContract -Description "markdown interpolation-modes contract"
}

Write-Host "Schedule:Compact interpolation-modes exact gate passed."
