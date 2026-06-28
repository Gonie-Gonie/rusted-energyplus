[CmdletBinding()]
param(
    [ValidateSet("steady-no-mass-only", "all-eio")]
    [string]$CtfSeedPolicy = "steady-no-mass-only",
    [ValidateSet("boundary-u-value", "energyplus-surf-initial")]
    [string]$CtfInitialHistoryPolicy = "boundary-u-value",
    [ValidateSet("simplified-analytical", "energyplus-heat-balance-compat-candidate", "energyplus-analytical-probe", "energyplus-analytical-surface-first-probe", "energyplus-analytical-coupled-probe", "energyplus-analytical-coupled-previous-inside-probe", "energyplus-analytical-coupled-previous-inside-doe2-probe", "energyplus-analytical-coupled-previous-inside-quick-outside-probe", "energyplus-analytical-coupled-previous-inside-quick-outside-interleaved-probe", "energyplus-analytical-coupled-previous-inside-quick-outside-interleaved-interior-longwave-probe", "energyplus-analytical-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-probe", "energyplus-analytical-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-current-longwave-probe", "energyplus-analytical-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-current-adiabatic-probe", "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-probe", "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-probe", "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-probe", "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-previous-mat-surface-convection-probe", "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-probe", "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-probe", "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-probe", "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-probe", "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-frozen-outside-probe", "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-inside-ctf-outside-history-probe", "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-inside-ctf-outside-history-commit-probe", "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-inside-ctf-outside-history-scriptf-probe", "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-inside-ctf-outside-history-scriptf-flat-probe", "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-inside-ctf-outside-history-scriptf-flat-live-reference-air-probe", "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-inside-ctf-outside-history-scriptf-flat-live-hconv-probe", "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-inside-ctf-outside-history-scriptf-flat-surface-reference-air-report-probe", "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-inside-ctf-outside-history-scriptf-flat-final-hconv-report-probe", "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-inside-ctf-outside-history-scriptf-flat-inside-ctf-report-probe", "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-inside-ctf-outside-history-scriptf-flat-adiabatic-report-probe", "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-adiabatic-history-commit-probe", "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-current-adiabatic-history-probe", "energyplus-analytical-coupled-previous-inside-quick-outside-interleaved-scriptf-interior-longwave-probe", "energyplus-analytical-coupled-previous-inside-quick-outside-doe2-probe", "energyplus-analytical-coupled-previous-inside-quick-outside-interior-longwave-probe", "energyplus-analytical-coupled-previous-inside-quick-outside-doe2-interior-longwave-probe", "energyplus-analytical-coupled-previous-inside-quick-outside-scriptf-interior-longwave-probe", "energyplus-analytical-coupled-previous-inside-quick-outside-doe2-scriptf-interior-longwave-probe", "energyplus-analytical-coupled-previous-boundary-probe", "energyplus-third-order-probe")]
    [string]$ZoneAirAlgorithm = "simplified-analytical",
    [ValidateRange(0, 365)]
    [int]$WarmupMinimumDays = 0,
    [ValidateRange(1, 200)]
    [int]$SurfaceIterations = 1,
    [ValidateRange(0, 200)]
    [int]$InsideHconvReevaluationInterval = 0,
    [ValidateSet("zone-state", "surface-report")]
    [string]$ZoneConductionReportSource = "zone-state",
    [ValidateSet("average", "last-system-state")]
    [string]$ZoneAirReportSampling = "average",
    [ValidateSet("each-surface-iteration", "after-surface-loop")]
    [string]$SurfaceLoopZoneAirCorrection = "each-surface-iteration",
    [string]$CaseId = "official_1zone_uncontrolled_dynamic_diagnostic_001",
    [string]$OutputRootRelativeOverride = ""
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
if (
    -not $PSBoundParameters.ContainsKey("SurfaceLoopZoneAirCorrection") -and
    $ZoneAirAlgorithm -eq "energyplus-heat-balance-compat-candidate"
) {
    $SurfaceLoopZoneAirCorrection = "after-surface-loop"
}
$AlgorithmOutputSuffix = switch ($ZoneAirAlgorithm) {
    "energyplus-heat-balance-compat-candidate" { "-compat-candidate" }
    "energyplus-analytical-probe" { "-analytical" }
    "energyplus-analytical-surface-first-probe" { "-analytical-surface-first" }
    "energyplus-analytical-coupled-probe" { "-analytical-coupled" }
    "energyplus-analytical-coupled-previous-inside-probe" { "-analytical-coupled-previous-inside" }
    "energyplus-analytical-coupled-previous-inside-doe2-probe" { "-analytical-coupled-previous-inside-doe2" }
    "energyplus-analytical-coupled-previous-inside-quick-outside-probe" { "-analytical-coupled-previous-inside-quick-outside" }
    "energyplus-analytical-coupled-previous-inside-quick-outside-interleaved-probe" { "-analytical-coupled-previous-inside-quick-outside-interleaved" }
    "energyplus-analytical-coupled-previous-inside-quick-outside-interleaved-interior-longwave-probe" { "-analytical-coupled-previous-inside-quick-outside-interleaved-lw" }
    "energyplus-analytical-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-probe" { "-analytical-coupled-previous-inside-quick-outside-interleaved-lw-frozen-hconv" }
    "energyplus-analytical-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-current-longwave-probe" { "-analytical-coupled-previous-inside-quick-outside-interleaved-lw-frozen-hconv-current-lw" }
    "energyplus-analytical-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-current-adiabatic-probe" { "-analytical-coupled-previous-inside-quick-outside-interleaved-lw-frozen-hconv-current-adiabatic" }
    "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-probe" { "-third-order-coupled-previous-inside-quick-outside-interleaved-lw" }
    "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-probe" { "-third-order-coupled-previous-inside-quick-outside-interleaved-lw-frozen-hconv" }
    "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-probe" { "-third-order-coupled-previous-inside-quick-outside-interleaved-lw-frozen-hconv-weather-air-storage" }
    "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-previous-mat-surface-convection-probe" { "-third-order-frozen-hconv-weather-storage-prevmat-surfconv" }
    "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-probe" { "-third-order-frozen-hconv-weather-storage-balance-surfconv" }
    "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-probe" { "-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair" }
    "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-probe" { "-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw" }
    "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-probe" { "-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged" }
    "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-frozen-outside-probe" { "-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-frozen-outside" }
    "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-inside-ctf-outside-history-probe" { "-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist" }
    "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-inside-ctf-outside-history-commit-probe" { "-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-commit" }
    "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-inside-ctf-outside-history-scriptf-probe" { "-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-scriptf" }
    "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-inside-ctf-outside-history-scriptf-flat-probe" { "-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-scriptf-flat" }
    "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-inside-ctf-outside-history-scriptf-flat-live-reference-air-probe" { "-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-scriptf-flat-live-refair" }
    "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-inside-ctf-outside-history-scriptf-flat-live-hconv-probe" { "-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-scriptf-flat-live-hconv" }
    "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-inside-ctf-outside-history-scriptf-flat-surface-reference-air-report-probe" { "-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-scriptf-flat-surf-refair-report" }
    "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-inside-ctf-outside-history-scriptf-flat-final-hconv-report-probe" { "-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-scriptf-flat-final-hconv-report" }
    "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-inside-ctf-outside-history-scriptf-flat-inside-ctf-report-probe" { "-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-scriptf-flat-inside-ctf-report" }
    "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-inside-ctf-outside-history-scriptf-flat-adiabatic-report-probe" { "-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-inside-ctf-out-hist-scriptf-flat-adhist-report" }
    "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-adiabatic-history-commit-probe" { "-third-order-frozen-hconv-weather-storage-balance-surfconv-frozen-refair-current-lw-converged-adhist-commit" }
    "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-current-adiabatic-history-probe" { "-third-order-frozen-hconv-weather-storage-balance-surfconv-current-adhist" }
    "energyplus-analytical-coupled-previous-inside-quick-outside-interleaved-scriptf-interior-longwave-probe" { "-analytical-coupled-previous-inside-quick-outside-interleaved-scriptf-lw" }
    "energyplus-analytical-coupled-previous-inside-quick-outside-doe2-probe" { "-analytical-coupled-previous-inside-quick-outside-doe2" }
    "energyplus-analytical-coupled-previous-inside-quick-outside-interior-longwave-probe" { "-analytical-coupled-previous-inside-quick-outside-lw" }
    "energyplus-analytical-coupled-previous-inside-quick-outside-doe2-interior-longwave-probe" { "-analytical-coupled-previous-inside-quick-outside-doe2-lw" }
    "energyplus-analytical-coupled-previous-inside-quick-outside-scriptf-interior-longwave-probe" { "-analytical-coupled-previous-inside-quick-outside-scriptf-lw" }
    "energyplus-analytical-coupled-previous-inside-quick-outside-doe2-scriptf-interior-longwave-probe" { "-analytical-coupled-previous-inside-quick-outside-doe2-scriptf-lw" }
    "energyplus-analytical-coupled-previous-boundary-probe" { "-analytical-coupled-previous-boundary" }
    "energyplus-third-order-probe" { "-third-order" }
    Default { "" }
}
$WarmupOutputSuffix = if ($WarmupMinimumDays -gt 0) {
    "-warmup-min$WarmupMinimumDays"
}
else {
    ""
}
$InitialHistoryOutputSuffix = if ($CtfInitialHistoryPolicy -eq "energyplus-surf-initial") {
    "-epseed"
}
else {
    ""
}
$SurfaceIterationOutputSuffix = if ($SurfaceIterations -gt 1) {
    "-surface-iter$SurfaceIterations"
}
else {
    ""
}
$InsideHconvReevaluationOutputSuffix = if ($InsideHconvReevaluationInterval -gt 0) {
    "-hconv-reeval$InsideHconvReevaluationInterval"
}
else {
    ""
}
$ZoneConductionReportOutputSuffix = if ($ZoneConductionReportSource -eq "surface-report") {
    "-zone-surf-report"
}
else {
    ""
}
$ZoneAirReportSamplingOutputSuffix = if ($ZoneAirReportSampling -eq "last-system-state") {
    "-zone-air-last"
}
else {
    ""
}
$SurfaceLoopZoneAirCorrectionOutputSuffix = if ($SurfaceLoopZoneAirCorrection -eq "after-surface-loop") {
    "-zone-after-surface-loop"
}
else {
    ""
}
$ComputedOutputRootRelative = if ($CtfSeedPolicy -eq "all-eio") {
    ".runtime\official-dynamic-diagnostic-all-ctf$AlgorithmOutputSuffix$InitialHistoryOutputSuffix$WarmupOutputSuffix$SurfaceIterationOutputSuffix$InsideHconvReevaluationOutputSuffix$ZoneConductionReportOutputSuffix$ZoneAirReportSamplingOutputSuffix$SurfaceLoopZoneAirCorrectionOutputSuffix\26.1.0"
}
else {
    ".runtime\official-dynamic-diagnostic$AlgorithmOutputSuffix$InitialHistoryOutputSuffix$WarmupOutputSuffix$SurfaceIterationOutputSuffix$InsideHconvReevaluationOutputSuffix$ZoneConductionReportOutputSuffix$ZoneAirReportSamplingOutputSuffix$SurfaceLoopZoneAirCorrectionOutputSuffix\26.1.0"
}
$OutputRootRelative = if ($OutputRootRelativeOverride.Trim().Length -gt 0) {
    $OutputRootRelativeOverride
}
else {
    $ComputedOutputRootRelative
}
$OutputRoot = Join-Path $RepoRoot $OutputRootRelative
$CasePath = Join-Path $RepoRoot "data\conformance_cases\$CaseId\case.toml"
$CaseOutputRoot = Join-Path $OutputRoot $CaseId
$CompareRoot = Join-Path $CaseOutputRoot "compare"

function Assert-RepoSubPath {
    param([Parameter(Mandatory = $true)][string]$Path)
    $full = [System.IO.Path]::GetFullPath($Path)
    $root = [System.IO.Path]::GetFullPath($RepoRoot)
    if (-not $full.StartsWith($root, [System.StringComparison]::OrdinalIgnoreCase)) {
        throw "Refusing to operate outside repository: $full"
    }
}

function Remove-RepoDirectory {
    param([Parameter(Mandatory = $true)][string]$Path)
    if (Test-Path -LiteralPath $Path) {
        Assert-RepoSubPath -Path $Path
        Remove-Item -LiteralPath $Path -Recurse -Force
    }
}

function Assert-Contains {
    param(
        [Parameter(Mandatory = $true)][string]$Text,
        [Parameter(Mandatory = $true)][string]$Pattern,
        [Parameter(Mandatory = $true)][string]$Description
    )
    if ($Text -notmatch [regex]::Escape($Pattern)) {
        Write-Host $Text
        throw "Missing $Description`: $Pattern"
    }
    Write-Host "OK $Description`: $Pattern"
}

function Assert-NotContains {
    param(
        [Parameter(Mandatory = $true)][string]$Text,
        [Parameter(Mandatory = $true)][string]$Pattern,
        [Parameter(Mandatory = $true)][string]$Description
    )
    if ($Text -match [regex]::Escape($Pattern)) {
        throw "Unexpected $Description`: $Pattern"
    }
    Write-Host "OK no $Description`: $Pattern"
}

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

function Get-SeriesDiagnostic {
    param(
        [Parameter(Mandatory = $true)][object]$Summary,
        [Parameter(Mandatory = $true)][string]$Key,
        [Parameter(Mandatory = $true)][string]$Variable
    )

    return @($Summary.series | Where-Object {
            $_.output.key -eq $Key -and $_.output.variable -eq $Variable
        })[0]
}

function Assert-SeriesRmseBelow {
    param(
        [Parameter(Mandatory = $true)][object]$Summary,
        [Parameter(Mandatory = $true)][string]$Key,
        [Parameter(Mandatory = $true)][string]$Variable,
        [Parameter(Mandatory = $true)][double]$MaxRmse,
        [Parameter(Mandatory = $true)][string]$Description
    )

    $series = Get-SeriesDiagnostic -Summary $Summary -Key $Key -Variable $Variable
    if ($null -eq $series) {
        throw "Missing series for ${Description}: ${Key} / ${Variable}"
    }
    if ([double]$series.rmse_delta_c -gt $MaxRmse) {
        throw "Expected ${Description} RMSE <= $MaxRmse, got $($series.rmse_delta_c)"
    }
    Write-Host "OK ${Description} RMSE: $($series.rmse_delta_c)"
}

foreach ($path in @(
    (Join-Path $OracleRoot "energyplus.exe"),
    (Join-Path $OracleRoot "ConvertInputFormat.exe"),
    (Join-Path $OracleRoot "ExampleFiles\1ZoneUncontrolled.idf"),
    (Join-Path $OracleRoot "WeatherData\USA_CO_Golden-NREL.724666_TMY3.epw"),
    $CasePath
)) {
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Missing required official dynamic diagnostic file: $path"
    }
}

