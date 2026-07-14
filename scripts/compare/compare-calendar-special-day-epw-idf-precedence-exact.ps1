[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$CaseId = "calendar_special_day_epw_idf_precedence_hourly_exact_001"
$CaseRoot = Join-Path $RepoRoot "data\conformance_cases\$CaseId"
$CasePath = Join-Path $CaseRoot "case.toml"
$IdfName = "calendar_special_day_epw_idf_precedence_hourly_exact.idf"
$IdfPath = Join-Path $CaseRoot $IdfName
$IdfRef = "data/conformance_cases/$CaseId/$IdfName"
$BaseIdfPath = Join-Path $RepoRoot "data\conformance_cases\calendar_epw_holiday_fixed_date_enabled_hourly_exact_001\calendar_epw_holiday_fixed_date_enabled_hourly_exact.idf"
$WeatherRef = "data/conformance_cases/calendar_epw_holiday_fixed_date_enabled_hourly_exact_001/calendar_epw_holiday_fixed_date_hourly_exact.epw"
$WeatherPath = Join-Path $RepoRoot ($WeatherRef -replace '/', '\')
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$OutputRoot = Join-Path $RepoRoot ".runtime\time-weather-schedule-conformance\26.1.0"
$CaseOutputRoot = Join-Path $OutputRoot $CaseId
$ExpectedEnvironment = "EPW HOLIDAY FIXED DATE RUN PERIOD"
$WarningNonClaim = "EnergyPlus-versus-Rust warning text, count, repetition, and diagnostics parity"

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

foreach ($path in @(
    (Join-Path $OracleRoot "energyplus.exe"),
    (Join-Path $OracleRoot "ConvertInputFormat.exe"),
    $CasePath,
    $IdfPath,
    $BaseIdfPath,
    $WeatherPath
)) {
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Missing required EPW-versus-IDF precedence conformance file: $path"
    }
}
$localWeather = @(Get-ChildItem -LiteralPath $CaseRoot -Filter "*.epw" -File)
if ($localWeather.Count -ne 0) {
    throw "$CaseId must reuse the isolated EPW holiday fixture, found $($localWeather.Count) local EPW files"
}

$caseText = Get-Content -LiteralPath $CasePath -Raw -Encoding UTF8
$idfText = Get-Content -LiteralPath $IdfPath -Raw -Encoding UTF8
$baseIdfText = Get-Content -LiteralPath $BaseIdfPath -Raw -Encoding UTF8
$weatherLines = Get-Content -LiteralPath $WeatherPath -Encoding UTF8
$weatherText = $weatherLines -join "`n"
$weatherRows = @($weatherLines | Select-Object -Skip 8 | Where-Object { -not [string]::IsNullOrWhiteSpace($_) })

Assert-Contains -Text $caseText -Pattern 'tier = "A"' -Description "Tier A attribution"
Assert-Contains -Text $caseText -Pattern 'comparison_class = "conformance"' -Description "conformance comparison class"
Assert-Contains -Text $caseText -Pattern 'conformance_claim = true' -Description "conformance claim"
Assert-Contains -Text $caseText -Pattern 'timestamp_contract = "ordered-exact-unique"' -Description "ordered timestamp contract"
Assert-Contains -Text $caseText -Pattern 'abs_tol = 0.0' -Description "zero absolute tolerance"
Assert-Contains -Text $caseText -Pattern 'rmse_tol = 0.0' -Description "zero RMSE tolerance"
Assert-Contains -Text $caseText -Pattern "idf = `"$IdfRef`"" -Description "manifest input.idf attribution"
Assert-Contains -Text $caseText -Pattern "weather = `"$WeatherRef`"" -Description "shared manifest input.weather attribution"
Assert-Contains -Text $caseText -Pattern 'script = "scripts/dev.cmd compare-calendar-special-day-epw-idf-precedence-exact"' -Description "blocking-gate attribution"
Assert-Contains -Text $caseText -Pattern 'blocking = true' -Description "blocking flag"
Assert-Contains -Text $caseText -Pattern "24 Sunday=1, 24 CustomDay1=11, and 24 Tuesday=3" -Description "narrow exact value claim"
Assert-Contains -Text $caseText -Pattern "weather-file-then-input-file resolved source order for this one exact overlap" -Description "narrow resolved-order claim"
Assert-Contains -Text $caseText -Pattern $WarningNonClaim -Description "Rust warning parity nonclaim"

Assert-Contains -Text $weatherText -Pattern "HOLIDAYS/DAYLIGHT SAVINGS,Yes,0,0,1,EPW Leap Holiday,2/29" -Description "one fixed EPW 2/29 holiday declaration"
Assert-Contains -Text $weatherText -Pattern "DATA PERIODS,1,1,Data,Sunday,2/28,3/1" -Description "three-day EPW period"
if ($weatherRows.Count -ne 72) {
    throw "Shared EPW holiday fixture must contain exactly 72 hourly rows, found $($weatherRows.Count)"
}
$calendarDays = @(
    [pscustomobject]@{ Month = 2; Day = 28 },
    [pscustomobject]@{ Month = 2; Day = 29 },
    [pscustomobject]@{ Month = 3; Day = 1 }
)
$weatherKeys = [System.Collections.Generic.HashSet[string]]::new([System.StringComparer]::Ordinal)
for ($index = 0; $index -lt 72; ++$index) {
    $fields = @($weatherRows[$index] -split ',')
    $dayOffset = [math]::Floor($index / 24)
    $expectedDate = $calendarDays[$dayOffset]
    $expectedHour = ($index % 24) + 1
    if ($fields.Count -ne 35 -or [int]$fields[0] -ne 2016 -or
        [int]$fields[1] -ne $expectedDate.Month -or [int]$fields[2] -ne $expectedDate.Day -or
        [int]$fields[3] -ne $expectedHour -or [int]$fields[4] -ne 60) {
        throw "Unexpected shared EPW row at sample $index`: $($weatherRows[$index])"
    }
    $key = "$($fields[0])-$($fields[1])-$($fields[2])-$($fields[3])-$($fields[4])"
    if (-not $weatherKeys.Add($key)) {
        throw "Duplicate shared EPW timestamp at sample $index`: $key"
    }
}

$runPeriods = [regex]::Matches($idfText, '(?im)^\s*RunPeriod\s*,')
if ($runPeriods.Count -ne 1) {
    throw "Fixture must declare exactly one RunPeriod, found $($runPeriods.Count)"
}
$runPeriodPrefix = '(?ims)^\s*RunPeriod\s*,\s*EPW Holiday Fixed Date Run Period\s*,\s*2\s*,\s*28\s*,\s*2016\s*,\s*3\s*,\s*1\s*,\s*2016\s*,\s*Sunday\s*,\s*Yes\s*,'
if (-not [regex]::IsMatch($idfText, $runPeriodPrefix)) {
    throw "RunPeriod must be exactly 2016-02-28 through 2016-03-01, start on Sunday, and enable weather-file holidays"
}
Assert-Contains -Text $idfText -Pattern "Yes, !- Use Weather File Holidays and Special Days" -Description "enabled weather-file holiday policy"
Assert-Contains -Text $idfText -Pattern "No,  !- Use Weather File Daylight Saving Period" -Description "disabled weather-file DST policy"
Assert-Contains -Text $idfText -Pattern "No,  !- Apply Weekend Holiday Rule" -Description "disabled weekend rule"
Assert-Contains -Text $idfText -Pattern "No,  !- Use Weather File Rain Indicators" -Description "disabled rain policy"
Assert-Contains -Text $idfText -Pattern "No,  !- Use Weather File Snow Indicators" -Description "disabled snow policy"
Assert-Contains -Text $idfText -Pattern "No;  !- Treat Weather as Actual" -Description "disabled actual-weather policy"
Assert-Contains -Text $idfText -Pattern "Site Day Type Index" -Description "day-type output request"

$specialDayPattern = '(?ims)^\s*RunPeriodControl:SpecialDays\s*,\s*(?<name>[^,!;]+?)\s*,\s*(?<date>[^,!;]+?)\s*,\s*(?<duration>[^,!;]+?)\s*,\s*(?<daytype>[^,!;]+?)\s*;'
$specialDays = [regex]::Matches($idfText, $specialDayPattern)
if ($specialDays.Count -ne 1) {
    throw "Fixture must declare exactly one input-file special day, found $($specialDays.Count)"
}
$declared = $specialDays[0]
if ($declared.Groups['name'].Value.Trim() -cne "Input Custom Day Definition" -or
    $declared.Groups['date'].Value.Trim() -cne "2/29" -or
    $declared.Groups['duration'].Value.Trim() -cne "1" -or
    $declared.Groups['daytype'].Value.Trim() -cne "CustomDay1") {
    throw "Input-file special day must be Input Custom Day Definition, 2/29, duration 1, CustomDay1"
}
$strippedIdf = [regex]::Replace($idfText, $specialDayPattern, '')
$normalizedStrippedIdf = [regex]::Replace($strippedIdf, '\s+', '')
$normalizedBaseIdf = [regex]::Replace($baseIdfText, '\s+', '')
if ($normalizedStrippedIdf -cne $normalizedBaseIdf) {
    throw "Precedence fixture must differ from the existing EPW-holiday-enabled fixture only by its one SpecialDays object"
}
Write-Host "OK precedence fixture adds only one SpecialDays object to the EPW-holiday-enabled fixture."

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Remove-RepoDirectory -Path $CaseOutputRoot
Write-Host "Running fixed-date EPW-versus-IDF special-day precedence exact gate."
$output = & $cargo.Source run -p ep_cli --quiet -- conformance time-weather-schedule-report $CasePath $OracleRoot $OutputRoot 2>&1
if ($LASTEXITCODE -ne 0) {
    $output | ForEach-Object { Write-Host $_ }
    throw "EPW-versus-IDF special-day precedence case failed."
}
$text = $output -join "`n"
Assert-Contains -Text $text -Pattern "id: $CaseId" -Description "report id"
Assert-Contains -Text $text -Pattern "status: pass" -Description "report status"

$summaryPath = Join-Path $CaseOutputRoot "compare\compare-summary.json"
$reportPath = Join-Path $CaseOutputRoot "compare\compare-report.md"
$oracleEsoPath = Join-Path $CaseOutputRoot "oracle\eplusout.eso"
$oracleErrPath = Join-Path $CaseOutputRoot "oracle\eplusout.err"
foreach ($path in @($summaryPath, $reportPath, $oracleEsoPath, $oracleErrPath)) {
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Missing EPW-versus-IDF precedence comparison artifact: $path"
    }
}

