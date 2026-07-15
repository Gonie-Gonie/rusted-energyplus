[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$CaseId = "calendar_epw_dst_cross_year_start_year_projection_hourly_exact_001"
$CaseRoot = Join-Path $RepoRoot "data\conformance_cases\$CaseId"
$CasePath = Join-Path $CaseRoot "case.toml"
$IdfPath = Join-Path $CaseRoot "calendar_epw_dst_cross_year_start_year_projection_hourly_exact.idf"
$WeatherPath = Join-Path $CaseRoot "calendar_epw_dst_cross_year_start_year_projection_hourly_exact.epw"
$IdfRef = "data/conformance_cases/$CaseId/calendar_epw_dst_cross_year_start_year_projection_hourly_exact.idf"
$WeatherRef = "data/conformance_cases/$CaseId/calendar_epw_dst_cross_year_start_year_projection_hourly_exact.epw"
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
        throw "Missing required cross-year daylight-saving file: $path"
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
    throw "Case manifest must bind manifest_v2.source_file and input.idf/weather to the canonical cross-year fixture"
}
Assert-SamePath -Actual (Resolve-RepoReference -Reference $manifestIdfRef -Description "manifest input.idf") -Expected $IdfPath -Description "manifest input.idf"
Assert-SamePath -Actual (Resolve-RepoReference -Reference $manifestWeatherRef -Description "manifest input.weather") -Expected $WeatherPath -Description "manifest input.weather"

Assert-Contains -Text $caseText -Pattern 'timestamp_contract = "ordered-exact-unique"' -Description "ordered cross-year timestamp contract"
Assert-Contains -Text $caseText -Pattern 'abs_tol = 0.0' -Description "zero absolute tolerance"
Assert-Contains -Text $caseText -Pattern 'rmse_tol = 0.0' -Description "zero RMSE tolerance"
Assert-Contains -Text $caseText -Pattern 'daily order 0, 0, 0, 1' -Description "narrow start-year daylight-saving claim"
Assert-Contains -Text $caseText -Pattern 'January 3 and later annual resets or reprojection' -Description "later annual reset and reprojection nonclaim"
Assert-Contains -Text $caseText -Pattern 'script = "scripts/dev.cmd compare-calendar-epw-dst-cross-year-start-year-projection-exact"' -Description "blocking gate attribution"
Assert-Contains -Text $idfText -Pattern "2031," -Description "explicit RunPeriod start year"
Assert-Contains -Text $idfText -Pattern "2032," -Description "explicit RunPeriod end year"
Assert-Contains -Text $idfText -Pattern "Yes,  !- Use Weather File Daylight Saving Period" -Description "enabled weather-file daylight-saving policy"
Assert-Contains -Text $idfText -Pattern "No;   !- Treat Weather as Actual" -Description "non-actual weather policy"
Assert-Contains -Text $weatherText -Pattern "HOLIDAYS/DAYLIGHT SAVINGS,Yes,1st Thursday in January,1st Friday in January,0" -Description "leap-observed EPW Nth-weekday daylight-saving range without holidays"
Assert-Contains -Text $weatherText -Pattern "DATA PERIODS,1,1,Data,Tuesday,12/30,1/2" -Description "single wrapping DATA PERIOD"

$runPeriodObjects = [regex]::Matches($idfText, '(?ims)^\s*RunPeriod\s*,(?<body>.*?);')
$specialDayObjects = [regex]::Matches($idfText, '(?ims)^\s*RunPeriodControl:SpecialDays\s*,(?<body>.*?);')
$inputDstObjects = [regex]::Matches($idfText, '(?ims)^\s*RunPeriodControl:DaylightSavingTime\s*,(?<body>.*?);')
$outputObjects = [regex]::Matches($idfText, '(?ims)^\s*Output:Variable\s*,(?<body>.*?);')
if ($runPeriodObjects.Count -ne 1 -or $specialDayObjects.Count -ne 0 -or
    $inputDstObjects.Count -ne 0 -or $outputObjects.Count -ne 1) {
    throw "Fixture must contain exactly one RunPeriod and one Output:Variable, with no input-file calendar-control objects"
}
$runPeriodBody = [regex]::Replace($runPeriodObjects[0].Groups["body"].Value, '(?m)!-.*$', '')
$runPeriodFields = @($runPeriodBody -split ',' | ForEach-Object { $_.Trim() })
$expectedRunPeriodFields = @(
    "Cross Year DST Start Year Run Period",
    "12", "30", "2031", "1", "2", "2032", "Tuesday",
    "No", "Yes", "No", "No", "No", "No"
)
if (($runPeriodFields -join '|') -cne ($expectedRunPeriodFields -join '|')) {
    throw "Fixture must retain the exact 2031-12-30 through 2032-01-02 Tuesday RunPeriod and isolated enabled EPW DST policy"
}
$outputBody = [regex]::Replace($outputObjects[0].Groups["body"].Value, '(?m)!-.*$', '')
$outputFields = @($outputBody -split ',' | ForEach-Object { $_.Trim() })
if (($outputFields -join '|') -cne "Environment|Site Daylight Saving Time Status|Hourly") {
    throw "Fixture must request exactly the hourly Environment Site Daylight Saving Time Status"
}