Remove-RepoDirectory -Path $OutputRoot

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Write-Host "Running official dynamic heat-balance diagnostic gate with CTF seed policy $CtfSeedPolicy, CTF initial history policy $CtfInitialHistoryPolicy, zone-air algorithm $ZoneAirAlgorithm, warmup minimum days $WarmupMinimumDays, surface iterations $SurfaceIterations, inside hconv reevaluation interval $InsideHconvReevaluationInterval, zone conduction report source $ZoneConductionReportSource, zone air report sampling $ZoneAirReportSampling, and surface loop zone-air correction $SurfaceLoopZoneAirCorrection."
$policyEnvName = "RUSTED_ENERGYPLUS_HEAT_BALANCE_CTF_SEED_POLICY"
$initialHistoryPolicyEnvName = "RUSTED_ENERGYPLUS_HEAT_BALANCE_CTF_INITIAL_HISTORY_POLICY"
$algorithmEnvName = "RUSTED_ENERGYPLUS_HEAT_BALANCE_ZONE_AIR_ALGORITHM"
$warmupEnvName = "RUSTED_ENERGYPLUS_HEAT_BALANCE_WARMUP_MINIMUM_DAYS"
$surfaceIterationsEnvName = "RUSTED_ENERGYPLUS_HEAT_BALANCE_SURFACE_ITERATIONS"
$insideHconvReevaluationIntervalEnvName = "RUSTED_ENERGYPLUS_HEAT_BALANCE_INSIDE_HCONV_REEVALUATION_INTERVAL"
$zoneConductionReportSourceEnvName = "RUSTED_ENERGYPLUS_HEAT_BALANCE_ZONE_CONDUCTION_REPORT_SOURCE"
$zoneAirReportSamplingEnvName = "RUSTED_ENERGYPLUS_HEAT_BALANCE_ZONE_AIR_REPORT_SAMPLING"
$surfaceLoopZoneAirCorrectionEnvName = "RUSTED_ENERGYPLUS_HEAT_BALANCE_SURFACE_LOOP_ZONE_AIR_CORRECTION"
$previousPolicy = [Environment]::GetEnvironmentVariable($policyEnvName, "Process")
$previousInitialHistoryPolicy = [Environment]::GetEnvironmentVariable($initialHistoryPolicyEnvName, "Process")
$previousAlgorithm = [Environment]::GetEnvironmentVariable($algorithmEnvName, "Process")
$previousWarmup = [Environment]::GetEnvironmentVariable($warmupEnvName, "Process")
$previousSurfaceIterations = [Environment]::GetEnvironmentVariable($surfaceIterationsEnvName, "Process")
$previousInsideHconvReevaluationInterval = [Environment]::GetEnvironmentVariable($insideHconvReevaluationIntervalEnvName, "Process")
$previousZoneConductionReportSource = [Environment]::GetEnvironmentVariable($zoneConductionReportSourceEnvName, "Process")
$previousZoneAirReportSampling = [Environment]::GetEnvironmentVariable($zoneAirReportSamplingEnvName, "Process")
$previousSurfaceLoopZoneAirCorrection = [Environment]::GetEnvironmentVariable($surfaceLoopZoneAirCorrectionEnvName, "Process")
try {
    [Environment]::SetEnvironmentVariable($policyEnvName, $CtfSeedPolicy, "Process")
    [Environment]::SetEnvironmentVariable($initialHistoryPolicyEnvName, $CtfInitialHistoryPolicy, "Process")
    [Environment]::SetEnvironmentVariable($algorithmEnvName, $ZoneAirAlgorithm, "Process")
    if ($WarmupMinimumDays -gt 0) {
        [Environment]::SetEnvironmentVariable($warmupEnvName, [string]$WarmupMinimumDays, "Process")
    }
    else {
        [Environment]::SetEnvironmentVariable($warmupEnvName, $null, "Process")
    }
    [Environment]::SetEnvironmentVariable($surfaceIterationsEnvName, [string]$SurfaceIterations, "Process")
    if ($InsideHconvReevaluationInterval -gt 0) {
        [Environment]::SetEnvironmentVariable($insideHconvReevaluationIntervalEnvName, [string]$InsideHconvReevaluationInterval, "Process")
    }
    else {
        [Environment]::SetEnvironmentVariable($insideHconvReevaluationIntervalEnvName, $null, "Process")
    }
    [Environment]::SetEnvironmentVariable($zoneConductionReportSourceEnvName, $ZoneConductionReportSource, "Process")
    [Environment]::SetEnvironmentVariable($zoneAirReportSamplingEnvName, $ZoneAirReportSampling, "Process")
    [Environment]::SetEnvironmentVariable($surfaceLoopZoneAirCorrectionEnvName, $SurfaceLoopZoneAirCorrection, "Process")
    $previousErrorActionPreference = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    try {
        $output = & $cargo.Source run -p ep_cli --quiet -- conformance heat-balance-diagnostic-report $CasePath $OracleRoot $OutputRoot 2>&1
    }
    finally {
        $ErrorActionPreference = $previousErrorActionPreference
    }
}
finally {
    [Environment]::SetEnvironmentVariable($policyEnvName, $previousPolicy, "Process")
    [Environment]::SetEnvironmentVariable($initialHistoryPolicyEnvName, $previousInitialHistoryPolicy, "Process")
    [Environment]::SetEnvironmentVariable($algorithmEnvName, $previousAlgorithm, "Process")
    [Environment]::SetEnvironmentVariable($warmupEnvName, $previousWarmup, "Process")
    [Environment]::SetEnvironmentVariable($surfaceIterationsEnvName, $previousSurfaceIterations, "Process")
    [Environment]::SetEnvironmentVariable($insideHconvReevaluationIntervalEnvName, $previousInsideHconvReevaluationInterval, "Process")
    [Environment]::SetEnvironmentVariable($zoneConductionReportSourceEnvName, $previousZoneConductionReportSource, "Process")
    [Environment]::SetEnvironmentVariable($zoneAirReportSamplingEnvName, $previousZoneAirReportSampling, "Process")
    [Environment]::SetEnvironmentVariable($surfaceLoopZoneAirCorrectionEnvName, $previousSurfaceLoopZoneAirCorrection, "Process")
}
if ($LASTEXITCODE -ne 0) {
    $output | ForEach-Object { Write-Host $_ }
    throw "Official dynamic heat-balance diagnostic failed to generate."
}

$text = ($output -join "`n")
Assert-Contains -Text $text -Pattern "Diagnostic Heat Balance Report" -Description "report header"
Assert-Contains -Text $text -Pattern "id: $CaseId" -Description "case id"
Assert-Contains -Text $text -Pattern "comparison_class: diagnostic-only" -Description "comparison class"
Assert-Contains -Text $text -Pattern "conformance_claim: false" -Description "claim boundary"
Assert-Contains -Text $text -Pattern "warmup_enabled: true" -Description "warmup enabled"
Assert-Contains -Text $text -Pattern "oracle_run_period_warmup_days: 20" -Description "oracle run-period warmup days"
Assert-Contains -Text $text -Pattern "zone_air_algorithm: $ZoneAirAlgorithm" -Description "zone-air algorithm metadata"
$expectedAlgorithmLane = if ($ZoneAirAlgorithm -eq "energyplus-heat-balance-compat-candidate") {
    "compatibility-source-order"
}
elseif ($ZoneAirAlgorithm -eq "simplified-analytical") {
    "diagnostic-only"
}
else {
    "diagnostic-probe"
}
$expectedPromotionAllowed = if ($ZoneAirAlgorithm -eq "energyplus-heat-balance-compat-candidate") { "true" } else { "false" }
$expectedCompatibilitySourceOrder = if ($ZoneAirAlgorithm -eq "energyplus-heat-balance-compat-candidate") { "true" } else { "false" }
$expectedDiagnosticProbeUsed = if ($expectedAlgorithmLane -eq "diagnostic-probe") { "true" } else { "false" }
Assert-Contains -Text $text -Pattern "zone_air_algorithm_lane: $expectedAlgorithmLane" -Description "zone-air algorithm lane metadata"
Assert-Contains -Text $text -Pattern "compatibility_source_order: $expectedCompatibilitySourceOrder" -Description "source-order compatibility metadata"
Assert-Contains -Text $text -Pattern "diagnostic_probe_used: $expectedDiagnosticProbeUsed" -Description "diagnostic probe metadata"
Assert-Contains -Text $text -Pattern "conformance_promotion_allowed: $expectedPromotionAllowed" -Description "conformance promotion lane metadata"
Assert-Contains -Text $text -Pattern "surface_iteration_count: $SurfaceIterations" -Description "surface iteration metadata"
$expectedInsideHconvReevaluationIntervalLabel = if ($InsideHconvReevaluationInterval -gt 0) { [string]$InsideHconvReevaluationInterval } else { "none" }
Assert-Contains -Text $text -Pattern "inside_hconv_reevaluation_interval: $expectedInsideHconvReevaluationIntervalLabel" -Description "inside hconv reevaluation interval metadata"
Assert-Contains -Text $text -Pattern "ctf_initial_history_policy: $CtfInitialHistoryPolicy" -Description "CTF initial history policy metadata"
Assert-Contains -Text $text -Pattern "zone_conduction_report_source: $ZoneConductionReportSource" -Description "zone conduction report source metadata"
Assert-Contains -Text $text -Pattern "zone_air_report_sampling: $ZoneAirReportSampling" -Description "zone air report sampling metadata"
Assert-Contains -Text $text -Pattern "surface_loop_zone_air_correction: $SurfaceLoopZoneAirCorrection" -Description "surface loop zone-air correction metadata"
Assert-Contains -Text $text -Pattern "compare_digest:" -Description "compact digest artifact path"
Assert-Contains -Text $text -Pattern "status: fail" -Description "current diagnostic status"

$summaryPath = Join-Path $CompareRoot "compare-summary.json"
$digestPath = Join-Path $CompareRoot "compare-digest.json"
$reportPath = Join-Path $CompareRoot "compare-report.md"
$zoneAirDebugPath = Join-Path $CompareRoot "rust-zone-air-diagnostics.json"
$isCompatibilityCandidateCase = $CaseId -eq "official_1zone_uncontrolled_dynamic_conformance_candidate_001"
Assert-FileExists -Path $summaryPath -Description "official dynamic diagnostic summary"
Assert-FileExists -Path $digestPath -Description "official dynamic diagnostic digest"
Assert-FileExists -Path $reportPath -Description "official dynamic diagnostic report"
Assert-FileExists -Path $zoneAirDebugPath -Description "official dynamic Rust zone-air diagnostics"

