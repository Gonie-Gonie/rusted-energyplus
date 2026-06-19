[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)][string]$CaseId
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$OutputRoot = Join-Path $RepoRoot ".runtime\ideal-loads-humidity-annual-meter-conformance\26.1.0"
$CasePath = Join-Path $RepoRoot "data\conformance_cases\$CaseId\case.toml"
$CaseOutputRoot = Join-Path $OutputRoot $CaseId
$CompareRoot = Join-Path $CaseOutputRoot "compare"

$ExpectedBranches = @{
    "ideal_loads_constant_supply_humidity_cooling_annual_meter_conformance_candidate_001" = "constant_supply_humidity_cooling"
    "ideal_loads_constant_supply_humidity_heating_annual_meter_conformance_candidate_001" = "constant_supply_humidity_heating"
    "ideal_loads_humidistat_dehumidification_annual_meter_conformance_candidate_001" = "humidistat_dehumidification"
    "ideal_loads_humidistat_humidification_annual_meter_conformance_candidate_001" = "humidistat_humidification"
}

if (-not $ExpectedBranches.ContainsKey($CaseId)) {
    throw "Unsupported humidity annual meter conformance CaseId: $CaseId"
}

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

function Assert-PurchasedAirSourceOrder {
    param([Parameter(Mandatory = $true)]$StageSummary)

    $purchasedAirStages = @($StageSummary.purchased_air_stages)
    $expectedPurchasedAirRoutines = @(
        "GetPurchasedAir",
        "InitPurchasedAir",
        "CalcPurchAirLoads",
        "UpdatePurchasedAir",
        "ReportPurchasedAir"
    )
    if ($purchasedAirStages.Count -ne $expectedPurchasedAirRoutines.Count) {
        throw "Expected $($expectedPurchasedAirRoutines.Count) PurchasedAir stages, found $($purchasedAirStages.Count)"
    }
    for ($stageIndex = 0; $stageIndex -lt $expectedPurchasedAirRoutines.Count; $stageIndex++) {
        $actualRoutine = $purchasedAirStages[$stageIndex].source_routine
        if ($actualRoutine -ne $expectedPurchasedAirRoutines[$stageIndex]) {
            throw "Unexpected PurchasedAir stage at index ${stageIndex}: $actualRoutine"
        }
    }
    Write-Host "OK purchased_air_source_order: $($expectedPurchasedAirRoutines -join ' -> ')"
}

function Assert-ZoneEquipmentDispatch {
    param([Parameter(Mandatory = $true)]$StageSummary)

    $expectedPath = "ZoneEquipmentManager::ManageZoneEquipment -> SimZoneEquipment -> ZoneEquipType::PurchasedAir -> PurchasedAirManager::SimPurchasedAir"
    if ($StageSummary.zone_equipment_dispatch_path -ne $expectedPath) {
        throw "Unexpected zone equipment dispatch path: $($StageSummary.zone_equipment_dispatch_path)"
    }
    if ($StageSummary.zone_equipment_dispatch_validation -ne "pass") {
        throw "Zone equipment dispatch validation did not pass: $($StageSummary.zone_equipment_dispatch_validation)"
    }
    if ($StageSummary.zone_equipment_conformance_candidate -ne "pass") {
        throw "Zone equipment dispatch is not a conformance candidate: $($StageSummary.zone_equipment_conformance_candidate)"
    }
    if (@($StageSummary.zone_equipment_dispatch_issues).Count -ne 0) {
        throw "Unexpected zone equipment dispatch issues: $($StageSummary.zone_equipment_dispatch_issues -join ', ')"
    }
    if (@($StageSummary.zone_equipment_dispatch_warnings).Count -ne 0) {
        throw "Unexpected zone equipment dispatch warnings: $($StageSummary.zone_equipment_dispatch_warnings -join ', ')"
    }
    Write-Host "OK zone equipment dispatch path: $expectedPath"
}

foreach ($required in @($CasePath, $OracleRoot)) {
    if (-not (Test-Path -LiteralPath $required)) {
        throw "Missing required IdealLoads humidity annual meter compare input: $required"
    }
}

Remove-RepoDirectory -Path $CaseOutputRoot

Write-Host "Generating IdealLoads humidity-control annual meter conformance candidate artifacts for $CaseId."
$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($null -eq $cargo) {
    throw "cargo was not found. Run .\scripts\dev.cmd setup -InstallRust first."
}

