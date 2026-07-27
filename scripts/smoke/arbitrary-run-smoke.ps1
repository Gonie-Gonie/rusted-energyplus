[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
Set-Location $RepoRoot

function Assert-File {
    param([Parameter(Mandatory = $true)][string]$Path)
    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        throw "Missing expected artifact: $Path"
    }
    Write-Host "OK artifact: $Path"
}

function Assert-Equal {
    param(
        [Parameter(Mandatory = $true)]$Actual,
        [Parameter(Mandatory = $true)]$Expected,
        [Parameter(Mandatory = $true)][string]$Description
    )
    if ($Actual -ne $Expected) {
        throw "Unexpected $Description`: expected $Expected, got $Actual"
    }
    Write-Host "OK $Description`: $Actual"
}

function Assert-Matches {
    param(
        [Parameter(Mandatory = $true)][string]$Text,
        [Parameter(Mandatory = $true)][string]$Pattern,
        [Parameter(Mandatory = $true)][string]$Description
    )
    if ($Text -notmatch $Pattern) {
        throw "Expected $Description to match pattern: $Pattern"
    }
    Write-Host "OK $Description"
}

function Assert-SnapshotPresent {
    param(
        [Parameter(Mandatory = $true)][object[]]$Snapshots,
        [Parameter(Mandatory = $true)][string]$StageName,
        [Parameter(Mandatory = $true)][string]$Point,
        [string]$Substage = "",
        [Parameter(Mandatory = $true)][string]$Description
    )

    $matches = @(
        $Snapshots | Where-Object {
            $_.stage_name -eq $StageName -and
            $_.point -eq $Point -and
            ([string]::IsNullOrWhiteSpace($Substage) -or $_.substage -eq $Substage)
        }
    )
    Assert-Equal -Actual $matches.Count -Expected 1 -Description $Description
}

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

$oracleRoot = ".runtime\energyplus\26.1.0"
$idf = Join-Path $oracleRoot "ExampleFiles\1ZoneUncontrolled.idf"
$weather = Join-Path $oracleRoot "WeatherData\USA_CO_Golden-NREL.724666_TMY3.epw"
$energyplusExe = Join-Path $oracleRoot "energyplus.exe"
$convertExe = Join-Path $oracleRoot "ConvertInputFormat.exe"
foreach ($required in @($idf, $weather, $energyplusExe, $convertExe)) {
    if (-not (Test-Path -LiteralPath $required -PathType Leaf)) {
        throw "Missing arbitrary-run smoke prerequisite: $required. Run .\scripts\dev.cmd setup first."
    }
}

$outputDir = ".runtime\arbitrary-run-smoke-script"
Write-Host "Building eplus-rs CLI for direct exit-code validation."
& $cargo.Source build -p ep_cli --quiet
if ($LASTEXITCODE -ne 0) {
    throw "Failed to build ep_cli."
}

$exe = ".\target\debug\eplus-rs.exe"
if (-not (Test-Path -LiteralPath $exe -PathType Leaf)) {
    throw "Missing built CLI binary: $exe"
}

Write-Host "Running arbitrary IDF smoke with oracle compare: $idf"
$output = & $exe run $idf -w $weather -d $outputDir --overwrite --compare-oracle 2>&1
$exitCode = $LASTEXITCODE
if ($exitCode -ne 8) {
    $output | ForEach-Object { Write-Host $_ }
    throw "Expected oracle-compare exit code 8 from direct eplus-rs run, got $exitCode."
}

$expectedArtifacts = @(
    "eplusrs.err",
    "diagnostics.json",
    "run-summary.json",
    "support-assessment.json",
    "support-report.md",
    "input\original.idf",
    "input\converted.epJSON",
    "input\input-hashes.json",
    "model\raw-model-summary.json",
    "model\typed-model-summary.json",
    "model\graph-summary.json",
    "model\execution-plan.json",
    "results\result-store.json",
    "results\selected-outputs.csv",
    "results\meters.csv",
    "reports\run-report.md",
    "reports\compatibility-boundary.md",
    "logs\command.log",
    "oracle\eplusout.eso",
    "compare\compare-summary.json",
    "compare\compare-report.md"
)
foreach ($relative in $expectedArtifacts) {
    Assert-File -Path (Join-Path $outputDir $relative)
}

