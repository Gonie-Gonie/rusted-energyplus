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

Assert-FileExists -Path "docs\src\operations\v0.5.0-plan.md" -Description "v0.5 plan"
Assert-FileExists -Path "docs\src\operations\v0.5.0-readiness.md" -Description "v0.5 readiness"
Assert-FileExists -Path "data\conformance_cases\surface_geometry_001\case.toml" -Description "surface geometry case manifest"
Assert-FileExists -Path "data\conformance_cases\surface_geometry_001\surface_geometry.idf" -Description "surface geometry case IDF"
Assert-FileExists -Path "scripts\compare\compare-surface-geometry-smoke.ps1" -Description "surface geometry compare smoke"

Invoke-DevCommand -Command "compare-geometry-smoke"
Invoke-DevCommand -Command "compare-surface-geometry-smoke"
Invoke-DevCommand -Command "compare-construction-materials-smoke"
Invoke-DevCommand -Command "compare-internal-gains-smoke"
Invoke-DevCommand -Command "compare-internal-convective-gain-smoke"
Invoke-DevCommand -Command "conformance-schema-smoke"
Invoke-DevCommand -Command "compare-regression"
Invoke-DevCommand -Command "check"

Write-Host "v0.5.0 geometry/internal-variable verification passed."
