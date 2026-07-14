[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$CaseId = "calendar_special_day_fixed_date_hourly_exact_001"
$CaseRoot = Join-Path $RepoRoot "data\conformance_cases\$CaseId"
$CasePath = Join-Path $CaseRoot "case.toml"
$IdfPath = Join-Path $CaseRoot "calendar_special_day_fixed_date_hourly_exact.idf"
$WeatherPath = Join-Path $CaseRoot "calendar_special_day_fixed_date_hourly_exact.epw"
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

foreach ($path in @(
    (Join-Path $OracleRoot "energyplus.exe"),
    (Join-Path $OracleRoot "ConvertInputFormat.exe"),
    $CasePath,
    $IdfPath,
    $WeatherPath
)) {
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Missing required fixed-date special-day conformance file: $path"
    }
}

$caseText = Get-Content -LiteralPath $CasePath -Raw -Encoding UTF8
$idfText = Get-Content -LiteralPath $IdfPath -Raw -Encoding UTF8
$weatherLines = Get-Content -LiteralPath $WeatherPath -Encoding UTF8
$weatherText = $weatherLines -join "`n"
$weatherRows = @($weatherLines | Select-Object -Skip 8 | Where-Object { -not [string]::IsNullOrWhiteSpace($_) })

Assert-Contains -Text $caseText -Pattern 'timestamp_contract = "ordered-exact-unique"' -Description "ordered special-day timestamp contract"
Assert-Contains -Text $caseText -Pattern 'abs_tol = 0.0' -Description "zero absolute tolerance"
Assert-Contains -Text $caseText -Pattern 'rmse_tol = 0.0' -Description "zero RMSE tolerance"
Assert-Contains -Text $caseText -Pattern 'idf = "data/conformance_cases/calendar_special_day_fixed_date_hourly_exact_001/calendar_special_day_fixed_date_hourly_exact.idf"' -Description "manifest input.idf attribution"
Assert-Contains -Text $caseText -Pattern 'weather = "data/conformance_cases/calendar_special_day_fixed_date_hourly_exact_001/calendar_special_day_fixed_date_hourly_exact.epw"' -Description "manifest input.weather attribution"
Assert-Contains -Text $caseText -Pattern 'script = "scripts/dev.cmd compare-calendar-special-day-fixed-date-exact"' -Description "manifest blocking-gate attribution"
Assert-Contains -Text $caseText -Pattern 'blocking = true' -Description "manifest blocking flag attribution"
Assert-Contains -Text $caseText -Pattern "24 Sunday index-1 samples, 24 Holiday index-8 samples, and 24 Tuesday index-3 samples" -Description "narrow day-type value claim"
Assert-Contains -Text $idfText -Pattern "No,  !- Use Weather File Holidays and Special Days" -Description "disabled weather-file holiday policy"
Assert-Contains -Text $idfText -Pattern "No,  !- Use Weather File Daylight Saving Period" -Description "disabled weather-file DST policy"
Assert-Contains -Text $idfText -Pattern "No,  !- Apply Weekend Holiday Rule" -Description "disabled weekend holiday rule"
Assert-Contains -Text $idfText -Pattern "RunPeriodControl:SpecialDays" -Description "input-file special-day object"
Assert-Contains -Text $idfText -Pattern "Leap Day Holiday" -Description "fixed-date special-day name"
Assert-Contains -Text $idfText -Pattern "Site Day Type Index" -Description "day-type output request"
Assert-Contains -Text $weatherText -Pattern "HOLIDAYS/DAYLIGHT SAVINGS,Yes,0,0,0" -Description "EPW leap-only calendar policy"

if ($weatherRows.Count -ne 72) {
    throw "Fixed-date special-day EPW must contain 72 hourly rows, found $($weatherRows.Count)"
}
foreach ($date in @("2016,2,28", "2016,2,29", "2016,3,1")) {
    $dateRows = @($weatherRows | Where-Object { $_ -match ('^' + [regex]::Escape($date) + ',') })
    if ($dateRows.Count -ne 24) {
        throw "Fixed-date special-day EPW must contain 24 rows for $date, found $($dateRows.Count)"
    }
}

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Remove-RepoDirectory -Path $CaseOutputRoot
Write-Host "Running fixed-date input-file special-day exact gate."
$output = & $cargo.Source run -p ep_cli --quiet -- conformance time-weather-schedule-report $CasePath $OracleRoot $OutputRoot 2>&1
if ($LASTEXITCODE -ne 0) {
    $output | ForEach-Object { Write-Host $_ }
    throw "Fixed-date input-file special-day exact gate failed."
}
$text = $output -join "`n"
Assert-Contains -Text $text -Pattern "id: $CaseId" -Description "report id"
Assert-Contains -Text $text -Pattern "status: pass" -Description "report status"

$summaryPath = Join-Path $CaseOutputRoot "compare\compare-summary.json"
$reportPath = Join-Path $CaseOutputRoot "compare\compare-report.md"
$oracleEsoPath = Join-Path $CaseOutputRoot "oracle\eplusout.eso"
foreach ($path in @($summaryPath, $reportPath, $oracleEsoPath)) {
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Missing fixed-date special-day comparison artifact: $path"
    }
}

$summary = Get-Content -LiteralPath $summaryPath -Raw -Encoding UTF8 | ConvertFrom-Json
if ($summary.status -ne "pass" -or $summary.conformance_claim -ne $true) {
    throw "Fixed-date special-day case must be passing conformance evidence"
}
if ($summary.time_axis_samples -ne 72) {
    throw "Expected 72 hourly TimeAxis samples, found $($summary.time_axis_samples)"
}

