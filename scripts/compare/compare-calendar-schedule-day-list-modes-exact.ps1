[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$CaseId = "calendar_schedule_day_list_modes_exact_001"
$CaseRoot = Join-Path $RepoRoot "data\conformance_cases\$CaseId"
$CasePath = Join-Path $CaseRoot "case.toml"
$IdfPath = Join-Path $CaseRoot "calendar_schedule_day_list_modes_exact.idf"
$WeatherPath = Join-Path $CaseRoot "calendar_schedule_day_list_modes_exact.epw"
$GateCommand = "scripts/dev.cmd compare-calendar-schedule-day-list-modes-exact"
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
        throw "Missing required Schedule:Day:List modes file: $path"
    }
}

$caseText = Get-Content -LiteralPath $CasePath -Raw -Encoding UTF8
$idfText = Get-Content -LiteralPath $IdfPath -Raw -Encoding UTF8
$weatherLines = @(Get-Content -LiteralPath $WeatherPath -Encoding UTF8)
$weatherRows = @($weatherLines | Select-Object -Skip 8 | Where-Object { -not [string]::IsNullOrWhiteSpace($_) })

foreach ($contract in @(
    'comparison_class = "conformance"',
    'conformance_claim = true',
    'source_file = "data/conformance_cases/calendar_schedule_day_list_modes_exact_001/calendar_schedule_day_list_modes_exact.idf"',
    'idf = "data/conformance_cases/calendar_schedule_day_list_modes_exact_001/calendar_schedule_day_list_modes_exact.idf"',
    'weather = "data/conformance_cases/calendar_schedule_day_list_modes_exact_001/calendar_schedule_day_list_modes_exact.epw"',
    'frequency = "timestep"',
    'timestamp_contract = "ordered-exact-unique"',
    'abs_tol = 0.0',
    'rmse_tol = 0.0',
    'Minutes per Item 20 and exactly 72 source-ordered values',
    'genuinely blank Interpolate to Timestep field',
    'Thursday default-No is [10, 70, 160, 160] then 92 values of 175',
    'Friday Average is [10, 50, 100, 160] then 92 values of 175',
    'Saturday EnergyPlus 26.1 source-actual Linear is [10, 70, 160, 160] then 92 values of 175',
    'Intended or documented Linear ramp semantics are not claimed',
    'exactly 0 Warning and 0 Severe errors',
    'Schedule:Week:Compact',
    'other Minutes per Item values',
    'other timestep counts',
    'Hourly aggregation',
    'broad input diagnostic parity',
    'Daylight-saving',
    'holidays',
    'today/tomorrow rollover',
    'downstream schedule consumption',
    'UpdateScheduleVals orchestration',
    'EMS override behavior',
    'currentVal state',
    'actual-weather',
    'warmup',
    'multiple environments',
    'Rust raw ESO serialization',
    'Rust EPW record selection',
    'broad EnergyPlus warning/error parity',
    'script = "scripts/dev.cmd compare-calendar-schedule-day-list-modes-exact"',
    'blocking = true'
)) {
    Assert-Contains -Text $caseText -Pattern $contract -Description "canonical manifest contract"
}
if (@([regex]::Matches($caseText, '(?m)^\[\[outputs\]\]$')).Count -ne 1) {
    throw "Manifest must retain exactly one output request"
}

