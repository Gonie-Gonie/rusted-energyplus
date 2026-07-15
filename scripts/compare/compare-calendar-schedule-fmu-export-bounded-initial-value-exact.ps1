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
$SuccessId = "calendar_schedule_fmu_export_bounded_initial_value_exact_001"
$FailureId = "calendar_schedule_fmu_export_bounded_initial_value_failure_001"
$SuccessRoot = Join-Path $RepoRoot "data\conformance_cases\$SuccessId"
$FailureRoot = Join-Path $RepoRoot "data\conformance_cases\$FailureId"
$SuccessCase = Join-Path $SuccessRoot "case.toml"
$FailureCase = Join-Path $FailureRoot "case.toml"
$SuccessIdf = Join-Path $SuccessRoot "calendar_schedule_fmu_export_bounded_initial_value_exact.idf"
$FailureIdf = Join-Path $FailureRoot "calendar_schedule_fmu_export_bounded_initial_value_failure.idf"
$WeatherPath = Join-Path $SuccessRoot "calendar_schedule_fmu_export_bounded_initial_value_exact.epw"
$SuccessOutput = Join-Path $OutputRoot $SuccessId
$FailureOutput = Join-Path $OutputRoot $FailureId
$FailureOracle = Join-Path $FailureOutput "oracle"
$FailureRust = Join-Path $FailureOutput "rust"
$FailureCompare = Join-Path $FailureOutput "compare"
$NegativeReport = Join-Path $FailureCompare "negative-report.md"
$NegativeSummary = Join-Path $FailureCompare "negative-summary.json"
$GateCommand = "scripts/dev.cmd compare-calendar-schedule-fmu-export-bounded-initial-value-exact"

function Assert-RepoSubPath {
    param([Parameter(Mandatory = $true)][string]$Path)
    $full = [System.IO.Path]::GetFullPath($Path)
    $root = [System.IO.Path]::GetFullPath($RepoRoot).TrimEnd(
        [System.IO.Path]::DirectorySeparatorChar,
        [System.IO.Path]::AltDirectorySeparatorChar
    )
    if (-not $full.StartsWith($root + [System.IO.Path]::DirectorySeparatorChar, [System.StringComparison]::OrdinalIgnoreCase)) {
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
    $SuccessCase,
    $FailureCase,
    $SuccessIdf,
    $FailureIdf,
    $WeatherPath
)) {
    Assert-File -Path $required -Description "bounded scalar ScheduleTypeLimits gate input"
}

$successManifest = Get-Content -LiteralPath $SuccessCase -Raw -Encoding UTF8
$failureManifest = Get-Content -LiteralPath $FailureCase -Raw -Encoding UTF8
foreach ($contract in @(
    'comparison_class = "conformance"',
    'conformance_claim = true',
    'tier = "A"',
    'level = "conformance"',
    'timestamp_contract = "ordered-exact-unique"',
    'abs_tol = 0.0',
    'rmse_tol = 0.0',
    'inclusive upper endpoint',
    'The promoted bounded claim is limited to this Continuous scalar FMU Export initial value',
    'Schedule:Compact, Schedule:File, and Schedule:Year resolved min/max',
    'Schedule:Day warning behavior and Discrete integer checks',
    'one-sided, reversed, or blank numeric bounds',
    'unit_type',
    'unknown type-limit reference parity',
    'multiple-violation diagnostic order, text, count, or process-exit parity',
    'EMS/currentVal or live updates',
    'cross-family duplicate source-order parity',
    "script = `"$GateCommand`"",
    'blocking = true'
)) {
    Assert-Contains -Text $successManifest -Pattern $contract -Description "success manifest contract"
}
foreach ($contract in @(
    'comparison_class = "smoke"',
    'conformance_claim = false',
    'tier = "B"',
    'level = "baseline"',
    'zero Schedule Value data samples and exactly 0 Warning and 1 Severe error',
    'diagnostic text, diagnostic count parity beyond that EnergyPlus completion summary, and numeric process exit code are not compared with Rust',
    'ScheduleValueOutsideTypeLimits',
    'produce no TypedModel',
    'semantic blocking evidence, not numerical Schedule Value conformance',
    "script = `"$GateCommand`"",
    'blocking = true'
)) {
    Assert-Contains -Text $failureManifest -Pattern $contract -Description "failure manifest contract"
}
if ($failureManifest -match '(?m)^\s*level\s*=\s*"conformance"\s*$' -or
    $failureManifest -match '(?m)^\s*\[\[tolerances\]\]\s*$') {
    throw "Expected-failure manifest must not declare conformance-level or tolerance evidence."
}

