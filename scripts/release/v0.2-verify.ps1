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

Write-Host "milestone: v0.2"
Write-Host "scope: conformance harness, baseline-only reports, no Rust-vs-EnergyPlus numerical claim"

Assert-FileExists -Path "docs\src\operations\v0.2.0-plan.md" -Description "v0.2 plan"
Assert-FileExists -Path "docs\src\operations\v0.2.0-readiness.md" -Description "v0.2 readiness"
Assert-FileExists -Path "crates\ep_conformance\src\lib.rs" -Description "conformance schema crate"
Assert-FileExists -Path "data\conformance_cases\schedule_constant_001\case.toml" -Description "schedule case manifest"
Assert-FileExists -Path "data\conformance_cases\weather_fields_001\case.toml" -Description "weather case manifest"
Assert-FileExists -Path "data\conformance_cases\surface_geometry_001\case.toml" -Description "surface geometry case manifest"
Assert-FileExists -Path "data\conformance_suites\foundation.toml" -Description "foundation suite manifest"

Write-Host "required commands:"
Write-Host "- conformance-schema-smoke"
Write-Host "- conformance-baseline-smoke"
Write-Host "- conformance-report-smoke"
Write-Host "- docs-check"
Write-Host "- strict-no-false-conformance"

Invoke-DevCommand -Command "conformance-schema-smoke"
Invoke-DevCommand -Command "conformance-baseline-smoke"
Invoke-DevCommand -Command "conformance-report-smoke"
Invoke-DevCommand -Command "docs-check"
Invoke-DevCommand -Command "strict-no-false-conformance"

Write-Host "result: pass"
Write-Host "v0.2.0 conformance harness verification passed."
