[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$OutputRoot = Join-Path $RepoRoot ".runtime\ideal-loads-non-constant-fuel-efficiency-conformance\26.1.0"
$CaseId = "ideal_loads_non_constant_fuel_efficiency_conformance_candidate_001"
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

function Read-JsonFile {
    param([Parameter(Mandatory = $true)][string]$Path)
    Get-Content -LiteralPath $Path -Encoding UTF8 -Raw | ConvertFrom-Json
}

function Read-TextFile {
    param([Parameter(Mandatory = $true)][string]$Path)
    Get-Content -LiteralPath $Path -Encoding UTF8 -Raw
}

function Read-CsvFile {
    param([Parameter(Mandatory = $true)][string]$Path)
    Get-Content -LiteralPath $Path -Encoding UTF8 | ConvertFrom-Csv
}

foreach ($path in @(
    (Join-Path $OracleRoot "energyplus.exe"),
    (Join-Path $OracleRoot "ConvertInputFormat.exe"),
    (Join-Path $OracleRoot "WeatherData\USA_CO_Golden-NREL.724666_TMY3.epw"),
    $CasePath
)) {
    Assert-FileExists -Path $path -Description "required IdealLoads compare input"
}

Remove-RepoDirectory -Path $CaseOutputRoot

$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

Write-Host "Generating IdealLoads non-constant fuel-efficiency conformance comparison artifacts."
$output = & $cargo.Source run -p ep_cli --quiet -- conformance ideal-loads-no-oa-sensible-report $CasePath $OracleRoot $OutputRoot 2>&1
if ($LASTEXITCODE -ne 0) {
    $output | ForEach-Object { Write-Host $_ }
    throw "IdealLoads non-constant fuel-efficiency conformance comparison failed."
}

$text = ($output -join "`n")
Assert-Contains -Text $text -Pattern "IdealLoads No-OA Sensible Report" -Description "report header"
Assert-Contains -Text $text -Pattern "id: $CaseId" -Description "case id"
Assert-Contains -Text $text -Pattern "comparison_class: conformance" -Description "comparison class"
Assert-Contains -Text $text -Pattern "conformance_claim: true" -Description "claim boundary"
Assert-Contains -Text $text -Pattern "series: 12" -Description "series count"
Assert-Contains -Text $text -Pattern "samples: 110" -Description "detailed sample count"
Assert-Contains -Text $text -Pattern "tolerance_policy: conformance-gate" -Description "tolerance policy"
Assert-Contains -Text $text -Pattern "status: pass" -Description "conformance status"

$summaryPath = Join-Path $CompareRoot "compare-summary.json"
$reportPath = Join-Path $CompareRoot "compare-report.md"
$selectedOutputsPath = Join-Path $CompareRoot "selected_outputs.json"
$resultStorePath = Join-Path $CompareRoot "rust-result-store.json"
$variableDeltasPath = Join-Path $CompareRoot "variable-deltas.csv"
$firstDivergencePath = Join-Path $CompareRoot "first-divergence.csv"
$toleranceFailuresPath = Join-Path $CompareRoot "tolerance-failures.csv"
$stageSummaryPath = Join-Path $CompareRoot "stage-summary.json"
$oracleMtrPath = Join-Path (Join-Path $CaseOutputRoot "oracle") "eplusout.mtr"

Assert-FileExists -Path $summaryPath -Description "IdealLoads compare summary"
Assert-FileExists -Path $reportPath -Description "IdealLoads markdown report"
Assert-FileExists -Path $selectedOutputsPath -Description "IdealLoads oracle selected outputs"
Assert-FileExists -Path $resultStorePath -Description "IdealLoads Rust result store"
Assert-FileExists -Path $variableDeltasPath -Description "IdealLoads variable deltas"
Assert-FileExists -Path $firstDivergencePath -Description "IdealLoads first divergence CSV"
Assert-FileExists -Path $toleranceFailuresPath -Description "IdealLoads tolerance failures CSV"
Assert-FileExists -Path $stageSummaryPath -Description "IdealLoads stage summary"
Assert-FileExists -Path $oracleMtrPath -Description "IdealLoads oracle MTR"

$summary = Read-JsonFile -Path $summaryPath
if ($summary.case_id -ne $CaseId) {
    throw "Unexpected case_id: $($summary.case_id)"
}
if ($summary.comparison_class -ne "conformance") {
    throw "Unexpected comparison_class: $($summary.comparison_class)"
}
if ($summary.conformance_claim -ne $true) {
    throw "IdealLoads non-constant fuel-efficiency conformance summary must set conformance_claim=true"
}
if ($summary.status -ne "pass") {
    throw "Unexpected IdealLoads non-constant fuel-efficiency conformance status: $($summary.status)"
}
if ($summary.tolerance_policy -ne "conformance-gate") {
    throw "Unexpected tolerance policy: $($summary.tolerance_policy)"
}
if ($summary.tolerance_failures -ne 0) {
    throw "IdealLoads non-constant fuel-efficiency conformance should have zero tolerance failures: $($summary.tolerance_failures)"
}
if ($summary.samples -ne 110) {
    throw "Unexpected IdealLoads sample count: $($summary.samples)"
}
if ($summary.series_count -ne 12) {
    throw "Unexpected IdealLoads series count: $($summary.series_count)"
}
if ([Math]::Abs([double]$summary.heating_fuel_efficiency - 0.8) -gt 1.0e-12) {
    throw "Unexpected heating fuel efficiency representative: $($summary.heating_fuel_efficiency)"
}
if ([Math]::Abs([double]$summary.cooling_fuel_efficiency - 0.75) -gt 1.0e-12) {
    throw "Unexpected cooling fuel efficiency representative: $($summary.cooling_fuel_efficiency)"
}
if ($summary.fuel_energy_rate_source -ne "EnergyPlus ReportPurchasedAir non-constant Schedule:Compact fuel-efficiency schedule branch") {
    throw "Unexpected fuel energy rate source: $($summary.fuel_energy_rate_source)"
}
if ($summary.fuel_energy_rate_rust_source -ne "rust-ideal-loads-non-constant-fuel-efficiency") {
    throw "Unexpected fuel energy rate Rust source: $($summary.fuel_energy_rate_rust_source)"
}
if ($summary.fuel_energy_rust_source -ne "rust-ideal-loads-non-constant-fuel-efficiency-time-step-energy") {
    throw "Unexpected fuel energy Rust source: $($summary.fuel_energy_rust_source)"
}
if ($summary.fuel_energy_output_level_policy -ne "conformance for declared no-OA non-constant Schedule:Compact fuel-efficiency rows only") {
    throw "Unexpected fuel energy output level policy: $($summary.fuel_energy_output_level_policy)"
}
if ($summary.energy_output_level_policy -ne "diagnostic-only until rate-to-energy parity is separately proven") {
    throw "Unexpected energy output level policy: $($summary.energy_output_level_policy)"
}
if ($summary.zone_demand_synthetic_rc_model -ne $false) {
    throw "IdealLoads non-constant fuel-efficiency conformance must not synthesize zone demand from an RC shortcut"
}
if ($summary.rate_output_source -ne "ReportPurchasedAir after UpdatePurchasedAir") {
    throw "Unexpected rate output source: $($summary.rate_output_source)"
}
if ($summary.energy_output_timestep_source -ne "ReportPurchasedAir rate * TimeStepSysSec") {
    throw "Unexpected energy output timestep source: $($summary.energy_output_timestep_source)"
}
if ($summary.requested_meter_count -ne 2 -or $summary.meter_series_count -ne 2) {
    throw "Expected 2 requested diagnostic meter rows, found requested=$($summary.requested_meter_count) series=$($summary.meter_series_count)"
}
if ($summary.meter_tolerance_failures -ne 0) {
    throw "Expected zero meter tolerance failures, found $($summary.meter_tolerance_failures)"
}
$meterRows = @($summary.meter_series)
if (@($meterRows | Where-Object { $_.status -ne "pass" }).Count -ne 0) {
    throw "All IdealLoads non-constant fuel-efficiency facility meter diagnostics must pass"
}

$conformanceRows = @($summary.series | Where-Object { $_.level -eq "conformance" })
if ($conformanceRows.Count -ne 8) {
    throw "Expected 8 conformance-level non-constant fuel-efficiency rows, found $($conformanceRows.Count)"
}
if (@($conformanceRows | Where-Object { $_.status -ne "pass" }).Count -ne 0) {
    throw "All IdealLoads non-constant fuel-efficiency conformance rows must pass"
}
$diagnosticRows = @($summary.series | Where-Object { $_.level -eq "diagnostic" })
if ($diagnosticRows.Count -ne 4) {
    throw "Expected 4 diagnostic raw rate rows, found $($diagnosticRows.Count)"
}
if (@($diagnosticRows | Where-Object { $_.status -ne "pass" }).Count -ne 0) {
    throw "All IdealLoads non-constant fuel-efficiency diagnostic rows must pass"
}
$rateRows = @($summary.series | Where-Object { $_.variable -like "*Fuel Energy Rate" })
if ($rateRows.Count -ne 4) {
    throw "Expected 4 fuel energy-rate rows, found $($rateRows.Count)"
}
if (@($rateRows | Where-Object { $_.level -ne "conformance" -or $_.rust_source -ne "rust-ideal-loads-non-constant-fuel-efficiency" }).Count -ne 0) {
    throw "Fuel energy-rate rows must be conformance rows using the non-constant fuel-efficiency Rust source"
}
$fuelEnergyRows = @($summary.series | Where-Object { $_.variable -like "*Fuel Energy" -and $_.variable -notlike "*Energy Rate" })
if ($fuelEnergyRows.Count -ne 4) {
    throw "Expected 4 fuel energy rows, found $($fuelEnergyRows.Count)"
}
if (@($fuelEnergyRows | Where-Object { $_.level -ne "conformance" -or $_.rust_source -ne "rust-ideal-loads-non-constant-fuel-efficiency-time-step-energy" }).Count -ne 0) {
    throw "Fuel energy rows must be conformance rows using the non-constant fuel-efficiency TimeStepSysSec source"
}
if (@($fuelEnergyRows | Where-Object { $_.units -ne "J" }).Count -ne 0) {
    throw "Fuel energy rows must use joule units"
}
$rawRateRows = @($summary.series | Where-Object { $_.variable -notlike "*Fuel Energy*" })
if ($rawRateRows.Count -ne 4) {
    throw "Expected 4 raw IdealLoads rate rows, found $($rawRateRows.Count)"
}
if (@($rawRateRows | Where-Object { $_.level -ne "diagnostic" -or $_.rust_source -ne "rust-ideal-loads-no-oa-sensible-calc" }).Count -ne 0) {
    throw "Raw IdealLoads rate rows must remain diagnostic and use the no-OA sensible Rust source"
}

$toleranceFailures = @(Read-CsvFile -Path $toleranceFailuresPath)
if ($toleranceFailures.Count -ne 0) {
    throw "Expected empty tolerance-failures.csv, found $($toleranceFailures.Count) row(s)"
}

$resultStore = Read-JsonFile -Path $resultStorePath
if ($resultStore.series_count -ne 12 -or $resultStore.sample_count -ne 110) {
    throw "Unexpected result store shape: series=$($resultStore.series_count) samples=$($resultStore.sample_count)"
}

$selectedOutputs = Read-JsonFile -Path $selectedOutputsPath
if (@($selectedOutputs.series).Count -ne 12) {
    throw "Unexpected selected_outputs series count: $(@($selectedOutputs.series).Count)"
}

$stageSummary = Read-JsonFile -Path $stageSummaryPath
if ($stageSummary.branch -ne "no-oa-no-limit-sensible") {
    throw "Unexpected IdealLoads branch: $($stageSummary.branch)"
}
if ([Math]::Abs([double]$stageSummary.heating_fuel_efficiency - 0.8) -gt 1.0e-12) {
    throw "Unexpected stage heating fuel efficiency representative: $($stageSummary.heating_fuel_efficiency)"
}
if ([Math]::Abs([double]$stageSummary.cooling_fuel_efficiency - 0.75) -gt 1.0e-12) {
    throw "Unexpected stage cooling fuel efficiency representative: $($stageSummary.cooling_fuel_efficiency)"
}
if ($stageSummary.fuel_energy_output_level_policy -ne "conformance for declared no-OA non-constant Schedule:Compact fuel-efficiency rows only") {
    throw "Unexpected stage fuel energy output level policy: $($stageSummary.fuel_energy_output_level_policy)"
}

$oracleMtrText = Read-TextFile -Path $oracleMtrPath
Assert-Contains -Text $oracleMtrText -Pattern "DistrictHeatingWater:Facility" -Description "oracle MTR heating meter"
Assert-Contains -Text $oracleMtrText -Pattern "DistrictCooling:Facility" -Description "oracle MTR cooling meter"

$reportText = Read-TextFile -Path $reportPath
Assert-Contains -Text $reportText -Pattern "claim_boundary: conformance no-OA non-constant Schedule:Compact IdealLoads fuel-efficiency for declared fuel-energy rows only" -Description "markdown claim boundary"
Assert-Contains -Text $reportText -Pattern "source_order_wrapper: ep_runtime::ideal_loads::sim_purchased_air_compat" -Description "markdown source-order wrapper"
Assert-Contains -Text $reportText -Pattern "ideal_loads_invocation_path: zone-equipment-validated source-order PurchasedAir wrapper" -Description "markdown IdealLoads invocation path"
Assert-Contains -Text $reportText -Pattern "direct_calc_helper_invocation: false" -Description "markdown direct calc helper invocation"
Assert-Contains -Text $reportText -Pattern "zone_equipment_dispatch_execution_boundary: validated typed ZoneEquipmentManager path; report generator invokes source-order PurchasedAir wrapper" -Description "markdown zone-equipment execution boundary"
Assert-Contains -Text $reportText -Pattern "ideal_loads_runtime_binding_source: compile-stage typed IdealLoadsAirSystemId, ZoneId, and NodeId binding" -Description "markdown typed-ID binding source"
Assert-Contains -Text $reportText -Pattern "purchased_air_name_lookup_policy: PurchAirName string lookup is compile/report only; simulation loop uses prebound typed IDs" -Description "markdown runtime string lookup policy"
Assert-Contains -Text $reportText -Pattern "ideal_loads_feature_flags:" -Description "markdown IdealLoads feature flags"
Assert-Contains -Text $reportText -Pattern "ideal_loads_feature_dispatch_policy: compile feature flags select branch-specific source-order compat functions; unsupported active feature combinations emit diagnostics instead of approximate fallback" -Description "markdown IdealLoads feature dispatch policy"
Assert-Contains -Text $reportText -Pattern "ideal_loads_prebound_id_contract: compile-stage IdealLoadsAirSystemId, ZoneId, supply NodeId, return NodeId, zone air NodeId, optional outdoor air NodeId, availability ScheduleId, heating availability ScheduleId, and cooling availability ScheduleId" -Description "markdown IdealLoads prebound ID contract"
Assert-Contains -Text $reportText -Pattern "ideal_loads_psychrometric_evaluation_policy: compatibility reports use source-order direct psychrometric evaluation; no cross-timestep cache or reordering is enabled" -Description "markdown IdealLoads psychrometric evaluation policy"
Assert-Contains -Text $reportText -Pattern "ideal_loads_psychrometric_cache_policy: future compatibility cache must key exact temperature, humidity ratio, and pressure tuple and preserve EnergyPlus evaluation order" -Description "markdown IdealLoads psychrometric cache policy"
Assert-Contains -Text $reportText -Pattern "trace_level_source: case manifest [trace].level" -Description "markdown trace level source"
Assert-Contains -Text $reportText -Pattern "trace_result_invariance_policy: trace level selects evidence payload only; ResultStore values are computed before report serialization" -Description "markdown trace result invariance policy"
Assert-Contains -Text $reportText -Pattern "trace_overhead_accounting: trace/report serialization overhead is outside numerical conformance comparison and measured separately from simulation results" -Description "markdown trace overhead accounting"
Assert-Contains -Text $reportText -Pattern "selected_purchased_air_branch: constant_shr" -Description "markdown PurchasedAir branch"
Assert-Contains -Text $reportText -Pattern "declared_ideal_loads_branch: no_oa_sensible" -Description "markdown declared branch"
Assert-Contains -Text $reportText -Pattern "inactive_branches: outdoor_air, economizer, heat_recovery, humidistat, dcv, autosizing, saturation_limit" -Description "markdown inactive branches"
Assert-Contains -Text $reportText -Pattern "source_map_anchor: docs/src/porting-map/ideal-loads-source-map.md" -Description "markdown source-map anchor"
Assert-Contains -Text $reportText -Pattern "node_output_timestamp_alignment: timestamp" -Description "markdown node timestamp alignment"
Assert-Contains -Text $reportText -Pattern "purchased_air_source_order: GetPurchasedAir -> InitPurchasedAir -> CalcPurchAirLoads -> UpdatePurchasedAir -> ReportPurchasedAir" -Description "markdown PurchasedAir source order"
Assert-Contains -Text $reportText -Pattern "fuel_energy_rate_source: EnergyPlus ReportPurchasedAir non-constant Schedule:Compact fuel-efficiency schedule branch" -Description "markdown fuel source"
Assert-Contains -Text $reportText -Pattern "fuel_efficiency: heating=0.800000000000 cooling=0.750000000000" -Description "markdown fuel efficiency representative values"
Assert-Contains -Text $reportText -Pattern "fuel_energy_output_level_policy: conformance for declared no-OA non-constant Schedule:Compact fuel-efficiency rows only" -Description "markdown fuel energy output level policy"
Assert-Contains -Text $reportText -Pattern "meter_source: EnergyPlus Output:Meter hourly MTR vs Rust aggregated fuel-energy diagnostic; rust_meter_time_series_comparison=true requested_meters=2" -Description "markdown meter source"
Assert-Contains -Text $reportText -Pattern "| DistrictHeatingWater:Facility | diagnostic | meter | hourly | mtr | rust-ideal-loads-hourly-facility-meter-from-fuel-energy" -Description "markdown heating meter row"
Assert-Contains -Text $reportText -Pattern "| DistrictCooling:Facility | diagnostic | meter | hourly | mtr | rust-ideal-loads-hourly-facility-meter-from-fuel-energy" -Description "markdown cooling meter row"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE IDEAL LOADS | Zone Ideal Loads Zone Heating Fuel Energy Rate | conformance" -Description "markdown zone heating fuel rate row"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE IDEAL LOADS | Zone Ideal Loads Zone Heating Fuel Energy | conformance" -Description "markdown zone heating fuel energy row"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE IDEAL LOADS | Zone Ideal Loads Zone Total Heating Rate | diagnostic" -Description "markdown raw rate diagnostic row"

Write-Host "IdealLoads non-constant fuel-efficiency conformance comparison artifacts generated."