$executionPlan = Get-Content -Encoding UTF8 -Raw -LiteralPath (Join-Path $outputDir "model\execution-plan.json") | ConvertFrom-Json
Assert-Equal -Actual @($executionPlan.stages)[0].kind -Expected "get_heat_balance_input" -Description "execution plan first source-order kind"
Assert-Equal -Actual @($executionPlan.stages)[4].kind -Expected "manage_surface_heat_balance" -Description "execution plan source-order surface manager kind"
Assert-Equal -Actual @($executionPlan.stages)[9].kind -Expected "manage_zone_air_updates" -Description "execution plan source-order zone air update kind"
Assert-Equal -Actual @($executionPlan.compatibility_stages)[0].kind -Expected "get_heat_balance_input" -Description "execution plan source-order first kind"
Assert-Equal -Actual @($executionPlan.compatibility_stages)[4].kind -Expected "manage_surface_heat_balance" -Description "execution plan source-order surface manager kind"
Assert-Equal -Actual @($executionPlan.compatibility_stages)[9].kind -Expected "manage_zone_air_updates" -Description "execution plan source-order zone air update compatibility kind"
Assert-Equal -Actual @($executionPlan.expected_source_order_stages)[0] -Expected "get-heat-balance-input" -Description "execution plan expected source-order first stage"
Assert-Equal -Actual @($executionPlan.actual_executed_source_order_stages)[0] -Expected "get-heat-balance-input" -Description "execution plan actual source-order first stage"
Assert-Equal -Actual @($executionPlan.actual_executed_source_order_stages).Count -Expected @($executionPlan.expected_source_order_stages).Count -Description "execution plan expected/actual source-order count"

$traceOutputDir = ".runtime\arbitrary-run-detailed-trace-smoke-script"
Write-Host "Running detailed trace dry-run smoke: $idf"
$traceOutput = & $exe run $idf -w $weather -d $traceOutputDir --overwrite --dry-run --trace-level detailed 2>&1
$traceExitCode = $LASTEXITCODE
if ($traceExitCode -ne 0) {
    $traceOutput | ForEach-Object { Write-Host $_ }
    throw "Expected dry-run exit code 0 from detailed trace run, got $traceExitCode."
}
Assert-File -Path (Join-Path $traceOutputDir "logs\source-order-stage-state-snapshots.json")
$stageStateSnapshots = Get-Content -Encoding UTF8 -Raw -LiteralPath (Join-Path $traceOutputDir "logs\source-order-stage-state-snapshots.json") | ConvertFrom-Json
Assert-Equal -Actual $stageStateSnapshots.schema_version -Expected 1 -Description "stage state snapshot schema version"
Assert-Equal -Actual $stageStateSnapshots.artifact_class -Expected "diagnostic-trace" -Description "stage state snapshot artifact class"
foreach ($stage in @(
    "init-heat-balance",
    "calc-heat-balance-outside-surf",
    "calc-heat-balance-inside-surf",
    "manage-air-heat-balance",
    "update-thermal-histories",
    "report-surface-heat-balance"
)) {
    Assert-SnapshotPresent -Snapshots @($stageStateSnapshots.snapshots) -StageName $stage -Point "before" -Description "$stage before snapshot"
    Assert-SnapshotPresent -Snapshots @($stageStateSnapshots.snapshots) -StageName $stage -Point "after" -Description "$stage after snapshot"
}
Assert-SnapshotPresent -Snapshots @($stageStateSnapshots.snapshots) -StageName "manage-zone-air-updates" -Point "before" -Substage "ZoneTempPredictorCorrector::PredictStep" -Description "PredictStep before snapshot"
Assert-SnapshotPresent -Snapshots @($stageStateSnapshots.snapshots) -StageName "manage-zone-air-updates" -Point "after" -Substage "ZoneTempPredictorCorrector::PredictStep" -Description "PredictStep after snapshot"
Assert-SnapshotPresent -Snapshots @($stageStateSnapshots.snapshots) -StageName "manage-zone-air-updates" -Point "before" -Substage "ZoneTempPredictorCorrector::CorrectStep" -Description "CorrectStep before snapshot"
Assert-SnapshotPresent -Snapshots @($stageStateSnapshots.snapshots) -StageName "manage-zone-air-updates" -Point "after" -Substage "ZoneTempPredictorCorrector::CorrectStep" -Description "CorrectStep after snapshot"

