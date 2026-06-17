[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
$ScriptsRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..")).Path
. (Join-Path $ScriptsRoot "lib\common.ps1")
Add-CargoBinToPath

$RepoRoot = Get-RepoRoot
$OracleRoot = Join-Path $RepoRoot ".runtime\energyplus\26.1.0"
$OutputRoot = Join-Path $RepoRoot ".runtime\ideal-loads-no-oa-facility-meter-conformance\26.1.0"
$CaseId = "ideal_loads_no_oa_facility_meter_conformance_candidate_001"
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
    Write-Host "OK PurchasedAir source order: $($expectedPurchasedAirRoutines -join ' -> ')"
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
    if ($StageSummary.zone_equipment_scope -ne "single-zone-single-equipment") {
        throw "Unexpected zone equipment dispatch scope: $($StageSummary.zone_equipment_scope)"
    }
    if (@($StageSummary.zone_equipment_dispatch_issues).Count -ne 0) {
        throw "Unexpected zone equipment dispatch issues: $($StageSummary.zone_equipment_dispatch_issues -join ', ')"
    }
    if (@($StageSummary.zone_equipment_dispatch_warnings).Count -ne 0) {
        throw "Unexpected zone equipment dispatch warnings: $($StageSummary.zone_equipment_dispatch_warnings -join ', ')"
    }
    Write-Host "OK zone equipment dispatch path: $expectedPath"
}

function Assert-ZoneSysEnergyDemandEvidence {
    param([Parameter(Mandatory = $true)]$StageSummary)

    if ($StageSummary.zone_demand_struct_source -ne "src/EnergyPlus/DataZoneEnergyDemands.hh::ZoneSysEnergyDemand") {
        throw "Unexpected ZoneSysEnergyDemand source: $($StageSummary.zone_demand_struct_source)"
    }
    if ($StageSummary.zone_demand_heating_field -ne "RemainingOutputReqToHeatSP") {
        throw "Unexpected heating demand field: $($StageSummary.zone_demand_heating_field)"
    }
    if ($StageSummary.zone_demand_cooling_field -ne "RemainingOutputReqToCoolSP") {
        throw "Unexpected cooling demand field: $($StageSummary.zone_demand_cooling_field)"
    }
    if ($StageSummary.zone_demand_mismatch_classification -ne "upstream_zone_heat_balance_input") {
        throw "Unexpected demand mismatch classification: $($StageSummary.zone_demand_mismatch_classification)"
    }
    if ($StageSummary.zone_demand_fixture_mode -ne "source-order-oracle-demand-input") {
        throw "Unexpected demand fixture mode: $($StageSummary.zone_demand_fixture_mode)"
    }
    Write-Host "OK ZoneSysEnergyDemand source/sign metadata"
}