$successIdfText = Normalize-Newlines -Text (Get-Content -LiteralPath $SuccessIdf -Raw -Encoding UTF8)
$failureIdfText = Normalize-Newlines -Text (Get-Content -LiteralPath $FailureIdf -Raw -Encoding UTF8)
$expectedFailureIdf = $successIdfText.Replace(
    "  0.875,`n  Continuous;",
    "  0.5,`n  Continuous;"
)
if ($expectedFailureIdf -ceq $successIdfText -or $expectedFailureIdf -cne $failureIdfText) {
    throw "Paired IDFs must differ only by the Continuous upper bound 0.875 versus 0.5."
}
foreach ($contract in @(
    "ScheduleTypeLimits,`n  Continuous,`n  0.0,`n  0.875,`n  Continuous;",
    "ExternalInterface:FunctionalMockupUnitExport:To:Schedule,`n  FMU Export Bounded Initial Value,`n  Continuous,`n  ProbeInput,`n  0.875;",
    "Output:Variable,`n  FMU EXPORT BOUNDED INITIAL VALUE,`n  Schedule Value,`n  Timestep;"
)) {
    Assert-Contains -Text $successIdfText -Pattern $contract -Description "success IDF exact object"
}
Assert-Contains -Text $failureIdfText -Pattern "ScheduleTypeLimits,`n  Continuous,`n  0.0,`n  0.5,`n  Continuous;" -Description "failure IDF exact bounds"
Write-Host "OK paired IDFs differ only by the upper bound."

$weatherLines = @(Get-Content -LiteralPath $WeatherPath -Encoding UTF8)
$weatherRows = @($weatherLines | Select-Object -Skip 8 | Where-Object { -not [string]::IsNullOrWhiteSpace($_) })
Assert-Equal -Actual $weatherLines.Count -Expected 32 -Description "EPW total row count"
Assert-Equal -Actual $weatherRows.Count -Expected 24 -Description "EPW hourly data row count"
Assert-Equal -Actual $weatherLines[4] -Expected "HOLIDAYS/DAYLIGHT SAVINGS,No,0,0,0" -Description "EPW no-holiday/DST header"
for ($index = 0; $index -lt 24; ++$index) {
    if ($weatherRows[$index] -notmatch "^2032,1,1,$($index + 1),60,") {
        throw "Unexpected EPW timestamp row at hour $($index + 1): $($weatherRows[$index])"
    }
}

Remove-RepoDirectory -Path $SuccessOutput
Remove-RepoDirectory -Path $FailureOutput
$cargo = Get-Command cargo -ErrorAction Stop
$successConsole = @(& $cargo.Source run -p ep_cli --quiet -- conformance time-weather-schedule-report $SuccessCase $OracleRoot $OutputRoot 2>&1)
if ($LASTEXITCODE -ne 0) {
    $successConsole | ForEach-Object { Write-Host $_ }
    throw "Inclusive-upper-bound conformance report failed."
}
$successConsole | ForEach-Object { Write-Host $_ }

$successSummaryPath = Join-Path $SuccessOutput "compare\compare-summary.json"
$successReportPath = Join-Path $SuccessOutput "compare\compare-report.md"
$successEsoPath = Join-Path $SuccessOutput "oracle\eplusout.eso"
$successEioPath = Join-Path $SuccessOutput "oracle\eplusout.eio"
$successErrPath = Join-Path $SuccessOutput "oracle\eplusout.err"
$successEndPath = Join-Path $SuccessOutput "oracle\eplusout.end"
$successEpjsonPath = Join-Path $SuccessOutput "oracle\input.epJSON"
foreach ($artifact in @(
    $successSummaryPath,
    $successReportPath,
    $successEsoPath,
    $successEioPath,
    $successErrPath,
    $successEndPath,
    $successEpjsonPath
)) {
    Assert-File -Path $artifact -Description "inclusive-upper-bound conformance artifact"
}

