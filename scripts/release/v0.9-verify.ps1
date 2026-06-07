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

Assert-FileExists -Path "docs\src\operations\v0.9.0-plan.md" -Description "v0.9 plan"
Assert-FileExists -Path "docs\src\operations\v0.9.0-readiness.md" -Description "v0.9 readiness"
Assert-FileExists -Path "data\conformance_cases\surface_temperature_nomass_001\case.toml" -Description "v0.9 surface-temperature case"
Assert-FileExists -Path "data\conformance_cases\surface_temperature_nomass_001\surface_temperature_nomass.idf" -Description "v0.9 surface-temperature IDF"
Assert-FileExists -Path "scripts\compare\compare-surface-temperature-conformance.ps1" -Description "v0.9 conformance gate"

Assert-Contains -Path "data\conformance_cases\surface_temperature_nomass_001\case.toml" -Pattern 'comparison_class = "conformance"' -Description "v0.9 case class"
Assert-Contains -Path "data\conformance_cases\surface_temperature_nomass_001\case.toml" -Pattern "conformance_claim = true" -Description "v0.9 conformance claim"
Assert-Contains -Path "data\conformance_cases\surface_temperature_nomass_001\case.toml" -Pattern 'variable = "Surface Inside Face Temperature"' -Description "v0.9 inside surface variable"
Assert-Contains -Path "data\conformance_cases\surface_temperature_nomass_001\case.toml" -Pattern 'variable = "Surface Outside Face Temperature"' -Description "v0.9 outside surface variable"
Assert-Contains -Path "data\conformance_cases\surface_temperature_nomass_001\case.toml" -Pattern 'variable_class = "surface-state"' -Description "v0.9 surface tolerance class"
Assert-Contains -Path "data\conformance_cases\surface_temperature_nomass_001\case.toml" -Pattern "blocking = true" -Description "v0.9 blocking gate"

Assert-Contains -Path "docs\src\operations\v0.9.0-plan.md" -Pattern "surface_temperature_nomass_001" -Description "v0.9 plan case"
Assert-Contains -Path "docs\src\operations\v0.9.0-plan.md" -Pattern "not a fenestration or solar-radiation claim" -Description "v0.9 claim boundary"
Assert-Contains -Path "docs\src\operations\v0.9.0-readiness.md" -Pattern "conformance-ready" -Description "v0.9 readiness status"
Assert-Contains -Path "docs\src\operations\v0.9.0-readiness.md" -Pattern "Surface Inside Face Temperature" -Description "v0.9 readiness inside variable"
Assert-Contains -Path "docs\src\porting-map\output-variable-source-map.md" -Pattern 'conformance for `surface_temperature_nomass_001`' -Description "output map v0.9 promotion"
Assert-Contains -Path "docs\src\conformance\output-variable-matrix.md" -Pattern 'conformance for `surface_temperature_nomass_001`' -Description "output matrix v0.9 promotion"

Write-Host "milestone: v0.9.0"
Write-Host "scope: first tolerance-gated surface temperature subset"
Write-Host "claim: surface_temperature_nomass_001 surface inside/outside face temperatures only"

Invoke-DevCommand -Command "source-smoke"
Invoke-DevCommand -Command "compare-surface-temperature-conformance"
Invoke-DevCommand -Command "test"
Invoke-DevCommand -Command "docs-check"
Invoke-DevCommand -Command "strict-no-false-conformance"

Write-Host "result: pass"
Write-Host "v0.9.0 surface-temperature conformance verification passed."