$idealLoadsTraceOutputDir = ".runtime\arbitrary-run-ideal-loads-detailed-trace-smoke-script"
$idealLoadsTraceInput = "data\conformance_cases\ideal_loads_flow_limit_diagnostic_001\ideal_loads_flow_limit_diagnostic.idf"
Write-Host "Running IdealLoads detailed trace dry-run smoke: $idealLoadsTraceInput"
$idealLoadsTraceOutput = & $exe run $idealLoadsTraceInput -d $idealLoadsTraceOutputDir --mode compatibility --partial deny --format rust-native --trace-level detailed --dry-run --overwrite 2>&1
$idealLoadsTraceExitCode = $LASTEXITCODE
if ($idealLoadsTraceExitCode -ne 0) {
    $idealLoadsTraceOutput | ForEach-Object { Write-Host $_ }
    throw "Expected dry-run exit code 0 from IdealLoads detailed trace run, got $idealLoadsTraceExitCode."
}
Assert-File -Path (Join-Path $idealLoadsTraceOutputDir "logs\source-order-stage-state-snapshots.json")
$idealLoadsStageStateSnapshots = Get-Content -Encoding UTF8 -Raw -LiteralPath (Join-Path $idealLoadsTraceOutputDir "logs\source-order-stage-state-snapshots.json") | ConvertFrom-Json
Assert-Equal -Actual $idealLoadsStageStateSnapshots.schema_version -Expected 1 -Description "IdealLoads stage state snapshot schema version"
Assert-Equal -Actual $idealLoadsStageStateSnapshots.artifact_class -Expected "diagnostic-trace" -Description "IdealLoads stage state snapshot artifact class"
foreach ($stage in @(
    "sim-purchased-air",
    "calc-purch-air-loads",
    "update-purchased-air",
    "report-purchased-air"
)) {
    Assert-SnapshotPresent -Snapshots @($idealLoadsStageStateSnapshots.snapshots) -StageName $stage -Point "before" -Description "$stage before snapshot"
    Assert-SnapshotPresent -Snapshots @($idealLoadsStageStateSnapshots.snapshots) -StageName $stage -Point "after" -Description "$stage after snapshot"
}

$runSummary = Get-Content -Encoding UTF8 -Raw -LiteralPath (Join-Path $outputDir "run-summary.json") | ConvertFrom-Json
Assert-Equal -Actual $runSummary.status -Expected "oracle-compare" -Description "run summary status"
Assert-Equal -Actual $runSummary.exit_code -Expected 8 -Description "run summary exit code"
Assert-Equal -Actual $runSummary.config.partial_policy -Expected "deny" -Description "partial policy"
Assert-Equal -Actual $runSummary.support.status -Expected "supported-compatibility" -Description "support status"
Assert-Equal -Actual $runSummary.support.run_result_state -Expected "supported_compatibility_run" -Description "run result state"
Assert-Equal -Actual $runSummary.selected_algorithm_lane.id -Expected "compatibility-source-order" -Description "selected algorithm lane"
Assert-Equal -Actual $runSummary.selected_algorithm_lane.diagnostic_probe_used -Expected $false -Description "selected algorithm lane diagnostic probe boundary"
Assert-Equal -Actual $runSummary.selected_algorithm_lane.conformance_promotion_allowed -Expected $true -Description "selected algorithm lane conformance promotion"
Assert-Equal -Actual $runSummary.support.selected_algorithm_lane.id -Expected "compatibility-source-order" -Description "support selected algorithm lane"
Assert-Equal -Actual $runSummary.support.capability_registry_loaded -Expected $true -Description "run summary capability registry loaded"
Assert-Equal -Actual $runSummary.support.conformance_claim -Expected $false -Description "run conformance claim"
Assert-Equal -Actual $runSummary.oracle_status -Expected "generated" -Description "oracle status"
Assert-Equal -Actual $runSummary.compare_status -Expected "fail" -Description "compare status"

