[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$CaseId = "official_1zone_uncontrolled_dynamic_conformance_candidate_001"
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$OutputRoot = Join-Path $RepoRoot ".runtime\official-dynamic-compat-candidate\26.1.0"
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
    Write-Host "OK absent $Description`: $Pattern"
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

foreach ($path in @(
    (Join-Path $OracleRoot "energyplus.exe"),
    (Join-Path $OracleRoot "ConvertInputFormat.exe"),
    (Join-Path $OracleRoot "WeatherData\USA_CO_Golden-NREL.724666_TMY3.epw"),
    $CasePath
)) {
    if (-not (Test-Path -LiteralPath $path -PathType Leaf)) {
        throw "Missing required official dynamic conformance file: $path"
    }
}

Remove-RepoDirectory -Path $OutputRoot

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Write-Host "Running official 1ZoneUncontrolled dynamic heat-balance conformance gate."
$output = & $cargo.Source run -p ep_cli --quiet -- conformance heat-balance-report $CasePath $OracleRoot $OutputRoot 2>&1
if ($LASTEXITCODE -ne 0) {
    $output | ForEach-Object { Write-Host $_ }
    throw "Official dynamic heat-balance conformance gate failed."
}

$text = ($output -join "`n")
Assert-Contains -Text $text -Pattern "Conformance Heat Balance Report" -Description "report header"
Assert-Contains -Text $text -Pattern "id: $CaseId" -Description "case id"
Assert-Contains -Text $text -Pattern "comparison_class: conformance" -Description "comparison class"
Assert-Contains -Text $text -Pattern "conformance_claim: true" -Description "conformance claim"
Assert-Contains -Text $text -Pattern "zone_air_algorithm_lane: compatibility-source-order" -Description "compatibility lane"
Assert-Contains -Text $text -Pattern "compatibility_source_order: true" -Description "source-order compatibility flag"
Assert-Contains -Text $text -Pattern "diagnostic_probe_used: false" -Description "diagnostic probe flag"
Assert-Contains -Text $text -Pattern "conformance_promotion_allowed: true" -Description "promotion eligibility"
Assert-Contains -Text $text -Pattern "heat_balance_active_branch_scope: official-1zone-declared-compatibility-branches-only" -Description "active branch scope"
Assert-Contains -Text $text -Pattern "heat_balance_unsupported_active_branch_policy: block-conformance-promotion" -Description "unsupported active branch policy"
Assert-Contains -Text $text -Pattern "pool_heat_transfer | not-active-in-target-case | excluded-from-conformance" -Description "pool inactive branch row"
Assert-Contains -Text $text -Pattern "radiant_system_surface_source | not-active-in-target-case | excluded-from-conformance" -Description "radiant inactive branch row"
Assert-Contains -Text $text -Pattern "hamt_heat_and_moisture_transfer | not-active-in-target-case | excluded-from-conformance" -Description "HAMT inactive branch row"
Assert-Contains -Text $text -Pattern "condfd_finite_difference_conduction | not-active-in-target-case | excluded-from-conformance" -Description "CondFD inactive branch row"
Assert-Contains -Text $text -Pattern "kiva_foundation_heat_transfer | not-active-in-target-case | excluded-from-conformance" -Description "Kiva inactive branch row"
Assert-Contains -Text $text -Pattern "surface_iteration_count: 20" -Description "surface iteration count"
Assert-Contains -Text $text -Pattern "surface_loop_zone_air_correction: after-surface-loop" -Description "surface loop zone-air correction"
Assert-Contains -Text $text -Pattern "ctf_initial_history_policy: energyplus-surf-initial" -Description "CTF history policy"
Assert-Contains -Text $text -Pattern "building_warmup_minimum_days: 6" -Description "Building warmup minimum day report"
Assert-Contains -Text $text -Pattern "building_warmup_maximum_days: 30" -Description "Building warmup maximum day report"
Assert-Contains -Text $text -Pattern "building_temperature_convergence_tolerance_delta_c: 0.004000000000" -Description "Building warmup temperature tolerance report"
Assert-Contains -Text $text -Pattern "building_loads_convergence_tolerance_w: 0.040000000000" -Description "Building warmup load tolerance report"
Assert-Contains -Text $text -Pattern "warmup_minimum_days: 20" -Description "effective warmup minimum day report"
Assert-Contains -Text $text -Pattern "warmup_maximum_days: 30" -Description "effective warmup maximum day report"
Assert-Contains -Text $text -Pattern "warmup_temperature_convergence_tolerance_delta_c: 0.004000000000" -Description "effective warmup temperature tolerance report"
Assert-Contains -Text $text -Pattern "warmup_loads_convergence_tolerance_w: 0.040000000000" -Description "effective warmup load tolerance report"
Assert-Contains -Text $text -Pattern "status: pass" -Description "gate status"

$summaryPath = Join-Path $CompareRoot "compare-summary.json"
$digestPath = Join-Path $CompareRoot "compare-digest.json"
$reportPath = Join-Path $CompareRoot "compare-report.md"
$selectedOutputsPath = Join-Path $CompareRoot "selected_outputs.json"
$performanceSummaryPath = Join-Path $CompareRoot "performance-summary.json"
$eioPath = Join-Path $CaseOutputRoot "oracle\eplusout.eio"
$iddPath = Join-Path $OracleRoot "Energy+.idd"
$sourceIdfPath = Join-Path $OracleRoot "ExampleFiles\1ZoneUncontrolled.idf"
Assert-FileExists -Path $summaryPath -Description "official dynamic summary"
Assert-FileExists -Path $digestPath -Description "official dynamic digest"
Assert-FileExists -Path $reportPath -Description "official dynamic report"
Assert-FileExists -Path $selectedOutputsPath -Description "official dynamic selected outputs"
Assert-FileExists -Path $performanceSummaryPath -Description "official dynamic performance summary"
Assert-FileExists -Path $eioPath -Description "official dynamic oracle EIO"
Assert-FileExists -Path $iddPath -Description "official EnergyPlus IDD"
Assert-FileExists -Path $sourceIdfPath -Description "official source IDF"

$summary = Get-Content -LiteralPath $summaryPath -Raw | ConvertFrom-Json
if ($summary.case_id -ne $CaseId) {
    throw "Unexpected case_id: $($summary.case_id)"
}
if ($summary.comparison_class -ne "conformance") {
    throw "Unexpected comparison_class: $($summary.comparison_class)"
}
if ($summary.conformance_claim -ne $true) {
    throw "Official dynamic candidate must retain conformance_claim=true"
}
if ($summary.gate.blocking -ne $true) {
    throw "Official dynamic candidate gate must be blocking"
}
if ($summary.status -ne "pass") {
    throw "Unexpected official dynamic conformance status: $($summary.status)"
}
if (@($summary.failure_reasons).Count -ne 0) {
    throw "Official dynamic conformance should not report failure reasons"
}
$selectedOutputs = Get-Content -LiteralPath $selectedOutputsPath -Raw | ConvertFrom-Json
$performanceSummary = Get-Content -LiteralPath $performanceSummaryPath -Raw | ConvertFrom-Json
if ($selectedOutputs.metadata_policy -ne "EnergyPlus ESO dictionary metadata and ep_runtime::ResultStore store type metadata") {
    throw "Unexpected selected output metadata policy: $($selectedOutputs.metadata_policy)"
}
if ($performanceSummary.conformance_status -ne "pass") {
    throw "Performance summary must be attached to a passing conformance gate, got $($performanceSummary.conformance_status)"
}
if ($performanceSummary.compatibility_mode_separated_from_fast_mode -ne $true) {
    throw "Performance summary must separate compatibility mode from fast mode"
}
if ($performanceSummary.speedup_claim_allowed -ne $true) {
    throw "Passing official candidate should allow reporting measured speedup evidence"
}
if ([double]$performanceSummary.energyplus_oracle_wall_seconds -lt 0.0 -or [double]$performanceSummary.rust_compare_report_wall_seconds -lt 0.0) {
    throw "Oracle/Rust performance comparison times must be non-negative"
}
if ($summary.time_axis.source -ne "shared TimeAxis for weather/schedule/output/report") {
    throw "Unexpected C1 time axis source: $($summary.time_axis.source)"
}
if ($summary.time_axis.zone_timesteps_per_hour -ne 4) {
    throw "Unexpected C1 zone timesteps per hour: $($summary.time_axis.zone_timesteps_per_hour)"
}
if ([math]::Abs([double]$summary.time_axis.zone_timestep_seconds - 900.0) -gt 0.000000000001) {
    throw "Unexpected C1 zone timestep seconds: $($summary.time_axis.zone_timestep_seconds)"
}
if ([math]::Abs([double]$summary.time_axis.system_timestep_nominal_seconds - 900.0) -gt 0.000000000001) {
    throw "Unexpected C1 system timestep seconds: $($summary.time_axis.system_timestep_nominal_seconds)"
}
if ($summary.time_axis.variable_system_timestep_support -ne "placeholder-state-backed") {
    throw "Unexpected C1 variable system timestep support: $($summary.time_axis.variable_system_timestep_support)"
}
if ($summary.time_axis.shorten_timestep_sys_state -ne $true -or $summary.time_axis.use_zone_timestep_history_state -ne $true) {
    throw "C1 EnergyPlus system timestep history state flags were not true"
}
if ($summary.time_axis.hvac_iteration_count -ne 0 -or $summary.time_axis.plant_iteration_count -ne 0) {
    throw "C1 HVAC/Plant iteration count state must start at zero"
}
if ($summary.time_axis.warmup_reported_samples -ne 0 -or $summary.time_axis.run_period_reported_samples -ne 8760 -or $summary.time_axis.design_day_reported_samples -ne 0) {
    throw "Unexpected C1 sample partition: warmup=$($summary.time_axis.warmup_reported_samples), run_period=$($summary.time_axis.run_period_reported_samples), design_day=$($summary.time_axis.design_day_reported_samples)"
}
foreach ($phaseName in @(
    "parse_time",
    "raw_model_build",
    "typed_model_compile",
    "simulation_model_compile",
    "model_graph_build",
    "execution_plan_build",
    "weather_schedule_precompute",
    "runtime_heat_balance_execution",
    "output_report_generation",
    "trace_write"
)) {
    $phase = @($performanceSummary.phases) | Where-Object { $_.name -eq $phaseName } | Select-Object -First 1
    if (-not $phase) {
        throw "Missing performance phase: $phaseName"
    }
    if ([double]$phase.wall_seconds -lt 0.0) {
        throw "Performance phase $phaseName had negative wall_seconds: $($phase.wall_seconds)"
    }
}
if ($selectedOutputs.timestamp_rule -ne "hour-ending hourly samples aligned to the run-period time axis") {
    throw "Unexpected selected output timestamp rule: $($selectedOutputs.timestamp_rule)"
}
if (@($selectedOutputs.series).Count -ne @($summary.series).Count) {
    throw "selected_outputs series count $(@($selectedOutputs.series).Count) did not match summary series count $(@($summary.series).Count)"
}
foreach ($row in @($summary.series)) {
    if ($row.samples -ne 8760 -or $row.oracle_count -ne 8760 -or $row.rust_count -ne 8760) {
        throw "Unexpected A15 sample counts for $($row.output.key)/$($row.output.variable): sample_count=$($row.samples), oracle_count=$($row.oracle_count), rust_count=$($row.rust_count)"
    }
    if ($row.energyplus_store_type -ne "average" -or $row.rust_store_type -ne "average" -or $row.store_type_match -ne $true) {
        throw "Unexpected A15 store type for $($row.output.key)/$($row.output.variable): EnergyPlus=$($row.energyplus_store_type), Rust=$($row.rust_store_type), match=$($row.store_type_match)"
    }
    if ($row.timestamp_match -ne $true) {
        throw "A15 timestamp mismatch for $($row.output.key)/$($row.output.variable)"
    }
    if ($row.first_reported_sample_hour_ending -ne $true) {
        throw "A15 first reported sample is not hour-ending for $($row.output.key)/$($row.output.variable)"
    }
}
$firstSelectedOutput = @($selectedOutputs.series)[0]
if ($firstSelectedOutput.first_timestamp -ne "env=RUN PERIOD 1;day=1;month=1;date=1;dst=0;hour=1;start=0.00;end=60.00;day_type=Tuesday") {
    throw "Unexpected first selected output timestamp: $($firstSelectedOutput.first_timestamp)"
}
if ($firstSelectedOutput.last_timestamp -ne "env=RUN PERIOD 1;day=365;month=12;date=31;dst=0;hour=24;start=0.00;end=60.00;day_type=Tuesday") {
    throw "Unexpected last selected output timestamp: $($firstSelectedOutput.last_timestamp)"
}
if ($summary.zone_air_algorithm -ne "energyplus-heat-balance-compat-candidate") {
    throw "Unexpected zone_air_algorithm: $($summary.zone_air_algorithm)"
}
if ($summary.zone_air_algorithm_lane -ne "compatibility-source-order") {
    throw "Unexpected algorithm lane: $($summary.zone_air_algorithm_lane)"
}
if ($summary.compatibility_source_order -ne $true) {
    throw "Compatibility candidate must mark compatibility_source_order=true"
}
if ($summary.diagnostic_probe_used -ne $false) {
    throw "Compatibility candidate must mark diagnostic_probe_used=false"
}
if ($summary.conformance_promotion_allowed -ne $true) {
    throw "Compatibility candidate must be promotion-eligible"
}
$compatibilityStages = @($summary.compatibility_stages)
if ($compatibilityStages.Count -lt 18) {
    throw "Expected source-order compatibility stage checklist rows, got $($compatibilityStages.Count)"
}
foreach ($routine in @(
    "ManageSurfaceHeatBalance",
    "CalcHeatBalanceOutsideSurf",
    "CalcHeatBalanceInsideSurf",
    "ManageAirHeatBalance",
    "ManageZoneAirUpdates",
    "UpdateThermalHistories",
    "ReportSurfaceHeatBalance"
)) {
    if (-not ($compatibilityStages | Where-Object { $_.source_routine -eq $routine })) {
        throw "Missing source-order compatibility stage routine: $routine"
    }
}
$branchStatus = @($summary.heat_balance_branch_status)
if ($branchStatus.Count -ne 6) {
    throw "Expected 6 heat-balance branch status rows, got $($branchStatus.Count)"
}
foreach ($branch in @(
    "pool_heat_transfer",
    "radiant_system_surface_source",
    "hamt_heat_and_moisture_transfer",
    "condfd_finite_difference_conduction",
    "kiva_foundation_heat_transfer"
)) {
    $row = $branchStatus | Where-Object { $_.branch -eq $branch } | Select-Object -First 1
    if (-not $row) {
        throw "Missing inactive heat-balance branch row: $branch"
    }
    if ($row.status -ne "not-active-in-target-case") {
        throw "Unexpected inactive branch status for ${branch}: $($row.status)"
    }
    if ($row.promotion_policy -ne "excluded-from-conformance") {
        throw "Unexpected inactive branch promotion policy for ${branch}: $($row.promotion_policy)"
    }
}
$unsupportedBranch = $branchStatus | Where-Object { $_.branch -eq "unsupported_active_heat_balance_branch" } | Select-Object -First 1
if (-not $unsupportedBranch) {
    throw "Missing unsupported active heat-balance branch policy row"
}
if ($unsupportedBranch.status -ne "blocked-if-active" -or $unsupportedBranch.promotion_policy -ne "block-conformance-promotion") {
    throw "Unexpected unsupported active branch policy: status=$($unsupportedBranch.status), promotion_policy=$($unsupportedBranch.promotion_policy)"
}
if ($summary.ctf_seed.policy -ne "all-eio") {
    throw "Expected all-EIO CTF seed policy, got $($summary.ctf_seed.policy)"
}
if ($summary.ctf_seed.included_coefficients -ne 10) {
    throw "Expected 10 included CTF coefficients, got $($summary.ctf_seed.included_coefficients)"
}
if ($summary.ctf_seed.skipped_coefficients -ne 0) {
    throw "Expected zero skipped CTF coefficients, got $($summary.ctf_seed.skipped_coefficients)"
}
if ($summary.construction_cache_entry_count -ne 3) {
    throw "Expected three construction cache entries, got $($summary.construction_cache_entry_count)"
}
if ($summary.construction_cache_eio_seeded_count -ne 3) {
    throw "Expected three EIO-seeded construction cache entries, got $($summary.construction_cache_eio_seeded_count)"
}
if ($summary.construction_cache_rust_generated_count -ne 0) {
    throw "Expected zero Rust-generated construction cache entries for all-EIO official candidate, got $($summary.construction_cache_rust_generated_count)"
}
if ([double]$summary.construction_cache_build_wall_seconds -lt 0.0) {
    throw "Construction cache build time must be non-negative"
}
if ("$($summary.construction_cache_hash)" -eq "0") {
    throw "Construction cache hash must be non-zero"
}
foreach ($construction in @("FLOOR", "R13WALL", "ROOF31")) {
    if (-not (@($summary.ctf_seed.included_constructions) -contains $construction)) {
        throw "Missing CTF seed construction: $construction"
    }
}
if (@($summary.ctf_component_first_samples).Count -ne 6) {
    throw "Expected six CTF component first-sample rows"
}
if (@($summary.ctf_history_first_sample_deltas).Count -ne 6) {
    throw "Expected six CTF history first-sample delta rows"
}
if (@($summary.ctf_history_series_deltas).Count -ne 6) {
    throw "Expected six CTF history series delta rows"
}
if (@($summary.ctf_history_run_period_initial_slots).Count -ne 10) {
    throw "Expected ten run-period initial CTF history slots"
}
if (@($summary.ctf_history_first_sample_slots).Count -ne 10) {
    throw "Expected ten first-sample CTF history slots"
}
if (@($summary.ctf_history_max_sample_slots).Count -ne 5) {
    throw "Expected five max-sample CTF history slots"
}
if (@($summary.ctf_history_max_sample_slots_after_advance).Count -ne 5) {
    throw "Expected five max-sample CTF history slots after advance"
}
$storageDelta = @($summary.ctf_storage_max_sample_deltas) | Select-Object -First 1
if (-not $storageDelta) {
    throw "Missing CTF storage max-sample delta row"
}
if ($storageDelta.key -ne "ZN001:FLR001" -or $storageDelta.dominant_storage_surface -ne $true) {
    throw "Expected FLR001 to be the dominant CTF storage surface"
}
if ($storageDelta.dominant_mismatch_source -ne "outside-history-total") {
    throw "Unexpected CTF storage dominant mismatch source: $($storageDelta.dominant_mismatch_source)"
}
$insideSourceTermSummaries = @($summary.inside_source_term_series_summaries)
if ($insideSourceTermSummaries.Count -ne 36) {
    throw "Expected 36 inside source-term summary rows, got $($insideSourceTermSummaries.Count)"
}
foreach ($term in @("inside-net-longwave", "radiant-internal-gain", "shortwave-absorbed", "additional-inside-heat-source", "radiant-hvac", "total-source")) {
    $rows = @($insideSourceTermSummaries | Where-Object { $_.term_name -eq $term })
    if ($rows.Count -ne 6) {
        throw "Expected six inside source-term rows for ${term}, got $($rows.Count)"
    }
}
foreach ($row in $insideSourceTermSummaries) {
    if ($row.samples -ne 8760) {
        throw "Expected 8760 samples for inside source term $($row.key)/$($row.term_name), got $($row.samples)"
    }
    if ([double]$row.area_residual_max_abs_w -gt 1.0e-8) {
        throw "Inside source rate/per-area residual too large for $($row.key)/$($row.term_name): $($row.area_residual_max_abs_w)"
    }
}
foreach ($term in @("radiant-internal-gain", "shortwave-absorbed", "additional-inside-heat-source", "radiant-hvac")) {
    foreach ($row in @($insideSourceTermSummaries | Where-Object { $_.term_name -eq $term })) {
        if ([double]$row.max_abs_w -gt 1.0e-9 -or [double]$row.max_abs_w_per_m2 -gt 1.0e-12) {
            throw "Official 1Zone expected zero $term source term for $($row.key), got max_abs_w=$($row.max_abs_w), max_abs_w_per_m2=$($row.max_abs_w_per_m2)"
        }
    }
}
$floorSourceTotal = @($insideSourceTermSummaries | Where-Object { $_.key -eq "ZN001:FLR001" -and $_.term_name -eq "total-source" }) | Select-Object -First 1
if (-not $floorSourceTotal) {
    throw "Missing floor total inside source-term summary"
}
if ([double]$floorSourceTotal.max_abs_w -le 0.0) {
    throw "Expected floor total inside source term to include nonzero longwave source"
}
$floorInsideSolve = @($summary.inside_solve_max_sample_deltas | Where-Object { $_.key -eq "ZN001:FLR001" }) | Select-Object -First 1
if (-not $floorInsideSolve) {
    throw "Missing floor inside solve max-sample source decomposition"
}
if ([double]$floorInsideSolve.tracked_solve_source_coverage_ratio -lt 0.98) {
    throw "Floor inside solve tracked source coverage too low: $($floorInsideSolve.tracked_solve_source_coverage_ratio)"
}
if ([math]::Abs([double]$floorInsideSolve.solve_source_residual_share) -gt 0.02) {
    throw "Floor inside solve source residual share too high: $($floorInsideSolve.solve_source_residual_share)"
}
if ([double]$floorInsideSolve.surface_temperature_sink_delta_w -le 0.0 -or [double]$floorInsideSolve.surface_temperature_sink_delta_w_per_m2 -le 0.0) {
    throw "Expected floor inside solve to report nonzero HConvInt*TSurf sink delta"
}
$floorFirstSurfaceTrace = @($summary.surface_first_sample_trace | Where-Object { $_.key -eq "ZN001:FLR001" }) | Select-Object -First 1
if (-not $floorFirstSurfaceTrace) {
    throw "Missing FLR001 first-sample surface trace"
}
foreach ($property in @("inside_net_surface_thermal_radiation_heat_gain_rate_per_area_w_per_m2", "inside_radiant_internal_gain_source_term_w", "inside_shortwave_absorbed_source_term_w", "inside_additional_heat_source_term_w", "inside_radiant_hvac_source_term_w", "inside_total_source_term_w")) {
    if (-not ($floorFirstSurfaceTrace.PSObject.Properties.Name -contains $property)) {
        throw "Missing first-sample inside source trace property: $property"
    }
}
foreach ($property in @("zone_mean_air_temperature_c", "inside_convection_input_inside_face_temperature_c", "inside_convection_input_reference_air_temperature_c", "inside_face_temperature_c")) {
    if (-not ($floorFirstSurfaceTrace.PSObject.Properties.Name -contains $property)) {
        throw "Missing first-sample inside hconv timing trace property: $property"
    }
}
foreach ($property in @("inside_convection_algorithm", "inside_convection_tarp_branch", "outside_convection_algorithm", "outside_convection_branch")) {
    if (-not ($floorFirstSurfaceTrace.PSObject.Properties.Name -contains $property)) {
        throw "Missing first-sample hconv branch trace property: $property"
    }
}
$roofFirstSurfaceTrace = @($summary.surface_first_sample_trace | Where-Object { $_.key -eq "ZN001:ROOF001" }) | Select-Object -First 1
$wallFirstSurfaceTrace = @($summary.surface_first_sample_trace | Where-Object { $_.key -eq "ZN001:WALL001" }) | Select-Object -First 1
if (-not $roofFirstSurfaceTrace -or -not $wallFirstSurfaceTrace) {
    throw "Expected first-sample hconv branch traces for roof and wall"
}
foreach ($trace in @($floorFirstSurfaceTrace, $roofFirstSurfaceTrace, $wallFirstSurfaceTrace)) {
    if ($trace.inside_convection_algorithm -ne "TARP") {
        throw "Expected TARP inside hconv algorithm for $($trace.key), got $($trace.inside_convection_algorithm)"
    }
    if ($trace.outside_convection_algorithm -ne "DOE-2") {
        throw "Expected DOE-2 outside hconv algorithm for $($trace.key), got $($trace.outside_convection_algorithm)"
    }
}
if (@("stable-horizontal-or-tilt", "unstable-horizontal-or-tilt") -notcontains $floorFirstSurfaceTrace.inside_convection_tarp_branch) {
    throw "Expected floor inside TARP branch to be horizontal/tilt, got $($floorFirstSurfaceTrace.inside_convection_tarp_branch)"
}
if (@("stable-horizontal-or-tilt", "unstable-horizontal-or-tilt") -notcontains $roofFirstSurfaceTrace.inside_convection_tarp_branch) {
    throw "Expected roof inside TARP branch to be horizontal/tilt, got $($roofFirstSurfaceTrace.inside_convection_tarp_branch)"
}
if ($wallFirstSurfaceTrace.inside_convection_tarp_branch -ne "vertical-wall") {
    throw "Expected wall inside TARP branch to be vertical-wall, got $($wallFirstSurfaceTrace.inside_convection_tarp_branch)"
}
if ($floorFirstSurfaceTrace.outside_convection_branch -ne "not-outdoors") {
    throw "Expected adiabatic floor outside hconv branch to be not-outdoors, got $($floorFirstSurfaceTrace.outside_convection_branch)"
}
if ($roofFirstSurfaceTrace.outside_convection_branch -ne "doe2-windward") {
    throw "Expected roof outside DOE-2 branch to be windward, got $($roofFirstSurfaceTrace.outside_convection_branch)"
}
if (@("doe2-windward", "doe2-leeward") -notcontains $wallFirstSurfaceTrace.outside_convection_branch) {
    throw "Expected wall outside DOE-2 branch to be windward/leeward, got $($wallFirstSurfaceTrace.outside_convection_branch)"
}
if ([math]::Abs([double]$floorFirstSurfaceTrace.inside_convection_input_reference_air_temperature_c - [double]$floorFirstSurfaceTrace.zone_mean_air_temperature_c) -gt 0.05) {
    throw "FLR001 inside hconv reference-air timing drift is unexpectedly large: ref=$($floorFirstSurfaceTrace.inside_convection_input_reference_air_temperature_c), MAT=$($floorFirstSurfaceTrace.zone_mean_air_temperature_c)"
}
$floorInsideCurrentDiagnostics = @($summary.floor_inside_current_diagnostics)
if ($floorInsideCurrentDiagnostics.Count -ne 1) {
    throw "Expected one FLR001 floor inside-current diagnostic row, got $($floorInsideCurrentDiagnostics.Count)"
}
$floorInsideCurrentDiagnostic = $floorInsideCurrentDiagnostics | Where-Object { $_.key -eq "ZN001:FLR001" } | Select-Object -First 1
if (-not $floorInsideCurrentDiagnostic) {
    throw "Missing FLR001 inside-current diagnostic row"
}
if ([int]$floorInsideCurrentDiagnostic.sample_index -ne [int]$storageDelta.sample_index) {
    throw "FLR001 inside-current diagnostic sample does not match storage max sample: diagnostic=$($floorInsideCurrentDiagnostic.sample_index), storage=$($storageDelta.sample_index)"
}
foreach ($property in @("ctf_inside_0_w_per_m2_k", "oracle_inside_face_temperature_c", "rust_inside_face_temperature_c", "oracle_reference_air_temperature_c", "rust_reference_air_temperature_c", "oracle_hconv_int_w_per_m2_k", "rust_hconv_int_w_per_m2_k", "oracle_inside_current_inside_term_w", "rust_inside_current_inside_term_w", "inside_current_inside_term_delta_w", "temperature_timing_coverage_ratio", "coefficient_delta_w_per_m2_k", "current_inside_mismatch_classification", "next_source_order_focus", "max_sample_source_terms")) {
    if (-not ($floorInsideCurrentDiagnostic.PSObject.Properties.Name -contains $property)) {
        throw "Missing FLR001 inside-current diagnostic property: $property"
    }
}
if ([double]$floorInsideCurrentDiagnostic.ctf_inside_0_w_per_m2_k -le 0.0) {
    throw "FLR001 CTFInside[0] must be positive in inside-current diagnostic"
}
if ([double]$floorInsideCurrentDiagnostic.inside_current_inside_term_delta_w -le 8.0) {
    throw "Expected FLR001 inside-current-inside term mismatch to remain visible, got $($floorInsideCurrentDiagnostic.inside_current_inside_term_delta_w)"
}
if ([double]$floorInsideCurrentDiagnostic.temperature_timing_coverage_ratio -lt 0.98) {
    throw "FLR001 inside-current mismatch is not explained by inside face temperature timing: coverage=$($floorInsideCurrentDiagnostic.temperature_timing_coverage_ratio)"
}
if ($floorInsideCurrentDiagnostic.current_inside_mismatch_classification -eq "coefficient") {
    throw "D3 expected FLR001 inside-current mismatch not to be classified as coefficient"
}
if ($floorInsideCurrentDiagnostic.current_inside_mismatch_classification -ne "temperature-timing") {
    throw "Unexpected FLR001 inside-current mismatch classification: $($floorInsideCurrentDiagnostic.current_inside_mismatch_classification)"
}
if ($floorInsideCurrentDiagnostic.next_source_order_focus -ne "UpdateThermalHistories/source-order") {
    throw "Unexpected FLR001 inside-current next source-order focus: $($floorInsideCurrentDiagnostic.next_source_order_focus)"
}
$floorInsideSourceTerms = @($floorInsideCurrentDiagnostic.max_sample_source_terms)
if ($floorInsideSourceTerms.Count -ne 6) {
    throw "Expected FLR001 max-sample all inside source terms, got $($floorInsideSourceTerms.Count)"
}
foreach ($term in @("inside-net-longwave", "radiant-internal-gain", "shortwave-absorbed", "additional-inside-heat-source", "radiant-hvac", "total-source")) {
    if (@($floorInsideSourceTerms | Where-Object { $_.term_name -eq $term }).Count -eq 0) {
        throw "Missing FLR001 max-sample inside source term: $term"
    }
}
$floorInsideCurrentTermSeries = @($summary.floor_inside_current_term_series)
if ($floorInsideCurrentTermSeries.Count -ne 1) {
    throw "Expected one FLR001 inside-current term series row, got $($floorInsideCurrentTermSeries.Count)"
}
$floorInsideCurrentTermSeriesRow = $floorInsideCurrentTermSeries | Where-Object { $_.key -eq "ZN001:FLR001" } | Select-Object -First 1
if (-not $floorInsideCurrentTermSeriesRow) {
    throw "Missing FLR001 inside-current term series row"
}
if ([int]$floorInsideCurrentTermSeriesRow.samples -ne 8760) {
    throw "Expected FLR001 inside-current term series to contain 8760 timesteps, got $($floorInsideCurrentTermSeriesRow.samples)"
}
if ([double]$floorInsideCurrentTermSeriesRow.max_abs_delta_w -le 8.0) {
    throw "Expected FLR001 inside-current term series max delta to remain visible, got $($floorInsideCurrentTermSeriesRow.max_abs_delta_w)"
}
if (@($floorInsideCurrentTermSeriesRow.sample_rows).Count -ne 8760) {
    throw "Expected FLR001 inside-current term summary JSON sample rows for every timestep"
}
if ($null -ne $summary.inside_hconv_reevaluation_interval) {
    throw "Official dynamic compatibility candidate should not enable an inside hconv diagnostic reevaluation interval"
}
$surfaceCoefficientDeltas = @($summary.zone_air_surface_coefficient_deltas)
if ($surfaceCoefficientDeltas.Count -ne 6) {
    throw "Expected six zone-air surface coefficient delta rows, got $($surfaceCoefficientDeltas.Count)"
}
foreach ($row in $surfaceCoefficientDeltas) {
    if ([int]$row.samples -ne 8760) {
        throw "Unexpected zone-air surface coefficient sample count for $($row.key): $($row.samples)"
    }
    if ([double]$row.inside_hconv_delta.rmse_delta_c -gt 0.0002) {
        throw "Inside hconv RMSE delta too high for $($row.key): $($row.inside_hconv_delta.rmse_delta_c)"
    }
    if ([double]$row.inside_hconv_delta.max_abs_delta_c -gt 0.003) {
        throw "Inside hconv max delta too high for $($row.key): $($row.inside_hconv_delta.max_abs_delta_c)"
    }
    if ([double]$row.reference_air_temperature_delta.rmse_delta_c -gt 0.0007) {
        throw "Inside hconv reference-air RMSE delta too high for $($row.key): $($row.reference_air_temperature_delta.rmse_delta_c)"
    }
    if ([double]$row.inside_convection_gain_delta.max_abs_delta_c -gt 0.07) {
        throw "Inside convection gain max delta too high for $($row.key): $($row.inside_convection_gain_delta.max_abs_delta_c)"
    }
}
$insideSolveSeriesDeltas = @($summary.inside_solve_series_deltas)
if ($insideSolveSeriesDeltas.Count -ne 6) {
    throw "Expected six inside solve series rows with outside boundary labels, got $($insideSolveSeriesDeltas.Count)"
}
$expectedOutsideBoundaryConditions = @{
    "ZN001:FLR001"  = "adiabatic"
    "ZN001:ROOF001" = "outdoors"
    "ZN001:WALL001" = "outdoors"
    "ZN001:WALL002" = "outdoors"
    "ZN001:WALL003" = "outdoors"
    "ZN001:WALL004" = "outdoors"
}
foreach ($entry in $expectedOutsideBoundaryConditions.GetEnumerator()) {
    $row = $insideSolveSeriesDeltas | Where-Object { $_.key -eq $entry.Key } | Select-Object -First 1
    if (-not $row) {
        throw "Missing inside solve boundary row for $($entry.Key)"
    }
    if ($row.outside_boundary_condition -ne $entry.Value) {
        throw "Unexpected outside boundary for $($entry.Key): $($row.outside_boundary_condition), expected $($entry.Value)"
    }
    if ([int]$row.samples -ne 8760) {
        throw "Unexpected inside solve sample count for $($entry.Key): $($row.samples)"
    }
}
$adiabaticHistoryRows = @($summary.adiabatic_history_max_sample_deltas)
if ($adiabaticHistoryRows.Count -ne 1 -or $adiabaticHistoryRows[0].key -ne "ZN001:FLR001") {
    throw "Expected one FLR001 adiabatic history diagnostic row"
}
if ([double]$adiabaticHistoryRows[0].outside_minus_inside_delta_c -gt 0.0001) {
    throw "Adiabatic outside-minus-inside delta unexpectedly large: $($adiabaticHistoryRows[0].outside_minus_inside_delta_c)"
}
function Assert-SeriesDelta {
    param(
        [Parameter(Mandatory = $true)]$Summary,
        [Parameter(Mandatory = $true)][string]$Key,
        [Parameter(Mandatory = $true)][string]$Variable,
        [Parameter(Mandatory = $true)][double]$MaxAbsLimit,
        [Parameter(Mandatory = $true)][double]$RmseLimit
    )
    $seriesRow = $Summary.series | Where-Object { $_.output.key -eq $Key -and $_.output.variable -eq $Variable -and $_.status -eq "extracted" } | Select-Object -First 1
    if (-not $seriesRow) {
        throw "Missing extracted series for $Key / $Variable"
    }
    if ([int]$seriesRow.samples -ne 8760) {
        throw "Unexpected sample count for $Key / $Variable`: $($seriesRow.samples)"
    }
    if ([double]$seriesRow.max_abs_delta_c -gt $MaxAbsLimit) {
        throw "Max delta too high for $Key / $Variable`: $($seriesRow.max_abs_delta_c)"
    }
    if ([double]$seriesRow.rmse_delta_c -gt $RmseLimit) {
        throw "RMSE delta too high for $Key / $Variable`: $($seriesRow.rmse_delta_c)"
    }
}
Assert-SeriesDelta -Summary $summary -Key "ZN001:ROOF001" -Variable "Surface Outside Face Outdoor Air Drybulb Temperature" -MaxAbsLimit 0.0000001 -RmseLimit 0.0000001
Assert-SeriesDelta -Summary $summary -Key "ZN001:ROOF001" -Variable "Surface Outside Face Outdoor Air Wetbulb Temperature" -MaxAbsLimit 0.00001 -RmseLimit 0.00001
Assert-SeriesDelta -Summary $summary -Key "ZN001:ROOF001" -Variable "Surface Outside Face Outdoor Air Wind Speed" -MaxAbsLimit 0.000000001 -RmseLimit 0.000000001
Assert-SeriesDelta -Summary $summary -Key "ZN001:ROOF001" -Variable "Surface Outside Face Outdoor Air Wind Direction" -MaxAbsLimit 0.000000001 -RmseLimit 0.000000001
Assert-SeriesDelta -Summary $summary -Key "ZN001:ROOF001" -Variable "Surface Outside Face Thermal Radiation to Sky Heat Transfer Coefficient" -MaxAbsLimit 0.00003 -RmseLimit 0.000004
Assert-SeriesDelta -Summary $summary -Key "ZN001:ROOF001" -Variable "Surface Outside Face Thermal Radiation to Ground Heat Transfer Coefficient" -MaxAbsLimit 0.000000001 -RmseLimit 0.000000001
Assert-SeriesDelta -Summary $summary -Key "ZN001:ROOF001" -Variable "Surface Outside Face Thermal Radiation to Air Heat Transfer Coefficient" -MaxAbsLimit 0.000000001 -RmseLimit 0.000000001
Assert-SeriesDelta -Summary $summary -Key "ZN001:ROOF001" -Variable "Surface Outside Face Convection Heat Transfer Coefficient" -MaxAbsLimit 0.0004 -RmseLimit 0.00001
Assert-SeriesDelta -Summary $summary -Key "ZN001:ROOF001" -Variable "Surface Outside Face Convection Heat Gain Rate" -MaxAbsLimit 2.0 -RmseLimit 0.3
Assert-SeriesDelta -Summary $summary -Key "ZN001:ROOF001" -Variable "Surface Outside Face Net Thermal Radiation Heat Gain Rate" -MaxAbsLimit 1.3 -RmseLimit 0.18
Assert-SeriesDelta -Summary $summary -Key "ZN001:ROOF001" -Variable "Surface Outside Face Solar Radiation Heat Gain Rate" -MaxAbsLimit 2.5 -RmseLimit 0.46
Assert-SeriesDelta -Summary $summary -Key "ZN001:ROOF001" -Variable "Surface Outside Face Conduction Heat Transfer Rate" -MaxAbsLimit 0.04 -RmseLimit 0.03
Assert-SeriesDelta -Summary $summary -Key "ZN001:FLR001" -Variable "Surface Outside Face Temperature" -MaxAbsLimit 0.001 -RmseLimit 0.0007
Assert-SeriesDelta -Summary $summary -Key "ZN001:FLR001" -Variable "Surface Outside Face Conduction Heat Transfer Rate" -MaxAbsLimit 0.32 -RmseLimit 0.02
Assert-SeriesDelta -Summary $summary -Key "ZONE ONE" -Variable "Zone Opaque Surface Outside Faces Conduction Rate" -MaxAbsLimit 0.31 -RmseLimit 0.04
Assert-SeriesDelta -Summary $summary -Key "ZONE ONE" -Variable "Zone Opaque Surface Outside Faces Conduction Heat Gain Rate" -MaxAbsLimit 0.41 -RmseLimit 0.04
Assert-SeriesDelta -Summary $summary -Key "ZONE ONE" -Variable "Zone Opaque Surface Outside Faces Conduction Heat Loss Rate" -MaxAbsLimit 0.18 -RmseLimit 0.03
$solarExposedSurfaceKeys = @("ZN001:WALL001", "ZN001:WALL002", "ZN001:WALL003", "ZN001:WALL004", "ZN001:ROOF001")
foreach ($surfaceKey in $solarExposedSurfaceKeys) {
    Assert-SeriesDelta -Summary $summary -Key $surfaceKey -Variable "Surface Outside Face Incident Solar Radiation Rate per Area" -MaxAbsLimit 0.02 -RmseLimit 0.003
    Assert-SeriesDelta -Summary $summary -Key $surfaceKey -Variable "Surface Outside Face Incident Beam Solar Radiation Rate per Area" -MaxAbsLimit 0.02 -RmseLimit 0.003
    Assert-SeriesDelta -Summary $summary -Key $surfaceKey -Variable "Surface Outside Face Incident Sky Diffuse Solar Radiation Rate per Area" -MaxAbsLimit 0.000000001 -RmseLimit 0.000000001
    Assert-SeriesDelta -Summary $summary -Key $surfaceKey -Variable "Surface Outside Face Incident Ground Diffuse Solar Radiation Rate per Area" -MaxAbsLimit 0.000000001 -RmseLimit 0.000000001
    Assert-SeriesDelta -Summary $summary -Key $surfaceKey -Variable "Surface Outside Face Solar Radiation Heat Gain Rate" -MaxAbsLimit 2.5 -RmseLimit 0.5
    Assert-SeriesDelta -Summary $summary -Key $surfaceKey -Variable "Surface Outside Face Net Thermal Radiation Heat Gain Rate" -MaxAbsLimit 1.3 -RmseLimit 0.6
}
Assert-SeriesDelta -Summary $summary -Key "ZN001:ROOF001" -Variable "Surface Outside Face Solar Radiation Heat Gain Rate per Area" -MaxAbsLimit 0.011 -RmseLimit 0.002
Assert-SeriesDelta -Summary $summary -Key "ZN001:ROOF001" -Variable "Surface Outside Face Net Thermal Radiation Heat Gain Rate per Area" -MaxAbsLimit 0.006 -RmseLimit 0.001
foreach ($surfaceKey in @("ZN001:WALL001", "ZN001:WALL002", "ZN001:WALL003", "ZN001:WALL004", "ZN001:FLR001", "ZN001:ROOF001")) {
    Assert-SeriesDelta -Summary $summary -Key $surfaceKey -Variable "Surface Inside Face Net Surface Thermal Radiation Heat Gain Rate" -MaxAbsLimit 0.4 -RmseLimit 0.03
}
$insideShortwaveSourceRows = @($summary.inside_source_term_series_summaries | Where-Object { $_.term_name -eq "shortwave-absorbed" })
if ($insideShortwaveSourceRows.Count -ne 6) {
    throw "Expected six inside shortwave absorbed source-term summary rows, got $($insideShortwaveSourceRows.Count)"
}
foreach ($row in $insideShortwaveSourceRows) {
    if ([int]$row.samples -ne 8760) {
        throw "Unexpected inside shortwave source sample count for $($row.key): $($row.samples)"
    }
    if ([double]$row.max_abs_w -gt 0.000000001 -or [double]$row.max_abs_w_per_m2 -gt 0.000000001) {
        throw "Expected zero inside shortwave absorbed source for no-window target case at $($row.key)"
    }
}
Assert-SeriesDelta -Summary $summary -Key "ZONE ONE" -Variable "Zone Air Heat Balance Internal Convective Heat Gain Rate" -MaxAbsLimit 0.000000001 -RmseLimit 0.000000001
foreach ($term in @("radiant-internal-gain", "shortwave-absorbed", "additional-inside-heat-source", "radiant-hvac")) {
    $rows = @($summary.inside_source_term_series_summaries | Where-Object { $_.term_name -eq $term })
    if ($rows.Count -ne 6) {
        throw "Expected six inside $term source-term summary rows, got $($rows.Count)"
    }
    foreach ($row in $rows) {
        if ([int]$row.samples -ne 8760) {
            throw "Unexpected inside $term source sample count for $($row.key): $($row.samples)"
        }
        if ([double]$row.max_abs_w -gt 0.000000001 -or [double]$row.max_abs_w_per_m2 -gt 0.000000001) {
            throw "Expected zero inside $term source for official 1Zone at $($row.key)"
        }
    }
}
if ($summary.heat_balance_warmup.enabled -ne $true) {
    throw "Official dynamic candidate must run model warmup"
}
if ($summary.heat_balance_warmup.building_minimum_days -ne 6) {
    throw "Unexpected Building warmup minimum days: $($summary.heat_balance_warmup.building_minimum_days)"
}
if ($summary.heat_balance_warmup.building_maximum_days -ne 30) {
    throw "Unexpected Building warmup maximum days: $($summary.heat_balance_warmup.building_maximum_days)"
}
if ([math]::Abs([double]$summary.heat_balance_warmup.building_temperature_convergence_tolerance_delta_c - 0.004) -gt 0.000000000001) {
    throw "Unexpected Building temperature convergence tolerance: $($summary.heat_balance_warmup.building_temperature_convergence_tolerance_delta_c)"
}
if ([math]::Abs([double]$summary.heat_balance_warmup.building_loads_convergence_tolerance_w - 0.04) -gt 0.000000000001) {
    throw "Unexpected Building loads convergence tolerance: $($summary.heat_balance_warmup.building_loads_convergence_tolerance_w)"
}
if ($summary.heat_balance_warmup.minimum_days -ne 20) {
    throw "Unexpected effective Rust warmup minimum days: $($summary.heat_balance_warmup.minimum_days)"
}
if ($summary.heat_balance_warmup.maximum_days -ne 30) {
    throw "Unexpected effective Rust warmup maximum days: $($summary.heat_balance_warmup.maximum_days)"
}
if ([math]::Abs([double]$summary.heat_balance_warmup.temperature_convergence_tolerance_delta_c - 0.004) -gt 0.000000000001) {
    throw "Unexpected effective Rust temperature convergence tolerance: $($summary.heat_balance_warmup.temperature_convergence_tolerance_delta_c)"
}
if ([math]::Abs([double]$summary.heat_balance_warmup.loads_convergence_tolerance_w - 0.04) -gt 0.000000000001) {
    throw "Unexpected effective Rust loads convergence tolerance: $($summary.heat_balance_warmup.loads_convergence_tolerance_w)"
}
if ($summary.heat_balance_warmup.day_count -ne 20) {
    throw "Unexpected Rust warmup day count: $($summary.heat_balance_warmup.day_count)"
}
if ($summary.heat_balance_warmup.timestep_count -ne 1920) {
    throw "Unexpected Rust warmup timestep count: $($summary.heat_balance_warmup.timestep_count)"
}
if ($summary.heat_balance_warmup.hours_per_day -ne 24) {
    throw "Unexpected Rust warmup hours_per_day: $($summary.heat_balance_warmup.hours_per_day)"
}
if ($summary.heat_balance_warmup.converged -ne $true) {
    throw "Rust warmup should report converged=true"
}
if ([double]$summary.heat_balance_warmup.final_max_zone_temperature_delta_c -gt [double]$summary.heat_balance_warmup.temperature_convergence_tolerance_delta_c) {
    throw "Rust warmup final MAT delta exceeds tolerance: $($summary.heat_balance_warmup.final_max_zone_temperature_delta_c)"
}
if ($summary.heat_balance_warmup.oracle_run_period_day_count -ne 20) {
    throw "Unexpected oracle warmup day count: $($summary.heat_balance_warmup.oracle_run_period_day_count)"
}
if ($summary.heat_balance_warmup.day_count_delta -ne 0) {
    throw "Warmup day count delta should be zero, got $($summary.heat_balance_warmup.day_count_delta)"
}
if ($summary.heat_balance_run_period_timesteps -ne 35040) {
    throw "Unexpected run-period timestep count: $($summary.heat_balance_run_period_timesteps)"
}
if ($summary.heat_balance_timesteps -ne ($summary.heat_balance_run_period_timesteps + $summary.heat_balance_warmup.timestep_count)) {
    throw "Total heat-balance timesteps must equal warmup plus run-period timesteps"
}
$zoneWarmupDayEndStates = @($summary.zone_air_warmup_day_end_states)
if ($zoneWarmupDayEndStates.Count -ne 20) {
    throw "Expected 20 warmup day-end zone-air trace rows, got $($zoneWarmupDayEndStates.Count)"
}
if ($zoneWarmupDayEndStates[0].day_index -ne 1 -or $zoneWarmupDayEndStates[-1].day_index -ne 20) {
    throw "Unexpected warmup day-end day indexes"
}
$lastWarmupMat = [double]$zoneWarmupDayEndStates[-1].state.mean_air_temperature_c
if ([double]::IsNaN($lastWarmupMat) -or [double]::IsInfinity($lastWarmupMat)) {
    throw "Warmup last-day MAT trace is not finite"
}
if ($null -eq $summary.warmup_end_state_deltas) {
    throw "Expected warmup_end_state_deltas in candidate summary"
}
if ([math]::Abs([double]$summary.warmup_end_state_deltas.mat_delta_c - [double]$summary.heat_balance_warmup.final_max_zone_temperature_delta_c) -gt 0.000000000001) {
    throw "Warmup MAT end-state delta should mirror heat_balance_warmup.final_max_zone_temperature_delta_c"
}
if ($null -eq $summary.warmup_end_state_deltas.surface_temperature) {
    throw "Expected warmup surface-temperature end-state delta"
}
if ($null -eq $summary.warmup_end_state_deltas.ctf_history) {
    throw "Expected warmup CTF-history end-state delta"
}
if ($null -eq $summary.warmup_end_state_deltas.zone_history) {
    throw "Expected warmup zone-history end-state delta"
}
if ($summary.warmup_end_state_deltas.ctf_history.sample_index -ne 0) {
    throw "Warmup CTF-history end-state delta should be tied to first run-period sample"
}
foreach ($blockerId in @("warmup-end-state-mat-delta", "warmup-end-state-surface-temperature-delta", "warmup-end-state-ctf-history-delta", "warmup-end-state-zone-history-delta")) {
    $blocker = @($summary.current_blockers | Where-Object { $_.blocker_id -eq $blockerId })
    if ($blocker.Count -lt 1) {
        throw "Expected current_blockers to include $blockerId"
    }
}
if ($summary.surface_iteration_count -ne 20) {
    throw "Unexpected surface_iteration_count: $($summary.surface_iteration_count)"
}
if ($summary.surface_loop_zone_air_correction -ne "after-surface-loop") {
    throw "Unexpected surface_loop_zone_air_correction: $($summary.surface_loop_zone_air_correction)"
}
if ($summary.ctf_initial_history_policy -ne "energyplus-surf-initial") {
    throw "Unexpected CTF history policy: $($summary.ctf_initial_history_policy)"
}
$zoneAirFirstTrace = @($summary.zone_air_first_sample_trace)
if ($zoneAirFirstTrace.Count -lt 1) {
    throw "Expected at least one zone-air first sample trace row"
}
$firstZoneAirTrace = $zoneAirFirstTrace[0]
foreach ($field in @("sum_mcp_w_per_k", "sum_mcp_t_w", "sum_sys_mcp_w_per_k", "sum_sys_mcp_t_w")) {
    if ([math]::Abs([double]$firstZoneAirTrace.$field) -gt 0.000000001) {
        throw "Expected ${field}=0 for uncontrolled 1Zone, got $($firstZoneAirTrace.$field)"
    }
}
if ([double]$firstZoneAirTrace.barometric_pressure_pa -le 1000.0) {
    throw "Invalid zone-air barometric pressure trace: $($firstZoneAirTrace.barometric_pressure_pa)"
}
if ([double]$firstZoneAirTrace.air_humidity_ratio -le 0.0) {
    throw "Invalid zone-air humidity ratio trace: $($firstZoneAirTrace.air_humidity_ratio)"
}
if ([double]$firstZoneAirTrace.cp_air_j_per_kg_k -le 1000.0) {
    throw "Invalid zone-air CpAir trace: $($firstZoneAirTrace.cp_air_j_per_kg_k)"
}
if ([double]$firstZoneAirTrace.rho_air_kg_per_m3 -le 0.0) {
    throw "Invalid zone-air rhoAir trace: $($firstZoneAirTrace.rho_air_kg_per_m3)"
}
if ($firstZoneAirTrace.use_zone_timestep_history -ne $true) {
    throw "Expected UseZoneTimeStepHistory=true for compatibility candidate"
}
if ($firstZoneAirTrace.shorten_timestep_sys -ne $false) {
    throw "Expected first-sample ShortenTimeStepSys=false for official 1Zone"
}
if ([math]::Abs([double]$firstZoneAirTrace.prior_timestep_seconds - 900.0) -gt 0.000000001) {
    throw "Expected first-sample PriorTimeStep=900 seconds, got $($firstZoneAirTrace.prior_timestep_seconds)"
}
$zoneAirCoefficientDeltas = @($summary.zone_air_coefficient_deltas)
if ($zoneAirCoefficientDeltas.Count -ne 1) {
    throw "Expected one zone-air coefficient delta row, got $($zoneAirCoefficientDeltas.Count)"
}
$zoneAirCoefficientDelta = $zoneAirCoefficientDeltas[0]
if ($zoneAirCoefficientDelta.first_divergence_source -ne "SumHATsurf") {
    throw "Unexpected first zone-air coefficient divergence source: $($zoneAirCoefficientDelta.first_divergence_source)"
}
if ([double]$zoneAirCoefficientDelta.air_power_cap_delta.max_abs_delta_c -gt 0.000000001) {
    throw "AirPowerCap delta should be exact-zero in the official candidate"
}

$conformanceOutputs = @($summary.outputs | Where-Object { $_.level -eq "conformance" })
$diagnosticOutputs = @($summary.outputs | Where-Object { $_.level -eq "diagnostic" })
if ($conformanceOutputs.Count -ne 128) {
    throw "Expected 128 conformance-level outputs, got $($conformanceOutputs.Count)"
}
if ($diagnosticOutputs.Count -ne 0) {
    throw "Expected zero diagnostic outputs, got $($diagnosticOutputs.Count)"
}
if (-not ($summary.series | Where-Object { $_.output.key -eq "Environment" -and $_.output.variable -eq "Site Outdoor Air Drybulb Temperature" -and $_.output.level -eq "conformance" -and $_.status -eq "extracted" })) {
    throw "Weather dry-bulb conformance series missing"
}
$wetbulbSeries = $summary.series | Where-Object { $_.output.key -eq "Environment" -and $_.output.variable -eq "Site Outdoor Air Wetbulb Temperature" -and $_.output.level -eq "conformance" -and $_.status -eq "extracted" } | Select-Object -First 1
if (-not $wetbulbSeries) {
    throw "Weather wet-bulb conformance series missing"
}
if ([double]$wetbulbSeries.max_abs_delta_c -gt 0.00001) {
    throw "Weather wet-bulb max_abs_delta_c exceeds 1e-5 C: $($wetbulbSeries.max_abs_delta_c)"
}
if ([double]$wetbulbSeries.rmse_delta_c -gt 0.00001) {
    throw "Weather wet-bulb rmse_delta_c exceeds 1e-5 C: $($wetbulbSeries.rmse_delta_c)"
}
foreach ($weatherVariable in @("Site Sky Temperature", "Site Horizontal Infrared Radiation Rate per Area")) {
    $weatherSeries = $summary.series | Where-Object { $_.output.key -eq "Environment" -and $_.output.variable -eq $weatherVariable -and $_.output.class -eq "weather" -and $_.output.level -eq "conformance" -and $_.status -eq "extracted" } | Select-Object -First 1
    if (-not $weatherSeries) {
        throw "Weather conformance series missing: $weatherVariable"
    }
    if ([double]$weatherSeries.max_abs_delta_c -gt 0.00001) {
        throw "Weather max_abs_delta_c exceeds 1e-5 for ${weatherVariable}: $($weatherSeries.max_abs_delta_c)"
    }
    if ([double]$weatherSeries.rmse_delta_c -gt 0.00001) {
        throw "Weather rmse_delta_c exceeds 1e-5 for ${weatherVariable}: $($weatherSeries.rmse_delta_c)"
    }
}
$rainSeries = $summary.series | Where-Object { $_.output.key -eq "Environment" -and $_.output.variable -eq "Site Rain Status" -and $_.output.level -eq "conformance" -and $_.status -eq "extracted" } | Select-Object -First 1
if (-not $rainSeries) {
    throw "Weather rain-status conformance series missing"
}
if ([double]$rainSeries.max_abs_delta_c -gt 0.000001) {
    throw "Weather rain-status max_abs_delta_c exceeds 1e-6: $($rainSeries.max_abs_delta_c)"
}
if ([double]$rainSeries.rmse_delta_c -gt 0.000001) {
    throw "Weather rain-status rmse_delta_c exceeds 1e-6: $($rainSeries.rmse_delta_c)"
}
foreach ($surfaceWeatherVariable in @(
    "Surface Outside Face Outdoor Air Drybulb Temperature",
    "Surface Outside Face Outdoor Air Wetbulb Temperature",
    "Surface Outside Face Outdoor Air Wind Speed",
    "Surface Outside Face Outdoor Air Wind Direction"
)) {
    $surfaceWeatherSeries = $summary.series | Where-Object { $_.output.key -eq "ZN001:ROOF001" -and $_.output.variable -eq $surfaceWeatherVariable -and $_.output.class -eq "weather" -and $_.output.level -eq "conformance" -and $_.status -eq "extracted" } | Select-Object -First 1
    if (-not $surfaceWeatherSeries) {
        throw "Roof surface-local weather conformance series missing: $surfaceWeatherVariable"
    }
    if ([double]$surfaceWeatherSeries.max_abs_delta_c -gt 0.00001) {
        throw "Roof surface-local weather max_abs_delta_c exceeds 1e-5 for ${surfaceWeatherVariable}: $($surfaceWeatherSeries.max_abs_delta_c)"
    }
    if ([double]$surfaceWeatherSeries.rmse_delta_c -gt 0.00001) {
        throw "Roof surface-local weather rmse_delta_c exceeds 1e-5 for ${surfaceWeatherVariable}: $($surfaceWeatherSeries.rmse_delta_c)"
    }
}
if (-not ($summary.series | Where-Object { $_.output.key -eq "ZONE ONE" -and $_.output.variable -eq "Zone Mean Air Temperature" -and $_.output.level -eq "conformance" -and $_.status -eq "extracted" })) {
    throw "Zone Mean Air Temperature conformance series missing"
}
$airStorageSeries = $summary.series | Where-Object { $_.output.key -eq "ZONE ONE" -and $_.output.variable -eq "Zone Air Heat Balance Air Energy Storage Rate" -and $_.output.class -eq "zone-state" -and $_.output.level -eq "conformance" -and $_.status -eq "extracted" } | Select-Object -First 1
if (-not $airStorageSeries) {
    throw "Zone Air Heat Balance Air Energy Storage Rate conformance series missing"
}
if ([double]$airStorageSeries.max_abs_delta_c -gt 0.08) {
    throw "Zone air storage max_abs_delta exceeds 0.08 W: $($airStorageSeries.max_abs_delta_c)"
}
if ([double]$airStorageSeries.rmse_delta_c -gt 0.006) {
    throw "Zone air storage rmse_delta exceeds 0.006 W: $($airStorageSeries.rmse_delta_c)"
}
$surfaceConvectionSeries = $summary.series | Where-Object { $_.output.key -eq "ZONE ONE" -and $_.output.variable -eq "Zone Air Heat Balance Surface Convection Rate" -and $_.output.class -eq "zone-state" -and $_.output.level -eq "conformance" -and $_.status -eq "extracted" } | Select-Object -First 1
if (-not $surfaceConvectionSeries) {
    throw "Zone Air Heat Balance Surface Convection Rate conformance series missing"
}
if ([double]$surfaceConvectionSeries.max_abs_delta_c -gt 0.09) {
    throw "Zone surface convection max_abs_delta exceeds 0.09 W: $($surfaceConvectionSeries.max_abs_delta_c)"
}
if ([double]$surfaceConvectionSeries.rmse_delta_c -gt 0.006) {
    throw "Zone surface convection rmse_delta exceeds 0.006 W: $($surfaceConvectionSeries.rmse_delta_c)"
}
$humiditySeries = $summary.series | Where-Object { $_.output.key -eq "ZONE ONE" -and $_.output.variable -eq "Zone Mean Air Humidity Ratio" -and $_.output.class -eq "zone-state" -and $_.output.level -eq "conformance" -and $_.status -eq "extracted" } | Select-Object -First 1
if (-not $humiditySeries) {
    throw "Zone Mean Air Humidity Ratio conformance series missing"
}
if ([double]$humiditySeries.max_abs_delta_c -gt 0.000001) {
    throw "Zone humidity ratio max_abs_delta exceeds 1e-6 kgWater/kgDryAir: $($humiditySeries.max_abs_delta_c)"
}
if ([double]$humiditySeries.rmse_delta_c -gt 0.000001) {
    throw "Zone humidity ratio rmse_delta exceeds 1e-6 kgWater/kgDryAir: $($humiditySeries.rmse_delta_c)"
}
$outdoorAirTransferSeries = $summary.series | Where-Object { $_.output.key -eq "ZONE ONE" -and $_.output.variable -eq "Zone Air Heat Balance Outdoor Air Transfer Rate" -and $_.output.class -eq "zone-state" -and $_.output.level -eq "conformance" -and $_.status -eq "extracted" } | Select-Object -First 1
if (-not $outdoorAirTransferSeries) {
    throw "Zone Air Heat Balance Outdoor Air Transfer Rate conformance series missing"
}
if ([double]$outdoorAirTransferSeries.max_abs_delta_c -gt 0.000001) {
    throw "Outdoor air transfer max_abs_delta exceeds 1e-6 W: $($outdoorAirTransferSeries.max_abs_delta_c)"
}
if ([double]$outdoorAirTransferSeries.rmse_delta_c -gt 0.000001) {
    throw "Outdoor air transfer rmse_delta exceeds 1e-6 W: $($outdoorAirTransferSeries.rmse_delta_c)"
}
$allSurfaceKeys = @("ZN001:WALL001", "ZN001:WALL002", "ZN001:WALL003", "ZN001:WALL004", "ZN001:FLR001", "ZN001:ROOF001")
$wallRoofSurfaceKeys = @("ZN001:WALL001", "ZN001:WALL002", "ZN001:WALL003", "ZN001:WALL004", "ZN001:ROOF001")
$adjacentAirSeries = @($summary.series | Where-Object {
        $allSurfaceKeys -contains $_.output.key `
            -and $_.output.variable -eq "Surface Inside Face Adjacent Air Temperature" `
            -and $_.output.class -eq "surface-state" `
            -and $_.output.level -eq "conformance" `
            -and $_.status -eq "extracted"
    })
if ($adjacentAirSeries.Count -ne 6) {
    throw "Expected six inside adjacent-air temperature conformance series, got $($adjacentAirSeries.Count)"
}
foreach ($series in $adjacentAirSeries) {
    if ([double]$series.max_abs_delta_c -gt 0.01) {
        throw "Adjacent-air temperature max_abs_delta_c exceeds 0.01 C for $($series.output.key): $($series.max_abs_delta_c)"
    }
    if ([double]$series.rmse_delta_c -gt 0.01) {
        throw "Adjacent-air temperature rmse_delta_c exceeds 0.01 C for $($series.output.key): $($series.rmse_delta_c)"
    }
}
$surfaceFluxSeries = @($summary.series | Where-Object { $_.output.class -eq "surface-flux-state" -and $_.output.level -eq "conformance" -and $_.status -eq "extracted" })
if ($surfaceFluxSeries.Count -ne 22) {
    throw "Expected 22 surface-flux-state conformance series, got $($surfaceFluxSeries.Count)"
}
foreach ($series in $surfaceFluxSeries) {
    if ([double]$series.max_abs_delta_c -gt 0.005) {
        throw "Surface flux-state max_abs_delta_c exceeds 0.005 W/m2 for $($series.output.key) / $($series.output.variable): $($series.max_abs_delta_c)"
    }
    if ([double]$series.rmse_delta_c -gt 0.0015) {
        throw "Surface flux-state rmse_delta_c exceeds 0.0015 W/m2 for $($series.output.key) / $($series.output.variable): $($series.rmse_delta_c)"
    }
}
$incidentSolarVariables = @(
    "Surface Outside Face Incident Solar Radiation Rate per Area",
    "Surface Outside Face Incident Beam Solar Radiation Rate per Area"
)
$incidentSolarSeries = @($summary.series | Where-Object {
        $wallRoofSurfaceKeys -contains $_.output.key `
            -and $incidentSolarVariables -contains $_.output.variable `
            -and $_.output.class -eq "surface-solar-flux-state" `
            -and $_.output.level -eq "conformance" `
            -and $_.status -eq "extracted"
    })
if ($incidentSolarSeries.Count -ne 10) {
    throw "Expected 10 named wall/roof incident total/beam solar conformance series, got $($incidentSolarSeries.Count)"
}
foreach ($series in $incidentSolarSeries) {
    if ([double]$series.max_abs_delta_c -gt 0.02) {
        throw "Incident total/beam solar max_abs_delta_c exceeds 0.02 W/m2 for $($series.output.key) / $($series.output.variable): $($series.max_abs_delta_c)"
    }
    if ([double]$series.rmse_delta_c -gt 0.003) {
        throw "Incident total/beam solar rmse_delta_c exceeds 0.003 W/m2 for $($series.output.key) / $($series.output.variable): $($series.rmse_delta_c)"
    }
}
$absorbedSolarRateSeries = @($summary.series | Where-Object {
        $wallRoofSurfaceKeys -contains $_.output.key `
            -and $_.output.variable -eq "Surface Outside Face Solar Radiation Heat Gain Rate" `
            -and $_.output.class -eq "surface-solar-rate-state" `
            -and $_.output.level -eq "conformance" `
            -and $_.status -eq "extracted"
    })
if ($absorbedSolarRateSeries.Count -ne 5) {
    throw "Expected five named wall/roof absorbed solar heat gain rate conformance series, got $($absorbedSolarRateSeries.Count)"
}
foreach ($series in $absorbedSolarRateSeries) {
    if ([double]$series.max_abs_delta_c -gt 2.5) {
        throw "Absorbed solar heat gain rate max_abs_delta_c exceeds 2.5 W for $($series.output.key): $($series.max_abs_delta_c)"
    }
    if ([double]$series.rmse_delta_c -gt 0.5) {
        throw "Absorbed solar heat gain rate rmse_delta_c exceeds 0.5 W for $($series.output.key): $($series.rmse_delta_c)"
    }
}
$absorbedSolarFluxSeries = @($summary.series | Where-Object {
        $_.output.key -eq "ZN001:ROOF001" `
            -and $_.output.variable -eq "Surface Outside Face Solar Radiation Heat Gain Rate per Area" `
            -and $_.output.class -eq "surface-solar-flux-state" `
            -and $_.output.level -eq "conformance" `
            -and $_.status -eq "extracted"
    })
if ($absorbedSolarFluxSeries.Count -ne 1) {
    throw "Expected one roof absorbed solar heat gain per-area conformance series, got $($absorbedSolarFluxSeries.Count)"
}
foreach ($series in $absorbedSolarFluxSeries) {
    if ([double]$series.max_abs_delta_c -gt 0.02) {
        throw "Roof absorbed solar heat gain per-area max_abs_delta_c exceeds 0.02 W/m2: $($series.max_abs_delta_c)"
    }
    if ([double]$series.rmse_delta_c -gt 0.003) {
        throw "Roof absorbed solar heat gain per-area rmse_delta_c exceeds 0.003 W/m2: $($series.rmse_delta_c)"
    }
}
$incidentDiffuseVariables = @(
    "Surface Outside Face Incident Sky Diffuse Solar Radiation Rate per Area",
    "Surface Outside Face Incident Ground Diffuse Solar Radiation Rate per Area"
)
$incidentDiffuseSeries = @($summary.series | Where-Object {
        $wallRoofSurfaceKeys -contains $_.output.key `
            -and $incidentDiffuseVariables -contains $_.output.variable `
            -and $_.output.class -eq "surface-flux-state" `
            -and $_.output.level -eq "conformance" `
            -and $_.status -eq "extracted"
    })
if ($incidentDiffuseSeries.Count -ne 10) {
    throw "Expected 10 named wall/roof incident sky/ground diffuse conformance series, got $($incidentDiffuseSeries.Count)"
}
foreach ($series in $incidentDiffuseSeries) {
    if ([double]$series.max_abs_delta_c -gt 0.000001) {
        throw "Incident sky/ground diffuse max_abs_delta_c exceeds 1e-6 W/m2 for $($series.output.key) / $($series.output.variable): $($series.max_abs_delta_c)"
    }
    if ([double]$series.rmse_delta_c -gt 0.000001) {
        throw "Incident sky/ground diffuse rmse_delta_c exceeds 1e-6 W/m2 for $($series.output.key) / $($series.output.variable): $($series.rmse_delta_c)"
    }
}
$surfaceCoefficientSeries = @($summary.series | Where-Object {
        (
            ($allSurfaceKeys -contains $_.output.key -and $_.output.variable -eq "Surface Inside Face Convection Heat Transfer Coefficient") `
                -or ($wallRoofSurfaceKeys -contains $_.output.key -and $_.output.variable -eq "Surface Outside Face Convection Heat Transfer Coefficient")
        ) `
            -and $_.output.class -eq "surface-coefficient-state" `
            -and $_.output.level -eq "conformance" `
            -and $_.status -eq "extracted"
    })
if ($surfaceCoefficientSeries.Count -ne 11) {
    throw "Expected 11 inside/outside convection coefficient conformance series, got $($surfaceCoefficientSeries.Count)"
}
foreach ($series in $surfaceCoefficientSeries) {
    if ([double]$series.max_abs_delta_c -gt 0.05) {
        throw "Surface convection coefficient max_abs_delta_c exceeds 0.05 W/m2-K for $($series.output.key) / $($series.output.variable): $($series.max_abs_delta_c)"
    }
    if ([double]$series.rmse_delta_c -gt 0.001) {
        throw "Surface convection coefficient rmse_delta_c exceeds 0.001 W/m2-K for $($series.output.key) / $($series.output.variable): $($series.rmse_delta_c)"
    }
}
$longwaveCoefficientVariables = @(
    "Surface Outside Face Thermal Radiation to Air Heat Transfer Coefficient",
    "Surface Outside Face Thermal Radiation to Sky Heat Transfer Coefficient",
    "Surface Outside Face Thermal Radiation to Ground Heat Transfer Coefficient"
)
$longwaveCoefficientSeries = @($summary.series | Where-Object {
        $_.output.key -eq "ZN001:ROOF001" `
            -and $longwaveCoefficientVariables -contains $_.output.variable `
            -and $_.output.class -eq "surface-coefficient-state" `
            -and $_.output.level -eq "conformance" `
            -and $_.status -eq "extracted"
    })
if ($longwaveCoefficientSeries.Count -ne 3) {
    throw "Expected three roof outside longwave coefficient conformance series, got $($longwaveCoefficientSeries.Count)"
}
foreach ($series in $longwaveCoefficientSeries) {
    if ([double]$series.max_abs_delta_c -gt 0.0001) {
        throw "Roof longwave coefficient max_abs_delta_c exceeds 1e-4 W/m2-K for $($series.output.variable): $($series.max_abs_delta_c)"
    }
    if ([double]$series.rmse_delta_c -gt 0.00001) {
        throw "Roof longwave coefficient rmse_delta_c exceeds 1e-5 W/m2-K for $($series.output.variable): $($series.rmse_delta_c)"
    }
}
$exteriorSourceVariables = @(
    "Surface Outside Face Convection Heat Gain Rate",
    "Surface Outside Face Net Thermal Radiation Heat Gain Rate"
)
$exteriorSourceFluxVariables = @(
    "Surface Outside Face Convection Heat Gain Rate per Area",
    "Surface Outside Face Net Thermal Radiation Heat Gain Rate per Area"
)
$surfaceExteriorRateSeries = @($summary.series | Where-Object {
        $wallRoofSurfaceKeys -contains $_.output.key `
            -and $exteriorSourceVariables -contains $_.output.variable `
            -and $_.output.class -eq "surface-exterior-rate-state" `
            -and $_.output.level -eq "conformance" `
            -and $_.status -eq "extracted"
    })
if ($surfaceExteriorRateSeries.Count -ne 10) {
    throw "Expected 10 wall/roof exterior convection/radiation rate conformance series, got $($surfaceExteriorRateSeries.Count)"
}
foreach ($series in $surfaceExteriorRateSeries) {
    if ([double]$series.max_abs_delta_c -gt 2.5) {
        throw "Exterior convection/radiation rate max_abs_delta_c exceeds 2.5 W for $($series.output.key) / $($series.output.variable): $($series.max_abs_delta_c)"
    }
    if ([double]$series.rmse_delta_c -gt 0.6) {
        throw "Exterior convection/radiation rate rmse_delta_c exceeds 0.6 W for $($series.output.key) / $($series.output.variable): $($series.rmse_delta_c)"
    }
}
$surfaceExteriorFluxSeries = @($summary.series | Where-Object {
        $_.output.key -eq "ZN001:ROOF001" `
            -and $exteriorSourceFluxVariables -contains $_.output.variable `
            -and $_.output.class -eq "surface-exterior-flux-state" `
            -and $_.output.level -eq "conformance" `
            -and $_.status -eq "extracted"
    })
if ($surfaceExteriorFluxSeries.Count -ne 2) {
    throw "Expected two roof exterior convection/radiation per-area conformance series, got $($surfaceExteriorFluxSeries.Count)"
}
foreach ($series in $surfaceExteriorFluxSeries) {
    if ([double]$series.max_abs_delta_c -gt 0.011) {
        throw "Roof exterior convection/radiation per-area max_abs_delta_c exceeds 0.011 W/m2 for $($series.output.variable): $($series.max_abs_delta_c)"
    }
    if ([double]$series.rmse_delta_c -gt 0.002) {
        throw "Roof exterior convection/radiation per-area rmse_delta_c exceeds 0.002 W/m2 for $($series.output.variable): $($series.rmse_delta_c)"
    }
}
$insideSourceVariables = @(
    "Surface Inside Face Convection Heat Gain Rate",
    "Surface Inside Face Net Surface Thermal Radiation Heat Gain Rate"
)
$insideSourceSeries = @($summary.series | Where-Object {
        $allSurfaceKeys -contains $_.output.key `
            -and $insideSourceVariables -contains $_.output.variable `
            -and $_.output.class -eq "surface-state" `
            -and $_.output.level -eq "conformance" `
            -and $_.status -eq "extracted"
    })
if ($insideSourceSeries.Count -ne 12) {
    throw "Expected 12 inside convection/radiation source conformance series, got $($insideSourceSeries.Count)"
}
foreach ($series in $insideSourceSeries) {
    if ([double]$series.max_abs_delta_c -gt 1.0) {
        throw "Inside convection/radiation source max_abs_delta_c exceeds 1.0 W for $($series.output.key) / $($series.output.variable): $($series.max_abs_delta_c)"
    }
    if ([double]$series.rmse_delta_c -gt 0.35) {
        throw "Inside convection/radiation source rmse_delta_c exceeds 0.35 W for $($series.output.key) / $($series.output.variable): $($series.rmse_delta_c)"
    }
}
$surfaceAggregateSeries = @($summary.series | Where-Object { $_.output.class -eq "surface-aggregate-state" -and $_.output.level -eq "conformance" -and $_.status -eq "extracted" })
if ($surfaceAggregateSeries.Count -ne 4) {
    throw "Expected four surface-aggregate-state conformance series, got $($surfaceAggregateSeries.Count)"
}
foreach ($series in $surfaceAggregateSeries) {
    if ([double]$series.max_abs_delta_c -gt 1.2) {
        throw "Zone opaque aggregate conduction max_abs_delta_c exceeds 1.2 W for $($series.output.key) / $($series.output.variable): $($series.max_abs_delta_c)"
    }
    if ([double]$series.rmse_delta_c -gt 0.2) {
        throw "Zone opaque aggregate conduction rmse_delta_c exceeds 0.2 W for $($series.output.key) / $($series.output.variable): $($series.rmse_delta_c)"
    }
}
$storageSeries = $summary.series | Where-Object { $_.output.key -eq "ZN001:FLR001" -and $_.output.variable -eq "Surface Heat Storage Rate" -and $_.output.class -eq "surface-storage-state" -and $_.output.level -eq "conformance" -and $_.status -eq "extracted" } | Select-Object -First 1
if (-not $storageSeries) {
    throw "Floor storage conformance series missing"
}
if ([double]$storageSeries.max_abs_delta_c -gt 1.2) {
    throw "Floor storage max_abs_delta_c exceeds 1.2 W: $($storageSeries.max_abs_delta_c)"
}
if ([double]$storageSeries.rmse_delta_c -gt 0.35) {
    throw "Floor storage rmse_delta_c exceeds 0.35 W: $($storageSeries.rmse_delta_c)"
}
$storageFluxSeries = $summary.series | Where-Object { $_.output.key -eq "ZN001:FLR001" -and $_.output.variable -eq "Surface Heat Storage Rate per Area" -and $_.output.class -eq "surface-storage-flux-state" -and $_.output.level -eq "conformance" -and $_.status -eq "extracted" } | Select-Object -First 1
if (-not $storageFluxSeries) {
    throw "Floor storage per-area conformance series missing"
}
if ([double]$storageFluxSeries.max_abs_delta_c -gt 0.005) {
    throw "Floor storage per-area max_abs_delta_c exceeds 0.005 W/m2: $($storageFluxSeries.max_abs_delta_c)"
}
if ([double]$storageFluxSeries.rmse_delta_c -gt 0.001) {
    throw "Floor storage per-area rmse_delta_c exceeds 0.001 W/m2: $($storageFluxSeries.rmse_delta_c)"
}

$iterationCountSeries = $summary.series | Where-Object { $_.output.key -eq "Simulation" -and $_.output.variable -eq "Surface Inside Face Heat Balance Calculation Iteration Count" -and $_.output.class -eq "surface-iteration-count-state" -and $_.output.level -eq "conformance" -and $_.status -eq "extracted" } | Select-Object -First 1
if (-not $iterationCountSeries) {
    throw "Surface iteration count conformance series missing"
}
if ([double]$iterationCountSeries.max_abs_delta_c -gt 1.0) {
    throw "Surface iteration count max_abs_delta exceeds 1 count: $($iterationCountSeries.max_abs_delta_c)"
}
if ([double]$iterationCountSeries.rmse_delta_c -gt 0.2) {
    throw "Surface iteration count rmse_delta exceeds 0.2 count: $($iterationCountSeries.rmse_delta_c)"
}

$reportText = Get-Content -LiteralPath $reportPath -Raw
$eioText = Get-Content -LiteralPath $eioPath -Raw
$iddText = Get-Content -LiteralPath $iddPath -Raw
$sourceIdfText = Get-Content -LiteralPath $sourceIdfPath -Raw
$candidateCaseText = Get-Content -LiteralPath $CasePath -Raw
$diagnosticCasePath = Join-Path $RepoRoot "data\conformance_cases\official_1zone_uncontrolled_dynamic_diagnostic_001\case.toml"
$diagnosticCaseText = Get-Content -LiteralPath $diagnosticCasePath -Raw
$insideConvectionSourcePath = Join-Path $RepoRoot "crates\ep_runtime\src\heat_balance\convection.rs"
$insideConvectionSourceText = Get-Content -LiteralPath $insideConvectionSourcePath -Raw
$mainSourcePath = Join-Path $RepoRoot "crates\ep_cli\src\main.rs"
$mainSourceText = Get-Content -LiteralPath $mainSourcePath -Raw
$algorithmSourcePath = Join-Path $RepoRoot "crates\ep_runtime\src\heat_balance\algorithm.rs"
$algorithmSourceText = Get-Content -LiteralPath $algorithmSourcePath -Raw
$diagnosticProbeSourcePath = Join-Path $RepoRoot "crates\ep_runtime\src\diagnostic_probes\heat_balance.rs"
$diagnosticProbeSourceText = Get-Content -LiteralPath $diagnosticProbeSourcePath -Raw
$resultStoreSourcePath = Join-Path $RepoRoot "crates\ep_runtime\src\output\result_store.rs"
$resultStoreSourceText = Get-Content -LiteralPath $resultStoreSourcePath -Raw
$outputSourcePath = Join-Path $RepoRoot "crates\ep_runtime\src\output.rs"
$outputSourceText = Get-Content -LiteralPath $outputSourcePath -Raw
$meterRegistrySourcePath = Join-Path $RepoRoot "crates\ep_runtime\src\output\meter_registry.rs"
$meterRegistrySourceText = Get-Content -LiteralPath $meterRegistrySourcePath -Raw
$outputDiagnosticsSourcePath = Join-Path $RepoRoot "crates\ep_runtime\src\output\diagnostics.rs"
$outputDiagnosticsSourceText = Get-Content -LiteralPath $outputDiagnosticsSourcePath -Raw
$readmePath = Join-Path $RepoRoot "README.md"
$readmeText = Get-Content -LiteralPath $readmePath -Raw
$currentStatusPath = Join-Path $RepoRoot "docs\src\current\current-status.md"
$currentStatusText = Get-Content -LiteralPath $currentStatusPath -Raw
$compilerSourcePath = Join-Path $RepoRoot "crates\ep_compiler\src\compiler.rs"
$compilerSourceText = Get-Content -LiteralPath $compilerSourcePath -Raw
$idsSourcePath = Join-Path $RepoRoot "crates\ep_model\src\ids.rs"
$idsSourceText = Get-Content -LiteralPath $idsSourcePath -Raw
$modelSourcePath = Join-Path $RepoRoot "crates\ep_model\src\model.rs"
$modelSourceText = Get-Content -LiteralPath $modelSourcePath -Raw
$nodeStateSourcePath = Join-Path $RepoRoot "crates\ep_runtime\src\node\state.rs"
$nodeStateSourceText = Get-Content -LiteralPath $nodeStateSourcePath -Raw
$nodeProjectionSourcePath = Join-Path $RepoRoot "crates\ep_runtime\src\node\projection.rs"
$nodeProjectionSourceText = Get-Content -LiteralPath $nodeProjectionSourcePath -Raw
$runtimeStateSourcePath = Join-Path $RepoRoot "crates\ep_runtime\src\heat_balance\state.rs"
$runtimeStateSourceText = Get-Content -LiteralPath $runtimeStateSourcePath -Raw
$warmupSourcePath = Join-Path $RepoRoot "crates\ep_runtime\src\heat_balance\warmup.rs"
$warmupSourceText = Get-Content -LiteralPath $warmupSourcePath -Raw
$surfaceLoopSourcePath = Join-Path $RepoRoot "crates\ep_runtime\src\heat_balance\surface_loop.rs"
$surfaceLoopSourceText = Get-Content -LiteralPath $surfaceLoopSourcePath -Raw
$surfaceManagerSourcePath = Join-Path $RepoRoot "crates\ep_runtime\src\heat_balance\surface_manager.rs"
$surfaceManagerSourceText = Get-Content -LiteralPath $surfaceManagerSourcePath -Raw
$timestepSourcePath = Join-Path $RepoRoot "crates\ep_runtime\src\heat_balance\timestep.rs"
$timestepSourceText = Get-Content -LiteralPath $timestepSourcePath -Raw
$surfaceObjectsSourcePath = Join-Path $RepoRoot "crates\ep_model\src\objects\surfaces.rs"
$surfaceObjectsSourceText = Get-Content -LiteralPath $surfaceObjectsSourcePath -Raw
$surfaceBoundarySourcePath = Join-Path $RepoRoot "crates\ep_runtime\src\heat_balance\surface_boundary.rs"
$surfaceBoundarySourceText = Get-Content -LiteralPath $surfaceBoundarySourcePath -Raw
$surfaceBalanceSourcePath = Join-Path $RepoRoot "crates\ep_runtime\src\heat_balance\surface_balance.rs"
$surfaceBalanceSourceText = Get-Content -LiteralPath $surfaceBalanceSourcePath -Raw
$ctfSourcePath = Join-Path $RepoRoot "crates\ep_runtime\src\heat_balance\ctf.rs"
$ctfSourceText = Get-Content -LiteralPath $ctfSourcePath -Raw
$surfaceWeatherSourcePath = Join-Path $RepoRoot "crates\ep_runtime\src\heat_balance\surface_weather.rs"
$surfaceWeatherSourceText = Get-Content -LiteralPath $surfaceWeatherSourcePath -Raw
$solarSourcePath = Join-Path $RepoRoot "crates\ep_runtime\src\heat_balance\solar.rs"
$solarSourceText = Get-Content -LiteralPath $solarSourcePath -Raw
$timeAxisSourcePath = Join-Path $RepoRoot "crates\ep_runtime\src\time_axis.rs"
$timeAxisSourceText = Get-Content -LiteralPath $timeAxisSourcePath -Raw
$weatherSourcePath = Join-Path $RepoRoot "crates\ep_runtime\src\weather.rs"
$weatherSourceText = Get-Content -LiteralPath $weatherSourcePath -Raw
$runtimeSourcePath = Join-Path $RepoRoot "crates\ep_runtime\src\runtime.rs"
$runtimeSourceText = Get-Content -LiteralPath $runtimeSourcePath -Raw
$runPeriodSourcePath = Join-Path $RepoRoot "crates\ep_runtime\src\heat_balance\run_period.rs"
$runPeriodSourceText = Get-Content -LiteralPath $runPeriodSourcePath -Raw
$radiationSourcePath = Join-Path $RepoRoot "crates\ep_runtime\src\heat_balance\radiation.rs"
$radiationSourceText = Get-Content -LiteralPath $radiationSourcePath -Raw
$longwaveSourcePath = Join-Path $RepoRoot "crates\ep_runtime\src\heat_balance\longwave.rs"
$longwaveSourceText = Get-Content -LiteralPath $longwaveSourcePath -Raw
$internalGainsSourcePath = Join-Path $RepoRoot "crates\ep_model\src\objects\internal_gains.rs"
$internalGainsSourceText = Get-Content -LiteralPath $internalGainsSourcePath -Raw
$schedulesSourcePath = Join-Path $RepoRoot "crates\ep_runtime\src\schedules.rs"
$schedulesSourceText = Get-Content -LiteralPath $schedulesSourcePath -Raw
$executionPlanSourcePath = Join-Path $RepoRoot "crates\ep_runtime\src\execution_plan.rs"
$executionPlanSourceText = Get-Content -LiteralPath $executionPlanSourcePath -Raw
$pipelineSourcePath = Join-Path $RepoRoot "crates\ep_run\src\pipeline.rs"
$pipelineSourceText = Get-Content -LiteralPath $pipelineSourcePath -Raw
$runConfigSourcePath = Join-Path $RepoRoot "crates\ep_run\src\config.rs"
$runConfigSourceText = Get-Content -LiteralPath $runConfigSourcePath -Raw
$runDiagnosticsSourcePath = Join-Path $RepoRoot "crates\ep_run\src\diagnostics.rs"
$runDiagnosticsSourceText = Get-Content -LiteralPath $runDiagnosticsSourcePath -Raw
$plantObjectsSourcePath = Join-Path $RepoRoot "crates\ep_model\src\objects\plant.rs"
$plantObjectsSourceText = Get-Content -LiteralPath $plantObjectsSourcePath -Raw
$hvacObjectsSourcePath = Join-Path $RepoRoot "crates\ep_model\src\objects\hvac.rs"
$hvacObjectsSourceText = Get-Content -LiteralPath $hvacObjectsSourcePath -Raw
$runtimeHvacSourcePath = Join-Path $RepoRoot "crates\ep_runtime\src\hvac.rs"
$runtimeHvacSourceText = Get-Content -LiteralPath $runtimeHvacSourcePath -Raw
$plantStateSourcePath = Join-Path $RepoRoot "crates\ep_runtime\src\plant\state.rs"
$plantStateSourceText = Get-Content -LiteralPath $plantStateSourcePath -Raw
$airloopFanFixturePath = Join-Path $RepoRoot "data\testcases\minimal\airloop-fan-only.epJSON"
$airloopFanFixtureText = Get-Content -LiteralPath $airloopFanFixturePath -Raw
$airloopFanCasePath = Join-Path $RepoRoot "data\conformance_cases\airloop_fan_only_diagnostic_001\case.toml"
$airloopFanCaseText = Get-Content -LiteralPath $airloopFanCasePath -Raw
$airloopFanSmokePath = Join-Path $RepoRoot "scripts\smoke\airloop-fan-only-diagnostic-smoke.ps1"
$airloopFanSmokeText = Get-Content -LiteralPath $airloopFanSmokePath -Raw
$airloopCoilFixturePath = Join-Path $RepoRoot "data\testcases\minimal\airloop-coil-only.epJSON"
$airloopCoilFixtureText = Get-Content -LiteralPath $airloopCoilFixturePath -Raw
$airloopCoilCasePath = Join-Path $RepoRoot "data\conformance_cases\airloop_coil_only_diagnostic_001\case.toml"
$airloopCoilCaseText = Get-Content -LiteralPath $airloopCoilCasePath -Raw
$ptacFixturePath = Join-Path $RepoRoot "data\testcases\minimal\ptac-diagnostic.epJSON"
$ptacFixtureText = Get-Content -LiteralPath $ptacFixturePath -Raw
$ptacCasePath = Join-Path $RepoRoot "data\conformance_cases\ptac_diagnostic_001\case.toml"
$ptacCaseText = Get-Content -LiteralPath $ptacCasePath -Raw
$airloop5ZoneFixturePath = Join-Path $RepoRoot "data\testcases\minimal\5zone-aircooled-diagnostic.epJSON"
$airloop5ZoneFixtureText = Get-Content -LiteralPath $airloop5ZoneFixturePath -Raw
$airloop5ZoneCasePath = Join-Path $RepoRoot "data\conformance_cases\airloop_5zone_aircooled_diagnostic_001\case.toml"
$airloop5ZoneCaseText = Get-Content -LiteralPath $airloop5ZoneCasePath -Raw
$airloopDiagnosticSmokePath = Join-Path $RepoRoot "scripts\smoke\airloop-diagnostic-fixtures-smoke.ps1"
$airloopDiagnosticSmokeText = Get-Content -LiteralPath $airloopDiagnosticSmokePath -Raw
$plantLoopFixturePath = Join-Path $RepoRoot "data\testcases\minimal\plant-loop-skeleton.epJSON"
$plantLoopFixtureText = Get-Content -LiteralPath $plantLoopFixturePath -Raw
$plantLoopCasePath = Join-Path $RepoRoot "data\conformance_cases\plant_loop_diagnostic_001\case.toml"
$plantLoopCaseText = Get-Content -LiteralPath $plantLoopCasePath -Raw
$plantLoopSmokePath = Join-Path $RepoRoot "scripts\smoke\plant-loop-skeleton-smoke.ps1"
$plantLoopSmokeText = Get-Content -LiteralPath $plantLoopSmokePath -Raw
$plantProjectionSmokePath = Join-Path $RepoRoot "scripts\smoke\plant-loop-projection-smoke.ps1"
$plantProjectionSmokeText = Get-Content -LiteralPath $plantProjectionSmokePath -Raw
$devCommandsPath = Join-Path $RepoRoot "scripts\dev\commands.json"
$devCommandsText = Get-Content -LiteralPath $devCommandsPath -Raw
Assert-Contains -Text $reportText -Pattern "Heat Balance Conformance Report" -Description "markdown report header"
Assert-Contains -Text $reportText -Pattern "comparison_class: conformance" -Description "markdown comparison class"
Assert-Contains -Text $reportText -Pattern "conformance_claim: true" -Description "markdown conformance claim"
Assert-Contains -Text $reportText -Pattern "gate_blocking: true" -Description "markdown blocking gate"
Assert-Contains -Text $reportText -Pattern "## EnergyPlus Compatibility Stage Order" -Description "D2 markdown source-order stage checklist"
Assert-Contains -Text $reportText -Pattern "ManageSurfaceHeatBalance" -Description "D2 markdown surface manager source-order stage"
Assert-Contains -Text $reportText -Pattern "CalcHeatBalanceOutsideSurf" -Description "D2 markdown outside surface source-order stage"
Assert-Contains -Text $reportText -Pattern "CalcHeatBalanceInsideSurf" -Description "D2 markdown inside surface source-order stage"
Assert-Contains -Text $reportText -Pattern "ManageAirHeatBalance" -Description "D2 markdown air manager source-order stage"
Assert-Contains -Text $reportText -Pattern "ManageZoneAirUpdates" -Description "D2 markdown zone air update source-order stage"
Assert-Contains -Text $reportText -Pattern "UpdateThermalHistories" -Description "D2 markdown thermal history source-order stage"
Assert-Contains -Text $reportText -Pattern "ReportSurfaceHeatBalance" -Description "D2 markdown surface report source-order stage"
Assert-Contains -Text $reportText -Pattern "## Heat-Balance Branch Status" -Description "D2 markdown active branch policy table"
Assert-Contains -Text $reportText -Pattern "unsupported_active_heat_balance_branch | blocked-if-active | block-conformance-promotion" -Description "D2 markdown unsupported active branch blocker"
Assert-Contains -Text $reportText -Pattern "| key | variable | class | sample_count | oracle_count | rust_count | store_type | timestamp_match |" -Description "A15 compare-report sample count columns"
Assert-Contains -Text $reportText -Pattern "| Environment | Site Outdoor Air Drybulb Temperature | weather | 8760 | 8760 | 8760 | average/average | true |" -Description "A15 compare-report every-variable count/store/timestamp row"
Assert-Contains -Text $reportText -Pattern "| output | sample | index | oracle_value | rust_value | delta | tolerance | status |" -Description "A15 compare-report delta sample oracle/rust/tolerance/status columns"
Assert-Contains -Text $reportText -Pattern "| output | index | oracle_value | rust_value | delta | tolerance | status |" -Description "A15 compare-report hourly sample oracle/rust/tolerance/status columns"
Assert-Contains -Text $mainSourceText -Pattern "fn heat_balance_report_tolerance_label" -Description "A15 report attaches class tolerance per output row"
Assert-Contains -Text $runPeriodSourceText -Pattern "for substep in 1..=steps" -Description "A15 hourly output samples aggregate zone timesteps"
Assert-Contains -Text $runPeriodSourceText -Pattern "let divisor = f64::from(steps)" -Description "A15 hourly average divisor from zone timestep count"
Assert-Contains -Text $resultStoreSourceText -Pattern "pub enum OutputStoreType" -Description "Rust ResultStore output store type enum"
Assert-Contains -Text $resultStoreSourceText -Pattern "OutputStoreType::Average" -Description "Rust ResultStore average store type"
Assert-Contains -Text $resultStoreSourceText -Pattern "OutputStoreType::Sum" -Description "Rust ResultStore sum store type"
Assert-Contains -Text $resultStoreSourceText -Pattern "units == `"j`"" -Description "energy variables use sum store type by J units"
Assert-Contains -Text $resultStoreSourceText -Pattern "variable.ends_with(`" energy`")" -Description "energy variables use sum store type by Energy suffix"
Assert-Contains -Text $mainSourceText -Pattern "sample count mismatch: oracle_count=" -Description "A15 sample count mismatch hard fail"
Assert-Contains -Text $mainSourceText -Pattern "timestamp mismatch: oracle_first=" -Description "A15 timestamp mismatch hard fail"
Assert-Contains -Text $mainSourceText -Pattern "first reported sample timestamp is not hour-ending" -Description "A15 first timestamp hour-ending hard fail"
Assert-Contains -Text $algorithmSourceText -Pattern "pub enum CompatibilityHeatBalanceAlgorithm" -Description "B1 compatibility heat-balance algorithm layer"
Assert-Contains -Text $algorithmSourceText -Pattern "pub enum HeatBalanceZoneAirSelection" -Description "B1 typed heat-balance algorithm selection"
Assert-Contains -Text $algorithmSourceText -Pattern "pub enum HeatBalanceAlgorithmLane" -Description "B1 heat-balance algorithm lane enum"
Assert-Contains -Text $algorithmSourceText -Pattern "DiagnosticOnly" -Description "B1 diagnostic-only default lane"
Assert-Contains -Text $algorithmSourceText -Pattern "DiagnosticProbe" -Description "B1 diagnostic-probe lane"
Assert-Contains -Text $algorithmSourceText -Pattern "allows_conformance_promotion" -Description "B1 lane promotion guard"
Assert-Contains -Text $algorithmSourceText -Pattern "EnergyPlusHeatBalanceCompatCandidate" -Description "D2 named compatibility candidate lane"
Assert-Contains -Text $algorithmSourceText -Pattern "CompatibilitySourceOrder" -Description "D2 source-order compatibility lane enum"
Assert-Contains -Text $mainSourceText -Pattern "if official_dynamic_candidate" -Description "D2 official dynamic candidate branch"
Assert-Contains -Text $mainSourceText -Pattern "HeatBalanceZoneAirAlgorithm::EnergyPlusHeatBalanceCompatCandidate" -Description "D2 candidate forces source-order algorithm"
Assert-Contains -Text $mainSourceText -Pattern "official dynamic conformance candidate requires source-order compatibility lane" -Description "D2 simplified shell promotion blocker"
Assert-Contains -Text $mainSourceText -Pattern "conformance claim cannot use diagnostic probe lane" -Description "D2 diagnostic probe promotion blocker"
Assert-Contains -Text $ctfSourceText -Pattern "surface_ctf_inside_current_inside_term_rate_w_from_sources" -Description "D3 named inside-current-inside CTF source equation"
Assert-Contains -Text $mainSourceText -Pattern "HeatBalanceFloorInsideCurrentDiagnostic" -Description "D3 FLR001 inside-current diagnostic row"
Assert-Contains -Text $mainSourceText -Pattern "heat_balance_floor_inside_current_term_series" -Description "D3 FLR001 inside-current timestep trace"
Assert-Contains -Text $mainSourceText -Pattern "temperature-timing" -Description "D3 inside-current mismatch classification"
Assert-Contains -Text $diagnosticProbeSourceText -Pattern "pub enum DiagnosticHeatBalanceProbe" -Description "B1 diagnostic probe namespace enum"
Assert-Contains -Text $diagnosticProbeSourceText -Pattern "pub struct DiagnosticProbeMetadata" -Description "B1 diagnostic probe metadata"
Assert-Contains -Text $diagnosticProbeSourceText -Pattern 'name: stringify!($variant)' -Description "B1 diagnostic probe name metadata"
Assert-Contains -Text $diagnosticProbeSourceText -Pattern "purpose:" -Description "B1 diagnostic probe purpose metadata"
Assert-Contains -Text $diagnosticProbeSourceText -Pattern "expected_bottleneck:" -Description "B1 diagnostic probe expected bottleneck metadata"
Assert-Contains -Text $readmeText -Pattern "compatibility from diagnostic probes, fast mode, experimental mode" -Description "B1 README excludes diagnostic probes from conformance"
Assert-Contains -Text $currentStatusText -Pattern 'diagnostic probe enum lives under `diagnostic_probes`' -Description "B1 current status diagnostic probe boundary"
Assert-Contains -Text $pipelineSourceText -Pattern "load_epjson_file(&prepared_input.converted_epjson_path)" -Description "B2 IDF/epJSON parsing ends in RawModel stage"
Assert-Contains -Text $pipelineSourceText -Pattern "compile_raw_model(&raw_model)" -Description "B2 RawModel to TypedModel compile boundary"
Assert-Contains -Text $pipelineSourceText -Pattern "SimulationModel::from_typed" -Description "B2 SimulationModel graph build boundary"
Assert-Contains -Text $modelSourceText -Pattern "pub struct SimulationModel" -Description "B2 SimulationModel typed payload"
Assert-Contains -Text $modelSourceText -Pattern "pub struct ModelGraph" -Description "B2 fixed model graph"
Assert-Contains -Text $modelSourceText -Pattern "zone_surfaces" -Description "B2 zone-surface graph edges"
Assert-Contains -Text $modelSourceText -Pattern "construction_materials" -Description "B2 construction-material graph edges"
Assert-Contains -Text $executionPlanSourceText -Pattern "pub struct ExecutionStageDependency" -Description "B2 per-stage dependency contract"
Assert-Contains -Text $executionPlanSourceText -Pattern "pub struct ExecutionStagePreboundSet" -Description "B2 stage prebound set"
Assert-Contains -Text $executionPlanSourceText -Pattern "pub struct ExecutionPlanRuntimePolicy" -Description "B2 runtime lookup policy"
Assert-Contains -Text $executionPlanSourceText -Pattern "post_typed_model_object_lookup" -Description "B2 RawModel/TypedModel object lookup ban"
Assert-Contains -Text $executionPlanSourceText -Pattern "stage_execution_string_comparison" -Description "B2 stage string comparison policy"
Assert-Contains -Text $executionPlanSourceText -Pattern "stage_execution_hash_map_lookup" -Description "B2 stage HashMap lookup policy"
Assert-Contains -Text $executionPlanSourceText -Pattern "compatibility_plan_order" -Description "B2 deterministic compatibility plan order policy"
Assert-Contains -Text $executionPlanSourceText -Pattern "fast_mode_grouping_policy" -Description "B2 fast-mode-only grouping policy"
Assert-Contains -Text $executionPlanSourceText -Pattern "EmsBeginZoneTimestepBeforeInitHeatBalance" -Description "B2 EMS begin callback barrier before InitHeatBalance"
Assert-Contains -Text $executionPlanSourceText -Pattern "EmsEndZoneTimestepAfterZoneReporting" -Description "B2 EMS end callback barrier after zone reporting"
Assert-Contains -Text $executionPlanSourceText -Pattern "is_source_order_barrier" -Description "B2 source-order barrier marker"
Assert-Contains -Text $executionPlanSourceText -Pattern "compile_stage_contracts" -Description "B2 execution plan contract precompile"
Assert-Contains -Text $executionPlanSourceText -Pattern "stage_prebound_set" -Description "B2 per-stage prebound set compiler"
Assert-Contains -Text $executionPlanSourceText -Pattern "surface_ids" -Description "B2 surface loop target prebind"
Assert-Contains -Text $executionPlanSourceText -Pattern "zone_ids" -Description "B2 zone loop target prebind"
Assert-Contains -Text $executionPlanSourceText -Pattern "construction_ids" -Description "B2 construction coefficient reference prebind"
Assert-Contains -Text $executionPlanSourceText -Pattern "schedule_ids" -Description "B2 schedule ID prebind"
Assert-Contains -Text $executionPlanSourceText -Pattern "weather_series_indices" -Description "B2 weather series index prebind"
Assert-Contains -Text $executionPlanSourceText -Pattern "output_handles" -Description "B2 output handle prebind"
Assert-Contains -Text $pipelineSourceText -Pattern '"runtime_lookup_policy"' -Description "B2 execution-plan JSON runtime policy"
Assert-Contains -Text $pipelineSourceText -Pattern '"fast_mode_grouping_policy"' -Description "B2 execution-plan JSON fast grouping policy"
Assert-Contains -Text $pipelineSourceText -Pattern '"prebound_summary"' -Description "B2 execution-plan JSON prebound summary"
Assert-Contains -Text $pipelineSourceText -Pattern '"dependencies"' -Description "B2 execution-plan JSON stage dependencies"
Assert-Contains -Text $pipelineSourceText -Pattern '"prebound"' -Description "B2 execution-plan JSON stage prebound IDs"
Assert-Contains -Text $pipelineSourceText -Pattern '"source_order_barrier"' -Description "B2 execution-plan JSON source-order barrier marker"
Assert-Contains -Text $pipelineSourceText -Pattern "source_order_stage_state_snapshots" -Description "B2 diagnostic stage snapshot export"
Assert-Contains -Text $runtimeStateSourceText -Pattern "pub surfaces: Vec<SurfaceHeatBalanceState>" -Description "B3 Vec-based surface runtime state"
Assert-Contains -Text $runtimeStateSourceText -Pattern "SurfaceId" -Description "B3 compact SurfaceId state"
Assert-Contains -Text $runtimeStateSourceText -Pattern "surfaces_by_zone" -Description "B3 surfaces_by_zone compile index"
Assert-Contains -Text $runtimeStateSourceText -Pattern "opaque_surfaces_by_zone" -Description "B3 opaque surfaces by zone index"
Assert-Contains -Text $runtimeStateSourceText -Pattern "fenestration_surfaces_by_zone" -Description "B3 fenestration surfaces by zone index"
Assert-Contains -Text $runtimeStateSourceText -Pattern "fenestration_surfaces" -Description "B3 fenestration surface index"
Assert-Contains -Text $runtimeStateSourceText -Pattern "exterior_surfaces" -Description "B3 exterior surface index"
Assert-Contains -Text $runtimeStateSourceText -Pattern "ground_surfaces" -Description "B3 ground surface index"
Assert-Contains -Text $runtimeStateSourceText -Pattern "adiabatic_surfaces" -Description "B3 adiabatic surface index"
Assert-Contains -Text $runtimeStateSourceText -Pattern "interzone_surfaces" -Description "B3 interzone surface index"
Assert-Contains -Text $runtimeStateSourceText -Pattern "surfaces_by_construction" -Description "B3 surfaces by construction index"
Assert-Contains -Text $runtimeStateSourceText -Pattern "output_requested_surfaces" -Description "B3 requested output surface index"
Assert-Contains -Text $runtimeStateSourceText -Pattern "ctf_surfaces" -Description "B3 CTF surface index"
Assert-Contains -Text $runtimeStateSourceText -Pattern "no_mass_surfaces" -Description "B3 no-mass surface index"
Assert-Contains -Text $surfaceLoopSourceText -Pattern "ctf_surface_indices().iter().copied()" -Description "B3 inside balance loop uses prebound CTF indices"
Assert-NotContains -Text $surfaceLoopSourceText -Pattern ".surface_name ==" -Description "B3 surface loop string name comparison"
Assert-Contains -Text $pipelineSourceText -Pattern '"output_handles": precomputed.output_registry.len()' -Description "B3 output/report loop handle prebind summary"
Assert-Contains -Text $surfaceManagerSourceText -Pattern "pub(crate) struct ConstructionThermalData" -Description "B4 construction thermal data cache entry"
Assert-Contains -Text $surfaceManagerSourceText -Pattern "pub(crate) struct ConstructionThermalDataCache" -Description "B4 construction thermal data cache"
Assert-Contains -Text $surfaceManagerSourceText -Pattern "material_layer_stack" -Description "B4 flattened material layer stack"
Assert-Contains -Text $surfaceManagerSourceText -Pattern "material_thermal_resistance_sum" -Description "B4 precomputed material resistance sum"
Assert-Contains -Text $surfaceManagerSourceText -Pattern "material_heat_capacity_per_area_sum" -Description "B4 precomputed areal heat capacity"
Assert-Contains -Text $surfaceManagerSourceText -Pattern "ctf_coefficients" -Description "B4 CTF coefficients in construction cache"
Assert-Contains -Text $surfaceManagerSourceText -Pattern "no_mass_construction_ids" -Description "B4 no-mass construction cache"
Assert-Contains -Text $surfaceManagerSourceText -Pattern "massive_ctf_construction_ids" -Description "B4 massive CTF construction cache"
Assert-Contains -Text $runtimeStateSourceText -Pattern "construction_thermal_data_index" -Description "B4 surface state construction cache reference"
Assert-Contains -Text $runtimeStateSourceText -Pattern "construction_cache_hash" -Description "B4 construction cache hash in runtime state"
Assert-Contains -Text $surfaceManagerSourceText -Pattern "pub enum ConstructionCtfCoefficientSource" -Description "B4 CTF coefficient source enum"
Assert-Contains -Text $surfaceManagerSourceText -Pattern "EnergyPlusEioSeeded" -Description "B4 EnergyPlus EIO-seeded coefficient source"
Assert-Contains -Text $surfaceManagerSourceText -Pattern "RustGeneratedSteady" -Description "B4 Rust-generated coefficient source"
Assert-Contains -Text $surfaceManagerSourceText -Pattern "build_wall_seconds" -Description "B4 coefficient cache build time profile"
Assert-Contains -Text $surfaceManagerSourceText -Pattern "ConstructionCacheInvalidationToken" -Description "B4 construction cache invalidation token"
Assert-Contains -Text $surfaceManagerSourceText -Pattern "invalidation_token" -Description "B4 construction cache invalidation token hook"
Assert-Contains -Text $surfaceManagerSourceText -Pattern "is_invalidated_by" -Description "B4 construction cache invalidation check"
Assert-Contains -Text $mainSourceText -Pattern "construction_cache_hash" -Description "B4 construction cache hash in report JSON"
Assert-Contains -Text $reportText -Pattern "construction_cache_hash:" -Description "B4 construction cache hash in markdown"
Assert-NotContains -Text $timestepSourceText -Pattern "surface_thermal_properties" -Description "B4 timestep coefficient recomputation"
Assert-Contains -Text $pipelineSourceText -Pattern "let weather_records = load_epw_records(weather_path)" -Description "B5 EPW parse in runtime setup"
Assert-Contains -Text $pipelineSourceText -Pattern "precompute_weather_timestep_series(" -Description "B5 weather timestep precompute in setup"
Assert-Contains -Text $pipelineSourceText -Pattern "weather_series: Option<WeatherTimestepSeries>" -Description "B5 prepared weather series"
Assert-Contains -Text $pipelineSourceText -Pattern "schedule_series: Vec<ScheduleValueSeries>" -Description "B5 prepared schedule series"
Assert-Contains -Text $pipelineSourceText -Pattern "precompute_schedule_value_series_for_time_axis" -Description "B5 schedule compile from TimeAxis"
Assert-Contains -Text $pipelineSourceText -Pattern "simulate_heat_balance_zone_air_temperatures_with_weather_series" -Description "B5 runtime uses precomputed weather series"
Assert-Contains -Text $weatherSourceText -Pattern "pub struct WeatherTimestepSeries" -Description "B5 WeatherTimestepSeries"
Assert-Contains -Text $weatherSourceText -Pattern "timestep_dry_bulb_c" -Description "B5 dry-bulb vector"
Assert-Contains -Text $weatherSourceText -Pattern "timestep_wet_bulb_c" -Description "B5 wet-bulb vector"
Assert-Contains -Text $weatherSourceText -Pattern "timestep_relative_humidity_percent" -Description "B5 RH vector"
Assert-Contains -Text $weatherSourceText -Pattern "timestep_atmospheric_pressure_pa" -Description "B5 pressure vector"
Assert-Contains -Text $weatherSourceText -Pattern "timestep_wind_speed_m_per_s" -Description "B5 wind vector"
Assert-Contains -Text $weatherSourceText -Pattern "timestep_global_horizontal_radiation_w_per_m2" -Description "B5 solar vector"
Assert-Contains -Text $weatherSourceText -Pattern "sample_for(&self, record_index: usize, timestep: u32)" -Description "B5 weather sample indexed access"
Assert-Contains -Text $timeAxisSourceText -Pattern "first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues" -Description "B5 TimeAxis first-hour interpolation policy"
Assert-Contains -Text $schedulesSourceText -Pattern "pub enum ScheduleSeriesKind" -Description "B5 schedule series kind"
Assert-Contains -Text $schedulesSourceText -Pattern "ConstantScalar" -Description "B5 Schedule:Constant scalar fast path"
Assert-Contains -Text $schedulesSourceText -Pattern "CompactIntervals" -Description "B5 Schedule:Compact interval fast path"
Assert-Contains -Text $schedulesSourceText -Pattern "pub struct CompiledScheduleInterval" -Description "B5 compiled compact schedule interval"
Assert-Contains -Text $schedulesSourceText -Pattern "precompile_compact_schedule_intervals" -Description "B5 compact schedule precompiled intervals"
Assert-Contains -Text $runtimeSourceText -Pattern "simulate_heat_balance_zone_air_temperatures_with_weather_series" -Description "B5 runtime weather-series entry point"
Assert-Contains -Text $warmupSourceText -Pattern "weather_series: Option<&WeatherTimestepSeries>" -Description "B5 warmup consumes weather series"
Assert-Contains -Text $runPeriodSourceText -Pattern "weather_series: Option<&WeatherTimestepSeries>" -Description "B5 run-period consumes weather series"
Assert-NotContains -Text $runtimeSourceText -Pattern "load_epw_records" -Description "B5 runtime EPW row parsing"
Assert-NotContains -Text $timestepSourceText -Pattern "ScheduleCompact" -Description "B5 timestep Schedule:Compact parsing"
Assert-Contains -Text $outputSourceText -Pattern "pub struct OutputVariableSpec" -Description "B6 output variable spec"
Assert-Contains -Text $outputSourceText -Pattern "RuntimeOutputRegistry" -Description "B6 output registry"
Assert-Contains -Text $outputSourceText -Pattern "handle: OutputHandle(self.outputs.len() as u32)" -Description "B6 u32 output handle index"
Assert-Contains -Text $resultStoreSourceText -Pattern "pub fn write_output_by_handle" -Description "B6 handle-index output write"
Assert-Contains -Text $resultStoreSourceText -Pattern "pub struct ResultStore" -Description "B6 typed result store"
Assert-Contains -Text $resultStoreSourceText -Pattern "pub series: Vec<OutputSeries>" -Description "B6 typed output series store"
Assert-Contains -Text $outputSourceText -Pattern "pub units: String" -Description "B6 units metadata in registry"
Assert-Contains -Text $outputSourceText -Pattern "pub frequency: RuntimeOutputFrequency" -Description "B6 frequency metadata in registry"
Assert-Contains -Text $resultStoreSourceText -Pattern "OutputStoreType" -Description "B6 store type metadata"
Assert-Contains -Text $pipelineSourceText -Pattern "is_primary_compare_series" -Description "B6 selected output export filter"
Assert-Contains -Text $pipelineSourceText -Pattern "write_runtime_artifacts" -Description "B6 post-runtime artifact export"
Assert-Contains -Text $pipelineSourceText -Pattern "selected_trace_enabled" -Description "B6 trace-level selected diagnostic output gate"
Assert-Contains -Text $meterRegistrySourceText -Pattern "push_meter_with_dependencies" -Description "B6 meter registry dependency compiler"
Assert-Contains -Text $outputSourceText -Pattern "dependency_output_handles" -Description "B6 meter dependency handle list"
Assert-Contains -Text $outputDiagnosticsSourceText -Pattern "DuplicateOutputRegistration" -Description "B6 duplicate output registration diagnostic"
Assert-Contains -Text $runConfigSourceText -Pattern "pub enum TraceLevel" -Description "B7 trace level enum"
foreach ($traceLevel in @("Off", "Summary", "Stage", "Zone", "Surface", "Ctf", "Full")) {
    Assert-Contains -Text $runConfigSourceText -Pattern $traceLevel -Description "B7 trace level $traceLevel"
}
Assert-Contains -Text $runConfigSourceText -Pattern '"summary" | "normal"' -Description "B7 default summary trace alias"
Assert-Contains -Text $pipelineSourceText -Pattern "ctf_split_trace_enabled" -Description "B7 selected CTF split trace gate"
Assert-Contains -Text $pipelineSourceText -Pattern "full_surface_trace_opt_in" -Description "B7 full surface trace opt-in"
Assert-Contains -Text $pipelineSourceText -Pattern "stage_snapshot_policy" -Description "B7 metadata-only stage trace policy"
Assert-Contains -Text $pipelineSourceText -Pattern "no simulation values are read or mutated" -Description "B7 trace does not mutate calculation state"
Assert-Contains -Text $pipelineSourceText -Pattern "trace_output_write_policy" -Description "B7 buffered trace write policy"
Assert-Contains -Text $pipelineSourceText -Pattern "buffered-json-writer" -Description "B7 buffered trace writer"
Assert-Contains -Text $pipelineSourceText -Pattern "trace_variable_handle_policy" -Description "B7 trace handles separate from OutputRegistry"
Assert-Contains -Text $pipelineSourceText -Pattern "trace_file_size_bytes" -Description "B7 trace file size report"
Assert-Contains -Text $pipelineSourceText -Pattern "compare-summary.json" -Description "B7 report generator compare-summary path"
Assert-Contains -Text $outputDiagnosticsSourceText -Pattern "pub enum RuntimeDiagnosticCode" -Description "B8 RuntimeDiagnosticCode enum"
foreach ($diagnosticCode in @(
    "UnsupportedHeatBalanceBranch",
    "UnsupportedSurfaceBoundary",
    "NonFiniteHeatBalanceState",
    "OutputVariableUnavailable",
    "TimestampMismatch",
    "ToleranceFailure"
)) {
    Assert-Contains -Text $outputDiagnosticsSourceText -Pattern $diagnosticCode -Description "B8 runtime diagnostic code $diagnosticCode"
}
Assert-Contains -Text $outputDiagnosticsSourceText -Pattern "stage: Option<String>" -Description "B8 runtime diagnostic stage context"
Assert-Contains -Text $outputDiagnosticsSourceText -Pattern "surface: Option<String>" -Description "B8 runtime diagnostic surface context"
Assert-Contains -Text $outputDiagnosticsSourceText -Pattern "zone: Option<String>" -Description "B8 runtime diagnostic zone context"
Assert-Contains -Text $outputDiagnosticsSourceText -Pattern "timestep: Option<u64>" -Description "B8 runtime diagnostic timestep context"
Assert-Contains -Text $outputDiagnosticsSourceText -Pattern "handle: Option<OutputHandle>" -Description "B8 runtime diagnostic output handle context"
Assert-Contains -Text $runDiagnosticsSourceText -Pattern "output_handle: Option<u32>" -Description "B8 run diagnostic output handle context"
Assert-Contains -Text $pipelineSourceText -Pattern "diagnostics.json" -Description "B8 JSON diagnostic artifact"
Assert-Contains -Text $pipelineSourceText -Pattern "render_eplusrs_err" -Description "B8 err-like diagnostic artifact"
Assert-Contains -Text $mainSourceText -Pattern "diagnostic_error_policy" -Description "B8 conformance diagnostic error policy"
Assert-Contains -Text $mainSourceText -Pattern "HeatBalanceGateDiagnostic" -Description "B8 structured conformance gate diagnostic"
Assert-Contains -Text $mainSourceText -Pattern 'diagnostic.severity == "error"' -Description "B8 conformance error severity fail gate"
Assert-Contains -Text $mainSourceText -Pattern "Result<HeatBalanceConformanceDiagnostic, String>" -Description "B8 conformance diagnostic Result path"
Assert-Contains -Text $mainSourceText -Pattern "HeatBalancePerformanceProfile" -Description "B9 performance profile type"
Assert-Contains -Text $mainSourceText -Pattern "performance-summary.json" -Description "B9 performance summary artifact"
foreach ($performancePhase in @(
    "parse_time",
    "raw_model_build",
    "typed_model_compile",
    "simulation_model_compile",
    "model_graph_build",
    "execution_plan_build",
    "weather_schedule_precompute",
    "runtime_heat_balance_execution",
    "output_report_generation",
    "trace_write"
)) {
    Assert-Contains -Text $mainSourceText -Pattern $performancePhase -Description "B9 performance phase $performancePhase"
}
Assert-Contains -Text $mainSourceText -Pattern 'conformance.status == "pass"' -Description "B9 pass-only speedup claim gate"
Assert-Contains -Text $mainSourceText -Pattern "speedup_claim_allowed" -Description "B9 speedup claim policy field"
Assert-Contains -Text $mainSourceText -Pattern "energyplus_oracle_wall_seconds" -Description "B9 EnergyPlus oracle runtime comparison"
Assert-Contains -Text $mainSourceText -Pattern "rust_compare_report_wall_seconds" -Description "B9 Rust runtime/report comparison"
Assert-Contains -Text $mainSourceText -Pattern "compatibility_mode_separated_from_fast_mode" -Description "B9 compatibility and fast mode separation"
Assert-Contains -Text $mainSourceText -Pattern "fast_mode" -Description "B9 fast mode field"
Assert-Contains -Text $reportText -Pattern "performance_summary: performance-summary.json" -Description "B9 markdown performance summary path"
Assert-Contains -Text $timeAxisSourceText -Pattern "pub struct ZoneTimestepAxis" -Description "C1 separated zone timestep structure"
Assert-Contains -Text $timeAxisSourceText -Pattern "pub struct SystemTimestepAxis" -Description "C1 separated system timestep structure"
Assert-Contains -Text $timeAxisSourceText -Pattern "variable_system_timestep_support" -Description "C1 variable system timestep placeholder"
Assert-Contains -Text $timeAxisSourceText -Pattern "shorten_timestep_sys_state" -Description "C1 ShortenTimeStepSys state flag"
Assert-Contains -Text $timeAxisSourceText -Pattern "use_zone_timestep_history_state" -Description "C1 UseZoneTimeStepHistory state flag"
Assert-Contains -Text $timeAxisSourceText -Pattern "pub struct TimeAxisSamplePartitions" -Description "C1 warmup/run-period/design-day partition structure"
Assert-Contains -Text $runtimeStateSourceText -Pattern "variable_system_timestep_placeholder" -Description "C1 runtime variable system timestep placeholder state"
Assert-Contains -Text $runtimeStateSourceText -Pattern "hvac_iteration_count" -Description "C1 HVAC iteration count state"
Assert-Contains -Text $runtimeStateSourceText -Pattern "plant_iteration_count" -Description "C1 Plant iteration count state"
Assert-Contains -Text $runtimeSourceText -Pattern "time_axis_source: `"shared TimeAxis for weather/schedule/output/report`"" -Description "C1 runtime TimeAxis source summary"
Assert-Contains -Text $mainSourceText -Pattern "HeatBalanceTimeAxisDiagnostic" -Description "C1 conformance time-axis diagnostic"
Assert-Contains -Text $mainSourceText -Pattern "heat_balance_time_axis_json" -Description "C1 time-axis JSON renderer"
Assert-Contains -Text $mainSourceText -Pattern "precompute_schedule_value_series_for_time_axis" -Description "C1 schedule precompute uses TimeAxis"
Assert-Contains -Text $mainSourceText -Pattern "precompute_weather_timestep_series" -Description "C1 weather precompute uses shared time-axis cadence"
Assert-Contains -Text $reportText -Pattern "time_axis_source: shared TimeAxis for weather/schedule/output/report" -Description "C1 markdown TimeAxis source"
Assert-Contains -Text $reportText -Pattern "zone_timesteps_per_hour: 4" -Description "C1 markdown zone timestep count"
Assert-Contains -Text $reportText -Pattern "system_timestep_nominal_seconds: 900.000000000000" -Description "C1 markdown system timestep seconds"
Assert-Contains -Text $reportText -Pattern "variable_system_timestep_support: placeholder-state-backed" -Description "C1 markdown variable system timestep placeholder"
Assert-Contains -Text $reportText -Pattern "warmup_reported_samples: 0" -Description "C1 markdown warmup sample partition"
Assert-Contains -Text $reportText -Pattern "run_period_reported_samples: 8760" -Description "C1 markdown run-period sample partition"
Assert-Contains -Text $reportText -Pattern "design_day_reported_samples: 0" -Description "C1 markdown design-day sample partition"
Assert-Contains -Text $idsSourceText -Pattern "typed_id!(NodeId)" -Description "C2 typed NodeId"
Assert-Contains -Text $compilerSourceText -Pattern "NormalizedName::new(node_name)" -Description "C2 node name normalization in compile stage"
Assert-Contains -Text $nodeStateSourceText -Pattern "pub struct NodeStateStore" -Description "C2 NodeStateStore"
Assert-Contains -Text $nodeProjectionSourceText -Pattern '"System Node Temperature"' -Description "C2 node temperature output from NodeStateStore"
Assert-Contains -Text $nodeProjectionSourceText -Pattern '"System Node Humidity Ratio"' -Description "C2 node humidity output from NodeStateStore"
Assert-Contains -Text $nodeProjectionSourceText -Pattern '"System Node Mass Flow Rate"' -Description "C2 node mass flow output from NodeStateStore"
Assert-Contains -Text $nodeProjectionSourceText -Pattern '"System Node Setpoint Temperature"' -Description "C2 node setpoint output from NodeStateStore"
Assert-Contains -Text $nodeProjectionSourceText -Pattern "temperature_setpoint_c" -Description "C2 setpoint state backing"
Assert-Contains -Text $outputSourceText -Pattern '"System Node Setpoint Temperature"' -Description "C2 output registry node setpoint handle"
Assert-Contains -Text $modelSourceText -Pattern "pub struct NodeGraph" -Description "C2 NodeGraph structure"
Assert-Contains -Text $modelSourceText -Pattern "pub struct ComponentNodeOwnershipEdge" -Description "C2 component inlet/outlet node ownership"
Assert-Contains -Text $modelSourceText -Pattern "pub enum NodeGraphDiagnosticCode" -Description "C2 node graph diagnostic code enum"
foreach ($nodeDiagnosticCode in @("DuplicateNode", "DanglingNode", "DisconnectedComponent")) {
    Assert-Contains -Text $modelSourceText -Pattern $nodeDiagnosticCode -Description "C2 node graph diagnostic $nodeDiagnosticCode"
}
Assert-Contains -Text $modelSourceText -Pattern "NodeGraphDiagnostic" -Description "C2 node diagnostics separated from conformance diagnostics"
Assert-Contains -Text $hvacObjectsSourceText -Pattern "pub struct AirLoopHvac" -Description "C3 AirLoopHVAC typed shell"
Assert-Contains -Text $modelSourceText -Pattern "pub struct AirLoopGraph" -Description "C3 AirLoopGraph skeleton"
Assert-Contains -Text $plantObjectsSourceText -Pattern "pub type BranchList = PlantBranchList" -Description "C3 generic BranchList typed alias"
Assert-Contains -Text $plantObjectsSourceText -Pattern "pub type ConnectorList = PlantConnectorList" -Description "C3 generic ConnectorList typed alias"
Assert-Contains -Text $modelSourceText -Pattern "pub struct ComponentRegistry" -Description "C3 ComponentRegistry"
Assert-Contains -Text $hvacObjectsSourceText -Pattern "pub enum FanComponentKind" -Description "C3 fan component enum"
Assert-Contains -Text $hvacObjectsSourceText -Pattern "pub enum CoilComponentKind" -Description "C3 coil component enum"
Assert-Contains -Text $modelSourceText -Pattern "ZoneEquipmentObjectType::IdealLoadsAirSystem" -Description "C3 zone equipment component enum bridge"
Assert-Contains -Text $hvacObjectsSourceText -Pattern "SETPOINT_MANAGER_SOURCE_MAP" -Description "C3 SetpointManager source map"
Assert-Contains -Text $hvacObjectsSourceText -Pattern "AVAILABILITY_MANAGER_SOURCE_MAP" -Description "C3 AvailabilityManager source map"
Assert-Contains -Text $modelSourceText -Pattern "execution_order: Vec<AirLoopExecutionStep>" -Description "C3 airloop execution order from graph"
Assert-Contains -Text $runtimeHvacSourceText -Pattern "HvacComponentNodeStateTrace" -Description "C3 node state before/after component trace"
Assert-Contains -Text $runtimeHvacSourceText -Pattern "state_before" -Description "C3 node state trace before snapshot"
Assert-Contains -Text $runtimeHvacSourceText -Pattern "state_after" -Description "C3 node state trace after snapshot"
Assert-Contains -Text $airloopFanCaseText -Pattern 'comparison_class = "diagnostic-only"' -Description "C3 fan-only diagnostic fixture"
Assert-Contains -Text $airloopFanCaseText -Pattern "conformance_claim = false" -Description "C3 fan-only no conformance claim"
Assert-Contains -Text $airloopFanCaseText -Pattern 'level = "baseline"' -Description "C3 fan-only baseline-only outputs"
Assert-Contains -Text $airloopFanFixtureText -Pattern '"AirLoopHVAC"' -Description "C3 fan-only AirLoopHVAC fixture"
Assert-Contains -Text $airloopFanFixtureText -Pattern '"Fan:ConstantVolume"' -Description "C3 fan-only fan fixture"
Assert-Contains -Text $airloopFanFixtureText -Pattern '"Fan Electricity Rate"' -Description "C3 fan output gate variable"
Assert-Contains -Text $airloopFanFixtureText -Pattern '"System Node Temperature"' -Description "C3 node output gate variable"
Assert-Contains -Text $airloopFanSmokeText -Pattern "air_loop_execution_steps: 1" -Description "C3 fan-only diagnostic graph gate"
Assert-Contains -Text $airloopFanSmokeText -Pattern "component_registry_entries: 4" -Description "C3 fan-only component registry gate"
Assert-Contains -Text $devCommandsText -Pattern "airloop-fan-only-diagnostic-smoke" -Description "C3 fan-only smoke dev command"
Assert-Contains -Text $airloopCoilCaseText -Pattern 'comparison_class = "diagnostic-only"' -Description "C3 coil-only diagnostic fixture"
Assert-Contains -Text $airloopCoilCaseText -Pattern "conformance_claim = false" -Description "C3 coil-only no conformance claim"
Assert-Contains -Text $airloopCoilCaseText -Pattern 'level = "baseline"' -Description "C3 coil-only baseline-only outputs"
Assert-Contains -Text $airloopCoilFixtureText -Pattern '"AirLoopHVAC"' -Description "C3 coil-only AirLoopHVAC fixture"
Assert-Contains -Text $airloopCoilFixtureText -Pattern '"Coil:Heating:Electric"' -Description "C3 coil-only coil fixture"
Assert-Contains -Text $airloopCoilFixtureText -Pattern '"Heating Coil Heating Rate"' -Description "C3 coil output gate variable"
Assert-Contains -Text $ptacCaseText -Pattern 'comparison_class = "diagnostic-only"' -Description "C3 PTAC diagnostic fixture"
Assert-Contains -Text $ptacCaseText -Pattern "conformance_claim = false" -Description "C3 PTAC no conformance claim"
Assert-Contains -Text $ptacCaseText -Pattern "ZoneHVAC:PackagedTerminalAirConditioner remains raw-only" -Description "C3 PTAC unsupported boundary"
Assert-Contains -Text $ptacFixtureText -Pattern '"ZoneHVAC:PackagedTerminalAirConditioner"' -Description "C3 PTAC fixture"
Assert-Contains -Text $ptacFixtureText -Pattern '"Fan:OnOff"' -Description "C3 PTAC fan fixture"
Assert-Contains -Text $ptacFixtureText -Pattern '"Coil:Heating:Electric"' -Description "C3 PTAC coil fixture"
Assert-Contains -Text $airloop5ZoneCaseText -Pattern "5ZoneAirCooled" -Description "C3 5ZoneAirCooled diagnostic case"
Assert-Contains -Text $airloop5ZoneCaseText -Pattern 'comparison_class = "diagnostic-only"' -Description "C3 5ZoneAirCooled diagnostic fixture"
Assert-Contains -Text $airloop5ZoneCaseText -Pattern "conformance_claim = false" -Description "C3 5ZoneAirCooled no conformance claim"
Assert-Contains -Text $airloop5ZoneFixtureText -Pattern '"AirLoopHVAC"' -Description "C3 5ZoneAirCooled AirLoopHVAC fixture"
Assert-Contains -Text $airloop5ZoneFixtureText -Pattern '"Fan:VariableVolume"' -Description "C3 5ZoneAirCooled fan fixture"
Assert-Contains -Text $airloop5ZoneFixtureText -Pattern '"Coil:Cooling:DX:SingleSpeed"' -Description "C3 5ZoneAirCooled coil fixture"
Assert-Contains -Text $airloopDiagnosticSmokeText -Pattern "air_loop_execution_steps: 2" -Description "C3 5ZoneAirCooled diagnostic graph gate"
Assert-Contains -Text $airloopDiagnosticSmokeText -Pattern "ZoneHVAC:PackagedTerminalAirConditioner" -Description "C3 PTAC diagnostic smoke gate"
Assert-Contains -Text $devCommandsText -Pattern "airloop-diagnostic-fixtures-smoke" -Description "C3 airloop diagnostic fixtures smoke dev command"
Assert-Contains -Text $modelSourceText -Pattern "pub struct PlantLoopGraph" -Description "C4 PlantLoopGraph skeleton"
Assert-Contains -Text $modelSourceText -Pattern "pub struct PlantHalfLoop" -Description "C4 PlantLoop half-loop structure"
Assert-Contains -Text $modelSourceText -Pattern "PlantLoopSide::Plant" -Description "C4 plant supply side separation"
Assert-Contains -Text $modelSourceText -Pattern "PlantLoopSide::Demand" -Description "C4 plant demand side separation"
Assert-Contains -Text $modelSourceText -Pattern "pub struct PlantComponentRegistryEntry" -Description "C4 plant component registry"
Assert-Contains -Text $modelSourceText -Pattern "pub struct PlantLoopGraphDiagnostic" -Description "C4 plant graph diagnostic"
Assert-Contains -Text $modelSourceText -Pattern "UnsupportedTopology" -Description "C4 unsupported topology diagnostic code"
Assert-Contains -Text $plantStateSourceText -Pattern "PLANT_PUMP_SOURCE_MAP" -Description "C4 pump source map"
Assert-Contains -Text $plantStateSourceText -Pattern "PLANT_BOILER_SOURCE_MAP" -Description "C4 boiler source map"
Assert-Contains -Text $plantStateSourceText -Pattern "PLANT_CHILLER_SOURCE_MAP" -Description "C4 chiller source map"
Assert-Contains -Text $plantStateSourceText -Pattern "PLANT_LOAD_PROFILE_SOURCE_MAP" -Description "C4 PlantLoadProfile source map"
Assert-Contains -Text $plantStateSourceText -Pattern "PLANT_OPERATION_SCHEME_SOURCE_MAP" -Description "C4 operation scheme source map"
Assert-Contains -Text $plantStateSourceText -Pattern "PLANT_SETPOINT_MANAGER_INTERACTION_SOURCE_MAP" -Description "C4 setpoint manager interaction map"
Assert-Contains -Text $plantStateSourceText -Pattern "pub struct PlantFlowRequestTrace" -Description "C4 plant flow request trace"
Assert-Contains -Text $plantStateSourceText -Pattern "pub struct PlantSelectedOperationSchemeTrace" -Description "C4 plant selected operation scheme trace"
Assert-Contains -Text $plantStateSourceText -Pattern "pub struct PlantResidualTrace" -Description "C4 plant residual trace"
Assert-Contains -Text $plantLoopFixtureText -Pattern '"PlantLoop"' -Description "C4 plant-loop skeleton fixture"
Assert-Contains -Text $plantLoopFixtureText -Pattern '"Pump:ConstantSpeed"' -Description "C4 pump fixture"
Assert-Contains -Text $plantLoopFixtureText -Pattern '"Boiler:HotWater"' -Description "C4 boiler fixture"
Assert-Contains -Text $plantLoopFixtureText -Pattern '"Chiller:Electric:EIR"' -Description "C4 chiller fixture"
Assert-Contains -Text $plantLoopCaseText -Pattern 'comparison_class = "diagnostic-only"' -Description "C4 plant baseline-only case"
Assert-Contains -Text $plantLoopCaseText -Pattern "conformance_claim = false" -Description "C4 plant no conformance claim"
Assert-Contains -Text $plantLoopCaseText -Pattern 'level = "baseline"' -Description "C4 plant baseline-only outputs"
Assert-Contains -Text $plantLoopSmokeText -Pattern "plant_loop_half_loops: 2" -Description "C4 plant half-loop smoke gate"
Assert-Contains -Text $plantLoopSmokeText -Pattern "plant_component_registry_entries: 3" -Description "C4 plant component registry smoke gate"
Assert-Contains -Text $plantLoopSmokeText -Pattern "plant_loop_graph_diagnostics: 0" -Description "C4 plant graph diagnostic smoke gate"
Assert-Contains -Text $plantProjectionSmokeText -Pattern "diagnostic-only" -Description "C4 plant projection remains diagnostic-only"
Assert-Contains -Text $devCommandsText -Pattern "plant-loop-skeleton-smoke" -Description "C4 plant skeleton smoke dev command"
Assert-Contains -Text $devCommandsText -Pattern "plant-loop-diagnostic-smoke" -Description "C4 plant diagnostic smoke dev command"
Assert-Contains -Text $devCommandsText -Pattern "plant-loop-projection-smoke" -Description "C4 plant projection smoke dev command"
Assert-Contains -Text $outputSourceText -Pattern "meter_registry: RuntimeMeterRegistry" -Description "C5 MeterRegistry separated from RuntimeOutputRegistry"
Assert-Contains -Text $outputSourceText -Pattern "aggregation_plan: RuntimeMeterAggregationPlan" -Description "C5 meter aggregation plan attached to meter definition"
Assert-Contains -Text $outputSourceText -Pattern "output_handles_for_meter_dependency" -Description "C5 meter dependency handle precompiler"
Assert-Contains -Text $meterRegistrySourceText -Pattern "RuntimeMeterAggregationKind::from_meter_name" -Description "C5 meter aggregation kind resolver"
Assert-Contains -Text $meterRegistrySourceText -Pattern "ELECTRICITY_FACILITY_METER" -Description "C5 Electricity:Facility aggregation path"
Assert-Contains -Text $meterRegistrySourceText -Pattern "GAS_FACILITY_METER" -Description "C5 Gas:Facility aggregation path"
Assert-Contains -Text $meterRegistrySourceText -Pattern "HEATING_ENERGY_TRANSFER_METER" -Description "C5 Heating:EnergyTransfer aggregation path"
Assert-Contains -Text $meterRegistrySourceText -Pattern "COOLING_ENERGY_TRANSFER_METER" -Description "C5 Cooling:EnergyTransfer aggregation path"
Assert-Contains -Text $meterRegistrySourceText -Pattern "meter_rate_to_energy_j" -Description "C5 rate-to-energy conversion helper"
Assert-Contains -Text $meterRegistrySourceText -Pattern "METER_RATE_TO_ENERGY_RULE" -Description "C5 rate-to-energy source rule"
Assert-Contains -Text $meterRegistrySourceText -Pattern "RuntimeMeterAggregationPeriod" -Description "C5 meter aggregation period split"
Assert-Contains -Text $meterRegistrySourceText -Pattern "RuntimeOutputFrequency::Hourly => Self::Hourly" -Description "C5 hourly aggregation split"
Assert-Contains -Text $meterRegistrySourceText -Pattern "RuntimeOutputFrequency::Monthly => Self::Monthly" -Description "C5 monthly aggregation split"
Assert-Contains -Text $meterRegistrySourceText -Pattern "RuntimeOutputFrequency::Annual => Self::Annual" -Description "C5 annual aggregation split"
Assert-Contains -Text $meterRegistrySourceText -Pattern "METER_ZERO_NEAR_TOLERANCE_J" -Description "C5 meter zero-near tolerance"
Assert-Contains -Text $meterRegistrySourceText -Pattern "component_output_to_facility_meter_source_map" -Description "C5 component output to facility meter source map"
Assert-Contains -Text $outputSourceText -Pattern "binding.fuel_energy_variable" -Description "C5 IdealLoads meter dependency source variable"
Assert-Contains -Text $outputSourceText -Pattern "dependency_output_handles.clone()" -Description "C5 meter dependency list precompiled into all frequencies"
Assert-Contains -Text $reportText -Pattern "Site Outdoor Air Wetbulb Temperature / hourly / weather / eso / conformance" -Description "wet-bulb weather conformance output"
Assert-Contains -Text $reportText -Pattern "Site Rain Status / hourly / weather / eso / conformance" -Description "rain-status weather conformance output"
Assert-Contains -Text $reportText -Pattern "Site Sky Temperature / hourly / weather / eso / conformance" -Description "sky temperature weather conformance output"
Assert-Contains -Text $reportText -Pattern "Site Horizontal Infrared Radiation Rate per Area / hourly / weather / eso / conformance" -Description "horizontal infrared weather conformance output"
Assert-Contains -Text $reportText -Pattern "Zone Mean Air Humidity Ratio / hourly / zone-state / eso / conformance" -Description "zone humidity ratio conformance output"
Assert-Contains -Text $reportText -Pattern "Zone Air Heat Balance Outdoor Air Transfer Rate / hourly / zone-state / eso / conformance" -Description "outdoor air transfer conformance output"
Assert-Contains -Text $reportText -Pattern "barometric_pressure_pa" -Description "zone-air barometric pressure trace"
Assert-Contains -Text $reportText -Pattern "rho_air_kg_per_m3" -Description "zone-air rhoAir trace"
Assert-Contains -Text $reportText -Pattern "cp_air_j_per_kg_k" -Description "zone-air CpAir trace"
Assert-Contains -Text $reportText -Pattern "sum_mcp_w_per_k" -Description "zone-air SumMCp trace"
Assert-Contains -Text $reportText -Pattern "use_zone_timestep_history" -Description "zone-air UseZoneTimeStepHistory trace"
Assert-Contains -Text $reportText -Pattern "shorten_timestep_sys" -Description "zone-air ShortenTimeStepSys trace"
Assert-Contains -Text $reportText -Pattern "prior_timestep_seconds" -Description "zone-air PriorTimeStep trace"
Assert-Contains -Text $reportText -Pattern "building_warmup_minimum_days: 6" -Description "markdown Building warmup minimum days"
Assert-Contains -Text $reportText -Pattern "building_warmup_maximum_days: 30" -Description "markdown Building warmup maximum days"
Assert-Contains -Text $reportText -Pattern "building_temperature_convergence_tolerance_delta_c: 0.004000000000" -Description "markdown Building temperature convergence tolerance"
Assert-Contains -Text $reportText -Pattern "building_loads_convergence_tolerance_w: 0.040000000000" -Description "markdown Building load convergence tolerance"
Assert-Contains -Text $reportText -Pattern "warmup_minimum_days: 20" -Description "markdown effective warmup minimum days"
Assert-Contains -Text $reportText -Pattern "warmup_maximum_days: 30" -Description "markdown effective warmup maximum days"
Assert-Contains -Text $reportText -Pattern "warmup_temperature_convergence_tolerance_delta_c: 0.004000000000" -Description "markdown effective warmup temperature tolerance"
Assert-Contains -Text $reportText -Pattern "warmup_loads_convergence_tolerance_w: 0.040000000000" -Description "markdown effective warmup load tolerance"
Assert-Contains -Text $reportText -Pattern "## Warmup End-State Deltas" -Description "markdown warmup end-state delta section"
Assert-Contains -Text $reportText -Pattern "warmup-end-state-mat-delta" -Description "markdown warmup MAT blocker"
Assert-Contains -Text $reportText -Pattern "warmup-end-state-surface-temperature-delta" -Description "markdown warmup surface blocker"
Assert-Contains -Text $reportText -Pattern "warmup-end-state-ctf-history-delta" -Description "markdown warmup CTF blocker"
Assert-Contains -Text $reportText -Pattern "warmup-end-state-zone-history-delta" -Description "markdown warmup zone history blocker"
Assert-Contains -Text $reportText -Pattern "## Rust Warmup Day-End Zone-Air Trace" -Description "markdown warmup day-end zone-air trace"
Assert-Contains -Text $eioText -Pattern "RoomAir Model,ZONE ONE,Mixing/Well-Stirred" -Description "oracle room-air model"
Assert-Contains -Text $eioText -Pattern "Building Information,Simple One Zone (Wireframe DXF),0.000,Suburbs,4.00000E-002,4.00000E-003,MinimalShadowing,30,6" -Description "oracle Building warmup settings"
Assert-Contains -Text $eioText -Pattern "Environment,RUN PERIOD 1,WeatherFileRunPeriod" -Description "oracle run-period environment"
Assert-Contains -Text $eioText -Pattern "Environment:WarmupDays, 20" -Description "oracle run-period warmup days"
Assert-Contains -Text $eioText -Pattern "Warmup Convergence Information,ZONE ONE,RunPeriod: RUN PERIOD 1" -Description "oracle run-period warmup convergence row"
Assert-Contains -Text $iddText -Pattern "\default ThirdOrderBackwardDifference" -Description "IDD ZoneAirHeatBalanceAlgorithm default"
if ($sourceIdfText -match "(?im)^\s*ZoneAirHeatBalanceAlgorithm\s*,") {
    throw "Official 1ZoneUncontrolled source IDF unexpectedly declares ZoneAirHeatBalanceAlgorithm"
}
Assert-Contains -Text $compilerSourceText -Pattern "loads_convergence_tolerance_w: self.number_default(" -Description "compiler Building load tolerance parse"
Assert-Contains -Text $compilerSourceText -Pattern "temperature_convergence_tolerance_delta_c: self.number_default(" -Description "compiler Building temperature tolerance parse"
Assert-Contains -Text $compilerSourceText -Pattern "maximum_number_of_warmup_days: self.u32_default(" -Description "compiler Building maximum warmup days parse"
Assert-Contains -Text $compilerSourceText -Pattern "minimum_number_of_warmup_days: self.u32_default(" -Description "compiler Building minimum warmup days parse"
Assert-Contains -Text $runtimeStateSourceText -Pattern "let minimum_days = building.minimum_number_of_warmup_days" -Description "runtime warmup minimum from Building"
Assert-Contains -Text $runtimeStateSourceText -Pattern "let maximum_days = building.maximum_number_of_warmup_days.max(minimum_days)" -Description "runtime warmup maximum from Building"
Assert-Contains -Text $runtimeStateSourceText -Pattern ".temperature_convergence_tolerance_delta_c" -Description "runtime warmup temperature tolerance from Building"
Assert-Contains -Text $runtimeStateSourceText -Pattern "loads_convergence_tolerance_w: building.loads_convergence_tolerance_w" -Description "runtime warmup load tolerance from Building"
Assert-Contains -Text $warmupSourceText -Pattern "run_heat_balance_run_period_warmup" -Description "runtime warmup loop owner"
Assert-Contains -Text $warmupSourceText -Pattern "previous_day_extrema" -Description "warmup previous daily extrema snapshot"
Assert-Contains -Text $warmupSourceText -Pattern "HeatBalanceWarmupDayTemperatureExtrema" -Description "warmup daily extrema tracker"
Assert-Contains -Text $warmupSourceText -Pattern "day_extrema.record_state(state)" -Description "warmup timestep extrema capture"
Assert-Contains -Text $warmupSourceText -Pattern "final_delta <= tolerance" -Description "warmup end-of-day convergence check"
Assert-Contains -Text $warmupSourceText -Pattern "day >= options.minimum_days" -Description "warmup minimum-day convergence guard"
Assert-Contains -Text $warmupSourceText -Pattern "day_end_zone_air_states.extend" -Description "warmup day-end MAT trace capture"
Assert-Contains -Text $warmupSourceText -Pattern "state.timestep_index - timestep_start" -Description "warmup timestep count isolation"
Assert-Contains -Text $mainSourceText -Pattern "eio_run_period_warmup_days(eio_path)" -Description "EIO run-period warmup day extraction"
Assert-Contains -Text $mainSourceText -Pattern ".with_warmup_minimum_days(warmup_days)" -Description "candidate effective warmup minimum override"
Assert-Contains -Text $mainSourceText -Pattern "zone_air_warmup_day_end_states" -Description "warmup day-end trace JSON"
Assert-Contains -Text $mainSourceText -Pattern "warmup_end_state_deltas" -Description "warmup end-state delta JSON"
Assert-Contains -Text $mainSourceText -Pattern "first run-period sample after warmup" -Description "warmup first run-period delta evidence"
Assert-Contains -Text $mainSourceText -Pattern "first run-period CTF history delta after warmup" -Description "warmup CTF handoff delta evidence"
Assert-Contains -Text $mainSourceText -Pattern "if context.conformance_claim && diagnostic.diagnostic_probe_used" -Description "diagnostic probe promotion blocker"
Assert-Contains -Text $mainSourceText -Pattern "zone_air_algorithm.allows_conformance_promotion()" -Description "algorithm promotion eligibility source"
Assert-Contains -Text $candidateCaseText -Pattern 'warmup_output = "run-period-only-with-diagnostic-trace"' -Description "candidate warmup output suppression policy"
Assert-Contains -Text $diagnosticCaseText -Pattern 'comparison_class = "diagnostic-only"' -Description "diagnostic case comparison class"
Assert-Contains -Text $diagnosticCaseText -Pattern 'conformance_claim = false' -Description "diagnostic case no conformance claim"
Assert-Contains -Text $diagnosticCaseText -Pattern 'warmup_output = "run-period-only-with-diagnostic-trace"' -Description "diagnostic warmup output suppression policy"
Assert-Contains -Text $sourceIdfText -Pattern "SurfaceConvectionAlgorithm:Inside,TARP" -Description "official inside convection algorithm"
Assert-Contains -Text $iddText -Pattern "\default TARP" -Description "IDD inside convection default TARP"
Assert-Contains -Text $insideConvectionSourceText -Pattern "energyplus_tarp_inside_convection_coefficient_w_per_m2_k" -Description "Rust TARP inside convection implementation"
Assert-Contains -Text $insideConvectionSourceText -Pattern "energyplus_tarp_inside_convection_branch_id" -Description "Rust TARP inside convection branch trace"
Assert-Contains -Text $insideConvectionSourceText -Pattern "stable-horizontal-or-tilt" -Description "Rust stable horizontal/tilt inside convection branch"
Assert-Contains -Text $insideConvectionSourceText -Pattern "unstable-horizontal-or-tilt" -Description "Rust unstable horizontal/tilt inside convection branch"
Assert-Contains -Text $insideConvectionSourceText -Pattern "vertical-wall" -Description "Rust vertical wall inside convection branch"
Assert-Contains -Text $insideConvectionSourceText -Pattern "ENERGYPLUS_LOW_CONVECTION_LIMIT_W_PER_M2_K: f64 = 0.1" -Description "inside convection low clamp"
Assert-Contains -Text $insideConvectionSourceText -Pattern "ENERGYPLUS_HIGH_CONVECTION_LIMIT_W_PER_M2_K: f64 = 1000.0" -Description "inside convection high clamp"
Assert-Contains -Text $insideConvectionSourceText -Pattern ".clamp(" -Description "inside convection clamp application"
Assert-Contains -Text $surfaceLoopSourceText -Pattern "inside_hconv_reevaluation_interval" -Description "inside hconv reevaluation cadence source"
Assert-Contains -Text $surfaceLoopSourceText -Pattern "freeze_inside_convection_for_timestep" -Description "inside hconv freeze source"
Assert-Contains -Text $sourceIdfText -Pattern "SurfaceConvectionAlgorithm:Outside,DOE-2" -Description "official outside convection algorithm"
Assert-Contains -Text $iddText -Pattern "\default DOE-2" -Description "IDD outside convection default DOE-2"
Assert-Contains -Text $insideConvectionSourceText -Pattern "energyplus_doe2_outside_convection_coefficient_w_per_m2_k" -Description "Rust DOE-2 outside convection implementation"
Assert-Contains -Text $insideConvectionSourceText -Pattern "energyplus_outside_convection_branch_id" -Description "Rust outside convection branch trace"
Assert-Contains -Text $insideConvectionSourceText -Pattern "doe2-windward" -Description "Rust DOE-2 windward branch"
Assert-Contains -Text $insideConvectionSourceText -Pattern "doe2-leeward" -Description "Rust DOE-2 leeward branch"
Assert-Contains -Text $surfaceObjectsSourceText -Pattern "pub enum OutsideBoundaryCondition" -Description "typed outside boundary condition enum"
foreach ($variant in @("Adiabatic", "Foundation", "Ground", "Outdoors", "Space", "Surface", "Zone", "Other")) {
    Assert-Contains -Text $surfaceObjectsSourceText -Pattern $variant -Description "outside boundary enum variant $variant"
}
Assert-Contains -Text $surfaceBoundarySourceText -Pattern "OutsideBoundaryCondition::Ground =>" -Description "ground boundary temperature path"
Assert-Contains -Text $surfaceBoundarySourceText -Pattern "ENERGYPLUS_DEFAULT_BUILDING_SURFACE_GROUND_TEMPERATURE_C: f64 = 18.0" -Description "default building-surface ground temperature"
Assert-Contains -Text $surfaceBoundarySourceText -Pattern "OutsideBoundaryCondition::Adiabatic => owning_zone_temperature_c" -Description "adiabatic boundary temperature path"
Assert-Contains -Text $surfaceBoundarySourceText -Pattern "OutsideBoundaryCondition::Surface" -Description "interzone surface boundary target path"
Assert-Contains -Text $surfaceBoundarySourceText -Pattern "OutsideBoundaryCondition::Zone | OutsideBoundaryCondition::Space" -Description "zone/space boundary target path"
Assert-Contains -Text $surfaceBalanceSourceText -Pattern "surface.outside_boundary_condition == OutsideBoundaryCondition::Outdoors" -Description "exterior boundary balance path"
Assert-Contains -Text $surfaceBalanceSourceText -Pattern "energyplus_exterior_wet_context_fraction" -Description "exterior wet/rain branch"
Assert-Contains -Text $surfaceWeatherSourceText -Pattern "energyplus_weather_record_is_rain_at_timestep_with_starting_values" -Description "rain interpolation path"
Assert-Contains -Text $surfaceBalanceSourceText -Pattern "energyplus_weather_wind_speed_for_context" -Description "outside wind speed interpolation path"
Assert-Contains -Text $surfaceBalanceSourceText -Pattern "energyplus_weather_wind_direction_for_context" -Description "outside wind direction interpolation path"
Assert-Contains -Text $surfaceBalanceSourceText -Pattern "energyplus_weather_horizontal_infrared_for_context" -Description "outside horizontal infrared path"
Assert-Contains -Text $surfaceBalanceSourceText -Pattern "surface_incident_solar_radiation_for_weather_context_w_per_m2" -Description "outside incident solar weather path"
Assert-Contains -Text $surfaceBalanceSourceText -Pattern "thermal_radiation_to_sky_coefficient_w_per_m2_k" -Description "outside longwave split report path"
Assert-Contains -Text $surfaceBalanceSourceText -Pattern "solar_radiation_heat_gain_rate_w" -Description "outside solar absorption report path"
if ($sourceIdfText -notmatch "(?is)Zn001:Flr001.*?Floor.*?Adiabatic") {
    throw "Official 1Zone floor boundary was expected to be Adiabatic"
}
if ($sourceIdfText -notmatch "(?is)Zn001:Roof001.*?Roof.*?Outdoors") {
    throw "Official 1Zone roof boundary was expected to be Outdoors"
}
if ($sourceIdfText -notmatch "(?is)Zn001:Wall001.*?Wall.*?Outdoors") {
    throw "Official 1Zone wall boundary was expected to be Outdoors"
}
if ($sourceIdfText -match "(?im)^\s*Site:GroundTemperature:BuildingSurface\s*,") {
    throw "Official 1Zone source IDF unexpectedly declares Site:GroundTemperature:BuildingSurface"
}
Assert-Contains -Text $sourceIdfText -Pattern "MinimalShadowing" -Description "official solar distribution"
Assert-Contains -Text $sourceIdfText -Pattern "Timestep,4" -Description "official zone timestep count"
Assert-Contains -Text $sourceIdfText -Pattern "! Windows:                        0" -Description "official no-window target evidence"
foreach ($unexpectedSolarObject in @("FenestrationSurface:Detailed", "WindowMaterial:SimpleGlazingSystem", "Window,")) {
    if ($sourceIdfText -match [regex]::Escape($unexpectedSolarObject)) {
        throw "Official 1Zone source IDF unexpectedly declares $unexpectedSolarObject"
    }
}
if ($sourceIdfText -match "(?im)^\s*Site:GroundReflectance\s*,") {
    throw "Official 1Zone source IDF unexpectedly declares Site:GroundReflectance"
}
Assert-Contains -Text $sourceIdfText -Pattern "RunPeriod," -Description "official run-period object"
Assert-Contains -Text $sourceIdfText -Pattern "Use Weather File Rain Indicators" -Description "official weather-file rain indicator use"
Assert-Contains -Text $timeAxisSourceText -Pattern "DEFAULT_RUN_PERIOD_YEAR: u32 = 2013" -Description "fixed run-period calendar year"
Assert-Contains -Text $weatherSourceText -Pattern "global_horizontal_radiation_wh_per_m2: parse_epw_f64(" -Description "EPW global horizontal solar parser"
Assert-Contains -Text $weatherSourceText -Pattern "direct_normal_radiation_wh_per_m2: parse_epw_f64(" -Description "EPW direct normal solar parser"
Assert-Contains -Text $weatherSourceText -Pattern "diffuse_horizontal_radiation_wh_per_m2: parse_epw_f64(" -Description "EPW diffuse horizontal solar parser"
Assert-Contains -Text $weatherSourceText -Pattern "timestep_global_horizontal_radiation_w_per_m2" -Description "weather timestep global horizontal solar series"
Assert-Contains -Text $weatherSourceText -Pattern "timestep_direct_normal_radiation_w_per_m2" -Description "weather timestep direct normal solar series"
Assert-Contains -Text $weatherSourceText -Pattern "timestep_diffuse_horizontal_radiation_w_per_m2" -Description "weather timestep diffuse horizontal solar series"
Assert-Contains -Text $solarSourceText -Pattern "day_of_year(DEFAULT_RUN_PERIOD_YEAR, record.month, record.day)" -Description "solar day-of-year ignores EPW source year"
Assert-Contains -Text $solarSourceText -Pattern "FirstHourInterpolationStartingValues::Hour24" -Description "solar first-hour Hour24 interpolation default"
Assert-Contains -Text $solarSourceText -Pattern "ENERGYPLUS_SHADOWING_CALC_FREQUENCY_DAYS: usize = 20" -Description "EnergyPlus shadowing calculation frequency"
Assert-Contains -Text $solarSourceText -Pattern "DEFAULT_SOLAR_GROUND_REFLECTANCE: f64 = 0.2" -Description "default solar ground reflectance"
Assert-Contains -Text $solarSourceText -Pattern "solar_position_rad_from_coefficients" -Description "solar position helper"
Assert-Contains -Text $solarSourceText -Pattern "surface_azimuth_deg(&surface.vertices)" -Description "surface solar azimuth source"
Assert-Contains -Text $solarSourceText -Pattern "surface_tilt_deg(surface.surface_type, &surface.vertices)" -Description "surface solar tilt source"
Assert-Contains -Text $solarSourceText -Pattern "energyplus_average_solar_coefficients" -Description "shadowing-period averaged solar coefficients"
Assert-Contains -Text $solarSourceText -Pattern "energyplus_shadowing_period_solar_coefficients" -Description "EnergyPlus shadowing-period solar coefficient helper"
Assert-Contains -Text $solarSourceText -Pattern "energyplus_anisotropic_sky_multiplier" -Description "Perez anisotropic sky multiplier"
Assert-Contains -Text $solarSourceText -Pattern "circumsolar_sunlit_fraction" -Description "surface sunlit fraction proxy"
Assert-Contains -Text $solarSourceText -Pattern "surface_ground_view_factor" -Description "ground-reflected solar view-factor path"
Assert-Contains -Text $surfaceManagerSourceText -Pattern "solar_absorptance: outside_material" -Description "outside material solar absorptance source"
Assert-Contains -Text $surfaceBalanceSourceText -Pattern "surface_state.solar_absorptance.clamp(0.0, 1.0) * incident_solar_w_per_m2.max(0.0)" -Description "exterior solar absorption source term"
Assert-Contains -Text $surfaceBalanceSourceText -Pattern "q_sol_w_per_m2: surface.inside_shortwave_absorbed_w_per_m2" -Description "inside shortwave source term mapping"
Assert-Contains -Text $surfaceBalanceSourceText -Pattern "absorbed_outside_source_w_per_m2: solar_absorptance * incident_solar_w_per_m2.max(0.0)" -Description "outside solar absorbed source term mapping"
Assert-Contains -Text $mainSourceText -Pattern "load_eio_heat_transfer_surfaces(eio_path)" -Description "EIO surface geometry parser path"
Assert-Contains -Text $mainSourceText -Pattern "surface_geometry_row_matches(rust_surface, oracle_surface, tolerance)" -Description "surface geometry comparison path"
Assert-Contains -Text $mainSourceText -Pattern "oracle_surface.azimuth_deg" -Description "EIO surface azimuth comparison"
Assert-Contains -Text $mainSourceText -Pattern "oracle_surface.tilt_deg" -Description "EIO surface tilt comparison"
Assert-Contains -Text $candidateCaseText -Pattern 'variable = "Surface Outside Face Incident Solar Radiation Rate per Area"' -Description "candidate incident solar proof variable"
Assert-Contains -Text $candidateCaseText -Pattern 'variable = "Surface Outside Face Solar Radiation Heat Gain Rate"' -Description "candidate absorbed solar proof variable"
Assert-Contains -Text $candidateCaseText -Pattern 'class = "surface-solar-rate-state"' -Description "candidate solar heat-gain conformance class"
Assert-Contains -Text $reportText -Pattern "outside_solar_w" -Description "surface first-sample outside solar source trace column"
Assert-Contains -Text $longwaveSourceText -Pattern "energyplus_exterior_longwave_terms" -Description "outside longwave source split"
Assert-Contains -Text $longwaveSourceText -Pattern "sky_coefficient_w_per_m2_k" -Description "outside sky longwave coefficient"
Assert-Contains -Text $longwaveSourceText -Pattern "ground_coefficient_w_per_m2_k" -Description "outside ground longwave coefficient"
Assert-Contains -Text $iddText -Pattern "Zone Radiant Exchange Algorithm" -Description "IDD zone radiant exchange algorithm field"
Assert-Contains -Text $iddText -Pattern "\key ScriptF" -Description "IDD ScriptF radiant exchange key"
Assert-Contains -Text $iddText -Pattern "\key CarrollMRT" -Description "IDD CarrollMRT radiant exchange key"
Assert-Contains -Text $iddText -Pattern "\default ScriptF" -Description "IDD default ScriptF radiant exchange algorithm"
if ($sourceIdfText -match "(?im)^\s*PerformancePrecisionTradeoffs\s*,") {
    throw "Official 1Zone source IDF unexpectedly declares PerformancePrecisionTradeoffs"
}
foreach ($unexpectedObject in @("InternalMass", "ZoneProperty:UserViewFactors:BySurfaceName")) {
    $escapedObject = [regex]::Escape($unexpectedObject)
    if ($sourceIdfText -match "(?im)^\s*$escapedObject\s*,") {
        throw "Official 1Zone source IDF unexpectedly declares $unexpectedObject"
    }
}
Assert-NotContains -Text $sourceIdfText -Pattern "ViewFactorInfo" -Description "official user/report view-factor object"
$buildingSurfaceDeclarations = [regex]::Matches($sourceIdfText, "(?im)^\s*BuildingSurface:Detailed\s*,")
if ($buildingSurfaceDeclarations.Count -ne 6) {
    throw "Expected six official 1Zone BuildingSurface:Detailed objects, got $($buildingSurfaceDeclarations.Count)"
}
foreach ($surfaceName in @("Zn001:Wall001", "Zn001:Wall002", "Zn001:Wall003", "Zn001:Wall004", "Zn001:Flr001", "Zn001:Roof001")) {
    Assert-Contains -Text $sourceIdfText -Pattern $surfaceName -Description "official longwave enclosure surface $surfaceName"
}
Assert-Contains -Text $sourceIdfText -Pattern "0.9000000,               !- Thermal Absorptance" -Description "official material thermal absorptance"
Assert-Contains -Text $radiationSourceText -Pattern "InteriorLongwaveExchangeProbe" -Description "interior longwave diagnostic algorithm enum"
Assert-Contains -Text $radiationSourceText -Pattern "GreyEnergyPlusDirectViewFactor" -Description "grey approximate direct-view-factor longwave path"
Assert-Contains -Text $radiationSourceText -Pattern "EnergyPlusScriptF" -Description "EnergyPlus ScriptF longwave path"
Assert-Contains -Text $radiationSourceText -Pattern "EnergyPlusScriptFFlatAccess" -Description "EnergyPlus ScriptF flat access longwave path"
Assert-Contains -Text $radiationSourceText -Pattern "energyplus_approximate_view_factors" -Description "EnergyPlus approximate view-factor builder"
Assert-Contains -Text $radiationSourceText -Pattern "fix_energyplus_approximate_view_factors" -Description "EnergyPlus view-factor reciprocity/completeness correction"
Assert-Contains -Text $radiationSourceText -Pattern "if surface_count <= 3" -Description "EnergyPlus small-enclosure special case"
Assert-Contains -Text $radiationSourceText -Pattern "largest_area > 0.99 * (total_area - largest_area)" -Description "EnergyPlus large-surface-area special case"
Assert-Contains -Text $radiationSourceText -Pattern "for _ in 0..400" -Description "EnergyPlus view-factor correction iteration cap"
Assert-Contains -Text $radiationSourceText -Pattern "convergence_new <= 0.001" -Description "EnergyPlus view-factor correction convergence threshold"
Assert-Contains -Text $radiationSourceText -Pattern "thermal_absorptance: surface.inside_thermal_absorptance.clamp(0.0, 1.0)" -Description "inside thermal absorptance source"
Assert-Contains -Text $radiationSourceText -Pattern "energyplus_scriptf_from_view_factors" -Description "ScriptF longwave matrix solver"
Assert-Contains -Text $surfaceLoopSourceText -Pattern "use_current_inside_for_first_longwave" -Description "inside longwave first-pass temperature timing switch"
Assert-Contains -Text $timestepSourceText -Pattern "HeatBalanceZoneAirAlgorithm::EnergyPlusSourceOrder1ZoneOpaqueCompatibility" -Description "conformance longwave algorithm owner"
Assert-Contains -Text $timestepSourceText -Pattern "InteriorLongwaveExchangeProbe::EnergyPlusScriptFFlatAccess" -Description "conformance ScriptF flat-access longwave selection"
Assert-Contains -Text $surfaceBalanceSourceText -Pattern "SurfQdotRadNetLWInPerArea" -Description "EnergyPlus inside net longwave source term mapping"
Assert-Contains -Text $surfaceBalanceSourceText -Pattern "q_lwx_w_per_m2: surface.inside_net_longwave_w_per_m2" -Description "inside longwave source term field mapping"
$insideLongwaveSourceRows = @($summary.inside_source_term_series_summaries | Where-Object { $_.term_name -eq "inside-net-longwave" })
if ($insideLongwaveSourceRows.Count -ne 6) {
    throw "Expected six inside-net-longwave source-term summary rows, got $($insideLongwaveSourceRows.Count)"
}
foreach ($row in $insideLongwaveSourceRows) {
    if ([double]$row.area_residual_max_abs_w -gt 0.000000001) {
        throw "Inside longwave source-term area residual exceeds tolerance for $($row.key): $($row.area_residual_max_abs_w)"
    }
}
$insideLongwaveSeries = @($summary.series | Where-Object {
        $_.output.variable -eq "Surface Inside Face Net Surface Thermal Radiation Heat Gain Rate" `
            -and $_.output.class -eq "surface-state" `
            -and $_.output.level -eq "conformance" `
            -and $_.status -eq "extracted"
    })
