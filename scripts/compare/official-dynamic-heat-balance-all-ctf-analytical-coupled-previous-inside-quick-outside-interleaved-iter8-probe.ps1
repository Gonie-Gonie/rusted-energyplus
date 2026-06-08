[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$diagnosticArgs = @{
    CtfSeedPolicy = "all-eio"
    ZoneAirAlgorithm = "energyplus-analytical-coupled-previous-inside-quick-outside-interleaved-probe"
    SurfaceIterations = 8
}

& (Join-Path $PSScriptRoot "official-dynamic-heat-balance-diagnostic.ps1") @diagnosticArgs