function Assert-NodeOutputEvidence {
    param([Parameter(Mandatory = $true)]$StageSummary)

    if ($StageSummary.node_output_store_type -ne "ep_runtime::ResultStore") {
        throw "Unexpected node output store type: $($StageSummary.node_output_store_type)"
    }
    if ($StageSummary.node_output_state_struct -ne "ep_runtime::node::IdealLoadsSupplyNodeUpdate") {
        throw "Unexpected node output state struct: $($StageSummary.node_output_state_struct)"
    }
    if ($StageSummary.node_output_update_source -ne "UpdatePurchasedAir") {
        throw "Unexpected node output update source: $($StageSummary.node_output_update_source)"
    }
    if ($StageSummary.node_output_report_source -ne "ReportPurchasedAir") {
        throw "Unexpected node output report source: $($StageSummary.node_output_report_source)"
    }
    Write-Host "OK node output store/update/report metadata"
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

Write-Host "Generating IdealLoads no-OA facility meter conformance comparison artifacts."
$output = & $cargo.Source run -p ep_cli --quiet -- conformance ideal-loads-no-oa-sensible-report $CasePath $OracleRoot $OutputRoot 2>&1
if ($LASTEXITCODE -ne 0) {
    $output | ForEach-Object { Write-Host $_ }
    throw "IdealLoads no-OA facility meter conformance comparison failed."
}

$text = ($output -join "`n")
Assert-Contains -Text $text -Pattern "IdealLoads No-OA Sensible Report" -Description "report header"
Assert-Contains -Text $text -Pattern "id: $CaseId" -Description "case id"
Assert-Contains -Text $text -Pattern "comparison_class: conformance" -Description "comparison class"
Assert-Contains -Text $text -Pattern "conformance_claim: true" -Description "claim boundary"
Assert-Contains -Text $text -Pattern "series: 28" -Description "series count"
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
    throw "IdealLoads meter conformance summary must set conformance_claim=true"
}
if ($summary.status -ne "pass") {
    throw "Unexpected IdealLoads meter conformance status: $($summary.status)"
}
if ($summary.tolerance_policy -ne "conformance-gate") {
    throw "Unexpected tolerance policy: $($summary.tolerance_policy)"
}
if ($summary.tolerance_failures -ne 0) {
    throw "IdealLoads meter conformance comparison must have zero tolerance failures: $($summary.tolerance_failures)"
}
if ($summary.samples -ne 110) {
    throw "Unexpected IdealLoads sample count: $($summary.samples)"
}
if ($summary.series_count -ne 28) {
    throw "Unexpected IdealLoads series count: $($summary.series_count)"
}
if ($summary.meter_source -ne "EnergyPlus Output:Meter hourly MTR vs Rust aggregated fuel-energy conformance") {
    throw "Unexpected meter source: $($summary.meter_source)"
}
if ($summary.zone_demand_synthetic_rc_model -ne $false) {
    throw "IdealLoads conformance must not synthesize zone demand from an RC shortcut"
}

$conformanceRows = @($summary.series | Where-Object { $_.level -eq "conformance" })
if ($conformanceRows.Count -ne 0) {
    throw "Expected zero conformance-level ESO output rows in meter-only candidate, found $($conformanceRows.Count)"
}
$diagnosticRows = @($summary.series | Where-Object { $_.level -eq "diagnostic" })
if ($diagnosticRows.Count -ne 28) {
    throw "Expected 28 diagnostic ESO output rows, found $($diagnosticRows.Count)"
}
if (@($diagnosticRows | Where-Object { $_.status -ne "pass" }).Count -ne 0) {
    throw "All diagnostic proof output rows must pass"
}

$nodeFlow = @($summary.series | Where-Object { $_.variable -eq "System Node Mass Flow Rate" })
if ($nodeFlow.Count -ne 1) {
    throw "Missing System Node Mass Flow Rate row"
}
if ($nodeFlow[0].alignment -ne "timestamp") {
    throw "System Node Mass Flow Rate must use timestamp alignment"
}
if ($nodeFlow[0].rust_source -ne "rust-ideal-loads-no-oa-sensible-calc") {
    throw "Unexpected node flow Rust source: $($nodeFlow[0].rust_source)"
}
if ($nodeFlow[0].level -ne "diagnostic") {
    throw "System Node Mass Flow Rate must remain diagnostic-level in the meter candidate"
}

$fuelRows = @($summary.series | Where-Object { $_.variable -like "Zone Ideal Loads *Fuel Energy Rate" })
if ($fuelRows.Count -ne 4) {
    throw "Expected 4 diagnostic fuel energy-rate rows, found $($fuelRows.Count)"
}
if (@($fuelRows | Where-Object { $_.level -ne "diagnostic" }).Count -ne 0) {
    throw "Fuel energy-rate rows must remain diagnostic-only"
}
if (@($fuelRows | Where-Object { $_.rust_source -ne "rust-ideal-loads-blank-fuel-efficiency" }).Count -ne 0) {
    throw "Fuel energy-rate rows must use the blank fuel-efficiency source"
}
$energyRows = @($summary.series | Where-Object { $_.variable -like "Zone Ideal Loads *Energy" -and $_.variable -notlike "*Energy Rate" })
if ($energyRows.Count -ne 8) {
    throw "Expected 8 diagnostic IdealLoads energy rows, found $($energyRows.Count)"
}
if (@($energyRows | Where-Object { $_.level -ne "diagnostic" -or $_.units -ne "J" }).Count -ne 0) {
    throw "IdealLoads energy rows must remain diagnostic joule rows"
}

