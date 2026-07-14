[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
Set-Location $RepoRoot
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$OutputRoot = Join-Path $RepoRoot ".runtime\time-weather-schedule-conformance\26.1.0"
$ExpectedFirstTimestamp = "env=LEAP DAY EXACT RUN PERIOD;day=1;month=2;date=28;dst=0;hour=1;start=0.00;end=60.00;day_type=Sunday"
$SharedIdfPath = Join-Path $RepoRoot "data\conformance_cases\calendar_schedule_hourly_exact_001\calendar_schedule_hourly_exact.idf"
$Cases = @(
    [pscustomobject]@{
        Id = "calendar_schedule_hourly_exact_001"
        CasePath = Join-Path $RepoRoot "data\conformance_cases\calendar_schedule_hourly_exact_001\case.toml"
        IdfPath = $SharedIdfPath
        WeatherPath = Join-Path $RepoRoot "data\conformance_cases\calendar_schedule_hourly_exact_001\calendar_schedule_hourly_exact.epw"
        AllowsLeapYears = $true
        Samples = 72
        EffectiveDays = 3
        SkippedLeapDays = 0
        EffectiveLeapYear = $true
        LastTimestamp = "env=LEAP DAY EXACT RUN PERIOD;day=3;month=3;date=1;dst=0;hour=24;start=0.00;end=60.00;day_type=Tuesday"
    },
    [pscustomobject]@{
        Id = "calendar_schedule_weather_leap_policy_no_001"
        CasePath = Join-Path $RepoRoot "data\conformance_cases\calendar_schedule_weather_leap_policy_no_001\case.toml"
        IdfPath = $SharedIdfPath
        WeatherPath = Join-Path $RepoRoot "data\conformance_cases\calendar_schedule_weather_leap_policy_no_001\calendar_schedule_weather_leap_policy_no.epw"
        AllowsLeapYears = $false
        Samples = 48
        EffectiveDays = 2
        SkippedLeapDays = 1
        EffectiveLeapYear = $false
        LastTimestamp = "env=LEAP DAY EXACT RUN PERIOD;day=2;month=3;date=1;dst=0;hour=24;start=0.00;end=60.00;day_type=Monday"
    }
)

function Assert-RepoSubPath {
    param([Parameter(Mandatory = $true)][string]$Path)
    $full = [System.IO.Path]::GetFullPath($Path)
    $root = [System.IO.Path]::GetFullPath($RepoRoot).TrimEnd([System.IO.Path]::DirectorySeparatorChar, [System.IO.Path]::AltDirectorySeparatorChar)
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
        Write-Host $Text
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
    $match = [regex]::Match($Text, $pattern)
    if (-not $match.Success) {
        throw "$Description is missing TOML section [$Name]"
    }
    return $match.Groups["body"].Value
}

function Get-TomlStringValue {
    param(
        [Parameter(Mandatory = $true)][string]$Text,
        [Parameter(Mandatory = $true)][string]$Name,
        [Parameter(Mandatory = $true)][string]$Description
    )

    $pattern = '(?m)^\s*' + [regex]::Escape($Name) + '\s*=\s*"(?<value>[^"]+)"\s*$'
    $match = [regex]::Match($Text, $pattern)
    if (-not $match.Success) {
        throw "$Description is missing TOML string key: $Name"
    }
    return $match.Groups["value"].Value
}

function Resolve-ManifestInputPath {
    param(
        [Parameter(Mandatory = $true)][string]$ManifestPath,
        [Parameter(Mandatory = $true)][string]$Reference,
        [Parameter(Mandatory = $true)][string]$Description
    )

    if ([System.IO.Path]::IsPathRooted($Reference)) {
        $candidate = [System.IO.Path]::GetFullPath($Reference)
    }
    else {
        $repoCandidate = [System.IO.Path]::GetFullPath((Join-Path $RepoRoot $Reference))
        if (Test-Path -LiteralPath $repoCandidate -PathType Leaf) {
            $candidate = $repoCandidate
        }
        else {
            $candidate = [System.IO.Path]::GetFullPath((Join-Path (Split-Path -Parent $ManifestPath) $Reference))
        }
    }

    if (-not (Test-Path -LiteralPath $candidate -PathType Leaf)) {
        throw "$Description does not resolve to a file: $Reference -> $candidate"
    }
    return (Resolve-Path -LiteralPath $candidate).Path
}

function Get-ManifestInputPaths {
    param(
        [Parameter(Mandatory = $true)][string]$ManifestPath,
        [Parameter(Mandatory = $true)][string]$CaseId
    )

    $text = Get-Content -LiteralPath $ManifestPath -Raw -Encoding UTF8
    $input = Get-TomlSectionBlock -Text $text -Name "input" -Description $CaseId
    $idfReference = Get-TomlStringValue -Text $input -Name "idf" -Description "$CaseId [input]"
    $weatherReference = Get-TomlStringValue -Text $input -Name "weather" -Description "$CaseId [input]"
    return [pscustomobject]@{
        Idf = Resolve-ManifestInputPath -ManifestPath $ManifestPath -Reference $idfReference -Description "$CaseId input.idf"
        Weather = Resolve-ManifestInputPath -ManifestPath $ManifestPath -Reference $weatherReference -Description "$CaseId input.weather"
    }
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
    Write-Host "OK $Description`: $actualFull"
}

foreach ($path in @(
    (Join-Path $OracleRoot "energyplus.exe"),
    (Join-Path $OracleRoot "ConvertInputFormat.exe")
) + @($Cases | ForEach-Object { @($_.CasePath, $_.IdfPath, $_.WeatherPath) })) {
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Missing required weather-effective calendar file: $path"
    }
}