$summary = Get-Content -LiteralPath $successSummaryPath -Raw -Encoding UTF8 | ConvertFrom-Json
if ($summary.case_id -cne $SuccessId -or $summary.oracle_version -cne "26.1.0" -or
    $summary.comparison_class -cne "conformance" -or $summary.conformance_claim -ne $true -or
    $summary.status -cne "pass" -or $summary.series_count -ne 1 -or
    $summary.conformance_series_count -ne 1 -or $summary.time_axis_samples -ne 96 -or
    $summary.gate.script -cne $GateCommand -or $summary.gate.blocking -ne $true -or
    $null -ne $summary.weather_record_selection) {
    throw "Unexpected inclusive-upper-bound summary contract."
}
$seriesRows = @($summary.series | Where-Object {
    $_.key -eq "FMU EXPORT BOUNDED INITIAL VALUE" -and $_.variable -eq "Schedule Value"
})
Assert-Equal -Actual $seriesRows.Count -Expected 1 -Description "bounded schedule series count"
$series = $seriesRows[0]
$firstTimestamp = "env=FMU EXPORT BOUNDED INITIAL VALUE RUN PERIOD;day=1;month=1;date=1;dst=0;hour=1;start=0.00;end=15.00;day_type=Thursday"
$lastTimestamp = "env=FMU EXPORT BOUNDED INITIAL VALUE RUN PERIOD;day=1;month=1;date=1;dst=0;hour=24;start=45.00;end=60.00;day_type=Thursday"
if ($series.level -cne "conformance" -or $series.frequency -cne "timestep" -or
    $series.source -cne "eso" -or $series.alignment -cne "timestamp" -or
    $series.expected_samples -ne 96 -or $series.observed_samples -ne 96 -or
    $series.compared_samples -ne 96 -or $series.timestamp_contract -cne "ordered-exact-unique" -or
    $series.timestamp_status -cne "pass" -or $series.timestamp_expected_unique -ne $true -or
    $series.timestamp_observed_unique -ne $true -or $series.timestamp_order_match -ne $true -or
    $series.expected_first_timestamp -cne $firstTimestamp -or $series.observed_first_timestamp -cne $firstTimestamp -or
    $series.expected_last_timestamp -cne $lastTimestamp -or $series.observed_last_timestamp -cne $lastTimestamp -or
    $series.max_abs_tolerance -ne 0.0 -or $series.max_rmse_tolerance -ne 0.0 -or
    $series.max_abs_delta -ne 0.0 -or $series.rmse_delta -ne 0.0 -or
    $null -ne $series.first_divergence -or $null -ne $series.first_timestamp_divergence -or
    $series.status -cne "pass") {
    throw "Inclusive-upper-bound series must match all 96 values and ordered timestamps exactly."
}
Write-Host "OK inclusive upper endpoint produces one exact 96-sample series."

$converted = Get-Content -LiteralPath $successEpjsonPath -Raw -Encoding UTF8 | ConvertFrom-Json
$convertedLimit = $converted.ScheduleTypeLimits.Continuous
$convertedFamily = $converted."ExternalInterface:FunctionalMockupUnitExport:To:Schedule"
$convertedSchedule = $convertedFamily."ExternalInterface:FunctionalMockupUnitExport:To:Schedule 1"
if (@($convertedLimit.PSObject.Properties).Count -ne 3 -or
    [double]$convertedLimit.lower_limit_value -ne 0.0 -or
    [double]$convertedLimit.upper_limit_value -ne 0.875 -or
    $convertedLimit.numeric_type -cne "Continuous" -or
    @($convertedSchedule.PSObject.Properties).Count -ne 4 -or
    $convertedSchedule.schedule_name -cne "FMU Export Bounded Initial Value" -or
    $convertedSchedule.schedule_type_limits_names -cne "Continuous" -or
    $convertedSchedule.fmu_variable_name -cne "ProbeInput" -or
    [double]$convertedSchedule.initial_value -ne 0.875 -or
    @($converted.PSObject.Properties | Where-Object { $_.Name -ceq "ExternalInterface" }).Count -ne 0) {
    throw "Converted epJSON changed the exact bounded inactive schedule shape."
}

$successEso = @(Get-Content -LiteralPath $successEsoPath -Encoding UTF8)
$dictionary = @($successEso | Where-Object { $_ -match '^\d+,1,FMU EXPORT BOUNDED INITIAL VALUE,Schedule Value \[\] !TimeStep$' })
Assert-Equal -Actual $dictionary.Count -Expected 1 -Description "success ESO dictionary row count"
$reportId = ([regex]::Match($dictionary[0], '^(\d+),')).Groups[1].Value
$successValues = @($successEso | Where-Object { $_ -match ('^' + [regex]::Escape($reportId) + ',\s*[-+0-9.E]+\s*$') } |
    ForEach-Object { [double](($_ -split ',', 2)[1].Trim()) })
