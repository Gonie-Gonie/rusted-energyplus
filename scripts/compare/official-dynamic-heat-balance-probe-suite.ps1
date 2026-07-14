[CmdletBinding()]
param(
    [switch]$SkipSummary,
    [switch]$IncludeClosed
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

# The default suite is the machine-readable active probe set. Each lane must
# map one-to-one to an unresolved source-state hypothesis in
# specs/diagnostic_probe_ledger.toml.
$activeLanes = @(
    "official-dynamic-heat-balance-warmup-surftempin-first-sample-probe.ps1"
)

# Historical cumulative probes are retained only for explicit replay. They do
# not represent unresolved hypotheses and are excluded from the default suite.
# They remain available through -IncludeClosed and their retained direct replay
# commands so historical evidence stays reproducible.
$closedLanes = @(
    "official-dynamic-heat-balance-all-ctf-analytical-coupled-iter3-probe.ps1",
    "official-dynamic-heat-balance-all-ctf-analytical-coupled-previous-boundary-iter3-probe.ps1",
    "official-dynamic-heat-balance-all-ctf-analytical-coupled-previous-inside-doe2-iter3-probe.ps1",
    "official-dynamic-heat-balance-all-ctf-analytical-coupled-previous-inside-iter3-probe.ps1",
    "official-dynamic-heat-balance-all-ctf-analytical-coupled-previous-inside-quick-outside-doe2-interior-longwave-iter5-probe.ps1",
    "official-dynamic-heat-balance-all-ctf-analytical-coupled-previous-inside-quick-outside-doe2-iter3-probe.ps1",
    "official-dynamic-heat-balance-all-ctf-analytical-coupled-previous-inside-quick-outside-doe2-iter5-probe.ps1",
    "official-dynamic-heat-balance-all-ctf-analytical-coupled-previous-inside-quick-outside-doe2-scriptf-lw-iter5-probe.ps1",
    "official-dynamic-heat-balance-all-ctf-analytical-coupled-previous-inside-quick-outside-epseed-iter5-probe.ps1",
    "official-dynamic-heat-balance-all-ctf-analytical-coupled-previous-inside-quick-outside-interior-longwave-iter20-probe.ps1",
    "official-dynamic-heat-balance-all-ctf-analytical-coupled-previous-inside-quick-outside-interior-longwave-iter5-probe.ps1",
    "official-dynamic-heat-balance-all-ctf-analytical-coupled-previous-inside-quick-outside-interleaved-iter20-probe.ps1",
    "official-dynamic-heat-balance-all-ctf-analytical-coupled-previous-inside-quick-outside-interleaved-iter8-probe.ps1",
    "official-dynamic-heat-balance-all-ctf-analytical-coupled-previous-inside-quick-outside-interleaved-lw-frozen-hconv-current-adiabatic-iter20-probe.ps1",
    "official-dynamic-heat-balance-all-ctf-analytical-coupled-previous-inside-quick-outside-interleaved-lw-frozen-hconv-current-lw-iter20-probe.ps1",
    "official-dynamic-heat-balance-all-ctf-analytical-coupled-previous-inside-quick-outside-interleaved-lw-frozen-hconv-iter20-probe.ps1",
    "official-dynamic-heat-balance-all-ctf-analytical-coupled-previous-inside-quick-outside-interleaved-lw-iter20-probe.ps1",
    "official-dynamic-heat-balance-all-ctf-analytical-coupled-previous-inside-quick-outside-interleaved-scriptf-lw-iter20-probe.ps1",
    "official-dynamic-heat-balance-all-ctf-analytical-coupled-previous-inside-quick-outside-iter3-probe.ps1",
    "official-dynamic-heat-balance-all-ctf-analytical-coupled-previous-inside-quick-outside-iter5-probe.ps1",
    "official-dynamic-heat-balance-all-ctf-analytical-coupled-previous-inside-quick-outside-iter8-probe.ps1",
    "official-dynamic-heat-balance-all-ctf-analytical-coupled-previous-inside-quick-outside-scriptf-lw-iter5-probe.ps1",
    "official-dynamic-heat-balance-all-ctf-analytical-coupled-probe.ps1",
    "official-dynamic-heat-balance-all-ctf-analytical-surface-first-iter3-probe.ps1",
    "official-dynamic-heat-balance-all-ctf-analytical-surface-first-probe.ps1",
    "official-dynamic-heat-balance-all-ctf-probe.ps1",
    "official-dynamic-heat-balance-all-ctf-surface-iter3-probe.ps1",
    "official-dynamic-heat-balance-all-ctf-third-order-coupled-previous-inside-quick-outside-interleaved-lw-frozen-hconv-iter20-probe.ps1",
    "official-dynamic-heat-balance-all-ctf-third-order-coupled-previous-inside-quick-outside-interleaved-lw-frozen-hconv-weather-air-storage-iter20-probe.ps1",
    "official-dynamic-heat-balance-all-ctf-third-order-coupled-previous-inside-quick-outside-interleaved-lw-iter20-probe.ps1",
    "official-dynamic-heat-balance-all-ctf-warmup-20-probe.ps1",
    "official-dynamic-heat-balance-analytical-probe.ps1",
    "official-dynamic-heat-balance-analytical-surface-first-probe.ps1",
    "official-dynamic-heat-balance-third-order-probe.ps1",
    "official-dynamic-heat-balance-third-order-weather-storage-balance-surfconv-current-adhist-iter20-probe.ps1",
    "official-dynamic-heat-balance-third-order-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-adhist-commit-iter20-probe.ps1",
    "official-dynamic-heat-balance-third-order-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-frozen-outside-iter20-probe.ps1",
    "official-dynamic-heat-balance-third-order-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-commit-iter20-probe.ps1",
    "official-dynamic-heat-balance-third-order-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-iter20-probe.ps1",
    "official-dynamic-heat-balance-third-order-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-scriptf-flat-hconv-reeval2-iter20-probe.ps1",
    "official-dynamic-heat-balance-third-order-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-scriptf-flat-hconv-reeval30-iter20-probe.ps1",
    "official-dynamic-heat-balance-third-order-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-scriptf-flat-iter20-probe.ps1",
    "official-dynamic-heat-balance-third-order-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-scriptf-iter20-probe.ps1",
    "official-dynamic-heat-balance-third-order-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-iter20-probe.ps1",
    "official-dynamic-heat-balance-third-order-weather-storage-balance-surfconv-frozen-refair-current-lw-iter20-probe.ps1",
    "official-dynamic-heat-balance-third-order-weather-storage-balance-surfconv-frozen-refair-iter20-probe.ps1",
    "official-dynamic-heat-balance-third-order-weather-storage-balance-surfconv-iter20-probe.ps1",
    "official-dynamic-heat-balance-third-order-weather-storage-prevmat-surfconv-iter20-probe.ps1",
    "official-dynamic-heat-balance-warmup-20-probe.ps1",
    "..\internal\probes\official-hb-flat-adhist-report-iter20.ps1",
    "..\internal\probes\official-hb-flat-final-hconv-report-iter20.ps1",
    "..\internal\probes\official-hb-flat-inside-ctf-report-iter20.ps1",
    "..\internal\probes\official-hb-flat-live-hconv-iter20.ps1",
    "..\internal\probes\official-hb-flat-live-refair-iter20.ps1",
    "..\internal\probes\official-hb-flat-surf-refair-report-iter20.ps1",
    "..\internal\probes\official-hb-flat-zone-surf-report-iter20.ps1"
)

$lanes = @($activeLanes)
if ($IncludeClosed) {
    $lanes += $closedLanes
}

foreach ($lane in $lanes) {
    Write-Host "Running $lane"
    & (Join-Path $PSScriptRoot $lane)
}

if (-not $SkipSummary -and $IncludeClosed) {
    Write-Host "Running official-dynamic-heat-balance-probe-summary.ps1"
    & (Join-Path $PSScriptRoot "official-dynamic-heat-balance-probe-summary.ps1")
}
elseif (-not $SkipSummary) {
    Write-Host "Skipping the historical probe summary for the active-only suite."
}
