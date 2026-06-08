[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

& (Join-Path $PSScriptRoot "official-dynamic-heat-balance-diagnostic.ps1") -CtfSeedPolicy all-eio -ZoneAirAlgorithm energyplus-analytical-coupled-previous-inside-probe -SurfaceIterations 3