$expectedHeaders = @(
    "LOCATION,Calendar EPW DST Cross Year Start Year Projection Fixture,CO,USA,Synthetic,999999,39.74,-105.18,-7.0,1829.0",
    "DESIGN CONDITIONS,0",
    "TYPICAL/EXTREME PERIODS,0",
    "GROUND TEMPERATURES,0",
    "HOLIDAYS/DAYLIGHT SAVINGS,Yes,1st Thursday in January,1st Friday in January,0",
    "COMMENTS 1,Deterministic four-day cross-year weather fixture for start-year annual daylight-saving projection",
    "COMMENTS 2,Weather row years are source-only because Treat Weather as Actual is No",
    "DATA PERIODS,1,1,Data,Tuesday,12/30,1/2"
)
$nonblankWeatherLines = @($weatherLines | Where-Object { -not [string]::IsNullOrWhiteSpace($_) })
if ($weatherLines.Count -ne 104 -or $nonblankWeatherLines.Count -ne 104 -or
    (($weatherLines[0..7] -join [Environment]::NewLine) -cne ($expectedHeaders -join [Environment]::NewLine)) -or
    $weatherRows.Count -ne 96) {
    throw "Cross-year daylight-saving EPW must retain exactly eight locked headers and 96 nonblank hourly rows"
}
$expectedYears = @(2031, 2031, 2032, 2032)
$expectedMonths = @(12, 12, 1, 1)
$expectedDays = @(30, 31, 1, 2)
$expectedPayload = "?9?9?9?9E0?9?9?9?9?9?9?9?9?9?9?9?9?9?9?9*9*9?9*9*9,10.0,5.0,50,80600,0,0,250,0,0,0,0,0,0,0,180,2.0,5,5,20.0,7777,9,999999999,0,0.0000,0,0,0.000,0.0,0.0"
for ($index = 0; $index -lt 96; ++$index) {
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
Write-Host "Running cross-year start-year daylight-saving projection exact gate."
$output = & $cargo.Source run -p ep_cli --quiet -- conformance time-weather-schedule-report $CasePath $OracleRoot $OutputRoot 2>&1
if ($LASTEXITCODE -ne 0) {
    $output | ForEach-Object { Write-Host $_ }
    throw "Cross-year start-year daylight-saving projection exact gate failed."
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
        throw "Missing cross-year daylight-saving comparison artifact: $path"
    }
}

$expandedText = Get-Content -LiteralPath $expandedManifestPath -Raw -Encoding UTF8
$expandedInput = Get-TomlSectionBlock -Text $expandedText -Name "input" -Description "expanded manifest"
$expandedIdfRef = Get-TomlStringValue -Text $expandedInput -Name "source_idf" -Description "expanded manifest [input]"
$expandedWeatherRef = Get-TomlStringValue -Text $expandedInput -Name "source_weather" -Description "expanded manifest [input]"
Assert-SamePath -Actual (Resolve-RepoReference -Reference $expandedIdfRef -Description "expanded source_idf") -Expected $IdfPath -Description "expanded source_idf"
Assert-SamePath -Actual (Resolve-RepoReference -Reference $expandedWeatherRef -Description "expanded source_weather") -Expected $WeatherPath -Description "expanded source_weather"

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
    throw "Oracle staged IDF must equal the canonical cross-year EPW DST fixture plus the locked no-op output-injection footer"
}
Write-Host "OK oracle staged IDF canonical text and output-injection footer"