$output = & $cargo.Source run -p ep_cli --quiet -- conformance ideal-loads-no-oa-sensible-report $CasePath $OracleRoot $OutputRoot 2>&1
if ($LASTEXITCODE -ne 0) {
    Write-Host ($output -join [Environment]::NewLine)
    throw "IdealLoads humidity-control annual meter conformance candidate comparison failed."
}
$text = $output -join [Environment]::NewLine
Assert-Contains -Text $text -Pattern "IdealLoads No-OA Sensible Report" -Description "report header"
Assert-Contains -Text $text -Pattern "comparison_class: conformance" -Description "comparison_class"
Assert-Contains -Text $text -Pattern "conformance_claim: true" -Description "conformance_claim"
Assert-Contains -Text $text -Pattern "tolerance_policy: conformance-gate" -Description "tolerance policy"
Assert-Contains -Text $text -Pattern "status: pass" -Description "status"

$summaryPath = Join-Path $CompareRoot "compare-summary.json"
$reportPath = Join-Path $CompareRoot "compare-report.md"
$selectedOutputsPath = Join-Path $CompareRoot "selected_outputs.json"
$resultStorePath = Join-Path $CompareRoot "rust-result-store.json"
$variableDeltasPath = Join-Path $CompareRoot "variable-deltas.csv"
$firstDivergencePath = Join-Path $CompareRoot "first-divergence.csv"
$toleranceFailuresPath = Join-Path $CompareRoot "tolerance-failures.csv"
$stageSummaryPath = Join-Path $CompareRoot "stage-summary.json"

Assert-FileExists -Path $summaryPath -Description "IdealLoads humidity annual meter compare-summary.json"
Assert-FileExists -Path $reportPath -Description "IdealLoads humidity annual meter compare-report.md"
Assert-FileExists -Path $selectedOutputsPath -Description "IdealLoads humidity annual meter selected outputs"
Assert-FileExists -Path $resultStorePath -Description "IdealLoads humidity annual meter Rust result store"
Assert-FileExists -Path $variableDeltasPath -Description "IdealLoads humidity annual meter variable deltas"
Assert-FileExists -Path $firstDivergencePath -Description "IdealLoads humidity annual meter first divergence"
Assert-FileExists -Path $toleranceFailuresPath -Description "IdealLoads humidity annual meter tolerance-failures.csv"
Assert-FileExists -Path $stageSummaryPath -Description "IdealLoads humidity annual meter stage-summary.json"

$summary = Read-JsonFile -Path $summaryPath
$stageSummary = Read-JsonFile -Path $stageSummaryPath
$reportText = Read-TextFile -Path $reportPath

if ($summary.case_id -ne $CaseId) {
    throw "Unexpected case_id: $($summary.case_id)"
}
if ($summary.comparison_class -ne "conformance") {
    throw "Unexpected comparison_class: $($summary.comparison_class)"
}
if ($summary.conformance_claim -ne $true) {
    throw "Summary must set conformance_claim=true"
}
if ($summary.status -ne "pass") {
    throw "Unexpected status: $($summary.status)"
}
if ($summary.tolerance_failures -ne 0) {
    throw "Expected zero tolerance_failures, got $($summary.tolerance_failures)"
}
if (($summary.PSObject.Properties.Name -contains "meter_tolerance_failures") -and $summary.meter_tolerance_failures -ne 0) {
    throw "Expected zero meter_tolerance_failures, got $($summary.meter_tolerance_failures)"
}
if ($summary.series_count -ne 2) {
    throw "Expected 2 diagnostic thermostat output series, found $($summary.series_count)"
}

$rows = @($summary.series)
$conformanceRows = @($rows | Where-Object { $_.level -eq "conformance" })
$diagnosticRows = @($rows | Where-Object { $_.level -eq "diagnostic" })
if ($conformanceRows.Count -ne 0) {
    throw "Annual meter-only candidate must not promote ESO output rows to conformance"
}
if ($diagnosticRows.Count -ne 2) {
    throw "Expected 2 diagnostic thermostat rows, found $($diagnosticRows.Count)"
}
if (@($diagnosticRows | Where-Object { $_.status -ne "pass" }).Count -ne 0) {
    throw "All diagnostic thermostat rows must pass"
}

if ($summary.requested_meter_count -ne 2) {
    throw "Expected 2 requested annual conformance meters, found $($summary.requested_meter_count)"
}
$requestedMeters = @($summary.requested_meters)
if ($requestedMeters.Count -ne 2) {
    throw "Expected 2 requested_meters entries, found $($requestedMeters.Count)"
}
foreach ($requested in $requestedMeters) {
    if ($requested.level -ne "conformance" -or $requested.frequency -ne "annual" -or $requested.source -ne "mtr") {
        throw "Unexpected requested meter contract: $($requested | ConvertTo-Json -Compress)"
    }
}

