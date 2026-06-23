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

$runSummary = Get-Content -Encoding UTF8 -Raw -LiteralPath (Join-Path $outputDir "run-summary.json") | ConvertFrom-Json
Assert-Equal -Actual $runSummary.status -Expected "oracle-compare" -Description "run summary status"
Assert-Equal -Actual $runSummary.exit_code -Expected 8 -Description "run summary exit code"
Assert-Equal -Actual $runSummary.config.partial_policy -Expected "deny" -Description "partial policy"
Assert-Equal -Actual $runSummary.support.status -Expected "supported-compatibility" -Description "support status"
Assert-Equal -Actual $runSummary.support.run_result_state -Expected "supported_compatibility_run" -Description "run result state"
Assert-Equal -Actual $runSummary.support.capability_registry_loaded -Expected $true -Description "run summary capability registry loaded"
Assert-Equal -Actual $runSummary.support.conformance_claim -Expected $false -Description "run conformance claim"
Assert-Equal -Actual $runSummary.oracle_status -Expected "generated" -Description "oracle status"
Assert-Equal -Actual $runSummary.compare_status -Expected "fail" -Description "compare status"

$support = Get-Content -Encoding UTF8 -Raw -LiteralPath (Join-Path $outputDir "support-assessment.json") | ConvertFrom-Json
Assert-Equal -Actual $support.run_result_state -Expected "supported_compatibility_run" -Description "support run result state"
Assert-Equal -Actual $support.partial_policy -Expected "deny" -Description "support partial policy"
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

Write-Host "Arbitrary IDF run smoke passed. Artifacts: $outputDir"
