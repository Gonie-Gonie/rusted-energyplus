[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$OutputRoot = Join-Path $RepoRoot ".runtime\ideal-loads-flow-capacity-limit\26.1.0"
$CaseId = "ideal_loads_flow_capacity_limit_diagnostic_001"
$CasePath = Join-Path $RepoRoot "data\conformance_cases\$CaseId\case.toml"
$CaseOutputRoot = Join-Path $OutputRoot $CaseId
$CompareRoot = Join-Path $CaseOutputRoot "compare"

function Assert-RepoSubPath {
    param([Parameter(Mandatory = $true)][string]$Path)
    $full = [System.IO.Path]::GetFullPath($Path)
    $root = [System.IO.Path]::GetFullPath($RepoRoot)
    if (-not $full.StartsWith($root, [System.StringComparison]::OrdinalIgnoreCase)) {
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

function Assert-FileExists {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Description
    )
    if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
        throw "Missing $Description`: $Path"
    }
    Write-Host "OK $Description`: $Path"
}

foreach ($path in @(
    (Join-Path $OracleRoot "energyplus.exe"),
    (Join-Path $OracleRoot "ConvertInputFormat.exe"),
    (Join-Path $OracleRoot "WeatherData\USA_CO_Golden-NREL.724666_TMY3.epw"),
    $CasePath
)) {
    Assert-FileExists -Path $path -Description "required IdealLoads compare input"
}

Remove-RepoDirectory -Path $CaseOutputRoot

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Write-Host "Generating IdealLoads no-OA flow-and-capacity-limit diagnostic comparison artifacts."
$output = & $cargo.Source run -p ep_cli --quiet -- conformance ideal-loads-no-oa-sensible-report $CasePath $OracleRoot $OutputRoot 2>&1
if ($LASTEXITCODE -ne 0) {
    $output | ForEach-Object { Write-Host $_ }
    throw "IdealLoads no-OA flow-and-capacity-limit diagnostic comparison failed."
}

$text = ($output -join "`n")
Assert-Contains -Text $text -Pattern "IdealLoads No-OA Sensible Report" -Description "report header"
Assert-Contains -Text $text -Pattern "id: $CaseId" -Description "case id"
Assert-Contains -Text $text -Pattern "comparison_class: diagnostic-only" -Description "comparison class"
Assert-Contains -Text $text -Pattern "conformance_claim: false" -Description "claim boundary"
Assert-Contains -Text $text -Pattern "series: 16" -Description "series count"
Assert-Contains -Text $text -Pattern "samples: 189" -Description "detailed sample count"
Assert-Contains -Text $text -Pattern "tolerance_failures_count: 2" -Description "tracked diagnostic tolerance failures"
Assert-Contains -Text $text -Pattern "tolerance_policy: diagnostic-draft" -Description "tolerance policy"
Assert-Contains -Text $text -Pattern "status: diagnostic" -Description "diagnostic status"

$summaryPath = Join-Path $CompareRoot "compare-summary.json"
$reportPath = Join-Path $CompareRoot "compare-report.md"
$selectedOutputsPath = Join-Path $CompareRoot "selected_outputs.json"
$resultStorePath = Join-Path $CompareRoot "rust-result-store.json"
$variableDeltasPath = Join-Path $CompareRoot "variable-deltas.csv"
$firstDivergencePath = Join-Path $CompareRoot "first-divergence.csv"
$toleranceFailuresPath = Join-Path $CompareRoot "tolerance-failures.csv"
$stageSummaryPath = Join-Path $CompareRoot "stage-summary.json"

Assert-FileExists -Path $summaryPath -Description "IdealLoads compare summary"
Assert-FileExists -Path $reportPath -Description "IdealLoads markdown report"
Assert-FileExists -Path $selectedOutputsPath -Description "IdealLoads oracle selected outputs"
Assert-FileExists -Path $resultStorePath -Description "IdealLoads Rust result store"
Assert-FileExists -Path $variableDeltasPath -Description "IdealLoads variable deltas"
Assert-FileExists -Path $firstDivergencePath -Description "IdealLoads first divergence CSV"
Assert-FileExists -Path $toleranceFailuresPath -Description "IdealLoads tolerance failures CSV"
Assert-FileExists -Path $stageSummaryPath -Description "IdealLoads stage summary"

$summary = Get-Content -LiteralPath $summaryPath -Raw | ConvertFrom-Json
if ($summary.case_id -ne $CaseId) {
    throw "Unexpected case_id: $($summary.case_id)"
}
if ($summary.comparison_class -ne "diagnostic-only") {
    throw "Unexpected comparison_class: $($summary.comparison_class)"
}
if ($summary.conformance_claim -ne $false) {
    throw "IdealLoads flow-and-capacity-limit diagnostic summary must set conformance_claim=false"
}
if ($summary.status -ne "diagnostic") {
    throw "Unexpected IdealLoads flow-and-capacity-limit diagnostic status: $($summary.status)"
}
if ($summary.tolerance_failures -ne 2) {
    throw "IdealLoads flow-and-capacity-limit diagnostic should track exactly two finite-limit node-state tolerance failures: $($summary.tolerance_failures)"
}
if ($summary.samples -ne 189) {
    throw "Unexpected IdealLoads sample count: $($summary.samples)"
}
if ($summary.series_count -ne 16) {
    throw "Unexpected IdealLoads series count: $($summary.series_count)"
}
if ($summary.zone_demand_synthetic_rc_model -ne $false) {
    throw "IdealLoads flow-and-capacity-limit diagnostic must not synthesize zone demand from an RC shortcut"
}
$conformanceRows = @($summary.series | Where-Object { $_.level -eq "conformance" })
if ($conformanceRows.Count -ne 0) {
    throw "Expected no conformance-level output rows in diagnostic case, found $($conformanceRows.Count)"
}
$diagnosticRows = @($summary.series | Where-Object { $_.level -eq "diagnostic" })
if ($diagnosticRows.Count -ne 16) {
    throw "Expected 16 diagnostic-level output rows, found $($diagnosticRows.Count)"
}
$failingRows = @($summary.series | Where-Object { $_.status -ne "pass" })
if ($failingRows.Count -ne 2) {
    throw "Expected exactly two diagnostic finite-limit node-state failures, found $($failingRows.Count)"
}
$failingVariables = @($failingRows | ForEach-Object { "$($_.key)|$($_.variable)" })
foreach ($expectedFailure in @(
    "ZONE ONE INLET|System Node Temperature",
    "ZONE ONE INLET|System Node Mass Flow Rate"
)) {
    if ($failingVariables -notcontains $expectedFailure) {
        throw "Missing expected diagnostic failure row: $expectedFailure"
    }
}
if (@($summary.series | Where-Object { $_.domain -eq "hvac" -and $_.status -ne "pass" }).Count -ne 0) {
    throw "All IdealLoads flow-and-capacity-limit HVAC rate rows must pass"
}
$nodeTemperature = @($summary.series | Where-Object { $_.key -eq "ZONE ONE INLET" -and $_.variable -eq "System Node Temperature" })
if ($nodeTemperature.Count -ne 1 -or $nodeTemperature[0].status -ne "fail") {
    throw "System Node Temperature should remain a tracked diagnostic gap in the flow-and-capacity-limit case"
}
$nodeFlow = @($summary.series | Where-Object { $_.variable -eq "System Node Mass Flow Rate" })
if ($nodeFlow.Count -ne 1) {
    throw "Missing System Node Mass Flow Rate row"
}
if ($nodeFlow[0].alignment -ne "timestamp") {
    throw "System Node Mass Flow Rate must use timestamp alignment"
}
if ($nodeFlow[0].rust_source -ne "rust-ideal-loads-no-oa-sensible-limited-calc") {
    throw "Unexpected node flow Rust source: $($nodeFlow[0].rust_source)"
}
if ($nodeFlow[0].level -ne "diagnostic") {
    throw "System Node Mass Flow Rate must be diagnostic-level in the flow-and-capacity-limit case"
}
if ($nodeFlow[0].status -ne "fail") {
    throw "System Node Mass Flow Rate should remain a tracked diagnostic gap until finite-limit demand state is matched"
}

$toleranceFailures = @(Import-Csv -LiteralPath $toleranceFailuresPath)
if ($toleranceFailures.Count -ne 2) {
    throw "Expected two tracked diagnostic tolerance-failures.csv rows, found $($toleranceFailures.Count)"
}

$resultStore = Get-Content -LiteralPath $resultStorePath -Raw | ConvertFrom-Json
if ($resultStore.series_count -ne 16 -or $resultStore.sample_count -ne 189) {
    throw "Unexpected result store shape: series=$($resultStore.series_count) samples=$($resultStore.sample_count)"
}

$selectedOutputs = Get-Content -LiteralPath $selectedOutputsPath -Raw | ConvertFrom-Json
if (@($selectedOutputs.series).Count -ne 16) {
    throw "Unexpected selected_outputs series count: $(@($selectedOutputs.series).Count)"
}

$stageSummary = Get-Content -LiteralPath $stageSummaryPath -Raw | ConvertFrom-Json
if ($stageSummary.branch -ne "no-oa-finite-limit-sensible") {
    throw "Unexpected IdealLoads branch: $($stageSummary.branch)"
}
if ($stageSummary.zone_demand_synthetic_rc_model -ne $false) {
    throw "Stage summary must record that no RC demand shortcut is used"
}

$reportText = Get-Content -LiteralPath $reportPath -Raw
Assert-Contains -Text $reportText -Pattern "claim_boundary: diagnostic-only no-OA finite-limit sensible IdealLoads branch" -Description "markdown claim boundary"
Assert-Contains -Text $reportText -Pattern "zone_demand_synthetic_rc_model: false" -Description "markdown demand source guard"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE INLET | System Node Mass Flow Rate | diagnostic" -Description "markdown node flow row"

Write-Host "IdealLoads no-OA flow-and-capacity-limit diagnostic comparison artifacts generated."