$support = Get-Content -Encoding UTF8 -Raw -LiteralPath (Join-Path $outputDir "support-assessment.json") | ConvertFrom-Json
Assert-Equal -Actual $support.run_result_state -Expected "supported_compatibility_run" -Description "support run result state"
Assert-Equal -Actual $support.partial_policy -Expected "deny" -Description "support partial policy"
Assert-Equal -Actual $support.selected_algorithm_lane.id -Expected "compatibility-source-order" -Description "support assessment selected algorithm lane"
Assert-Equal -Actual $support.selected_algorithm_lane.diagnostic_probe_used -Expected $false -Description "support assessment diagnostic probe boundary"
Assert-Equal -Actual $support.selected_algorithm_lane.conformance_promotion_allowed -Expected $true -Description "support assessment conformance promotion"
Assert-Equal -Actual $support.capability_registry -Expected "specs/capabilities.toml" -Description "capability registry"
Assert-Equal -Actual $support.capability_registry_loaded -Expected $true -Description "support capability registry loaded"
Assert-Equal -Actual $support.claim_boundary.conformance_claim -Expected $false -Description "support conformance claim"

$compare = Get-Content -Encoding UTF8 -Raw -LiteralPath (Join-Path $outputDir "compare\compare-summary.json") | ConvertFrom-Json
Assert-Equal -Actual $compare.status -Expected "fail" -Description "comparison status"
Assert-Equal -Actual $compare.conformance_claim -Expected $false -Description "comparison conformance claim"
$matchedSeries = @($compare.series | Where-Object { $_.compared_samples -gt 0 })
if ($matchedSeries.Count -eq 0) {
    throw "Expected at least one matched oracle/Rust series."
}
foreach ($series in $matchedSeries) {
    Assert-Equal -Actual $series.oracle_samples -Expected $series.rust_samples -Description "sample count for $($series.key) / $($series.variable_name)"
}

$blockedOutputDir = ".runtime\arbitrary-run-blocked-smoke-script"
$blockedInput = "data\testcases\minimal\plant-loop-skeleton.epJSON"
Write-Host "Running blocked arbitrary IDF smoke: $blockedInput"
$blockedOutput = & $exe run $blockedInput -d $blockedOutputDir --overwrite 2>&1
$blockedExitCode = $LASTEXITCODE
if ($blockedExitCode -ne 4) {
    $blockedOutput | ForEach-Object { Write-Host $_ }
    throw "Expected unsupported exit code 4 from blocked eplus-rs run, got $blockedExitCode."
}

foreach ($relative in @(
    "eplusrs.err",
    "diagnostics.json",
    "run-summary.json",
    "support-assessment.json",
    "support-report.md",
    "input\original.epJSON",
    "input\converted.epJSON",
    "model\raw-model-summary.json",
    "model\typed-model-summary.json",
    "reports\run-report.md",
    "reports\compatibility-boundary.md"
)) {
    Assert-File -Path (Join-Path $blockedOutputDir $relative)
}

$blockedSummary = Get-Content -Encoding UTF8 -Raw -LiteralPath (Join-Path $blockedOutputDir "run-summary.json") | ConvertFrom-Json
Assert-Equal -Actual $blockedSummary.status -Expected "unsupported" -Description "blocked run summary status"
Assert-Equal -Actual $blockedSummary.exit_code -Expected 4 -Description "blocked run summary exit code"
Assert-Equal -Actual $blockedSummary.support.status -Expected "unsupported" -Description "blocked support status"
Assert-Equal -Actual $blockedSummary.support.run_result_state -Expected "run_blocked" -Description "blocked run result state"
Assert-Equal -Actual $blockedSummary.support.runtime_class -Expected "none" -Description "blocked runtime class"
if ($null -ne $blockedSummary.rust_runtime) {
    throw "Blocked run must not write a rust_runtime summary."
}
if (Test-Path -LiteralPath (Join-Path $blockedOutputDir "results\result-store.json")) {
    throw "Blocked run must not write result-store.json."
}

