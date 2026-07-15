[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$CaseId = "calendar_schedule_file_8760_leap_exact_001"
$CaseRoot = Join-Path $RepoRoot "data\conformance_cases\$CaseId"
$CasePath = Join-Path $CaseRoot "case.toml"
$IdfPath = Join-Path $CaseRoot "calendar_schedule_file_8760_leap_exact.idf"
$WeatherPath = Join-Path $CaseRoot "calendar_schedule_file_8760_leap_exact.epw"
$CsvPath = Join-Path $CaseRoot "schedule_file_8760.csv"
$GateCommand = "scripts/dev.cmd compare-calendar-schedule-file-8760-leap-exact"
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
    $WeatherPath,
    $CsvPath
)) {
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Missing required Schedule:File exact-case file: $path"
    }
}

$caseText = Get-Content -LiteralPath $CasePath -Raw -Encoding UTF8
$idfText = Get-Content -LiteralPath $IdfPath -Raw -Encoding UTF8
$weatherLines = @(Get-Content -LiteralPath $WeatherPath -Encoding UTF8)
$weatherRows = @($weatherLines | Select-Object -Skip 8 | Where-Object { -not [string]::IsNullOrWhiteSpace($_) })
$csvLines = @(Get-Content -LiteralPath $CsvPath -Encoding UTF8)

foreach ($contract in @(
    'comparison_class = "conformance"',
    'conformance_claim = true',
    'source_file = "data/conformance_cases/calendar_schedule_file_8760_leap_exact_001/calendar_schedule_file_8760_leap_exact.idf"',
    'idf = "data/conformance_cases/calendar_schedule_file_8760_leap_exact_001/calendar_schedule_file_8760_leap_exact.idf"',
    'weather = "data/conformance_cases/calendar_schedule_file_8760_leap_exact_001/calendar_schedule_file_8760_leap_exact.epw"',
    'auxiliary_files = ["schedule_file_8760.csv"]',
    'frequency = "hourly"',
    'timestamp_contract = "ordered-exact-unique"',
    'abs_tol = 0.0',
    'rmse_tol = 0.0',
    'February 28 is 1393 through 1416, February 29 repeats 1393 through 1416 exactly, and March 1 is 1417 through 1440',
    'An 8784-row file',
    'subhourly minutes per item',
    'Interpolate:Yes behavior',
    'daylight-saving adjustment',
    'multiple Schedule:File objects',
    'Schedule:File:Shading',
    'arbitrary-run sidecar staging',
    'Rust raw ESO serialization',
    'broad warning/error parity',
    'script = "scripts/dev.cmd compare-calendar-schedule-file-8760-leap-exact"',
    'blocking = true'
)) {
    Assert-Contains -Text $caseText -Pattern $contract -Description "canonical manifest contract"
}
if (@([regex]::Matches($caseText, '(?m)^\[\[outputs\]\]$')).Count -ne 1) {
    throw "Manifest must retain exactly one output request"
}

