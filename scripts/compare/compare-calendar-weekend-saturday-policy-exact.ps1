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
$WeatherRef = "data/conformance_cases/calendar_special_day_weekend_saturday_enabled_hourly_exact_001/calendar_special_day_weekend_saturday_hourly_exact.epw"
$WeatherPath = Join-Path $RepoRoot ($WeatherRef -replace '/', '\')
$ExpectedEnvironment = "SPECIAL DAY WEEKEND SATURDAY RUN PERIOD"
$Cases = @(
    [pscustomobject]@{
        Id = "calendar_special_day_weekend_saturday_enabled_hourly_exact_001"
        IdfName = "calendar_special_day_weekend_saturday_enabled_hourly_exact.idf"
        WeekendPolicy = $true
        PolicyText = "Yes, !- Apply Weekend Holiday Rule"
        DayValues = @(7.0, 1.0, 8.0)
        DayLabels = @("Saturday", "Sunday", "Holiday")
        StartDay = 29
        StartDayOfYear = 60
        ShiftDays = 2
        ExpectedFirstTimestamp = "env=$ExpectedEnvironment;day=1;month=2;date=27;dst=0;hour=1;start=0.00;end=60.00;day_type=Saturday"
        ExpectedLastTimestamp = "env=$ExpectedEnvironment;day=3;month=2;date=29;dst=0;hour=24;start=0.00;end=60.00;day_type=Holiday"
    },
    [pscustomobject]@{
        Id = "calendar_special_day_weekend_saturday_disabled_hourly_exact_001"
        IdfName = "calendar_special_day_weekend_saturday_disabled_hourly_exact.idf"
        WeekendPolicy = $false
        PolicyText = "No,  !- Apply Weekend Holiday Rule"
        DayValues = @(8.0, 1.0, 2.0)
        DayLabels = @("Holiday", "Sunday", "Monday")
        StartDay = 27
        StartDayOfYear = 58
        ShiftDays = 0
        ExpectedFirstTimestamp = "env=$ExpectedEnvironment;day=1;month=2;date=27;dst=0;hour=1;start=0.00;end=60.00;day_type=Holiday"
        ExpectedLastTimestamp = "env=$ExpectedEnvironment;day=3;month=2;date=29;dst=0;hour=24;start=0.00;end=60.00;day_type=Monday"
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
    $case | Add-Member -NotePropertyName IdfRef -NotePropertyValue "data/conformance_cases/$($case.Id)/$($case.IdfName)"
    $case | Add-Member -NotePropertyName OutputRoot -NotePropertyValue (Join-Path $OutputRoot $case.Id)
}

foreach ($path in @(
    (Join-Path $OracleRoot "energyplus.exe"),
    (Join-Path $OracleRoot "ConvertInputFormat.exe"),
    $WeatherPath
) + @($Cases | ForEach-Object { @($_.CasePath, $_.IdfPath) })) {
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Missing required fixed-Saturday weekend policy conformance file: $path"
    }
}

$weatherLines = Get-Content -LiteralPath $WeatherPath -Encoding UTF8
$weatherText = $weatherLines -join "`n"
$weatherRows = @($weatherLines | Select-Object -Skip 8 | Where-Object { -not [string]::IsNullOrWhiteSpace($_) })
Assert-Contains -Text $weatherText -Pattern "HOLIDAYS/DAYLIGHT SAVINGS,Yes,0,0,0" -Description "shared EPW without holidays or DST"
if ($weatherRows.Count -ne 72) {
    throw "Fixed-Saturday weekend policy EPW must contain 72 hourly rows, found $($weatherRows.Count)"
}
for ($index = 0; $index -lt 72; ++$index) {
    $fields = @($weatherRows[$index] -split ',')
    if ($fields.Count -lt 5) {
        throw "Malformed shared EPW row at sample $index"
    }
    $dayOffset = [math]::Floor($index / 24)
    $expectedDate = 27 + $dayOffset
    $expectedHour = ($index % 24) + 1
    if ([int]$fields[0] -ne 2016 -or [int]$fields[1] -ne 2 -or
        [int]$fields[2] -ne $expectedDate -or [int]$fields[3] -ne $expectedHour -or
        [int]$fields[4] -ne 60) {
        throw "Unexpected shared EPW date/hour at sample $index`: $($weatherRows[$index])"
    }
}

$idfTexts = @{}
foreach ($case in $Cases) {
    $caseText = Get-Content -LiteralPath $case.CasePath -Raw -Encoding UTF8
    $idfText = Get-Content -LiteralPath $case.IdfPath -Raw -Encoding UTF8
    $idfTexts[$case.Id] = $idfText
    Assert-Contains -Text $caseText -Pattern "timestamp_contract = `"ordered-exact-unique`"" -Description "$($case.Id) ordered timestamp contract"
    Assert-Contains -Text $caseText -Pattern "abs_tol = 0.0" -Description "$($case.Id) zero absolute tolerance"
    Assert-Contains -Text $caseText -Pattern "rmse_tol = 0.0" -Description "$($case.Id) zero RMSE tolerance"
    Assert-Contains -Text $caseText -Pattern "idf = `"$($case.IdfRef)`"" -Description "$($case.Id) manifest input.idf attribution"
    Assert-Contains -Text $caseText -Pattern "weather = `"$WeatherRef`"" -Description "$($case.Id) shared EPW attribution"
    Assert-Contains -Text $caseText -Pattern 'script = "scripts/dev.cmd compare-calendar-weekend-saturday-policy-exact"' -Description "$($case.Id) manifest gate attribution"
    Assert-Contains -Text $caseText -Pattern "blocking = true" -Description "$($case.Id) manifest blocking flag"
    Assert-Contains -Text $idfText -Pattern $case.PolicyText -Description "$($case.Id) explicit weekend policy"
    Assert-Contains -Text $idfText -Pattern "No,  !- Use Weather File Holidays and Special Days" -Description "$($case.Id) disabled EPW holiday policy"
    Assert-Contains -Text $idfText -Pattern "No,  !- Use Weather File Daylight Saving Period" -Description "$($case.Id) disabled DST policy"
    Assert-Contains -Text $idfText -Pattern "Saturday Holiday" -Description "$($case.Id) fixed Saturday special day"
    Assert-Contains -Text $idfText -Pattern "2/27," -Description "$($case.Id) fixed February 27 date"
    Assert-Contains -Text $idfText -Pattern "  1," -Description "$($case.Id) duration one day"
    Assert-Contains -Text $idfText -Pattern "  Holiday;" -Description "$($case.Id) Holiday day type"
    if ([regex]::Matches($idfText, '(?im)^\s*RunPeriodControl:SpecialDays\s*,').Count -ne 1) {
        throw "$($case.Id) must declare exactly one input-file special day"
    }
    if ($idfText -match '(?im)^\s*RunPeriodControl:DaylightSavingTime\s*,') {
        throw "$($case.Id) must not declare an input-file daylight-saving period"
    }
}

$enabledNormalized = $idfTexts[$Cases[0].Id].Replace($Cases[0].PolicyText, "<WEEKEND HOLIDAY POLICY>")
$disabledNormalized = $idfTexts[$Cases[1].Id].Replace($Cases[1].PolicyText, "<WEEKEND HOLIDAY POLICY>")
if ($enabledNormalized -cne $disabledNormalized) {
    throw "Paired fixed-Saturday IDFs differ outside the explicit weekend policy"
}

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Write-Host "Running paired fixed-Saturday weekend holiday policy exact gate."
foreach ($case in $Cases) {
    Remove-RepoDirectory -Path $case.OutputRoot
    $output = & $cargo.Source run -p ep_cli --quiet -- conformance time-weather-schedule-report $case.CasePath $OracleRoot $OutputRoot 2>&1
    if ($LASTEXITCODE -ne 0) {
        $output | ForEach-Object { Write-Host $_ }
        throw "Fixed-Saturday weekend policy case failed: $($case.Id)"
    }
    $text = $output -join "`n"
    Assert-Contains -Text $text -Pattern "id: $($case.Id)" -Description "$($case.Id) report id"
    Assert-Contains -Text $text -Pattern "status: pass" -Description "$($case.Id) report status"

    $summaryPath = Join-Path $case.OutputRoot "compare\compare-summary.json"
    $reportPath = Join-Path $case.OutputRoot "compare\compare-report.md"
    $oracleEsoPath = Join-Path $case.OutputRoot "oracle\eplusout.eso"
    foreach ($path in @($summaryPath, $reportPath, $oracleEsoPath)) {
        if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
            throw "Missing $($case.Id) comparison artifact: $path"
        }
    }

    $summary = Get-Content -LiteralPath $summaryPath -Raw -Encoding UTF8 | ConvertFrom-Json
    if ($summary.status -ne "pass" -or $summary.conformance_claim -ne $true -or $summary.time_axis_samples -ne 72) {
        throw "$($case.Id) must be passing 72-sample conformance evidence"
    }
    $specialDays = $summary.special_days
    if ($specialDays.weather_file_declared -ne 0 -or $specialDays.run_period_uses_weather_file -ne $false -or
        $specialDays.weather_file_resolved -ne 0 -or $specialDays.input_file_declared -ne 1 -or
        $specialDays.apply_weekend_rule -ne $case.WeekendPolicy -or $specialDays.resolved_count -ne 1 -or
        $specialDays.hourly_samples -ne 24) {
        throw "Unexpected $($case.Id) fixed-Saturday JSON state"
    }
    $resolved = @($specialDays.resolved)
    if ($resolved.Count -ne 1 -or $resolved[0].name -ne "SATURDAY HOLIDAY" -or
        $resolved[0].source -ne "input-file" -or $resolved[0].start_month -ne 2 -or
        $resolved[0].start_day -ne $case.StartDay -or $resolved[0].start_day_of_year -ne $case.StartDayOfYear -or
        $resolved[0].duration_days -ne 1 -or $resolved[0].day_type -ne "Holiday" -or
        $resolved[0].day_type_index -ne 8 -or $resolved[0].weekend_shift_days -ne $case.ShiftDays) {
        throw "Unexpected $($case.Id) resolved fixed-Saturday projection"
    }

    $series = @($summary.series | Where-Object { $_.key -eq "ENVIRONMENT" -and $_.variable -eq "Site Day Type Index" })
    if ($series.Count -ne 1) {
        throw "Missing unique Site Day Type Index series for $($case.Id)"
    }
    $series = $series[0]
    if ($series.expected_samples -ne 72 -or $series.observed_samples -ne 72 -or $series.compared_samples -ne 72 -or
        $series.timestamp_contract -ne "ordered-exact-unique" -or $series.timestamp_status -ne "pass" -or
        $series.timestamp_expected_unique -ne $true -or $series.timestamp_observed_unique -ne $true -or
        $series.timestamp_order_match -ne $true -or
        $series.expected_first_timestamp -ne $case.ExpectedFirstTimestamp -or $series.observed_first_timestamp -ne $case.ExpectedFirstTimestamp -or
        $series.expected_last_timestamp -ne $case.ExpectedLastTimestamp -or $series.observed_last_timestamp -ne $case.ExpectedLastTimestamp -or
        $series.max_abs_tolerance -ne 0.0 -or $series.max_rel_tolerance -ne 0.0 -or $series.max_rmse_tolerance -ne 0.0 -or
        $series.max_abs_delta -ne 0.0 -or $series.rmse_delta -ne 0.0 -or $series.max_rel_delta -ne 0.0 -or
        $series.status -ne "pass") {
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
        throw "Expected 72 oracle day-type values and timestamps for $($case.Id)"
    }
    for ($index = 0; $index -lt 72; ++$index) {
        $dayOffset = [math]::Floor($index / 24)
        $expectedValue = $case.DayValues[$dayOffset]
        $expectedLabel = $case.DayLabels[$dayOffset]
        $expectedDate = 27 + $dayOffset
        $expectedHour = ($index % 24) + 1
        if ($values[$index] -ne $expectedValue) {
            throw "Unexpected $($case.Id) oracle Site Day Type Index at sample $index`: $($values[$index])"
        }
        $timestampMatch = [regex]::Match(
            $timestampRows[$index],
            '^2,\s*(\d+),\s*(\d+),\s*(\d+),\s*(\d+),\s*(\d+),\s*([0-9.]+),\s*([0-9.]+),(.+)$'
        )
        if (-not $timestampMatch.Success -or [int]$timestampMatch.Groups[1].Value -ne ($dayOffset + 1) -or
            [int]$timestampMatch.Groups[2].Value -ne 2 -or [int]$timestampMatch.Groups[3].Value -ne $expectedDate -or
            [int]$timestampMatch.Groups[4].Value -ne 0 -or [int]$timestampMatch.Groups[5].Value -ne $expectedHour -or
            [double]$timestampMatch.Groups[6].Value -ne 0.0 -or [double]$timestampMatch.Groups[7].Value -ne 60.0 -or
            $timestampMatch.Groups[8].Value.Trim() -cne $expectedLabel) {
            throw "Unexpected $($case.Id) oracle timestamp at sample $index`: $($timestampRows[$index])"
        }
    }

    $reportText = Get-Content -LiteralPath $reportPath -Raw -Encoding UTF8
    Assert-Contains -Text $reportText -Pattern "weather_file_holidays_declared: 0" -Description "$($case.Id) markdown weather-file holiday count"
    Assert-Contains -Text $reportText -Pattern "run_period_uses_weather_file_holidays: false" -Description "$($case.Id) markdown EPW holiday policy"
    Assert-Contains -Text $reportText -Pattern "weather_file_holidays_resolved: 0" -Description "$($case.Id) markdown resolved weather-file holiday count"
    Assert-Contains -Text $reportText -Pattern "input_file_special_days_declared: 1" -Description "$($case.Id) markdown input special-day count"
    Assert-Contains -Text $reportText -Pattern "special_day_weekend_rule: $($case.WeekendPolicy.ToString().ToLowerInvariant())" -Description "$($case.Id) markdown weekend policy"
    Assert-Contains -Text $reportText -Pattern "special_days_resolved: 1" -Description "$($case.Id) markdown resolved count"
    Assert-Contains -Text $reportText -Pattern "special_day_hourly_samples: 24" -Description "$($case.Id) markdown active samples"
    Assert-Contains -Text $reportText -Pattern "special_day_resolved: SATURDAY HOLIDAY 2/$($case.StartDay) duration=1 day_type=Holiday weekend_shift_days=$($case.ShiftDays) source=input-file" -Description "$($case.Id) markdown resolved projection"
}

Write-Host "Paired fixed-Saturday weekend holiday policy exact gate passed."
