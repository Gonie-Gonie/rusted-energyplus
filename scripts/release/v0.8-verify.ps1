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

Assert-FileExists -Path "docs\src\operations\v0.8.0-plan.md" -Description "v0.8 plan"
Assert-FileExists -Path "docs\src\operations\v0.8.0-readiness.md" -Description "v0.8 readiness"
Assert-FileExists -Path "data\conformance_cases\heat_balance_nomass_001\case.toml" -Description "v0.8 heat-balance case"
Assert-FileExists -Path "data\conformance_cases\heat_balance_nomass_001\heat_balance_nomass.idf" -Description "v0.8 heat-balance IDF"
Assert-FileExists -Path "scripts\compare\compare-heat-balance-conformance.ps1" -Description "v0.8 conformance gate"

Assert-Contains -Path "data\conformance_cases\heat_balance_nomass_001\case.toml" -Pattern 'comparison_class = "conformance"' -Description "v0.8 case class"
Assert-Contains -Path "data\conformance_cases\heat_balance_nomass_001\case.toml" -Pattern "conformance_claim = true" -Description "v0.8 conformance claim"
Assert-Contains -Path "data\conformance_cases\heat_balance_nomass_001\case.toml" -Pattern "max_abs = 0.000001" -Description "v0.8 absolute tolerance"
Assert-Contains -Path "data\conformance_cases\heat_balance_nomass_001\case.toml" -Pattern "max_rmse = 0.000001" -Description "v0.8 rmse tolerance"
Assert-Contains -Path "data\conformance_cases\heat_balance_nomass_001\case.toml" -Pattern "blocking = true" -Description "v0.8 blocking gate"

Assert-Contains -Path "docs\src\operations\v0.8.0-plan.md" -Pattern "heat_balance_nomass_001" -Description "v0.8 plan case"
Assert-Contains -Path "docs\src\operations\v0.8.0-plan.md" -Pattern "DataHeatBalance::ZoneInitialTemp" -Description "v0.8 source reference"
Assert-Contains -Path "docs\src\operations\v0.8.0-readiness.md" -Pattern "conformance-ready" -Description "v0.8 readiness status"
Assert-Contains -Path "docs\src\operations\v0.8.0-readiness.md" -Pattern "not a dynamic exterior heat-balance claim" -Description "v0.8 claim boundary"
Assert-Contains -Path "docs\src\porting-map\heat-balance.md" -Pattern 'v0.8 promoted case: `heat_balance_nomass_001`' -Description "heat-balance map v0.8 promotion"
Assert-Contains -Path "docs\src\conformance\output-variable-matrix.md" -Pattern 'conformance for `heat_balance_nomass_001`' -Description "output matrix v0.8 promotion"

Write-Host "milestone: v0.8.0"
Write-Host "scope: first tolerance-gated uncontrolled zone heat-balance subset"
Write-Host "claim: heat_balance_nomass_001 Zone Mean Air Temperature only"

Invoke-DevCommand -Command "source-smoke"
Invoke-DevCommand -Command "compare-heat-balance-conformance"
Invoke-DevCommand -Command "test"
Invoke-DevCommand -Command "docs-check"
Invoke-DevCommand -Command "strict-no-false-conformance"

Write-Host "result: pass"
Write-Host "v0.8.0 heat-balance conformance verification passed."
