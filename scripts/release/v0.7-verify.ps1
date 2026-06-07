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

$SourceRoot = ".reference\energyplus-src\26.1.0"

Assert-FileExists -Path "docs\src\porting-map\heat-balance-source-map.md" -Description "heat-balance source map"
Assert-FileExists -Path "docs\src\porting-map\output-variable-source-map.md" -Description "output-variable source map"
Assert-FileExists -Path "docs\src\porting-map\algorithm-porting-readiness.md" -Description "algorithm porting readiness"

foreach ($relative in @(
    "src\EnergyPlus\HeatBalanceManager.cc",
    "src\EnergyPlus\HeatBalanceManager.hh",
    "src\EnergyPlus\DataHeatBalance.hh",
    "src\EnergyPlus\HeatBalanceSurfaceManager.cc",
    "src\EnergyPlus\HeatBalanceSurfaceManager.hh",
    "src\EnergyPlus\ZoneTempPredictorCorrector.cc",
    "src\EnergyPlus\ZoneTempPredictorCorrector.hh",
    "src\EnergyPlus\HeatBalanceInternalHeatGains.cc",
    "src\EnergyPlus\InternalHeatGains.cc",
    "src\EnergyPlus\OutputProcessor.cc"
)) {
    Assert-FileExists -Path (Join-Path $SourceRoot $relative) -Description "EnergyPlus reference source"
}

Assert-Contains -Path "docs\src\porting-map\heat-balance-source-map.md" -Pattern "Reference version: EnergyPlus 26.1.0" -Description "source-map version"
Assert-Contains -Path "docs\src\porting-map\heat-balance-source-map.md" -Pattern "ManageHeatBalance" -Description "source-map driver routine"
Assert-Contains -Path "docs\src\porting-map\heat-balance-source-map.md" -Pattern "CalcHeatBalanceInsideSurf" -Description "source-map inside surface routine"
Assert-Contains -Path "docs\src\porting-map\heat-balance-source-map.md" -Pattern "correctZoneAirTemps" -Description "source-map zone correction routine"
Assert-Contains -Path "docs\src\porting-map\heat-balance-source-map.md" -Pattern "source-map entry" -Description "source-map stop rule"

Assert-Contains -Path "docs\src\porting-map\output-variable-source-map.md" -Pattern "Zone Mean Air Temperature" -Description "output-variable MAT map"
Assert-Contains -Path "docs\src\porting-map\output-variable-source-map.md" -Pattern "Surface Inside Face Temperature" -Description "output-variable surface map"
Assert-Contains -Path "docs\src\porting-map\output-variable-source-map.md" -Pattern "Zone Air Heat Balance Surface Convection Rate" -Description "output-variable heat-balance map"
Assert-Contains -Path "docs\src\porting-map\output-variable-source-map.md" -Pattern "SetupOutputVariable" -Description "output registration marker"

Assert-Contains -Path "docs\src\porting-map\algorithm-porting-readiness.md" -Pattern "Not Allowed In v0.7" -Description "algorithm no-claim section"
Assert-Contains -Path "docs\src\porting-map\algorithm-porting-readiness.md" -Pattern "zone-temperature pass wording" -Description "algorithm false-claim boundary"
Assert-Contains -Path "docs\src\porting-map\algorithm-porting-readiness.md" -Pattern "v0.8 Entry Rule" -Description "algorithm entry rule"

Write-Host "milestone: v0.7.0"
Write-Host "scope: EnergyPlus source mapping and algorithm porting readiness"
Write-Host "claim: planning guard only; no heat-balance conformance"

Invoke-DevCommand -Command "source-smoke"
Invoke-DevCommand -Command "test"
Invoke-DevCommand -Command "docs-check"
Invoke-DevCommand -Command "strict-no-false-conformance"

Write-Host "result: pass"
Write-Host "v0.7.0 source mapping verification passed."