$actualVectors = @(Get-CompleteIdfObjectVectors -Text $idfText -Description "canonical fixture")
$expectedVectors = @(
    "Version|26.1",
    "Building|Schedule File 8760 Leap Exact Fixture|0.0|Suburbs|0.04|0.4|FullExterior|25|6",
    "Timestep|4",
    "GlobalGeometryRules|UpperLeftCorner|CounterClockWise|World",
    "RunPeriod|Schedule File 8760 Leap Run Period|2|28|2016|3|1|2016|Sunday|No|No|No|No|No|No",
    "ScheduleTypeLimits|Any Number",
    "Schedule:File|Selected Column 8760 Schedule|Any Number|schedule_file_8760.csv|2|1|8760|Comma|No|60|No",
    "Output:Variable|SELECTED COLUMN 8760 SCHEDULE|Schedule Value|Hourly"
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
    "LOCATION,Schedule File 8760 Leap Exact Fixture,CO,USA,Synthetic,999999,39.74,-105.18,-7.0,1829.0",
    "DESIGN CONDITIONS,0",
    "TYPICAL/EXTREME PERIODS,0",
    "GROUND TEMPERATURES,0",
    "HOLIDAYS/DAYLIGHT SAVINGS,Yes,0,0,0",
    "COMMENTS 1,Deterministic 72-hour leap-day weather fixture for Schedule File 8760 duplication",
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

if ($csvLines.Count -ne 8761 -or $csvLines[0] -cne "decoy,selected,extra") {
    throw "Schedule CSV must retain one exact header and exactly 8760 data rows"
}
for ($index = 1; $index -le 8760; ++$index) {
    $expected = "$(-$index),$index,$(10000 + $index)"
    if ($csvLines[$index] -cne $expected) {
        throw "Unexpected Schedule CSV row $($index): $($csvLines[$index])"
    }
}
Write-Host "OK exact three-column 8760-row Schedule:File CSV"

Remove-RepoDirectory -Path $CaseOutputRoot
$cargo = Get-Command cargo -ErrorAction Stop
$output = & $cargo.Source run -p ep_cli --quiet -- conformance time-weather-schedule-report $CasePath $OracleRoot $OutputRoot 2>&1
if ($LASTEXITCODE -ne 0) {
    $joinedOutput = $output -join [Environment]::NewLine
    throw "Schedule:File report failed with exit code $LASTEXITCODE $joinedOutput"
}
$output | ForEach-Object { Write-Host $_ }

$summaryPath = Join-Path $CaseOutputRoot "compare\compare-summary.json"
$reportPath = Join-Path $CaseOutputRoot "compare\compare-report.md"
$oracleEsoPath = Join-Path $CaseOutputRoot "oracle\eplusout.eso"
$oracleEioPath = Join-Path $CaseOutputRoot "oracle\eplusout.eio"
$oracleErrPath = Join-Path $CaseOutputRoot "oracle\eplusout.err"
$oracleEndPath = Join-Path $CaseOutputRoot "oracle\eplusout.end"
$stagedCsvPath = Join-Path $CaseOutputRoot "oracle\schedule_file_8760.csv"
foreach ($path in @(
    $summaryPath,
    $reportPath,
    $oracleEsoPath,
    $oracleEioPath,
    $oracleErrPath,
    $oracleEndPath,
    $stagedCsvPath
)) {
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Missing Schedule:File report artifact: $path"
    }
}
if ((Get-FileHash -LiteralPath $CsvPath -Algorithm SHA256).Hash -cne
    (Get-FileHash -LiteralPath $stagedCsvPath -Algorithm SHA256).Hash) {
    throw "Staged Schedule:File CSV must be byte-identical to the canonical auxiliary file"
}
Write-Host "OK canonical CSV staged beside oracle input.idf"

$summary = Get-Content -LiteralPath $summaryPath -Raw -Encoding UTF8 | ConvertFrom-Json
if ($summary.case_id -cne $CaseId -or $summary.oracle_version -cne "26.1.0" -or
    $summary.comparison_class -cne "conformance" -or $summary.conformance_claim -ne $true -or
    $summary.status -cne "pass" -or $summary.series_count -ne 1 -or
    $summary.conformance_series_count -ne 1 -or $summary.time_axis_samples -ne 72 -or
    $summary.timestamp_rule -cne "hour-ending hourly samples aligned by EnergyPlus ESO timestamp labels" -or
    $summary.gate.script -cne $GateCommand -or $summary.gate.blocking -ne $true) {
    throw "Unexpected Schedule:File report summary contract"
}
if ($null -ne $summary.weather_record_selection) {
    throw "Schedule-only comparison must not claim Rust EPW record selection"
}
$seriesRows = @($summary.series | Where-Object {
    $_.key -eq "SELECTED COLUMN 8760 SCHEDULE" -and $_.variable -eq "Schedule Value"
})
if ($seriesRows.Count -ne 1) {
    throw "Expected exactly one Schedule:File Schedule Value series"
}
$series = $seriesRows[0]
if ($series.level -cne "conformance" -or $series.class -cne "schedule" -or
    $series.frequency -cne "hourly" -or $series.source -cne "eso" -or
    $series.alignment -cne "timestamp" -or $series.expected_samples -ne 72 -or
    $series.observed_samples -ne 72 -or $series.compared_samples -ne 72 -or
    $series.timestamp_contract -cne "ordered-exact-unique" -or
    $series.timestamp_status -cne "pass" -or $series.timestamp_expected_unique -ne $true -or
    $series.timestamp_observed_unique -ne $true -or $series.timestamp_order_match -ne $true) {
    throw "Unexpected Schedule:File series metadata, count, or timestamp contract"
}
$firstTimestamp = "env=SCHEDULE FILE 8760 LEAP RUN PERIOD;day=1;month=2;date=28;dst=0;hour=1;start=0.00;end=60.00;day_type=Sunday"
$lastTimestamp = "env=SCHEDULE FILE 8760 LEAP RUN PERIOD;day=3;month=3;date=1;dst=0;hour=24;start=0.00;end=60.00;day_type=Tuesday"
if ($series.expected_first_timestamp -cne $firstTimestamp -or
    $series.observed_first_timestamp -cne $firstTimestamp -or
    $series.expected_last_timestamp -cne $lastTimestamp -or
    $series.observed_last_timestamp -cne $lastTimestamp -or
    $series.max_abs_tolerance -ne 0.0 -or $series.max_rel_tolerance -ne 0.0 -or
    $series.max_rmse_tolerance -ne 0.0 -or $series.max_abs_delta -ne 0.0 -or
    $series.rmse_delta -ne 0.0 -or $series.max_rel_delta -ne 0.0 -or
    $null -ne $series.first_divergence -or $null -ne $series.first_timestamp_divergence -or
    $series.status -cne "pass") {
    throw "Schedule:File values and timestamps must match exactly at zero tolerance"
}