if ([Math]::Abs([double]$summary.system_timestep_substeps - 8.0) -gt 1.0e-9) {
    throw "Unexpected IdealLoads system timestep substeps: $($summary.system_timestep_substeps)"
}
if ([Math]::Abs([double]$summary.system_timestep_seconds - 112.5) -gt 1.0e-9) {
    throw "Unexpected IdealLoads system timestep seconds: $($summary.system_timestep_seconds)"
}
if ([Math]::Abs([double]$summary.energy_report_interval_seconds - 900.0) -gt 1.0e-9) {
    throw "Unexpected IdealLoads energy report interval seconds: $($summary.energy_report_interval_seconds)"
}
if ($summary.rust_meter_time_series_comparison -ne $true) {
    throw "IdealLoads meter requests must compare Rust hourly facility meter series"
}
if ($summary.meter_aggregation_source -ne "ep_runtime::RuntimeMeterRegistry") {
    throw "Unexpected meter aggregation source: $($summary.meter_aggregation_source)"
}
if ($summary.meter_fuel_energy_binding_source -ne "ep_runtime::ideal_loads_facility_meter_binding") {
    throw "Unexpected meter fuel-energy binding source: $($summary.meter_fuel_energy_binding_source)"
}
if ($summary.requested_meter_count -ne 2) {
    throw "Expected 2 requested conformance meter rows, found $($summary.requested_meter_count)"
}

$requestedMeters = @($summary.requested_meters)
if ($requestedMeters.Count -ne 2) {
    throw "Expected 2 requested_meters entries, found $($requestedMeters.Count)"
}
if (@($requestedMeters | Where-Object { $_.name -eq "DistrictHeatingWater:Facility" -and $_.source -eq "mtr" -and $_.level -eq "conformance" }).Count -ne 1) {
    throw "Missing conformance DistrictHeatingWater:Facility MTR request in summary"
}
if (@($requestedMeters | Where-Object { $_.name -eq "DistrictCooling:Facility" -and $_.source -eq "mtr" -and $_.level -eq "conformance" }).Count -ne 1) {
    throw "Missing conformance DistrictCooling:Facility MTR request in summary"
}

$meterRows = @($summary.meter_series)
if ($summary.meter_series_count -ne 2 -or $meterRows.Count -ne 2) {
    throw "Expected 2 compared meter series, found count=$($summary.meter_series_count) rows=$($meterRows.Count)"
}
if ($summary.meter_tolerance_failures -ne 0) {
    throw "Expected zero meter tolerance failures, found $($summary.meter_tolerance_failures)"
}
if (@($meterRows | Where-Object { $_.level -ne "conformance" -or $_.status -ne "pass" }).Count -ne 0) {
    throw "All IdealLoads facility meter conformance rows must pass"
}
if (@($meterRows | Where-Object { $_.name -eq "DistrictHeatingWater:Facility" -and $_.rust_source -eq "rust-ideal-loads-hourly-facility-meter-from-fuel-energy" -and $_.alignment -eq "timestamp" -and $_.expected_samples -eq 24 -and $_.observed_samples -eq 24 -and $_.units -eq "J" }).Count -ne 1) {
    throw "Missing passing hourly heating facility meter conformance row"
}
if (@($meterRows | Where-Object { $_.name -eq "DistrictCooling:Facility" -and $_.rust_source -eq "rust-ideal-loads-hourly-facility-meter-from-fuel-energy" -and $_.alignment -eq "timestamp" -and $_.expected_samples -eq 24 -and $_.observed_samples -eq 24 -and $_.units -eq "J" }).Count -ne 1) {
    throw "Missing passing hourly cooling facility meter conformance row"
}

$toleranceFailures = @(Read-CsvFile -Path $toleranceFailuresPath)
if ($toleranceFailures.Count -ne 0) {
    throw "Expected empty tolerance-failures.csv, found $($toleranceFailures.Count) row(s)"
}

$resultStore = Read-JsonFile -Path $resultStorePath
if ($resultStore.series_count -ne 28 -or $resultStore.sample_count -ne 110) {
    throw "Unexpected result store shape: series=$($resultStore.series_count) samples=$($resultStore.sample_count)"
}

$selectedOutputs = Read-JsonFile -Path $selectedOutputsPath
if (@($selectedOutputs.series).Count -ne 28) {
    throw "Unexpected selected_outputs series count: $(@($selectedOutputs.series).Count)"
}

