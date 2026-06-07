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

Assert-FileExists -Path "docs\src\archive\pre-alpha\v0.5.0-plan.md" -Description "v0.5 plan"
Assert-FileExists -Path "docs\src\archive\pre-alpha\v0.5.0-readiness.md" -Description "v0.5 readiness"
Assert-FileExists -Path "docs\src\porting-map\geometry.md" -Description "v0.5 geometry porting map"
Assert-FileExists -Path "docs\src\conformance\output-variable-matrix.md" -Description "v0.5 output variable matrix"
Assert-FileExists -Path "data\conformance_cases\surface_geometry_001\case.toml" -Description "surface geometry case manifest"
Assert-FileExists -Path "data\conformance_cases\surface_geometry_001\surface_geometry.idf" -Description "surface geometry case IDF"
Assert-FileExists -Path "data\conformance_cases\construction_materials_001\case.toml" -Description "construction/material case manifest"
Assert-FileExists -Path "data\conformance_cases\construction_materials_001\construction_materials.idf" -Description "construction/material case IDF"
Assert-FileExists -Path "data\conformance_cases\internal_gains_001\case.toml" -Description "internal-gains case manifest"
Assert-FileExists -Path "data\conformance_cases\internal_gains_001\internal_gains.idf" -Description "internal-gains case IDF"
Assert-FileExists -Path "scripts\compare\compare-surface-geometry-smoke.ps1" -Description "surface geometry compare smoke"
Assert-FileExists -Path "scripts\compare\compare-construction-materials-smoke.ps1" -Description "construction/material compare smoke"
Assert-FileExists -Path "scripts\compare\compare-internal-gains-smoke.ps1" -Description "internal-gains compare smoke"
Assert-FileExists -Path "scripts\compare\compare-internal-convective-gain-smoke.ps1" -Description "internal convective gain compare smoke"

Write-Host "milestone: v0.5.0"
Write-Host "scope: static geometry, construction/material, and internal-gain smoke evidence"
Write-Host "claim: conformance_claim false; no heat-balance or HVAC conformance"

Invoke-DevCommand -Command "compare-geometry-smoke"
Invoke-DevCommand -Command "compare-surface-geometry-smoke"
Invoke-DevCommand -Command "compare-construction-materials-smoke"
Invoke-DevCommand -Command "compare-internal-gains-smoke"
Invoke-DevCommand -Command "compare-internal-convective-gain-smoke"
Invoke-DevCommand -Command "conformance-schema-smoke"
Invoke-DevCommand -Command "test"
Invoke-DevCommand -Command "docs-check"
Invoke-DevCommand -Command "strict-no-false-conformance"

Write-Host "result: pass"
Write-Host "v0.5.0 geometry/internal-variable verification passed."