$blockedSupport = Get-Content -Encoding UTF8 -Raw -LiteralPath (Join-Path $blockedOutputDir "support-assessment.json") | ConvertFrom-Json
Assert-Equal -Actual $blockedSupport.run_result_state -Expected "run_blocked" -Description "blocked support run result state"
Assert-Equal -Actual $blockedSupport.unsupported_objects[0].object_type -Expected "PlantLoop/PlantEquipment" -Description "blocked unsupported object"
Assert-Equal -Actual $blockedSupport.claim_boundary.conformance_claim -Expected $false -Description "blocked support conformance claim"

$blockedOracleOutputDir = ".runtime\arbitrary-run-blocked-oracle-smoke-script"
$blockedOracleInput = ".runtime\arbitrary-run-blocked-oracle-smoke.idf"
if (Test-Path -LiteralPath $blockedOracleOutputDir) {
    Remove-Item -Recurse -Force -LiteralPath $blockedOracleOutputDir
}
$blockedOracleInputText = Get-Content -Encoding UTF8 -Raw -LiteralPath $idf
$blockedOracleInputText += @"

EnergyManagementSystem:Program,
  BlockedOracleSmokeProgram,  !- Name
  SET BlockedOracleSmokeValue = 1;  !- Program Line 1
"@
Set-Content -Encoding UTF8 -LiteralPath $blockedOracleInput -Value $blockedOracleInputText

Write-Host "Running blocked arbitrary IDF smoke with oracle baseline: $blockedOracleInput"
$blockedOracleOutput = & $exe run $blockedOracleInput -w $weather -d $blockedOracleOutputDir --overwrite --oracle-baseline --compare-oracle --oracle-root $oracleRoot 2>&1
$blockedOracleExitCode = $LASTEXITCODE
if ($blockedOracleExitCode -ne 4) {
    $blockedOracleOutput | ForEach-Object { Write-Host $_ }
    throw "Expected unsupported exit code 4 from blocked oracle eplus-rs run, got $blockedOracleExitCode."
}

foreach ($relative in @(
    "eplusrs.err",
    "diagnostics.json",
    "run-summary.json",
    "support-assessment.json",
    "support-report.md",
    "input\original.idf",
    "input\converted.epJSON",
    "model\raw-model-summary.json",
    "model\typed-model-summary.json",
    "reports\run-report.md",
    "reports\compatibility-boundary.md",
    "oracle\eplusout.eso",
    "oracle\eplusout.eio",
    "oracle\eplusout.err",
    "compare\compare-summary.json",
    "compare\compare-report.md"
)) {
    Assert-File -Path (Join-Path $blockedOracleOutputDir $relative)
}

$blockedOracleSummary = Get-Content -Encoding UTF8 -Raw -LiteralPath (Join-Path $blockedOracleOutputDir "run-summary.json") | ConvertFrom-Json
Assert-Equal -Actual $blockedOracleSummary.status -Expected "unsupported" -Description "blocked oracle run summary status"
Assert-Equal -Actual $blockedOracleSummary.exit_code -Expected 4 -Description "blocked oracle run summary exit code"
Assert-Equal -Actual $blockedOracleSummary.oracle_status -Expected "generated" -Description "blocked oracle status"
Assert-Equal -Actual $blockedOracleSummary.compare_status -Expected "skipped-rust-unsupported-or-oracle-missing" -Description "blocked oracle compare status"
Assert-Equal -Actual $blockedOracleSummary.support.status -Expected "unsupported" -Description "blocked oracle support status"
Assert-Equal -Actual $blockedOracleSummary.support.run_result_state -Expected "run_blocked" -Description "blocked oracle run result state"
Assert-Equal -Actual $blockedOracleSummary.support.runtime_class -Expected "none" -Description "blocked oracle runtime class"
if ($null -eq $blockedOracleSummary.oracle) {
    throw "Blocked run with --oracle-baseline must still write an oracle summary."
}
if ($null -ne $blockedOracleSummary.rust_runtime) {
    throw "Blocked run with --oracle-baseline must not write a rust_runtime summary."
}
if (Test-Path -LiteralPath (Join-Path $blockedOracleOutputDir "results\result-store.json")) {
    throw "Blocked run with --oracle-baseline must not write result-store.json."
}