$successTimestamps = @($successEso | Where-Object { $_ -match '^2,\s*\d+,\s*\d+,\s*\d+,\s*\d+,\s*\d+,' })
Assert-Equal -Actual $successValues.Count -Expected 96 -Description "success raw Schedule Value sample count"
Assert-Equal -Actual $successTimestamps.Count -Expected 96 -Description "success raw timestamp count"
for ($index = 0; $index -lt 96; ++$index) {
    if ($successValues[$index] -ne 0.875) {
        throw "Unexpected success value at sample $index`: $($successValues[$index])"
    }
    $hour = [int][Math]::Floor($index / 4) + 1
    $startMinute = ($index % 4) * 15
    $endMinute = (($index % 4) + 1) * 15
    $timestamp = "2,1, 1, 1, 0,$('{0,2}' -f $hour),$('{0,5:N2}' -f $startMinute),$('{0,5:N2}' -f $endMinute),Thursday"
    if (($successTimestamps[$index] -replace '\s+', '') -cne ($timestamp -replace '\s+', '')) {
        throw "Unexpected success timestamp at sample $index`: $($successTimestamps[$index])"
    }
}

$successEio = @(Get-Content -LiteralPath $successEioPath -Encoding UTF8)
$environmentRow = "Environment,FMU EXPORT BOUNDED INITIAL VALUE RUN PERIOD,WeatherFileRunPeriod,01/01/2032,01/01/2032,Thursday,1,Use RunPeriod Specified Day,No,No,No,No,No,Clark and Allen"
Assert-Equal -Actual @($successEio | Where-Object { $_ -ceq $environmentRow }).Count -Expected 1 -Description "success EIO Environment row count"
Assert-Equal -Actual @($successEio | Where-Object { $_ -ceq "Environment:Daylight Saving,No,RunPeriod Object" }).Count -Expected 1 -Description "success EIO disabled DST row count"
$successErr = Get-Content -LiteralPath $successErrPath -Raw -Encoding UTF8
$successEnd = Get-Content -LiteralPath $successEndPath -Raw -Encoding UTF8
$warningHeader = '   ** Warning ** IDF file contains object "ExternalInterface:FunctionalMockupUnitExport:To:Schedule",'
$warningContinuation = '   **   ~~~   ** but object "ExternalInterface" with appropriate key entry is not specified. Values will not be updated.'
Assert-Equal -Actual ([regex]::Matches($successErr, [regex]::Escape($warningHeader))).Count -Expected 1 -Description "success missing-activation warning header count"
Assert-Equal -Actual ([regex]::Matches($successErr, [regex]::Escape($warningContinuation))).Count -Expected 1 -Description "success missing-activation warning continuation count"
$successCompletion = "EnergyPlus Completed Successfully-- 1 Warning; 0 Severe Errors;"
Assert-Contains -Text $successErr -Pattern $successCompletion -Description "success ERR completion"
Assert-Contains -Text $successEnd -Pattern $successCompletion -Description "success END completion"

