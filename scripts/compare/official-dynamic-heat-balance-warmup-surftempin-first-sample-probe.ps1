[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

# Observation-only replay of the promoted compatibility runtime. This lane
# changes no algorithm flag; it directly compares EnergyPlus SurfTempIn
# (reported as Surface Inside Face Temperature) with the Rust inside-face
# state at the first run-period sample after repeated-day warmup.
$OutputRootRelative = ".runtime\official-dynamic-probe-warmup-surftempin-first-sample\26.1.0"
$diagnosticArgs = @{
    CtfSeedPolicy = "all-eio"
    CtfInitialHistoryPolicy = "energyplus-surf-initial"
    ZoneAirAlgorithm = "energyplus-heat-balance-compat-candidate"
    WarmupMinimumDays = 20
    SurfaceIterations = 20
    OutputRootRelativeOverride = $OutputRootRelative
    ObservationOnly = $true
}

& (Join-Path $PSScriptRoot "official-dynamic-heat-balance-diagnostic.ps1") @diagnosticArgs

$RepoRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..\..")).Path
$DigestPath = Join-Path $RepoRoot "$OutputRootRelative\official_1zone_uncontrolled_dynamic_diagnostic_001\compare\compare-digest.json"
if (-not (Test-Path -LiteralPath $DigestPath -PathType Leaf)) {
    throw "Missing SurfTempIn first-sample digest: $DigestPath"
}

$digest = Get-Content -LiteralPath $DigestPath -Raw | ConvertFrom-Json
if ($null -eq $digest.zone_air_warmup_day_end_states -or $digest.zone_air_warmup_day_end_states.Count -eq 0) {
    throw "SurfTempIn first-sample probe requires a nonempty warmup day-end trace."
}

$firstSampleRows = @($digest.ctf_history_first_sample_deltas)
if ($firstSampleRows.Count -eq 0) {
    throw "SurfTempIn first-sample probe requires first-run-period CTF rows."
}
$ObservationKey = "ZN001:FLR001"
$MinimumAbsDeltaC = 1.0e-9
$mismatchRows = @($firstSampleRows | Where-Object {
        if ($_.key -ne $ObservationKey) {
            return $false
        }
        $oracle = [double]$_.oracle_inside_face_temperature_c
        $rust = [double]$_.rust_inside_face_temperature_c
        $reportedDelta = [double]$_.inside_face_temperature_delta_c
        return (
            -not [double]::IsNaN($oracle) -and -not [double]::IsInfinity($oracle) -and
            -not [double]::IsNaN($rust) -and -not [double]::IsInfinity($rust) -and
            -not [double]::IsNaN($reportedDelta) -and -not [double]::IsInfinity($reportedDelta) -and
            [Math]::Abs($oracle - $rust) -gt $MinimumAbsDeltaC -and
            [Math]::Abs($reportedDelta - [Math]::Abs($oracle - $rust)) -le 1.0e-12
        )
    })
if ($mismatchRows.Count -eq 0) {
    throw "SurfTempIn first-sample hypothesis was falsified: no direct EnergyPlus/Rust state mismatch was observed for $ObservationKey."
}

$strongestMismatch = $mismatchRows |
    Sort-Object { [Math]::Abs([double]$_.inside_face_temperature_delta_c) } -Descending |
    Select-Object -First 1
Write-Host (
    "OK SurfTempIn first-sample state mismatch: key=$($strongestMismatch.key), " +
    "energyplus_c=$($strongestMismatch.oracle_inside_face_temperature_c), " +
    "rust_c=$($strongestMismatch.rust_inside_face_temperature_c), " +
    "delta_c=$($strongestMismatch.inside_face_temperature_delta_c), digest=$DigestPath"
)
