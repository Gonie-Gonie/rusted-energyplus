[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$OutputRoot = Join-Path $RepoRoot ".runtime\plant-loop-diagnostic\26.1.0"
$CaseId = "plant_loop_diagnostic_001"
$CaseOutputRoot = Join-Path $OutputRoot $CaseId
$EpJsonPath = Join-Path $CaseOutputRoot "input.epJSON"
$ProjectionRoot = Join-Path $OutputRoot "plant-state-projection"

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

if (-not (Test-Path -LiteralPath $EpJsonPath -PathType Leaf)) {
    Write-Host "Generating prerequisite v0.15 plant-loop diagnostic baseline."
    Invoke-DevCommand -Command "plant-loop-diagnostic-smoke"
}

Remove-RepoDirectory -Path $ProjectionRoot

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Write-Host "Writing diagnostic Rust plant-state projection addendum."
$projectionOutput = & $cargo.Source run -p ep_cli --quiet -- run plant-state-projection $EpJsonPath $ProjectionRoot 2>&1
if ($LASTEXITCODE -ne 0) {
    $projectionOutput | ForEach-Object { Write-Host $_ }
    throw "Plant-loop diagnostic Rust plant-state projection failed."
}
$projectionText = ($projectionOutput -join "`n")
Assert-Contains -Text $projectionText -Pattern "Plant State Projection" -Description "projection header"
Assert-Contains -Text $projectionText -Pattern "runtime_class: plant-loop-state-projection" -Description "projection runtime class"
Assert-Contains -Text $projectionText -Pattern "comparison_class: diagnostic-only" -Description "projection diagnostic class"
Assert-Contains -Text $projectionText -Pattern "conformance_claim: false" -Description "projection claim boundary"
Assert-Contains -Text $projectionText -Pattern "algorithm_parity: false" -Description "projection algorithm boundary"
Assert-Contains -Text $projectionText -Pattern "tolerance_policy: none" -Description "projection tolerance boundary"
Assert-Contains -Text $projectionText -Pattern "loops: 1" -Description "projection loop count"
Assert-Contains -Text $projectionText -Pattern "equipment: 3" -Description "projection equipment count"
Assert-Contains -Text $projectionText -Pattern "samples: 48" -Description "projection sample count"
Assert-Contains -Text $projectionText -Pattern "series: 8" -Description "projection series count"
Assert-Contains -Text $projectionText -Pattern "status: projected" -Description "projection status"

$ProjectionMarkdown = Join-Path $ProjectionRoot "plant-state-summary.md"
$ProjectionSummary = Join-Path $ProjectionRoot "plant-state-summary.json"
Assert-FileExists -Path $ProjectionMarkdown -Description "Rust plant-state addendum markdown summary"
Assert-FileExists -Path $ProjectionSummary -Description "Rust plant-state addendum JSON summary"

$projectionMarkdownText = Get-Content -LiteralPath $ProjectionMarkdown -Raw
Assert-Contains -Text $projectionMarkdownText -Pattern "comparison_class: diagnostic-only" -Description "projection markdown diagnostic class"
Assert-Contains -Text $projectionMarkdownText -Pattern "conformance_claim: false" -Description "projection markdown claim boundary"
Assert-Contains -Text $projectionMarkdownText -Pattern "algorithm_parity: false" -Description "projection markdown algorithm boundary"
Assert-Contains -Text $projectionMarkdownText -Pattern "status: projected" -Description "projection markdown status"
Assert-Contains -Text $projectionMarkdownText -Pattern "source_map: docs/src/porting-map/plant-source-map.md" -Description "projection markdown source map"
Assert-Contains -Text $projectionMarkdownText -Pattern "timestamp_rule: hour-ending hourly samples aligned to the plant diagnostic case time axis" -Description "projection markdown timestamp rule"
Assert-Contains -Text $projectionMarkdownText -Pattern "warmup_rule: EnergyPlus warmup samples are not represented in this diagnostic projection" -Description "projection markdown warmup rule"
Assert-Contains -Text $projectionMarkdownText -Pattern "sizing_rule: PlantLoop sizing-period baseline rows remain diagnostic-only" -Description "projection markdown sizing rule"
Assert-Contains -Text $projectionMarkdownText -Pattern "plant-state" -Description "projection markdown plant-state class"
Assert-Contains -Text $projectionMarkdownText -Pattern "plant-equipment" -Description "projection markdown plant-equipment class"
Assert-Contains -Text $projectionMarkdownText -Pattern "Pump Electricity Rate" -Description "projection markdown pump row"
Assert-Contains -Text $projectionMarkdownText -Pattern "Plant Load Profile Heat Transfer Rate" -Description "projection markdown load-profile row"

$projection = Get-Content -LiteralPath $ProjectionSummary -Raw | ConvertFrom-Json
if ($projection.comparison_class -ne "diagnostic-only") {
    throw "Unexpected projection addendum comparison_class: $($projection.comparison_class)"
}
if ($projection.conformance_claim -ne $false) {
    throw "plant projection addendum must keep conformance_claim=false"
}
if ($projection.algorithm_parity -ne $false) {
    throw "plant projection addendum must keep algorithm_parity=false"
}
if ($projection.tolerance_policy -ne "none") {
    throw "plant projection addendum must keep tolerance_policy=none"
}
if ($projection.status -ne "projected") {
    throw "Unexpected projection addendum status: $($projection.status)"
}
if ([int]$projection.loops -ne 1) {
    throw "Unexpected projection addendum loop count: $($projection.loops)"
}
if ([int]$projection.equipment -ne 3) {
    throw "Unexpected projection addendum equipment count: $($projection.equipment)"
}
if ([int]$projection.samples -ne 48) {
    throw "Unexpected projection addendum sample count: $($projection.samples)"
}
if ([int]$projection.series -ne 8) {
    throw "Unexpected projection addendum series count: $($projection.series)"
}
if ($projection.evidence_policy.source_map -ne "docs/src/porting-map/plant-source-map.md") {
    throw "Unexpected plant projection source map: $($projection.evidence_policy.source_map)"
}
if ($projection.evidence_policy.timestamp_rule -ne "hour-ending hourly samples aligned to the plant diagnostic case time axis") {
    throw "Unexpected plant projection timestamp rule: $($projection.evidence_policy.timestamp_rule)"
}
if ($projection.evidence_policy.warmup_rule -ne "EnergyPlus warmup samples are not represented in this diagnostic projection") {
    throw "Unexpected plant projection warmup rule: $($projection.evidence_policy.warmup_rule)"
}
if (-not $projection.evidence_policy.sizing_rule.StartsWith("PlantLoop sizing-period baseline rows remain diagnostic-only")) {
    throw "Unexpected plant projection sizing rule: $($projection.evidence_policy.sizing_rule)"
}
if ($projection.loop_order.Count -ne 1) {
    throw "Unexpected projection addendum loop order count: $($projection.loop_order.Count)"
}
if ($projection.equipment_order.Count -ne 3) {
    throw "Unexpected projection addendum equipment order count: $($projection.equipment_order.Count)"
}
if ($projection.result_series.Count -ne 8) {
    throw "Unexpected projection addendum result series count: $($projection.result_series.Count)"
}

foreach ($spec in @(
    @("MAIN LOOP", "Plant Supply Side Cooling Demand Rate", "plant-state"),
    @("MAIN LOOP", "Plant Supply Side Heating Demand Rate", "plant-state"),
    @("MAIN LOOP", "Plant Supply Side Inlet Mass Flow Rate", "plant-state"),
    @("MAIN LOOP", "Plant Supply Side Inlet Temperature", "plant-state"),
    @("MAIN LOOP", "Plant Supply Side Outlet Temperature", "plant-state"),
    @("PUMP", "Pump Electricity Rate", "plant-equipment"),
    @("PURCHASED HEATING", "District Heating Water Rate", "plant-equipment"),
    @("LOAD PROFILE 1", "Plant Load Profile Heat Transfer Rate", "plant-equipment")
)) {
    $key = $spec[0]
    $variable = $spec[1]
    $class = $spec[2]
    $row = $projection.result_series | Where-Object {
        $_.key -eq $key -and $_.variable -eq $variable -and $_.class -eq $class
    }
    if (-not $row) {
        throw "Missing Rust projection addendum row for $key / $variable / $class"
    }
    if ([int]$row.samples -ne 48) {
        throw "Unexpected Rust projection sample count for $key / $variable`: $($row.samples)"
    }
    if ([int]$row.nonzero_count -ne 48) {
        throw "Rust projection must include nonzero hourly samples for $key / $variable"
    }
    if ($row.status -ne "projected") {
        throw "Unexpected Rust projection status for $key / $variable`: $($row.status)"
    }
}

Write-Host "Plant-loop projection smoke passed."
