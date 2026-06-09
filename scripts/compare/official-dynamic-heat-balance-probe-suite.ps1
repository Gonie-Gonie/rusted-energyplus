[CmdletBinding()]
param(
    [switch]$SkipSummary
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$lanes = @(
    "official-dynamic-heat-balance-diagnostic.ps1",
    "official-dynamic-heat-balance-all-ctf-probe.ps1",
    "official-dynamic-heat-balance-all-ctf-warmup-20-probe.ps1",
    "official-dynamic-heat-balance-all-ctf-surface-iter3-probe.ps1",
    "official-dynamic-heat-balance-analytical-probe.ps1",
    "official-dynamic-heat-balance-analytical-surface-first-probe.ps1",
    "official-dynamic-heat-balance-all-ctf-analytical-surface-first-probe.ps1",
    "official-dynamic-heat-balance-all-ctf-analytical-coupled-probe.ps1",
    "official-dynamic-heat-balance-all-ctf-analytical-coupled-iter3-probe.ps1",
    "official-dynamic-heat-balance-all-ctf-analytical-coupled-previous-inside-iter3-probe.ps1",
    "official-dynamic-heat-balance-all-ctf-analytical-coupled-previous-inside-doe2-iter3-probe.ps1",
    "official-dynamic-heat-balance-all-ctf-analytical-coupled-previous-inside-quick-outside-iter3-probe.ps1",
    "official-dynamic-heat-balance-all-ctf-analytical-coupled-previous-inside-quick-outside-doe2-iter3-probe.ps1",
    "official-dynamic-heat-balance-all-ctf-analytical-coupled-previous-inside-quick-outside-iter5-probe.ps1",
    "official-dynamic-heat-balance-all-ctf-analytical-coupled-previous-inside-quick-outside-iter8-probe.ps1",
    "official-dynamic-heat-balance-all-ctf-analytical-coupled-previous-inside-quick-outside-interleaved-iter8-probe.ps1",
    "official-dynamic-heat-balance-all-ctf-analytical-coupled-previous-inside-quick-outside-interleaved-iter20-probe.ps1",
    "official-dynamic-heat-balance-all-ctf-analytical-coupled-previous-inside-quick-outside-interleaved-lw-iter20-probe.ps1",
    "official-dynamic-heat-balance-all-ctf-third-order-coupled-previous-inside-quick-outside-interleaved-lw-frozen-hconv-iter20-probe.ps1",
    "official-dynamic-heat-balance-all-ctf-third-order-coupled-previous-inside-quick-outside-interleaved-lw-iter20-probe.ps1",
    "official-dynamic-heat-balance-all-ctf-third-order-coupled-previous-inside-quick-outside-interleaved-lw-frozen-hconv-weather-air-storage-iter20-probe.ps1",
    "official-dynamic-heat-balance-third-order-weather-storage-prevmat-surfconv-iter20-probe.ps1",
    "official-dynamic-heat-balance-third-order-weather-storage-balance-surfconv-iter20-probe.ps1",
    "official-dynamic-heat-balance-third-order-weather-storage-balance-surfconv-current-adhist-iter20-probe.ps1",
    "official-dynamic-heat-balance-all-ctf-analytical-coupled-previous-inside-quick-outside-epseed-iter5-probe.ps1",
    "official-dynamic-heat-balance-all-ctf-analytical-coupled-previous-inside-quick-outside-doe2-iter5-probe.ps1",
    "official-dynamic-heat-balance-all-ctf-analytical-coupled-previous-inside-quick-outside-interior-longwave-iter5-probe.ps1",
    "official-dynamic-heat-balance-all-ctf-analytical-coupled-previous-inside-quick-outside-doe2-interior-longwave-iter5-probe.ps1",
    "official-dynamic-heat-balance-all-ctf-analytical-coupled-previous-inside-quick-outside-scriptf-lw-iter5-probe.ps1",
    "official-dynamic-heat-balance-all-ctf-analytical-coupled-previous-inside-quick-outside-doe2-scriptf-lw-iter5-probe.ps1",
    "official-dynamic-heat-balance-all-ctf-analytical-coupled-previous-boundary-iter3-probe.ps1",
    "official-dynamic-heat-balance-all-ctf-analytical-surface-first-iter3-probe.ps1",
    "official-dynamic-heat-balance-third-order-probe.ps1",
    "official-dynamic-heat-balance-warmup-20-probe.ps1"
)

foreach ($lane in $lanes) {
    Write-Host "Running $lane"
    & (Join-Path $PSScriptRoot $lane)
}

if (-not $SkipSummary) {
    Write-Host "Running official-dynamic-heat-balance-probe-summary.ps1"
    & (Join-Path $PSScriptRoot "official-dynamic-heat-balance-probe-summary.ps1")
}