$summary = Get-Content -LiteralPath $summaryPath -Raw -Encoding UTF8 | ConvertFrom-Json
if ($summary.status -ne "pass" -or $summary.conformance_claim -ne $true -or
    $summary.time_axis_samples -ne 72 -or $summary.series_count -ne 1 -or
    $summary.conformance_series_count -ne 1) {
    throw "Case must be passing one-series, 72-sample conformance evidence"
}

$summarySpecialDays = $summary.special_days
if ($summarySpecialDays.weather_file_declared -ne 1 -or
    $summarySpecialDays.run_period_uses_weather_file -ne $true -or
    $summarySpecialDays.weather_file_resolved -ne 1 -or
    $summarySpecialDays.input_file_declared -ne 1 -or
    $summarySpecialDays.apply_weekend_rule -ne $false -or
    $summarySpecialDays.resolved_count -ne 2 -or
    $summarySpecialDays.hourly_samples -ne 24) {
    throw "Unexpected EPW-versus-IDF special-day JSON state"
}
$resolved = @($summarySpecialDays.resolved)
if ($resolved.Count -ne 2) {
    throw "Expected exactly two resolved special-day sources"
}
$expectedResolved = @(
    [pscustomobject]@{ Name = "EPW LEAP HOLIDAY"; Source = "weather-file"; DayType = "Sunday"; DayTypeIndex = 1 },
    [pscustomobject]@{ Name = "INPUT CUSTOM DAY DEFINITION"; Source = "input-file"; DayType = "CustomDay1"; DayTypeIndex = 11 }
)
for ($index = 0; $index -lt 2; ++$index) {
    $actual = $resolved[$index]
    $expected = $expectedResolved[$index]
    if ($actual.name -cne $expected.Name -or $actual.source -cne $expected.Source -or
        $actual.start_month -ne 2 -or $actual.start_day -ne 29 -or
        $actual.start_day_of_year -ne 60 -or $actual.duration_days -ne 1 -or
        $actual.day_type -cne $expected.DayType -or $actual.day_type_index -ne $expected.DayTypeIndex -or
        $actual.weekend_shift_days -ne 0) {
        throw "Unexpected resolved special-day source-order entry $index"
    }
}