New-Item -ItemType Directory -Force -Path $FailureOracle, $FailureCompare | Out-Null
$energyplus = Join-Path $OracleRoot "energyplus.exe"
$savedPreference = $ErrorActionPreference
$ErrorActionPreference = "Continue"
$failureOracleConsole = @(& $energyplus -w $WeatherPath -d $FailureOracle $FailureIdf 2>&1)
$failureOracleExit = $LASTEXITCODE
$ErrorActionPreference = $savedPreference
if ($failureOracleExit -eq 0) {
    $failureOracleConsole | ForEach-Object { Write-Host $_ }
    throw "Expected EnergyPlus to reject the upper-bound violation."
}
$failureErrPath = Join-Path $FailureOracle "eplusout.err"
$failureEndPath = Join-Path $FailureOracle "eplusout.end"
$failureEsoPath = Join-Path $FailureOracle "eplusout.eso"
foreach ($artifact in @($failureErrPath, $failureEndPath, $failureEsoPath)) {
    Assert-File -Path $artifact -Description "EnergyPlus upper-bound failure artifact"
}
$failureErr = Get-Content -LiteralPath $failureErrPath -Raw -Encoding UTF8
$failureEnd = Get-Content -LiteralPath $failureEndPath -Raw -Encoding UTF8
$failureCompletion = "EnergyPlus Terminated--Fatal Error Detected. 0 Warning; 1 Severe Errors;"
Assert-Contains -Text $failureErr -Pattern $failureCompletion -Description "failure ERR 0-warning/1-severe completion"
Assert-Contains -Text $failureEnd -Pattern $failureCompletion -Description "failure END 0-warning/1-severe completion"
Assert-Contains -Text $failureErr -Pattern "ProcessScheduleInput: Schedule = FMU EXPORT BOUNDED INITIAL VALUE" -Description "failure ProcessScheduleInput ownership"
Assert-Contains -Text $failureErr -Pattern "ProcessScheduleInput: Preceding Errors cause termination." -Description "failure ProcessScheduleInput fatal"
if ($failureErr.Contains("EnergyPlus Completed Successfully")) {
    throw "Upper-bound violation must not report successful EnergyPlus completion."
}
$failureEso = @(Get-Content -LiteralPath $failureEsoPath -Encoding UTF8)
$failureDictionary = @($failureEso | Where-Object { $_ -match '^\d+,1,FMU EXPORT BOUNDED INITIAL VALUE,Schedule Value \[\] !TimeStep$' })
$failureReportId = $null
if ($failureDictionary.Count -eq 1) {
    $failureReportId = ([regex]::Match($failureDictionary[0], '^(\d+),')).Groups[1].Value
}
$failureValues = @(
    if ($null -ne $failureReportId) {
        $failureEso | Where-Object { $_ -match ('^' + [regex]::Escape($failureReportId) + ',\s*[-+0-9.E]+\s*$') }
    }
)
$failureTimestamps = @($failureEso | Where-Object { $_ -match '^2,\s*\d+,\s*\d+,\s*\d+,\s*\d+,\s*\d+,' })
Assert-Equal -Actual $failureValues.Count -Expected 0 -Description "failure EnergyPlus Schedule Value data count"
Assert-Equal -Actual $failureTimestamps.Count -Expected 0 -Description "failure EnergyPlus timestamp data count"

$buildConsole = @(& $cargo.Source build -p ep_cli --quiet 2>&1)
if ($LASTEXITCODE -ne 0) {
    $buildConsole | ForEach-Object { Write-Host $_ }
    throw "Failed to build ep_cli for the Rust rejection lane."
}
$exe = Join-Path $RepoRoot "target\debug\eplus-rs.exe"
Assert-File -Path $exe -Description "built eplus-rs CLI"
foreach ($casePath in @($SuccessCase, $FailureCase)) {
    $validation = @(& $exe conformance validate-case-v2 $casePath 2>&1)
    if ($LASTEXITCODE -ne 0) {
        $validation | ForEach-Object { Write-Host $_ }
        throw "Manifest v2 validation failed: $casePath"
    }
}

$ErrorActionPreference = "Continue"
$failureRustConsole = @(& $exe run $FailureIdf -w $WeatherPath -d $FailureRust --mode compatibility --partial allow --format rust-native --trace-level normal --overwrite --oracle-root $OracleRoot 2>&1)
$failureRustExit = $LASTEXITCODE
$ErrorActionPreference = $savedPreference
if ($failureRustExit -eq 0) {
    $failureRustConsole | ForEach-Object { Write-Host $_ }
    throw "Expected Rust typed compilation to reject the upper-bound violation."
}
$rustSummaryPath = Join-Path $FailureRust "run-summary.json"
$rustDiagnosticsPath = Join-Path $FailureRust "diagnostics.json"
$rustTypedPath = Join-Path $FailureRust "model\typed-model-summary.json"
foreach ($artifact in @($rustSummaryPath, $rustDiagnosticsPath, $rustTypedPath)) {
    Assert-File -Path $artifact -Description "Rust upper-bound failure artifact"
}
$rustSummary = Get-Content -LiteralPath $rustSummaryPath -Raw -Encoding UTF8 | ConvertFrom-Json
$rustTyped = Get-Content -LiteralPath $rustTypedPath -Raw -Encoding UTF8 | ConvertFrom-Json
$rustDiagnostics = Get-Content -LiteralPath $rustDiagnosticsPath -Raw -Encoding UTF8 | ConvertFrom-Json
Assert-Equal -Actual $rustSummary.status -Expected "compile-reference" -Description "Rust blocked compile status"
Assert-Equal -Actual $rustTyped.status -Expected "failed" -Description "Rust typed model absence status"
if ($null -ne $rustSummary.rust_runtime -or $null -ne $rustSummary.source_order_gate) {
    throw "Rust upper-bound failure must enter no runtime schedule lane."
}
$limitDiagnostics = @($rustDiagnostics.diagnostics | Where-Object {
    $_.severity -eq "error" -and $_.code -eq "ScheduleValueOutsideTypeLimits" -and
    $_.stage -eq "compile" -and
    $_.object_type -eq "ExternalInterface:FunctionalMockupUnitExport:To:Schedule" -and
    $_.object_name -eq "FMU EXPORT BOUNDED INITIAL VALUE" -and $_.field -eq "initial_value"
})
if ($limitDiagnostics.Count -lt 1) {
    throw "Rust compile artifacts must contain ScheduleValueOutsideTypeLimits for the FMU Export initial value."
}
foreach ($forbidden in @(
    (Join-Path $FailureRust "model\graph-summary.json"),
    (Join-Path $FailureRust "model\execution-plan.json"),
    (Join-Path $FailureRust "results\result-store.json"),
    (Join-Path $FailureRust "results\selected-outputs.csv")
)) {
    if (Test-Path -LiteralPath $forbidden) {
        throw "Rust compile rejection must not produce model/runtime sample artifact: $forbidden"
    }
}
Write-Host "OK Rust rejected before TypedModel, execution plan, or runtime samples."