$summary = Get-Content -LiteralPath $summaryPath -Raw -Encoding UTF8 | ConvertFrom-Json
if ($summary.case_id -cne $CaseId -or $summary.oracle_version -cne "26.1.0" -or
    $summary.comparison_class -cne "conformance" -or $summary.conformance_claim -ne $true -or
    $summary.status -cne "pass" -or $summary.time_axis_samples -ne 96 -or
    $summary.series_count -ne 1 -or $summary.conformance_series_count -ne 1 -or
    $summary.gate.script -cne "scripts/dev.cmd compare-calendar-epw-dst-cross-year-start-year-projection-exact" -or
    $summary.gate.blocking -ne $true) {
    throw "Cross-year daylight-saving case must retain its exact passing single-series conformance and blocking-gate contract"
}
$calendar = $summary.weather_calendar
if ($calendar.policy_applied -ne $true -or $calendar.weather_file_allows_leap_years -ne $true -or
    $calendar.start_year -ne 2031 -or $calendar.end_year -ne 2032 -or
    $calendar.gregorian_calendar_days -ne 4 -or $calendar.weather_effective_calendar_days -ne 4 -or
    $calendar.leap_days_skipped -ne 0 -or
    $calendar.start_year_gregorian_leap -ne $false -or
    $calendar.start_year_weather_effective_leap -ne $false -or
    $calendar.end_year_gregorian_leap -ne $true -or
    $calendar.end_year_weather_effective_leap -ne $true) {
    throw "Unexpected cross-year weather calendar summary"
}
$dst = $calendar.daylight_saving
if ($dst.weather_file_period_declared -ne $true -or
    $dst.run_period_uses_weather_file_period -ne $true -or
    $dst.input_file_period_declared -ne $false -or $dst.active -ne $true -or
    $dst.effective_source -cne "weather-file" -or
    $calendar.daylight_saving_hourly_samples -ne 24) {
    throw "Cross-year daylight-saving declaration, source, policy, or sample count is incorrect"
}
$resolvedDst = $dst.resolved_period
if ($null -eq $resolvedDst -or
    $resolvedDst.start_month -ne 1 -or $resolvedDst.start_day -ne 2 -or
    $resolvedDst.start_day_of_year -ne 2 -or
    $resolvedDst.end_month -ne 1 -or $resolvedDst.end_day -ne 3 -or
    $resolvedDst.end_day_of_year -ne 3 -or $resolvedDst.wraps_year -ne $false) {
    throw "Unexpected start-year Nth-weekday daylight-saving projection diagnostic"
}
$selection = $summary.weather_record_selection
if ($selection.applied -ne $true -or $selection.data_period_index -ne 1 -or
    $selection.source_start_record_index -ne 0 -or $selection.initial_tomorrow_source_record_index -ne 0 -or
    $selection.selected_hourly_records -ne 96 -or
    $selection.skipped_raw_february_29_days -ne 0 -or $selection.day_buffer_transitions -ne 4) {
    throw "Unexpected cross-year source selection summary"
}
$specialDays = $summary.special_days
if ($specialDays.weather_file_declared -ne 0 -or
    $specialDays.run_period_uses_weather_file -ne $false -or
    $specialDays.weather_file_resolved -ne 0 -or
    $specialDays.input_file_declared -ne 0 -or $specialDays.apply_weekend_rule -ne $false -or
    $specialDays.resolved_count -ne 0 -or $specialDays.hourly_samples -ne 0 -or
    @($specialDays.resolved).Count -ne 0) {
    throw "Cross-year daylight-saving fixture must not activate special days"
}

$allSeries = @($summary.series)
if ($allSeries.Count -ne 1) {
    throw "Summary must contain exactly one series object"
}
$seriesRows = @($allSeries | Where-Object {
    $_.key -eq "ENVIRONMENT" -and $_.variable -eq "Site Daylight Saving Time Status"
})
if ($seriesRows.Count -ne 1) {
    throw "Expected exactly one Environment Site Daylight Saving Time Status series"
}
$series = $seriesRows[0]
if ($series.level -cne "conformance" -or $series.class -cne "weather" -or
    $series.frequency -cne "hourly" -or $series.source -cne "eso" -or
    $series.alignment -cne "timestamp" -or $series.expected_samples -ne 96 -or
    $series.observed_samples -ne 96 -or $series.compared_samples -ne 96) {
    throw "Unexpected cross-year Site Daylight Saving Time Status series counts"
}
if ($series.timestamp_contract -ne "ordered-exact-unique" -or $series.timestamp_status -ne "pass" -or
    $series.timestamp_expected_unique -ne $true -or $series.timestamp_observed_unique -ne $true -or
    $series.timestamp_order_match -ne $true) {
    throw "Cross-year timestamp uniqueness/order failed"
}
$expectedFirst = "env=CROSS YEAR DST START YEAR RUN PERIOD;day=1;month=12;date=30;dst=0;hour=1;start=0.00;end=60.00;day_type=Tuesday"
$expectedLast = "env=CROSS YEAR DST START YEAR RUN PERIOD;day=4;month=1;date=2;dst=1;hour=24;start=0.00;end=60.00;day_type=Friday"
if ($series.expected_first_timestamp -ne $expectedFirst -or $series.observed_first_timestamp -ne $expectedFirst -or
    $series.expected_last_timestamp -ne $expectedLast -or $series.observed_last_timestamp -ne $expectedLast) {
    throw "Unexpected cross-year timestamp endpoints"
}
if ($series.max_abs_tolerance -ne 0.0 -or $series.max_rel_tolerance -ne 0.0 -or
    $series.max_rmse_tolerance -ne 0.0 -or $series.max_abs_delta -ne 0.0 -or
    $series.rmse_delta -ne 0.0 -or $series.max_rel_delta -ne 0.0 -or
    $null -ne $series.first_divergence -or $null -ne $series.first_timestamp_divergence -or
    $series.status -cne "pass") {
    throw "Cross-year daylight-saving values must match exactly at zero tolerance"
}