$esoLines = @(Get-Content -LiteralPath $oracleEsoPath -Encoding UTF8)
$dictionaryRows = @($esoLines | Where-Object {
    $_ -ceq "7,1,SELECTED COLUMN 8760 SCHEDULE,Schedule Value [] !Hourly"
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
for ($index = 0; $index -lt 72; ++$index) {
    $dayIndex = [int][Math]::Floor($index / 24)
    $hourIndex = $index % 24
    $hour = $hourIndex + 1
    $expectedValue = if ($dayIndex -lt 2) { 1393.0 + $hourIndex } else { 1417.0 + $hourIndex }
    if ($values[$index] -ne $expectedValue) {
        throw "Unexpected raw Schedule:File value at sample $($index): $($values[$index])"
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
        throw "Unexpected raw Schedule:File timestamp at sample $($index): $($timestamps[$index])"
    }
}
if (($values[0..23] -join ',') -cne ($values[24..47] -join ',')) {
    throw "February 29 must repeat the complete February 28 Schedule:File day"
}
if ($values[48] -ne 1417.0 -or $values[71] -ne 1440.0) {
    throw "March 1 must resume at the first common-year post-February value"
}
Write-Host "OK exact raw ESO 8760-row leap-day duplication values and timestamps"

$eioLines = @(Get-Content -LiteralPath $oracleEioPath -Encoding UTF8)
$environmentRow = "Environment,SCHEDULE FILE 8760 LEAP RUN PERIOD,WeatherFileRunPeriod,02/28/2016,03/01/2016,Sunday,3,Use RunPeriod Specified Day,No,No,No,No,No,Clark and Allen"
$daylightRow = "Environment:Daylight Saving,No,RunPeriod Object"
if (@($eioLines | Where-Object { $_ -ceq $environmentRow }).Count -ne 1 -or
    @($eioLines | Where-Object { $_ -ceq $daylightRow }).Count -ne 1) {
    throw "Unexpected exact Environment or disabled daylight-saving EIO row"
}
$completion = "EnergyPlus Completed Successfully-- 0 Warning; 0 Severe Errors;"
$errText = Get-Content -LiteralPath $oracleErrPath -Raw -Encoding UTF8
$endText = Get-Content -LiteralPath $oracleEndPath -Raw -Encoding UTF8
Assert-Contains -Text $errText -Pattern $completion -Description "clean EnergyPlus error-file completion"
Assert-Contains -Text $endText -Pattern $completion -Description "clean EnergyPlus end record"

$reportText = Get-Content -LiteralPath $reportPath -Raw -Encoding UTF8
foreach ($reportContract in @(
    "status: pass",
    "series: 1",
    "conformance_series: 1",
    "time_axis_samples: 72",
    "timestamp_rule: hour-ending hourly samples aligned by EnergyPlus ESO timestamp labels",
    "weather_record_selection_applied: false",
    "| SELECTED COLUMN 8760 SCHEDULE | Schedule Value | conformance | schedule | hourly | eso | timestamp | 72 | 72 | 72 | 0.000000000000 | 0.000000000000 | 0.000000000000 |"
)) {
    Assert-Contains -Text $reportText -Pattern $reportContract -Description "markdown Schedule:File contract"
}

Write-Host "Schedule:File 8760 leap-day exact gate passed."