$oracleMtrText = Read-TextFile -Path $oracleMtrPath
Assert-Contains -Text $oracleMtrText -Pattern "DistrictHeatingWater:Facility" -Description "oracle MTR heating meter"
Assert-Contains -Text $oracleMtrText -Pattern "DistrictCooling:Facility" -Description "oracle MTR cooling meter"

$stageSummary = Read-JsonFile -Path $stageSummaryPath
if ($stageSummary.branch -ne "no-oa-no-limit-sensible") {
    throw "Unexpected IdealLoads branch: $($stageSummary.branch)"
}
if ($stageSummary.selected_purchased_air_branch -ne "constant_shr") {
    throw "Unexpected PurchasedAir branch: $($stageSummary.selected_purchased_air_branch)"
}
if ($stageSummary.declared_ideal_loads_branch -ne "no_oa_sensible") {
    throw "Unexpected declared IdealLoads branch: $($stageSummary.declared_ideal_loads_branch)"
}
if ($stageSummary.zone_demand_synthetic_rc_model -ne $false) {
    throw "Stage summary must record that no RC demand shortcut is used"
}
Assert-PurchasedAirSourceOrder -StageSummary $stageSummary
Assert-ZoneEquipmentDispatch -StageSummary $stageSummary
Assert-ZoneSysEnergyDemandEvidence -StageSummary $stageSummary
Assert-NodeOutputEvidence -StageSummary $stageSummary
if ($stageSummary.rate_output_source -ne "ReportPurchasedAir after UpdatePurchasedAir") {
    throw "Unexpected stage rate output source: $($stageSummary.rate_output_source)"
}
if ($stageSummary.energy_output_timestep_source -ne "ReportPurchasedAir rate * TimeStepSysSec") {
    throw "Unexpected stage energy timestep source: $($stageSummary.energy_output_timestep_source)"
}
if ($stageSummary.meter_aggregation_source -ne "ep_runtime::RuntimeMeterRegistry") {
    throw "Unexpected stage meter aggregation source: $($stageSummary.meter_aggregation_source)"
}
if ($stageSummary.meter_fuel_energy_binding_source -ne "ep_runtime::ideal_loads_facility_meter_binding") {
    throw "Unexpected stage meter fuel-energy binding source: $($stageSummary.meter_fuel_energy_binding_source)"
}