$digestText = Get-Content -LiteralPath $digestPath -Raw
Assert-NotContains -Text $digestText -Pattern '"sample_rows"' -Description "compact digest sample row payload"
Assert-Contains -Text $digestText -Pattern '"compatibility_stages"' -Description "compact digest compatibility stage order"
Assert-Contains -Text $digestText -Pattern '"surface_iteration_max_sample_trace"' -Description "compact digest surface-iteration max-sample trace"
Assert-Contains -Text $digestText -Pattern '"source_routine": "UpdateThermalHistories"' -Description "compact digest UpdateThermalHistories stage"
Assert-Contains -Text $digestText -Pattern '"zone_air_coefficient_deltas"' -Description "compact digest zone-air coefficient diagnostics"
Assert-Contains -Text $digestText -Pattern '"zone_air_surface_convection_closure_deltas"' -Description "compact digest zone-air surface convection closure diagnostics"
Assert-Contains -Text $digestText -Pattern '"zone_air_surface_coefficient_deltas"' -Description "compact digest zone-air surface coefficient diagnostics"
if (-not $isCompatibilityCandidateCase) {
    Assert-Contains -Text $digestText -Pattern '"temp_dependent_coefficient_delta"' -Description "compact digest zone-air TempDepCoef delta"
    Assert-Contains -Text $digestText -Pattern '"temp_history_term_delta"' -Description "compact digest zone-air history-term delta"
    Assert-Contains -Text $digestText -Pattern '"closure_residual_delta"' -Description "compact digest zone-air surface convection closure residual delta"
    Assert-Contains -Text $digestText -Pattern '"reference_air_temperature_delta"' -Description "compact digest zone-air surface reference-air temperature delta"
}
$summary = $digestText | ConvertFrom-Json
if ($summary.case_id -ne $CaseId) {
    throw "Unexpected case_id: $($summary.case_id)"
}
if ($summary.comparison_class -ne "diagnostic-only") {
    throw "Unexpected comparison_class: $($summary.comparison_class)"
}
if ($summary.conformance_claim -ne $false) {
    throw "Official dynamic diagnostic must not claim conformance"
}
if ($summary.gate.blocking -ne $false) {
    throw "Official dynamic diagnostic gate must be non-blocking"
}
if ($summary.artifacts.compare_summary_json -ne "compare-summary.json") {
    throw "Unexpected summary artifact pointer: $($summary.artifacts.compare_summary_json)"
}
if ($summary.artifacts.compare_digest_json -ne "compare-digest.json") {
    throw "Unexpected digest artifact pointer: $($summary.artifacts.compare_digest_json)"
}
if ($summary.artifacts.rust_zone_air_diagnostics_json -ne "rust-zone-air-diagnostics.json") {
    throw "Unexpected Rust zone-air diagnostic artifact pointer: $($summary.artifacts.rust_zone_air_diagnostics_json)"
}
if ($summary.status -ne "fail") {
    throw "Official dynamic diagnostic should remain fail until the case is promoted intentionally: $($summary.status)"
}
if ($summary.samples -ne 8760) {
    throw "Expected RUN PERIOD filtered sample count 8760, got $($summary.samples)"
}
if ($summary.heat_balance_run_period_timesteps -ne 35040) {
    throw "Expected run-period timestep count 35040, got $($summary.heat_balance_run_period_timesteps)"
}
if ($summary.heat_balance_timesteps -le $summary.heat_balance_run_period_timesteps) {
    throw "Expected heat_balance_timesteps to include warmup, got total $($summary.heat_balance_timesteps) and run-period $($summary.heat_balance_run_period_timesteps)"
}
if ($summary.heat_balance_warmup.enabled -ne $true) {
    throw "Expected Rust warmup to be enabled"
}
if ($summary.heat_balance_warmup.timestep_count -le 0) {
    throw "Expected Rust warmup timesteps to be recorded"
}
if ($summary.heat_balance_warmup.oracle_run_period_day_count -ne 20) {
    throw "Expected oracle run-period warmup days 20, got $($summary.heat_balance_warmup.oracle_run_period_day_count)"
}
if ($WarmupMinimumDays -gt 0 -and $summary.heat_balance_warmup.day_count -lt $WarmupMinimumDays) {
    throw "Expected Rust warmup days >= $WarmupMinimumDays, got $($summary.heat_balance_warmup.day_count)"
}
if ($summary.ctf_seed.policy -ne $CtfSeedPolicy) {
    throw "Expected CTF seed policy $CtfSeedPolicy, got $($summary.ctf_seed.policy)"
}
if ($summary.zone_air_algorithm -ne $ZoneAirAlgorithm) {
    throw "Expected zone-air algorithm $ZoneAirAlgorithm, got $($summary.zone_air_algorithm)"
}
if ($summary.zone_air_algorithm_lane -ne $expectedAlgorithmLane) {
    throw "Expected zone_air_algorithm_lane $expectedAlgorithmLane, got $($summary.zone_air_algorithm_lane)"
}
$expectedPromotionAllowedBoolean = $ZoneAirAlgorithm -eq "energyplus-heat-balance-compat-candidate"
$expectedCompatibilitySourceOrderBoolean = $ZoneAirAlgorithm -eq "energyplus-heat-balance-compat-candidate"
$expectedDiagnosticProbeUsedBoolean = $expectedAlgorithmLane -eq "diagnostic-probe"
if ($summary.conformance_promotion_allowed -ne $expectedPromotionAllowedBoolean) {
    throw "Expected conformance_promotion_allowed $expectedPromotionAllowed, got $($summary.conformance_promotion_allowed)"
}
if ($summary.compatibility_source_order -ne $expectedCompatibilitySourceOrderBoolean) {
    throw "Expected compatibility_source_order $expectedCompatibilitySourceOrder, got $($summary.compatibility_source_order)"
}
if ($summary.diagnostic_probe_used -ne $expectedDiagnosticProbeUsedBoolean) {
    throw "Expected diagnostic_probe_used $expectedDiagnosticProbeUsed, got $($summary.diagnostic_probe_used)"
}
if ($summary.surface_iteration_count -ne $SurfaceIterations) {
    throw "Expected surface_iteration_count $SurfaceIterations, got $($summary.surface_iteration_count)"
}
$expectedInsideHconvReevaluationInterval = if ($InsideHconvReevaluationInterval -gt 0) { $InsideHconvReevaluationInterval } else { $null }
if ($null -eq $expectedInsideHconvReevaluationInterval) {
    if ($null -ne $summary.inside_hconv_reevaluation_interval) {
        throw "Expected inside_hconv_reevaluation_interval null, got $($summary.inside_hconv_reevaluation_interval)"
    }
}
elseif ($summary.inside_hconv_reevaluation_interval -ne $expectedInsideHconvReevaluationInterval) {
    throw "Expected inside_hconv_reevaluation_interval $expectedInsideHconvReevaluationInterval, got $($summary.inside_hconv_reevaluation_interval)"
}
if ($summary.ctf_initial_history_policy -ne $CtfInitialHistoryPolicy) {
    throw "Expected ctf_initial_history_policy $CtfInitialHistoryPolicy, got $($summary.ctf_initial_history_policy)"
}
if ($summary.zone_conduction_report_source -ne $ZoneConductionReportSource) {
    throw "Expected zone_conduction_report_source $ZoneConductionReportSource, got $($summary.zone_conduction_report_source)"
}
if ($summary.zone_air_report_sampling -ne $ZoneAirReportSampling) {
    throw "Expected zone_air_report_sampling $ZoneAirReportSampling, got $($summary.zone_air_report_sampling)"
}
if ($summary.surface_loop_zone_air_correction -ne $SurfaceLoopZoneAirCorrection) {
    throw "Expected surface_loop_zone_air_correction $SurfaceLoopZoneAirCorrection, got $($summary.surface_loop_zone_air_correction)"
}
$floorCtfSummary = $summary.ctf_seed.construction_summaries | Where-Object { $_.construction_name -eq "FLOOR" } | Select-Object -First 1
if ($null -eq $floorCtfSummary) {
    throw "Expected CTF construction summaries to include FLOOR"
}
if ($floorCtfSummary.ctf_count -ne 5) {
    throw "Expected FLOOR #CTFs=5 in CTF construction summaries, got $($floorCtfSummary.ctf_count)"
}
if ([Math]::Abs([double]$floorCtfSummary.timestep_hours - 0.25) -gt 1.0e-9) {
    throw "Expected FLOOR CTF timestep 0.25h, got $($floorCtfSummary.timestep_hours)"
}
if ($CtfSeedPolicy -eq "steady-no-mass-only") {
    if (-not ($summary.ctf_seed.skipped_constructions | Where-Object { $_.construction_name -eq "FLOOR" -and $_.ctf_count -eq 5 })) {
        throw "Expected steady/no-mass policy to skip FLOOR #CTFs=5"
    }
    if ($floorCtfSummary.included) {
        throw "Expected steady/no-mass policy to mark FLOOR CTF summary as skipped"
    }
}
else {
    if (-not ($summary.ctf_seed.included_constructions -contains "FLOOR")) {
        throw "Expected all-eio policy to include FLOOR"
    }
    if ($summary.ctf_seed.skipped_constructions.Count -ne 0) {
        throw "Expected all-eio policy to skip no constructions"
    }
    if (-not $floorCtfSummary.included) {
        throw "Expected all-eio policy to mark FLOOR CTF summary as included"
    }
}

if ($isCompatibilityCandidateCase) {
    if ($summary.series_count -ne 30) {
        throw "Unexpected candidate series_count: $($summary.series_count)"
    }
    if ($summary.zone_air_algorithm_lane -ne "compatibility-source-order") {
        throw "Candidate case must run the compatibility-source-order lane, got $($summary.zone_air_algorithm_lane)"
    }
    if ($summary.compatibility_source_order -ne $true) {
        throw "Candidate case must mark compatibility_source_order=true"
    }
    if ($summary.diagnostic_probe_used -ne $false) {
        throw "Candidate case must mark diagnostic_probe_used=false"
    }
    if ($summary.conformance_promotion_allowed -ne $true) {
        throw "Candidate case must mark conformance_promotion_allowed=true"
    }
    if ($summary.active_blocker_summary -notmatch "ZN001:FLR001") {
        throw "Expected candidate active blocker summary to reference ZN001:FLR001, got $($summary.active_blocker_summary)"
    }
    if ($summary.next_pr_target -ne "outside-ctf-history-handoff") {
        throw "Expected candidate next_pr_target to focus outside CTF history handoff, got $($summary.next_pr_target)"
    }
    if ($null -eq $summary.top_blocker) {
        throw "Expected candidate summary to include a single top_blocker object"
    }
    if ([int]$summary.top_blocker.rank -ne 1) {
        throw "Expected candidate top_blocker rank to be 1, got $($summary.top_blocker.rank)"
    }
    if ($summary.top_blocker.blocker_id -ne "floor-storage-mismatch") {
        throw "Expected candidate top_blocker to be floor-storage-mismatch, got $($summary.top_blocker.blocker_id)"
    }
    if ([string]::IsNullOrWhiteSpace($summary.top_blocking_mismatch)) {
        throw "Expected candidate top_blocking_mismatch to classify the active mismatch"
    }
    if ($summary.active_lane -ne "compatibility-source-order") {
        throw "Expected candidate active_lane compatibility-source-order, got $($summary.active_lane)"
    }
    if ($summary.best_diagnostic_lane -ne "energyplus-heat-balance-compat-candidate") {
        throw "Expected candidate best_diagnostic_lane energyplus-heat-balance-compat-candidate, got $($summary.best_diagnostic_lane)"
    }
    $candidateCurrentBlockers = @($summary.current_blockers)
    if ($candidateCurrentBlockers.Count -lt 8) {
        throw "Expected candidate current_blockers to include detailed blocker rows, got $($candidateCurrentBlockers.Count)"
    }
    foreach ($requiredBlocker in @(
            "floor-storage-mismatch",
            "floor-face-temperature-current-inside-mismatch",
            "ctf-current-term-delta",
            "ctf-history-temperature-term-delta",
            "ctf-history-flux-term-delta",
            "longwave-radiation-source-delta",
            "hconv-source-timing-delta",
            "warmup-end-state-mat-delta",
            "warmup-end-state-surface-temperature-delta",
            "warmup-end-state-ctf-history-delta",
            "warmup-end-state-zone-history-delta"
        )) {
        if (-not ($candidateCurrentBlockers | Where-Object { $_.blocker_id -eq $requiredBlocker })) {
            throw "Expected candidate current_blockers to include $requiredBlocker"
        }
    }
    if ($null -eq $summary.warmup_end_state_deltas) {
        throw "Expected candidate summary to include warmup_end_state_deltas"
    }
    if ($null -eq $summary.warmup_end_state_deltas.surface_temperature) {
        throw "Expected candidate warmup_end_state_deltas to include surface_temperature"
    }
    if ($null -eq $summary.warmup_end_state_deltas.ctf_history) {
        throw "Expected candidate warmup_end_state_deltas to include ctf_history"
    }
    if ($null -eq $summary.warmup_end_state_deltas.zone_history) {
        throw "Expected candidate warmup_end_state_deltas to include zone_history"
    }
    if (@($summary.first_divergence_by_variable).Count -lt 1) {
        throw "Expected candidate first_divergence_by_variable rows"
    }
    foreach ($requiredVariable in @(
            "Site Outdoor Air Drybulb Temperature",
            "Zone Mean Air Temperature",
            "Zone Air Heat Balance Internal Convective Heat Gain Rate",
            "Zone Air Heat Balance Surface Convection Rate",
            "Zone Air Heat Balance Air Energy Storage Rate",
            "Surface Inside Face Temperature",
            "Surface Outside Face Temperature",
            "Surface Inside Face Conduction Heat Transfer Rate",
            "Surface Outside Face Conduction Heat Transfer Rate",
            "Surface Heat Storage Rate"
        )) {
        if (-not ($summary.series | Where-Object { $_.output.variable -eq $requiredVariable -and $_.status -eq "extracted" })) {
            throw "Missing extracted candidate series: $requiredVariable"
        }
    }
    $floorStorageMaxSampleDelta = @($summary.ctf_storage_max_sample_deltas | Where-Object { $_.key -eq "ZN001:FLR001" })[0]
    if ($null -eq $floorStorageMaxSampleDelta) {
        throw "Expected candidate ctf_storage_max_sample_deltas to include ZN001:FLR001"
    }
    if ($floorStorageMaxSampleDelta.dominant_mismatch_source -ne "outside-history-total") {
        throw "Expected candidate dominant mismatch source outside-history-total, got $($floorStorageMaxSampleDelta.dominant_mismatch_source)"
    }

    $reportText = Get-Content -LiteralPath $reportPath -Raw
    Assert-Contains -Text $reportText -Pattern "zone_air_algorithm_lane: compatibility-source-order" -Description "candidate markdown algorithm lane"
    Assert-Contains -Text $reportText -Pattern "compatibility_source_order: true" -Description "candidate markdown source-order compatibility flag"
    Assert-Contains -Text $reportText -Pattern "diagnostic_probe_used: false" -Description "candidate markdown diagnostic probe flag"
    Assert-Contains -Text $reportText -Pattern "conformance_promotion_allowed: true" -Description "candidate markdown promotion flag"
    Assert-Contains -Text $reportText -Pattern "## Bottleneck Tracker" -Description "candidate compact bottleneck tracker"
    Assert-Contains -Text $reportText -Pattern "top_blocking_mismatch:" -Description "candidate top blocking mismatch"
    Assert-Contains -Text $reportText -Pattern "active_lane: compatibility-source-order" -Description "candidate active lane"
    Assert-Contains -Text $reportText -Pattern "best_diagnostic_lane: energyplus-heat-balance-compat-candidate" -Description "candidate best diagnostic lane"
    Assert-Contains -Text $reportText -Pattern "## Top Blocker" -Description "candidate top blocker section"
    Assert-Contains -Text $reportText -Pattern "floor-storage-mismatch" -Description "candidate floor storage blocker row"
    Assert-Contains -Text $reportText -Pattern "## Surface Family RMSE" -Description "candidate surface family RMSE table"
    Assert-Contains -Text $reportText -Pattern "| floor |" -Description "candidate floor RMSE row"
    Assert-Contains -Text $reportText -Pattern "| roof |" -Description "candidate roof RMSE row"
    Assert-Contains -Text $reportText -Pattern "| wall |" -Description "candidate wall RMSE row"
    Assert-Contains -Text $reportText -Pattern "## Warmup End-State Deltas" -Description "candidate warmup end-state delta section"
    Assert-Contains -Text $reportText -Pattern "warmup-end-state-mat-delta" -Description "candidate warmup MAT blocker row"
    Assert-Contains -Text $reportText -Pattern "warmup-end-state-surface-temperature-delta" -Description "candidate warmup surface temperature blocker row"
    Assert-Contains -Text $reportText -Pattern "warmup-end-state-ctf-history-delta" -Description "candidate warmup CTF history blocker row"
    Assert-Contains -Text $reportText -Pattern "warmup-end-state-zone-history-delta" -Description "candidate warmup zone history blocker row"
Assert-Contains -Text $reportText -Pattern "## First Divergence by Variable" -Description "candidate first divergence by variable section"
Assert-Contains -Text $reportText -Pattern "first_divergence_rows: top-" -Description "candidate compact first divergence rows"
    Assert-Contains -Text $reportText -Pattern "## Diagnostic Evidence" -Description "candidate compact diagnostic evidence section"
    Assert-Contains -Text $reportText -Pattern "next_pr_target: outside-ctf-history-handoff" -Description "candidate next PR target"
    Assert-NotContains -Text $reportText -Pattern "## Current Blockers" -Description "candidate compact report current blocker appendix"
    Assert-NotContains -Text $reportText -Pattern "## Bottlenecks" -Description "candidate compact report bottleneck appendix"
    Assert-NotContains -Text $reportText -Pattern "## Max-Sample Contexts" -Description "candidate compact report max-sample appendix"
    Assert-Contains -Text $reportText -Pattern "status: fail" -Description "candidate diagnostic status"

    Write-Host "Official dynamic heat-balance compatibility candidate passed structural checks."
    return
}