$manifestInputs = @{}
foreach ($case in $Cases) {
    $inputs = Get-ManifestInputPaths -ManifestPath $case.CasePath -CaseId $case.Id
    Assert-SamePath -Actual $inputs.Idf -Expected $case.IdfPath -Description "$($case.Id) manifest input.idf"
    Assert-SamePath -Actual $inputs.Weather -Expected $case.WeatherPath -Description "$($case.Id) manifest input.weather"
    $manifestInputs[$case.Id] = $inputs
}
Assert-SamePath `
    -Actual $manifestInputs[$Cases[0].Id].Idf `
    -Expected $manifestInputs[$Cases[1].Id].Idf `
    -Description "paired manifests shared input.idf"

$leapWeather = Get-Content -LiteralPath $Cases[0].WeatherPath
$noLeapWeather = Get-Content -LiteralPath $Cases[1].WeatherPath
$leapDataRows = @($leapWeather | Select-Object -Skip 8 | Where-Object { -not [string]::IsNullOrWhiteSpace($_) })
$noLeapDataRows = @($noLeapWeather | Select-Object -Skip 8 | Where-Object { -not [string]::IsNullOrWhiteSpace($_) })
$noLeapFebruary29Rows = @($noLeapDataRows | Where-Object { $_ -match '^2016,2,29,' })
if ($noLeapDataRows.Count -ne 72) {
    throw "No-leap EPW must retain all 72 raw rows, found $($noLeapDataRows.Count)"
}
if ($noLeapFebruary29Rows.Count -ne 24) {
    throw "No-leap EPW must retain 24 February 29 rows, found $($noLeapFebruary29Rows.Count)"
}
if (($leapDataRows -join "`n") -cne ($noLeapDataRows -join "`n")) {
    throw "Paired EPW data rows differ; only header policy metadata may change"
}
Assert-Contains -Text ($leapWeather -join "`n") -Pattern "HOLIDAYS/DAYLIGHT SAVINGS,Yes,0,0,0" -Description "Leap-observed EPW policy"
Assert-Contains -Text ($noLeapWeather -join "`n") -Pattern "HOLIDAYS/DAYLIGHT SAVINGS,No,0,0,0" -Description "No-leap EPW policy"

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

