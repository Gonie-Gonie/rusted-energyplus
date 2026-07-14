[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$CaseId = "calendar_special_day_nonexistent_fifth_weekday_failure_001"
$CaseRoot = Join-Path $RepoRoot "data\conformance_cases\$CaseId"
$CasePath = Join-Path $CaseRoot "case.toml"
$IdfPath = Join-Path $CaseRoot "calendar_special_day_nonexistent_fifth_weekday_failure.idf"
$BaseIdfPath = Join-Path $RepoRoot "data\conformance_cases\heat_balance_nomass_001\heat_balance_nomass.idf"
$WeatherRef = "data/conformance_cases/calendar_special_day_fixed_date_hourly_exact_001/calendar_special_day_fixed_date_hourly_exact.epw"
$WeatherPath = Join-Path $RepoRoot ($WeatherRef -replace '/', '\')
$CaseOutputRoot = Join-Path $RepoRoot ".runtime\time-weather-schedule-conformance\26.1.0\$CaseId"
$OracleOutputRoot = Join-Path $CaseOutputRoot "oracle"
$RustOutputRoot = Join-Path $CaseOutputRoot "rust"
$CompareRoot = Join-Path $CaseOutputRoot "compare"
$ReportPath = Join-Path $CompareRoot "negative-report.md"
$SummaryPath = Join-Path $CompareRoot "negative-summary.json"
$ExpectedRustError = "failed to build weather-aware time axis: run period MISSING FIFTH SUNDAY RUN PERIOD special day MISSING FIFTH SUNDAY HOLIDAY has no occurrence 5 of Sunday in month 2"

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

function Assert-File {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Description
    )

    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        throw "Missing $Description`: $Path"
    }
}

function Assert-Equal {
    param(
        $Actual,
        $Expected,
        [Parameter(Mandatory = $true)][string]$Description
    )

    if ($Actual -ne $Expected) {
        throw "Unexpected $Description`: expected '$Expected', got '$Actual'"
    }
    Write-Host "OK $Description`: $Expected"
}

function Assert-Contains {
    param(
        [Parameter(Mandatory = $true)][string]$Text,
        [Parameter(Mandatory = $true)][string]$Pattern,
        [Parameter(Mandatory = $true)][string]$Description
    )

    if (-not $Text.Contains($Pattern)) {
        throw "Missing $Description`: $Pattern"
    }
    Write-Host "OK $Description`: $Pattern"
}

function Normalize-Newlines {
    param([Parameter(Mandatory = $true)][string]$Text)
    return (($Text -replace "`r`n", "`n") -replace "`r", "`n")
}

foreach ($required in @(
    (Join-Path $OracleRoot "energyplus.exe"),
    (Join-Path $OracleRoot "ConvertInputFormat.exe"),
    $CasePath,
    $IdfPath,
    $BaseIdfPath,
    $WeatherPath
)) {
    Assert-File -Path $required -Description "required nonexistent-Nth gate input"
}

$caseFiles = @(Get-ChildItem -LiteralPath $CaseRoot -File | Sort-Object Name | ForEach-Object { $_.Name })
$expectedCaseFiles = @("calendar_special_day_nonexistent_fifth_weekday_failure.idf", "case.toml")
if (($caseFiles -join "|") -cne ($expectedCaseFiles -join "|")) {
    throw "Expected-failure fixture directory must contain only the IDF and manifest; found: $($caseFiles -join ', ')"
}

$idfText = Normalize-Newlines -Text (Get-Content -LiteralPath $IdfPath -Raw -Encoding UTF8)
$baseIdfText = Normalize-Newlines -Text (Get-Content -LiteralPath $BaseIdfPath -Raw -Encoding UTF8)
$baseRunPeriod = "RunPeriod,Run Period 1,1,1,2013,1,1,2013,Tuesday,Yes,Yes,No,Yes,Yes;"
$failureRunPeriod = "RunPeriod,Missing Fifth Sunday Run Period,2,28,2016,3,1,2016,Sunday,No,No,Yes,No,No,No;"
$baseOutput = "Output:Variable,ZONE ONE,Zone Mean Air Temperature,Hourly;"
$failureSpecialDay = @(
    "RunPeriodControl:SpecialDays,",
    "  Missing Fifth Sunday Holiday,",
    "  5th Sunday in February,",
    "  1,",
    "  Holiday;"
) -join "`n"
$failureOutput = "Output:Variable,Environment,Site Day Type Index,Hourly;"
$expectedIdfText = $baseIdfText.Replace($baseRunPeriod, $failureRunPeriod)
$expectedIdfText = $expectedIdfText.Replace($baseOutput, "$failureSpecialDay`n`n$failureOutput")
if ($idfText.TrimEnd() -cne $expectedIdfText.TrimEnd()) {
    throw "Expected-failure IDF must preserve heat_balance_nomass topology and differ only by RunPeriod, special day, and output request."
}