$ExpectedSeriesCount = 119
if ($summary.series_count -ne $ExpectedSeriesCount) {
    throw "Unexpected series_count: $($summary.series_count)"
}
if ($summary.max_abs_delta_c -le 1.0) {
    throw "Expected current official dynamic diagnostic delta to remain visible above 1.0, got $($summary.max_abs_delta_c)"
}
$topBottleneck = @($summary.bottlenecks)[0]
if ($null -eq $topBottleneck) {
    throw "Expected at least one bottleneck row in heat-balance diagnostic summary"
}
if ($null -eq $topBottleneck.first_delta_sample) {
    throw "Expected top bottleneck to include a first_delta_sample fingerprint"
}
if ($null -eq $topBottleneck.max_delta_sample) {
    throw "Expected top bottleneck to include a max_delta_sample fingerprint"
}
if ($null -eq $summary.top_blocker) {
    throw "Expected summary to include a single top_blocker object"
}
if ([int]$summary.top_blocker.rank -ne 1) {
    throw "Expected top_blocker rank to be 1, got $($summary.top_blocker.rank)"
}
if ([string]::IsNullOrWhiteSpace($summary.top_blocking_mismatch)) {
    throw "Expected summary to include top_blocking_mismatch"
}
if ($summary.active_lane -ne $expectedAlgorithmLane) {
    throw "Expected active_lane $expectedAlgorithmLane, got $($summary.active_lane)"
}
if ([string]::IsNullOrWhiteSpace($summary.best_diagnostic_lane)) {
    throw "Expected summary to include best_diagnostic_lane"
}
$currentBlockers = @($summary.current_blockers)
if ($currentBlockers.Count -lt 8) {
    throw "Expected current_blockers to include detailed blocker rows, got $($currentBlockers.Count)"
}
foreach ($requiredBlocker in @(
        "ctf-current-term-delta",
        "ctf-history-temperature-term-delta",
        "ctf-history-flux-term-delta",
        "longwave-radiation-source-delta",
        "hconv-source-timing-delta",
        "warmup-end-state-mat-delta",
        "warmup-end-state-surface-temperature-delta",
        "warmup-end-state-ctf-history-delta",
        "warmup-end-state-zone-history-delta"
    )) {
    if (-not ($currentBlockers | Where-Object { $_.blocker_id -eq $requiredBlocker })) {
        throw "Expected current_blockers to include $requiredBlocker"
    }
}
if ($null -eq $summary.warmup_end_state_deltas) {
    throw "Expected summary to include warmup_end_state_deltas"
}
if ($null -eq $summary.warmup_end_state_deltas.surface_temperature) {
    throw "Expected warmup_end_state_deltas to include surface_temperature"
}
if ($null -eq $summary.warmup_end_state_deltas.ctf_history) {
    throw "Expected warmup_end_state_deltas to include ctf_history"
}
if ($null -eq $summary.warmup_end_state_deltas.zone_history) {
    throw "Expected warmup_end_state_deltas to include zone_history"
}
$firstDivergenceByVariable = @($summary.first_divergence_by_variable)
if ($firstDivergenceByVariable.Count -lt 1) {
    throw "Expected first_divergence_by_variable rows"
}
if ($null -eq (@($firstDivergenceByVariable | Where-Object { $_.variable -eq $topBottleneck.output.variable -and $_.key -eq $topBottleneck.output.key })[0])) {
    throw "Expected first_divergence_by_variable to include the top bottleneck variable"
}
$topMaxSampleContext = @($summary.max_sample_contexts)[0]
if ($null -eq $topMaxSampleContext) {
    throw "Expected at least one max-sample context row in heat-balance diagnostic summary"
}
if ($topMaxSampleContext.sample_index -ne $topBottleneck.max_delta_sample.index) {
    throw "Expected first max-sample context to use top bottleneck sample index $($topBottleneck.max_delta_sample.index), got $($topMaxSampleContext.sample_index)"
}
if (@($topMaxSampleContext.rows).Count -lt 1) {
    throw "Expected first max-sample context to include related output rows"
}
$topFirstSampleBottleneck = @($summary.first_sample_bottlenecks)[0]
if ($null -eq $topFirstSampleBottleneck) {
    throw "Expected at least one first-sample bottleneck row in heat-balance diagnostic summary"
}
if ($null -eq $topFirstSampleBottleneck.first_sample_delta) {
    throw "Expected first-sample bottleneck to include a first_sample_delta fingerprint"
}
if ([int]$topFirstSampleBottleneck.first_sample_delta.index -ne 0) {
    throw "Expected first-sample bottleneck index 0, got $($topFirstSampleBottleneck.first_sample_delta.index)"
}
$zoneAirFirstSampleTrace = @($summary.zone_air_first_sample_trace)
if ($zoneAirFirstSampleTrace.Count -lt 4) {
    throw "Expected zone_air_first_sample_trace to include first-hour per-zone timestep rows, got $($zoneAirFirstSampleTrace.Count)"
}
$zoneAirWarmupDayEndStates = @($summary.zone_air_warmup_day_end_states)
$expectedWarmupDayEndRows = [Math]::Max([int]$summary.heat_balance_warmup.day_count, $WarmupMinimumDays)
if ($zoneAirWarmupDayEndStates.Count -lt $expectedWarmupDayEndRows) {
    throw "Expected zone_air_warmup_day_end_states to include at least $expectedWarmupDayEndRows warmup day-end rows, got $($zoneAirWarmupDayEndStates.Count)"
}
$zoneAirFirstTrace = @($zoneAirFirstSampleTrace | Where-Object { $_.key -eq "ZONE ONE" -and [int]$_.timestep_index -eq 1 })[0]
if ($null -eq $zoneAirFirstTrace) {
    throw "Expected zone_air_first_sample_trace to include ZONE ONE timestep 1"
}
if ($null -eq $zoneAirFirstTrace.zone_air_temperature_coefficients) {
    throw "Expected zone_air_first_sample_trace rows to include zone_air_temperature_coefficients"
}
if ($null -eq $zoneAirFirstTrace.third_order_solution_temperature_c) {
    throw "Expected zone_air_first_sample_trace rows to include third_order_solution_temperature_c"
}
$surfaceFirstSampleTrace = @($summary.surface_first_sample_trace)
if ($surfaceFirstSampleTrace.Count -lt 6) {
    throw "Expected surface_first_sample_trace to include first-hour per-surface rows, got $($surfaceFirstSampleTrace.Count)"
}
$floorSurfaceFirstTrace = @($surfaceFirstSampleTrace | Where-Object { $_.key -eq "ZN001:FLR001" -and [int]$_.timestep_index -eq 1 })[0]
if ($null -eq $floorSurfaceFirstTrace) {
    throw "Expected surface_first_sample_trace to include ZN001:FLR001 timestep 1"
}
if ($null -eq $floorSurfaceFirstTrace.outdoor_dry_bulb_c) {
    throw "Expected surface_first_sample_trace rows to include outdoor_dry_bulb_c"
}
if ([Math]::Abs([double]$floorSurfaceFirstTrace.outdoor_dry_bulb_c - -6.0) -gt 1.0e-9) {
    throw "Expected first-hour weather interpolation to seed from run-period day Hour24, got outdoor_dry_bulb_c $($floorSurfaceFirstTrace.outdoor_dry_bulb_c)"
}
if ($null -eq $floorSurfaceFirstTrace.outside_face_temperature_c) {
    throw "Expected surface_first_sample_trace rows to include outside_face_temperature_c"
}
$zoneAirDebug = (Get-Content -LiteralPath $zoneAirDebugPath -Raw -Encoding UTF8) | ConvertFrom-Json
$zoneAirDebugFirstSampleTrace = @($zoneAirDebug.zone_air_first_sample_trace)
if ($zoneAirDebugFirstSampleTrace.Count -lt 4) {
    throw "Expected rust-zone-air-diagnostics.json to include first-hour zone-air timestep rows, got $($zoneAirDebugFirstSampleTrace.Count)"
}
$zoneAirDebugWarmupDayEndStates = @($zoneAirDebug.warmup_day_end_states)
if ($zoneAirDebugWarmupDayEndStates.Count -lt $expectedWarmupDayEndRows) {
    throw "Expected rust-zone-air-diagnostics.json to include at least $expectedWarmupDayEndRows warmup day-end rows, got $($zoneAirDebugWarmupDayEndStates.Count)"
}
$floorCtfComponent = @($summary.ctf_component_first_samples | Where-Object { $_.key -eq "ZN001:FLR001" })[0]
if ($null -eq $floorCtfComponent) {
    throw "Expected ctf_component_first_samples to include ZN001:FLR001"
}
$insideComponentSum = [double]$floorCtfComponent.inside_current_outside_term_w +
    [double]$floorCtfComponent.inside_current_inside_term_w +
    [double]$floorCtfComponent.inside_history_term_w
if ([Math]::Abs($insideComponentSum - [double]$floorCtfComponent.inside_conduction_rate_w) -gt 1.0e-6) {
    throw "Expected FLOOR inside CTF component sum to match inside conduction rate"
}
$outsideComponentSum = [double]$floorCtfComponent.outside_current_outside_term_w +
    [double]$floorCtfComponent.outside_current_inside_term_w +
    [double]$floorCtfComponent.outside_history_term_w
