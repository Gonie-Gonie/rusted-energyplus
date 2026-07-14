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
$WeatherRef = "data/conformance_cases/calendar_special_day_fixed_date_hourly_exact_001/calendar_special_day_fixed_date_hourly_exact.epw"
$WeatherPath = Join-Path $RepoRoot ($WeatherRef -replace '/', '\')
$ExpectedEnvironment = "SPECIAL DAY OVERLAP RUN PERIOD"
$WarningNonClaim = "The numerical claim covers only the 72 successful ordered, unique hour-ending Site Day Type Index values and timestamps. EnergyPlus-versus-Rust warning text, count, repetition, and diagnostics parity; other dates, durations, day types, overlap shapes, duplicate names, mixed EPW/IDF precedence, year wrap, schedule lookup, tomorrow state, raw ESO serialization, actual weather, and cross-year traversal remain outside this claim."
$Cases = @(
    [pscustomobject]@{
        Id = "calendar_special_day_overlap_zulu_then_alpha_hourly_exact_001"
        IdfName = "calendar_special_day_overlap_zulu_then_alpha_hourly_exact.idf"
        ExpectedNames = @("ZULU HOLIDAY DEFINITION", "ALPHA CUSTOM DAY DEFINITION")
        ExpectedDayTypes = @("Holiday", "CustomDay1")
        ExpectedDayTypeIndices = @(8, 11)
        MiddleDayType = "CustomDay1"
        MiddleDayTypeIndex = 11.0
        EarlierName = "ZULU HOLIDAY DEFINITION"
        LaterName = "ALPHA CUSTOM DAY DEFINITION"
        DailyClaim = "24 Sunday=1, 24 CustomDay1=11, and 24 Tuesday=3"
    },
    [pscustomobject]@{
        Id = "calendar_special_day_overlap_alpha_then_zulu_hourly_exact_001"
        IdfName = "calendar_special_day_overlap_alpha_then_zulu_hourly_exact.idf"
        ExpectedNames = @("ALPHA CUSTOM DAY DEFINITION", "ZULU HOLIDAY DEFINITION")
        ExpectedDayTypes = @("CustomDay1", "Holiday")
        ExpectedDayTypeIndices = @(11, 8)
        MiddleDayType = "Holiday"
        MiddleDayTypeIndex = 8.0
        EarlierName = "ALPHA CUSTOM DAY DEFINITION"
        LaterName = "ZULU HOLIDAY DEFINITION"
        DailyClaim = "24 Sunday=1, 24 Holiday=8, and 24 Tuesday=3"
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
    $case | Add-Member -NotePropertyName CaseOutputRoot -NotePropertyValue (Join-Path $OutputRoot $case.Id)
}

$requiredPaths = @(
    (Join-Path $OracleRoot "energyplus.exe"),
    (Join-Path $OracleRoot "ConvertInputFormat.exe"),
    $WeatherPath
)
foreach ($case in $Cases) {
    $requiredPaths += @($case.CasePath, $case.IdfPath)
}
foreach ($path in $requiredPaths) {
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Missing required overlap-order conformance file: $path"
    }
}
if ([System.IO.Path]::GetFullPath($Cases[0].CaseRoot) -eq [System.IO.Path]::GetFullPath($Cases[1].CaseRoot) -or
    [System.IO.Path]::GetFullPath($Cases[0].IdfPath) -eq [System.IO.Path]::GetFullPath($Cases[1].IdfPath)) {
    throw "Overlap-order cases must use independent case directories and IDFs"
}
foreach ($case in $Cases) {
    $localWeather = @(Get-ChildItem -LiteralPath $case.CaseRoot -Filter "*.epw" -File)
    if ($localWeather.Count -ne 0) {
        throw "$($case.Id) must reuse only the isolated shared fixed-date EPW, found $($localWeather.Count) local EPW files"
    }
}

$weatherLines = Get-Content -LiteralPath $WeatherPath -Encoding UTF8
$weatherText = $weatherLines -join "`n"
$weatherRows = @($weatherLines | Select-Object -Skip 8 | Where-Object { -not [string]::IsNullOrWhiteSpace($_) })
Assert-Contains -Text $weatherText -Pattern "HOLIDAYS/DAYLIGHT SAVINGS,Yes,0,0,0" -Description "shared EPW leap policy without holidays or DST"
Assert-Contains -Text $weatherText -Pattern "DATA PERIODS,1,1,Data,Sunday,2/28,3/1" -Description "shared three-day EPW period"
if ($weatherRows.Count -ne 72) {
    throw "Shared overlap-order EPW must contain exactly 72 hourly rows, found $($weatherRows.Count)"
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
        throw "Unexpected shared overlap-order EPW row at sample $index`: $($weatherRows[$index])"
    }
    $key = "$($fields[0])-$($fields[1])-$($fields[2])-$($fields[3])-$($fields[4])"
    if (-not $weatherKeys.Add($key)) {
        throw "Duplicate shared overlap-order EPW timestamp at sample $index`: $key"
    }
}