if ($insideLongwaveSeries.Count -ne 6) {
    throw "Expected six inside longwave conformance series rows, got $($insideLongwaveSeries.Count)"
}
Assert-Contains -Text $reportText -Pattern "longwave-radiation-source-delta" -Description "interior longwave source-order report row"
Assert-Contains -Text $reportText -Pattern "inside-net-longwave-source" -Description "inside net longwave source report label"
$bottleneckStart = $reportText.IndexOf("## Bottlenecks", [System.StringComparison]::Ordinal)
if ($bottleneckStart -lt 0) {
    throw "Missing Bottlenecks section in compare report"
}
$bottleneckEnd = $reportText.IndexOf("`n## ", $bottleneckStart + 1, [System.StringComparison]::Ordinal)
if ($bottleneckEnd -lt 0) {
    $bottleneckEnd = $reportText.Length
}
$bottleneckSection = $reportText.Substring($bottleneckStart, $bottleneckEnd - $bottleneckStart)
$firstBottleneckRow = @($bottleneckSection -split "`r?`n" | Where-Object { $_ -match '^\| 1 \|' } | Select-Object -First 1)
if ($firstBottleneckRow.Count -ne 1) {
    throw "Missing first bottleneck row in compare report"
}
if ($firstBottleneckRow[0] -match [regex]::Escape("Surface Inside Face Net Surface Thermal Radiation Heat Gain Rate")) {
    throw "Inside longwave unexpectedly appears as the top RMSE bottleneck"
}
Assert-Contains -Text $sourceIdfText -Pattern "! People:                         None" -Description "official no People object evidence"
Assert-Contains -Text $sourceIdfText -Pattern "! Lights:                         None" -Description "official no Lights object evidence"
if ($sourceIdfText -match "(?im)^\s*People\s*,") {
    throw "Official 1Zone source IDF unexpectedly declares People"
}
if ($sourceIdfText -match "(?im)^\s*Lights\s*,") {
    throw "Official 1Zone source IDF unexpectedly declares Lights"
}
if ($sourceIdfText -match "(?im)^\s*ElectricEquipment\s*,") {
    throw "Official 1Zone source IDF unexpectedly declares ElectricEquipment"
}
$otherEquipmentDeclarations = [regex]::Matches($sourceIdfText, "(?im)^\s*OtherEquipment\s*,")
if ($otherEquipmentDeclarations.Count -ne 2) {
    throw "Expected two official 1Zone OtherEquipment objects, got $($otherEquipmentDeclarations.Count)"
}
Assert-Contains -Text $sourceIdfText -Pattern "Test 352a" -Description "official positive OtherEquipment object"
Assert-Contains -Text $sourceIdfText -Pattern "Test 352 minus" -Description "official negative OtherEquipment object"
Assert-Contains -Text $sourceIdfText -Pattern "None,                    !- Fuel Type" -Description "official OtherEquipment fuel type"
Assert-Contains -Text $sourceIdfText -Pattern "AlwaysOn,                !- Schedule Name" -Description "official OtherEquipment schedule"
Assert-Contains -Text $sourceIdfText -Pattern "EquipmentLevel,          !- Design Level Calculation Method" -Description "official OtherEquipment design-level method"
Assert-Contains -Text $sourceIdfText -Pattern "Schedule:Constant,AlwaysOn,On/Off,1.0" -Description "official AlwaysOn schedule value"
Assert-Contains -Text $eioText -Pattern "Zone Internal Gains Nominal, ZONE ONE,232.26,0.0,N/A,0.000,0.000,0.000,0.000,0.000" -Description "oracle zero zone internal gains nominal"
Assert-Contains -Text $eioText -Pattern "OtherEquipment Internal Gains Nominal, TEST 352A,ALWAYSON,ZONE ONE" -Description "oracle positive OtherEquipment nominal row"
Assert-Contains -Text $eioText -Pattern "OtherEquipment Internal Gains Nominal, TEST 352 MINUS,ALWAYSON,ZONE ONE" -Description "oracle negative OtherEquipment nominal row"
Assert-Contains -Text $eioText -Pattern "0.000,0.000,0.000,1.000" -Description "oracle latent/radiant/lost/convected fractions"
Assert-Contains -Text $internalGainsSourceText -Pattern "pub struct OtherEquipment" -Description "typed OtherEquipment object"
Assert-Contains -Text $internalGainsSourceText -Pattern "pub enum OtherEquipmentDesignLevelCalculationMethod" -Description "typed OtherEquipment design-level calculation method"
Assert-Contains -Text $internalGainsSourceText -Pattern "WattsPerZoneFloorArea" -Description "typed OtherEquipment W/m2 design-level method"
Assert-Contains -Text $internalGainsSourceText -Pattern "WattsPerPerson" -Description "typed OtherEquipment W/person design-level method"
Assert-Contains -Text $internalGainsSourceText -Pattern "pub fuel_type: NormalizedName" -Description "typed OtherEquipment fuel type"
Assert-Contains -Text $internalGainsSourceText -Pattern "pub design_level_calculation_method" -Description "typed OtherEquipment design-level calculation field"
Assert-Contains -Text $internalGainsSourceText -Pattern "pub design_level_w: f64" -Description "typed OtherEquipment design level"
Assert-Contains -Text $internalGainsSourceText -Pattern "pub power_per_floor_area_w_per_m2: f64" -Description "typed OtherEquipment floor-area design level"
Assert-Contains -Text $internalGainsSourceText -Pattern "pub power_per_person_w: f64" -Description "typed OtherEquipment per-person design level"
Assert-Contains -Text $internalGainsSourceText -Pattern "pub fraction_latent: f64" -Description "typed latent fraction"
Assert-Contains -Text $internalGainsSourceText -Pattern "pub fraction_radiant: f64" -Description "typed radiant fraction"
Assert-Contains -Text $internalGainsSourceText -Pattern "pub fraction_lost: f64" -Description "typed lost fraction"
Assert-Contains -Text $internalGainsSourceText -Pattern "pub carbon_dioxide_generation_rate_m3_per_s_w: f64" -Description "typed OtherEquipment CO2 generation rate"
Assert-Contains -Text $compilerSourceText -Pattern "parse_other_equipment_design_level_calculation_method" -Description "compiler OtherEquipment design-level method parser"
Assert-Contains -Text $compilerSourceText -Pattern "Power/Area" -Description "compiler OtherEquipment Power/Area design-level key"
Assert-Contains -Text $compilerSourceText -Pattern "Power/Person" -Description "compiler OtherEquipment Power/Person design-level key"
Assert-Contains -Text $compilerSourceText -Pattern "carbon_dioxide_generation_rate" -Description "compiler OtherEquipment CO2 generation rate parser"
Assert-Contains -Text $compilerSourceText -Pattern "InvalidOtherEquipmentFractionSum" -Description "compiler OtherEquipment fraction sum validation"
Assert-Contains -Text $compilerSourceText -Pattern "fraction_latent+fraction_radiant+fraction_lost" -Description "compiler OtherEquipment fraction sum fields"
Assert-Contains -Text $schedulesSourceText -Pattern "other_equipment_design_level_w(model, equipment) * schedule_multiplier" -Description "internal gain schedule multiplier"
Assert-Contains -Text $schedulesSourceText -Pattern "ZONE_TOTAL_INTERNAL_CONVECTIVE_HEATING_RATE_VARIABLE" -Description "zone total internal convective output variable source map"
Assert-Contains -Text $schedulesSourceText -Pattern "Zone Total Internal Radiant Heating Rate" -Description "zone total internal radiant output variable source map"
Assert-Contains -Text $schedulesSourceText -Pattern "OtherEquipmentDesignLevelCalculationMethod::WattsPerZoneFloorArea" -Description "internal gain W/m2 design-level calculation"
Assert-Contains -Text $schedulesSourceText -Pattern "OtherEquipmentDesignLevelCalculationMethod::WattsPerPerson" -Description "internal gain W/person design-level calculation"
Assert-Contains -Text $schedulesSourceText -Pattern "zone_people_design_count" -Description "internal gain people design count lookup"
Assert-NotContains -Text $schedulesSourceText -Pattern "fuel_type" -Description "fuel type in heat-balance internal gain calculation"
Assert-NotContains -Text $schedulesSourceText -Pattern "carbon_dioxide_generation_rate" -Description "CO2 generation in heat-balance internal gain calculation"
Assert-Contains -Text $schedulesSourceText -Pattern "equipment.fraction_latent - equipment.fraction_radiant - equipment.fraction_lost" -Description "convective fraction formula"
Assert-Contains -Text $schedulesSourceText -Pattern "radiant_internal_gain_for_equipment_w" -Description "radiant internal gain fraction path"
Assert-Contains -Text $schedulesSourceText -Pattern "simulate_zone_internal_radiant_gains" -Description "zone internal radiant gain trace"
Assert-Contains -Text $schedulesSourceText -Pattern "update_surface_radiant_internal_gain_source_terms" -Description "surface radiant internal gain distribution"
Assert-Contains -Text $schedulesSourceText -Pattern "precompute_schedule_value_series" -Description "hourly schedule precompute path"
Assert-Contains -Text $schedulesSourceText -Pattern "simulate_zone_internal_convective_gains" -Description "zone internal convective gain trace"
Assert-Contains -Text $reportText -Pattern "Surface Outside Face Outdoor Air Drybulb Temperature / hourly / weather / eso / conformance" -Description "roof local dry-bulb conformance output"
Assert-Contains -Text $reportText -Pattern "Surface Outside Face Outdoor Air Wetbulb Temperature / hourly / weather / eso / conformance" -Description "roof local wet-bulb conformance output"
Assert-Contains -Text $reportText -Pattern "Surface Outside Face Outdoor Air Wind Speed / hourly / weather / eso / conformance" -Description "roof local wind-speed conformance output"
Assert-Contains -Text $reportText -Pattern "Surface Outside Face Outdoor Air Wind Direction / hourly / weather / eso / conformance" -Description "roof local wind-direction conformance output"
Assert-Contains -Text $reportText -Pattern "Surface Inside Face Conduction Heat Transfer Rate per Area / hourly / surface-flux-state / eso / conformance" -Description "surface conduction per-area conformance output"
Assert-Contains -Text $reportText -Pattern "Surface Outside Face Incident Solar Radiation Rate per Area / hourly / surface-solar-flux-state / eso / conformance" -Description "incident total solar conformance output"
Assert-Contains -Text $reportText -Pattern "Surface Outside Face Incident Beam Solar Radiation Rate per Area / hourly / surface-solar-flux-state / eso / conformance" -Description "incident beam solar conformance output"
Assert-Contains -Text $reportText -Pattern "Surface Outside Face Solar Radiation Heat Gain Rate / hourly / surface-solar-rate-state / eso / conformance" -Description "absorbed solar heat gain rate conformance output"
Assert-Contains -Text $reportText -Pattern "Surface Outside Face Solar Radiation Heat Gain Rate per Area / hourly / surface-solar-flux-state / eso / conformance" -Description "absorbed solar heat gain per-area conformance output"
Assert-Contains -Text $reportText -Pattern "Surface Inside Face Convection Heat Transfer Coefficient / hourly / surface-coefficient-state / eso / conformance" -Description "inside convection coefficient conformance output"
Assert-Contains -Text $reportText -Pattern "Surface Outside Face Convection Heat Transfer Coefficient / hourly / surface-coefficient-state / eso / conformance" -Description "outside convection coefficient conformance output"
Assert-Contains -Text $reportText -Pattern "Surface Outside Face Thermal Radiation to Air Heat Transfer Coefficient / hourly / surface-coefficient-state / eso / conformance" -Description "outside longwave air coefficient conformance output"
Assert-Contains -Text $reportText -Pattern "Surface Outside Face Thermal Radiation to Sky Heat Transfer Coefficient / hourly / surface-coefficient-state / eso / conformance" -Description "outside longwave sky coefficient conformance output"
Assert-Contains -Text $reportText -Pattern "Surface Outside Face Thermal Radiation to Ground Heat Transfer Coefficient / hourly / surface-coefficient-state / eso / conformance" -Description "outside longwave ground coefficient conformance output"
Assert-Contains -Text $reportText -Pattern "Surface Outside Face Convection Heat Gain Rate / hourly / surface-exterior-rate-state / eso / conformance" -Description "outside convection heat-gain rate conformance output"
Assert-Contains -Text $reportText -Pattern "Surface Outside Face Net Thermal Radiation Heat Gain Rate / hourly / surface-exterior-rate-state / eso / conformance" -Description "outside net thermal radiation heat-gain rate conformance output"
Assert-Contains -Text $reportText -Pattern "Surface Outside Face Convection Heat Gain Rate per Area / hourly / surface-exterior-flux-state / eso / conformance" -Description "outside convection heat-gain per-area conformance output"
Assert-Contains -Text $reportText -Pattern "Surface Outside Face Net Thermal Radiation Heat Gain Rate per Area / hourly / surface-exterior-flux-state / eso / conformance" -Description "outside net thermal radiation heat-gain per-area conformance output"
Assert-Contains -Text $reportText -Pattern "Surface Inside Face Adjacent Air Temperature / hourly / surface-state / eso / conformance" -Description "inside adjacent-air temperature conformance output"
Assert-Contains -Text $reportText -Pattern "Surface Inside Face Convection Heat Gain Rate / hourly / surface-state / eso / conformance" -Description "inside convection source conformance output"
Assert-Contains -Text $reportText -Pattern "Surface Inside Face Net Surface Thermal Radiation Heat Gain Rate / hourly / surface-state / eso / conformance" -Description "inside radiation source conformance output"
Assert-Contains -Text $reportText -Pattern "## Inside Source Term Series Summaries" -Description "inside source-term summary section"
Assert-Contains -Text $reportText -Pattern "area_residual_max_abs_w" -Description "inside source-term area residual column"
Assert-Contains -Text $reportText -Pattern "## Floor Inside-Current Diagnostic" -Description "D3 floor inside-current diagnostic report section"
Assert-Contains -Text $reportText -Pattern "current_inside_delta_w" -Description "D3 floor inside-current delta report column"
Assert-Contains -Text $reportText -Pattern "temperature-timing" -Description "D3 floor inside-current temperature timing classification"
Assert-Contains -Text $reportText -Pattern "## Floor Inside-Current Term Series" -Description "D3 floor inside-current timestep series report section"
Assert-Contains -Text $reportText -Pattern "max_signed_delta_w" -Description "D3 floor inside-current term series summary"
Assert-Contains -Text $reportText -Pattern "total_src_w" -Description "surface first-sample total source column"
Assert-Contains -Text $reportText -Pattern "inside_hconv_branch" -Description "surface first-sample inside hconv branch column"
Assert-Contains -Text $reportText -Pattern "outside_hconv_branch" -Description "surface first-sample outside hconv branch column"
Assert-Contains -Text $reportText -Pattern "vertical-wall" -Description "surface first-sample wall hconv branch report"
Assert-Contains -Text $reportText -Pattern "doe2-windward" -Description "surface first-sample outside DOE-2 branch report"
Assert-Contains -Text $reportText -Pattern "surface_temp_sink_delta_w" -Description "inside solve surface-temperature sink column"
Assert-Contains -Text $reportText -Pattern "radiant-internal-gain" -Description "inside radiant internal gain source term row"
Assert-Contains -Text $reportText -Pattern "inside_hconv_reevaluation_interval: none" -Description "inside hconv no reevaluation interval report"
Assert-Contains -Text $reportText -Pattern "## Zone-Air Surface Coefficient Deltas" -Description "zone-air surface coefficient hconv report"
Assert-Contains -Text $reportText -Pattern "inside_hconv_rmse" -Description "inside hconv RMSE report column"
Assert-Contains -Text $reportText -Pattern "hconv-source-timing-delta" -Description "inside hconv timing blocker row"
Assert-Contains -Text $reportText -Pattern "## Inside Solve Series Deltas" -Description "outside boundary labels in solve series report"
Assert-Contains -Text $reportText -Pattern "adiabatic" -Description "adiabatic boundary label report"
Assert-Contains -Text $reportText -Pattern "outdoors" -Description "outdoors boundary label report"
Assert-Contains -Text $reportText -Pattern "## Adiabatic History Max-Sample Deltas" -Description "adiabatic floor outside-history report"
Assert-Contains -Text $reportText -Pattern "Surface Outside Face Incident Sky Diffuse Solar Radiation Rate per Area / hourly / surface-flux-state / eso / conformance" -Description "incident sky diffuse conformance output"
Assert-Contains -Text $reportText -Pattern "Surface Outside Face Incident Ground Diffuse Solar Radiation Rate per Area / hourly / surface-flux-state / eso / conformance" -Description "incident ground diffuse conformance output"
Assert-Contains -Text $reportText -Pattern "Zone Opaque Surface Outside Faces Conduction Rate / hourly / surface-aggregate-state / eso / conformance" -Description "zone opaque aggregate conduction conformance output"
Assert-Contains -Text $reportText -Pattern "Zone Opaque Surface Outside Faces Conduction Heat Gain Rate / hourly / surface-aggregate-state / eso / conformance" -Description "zone opaque aggregate outside gain conformance output"
Assert-Contains -Text $reportText -Pattern "Zone Opaque Surface Outside Faces Conduction Heat Loss Rate / hourly / surface-aggregate-state / eso / conformance" -Description "zone opaque aggregate outside loss conformance output"
Assert-Contains -Text $reportText -Pattern "Surface Heat Storage Rate / hourly / surface-storage-state / eso / conformance" -Description "surface storage conformance output"
Assert-Contains -Text $reportText -Pattern "Surface Heat Storage Rate per Area / hourly / surface-storage-flux-state / eso / conformance" -Description "surface storage per-area conformance output"
Assert-Contains -Text $reportText -Pattern "Surface Inside Face Heat Balance Calculation Iteration Count / hourly / surface-iteration-count-state / eso / conformance" -Description "surface iteration count conformance output"
Assert-Contains -Text $reportText -Pattern "status: pass" -Description "markdown status"

Write-Host "Official dynamic heat-balance conformance gate passed."