if ([Math]::Abs($outsideComponentSum - [double]$floorCtfComponent.outside_conduction_rate_w) -gt 1.0e-6) {
    throw "Expected FLOOR outside CTF component sum to match outside conduction rate"
}
$storageFromConduction = -([double]$floorCtfComponent.inside_conduction_rate_w + [double]$floorCtfComponent.outside_conduction_rate_w)
if ([Math]::Abs($storageFromConduction - [double]$floorCtfComponent.heat_storage_rate_w) -gt 1.0e-6) {
    throw "Expected FLOOR storage to match the negated inside/outside conduction sum"
}
$zoneAirCoefficientDelta = @($summary.zone_air_coefficient_deltas | Where-Object { $_.key -eq "ZONE ONE" })[0]
if ($null -eq $zoneAirCoefficientDelta) {
    throw "Expected zone_air_coefficient_deltas to include ZONE ONE"
}
if ([int]$zoneAirCoefficientDelta.samples -ne 8760) {
    throw "Expected ZONE ONE zone-air coefficient deltas to use 8760 samples, got $($zoneAirCoefficientDelta.samples)"
}
foreach ($propertyName in @(
        "first_divergence_source",
        "first_divergence_sample_index",
        "first_divergence_delta",
        "sum_ha_delta",
        "sum_hat_surf_delta",
        "sum_hat_ref_delta",
        "temp_dependent_coefficient_delta",
        "temp_independent_coefficient_delta",
        "air_power_cap_delta",
        "temp_history_term_delta",
        "mat_delta",
        "air_storage_delta",
        "surface_convection_delta"
    )) {
    if ($zoneAirCoefficientDelta.PSObject.Properties.Name -notcontains $propertyName) {
        throw "Expected ZONE ONE zone-air coefficient row to include $propertyName"
    }
}
if ($zoneAirCoefficientDelta.first_divergence_source -eq "none") {
    throw "Expected active zone-air coefficient row to retain a visible first divergence source"
}
if ([double]$zoneAirCoefficientDelta.temp_dependent_coefficient_delta.rmse_delta_c -lt 0.0) {
    throw "Expected TempDepCoef RMSE to be numeric"
}
if ([double]$zoneAirCoefficientDelta.temp_history_term_delta.rmse_delta_c -lt 0.0) {
    throw "Expected TempHistoryTerm RMSE to be numeric"
}
$zoneAirSurfaceConvectionClosureDelta = @($summary.zone_air_surface_convection_closure_deltas | Where-Object { $_.key -eq "ZONE ONE" })[0]
if ($null -eq $zoneAirSurfaceConvectionClosureDelta) {
    throw "Expected zone_air_surface_convection_closure_deltas to include ZONE ONE"
}
if ([int]$zoneAirSurfaceConvectionClosureDelta.samples -ne 8760) {
    throw "Expected ZONE ONE surface-convection closure row to use 8760 samples, got $($zoneAirSurfaceConvectionClosureDelta.samples)"
}
if ([int]$zoneAirSurfaceConvectionClosureDelta.surface_count -lt 6) {
    throw "Expected ZONE ONE surface-convection closure row to include six opaque surfaces, got $($zoneAirSurfaceConvectionClosureDelta.surface_count)"
}
foreach ($propertyName in @(
        "zone_surface_convection_delta",
        "surface_report_sum_delta",
        "oracle_closure_residual",
        "rust_closure_residual",
        "closure_residual_delta"
    )) {
    if ($zoneAirSurfaceConvectionClosureDelta.PSObject.Properties.Name -notcontains $propertyName) {
        throw "Expected ZONE ONE surface-convection closure row to include $propertyName"
    }
}
if ([double]$zoneAirSurfaceConvectionClosureDelta.oracle_closure_residual.rmse_delta_c -le 0.0) {
    throw "Expected oracle surface-convection closure residual to stay visible, got $($zoneAirSurfaceConvectionClosureDelta.oracle_closure_residual.rmse_delta_c)"
}
if ([double]$zoneAirSurfaceConvectionClosureDelta.closure_residual_delta.rmse_delta_c -lt 0.0) {
    throw "Expected closure residual delta RMSE to be numeric"
}
$zoneAirSurfaceCoefficientDeltas = @($summary.zone_air_surface_coefficient_deltas)
if ($zoneAirSurfaceCoefficientDeltas.Count -lt 6) {
    throw "Expected zone_air_surface_coefficient_deltas to include the six opaque 1Zone surfaces, got $($zoneAirSurfaceCoefficientDeltas.Count)"
}
$floorZoneAirSurfaceCoefficientDelta = @($zoneAirSurfaceCoefficientDeltas | Where-Object { $_.key -eq "ZN001:FLR001" })[0]
if ($null -eq $floorZoneAirSurfaceCoefficientDelta) {
    throw "Expected zone_air_surface_coefficient_deltas to include ZN001:FLR001"
}
foreach ($propertyName in @(
        "zone_key",
        "area_m2",
        "sum_ha_delta",
        "sum_hat_surf_delta",
        "sum_hat_ref_delta",
        "reference_air_temperature_delta",
        "inside_face_temperature_delta",
        "inside_hconv_delta",
        "inside_convection_gain_delta"
    )) {
    if ($floorZoneAirSurfaceCoefficientDelta.PSObject.Properties.Name -notcontains $propertyName) {
        throw "Expected FLOOR zone-air surface coefficient row to include $propertyName"
    }
}
if ([int]$floorZoneAirSurfaceCoefficientDelta.samples -ne 8760) {
    throw "Expected FLOOR zone-air surface coefficient row to use 8760 samples, got $($floorZoneAirSurfaceCoefficientDelta.samples)"
}
if ([double]$floorZoneAirSurfaceCoefficientDelta.area_m2 -le 0.0) {
    throw "Expected FLOOR zone-air surface coefficient row to include positive area"
}
if ($CtfSeedPolicy -eq "all-eio") {
    $floorHistoryDelta = @($summary.ctf_history_first_sample_deltas | Where-Object { $_.key -eq "ZN001:FLR001" })[0]
    if ($null -eq $floorHistoryDelta) {
        throw "Expected ctf_history_first_sample_deltas to include ZN001:FLR001 in all-eio mode"
    }
    if ([double]$floorHistoryDelta.inside_history_delta_w -lt 0.0) {
        throw "Expected active FLOOR inside history delta to be numeric and non-negative, got $($floorHistoryDelta.inside_history_delta_w)"
    }
    if ([double]$floorHistoryDelta.outside_history_delta_w -lt 0.0) {
        throw "Expected active FLOOR outside history delta to be numeric and non-negative, got $($floorHistoryDelta.outside_history_delta_w)"
    }
    $floorHistorySeriesDelta = @($summary.ctf_history_series_deltas | Where-Object { $_.key -eq "ZN001:FLR001" })[0]
    if ($null -eq $floorHistorySeriesDelta) {
        throw "Expected ctf_history_series_deltas to include ZN001:FLR001 in all-eio mode"
    }
    if ([int]$floorHistorySeriesDelta.samples -ne 8760) {
        throw "Expected FLOOR CTF history series deltas to use 8760 samples, got $($floorHistorySeriesDelta.samples)"
    }
    foreach ($propertyName in @(
            "inside_current_outside_term_delta",
            "inside_current_inside_term_delta",
            "inside_history_temperature_term_delta",
            "inside_history_flux_term_delta",
            "outside_current_outside_term_delta",
            "outside_current_inside_term_delta"
        )) {
        if ($floorHistorySeriesDelta.PSObject.Properties.Name -notcontains $propertyName) {
            throw "Expected FLOOR CTF history series row to include $propertyName"
        }
    }
    if ([double]$floorHistorySeriesDelta.inside_current_inside_term_delta.rmse_delta_c -le 8.0) {
        throw "Expected active FLOOR inside current-inside series delta to remain visible above 8 W RMSE, got $($floorHistorySeriesDelta.inside_current_inside_term_delta.rmse_delta_c)"
    }
    if ([double]$floorHistorySeriesDelta.inside_history_delta.rmse_delta_c -le 8.0) {
        throw "Expected active FLOOR inside history series delta to remain visible above 8 W RMSE, got $($floorHistorySeriesDelta.inside_history_delta.rmse_delta_c)"
    }
    if ([double]$floorHistorySeriesDelta.outside_history_delta.rmse_delta_c -le 8.0) {
        throw "Expected active FLOOR outside history series delta to remain visible above 8 W RMSE, got $($floorHistorySeriesDelta.outside_history_delta.rmse_delta_c)"
    }
    $floorStorageMaxSampleDelta = @($summary.ctf_storage_max_sample_deltas | Where-Object { $_.key -eq "ZN001:FLR001" })[0]
    if ($null -eq $floorStorageMaxSampleDelta) {
        throw "Expected ctf_storage_max_sample_deltas to include ZN001:FLR001 in all-eio mode"
    }
    if ([int]$floorStorageMaxSampleDelta.sample_index -lt 0) {
        throw "Expected floor storage max-sample CTF delta to include a non-negative sample index"
    }
    if ([double]$floorStorageMaxSampleDelta.storage_delta_w -le 0.0) {
        throw "Expected floor storage max-sample CTF delta to retain visible storage_delta_w"
    }
    foreach ($propertyName in @(
            "storage_delta_rank",
            "dominant_storage_surface",
            "dominant_mismatch_source",
            "dominant_mismatch_delta_w",
            "storage_balance_residual_delta_w",
            "inside_face_temperature_delta_c",
            "inside_current_outside_term_signed_delta_w",
            "inside_current_inside_term_signed_delta_w",
            "inside_current_split_abs_sum_w",
            "rust_inside_history_temperature_term_w",
            "rust_inside_history_flux_term_w"
        )) {
        if ($floorStorageMaxSampleDelta.PSObject.Properties.Name -notcontains $propertyName) {
            throw "Expected FLOOR storage max-sample row to include $propertyName"
        }
    }
    if ([int]$floorStorageMaxSampleDelta.storage_delta_rank -ne 1) {
        throw "Expected FLOOR to be the dominant storage max-sample CTF row, got rank $($floorStorageMaxSampleDelta.storage_delta_rank)"
    }
    if (-not [bool]$floorStorageMaxSampleDelta.dominant_storage_surface) {
        throw "Expected FLOOR storage max-sample row to be marked dominant"
    }
    $expectedFloorStorageNextTarget = switch ($floorStorageMaxSampleDelta.dominant_mismatch_source) {
        "face-temperature-current-inside" { "floor-inside-current-face-temperature-source-timing" }
        "face-temperature-current-outside" { "floor-outside-current-face-temperature-source-timing" }
        "history-vector-inside-total" { "warmup-ctf-inside-history-handoff" }
        "outside-current-total" { "outside-current-boundary-source-timing" }
        "outside-history-total" { "outside-ctf-history-handoff" }
        "output-aggregation-storage-balance" { "storage-output-aggregation-and-sign-convention" }
        default { $null }
    }
    if ($null -eq $expectedFloorStorageNextTarget) {
        throw "Unexpected FLOOR storage max-sample dominant mismatch source: $($floorStorageMaxSampleDelta.dominant_mismatch_source)"
    }
    $floorStorageBlocker = @($summary.current_blockers | Where-Object { $_.blocker_id -eq "floor-storage-mismatch" -and $_.key -eq "ZN001:FLR001" })[0]
    if ($null -eq $floorStorageBlocker) {
        throw "Expected current_blockers to include active FLOOR floor-storage-mismatch"
    }
    if ($floorStorageBlocker.next_target -ne $expectedFloorStorageNextTarget) {
        throw "Expected FLOOR storage next target $expectedFloorStorageNextTarget for dominant mismatch source $($floorStorageMaxSampleDelta.dominant_mismatch_source), got $($floorStorageBlocker.next_target)"
    }
    $floorInsideBalanceMaxSampleDelta = @($summary.inside_balance_max_sample_deltas | Where-Object { $_.key -eq "ZN001:FLR001" })[0]
    if ($null -eq $floorInsideBalanceMaxSampleDelta) {
        throw "Expected inside_balance_max_sample_deltas to include ZN001:FLR001 in all-eio mode"
    }
    if ([int]$floorInsideBalanceMaxSampleDelta.sample_index -ne [int]$floorStorageMaxSampleDelta.sample_index) {
        throw "Expected FLOOR inside-balance max-sample row to share storage sample index $($floorStorageMaxSampleDelta.sample_index), got $($floorInsideBalanceMaxSampleDelta.sample_index)"
    }
    if ($null -eq $floorInsideBalanceMaxSampleDelta.inside_balance_residual_delta_w) {
        throw "Expected FLOOR inside-balance max-sample row to include inside_balance_residual_delta_w"
    }
    $floorInsideSolveMaxSampleDelta = @($summary.inside_solve_max_sample_deltas | Where-Object { $_.key -eq "ZN001:FLR001" })[0]
    if ($null -eq $floorInsideSolveMaxSampleDelta) {
        throw "Expected inside_solve_max_sample_deltas to include ZN001:FLR001 in all-eio mode"
    }
    if ([int]$floorInsideSolveMaxSampleDelta.sample_index -ne [int]$floorStorageMaxSampleDelta.sample_index) {
        throw "Expected FLOOR inside-solve max-sample row to share storage sample index $($floorStorageMaxSampleDelta.sample_index), got $($floorInsideSolveMaxSampleDelta.sample_index)"
    }
    if ($null -eq $floorInsideSolveMaxSampleDelta.implied_solve_numerator_delta_w) {
        throw "Expected FLOOR inside-solve max-sample row to include implied_solve_numerator_delta_w"
    }
    if ($null -eq $floorInsideSolveMaxSampleDelta.inferred_reference_air_temperature_delta_c) {
        throw "Expected FLOOR inside-solve max-sample row to include inferred_reference_air_temperature_delta_c"
    }
    if ($null -eq $floorInsideSolveMaxSampleDelta.rust_inside_history_temperature_term_w) {
        throw "Expected FLOOR inside-solve max-sample row to include rust_inside_history_temperature_term_w"
    }
    if ($null -eq $floorInsideSolveMaxSampleDelta.rust_inside_history_flux_term_w) {
        throw "Expected FLOOR inside-solve max-sample row to include rust_inside_history_flux_term_w"
    }
    if ($null -eq $floorInsideSolveMaxSampleDelta.tracked_solve_source_delta_w) {
        throw "Expected FLOOR inside-solve max-sample row to include tracked_solve_source_delta_w"
    }
    if ($null -eq $floorInsideSolveMaxSampleDelta.solve_source_residual_delta_w) {
        throw "Expected FLOOR inside-solve max-sample row to include solve_source_residual_delta_w"
    }
    if ($null -eq $floorInsideSolveMaxSampleDelta.tracked_solve_source_coverage_ratio) {
        throw "Expected FLOOR inside-solve max-sample row to include tracked_solve_source_coverage_ratio"
    }
    if ($null -eq $floorInsideSolveMaxSampleDelta.reference_air_coefficient_source_delta_w) {
        throw "Expected FLOOR inside-solve max-sample row to include reference_air_coefficient_source_delta_w"
    }
    if ($null -eq $floorInsideSolveMaxSampleDelta.reference_air_temperature_source_delta_w) {
        throw "Expected FLOOR inside-solve max-sample row to include reference_air_temperature_source_delta_w"
    }
    if ($null -eq $floorInsideSolveMaxSampleDelta.reference_air_coefficient_source_share) {
        throw "Expected FLOOR inside-solve max-sample row to include reference_air_coefficient_source_share"
    }
    if ($null -eq $floorInsideSolveMaxSampleDelta.reference_air_temperature_source_share) {
        throw "Expected FLOOR inside-solve max-sample row to include reference_air_temperature_source_share"
    }
    if ($null -eq $floorInsideSolveMaxSampleDelta.reference_air_source_signed_delta_w) {
        throw "Expected FLOOR inside-solve max-sample row to include reference_air_source_signed_delta_w"
    }
    if ($null -eq $floorInsideSolveMaxSampleDelta.reference_air_source_split_abs_sum_w) {
        throw "Expected FLOOR inside-solve max-sample row to include reference_air_source_split_abs_sum_w"
    }
    if ($null -eq $floorInsideSolveMaxSampleDelta.reference_air_source_cancellation_delta_w) {
        throw "Expected FLOOR inside-solve max-sample row to include reference_air_source_cancellation_delta_w"
    }
    if ($null -eq $floorInsideSolveMaxSampleDelta.reference_air_coefficient_source_signed_delta_w) {
        throw "Expected FLOOR inside-solve max-sample row to include reference_air_coefficient_source_signed_delta_w"
    }
    if ($null -eq $floorInsideSolveMaxSampleDelta.reference_air_temperature_source_signed_delta_w) {
        throw "Expected FLOOR inside-solve max-sample row to include reference_air_temperature_source_signed_delta_w"
    }
    $referenceAirSignedSplitSum = [double]$floorInsideSolveMaxSampleDelta.reference_air_coefficient_source_signed_delta_w + [double]$floorInsideSolveMaxSampleDelta.reference_air_temperature_source_signed_delta_w
    if ([Math]::Abs($referenceAirSignedSplitSum - [double]$floorInsideSolveMaxSampleDelta.reference_air_source_signed_delta_w) -gt 1.0e-6) {
        throw "Expected FLOOR inside-solve signed reference-air split terms to reconstruct reference_air_source_signed_delta_w"
    }
    $referenceAirAbsSplitSum = [double]$floorInsideSolveMaxSampleDelta.reference_air_coefficient_source_delta_w + [double]$floorInsideSolveMaxSampleDelta.reference_air_temperature_source_delta_w
    if ([Math]::Abs($referenceAirAbsSplitSum - [double]$floorInsideSolveMaxSampleDelta.reference_air_source_split_abs_sum_w) -gt 1.0e-6) {
        throw "Expected FLOOR inside-solve absolute reference-air split terms to reconstruct reference_air_source_split_abs_sum_w"
    }
    $referenceAirCancellation = [double]$floorInsideSolveMaxSampleDelta.reference_air_source_split_abs_sum_w - [double]$floorInsideSolveMaxSampleDelta.reference_air_source_delta_w
    if ([Math]::Abs($referenceAirCancellation - [double]$floorInsideSolveMaxSampleDelta.reference_air_source_cancellation_delta_w) -gt 1.0e-6) {
        throw "Expected FLOOR inside-solve reference-air cancellation delta to match abs split sum minus absolute source delta"
    }
    $rustInsideHistorySplitSum = [double]$floorInsideSolveMaxSampleDelta.rust_inside_history_temperature_term_w + [double]$floorInsideSolveMaxSampleDelta.rust_inside_history_flux_term_w
    if ([Math]::Abs($rustInsideHistorySplitSum - [double]$floorInsideSolveMaxSampleDelta.rust_inside_history_term_w) -gt 1.0e-6) {
        throw "Expected FLOOR inside-solve Rust history split terms to sum to rust_inside_history_term_w"
    }
    $floorInsideSolveSeriesDelta = @($summary.inside_solve_series_deltas | Where-Object { $_.key -eq "ZN001:FLR001" })[0]
    if ($null -eq $floorInsideSolveSeriesDelta) {
        throw "Expected inside_solve_series_deltas to include ZN001:FLR001 in all-eio mode"
    }
    if ([int]$floorInsideSolveSeriesDelta.samples -ne 8760) {
        throw "Expected FLOOR inside-solve series deltas to use 8760 samples, got $($floorInsideSolveSeriesDelta.samples)"
    }
    foreach ($propertyName in @(
            "inside_face_temperature_delta",
            "implied_solve_numerator_delta",
            "reference_air_source_delta",
            "reference_air_coefficient_source_delta",
            "reference_air_temperature_source_delta",
            "outside_temperature_source_delta",
            "inside_history_delta",
            "inside_net_longwave_delta",
            "tracked_solve_source_delta",
            "solve_source_residual_delta"
        )) {
        if ($floorInsideSolveSeriesDelta.PSObject.Properties.Name -notcontains $propertyName) {
            throw "Expected FLOOR inside-solve series row to include $propertyName"
        }
    }
    if ([double]$floorInsideSolveSeriesDelta.implied_solve_numerator_delta.rmse_delta_c -le 0.0) {
        throw "Expected active FLOOR inside-solve implied numerator series delta to remain visible"
    }
    $floorAdiabaticHistoryMaxSampleDelta = @($summary.adiabatic_history_max_sample_deltas | Where-Object { $_.key -eq "ZN001:FLR001" })[0]
    if ($null -eq $floorAdiabaticHistoryMaxSampleDelta) {
        throw "Expected adiabatic_history_max_sample_deltas to include ZN001:FLR001 in all-eio mode"
    }
    if ([int]$floorAdiabaticHistoryMaxSampleDelta.sample_index -ne [int]$floorStorageMaxSampleDelta.sample_index) {
        throw "Expected FLOOR adiabatic-history max-sample row to share storage sample index $($floorStorageMaxSampleDelta.sample_index), got $($floorAdiabaticHistoryMaxSampleDelta.sample_index)"
    }
    if ($null -eq $floorAdiabaticHistoryMaxSampleDelta.outside_minus_inside_delta_c) {
        throw "Expected FLOOR adiabatic-history max-sample row to include outside_minus_inside_delta_c"
    }
    if ($null -eq $floorAdiabaticHistoryMaxSampleDelta.inside_current_if_outside_synced_delta_w) {
        throw "Expected FLOOR adiabatic-history max-sample row to include inside_current_if_outside_synced_delta_w"
    }
    $floorRunPeriodInitialSlots = @($summary.ctf_history_run_period_initial_slots | Where-Object { $_.key -eq "ZN001:FLR001" })
    if ($floorRunPeriodInitialSlots.Count -lt 5) {
        throw "Expected ctf_history_run_period_initial_slots to include FLOOR #CTFs=5 rows, got $($floorRunPeriodInitialSlots.Count)"
    }
    $floorHistorySlots = @($summary.ctf_history_first_sample_slots | Where-Object { $_.key -eq "ZN001:FLR001" })
    if ($floorHistorySlots.Count -lt 5) {
        throw "Expected ctf_history_first_sample_slots to include FLOOR #CTFs=5 rows, got $($floorHistorySlots.Count)"
    }
    $floorMaxSampleHistorySlots = @($summary.ctf_history_max_sample_slots | Where-Object { $_.key -eq "ZN001:FLR001" })
    if ($floorMaxSampleHistorySlots.Count -lt 5) {
        throw "Expected ctf_history_max_sample_slots to include FLOOR #CTFs=5 rows, got $($floorMaxSampleHistorySlots.Count)"
    }
    $floorMaxSampleHistorySlotsAfterAdvance = @($summary.ctf_history_max_sample_slots_after_advance | Where-Object { $_.key -eq "ZN001:FLR001" })
    if ($floorMaxSampleHistorySlotsAfterAdvance.Count -lt 5) {
        throw "Expected ctf_history_max_sample_slots_after_advance to include FLOOR #CTFs=5 rows, got $($floorMaxSampleHistorySlotsAfterAdvance.Count)"
    }
    foreach ($slot in $floorMaxSampleHistorySlots) {
        if ([int]$slot.sample_index -ne [int]$floorStorageMaxSampleDelta.sample_index) {
            throw "Expected FLOOR max-sample CTF history slot to share storage sample index $($floorStorageMaxSampleDelta.sample_index), got $($slot.sample_index)"
        }
    }
    foreach ($slot in $floorMaxSampleHistorySlotsAfterAdvance) {
        if ([int]$slot.sample_index -ne [int]$floorStorageMaxSampleDelta.sample_index) {
            throw "Expected FLOOR post-advance max-sample CTF history slot to share storage sample index $($floorStorageMaxSampleDelta.sample_index), got $($slot.sample_index)"
        }
    }
    $maxSampleInsideSlotSum = 0.0
    foreach ($slot in $floorMaxSampleHistorySlots) {
        $maxSampleInsideSlotSum += [double]$slot.inside_total_term_w
    }
    if ([Math]::Abs($maxSampleInsideSlotSum - [double]$floorInsideSolveMaxSampleDelta.rust_inside_history_term_w) -gt 1.0e-6) {
        throw "Expected FLOOR max-sample CTF slot sum to match Rust inside history term"
    }
    $insideSlotSum = 0.0
    $outsideSlotSum = 0.0
    foreach ($slot in $floorHistorySlots) {
        $insideSlotSum += [double]$slot.inside_total_term_w
        $outsideSlotSum += [double]$slot.outside_total_term_w
    }
    if ([Math]::Abs($insideSlotSum - [double]$floorCtfComponent.inside_history_term_w) -gt 1.0e-6) {
        throw "Expected FLOOR inside CTF slot sum to match aggregate history term"
    }
    if ([Math]::Abs($outsideSlotSum - [double]$floorCtfComponent.outside_history_term_w) -gt 1.0e-6) {
        throw "Expected FLOOR outside CTF slot sum to match aggregate history term"
    }
}
$expectedTopCandidates = @(
    @{
        Key = "ZN001:FLR001"
        Variable = "Surface Inside Face Net Surface Thermal Radiation Heat Gain Rate"
        Description = "floor inside net surface thermal radiation heat gain"
    },
    @{
        Key = "ZN001:FLR001"
        Variable = "Surface Inside Face Convection Heat Gain Rate"
        Description = "floor inside convection heat gain"
    },
    @{
        Key = "ZN001:FLR001"
        Variable = "Surface Heat Storage Rate"
        Description = "floor heat storage"
    },
    @{
        Key = "ZN001:FLR001"
        Variable = "Surface Inside Face Conduction Heat Transfer Rate"
        Description = "floor inside conduction"
    },
    @{
        Key = "ZN001:FLR001"
        Variable = "Surface Outside Face Conduction Heat Transfer Rate"
        Description = "floor outside conduction"
    },
    @{
        Key = "ZONE ONE"
        Variable = "Zone Opaque Surface Outside Faces Conduction Rate"
        Description = "zone outside opaque conduction aggregate"
    },
    @{
        Key = "Simulation"
        Variable = "Surface Inside Face Heat Balance Calculation Iteration Count"
        Description = "inside surface heat-balance iteration count"
    },
    @{
        Key = "ZN001:ROOF001"
        Variable = "Surface Inside Face Net Surface Thermal Radiation Heat Gain Rate"
        Description = "roof inside net surface thermal radiation heat gain"
    },
    @{
        Key = "ZN001:ROOF001"
        Variable = "Surface Inside Face Convection Heat Gain Rate"
        Description = "roof inside convection heat gain"
    },
    @{
        Key = "ZN001:ROOF001"
        Variable = "Surface Outside Face Solar Radiation Heat Gain Rate"
        Description = "roof outside solar heat gain"
    },
    @{
        Key = "ZN001:ROOF001"
        Variable = "Surface Outside Face Convection Heat Gain Rate"
        Description = "roof outside convection heat gain"
    },
    @{
        Key = "ZN001:ROOF001"
        Variable = "Surface Outside Face Net Thermal Radiation Heat Gain Rate"
        Description = "roof outside net thermal radiation heat gain"
    }
)
if (
    $ZoneAirAlgorithm -eq "energyplus-analytical-probe" -or
    $ZoneAirAlgorithm -eq "energyplus-third-order-probe" -or
    $ZoneAirAlgorithm -eq "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-previous-mat-surface-convection-probe" -or
    $ZoneAirAlgorithm -eq "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-inside-ctf-outside-history-scriptf-flat-surface-reference-air-report-probe"
) {
    $expectedTopCandidates += @{
        Key = "ZONE ONE"
        Variable = "Zone Air Heat Balance Surface Convection Rate"
        Description = "zone air heat-balance surface convection"
    }
}
if ($CtfSeedPolicy -eq "all-eio" -and $ZoneAirAlgorithm -eq "simplified-analytical") {
    $expectedTopCandidates += @{
        Key = "ZONE ONE"
        Variable = "Zone Air Heat Balance Air Energy Storage Rate"
        Description = "zone air heat-balance air energy storage"
    }
}
if ($ZoneAirReportSampling -eq "last-system-state") {
    $expectedTopCandidates += @{
        Key = "ZONE ONE"
        Variable = "Zone Air Heat Balance Air Energy Storage Rate"
        Description = "zone air heat-balance last-state air energy storage"
    }
}
foreach ($wallKey in @("ZN001:WALL001", "ZN001:WALL002", "ZN001:WALL003", "ZN001:WALL004")) {
    $expectedTopCandidates += @(
        @{
            Key = $wallKey
            Variable = "Surface Inside Face Net Surface Thermal Radiation Heat Gain Rate"
            Description = "wall inside net surface thermal radiation heat gain"
        },
        @{
            Key = $wallKey
            Variable = "Surface Inside Face Convection Heat Gain Rate"
            Description = "wall inside convection heat gain"
        },
        @{
            Key = $wallKey
            Variable = "Surface Outside Face Convection Heat Gain Rate"
            Description = "wall outside convection heat gain"
        },
        @{
            Key = $wallKey
            Variable = "Surface Outside Face Net Thermal Radiation Heat Gain Rate"
            Description = "wall outside net thermal radiation heat gain"
        },
        @{
            Key = $wallKey
            Variable = "Surface Outside Face Solar Radiation Heat Gain Rate"
            Description = "wall outside solar heat gain"
        }
    )
}
$expectedTopMatch = $expectedTopCandidates | Where-Object {
    $_.Key -eq $topBottleneck.output.key -and $_.Variable -eq $topBottleneck.output.variable
} | Select-Object -First 1
if ($null -eq $expectedTopMatch) {
    $expectedTopDescriptions = ($expectedTopCandidates | ForEach-Object {
        "$($_.Description) [$($_.Key) / $($_.Variable)]"
    }) -join "; "
    throw "Expected top bottleneck to be one of $expectedTopDescriptions, got $($topBottleneck.output.key) / $($topBottleneck.output.variable)"
}
if (-not ($summary.series | Where-Object { $_.output.variable -eq "Zone Mean Air Temperature" -and $_.status -eq "extracted" })) {
    throw "Missing extracted Zone Mean Air Temperature series"
}
if (-not ($summary.series | Where-Object { $_.output.variable -eq "Surface Inside Face Temperature" -and $_.status -eq "extracted" })) {
    throw "Missing extracted Surface Inside Face Temperature series"
}
if (-not ($summary.series | Where-Object { $_.output.variable -eq "Surface Inside Face Adjacent Air Temperature" -and $_.status -eq "extracted" })) {
    throw "Missing extracted Surface Inside Face Adjacent Air Temperature series"
}
foreach ($insideVariable in @(
        "Surface Inside Face Convection Heat Transfer Coefficient",
        "Surface Inside Face Convection Heat Gain Rate",
        "Surface Inside Face Net Surface Thermal Radiation Heat Gain Rate"
    )) {
    if (-not ($summary.series | Where-Object { $_.output.variable -eq $insideVariable -and $_.status -eq "extracted" })) {
        throw "Missing extracted $insideVariable series"
    }
}
if (-not ($summary.series | Where-Object { $_.output.variable -eq "Surface Outside Face Temperature" -and $_.status -eq "extracted" })) {
    throw "Missing extracted Surface Outside Face Temperature series"
}
foreach ($weatherVariable in @(
        "Site Sky Temperature",
        "Site Horizontal Infrared Radiation Rate per Area"
    )) {
    if (-not ($summary.series | Where-Object { $_.output.key -eq "Environment" -and $_.output.variable -eq $weatherVariable -and $_.status -eq "extracted" })) {
        throw "Missing extracted diagnostic weather series: $weatherVariable"
    }
}
if (-not ($summary.series | Where-Object { $_.output.variable -eq "Surface Outside Face Incident Solar Radiation Rate per Area" -and $_.status -eq "extracted" })) {
    throw "Missing extracted Surface Outside Face Incident Solar Radiation Rate per Area series"
}
foreach ($solarComponentVariable in @(
        "Surface Outside Face Incident Beam Solar Radiation Rate per Area",
        "Surface Outside Face Incident Sky Diffuse Solar Radiation Rate per Area",
        "Surface Outside Face Incident Ground Diffuse Solar Radiation Rate per Area"
    )) {
    if (-not ($summary.series | Where-Object { $_.output.variable -eq $solarComponentVariable -and $_.status -eq "extracted" })) {
        throw "Missing extracted $solarComponentVariable series"
    }
}
if (-not ($summary.series | Where-Object { $_.output.key -eq "ZN001:ROOF001" -and $_.output.variable -eq "Surface Outside Face Convection Heat Gain Rate" -and $_.status -eq "extracted" })) {
    throw "Missing extracted roof outside convection heat gain series"
}
if (-not ($summary.series | Where-Object { $_.output.key -eq "ZN001:ROOF001" -and $_.output.variable -eq "Surface Outside Face Convection Heat Transfer Coefficient" -and $_.status -eq "extracted" })) {
    throw "Missing extracted roof outside convection coefficient series"
}
if (-not ($summary.series | Where-Object { $_.output.key -eq "ZN001:ROOF001" -and $_.output.variable -eq "Surface Outside Face Net Thermal Radiation Heat Gain Rate" -and $_.status -eq "extracted" })) {
    throw "Missing extracted roof outside net thermal radiation heat gain series"
}
foreach ($roofOutsideRadiationCoefficientVariable in @(
        "Surface Outside Face Thermal Radiation to Air Heat Transfer Coefficient",
        "Surface Outside Face Thermal Radiation to Sky Heat Transfer Coefficient",
        "Surface Outside Face Thermal Radiation to Ground Heat Transfer Coefficient"
    )) {
    if (-not ($summary.series | Where-Object { $_.output.key -eq "ZN001:ROOF001" -and $_.output.variable -eq $roofOutsideRadiationCoefficientVariable -and $_.status -eq "extracted" })) {
        throw "Missing extracted roof outside radiation coefficient series: $roofOutsideRadiationCoefficientVariable"
    }
}
if (-not ($summary.series | Where-Object { $_.output.key -eq "ZN001:ROOF001" -and $_.output.variable -eq "Surface Outside Face Solar Radiation Heat Gain Rate" -and $_.status -eq "extracted" })) {
    throw "Missing extracted roof outside solar radiation heat gain series"
}
foreach ($solarComponentVariable in @(
        "Surface Outside Face Incident Beam Solar Radiation Rate per Area",
        "Surface Outside Face Incident Sky Diffuse Solar Radiation Rate per Area",
        "Surface Outside Face Incident Ground Diffuse Solar Radiation Rate per Area"
    )) {
    if (-not ($summary.series | Where-Object { $_.output.key -eq "ZN001:ROOF001" -and $_.output.variable -eq $solarComponentVariable -and $_.status -eq "extracted" })) {
        throw "Missing extracted roof solar decomposition series: $solarComponentVariable"
    }
}
if (-not ($summary.series | Where-Object { $_.output.variable -eq "Surface Inside Face Conduction Heat Transfer Rate" -and $_.status -eq "extracted" })) {
    throw "Missing extracted Surface Inside Face Conduction Heat Transfer Rate series"
}
if (-not ($summary.series | Where-Object { $_.output.variable -eq "Zone Opaque Surface Inside Faces Conduction Rate" -and $_.status -eq "extracted" })) {
    throw "Missing extracted Zone Opaque Surface Inside Faces Conduction Rate series"
}
if (-not ($summary.series | Where-Object { $_.output.variable -eq "Zone Opaque Surface Outside Faces Conduction Rate" -and $_.status -eq "extracted" })) {
    throw "Missing extracted Zone Opaque Surface Outside Faces Conduction Rate series"
}
if (-not ($summary.series | Where-Object { $_.output.variable -eq "Zone Opaque Surface Outside Faces Conduction Heat Gain Rate" -and $_.status -eq "extracted" })) {
    throw "Missing extracted Zone Opaque Surface Outside Faces Conduction Heat Gain Rate series"
}
if (-not ($summary.series | Where-Object { $_.output.variable -eq "Zone Opaque Surface Outside Faces Conduction Heat Loss Rate" -and $_.status -eq "extracted" })) {
    throw "Missing extracted Zone Opaque Surface Outside Faces Conduction Heat Loss Rate series"
}
if (-not ($summary.series | Where-Object { $_.output.key -eq "Simulation" -and $_.output.variable -eq "Surface Inside Face Heat Balance Calculation Iteration Count" -and $_.status -eq "extracted" })) {
    throw "Missing extracted Surface Inside Face Heat Balance Calculation Iteration Count series"
}
if (-not ($summary.series | Where-Object { $_.output.variable -eq "Zone Mean Air Humidity Ratio" -and $_.status -eq "extracted" })) {
    throw "Missing extracted Zone Mean Air Humidity Ratio series"
}
if (-not ($summary.series | Where-Object { $_.output.variable -eq "Zone Air Heat Balance Internal Convective Heat Gain Rate" -and $_.status -eq "extracted" })) {
    throw "Missing extracted Zone Air Heat Balance Internal Convective Heat Gain Rate series"
}
if (-not ($summary.series | Where-Object { $_.output.variable -eq "Zone Air Heat Balance Surface Convection Rate" -and $_.status -eq "extracted" })) {
    throw "Missing extracted Zone Air Heat Balance Surface Convection Rate series"
}
if (-not ($summary.series | Where-Object { $_.output.variable -eq "Zone Air Heat Balance Air Energy Storage Rate" -and $_.status -eq "extracted" })) {
    throw "Missing extracted Zone Air Heat Balance Air Energy Storage Rate series"
}
if ($CtfSeedPolicy -eq "steady-no-mass-only" -and $ZoneAirAlgorithm -eq "simplified-analytical" -and $SurfaceIterations -eq 1) {
    Assert-SeriesRmseBelow `
        -Summary $summary `
        -Key "ZONE ONE" `
        -Variable "Zone Air Heat Balance Air Energy Storage Rate" `
        -MaxRmse 112.0 `
        -Description "analytical zone air heat-balance storage"
}
foreach ($wallKey in @("ZN001:WALL001", "ZN001:WALL002", "ZN001:WALL003", "ZN001:WALL004")) {
    if (-not ($summary.series | Where-Object { $_.output.key -eq $wallKey -and $_.output.variable -eq "Surface Inside Face Conduction Heat Transfer Rate" -and $_.status -eq "extracted" })) {
        throw "Missing extracted wall decomposition conduction series for $wallKey"
    }
    if (-not ($summary.series | Where-Object { $_.output.key -eq $wallKey -and $_.output.variable -eq "Surface Outside Face Conduction Heat Transfer Rate" -and $_.status -eq "extracted" })) {
        throw "Missing extracted wall outside conduction series for $wallKey"
    }
    foreach ($sourceVariable in @(
            "Surface Outside Face Incident Solar Radiation Rate per Area",
            "Surface Outside Face Incident Beam Solar Radiation Rate per Area",
            "Surface Outside Face Incident Sky Diffuse Solar Radiation Rate per Area",
            "Surface Outside Face Incident Ground Diffuse Solar Radiation Rate per Area",
            "Surface Outside Face Convection Heat Gain Rate",
            "Surface Outside Face Convection Heat Transfer Coefficient",
            "Surface Outside Face Net Thermal Radiation Heat Gain Rate",
            "Surface Outside Face Solar Radiation Heat Gain Rate"
        )) {
        if (-not ($summary.series | Where-Object { $_.output.key -eq $wallKey -and $_.output.variable -eq $sourceVariable -and $_.status -eq "extracted" })) {
            throw "Missing extracted wall exterior source series for ${wallKey}: $sourceVariable"
        }
    }
}
if (-not ($summary.series | Where-Object { $_.output.key -eq "ZN001:FLR001" -and $_.output.variable -eq "Surface Inside Face Conduction Heat Transfer Rate" -and $_.status -eq "extracted" })) {
    throw "Missing extracted floor decomposition conduction series"
}
if (-not ($summary.series | Where-Object { $_.output.key -eq "ZN001:FLR001" -and $_.output.variable -eq "Surface Outside Face Conduction Heat Transfer Rate" -and $_.status -eq "extracted" })) {
    throw "Missing extracted floor outside conduction series"
}
if (-not ($summary.series | Where-Object { $_.output.key -eq "ZN001:FLR001" -and $_.output.variable -eq "Surface Inside Face Conduction Heat Transfer Rate per Area" -and $_.status -eq "extracted" })) {
    throw "Missing extracted floor inside conduction per-area series"
}
if (-not ($summary.series | Where-Object { $_.output.key -eq "ZN001:FLR001" -and $_.output.variable -eq "Surface Outside Face Conduction Heat Transfer Rate per Area" -and $_.status -eq "extracted" })) {
    throw "Missing extracted floor outside conduction per-area series"
}
if (-not ($summary.series | Where-Object { $_.output.key -eq "ZN001:FLR001" -and $_.output.variable -eq "Surface Heat Storage Rate" -and $_.status -eq "extracted" })) {
    throw "Missing extracted floor heat storage series"
}
if (-not ($summary.series | Where-Object { $_.output.key -eq "ZN001:FLR001" -and $_.output.variable -eq "Surface Heat Storage Rate per Area" -and $_.status -eq "extracted" })) {
    throw "Missing extracted floor heat storage per-area series"
}

$reportText = Get-Content -LiteralPath $reportPath -Raw
Assert-Contains -Text $reportText -Pattern "Heat Balance Diagnostic Report" -Description "markdown report header"
Assert-Contains -Text $reportText -Pattern "comparison_class: diagnostic-only" -Description "markdown comparison class"
Assert-Contains -Text $reportText -Pattern "conformance_claim: false" -Description "markdown claim boundary"
Assert-Contains -Text $reportText -Pattern "warmup_enabled: true" -Description "markdown warmup enabled"
Assert-Contains -Text $reportText -Pattern "oracle_run_period_warmup_days: 20" -Description "markdown oracle warmup days"
Assert-Contains -Text $reportText -Pattern "ctf_seed_policy: $CtfSeedPolicy" -Description "markdown CTF seed policy"
Assert-Contains -Text $reportText -Pattern "zone_air_algorithm: $ZoneAirAlgorithm" -Description "markdown zone-air algorithm"
Assert-Contains -Text $reportText -Pattern "surface_iteration_count: $SurfaceIterations" -Description "markdown surface iteration metadata"
Assert-Contains -Text $reportText -Pattern "inside_hconv_reevaluation_interval: $expectedInsideHconvReevaluationIntervalLabel" -Description "markdown inside hconv reevaluation interval metadata"
Assert-Contains -Text $reportText -Pattern "ctf_initial_history_policy: $CtfInitialHistoryPolicy" -Description "markdown CTF initial history policy metadata"
Assert-Contains -Text $reportText -Pattern "zone_conduction_report_source: $ZoneConductionReportSource" -Description "markdown zone conduction report source metadata"
Assert-Contains -Text $reportText -Pattern "zone_air_report_sampling: $ZoneAirReportSampling" -Description "markdown zone air report sampling metadata"
Assert-Contains -Text $reportText -Pattern "surface_loop_zone_air_correction: $SurfaceLoopZoneAirCorrection" -Description "markdown surface loop zone-air correction metadata"
Assert-Contains -Text $reportText -Pattern "## Bottleneck Tracker" -Description "markdown compact bottleneck tracker"
Assert-Contains -Text $reportText -Pattern "top_blocking_mismatch:" -Description "markdown top blocking mismatch"
Assert-Contains -Text $reportText -Pattern "next_blocking_source_mismatch:" -Description "markdown next blocking source mismatch"
Assert-Contains -Text $reportText -Pattern "active_lane: $expectedAlgorithmLane" -Description "markdown active lane"
Assert-Contains -Text $reportText -Pattern "active_algorithm: $ZoneAirAlgorithm" -Description "markdown active algorithm"
Assert-Contains -Text $reportText -Pattern "best_diagnostic_lane:" -Description "markdown best diagnostic lane"
if ($CtfSeedPolicy -eq "steady-no-mass-only") {
    Assert-Contains -Text $reportText -Pattern "ctf_seed_skipped_constructions: FLOOR (#CTFs=5)" -Description "markdown skipped mass CTF construction"
    Assert-Contains -Text $reportText -Pattern "FLOOR (#CTFs=5) @ dt=0.250h [skipped]" -Description "markdown skipped mass CTF summary"
}
else {
    Assert-Contains -Text $reportText -Pattern "ctf_seed_included_constructions: FLOOR, R13WALL, ROOF31" -Description "markdown all-eio included mass CTF construction"
    Assert-Contains -Text $reportText -Pattern "ctf_seed_skipped_constructions: none" -Description "markdown all-eio skipped construction list"
    Assert-Contains -Text $reportText -Pattern "FLOOR (#CTFs=5) @ dt=0.250h [included]" -Description "markdown all-eio mass CTF summary"
}
Assert-Contains -Text $reportText -Pattern "failure_reasons:" -Description "markdown failure diagnostics"
Assert-Contains -Text $reportText -Pattern "## Top Blocker" -Description "markdown top blocker section"
Assert-Contains -Text $reportText -Pattern "blocker_id" -Description "markdown top blocker id column"
Assert-Contains -Text $reportText -Pattern "## Top 10 RMSE Variables" -Description "markdown top 10 RMSE section"
Assert-Contains -Text $reportText -Pattern "| rank | key | variable | category | family | class | first_hour_abs_delta_c | annual_rmse_delta_c | max_abs_delta_c | status |" -Description "markdown top 10 RMSE columns"
Assert-Contains -Text $reportText -Pattern "| 1 | ZN001:ROOF001 | Surface Outside Face Convection Heat Gain Rate | surface | roof | surface-state |" -Description "markdown top RMSE classified roof row"
Assert-Contains -Text $reportText -Pattern "## Blocking Diagnostic Split" -Description "markdown blocking diagnostic split section"
Assert-Contains -Text $reportText -Pattern "| mat-rmse | ZONE ONE | Zone Mean Air Temperature | zone |" -Description "markdown MAT RMSE split row"
Assert-Contains -Text $reportText -Pattern "| surface-conduction-rmse |" -Description "markdown surface conduction RMSE split row"
Assert-Contains -Text $reportText -Pattern "| zone-air-storage-rmse | ZONE ONE | Zone Air Heat Balance Air Energy Storage Rate | zone |" -Description "markdown zone air storage RMSE split row"
Assert-Contains -Text $reportText -Pattern "| zone-surface-convection-rmse | ZONE ONE | Zone Air Heat Balance Surface Convection Rate | zone |" -Description "markdown zone surface convection RMSE split row"
Assert-Contains -Text $reportText -Pattern "## Zone-Air Coefficient Split" -Description "markdown zone-air coefficient split section"
Assert-Contains -Text $reportText -Pattern "| key | samples | first_divergence_source | first_divergence_sample | first_divergence_delta | SumHA_rmse | SumHATsurf_rmse | SumHATref_rmse | TempDepCoef_rmse | TempIndCoef_rmse | AirPowerCap_rmse | TempHistoryTerm_rmse | MAT_rmse | AirStorage_rmse | SurfaceConvection_rmse |" -Description "markdown zone-air coefficient split columns"
$zoneAirFirstDivergenceSample = if ($null -eq $zoneAirCoefficientDelta.first_divergence_sample_index) {
    "n/a"
}
else {
    [string]$zoneAirCoefficientDelta.first_divergence_sample_index
}
Assert-Contains -Text $reportText -Pattern "| ZONE ONE | 8760 | $($zoneAirCoefficientDelta.first_divergence_source) | $zoneAirFirstDivergenceSample |" -Description "markdown zone-air first divergence row"
Assert-Contains -Text $reportText -Pattern "## Surface Family RMSE" -Description "markdown surface family RMSE section"
Assert-Contains -Text $reportText -Pattern "| floor |" -Description "markdown floor RMSE row"
Assert-Contains -Text $reportText -Pattern "| roof |" -Description "markdown roof RMSE row"
Assert-Contains -Text $reportText -Pattern "| wall |" -Description "markdown wall RMSE row"
Assert-Contains -Text $reportText -Pattern "## Source-Order Trace" -Description "markdown source-order trace section"
Assert-Contains -Text $reportText -Pattern "source_order_wrapper: ep_runtime::heat_balance::manager::manage_heat_balance_source_order_path" -Description "markdown ManageHeatBalance source-order wrapper"
Assert-Contains -Text $reportText -Pattern "rust_execution_plan_order: ExecutionPlan.compatibility_stages" -Description "markdown Rust ExecutionPlan order"
Assert-Contains -Text $reportText -Pattern "stage_snapshot_policy: start/end surface+MAT snapshot anchors" -Description "markdown source-order stage snapshot policy"
Assert-Contains -Text $reportText -Pattern "| ManageHeatBalance | ManageHeatBalance | manage-heat-balance-wrapper |" -Description "markdown ManageHeatBalance trace row"
Assert-Contains -Text $reportText -Pattern "| InitHeatBalance | InitHeatBalance | init-heat-balance |" -Description "markdown InitHeatBalance trace row"
Assert-Contains -Text $reportText -Pattern "| CalcHeatBalanceOutsideSurf | CalcHeatBalanceOutsideSurf | calc-heat-balance-outside-surf |" -Description "markdown outside surface trace row"
Assert-Contains -Text $reportText -Pattern "| CalcHeatBalanceInsideSurf | CalcHeatBalanceInsideSurf | calc-heat-balance-inside-surf |" -Description "markdown inside surface trace row"
Assert-Contains -Text $reportText -Pattern "| ManageAirHeatBalance | ManageAirHeatBalance | manage-air-heat-balance |" -Description "markdown air heat-balance trace row"
Assert-Contains -Text $reportText -Pattern "| UpdateThermalHistories | UpdateThermalHistories | update-thermal-histories |" -Description "markdown thermal history trace row"
Assert-Contains -Text $reportText -Pattern "| ReportSurfaceHeatBalance | ReportSurfaceHeatBalance | report-surface-heat-balance |" -Description "markdown surface report trace row"
Assert-Contains -Text $reportText -Pattern "| ReportZoneMeanAirTemp | ReportHeatBalance -> ReportZoneMeanAirTemp | report-heat-balance |" -Description "markdown zone MAT report trace row"
Assert-Contains -Text $reportText -Pattern "## Warmup End-State Deltas" -Description "markdown warmup end-state delta section"
Assert-Contains -Text $reportText -Pattern "warmup-end-state-mat-delta" -Description "markdown warmup MAT delta row"
Assert-Contains -Text $reportText -Pattern "warmup-end-state-surface-temperature-delta" -Description "markdown warmup surface temperature delta row"
Assert-Contains -Text $reportText -Pattern "warmup-end-state-ctf-history-delta" -Description "markdown warmup CTF history delta row"
Assert-Contains -Text $reportText -Pattern "warmup-end-state-zone-history-delta" -Description "markdown warmup zone history delta row"
Assert-Contains -Text $reportText -Pattern "## First Divergence by Variable" -Description "markdown first divergence by variable section"
Assert-Contains -Text $reportText -Pattern "first_divergence_rows: top-" -Description "markdown compact first divergence rows"
Assert-Contains -Text $reportText -Pattern "## Diagnostic Evidence" -Description "markdown compact diagnostic evidence section"
Assert-Contains -Text $reportText -Pattern "compare_summary_json: compare-summary.json" -Description "markdown summary artifact path"
Assert-Contains -Text $reportText -Pattern "compare_digest_json: compare-digest.json" -Description "markdown digest artifact path"
Assert-NotContains -Text $reportText -Pattern "## Current Blockers" -Description "markdown current blocker appendix"
Assert-NotContains -Text $reportText -Pattern "## EnergyPlus Compatibility Stage Order" -Description "markdown compatibility stage appendix"
Assert-NotContains -Text $reportText -Pattern "## Bottlenecks" -Description "markdown bottleneck appendix"
Assert-NotContains -Text $reportText -Pattern "## Max-Sample Contexts" -Description "markdown max-sample appendix"
Assert-NotContains -Text $reportText -Pattern "## First-Sample Bottlenecks" -Description "markdown first-sample appendix"
Assert-NotContains -Text $reportText -Pattern "## Rust Zone-Air First-Sample Trace" -Description "markdown zone-air trace appendix"
Assert-NotContains -Text $reportText -Pattern "## Rust Surface First-Sample Trace" -Description "markdown surface trace appendix"
Assert-NotContains -Text $reportText -Pattern "## Rust CTF First-Sample Components" -Description "markdown CTF component appendix"
Assert-NotContains -Text $reportText -Pattern "## Zone-Air Coefficient Deltas" -Description "markdown zone-air coefficient appendix"
Assert-NotContains -Text $reportText -Pattern "## CTF History Series Deltas" -Description "markdown CTF history series appendix"
Assert-NotContains -Text $reportText -Pattern "## Inside Solve Series Deltas" -Description "markdown inside-solve series appendix"
Assert-NotContains -Text $reportText -Pattern "## Hourly Samples" -Description "markdown hourly sample appendix"
Assert-Contains -Text $reportText -Pattern "status: fail" -Description "markdown diagnostic status"

Write-Host "Official dynamic heat-balance diagnostic passed with CTF seed policy $CtfSeedPolicy."