$seriesMatches = @($summary.series | Where-Object {
    $_.key -eq "ENVIRONMENT" -and $_.variable -eq "Site Day Type Index"
})
if ($seriesMatches.Count -ne 1) {
    throw "Missing unique Site Day Type Index series"
}
$series = $seriesMatches[0]
$expectedFirst = "env=$ExpectedEnvironment;day=1;month=2;date=28;dst=0;hour=1;start=0.00;end=60.00;day_type=Sunday"
$expectedLast = "env=$ExpectedEnvironment;day=3;month=3;date=1;dst=0;hour=24;start=0.00;end=60.00;day_type=Tuesday"
if ($series.expected_samples -ne 72 -or $series.observed_samples -ne 72 -or
    $series.compared_samples -ne 72 -or $series.timestamp_contract -ne "ordered-exact-unique" -or
    $series.timestamp_status -ne "pass" -or $series.timestamp_expected_unique -ne $true -or
    $series.timestamp_observed_unique -ne $true -or $series.timestamp_order_match -ne $true -or
    $series.expected_first_timestamp -ne $expectedFirst -or $series.observed_first_timestamp -ne $expectedFirst -or
    $series.expected_last_timestamp -ne $expectedLast -or $series.observed_last_timestamp -ne $expectedLast -or
    $series.max_abs_tolerance -ne 0.0 -or $series.max_rel_tolerance -ne 0.0 -or
    $series.max_rmse_tolerance -ne 0.0 -or $series.max_abs_delta -ne 0.0 -or
    $series.max_rel_delta -ne 0.0 -or $series.rmse_delta -ne 0.0 -or
    $null -ne $series.first_divergence -or $null -ne $series.first_timestamp_divergence -or
    $series.status -ne "pass") {
    throw "Ordered exact Site Day Type Index contract failed"
}