$blockedOracleSupport = Get-Content -Encoding UTF8 -Raw -LiteralPath (Join-Path $blockedOracleOutputDir "support-assessment.json") | ConvertFrom-Json
Assert-Equal -Actual $blockedOracleSupport.run_result_state -Expected "run_blocked" -Description "blocked oracle support run result state"
Assert-Equal -Actual $blockedOracleSupport.unsupported_objects[0].object_type -Expected "EnergyManagementSystem:Program" -Description "blocked oracle unsupported object"
Assert-Equal -Actual $blockedOracleSupport.claim_boundary.conformance_claim -Expected $false -Description "blocked oracle support conformance claim"

$blockedOracleCompare = Get-Content -Encoding UTF8 -Raw -LiteralPath (Join-Path $blockedOracleOutputDir "compare\compare-summary.json") | ConvertFrom-Json
Assert-Equal -Actual $blockedOracleCompare.status -Expected "skipped-rust-unsupported-or-oracle-missing" -Description "blocked oracle comparison status"
Assert-Equal -Actual $blockedOracleCompare.conformance_claim -Expected $false -Description "blocked oracle comparison conformance claim"

$invalidIdfOutputDir = ".runtime\arbitrary-run-invalid-idf-smoke-script"
$invalidIdfInput = ".runtime\arbitrary-run-invalid-idf-smoke.idf"
if (Test-Path -LiteralPath $invalidIdfOutputDir) {
    Remove-Item -Recurse -Force -LiteralPath $invalidIdfOutputDir
}
Set-Content -Encoding UTF8 -LiteralPath $invalidIdfInput -Value @"
Version,
  26.1;

Building,
"@

Write-Host "Running invalid IDF conversion smoke: $invalidIdfInput"
$invalidIdfOutput = & $exe run $invalidIdfInput -d $invalidIdfOutputDir --overwrite --oracle-root $oracleRoot 2>&1
$invalidIdfExitCode = $LASTEXITCODE
if ($invalidIdfExitCode -ne 2) {
    $invalidIdfOutput | ForEach-Object { Write-Host $_ }
    throw "Expected import-parse exit code 2 from invalid IDF conversion, got $invalidIdfExitCode."
}

foreach ($relative in @(
    "eplusrs.err",
    "diagnostics.json",
    "run-summary.json",
    "input\original.idf"
)) {
    Assert-File -Path (Join-Path $invalidIdfOutputDir $relative)
}

$invalidIdfSummary = Get-Content -Encoding UTF8 -Raw -LiteralPath (Join-Path $invalidIdfOutputDir "run-summary.json") | ConvertFrom-Json
Assert-Equal -Actual $invalidIdfSummary.status -Expected "import-parse" -Description "invalid IDF run summary status"
Assert-Equal -Actual $invalidIdfSummary.exit_code -Expected 2 -Description "invalid IDF run summary exit code"
Assert-Equal -Actual $invalidIdfSummary.run_result_state -Expected "run_blocked" -Description "invalid IDF run result state"
Assert-Equal -Actual $invalidIdfSummary.support_status -Expected "unsupported" -Description "invalid IDF support status"
Assert-Matches -Text (Get-Content -Encoding UTF8 -Raw -LiteralPath (Join-Path $invalidIdfOutputDir "diagnostics.json")) -Pattern "ConvertInputFormatFailed" -Description "invalid IDF conversion diagnostic"

Write-Host "Arbitrary IDF run smoke passed. Artifacts: $outputDir"
