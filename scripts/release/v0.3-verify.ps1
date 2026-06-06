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

Write-Host "milestone: v0.3"
Write-Host "scope: input interpretation contract, no runtime or numerical conformance claim"

Assert-FileExists -Path "docs\src\operations\v0.3.0-plan.md" -Description "v0.3 plan"
Assert-FileExists -Path "docs\src\operations\v0.3.0-readiness.md" -Description "v0.3 readiness"
Assert-FileExists -Path "data\testcases\minimal\minimal.epJSON" -Description "RawModel fixture"
Assert-FileExists -Path "data\testcases\minimal\typed-model.epJSON" -Description "TypedModel fixture"
Assert-FileExists -Path "data\testcases\minimal\missing-reference.epJSON" -Description "missing-reference fixture"
Assert-FileExists -Path "data\testcases\minimal\invalid-enum.epJSON" -Description "invalid-enum fixture"
Assert-FileExists -Path "data\testcases\minimal\duplicate-normalized-name.epJSON" -Description "duplicate normalized-name fixture"
Assert-FileExists -Path "data\testcases\minimal\invalid-numeric-field.epJSON" -Description "invalid numeric fixture"

Write-Host "required commands:"
Write-Host "- raw-model-smoke"
Write-Host "- typed-model-smoke"
Write-Host "- test"
Write-Host "- docs-check"
Write-Host "- strict-no-false-conformance"

Invoke-DevCommand -Command "raw-model-smoke"
Invoke-DevCommand -Command "typed-model-smoke"
Invoke-DevCommand -Command "test"
Invoke-DevCommand -Command "docs-check"
Invoke-DevCommand -Command "strict-no-false-conformance"

Write-Host "result: pass"
Write-Host "v0.3.0 input interpretation verification passed."
