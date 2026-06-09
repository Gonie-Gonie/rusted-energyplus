[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$diagnosticArgs = @{
    CtfSeedPolicy = "all-eio"
    ZoneAirAlgorithm = "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-current-adiabatic-history-probe"
    SurfaceIterations = 20
}

& (Join-Path $PSScriptRoot "official-dynamic-heat-balance-diagnostic.ps1") @diagnosticArgs
