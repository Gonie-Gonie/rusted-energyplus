[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$CaseId = "calendar_schedule_file_shading_exact_001"
$CaseRoot = Join-Path $RepoRoot "data\conformance_cases\$CaseId"
$CasePath = Join-Path $CaseRoot "case.toml"
$IdfPath = Join-Path $CaseRoot "calendar_schedule_file_shading_exact.idf"
$WeatherPath = Join-Path $CaseRoot "calendar_schedule_file_shading_exact.epw"
$CsvPath = Join-Path $CaseRoot "schedule_file_shading.csv"
$GateCommand = "scripts/dev.cmd compare-calendar-schedule-file-shading-exact"
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$OutputRoot = Join-Path $RepoRoot ".runtime\time-weather-schedule-conformance\26.1.0"
$CaseOutputRoot = Join-Path $OutputRoot $CaseId

function Assert-Contains {
    param([string]$Text, [string]$Pattern, [string]$Description)
    if ($Text -notmatch [regex]::Escape($Pattern)) {
        throw "Missing $($Description): $Pattern"
    }
    Write-Host "OK $($Description): $Pattern"
}

function Get-CompleteIdfObjectVectors {
    param([string]$Text)
    $withoutComments = [regex]::Replace($Text, '(?m)!.*$', '')
    $chunks = @($withoutComments -split ';' | ForEach-Object { $_.Trim() } | Where-Object { $_.Length -gt 0 })
    $vectors = @()
    foreach ($chunk in $chunks) {
        $object = [regex]::Match($chunk, '(?s)^(?<type>[A-Za-z0-9:]+)\s*,(?<body>.*)$')
        if (-not $object.Success) {
            throw "Fixture contains a non-object semicolon-delimited chunk: $chunk"
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
        throw "Missing required Schedule:File:Shading exact-case file: $path"
    }
}

$caseText = Get-Content -LiteralPath $CasePath -Raw -Encoding UTF8
$idfText = Get-Content -LiteralPath $IdfPath -Raw -Encoding UTF8
$weatherLines = @(Get-Content -LiteralPath $WeatherPath -Encoding UTF8)
$csvLines = @(Get-Content -LiteralPath $CsvPath -Encoding UTF8)

foreach ($contract in @(
    'comparison_class = "conformance"',
    'conformance_claim = true',
    'auxiliary_files = ["schedule_file_shading.csv"]',
    'ALPHA SURFACE rises from 0.01 through 0.96',
    'BETA SURFACE falls from 0.96 through 0.01',
    'There are deliberately no zones, surfaces, or ShadowCalculation object',
    'does not claim Imported ShadowCalculation',
    'JSON and unknown-extension inputs',
    'multiple Schedule:File:Shading objects',
    'timestamp parsing or validation',
    'the 366-day row-count branch',
    'script = "scripts/dev.cmd compare-calendar-schedule-file-shading-exact"',
    'blocking = true'
)) {
    Assert-Contains $caseText $contract "canonical manifest contract"
}
if (@([regex]::Matches($caseText, '(?m)^\[\[outputs\]\]$')).Count -ne 2 -or
    @([regex]::Matches($caseText, '(?m)^frequency = "timestep"$')).Count -ne 2 -or
    @([regex]::Matches($caseText, '(?m)^timestamp_contract = "ordered-exact-unique"$')).Count -ne 2) {
    throw "Manifest must retain exactly two timestep ordered-exact-unique outputs"
}

$expectedVectors = @(
    "Version|26.1",
    "Building|Schedule File Shading Exact Fixture|0.0|Suburbs|0.04|0.4|FullExterior|25|6",
    "Timestep|4",
    "GlobalGeometryRules|UpperLeftCorner|CounterClockWise|World",
    "RunPeriod|Schedule File Shading Run Period|1|1|2013|1|1|2013|Tuesday|No|No|No|No|No|No",
    "Schedule:File:Shading|schedule_file_shading.csv",
    "Output:Variable|ALPHA SURFACE_SHADING|Schedule Value|Timestep",
    "Output:Variable|BETA SURFACE_SHADING|Schedule Value|Timestep"
)
$actualVectors = @(Get-CompleteIdfObjectVectors $idfText)
if (($actualVectors -join '||') -cne ($expectedVectors -join '||')) {
    throw "Fixture must retain the exact complete IDF object order and fields"
}
Write-Host "OK exact complete fixture IDF vectors"

$expectedHeaders = @(
    "LOCATION,Schedule File Shading Exact Fixture,CO,USA,Synthetic,999999,39.74,-105.18,-7.0,1829.0",
    "DESIGN CONDITIONS,0",
    "TYPICAL/EXTREME PERIODS,0",
    "GROUND TEMPERATURES,0",
    "HOLIDAYS/DAYLIGHT SAVINGS,No,0,0,0",
    "COMMENTS 1,Deterministic one-day Schedule File Shading timestamp and column mapping fixture",
    "COMMENTS 2,Weather values are constant because only generated schedule values are compared",
    "DATA PERIODS,1,1,Data,Tuesday,1/1,1/1"
)
$weatherRows = @($weatherLines | Select-Object -Skip 8 | Where-Object { -not [string]::IsNullOrWhiteSpace($_) })
$weatherPayload = "?9?9?9?9E0?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9*9*9?9*9*9,10.0,5.0,50,80600,0,0,250,0,0,0,0,0,0,0,180,2.0,5,5,20.0,7777,9,999999999,0,0.0000,0,0,0.000,0.0,0.0"
if ($weatherLines.Count -ne 32 -or $weatherRows.Count -ne 24 -or
    (($weatherLines[0..7] -join '||') -cne ($expectedHeaders -join '||'))) {
    throw "Fixture EPW must retain eight exact headers and 24 hourly rows"
}
for ($index = 0; $index -lt 24; ++$index) {
    $expected = "2013,1,1,$($index + 1),60,$weatherPayload"
    if ($weatherRows[$index] -cne $expected) {
        throw "Unexpected EPW row at index $index"
    }
}
Write-Host "OK exact one-day common-year EPW"

if ($csvLines.Count -ne 35041 -or $csvLines[0] -cne "Date/Time,ALPHA SURFACE,BETA SURFACE") {
    throw "CSV must retain one exact header and exactly 35,040 data rows"
}
$culture = [System.Globalization.CultureInfo]::InvariantCulture
$startDate = [datetime]::new(2013, 1, 1)
for ($row = 1; $row -le 35040; ++$row) {
    $zeroBased = $row - 1
    $dayOffset = [int][Math]::Floor($zeroBased / 96)
    $sample = ($zeroBased % 96) + 1
    $endMinutes = $sample * 15
    $hour = [int][Math]::Floor($endMinutes / 60)
    $minute = $endMinutes % 60
    $monthDay = $startDate.AddDays($dayOffset).ToString("MM/dd", $culture)
    $timestamp = '{0} {1:00}:{2:00}' -f $monthDay, $hour, $minute
    $alpha = ($sample / 100.0).ToString("0.00", $culture)
    $beta = ((97 - $sample) / 100.0).ToString("0.00", $culture)
    if ($csvLines[$row] -cne "$timestamp,$alpha,$beta") {
        throw "Unexpected shading CSV row $row"
    }
}
Write-Host "OK exact header plus 35,040 source-ordered comma rows"

if (Test-Path -LiteralPath $CaseOutputRoot) {
    $fullOutput = [System.IO.Path]::GetFullPath($CaseOutputRoot)
    $fullRepo = [System.IO.Path]::GetFullPath($RepoRoot)
    if (-not $fullOutput.StartsWith($fullRepo + [System.IO.Path]::DirectorySeparatorChar, [System.StringComparison]::OrdinalIgnoreCase)) {
        throw "Refusing to remove output outside repository: $fullOutput"
    }
    Remove-Item -LiteralPath $CaseOutputRoot -Recurse -Force
}

$cargo = Get-Command cargo -ErrorAction Stop
$output = & $cargo.Source run -p ep_cli --quiet -- conformance time-weather-schedule-report $CasePath $OracleRoot $OutputRoot 2>&1
if ($LASTEXITCODE -ne 0) {
    throw "Schedule:File:Shading report failed: $($output -join [Environment]::NewLine)"
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
$stagedCsvPath = Join-Path $CaseOutputRoot "oracle\schedule_file_shading.csv"
foreach ($path in @(
    $summaryPath, $reportPath, $oracleEsoPath, $oracleEioPath, $oracleErrPath,
    $oracleEndPath, $stagedIdfPath, $convertedEpjsonPath, $stagedCsvPath
)) {
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Missing Schedule:File:Shading report artifact: $path"
    }
}
if ((Get-FileHash -LiteralPath $CsvPath -Algorithm SHA256).Hash -cne
    (Get-FileHash -LiteralPath $stagedCsvPath -Algorithm SHA256).Hash) {
    throw "Staged shading CSV must be byte-identical to the canonical auxiliary file"
}

$summary = Get-Content -LiteralPath $summaryPath -Raw -Encoding UTF8 | ConvertFrom-Json
if ($summary.case_id -cne $CaseId -or $summary.oracle_version -cne "26.1.0" -or
    $summary.comparison_class -cne "conformance" -or $summary.conformance_claim -ne $true -or
    $summary.status -cne "pass" -or $summary.series_count -ne 2 -or
    $summary.conformance_series_count -ne 2 -or $summary.time_axis_samples -ne 96 -or
    $summary.timestamp_rule -cne "zone-timestep ending samples aligned by EnergyPlus ESO timestamp labels" -or
    $summary.gate.script -cne $GateCommand -or $summary.gate.blocking -ne $true -or
    $null -ne $summary.weather_record_selection) {
    throw "Unexpected Schedule:File:Shading summary contract"
}
$firstTimestamp = "env=SCHEDULE FILE SHADING RUN PERIOD;day=1;month=1;date=1;dst=0;hour=1;start=0.00;end=15.00;day_type=Tuesday"
$lastTimestamp = "env=SCHEDULE FILE SHADING RUN PERIOD;day=1;month=1;date=1;dst=0;hour=24;start=45.00;end=60.00;day_type=Tuesday"
$expectedKeys = @("ALPHA SURFACE_SHADING", "BETA SURFACE_SHADING")
$seriesRows = @($summary.series)
if ($seriesRows.Count -ne 2) {
    throw "Expected exactly two generated-schedule summary rows"
}
for ($index = 0; $index -lt 2; ++$index) {
    $series = $seriesRows[$index]
    if ($series.key -cne $expectedKeys[$index] -or $series.variable -cne "Schedule Value" -or
        $series.level -cne "conformance" -or $series.class -cne "schedule" -or
        $series.frequency -cne "timestep" -or $series.source -cne "eso" -or
        $series.alignment -cne "timestamp" -or $series.expected_samples -ne 96 -or
        $series.observed_samples -ne 96 -or $series.compared_samples -ne 96 -or
        $series.timestamp_status -cne "pass" -or $series.timestamp_order_match -ne $true -or
        $series.expected_first_timestamp -cne $firstTimestamp -or
        $series.observed_first_timestamp -cne $firstTimestamp -or
        $series.expected_last_timestamp -cne $lastTimestamp -or
        $series.observed_last_timestamp -cne $lastTimestamp -or
        $series.max_abs_delta -ne 0.0 -or $series.rmse_delta -ne 0.0 -or
        $null -ne $series.first_divergence -or $series.status -cne "pass") {
        throw "Unexpected exact summary series contract for $($expectedKeys[$index])"
    }
}
Write-Host "OK two exact zero-delta generated schedule series"

$converted = Get-Content -LiteralPath $convertedEpjsonPath -Raw -Encoding UTF8 | ConvertFrom-Json
$shadingFamily = $converted."Schedule:File:Shading"
$convertedShading = $shadingFamily."Schedule:File:Shading 1"
$convertedOutputs = @($converted."Output:Variable".PSObject.Properties | ForEach-Object { $_.Value })
if (@($shadingFamily.PSObject.Properties).Count -ne 1 -or
    $convertedShading.file_name -cne "schedule_file_shading.csv" -or
    $convertedOutputs.Count -ne 2 -or
    $convertedOutputs[0].key_value -cne "ALPHA SURFACE_SHADING" -or
    $convertedOutputs[1].key_value -cne "BETA SURFACE_SHADING" -or
    @($converted.PSObject.Properties | Where-Object { $_.Name -ceq "ShadowCalculation" }).Count -ne 0) {
    throw "Converted epJSON changed shading file-name/output ownership or added ShadowCalculation"
}
Write-Host "OK converted epJSON exact Schedule:File:Shading shape"

$esoLines = @(Get-Content -LiteralPath $oracleEsoPath -Encoding UTF8)
$dictionaries = @($esoLines | Where-Object { $_ -match '^[78],1,.*Schedule Value \[\] !TimeStep$' })
$expectedDictionaries = @(
    "7,1,ALPHA SURFACE_shading,Schedule Value [] !TimeStep",
    "8,1,BETA SURFACE_shading,Schedule Value [] !TimeStep"
)
if (($dictionaries -join '||') -cne ($expectedDictionaries -join '||')) {
    throw "Unexpected generated-schedule ESO dictionary rows"
}
$alphaValues = @($esoLines | Where-Object { $_ -match '^7,\s*[-+0-9.E]+\s*$' } | ForEach-Object { [double](($_ -split ',', 2)[1]) })
$betaValues = @($esoLines | Where-Object { $_ -match '^8,\s*[-+0-9.E]+\s*$' } | ForEach-Object { [double](($_ -split ',', 2)[1]) })
$timestamps = @($esoLines | Where-Object { $_ -match '^2,\d+,\s*\d+,\s*\d+,\s*\d+,\s*\d+,' })
if ($alphaValues.Count -ne 96 -or $betaValues.Count -ne 96 -or $timestamps.Count -ne 96) {
    throw "Expected two 96-value series and 96 raw timestep rows"
}
for ($index = 0; $index -lt 96; ++$index) {
    if ($alphaValues[$index] -ne (($index + 1) / 100.0) -or
        $betaValues[$index] -ne ((96 - $index) / 100.0)) {
        throw "Unexpected raw values at sample $index"
    }
    $hour = [int][Math]::Floor($index / 4) + 1
    $quarter = $index % 4
    $timestampMatch = [regex]::Match(
        $timestamps[$index],
        '^2,\s*1,\s*1,\s*1,\s*0,\s*(\d+),\s*([-+0-9.]+),\s*([-+0-9.]+),Tuesday$'
    )
    if (-not $timestampMatch.Success -or
        [int]$timestampMatch.Groups[1].Value -ne $hour -or
        [double]$timestampMatch.Groups[2].Value -ne ($quarter * 15) -or
        [double]$timestampMatch.Groups[3].Value -ne (($quarter + 1) * 15)) {
        throw "Unexpected raw timestamp at sample $index"
    }
}
Write-Host "OK exact raw ESO column mapping and timestep order"

$eioLines = @(Get-Content -LiteralPath $oracleEioPath -Encoding UTF8)
$environmentRow = "Environment,SCHEDULE FILE SHADING RUN PERIOD,WeatherFileRunPeriod,01/01/2013,01/01/2013,Tuesday,1,Use RunPeriod Specified Day,No,No,No,No,No,Clark and Allen"
$daylightRow = "Environment:Daylight Saving,No,RunPeriod Object"
$shadowingRow = "Shadowing/Sun Position Calculations Annual Simulations,PolygonClipping,Periodic,20,15000,SutherlandHodgman,512,SimpleSkyDiffuseModeling,No,No,No"
if (@($eioLines | Where-Object { $_ -ceq $environmentRow }).Count -ne 1 -or
    @($eioLines | Where-Object { $_ -ceq $daylightRow }).Count -ne 1 -or
    @($eioLines | Where-Object { $_ -ceq $shadowingRow }).Count -ne 1 -or
    @($eioLines | Where-Object { $_ -match '^Shadowing/Sun Position Calculations Annual Simulations,Imported,' }).Count -ne 0) {
    throw "Unexpected exact EIO environment or non-Imported boundary"
}
$completion = "EnergyPlus Completed Successfully-- 0 Warning; 0 Severe Errors;"
$errText = Get-Content -LiteralPath $oracleErrPath -Raw -Encoding UTF8
$endText = Get-Content -LiteralPath $oracleEndPath -Raw -Encoding UTF8
Assert-Contains $errText $completion "clean EnergyPlus ERR completion"
Assert-Contains $endText $completion "clean EnergyPlus END completion"

$reportText = Get-Content -LiteralPath $reportPath -Raw -Encoding UTF8
foreach ($contract in @(
    "status: pass",
    "series: 2",
    "conformance_series: 2",
    "time_axis_samples: 96",
    "| ALPHA SURFACE_SHADING | Schedule Value | conformance | schedule | timestep | eso | timestamp | 96 | 96 | 96 | 0.000000000000 | 0.000000000000 | 0.000000000000 |",
    "| BETA SURFACE_SHADING | Schedule Value | conformance | schedule | timestep | eso | timestamp | 96 | 96 | 96 | 0.000000000000 | 0.000000000000 | 0.000000000000 |"
)) {
    Assert-Contains $reportText $contract "markdown exact contract"
}

Write-Host "Schedule:File:Shading exact CSV mapping gate passed."