$oracleEsoLines = Get-Content -LiteralPath $oracleEsoPath -Encoding UTF8
$dictionaryRows = @($oracleEsoLines | Where-Object { $_ -match '^\d+,\d+,Environment,Site Day Type Index' })
if ($dictionaryRows.Count -ne 1) {
    throw "Expected one Site Day Type Index ESO dictionary entry, found $($dictionaryRows.Count)"
}
$dictionaryMatch = [regex]::Match($dictionaryRows[0], '^(\d+),')
if (-not $dictionaryMatch.Success) {
    throw "Malformed Site Day Type Index ESO dictionary entry"
}
$reportId = $dictionaryMatch.Groups[1].Value
$valueRows = @($oracleEsoLines | Where-Object { $_ -match ('^' + $reportId + ',\s*[-+0-9.E]+\s*$') })
$values = @($valueRows | ForEach-Object { [double](($_ -split ',', 2)[1].Trim()) })
$timestampRows = @($oracleEsoLines | Where-Object { $_ -match '^2,\d+,\s*\d+,\s*\d+,\s*\d+,\s*\d+,' })
if ($values.Count -ne 72 -or $timestampRows.Count -ne 72) {
    throw "Expected 72 EnergyPlus day-type values and timestamps"
}
for ($index = 0; $index -lt 72; ++$index) {
    $dayOffset = [math]::Floor($index / 24)
    $expectedDate = $calendarDays[$dayOffset]
    $expectedValue = if ($dayOffset -eq 0) { 1.0 } elseif ($dayOffset -eq 1) { 11.0 } else { 3.0 }
    $expectedLabel = if ($dayOffset -eq 0) { "Sunday" } elseif ($dayOffset -eq 1) { "CustomDay1" } else { "Tuesday" }
    $expectedHour = ($index % 24) + 1
    if ($values[$index] -ne $expectedValue) {
        throw "Unexpected EnergyPlus Site Day Type Index at sample $index`: $($values[$index])"
    }
    $timestampMatch = [regex]::Match(
        $timestampRows[$index],
        '^2,\s*(\d+),\s*(\d+),\s*(\d+),\s*(\d+),\s*(\d+),\s*([0-9.]+),\s*([0-9.]+),(.+)$'
    )
    if (-not $timestampMatch.Success -or [int]$timestampMatch.Groups[1].Value -ne ($dayOffset + 1) -or
        [int]$timestampMatch.Groups[2].Value -ne $expectedDate.Month -or
        [int]$timestampMatch.Groups[3].Value -ne $expectedDate.Day -or
        [int]$timestampMatch.Groups[4].Value -ne 0 -or
        [int]$timestampMatch.Groups[5].Value -ne $expectedHour -or
        [double]$timestampMatch.Groups[6].Value -ne 0.0 -or
        [double]$timestampMatch.Groups[7].Value -ne 60.0 -or
        $timestampMatch.Groups[8].Value.Trim() -cne $expectedLabel) {
        throw "Unexpected EnergyPlus timestamp at sample $index`: $($timestampRows[$index])"
    }
}

