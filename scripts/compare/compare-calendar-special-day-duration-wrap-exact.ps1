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
$ExpectedEnvironment = "SPECIAL DAY DURATION WRAP RUN PERIOD"
$ExpectedSpecialDayName = "THREE DAY YEAR WRAP HOLIDAY"
$Cases = @(
    [pscustomobject]@{
        Id = "calendar_special_day_duration_wrap_common_year_hourly_exact_001"
        IdfName = "calendar_special_day_duration_wrap_common_year_hourly_exact.idf"
        WeatherName = "calendar_special_day_duration_wrap_common_year_hourly_exact.epw"
        Year = 2017
        LeapObserved = "No"
        StartWeekday = "Sunday"
        StartDayOfYear = 365
        FinalDayType = "Tuesday"
        FinalDayTypeIndex = 3.0
        BoundaryClaim = "common-year day-of-year 365"
    },
    [pscustomobject]@{
        Id = "calendar_special_day_duration_wrap_leap_year_hourly_exact_001"
        IdfName = "calendar_special_day_duration_wrap_leap_year_hourly_exact.idf"
        WeatherName = "calendar_special_day_duration_wrap_leap_year_hourly_exact.epw"
        Year = 2016
        LeapObserved = "Yes"
        StartWeekday = "Friday"
        StartDayOfYear = 366
        FinalDayType = "Sunday"
        FinalDayTypeIndex = 1.0
        BoundaryClaim = "leap-year day-of-year 366"
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
    $case | Add-Member -NotePropertyName CasePath -NotePropertyValue (Join-Path $caseRoot "case.toml")
    $case | Add-Member -NotePropertyName IdfPath -NotePropertyValue (Join-Path $caseRoot $case.IdfName)
    $case | Add-Member -NotePropertyName WeatherPath -NotePropertyValue (Join-Path $caseRoot $case.WeatherName)
    $case | Add-Member -NotePropertyName IdfRef -NotePropertyValue "data/conformance_cases/$($case.Id)/$($case.IdfName)"
    $case | Add-Member -NotePropertyName WeatherRef -NotePropertyValue "data/conformance_cases/$($case.Id)/$($case.WeatherName)"
    $case | Add-Member -NotePropertyName CaseOutputRoot -NotePropertyValue (Join-Path $OutputRoot $case.Id)
}

foreach ($path in @(
    (Join-Path $OracleRoot "energyplus.exe"),
    (Join-Path $OracleRoot "ConvertInputFormat.exe")
)) {
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Missing required EnergyPlus oracle file: $path"
    }
}
foreach ($case in $Cases) {
    foreach ($path in @($case.CasePath, $case.IdfPath, $case.WeatherPath)) {
        if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
            throw "Missing required duration-wrap conformance file: $path"
        }
    }
}
if ([System.IO.Path]::GetFullPath($Cases[0].WeatherPath) -eq [System.IO.Path]::GetFullPath($Cases[1].WeatherPath)) {
    throw "Common-year and leap-year duration-wrap fixtures must use independent EPW files"
}