$defaultDay = "List Default No Day"
$averageDay = "List Average Day"
$linearDay = "List Linear Day"
$sourceValues = @("10", "70", "160") + @(for ($index = 0; $index -lt 69; ++$index) { "175" })
if ($sourceValues.Count -ne 72) {
    throw "Gate self-check must construct exactly 72 source values"
}
$sourceVector = $sourceValues -join "|"
$expectedVectors = @(
    "Version|26.1",
    "Building|Schedule Day List Modes Exact Fixture|0.0|Suburbs|0.04|0.4|FullExterior|25|6",
    "Timestep|4",
    "GlobalGeometryRules|UpperLeftCorner|CounterClockWise|World",
    "RunPeriod|Schedule Day List Modes Run Period|1|1|2032|1|3|2032|Thursday|No|No|No|No|No|No",
    "ScheduleTypeLimits|Any Number",
    "Schedule:Day:List|$defaultDay|Any Number||20|$sourceVector",
    "Schedule:Day:List|$averageDay|Any Number|Average|20|$sourceVector",
    "Schedule:Day:List|$linearDay|Any Number|Linear|20|$sourceVector",
    "Schedule:Week:Daily|List Modes Week|$defaultDay|$defaultDay|$defaultDay|$defaultDay|$defaultDay|$averageDay|$linearDay|$defaultDay|$defaultDay|$defaultDay|$defaultDay|$defaultDay",
    "Schedule:Year|Day List Modes Schedule|Any Number|List Modes Week|1|1|12|31",
    "Output:Variable|DAY LIST MODES SCHEDULE|Schedule Value|Timestep"
)
$actualVectors = @(Get-CompleteIdfObjectVectors -Text $idfText -Description "canonical fixture")
if (($actualVectors -join '||') -cne ($expectedVectors -join '||')) {
    throw "Fixture must retain the exact complete IDF object order, interpolation declarations, Minutes per Item, and all source values"
}
$actualDayLists = @($actualVectors | Where-Object { $_.StartsWith("Schedule:Day:List|", [System.StringComparison]::Ordinal) })
if ($actualDayLists.Count -ne 3) {
    throw "Fixture must retain exactly three Schedule:Day:List objects"
}
foreach ($vector in $actualDayLists) {
    $fields = @($vector -split '\|')
    if ($fields.Count -ne 77 -or (($fields[5..76] -join '|') -cne $sourceVector)) {
        throw "Every Schedule:Day:List object must retain exactly 72 source-ordered values"
    }
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
Write-Host "OK complete Schedule:Day:List fixture vectors, three interpolation declarations, 72-value counts, source order, and parser self-check"

$expectedHeaders = @(
    "LOCATION,Schedule Day List Modes Exact Fixture,CO,USA,Synthetic,999999,39.74,-105.18,-7.0,1829.0",
    "DESIGN CONDITIONS,0",
    "TYPICAL/EXTREME PERIODS,0",
    "GROUND TEMPERATURES,0",
    "HOLIDAYS/DAYLIGHT SAVINGS,No,0,0,0",
    "COMMENTS 1,Deterministic three-day Schedule Day List interpolation modes fixture",
    "COMMENTS 2,Weather values are constant because only Schedule Value is compared",
    "DATA PERIODS,1,1,Data,Thursday,1/1,1/3"
)
if ($weatherLines.Count -ne 80 -or $weatherRows.Count -ne 72 -or
    (($weatherLines[0..7] -join '||') -cne ($expectedHeaders -join '||'))) {
    throw "Fixture EPW must retain eight exact headers and 72 hourly rows"
}
$weatherPayload = "?9?9?9?9E0?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9*9*9?9*9*9,10.0,5.0,50,80600,0,0,250,0,0,0,0,0,0,0,180,2.0,5,5,20.0,7777,9,999999999,0,0.0000,0,0,0.000,0.0,0.0"
for ($index = 0; $index -lt 72; ++$index) {
    $day = [int][Math]::Floor($index / 24) + 1
    $hour = ($index % 24) + 1
    $expected = "2032,1,$day,$hour,60,$weatherPayload"
    if ($weatherRows[$index] -cne $expected) {
        throw "Unexpected EPW row at index $($index): $($weatherRows[$index])"
    }
}
Write-Host "OK exact 72-hour Thursday-through-Saturday EPW"

Remove-RepoDirectory -Path $CaseOutputRoot
$cargo = Get-Command cargo -ErrorAction Stop
$output = & $cargo.Source run -p ep_cli --quiet -- conformance time-weather-schedule-report $CasePath $OracleRoot $OutputRoot 2>&1
if ($LASTEXITCODE -ne 0) {
    $joinedOutput = $output -join [Environment]::NewLine
    throw "Schedule:Day:List modes report failed with exit code $LASTEXITCODE $joinedOutput"
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
foreach ($path in @(
    $summaryPath,
    $reportPath,
    $oracleEsoPath,
    $oracleEioPath,
    $oracleErrPath,
    $oracleEndPath,
    $stagedIdfPath,
    $convertedEpjsonPath
)) {
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Missing Schedule:Day:List modes report artifact: $path"
    }
}

$summary = Get-Content -LiteralPath $summaryPath -Raw -Encoding UTF8 | ConvertFrom-Json
if ($summary.case_id -cne $CaseId -or $summary.oracle_version -cne "26.1.0" -or
    $summary.comparison_class -cne "conformance" -or $summary.conformance_claim -ne $true -or
    $summary.status -cne "pass" -or $summary.series_count -ne 1 -or
    $summary.conformance_series_count -ne 1 -or $summary.time_axis_samples -ne 288 -or
    $summary.timestamp_rule -cne "zone-timestep ending samples aligned by EnergyPlus ESO timestamp labels" -or
    $summary.gate.script -cne $GateCommand -or $summary.gate.blocking -ne $true) {
    throw "Unexpected Schedule:Day:List modes report summary contract"
}
if ($null -ne $summary.weather_record_selection) {
    throw "Schedule-only comparison must retain null Rust EPW record selection"
}

$seriesRows = @($summary.series | Where-Object {
    $_.key -eq "DAY LIST MODES SCHEDULE" -and $_.variable -eq "Schedule Value"
})
if ($seriesRows.Count -ne 1) {
    throw "Expected exactly one Schedule:Day:List Schedule Value series"
}
$series = $seriesRows[0]
$firstTimestamp = "env=SCHEDULE DAY LIST MODES RUN PERIOD;day=1;month=1;date=1;dst=0;hour=1;start=0.00;end=15.00;day_type=Thursday"
$lastTimestamp = "env=SCHEDULE DAY LIST MODES RUN PERIOD;day=3;month=1;date=3;dst=0;hour=24;start=45.00;end=60.00;day_type=Saturday"
if ($series.level -cne "conformance" -or $series.class -cne "schedule" -or
    $series.frequency -cne "timestep" -or $series.source -cne "eso" -or
    $series.alignment -cne "timestamp" -or $series.expected_samples -ne 288 -or
    $series.observed_samples -ne 288 -or $series.compared_samples -ne 288 -or
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
    throw "Schedule:Day:List values and exact first/last timestamps must match at zero delta"
}
Write-Host "OK JSON single 288-sample series with exact timestamps and zero delta"

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
$dayListFamily = $converted."Schedule:Day:List"
$convertedDefault = $dayListFamily.$defaultDay
$convertedAverage = $dayListFamily.$averageDay
$convertedLinear = $dayListFamily.$linearDay
if (@($dayListFamily.PSObject.Properties).Count -ne 3 -or
    @($convertedDefault.PSObject.Properties | Where-Object { $_.Name -ceq "interpolate_to_timestep" }).Count -ne 0 -or
    $convertedAverage.interpolate_to_timestep -cne "Average" -or
    $convertedLinear.interpolate_to_timestep -cne "Linear" -or
    $convertedDefault.minutes_per_item -ne 20 -or
    $convertedAverage.minutes_per_item -ne 20 -or
    $convertedLinear.minutes_per_item -ne 20) {
    throw "Converted epJSON must omit the default-No key, retain explicit Average and Linear keys, and retain Minutes per Item 20"
}
$expectedExtensionValues = @($sourceValues | ForEach-Object { [double]$_ })
$defaultData = @($convertedDefault.extensions | ForEach-Object { [double]$_.value })
$averageData = @($convertedAverage.extensions | ForEach-Object { [double]$_.value })
$linearData = @($convertedLinear.extensions | ForEach-Object { [double]$_.value })
if ($defaultData.Count -ne 72 -or $averageData.Count -ne 72 -or $linearData.Count -ne 72 -or
    (($defaultData -join '|') -cne ($expectedExtensionValues -join '|')) -or
    (($averageData -join '|') -cne ($expectedExtensionValues -join '|')) -or
    (($linearData -join '|') -cne ($expectedExtensionValues -join '|'))) {
    throw "Converted epJSON must retain all 72 exact source-ordered values for every day profile"
}
Write-Host "OK staged IDF and converted epJSON blank/default ownership, interpolation declarations, Minutes per Item, and 72-value source order"

$esoLines = @(Get-Content -LiteralPath $oracleEsoPath -Encoding UTF8)
$dictionaryRows = @($esoLines | Where-Object {
    $_ -ceq "7,1,DAY LIST MODES SCHEDULE,Schedule Value [] !TimeStep"
})
$values = @($esoLines | Where-Object { $_ -match '^7,\s*[-+0-9.E]+\s*$' } |
    ForEach-Object { [double](($_ -split ',', 2)[1].Trim()) })
$timestamps = @($esoLines | Where-Object { $_ -match '^2,\d+,\s*\d+,\s*\d+,\s*\d+,\s*\d+,' })
if ($dictionaryRows.Count -ne 1 -or $values.Count -ne 288 -or $timestamps.Count -ne 288) {
    throw "Expected one exact Timestep dictionary and 288 raw values/timestamps"
}
$dayTypes = @("Thursday", "Friday", "Saturday")
for ($index = 0; $index -lt 288; ++$index) {
    $dayIndex = [int][Math]::Floor($index / 96)
    $daySample = $index % 96
    $expectedValue = switch ($dayIndex) {
        0 {
            if ($daySample -eq 0) { 10.0 }
            elseif ($daySample -eq 1) { 70.0 }
            elseif ($daySample -eq 2 -or $daySample -eq 3) { 160.0 }
            else { 175.0 }
        }
        1 {
            if ($daySample -eq 0) { 10.0 }
            elseif ($daySample -eq 1) { 50.0 }
            elseif ($daySample -eq 2) { 100.0 }
            elseif ($daySample -eq 3) { 160.0 }
            else { 175.0 }
        }
        2 {
            if ($daySample -eq 0) { 10.0 }
            elseif ($daySample -eq 1) { 70.0 }
            elseif ($daySample -eq 2 -or $daySample -eq 3) { 160.0 }
            else { 175.0 }
        }
    }
    if ($values[$index] -ne $expectedValue) {
        throw "Unexpected raw Schedule:Day:List value at sample $($index): $($values[$index])"
    }

    $hour = [int][Math]::Floor($daySample / 4) + 1
    $zoneTimestep = $daySample % 4
    $startMinute = $zoneTimestep * 15
    $endMinute = ($zoneTimestep + 1) * 15
    $timestampMatch = [regex]::Match(
        $timestamps[$index],
        '^2,\s*(\d+),\s*1,\s*(\d+),\s*0,\s*(\d+),\s*([-+0-9.]+),\s*([-+0-9.]+),(Thursday|Friday|Saturday)$'
    )
    if (-not $timestampMatch.Success -or
        [int]$timestampMatch.Groups[1].Value -ne ($dayIndex + 1) -or
        [int]$timestampMatch.Groups[2].Value -ne ($dayIndex + 1) -or
        [int]$timestampMatch.Groups[3].Value -ne $hour -or
        [double]$timestampMatch.Groups[4].Value -ne $startMinute -or
        [double]$timestampMatch.Groups[5].Value -ne $endMinute -or
        $timestampMatch.Groups[6].Value -cne $dayTypes[$dayIndex]) {
        throw "Unexpected raw zone-timestep timestamp at sample $($index): $($timestamps[$index])"
    }
}
if (($values[0..95] -join '|') -cne ($values[192..287] -join '|')) {
    throw "EnergyPlus 26.1 source-actual Day:List Linear values must exactly equal default-No for this fixture"
}
Write-Host "OK exact raw ESO default-No, Average, version-26.1 source-actual Linear-equals-No values and 288 timestamps"

$eioLines = @(Get-Content -LiteralPath $oracleEioPath -Encoding UTF8)
$environmentRow = "Environment,SCHEDULE DAY LIST MODES RUN PERIOD,WeatherFileRunPeriod,01/01/2032,01/03/2032,Thursday,3,Use RunPeriod Specified Day,No,No,No,No,No,Clark and Allen"
$daylightRow = "Environment:Daylight Saving,No,RunPeriod Object"
if (@($eioLines | Where-Object { $_ -ceq $environmentRow }).Count -ne 1 -or
    @($eioLines | Where-Object { $_ -ceq $daylightRow }).Count -ne 1) {
    throw "Unexpected exact Environment or disabled daylight-saving EIO row"
}

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
    "time_axis_samples: 288",
    "timestamp_rule: zone-timestep ending samples aligned by EnergyPlus ESO timestamp labels",
    "weather_record_selection_applied: false",
    "| DAY LIST MODES SCHEDULE | Schedule Value | conformance | schedule | timestep | eso | timestamp | 288 | 288 | 288 | 0.000000000000 | 0.000000000000 | 0.000000000000 |"
)) {
    Assert-Contains -Text $reportText -Pattern $reportContract -Description "markdown Schedule:Day:List contract"
}

Write-Host "Schedule:Day:List interpolation-modes exact gate passed."