$specialDays = $summary.special_days
if ($specialDays.input_file_declared -ne 1 -or $specialDays.apply_weekend_rule -ne $false -or
    $specialDays.resolved_count -ne 1 -or $specialDays.hourly_samples -ne 24) {
    throw "Unexpected fixed-date special-day JSON summary"
}
$resolved = @($specialDays.resolved)
if ($resolved.Count -ne 1 -or $resolved[0].name -ne "LEAP DAY HOLIDAY" -or
    $resolved[0].start_month -ne 2 -or $resolved[0].start_day -ne 29 -or
    $resolved[0].start_day_of_year -ne 60 -or
    $resolved[0].duration_days -ne 1 -or $resolved[0].day_type -ne "Holiday" -or
    $resolved[0].day_type_index -ne 8 -or $resolved[0].weekend_shift_days -ne 0) {
    throw "Unexpected resolved fixed-date special-day JSON diagnostic"
}

$series = $summary.series | Where-Object {
    $_.key -eq "ENVIRONMENT" -and $_.variable -eq "Site Day Type Index"
}
if ($null -eq $series) {
    throw "Missing Site Day Type Index series"
}
if ($series.expected_samples -ne 72 -or $series.observed_samples -ne 72 -or $series.compared_samples -ne 72) {
    throw "Unexpected special-day series sample counts"
}
if ($series.timestamp_contract -ne "ordered-exact-unique" -or $series.timestamp_status -ne "pass") {
    throw "Ordered special-day timestamp contract failed"
}
if ($series.timestamp_expected_unique -ne $true -or $series.timestamp_observed_unique -ne $true -or $series.timestamp_order_match -ne $true) {
    throw "Special-day timestamp uniqueness/order failed"
}
$expectedFirst = "env=SPECIAL DAY FIXED DATE RUN PERIOD;day=1;month=2;date=28;dst=0;hour=1;start=0.00;end=60.00;day_type=Sunday"
$expectedLast = "env=SPECIAL DAY FIXED DATE RUN PERIOD;day=3;month=3;date=1;dst=0;hour=24;start=0.00;end=60.00;day_type=Tuesday"
if ($series.expected_first_timestamp -ne $expectedFirst -or $series.observed_first_timestamp -ne $expectedFirst) {
    throw "Unexpected first fixed-date special-day timestamp"
}
if ($series.expected_last_timestamp -ne $expectedLast -or $series.observed_last_timestamp -ne $expectedLast) {
    throw "Unexpected last fixed-date special-day timestamp"
}
if ($series.max_abs_tolerance -ne 0.0 -or $series.max_rel_tolerance -ne 0.0 -or $series.max_rmse_tolerance -ne 0.0) {
    throw "Fixed-date special-day case must use zero tolerances"
}
if ($series.max_abs_delta -ne 0.0 -or $series.rmse_delta -ne 0.0 -or $series.max_rel_delta -ne 0.0 -or $series.status -ne "pass") {
    throw "Special-day state values must match exactly"
}

$oracleEsoLines = Get-Content -LiteralPath $oracleEsoPath -Encoding UTF8
$dictionary = $oracleEsoLines | Where-Object { $_ -match '^\d+,\d+,Environment,Site Day Type Index' } | Select-Object -First 1
if ($null -eq $dictionary -or $dictionary -notmatch '^(\d+),') {
    throw "Missing Site Day Type Index ESO dictionary entry"
}
$reportId = $Matches[1]
$valueRows = @($oracleEsoLines | Where-Object { $_ -match ('^' + $reportId + ',\s*[-+0-9.E]+\s*$') })
$values = @($valueRows | ForEach-Object { [double](($_ -split ',', 2)[1].Trim()) })
$timestampRows = @($oracleEsoLines | Where-Object { $_ -match '^2,\d+,\s*\d+,\s*\d+,\s*\d+,\s*\d+,' })
if ($values.Count -ne 72 -or $timestampRows.Count -ne 72) {
    throw "Expected 72 oracle day-type values and hourly timestamps"
}
for ($index = 0; $index -lt 72; ++$index) {
    $expectedValue = if ($index -lt 24) { 1.0 } elseif ($index -lt 48) { 8.0 } else { 3.0 }
    if ($values[$index] -ne $expectedValue) {
        throw "Unexpected oracle Site Day Type Index at sample $index`: $($values[$index])"
    }
    $expectedDayType = if ($index -lt 24) { "Sunday" } elseif ($index -lt 48) { "Holiday" } else { "Tuesday" }
    if ($timestampRows[$index] -notmatch (',' + $expectedDayType + '$')) {
        throw "Unexpected oracle day type at sample $index`: $($timestampRows[$index])"
    }
}

$reportText = Get-Content -LiteralPath $reportPath -Raw -Encoding UTF8
Assert-Contains -Text $reportText -Pattern "input_file_special_days_declared: 1" -Description "markdown special-day declaration count"
Assert-Contains -Text $reportText -Pattern "special_day_weekend_rule: false" -Description "markdown weekend policy"
Assert-Contains -Text $reportText -Pattern "special_days_resolved: 1" -Description "markdown resolved special-day count"
Assert-Contains -Text $reportText -Pattern "special_day_hourly_samples: 24" -Description "markdown special-day sample count"
Assert-Contains -Text $reportText -Pattern "special_day_resolved: LEAP DAY HOLIDAY 2/29 duration=1 day_type=Holiday weekend_shift_days=0" -Description "markdown resolved special day"
Assert-Contains -Text $reportText -Pattern "Site Day Type Index" -Description "markdown day-type output row"
Write-Host "Fixed-date input-file special-day exact gate passed."