$idfTexts = @{}
foreach ($case in $Cases) {
    $caseText = Get-Content -LiteralPath $case.CasePath -Raw -Encoding UTF8
    $idfText = Get-Content -LiteralPath $case.IdfPath -Raw -Encoding UTF8
    $idfTexts[$case.Id] = $idfText

    Assert-Contains -Text $caseText -Pattern 'comparison_class = "conformance"' -Description "$($case.Id) conformance comparison class"
    Assert-Contains -Text $caseText -Pattern 'conformance_claim = true' -Description "$($case.Id) conformance claim"
    Assert-Contains -Text $caseText -Pattern 'tier = "A"' -Description "$($case.Id) Tier A attribution"
    Assert-Contains -Text $caseText -Pattern 'domains = ["weather"]' -Description "$($case.Id) weather-only scope"
    Assert-Contains -Text $caseText -Pattern "idf = `"$($case.IdfRef)`"" -Description "$($case.Id) independent IDF attribution"
    Assert-Contains -Text $caseText -Pattern "weather = `"$($case.WeatherRef)`"" -Description "$($case.Id) independent EPW attribution"
    Assert-Contains -Text $caseText -Pattern 'timestamp_contract = "ordered-exact-unique"' -Description "$($case.Id) ordered timestamp contract"
    Assert-Contains -Text $caseText -Pattern 'abs_tol = 0.0' -Description "$($case.Id) zero absolute tolerance"
    Assert-Contains -Text $caseText -Pattern 'rmse_tol = 0.0' -Description "$($case.Id) zero RMSE tolerance"
    Assert-Contains -Text $caseText -Pattern 'script = "scripts/dev.cmd compare-calendar-special-day-duration-wrap-exact"' -Description "$($case.Id) blocking gate attribution"
    Assert-Contains -Text $caseText -Pattern 'blocking = true' -Description "$($case.Id) blocking gate flag"
    Assert-Contains -Text $caseText -Pattern $case.BoundaryClaim -Description "$($case.Id) narrow resolved-day claim"
    Assert-Contains -Text $caseText -Pattern "same-year cyclic annual-table wrap, not a cross-year RunPeriod" -Description "$($case.Id) annual-table boundary"

    Assert-Contains -Text $idfText -Pattern "  $($case.Year)," -Description "$($case.Id) explicit run-period year"
    Assert-Contains -Text $idfText -Pattern "  $($case.StartWeekday)," -Description "$($case.Id) explicit start weekday"
    Assert-Contains -Text $idfText -Pattern "No,  !- Use Weather File Holidays and Special Days" -Description "$($case.Id) disabled EPW holidays"
    Assert-Contains -Text $idfText -Pattern "No,  !- Use Weather File Daylight Saving Period" -Description "$($case.Id) disabled EPW DST"
    Assert-Contains -Text $idfText -Pattern "No,  !- Apply Weekend Holiday Rule" -Description "$($case.Id) disabled weekend rule"
    Assert-Contains -Text $idfText -Pattern "No,  !- Use Weather File Rain Indicators" -Description "$($case.Id) disabled EPW rain"
    Assert-Contains -Text $idfText -Pattern "No,  !- Use Weather File Snow Indicators" -Description "$($case.Id) disabled EPW snow"
    Assert-Contains -Text $idfText -Pattern "No;  !- Treat Weather as Actual" -Description "$($case.Id) disabled actual weather"
    Assert-Contains -Text $idfText -Pattern "Three Day Year Wrap Holiday" -Description "$($case.Id) duration-wrap special-day name"
    Assert-Contains -Text $idfText -Pattern "  12/31," -Description "$($case.Id) exact year-end date"
    Assert-Contains -Text $idfText -Pattern "  3," -Description "$($case.Id) duration three"
    Assert-Contains -Text $idfText -Pattern "  Holiday;" -Description "$($case.Id) Holiday day type"
    if ([regex]::Matches($idfText, '(?im)^\s*RunPeriodControl:SpecialDays\s*,').Count -ne 1) {
        throw "$($case.Id) must declare exactly one input-file special day"
    }
    if ($idfText -match '(?im)^\s*RunPeriodControl:DaylightSavingTime\s*,') {
        throw "$($case.Id) must not declare input-file daylight saving"
    }

    $weatherLines = Get-Content -LiteralPath $case.WeatherPath -Encoding UTF8
    $weatherText = $weatherLines -join "`n"
    $weatherRows = @($weatherLines | Select-Object -Skip 8 | Where-Object { -not [string]::IsNullOrWhiteSpace($_) })
    Assert-Contains -Text $weatherText -Pattern "HOLIDAYS/DAYLIGHT SAVINGS,$($case.LeapObserved),0,0,0" -Description "$($case.Id) leap policy without holidays or DST"
    Assert-Contains -Text $weatherText -Pattern "DATA PERIODS,1,1,Data,$($case.StartWeekday),1/1,1/3" -Description "$($case.Id) three-day independent data period"
    if ($weatherRows.Count -ne 72) {
        throw "$($case.Id) EPW must contain 72 hourly rows, found $($weatherRows.Count)"
    }
    $weatherKeys = [System.Collections.Generic.HashSet[string]]::new([System.StringComparer]::Ordinal)
    for ($index = 0; $index -lt 72; ++$index) {
        $fields = @($weatherRows[$index] -split ',')
        $expectedDay = [math]::Floor($index / 24) + 1
        $expectedHour = ($index % 24) + 1
        if ($fields.Count -ne 35 -or [int]$fields[0] -ne $case.Year -or [int]$fields[1] -ne 1 -or
            [int]$fields[2] -ne $expectedDay -or [int]$fields[3] -ne $expectedHour -or [int]$fields[4] -ne 60) {
            throw "Unexpected $($case.Id) EPW row at sample $index`: $($weatherRows[$index])"
        }
        $key = "$($fields[0])-$($fields[1])-$($fields[2])-$($fields[3])-$($fields[4])"
        if (-not $weatherKeys.Add($key)) {
            throw "Duplicate $($case.Id) EPW timestamp at sample $index`: $key"
        }
    }
}