$meterRows = @($summary.meter_series)
if ($summary.meter_series_count -ne 2 -or $meterRows.Count -ne 2) {
    throw "Expected 2 annual meter series, found count=$($summary.meter_series_count) rows=$($meterRows.Count)"
}
foreach ($meterRow in $meterRows) {
    if ($meterRow.level -ne "conformance") {
        throw "Annual meter row must be conformance-level: $($meterRow.name)"
    }
    if ($meterRow.frequency -ne "annual") {
        throw "Annual meter row used unexpected frequency: $($meterRow.frequency)"
    }
    if ($meterRow.expected_samples -ne 1 -or $meterRow.observed_samples -ne 1 -or $meterRow.compared_samples -ne 1) {
        throw "Annual meter row must compare one full-year sample: $($meterRow.name)"
    }
    if ($meterRow.status -ne "pass") {
        throw "Annual meter row did not pass: $($meterRow.name) max_abs=$($meterRow.max_abs_delta) rmse=$($meterRow.rmse_delta) max_rel=$($meterRow.max_rel_delta)"
    }
}

$expectedBranch = $ExpectedBranches[$CaseId]
if ($summary.selected_purchased_air_branch -ne $expectedBranch) {
    throw "Unexpected selected_purchased_air_branch: $($summary.selected_purchased_air_branch)"
}
if ($summary.declared_ideal_loads_branch -ne $expectedBranch) {
    throw "Unexpected declared_ideal_loads_branch: $($summary.declared_ideal_loads_branch)"
}
if ($summary.source_order_wrapper -ne "ep_runtime::ideal_loads::sim_purchased_air_compat") {
    throw "Unexpected source_order_wrapper: $($summary.source_order_wrapper)"
}
if ($summary.energy_output_level_policy -ne "diagnostic-only until rate-to-energy parity is separately proven") {
    throw "Annual meter-only candidate must keep energy outputs diagnostic-only"
}
if ($summary.fuel_energy_output_level_policy -ne "diagnostic-only until fuel-efficiency path is separately proven") {
    throw "Annual meter-only candidate must keep fuel-energy outputs diagnostic-only"
}
if ($summary.meter_source -ne "EnergyPlus Output:Meter annual full-year MTR vs Rust aggregated fuel-energy conformance") {
    throw "Unexpected meter_source: $($summary.meter_source)"
}
if ($summary.meter_aggregation_source -ne "ep_runtime::RuntimeMeterRegistry") {
    throw "Unexpected meter_aggregation_source: $($summary.meter_aggregation_source)"
}

Assert-PurchasedAirSourceOrder -StageSummary $stageSummary
Assert-ZoneEquipmentDispatch -StageSummary $stageSummary

