[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$CaseId = "calendar_schedule_fmu_import_initial_value_exact_001"
$CaseRoot = Join-Path $RepoRoot "data\conformance_cases\$CaseId"
$CasePath = Join-Path $CaseRoot "case.toml"
$IdfPath = Join-Path $CaseRoot "calendar_schedule_fmu_import_initial_value_exact.idf"
$WeatherPath = Join-Path $CaseRoot "calendar_schedule_fmu_import_initial_value_exact.epw"
$GateCommand = "scripts/dev.cmd compare-calendar-schedule-fmu-import-initial-value-exact"
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
        throw "Missing required ExternalInterface:FunctionalMockupUnitImport:To:Schedule exact-case file: $path"
    }
}

$caseText = Get-Content -LiteralPath $CasePath -Raw -Encoding UTF8
$idfText = Get-Content -LiteralPath $IdfPath -Raw -Encoding UTF8
$weatherLines = @(Get-Content -LiteralPath $WeatherPath -Encoding UTF8)
$weatherRows = @($weatherLines | Select-Object -Skip 8 | Where-Object { -not [string]::IsNullOrWhiteSpace($_) })

foreach ($contract in @(
    'comparison_class = "conformance"',
    'conformance_claim = true',
    'frequency = "timestep"',
    'timestamp_contract = "ordered-exact-unique"',
    'abs_tol = 0.0',
    'rmse_tol = 0.0',
    'exactly 96 ordered, unique Timestep Schedule Value samples and timestamps, each equal to 0.625',
    'No ExternalInterface activation object or FunctionalMockupUnitImport provider object is present',
    'FMU provider resolution',
    'object-list validation or diagnostics',
    'FMU file opening/unpacking and modelDescription parsing',
    'FMI 1 import binding',
    'From:Variable instances',
    'value-reference or causality checks',
    'live exchange and warmup updates',
    'activated-without-provider permissive EnergyPlus no-op branch',
    'bounded ScheduleTypeLimits validation',
    'downstream consumers',
    'multiple objects',
    'broad diagnostic parity',
    'script = "scripts/dev.cmd compare-calendar-schedule-fmu-import-initial-value-exact"',
    'blocking = true'
)) {
    Assert-Contains -Text $caseText -Pattern $contract -Description "canonical manifest contract"
}
if (@([regex]::Matches($caseText, '(?m)^\[\[outputs\]\]$')).Count -ne 1) {
    throw "Manifest must retain exactly one promoted output"
}

$actualVectors = @(Get-CompleteIdfObjectVectors -Text $idfText -Description "canonical fixture")
$expectedVectors = @(
    "Version|26.1",
    "Building|FMU Import Schedule Initial Value Exact Fixture|0.0|Suburbs|0.04|0.4|FullExterior|25|6",
    "Timestep|4",
    "GlobalGeometryRules|UpperLeftCorner|CounterClockWise|World",
    "RunPeriod|FMU Import Schedule Initial Value Run Period|1|1|2032|1|1|2032|Thursday|No|No|No|No|No|No",
    "ScheduleTypeLimits|Any Number",
    "ExternalInterface:FunctionalMockupUnitImport:To:Schedule|FMU Import Initial Value|Any Number|missing.fmu|ProbeInstance|ProbeOutput|0.625",
    "Output:Variable|FMU IMPORT INITIAL VALUE|Schedule Value|Timestep"
)
if (($actualVectors -join '||') -cne ($expectedVectors -join '||')) {
    throw "Fixture must retain the exact complete IDF object order and fields"
}
$mutated = $idfText.Replace(
    "Version,26.1;",
    "Version,26.1; ExternalInterface,FunctionalMockupUnitImport;"
)
$mutatedVectors = @(Get-CompleteIdfObjectVectors -Text $mutated -Description "same-line activation mutation")
if ($mutatedVectors.Count -ne ($expectedVectors.Count + 1) -or
    $mutatedVectors[1] -cne "ExternalInterface|FunctionalMockupUnitImport") {
    throw "Complete IDF parser self-check did not expose the same-line activation object"
}
Write-Host "OK exact complete fixture IDF vectors and no-activation parser self-check"