# These are EnergyPlus-only oracle diagnostics. Rust warning text/count/repetition parity is explicitly not claimed.
$oracleErrText = Get-Content -LiteralPath $oracleErrPath -Raw -Encoding UTF8
Assert-Contains -Text $oracleErrText -Pattern "EnergyPlus Completed Successfully-- 3 Warning; 0 Severe Errors;" -Description "EnergyPlus warning/severe completion count"
$warningLineCount = [regex]::Matches($oracleErrText, '(?m)^\s*\*\* Warning \*\*').Count
$severeLineCount = [regex]::Matches($oracleErrText, '(?m)^\s*\*\* Severe\s+\*\*').Count
$inputOverwrite = "SetSpecialDayDates: Special Day definition (INPUT CUSTOM DAY DEFINITION) is overwriting previously entered special day period"
$weatherContinuation = "...This could be caused by definitions on the Weather File."
$duplicateContinuation = "...This could be caused by duplicate definitions in the Input File."
if ($warningLineCount -ne 3 -or $severeLineCount -ne 0 -or
    [regex]::Matches($oracleErrText, [regex]::Escape($inputOverwrite)).Count -ne 3 -or
    [regex]::Matches($oracleErrText, [regex]::Escape($weatherContinuation)).Count -ne 3 -or
    [regex]::Matches($oracleErrText, [regex]::Escape($duplicateContinuation)).Count -ne 3) {
    throw "Unexpected EnergyPlus-only EPW-versus-IDF overwrite diagnostics"
}

$reportText = Get-Content -LiteralPath $reportPath -Raw -Encoding UTF8
Assert-Contains -Text $reportText -Pattern "conformance_claim: true" -Description "markdown conformance claim"
Assert-Contains -Text $reportText -Pattern "status: pass" -Description "markdown pass status"
Assert-Contains -Text $reportText -Pattern "time_axis_samples: 72" -Description "markdown time-axis count"
Assert-Contains -Text $reportText -Pattern "weather_file_holidays_declared: 1" -Description "markdown EPW declaration count"
Assert-Contains -Text $reportText -Pattern "run_period_uses_weather_file_holidays: true" -Description "markdown EPW use policy"
Assert-Contains -Text $reportText -Pattern "weather_file_holidays_resolved: 1" -Description "markdown EPW resolved count"
Assert-Contains -Text $reportText -Pattern "input_file_special_days_declared: 1" -Description "markdown input declaration count"
Assert-Contains -Text $reportText -Pattern "special_day_weekend_rule: false" -Description "markdown weekend rule"
Assert-Contains -Text $reportText -Pattern "special_days_resolved: 2" -Description "markdown combined resolved count"
Assert-Contains -Text $reportText -Pattern "special_day_hourly_samples: 24" -Description "markdown winning special-day samples"
$weatherResolvedLine = "special_day_resolved: EPW LEAP HOLIDAY 2/29 duration=1 day_type=Sunday weekend_shift_days=0 source=weather-file"
$inputResolvedLine = "special_day_resolved: INPUT CUSTOM DAY DEFINITION 2/29 duration=1 day_type=CustomDay1 weekend_shift_days=0 source=input-file"
Assert-Contains -Text $reportText -Pattern $weatherResolvedLine -Description "markdown weather-file resolved entry"
Assert-Contains -Text $reportText -Pattern $inputResolvedLine -Description "markdown input-file resolved entry"
if ($reportText.IndexOf($weatherResolvedLine, [System.StringComparison]::Ordinal) -ge
    $reportText.IndexOf($inputResolvedLine, [System.StringComparison]::Ordinal)) {
    throw "Markdown resolved entries must preserve weather-file-then-input-file source order"
}
Assert-Contains -Text $reportText -Pattern "Site Day Type Index" -Description "markdown exact output row"

Write-Host "Fixed-date EPW-versus-IDF special-day precedence exact gate passed. EnergyPlus warning diagnostics were checked without claiming Rust warning parity."
