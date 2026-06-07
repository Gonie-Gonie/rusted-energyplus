[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
Set-Location $RepoRoot

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

function Assert-Contains {
    param(
        [Parameter(Mandatory = $true)][string]$Path,
        [Parameter(Mandatory = $true)][string]$Pattern,
        [Parameter(Mandatory = $true)][string]$Description
    )
    Assert-FileExists -Path $Path -Description $Description
    $match = Select-String -LiteralPath $Path -SimpleMatch -Pattern $Pattern -ErrorAction SilentlyContinue
    if ($null -eq $match) {
        throw "Missing $Description marker in $Path`: $Pattern"
    }
    Write-Host "OK $Description marker: $Pattern"
}

function Assert-ZipEntry {
    param(
        [Parameter(Mandatory = $true)][string]$ZipPath,
        [Parameter(Mandatory = $true)][string]$Entry,
        [Parameter(Mandatory = $true)][string]$Description
    )

    Assert-FileExists -Path $ZipPath -Description $Description
    Add-Type -AssemblyName System.IO.Compression.FileSystem
    $archive = [System.IO.Compression.ZipFile]::OpenRead((Resolve-Path -LiteralPath $ZipPath).Path)
    try {
        $expected = $Entry.Replace("/", "\")
        $match = $archive.Entries | Where-Object {
            $_.FullName.Replace("/", "\") -eq $expected
        }
        if ($null -eq $match) {
            throw "Missing $Description zip entry in $ZipPath`: $Entry"
        }
        Write-Host "OK $Description zip entry: $Entry"
    }
    finally {
        $archive.Dispose()
    }
}

Assert-FileExists -Path "docs\src\operations\v0.11.0-plan.md" -Description "v0.11 plan"
Assert-FileExists -Path "docs\src\operations\v0.11.0-readiness.md" -Description "v0.11 readiness"
Assert-FileExists -Path "docs\src\releases\v0.11.0.md" -Description "v0.11 release note"
Assert-FileExists -Path "data\conformance_cases\air_side_node_diagnostic_001\case.toml" -Description "v0.11 air-side node diagnostic case"
Assert-FileExists -Path "data\conformance_cases\air_side_node_diagnostic_001\air_side_node_diagnostic.idf" -Description "v0.11 air-side node diagnostic IDF"
Assert-FileExists -Path "scripts\smoke\air-side-node-diagnostic-smoke.ps1" -Description "v0.11 node diagnostic gate"

Assert-Contains -Path "data\conformance_cases\air_side_node_diagnostic_001\case.toml" -Pattern 'comparison_class = "diagnostic-only"' -Description "v0.11 diagnostic case class"
Assert-Contains -Path "data\conformance_cases\air_side_node_diagnostic_001\case.toml" -Pattern "conformance_claim = false" -Description "v0.11 no conformance claim"
Assert-Contains -Path "data\conformance_cases\air_side_node_diagnostic_001\case.toml" -Pattern 'class = "node-state"' -Description "v0.11 node output class"
Assert-Contains -Path "data\conformance_cases\air_side_node_diagnostic_001\case.toml" -Pattern "System Node Mass Flow Rate" -Description "v0.11 node flow output"
Assert-Contains -Path "data\conformance_cases\air_side_node_diagnostic_001\case.toml" -Pattern "blocking = true" -Description "v0.11 blocking smoke gate"

Assert-Contains -Path "crates\ep_conformance\src\conformance.rs" -Pattern "NodeState" -Description "v0.11 node-state manifest class"
Assert-Contains -Path "crates\ep_model\src\model.rs" -Pattern "pub struct Node" -Description "v0.11 node typed model"
Assert-Contains -Path "crates\ep_model\src\model.rs" -Pattern "pub struct NodeList" -Description "v0.11 NodeList typed model"
Assert-Contains -Path "crates\ep_model\src\model.rs" -Pattern "pub struct ZoneAirNodeEdge" -Description "v0.11 zone air node graph edge"
Assert-Contains -Path "crates\ep_model\src\model.rs" -Pattern "pub struct IdealLoadsSupplyNodeEdge" -Description "v0.11 IdealLoads supply node graph edge"
Assert-Contains -Path "crates\ep_runtime\src\runtime.rs" -Pattern "pub struct NodeStateStore" -Description "v0.11 Rust node state store"
Assert-Contains -Path "crates\ep_runtime\src\runtime.rs" -Pattern "simulate_ideal_loads_node_state_projection" -Description "v0.11 Rust node-state projection"
Assert-Contains -Path "crates\ep_cli\src\main.rs" -Pattern "run node-state-projection" -Description "v0.11 node-state projection CLI"
Assert-Contains -Path "scripts\smoke\air-side-node-diagnostic-smoke.ps1" -Pattern "algorithm_parity: false" -Description "v0.11 node projection boundary"
Assert-Contains -Path "scripts\smoke\air-side-node-diagnostic-smoke.ps1" -Pattern "state_nodes: 3" -Description "v0.11 node projection state count"
Assert-Contains -Path "scripts\smoke\air-side-node-diagnostic-smoke.ps1" -Pattern "status: projected" -Description "v0.11 node projection status"

Assert-Contains -Path "docs\src\operations\v0.11.0-plan.md" -Pattern "air_side_node_diagnostic_001" -Description "v0.11 plan case"
Assert-Contains -Path "docs\src\operations\v0.11.0-plan.md" -Pattern "not a node or HVAC numerical conformance claim" -Description "v0.11 plan claim boundary"
Assert-Contains -Path "docs\src\operations\v0.11.0-readiness.md" -Pattern "diagnostic-ready" -Description "v0.11 readiness status"
Assert-Contains -Path "docs\src\operations\v0.11.0-readiness.md" -Pattern "not a node or HVAC numerical conformance claim" -Description "v0.11 readiness claim boundary"
Assert-Contains -Path "docs\src\conformance\output-variable-matrix.md" -Pattern "air_side_node_diagnostic_001" -Description "v0.11 output matrix"
Assert-Contains -Path "docs\src\porting-map\hvac.md" -Pattern "air_side_node_diagnostic_001" -Description "v0.11 HVAC map"

Write-Host "milestone: v0.11.0"
Write-Host "scope: baseline-only air-side node output diagnostics plus Rust projection plumbing for the typed IdealLoads node graph"
Write-Host "claim: diagnostic-only evidence for air_side_node_diagnostic_001; no node or HVAC numerical conformance"

Invoke-DevCommand -Command "source-smoke"
Invoke-DevCommand -Command "air-side-node-diagnostic-smoke"
Invoke-DevCommand -Command "test"
Invoke-DevCommand -Command "docs-check"
Invoke-DevCommand -Command "strict-no-false-conformance"
Invoke-DevCommand -Command "package" -Arguments @("-Version", "0.11.0")

$package = Join-Path $RepoRoot "dist\eplus-rs-v0.11.0-windows-x64.zip"
Assert-FileExists -Path $package -Description "v0.11 release package"
Assert-ZipEntry -ZipPath $package -Entry "data/conformance_cases/air_side_node_diagnostic_001/case.toml" -Description "v0.11 packaged node case manifest"
Assert-ZipEntry -ZipPath $package -Entry "data/conformance_cases/air_side_node_diagnostic_001/air_side_node_diagnostic.idf" -Description "v0.11 packaged node case IDF"
Assert-ZipEntry -ZipPath $package -Entry "docs/src/releases/v0.11.0.md" -Description "v0.11 packaged release note"

Write-Host "result: pass"
Write-Host "v0.11.0 air-side node diagnostic verification passed."