Assert-Contains -Text $reportText -Pattern "claim_boundary: conformance no-OA full-year humidity-control annual IdealLoads facility meter aggregation for declared facility meters only" -Description "markdown claim boundary"
Assert-Contains -Text $reportText -Pattern "meter_source: EnergyPlus Output:Meter annual full-year MTR vs Rust aggregated fuel-energy conformance; rust_meter_time_series_comparison=true requested_meters=2" -Description "markdown meter source"
Assert-Contains -Text $reportText -Pattern "source_order_wrapper: ep_runtime::ideal_loads::sim_purchased_air_compat" -Description "markdown source_order_wrapper:"
Assert-Contains -Text $reportText -Pattern "ideal_loads_invocation_path: zone-equipment-validated source-order PurchasedAir wrapper" -Description "markdown IdealLoads invocation path"
Assert-Contains -Text $reportText -Pattern "direct_calc_helper_invocation: false" -Description "markdown direct calc helper"
Assert-Contains -Text $reportText -Pattern "zone_equipment_dispatch_execution_boundary: validated typed ZoneEquipmentManager path; report generator invokes source-order PurchasedAir wrapper" -Description "markdown dispatch boundary"
Assert-Contains -Text $reportText -Pattern "ideal_loads_runtime_binding_source: compile-stage typed IdealLoadsAirSystemId, ZoneId, and NodeId binding" -Description "markdown runtime binding"
Assert-Contains -Text $reportText -Pattern "purchased_air_name_lookup_policy: PurchAirName string lookup is compile/report only; simulation loop uses prebound typed IDs" -Description "markdown name lookup"
Assert-Contains -Text $reportText -Pattern "ideal_loads_feature_flags:" -Description "markdown ideal_loads_feature_flags"
Assert-Contains -Text $reportText -Pattern "ideal_loads_feature_dispatch_policy: compile feature flags select branch-specific source-order compat functions; unsupported active feature combinations emit diagnostics instead of approximate fallback" -Description "markdown feature dispatch"
Assert-Contains -Text $reportText -Pattern "ideal_loads_prebound_id_contract: compile-stage IdealLoadsAirSystemId, ZoneId, supply NodeId, return NodeId, zone air NodeId, optional outdoor air NodeId, availability ScheduleId, heating availability ScheduleId, and cooling availability ScheduleId" -Description "markdown prebound ID contract"
Assert-Contains -Text $reportText -Pattern "ideal_loads_psychrometric_evaluation_policy: compatibility reports use source-order direct psychrometric evaluation with EnergyPlus Psat cache-temperature quantization; no reordering is enabled" -Description "markdown psychrometric policy"
Assert-Contains -Text $reportText -Pattern "ideal_loads_psychrometric_cache_policy: saturation-pressure evaluation mirrors EnergyPlus default PsyPsatFnTemp cache temperature-key truncation before the raw polynomial" -Description "markdown psychrometric cache"
Assert-Contains -Text $reportText -Pattern "ideal_loads_output_handle_registration_policy: manifest output requests are resolved to stable OutputHandle values before IdealLoads comparison rows are evaluated" -Description "markdown handle registration"
Assert-Contains -Text $reportText -Pattern "ideal_loads_output_handle_write_policy: rate and node ResultStore series use pre-resolved OutputHandle values; meter rows use RuntimeMeterRegistry-resolved handles before aggregation" -Description "markdown handle write"
Assert-Contains -Text $reportText -Pattern "ideal_loads_diagnostic_output_request_policy: diagnostic rows are emitted only for manifest-declared diagnostic outputs or meters and are separated from conformance rows" -Description "markdown diagnostic output policy"
Assert-Contains -Text $reportText -Pattern "ideal_loads_report_export_order_policy: compare artifacts are exported after IdealLoads calculations populate comparison rows, meter rows, and ResultStore" -Description "markdown report export order"
Assert-Contains -Text $reportText -Pattern "ideal_loads_detailed_output_lookup_policy: Detailed output key/variable lookup is confined to post-calculation report assembly; simulation calculations use typed IDs and pre-resolved handles" -Description "markdown detailed output lookup"
Assert-Contains -Text $reportText -Pattern "ideal_loads_duplicate_output_handle_policy: duplicate manifest output requests fail during handle setup; duplicate ResultStore handles and identities fail ep_runtime::ResultStore::diagnostics" -Description "markdown duplicate handle policy"
Assert-Contains -Text $reportText -Pattern "trace_level_source: case manifest [trace].level" -Description "markdown trace_level_source"
Assert-Contains -Text $reportText -Pattern "trace_result_invariance_policy: trace level selects evidence payload only; ResultStore values are computed before report serialization" -Description "markdown trace result invariance"
Assert-Contains -Text $reportText -Pattern "trace_overhead_accounting: trace/report serialization overhead is outside numerical conformance comparison and measured separately from simulation results" -Description "markdown trace overhead accounting"
Assert-Contains -Text $reportText -Pattern "selected_purchased_air_branch: $expectedBranch" -Description "markdown selected_purchased_air_branch"
Assert-Contains -Text $reportText -Pattern "declared_ideal_loads_branch: $expectedBranch" -Description "markdown declared_ideal_loads_branch"
Assert-Contains -Text $reportText -Pattern "inactive_branches:" -Description "markdown inactive_branches"
Assert-Contains -Text $reportText -Pattern "source_map_anchor: docs/src/porting-map/ideal-loads-source-map.md" -Description "markdown source map anchor"
Assert-Contains -Text $reportText -Pattern "node_output_timestamp_alignment: timestamp" -Description "markdown node output timestamp alignment"
Assert-Contains -Text $reportText -Pattern "purchased_air_source_order: GetPurchasedAir -> InitPurchasedAir -> CalcPurchAirLoads -> UpdatePurchasedAir -> ReportPurchasedAir" -Description "markdown purchased air source order"

Write-Host "IdealLoads humidity-control annual meter conformance candidate artifacts generated for $CaseId."