$semanticSummary = [ordered]@{
    schema_version = 1
    case_id = $FailureId
    status = "pass"
    evidence_class = "expected-failure-smoke"
    conformance_claim = $false
    pair_success_case = $SuccessId
    input = [ordered]@{
        schedule_family = "ExternalInterface:FunctionalMockupUnitExport:To:Schedule"
        schedule_name = "FMU Export Bounded Initial Value"
        initial_value = 0.875
        lower_bound = 0.0
        upper_bound = 0.5
        numeric_type = "Continuous"
    }
    oracle = [ordered]@{
        engine = "EnergyPlus 26.1.0"
        process_failed = $true
        warnings = 0
        severe_errors = 1
        schedule_value_samples = $failureValues.Count
        timestamp_samples = $failureTimestamps.Count
    }
    rust = [ordered]@{
        engine = "rusted-energyplus"
        process_failed = $true
        compile_status = [string]$rustSummary.status
        typed_model_status = [string]$rustTyped.status
        schedule_limit_diagnostic_present = ($limitDiagnostics.Count -ge 1)
        rust_runtime_is_null = ($null -eq $rustSummary.rust_runtime)
        result_store_exists = (Test-Path -LiteralPath (Join-Path $FailureRust "results\result-store.json"))
    }
    semantic_comparison = [ordered]@{
        status = "pass"
        shared_behavior = "reject Initial Value 0.875 above resolved Continuous upper bound 0.5 before schedule samples"
        numeric_output_conformance_claimed = $false
        diagnostic_text_parity_claimed = $false
        diagnostic_count_parity_claimed = $false
        process_exit_code_parity_claimed = $false
    }
}
$utf8NoBom = New-Object System.Text.UTF8Encoding($false)
[System.IO.File]::WriteAllText(
    $NegativeSummary,
    ($semanticSummary | ConvertTo-Json -Depth 10) + "`n",
    $utf8NoBom
)
$negativeMarkdown = @(
    "# FMU Export Bounded Initial-Value Expected-Failure Report",
    "",
    "- case: ``$FailureId``",
    "- paired success: ``$SuccessId``",
    "- status: pass",
    "- evidence class: expected-failure smoke",
    "- conformance claim: false",
    "",
    "Both engines reject Initial Value ``0.875`` above the resolved Continuous upper bound ``0.5`` before Schedule Value samples. EnergyPlus reports 0 warnings, 1 severe error, and zero value/timestamp data rows. Rust blocks typed compilation and produces no TypedModel, execution plan, result store, or runtime samples.",
    "",
    "Diagnostic text, diagnostic-count equality, and numeric process-exit equality are not claimed. The paired success case alone promotes the inclusive ``[0.0,0.875]`` upper endpoint through 96 exact values and timestamps.",
    "",
    "The boundary excludes resolved Compact/File/Year minmax, Day warning and Discrete-integer behavior, one-sided/reversed/blank numeric bounds, unit_type, unknown-reference parity, multiple violations, EMS/currentVal or live updates, and cross-family duplicate source-order parity."
) -join "`n"
[System.IO.File]::WriteAllText($NegativeReport, $negativeMarkdown + "`n", $utf8NoBom)
Assert-File -Path $NegativeReport -Description "negative markdown report"
Assert-File -Path $NegativeSummary -Description "negative JSON summary"

Write-Host "Bounded scalar ScheduleTypeLimits paired gate passed."
Write-Host "  success: $successReportPath"
Write-Host "  failure: $NegativeReport"