$expectedHeaders = @(
    "LOCATION,FMU Import Schedule Initial Value Exact Fixture,CO,USA,Synthetic,999999,39.74,-105.18,-7.0,1829.0",
    "DESIGN CONDITIONS,0",
    "TYPICAL/EXTREME PERIODS,0",
    "GROUND TEMPERATURES,0",
    "HOLIDAYS/DAYLIGHT SAVINGS,No,0,0,0",
    "COMMENTS 1,Deterministic one-day FMU Import To Schedule inactive initial-value fixture",
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
Write-Host "OK exact one-day EPW without holidays or daylight saving"

Remove-RepoDirectory -Path $CaseOutputRoot
$cargo = Get-Command cargo -ErrorAction Stop
$output = & $cargo.Source run -p ep_cli --quiet -- conformance time-weather-schedule-report $CasePath $OracleRoot $OutputRoot 2>&1
if ($LASTEXITCODE -ne 0) {
    throw "ExternalInterface:FunctionalMockupUnitImport:To:Schedule report failed with exit code $LASTEXITCODE $($output -join [Environment]::NewLine)"
}
$output | ForEach-Object { Write-Host $_ }

$summaryPath = Join-Path $CaseOutputRoot "compare\compare-summary.json"
$reportPath = Join-Path $CaseOutputRoot "compare\compare-report.md"
$oracleEsoPath = Join-Path $CaseOutputRoot "oracle\eplusout.eso"
$oracleEioPath = Join-Path $CaseOutputRoot "oracle\eplusout.eio"
$oracleErrPath = Join-Path $CaseOutputRoot "oracle\eplusout.err"
$oracleEndPath = Join-Path $CaseOutputRoot "oracle\eplusout.end"
$convertedEpjsonPath = Join-Path $CaseOutputRoot "oracle\input.epJSON"
foreach ($path in @(
    $summaryPath,
    $reportPath,
    $oracleEsoPath,
    $oracleEioPath,
    $oracleErrPath,
    $oracleEndPath,
    $convertedEpjsonPath
)) {
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Missing ExternalInterface:FunctionalMockupUnitImport:To:Schedule report artifact: $path"
    }
}

$summary = Get-Content -LiteralPath $summaryPath -Raw -Encoding UTF8 | ConvertFrom-Json
if ($summary.case_id -cne $CaseId -or $summary.oracle_version -cne "26.1.0" -or
    $summary.comparison_class -cne "conformance" -or $summary.conformance_claim -ne $true -or
    $summary.status -cne "pass" -or $summary.series_count -ne 1 -or
    $summary.conformance_series_count -ne 1 -or $summary.time_axis_samples -ne 96 -or
    $summary.timestamp_rule -cne "zone-timestep ending samples aligned by EnergyPlus ESO timestamp labels" -or
    $summary.gate.script -cne $GateCommand -or $summary.gate.blocking -ne $true -or
    $null -ne $summary.weather_record_selection) {
    throw "Unexpected ExternalInterface:FunctionalMockupUnitImport:To:Schedule report summary contract"
}
$seriesRows = @($summary.series | Where-Object {
    $_.key -eq "FMU IMPORT INITIAL VALUE" -and $_.variable -eq "Schedule Value"
})
if ($seriesRows.Count -ne 1) {
    throw "Expected exactly one FMU import initial-value Schedule Value series"
}
$series = $seriesRows[0]
$firstTimestamp = "env=FMU IMPORT SCHEDULE INITIAL VALUE RUN PERIOD;day=1;month=1;date=1;dst=0;hour=1;start=0.00;end=15.00;day_type=Thursday"
$lastTimestamp = "env=FMU IMPORT SCHEDULE INITIAL VALUE RUN PERIOD;day=1;month=1;date=1;dst=0;hour=24;start=45.00;end=60.00;day_type=Thursday"
if ($series.level -cne "conformance" -or $series.class -cne "schedule" -or
    $series.frequency -cne "timestep" -or $series.source -cne "eso" -or
    $series.alignment -cne "timestamp" -or $series.expected_samples -ne 96 -or
    $series.observed_samples -ne 96 -or $series.compared_samples -ne 96 -or
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
    throw "FMU import initial-value samples and timestamps must match exactly at zero tolerance"
}
Write-Host "OK one exact zero-delta initial-value schedule series"

$converted = Get-Content -LiteralPath $convertedEpjsonPath -Raw -Encoding UTF8 | ConvertFrom-Json
$scheduleFamily = $converted."ExternalInterface:FunctionalMockupUnitImport:To:Schedule"
$convertedSchedule = $scheduleFamily."FMU Import Initial Value"
$scheduleTypeFamily = $converted."ScheduleTypeLimits"
if (@($scheduleFamily.PSObject.Properties).Count -ne 1 -or
    @($convertedSchedule.PSObject.Properties).Count -ne 5 -or
    $convertedSchedule.schedule_type_limits_names -cne "Any Number" -or
    $convertedSchedule.fmu_file_name -cne "missing.fmu" -or
    $convertedSchedule.fmu_instance_name -cne "ProbeInstance" -or
    $convertedSchedule.fmu_variable_name -cne "ProbeOutput" -or
    [double]$convertedSchedule.initial_value -ne 0.625 -or
    @($scheduleTypeFamily.PSObject.Properties).Count -ne 1 -or
    $null -eq $scheduleTypeFamily."Any Number" -or
    @($converted.PSObject.Properties | Where-Object {
        $_.Name -ceq "ExternalInterface" -or
        $_.Name -ceq "ExternalInterface:FunctionalMockupUnitImport"
    }).Count -ne 0) {
    throw "Converted epJSON changed the exact FMU import schedule fields or added activation/provider families"
}
Write-Host "OK converted epJSON exact inactive ExternalInterface:FunctionalMockupUnitImport:To:Schedule shape"