$summaries = @{}
Write-Host "Running paired EPW weather-effective calendar gate."
foreach ($case in $Cases) {
    $caseOutputRoot = Join-Path $OutputRoot $case.Id
    Remove-RepoDirectory -Path $caseOutputRoot

    $output = & $cargo.Source run -p ep_cli --quiet -- conformance time-weather-schedule-report $case.CasePath $OracleRoot $OutputRoot 2>&1
    if ($LASTEXITCODE -ne 0) {
        $output | ForEach-Object { Write-Host $_ }
        throw "Weather-effective calendar case failed: $($case.Id)"
    }
    $text = ($output -join "`n")
    Assert-Contains -Text $text -Pattern "id: $($case.Id)" -Description "$($case.Id) report id"
    Assert-Contains -Text $text -Pattern "status: pass" -Description "$($case.Id) status"

    $summaryPath = Join-Path $caseOutputRoot "compare\compare-summary.json"
    $reportPath = Join-Path $caseOutputRoot "compare\compare-report.md"
    if (-not (Test-Path -LiteralPath $summaryPath -PathType Leaf)) {
        throw "Missing compare summary: $summaryPath"
    }
    if (-not (Test-Path -LiteralPath $reportPath -PathType Leaf)) {
        throw "Missing compare report: $reportPath"
    }

    $summary = Get-Content -LiteralPath $summaryPath -Raw | ConvertFrom-Json
    $summaries[$case.Id] = $summary
    if ($summary.status -ne "pass" -or $summary.conformance_claim -ne $true) {
        throw "Case must be passing conformance evidence: $($case.Id)"
    }
    if ($summary.time_axis_samples -ne $case.Samples) {
        throw "Unexpected time-axis samples for $($case.Id): $($summary.time_axis_samples)"
    }
    if ($summary.weather_calendar.policy_applied -ne $true) {
        throw "EPW calendar policy was not applied for $($case.Id)"
    }
    if ($summary.weather_calendar.weather_file_allows_leap_years -ne $case.AllowsLeapYears) {
        throw "Unexpected EPW leap policy for $($case.Id)"
    }
    if ($summary.weather_calendar.gregorian_calendar_days -ne 3) {
        throw "Gregorian input calendar must retain three days for $($case.Id)"
    }
    if ($summary.weather_calendar.weather_effective_calendar_days -ne $case.EffectiveDays) {
        throw "Unexpected weather-effective day count for $($case.Id)"
    }
    if ($summary.weather_calendar.leap_days_skipped -ne $case.SkippedLeapDays) {
        throw "Unexpected skipped leap-day count for $($case.Id)"
    }
    if ($summary.weather_calendar.start_year_gregorian_leap -ne $true) {
        throw "The paired fixtures must retain Gregorian leap-year state"
    }
    if ($summary.weather_calendar.start_year_weather_effective_leap -ne $case.EffectiveLeapYear) {
        throw "Unexpected weather-effective leap state for $($case.Id)"
    }

    $series = $summary.series | Where-Object {
        $_.key -eq "CALENDAR HOURLY 1 TO 24" -and $_.variable -eq "Schedule Value"
    }
    if ($null -eq $series) {
        throw "Missing Schedule Value series for $($case.Id)"
    }
    if ($series.timestamp_contract -ne "ordered-exact-unique" -or $series.timestamp_status -ne "pass") {
        throw "Ordered timestamp contract failed for $($case.Id)"
    }
    if ($series.expected_samples -ne $case.Samples -or $series.observed_samples -ne $case.Samples -or $series.compared_samples -ne $case.Samples) {
        throw "Unexpected series sample counts for $($case.Id)"
    }
    if ($series.timestamp_expected_unique -ne $true -or $series.timestamp_observed_unique -ne $true -or $series.timestamp_order_match -ne $true) {
        throw "Timestamp uniqueness/order failed for $($case.Id)"
    }
    if ($null -ne $series.first_timestamp_divergence) {
        throw "Timestamp sequence diverged for $($case.Id)"
    }
    if ($series.expected_first_timestamp -ne $ExpectedFirstTimestamp -or $series.observed_first_timestamp -ne $ExpectedFirstTimestamp) {
        throw "Unexpected first timestamp for $($case.Id)"
    }
    if ($series.expected_last_timestamp -ne $case.LastTimestamp -or $series.observed_last_timestamp -ne $case.LastTimestamp) {
        throw "Unexpected last timestamp for $($case.Id)"
    }
    if ($series.max_abs_tolerance -ne 0.0 -or $series.max_rel_tolerance -ne 0.0 -or $series.max_rmse_tolerance -ne 0.0) {
        throw "Paired calendar cases must use zero tolerances"
    }
    if ($series.max_abs_delta -ne 0.0 -or $series.rmse_delta -ne 0.0 -or $series.max_rel_delta -ne 0.0 -or $series.status -ne "pass") {
        throw "Schedule values must match exactly for $($case.Id)"
    }

    $reportText = Get-Content -LiteralPath $reportPath -Raw
    Assert-Contains -Text $reportText -Pattern "weather_calendar_policy_applied: true" -Description "$($case.Id) markdown weather policy"
    Assert-Contains -Text $reportText -Pattern "weather_effective_calendar_days: $($case.EffectiveDays)" -Description "$($case.Id) markdown effective days"
}

$yesSamples = $summaries[$Cases[0].Id].time_axis_samples
$noSamples = $summaries[$Cases[1].Id].time_axis_samples
if ($yesSamples - $noSamples -ne 24) {
    throw "Leap policy must remove exactly 24 hourly samples: Yes=$yesSamples No=$noSamples"
}

Write-Host "Weather-effective calendar gate passed."