$commonNormalized = $idfTexts[$Cases[0].Id].Replace("2017", "<YEAR>").Replace("  Sunday,", "  <START WEEKDAY>,")
$leapNormalized = $idfTexts[$Cases[1].Id].Replace("2016", "<YEAR>").Replace("  Friday,", "  <START WEEKDAY>,")
if ($commonNormalized -cne $leapNormalized) {
    throw "Duration-wrap IDFs differ outside the explicit year and start-weekday fields"
}

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Write-Host "Running common-year and leap-year duration-wrap special-day exact gate."
foreach ($case in $Cases) {
    Remove-RepoDirectory -Path $case.CaseOutputRoot
    $output = & $cargo.Source run -p ep_cli --quiet -- conformance time-weather-schedule-report $case.CasePath $OracleRoot $OutputRoot 2>&1
    if ($LASTEXITCODE -ne 0) {
        $output | ForEach-Object { Write-Host $_ }
        throw "Duration-wrap special-day case failed: $($case.Id)"
    }
    $text = $output -join "`n"
    Assert-Contains -Text $text -Pattern "id: $($case.Id)" -Description "$($case.Id) report id"
    Assert-Contains -Text $text -Pattern "status: pass" -Description "$($case.Id) report status"

    $summaryPath = Join-Path $case.CaseOutputRoot "compare\compare-summary.json"
    $reportPath = Join-Path $case.CaseOutputRoot "compare\compare-report.md"
    $oracleEsoPath = Join-Path $case.CaseOutputRoot "oracle\eplusout.eso"
    $oracleErrPath = Join-Path $case.CaseOutputRoot "oracle\eplusout.err"
    foreach ($path in @($summaryPath, $reportPath, $oracleEsoPath, $oracleErrPath)) {
        if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
            throw "Missing $($case.Id) comparison artifact: $path"
        }
    }

    $oracleErrText = Get-Content -LiteralPath $oracleErrPath -Raw -Encoding UTF8
    Assert-Contains -Text $oracleErrText -Pattern "EnergyPlus Completed Successfully-- 0 Warning; 0 Severe Errors;" -Description "$($case.Id) clean EnergyPlus completion"

    $summary = Get-Content -LiteralPath $summaryPath -Raw -Encoding UTF8 | ConvertFrom-Json
    if ($summary.status -ne "pass" -or $summary.conformance_claim -ne $true -or
        $summary.time_axis_samples -ne 72 -or $summary.series_count -ne 1 -or
        $summary.conformance_series_count -ne 1) {
        throw "$($case.Id) must be passing 72-sample conformance evidence"
    }
    $expectedLeap = $case.LeapObserved -eq "Yes"
    if ($summary.weather_calendar.weather_file_allows_leap_years -ne $expectedLeap -or
        $summary.weather_calendar.gregorian_calendar_days -ne 3 -or
        $summary.weather_calendar.weather_effective_calendar_days -ne 3 -or
        $summary.weather_calendar.leap_days_skipped -ne 0) {
        throw "Unexpected $($case.Id) weather calendar projection"
    }
    $specialDays = $summary.special_days
    if ($specialDays.weather_file_declared -ne 0 -or $specialDays.run_period_uses_weather_file -ne $false -or
        $specialDays.weather_file_resolved -ne 0 -or $specialDays.input_file_declared -ne 1 -or
        $specialDays.apply_weekend_rule -ne $false -or $specialDays.resolved_count -ne 1 -or
        $specialDays.hourly_samples -ne 48) {
        throw "Unexpected $($case.Id) duration-wrap special-day JSON state"
    }
    $resolved = @($specialDays.resolved)
    if ($resolved.Count -ne 1 -or $resolved[0].name -ne $ExpectedSpecialDayName -or
        $resolved[0].source -ne "input-file" -or $resolved[0].start_month -ne 12 -or
        $resolved[0].start_day -ne 31 -or $resolved[0].start_day_of_year -ne $case.StartDayOfYear -or
        $resolved[0].duration_days -ne 3 -or $resolved[0].day_type -ne "Holiday" -or
        $resolved[0].day_type_index -ne 8 -or $resolved[0].weekend_shift_days -ne 0) {
        throw "Unexpected $($case.Id) resolved duration-wrap projection"
    }

    $seriesMatches = @($summary.series | Where-Object {
        $_.key -eq "ENVIRONMENT" -and $_.variable -eq "Site Day Type Index"
    })
    if ($seriesMatches.Count -ne 1) {
        throw "Missing unique Site Day Type Index series for $($case.Id)"
    }
    $series = $seriesMatches[0]
    $expectedFirst = "env=$ExpectedEnvironment;day=1;month=1;date=1;dst=0;hour=1;start=0.00;end=60.00;day_type=Holiday"
    $expectedLast = "env=$ExpectedEnvironment;day=3;month=1;date=3;dst=0;hour=24;start=0.00;end=60.00;day_type=$($case.FinalDayType)"
    if ($series.expected_samples -ne 72 -or $series.observed_samples -ne 72 -or
        $series.compared_samples -ne 72 -or $series.timestamp_contract -ne "ordered-exact-unique" -or
        $series.timestamp_status -ne "pass" -or $series.timestamp_expected_unique -ne $true -or
        $series.timestamp_observed_unique -ne $true -or $series.timestamp_order_match -ne $true -or
        $series.expected_first_timestamp -ne $expectedFirst -or $series.observed_first_timestamp -ne $expectedFirst -or
        $series.expected_last_timestamp -ne $expectedLast -or $series.observed_last_timestamp -ne $expectedLast -or
        $series.max_abs_tolerance -ne 0.0 -or $series.max_rel_tolerance -ne 0.0 -or
        $series.max_rmse_tolerance -ne 0.0 -or $series.max_abs_delta -ne 0.0 -or
        $series.max_rel_delta -ne 0.0 -or $series.rmse_delta -ne 0.0 -or $series.status -ne "pass") {
        throw "Ordered exact Site Day Type Index contract failed for $($case.Id)"
    }

    $oracleEsoLines = Get-Content -LiteralPath $oracleEsoPath -Encoding UTF8
    $dictionary = $oracleEsoLines | Where-Object { $_ -match '^\d+,\d+,Environment,Site Day Type Index' } | Select-Object -First 1
    $dictionaryMatch = [regex]::Match([string]$dictionary, '^(\d+),')
    if (-not $dictionaryMatch.Success) {
        throw "Missing Site Day Type Index ESO dictionary entry for $($case.Id)"
    }
    $reportId = $dictionaryMatch.Groups[1].Value
    $valueRows = @($oracleEsoLines | Where-Object { $_ -match ('^' + $reportId + ',\s*[-+0-9.E]+\s*$') })
    $values = @($valueRows | ForEach-Object { [double](($_ -split ',', 2)[1].Trim()) })
    $timestampRows = @($oracleEsoLines | Where-Object { $_ -match '^2,\d+,\s*\d+,\s*\d+,\s*\d+,\s*\d+,' })
    if ($values.Count -ne 72 -or $timestampRows.Count -ne 72) {
        throw "Expected 72 EnergyPlus day-type values and timestamps for $($case.Id)"
    }
    for ($index = 0; $index -lt 72; ++$index) {
        $dayOffset = [math]::Floor($index / 24)
        $expectedValue = if ($index -lt 48) { 8.0 } else { $case.FinalDayTypeIndex }
        $expectedLabel = if ($index -lt 48) { "Holiday" } else { $case.FinalDayType }
        $expectedHour = ($index % 24) + 1
        if ($values[$index] -ne $expectedValue) {
            throw "Unexpected $($case.Id) EnergyPlus Site Day Type Index at sample $index`: $($values[$index])"
        }
        $timestampMatch = [regex]::Match(
            $timestampRows[$index],
            '^2,\s*(\d+),\s*(\d+),\s*(\d+),\s*(\d+),\s*(\d+),\s*([0-9.]+),\s*([0-9.]+),(.+)$'
        )
        if (-not $timestampMatch.Success -or [int]$timestampMatch.Groups[1].Value -ne ($dayOffset + 1) -or
            [int]$timestampMatch.Groups[2].Value -ne 1 -or
            [int]$timestampMatch.Groups[3].Value -ne ($dayOffset + 1) -or
            [int]$timestampMatch.Groups[4].Value -ne 0 -or
            [int]$timestampMatch.Groups[5].Value -ne $expectedHour -or
            [double]$timestampMatch.Groups[6].Value -ne 0.0 -or
            [double]$timestampMatch.Groups[7].Value -ne 60.0 -or
            $timestampMatch.Groups[8].Value.Trim() -cne $expectedLabel) {
            throw "Unexpected $($case.Id) EnergyPlus timestamp at sample $index`: $($timestampRows[$index])"
        }
    }

    $reportText = Get-Content -LiteralPath $reportPath -Raw -Encoding UTF8
    Assert-Contains -Text $reportText -Pattern "conformance_claim: true" -Description "$($case.Id) markdown conformance claim"
    Assert-Contains -Text $reportText -Pattern "status: pass" -Description "$($case.Id) markdown pass status"
    Assert-Contains -Text $reportText -Pattern "time_axis_samples: 72" -Description "$($case.Id) markdown time-axis count"
    Assert-Contains -Text $reportText -Pattern "weather_file_holidays_declared: 0" -Description "$($case.Id) markdown EPW holiday count"
    Assert-Contains -Text $reportText -Pattern "run_period_uses_weather_file_holidays: false" -Description "$($case.Id) markdown EPW holiday policy"
    Assert-Contains -Text $reportText -Pattern "input_file_special_days_declared: 1" -Description "$($case.Id) markdown input special-day count"
    Assert-Contains -Text $reportText -Pattern "special_day_weekend_rule: false" -Description "$($case.Id) markdown weekend rule"
    Assert-Contains -Text $reportText -Pattern "special_days_resolved: 1" -Description "$($case.Id) markdown resolved count"
    Assert-Contains -Text $reportText -Pattern "special_day_hourly_samples: 48" -Description "$($case.Id) markdown duration-wrap samples"
    Assert-Contains -Text $reportText -Pattern "special_day_resolved: $ExpectedSpecialDayName 12/31 duration=3 day_type=Holiday weekend_shift_days=0 source=input-file" -Description "$($case.Id) markdown resolved projection"
    Assert-Contains -Text $reportText -Pattern "Site Day Type Index" -Description "$($case.Id) markdown exact output row"
}

Write-Host "Common-year and leap-year duration-wrap special-day exact gate passed."