$specialDayPattern = '(?ims)^\s*RunPeriodControl:SpecialDays\s*,\s*(?<name>[^,!;]+?)\s*,\s*(?<date>[^,!;]+?)\s*,\s*(?<duration>[^,!;]+?)\s*,\s*(?<daytype>[^,!;]+?)\s*;'
$specialDayRegex = [regex]::new($specialDayPattern)
$idfTexts = @{}
$specialDayBlocks = @{}
foreach ($case in $Cases) {
    $caseText = Get-Content -LiteralPath $case.CasePath -Raw -Encoding UTF8
    $idfText = Get-Content -LiteralPath $case.IdfPath -Raw -Encoding UTF8
    $idfTexts[$case.Id] = $idfText
    $idfWithoutComments = [regex]::Replace($idfText, '(?m)!.*$', '')
    $normalizedIdf = [regex]::Replace($idfWithoutComments, '\s+', '')
    $expectedRunPeriod = 'RunPeriod,SpecialDayOverlapRunPeriod,2,28,2016,3,1,2016,Sunday,No,No,No,No,No,No;'
    if ([regex]::Matches($normalizedIdf, [regex]::Escape($expectedRunPeriod)).Count -ne 1) {
        throw "$($case.Id) must contain exactly one explicit 2/28/2016 through 3/1/2016 RunPeriod with all six policies No"
    }
    Write-Host "OK $($case.Id) exact 2016 RunPeriod and six-policy isolation."

    Assert-Contains -Text $caseText -Pattern 'comparison_class = "conformance"' -Description "$($case.Id) conformance comparison class"
    Assert-Contains -Text $caseText -Pattern 'conformance_claim = true' -Description "$($case.Id) conformance claim"
    Assert-Contains -Text $caseText -Pattern 'tier = "A"' -Description "$($case.Id) Tier A attribution"
    Assert-Contains -Text $caseText -Pattern 'domains = ["weather"]' -Description "$($case.Id) weather-only scope"
    Assert-Contains -Text $caseText -Pattern "idf = `"$($case.IdfRef)`"" -Description "$($case.Id) independent IDF attribution"
    Assert-Contains -Text $caseText -Pattern "weather = `"$WeatherRef`"" -Description "$($case.Id) shared EPW attribution"
    Assert-Contains -Text $caseText -Pattern 'key = "ENVIRONMENT"' -Description "$($case.Id) output key"
    Assert-Contains -Text $caseText -Pattern 'variable = "Site Day Type Index"' -Description "$($case.Id) output variable"
    Assert-Contains -Text $caseText -Pattern 'frequency = "hourly"' -Description "$($case.Id) hourly output frequency"
    Assert-Contains -Text $caseText -Pattern 'class = "weather"' -Description "$($case.Id) weather output class"
    Assert-Contains -Text $caseText -Pattern 'source = "eso"' -Description "$($case.Id) ESO output source"
    Assert-Contains -Text $caseText -Pattern 'domain = "weather"' -Description "$($case.Id) weather output domain"
    Assert-Contains -Text $caseText -Pattern 'level = "conformance"' -Description "$($case.Id) output conformance level"
    Assert-Contains -Text $caseText -Pattern 'timestamp_contract = "ordered-exact-unique"' -Description "$($case.Id) ordered timestamp contract"
    Assert-Contains -Text $caseText -Pattern 'abs_tol = 0.0' -Description "$($case.Id) zero absolute tolerance"
    Assert-Contains -Text $caseText -Pattern 'rmse_tol = 0.0' -Description "$($case.Id) zero RMSE tolerance"
    Assert-Contains -Text $caseText -Pattern 'script = "scripts/dev.cmd compare-calendar-special-day-overlap-order-exact"' -Description "$($case.Id) blocking gate attribution"
    Assert-Contains -Text $caseText -Pattern 'blocking = true' -Description "$($case.Id) blocking gate flag"
    Assert-Contains -Text $caseText -Pattern $case.DailyClaim -Description "$($case.Id) narrow daily value claim"
    Assert-Contains -Text $caseText -Pattern $WarningNonClaim -Description "$($case.Id) warning and diagnostics non-claim"

    Assert-Contains -Text $idfText -Pattern "  Sunday," -Description "$($case.Id) explicit start weekday"
    Assert-Contains -Text $idfText -Pattern "No,  !- Use Weather File Holidays and Special Days" -Description "$($case.Id) disabled EPW holidays"
    Assert-Contains -Text $idfText -Pattern "No,  !- Use Weather File Daylight Saving Period" -Description "$($case.Id) disabled EPW DST"
    Assert-Contains -Text $idfText -Pattern "No,  !- Apply Weekend Holiday Rule" -Description "$($case.Id) disabled weekend rule"
    Assert-Contains -Text $idfText -Pattern "No,  !- Use Weather File Rain Indicators" -Description "$($case.Id) disabled EPW rain"
    Assert-Contains -Text $idfText -Pattern "No,  !- Use Weather File Snow Indicators" -Description "$($case.Id) disabled EPW snow"
    Assert-Contains -Text $idfText -Pattern "No;  !- Treat Weather as Actual" -Description "$($case.Id) disabled actual weather"
    Assert-Contains -Text $idfText -Pattern "Site Day Type Index" -Description "$($case.Id) day-type output request"
    if ($idfText -match '(?im)^\s*RunPeriodControl:DaylightSavingTime\s*,') {
        throw "$($case.Id) must not declare input-file daylight saving"
    }

    $blocks = @($specialDayRegex.Matches($idfText))
    $specialDayBlocks[$case.Id] = $blocks
    if ($blocks.Count -ne 2) {
        throw "$($case.Id) must declare exactly two parseable input-file special days, found $($blocks.Count)"
    }
    for ($definitionIndex = 0; $definitionIndex -lt 2; ++$definitionIndex) {
        $block = $blocks[$definitionIndex]
        $actualName = $block.Groups['name'].Value.Trim().ToUpperInvariant()
        $actualDate = $block.Groups['date'].Value.Trim()
        $actualDuration = $block.Groups['duration'].Value.Trim()
        $actualDayType = $block.Groups['daytype'].Value.Trim()
        if ($actualName -cne $case.ExpectedNames[$definitionIndex] -or $actualDate -cne "2/29" -or
            $actualDuration -cne "1" -or $actualDayType -cne $case.ExpectedDayTypes[$definitionIndex]) {
            throw "Unexpected $($case.Id) special-day declaration $definitionIndex`: $($block.Value.Trim())"
        }
    }
}