$reportText = Read-TextFile -Path $reportPath
Assert-Contains -Text $reportText -Pattern "claim_boundary: conformance no-OA hourly IdealLoads facility meter aggregation for declared facility meters only" -Description "markdown claim boundary"
Assert-Contains -Text $reportText -Pattern "zone_demand_synthetic_rc_model: false" -Description "markdown demand source guard"
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
Assert-Contains -Text $reportText -Pattern "ideal_loads_output_handle_registration_policy: manifest output requests are resolved to stable OutputHandle values before IdealLoads comparison rows are evaluated" -Description "markdown IdealLoads output handle registration policy"
Assert-Contains -Text $reportText -Pattern "ideal_loads_output_handle_write_policy: rate and node ResultStore series use pre-resolved OutputHandle values; meter rows use RuntimeMeterRegistry-resolved handles before aggregation" -Description "markdown IdealLoads output handle write policy"
Assert-Contains -Text $reportText -Pattern "ideal_loads_diagnostic_output_request_policy: diagnostic rows are emitted only for manifest-declared diagnostic outputs or meters and are separated from conformance rows" -Description "markdown IdealLoads diagnostic output request policy"
Assert-Contains -Text $reportText -Pattern "ideal_loads_report_export_order_policy: compare artifacts are exported after IdealLoads calculations populate comparison rows, meter rows, and ResultStore" -Description "markdown IdealLoads report export order policy"
Assert-Contains -Text $reportText -Pattern "ideal_loads_detailed_output_lookup_policy: Detailed output key/variable lookup is confined to post-calculation report assembly; simulation calculations use typed IDs and pre-resolved handles" -Description "markdown IdealLoads detailed output lookup policy"
Assert-Contains -Text $reportText -Pattern "ideal_loads_duplicate_output_handle_policy: duplicate manifest output requests fail during handle setup; duplicate ResultStore handles and identities fail ep_runtime::ResultStore::diagnostics" -Description "markdown IdealLoads duplicate output handle policy"
Assert-Contains -Text $reportText -Pattern "trace_level_source: case manifest [trace].level" -Description "markdown trace level source"
Assert-Contains -Text $reportText -Pattern "trace_result_invariance_policy: trace level selects evidence payload only; ResultStore values are computed before report serialization" -Description "markdown trace result invariance policy"
Assert-Contains -Text $reportText -Pattern "trace_overhead_accounting: trace/report serialization overhead is outside numerical conformance comparison and measured separately from simulation results" -Description "markdown trace overhead accounting"
Assert-Contains -Text $reportText -Pattern "selected_purchased_air_branch: constant_shr" -Description "markdown PurchasedAir branch"
Assert-Contains -Text $reportText -Pattern "declared_ideal_loads_branch: no_oa_sensible" -Description "markdown declared branch"
Assert-Contains -Text $reportText -Pattern "inactive_branches: outdoor_air, economizer, heat_recovery, humidistat, dcv, autosizing, saturation_limit" -Description "markdown inactive branches"
Assert-Contains -Text $reportText -Pattern "source_map_anchor: docs/src/porting-map/ideal-loads-source-map.md" -Description "markdown source-map anchor"
Assert-Contains -Text $reportText -Pattern "node_output_timestamp_alignment: timestamp" -Description "markdown node timestamp alignment"
Assert-Contains -Text $reportText -Pattern "zone_equipment_dispatch_path: ZoneEquipmentManager::ManageZoneEquipment -> SimZoneEquipment -> ZoneEquipType::PurchasedAir -> PurchasedAirManager::SimPurchasedAir" -Description "markdown zone equipment dispatch path"
Assert-Contains -Text $reportText -Pattern "purchased_air_source_order: GetPurchasedAir -> InitPurchasedAir -> CalcPurchAirLoads -> UpdatePurchasedAir -> ReportPurchasedAir" -Description "markdown PurchasedAir source order"
Assert-Contains -Text $reportText -Pattern "rate_output_source: ReportPurchasedAir after UpdatePurchasedAir" -Description "markdown rate output source"
Assert-Contains -Text $reportText -Pattern "energy_output_timestep_source: ReportPurchasedAir rate * TimeStepSysSec" -Description "markdown energy output timestep source"
Assert-Contains -Text $reportText -Pattern "meter_source: EnergyPlus Output:Meter hourly MTR vs Rust aggregated fuel-energy conformance; rust_meter_time_series_comparison=true requested_meters=2" -Description "markdown meter source"
Assert-Contains -Text $reportText -Pattern "meter_aggregation_source: ep_runtime::RuntimeMeterRegistry" -Description "markdown meter aggregation source"
Assert-Contains -Text $reportText -Pattern "meter_fuel_energy_binding_source: ep_runtime::ideal_loads_facility_meter_binding" -Description "markdown meter fuel-energy binding source"
Assert-Contains -Text $reportText -Pattern "meter_requests: DistrictHeatingWater:Facility, DistrictCooling:Facility" -Description "markdown meter requests"
Assert-Contains -Text $reportText -Pattern "| DistrictHeatingWater:Facility | conformance | meter | hourly | mtr | rust-ideal-loads-hourly-facility-meter-from-fuel-energy" -Description "markdown heating meter row"
Assert-Contains -Text $reportText -Pattern "| DistrictCooling:Facility | conformance | meter | hourly | mtr | rust-ideal-loads-hourly-facility-meter-from-fuel-energy" -Description "markdown cooling meter row"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE INLET | System Node Mass Flow Rate | diagnostic" -Description "markdown diagnostic node flow row"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE IDEAL LOADS | Zone Ideal Loads Zone Heating Fuel Energy Rate | diagnostic" -Description "markdown diagnostic zone heating fuel rate row"
Assert-Contains -Text $reportText -Pattern "| ZONE ONE IDEAL LOADS | Zone Ideal Loads Zone Heating Fuel Energy | diagnostic" -Description "markdown diagnostic zone heating fuel energy row"

Write-Host "IdealLoads no-OA facility meter conformance comparison artifacts generated."
