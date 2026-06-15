[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$OutputRoot = Join-Path $RepoRoot ".runtime\ideal-loads-outdoor-air-design-flow\26.1.0"
$CaseId = "ideal_loads_outdoor_air_design_flow_diagnostic_001"
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
    Assert-FileExists -Path $path -Description "required IdealLoads outdoor-air compare input"
}

Remove-RepoDirectory -Path $CaseOutputRoot

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Write-Host "Generating IdealLoads outdoor-air design-flow diagnostic comparison artifacts."
$output = & $cargo.Source run -p ep_cli --quiet -- conformance ideal-loads-outdoor-air-design-flow-report $CasePath $OracleRoot $OutputRoot 2>&1
if ($LASTEXITCODE -ne 0) {
    $output | ForEach-Object { Write-Host $_ }
    throw "IdealLoads outdoor-air design-flow diagnostic comparison failed."
}

$text = ($output -join "`n")
Assert-Contains -Text $text -Pattern "IdealLoads Outdoor-Air Design-Flow Report" -Description "report header"
Assert-Contains -Text $text -Pattern "id: $CaseId" -Description "case id"
Assert-Contains -Text $text -Pattern "comparison_class: diagnostic-only" -Description "comparison class"
Assert-Contains -Text $text -Pattern "conformance_claim: false" -Description "claim boundary"
Assert-Contains -Text $text -Pattern "series: 2" -Description "series count"
Assert-Contains -Text $text -Pattern "samples: 96" -Description "detailed sample count"
Assert-Contains -Text $text -Pattern "tolerance_failures_count: 0" -Description "tolerance failures"
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

Assert-FileExists -Path $summaryPath -Description "IdealLoads outdoor-air compare summary"
Assert-FileExists -Path $reportPath -Description "IdealLoads outdoor-air markdown report"
Assert-FileExists -Path $selectedOutputsPath -Description "IdealLoads outdoor-air selected outputs"
Assert-FileExists -Path $resultStorePath -Description "IdealLoads outdoor-air Rust result store"
Assert-FileExists -Path $variableDeltasPath -Description "IdealLoads outdoor-air variable deltas"
Assert-FileExists -Path $firstDivergencePath -Description "IdealLoads outdoor-air first divergence CSV"
Assert-FileExists -Path $toleranceFailuresPath -Description "IdealLoads outdoor-air tolerance failures CSV"
Assert-FileExists -Path $stageSummaryPath -Description "IdealLoads outdoor-air stage summary"

$summary = Get-Content -LiteralPath $summaryPath -Raw | ConvertFrom-Json
if ($summary.case_id -ne $CaseId) {
    throw "Unexpected case_id: $($summary.case_id)"
}
if ($summary.comparison_class -ne "diagnostic-only") {
    throw "Unexpected comparison_class: $($summary.comparison_class)"
}
if ($summary.conformance_claim -ne $false) {
    throw "IdealLoads outdoor-air diagnostic summary must set conformance_claim=false"
}
if ($summary.status -ne "diagnostic") {
    throw "Unexpected IdealLoads outdoor-air diagnostic status: $($summary.status)"
}
if ($summary.tolerance_failures -ne 0) {
    throw "IdealLoads outdoor-air diagnostic should have zero tolerance failures: $($summary.tolerance_failures)"
}
if ($summary.series_count -ne 2) {
    throw "Unexpected IdealLoads outdoor-air series count: $($summary.series_count)"
}
if ($summary.samples -ne 96) {
    throw "Unexpected IdealLoads outdoor-air sample count: $($summary.samples)"
}
if ($summary.design_volume_flow_rate_m3_per_s -ne 0.05) {
    throw "Unexpected outdoor-air design volume flow: $($summary.design_volume_flow_rate_m3_per_s)"
}

$rows = @($summary.series)
if ($rows.Count -ne 2) {
    throw "Expected two outdoor-air diagnostic rows, found $($rows.Count)"
}
foreach ($row in $rows) {
    if ($row.level -ne "diagnostic") {
        throw "Outdoor-air row must be diagnostic-level: $($row.variable)"
    }
    if ($row.status -ne "pass") {
        throw "Outdoor-air row should pass: $($row.variable)"
    }
    if ($row.rust_source -ne "rust-ideal-loads-outdoor-air-design-flow") {
        throw "Unexpected Rust source for $($row.variable): $($row.rust_source)"
    }
}

$massRow = @($rows | Where-Object { $_.variable -eq "Zone Ideal Loads Outdoor Air Mass Flow Rate" })
if ($massRow.Count -ne 1 -or $massRow[0].units -ne "kg/s") {
    throw "Missing kg/s outdoor-air mass-flow row"
}
$volumeRow = @($rows | Where-Object { $_.variable -eq "Zone Ideal Loads Outdoor Air Standard Density Volume Flow Rate" })
if ($volumeRow.Count -ne 1 -or $volumeRow[0].units -ne "m3/s") {
    throw "Missing m3/s outdoor-air standard-density volume-flow row"
}

$toleranceFailures = @(Import-Csv -LiteralPath $toleranceFailuresPath)
if ($toleranceFailures.Count -ne 0) {
    throw "Expected empty tolerance-failures.csv, found $($toleranceFailures.Count) rows"
}

$resultStore = Get-Content -LiteralPath $resultStorePath -Raw | ConvertFrom-Json
if ($resultStore.series_count -ne 2 -or $resultStore.sample_count -ne $summary.samples) {
    throw "Unexpected result store shape: series=$($resultStore.series_count) samples=$($resultStore.sample_count)"
}

$selectedOutputs = Get-Content -LiteralPath $selectedOutputsPath -Raw | ConvertFrom-Json
if (@($selectedOutputs.series).Count -ne 2) {
    throw "Unexpected selected_outputs series count: $(@($selectedOutputs.series).Count)"
}

$stageSummary = Get-Content -LiteralPath $stageSummaryPath -Raw | ConvertFrom-Json
if ($stageSummary.branch -ne "outdoor-air-design-flow") {
    throw "Unexpected IdealLoads branch: $($stageSummary.branch)"
}
if ($stageSummary.outdoor_air -ne $true) {
    throw "Stage summary must record outdoor_air=true"
}

$reportText = Get-Content -LiteralPath $reportPath -Raw
Assert-Contains -Text $reportText -Pattern "claim_boundary: diagnostic-only IdealLoads outdoor-air design-flow mass/standard-density volume flow" -Description "markdown claim boundary"
Assert-Contains -Text $reportText -Pattern "outdoor_air_schedule: blank-always-1.0" -Description "markdown OA schedule guard"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE IDEAL LOADS | Zone Ideal Loads Outdoor Air Mass Flow Rate | diagnostic" -Description "markdown OA mass row"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE IDEAL LOADS | Zone Ideal Loads Outdoor Air Standard Density Volume Flow Rate | diagnostic" -Description "markdown OA volume row"

Write-Host "IdealLoads outdoor-air design-flow diagnostic comparison artifacts generated."