$oracleEsoLines = Get-Content -LiteralPath $oracleEsoPath -Encoding UTF8
$dictionaryRows = @($oracleEsoLines | Where-Object {
    $_ -match '^\d+,1,Environment,Site Daylight Saving Time Status \[\] !Hourly$'
})
if ($dictionaryRows.Count -ne 1) {
    throw "Expected one exact Site Daylight Saving Time Status ESO dictionary entry"
}
$dictionaryMatch = [regex]::Match([string]$dictionaryRows[0], '^(\d+),')
if (-not $dictionaryMatch.Success) {
    throw "Malformed Site Daylight Saving Time Status ESO dictionary entry"
}
$reportId = $dictionaryMatch.Groups[1].Value
$values = @($oracleEsoLines | Where-Object { $_ -match ('^' + $reportId + ',\s*[-+0-9.E]+\s*$') } | ForEach-Object {
    [double](($_ -split ',', 2)[1].Trim())
})
$timestampRows = @($oracleEsoLines | Where-Object { $_ -match '^2,\d+,\s*\d+,\s*\d+,\s*\d+,\s*\d+,' })
if ($values.Count -ne 96 -or $timestampRows.Count -ne 96) {
    throw "Expected 96 oracle values and hourly timestamps"
}
$expectedDailyValues = @(0.0, 0.0, 0.0, 1.0)
$expectedDailyTypes = @("Tuesday", "Wednesday", "Thursday", "Friday")
for ($index = 0; $index -lt 96; ++$index) {
    $dayIndex = [Math]::Floor($index / 24)
    $expectedHour = ($index % 24) + 1
    if ($values[$index] -ne $expectedDailyValues[$dayIndex]) {
        throw "Unexpected oracle daylight-saving value at sample ${index}: $($values[$index])"
    }
    $timestampMatch = [regex]::Match(
        $timestampRows[$index],
        '^2,\s*(\d+),\s*(\d+),\s*(\d+),\s*(\d+),\s*(\d+),\s*([-+0-9.]+),\s*([-+0-9.]+),([^,]+)$'
    )
    if (-not $timestampMatch.Success -or
        [int]$timestampMatch.Groups[1].Value -ne ($dayIndex + 1) -or
        [int]$timestampMatch.Groups[2].Value -ne $expectedMonths[$dayIndex] -or
        [int]$timestampMatch.Groups[3].Value -ne $expectedDays[$dayIndex] -or
        [int]$timestampMatch.Groups[4].Value -ne [int]$expectedDailyValues[$dayIndex] -or
        [int]$timestampMatch.Groups[5].Value -ne $expectedHour -or
        $timestampMatch.Groups[6].Value -cne "0.00" -or
        $timestampMatch.Groups[7].Value -cne "60.00" -or
        $timestampMatch.Groups[8].Value.Trim() -cne $expectedDailyTypes[$dayIndex]) {
        throw "Unexpected oracle daylight-saving timestamp at sample ${index}: $($timestampRows[$index])"
    }
}

