[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$OutputRoot = Join-Path $RepoRoot ".runtime\time-weather-schedule-conformance\26.1.0"
$WeatherRef = "data/conformance_cases/calendar_dst_fixed_date_hourly_exact_001/calendar_dst_fixed_date_hourly_exact.epw"
$WeatherPath = Join-Path $RepoRoot ($WeatherRef -replace '/', '\')
$ExpectedHeader = "HOLIDAYS/DAYLIGHT SAVINGS,Yes,2/29,3/1,0"
$ExpectedDataPeriod = "DATA PERIODS,1,1,Data,Sunday,2/28,3/1"
$ExpectedMonths = @(2, 2, 3)
$ExpectedDates = @(28, 29, 1)
$ExpectedDayTypes = @("Sunday", "Monday", "Tuesday")
$ExpectedFirstTimestamp = "env=DST FIXED DATE RUN PERIOD;day=1;month=2;date=28;dst=0;hour=1;start=0.00;end=60.00;day_type=Sunday"
$Cases = @(
    [pscustomobject]@{
        Id = "calendar_dst_fixed_date_hourly_exact_001"
        IdfName = "calendar_dst_fixed_date_hourly_exact.idf"
        UsePolicy = $true
        PolicyText = "Yes, !- Use Weather File Daylight Saving Period"
        ExpectedDst = @(0, 1, 1)
        ActiveSamples = 48
        HasResolvedPeriod = $true
        ExpectedLastTimestamp = "env=DST FIXED DATE RUN PERIOD;day=3;month=3;date=1;dst=1;hour=24;start=0.00;end=60.00;day_type=Tuesday"
        ExpectedEnvironmentEio = "Environment,DST FIXED DATE RUN PERIOD,WeatherFileRunPeriod,02/28/2016,03/01/2016,Sunday,3,Use RunPeriod Specified Day,Yes,No,No,No,No,Clark and Allen"
        ExpectedDaylightSavingEio = "Environment:Daylight Saving,Yes,WeatherFile,02/29,03/01"
        ExpectedDaylightSavingEioFieldCount = 5
    },
    [pscustomobject]@{
        Id = "calendar_dst_fixed_date_disabled_hourly_exact_001"
        IdfName = "calendar_dst_fixed_date_disabled_hourly_exact.idf"
        UsePolicy = $false
        PolicyText = "No,  !- Use Weather File Daylight Saving Period"
        ExpectedDst = @(0, 0, 0)
        ActiveSamples = 0
        HasResolvedPeriod = $false
        ExpectedLastTimestamp = "env=DST FIXED DATE RUN PERIOD;day=3;month=3;date=1;dst=0;hour=24;start=0.00;end=60.00;day_type=Tuesday"
        ExpectedEnvironmentEio = "Environment,DST FIXED DATE RUN PERIOD,WeatherFileRunPeriod,02/28/2016,03/01/2016,Sunday,3,Use RunPeriod Specified Day,No,No,No,No,No,Clark and Allen"
        ExpectedDaylightSavingEio = "Environment:Daylight Saving,No,RunPeriod Object"
        ExpectedDaylightSavingEioFieldCount = 3
    }
)

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

foreach ($case in $Cases) {
    $caseRoot = Join-Path $RepoRoot "data\conformance_cases\$($case.Id)"
    $case | Add-Member -NotePropertyName CaseRoot -NotePropertyValue $caseRoot
    $case | Add-Member -NotePropertyName CasePath -NotePropertyValue (Join-Path $caseRoot "case.toml")
    $case | Add-Member -NotePropertyName IdfPath -NotePropertyValue (Join-Path $caseRoot $case.IdfName)
    $case | Add-Member -NotePropertyName IdfRef -NotePropertyValue "data/conformance_cases/$($case.Id)/$($case.IdfName)"
    $case | Add-Member -NotePropertyName OutputRoot -NotePropertyValue (Join-Path $OutputRoot $case.Id)
}

foreach ($path in @(
    (Join-Path $OracleRoot "energyplus.exe"),
    (Join-Path $OracleRoot "ConvertInputFormat.exe"),
    $WeatherPath
) + @($Cases | ForEach-Object { @($_.CasePath, $_.IdfPath) })) {
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Missing required fixed-date EPW daylight-saving policy conformance file: $path"
    }
}

$weatherLines = Get-Content -LiteralPath $WeatherPath -Encoding UTF8
$weatherNonblankLines = @($weatherLines | Where-Object { -not [string]::IsNullOrWhiteSpace($_) })
if ($weatherLines.Count -ne 81 -or $weatherNonblankLines.Count -ne 80 -or
    -not [string]::IsNullOrWhiteSpace($weatherLines[-1])) {
    throw "Fixed-date DST policy EPW must contain exactly 80 nonblank lines followed by one trailing blank line"
}
$calendarHeaders = @($weatherNonblankLines | Where-Object { $_ -match '^\s*HOLIDAYS/DAYLIGHT SAVING' })
if ($calendarHeaders.Count -ne 1 -or $calendarHeaders[0] -cne $ExpectedHeader) {
    throw "Fixed-date DST policy EPW must contain exactly the expected 2/29 through 3/1 header"
}
$dataPeriodHeaders = @($weatherNonblankLines | Where-Object { $_ -match '^\s*DATA PERIODS,' })
if ($dataPeriodHeaders.Count -ne 1 -or $dataPeriodHeaders[0] -cne $ExpectedDataPeriod) {
    throw "Fixed-date DST policy EPW must contain exactly the expected Sunday-start DATA PERIODS header"
}
$weatherRows = @($weatherNonblankLines | Select-Object -Skip 8)
if ($weatherRows.Count -ne 72) {
    throw "Fixed-date DST policy EPW must contain 72 hourly rows, found $($weatherRows.Count)"
}
$orderedPayloads = @()
for ($rowIndex = 0; $rowIndex -lt 72; ++$rowIndex) {
    $dayIndex = [int][math]::Floor($rowIndex / 24)
    $expectedHour = ($rowIndex % 24) + 1
    $fields = $weatherRows[$rowIndex] -split ','
    if ($fields.Count -ne 35 -or $fields[0] -ne "2016" -or
        [int]$fields[1] -ne $ExpectedMonths[$dayIndex] -or
        [int]$fields[2] -ne $ExpectedDates[$dayIndex] -or
        [int]$fields[3] -ne $expectedHour -or $fields[4] -ne "60") {
        throw "Fixed-date DST policy EPW row $rowIndex must retain exact date/hour order, 35 fields, and minute 60"
    }
    $orderedPayloads += ($fields[5..34] -join ',')
}
if (@($orderedPayloads | Select-Object -Unique).Count -ne 1) {
    throw "Fixed-date DST policy EPW weather payload must remain constant across all 72 rows"
}
for ($dayIndex = 0; $dayIndex -lt 3; ++$dayIndex) {
    $date = "2016,$($ExpectedMonths[$dayIndex]),$($ExpectedDates[$dayIndex])"
    $dateRows = @($weatherRows | Where-Object { $_ -match ('^' + [regex]::Escape($date) + ',') })
    $hours = @($dateRows | ForEach-Object { [int](($_ -split ',')[3]) })
    if ($dateRows.Count -ne 24 -or ($hours -join ',') -cne ((1..24) -join ',')) {
        throw "Fixed-date DST policy EPW must contain ordered hours 1..24 for $date"
    }
}

$idfTexts = @{}
foreach ($case in $Cases) {
    $caseText = Get-Content -LiteralPath $case.CasePath -Raw -Encoding UTF8
    $idfText = Get-Content -LiteralPath $case.IdfPath -Raw -Encoding UTF8
    $idfTexts[$case.Id] = $idfText

    foreach ($contract in @(
        'comparison_class = "conformance"',
        'conformance_claim = true',
        'timestamp_contract = "ordered-exact-unique"',
        'frequency = "hourly"',
        'class = "weather"',
        'source = "eso"',
        'level = "conformance"',
        'abs_tol = 0.0',
        'rmse_tol = 0.0',
        'blocking = true'
    )) {
        Assert-Contains -Text $caseText -Pattern $contract -Description "$($case.Id) manifest contract"
    }
    Assert-Contains -Text $caseText -Pattern "idf = `"$($case.IdfRef)`"" -Description "$($case.Id) manifest input.idf attribution"
    Assert-Contains -Text $caseText -Pattern "weather = `"$WeatherRef`"" -Description "$($case.Id) shared manifest weather attribution"
    Assert-Contains -Text $caseText -Pattern 'script = "scripts/dev.cmd compare-calendar-dst-fixed-date-policy-exact"' -Description "$($case.Id) paired gate attribution"

    $policyRows = [regex]::Matches($idfText, '(?m)^\s*(?:Yes|No),\s*!- Use Weather File Daylight Saving Period\s*$')
    if ($policyRows.Count -ne 1 -or $policyRows[0].Value.Trim() -cne $case.PolicyText) {
        throw "$($case.Id) must contain exactly its declared RunPeriod weather-file DST policy token"
    }
    foreach ($policy in @(
        "No,  !- Use Weather File Holidays and Special Days",
        "No,  !- Apply Weekend Holiday Rule",
        "No,  !- Use Weather File Rain Indicators",
        "No,  !- Use Weather File Snow Indicators",
        "No;  !- Treat Weather as Actual"
    )) {
        Assert-Contains -Text $idfText -Pattern $policy -Description "$($case.Id) explicit non-DST RunPeriod policy"
    }

    $runPeriodObjects = [regex]::Matches($idfText, '(?ims)^\s*RunPeriod\s*,(?<body>.*?);')
    $outputObjects = [regex]::Matches($idfText, '(?ims)^\s*Output:Variable\s*,(?<body>.*?);')
    if ($runPeriodObjects.Count -ne 1 -or $outputObjects.Count -ne 1) {
        throw "$($case.Id) must contain exactly one RunPeriod and one Output:Variable"
    }
    $runPeriodBody = [regex]::Replace($runPeriodObjects[0].Groups["body"].Value, '(?m)!-.*$', '')
    $runPeriodFields = @($runPeriodBody -split ',' | ForEach-Object { $_.Trim() } | Where-Object { $_ -ne "" })
    $expectedUseField = if ($case.UsePolicy) { "Yes" } else { "No" }
    $expectedRunPeriodFields = @(
        "DST Fixed Date Run Period", "2", "28", "2016", "3", "1", "2016", "Sunday",
        "No", $expectedUseField, "No", "No", "No", "No"
    )
    if (($runPeriodFields -join '|') -cne ($expectedRunPeriodFields -join '|')) {
        throw "$($case.Id) must retain the exact fixed-date RunPeriod and explicit policies"
    }
    $outputFields = @($outputObjects[0].Groups["body"].Value -split ',' | ForEach-Object { $_.Trim() } | Where-Object { $_ -ne "" })
    if (($outputFields -join '|') -cne "Environment|Site Daylight Saving Time Status|Hourly") {
        throw "$($case.Id) must request exactly the hourly Site Daylight Saving Time Status"
    }
    if ([regex]::Matches($idfText, '(?im)^\s*RunPeriodControl:SpecialDays\s*,').Count -ne 0 -or
        [regex]::Matches($idfText, '(?im)^\s*RunPeriodControl:DaylightSavingTime\s*,').Count -ne 0) {
        throw "$($case.Id) must not contain input-file special-day or daylight-saving objects"
    }
}

$normalizationToken = "<RUNPERIOD WEATHER-FILE DST POLICY>"
$strictUtf8 = New-Object System.Text.UTF8Encoding($false, $true)
$enabledRawText = $strictUtf8.GetString([System.IO.File]::ReadAllBytes($Cases[0].IdfPath))
$disabledRawText = $strictUtf8.GetString([System.IO.File]::ReadAllBytes($Cases[1].IdfPath))
$enabledPolicyMatches = [regex]::Matches($enabledRawText, [regex]::Escape($Cases[0].PolicyText))
$disabledPolicyMatches = [regex]::Matches($disabledRawText, [regex]::Escape($Cases[1].PolicyText))
if ($enabledPolicyMatches.Count -ne 1 -or $disabledPolicyMatches.Count -ne 1) {
    throw "Paired fixed-date DST IDFs must each contain exactly one normalizable policy token"
}
$enabledNormalizedBytes = $strictUtf8.GetBytes(
    $enabledRawText.Replace($Cases[0].PolicyText, $normalizationToken)
)
$disabledNormalizedBytes = $strictUtf8.GetBytes(
    $disabledRawText.Replace($Cases[1].PolicyText, $normalizationToken)
)
if ([Convert]::ToBase64String($enabledNormalizedBytes) -cne [Convert]::ToBase64String($disabledNormalizedBytes)) {
    throw "Paired fixed-date DST IDFs differ outside the RunPeriod weather-file DST use-policy token"
}

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

$observedEnvironmentEio = @{}
Write-Host "Running paired fixed-date EPW daylight-saving RunPeriod use-policy exact gate."
foreach ($case in $Cases) {
    Remove-RepoDirectory -Path $case.OutputRoot
    $output = & $cargo.Source run -p ep_cli --quiet -- conformance time-weather-schedule-report $case.CasePath $OracleRoot $OutputRoot 2>&1
    if ($LASTEXITCODE -ne 0) {
        $output | ForEach-Object { Write-Host $_ }
        throw "Fixed-date EPW daylight-saving policy case failed: $($case.Id)"
    }
    $outputText = $output -join "`n"
    Assert-Contains -Text $outputText -Pattern "id: $($case.Id)" -Description "$($case.Id) report id"
    Assert-Contains -Text $outputText -Pattern "status: pass" -Description "$($case.Id) report status"

    $summaryPath = Join-Path $case.OutputRoot "compare\compare-summary.json"
    $reportPath = Join-Path $case.OutputRoot "compare\compare-report.md"
    $oracleEsoPath = Join-Path $case.OutputRoot "oracle\eplusout.eso"
    $oracleEioPath = Join-Path $case.OutputRoot "oracle\eplusout.eio"
    $oracleErrPath = Join-Path $case.OutputRoot "oracle\eplusout.err"
    $oracleEndPath = Join-Path $case.OutputRoot "oracle\eplusout.end"
    foreach ($path in @($summaryPath, $reportPath, $oracleEsoPath, $oracleEioPath, $oracleErrPath, $oracleEndPath)) {
        if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
            throw "Missing $($case.Id) comparison artifact: $path"
        }
    }

    $summary = Get-Content -LiteralPath $summaryPath -Raw -Encoding UTF8 | ConvertFrom-Json
    if ($summary.case_id -cne $case.Id -or $summary.oracle_version -cne "26.1.0" -or
        $summary.comparison_class -cne "conformance" -or $summary.conformance_claim -ne $true -or
        $summary.status -cne "pass" -or $summary.time_axis_samples -ne 72 -or
        $summary.series_count -ne 1 -or $summary.conformance_series_count -ne 1 -or
        $summary.gate.script -cne "scripts/dev.cmd compare-calendar-dst-fixed-date-policy-exact" -or
        $summary.gate.blocking -ne $true) {
        throw "$($case.Id) must retain its exact passing single-series conformance and paired blocking-gate contract"
    }

    $calendar = $summary.weather_calendar
    if ($calendar.policy_applied -ne $true -or $calendar.weather_file_allows_leap_years -ne $true -or
        $calendar.gregorian_calendar_days -ne 3 -or $calendar.weather_effective_calendar_days -ne 3 -or
        $calendar.leap_days_skipped -ne 0) {
        throw "Unexpected $($case.Id) three-day leap-year weather calendar state"
    }
    $daylightSaving = $calendar.daylight_saving
    if ($daylightSaving.weather_file_period_declared -ne $true -or
        $daylightSaving.run_period_uses_weather_file_period -ne $case.UsePolicy -or
        $daylightSaving.active -ne $case.UsePolicy) {
        throw "Unexpected $($case.Id) declared/use/active daylight-saving summary state"
    }
    if ($case.HasResolvedPeriod) {
        $resolvedPeriod = $daylightSaving.resolved_period
        if ($null -eq $resolvedPeriod -or
            $resolvedPeriod.start_month -ne 2 -or $resolvedPeriod.start_day -ne 29 -or
            $resolvedPeriod.start_day_of_year -ne 60 -or
            $resolvedPeriod.end_month -ne 3 -or $resolvedPeriod.end_day -ne 1 -or
            $resolvedPeriod.end_day_of_year -ne 61 -or $resolvedPeriod.wraps_year -ne $false) {
            throw "Unexpected enabled fixed-date daylight-saving resolved period"
        }
    } elseif ($null -ne $daylightSaving.resolved_period) {
        throw "Disabled fixed-date daylight-saving summary must retain a null resolved period"
    }
    if ($calendar.daylight_saving_hourly_samples -ne $case.ActiveSamples) {
        throw "Expected $($case.ActiveSamples) DST-active samples for $($case.Id), found $($calendar.daylight_saving_hourly_samples)"
    }

    $specialDays = $summary.special_days
    if ($specialDays.weather_file_declared -ne 0 -or $specialDays.run_period_uses_weather_file -ne $false -or
        $specialDays.weather_file_resolved -ne 0 -or $specialDays.input_file_declared -ne 0 -or
        $specialDays.apply_weekend_rule -ne $false -or $specialDays.resolved_count -ne 0 -or
        $specialDays.hourly_samples -ne 0) {
        throw "$($case.Id) must not activate holidays or input-file special days"
    }
    $selection = $summary.weather_record_selection
    if ($selection.applied -ne $true -or $selection.data_period_index -ne 1 -or
        $selection.source_start_record_index -ne 0 -or $selection.initial_tomorrow_source_record_index -ne 0 -or
        $selection.selected_hourly_records -ne 72 -or $selection.skipped_raw_february_29_days -ne 0 -or
        $selection.day_buffer_transitions -ne 3) {
        throw "Unexpected $($case.Id) weather record selection state"
    }

    $seriesRows = @($summary.series | Where-Object {
        $_.key -eq "ENVIRONMENT" -and $_.variable -eq "Site Daylight Saving Time Status"
    })
    if ($seriesRows.Count -ne 1) {
        throw "Missing unique Site Daylight Saving Time Status series for $($case.Id)"
    }
    $series = $seriesRows[0]
    if ($series.level -cne "conformance" -or $series.class -cne "weather" -or
        $series.frequency -cne "hourly" -or $series.source -cne "eso" -or
        $series.alignment -cne "timestamp" -or
        $series.expected_samples -ne 72 -or $series.observed_samples -ne 72 -or $series.compared_samples -ne 72 -or
        $series.timestamp_contract -cne "ordered-exact-unique" -or $series.timestamp_status -cne "pass" -or
        $series.timestamp_expected_unique -ne $true -or $series.timestamp_observed_unique -ne $true -or
        $series.timestamp_order_match -ne $true -or
        $series.expected_first_timestamp -cne $ExpectedFirstTimestamp -or
        $series.observed_first_timestamp -cne $ExpectedFirstTimestamp -or
        $series.expected_last_timestamp -cne $case.ExpectedLastTimestamp -or
        $series.observed_last_timestamp -cne $case.ExpectedLastTimestamp -or
        $series.max_abs_tolerance -ne 0.0 -or $series.max_rel_tolerance -ne 0.0 -or
        $series.max_rmse_tolerance -ne 0.0 -or $series.max_abs_delta -ne 0.0 -or
        $series.rmse_delta -ne 0.0 -or $series.max_rel_delta -ne 0.0 -or
        $null -ne $series.first_divergence -or $null -ne $series.first_timestamp_divergence -or
        $series.status -cne "pass") {
        throw "Ordered exact Site Daylight Saving Time Status contract failed for $($case.Id)"
    }

    $oracleEsoLines = Get-Content -LiteralPath $oracleEsoPath -Encoding UTF8
    $dictionaryRows = @($oracleEsoLines | Where-Object {
        $_ -match '^\d+,1,Environment,Site Daylight Saving Time Status \[\] !Hourly$'
    })
    if ($dictionaryRows.Count -ne 1) {
        throw "Expected one exact Site Daylight Saving Time Status ESO dictionary row for $($case.Id)"
    }
    $dictionaryMatch = [regex]::Match([string]$dictionaryRows[0], '^(\d+),')
    if (-not $dictionaryMatch.Success) {
        throw "Missing Site Daylight Saving Time Status ESO report id for $($case.Id)"
    }
    $reportId = $dictionaryMatch.Groups[1].Value
    $valueRows = @($oracleEsoLines | Where-Object { $_ -match ('^' + $reportId + ',\s*[-+0-9.E]+\s*$') })
    $values = @($valueRows | ForEach-Object { [double](($_ -split ',', 2)[1].Trim()) })
    $timestampRows = @($oracleEsoLines | Where-Object { $_ -match '^2,\d+,\s*\d+,\s*\d+,\s*\d+,\s*\d+,' })
    if ($values.Count -ne 72 -or $timestampRows.Count -ne 72) {
        throw "Expected 72 raw oracle daylight-saving values and timestamps for $($case.Id)"
    }
    for ($index = 0; $index -lt 72; ++$index) {
        $dayOffset = [int][math]::Floor($index / 24)
        $expectedHour = ($index % 24) + 1
        $expectedDst = [int]$case.ExpectedDst[$dayOffset]
        if ($values[$index] -ne [double]$expectedDst) {
            throw "Unexpected $($case.Id) oracle Site Daylight Saving Time Status at sample $index`: $($values[$index])"
        }
        $timestampMatch = [regex]::Match(
            $timestampRows[$index],
            '^2,\s*(\d+),\s*(\d+),\s*(\d+),\s*(\d+),\s*(\d+),\s*([-+0-9.]+),\s*([-+0-9.]+),([^,]+)$'
        )
        if (-not $timestampMatch.Success -or
            [int]$timestampMatch.Groups[1].Value -ne ($dayOffset + 1) -or
            [int]$timestampMatch.Groups[2].Value -ne $ExpectedMonths[$dayOffset] -or
            [int]$timestampMatch.Groups[3].Value -ne $ExpectedDates[$dayOffset] -or
            [int]$timestampMatch.Groups[4].Value -ne $expectedDst -or
            [int]$timestampMatch.Groups[5].Value -ne $expectedHour -or
            $timestampMatch.Groups[6].Value -cne "0.00" -or
            $timestampMatch.Groups[7].Value -cne "60.00" -or
            $timestampMatch.Groups[8].Value.Trim() -cne $ExpectedDayTypes[$dayOffset]) {
            throw "Unexpected $($case.Id) oracle daylight-saving timestamp at sample $index`: $($timestampRows[$index])"
        }
    }

    $oracleEioLines = Get-Content -LiteralPath $oracleEioPath -Encoding UTF8
    $environmentEioRows = @($oracleEioLines | Where-Object { $_ -match '^Environment,' })
    if ($environmentEioRows.Count -ne 1 -or
        [string]$environmentEioRows[0] -cne $case.ExpectedEnvironmentEio -or
        @(([string]$environmentEioRows[0]) -split ',').Count -ne 14) {
        throw "Expected exact 14-field EnergyPlus Environment EIO row for $($case.Id)"
    }
    $observedEnvironmentEio[$case.Id] = [string]$environmentEioRows[0]
    $daylightSavingEioRows = @($oracleEioLines | Where-Object { $_ -match '^Environment:Daylight Saving,' })
    if ($daylightSavingEioRows.Count -ne 1 -or
        [string]$daylightSavingEioRows[0] -cne $case.ExpectedDaylightSavingEio -or
        @(([string]$daylightSavingEioRows[0]) -split ',').Count -ne $case.ExpectedDaylightSavingEioFieldCount) {
        throw "Expected exact EnergyPlus Environment:Daylight Saving EIO row for $($case.Id)"
    }

    $oracleErrText = Get-Content -LiteralPath $oracleErrPath -Raw -Encoding UTF8
    Assert-Contains -Text $oracleErrText -Pattern "EnergyPlus Completed Successfully-- 0 Warning; 0 Severe Errors;" -Description "$($case.Id) clean EnergyPlus completion"
    if ([regex]::Matches($oracleErrText, '(?m)^\s*\*\* Warning \*\*').Count -ne 0 -or
        [regex]::Matches($oracleErrText, '(?m)^\s*\*\* Severe\s+\*\*').Count -ne 0) {
        throw "$($case.Id) oracle must complete without warning or severe markers"
    }
    $oracleEndText = Get-Content -LiteralPath $oracleEndPath -Raw -Encoding UTF8
    Assert-Contains -Text $oracleEndText -Pattern "EnergyPlus Completed Successfully-- 0 Warning; 0 Severe Errors;" -Description "$($case.Id) clean EnergyPlus end record"

    $reportText = Get-Content -LiteralPath $reportPath -Raw -Encoding UTF8
    $useLabel = $case.UsePolicy.ToString().ToLowerInvariant()
    $resolvedLabel = if ($case.HasResolvedPeriod) { "2/29 through 3/1 (wraps_year=false)" } else { "none" }
    foreach ($entry in @(
        "weather_file_daylight_saving_period_declared: true",
        "run_period_uses_weather_file_daylight_saving_period: $useLabel",
        "daylight_saving_active: $useLabel",
        "daylight_saving_resolved_period: $resolvedLabel",
        "daylight_saving_hourly_samples: $($case.ActiveSamples)",
        "weather_file_holidays_declared: 0",
        "special_days_resolved: 0",
        "weather_selected_hourly_records: 72",
        "weather_day_buffer_transitions: 3"
    )) {
        Assert-Contains -Text $reportText -Pattern $entry -Description "$($case.Id) markdown fixed-date DST policy state"
    }
}

$enabledEnvironmentFields = @($observedEnvironmentEio[$Cases[0].Id] -split ',')
$disabledEnvironmentFields = @($observedEnvironmentEio[$Cases[1].Id] -split ',')
if ($enabledEnvironmentFields.Count -ne 14 -or $disabledEnvironmentFields.Count -ne 14 -or
    $enabledEnvironmentFields[8] -cne "Yes" -or $disabledEnvironmentFields[8] -cne "No") {
    throw "Paired Environment EIO rows must expose the exact Yes/No Use Daylight Saving distinction at field 8"
}
$enabledEnvironmentFields[8] = "<USE DST>"
$disabledEnvironmentFields[8] = "<USE DST>"
if (($enabledEnvironmentFields -join ',') -cne ($disabledEnvironmentFields -join ',')) {
    throw "Paired Environment EIO rows differ outside the Use Daylight Saving field"
}

Write-Host "Paired fixed-date EPW daylight-saving RunPeriod use-policy exact gate passed."
