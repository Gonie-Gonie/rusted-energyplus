[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$params = @{
    CtfSeedPolicy = "all-eio"
    ZoneAirAlgorithm = "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-inside-ctf-outside-history-scriptf-flat-inside-ctf-report-probe"
    WarmupMinimumDays = 20
    SurfaceIterations = 20
}

& (Join-Path $PSScriptRoot "official-dynamic-heat-balance-diagnostic.ps1") @params