$oracleEioLines = Get-Content -LiteralPath $oracleEioPath -Encoding UTF8
$expectedEnvironmentRow = "Environment,CROSS YEAR DST START YEAR RUN PERIOD,WeatherFileRunPeriod,12/30/2031,01/02/2032,Tuesday,4,Use RunPeriod Specified Day,Yes,No,No,No,No,Clark and Allen"
$expectedDstRow = "Environment:Daylight Saving,Yes,WeatherFile,01/02,01/03"
$environmentRows = @($oracleEioLines | Where-Object { $_ -like "Environment,CROSS YEAR DST START YEAR RUN PERIOD,*" })
$dstRows = @($oracleEioLines | Where-Object { $_ -like "Environment:Daylight Saving,*" })
$specialDayRows = @($oracleEioLines | Where-Object { $_ -like "Environment:Special Days,*" })
if ($environmentRows.Count -ne 1 -or $environmentRows[0] -cne $expectedEnvironmentRow -or
    @($environmentRows[0] -split ',').Count -ne 14 -or
    $dstRows.Count -ne 1 -or $dstRows[0] -cne $expectedDstRow -or
    @($dstRows[0] -split ',').Count -ne 5 -or $specialDayRows.Count -ne 0) {
    throw "Unexpected exact cross-year Environment, Daylight Saving, or Special Days EIO rows"
}
Write-Host "OK exact cross-year Environment and start-year Daylight Saving EIO rows"
$errText = Get-Content -LiteralPath $oracleErrPath -Raw -Encoding UTF8
Assert-Contains -Text $errText -Pattern "EnergyPlus Completed Successfully-- 0 Warning; 0 Severe Errors;" -Description "clean EnergyPlus completion"
if ([regex]::Matches($errText, '(?m)^\s*\*\* Warning \*\*').Count -ne 0 -or
    [regex]::Matches($errText, '(?m)^\s*\*\* Severe\s+\*\*').Count -ne 0) {
    throw "Cross-year daylight-saving oracle must complete without warning or severe markers"
}

$reportText = Get-Content -LiteralPath $reportPath -Raw -Encoding UTF8
Assert-Contains -Text $reportText -Pattern "gregorian_calendar_years: 2031..2032" -Description "markdown cross-year calendar range"
Assert-Contains -Text $reportText -Pattern "start_year_gregorian_leap: false" -Description "markdown common start year"
Assert-Contains -Text $reportText -Pattern "start_year_weather_effective_leap: false" -Description "markdown common weather-effective start year"
Assert-Contains -Text $reportText -Pattern "end_year_gregorian_leap: true" -Description "markdown Gregorian leap end year"
Assert-Contains -Text $reportText -Pattern "end_year_weather_effective_leap: true" -Description "markdown weather-effective leap end year"
Assert-Contains -Text $reportText -Pattern "weather_file_daylight_saving_period_declared: true" -Description "markdown EPW daylight-saving declaration"
Assert-Contains -Text $reportText -Pattern "run_period_uses_weather_file_daylight_saving_period: true" -Description "markdown enabled EPW daylight-saving policy"
Assert-Contains -Text $reportText -Pattern "input_file_daylight_saving_period_declared: false" -Description "markdown absent input-file daylight-saving declaration"
Assert-Contains -Text $reportText -Pattern "daylight_saving_active: true" -Description "markdown active daylight-saving state"
Assert-Contains -Text $reportText -Pattern "daylight_saving_effective_source: weather-file" -Description "markdown weather-file daylight-saving source"
Assert-Contains -Text $reportText -Pattern "daylight_saving_resolved_period: 1/2 through 1/3 (wraps_year=false)" -Description "markdown start-year daylight-saving projection"
Assert-Contains -Text $reportText -Pattern "daylight_saving_hourly_samples: 24" -Description "markdown active daylight-saving samples"
Assert-Contains -Text $reportText -Pattern "weather_file_holidays_declared: 0" -Description "markdown zero EPW holidays"
Assert-Contains -Text $reportText -Pattern "run_period_uses_weather_file_holidays: false" -Description "markdown disabled EPW holiday policy"
Assert-Contains -Text $reportText -Pattern "weather_file_holidays_resolved: 0" -Description "markdown zero resolved EPW holidays"
Assert-Contains -Text $reportText -Pattern "input_file_special_days_declared: 0" -Description "markdown zero input-file special days"
Assert-Contains -Text $reportText -Pattern "special_days_resolved: 0" -Description "markdown zero resolved special days"
Assert-Contains -Text $reportText -Pattern "special_day_hourly_samples: 0" -Description "markdown zero special-day samples"
Assert-Contains -Text $reportText -Pattern "weather_selected_hourly_records: 96" -Description "markdown selected source rows"
Assert-Contains -Text $reportText -Pattern "weather_initial_tomorrow_record_index: 0" -Description "markdown initial tomorrow source row"
$specialDayMarkdownRows = @(($reportText -split '\r?\n') | Where-Object { $_ -like "special_day_resolved:*" })
if ($specialDayMarkdownRows.Count -ne 0) {
    throw "Markdown must not contain resolved special-day rows"
}
Write-Host "OK markdown contains no resolved special-day rows"
Write-Host "Cross-year start-year daylight-saving projection exact gate passed."