foreach ($objectExpectation in @(
    [pscustomobject]@{ Type = "RunPeriod"; Count = 1 },
    [pscustomobject]@{ Type = "RunPeriodControl:SpecialDays"; Count = 1 },
    [pscustomobject]@{ Type = "Zone"; Count = 1 },
    [pscustomobject]@{ Type = "BuildingSurface:Detailed"; Count = 6 },
    [pscustomobject]@{ Type = "Output:Variable"; Count = 1 }
)) {
    $count = [regex]::Matches(
        $idfText,
        "(?im)^\s*" + [regex]::Escape($objectExpectation.Type) + "\s*,"
    ).Count
    Assert-Equal -Actual $count -Expected $objectExpectation.Count -Description "$($objectExpectation.Type) object count"
}
Assert-Contains -Text $idfText -Pattern $failureRunPeriod -Description "explicit 2016 RunPeriod and policy tuple"
Assert-Contains -Text $idfText -Pattern "Missing Fifth Sunday Holiday" -Description "special-day name"
Assert-Contains -Text $idfText -Pattern "5th Sunday in February" -Description "nonexistent Nth rule"
Assert-Contains -Text $idfText -Pattern $failureOutput -Description "single baseline output request"

$manifestText = Get-Content -LiteralPath $CasePath -Raw -Encoding UTF8
foreach ($manifestExpectation in @(
    "id = `"$CaseId`"",
    'comparison_class = "smoke"',
    'conformance_claim = false',
    'tier = "B"',
    'domains = ["weather", "zone", "surface"]',
    "idf = `"data/conformance_cases/$CaseId/calendar_special_day_nonexistent_fifth_weekday_failure.idf`"",
    "weather = `"$WeatherRef`"",
    'variable = "Site Day Type Index"',
    'level = "baseline"',
    'path = ".runtime/time-weather-schedule-conformance/26.1.0/calendar_special_day_nonexistent_fifth_weekday_failure_001/compare/negative-report.md"',
    'script = "scripts/dev.cmd compare-calendar-special-day-nonexistent-nth-error"',
    'blocking = true'
)) {
    Assert-Contains -Text $manifestText -Pattern $manifestExpectation -Description "manifest isolation contract"
}
if ($manifestText -match '(?m)^\s*level\s*=\s*"conformance"\s*$' -or
    $manifestText -match '(?m)^\s*\[\[tolerances\]\]\s*$') {
    throw "Expected-failure smoke manifest must not declare conformance-level or tolerance evidence."
}

$weatherLines = Get-Content -LiteralPath $WeatherPath -Encoding UTF8
$weatherRows = @($weatherLines | Select-Object -Skip 8 | Where-Object { -not [string]::IsNullOrWhiteSpace($_) })
Assert-Equal -Actual $weatherRows.Count -Expected 72 -Description "shared three-day EPW row count"
Assert-Contains -Text ($weatherLines -join "`n") -Pattern "HOLIDAYS/DAYLIGHT SAVINGS,Yes,0,0,0" -Description "shared EPW without holiday or DST definitions"

Remove-RepoDirectory -Path $CaseOutputRoot
New-Item -ItemType Directory -Force -Path $OracleOutputRoot, $CompareRoot | Out-Null

$energyplus = Join-Path $OracleRoot "energyplus.exe"
Write-Host "Running EnergyPlus nonexistent fifth-weekday expected-failure lane."
$savedErrorActionPreference = $ErrorActionPreference
$ErrorActionPreference = "Continue"
$oracleConsole = @(& $energyplus -w $WeatherPath -d $OracleOutputRoot $IdfPath 2>&1)
$oracleExitCode = $LASTEXITCODE
$ErrorActionPreference = $savedErrorActionPreference
if ($oracleExitCode -ne 1) {
    $oracleConsole | ForEach-Object { Write-Host $_ }
    throw "Expected EnergyPlus fatal exit code 1, got $oracleExitCode."
}

$oracleErrPath = Join-Path $OracleOutputRoot "eplusout.err"
$oracleEndPath = Join-Path $OracleOutputRoot "eplusout.end"
$oracleEsoPath = Join-Path $OracleOutputRoot "eplusout.eso"
foreach ($artifact in @($oracleErrPath, $oracleEndPath, $oracleEsoPath)) {
    Assert-File -Path $artifact -Description "EnergyPlus expected-failure artifact"
}
$oracleErrText = Get-Content -LiteralPath $oracleErrPath -Raw -Encoding UTF8
$oracleEndText = Get-Content -LiteralPath $oracleEndPath -Raw -Encoding UTF8
$oracleFailureText = $oracleErrText + "`n" + $oracleEndText
$oracleSevere = "SetSpecialDayDates: Special Day Date, Nth Day of Month, not enough Nths, for SpecialDay=MISSING FIFTH SUNDAY HOLIDAY"
$oracleFatal = "SetSpecialDayDates: Program terminates due to preceding condition(s)."
Assert-Contains -Text $oracleFailureText -Pattern $oracleSevere -Description "EnergyPlus exact not-enough-Nths Severe"
Assert-Contains -Text $oracleFailureText -Pattern $oracleFatal -Description "EnergyPlus exact SetSpecialDayDates Fatal"
Assert-Contains -Text $oracleFailureText -Pattern "EnergyPlus Terminated--Fatal Error Detected. 0 Warning; 1 Severe Errors;" -Description "EnergyPlus zero-warning one-severe summary"
$oracleSevereIndex = $oracleErrText.IndexOf($oracleSevere, [System.StringComparison]::Ordinal)
$oracleFatalIndex = $oracleErrText.IndexOf($oracleFatal, [System.StringComparison]::Ordinal)
if ($oracleSevereIndex -lt 0 -or $oracleFatalIndex -lt 0 -or $oracleSevereIndex -ge $oracleFatalIndex) {
    throw "EnergyPlus not-enough-Nths Severe must precede the SetSpecialDayDates Fatal."
}
Write-Host "OK EnergyPlus Severe precedes Fatal."
if ($oracleFailureText.Contains("EnergyPlus Completed Successfully")) {
    throw "EnergyPlus expected-failure lane must not report successful completion."
}

$oracleEsoLines = @(Get-Content -LiteralPath $oracleEsoPath -Encoding UTF8)
$dictionaryEndIndex = [Array]::IndexOf([string[]]$oracleEsoLines, "End of Data Dictionary")
$oracleDataLines = @(
    if ($dictionaryEndIndex -ge 0 -and $dictionaryEndIndex + 1 -lt $oracleEsoLines.Count) {
        $oracleEsoLines[($dictionaryEndIndex + 1)..($oracleEsoLines.Count - 1)]
    }
)
$dayTypeDictionary = @($oracleEsoLines | Where-Object { $_ -match '^\d+,\d+,Environment,Site Day Type Index' } | Select-Object -First 1)
$dayTypeReportId = $null
if ($dayTypeDictionary.Count -eq 1) {
    $dictionaryMatch = [regex]::Match([string]$dayTypeDictionary[0], '^(\d+),')
    if ($dictionaryMatch.Success) {
        $dayTypeReportId = $dictionaryMatch.Groups[1].Value
    }
}
$oracleValueRows = @(
    if ($null -ne $dayTypeReportId) {
        $oracleDataLines | Where-Object { $_ -match ('^' + [regex]::Escape($dayTypeReportId) + ',\s*[-+0-9.E]+\s*$') }
    }
)
$oracleTimestampRows = @($oracleDataLines | Where-Object { $_ -match '^2,\s*\d+,\s*\d+,\s*\d+,\s*\d+,\s*\d+,' })
Assert-Equal -Actual $oracleValueRows.Count -Expected 0 -Description "EnergyPlus hourly day-type value count before simulation"
Assert-Equal -Actual $oracleTimestampRows.Count -Expected 0 -Description "EnergyPlus hourly timestamp count before simulation"

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}
Write-Host "Building eplus-rs CLI for direct runtime exit-code validation."
$buildOutput = @(& $cargo.Source build -p ep_cli --quiet 2>&1)
if ($LASTEXITCODE -ne 0) {
    $buildOutput | ForEach-Object { Write-Host $_ }
    throw "Failed to build ep_cli."
}
$exe = Join-Path $RepoRoot "target\debug\eplus-rs.exe"
Assert-File -Path $exe -Description "built eplus-rs CLI"

$manifestOutput = @(& $exe conformance validate-case-v2 $CasePath 2>&1)
if ($LASTEXITCODE -ne 0) {
    $manifestOutput | ForEach-Object { Write-Host $_ }
    throw "Expected-failure manifest v2 validation failed."
}
Assert-Contains -Text ($manifestOutput -join "`n") -Pattern "status: valid" -Description "manifest v2 validation status"

Write-Host "Running Rust nonexistent fifth-weekday expected-failure lane."
$savedErrorActionPreference = $ErrorActionPreference
$ErrorActionPreference = "Continue"
$rustConsole = @(& $exe run $IdfPath -w $WeatherPath -d $RustOutputRoot --mode compatibility --partial allow --format rust-native --trace-level normal --overwrite --oracle-root $OracleRoot 2>&1)
$rustExitCode = $LASTEXITCODE
$ErrorActionPreference = $savedErrorActionPreference
if ($rustExitCode -ne 6) {
    $rustConsole | ForEach-Object { Write-Host $_ }
    throw "Expected Rust runtime exit code 6, got $rustExitCode."
}
Assert-Contains -Text ($rustConsole -join "`n") -Pattern "status: runtime" -Description "Rust CLI runtime status"

$rustSummaryPath = Join-Path $RustOutputRoot "run-summary.json"
$rustDiagnosticsPath = Join-Path $RustOutputRoot "diagnostics.json"
$rustErrPath = Join-Path $RustOutputRoot "eplusrs.err"
$rustSupportPath = Join-Path $RustOutputRoot "support-assessment.json"
foreach ($artifact in @($rustSummaryPath, $rustDiagnosticsPath, $rustErrPath, $rustSupportPath)) {
    Assert-File -Path $artifact -Description "Rust expected-failure artifact"
}

$rustSummary = Get-Content -LiteralPath $rustSummaryPath -Raw -Encoding UTF8 | ConvertFrom-Json
Assert-Equal -Actual $rustSummary.status -Expected "runtime" -Description "Rust run-summary status"
Assert-Equal -Actual $rustSummary.exit_code -Expected 6 -Description "Rust run-summary exit code"
Assert-Equal -Actual $rustSummary.message -Expected "Rust runtime failed" -Description "Rust run-summary message"
Assert-Equal -Actual $rustSummary.support.status -Expected "supported-compatibility" -Description "Rust support status before runtime failure"
Assert-Equal -Actual $rustSummary.support.run_result_state -Expected "supported_compatibility_run" -Description "Rust run-result state before runtime failure"
Assert-Equal -Actual $rustSummary.support.runtime_class -Expected "one-zone-heat-balance-compatibility" -Description "Rust runtime class"
if (@($rustSummary.support.matched_capability_ids) -notcontains "official_1zone_uncontrolled_declared_heat_balance") {
    throw "Rust expected-failure lane did not enter the declared one-zone compatibility runtime."
}
if ($null -ne $rustSummary.rust_runtime) {
    throw "Rust expected-failure lane must keep rust_runtime null."
}
if (Test-Path -LiteralPath (Join-Path $RustOutputRoot "results\result-store.json")) {
    throw "Rust expected-failure lane must not write result-store.json."
}

$rustDiagnostics = Get-Content -LiteralPath $rustDiagnosticsPath -Raw -Encoding UTF8 | ConvertFrom-Json
$runtimeErrors = @($rustDiagnostics.diagnostics | Where-Object {
    $_.severity -eq "error" -and $_.code -eq "RuntimeConvergenceFailure" -and $_.stage -eq "runtime"
})
Assert-Equal -Actual $runtimeErrors.Count -Expected 1 -Description "unique structured RuntimeConvergenceFailure"
Assert-Equal -Actual $runtimeErrors[0].message -Expected $ExpectedRustError -Description "exact structured nonexistent-Nth error"
$rustErrText = Get-Content -LiteralPath $rustErrPath -Raw -Encoding UTF8
Assert-Contains -Text $rustErrText -Pattern "exit_status: runtime (6)" -Description "Rust text diagnostic exit status"
Assert-Contains -Text $rustErrText -Pattern "error [RuntimeConvergenceFailure] runtime: $ExpectedRustError" -Description "Rust text diagnostic projection"

$rustSupport = Get-Content -LiteralPath $rustSupportPath -Raw -Encoding UTF8 | ConvertFrom-Json
Assert-Equal -Actual $rustSupport.status -Expected "SupportedCompatibility" -Description "support-assessment status"
Assert-Equal -Actual $rustSupport.runtime_class -Expected "one-zone-heat-balance-compatibility" -Description "support-assessment runtime class"

$negativeSummary = [ordered]@{
    schema_version = 1
    case_id = $CaseId
    status = "pass"
    evidence_class = "expected-failure-smoke"
    conformance_claim = $false
    input_rule = "5th Sunday in February"
    calendar_year = 2016
    oracle = [ordered]@{
        engine = "EnergyPlus"
        version = "26.1.0"
        exit_code = $oracleExitCode
        warnings = 0
        severe_errors = 1
        hourly_values = $oracleValueRows.Count
        hourly_timestamps = $oracleTimestampRows.Count
        severe = $oracleSevere
        fatal = $oracleFatal
    }
    rust = [ordered]@{
        engine = "rusted-energyplus"
        exit_code = $rustExitCode
        status = [string]$rustSummary.status
        diagnostic_code = [string]$runtimeErrors[0].code
        diagnostic_stage = [string]$runtimeErrors[0].stage
        diagnostic_message = [string]$runtimeErrors[0].message
        support_status = [string]$rustSummary.support.status
        runtime_class = [string]$rustSummary.support.runtime_class
        rust_runtime_is_null = ($null -eq $rustSummary.rust_runtime)
        result_store_exists = (Test-Path -LiteralPath (Join-Path $RustOutputRoot "results\result-store.json"))
    }
    semantic_comparison = [ordered]@{
        status = "pass"
        shared_behavior = "reject nonexistent fifth Sunday before hourly simulation samples"
        weekend_rule_behavior = "Apply Weekend Holiday Rule is explicitly Yes and does not rescue or shift the nonexistent Nth-weekday rule"
        numeric_exit_code_equality_claimed = $false
        numeric_output_conformance_claimed = $false
    }
    boundaries = @(
        "Only the explicit 2016 input-file `5th Sunday in February` duration-one Holiday is covered.",
        "Apply Weekend Holiday Rule is explicitly Yes; this invalid Nth rule is rejected without a rescue or shift.",
        "No other ordinal, weekday, month, year, date form, duration, overlap, precedence, weekend behavior, or successful numeric output is claimed."
    )
}

$utf8NoBom = New-Object System.Text.UTF8Encoding($false)
$summaryJson = $negativeSummary | ConvertTo-Json -Depth 12
[System.IO.File]::WriteAllText($SummaryPath, $summaryJson + "`n", $utf8NoBom)
$markdown = @(
    "# Nonexistent Fifth-Weekday Expected-Failure Report",
    "",
    "- case: ``$CaseId``",
    "- status: pass",
    "- evidence class: expected-failure smoke",
    "- conformance claim: false",
    "- input rule: ``5th Sunday in February`` on the explicit 2016 calendar",
    "",
    "## Engine Results",
    "",
    "| Engine | Process exit | Structured outcome | Hourly values | Hourly timestamps |",
    "|---|---:|---|---:|---:|",
    "| EnergyPlus 26.1.0 | $oracleExitCode | SetSpecialDayDates Severe/Fatal; 0 warnings and 1 severe | $($oracleValueRows.Count) | $($oracleTimestampRows.Count) |",
    "| rusted-energyplus | $rustExitCode | RuntimeConvergenceFailure at runtime setup | 0 | 0 |",
    "",
    "## Semantic Comparison",
    "",
    "Both engines reject the same nonexistent fifth-Sunday rule before producing hourly simulation samples. Their numeric process exit codes belong to different contracts and are not compared for equality.",
    "",
    "Apply Weekend Holiday Rule is explicitly Yes and does not rescue or shift the nonexistent Nth-weekday rule.",
    "",
    "EnergyPlus: ``$oracleSevere``",
    "",
    "Rust: ``$ExpectedRustError``",
    "",
    "## Boundary",
    "",
    "This report covers only the explicit 2016 input-file ``5th Sunday in February`` duration-one Holiday. It does not claim numerical Site Day Type Index conformance or any other ordinal, weekday, month, year, date form, duration, overlap, precedence, other weekend behavior, or successful-run output behavior."
) -join "`n"
[System.IO.File]::WriteAllText($ReportPath, $markdown + "`n", $utf8NoBom)

Assert-File -Path $ReportPath -Description "negative markdown report at manifest path"
Assert-File -Path $SummaryPath -Description "negative JSON summary"
$writtenSummary = Get-Content -LiteralPath $SummaryPath -Raw -Encoding UTF8 | ConvertFrom-Json
Assert-Equal -Actual $writtenSummary.status -Expected "pass" -Description "negative JSON report status"
Assert-Equal -Actual $writtenSummary.semantic_comparison.numeric_exit_code_equality_claimed -Expected $false -Description "separate engine exit-code contracts"
$writtenReport = Get-Content -LiteralPath $ReportPath -Raw -Encoding UTF8
Assert-Contains -Text $writtenReport -Pattern "conformance claim: false" -Description "negative markdown no-conformance boundary"
Assert-Contains -Text $writtenReport -Pattern "numeric process exit codes belong to different contracts" -Description "negative markdown exit-code boundary"

Write-Host "Nonexistent fifth-weekday expected-failure gate passed."
Write-Host "  markdown: $ReportPath"
Write-Host "  json: $SummaryPath"