$esoLines = @(Get-Content -LiteralPath $oracleEsoPath -Encoding UTF8)
$dictionaryRows = @($esoLines | Where-Object {
    $_ -ceq "7,1,FMU IMPORT INITIAL VALUE,Schedule Value [] !TimeStep"
})
$values = @($esoLines | Where-Object { $_ -match '^7,\s*[-+0-9.E]+\s*$' } |
    ForEach-Object { [double](($_ -split ',', 2)[1].Trim()) })
$timestamps = @($esoLines | Where-Object { $_ -match '^2,\d+,\s*\d+,\s*\d+,\s*\d+,\s*\d+,' })
if ($dictionaryRows.Count -ne 1 -or $values.Count -ne 96 -or $timestamps.Count -ne 96) {
    throw "Expected one exact TimeStep dictionary and 96 raw values/timestamps"
}
for ($index = 0; $index -lt 96; ++$index) {
    if ($values[$index] -ne 0.625) {
        throw "Unexpected raw FMU import initial value at sample $($index): $($values[$index])"
    }
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
        throw "Unexpected raw FMU-import-schedule timestamp at sample $($index): $($timestamps[$index])"
    }
}
Write-Host "OK exact raw ESO initial values and timestep timestamps"

$eioLines = @(Get-Content -LiteralPath $oracleEioPath -Encoding UTF8)
$environmentRow = "Environment,FMU IMPORT SCHEDULE INITIAL VALUE RUN PERIOD,WeatherFileRunPeriod,01/01/2032,01/01/2032,Thursday,1,Use RunPeriod Specified Day,No,No,No,No,No,Clark and Allen"
$daylightRow = "Environment:Daylight Saving,No,RunPeriod Object"
if (@($eioLines | Where-Object { $_ -ceq $environmentRow }).Count -ne 1 -or
    @($eioLines | Where-Object { $_ -ceq $daylightRow }).Count -ne 1) {
    throw "Unexpected exact Environment or disabled daylight-saving EIO row"
}

$errText = Get-Content -LiteralPath $oracleErrPath -Raw -Encoding UTF8
$endText = Get-Content -LiteralPath $oracleEndPath -Raw -Encoding UTF8
$warningHeader = '   ** Warning ** IDF file contains object "ExternalInterface:FunctionalMockupUnitImport:To:Schedule",'
$warningContinuation = '   **   ~~~   ** but object "ExternalInterface" with appropriate key entry is not specified. Values will not be updated.'
if ([regex]::Matches($errText, [regex]::Escape($warningHeader)).Count -ne 1 -or
    [regex]::Matches($errText, [regex]::Escape($warningContinuation)).Count -ne 1 -or
    [regex]::Matches($errText, '(?m)^   \*\* Warning \*\*').Count -ne 1) {
    throw "Expected the exact missing ExternalInterface activation warning exactly once"
}
$completion = "EnergyPlus Completed Successfully-- 1 Warning; 0 Severe Errors;"
Assert-Contains -Text $errText -Pattern $completion -Description "one-warning EnergyPlus error-file completion"
Assert-Contains -Text $endText -Pattern $completion -Description "one-warning EnergyPlus end record"

$reportText = Get-Content -LiteralPath $reportPath -Raw -Encoding UTF8
foreach ($reportContract in @(
    "status: pass",
    "series: 1",
    "conformance_series: 1",
    "time_axis_samples: 96",
    "timestamp_rule: zone-timestep ending samples aligned by EnergyPlus ESO timestamp labels",
    "weather_record_selection_applied: false",
    "| FMU IMPORT INITIAL VALUE | Schedule Value | conformance | schedule | timestep | eso | timestamp | 96 | 96 | 96 | 0.000000000000 | 0.000000000000 | 0.000000000000 |"
)) {
    Assert-Contains -Text $reportText -Pattern $reportContract -Description "markdown FMU import initial-value contract"
}

Write-Host "ExternalInterface:FunctionalMockupUnitImport:To:Schedule inactive initial-value exact gate passed."