$firstCase = $Cases[0]
$secondCase = $Cases[1]
$firstText = $idfTexts[$firstCase.Id]
$secondText = $idfTexts[$secondCase.Id]
$firstBlocks = $specialDayBlocks[$firstCase.Id]
$secondBlocks = $specialDayBlocks[$secondCase.Id]
if ($firstText -ceq $secondText) {
    throw "Overlap-order IDFs must differ by their declaration order"
}
if ($firstBlocks[0].Value.Trim() -cne $secondBlocks[1].Value.Trim() -or
    $firstBlocks[1].Value.Trim() -cne $secondBlocks[0].Value.Trim()) {
    throw "Overlap-order IDFs must contain identical SpecialDays definitions in exact reverse order"
}
$firstSkeleton = $specialDayRegex.Replace($firstText, "<SPECIAL-DAY-DEFINITION>")
$secondSkeleton = $specialDayRegex.Replace($secondText, "<SPECIAL-DAY-DEFINITION>")
if ($firstSkeleton -cne $secondSkeleton) {
    throw "Overlap-order IDFs differ outside the order of the two SpecialDays blocks"
}
Write-Host "OK paired IDFs isolate only reversed SpecialDays declaration order."

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Write-Host "Running paired special-day overlap declaration-order exact gate."
foreach ($case in $Cases) {
    Remove-RepoDirectory -Path $case.CaseOutputRoot
    $output = & $cargo.Source run -p ep_cli --quiet -- conformance time-weather-schedule-report $case.CasePath $OracleRoot $OutputRoot 2>&1
    if ($LASTEXITCODE -ne 0) {
        $output | ForEach-Object { Write-Host $_ }
        throw "Special-day overlap-order case failed: $($case.Id)"
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

    $summary = Get-Content -LiteralPath $summaryPath -Raw -Encoding UTF8 | ConvertFrom-Json
    if ($summary.status -ne "pass" -or $summary.conformance_claim -ne $true -or
        $summary.time_axis_samples -ne 72 -or $summary.series_count -ne 1 -or
        $summary.conformance_series_count -ne 1) {
        throw "$($case.Id) must be passing 72-sample conformance evidence"
    }
    $specialDays = $summary.special_days
    if ($specialDays.weather_file_declared -ne 0 -or $specialDays.run_period_uses_weather_file -ne $false -or
        $specialDays.weather_file_resolved -ne 0 -or $specialDays.input_file_declared -ne 2 -or
        $specialDays.apply_weekend_rule -ne $false -or $specialDays.resolved_count -ne 2 -or
        $specialDays.hourly_samples -ne 24) {
        throw "Unexpected $($case.Id) overlap-order special-day JSON state"
    }
    $resolved = @($specialDays.resolved)
    if ($resolved.Count -ne 2) {
        throw "$($case.Id) must resolve exactly two special-day declarations"
    }
    for ($definitionIndex = 0; $definitionIndex -lt 2; ++$definitionIndex) {
        $entry = $resolved[$definitionIndex]
        if ($entry.name -cne $case.ExpectedNames[$definitionIndex] -or $entry.source -cne "input-file" -or
            $entry.start_month -ne 2 -or $entry.start_day -ne 29 -or $entry.start_day_of_year -ne 60 -or
            $entry.duration_days -ne 1 -or $entry.day_type -cne $case.ExpectedDayTypes[$definitionIndex] -or
            $entry.day_type_index -ne $case.ExpectedDayTypeIndices[$definitionIndex] -or
            $entry.weekend_shift_days -ne 0) {
            throw "Unexpected $($case.Id) resolved declaration-order entry $definitionIndex"
        }
    }

    $seriesMatches = @($summary.series | Where-Object {
        $_.key -eq "ENVIRONMENT" -and $_.variable -eq "Site Day Type Index"
    })
    if ($seriesMatches.Count -ne 1) {
        throw "Missing unique Site Day Type Index series for $($case.Id)"
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
        throw "Ordered exact Site Day Type Index contract failed for $($case.Id)"
    }

    $oracleEsoLines = Get-Content -LiteralPath $oracleEsoPath -Encoding UTF8
    $dictionaryRows = @($oracleEsoLines | Where-Object { $_ -match '^\d+,\d+,Environment,Site Day Type Index' })
    if ($dictionaryRows.Count -ne 1) {
        throw "Expected one Site Day Type Index ESO dictionary entry for $($case.Id), found $($dictionaryRows.Count)"
    }
    $dictionaryMatch = [regex]::Match($dictionaryRows[0], '^(\d+),')
    if (-not $dictionaryMatch.Success) {
        throw "Malformed Site Day Type Index ESO dictionary entry for $($case.Id)"
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
        $expectedDate = $calendarDays[$dayOffset]
        $expectedValue = if ($dayOffset -eq 0) { 1.0 } elseif ($dayOffset -eq 1) { $case.MiddleDayTypeIndex } else { 3.0 }
        $expectedLabel = if ($dayOffset -eq 0) { "Sunday" } elseif ($dayOffset -eq 1) { $case.MiddleDayType } else { "Tuesday" }
        $expectedHour = ($index % 24) + 1
        if ($values[$index] -ne $expectedValue) {
            throw "Unexpected $($case.Id) EnergyPlus Site Day Type Index at sample $index`: $($values[$index])"
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
            throw "Unexpected $($case.Id) EnergyPlus timestamp at sample $index`: $($timestampRows[$index])"
        }
    }

    # These are EnergyPlus-only oracle diagnostics. Rust warning text/count/repetition parity is not claimed.
    $oracleErrText = Get-Content -LiteralPath $oracleErrPath -Raw -Encoding UTF8
    Assert-Contains -Text $oracleErrText -Pattern "EnergyPlus Completed Successfully-- 3 Warning; 0 Severe Errors;" -Description "$($case.Id) EnergyPlus warning/severe completion count"
    $warningLineCount = [regex]::Matches($oracleErrText, '(?m)^\s*\*\* Warning \*\*').Count
    $severeLineCount = [regex]::Matches($oracleErrText, '(?m)^\s*\*\* Severe\s+\*\*').Count
    $laterOverwrite = "SetSpecialDayDates: Special Day definition ($($case.LaterName)) is overwriting previously entered special day period"
    $earlierOverwrite = "SetSpecialDayDates: Special Day definition ($($case.EarlierName)) is overwriting previously entered special day period"
    $duplicateContinuation = "...This could be caused by duplicate definitions in the Input File."
    if ($warningLineCount -ne 3 -or $severeLineCount -ne 0 -or
        [regex]::Matches($oracleErrText, [regex]::Escape($laterOverwrite)).Count -ne 3 -or
        [regex]::Matches($oracleErrText, [regex]::Escape($earlierOverwrite)).Count -ne 0 -or
        [regex]::Matches($oracleErrText, [regex]::Escape($duplicateContinuation)).Count -ne 3) {
        throw "Unexpected EnergyPlus-only overwrite diagnostics for $($case.Id)"
    }

    $reportText = Get-Content -LiteralPath $reportPath -Raw -Encoding UTF8
    Assert-Contains -Text $reportText -Pattern "conformance_claim: true" -Description "$($case.Id) markdown conformance claim"
    Assert-Contains -Text $reportText -Pattern "status: pass" -Description "$($case.Id) markdown pass status"
    Assert-Contains -Text $reportText -Pattern "time_axis_samples: 72" -Description "$($case.Id) markdown time-axis count"
    Assert-Contains -Text $reportText -Pattern "input_file_special_days_declared: 2" -Description "$($case.Id) markdown declaration count"
    Assert-Contains -Text $reportText -Pattern "special_day_weekend_rule: false" -Description "$($case.Id) markdown weekend rule"
    Assert-Contains -Text $reportText -Pattern "special_days_resolved: 2" -Description "$($case.Id) markdown resolved count"
    Assert-Contains -Text $reportText -Pattern "special_day_hourly_samples: 24" -Description "$($case.Id) markdown unique overwritten-hour count"
    $firstResolvedLine = "special_day_resolved: $($case.ExpectedNames[0]) 2/29 duration=1 day_type=$($case.ExpectedDayTypes[0]) weekend_shift_days=0 source=input-file"
    $secondResolvedLine = "special_day_resolved: $($case.ExpectedNames[1]) 2/29 duration=1 day_type=$($case.ExpectedDayTypes[1]) weekend_shift_days=0 source=input-file"
    Assert-Contains -Text $reportText -Pattern $firstResolvedLine -Description "$($case.Id) markdown first resolved declaration"
    Assert-Contains -Text $reportText -Pattern $secondResolvedLine -Description "$($case.Id) markdown second resolved declaration"
    if ($reportText.IndexOf($firstResolvedLine, [System.StringComparison]::Ordinal) -ge
        $reportText.IndexOf($secondResolvedLine, [System.StringComparison]::Ordinal)) {
        throw "$($case.Id) markdown resolved declarations do not preserve IDF source order"
    }
    Assert-Contains -Text $reportText -Pattern "Site Day Type Index" -Description "$($case.Id) markdown exact output row"
}

Write-Host "Paired special-day overlap declaration-order exact gate passed. EnergyPlus warning diagnostics were checked without claiming Rust warning parity."
