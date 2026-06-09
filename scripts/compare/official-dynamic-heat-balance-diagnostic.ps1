[CmdletBinding()]
param(
    [ValidateSet("steady-no-mass-only", "all-eio")]
    [string]$CtfSeedPolicy = "steady-no-mass-only",
    [ValidateSet("boundary-u-value", "energyplus-surf-initial")]
    [string]$CtfInitialHistoryPolicy = "boundary-u-value",
    [ValidateSet("simplified-analytical", "energyplus-analytical-probe", "energyplus-analytical-surface-first-probe", "energyplus-analytical-coupled-probe", "energyplus-analytical-coupled-previous-inside-probe", "energyplus-analytical-coupled-previous-inside-doe2-probe", "energyplus-analytical-coupled-previous-inside-quick-outside-probe", "energyplus-analytical-coupled-previous-inside-quick-outside-interleaved-probe", "energyplus-analytical-coupled-previous-inside-quick-outside-interleaved-interior-longwave-probe", "energyplus-analytical-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-probe", "energyplus-analytical-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-current-longwave-probe", "energyplus-analytical-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-current-adiabatic-probe", "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-probe", "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-probe", "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-probe", "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-previous-mat-surface-convection-probe", "energyplus-analytical-coupled-previous-inside-quick-outside-interleaved-scriptf-interior-longwave-probe", "energyplus-analytical-coupled-previous-inside-quick-outside-doe2-probe", "energyplus-analytical-coupled-previous-inside-quick-outside-interior-longwave-probe", "energyplus-analytical-coupled-previous-inside-quick-outside-doe2-interior-longwave-probe", "energyplus-analytical-coupled-previous-inside-quick-outside-scriptf-interior-longwave-probe", "energyplus-analytical-coupled-previous-inside-quick-outside-doe2-scriptf-interior-longwave-probe", "energyplus-analytical-coupled-previous-boundary-probe", "energyplus-third-order-probe")]
    [string]$ZoneAirAlgorithm = "simplified-analytical",
    [ValidateRange(0, 365)]
    [int]$WarmupMinimumDays = 0,
    [ValidateRange(1, 200)]
    [int]$SurfaceIterations = 1
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$AlgorithmOutputSuffix = switch ($ZoneAirAlgorithm) {
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
$OutputRootRelative = if ($CtfSeedPolicy -eq "all-eio") {
    ".runtime\official-dynamic-diagnostic-all-ctf$AlgorithmOutputSuffix$InitialHistoryOutputSuffix$WarmupOutputSuffix$SurfaceIterationOutputSuffix\26.1.0"
}
else {
    ".runtime\official-dynamic-diagnostic$AlgorithmOutputSuffix$InitialHistoryOutputSuffix$WarmupOutputSuffix$SurfaceIterationOutputSuffix\26.1.0"
}
$OutputRoot = Join-Path $RepoRoot $OutputRootRelative
$CaseId = "official_1zone_uncontrolled_dynamic_diagnostic_001"
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

Write-Host "Running official dynamic heat-balance diagnostic gate with CTF seed policy $CtfSeedPolicy, CTF initial history policy $CtfInitialHistoryPolicy, zone-air algorithm $ZoneAirAlgorithm, warmup minimum days $WarmupMinimumDays, and surface iterations $SurfaceIterations."
$policyEnvName = "RUSTED_ENERGYPLUS_HEAT_BALANCE_CTF_SEED_POLICY"
$initialHistoryPolicyEnvName = "RUSTED_ENERGYPLUS_HEAT_BALANCE_CTF_INITIAL_HISTORY_POLICY"
$algorithmEnvName = "RUSTED_ENERGYPLUS_HEAT_BALANCE_ZONE_AIR_ALGORITHM"
$warmupEnvName = "RUSTED_ENERGYPLUS_HEAT_BALANCE_WARMUP_MINIMUM_DAYS"
$surfaceIterationsEnvName = "RUSTED_ENERGYPLUS_HEAT_BALANCE_SURFACE_ITERATIONS"
$previousPolicy = [Environment]::GetEnvironmentVariable($policyEnvName, "Process")
$previousInitialHistoryPolicy = [Environment]::GetEnvironmentVariable($initialHistoryPolicyEnvName, "Process")
$previousAlgorithm = [Environment]::GetEnvironmentVariable($algorithmEnvName, "Process")
$previousWarmup = [Environment]::GetEnvironmentVariable($warmupEnvName, "Process")
$previousSurfaceIterations = [Environment]::GetEnvironmentVariable($surfaceIterationsEnvName, "Process")
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
Assert-Contains -Text $text -Pattern "surface_iteration_count: $SurfaceIterations" -Description "surface iteration metadata"
Assert-Contains -Text $text -Pattern "ctf_initial_history_policy: $CtfInitialHistoryPolicy" -Description "CTF initial history policy metadata"
Assert-Contains -Text $text -Pattern "compare_digest:" -Description "compact digest artifact path"
Assert-Contains -Text $text -Pattern "status: fail" -Description "current diagnostic status"

$summaryPath = Join-Path $CompareRoot "compare-summary.json"
$digestPath = Join-Path $CompareRoot "compare-digest.json"
$reportPath = Join-Path $CompareRoot "compare-report.md"
Assert-FileExists -Path $summaryPath -Description "official dynamic diagnostic summary"
Assert-FileExists -Path $digestPath -Description "official dynamic diagnostic digest"
Assert-FileExists -Path $reportPath -Description "official dynamic diagnostic report"

$digestText = Get-Content -LiteralPath $digestPath -Raw
Assert-NotContains -Text $digestText -Pattern '"sample_rows"' -Description "compact digest sample row payload"
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
if ($summary.surface_iteration_count -ne $SurfaceIterations) {
    throw "Expected surface_iteration_count $SurfaceIterations, got $($summary.surface_iteration_count)"
}
if ($summary.ctf_initial_history_policy -ne $CtfInitialHistoryPolicy) {
    throw "Expected ctf_initial_history_policy $CtfInitialHistoryPolicy, got $($summary.ctf_initial_history_policy)"
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
if ($summary.series_count -ne 99) {
    throw "Unexpected series_count: $($summary.series_count)"
}
if ($summary.max_abs_delta_c -le 1.0) {
    throw "Expected current official dynamic diagnostic delta to remain visible, got $($summary.max_abs_delta_c)"
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
if ($CtfSeedPolicy -eq "all-eio") {
    $floorHistoryDelta = @($summary.ctf_history_first_sample_deltas | Where-Object { $_.key -eq "ZN001:FLR001" })[0]
    if ($null -eq $floorHistoryDelta) {
        throw "Expected ctf_history_first_sample_deltas to include ZN001:FLR001 in all-eio mode"
    }
    if ([double]$floorHistoryDelta.inside_history_delta_w -le 100.0) {
        throw "Expected active FLOOR inside history delta to remain visible, got $($floorHistoryDelta.inside_history_delta_w)"
    }
    if ([double]$floorHistoryDelta.outside_history_delta_w -le 100.0) {
        throw "Expected active FLOOR outside history delta to remain visible, got $($floorHistoryDelta.outside_history_delta_w)"
    }
    $floorHistorySeriesDelta = @($summary.ctf_history_series_deltas | Where-Object { $_.key -eq "ZN001:FLR001" })[0]
    if ($null -eq $floorHistorySeriesDelta) {
        throw "Expected ctf_history_series_deltas to include ZN001:FLR001 in all-eio mode"
    }
    if ([int]$floorHistorySeriesDelta.samples -ne 8760) {
        throw "Expected FLOOR CTF history series deltas to use 8760 samples, got $($floorHistorySeriesDelta.samples)"
    }
    if ([double]$floorHistorySeriesDelta.inside_history_delta.rmse_delta_c -le 10.0) {
        throw "Expected active FLOOR inside history series delta to remain visible, got $($floorHistorySeriesDelta.inside_history_delta.rmse_delta_c)"
    }
    if ([double]$floorHistorySeriesDelta.outside_history_delta.rmse_delta_c -le 10.0) {
        throw "Expected active FLOOR outside history series delta to remain visible, got $($floorHistorySeriesDelta.outside_history_delta.rmse_delta_c)"
    }
    $floorRunPeriodInitialSlots = @($summary.ctf_history_run_period_initial_slots | Where-Object { $_.key -eq "ZN001:FLR001" })
    if ($floorRunPeriodInitialSlots.Count -lt 5) {
        throw "Expected ctf_history_run_period_initial_slots to include FLOOR #CTFs=5 rows, got $($floorRunPeriodInitialSlots.Count)"
    }
    $floorHistorySlots = @($summary.ctf_history_first_sample_slots | Where-Object { $_.key -eq "ZN001:FLR001" })
    if ($floorHistorySlots.Count -lt 5) {
        throw "Expected ctf_history_first_sample_slots to include FLOOR #CTFs=5 rows, got $($floorHistorySlots.Count)"
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
    $ZoneAirAlgorithm -eq "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-previous-mat-surface-convection-probe"
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
        -MaxRmse 100.0 `
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
Assert-Contains -Text $reportText -Pattern "ctf_initial_history_policy: $CtfInitialHistoryPolicy" -Description "markdown CTF initial history policy metadata"
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
Assert-Contains -Text $reportText -Pattern "mean_abs_delta_c" -Description "markdown mean absolute delta column"
Assert-Contains -Text $reportText -Pattern "## Bottlenecks" -Description "markdown bottleneck ranking section"
Assert-Contains -Text $reportText -Pattern "## First-Sample Bottlenecks" -Description "markdown first-sample bottleneck ranking section"
Assert-Contains -Text $reportText -Pattern "## Rust CTF First-Sample Components" -Description "markdown CTF first-sample component section"
Assert-Contains -Text $reportText -Pattern "in_history_w" -Description "markdown CTF component history column"
Assert-Contains -Text $reportText -Pattern "## CTF History First-Sample Deltas" -Description "markdown CTF first-sample history delta section"
Assert-Contains -Text $reportText -Pattern "ctf_y0" -Description "markdown CTF zero cross coefficient column"
Assert-Contains -Text $reportText -Pattern "in_temp_abs_delta_c" -Description "markdown CTF inside face temperature delta column"
Assert-Contains -Text $reportText -Pattern "out_temp_abs_delta_c" -Description "markdown CTF outside face temperature delta column"
Assert-Contains -Text $reportText -Pattern "in_current_abs_delta_w" -Description "markdown CTF current delta column"
Assert-Contains -Text $reportText -Pattern "in_history_abs_delta_w" -Description "markdown CTF history delta column"
Assert-Contains -Text $reportText -Pattern "## CTF History Series Deltas" -Description "markdown CTF history series delta section"
Assert-Contains -Text $reportText -Pattern "in_history_rmse_w" -Description "markdown CTF history series RMSE column"
Assert-Contains -Text $reportText -Pattern "out_history_rmse_w" -Description "markdown CTF outside history series RMSE column"
Assert-Contains -Text $reportText -Pattern "## Rust CTF History Run-Period Initial Slots" -Description "markdown CTF run-period initial slot section"
Assert-Contains -Text $reportText -Pattern "## Rust CTF History First-Sample Slots" -Description "markdown CTF first-sample slot section"
Assert-Contains -Text $reportText -Pattern "## Hourly Samples" -Description "markdown hourly sample section"
Assert-Contains -Text $reportText -Pattern "Surface Inside Face Temperature" -Description "markdown inside face temperature variable"
Assert-Contains -Text $reportText -Pattern "Surface Inside Face Convection Heat Transfer Coefficient" -Description "markdown inside convection coefficient variable"
Assert-Contains -Text $reportText -Pattern "Surface Inside Face Convection Heat Gain Rate" -Description "markdown inside convection source variable"
Assert-Contains -Text $reportText -Pattern "Surface Inside Face Net Surface Thermal Radiation Heat Gain Rate" -Description "markdown inside radiation source variable"
Assert-Contains -Text $reportText -Pattern "Surface Outside Face Temperature" -Description "markdown outside face temperature variable"
Assert-Contains -Text $reportText -Pattern "Surface Outside Face Incident Solar Radiation Rate per Area" -Description "markdown outside incident solar variable"
Assert-Contains -Text $reportText -Pattern "Surface Outside Face Incident Beam Solar Radiation Rate per Area" -Description "markdown outside incident beam solar variable"
Assert-Contains -Text $reportText -Pattern "Surface Outside Face Incident Sky Diffuse Solar Radiation Rate per Area" -Description "markdown outside incident sky diffuse solar variable"
Assert-Contains -Text $reportText -Pattern "Surface Outside Face Incident Ground Diffuse Solar Radiation Rate per Area" -Description "markdown outside incident ground diffuse solar variable"
Assert-Contains -Text $reportText -Pattern "Surface Outside Face Convection Heat Gain Rate" -Description "markdown outside convection source variable"
Assert-Contains -Text $reportText -Pattern "Surface Outside Face Net Thermal Radiation Heat Gain Rate" -Description "markdown outside radiation source variable"
Assert-Contains -Text $reportText -Pattern "Surface Outside Face Solar Radiation Heat Gain Rate" -Description "markdown outside solar source variable"
Assert-Contains -Text $reportText -Pattern "Zone Opaque Surface Inside Faces Conduction Rate" -Description "markdown zone conduction variable"
Assert-Contains -Text $reportText -Pattern "Zone Opaque Surface Outside Faces Conduction Rate" -Description "markdown zone outside conduction variable"
Assert-Contains -Text $reportText -Pattern "Zone Air Heat Balance Surface Convection Rate" -Description "markdown zone air heat-balance variable"
Assert-Contains -Text $reportText -Pattern "ZN001:FLR001" -Description "markdown floor decomposition key"
Assert-Contains -Text $reportText -Pattern "ZN001:WALL001" -Description "markdown wall decomposition key"
Assert-Contains -Text $reportText -Pattern "Surface Outside Face Conduction Heat Transfer Rate" -Description "markdown floor outside conduction variable"
Assert-Contains -Text $reportText -Pattern "Surface Heat Storage Rate" -Description "markdown floor storage variable"
Assert-Contains -Text $reportText -Pattern "Surface Heat Storage Rate per Area" -Description "markdown floor storage per-area variable"
Assert-Contains -Text $reportText -Pattern "status: fail" -Description "markdown diagnostic status"

Write-Host "Official dynamic heat-balance diagnostic passed with CTF seed policy $CtfSeedPolicy."
